use {
    super::resource::{
        file_type, Chunk, FaceMaterialChunk, FacesChunk, FileInfoChunk, FromStream,
        MaterialIndexChunk, PivotChunk, ResourceStack, ResourceTag, Vec3f, VertexUV, VertexUvChunk,
        VerticesChunk,
    },
    crate::support::Error,
    bevy::render::mesh::VertexAttributeValues,
    byteorder::ReadBytesExt,
    carma_derive::ResourceTag,
    cgmath::{InnerSpace, Vector3},
    culpa::{throw, throws},
    std::{
        fs::File,
        io::{prelude::BufRead, BufReader},
    },
};

// @todo ❌ convert meshes to bevy_render::mesh::Mesh

#[derive(Default, ResourceTag)]
pub struct Model {
    pub name: String,
    pub vertices: Vec<Vec3f>,
    pub vertex_normals: Vec<Vec3f>,
    pub vertex_uvs: Vec<VertexUV>,
    pub faces: FacesChunk,
    pub material_names: Vec<String>, // vvv @todo ❌ convert to material refs
    // pub materials: Vec<Material>,
    pub face_material_indices: Vec<u16>,
    pub pivot: Vec3f,
}

impl Model {
    fn bevy_vertices(&self) -> VertexAttributeValues {
        VertexAttributeValues::Float32x3(self.vertices.iter().map(|v| [v.x, v.y, v.z]).collect())
    }

    fn bevy_vertex_normals(&self) -> VertexAttributeValues {
        VertexAttributeValues::Float32x3(
            self.vertex_normals
                .iter()
                .map(|n| [n.x, n.y, n.z])
                .collect(),
        )
    }

    fn bevy_vertex_uvs(&self) -> VertexAttributeValues {
        VertexAttributeValues::Float32x2(self.vertex_uvs.iter().map(|t| [t.u, t.v]).collect())
    }

    fn bevy_faces(&self) -> bevy::render::mesh::Indices {
        bevy::render::mesh::Indices::U16(
            self.faces
                .faces
                .iter()
                .map(|f| [f.v1, f.v2, f.v3])
                .flatten()
                .collect(),
        )
    }

    pub fn bevy_mesh(&self) -> bevy::render::mesh::Mesh {
        use bevy::render::mesh::{Mesh, PrimitiveTopology};

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        // Positions of the vertices
        // See https://bevy-cheatbook.github.io/features/coords.html
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            self.bevy_vertices(),
            // vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
        );

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            self.bevy_vertex_normals(),
            //vec![[0., 1., 0.]; 3]
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            self.bevy_vertex_uvs(),
            //vec![[0., 0.]; 3]
        );

        // A triangle using vertices 0, 2, and 1.
        // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
        mesh.set_indices(Some(
            self.bevy_faces(),
            // vec![0, 2, 1])
        ));

        // @todo Materials!

        // Optionally, for more complicated geometry, instead of setting normals manually with
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL,...), you can do:
        // mesh.duplicate_vertices();
        //      It's because by default the normals are interpolated. You do have the option
        //      in WGSL to disable this using `flat` interpolation, but then all meshes rendered
        //      using that shader would loose the interpolated normals as discussed here. https://stackoverflow.com/questions/60022613/how-to-implement-flat-shading-in-opengl-without-duplicate-vertices
        //      So if you want to mix flat and smooth shaded meshes using the same shader,
        //      duplicating vertices is the only option you have.
        // mesh.compute_flat_normals();
        // after mesh.set_indices(...).
        // Note the warnings about increasing the vertex count.
        mesh
    }
}

impl FromStream for Model {
    type Output = Box<Model>;

    #[throws]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut stack = ResourceStack::new();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::MODEL {
                        throw!(
                            Error::InvalidResourceType /*{
                                                       expected: file_type::MODEL,
                                                       received: file_type,
                                                       }*/
                        );
                    }
                }
                // Chunk::FileName { name, subtype } => {
                //     m.name = name;
                //     if subtype != support::MODEL_FILE_SUBTYPE {
                //         return Err(anyhow!("Invalid mesh file subtype {}", subtype));
                //     }
                // }
                Chunk::Model(model) => {
                    stack.push(Box::new(model));
                }
                Chunk::MaterialIndex(MaterialIndexChunk { materials }) => {
                    let model = stack.top::<Model>().ok_or(Error::InvalidResourceType)?;
                    model.material_names = materials;
                }
                Chunk::Vertices(VerticesChunk { vertices }) => {
                    let model = stack.top::<Model>().ok_or(Error::InvalidResourceType)?;
                    model.vertices = vertices;
                }
                Chunk::VertexUV(VertexUvChunk { uvs }) => {
                    let model = stack.top::<Model>().ok_or(Error::InvalidResourceType)?;
                    model.vertex_uvs = uvs;
                }
                Chunk::Faces(faces) => {
                    let model = stack.top::<Model>().ok_or(Error::InvalidResourceType)?;
                    model.faces = faces;
                }
                Chunk::FaceMaterial(FaceMaterialChunk {
                    face_material_indices,
                }) => {
                    let model = stack.top::<Model>().ok_or(Error::InvalidResourceType)?;
                    model.face_material_indices = face_material_indices;
                }
                Chunk::Pivot(PivotChunk { pivot }) => {
                    let model = stack.top::<Model>().ok_or(Error::InvalidResourceType)?;
                    model.pivot = pivot;
                }
                _ => unimplemented!(), // unexpected type for a model file
            }
        }

        stack.pop::<Model>()?
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
    #[throws]
    pub fn load_many(fname: String) -> Vec<Box<Model>> {
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
        // @todo replace with mesh.compute_flat_normals()?
        // for face in &self.faces.faces {
        //     let normal: [f32; 3] = calc_plane_normal(
        //         self.vertices[face.v1 as usize].into(),
        //         self.vertices[face.v2 as usize].into(),
        //         self.vertices[face.v3 as usize].into(),
        //     )
        //     .into();
        //     self.vertices[face.v1 as usize].normal = normal;
        //     self.vertices[face.v2 as usize].normal = normal;
        //     self.vertices[face.v3 as usize].normal = normal;
        // }
    }
}

/// Calculate normal from three vertices in counter-clockwise order.
pub fn calc_plane_normal(v1: Vector3<f32>, v2: Vector3<f32>, v3: Vector3<f32>) -> Vector3<f32> {
    (v1 - v2).cross(v2 - v3).normalize()
}

#[cfg(test)]
mod tests {
    use {super::*, cgmath::Zero, std::io::Cursor};

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
