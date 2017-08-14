use byteorder::{BigEndian, ReadBytesExt};
use support::Error;

// A binary resource file consisting of chunks with specific size.
// Reading from such file yields chunk results, some of these chunks are service,
// some are useful to the client.
#[derive(Default)]
pub struct ChunkedBinaryResource
{

}

impl ChunkedBinaryResource {
    // Read chunks
    // Certain chunks initialize certain properties
    // match (ReadChunk) {
    //     FILE_NAME_CHUNK: name = name,
    //     VERTEX_LIST: vertices = list,
    //     UVMAP_LIST: tex_coords = list,
    //     FACE_LIST: faces = list,
    //     MATERIAL_LIST: materials = list,
    //     FACE_MAT_LIST: faces.materials = list,
    //     NULL_CHUNK: done,
    // }
}

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

#[derive(Default)]
struct Chunk
{
    header: ChunkHeader,
    num_entries: u32, // number of entires -- only in DAT, not part of chunk header actually (different for other types)
}

impl Chunk {
    pub fn load<R: ReadBytesExt>(rdr: &mut R) -> Chunk {
        let mut h = Chunk::default();
        h.header = ChunkHeader::load(rdr);
        h.num_entries = rdr.read_u32::<BigEndian>().unwrap();
        h
    }
}

