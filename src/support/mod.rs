#![allow(dead_code)]

// extern crate genmesh;
// extern crate obj;

use std;
use std::thread;
use std::time::{Duration, Instant};
use std::io::BufRead;
use std::convert::From;
// use glium::{self, Display};
// use glium::vertex::VertexBufferAny;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

pub mod camera;
pub mod material;
pub mod mesh;
pub mod model;
pub mod texture;
pub mod resource;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Utf8(std::str::Utf8Error),
    FromUtf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Utf8(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8(error)
    }
}

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

// Load a C-style 0-terminated string from the file and return it
pub fn read_c_string<R: BufRead>(reader: &mut R) -> Result<String, Error> {
    let mut buf = vec![];
    /*let num_bytes =*/ reader.read_until(0, &mut buf)?;
    buf.pop();
    let s = String::from_utf8(buf)?;
    Ok(s)
}

// Returns a vertex buffer that should be rendered as `TrianglesList`.
// pub fn load_wavefront(display: &Display, data: &[u8]) -> VertexBufferAny {
//     #[derive(Copy, Clone)]
//     struct Vertex {
//         position: [f32; 3],
//         normal: [f32; 3],
//         texture: [f32; 2],
//     }

//     implement_vertex!(Vertex, position, normal, texture);

//     let mut data = ::std::io::BufReader::new(data);
//     let data = obj::Obj::load(&mut data);

//     let mut vertex_data = Vec::new();

//     for object in data.object_iter() {
//         for shape in object.group_iter().flat_map(|g| g.indices().iter()) {
//             match shape {
//                 &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
//                     for v in [v1, v2, v3].iter() {
//                         let position = data.position()[v.0];
//                         let texture = v.1.map(|index| data.texture()[index]);
//                         let normal = v.2.map(|index| data.normal()[index]);

//                         let texture = texture.unwrap_or([0.0, 0.0]);
//                         let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

//                         vertex_data.push(Vertex {
//                             position: position,
//                             normal: normal,
//                             texture: texture,
//                         })
//                     }
//                 },
//                 _ => unimplemented!()
//             }
//         }
//     }

//     glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into_vertex_buffer_any()
// }

#[cfg(test)]
mod tests {

use std::io::{Cursor, BufReader};
use byteorder::ReadBytesExt;
use super::*;

#[test]
fn test_read_c_string()
{
    let data = Cursor::new(b"hello world\0abc");
    let mut reader = BufReader::new(data);

    let s = read_c_string(&mut reader).unwrap();
    let t = reader.read_u8().unwrap();
    println!("{:?}", t);
    let u = reader.read_u8().unwrap();
    println!("{:?}", u);
    let v = reader.read_u8().unwrap();
    println!("{:?}", v);
    assert_eq!("hello world", s);
    assert_eq!(b"a"[0], t);
    assert_eq!(b"b"[0], u);
    assert_eq!(b"c"[0], v);
}

}
