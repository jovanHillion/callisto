use crate::vector_3d::Vector3d;
use std::ops;

// Column major order
#[derive(Default, Debug, Clone, Copy)]
pub struct Matrix3d {
    pub m: [[f32; 3]; 3],
}

// Constructors
impl Matrix3d {
    pub fn new(a: Vector3d, b: Vector3d, c:Vector3d) -> Self {
        Self { m:
            [
                [a.x, a.y, a.z],
                [b.x, b.y, b.z],
                [c.x, c.y, c.z]
            ]
        }
    }

    pub fn identity() -> Self {
        Self { m:
            [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        }
    }
}

// Operators
impl ops::Index<usize> for Matrix3d {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.m[0],
            1 => &self.m[1],
            2 => &self.m[2],
            _ => panic!("Index out of range"),
        }
    }
}
