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

struct Render {
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u32>,
    program: Program,
    label: &'static str,
}

pub struct Renderer {
    window: Box<Window>,
    display: Display<WindowSurface>,
    time: f32,
    time_delta: f32,

    objects: Vec<Render>,
}

impl Renderer {
    pub fn new(event_loop: &EventLoop<()>, title: &'static str) -> Self {
        let (window, display) = SimpleWindowBuilder::new().build(event_loop);
        window.set_title(title);

        let mut frame = display.draw();
        frame.clear_color(0.15, 0.45, 0.75, 1.0);
        frame.finish().unwrap();

        Self {
            window: Box::new(window),
            display,
            time: 0.0,
            time_delta: 0.005,
            objects: Vec::new(),
        }
    }

    pub fn with_objects(
        mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        vertex: &'static str,
        fragment: &'static str,
        label: Option<&'static str>,
    ) -> Self {
        let program = Program::from_source(&self.display, vertex, fragment, None)
            .expect("Failed to build GPU program");

        let vertex_buffer =
            VertexBuffer::new(&self.display, &vertices).expect("Failed to build schene's vertices");

        let index_buffer = IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &indices)
            .expect("Failed to build vertex mesh");

        self.objects.push(Render {
            vertices: vertex_buffer,
            indices: index_buffer,
            program,
            label: label.unwrap_or("<unknown>"),
        });

        self
    }
}

impl ApplicationHandler for Renderer {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop
            .create_window(WindowAttributes::default())
            .expect("Faile to create application window");

        self.window.request_redraw();
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(window_size) => self.display.resize(window_size.into()),
            WindowEvent::RedrawRequested => {
                self.time += self.time_delta;

                let mut target = self.display.draw();
                target.clear_color(0.05, 0.45, 0.75, 1.0);

                for render in &self.objects {
                    target
                        .draw(
                            &render.vertices,
                            &render.indices,
                            &render.program,
                            &uniform! { t: self.time },
                            &DrawParameters::default(),
                        )
                        .expect(
                            format!("|{:?}| failed to draw {}", window_id, render.label).as_str(),
                        );
                }
                target.finish().unwrap();
            }
            _ => (),
        };
    }
}
