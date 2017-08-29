//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use std::str;
use std::collections::HashMap;
use std::vec::Vec;
use glium;
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium::index::*;
use glium::texture::{RawImage2d, SrgbTexture2d};
use support::car::Car;
use support::Vertex;
use support::camera::CameraState;
use support::actor::ActorNode;
use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4};

/// Provide storage for in-memory level-data - models, meshes, textures etc.
pub struct RenderManager {
    vertices: HashMap<String, VertexBuffer<Vertex>>,
    indices: HashMap<String, HashMap<u16, IndexBuffer<u16>>>, // MaterialId -> index buffer
    bound_textures: HashMap<String, HashMap<u16, SrgbTexture2d>>, // MaterialId -> texture
    program: Program,
}

// Load VertexBuffer
// Load IndexBuffers for each material
// Set up renderlist (VertexBuffer, IndexBuffer)
impl RenderManager {
    pub fn new(display: &Display) -> Self {
        let vertex_shader_src = str::from_utf8(include_bytes!("../../shaders/first.vert")).unwrap();
        let fragment_shader_src =
            str::from_utf8(include_bytes!("../../shaders/first.frag")).unwrap();

        Self {
            vertices: HashMap::new(),
            indices: HashMap::new(),
            bound_textures: HashMap::new(),
            program: Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
        }
    }

    fn debug_indices(&self) {
        for (name, _indices) in &self.indices {
            println!("Indices for {}:", name);
            // for () in &indices {
            //     println!("  ", )
            // }
        }
    }

    fn bind_default_texture(
        textures: &mut HashMap<u16, SrgbTexture2d>,
        mat: u16,
        display: &Display,
    ) {
        let black_data = [0; 32 * 32 * 4];
        let black_image = RawImage2d::from_raw_rgba_reversed(&black_data, (32, 32));
        let black_texture = SrgbTexture2d::new(display, black_image).unwrap();
        textures.insert(mat, black_texture);
    }

    // @todo Prepare megatexture from all these small textures and keep a map
    // of texture ID to the rect region, scale u,v appropriately in vertices.
    // In theory, whole of the game could fit in 4096x4096 megatex.
    fn bind_textures(&mut self, actor_name: &String, car: &Car, display: &Display) {
        for (&mat, _) in &self.indices[actor_name] {
            let textures = self.bound_textures
                .entry(actor_name.clone())
                .or_insert(HashMap::new());
            if mat == 0 {
                RenderManager::bind_default_texture(textures, mat, display);
            } else {
                let material = &car.meshes[actor_name].material_names[(mat - 1) as usize];
                // println!("Referred material {} index {}", material, mat);
                if let Some(m) = car.materials.get(material) {
                    // println!("Found material {}", m);
                    let mut name = m.pixelmap_name.clone();
                    if name.is_empty() {
                        // @fixme hack
                        name = material.replace(".MAT", ".pix").to_lowercase();
                    }
                    if let Some(tex) = car.textures.get(&name) {
                        // println!("Found texture {}", tex);
                        let image = RawImage2d::from_raw_rgba_reversed(
                            &tex.data,
                            (tex.w as u32, tex.h as u32),
                        );
                        let bound_texture = SrgbTexture2d::new(display, image).unwrap();
                        textures.insert(mat, bound_texture);
                    } else {
                        RenderManager::bind_default_texture(textures, mat, display);
                    }
                }
            }
        }
    }

    pub fn prepare_car(&mut self, car: &Car, display: &Display) {
        for actor in car.actors.traverse() {
            match actor.data() {
                &ActorNode::MeshfileRef(ref name) => {
                    println!("Actor meshfile {}", name);
                    self.prepare_car_actor(name, car, display);
                }
                _ => (),
            }
        }
    }

    pub fn prepare_car_actor(&mut self, name: &String, car: &Car, display: &Display) {
        println!("prepare_car_actor({}): loading vertices", name);
        let vbo = VertexBuffer::<Vertex>::new(display, &car.meshes[name].vertices).unwrap();
        self.vertices.insert(name.clone(), vbo);

        println!("prepare_car_actor({}): partitioning faces", name);

        let faces = &car.meshes[name].faces;

        let mut partitioned_by_material = HashMap::<u16, Vec<u16>>::new();

        for face in faces {
            let indices = partitioned_by_material
                .entry(face.material_id)
                .or_insert(Vec::new());
            indices.push(face.v1);
            indices.push(face.v2);
            indices.push(face.v3);
        }

        for (mat, list) in &partitioned_by_material {
            println!(
                "Material {}: {} vertices, {} faces",
                mat,
                list.len(),
                list.len() as f32 / 3f32
            );
        }

        self.indices.insert(
            name.clone(),
            partitioned_by_material
                .iter()
                .map(|(key, item)| {
                    (
                        *key,
                        IndexBuffer::new(display, PrimitiveType::TrianglesList, &item).unwrap(),
                    )
                })
                .collect(),
        );

        // each material from partitioned_by_material - load and bind it in bound_textures
        // then remap to a set of HashMap from String material name to Vec<u16> indices
        self.bind_textures(name, car, display);
    }

    /// Draw all visible actors
    pub fn draw_car<T>(&self, car: &Car, target: &mut T, camera: &CameraState)
    where
        T: Surface,
    {
        let mut v = false;
        let mut transform_stack = Vec::<Matrix4<f32>>::new();
        let mut model = Matrix4::<f32>::identity();
        let mut last_depth = 0;

        for actor in car.actors.traverse() {
            match actor.data() {
                &ActorNode::Actor { name: _, visible } => v = visible,
                &ActorNode::MeshfileRef(ref name) => if v {
                    // println!("Drawing actor {}", name);
                    self.draw_actor(name, &model, target, camera);
                },
                &ActorNode::Transform(t) => {
                    let transform = Matrix4::from_nonuniform_scale(t[0], t[4], t[8]);
                    let transform = Matrix4::from_translation(
                        Vector3 { x: t[9], y: t[10], z: t[11] }) * transform;
 
                    let depth = car.actors.get_node_depth(actor.parent().unwrap());
                    if depth > last_depth {
                        transform_stack.push(model);
                        model = transform * model;
                    } else if depth < last_depth {
                        model = transform_stack.pop().unwrap();
                    } else {
                        model = transform * transform_stack.last().unwrap();
                    }
                    last_depth = depth;
                }
                _ => (),
            }
        }
    }

    /// Uses single mesh, but specific indices to draw with each material.
    fn draw_actor<T>(
        &self,
        mesh_name: &String,
        model: &Matrix4<f32>,
        target: &mut T,
        camera: &CameraState,
    ) where
        T: Surface,
    {
        println!("Rendering {} with model {:?}", mesh_name, model);

        // the direction of the light - @todo more light sources?
        let light = [-5.0, 5.0, 10.0f32];
        // Ambient lighting: 0.5, 0.5, 0.5, 1.0

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let model: [[f32; 4]; 4] = model.clone().into();

        for (mat, indices) in &self.indices[mesh_name] {
            let uniforms = uniform! {
                model: model,
                view: camera.get_view(),
                perspective: camera.get_perspective(),
                u_light: light,
                u_specular_color: [1.0, 1.0, 1.0f32],
                diffuse_tex: &self.bound_textures[mesh_name][mat],
                // normal_tex: &self.bound_textures[mat],
            };

            target
                .draw(
                    &self.vertices[mesh_name],
                    indices,
                    &self.program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }
    }
}
