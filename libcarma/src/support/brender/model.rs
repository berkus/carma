use {
    super::resource::{
        file_type, stack, Chunk, FaceMaterialChunk, FileInfoChunk, FromStream, MaterialIndexChunk,
        PivotChunk, ResourceStack, ResourceTag, VertexUvChunk,
    },
    crate::support,
    byteorder::ReadBytesExt,
    cgmath::{InnerSpace, Vector3, Zero},
    fehler::{throw, throws},
    std::{
        fs::File,
        io::{prelude::BufRead, BufReader},
    },
};

// @todo ❌ convert meshes to bevy_render::mesh::Mesh

#[derive(Default)]
struct Model {
    pub name: String,
    // pub vertices: Vec<Vertex>,
    // pub faces: Vec<Face>,
    // pub material_names: Vec<String>,
    pivot: Vec3f,
}

impl FromStream for Model {
    type Output = Model;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut stack = ResourceStack::default();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::MODEL {
                        throw!(support::Error::InvalidResourceType {
                            expected: file_type::MODEL,
                            received: file_type,
                        });
                    }
                }
                // Chunk::FileName { name, subtype } => {
                //     m.name = name;
                //     if subtype != support::MODEL_FILE_SUBTYPE {
                //         return Err(anyhow!("Invalid mesh file subtype {}", subtype));
                //     }
                // }
                Chunk::Model(model) => {
                    stack.push(stack::MODEL, ResourceTag::Model(Box::new(model)));
                }
                Chunk::MaterialIndex(MaterialIndexChunk { materials }) => {
                    let mut model = stack.top(stack::MODEL)?;
                    model.materials = materials;
                }
                Chunk::Vertices(VerticesChunk { vertices }) => {
                    let mut model = stack.top(stack::MODEL)?;
                    model.vertices = vertices;
                }
                Chunk::VertexUV(VertexUvChunk { uvs }) => {
                    let mut model = stack.top(stack::MODEL)?;
                    model.vertex_uv = uvs;
                }
                Chunk::Faces(faces) => {
                    let mut model = stack.top(stack::MODEL)?;
                    model.faces = faces;
                }
                Chunk::FaceMaterial(FaceMaterialChunk {
                    face_material_indices,
                }) => {
                    let mut model = stack.top(stack::MODEL)?;
                    model.face_material_indices = face_material_indices;
                }
                Chunk::Pivot(PivotChunk { pivot }) => {
                    let mut model = stack.top(stack::MODEL)?;
                    model.pivot = pivot;
                }
                _ => unimplemented!(), // unexpected type for a model file
            }
        }

        stack.pop(stack::Model)?
    }
}

// pub fn load<R: ReadBytesExt + BufRead>(_reader: &mut R) -> Result<Mesh> {
//     for (i, fm) in fmlist.iter().enumerate().take(m.faces.len()) {
//         m.faces[i].material_id = *fm;
//     }

//     for (n, uvcoord) in uvcoords.iter().enumerate() {
//         // Carma uses 0.0,0.0 for the top left corner, OpenGL for the bottom left.
//         m.vertices[n].tex_coords = [uvcoord.u, 1.0 - uvcoord.v];
//     }

//     m.calc_normals();
//     Ok(m)
// }

impl Model {
    // Single model file may contain multiple models
    #[throws(support::Error)]
    pub fn load_many(fname: String) -> Vec<Model> {
        let file = File::open(fname)?;
        let mut file = BufReader::new(file);
        let mut models = Vec::<_>::new();
        loop {
            let m = Model::from_stream(&mut file);
            match m {
                Err(_) => break, // fixme: allow only Eof here
                Ok(m) => models.push(m),
            }
        }
        models
    }

    pub fn calc_normals(&mut self) {
        for face in &self.faces {
            let normal: [f32; 3] = calc_plane_normal(
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

/// Calculate normal from three vertices in counter-clockwise order.
pub fn calc_plane_normal(v1: Vector3<f32>, v2: Vector3<f32>, v3: Vector3<f32>) -> Vector3<f32> {
    (v1 - v2).cross(v2 - v3).normalize()
}

#[cfg(test)]
mod tests {
    use {super::*, std::io::Cursor};

    #[test]
    fn test_load_model() {
        #[rustfmt::skip]
        let mut data = Cursor::new(vec![
            0x0, 0x0, 0x0, 0x36, // Chunk type - FILE_INFO_CHUNK
            0x0, 0x0, 0x0, 0x8, // Chunk size
            0x0, 0x3, // subtype u16
            b'h', b'e', b'l', b'l', b'o', 0, // Chunk contents
            0x0, 0x0, 0x0, 0x0, // Chunk type - NULL_CHUNK
            0x0, 0x0, 0x0, 0x0, // Chunk size
        ]);
        let m = Model::from_stream(&mut data).unwrap();
        assert_eq!("hello", m.name);
        // assert_eq!(0xbeef, m.v2);
        // assert_eq!(0xcafe, m.v3);
        // assert_eq!(0xbabe, m.flags);
    }

    // test that normals to unit vectors will be the third unit vector
    #[test]
    fn test_calc_normal() {
        assert_eq!(
            calc_plane_normal(Vector3::unit_y(), Vector3::zero(), Vector3::unit_x()),
            Vector3::unit_z()
        );
        assert_eq!(
            calc_plane_normal(Vector3::unit_x(), Vector3::zero(), Vector3::unit_y()),
            -Vector3::unit_z()
        );
    }
} // tests mod
