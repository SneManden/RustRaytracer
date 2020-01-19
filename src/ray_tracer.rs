pub mod vector_library;
pub mod utility;
pub mod sphere3d;
pub mod ray;
pub mod ppm;

use vector_library::{Vec3D, Point3D};
use utility::deg_2_rad;
use sphere3d::Sphere3D;
use ray::Ray;

pub struct Settings {
    pub width: u16,
    pub height: u16,
    pub fov: f32
}

impl Settings {
    fn aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }

    fn fov_rad(&self) -> f32 {
        deg_2_rad(self.fov)
    }
}

pub struct Scene {
    pub object: Sphere3D
}

pub fn render(settings: &Settings, scene: &Scene) {
    // println!(
    //     "render(w: {}, h: {}, aspect_ratio: {}, fov: {})...",
    //     settings.width,
    //     settings.height,
    //     settings.aspect_ratio(),
    //     settings.fov);

    // for fx in 0..settings.width {
    //     print!("==");
    // }
    // println!("");

    ppm::write_header(settings.width, settings.height);

    for y in 0..settings.height {
        // print!("{}: ", y + 1);
        for x in 0..settings.width {
            let p = image_coord_to_camera_space(x, y, &settings);

            let origin = Point3D::new(0.0, 0.0, 0.0);

            let ray = Ray::new(origin, &p);
            let rayDir = ray.direction();

            let intersection = nearest_intersection(&ray, &scene);

            let color = match intersection {
                Some(_) => ppm::Color::new(255, 255, 255),
                None => ppm::Color::new(0, 0, 0)
            };

            ppm::write_color(&color);
        }
        // println!("");
    }

    // for fx in 0..settings.width {
    //     print!("==");
    // }
    // println!("");
}

fn nearest_intersection<'a>(ray: &Ray, scene: &'a Scene) -> Option<(&'a Sphere3D, f32)> {
    // for each object in scene
    let object = &scene.object;

    let intersection = object.intersect(&ray);

    match intersection {
        Some(p) => {
            let dist = Vec3D::between(&ray.origin(), &p).len();
            Some((&object, dist))
        },
        None => None
    }
}

fn image_coord_to_camera_space(x: u16, y: u16, settings: &Settings) -> Point3D {
    let x = x as f32;
    let y = y as f32;
    let w = settings.width as f32;
    let h = settings.height as f32;
    let alpha = settings.fov_rad();
    let aspect_ratio = settings.aspect_ratio();

    let p_ndc_x = (x + 0.5) / w; // x:[0, w] -> (0, 1)
    let p_ndc_y = (y + 0.5) / h; // y:[0, h] -> (0, 1)

    let p_screen_x = 2.0 * p_ndc_x - 1.0; // -> (-1, 1) positive right
    let p_screen_y = 1.0 - 2.0 * p_ndc_y; // -> (1, -1) positive up

    let tana = (alpha / 2.0).tan();

    let p_camera_x = p_screen_x * aspect_ratio * tana;
    let p_camera_y = p_screen_y * tana;

    Point3D::new(p_camera_x, p_camera_y, -1.0)
}

