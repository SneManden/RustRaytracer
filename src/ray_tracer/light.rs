use super::vector_library::Point3D;

pub struct Light {
    pub center: Point3D,
    pub intensity: f32
}

impl Light {
    pub fn new(cx: f32, cy: f32, cz: f32, intensity: f32) -> Light {
        Light {
            center: Point3D::new(cx, cy, cz),
            intensity
        }
    }
}
