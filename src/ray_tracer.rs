pub mod vector_library;
pub mod utility;
pub mod sphere3d;
pub mod light;
pub mod ray;
pub mod ppm;

use vector_library::{Vec3D, Point3D};
use utility::deg_2_rad;
use sphere3d::Sphere3D;
use ray::Ray;
use light::Light;

use std::f32::consts::PI;

pub struct Settings {
    pub width: u16,
    pub height: u16,
    pub fov: f32,

    pub ambient_light: f32// ambient coefficient
}

impl Settings {
    fn aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }

    fn fov_rad(&self) -> f32 {
        deg_2_rad(self.fov)
    }

    fn ambient_coefficient(&self) -> f32 {
        self.ambient_light
    }

    fn diffuse_coefficient(&self) -> f32 {
        1.0 - self.ambient_coefficient()
    }
}

pub struct Scene {
    pub objects: Vec<Sphere3D>,
    pub light: Light
}

pub fn render(settings: &Settings, scene: &Scene) {
    ppm::write_header(settings.width, settings.height);

    for y in 0..settings.height {
        for x in 0..settings.width {
            let p = image_coord_to_camera_space(x, y, &settings);

            let origin = Point3D::new(0.0, 0.0, 0.0);

            let ray = Ray::new(origin, &p);

            let intersection = nearest_intersection(&ray, &scene);

            let color = if let Some((obj, dist)) = intersection {
                let intersection_point = ray.point_at(dist);  //ray.origin().plus(&ray_dir.scale(dist));
                let surface_normal = obj.get_normal(&intersection_point);

                let light = &scene.light;
                let ray_to_light = Vec3D::between(&intersection_point, &light.center).unit();

                // L = 1/pi cos(angle) I / r^2
                let cos_angle = surface_normal.dot(&ray_to_light).max(0.0);

                let reflected_light = (1.0 / PI) * cos_angle * (light.intensity / ray_to_light.len().powi(2));

                let val = reflected_light.min(255.0).max(0.0);

                // let pixel_color = ((32.0 * settings.ambient_coefficient()) + (settings.diffuse_coefficient() * val)) as u8;

                set_color(val, &obj, &settings)
                // ppm::Color::new(pixel_color, pixel_color, pixel_color)
            } else {
                ppm::Color::new(0, 0, 0)
            };

            ppm::write_color(&color).expect("something wrong when writing color");
        }
    }
}

// value: [0, 255]
fn set_color(value: f32, sphere: &Sphere3D, settings: &Settings) -> ppm::Color {
    let a = settings.ambient_coefficient();
    let d = settings.diffuse_coefficient();
    let intensity = (((32.0 * a) + (d * value))) / 255.0;
    ppm::Color::new(
        (intensity * sphere.color.r as f32) as u8,
        (intensity * sphere.color.g as f32) as u8,
        (intensity * sphere.color.b as f32) as u8
    )
}

fn nearest_intersection<'a>(ray: &Ray, scene: &'a Scene) -> Option<(&'a Sphere3D, f32)> {
    let mut nearest: Option<(&Sphere3D, f32)> = None;

    for object in &scene.objects {
        let intersection = object.intersect(&ray);
        nearest = match intersection {
            Some(p) => {
                let dist = Vec3D::between(&ray.origin(), &p).len();
                match nearest {
                    Some((nobj, ndist)) => {
                        if dist < ndist {
                            Some((object, dist))
                        } else {
                            Some((nobj, ndist))
                        }
                    }
                    None => Some((object, dist))
                }
            },
            None => nearest
        }
    }
    nearest
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn nearest_intersection1() {
        // Arrange
        let sphere1 = Sphere3D::new(0.0, 0.0, -3.0, 1.0);
        let sphere2 = Sphere3D::new(0.0, 0.0, -5.0, 1.0);
        let scene = Scene {
            objects: vec!(sphere1, sphere2),
            light: Light::new(2.0, 1.0, 0.0, 200.0)
        };
        let o = Point3D::zero();
        let t = Point3D::new(0.0, 0.0, -1.0);
        let ray = Ray::new(o, &t);

        // Act
        let (sphere, dist) = nearest_intersection(&ray, &scene).expect("should yield intersection");

        // Assert
        assert!(ptr::eq(sphere, &scene.objects[0]));
        assert_eq!(dist, 2.0);
    }

    #[test]
    fn nearest_intersection2() {
        // Arrange
        let sphere1 = Sphere3D::new(0.0, 0.0, -3.0, 1.0);
        let sphere2 = Sphere3D::new(0.0, 0.0, -5.0, 1.0);
        let scene = Scene {
            objects: vec!(sphere2, sphere1),
            light: Light::new(2.0, 1.0, 0.0, 200.0)
        };
        let o = Point3D::zero();
        let t = Point3D::new(0.0, 0.0, -1.0);
        let ray = Ray::new(o, &t);

        // Act
        let (sphere, dist) = nearest_intersection(&ray, &scene).expect("should yield intersection");

        // Assert
        assert!(ptr::eq(sphere, &scene.objects[1]));
        assert_eq!(dist, 2.0);
    }

    #[test]
    fn nearest_intersection_side_by_side_nearest_is_first_in_vec() {
        // Arrange
        let sphere1 = Sphere3D::new( 0.0, 0.0, -3.0, 1.0);
        let sphere2 = Sphere3D::new( 2.0, 0.0, -3.0, 1.0);
        let scene = Scene {
            objects: vec!(sphere1, sphere2),
            light: Light::new(2.0, 1.0, 0.0, 200.0)
        };
        let o = Point3D::zero();
        let t = Point3D::new(0.0, 0.0, -1.0);
        let ray = Ray::new(o, &t); // Shooting right at sphere1 (left)

        // Act
        let (sphere, dist) = nearest_intersection(&ray, &scene).expect("should yield intersection");

        // Assert
        assert!(ptr::eq(sphere, &scene.objects[0]));
        assert_eq!(dist, 2.0);
    }


    #[test]
    fn nearest_intersection_side_by_side_nearest_is_last_in_vec() {
        // Arrange
        let sphere1 = Sphere3D::new( 0.0, 0.0, -3.0, 1.0);
        let sphere2 = Sphere3D::new( 2.0, 0.0, -3.0, 1.0);
        let scene = Scene {
            objects: vec!(sphere2, sphere1),
            light: Light::new(2.0, 1.0, 0.0, 200.0)
        };
        let o = Point3D::zero();
        let t = Point3D::new(0.0, 0.0, -1.0);
        let ray = Ray::new(o, &t); // Shooting right at sphere1 (left)

        // Act
        let (sphere, dist) = nearest_intersection(&ray, &scene).expect("should yield intersection");

        // Assert
        assert!(ptr::eq(sphere, &scene.objects[1]));
        assert_eq!(dist, 2.0);
    }
}
