//
// Part of Roadkill Project.
//
// Copyright 2010-2018, Berkus <berkus+github@metta.systems>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
pub mod support;

use cgmath::Vector3;
#[cfg(feature = "convert")]
use crate::support::texture::PixelMap;
use crate::support::{camera::CameraState, car::Car, render_manager::RenderManager};
use log::*;

fn setup_logging() -> Result<(), fern::InitError> {
    let base_config = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    });

    let stdout_config = fern::Dispatch::new()
        .level(log::LogLevelFilter::Info)
        .chain(std::io::stdout());

    let file_config = fern::Dispatch::new()
        .level(log::LogLevelFilter::Trace)
        .chain(
            std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // start log file anew each run
            .open("debug.log")?,
        );

    base_config
        .chain(stdout_config)
        .chain(file_config)
        .apply()?;

    Ok(())
}

use std::fs::{self, DirEntry};
use std::path::Path;
#[cfg(feature = "convert")]
use std::{fs::File, io::BufWriter, path::PathBuf};

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &mut FnMut(&DirEntry)) -> Result<(), support::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

#[cfg(feature = "convert")]
fn convert_pixmap(fname: String, palette: &PixelMap) -> Result<(), support::Error> {
    let pmap = PixelMap::load_from(fname.clone())
        .expect(format!("Couldnt open pix file {:?}", fname).as_ref());
    // let mut counter = 0;
    for pix in pmap {
        // counter += 1;
        let mut pngname = PathBuf::from(&fname);
        // let name = String::from(pngname.file_name().unwrap().to_str().unwrap());
        pngname.set_file_name(&pix.name);
        pngname.set_extension("png");

        info!("Creating file {:?}", pngname);
        let file = File::create(&pngname)
            .expect(format!("Couldnt create png file {:?}", pngname).as_ref());
        let w = &mut BufWriter::new(file);

        pix.write_png_remapped_via(palette, w)
            .expect("Failed to write PNG");
    }
    Ok(())
}

/// Uses different palette for race-selection part
#[cfg(feature = "convert")]
fn convert_menu_pixmap(fname: String) -> Result<(), support::Error> {
    let palette =
        &PixelMap::load_from(String::from("DecodedData/DATA/REG/PALETTES/DRACEFLC.PAL"))?[0];
    convert_pixmap(fname, palette)
}

#[cfg(feature = "convert")]
fn convert_game_pixmap(fname: String) -> Result<(), support::Error> {
    let palette =
        &PixelMap::load_from(String::from("DecodedData/DATA/REG/PALETTES/DRRENDER.PAL"))?[0];
    convert_pixmap(fname, palette)
}

/// Load palette once and then apply to a bunch of pixmap data
#[cfg(feature = "convert")]
fn convert_all_pixmaps() -> Result<(), support::Error> {
    let palette =
        &PixelMap::load_from(String::from("DecodedData/DATA/REG/PALETTES/DRRENDER.PAL"))?[0];
    visit_dirs(Path::new("DecodedData"), &mut |dir_entry| {
        if let Ok(file_type) = dir_entry.file_type() {
            let fname = String::from(dir_entry.path().to_str().unwrap());
            if file_type.is_file() && fname.ends_with(".PIX") {
                convert_pixmap(fname, palette).unwrap();
            }
        }
    })
}

fn main() {
    setup_logging().expect("failed to initialize logging");

    #[cfg(feature = "convert")]
    {
        convert_all_pixmaps().expect("Listing failed");
        convert_game_pixmap(String::from("DecodedData/DATA/PIXELMAP/EAGYELE.PIX"))
            .expect("Conversion failed");
    }

    // Load all cars and arrange in a grid 6x7 (40 cars total)

    let mut cars = Vec::new();
    let mut counter = 0;
    visit_dirs(Path::new("DecodedData/DATA/CARS"), &mut |entry| {
        if let Ok(file_type) = entry.file_type() {
            let fname = String::from(entry.path().to_str().unwrap());
            if file_type.is_file() && fname.ends_with(".ENC") {
                let mut car = Car::load_from(fname).unwrap();

                let z = 1.0f32 * f32::from(counter / 7);
                let x = 1.0f32 * f32::from(counter % 7 as u16);
                counter += 1;

                info!("Moving car {} to {},0,{}", counter, x, -z);

                car.base_translation = Vector3::from([x, 0f32, -z]);

                cars.push(car);
            }
        }
    }).unwrap();

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("carma")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut render_manager = RenderManager::new(&display);
    for car in &cars {
        render_manager.prepare_car(car, &display);
    }

    let mut camera = CameraState::new();

    support::start_loop(|| {
        camera.update();

        let mut target = display.draw();
        target.clear_color_and_depth((0.4, 0.4, 0.4, 0.0), 1.0);

        for car in &cars {
            render_manager.draw_car(car, &mut target, &camera);
        }
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|ev| {
            if let glutin::Event::WindowEvent { event, .. } = ev {
                match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    _ => camera.process_input(&event),
                }
            }
        });

        action
    });
}
