//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use support::Error;
use std::io::BufRead;
use byteorder::ReadBytesExt;
use support::resource::Chunk;

// Pixmap consists of two chunks: name and data
// TODO: use shared_data_t for pixmap contents to avoid copying.
#[derive(Default)]
pub struct PixelMap
{
    name: String,
    w: u16, // Actual texture w & h
    h: u16,
    use_w: u16, // and how much of that is used for useful data
    use_h: u16,
    units: u32,
    unit_bytes: u32,
    data: Vec<u8>,
}

#[derive(Default)]
pub struct Texture {

}

impl PixelMap {
    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<PixelMap, Error> {
        let mut pm = PixelMap::default();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            let c = Chunk::load(rdr)?;
            match c {
                Chunk::PixelmapHeader { name, w, h, use_w, use_h } => {
                    pm.name = name;
                    pm.w = w;
                    pm.h = h;
                    pm.use_w = use_w;
                    pm.use_h = use_h;
                },
                Chunk::PixelmapData { units, unit_bytes, data } => {
                    pm.units = units;
                    pm.unit_bytes = unit_bytes;
                    pm.data = data;
                },
                Chunk::Null() => break,
                _ => unimplemented!(), // unexpected type here
            }
        }

        Ok(pm)
    }
}

impl Texture {
    pub fn load() -> Texture {
        Texture {}
    }
}
