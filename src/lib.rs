pub mod ray_tracer;

use ray_tracer::{render, Settings, Scene};
use ray_tracer::sphere3d::Sphere3D;
use ray_tracer::light::Light;
use ray_tracer::ppm::Color;

pub fn run(settings: &Settings) {

    let mut sphere1 = Sphere3D::new(-1.5, 0.0, -2.5, 0.5);
    sphere1.color = Color { r: 255, g: 0, b: 0 };

    let mut sphere2 = Sphere3D::new( 0.0, 0.0, -2.5, 0.5);
    sphere2.color = Color { r: 0, g: 255, b: 0 };

    let mut sphere3 = Sphere3D::new( 1.5, 0.0, -2.5, 0.5);
    sphere3.color = Color { r: 0, g: 0, b: 255 };

    let light = Light::new(3.0, 2.0, 0.0, 200.0);

    let scene = Scene {
        objects: vec!(sphere1, sphere2, sphere3),
        light
    };

    render(&settings, &scene);
}
