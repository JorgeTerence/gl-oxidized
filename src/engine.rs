use winit::event_loop::EventLoop;

use crate::renderer::{self, Vertex};

pub const EARTH_GRAVITY: f32 = 9.81;

pub struct Scene {
    lifecycle: EventLoop<()>,
    title: &'static str,

    environment: Vec<Environment>,
    bodies: Vec<Body>,
    interface: Vec<UI>,

    gravity: f32,
}

impl Scene {
    pub fn new(title: &'static str, gravity: f32) -> Self {
        let event_loop = EventLoop::new().expect("Failed event loop init");

        Self {
            lifecycle: event_loop,
            title,

            environment: Vec::new(),
            bodies: Vec::new(),
            interface: Vec::new(),

            gravity,
        }
    }

    pub fn execute(self) {
        // TODO: generate VBs for everything in the scene
        let mut vertices = Vec::<Vertex>::new();
        let mut indices = Vec::<u32>::new();

        for obj in self.bodies {
            let initial_index = vertices.len() as u32;

            // Four vertices of the square
            vertices.push(obj.pos); // 1
            vertices.push(obj.pos + Vertex::new(0.0, obj.size));
            vertices.push(obj.pos + Vertex::new(obj.size, 0.0));
            vertices.push(obj.pos + Vertex::new(obj.size, obj.size));

            // Six edges that connect the triangles that make up the square
            // 0, 1, 2
            // 1, 2, 3
            indices.extend(initial_index..initial_index + 3);
            indices.extend(initial_index + 1..initial_index + 4);
        }

        let mut program = renderer::Renderer::new(&self.lifecycle, self.title, vertices, indices);
        self.lifecycle
            .run_app(&mut program)
            .expect("Failed to run app");
    }

    pub fn add_obj(&mut self, obj: Body) -> Result<(), &'static str> {
        self.bodies.push(obj);
        Ok(())
    }
}

pub struct Environment {
    friction: f32,
}

// Assumes all bodies are squares
pub struct Body {
    pub mass: f32, // >= 0
    pub acceleration: f32,
    pub velocity: (f32, f32, f32), // 3D vector
    pub size: f32,
    pub pos: Vertex,
}

pub struct UI {
    // TODO: create ascii texture
    text: String,
    position: (f32, f32),
    border: bool,
    padding: f32,
}
