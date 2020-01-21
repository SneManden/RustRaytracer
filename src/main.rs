use rust_raytracer::run;

use rust_raytracer::ray_tracer::Settings;

fn main() {
    let settings = Settings {
        width: 320,
        height: 240,
        fov: 90.0,
        ambient_light: 0.2
    };
    run(&settings);
}
