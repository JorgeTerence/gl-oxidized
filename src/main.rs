use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::winit::event::WindowEvent;
use glium::winit::event_loop::EventLoop;
use glium::{implement_vertex, uniform, Display, Program, Surface, VertexBuffer};
use winit::application::ApplicationHandler;
use winit::window::{Window, WindowAttributes};

static VERTEX_SRC: &str = include_str!("shaders/triangle-vertex.glsl");
static FRAGMENT_SRC: &str = include_str!("shaders/triangle-fragment.glsl");

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

struct App<'a> {
    title: &'a str,
    vertices: Vec<Vertex>,
    indices: Vec<usize>,

    time_delta: f32,
    time: f32,

    window: Window,
    display: Display<WindowSurface>,

    program: Option<Program>,
    vb: Option<VertexBuffer<Vertex>>,
}

impl<'a> App<'a> {
    pub fn new(event_loop: &EventLoop<()>, title: &'a str) -> Self {
        let (window, display) = SimpleWindowBuilder::new().build(event_loop);

        Self {
            title,
            vertices: vec![],
            indices: vec![],
            time_delta: 0.001,
            time: 0.0,
            window,
            display,
            program: None,
            vb: None,
        }
    }

    fn add_obj(&mut self, v: [Vertex; 3]) -> Result<(), &'static String> {
        assert!(
            v.iter().all(|v| 1.0 >= v.position[0]
                && v.position[0] >= -1.0
                && 1.0 >= v.position[1]
                && v.position[1] >= -1.0),
            "Vertices out of bounds"
        );

        self.indices
            .extend(self.vertices.len()..self.vertices.len() + 3);
        self.vertices.extend(v);

        Ok(())
    }

    fn compile(
        &mut self,
        vertex_src: &'a str,
        frag_src: &'a str,
    ) -> Result<&Self, &'static String> {
        self.program =
            Some(Program::from_source(&self.display, vertex_src, frag_src, None).unwrap());

        self.vb = Some(VertexBuffer::new(&self.display, &self.vertices).unwrap());

        Ok(self)
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title(self.title)
                    .with_blur(true),
            )
            .expect("Faile to create application window");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(window_size) => self.display.resize(window_size.into()),
            WindowEvent::RedrawRequested => {
                self.time += self.time_delta;

                let t_off = self.time.sin() * 0.5;

                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);

                target
                    .draw(
                        &self.vb,
                        &self.indices,
                        &self.program.unwrap(),
                        &uniform! { x: t_off },
                        &Default::default(),
                    )
                    .unwrap();

                target.finish().unwrap();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("Failed event loop init");
    let mut app = App::new(&event_loop, "OpenGL Oxidized");

    app.add_obj([
        Vertex::new(-0.5, -0.5),
        Vertex::new(0.0, 0.5),
        Vertex::new(0.75, -0.25),
    ]);

    app.compile(VERTEX_SRC, FRAGMENT_SRC).unwrap();

    event_loop.run_app(&mut app);
}
