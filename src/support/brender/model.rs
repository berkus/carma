use {
    super::resource::{
        file_type, stack, Chunk, FaceMaterialChunk, FileInfoChunk, FromStream, MaterialIndexChunk,
        PivotChunk, ResourceStack, ResourceTag, VertexUvChunk,
    },
    crate::support,
    byteorder::ReadBytesExt,
    fehler::{throw, throws},
    std::io::prelude::BufRead,
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
//     let mut m = Mesh::default();
//     let /*mut*/ fmlist = Vec::<u16>::new();
//     let /*mut*/ uvcoords = Vec::<UvCoord>::new();

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
