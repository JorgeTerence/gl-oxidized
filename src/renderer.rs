use std::ops::Add;

use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{
    implement_vertex, uniform, Display, DrawParameters, IndexBuffer, Surface, VertexBuffer,
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes};

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

pub struct Program {
    _window: Box<Window>,
    display: Display<WindowSurface>,
    time: u32,

    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u32>,
}

impl Program {
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

        Self {
            _window: Box::new(window),
            display,
            time: 0,
            vertices: vertex_buffer,
            indices: index_buffer,
        }
    }
}

impl ApplicationHandler for Program {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop
            .create_window(WindowAttributes::default())
            .expect("Faile to create application window");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(window_size) => self.display.resize(window_size.into()),
            WindowEvent::RedrawRequested => {
                self.time += 1;

                let mut target = self.display.draw();
                target.clear_color(0.05, 0.45, 0.75, 1.0);

                let program = glium::Program::from_source(
                    &self.display,
                    VERTEX_SRC,
                    FRAGMENT_SRC,
                    None,
                )
                .unwrap();

                target
                    .draw(
                        &self.vertices,
                        &self.indices,
                        &program,
                        &uniform! { t: self.time },
                        &DrawParameters::default(),
                    )
                    .expect("Failed GPU drawing call to");

                target.finish().unwrap();
            }
            _ => (),
        };
    }
}
