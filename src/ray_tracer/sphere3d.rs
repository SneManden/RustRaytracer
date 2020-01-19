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
        let E = ray.origin();
        let V = ray.direction();
        let O = &self.center;
        let EO = Vec3D::between(&E, &O);
        let v = EO.dot(&V);
        let r = &self.radius;

        let disc = r * r - (EO.dot(&EO) - v * v);

        if disc < 0.0 {
            None
        } else {
            let d = disc.sqrt();
            let P = E.on_half_line(V, v - d);
            Some(P)
        }
    }
}
