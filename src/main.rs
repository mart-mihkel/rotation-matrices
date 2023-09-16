#[macro_use]
extern crate glium;

mod teapot;

use std::fs;
use std::path::Path;
use glium::{Depth, DepthTest, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::index::PrimitiveType;

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 800.0))
        .with_title("Rotation matrices");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    // load teapot
    let positions = VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    // load shaders
    let vertex_shader = fs::read_to_string(Path::new("src/shader/shader.vert")).unwrap();
    let fragment_shader = fs::read_to_string(Path::new("src/shader/shader.frag")).unwrap();
    let program = Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

    // rotation angles
    let mut alpha = 0.0f32;
    let mut beta = 0.0f32;
    let mut gamma = 0.0f32;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            match key {
                                VirtualKeyCode::Escape => control_flow.set_exit(),
                                VirtualKeyCode::Left => beta += 0.05,
                                VirtualKeyCode::Right => beta -= 0.05,
                                VirtualKeyCode::Up => alpha += 0.05,
                                VirtualKeyCode::Down => alpha -= 0.05,
                                VirtualKeyCode::L => gamma += 0.05,
                                VirtualKeyCode::R => gamma -= 0.05,
                                _ => ()
                            }
                            display.gl_window().window().request_redraw();
                        }
                    }
                    _ => ()
                }
            }
            Event::RedrawRequested(_) => {
                // rotation matrices
                let roll = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, alpha.cos(), alpha.sin(), 0.0],
                    [0.0, -alpha.sin(), alpha.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];

                let pitch = [
                    [beta.cos(), 0.0, -beta.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [beta.sin(), 0.0, beta.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];

                let yaw = [
                    [gamma.cos(), gamma.sin(), 0.0, 0.0],
                    [-gamma.sin(), gamma.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];

                // light direction
                let light = [-1.0, 0.4, -0.1f32];

                // rendering
                let params = DrawParameters {
                    depth: Depth {
                        test: DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let mut target = display.draw();
                target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
                target.draw(
                    (&positions, &normals),
                    &indices,
                    &program,
                    &uniform! { roll: roll, pitch: pitch, yaw: yaw , u_light: light},
                    &params,
                ).unwrap();
                target.finish().unwrap();
            }
            _ => ()
        }
    });
}