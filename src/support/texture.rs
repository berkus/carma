//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use support::Error;
use std::io::{BufRead, BufReader};
use byteorder::ReadBytesExt;
use support::resource::Chunk;
use std::fs::File;
use std::fmt;
use support;

// Pixmap consists of two chunks: name and data
// TODO: use shared_data_t for pixmap contents to avoid copying.
#[derive(Default, Clone)]
pub struct PixelMap {
    pub name: String,
    pub w: u16, // Actual texture w & h
    pub h: u16,
    use_w: u16, // and how much of that is used for useful data
    use_h: u16,
    units: u32,
    unit_bytes: u32,
    pub data: Vec<u8>, // temp pub
}

impl fmt::Display for PixelMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}x{}, use {}x{}) {} units {} bytes each",
            self.name,
            self.w,
            self.h,
            self.use_w,
            self.use_h,
            self.units,
            self.unit_bytes
        )
    }
}

#[derive(Default)]
pub struct Texture {
    // A pixelmap binding for opengl
}

impl PixelMap {
    /// Convert indexed-color image to RGBA using provided palette.
    pub fn remap_via(&self, palette: &PixelMap) -> Result<PixelMap, Error> {
        let mut pm = self.clone();
        pm.data = Vec::<u8>::with_capacity(self.data.len() * 4);
        pm.unit_bytes = 4;

        for i in 0..pm.units {
            pm.data.push(
                palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 1) as usize],
            ); // R
            pm.data.push(
                palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 2) as usize],
            ); // G
            pm.data.push(
                palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 3) as usize],
            ); // B
            pm.data.push(
                palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 0) as usize],
            ); // A
        }

        Ok(pm)
    }

    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<PixelMap, Error> {
        let mut pm = PixelMap::default();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            let c = Chunk::load(rdr)?;
            match c {
                Chunk::PixelmapHeader {
                    name,
                    w,
                    h,
                    use_w,
                    use_h,
                } => {
                    pm.name = name;
                    pm.w = w;
                    pm.h = h;
                    pm.use_w = use_w;
                    pm.use_h = use_h;
                    println!("Pixelmap {}x{} use {}x{}", w, h, use_w, use_h);
                }
                Chunk::PixelmapData {
                    units,
                    unit_bytes,
                    data,
                } => {
                    pm.units = units;
                    pm.unit_bytes = unit_bytes;
                    pm.data = data;
                    println!("Pixelmap data {} units, {} bytes each", units, unit_bytes);
                }
                Chunk::Null() => break,
                Chunk::FileHeader { file_type } => if file_type != support::PIXELMAP_FILE_TYPE {
                    panic!("Invalid pixelmap file type {}", file_type);
                },
                _ => unimplemented!(), // unexpected type here
            }
        }

        Ok(pm)
    }

    pub fn load_from(fname: String) -> Result<Vec<PixelMap>, Error> {
        let file = File::open(fname)?;
        let mut file = BufReader::new(file);
        let mut pmaps = Vec::<PixelMap>::new();
        loop {
            let pmap = PixelMap::load(&mut file);
            match pmap {
                Err(_) => break, // fixme: allow only Eof here
                Ok(pmap) => pmaps.push(pmap),
            }
        }
        Ok(pmaps)
    }
}

impl Texture {
    pub fn load() -> Texture {
        Texture {}
    }
}
