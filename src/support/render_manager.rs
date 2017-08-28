//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use glium::vertex::*;
use glium::index::*;
use glium::Display;
use glium::texture::TextureAny;
use support::car::Car;
use support::actor::Actor;
use support::Vertex;
use std::collections::HashMap;
use std::vec::Vec;

/// Provide storage for in-memory level-data - models, meshes, textures etc.
#[derive(Default)]
pub struct RenderManager {
    pub vertices: Vec<VertexBuffer<Vertex>>,
    pub indices: Vec<IndexBuffer<u16>>,
    pub bound_textures: Vec<TextureAny>,
}

// Load VertexBuffer
// Load IndexBuffers for each material
// Set up renderlist (VertexBuffer, IndexBuffer)
impl RenderManager {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            bound_textures: Vec::new(),
        }
    }

    pub fn prepare_car(&mut self, car: &Car, display: &Display) {
        let vbo = VertexBuffer::<Vertex>::new(display, &car.meshes["SCREWIE"].vertices).unwrap();
        self.vertices.push(vbo);

        let faces = &car.meshes["SCREWIE"].faces;

        let mut sorted_by_material = HashMap::<u16, Vec<u16>>::new();

        for face in faces {
            if let None = sorted_by_material.get(&face.material_id) {
                sorted_by_material.insert(face.material_id, Vec::new());
            }
            let indices = sorted_by_material.get_mut(&face.material_id).unwrap();
            indices.push(face.v1);
            indices.push(face.v2);
            indices.push(face.v3);
        }

        // each material from sorted_by_material - load and bind it in bound_textures
        // then remap to a set of HashMap from String material name to Vec<u16> indices
        //

        let indices = IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &sorted_by_material[&0u16],
        ).unwrap();

        self.indices.push(indices);
    }

    pub fn draw_car(_car: &Car) {}

    pub fn draw_actor(_actor: &Actor) {
        // Single mesh, but specific indices to draw with each material.
    }
}
