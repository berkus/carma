//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use byteorder::ReadBytesExt;
use crate::support::{self, resource::Chunk, Error};
use log::*;
use png::{self, HasParameters};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

// Pixmap consists of two chunks: name and data
// TODO: use shared_data_t for pixmap contents to avoid copying.
#[derive(Default, Clone)]
pub struct PixelMap {
    pub name: String,
    pub w: u16, // Actual texture w & h
    pub h: u16,
    use_w: u16, // and how much of that is used for useful data
    use_h: u16,
    pub units: u32,
    pub unit_bytes: u32,
    pub data: Vec<u8>, // temp pub
}

impl std::fmt::Display for PixelMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({}x{}, use {}x{}) {} units {} bytes each",
            self.name, self.w, self.h, self.use_w, self.use_h, self.units, self.unit_bytes
        )
    }
}

/**
 * Megatexture for storing all loaded textures.
 * Usually 1024x1024 or 4096x4096 texture with multiple smaller textures inside.
 */
#[derive(Default)]
pub struct Texture {
    pub w: u16,
    pub h: u16,
    pub data: Vec<u8>,
}

/**
 * Named reference into the megatexture.
 */
pub struct TextureReference {
    pub id: i32,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub name: String,
}

impl PixelMap {
    /// Convert indexed-color image to RGBA using provided palette.
    pub fn remap_via(&self, palette: &PixelMap) -> Result<PixelMap, Error> {
        let mut pm = self.clone();
        pm.data = Vec::<u8>::with_capacity(self.data.len() * 4);
        pm.unit_bytes = 4;

        for i in 0..pm.units {
            // temp use color index 0 as transparency
            if self.data[i as usize] == 0 {
                pm.data.push(0); // R
                pm.data.push(0); // G
                pm.data.push(0); // B
                pm.data.push(255); // A = transparent
            } else {
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
                    255 - palette.data
                        [(self.data[i as usize] as u32 * palette.unit_bytes + 0) as usize],
                ); // A
                if self.name == "BGLSPIKE.PIX" {
                    trace!("spike alpha {}", pm.data.last().unwrap());
                }
            }
        }

        Ok(pm)
    }

    pub fn write_png_remapped_via<W: Write>(
        &self,
        palette: &PixelMap,
        w: &mut W,
    ) -> Result<(), Error> {
        self.dump();

        let mut encoder = png::Encoder::new(w, self.w as u32, self.h as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let mut data = Vec::<u8>::with_capacity(self.data.len() * 4);

        match self.unit_bytes {
            1 => {
                for i in 0..self.units {
                    data.push(
                        palette.data
                            [(self.data[i as usize] as u32 * palette.unit_bytes + 1) as usize],
                    ); // R
                    data.push(
                        palette.data
                            [(self.data[i as usize] as u32 * palette.unit_bytes + 2) as usize],
                    ); // G
                    data.push(
                        palette.data
                            [(self.data[i as usize] as u32 * palette.unit_bytes + 3) as usize],
                    ); // B
                       // data.push(
                       // 255-palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 0) as
                       // usize],
                       // ); // A
                }
            }
            3 => {
                for i in 0..self.units {
                    data.push(self.data[(i * self.unit_bytes + 0) as usize]); // R
                    data.push(self.data[(i * self.unit_bytes + 1) as usize]); // G
                    data.push(self.data[(i * self.unit_bytes + 2) as usize]); // B
                                                                              // data.push(255); // A
                }
            }
            4 => {
                for i in 0..self.units {
                    data.push(self.data[(i * self.unit_bytes + 0) as usize]); // R
                    data.push(self.data[(i * self.unit_bytes + 1) as usize]); // G
                    data.push(self.data[(i * self.unit_bytes + 2) as usize]); // B
                                                                              // data.push(self.data[(i * self.unit_bytes + 3) as usize]); // A
                }
            }
            _ => unimplemented!(),
        }

        writer.write_image_data(&data).unwrap();
        Ok(())
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
                    mipmap_w,
                    mipmap_h,
                } => {
                    pm.name = name;
                    pm.w = w;
                    pm.h = h;
                    pm.use_w = mipmap_w;
                    pm.use_h = mipmap_h;
                    debug!("Pixelmap {}x{} use {}x{}", w, h, mipmap_w, mipmap_h);
                }
                Chunk::PixelmapData {
                    units,
                    unit_bytes,
                    data,
                } => {
                    pm.units = units;
                    pm.unit_bytes = unit_bytes;
                    pm.data = data;
                    debug!("Pixelmap data {} units, {} bytes each", units, unit_bytes);
                }
                Chunk::Null() => break,
                Chunk::FileHeader { file_type } => {
                    if file_type != support::PIXELMAP_FILE_TYPE {
                        panic!("Invalid pixelmap file type {}", file_type);
                    }
                }
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

    fn dump(&self) {
        info!(
            "Pixelmap {}: {}x{}, mm {}x{}, {}x{} bytes",
            self.name, self.w, self.h, self.use_w, self.use_h, self.units, self.unit_bytes
        );
    }
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            w: 1024,
            h: 1024,
            data: Vec::new(),
        }
    }
}
