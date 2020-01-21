pub struct Vec3D {
    x: f32,
    y: f32,
    z: f32 
}

pub struct Point3D {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3D {
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3D {
        Vec3D { x, y, z }
    }

    pub fn dot(&self, other: &Vec3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> f32 {
        (self.dot(&self)).sqrt()
    }

    pub fn scale(&self, k: f32) -> Vec3D {
        Vec3D::new(k * self.x, k * self.y, k * self.z)
    }

    pub fn plus(&self, other: &Vec3D) -> Vec3D {
        Vec3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn minus(&self, other: &Vec3D) -> Vec3D {
        self.plus(&other.scale(-1.0))
    }

    pub fn unit(&self) -> Vec3D {
        self.scale(1.0 / self.len())
    }

    pub fn from(p: &Point3D) -> Vec3D {
        Vec3D::new(p.x, p.y, p.z)
    }

    pub fn between(p1: &Point3D, p2: &Point3D) -> Vec3D {
        let v1 = Vec3D::from(p1);
        let v2 = Vec3D::from(p2);
        v2.minus(&v1)
    }
}


impl Point3D {
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }

    pub fn new(x: f32, y: f32, z: f32) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn add(&self, other: &Point3D) -> Point3D {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Point3D) -> Point3D {
        Point3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn from(v: &Vec3D) -> Point3D {
        Point3D::new(v.x, v.y, v.z)
    }

    pub fn on_half_line(&self, direction: &Vec3D, length: f32) -> Point3D {
        let p = Point3D::from(&direction.scale(length));
        self.add(&p)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale() {
        // Arrange
        let v = Vec3D::new(3.0, 4.0, 1.0);
        let k = 2.5;

        // Act
        let kv = v.scale(k);

        // Assert
        assert_eq!(kv.x, 7.5);
        assert_eq!(kv.y, 10.0);
        assert_eq!(kv.z, 2.5);
    }

    #[test]
    fn plus() {
        // Arrange
        let v = Vec3D::new(2.0, 1.0, 3.0);
        let u = Vec3D::new(1.0, 5.0, 3.0);

        // Act
        let v_plus_u = v.plus(&u);

        // Assert
        assert_eq!(v_plus_u.x, 3.0);
        assert_eq!(v_plus_u.y, 6.0);
        assert_eq!(v_plus_u.z, 6.0);
    }

    #[test]
    fn minus() {
        // Arrange
        let v = Vec3D::new(2.0, 1.0, 3.0);
        let u = Vec3D::new(1.0, 5.0, 3.0);

        // Act
        let v_minus_u = v.minus(&u);

        // Assert
        assert_eq!(v_minus_u.x, 1.0);
        assert_eq!(v_minus_u.y, -4.0);
        assert_eq!(v_minus_u.z, 0.0);
    }

    #[test]
    fn dot() {
        // Arrange
        let v = Vec3D::new(2.0, 5.0, 3.0);
        let u = Vec3D::new(1.0, 2.0, 3.0);

        // Act
        let dot = v.dot(&u);

        // Assert
        assert_eq!(dot, 2.0 + 10.0 + 9.0);
    }

    #[test]
    fn dot_with_self() {
        // Arrange
        let v = Vec3D::new(1.0, 2.0, 3.0);

        // Act
        let dot = v.dot(&v);

        // Assert
        assert_eq!(dot, 1.0*1.0 + 2.0*2.0 + 3.0*3.0);
    }

    #[test]
    fn len_unit_vector() {
        // Arrange
        let v = Vec3D::new(1.0, 0.0, 0.0);

        // Act
        let len = v.len();

        // Assert
        assert_eq!(len, 1.0);
    }

    #[test]
    fn len() {
        // Arrange
        let v = Vec3D::new(2.0, 6.0, 3.0);

        // Act
        let len = v.len();

        // Assert
        assert_eq!(len, 7.0);
    }
}
