#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::texture::PixelValue;
use glium::texture::ClientFormat;
use cgmath::*;

#[derive(Copy, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

unsafe impl PixelValue for Pixel {
    fn get_format() -> ClientFormat {
        return ClientFormat::U8U8U8U8;
    }
}

fn get_texture(display: &glium::backend::glutin_backend::GlutinFacade) -> glium::texture::Texture2d {
    let size = display.get_window().unwrap().get_inner_size_pixels().unwrap();
    let mut pixels = vec![Pixel {r: 0, g: 0, b: 0, a: 0}; (size.0 * size.1) as usize];

    let fov = Rad::from(deg(90.0));

    for y in 0..size.1 {
        for x in 0..size.0 {
            let x_adj = (x as f32 / size.0 as f32) - 0.5;
            let y_adj = (y as f32 / size.1 as f32) - 0.5;

            let x_angle = fov * x_adj;
            let y_angle = fov * y_adj;

            let dir = Basis3::from_euler(y_angle, x_angle, Rad::zero())
                          .rotate_vector(Vector3::unit_z());

            pixels[(y * size.0 + x) as usize] = Pixel {
                r: ((dir.x * 0.5 + 0.5) * 255.0) as u8,
                g: ((dir.y * 0.5 + 0.5) * 255.0) as u8,
                b: ((dir.z * 0.5 + 0.5) * 255.0) as u8,
                a: 255,
            };
        }
    }

    let image = glium::texture::RawImage2d::from_raw_rgba(pixels, size);
    glium::texture::Texture2d::new(display, image).unwrap()
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
                      .with_dimensions(320, 320)
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
