use crate::point2d::Point2D;

pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn translate_x(&self, delta: f32) -> Self {
        Self {
            x: self.x + delta,
            y: self.y,
            z: self.z,
        }
    }

    pub fn translate_y(&self, delta: f32) -> Self {
        Self {
            x: self.x,
            y: self.y + delta,
            z: self.z,
        }
    }

    pub fn translate_z(&self, delta: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + delta,
        }
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        let c = f32::cos(angle.to_radians());
        let s = f32::sin(angle.to_radians());

        Self {
            x: self.x,
            y: self.y * c - self.z * s,
            z: self.y * s + self.z * c,
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        let c = f32::cos(angle.to_radians());
        let s = f32::sin(angle.to_radians());

        Self {
            x: self.x * c - self.z * s,
            y: self.y,
            z: self.x * s + self.z * c,
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        let c = f32::cos(angle.to_radians());
        let s = f32::sin(angle.to_radians());

        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
            z: self.z,
        }
    }

    pub fn project(&self) -> Point2D {
        match self.z {
            0.0 => Point2D::new(0.0, 0.0),
            _ => Point2D::new(self.x / self.z, self.y / self.z),
        }
    }
}
