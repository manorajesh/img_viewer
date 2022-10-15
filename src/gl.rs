#[macro_use]
extern crate glium;
extern crate image;

use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let img = image::io::Reader::open("orange_v7.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();

    let image_dimensions = img.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);
    
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    implement_vertex!(Vertex, position);
    let v1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let v2 = Vertex { position: [0.0, 0.5], tex_coords: [0.0, 1.0] };
    let v3 = Vertex { position: [0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![v1, v2, v3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        
        uniform mat4 matrix;
        
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
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
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex: &texture,
    };
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
        
        *control_flow = glutin::event_loop::ControlFlow::Wait;
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
