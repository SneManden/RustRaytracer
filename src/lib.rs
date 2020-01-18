pub mod ray_tracer;

use ray_tracer::{render, Settings, Scene, Sphere3D};

pub fn run() {
    let settings = Settings {
        width: 160,
        height: 120,
        fov: 90.0
    };

    let scene = Scene {
        object: Sphere3D::new(0.0, 0.0, -3.0, 1.0)
    };

    render(&settings, &scene);
}
