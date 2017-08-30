//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//

extern crate carma;
extern crate glium;

use std::env;
use carma::support;
use carma::support::camera::CameraState;
use carma::support::car::Car;
use carma::support::render_manager::RenderManager;

fn main() {
    let car = Car::load_from(env::args().nth(1).unwrap()).unwrap();
    car.dump();

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("carma")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut render_manager = RenderManager::new(&display);
    render_manager.prepare_car(&car, &display);

    let mut camera = CameraState::new();

    support::start_loop(|| {
        camera.update();

        let mut target = display.draw();
        target.clear_color_and_depth((0.4, 0.4, 0.4, 0.0), 1.0);

        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        camera.set_aspect_ratio(aspect_ratio);

        render_manager.draw_car(&car, &mut target, &camera);
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    _ => camera.process_input(&event),
                }
            }
            _ => (),
        });

        action
    });
}
