pub mod vector_library;

use std::f32::consts::PI;
use vector_library::{Vec3D, Point3D};

struct Ray {
    origin: Point3D,
    direction: Vec3D // Normalized
}

impl Ray {
    fn new(origin: Point3D, target: &Point3D) -> Ray {
        let direction = Vec3D::between(&origin, &target).unit();
        Ray { origin: origin, direction }
    }

    fn origin(&self) -> &Point3D {
        &self.origin
    }

    fn direction(&self) -> &Vec3D {
        &self.direction
    }
}

struct Point2D {
    x: f32,
    y: f32
}

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
    println!(
        "render(w: {}, h: {}, aspect_ratio: {}, fov: {})...",
        settings.width,
        settings.height,
        settings.aspect_ratio(),
        settings.fov);

    for x in 0..settings.width {
        for y in 0..settings.height {
            
            let p = image_coord_to_camera_space(x, y, &settings);

            println!("Pixel ({}, {}) -> ({}, {}, {})", x, y, p.x(), p.y(), p.z());

            let origin = Point3D::new(0.0, 0.0, 0.0);

            let ray = Ray::new(origin, &p);

            let rayDir = ray.direction();
            println!(" --> ray with direction ({}, {}, {})", rayDir.x(), rayDir.y(), rayDir.z());

            let intersection = nearest_intersection(&ray, &scene);

            match intersection {
                Some(_) => println!(" --> color = {}", "white"),
                None => println!(" --> color = {}", "black")
            }
        }
    }
    println!("=> Done.");
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

fn deg_2_rad(deg: f32) -> f32 {
    PI / 180.0 * deg
}

fn rad_2_deg(rad: f32) -> f32 {
    rad * 180.0 / PI
}

fn image_coord_to_camera_space(x: u16, y: u16, settings: &Settings) -> Point3D {
    let x = x as f32;
    let y = y as f32;
    let w = settings.width as f32;
    let h = settings.height as f32;
    let alpha = settings.fov_rad();
    let aspect_ratio = settings.aspect_ratio();

    // println!("  i_c_t_c_s(x:{}, y:{})", x, y);

    let p_ndc_x = (x + 0.5) / w; // x:[0, w] -> (0, 1)
    let p_ndc_y = (y + 0.5) / h; // y:[0, h] -> (0, 1)

    // println!("  --> p_ndc ({}, {})", p_ndc_x, p_ndc_y);

    let p_screen_x = 2.0 * p_ndc_x - 1.0; // -> (-1, 1) positive right
    let p_screen_y = 1.0 - 2.0 * p_ndc_y; // -> (1, -1) positive up

    // println!("  --> p_screen ({}, {})", p_screen_x, p_screen_y);

    let tana = (alpha / 2.0).tan();

    // println!("  tan(alpha / 2) = {}", tana);

    let p_camera_x = p_screen_x * aspect_ratio * tana;
    let p_camera_y = p_screen_y * tana;

    Point3D::new(p_camera_x, p_camera_y, -1.0)
}

enum Object {
    Sphere(Sphere3D)
}

pub struct Sphere3D {
    pub center: Point3D,
    pub radius: f32
}

impl Sphere3D {
    pub fn new(cx: f32, cy: f32, cz: f32, radius: f32) -> Sphere3D {
        Sphere3D {
            center: Point3D::new(cx, cy, cz),
            radius
        }
    }

    // www.cs.unc.edu/~rademach/xroads-RT/RTarticle.html
    fn intersect(&self, ray: &Ray) -> Option<Point3D> {
        let E = ray.origin();
        let V = ray.direction();
        let O = &self.center;
        let EO = Vec3D::between(&E, &O);
        let v = EO.dot(&V);

        let disc = EO.dot(&EO) - v * v;

        if disc < 0.0 {
            None
        } else {
            let d = disc.sqrt();
            let P = E.on_half_line(V, v - d);
            Some(P)
        }
    }
}
