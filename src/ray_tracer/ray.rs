use super::vector_library::{Point3D, Vec3D};

pub struct Ray {
    origin: Point3D,
    direction: Vec3D // Normalized
}

impl Ray {
    pub fn new(origin: Point3D, target: &Point3D) -> Ray {
        let direction = Vec3D::between(&origin, &target).unit();
        Ray { origin: origin, direction }
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3D {
        &self.direction
    }

    pub fn point_at(&self, distance: f32) -> Point3D {
        self.origin().add(&Point3D::from(&self.direction.scale(distance)))
    }
}
