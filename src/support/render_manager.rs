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
use cgmath::prelude::*;
use cgmath::*;

/// Provide storage for in-memory level-data - models, meshes, textures etc.
pub struct RenderManager {
    vertices: Vec<VertexBuffer<Vertex>>,
    indices: HashMap<u16, IndexBuffer<u16>>, // MaterialId -> index buffer
    bound_textures: HashMap<u16, SrgbTexture2d>, // MaterialId -> texture
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
            vertices: Vec::new(),
            indices: HashMap::new(),
            bound_textures: HashMap::new(),
            program: Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
        }
    }

    fn bind_textures(&mut self, car: &Car, display: &Display) {
        for (&mat, _) in &self.indices {
            let material = &car.meshes["SCREWIE"].material_names[(mat - 1) as usize];
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
                    let image =
                        RawImage2d::from_raw_rgba_reversed(&tex.data, (tex.w as u32, tex.h as u32));
                    let bound_texture = SrgbTexture2d::new(display, image).unwrap();
                    self.bound_textures.insert(mat, bound_texture);
                } else {
                    let black_data = [0; 32 * 32 * 4];
                    let black_image = RawImage2d::from_raw_rgba_reversed(&black_data, (32, 32));
                    let black_texture = SrgbTexture2d::new(display, black_image).unwrap();
                    self.bound_textures.insert(mat, black_texture);
                }
            }
        }
    }

    // @todo Extend this to run for multiple actors, currently only SCREWIE is done
    pub fn prepare_car(&mut self, car: &Car, display: &Display) {
        let vbo = VertexBuffer::<Vertex>::new(display, &car.meshes["SCREWIE"].vertices).unwrap();
        self.vertices.push(vbo);

        let faces = &car.meshes["SCREWIE"].faces;

        let mut partitioned_by_material = HashMap::<u16, Vec<u16>>::new();

        for face in faces {
            if let None = partitioned_by_material.get(&face.material_id) {
                partitioned_by_material.insert(face.material_id, Vec::new());
            } //@todo .entry(..).or_insert(..) here?
            let indices = partitioned_by_material.get_mut(&face.material_id).unwrap();
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

        self.indices = partitioned_by_material
            .iter()
            .map(|(key, item)| {
                (
                    *key,
                    IndexBuffer::new(display, PrimitiveType::TrianglesList, &item).unwrap(),
                )
            })
            .collect();

        // each material from partitioned_by_material - load and bind it in bound_textures
        // then remap to a set of HashMap from String material name to Vec<u16> indices
        self.bind_textures(car, display);
    }

    pub fn draw_car<T>(&self, _car: &Car, target: &mut T, camera: &CameraState)
    where
        T: Surface,
    {
        // Single mesh, but specific indices to draw with each material.

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

        let model: [[f32; 4]; 4] = (Matrix4::from_angle_y(Deg(45.0)) * Matrix4::identity()).into();

        for (mat, indices) in &self.indices {
            let uniforms = uniform! {
                model: model,
                view: camera.get_view(),
                perspective: camera.get_perspective(),
                u_light: light,
                u_specular_color: [1.0, 1.0, 1.0f32],
                diffuse_tex: &self.bound_textures[mat],
                // normal_tex: &self.bound_textures[mat],
            };

            target
                .draw(
                    &self.vertices[0],
                    indices,
                    &self.program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }
    }
}
