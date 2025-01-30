use std::ops::Add;

use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{
    implement_vertex, uniform, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::{FRAGMENT_SRC, VERTEX_SRC};

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

impl Add<Vertex> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vertex) -> Self::Output {
        Self::Output {
            position: [
                self.position[0] + rhs.position[0],
                self.position[1] + rhs.position[1],
            ],
        }
    }
}

pub trait Primitive {
    fn get_primitives(self: Self, i: u32) -> (Vec<Vertex>, Vec<u32>);
}

pub struct Renderer {
    _window: Box<Window>,
    display: Display<WindowSurface>,
    time: u32,

    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u32>,
    program: Program,
}

impl Renderer {
    pub fn new(
        event_loop: &EventLoop<()>,
        title: &'static str,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Self {
        let (window, display) = SimpleWindowBuilder::new().build(event_loop);
        window.set_title(title);

        let mut frame = display.draw();
        frame.clear_color(0.15, 0.45, 0.75, 1.0);
        frame.finish().unwrap();

        let vertex_buffer =
            VertexBuffer::new(&display, &vertices).expect("Failed to build schene's vertices");

        let index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices)
            .expect("Failed to build vertex mesh");

        let program = Program::from_source(&display, VERTEX_SRC, FRAGMENT_SRC, None)
            .expect("Failed to build GPU program");

        Self {
            _window: Box::new(window),
            display,
            time: 0,
            vertices: vertex_buffer,
            indices: index_buffer,
            program,
        }
    }
}

impl ApplicationHandler for Renderer {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop
            .create_window(WindowAttributes::default())
            .expect("Faile to create application window");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(window_size) => self.display.resize(window_size.into()),
            WindowEvent::RedrawRequested => {
                self.time += 1;

                let mut target = self.display.draw();
                target.clear_color(0.05, 0.45, 0.75, 1.0);

                // Draw call for environment
                // target.draw(

                // ).expect("Failed to draw environment");

                // Draw call for bodies
                target
                    .draw(
                        &self.vertices,
                        &self.indices,
                        &self.program,
                        &uniform! { t: self.time },
                        &DrawParameters::default(),
                    )
                    .expect("Failed to draw bodies");

                // Draw call for UI

                target.finish().unwrap();
            }
            _ => (),
        };
    }
}
