use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::{implement_vertex, Display, Surface};
use winit::window::{Window, WindowAttributes};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

pub struct Program {
    display: Display<WindowSurface>,
    _window: Box<Window>,
}

impl Program {
    pub fn new(event_loop: &EventLoop<()>, title: &'static str) -> Self {
        let (window, display) = SimpleWindowBuilder::new().build(event_loop);
        window.set_title(title);

        let mut frame = display.draw();
        frame.clear_color(0.15, 0.45, 0.75, 1.0);
        frame.finish().unwrap();

        Self {
            display,
            _window: Box::new(window),
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
                let mut target = self.display.draw();
                target.clear_color(0.15, 0.45, 0.75, 1.0);
                target.finish().unwrap();
            }
            _ => (),
        };
    }
}
