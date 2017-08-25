//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#[macro_use]
extern crate glium;
extern crate image;
extern crate carma;

use std::env;
use std::str;
use carma::support;
use carma::support::camera;
use carma::support::Vertex;
use carma::support::car::Car;

fn main()
{
    let car = Car::load_from(env::args().nth(1).unwrap()).unwrap();
    car.dump();

    use glium::{glutin, Surface};
    use std::io::Cursor;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("carma")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    //
    // texture loading
    //
    let tex = &car.textures[0];
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&tex.data, (tex.w as u32, tex.h as u32));
    let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("tuto-14-normal.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

    //
    // shaders
    //
    let vertex_shader_src = str::from_utf8(include_bytes!("../shaders/first.vert")).unwrap();
    let fragment_shader_src = str::from_utf8(include_bytes!("../shaders/first.frag")).unwrap();

    // shader program
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // the direction of the light - @todo more light sources?
    let light = [-1.0, 0.4, 0.9f32];

    let wall = glium::vertex::VertexBuffer::new(&display, &[
        Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [-1.0,  1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [ 1.0,  1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [-1.0, -1.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [ 1.0, -1.0] },
    ]).unwrap();

    let mut camera = camera::CameraState::new();
    // camera.set_position((2.0, -1.0, 1.0));
    // camera.set_direction((-2.0, 1.0, 1.0));
    // camera.set_position((0.0, 0.0, 1.0));
    // camera.set_direction((0.0, 0.0, 0.0));

    support::start_loop(|| {
        camera.update();

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        camera.set_aspect_ratio(aspect_ratio);

        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.001, 0.0],
                [0.0, 0.0, 0.03, 1.0f32]
            ],
            view: camera.get_view(),
            perspective: camera.get_perspective(),
            u_light: light,
            u_specular_color: [1.0, 1.0, 1.0f32],
            diffuse_tex: &diffuse_texture,
            normal_tex: &normal_map,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        // target.draw((&positions, &normals), &indices, &program, &uniforms,
            // &params).unwrap();
        target.draw(&wall, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    _ => camera.process_input(&event),
                },
                _ => (),
            }
        });

        action
    });
}
