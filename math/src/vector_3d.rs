use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Operators
impl ops::Index<usize> for Vector3d {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Vector3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl ops::MulAssign<f32> for Vector3d {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl ops::DivAssign<f32> for Vector3d {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl ops::AddAssign<Vector3d> for Vector3d {
    fn add_assign(&mut self, rhs: Vector3d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign<Vector3d> for Vector3d {
    fn sub_assign(&mut self, rhs: Vector3d) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<f32> for &Vector3d {
    type Output = Vector3d;

    fn mul(self, saclar: f32) -> Self::Output {
        Vector3d {
            x: self.x * saclar,
            y: self.y * saclar,
            z: self.z * saclar,
        }
    }
}

impl ops::Div<f32> for &Vector3d {
    type Output = Vector3d;

    fn div(self, saclar: f32) -> Self::Output {
        Vector3d {
            x: self.x / saclar,
            y: self.y / saclar,
            z: self.z / saclar,
        }
    }
}

impl ops::Add<Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Functions
impl Vector3d {
    fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    fn normalize(&self) -> Vector3d {
        let magnitude = self.magnitude();

        Vector3d {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }
}
