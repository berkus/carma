//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//

//
// Provide storage for in-memory level-data - models, meshes, textures etc.
//
struct RenderManager {
    vertices: Vec<VertexBuffer>,
    indices: Vec<IndexBuffer>,
    bound_textures: Vec<TextureAny>,
}

impl RenderManager {
    fn prepare_car(car: &Car) {
        // Load VertexBuffer
        // Load IndexBuffers for each material
        // Set up renderlist (VertexBuffer, IndexBuffer)
        let vbo = glium::vertex::VertexBuffer::new(&display, &car.meshes["SCREWIE"].vertices).unwrap();

        let faces = &car.meshes["SCREWIE"].faces;
        let mut indices: Vec<u16> = Vec::<u16>::with_capacity(faces.len()*3);

        for face in faces {
            indices.push(face.v1);
            indices.push(face.v2);
            indices.push(face.v3);
        }

        let indices = glium::index::IndexBuffer::new(
            &display, PrimitiveType::TrianglesList, &indices).unwrap();

        faces.partition(|face| face.material_id)
    }

    pub fn draw_car(car: &Car) {

    }

    pub fn draw_actor(actor: &Actor) {
        // Single mesh, but specific indices to draw with each material.

    }
}
