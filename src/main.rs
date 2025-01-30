mod engine;
mod renderer;

// static VERTEX_SRC: &str = include_str!("shaders/triangle-vertex.glsl");
// static FRAGMENT_SRC: &str = include_str!("shaders/triangle-fragment.glsl");

fn main() {
    let scene = engine::Scene::new("OpenGL Oxidized", engine::EARTH_GRAVITY);
    scene.execute();
}
