#[macro_use]
extern crate glium;
extern crate image;
extern crate carma;

use carma::teapot;

// #[derive(Copy, Clone)]
// pub struct Vertex {
//     position: [f32; 3],
//     tex_coords: [f32; 2],
// }

// implement_vertex!(Vertex, position, tex_coords);

fn main()
{
    use glium::{glutin, Surface};

    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("in-the-name-of.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("oglTest")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new();

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    // let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] };
    // let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
    // let shape = vec![vertex1, vertex2, vertex3];

    // let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

        // in vec2 tex_coords;
        // out vec2 v_tex_coords;
        //     v_tex_coords = tex_coords;

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

        // in vec2 v_tex_coords;
        // uniform sampler2D tex;
        //     color = texture(tex, v_tex_coords);

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;

        out vec4 color;

        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    // the direction of the light
    let light = [-1.0, 0.4, 0.9f32];

    let mut t: f32 = -0.5;

    let mut closed = false;
    while !closed {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            // matrix: [
            //     [ t.cos()*0.01, t.sin(), 0.0, 0.0],
            //     [-t.sin(), t.cos()*0.01, 0.0, 0.0],
            //     [0.0, 0.0, 0.001, 0.0],
            //     [0.0, 0.0, 0.0, 1.0f32],
            // ],
            // tex: &texture,
            matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            u_light: light,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw((&positions, &normals), &indices, &program, &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
