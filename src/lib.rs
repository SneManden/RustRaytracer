pub mod ray_tracer;

use ray_tracer::{render, Settings, Scene};
use ray_tracer::sphere3d::Sphere3D;
use ray_tracer::light::Light;

pub fn run(settings: &Settings) {

    let sphere1 = Sphere3D::new(-1.5, 0.0, -2.5, 0.5);
    let sphere2 = Sphere3D::new( 0.0, 0.0, -2.5, 0.5);
    let sphere3 = Sphere3D::new( 1.5, 0.0, -2.5, 0.5);
    let light = Light::new(3.0, 2.0, 0.0, 200.0);

    let scene = Scene {
        objects: vec!(sphere1, sphere2, sphere3),
        light
    };

    render(&settings, &scene);
}
