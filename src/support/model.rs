//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use support::Error;
use std::io::BufRead;
use byteorder::ReadBytesExt;
use support::resource::Chunk;

#[derive(Default)]
pub struct Model {
    name: String,
    visible: bool,
    transform: [f32; 12],
    material_file: String,
    mesh_file: String,
}

impl Model {
    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<Model, Error> {
        let mut m = Model::default();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            let c = Chunk::load(rdr)?;
            match c {
                Chunk::ActorName { name, visible } => {
                    m.name = name;
                    m.visible = visible;
                },
                Chunk::ActorTransform(transform) => {
                    m.transform = transform;
                },
                Chunk::MaterialRef(name) => {
                    m.material_file = name;
                },
                Chunk::MeshFileRef(name) => {
                    m.mesh_file = name;
                },
                Chunk::Unknown25() => {},
                Chunk::Unknown2A() => {},
                Chunk::Null() => break,
                _ => unimplemented!(), // unexpected type here
            }
        }

        Ok(m)
    }
}
