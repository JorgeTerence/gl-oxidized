use glium::backend::glutin::SimpleWindowBuilder;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::EventLoop;
use glium::{implement_vertex, uniform, Surface, VertexBuffer};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

implement_vertex!(Vertex, position);

fn centroid(triangle: Vec<Vertex>) -> [f32; 2] {
    [
        (triangle[0].position[0] + triangle[1].position[0] + triangle[2].position[0]) / 3.0,
        (triangle[0].position[1] + triangle[1].position[1] + triangle[2].position[1]) / 3.0,
    ]
}

fn main() {
    let event_loop = EventLoop::builder().build().expect("event loop building");
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
    window.set_title("OpenGL Oxidized");

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();

    let triangle = vec![
        Vertex::new(-0.5, -0.5),
        Vertex::new(0.0, 0.5),
        Vertex::new(0.75, -0.25),
    ];

    let vb = VertexBuffer::new(&display, &triangle).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform float x;

        void main() {
            float radius = sqrt(pow(position.x, 2) + pow(position.y, 2));
            vec2 warped = vec2(position.x, sin(position.x + x / radius));
            gl_Position = vec4(warped, 0.0, 1.0);
        }"#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }"#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;
    let _c = centroid(triangle);

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::Resized(window_size) => display.resize(window_size.into()),
                WindowEvent::RedrawRequested => {
                    t += 0.001;

                    let t_off = t.sin() * 0.5;

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);

                    target
                        .draw(
                            &vb,
                            &indices,
                            &program,
                            &uniform! { x: t_off },
                            &Default::default(),
                        )
                        .unwrap();

                    target.finish().unwrap();
                }
                _ => (),
            },
            Event::AboutToWait => window.request_redraw(),
            _ => (),
        };
    });
}
