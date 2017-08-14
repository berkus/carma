#[macro_use]
extern crate glium;
extern crate image;
extern crate carma;

use carma::support;
use carma::support::camera;
use carma::support::Vertex;

fn main()
{
    use glium::{glutin, Surface};
    use std::io::Cursor;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("oglTest")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    //
    // texture loading
    //
    let image = image::load(Cursor::new(&include_bytes!("tuto-14-diffuse.jpg")[..]),
                            image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("tuto-14-normal.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

    //
    // shaders
    //
    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;
        in vec2 tex_coords;

        out vec3 v_normal;
        out vec3 v_position;
        out vec2 v_tex_coords;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            v_tex_coords = tex_coords;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        in vec3 v_position;
        in vec2 v_tex_coords;

        out vec4 color;

        uniform vec3 u_light;
        uniform vec3 u_specular_color;
        uniform sampler2D diffuse_tex;
        uniform sampler2D normal_tex;

        mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
            vec3 dp1 = dFdx(pos);
            vec3 dp2 = dFdy(pos);
            vec2 duv1 = dFdx(uv);
            vec2 duv2 = dFdy(uv);

            vec3 dp2perp = cross(dp2, normal);
            vec3 dp1perp = cross(normal, dp1);
            vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
            vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;

            float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
            return mat3(T * invmax, B * invmax, normal);
        }

        void main() {
            vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
            vec3 ambient_color = diffuse_color * 0.2;

            vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;

            // Tangent Binormal Normal
            mat3 tbn = cotangent_frame(v_normal, v_position, v_tex_coords);
            vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));

            float diffuse = max(dot(real_normal, normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(real_normal)), 0.0), 16.0);

            color = vec4(ambient_color + diffuse * diffuse_color + specular * u_specular_color, 1.0);
        }
    "#;

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
