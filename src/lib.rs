pub mod ray_tracer;

use ray_tracer::{render, Settings, Scene, Sphere3D};

use std::{thread, time};

pub fn run() {
    let settings = Settings {
        width: 48,
        height: 32,
        fov: 90.0
    };

    let sphere = Sphere3D::new(0.0, 0.0, -2.0, 1.0);

    let scene = Scene {
        object: sphere
    };

    render(&settings, &scene);
}
