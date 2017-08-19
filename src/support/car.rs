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
use std::fs::File;
// use byteorder::ReadBytesExt;
// use support::resource::Chunk;
use support::path_subst;
use std::path::Path;

// Car assembles the gameplay object (a car in this case) from various model and texture files.
#[derive(Default)]
pub struct Car {
}

impl Car {
    pub fn load_from(fname: String) -> Result<Car, Error> {
        let mut car = Car::default();

        // Load description file.
        let description_file_name = path_subst(&Path::new(fname.as_str()), &Path::new("CARS"), Some(String::from("ENC")));
        println!("Opening car {:?}", description_file_name);

        let description_file = File::open(description_file_name)?;
        let description_file = BufReader::new(description_file);

        let mut in_mechanics = false;
        let mut mechanics_count = 0;

        let mut input_lines = description_file.lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.starts_with("//")) // Skip whole-line comments
            .map(|line| line.split("//").next().unwrap().trim().to_owned()); // Separate in-line comments from data

        let car_name = input_lines.next().unwrap();
        println!("Car name {}", car_name);

        for line in input_lines
        {
            // println!("{}", line);

            // State machine triggers:
            //
            // START OF DRIVABLE STUFF
            // END OF DRIVABLE STUFF
            // START OF FUNK
            // END OF FUNK
            // START OF GROOVE
            // NEXT GROOVE
            // END OF GROOVE
            // START OF MECHANICS STUFF version 3
            // END OF MECHANICS STUFF
        }

        Ok(car)
    }
}
