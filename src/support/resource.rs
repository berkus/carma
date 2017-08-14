use byteorder::{BigEndian, ReadBytesExt};
use std::io::BufRead;
use support::{read_c_string, Vertex, Error};
use support::mesh::{Face, UvCoord};
use support;

// A binary resource file consisting of chunks with specific size.
// Reading from such file yields chunk results, some of these chunks are service,
// some are useful to the client.
#[derive(Default)]
struct ChunkHeader
{
    chunk_type: u32,
    size: u32, // size of chunk -4
}

impl ChunkHeader {
    pub fn load<R: ReadBytesExt>(rdr: &mut R) -> Result<ChunkHeader, Error> {
        let mut h = ChunkHeader::default();
        h.chunk_type = rdr.read_u32::<BigEndian>()?;
        h.size = rdr.read_u32::<BigEndian>()?;
        Ok(h)
    }
}

pub enum Chunk {
    FileName(String),
    VertexList(Vec<Vertex>),
    UvMapList(Vec<UvCoord>),
    FaceList(Vec<Face>),
    MaterialList(Vec<String>),
    FaceMatList(Vec<u16>),
    Null(),
    MoarChunks(),
}

impl Chunk {
    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<Chunk, Error> {
        let header = ChunkHeader::load(rdr)?;
        match header.chunk_type {
            support::FILE_NAME_CHUNK => {
                println!("Reading filename entry...");
                let s = read_c_string(rdr)?;
                Ok(Chunk::FileName(s))
            },
            support::VERTEX_LIST_CHUNK => {
                println!("Reading vertex list...");
                let n = rdr.read_u32::<BigEndian>()?;
                let mut r = Vec::<Vertex>::with_capacity(n as usize);
                for _ in 0 .. n {
                    let v = Vertex::load(rdr)?;
                    r.push(v);
                }
                Ok(Chunk::VertexList(r))
            },
            support::UVMAP_LIST_CHUNK => {
                println!("Reading uvmap list...");
                let n = rdr.read_u32::<BigEndian>()?;
                let mut r = Vec::<UvCoord>::with_capacity(n as usize);
                for _ in 0 .. n {
                    let v = UvCoord::load(rdr)?;
                    r.push(v);
                }
                Ok(Chunk::UvMapList(r))
            },
            support::FACE_LIST_CHUNK => {
                println!("Reading face list...");
                let n = rdr.read_u32::<BigEndian>()?;
                let mut r = Vec::<Face>::with_capacity(n as usize);
                for _ in 0 .. n {
                    let v = Face::load(rdr)?;
                    r.push(v);
                }
                Ok(Chunk::FaceList(r))
            },
            support::MATERIAL_LIST_CHUNK => {
                println!("Reading material list...");
                let n = rdr.read_u32::<BigEndian>()?;
                let mut r = Vec::<String>::with_capacity(n as usize);
                for _ in 0 .. n {
                    let v = read_c_string(rdr)?;
                    r.push(v);
                }
                Ok(Chunk::MaterialList(r))
            },
            support::FACE_MAT_LIST_CHUNK => { // faces.materials = list,
                println!("Reading face material list...");
                let n = rdr.read_u32::<BigEndian>()?;

                /*let dummy =*/ rdr.read_u32::<BigEndian>()?;

                let mut r = Vec::<u16>::with_capacity(n as usize);
                for _ in 0 .. n {
                    let v = rdr.read_u16::<BigEndian>()?;
                    r.push(v);
                }
                Ok(Chunk::FaceMatList(r))
            },
            support::NULL_CHUNK => {
                Ok(Chunk::Null())
            },
            _ => unimplemented!(), // should not appear here, but in general chunk reader is possible
        }
    }
}

// #[derive(Default)]
// struct Chunk
// {
//     header: ChunkHeader,
//     num_entries: u32, // number of entires -- only in DAT, not part of chunk header actually (different for other types)
// }

// impl Chunk {
//     pub fn load<R: ReadBytesExt>(rdr: &mut R) -> Chunk {
//         let mut h = Chunk::default();
//         h.header = ChunkHeader::load(rdr);
//         h.num_entries = rdr.read_u32::<BigEndian>().unwrap();
//         h
//     }
// }

