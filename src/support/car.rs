//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use cgmath::Vector3;
use crate::support::{
    actor::{Actor, ActorNode},
    material::Material,
    mesh::Mesh,
    path_subst,
    texture::PixelMap,
    Error,
};
use log::*;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    path::{Path, PathBuf},
};

// Car assembles the gameplay object (a car in this case) from various model and texture files.
pub struct Car {
    pub name: String,
    pub actors: Actor,
    pub meshes: HashMap<String, Mesh>,
    pub materials: HashMap<String, Material>,
    pub textures: HashMap<String, PixelMap>,
    pub base_translation: Vector3<f32>,
}

/// Expect next line to match provided text exactly.
fn expect_match<Iter: Iterator<Item = String>>(input: &mut Iter, text: &str) {
    if let Some(line) = input.next() {
        if line == text {
            return;
        }
        panic!("Expected {:?} but got {:?}", text, line);
    }
    panic!("Expected {:?} but got empty line", text);
}

/// Parse a three-component vector from a comma-separated string.
fn parse_vector(line: &String) -> Vector3<f32> {
    let line: Vec<f32> = line.split(',').map(|i| i.trim().parse().unwrap()).collect();
    Vector3::from((line[0], line[1], line[2]))
}

/// Read systems in a single damage spec clause.
fn read_systems<Iter: Iterator<Item = String>>(input: &mut Iter) {
    // read condition flag for this clause
    /*let condition =*/    input.next().unwrap();
    // read systems count, read this many systems
    let systems_count = input.next().unwrap().parse().unwrap();
    for _ in 0..systems_count {
        input.next();
    }
}

/// Read all damage spec clauses.
fn read_clauses<Iter: Iterator<Item = String>>(input: &mut Iter) {
    // read clause count, read this many systems
    let clause_count = input.next().unwrap().parse().unwrap();
    for _ in 0..clause_count {
        read_systems(input);
    }
}

/// Read a vector of strings.
fn read_vector<Iter: Iterator<Item = String>>(input: &mut Iter) -> Vec<String> {
    // read vector size, read this many strings
    let size = input.next().unwrap().parse().unwrap();
    let mut vec = Vec::<String>::with_capacity(size);
    for _ in 0..size {
        vec.push(input.next().unwrap());
    }
    vec
}

fn read_funk<Iter: Iterator<Item = String>>(input: &mut Iter) {
    expect_match(input, "START OF FUNK");
    // for now just ignore everything here, read until END OF FUNK
    loop {
        // @todo read funk loop with NEXT FUNK as trigger
        // read_funk();
        // NEXT FUNK
        let line = input.next().unwrap();
        if line == "END OF FUNK" {
            return;
        }
    }
}

struct Groove {}

// Read a single groove
// fn read_groove<Iter: Iterator<Item=String>>(input: &mut Iter) -> Groove {
// }

fn read_grooves<Iter: Iterator<Item = String>>(input: &mut Iter) {
    expect_match(input, "START OF GROOVE");
    // for now just ignore everything here, read until END OF GROOVE
    loop {
        // @todo read groove loop with NEXT GROOVE as trigger
        // read_groove();
        // NEXT GROOVE
        let line = input.next().unwrap();
        if line == "END OF GROOVE" {
            return;
        }
    }
}

/// A bunch of some matrices and mappings or vertex-pairs, ignore for now.
fn read_some_metadata<Iter: Iterator<Item = String>>(input: &mut Iter) {
    input.next(); // 0.700000
    input.next(); // 0.050000,0.300000
    input.next(); // 0.050000
    input.next(); // 0.050000
    input.next(); // 0.000000
    input.next(); // 0.000000
    let size = input.next().unwrap().parse().unwrap();
    for _ in 0..size {
        input.next(); // 11
        input.next(); // -0.107444, -0.080211, 0.106640
        input.next(); // -0.057444, 0.054463, 0.206640
        input.next(); // 0.038245, 0.352418, 0.220975
        input.next(); // 0.111755, 0.051602, 0.079025
        let pair_count = input.next().unwrap().parse().unwrap();
        for _ in 0..pair_count {
            input.next();
            input.next();
        }
    }
}

// @fixme used to patch actors now
// @todo should support extra wheels
pub struct Mechanics {
    pub lrwheel_pos: Vector3<f32>,
    pub rrwheel_pos: Vector3<f32>,
    pub lfwheel_pos: Vector3<f32>,
    pub rfwheel_pos: Vector3<f32>,
}

fn read_mechanics_block_v1_1<Iter: Iterator<Item = String>>(
    input: &mut Iter,
) -> Result<Mechanics, Error> {
    let lrwheel_pos = parse_vector(&input.next().unwrap());
    trace!("Left rear wheel position: {:?}", lrwheel_pos);

    let rrwheel_pos = parse_vector(&input.next().unwrap());
    trace!("Right rear wheel position: {:?}", rrwheel_pos);

    let lfwheel_pos = parse_vector(&input.next().unwrap());
    trace!("Left front wheel position: {:?}", lfwheel_pos);

    let rfwheel_pos = parse_vector(&input.next().unwrap());
    trace!("Right front wheel position: {:?}", rfwheel_pos);

    let centre_of_mass_pos = parse_vector(&input.next().unwrap());
    trace!("Centre of mass position: {:?}", centre_of_mass_pos);

    Ok(Mechanics {
        lrwheel_pos,
        rrwheel_pos,
        lfwheel_pos,
        rfwheel_pos,
    })
}

fn read_mechanics_block_v1_1_v3<Iter: Iterator<Item = String>>(input: &mut Iter) {
    let min_bb = parse_vector(&input.next().unwrap());
    let max_bb = parse_vector(&input.next().unwrap());
    trace!("Bounding box: ({:?} - {:?})", min_bb, max_bb);
}

// Version 2 contains count for bounding boxes (which is always 1, that's why it's removed in ver 3)
fn read_mechanics_block_v1_1_v2<Iter: Iterator<Item = String>>(input: &mut Iter) {
    expect_match(input, "1");
    read_mechanics_block_v1_1_v3(input);
}

fn read_mechanics_block_v1_2<Iter: Iterator<Item = String>>(input: &mut Iter) {
    // 0.5                                     // min turning circle radius
    input.next();
    // 0.025,  0.025                           // suspension give (forward, back)
    input.next();
    // 0.090                  // ride height (must be more than miny in bounding box )
    input.next();
    // 0.5                                     // damping factor
    input.next();
    // 1.5                                     // mass in tonnes
    input.next();
    // 1                                       // fractional reduction in friction when slipping
    input.next();
    // 79, 80                                  // friction angle ( front and rear )
    input.next();
    // 0.4,    0.2,    0.816 // width, height, length(0.816, 1.216) for angular momentum calculation
    input.next();
}

fn read_mechanics_block_v1_3<Iter: Iterator<Item = String>>(input: &mut Iter) {
    // 0.05, 0.05                              // rolling resistance front and back
    input.next();
    // 6                                       // number of gears
    input.next();
    // 200                                     // speed at red line in highest gear
    input.next();
    // 4                           // acceleration in highest gear m/s^2 (i.e. engine strength)
    input.next();
}

fn read_mechanics_block_v2<Iter: Iterator<Item = String>>(input: &mut Iter) {
    // 2.0                                     // traction fractional multiplier v. 2
    input.next();
    // 50                                      // speed at which down force = weight v. 2
    input.next();
    // 1.0                                     // brake multiplier, 1 = nomral brakes v. 2
    input.next();
    // 1.0                                     // increase in brakes per second 1 = normal v. 2
    input.next();
}

fn read_mechanics_block_v3<Iter: Iterator<Item = String>>(input: &mut Iter) {
    // 3
    // 0,-0.18,-0.52                               // extra point 1            v. 3
    // -0.07,0.07,0.18                         // extra point 2            v. 3
    // 0.07,0.07,0.18                          // extra point 3            v. 3
    read_vector(input);
}

fn read_mechanics_v2<Iter: Iterator<Item = String>>(input: &mut Iter) -> Result<Mechanics, Error> {
    let mech = read_mechanics_block_v1_1(input)?;
    read_mechanics_block_v1_1_v2(input);
    read_mechanics_block_v1_2(input);
    read_mechanics_block_v2(input);
    read_mechanics_block_v1_3(input);
    Ok(mech)
}

fn read_mechanics_v3<Iter: Iterator<Item = String>>(input: &mut Iter) -> Result<Mechanics, Error> {
    let mech = read_mechanics_block_v1_1(input)?;
    read_mechanics_block_v1_1_v3(input);
    read_mechanics_block_v3(input);
    read_mechanics_block_v1_2(input);
    read_mechanics_block_v2(input);
    read_mechanics_block_v1_3(input);
    Ok(mech)
}

fn read_mechanics_v4<Iter: Iterator<Item = String>>(input: &mut Iter) -> Result<Mechanics, Error> {
    read_mechanics_v3(input)
}

fn read_meshes(
    fname: &String,
    load_models: &Vec<String>,
    car_meshes: &mut HashMap<String, Mesh>,
) -> Result<(), Error> {
    let mut load_models = load_models.clone();
    load_models.sort();
    load_models.dedup();
    debug!("Models to load: {:?}", load_models);

    // Now iterate all meshes and load them.
    for mesh in load_models {
        let mut mesh_file_name = PathBuf::from(&fname);
        mesh_file_name.set_file_name(mesh);
        let mesh_file_name = path_subst(
            &mesh_file_name,
            &Path::new("MODELS"),
            Some(String::from("DAT")),
        );
        info!("### Opening mesh file {:?}", mesh_file_name);
        let meshes = Mesh::load_from(
            mesh_file_name
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
        )?;
        for mesh in meshes {
            car_meshes.insert(mesh.name.clone(), mesh);
        }
    }
    Ok(())
}

fn read_materials(
    fname: &String,
    load_materials: &HashSet<String>,
    car_materials: &mut HashMap<String, Material>,
) -> Result<(), Error> {
    for material in load_materials {
        let mut mat_file_name = PathBuf::from(&fname);
        mat_file_name.set_file_name(material);
        let mat_file_name = path_subst(&mat_file_name, &Path::new("MATERIAL"), None);
        info!("### Opening material {:?}", mat_file_name);
        let materials = Material::load_from(
            mat_file_name
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
        )?;
        for mat in materials {
            car_materials.insert(mat.name.clone(), mat);
        }
    }
    Ok(())
}

impl Car {
    pub fn dump(&self) {
        self.actors.dump();
        self.actors.dump_actor_points();
        for tex in &self.textures {
            println!("Texture {}: {}", tex.0, tex.1);
        }
        self.debug_meshes();
        for mat in &self.materials {
            println!("Material {}:", mat.0);
        }
    }

    pub fn debug_meshes(&self) {
        for mesh in &self.meshes {
            debug!("Mesh {}:", mesh.0);
            for mat in &mesh.1.material_names {
                debug!("... Material {}", mat);
            }
        }
    }

    pub fn load_from(fname: String) -> Result<Car, Error> {
        // Load description file.
        let description_file_name = path_subst(
            &Path::new(fname.as_str()),
            &Path::new("CARS"),
            Some(String::from("ENC")),
        );
        info!("### Opening car {:?}", description_file_name);

        let description_file = File::open(description_file_name)?;
        let description_file = BufReader::new(description_file);

        let mut input_lines = description_file.lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.starts_with("//")) // Skip whole-line comments
            .filter(|line| !line.is_empty()) // Skip empty lines
            // Separate in-line comments from data
            .map(|line| line.split("//").next().unwrap().trim().to_owned());

        let car_name = input_lines.next().unwrap();
        debug!("Car name {}", car_name);

        expect_match(&mut input_lines, "START OF DRIVABLE STUFF");

        let driver_head_3d_offset = parse_vector(&input_lines.next().unwrap());
        trace!(
            "Offset of driver's head in 3D space {:?}",
            driver_head_3d_offset
        );

        let head_turn_angles = input_lines.next().unwrap();
        trace!(
            "Angles to turn to make head go left and right {}",
            head_turn_angles
        );

        let mirror_3d_offset_and_fov = input_lines.next().unwrap();
        trace!(
            "Offset of 'mirror camera' in 3D space, viewing angle of mirror {}",
            mirror_3d_offset_and_fov
        );

        let pratcam_borders = input_lines.next().unwrap();
        trace!(
            "Pratcam border names (left, top, right, bottom) {}",
            pratcam_borders
        );

        expect_match(&mut input_lines, "END OF DRIVABLE STUFF");

        let engine_noise = input_lines.next().unwrap();
        trace!(
            "Engine noise (normal, enclosed space, underwater) {}",
            engine_noise
        );

        let stealworthy = input_lines.next().unwrap();
        trace!("Cannot be stolen (without cheat): {}", stealworthy);

        read_clauses(&mut input_lines);
        read_clauses(&mut input_lines);
        read_clauses(&mut input_lines);
        read_clauses(&mut input_lines);
        read_clauses(&mut input_lines);
        read_clauses(&mut input_lines);

        let grid_image = input_lines.next().unwrap();
        trace!("Grid image (opponent, frank, annie): {}", grid_image);

        let mut load_pixmaps = read_vector(&mut input_lines);
        load_pixmaps.append(&mut read_vector(&mut input_lines));
        load_pixmaps.append(&mut read_vector(&mut input_lines));

        let load_shadetable = read_vector(&mut input_lines);
        debug!("Shadetable to load: {:?}", load_shadetable);

        let mut load_materials = read_vector(&mut input_lines);
        load_materials.append(&mut read_vector(&mut input_lines));
        load_materials.append(&mut read_vector(&mut input_lines));

        let mut load_models = read_vector(&mut input_lines);

        let load_actors = read_vector(&mut input_lines);
        let load_actors: HashMap<isize, String> = load_actors
            .iter()
            .map(|act| act.split(","))
            .map(|mut split| {
                (
                    split.next().unwrap().parse().unwrap(),
                    String::from(split.next().unwrap()),
                )
            }).collect();
        debug!("Actors to load: {:?}", load_actors);

        let reflective_material = input_lines.next().unwrap();
        trace!(
            "Name of reflective screen material (or none if non-reflective): {}",
            reflective_material
        );

        // Number of steerable wheels
        // GroovyFunkRef of 1st steerable wheel -- this is index in the GROOVE array below
        // GroovyFunkRef of 2nd steerable wheel
        let steerable_wheels = read_vector(&mut input_lines);
        trace!("Steerable wheels GroovyFunkRefs: {:?}", steerable_wheels);

        let lfsus_gfref = input_lines.next().unwrap();
        trace!("Left-front suspension parts GroovyFunkRef: {}", lfsus_gfref);

        let rfsus_gfref = input_lines.next().unwrap();
        trace!(
            "Right-front suspension parts GroovyFunkRef: {}",
            rfsus_gfref
        );

        let lrsus_gfref = input_lines.next().unwrap();
        trace!("Left-rear suspension parts GroovyFunkRef: {}", lrsus_gfref);

        let rrsus_gfref = input_lines.next().unwrap();
        trace!("Right-rear suspension parts GroovyFunkRef: {}", rrsus_gfref);

        let driven_wheels_gfref = input_lines.next().unwrap();
        trace!(
            "Driven wheels GroovyFunkRefs (for spinning) - MUST BE 4 ITEMS: {}",
            driven_wheels_gfref
        );

        let nondriven_wheels_gfref = input_lines.next().unwrap();
        trace!(
            "Non-driven wheels GroovyFunkRefs (for spinning) - MUST BE 4 ITEMS: {}",
            nondriven_wheels_gfref
        );

        let driven_wheels_diameter = input_lines.next().unwrap();
        trace!("Driven wheels diameter: {}", driven_wheels_diameter);

        let nondriven_wheels_diameter = input_lines.next().unwrap();
        trace!("Non-driven wheels diameter: {}", nondriven_wheels_diameter);

        read_funk(&mut input_lines);
        read_grooves(&mut input_lines);

        read_some_metadata(&mut input_lines);
        read_some_metadata(&mut input_lines);
        read_some_metadata(&mut input_lines);

        let mechanics = input_lines.next().unwrap();
        if !mechanics.starts_with("START OF MECHANICS STUFF") {
            panic!(
                "Expected START OF MECHANICS STUFF, got {:?} instead",
                mechanics
            );
        }
        let version = mechanics
            .split(" version ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let _mech = match version {
            2 => read_mechanics_v2(&mut input_lines),
            3 => read_mechanics_v3(&mut input_lines),
            4 => read_mechanics_v4(&mut input_lines),
            x => panic!("Unsupported mechanics version {}", x),
        }?;

        expect_match(&mut input_lines, "END OF MECHANICS STUFF");

        let some_materials = read_vector(&mut input_lines);
        debug!("Some other materials to use: {:?}", some_materials);

        // @todo More post-mechanics stuff

        //
        // Meshes
        //
        let mut car_meshes = HashMap::<String, Mesh>::new();

        debug!("Meshes to load: {:?}", load_models);

        // Read meshes referenced from text description
        read_meshes(&fname, &load_models, &mut car_meshes)?;

        // Load actor file.
        let mut actor_file_name = PathBuf::from(&fname);
        let idx: isize = 0;
        actor_file_name.set_file_name(&load_actors[&idx]); // Read mipmap 0 actor
        let actor_file_name = path_subst(
            &actor_file_name,
            &Path::new("ACTORS"),
            Some(String::from("ACT")),
        );
        info!("### Opening actor {:?}", actor_file_name);
        let car_actors = Actor::load_from(actor_file_name.into_os_string().into_string().unwrap())?;

        // Read meshes referenced from actor file
        load_models.clear();
        for actor in car_actors.traverse() {
            match actor.data() {
                &ActorNode::MeshfileRef(ref name) => {
                    if !car_meshes.contains_key(name) {
                        load_models.push(name.clone())
                    }
                }
                _ => (),
            }
        }

        debug!("Extra meshes to load: {:?}", load_models);
        read_meshes(&fname, &load_models, &mut car_meshes)?;

        //
        // Materials
        //
        let mut load_materials: HashSet<String> =
            load_materials.iter().map(|s| s.clone()).collect();
        debug!("Materials to load: {:?}", load_materials);

        let mut car_materials = HashMap::<String, Material>::new();

        read_materials(&fname, &load_materials, &mut car_materials)?;

        load_materials.clear();
        for mat in some_materials {
            if !car_materials.contains_key(&mat) {
                load_materials.insert(mat.clone());
            }
        }

        debug!("Extra materials to load: {:?}", load_materials);
        read_materials(&fname, &load_materials, &mut car_materials)?;

        // Load palette from PIX file.
        let mut pal_file_name = PathBuf::from(&fname);
        pal_file_name.set_file_name("DRRENDER.PAL");
        let pal_file_name = path_subst(&pal_file_name, &Path::new("REG/PALETTES"), None);
        info!("### Opening palette {:?}", pal_file_name);
        let palette =
            &PixelMap::load_from(pal_file_name.into_os_string().into_string().unwrap())?[0];

        for x in 0..palette.units {
            trace!(
                "Palette alpha {}",
                palette.data[(x * palette.unit_bytes + 0) as usize]
            );
        }

        let load_pixmaps: HashSet<_> = load_pixmaps.iter().collect();
        debug!("Pixmaps to load: {:?}", load_pixmaps);

        let mut car_textures = HashMap::<String, PixelMap>::new();
        for pixmap in load_pixmaps {
            let mut pix_file_name = PathBuf::from(&fname);
            pix_file_name.set_file_name(pixmap);
            let pix_file_name = path_subst(&pix_file_name, &Path::new("PIXELMAP"), None);
            info!("### Opening pixelmap {:?}", pix_file_name);
            let pix = PixelMap::load_from(
                pix_file_name
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            )?;
            for pmap in pix {
                let pmap = pmap.remap_via(&palette)?;
                car_textures.insert(pmap.name.clone(), pmap);
            }
        }

        Ok(Car {
            name: car_name,
            actors: car_actors,
            meshes: car_meshes,
            materials: car_materials,
            textures: car_textures,
            base_translation: Vector3::from([0f32, 0f32, 0f32]),
        })
    }
}
