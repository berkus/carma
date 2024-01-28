//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#![allow(dead_code)]

// extern crate genmesh;
// extern crate obj;

use {
    anyhow::{anyhow, Result},
    byteorder::{BigEndian, ReadBytesExt},
    cgmath::Vector3,
    glium::implement_vertex,
    std::{
        self,
        convert::From,
        io::BufRead,
        ops::Sub,
        path::{Path, PathBuf},
        thread,
        time::{Duration, Instant},
    },
    thiserror::Error as ThisError,
};

pub mod actor;
pub mod camera;
pub mod car;
pub mod material;
pub mod mesh;
pub mod render_manager;
pub mod resource;
pub mod texture;
// pub mod animated_parameter;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

impl Vertex {
    pub fn load<R: ReadBytesExt>(rdr: &mut R) -> Result<Vertex, Error> {
        let mut vertex = Vertex::default();
        vertex.position[0] = rdr.read_f32::<BigEndian>()?;
        vertex.position[1] = rdr.read_f32::<BigEndian>()?;
        vertex.position[2] = rdr.read_f32::<BigEndian>()?;
        Ok(vertex)
    }
}

// This is used only for vector math, using positions
// Not a general implementation - todo: replace with sub fun
impl Sub for Vertex {
    type Output = Vector3<f32>;

    fn sub(self, other: Vertex) -> Vector3<f32> {
        Vector3 {
            x: self.position[0] - other.position[0],
            y: self.position[1] - other.position[1],
            z: self.position[2] - other.position[2],
        }
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("i/o error {0:?}")]
    IO(#[from] std::io::Error),
    #[error("utf-8 conversion error {0:?}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("from utf-8 conversion error {0:?}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F)
where
    F: FnMut() -> Action,
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667); // 16ms for 60 FPS
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
    /*let num_bytes =*/
    reader.read_until(0, &mut buf)?;
    buf.pop();
    let s = String::from_utf8(buf)?; //@todo from_utf8_lossy
    Ok(s)
}

/*
 * Creates a pathname to filepath with the last directory replaced by newdir
 * and optionally changing extension to newext.
 */
pub fn path_subst<P: AsRef<Path>>(
    filepath: P,
    newdir: P,
    newext: Option<String>,
) -> Result<PathBuf> {
    let fname = filepath.as_ref().file_name();
    let mut dir = filepath.as_ref().to_path_buf();
    if let Some(_) = fname {
        dir.pop(); // remove file name
    }
    dir.pop(); // remove parent dir
    dir.push(newdir); // replace parent dir
    if let Some(fname) = fname {
        dir.push(fname); // add back file name
    }
    if let Some(ext) = newext {
        dir.set_extension(ext);
    }
    Ok(dir)
}

pub const NULL_CHUNK: u32 = 0x0;
pub const PIXELMAP_HEADER_CHUNK: u32 = 0x3;
pub const MATERIAL_DESC_CHUNK: u32 = 0x4;
pub const FILE_HEADER_CHUNK: u32 = 0x12;
pub const MATERIAL_LIST_CHUNK: u32 = 0x16;
pub const VERTEX_LIST_CHUNK: u32 = 0x17;
pub const UVMAP_LIST_CHUNK: u32 = 0x18;
pub const FACE_MAT_LIST_CHUNK: u32 = 0x1a;
pub const PIXELMAP_REF_CHUNK: u32 = 0x1c;
pub const RENDERTAB_REF_CHUNK: u32 = 0x1f;
pub const PIXELMAP_DATA_CHUNK: u32 = 0x21;
pub const ACTOR_NAME_CHUNK: u32 = 0x23;
pub const ACTOR_NODE_DOWN_CHUNK: u32 = 0x25;
pub const MESHFILE_REF_CHUNK: u32 = 0x24;
pub const MATERIAL_REF_CHUNK: u32 = 0x26;
pub const UNKNOWN_29_CHUNK: u32 = 0x29;
pub const ACTOR_NODE_UP_CHUNK: u32 = 0x2a;
pub const ACTOR_TRANSFORM_CHUNK: u32 = 0x2b;
pub const MAP_BOUNDINGBOX_CHUNK: u32 = 0x32;
pub const FACE_LIST_CHUNK: u32 = 0x35;
pub const FILE_NAME_CHUNK: u32 = 0x36;

pub const ACTOR_FILE_TYPE: u32 = 0x1;
pub const PIXELMAP_FILE_TYPE: u32 = 0x2;
pub const MATERIAL_FILE_TYPE: u32 = 0x5;
pub const MESH_FILE_TYPE: u32 = 0xface;

pub const MODEL_FILE_SUBTYPE: u16 = 0x3;

#[cfg(test)]
mod tests {

    use {
        super::*,
        byteorder::ReadBytesExt,
        std::io::{BufReader, Cursor},
    };

    #[test]
    fn test_read_c_string() {
        let data = Cursor::new(b"hello world\0abc");
        let mut reader = BufReader::new(data);

        let s = read_c_string(&mut reader).unwrap();
        let t = reader.read_u8().unwrap();
        let u = reader.read_u8().unwrap();
        let v = reader.read_u8().unwrap();
        assert_eq!("hello world", s);
        assert_eq!(b"a"[0], t);
        assert_eq!(b"b"[0], u);
        assert_eq!(b"c"[0], v);
    }

    #[test]
    fn test_path_subst() {
        assert_eq!(
            PathBuf::from("/path/file.ext2"),
            path_subst(
                &Path::new("/old/file.ext"),
                &Path::new("path"),
                Some(String::from("ext2")),
            )
        );
        assert_eq!(
            PathBuf::from("/path/file.ext"),
            path_subst(&Path::new("/old/file.ext"), &Path::new("path"), None)
        );
    }
}
