use renderer::Vertex;

mod engine;
mod renderer;

static VERTEX_SRC: &str = include_str!("shaders/bodies/vertex.glsl");
static FRAGMENT_SRC: &str = include_str!("shaders/bodies/fragment.glsl");

fn main() {
    let mut scene = engine::Scene::new("OpenGL Oxidized", engine::EARTH_GRAVITY);

    scene
        .add_obj(engine::Body {
            mass: 10.0,
            acceleration: 0.0,
            velocity: (2.0, 0.0, 0.0),
            size: 1.0,
            pos: Vertex::new(-0.25, -0.25),
        })
        .expect("Failed to load object");

    scene.execute();
}
