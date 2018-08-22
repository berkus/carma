//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use byteorder::{BigEndian, ReadBytesExt};
use crate::support::{
    self,
    mesh::{Face, UvCoord},
    read_c_string, Error, Vertex,
};
use log::*;
use std::io::BufRead;

// A binary resource file consisting of chunks with specific size.
// Reading from such file yields chunk results, some of these chunks are service,
// some are useful to the client.
#[derive(Default)]
struct ChunkHeader {
    chunk_type: u32,
    size: u32, // size of chunk -4
}

impl ChunkHeader {
    pub fn load<R: ReadBytesExt>(source: &mut R) -> Result<ChunkHeader, Error> {
        let mut h = ChunkHeader::default();
        h.chunk_type = source.read_u32::<BigEndian>()?;
        h.size = source.read_u32::<BigEndian>()?;
        debug!("Loaded chunk type {} size {}", h.chunk_type, h.size);
        Ok(h)
    }
}

pub enum Chunk {
    Null(),
    FileHeader {
        file_type: u32,
    },
    FileName {
        name: String,
        subtype: u16,
    },
    VertexList(Vec<Vertex>),
    UvMapList(Vec<UvCoord>),
    FaceList(Vec<Face>),
    MaterialList(Vec<String>),
    MaterialDesc {
        name: String,
        params: [f32; 12], // a matrix?
    },
    FaceMatList(Vec<u16>),
    PixelmapHeader {
        name: String,
        w: u16,
        h: u16,
        mipmap_w: u16,
        mipmap_h: u16,
    },
    PixelmapData {
        units: u32,
        unit_bytes: u32,
        data: Vec<u8>,
    },
    PixelmapRef(String),
    RenderTabRef(String),
    MeshFileRef(String),
    MaterialRef(String),
    ActorName {
        name: String,
        visible: bool,
    },
    ActorTransform([f32; 12]),
    MapBoundingBox(),
    ActorNodeDown(),
    Unknown29(),
    ActorNodeUp(),
}

impl Chunk {
    pub fn load<R: ReadBytesExt + BufRead>(source: &mut R) -> Result<Chunk, Error> {
        let header = ChunkHeader::load(source)?;
        match header.chunk_type {
            support::NULL_CHUNK => Ok(Chunk::Null()),
            support::FILE_HEADER_CHUNK => {
                trace!("Reading file header...");
                assert_eq!(header.size, 8);
                let file_type = source.read_u32::<BigEndian>()?;
                source.read_u32::<BigEndian>()?; // dummy?
                Ok(Chunk::FileHeader { file_type })
            }
            support::FILE_NAME_CHUNK => {
                let subtype = source.read_u16::<BigEndian>()?;
                trace!("Reading filename entry... (subtype {})", subtype);
                let name = read_c_string(source)?;
                trace!("... {}", name);
                Ok(Chunk::FileName { name, subtype })
            }
            support::VERTEX_LIST_CHUNK => {
                trace!("Reading vertex list...");
                let n = source.read_u32::<BigEndian>()?;
                let mut r = Vec::<Vertex>::with_capacity(n as usize);
                for _ in 0..n {
                    let v = Vertex::load(source)?;
                    r.push(v);
                }
                Ok(Chunk::VertexList(r))
            }
            support::UVMAP_LIST_CHUNK => {
                trace!("Reading uvmap list...");
                let n = source.read_u32::<BigEndian>()?;
                let mut r = Vec::<UvCoord>::with_capacity(n as usize);
                for _ in 0..n {
                    let v = UvCoord::load(source)?;
                    r.push(v);
                }
                Ok(Chunk::UvMapList(r))
            }
            support::FACE_LIST_CHUNK => {
                trace!("Reading face list...");
                let n = source.read_u32::<BigEndian>()?;
                let mut r = Vec::<Face>::with_capacity(n as usize);
                for _ in 0..n {
                    let v = Face::load(source)?;
                    r.push(v);
                }
                Ok(Chunk::FaceList(r))
            }
            support::MATERIAL_LIST_CHUNK => {
                trace!("Reading material list...");
                let n = source.read_u32::<BigEndian>()?;
                let mut r = Vec::<String>::with_capacity(n as usize);
                for _ in 0..n {
                    let v = read_c_string(source)?;
                    trace!("... {}", v);
                    r.push(v);
                }
                Ok(Chunk::MaterialList(r))
            }
            support::MATERIAL_DESC_CHUNK => {
                trace!("Reading material descriptor...");
                let mut params = [0f32; 12];
                for i in 0..12 {
                    params[i] = source.read_f32::<BigEndian>()?;
                }
                let name = read_c_string(source)?;
                trace!("... {}", name);
                Ok(Chunk::MaterialDesc { params, name })
            }
            support::FACE_MAT_LIST_CHUNK => {
                trace!("Reading face material list...");
                let n = source.read_u32::<BigEndian>()?;

                /*let dummy =*/
                source.read_u32::<BigEndian>()?;

                let mut r = Vec::<u16>::with_capacity(n as usize);
                for _ in 0..n {
                    let v = source.read_u16::<BigEndian>()?;
                    r.push(v);
                }
                Ok(Chunk::FaceMatList(r))
            }
            support::PIXELMAP_HEADER_CHUNK => {
                trace!("Reading pixelmap header...");
                let what1 = source.read_u8()?; // what1 -- somethiing
                let what2 = source.read_u16::<BigEndian>()?; // what2 -- somethiing
                let w = source.read_u16::<BigEndian>()?;
                let h = source.read_u16::<BigEndian>()?;
                let mipmap_w = source.read_u16::<BigEndian>()?;
                let mipmap_h = source.read_u16::<BigEndian>()?;
                let name = read_c_string(source)?;
                trace!(
                    "... {}, {}x{}, {}x{}, what1 {}, what2 {}",
                    name,
                    w,
                    h,
                    mipmap_w,
                    mipmap_h,
                    what1,
                    what2
                );
                Ok(Chunk::PixelmapHeader {
                    name,
                    w,
                    h,
                    mipmap_w,
                    mipmap_h,
                })
            }
            support::PIXELMAP_DATA_CHUNK => {
                trace!("Reading pixelmap data...");

                let units = source.read_u32::<BigEndian>()?;
                let unit_bytes = source.read_u32::<BigEndian>()?;

                let payload_size = (units * unit_bytes) as usize;

                let mut data = vec![0u8; payload_size];

                source.read_exact(&mut data)?;

                Ok(Chunk::PixelmapData {
                    units,
                    unit_bytes,
                    data,
                })
            }
            support::PIXELMAP_REF_CHUNK => {
                trace!("Reading pixelmap ref...");
                let pixelmap_name = read_c_string(source)?;
                trace!("... {}", pixelmap_name);
                Ok(Chunk::PixelmapRef(pixelmap_name))
            }
            support::RENDERTAB_REF_CHUNK => {
                trace!("Reading rendertab ref...");
                let rendertab_name = read_c_string(source)?;
                trace!("... {}", rendertab_name);
                Ok(Chunk::RenderTabRef(rendertab_name))
            }
            support::ACTOR_NAME_CHUNK => {
                trace!("Reading actor name...");
                let visible = source.read_u8()? == 0x1;
                source.read_u8()?; // what2
                let name = read_c_string(source)?;
                trace!("... {}", name);
                Ok(Chunk::ActorName { name, visible })
            }
            support::ACTOR_NODE_DOWN_CHUNK => {
                trace!("Reading actor node down...");
                Ok(Chunk::ActorNodeDown())
            }
            support::UNKNOWN_29_CHUNK => {
                trace!("Reading unknown 29...");
                Ok(Chunk::Unknown29())
            }
            support::ACTOR_NODE_UP_CHUNK => {
                trace!("Reading actor node up...");
                Ok(Chunk::ActorNodeUp())
            }
            support::MESHFILE_REF_CHUNK => {
                trace!("Reading meshfile ref...");
                let mesh_name = read_c_string(source)?;
                trace!("... {}", mesh_name);
                Ok(Chunk::MeshFileRef(mesh_name))
            }
            support::MATERIAL_REF_CHUNK => {
                trace!("Reading material ref...");
                let material_name = read_c_string(source)?;
                trace!("... {}", material_name);
                Ok(Chunk::MaterialRef(material_name))
            }
            support::ACTOR_TRANSFORM_CHUNK => {
                trace!("Reading actor transform...");
                let mut params = [0f32; 12];
                for i in 0..12 {
                    params[i] = source.read_f32::<BigEndian>()?;
                }
                for row in 0..4 {
                    trace!(
                        "[{} {} {}]",
                        params[row * 3 + 0],
                        params[row * 3 + 1],
                        params[row * 3 + 2]
                    );
                }
                // CHECK_READ(v.read(f));
                // actor->scale.x[0][0] = v.x;
                // actor->scale.x[1][0] = v.y;
                // actor->scale.x[2][0] = v.z;
                // CHECK_READ(v.read(f));
                // actor->scale.x[0][1] = v.x;
                // actor->scale.x[1][1] = v.y;
                // actor->scale.x[2][1] = v.z;
                // CHECK_READ(v.read(f));
                // actor->scale.x[0][2] = v.x;
                // actor->scale.x[1][2] = v.y;
                // actor->scale.x[2][2] = v.z;
                // CHECK_READ(actor->translate.read(f));
                Ok(Chunk::ActorTransform(params))
            }
            support::MAP_BOUNDINGBOX_CHUNK => {
                trace!("Reading map bounding box (or?)...");
                Ok(Chunk::MapBoundingBox())
            }
            _ => unimplemented!(), // should not appear here, in general chunk reader is possible
        }
    }
}
