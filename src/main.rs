//#![feature(try_trait)]
#![allow(unused_imports)]

//
// Part of Roadkill Project.
//
// Copyright 2010-2024, Berkus <berkus+github@metta.systems>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{camera::CameraState, car::Car, render_manager::RenderManager},
    anyhow::{anyhow, Context, Error, Result},
    bevy::prelude::*,
    cgmath::Vector3,
    fehler::throws,
    glium::{
        glutin::{
            event::{Event, WindowEvent},
            event_loop::ControlFlow,
        },
        Surface,
    }, log::info, std::path::Path, support::visitor::visit_files
};

pub mod support;

#[throws(fern::InitError)]
fn setup_logging() {
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
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout());

    let file_config = fern::Dispatch::new().level(log::LevelFilter::Trace).chain(
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
}

// @todo these are all resource types under support, just implement AssetLoadRequestHandler for them?
// ACT
// DAT
// MAT
// PAL
// PIX

fn setup_cars(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    // mut textures: ResMut<Assets<Texture>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) /*-> Result<Vec<Car>>*/
{
    // bevy @todo: load all textures into the texture atlas
    // TextureAtlasBuilder

    // I'd say use the bevy_gltf crate/plugin as a reference: https://github.com/bevyengine/bevy/tree/master/crates/bevy_gltf/src
    // Specifically, you need to implement AssetLoader<MyAsset>  for MyAssetLoader and call:
    // app.add_asset::<MyAsset>()
    //     .add_asset_loader::<MyAsset, MyAssetLoader>();

    // add handler for ENC assets, then
    // asset_server.add_handler(crate::support::car::CarLoadRequestHandler);
    // asset_server.add_loader(crate::support::car::CarLoader);
    asset_server
        .load_asset_folder("DecodedData/DATA/CARS")
        .unwrap();

    // let texture_handle = asset_server
    //     .load_sync(
    //         &mut textures,
    //         "assets/textures/rpg/chars/gabe/gabe-idle-run.png",
    //     )
    //     .unwrap();
    // let texture = textures.get(&texture_handle).unwrap();
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 7, 1);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Load all cars and arrange in a grid 6x7 (40 cars total)

    // let mut cars = Vec::new();
    // let mut counter = 0;
    // visit_files(Path::new("DecodedData/DATA/CARS"), &mut |entry| {
    //     if let Ok(file_type) = entry.file_type() {
    //         let fname = entry
    //             .path()
    //             .to_str()
    //             .map(String::from)
    //             .ok_or_else(|| anyhow!("Failed to make filename"))?;
    //         if file_type.is_file() && fname.ends_with(".ENC") {
    //             let mut car = Car::load_from(fname)?;
    //
    //             let z = 1.0f32 * f32::from(counter / 7);
    //             let x = 1.0f32 * f32::from(counter % 7 as u16);
    //             counter += 1;
    //
    //             info!("Moving car {} to {},0,{}", counter, x, -z);
    //
    //             car.base_translation = Vector3::from([x, 0f32, -z]);
    //
    //             cars.push(car);
    //         }
    //     }
    //     Ok(())
    // })?;
    //
    // Ok(cars)

    // commands
    //     .spawn(Camera2dComponents::default())
    //     .spawn(SpriteSheetComponents {
    //         texture_atlas: texture_atlas_handle,
    //         scale: Scale(6.0),
    //         ..Default::default()
    //     })
    //     .with(Timer::from_seconds(0.1));
}

#[throws]
fn main() {
    setup_logging().context("failed to initialize logging")?;
    // ❌ Resource loading from files
    // ❌ Texture placement in atlas
    // ❌ Compose animatable models from disparate Car resources
    // ❌ Set up bevy animation systems for each model
    // ❌ Set up view and camera
    // ❌ Render

    // let cars = setup_textures()?;

    // Prepare window -- move to bevy init

    // let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // let window = glium::glutin::window::WindowBuilder::new()
    //     .with_title("carma")
    //     .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 600.0));
    // let windowed_context = glium::glutin::ContextBuilder::new();
    //
    // let display = glium::Display::new(window, windowed_context, &events_loop)?;
    //
    // let mut render_manager = RenderManager::new(&display);
    // for car in &cars {
    //     render_manager.prepare_car(car, &display);
    // }
    //
    // let mut camera = CameraState::new();

    App::build()
        .add_default_plugins()
        .add_asset::<Car>()
        // .add_asset_loader::<Car, CarAssetLoader>()
        .add_startup_system(setup_cars.system())
        // .add_system(animate_camera.system())
        .run()

    // events_loop.run(move |event, _, control_flow| {
    //     println!("{:?}", event);
    //     *control_flow = ControlFlow::Wait;
    //
    //     camera.update();
    //
    //     match event {
    //         Event::LoopDestroyed => return,
    //         Event::WindowEvent { event, .. } => match event {
    //             // WindowEvent::Resized(physical_size) => display.resize(physical_size),
    //             WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
    //             _ => camera.process_input(&event),
    //         },
    //         Event::RedrawRequested(_) => {
    //             let mut frame = display.draw();
    //             frame.clear_color(0.4, 0.4, 0.4, 0.0);
    //             frame.clear_depth(1.0);
    //
    //             for car in &cars {
    //                 render_manager.draw_car(car, &mut frame, &camera);
    //             }
    //             frame.finish().unwrap();
    //             // windowed_context.swap_buffers().unwrap();
    //         }
    //         _ => (),
    //     }
    // });
}

// fn setup() {
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
//         ..default()
//     });
// }
