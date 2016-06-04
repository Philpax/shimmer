#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate stopwatch;

mod scene;
mod root_find;

use cgmath::*;
use stopwatch::Stopwatch;
use scene::Object;

fn get_texture(display: &glium::backend::glutin_backend::GlutinFacade) -> glium::texture::Texture2d {
    let size = display.get_window().unwrap().get_inner_size_pixels().unwrap();
    let aspect_ratio = size.0 as f32 / size.1 as f32;
    let mut pixels = vec![scene::Colour::zero(); (size.0 * size.1) as usize];

    let fov = Rad::from(deg(90.0));
    let fov_horz = fov * aspect_ratio;

    let line_origin = Point3::new(0.0, 1.0, -1.0);
    let line_base_dir = Vector3::unit_z();
    let min_distance = 0.0;
    let max_distance = 5.0;
    let enable_logging = false;

    let plane = scene::Plane::new(Vector3::new(0.0, 1.0, 0.0),
                                  Point3::origin(),
                                  scene::Colour::new(0, 255, 0, 255));
    let sphere1 = scene::Sphere::new(Point3::new(0.0, 0.0, 2.0), 1.0, scene::Colour::white());
    let sphere2 = scene::Sphere::new(Point3::new(0.0, 1.25, 2.0), 0.5, scene::Colour::white());

    let composite_function = |point: Point3<f32>| {
        plane.evaluate(point).union(sphere1.evaluate(point)).union(sphere2.evaluate(point))
    };

    let sw = Stopwatch::start_new();

    for y in 0..size.1 {
        for x in 0..size.0 {
            let x_adj = (x as f32 / size.0 as f32) - 0.5;
            let y_adj = (y as f32 / size.1 as f32) - 0.5;

            let x_angle = fov_horz * x_adj;
            let y_angle = fov * y_adj;

            let line_dir = Basis3::from_euler(-y_angle, x_angle, Rad::zero())
                               .rotate_vector(line_base_dir);

            let distance = root_find::ray_march(min_distance,
                                                max_distance,
                                                0.5,
                                                0.05,
                                                |distance: f32| {
                                                    composite_function(line_origin +
                                                                       (distance * line_dir))
                                                })
                               .unwrap_or(max_distance)
                               .min(max_distance);

            // Remap a value in [min_distance, max_distance] to [1.0, 0.0]
            let brightness = if distance < 0.0 {
                0.0
            } else {
                1.0 - ((distance - min_distance) / (max_distance - min_distance))
            };

            if enable_logging {
                if x % 20 == 0 && y % 20 == 0 {
                    println!("{} {} {:?}: {} {}",
                             x_adj,
                             y_adj,
                             line_dir,
                             distance,
                             brightness);
                }
            }

            let point = composite_function(line_origin + distance * line_dir);
            pixels[(y * size.0 + x) as usize] = point.colour * brightness;
        }
    }

    println!("Image took {}ms", sw.elapsed_ms());

    let image = glium::texture::RawImage2d::from_raw_rgba(pixels, size);
    glium::texture::Texture2d::new(display, image).unwrap()
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
                      .with_dimensions(640, 360)
                      .with_title("Shimmer")
                      .build_glium()
                      .unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let shape = vec![
        Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None)
                      .unwrap();

    let texture = get_texture(&display);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! { tex: &texture };

        target.draw(&vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default())
              .unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
