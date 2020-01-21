use super::vector_library::{Point3D, Vec3D};
use super::ray::Ray;

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

    pub fn translate(&self, tx: f32, ty: f32, tz: f32) -> Sphere3D {
        let c = &self.center;
        Sphere3D::new(c.x() + tx, c.y() + ty, c.z() + tz, self.radius)
    }

    // www.cs.unc.edu/~rademach/xroads-RT/RTarticle.html
    pub fn intersect(&self, ray: &Ray) -> Option<Point3D> {
        let point_e = ray.origin();
        let vec_v = ray.direction();
        let point_o = &self.center;
        let vec_eo = Vec3D::between(&point_e, &point_o);
        let val = vec_eo.dot(&vec_v);
        let r = &self.radius;

        let disc = r * r - (vec_eo.dot(&vec_eo) - val * val);

        if disc < 0.0 {
            None
        } else {
            let d = disc.sqrt();
            let point_p = point_e.on_half_line(vec_v, val - d);
            Some(point_p)
        }
    }

    pub fn get_normal(&self, point: &Point3D) -> Vec3D {
        Vec3D::between(&self.center, &point).scale(1.0 / self.radius)
    }
}
