pub mod ray_tracer;

use ray_tracer::{render, Settings, Scene};
use ray_tracer::sphere3d::Sphere3D;

pub fn run(settings: &Settings) {

    let sphere = Sphere3D::new(0.0, 0.0, -2.0, 1.0);

    let scene = Scene {
        object: sphere
    };

    render(&settings, &scene);
}
