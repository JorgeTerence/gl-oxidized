mod engine;
mod renderer;

use glium::winit::event_loop::EventLoop;
use renderer::Program;

// static VERTEX_SRC: &str = include_str!("shaders/triangle-vertex.glsl");
// static FRAGMENT_SRC: &str = include_str!("shaders/triangle-fragment.glsl");

fn main() {
    let event_loop = EventLoop::new().expect("Failed event loop init");
    let mut app = Program::new(&event_loop, "OpenGL Oxidized");
    event_loop.run_app(&mut app).expect("Failed to run app");
}
