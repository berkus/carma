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
    crate::{
        assets::car_asset::{CarAsset, CarAssetLoader},
        support::{camera::CameraState, car::Car, render_manager::RenderManager},
    },
    anyhow::{anyhow, Context, Error, Result},
    bevy::{
        prelude::*,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    cgmath::Vector3,
    fehler::throws,
    glium::{
        glutin::{
            event::{Event, WindowEvent},
            event_loop::ControlFlow,
        },
        Surface,
    },
    log::info,
    smooth_bevy_cameras::{
        controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
        LookTransformPlugin,
    },
    std::{f32::consts::PI, path::Path},
    support::visitor::visit_files,
};

mod assets;
mod support;

// @todo these are all resource types under support, just implement AssetLoadRequestHandler for them?
// ACT
// DAT
// MAT
// PAL
// PIX

fn setup_cars(
    _commands: Commands,
    asset_server: ResMut<AssetServer>,
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
    let _handle = asset_server.load_folder("DecodedData/DATA/CARS");

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
    support::logger::setup_logging().expect("failed to initialize logging");

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

    let mut app = App::new();

    // app.register_asset_loader(CarAssetLoader)
    //     .init_asset::<CarAsset>();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LookTransformPlugin)
        .add_plugins(UnrealCameraPlugin::default())
        .add_systems(Startup, setup_cars)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate);
    // .add_system(animate_camera)

    app.run();

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

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.;

// fn setup() {
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
//         ..default()
//     });
// }

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(shape::Cube::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Capsule::default().into()),
        meshes.add(shape::Torus::default().into()),
        // meshes.add(shape::Icosphere::default().into()),
        meshes.add(shape::UVSphere::default().into()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            shape::Plane {
                size: 50.,
                subdivisions: 2,
            }
            .into(),
        ),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert(UnrealCameraBundle::new(
            UnrealCameraController::default(),
            Vec3::new(0.0, 6.0, 12.0),
            Vec3::new(0., 1., 0.),
            Vec3::Y,
        ));
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
