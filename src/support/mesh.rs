use byteorder::{BigEndian, ReadBytesExt};
use support::{read_c_string, Error};
use std::io::BufRead;

#[derive(Default)]
pub struct UvCoord {
    u: f32,
    v: f32,
}

impl UvCoord {
    pub fn load<R: ReadBytesExt>(rdr: &mut R) -> Result<UvCoord, Error> {
        let mut uv = UvCoord::default();
        uv.u = rdr.read_f32::<BigEndian>()?;
        uv.v = rdr.read_f32::<BigEndian>()?;
        Ok(uv)
    }
}

#[derive(Default)]
pub struct Face {
    v1: u16, // vertex indices (works with glDrawElements() e.g.)
    v2: u16,
    v3: u16,
    flags: u16, // looks like flags, always only one bit set -- not always, see CITYA81.DAT!!
    material_id: u16, // synthetic field
}

impl Face {
    pub fn load<R: ReadBytesExt>(rdr: &mut R) -> Result<Face, Error> {
        let mut s = Face::default();
        s.v1 = rdr.read_u16::<BigEndian>()?;
        s.v2 = rdr.read_u16::<BigEndian>()?;
        s.v3 = rdr.read_u16::<BigEndian>()?;
        s.flags = rdr.read_u16::<BigEndian>()?;
        rdr.read_i8()?; // something, no idea yet, might be related to flags
        Ok(s)
    }
}

#[derive(Default)]
pub struct Mesh {
    name: String,
    // vertices: Vec<Vertex>,
    // normals: Vec<Vertex>, // calculated normals for each vertex
    uvcoords: Vec<UvCoord>,
    faces: Vec<Face>,
    material_names: Vec<String>,
}

impl Mesh {
    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<Mesh, Error> {
        let mut m = Mesh::default();
        m.name = read_c_string(rdr)?;
        m.calc_normals();
        Ok(m)
    }

    pub fn calc_normals(&mut self) {

    }
}

#[cfg(test)]
mod tests {

use std::io::Cursor;
use super::*;

#[test]
fn load_face()
{
    let mut data = Cursor::new(vec![0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0]);
    let f = Face::load(&mut data).unwrap();
    assert_eq!(0xdead, f.v1);
    assert_eq!(0xbeef, f.v2);
    assert_eq!(0xcafe, f.v3);
    assert_eq!(0xbabe, f.flags);
}

#[test]
fn load_mesh()
{
    let mut data = Cursor::new(vec![b'h', b'e', b'l', b'l', b'o', 0, 0xba, 0xbe, 0, 2, 5]);
    let m = Mesh::load(&mut data).unwrap();
    assert_eq!("hello", m.name);
    // assert_eq!(0xbeef, m.v2);
    // assert_eq!(0xcafe, m.v3);
    // assert_eq!(0xbabe, m.flags);
}

} // tests mod
