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
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// MAT file is an index of: material internal name, PIX file name and TAB file name.
// @todo: keep material properties and internal name, replace pix and tab with megatexture reference
#[derive(Default, Debug)]
pub struct Material {
    params: [f32; 12],
    pub name: String,
    pub pixelmap_name: String,
    rendertab_name: String, // Palette file used to convert u8 indexed color to RGBA
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} pixelmap {} rendertab {}",
            self.name, self.pixelmap_name, self.rendertab_name,
        )
    }
}

impl Material {
    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<Material, Error> {
        let mut mat = Material::default();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            let c = Chunk::load(rdr)?;
            match c {
                Chunk::MaterialDesc { name, params } => {
                    mat.params = params;
                    mat.name = name;
                }
                Chunk::PixelmapRef(name) => mat.pixelmap_name = name,
                Chunk::RenderTabRef(name) => mat.rendertab_name = name,
                Chunk::Null() => break,
                Chunk::FileHeader { file_type } => {
                    if file_type != support::MATERIAL_FILE_TYPE {
                        panic!("Invalid material file type {}", file_type);
                    }
                }
                _ => unimplemented!(), // unexpected type here
            }
        }

        Ok(mat)
    }

    /**
     * Load multiple materials from a file.
     */
    pub fn load_from(fname: String) -> Result<Vec<Material>, Error> {
        let file = File::open(fname)?;
        let mut file = BufReader::new(file);
        let mut materials = Vec::<Material>::new();
        loop {
            let mat = Material::load(&mut file);
            match mat {
                Err(_) => break, // fixme: allow only Eof here
                Ok(mat) => materials.push(mat),
            }
        }
        Ok(materials)
    }
}
