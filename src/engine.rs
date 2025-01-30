use winit::event_loop::EventLoop;

use crate::{
    renderer::{self, Primitive, Vertex},
    FRAGMENT_SRC, VERTEX_SRC,
};

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
        let mut program = renderer::Renderer::new(&self.lifecycle, self.title);

        let mut vertices = Vec::<Vertex>::new();
        let mut indices = Vec::<u32>::new();

        for obj in self.bodies {
            let initial_index = vertices.len() as u32;
            let (v, i) = obj.get_primitives(initial_index);
            vertices.extend(v);
            indices.extend(i);
        }

        program =
            program.with_objects(vertices, indices, VERTEX_SRC, FRAGMENT_SRC, Some("objects"));

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

impl Primitive for Body {
    fn get_primitives(self: Self, i: u32) -> (Vec<Vertex>, Vec<u32>) {
        (
            // Four vertices of the square
            vec![
                self.pos,
                self.pos + Vertex::new(0.0, self.size),
                self.pos + Vertex::new(self.size, 0.0),
                self.pos + Vertex::new(self.size, self.size),
            ],
            // Six edges that connect the triangles that make up the square
            (i..i + 3).chain(i + 1..i + 4).collect(),
        )
    }
}

pub struct UI {
    // TODO: create ascii texture
    text: String,
    position: (f32, f32),
    border: bool,
    padding: f32,
}
