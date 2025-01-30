struct Scene {
    environment: Vec<Environment>,
    bodies: Vec<Body>,
    interface: Vec<UI>,

    gravity: f32,
}

struct Environment {
    friction: f32,
}

struct Body {
    mass: f32, // >= 0
    acceleration: f32,
    velocity: (f32, f32, f32), // 3D vector
}

struct UI {
    text: String,
    position: (f32, f32),
    border: bool,
    padding: f32,
}
