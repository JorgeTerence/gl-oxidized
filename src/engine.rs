use winit::event_loop::EventLoop;

use crate::renderer;

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
        let mut app = renderer::Program::new(&self.lifecycle, self.title);
        self.lifecycle.run_app(&mut app).expect("Failed to run app");
    }
}

pub struct Environment {
    friction: f32,
}

pub struct Body {
    mass: f32, // >= 0
    acceleration: f32,
    velocity: (f32, f32, f32), // 3D vector
    size: f32,
}

pub struct UI {
    // TODO: create ascii texture
    text: String,
    position: (f32, f32),
    border: bool,
    padding: f32,
}
