//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{brender::resource::VertexUV, Vertex},
    anyhow::Result,
    byteorder::ReadBytesExt,
    cgmath::{InnerSpace, Vector3},
    std::{
        fs::File,
        io::{BufRead, BufReader},
    },
};

// @todo ❌ replace with conversion to usual Bevy mesh
// @todo ❌ impl Into<Mesh>/TryInto<Mesh>?

// VertexUV in resource.rs
type UvCoord = VertexUV;

// This should be Model @todo
impl Mesh {
    // Single mesh file may contain multiple meshes
    #[throws(support::Error)]
    pub fn load_many(fname: String) -> Vec<Mesh> {
        let file = File::open(fname)?;
        let mut file = BufReader::new(file);
        let mut meshes = Vec::<Mesh>::new();
        loop {
            let m = Mesh::from_stream(&mut file);
            match m {
                Err(_) => break, // fixme: allow only Eof here
                Ok(m) => meshes.push(m),
            }
        }
        Ok(meshes)
    }

    // Calculate normal from vertices in counter-clockwise order.
    pub fn calc_normal(v1: Vector3<f32>, v2: Vector3<f32>, v3: Vector3<f32>) -> Vector3<f32> {
        (v1 - v2).cross(v2 - v3).normalize()
    }

    pub fn calc_normals(&mut self) {
        for face in &self.faces {
            let normal: [f32; 3] = Self::calc_normal(
                self.vertices[face.v1 as usize].position.into(),
                self.vertices[face.v2 as usize].position.into(),
                self.vertices[face.v3 as usize].position.into(),
            )
            .into();
            self.vertices[face.v1 as usize].normal = normal;
            self.vertices[face.v2 as usize].normal = normal;
            self.vertices[face.v3 as usize].normal = normal;
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::io::Cursor};

    #[test]
    fn test_load_mesh() {
        #[rustfmt::skip]
        let mut data = Cursor::new(vec![
            0x0, 0x0, 0x0, 0x36, // Chunk type - FILE_NAME_CHUNK
            0x0, 0x0, 0x0, 0x8, // Chunk size
            0x0, 0x3, // subtype u16
            b'h', b'e', b'l', b'l', b'o', 0, // Chunk contents
            0x0, 0x0, 0x0, 0x0, // Chunk type - NULL_CHUNK
            0x0, 0x0, 0x0, 0x0, // Chunk size
        ]);
        let m = Mesh::from_stream(&mut data).unwrap();
        assert_eq!("hello", m.name);
        // assert_eq!(0xbeef, m.v2);
        // assert_eq!(0xcafe, m.v3);
        // assert_eq!(0xbabe, m.flags);
    }

    // test that normals to unit vectors will be the third unit vector
    #[test]
    fn test_calc_normal() {
        assert_eq!(
            Mesh::calc_normal(Vector3::unit_y(), Vector3::zero(), Vector3::unit_x()),
            Vector3::unit_z()
        );
        assert_eq!(
            Mesh::calc_normal(Vector3::unit_x(), Vector3::zero(), Vector3::unit_y()),
            -Vector3::unit_z()
        );
    }
} // tests mod
