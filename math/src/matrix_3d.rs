use crate::vector_3d::Vector3d;
use std::ops;

// Row major order
#[derive(Default, Debug, Clone, Copy)]
pub struct Matrix3d {
    pub m: [[f32; 3]; 3],
}

// Constructors
impl Matrix3d {
    pub fn new(a: Vector3d, b: Vector3d, c: Vector3d) -> Self {
        Self {
            m: [[a.x, a.y, a.z], [b.x, b.y, b.z], [c.x, c.y, c.z]],
        }
    }

    pub fn identity() -> Self {
        Self {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}

// Operators
impl ops::IndexMut<usize> for Matrix3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.m[0],
            1 => &mut self.m[1],
            2 => &mut self.m[2],
            _ => panic!("Index out of range"),
        }
    }
}

impl ops::MulAssign<f32> for Matrix3d {
    fn mul_assign(&mut self, rhs: f32) {
        self.m[0][0] *= rhs;
        self.m[0][1] *= rhs;
        self.m[0][2] *= rhs;

        self.m[1][0] *= rhs;
        self.m[1][1] *= rhs;
        self.m[1][2] *= rhs;

        self.m[2][0] *= rhs;
        self.m[2][1] *= rhs;
        self.m[2][2] *= rhs;
    }
}

impl ops::AddAssign<f32> for Matrix3d {
    fn add_assign(&mut self, rhs: f32) {
        self.m[0][0] += rhs;
        self.m[0][1] += rhs;
        self.m[0][2] += rhs;

        self.m[1][0] += rhs;
        self.m[1][1] += rhs;
        self.m[1][2] += rhs;

        self.m[2][0] += rhs;
        self.m[2][1] += rhs;
        self.m[2][2] += rhs;
    }
}

impl ops::SubAssign<f32> for Matrix3d {
    fn sub_assign(&mut self, rhs: f32) {
        self.m[0][0] -= rhs;
        self.m[0][1] -= rhs;
        self.m[0][2] -= rhs;

        self.m[1][0] -= rhs;
        self.m[1][1] -= rhs;
        self.m[1][2] -= rhs;

        self.m[2][0] -= rhs;
        self.m[2][1] -= rhs;
        self.m[2][2] -= rhs;
    }
}

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

impl ops::Mul<Matrix3d> for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, rhs: Matrix3d) -> Self::Output {
        Matrix3d {
            m: [
                [
                    self.m[0][0] * rhs[0][0] + self.m[0][1] * rhs[1][0] + self.m[0][2] * rhs[2][0],
                    self.m[0][0] * rhs[0][1] + self.m[0][1] * rhs[1][1] + self.m[0][2] * rhs[2][1],
                    self.m[0][0] * rhs[0][2] + self.m[0][1] * rhs[1][2] + self.m[0][2] * rhs[2][2]
                ],
                [
                    self.m[1][0] * rhs[0][0] + self.m[1][1] * rhs[1][0] + self.m[1][2] * rhs[2][0],
                    self.m[1][0] * rhs[0][1] + self.m[1][1] * rhs[1][1] + self.m[1][2] * rhs[2][1],
                    self.m[1][0] * rhs[0][2] + self.m[1][1] * rhs[1][2] + self.m[1][2] * rhs[2][2]
                ],
                [
                    self.m[2][0] * rhs[0][0] + self.m[2][1] * rhs[1][0] + self.m[2][2] * rhs[2][0],
                    self.m[2][0] * rhs[0][1] + self.m[2][1] * rhs[1][1] + self.m[2][2] * rhs[2][1],
                    self.m[2][0] * rhs[0][2] + self.m[2][1] * rhs[1][2] + self.m[2][2] * rhs[2][2]
                ],
            ]
        }
    }
}

impl ops::Mul<Vector3d> for Matrix3d {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.m[0][0] * rhs.x + self.m[0][1] * rhs.y + self.m[0][2] * rhs.z,
            y: self.m[1][0] * rhs.x + self.m[1][1] * rhs.y + self.m[1][2] * rhs.z,
            z: self.m[2][0] * rhs.x + self.m[2][1] * rhs.y + self.m[2][2] * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix3d { m: {
                [
                    [self.m[0][0] * rhs, self.m[0][1] * rhs, self.m[0][2] * rhs],
                    [self.m[1][0] * rhs, self.m[1][1] * rhs, self.m[1][2] * rhs],
                    [self.m[2][0] * rhs, self.m[2][1] * rhs, self.m[2][2] * rhs],
                ]
            }
        }
    }
}

impl ops::Add<f32> for Matrix3d {
    type Output = Matrix3d;

    fn add(self, rhs: f32) -> Self::Output {
        Matrix3d { m: {
                [
                    [self.m[0][0] + rhs, self.m[0][1] + rhs, self.m[0][2] + rhs],
                    [self.m[1][0] + rhs, self.m[1][1] + rhs, self.m[1][2] + rhs],
                    [self.m[2][0] + rhs, self.m[2][1] + rhs, self.m[2][2] + rhs],
                ]
            }
        }
    }
}

impl ops::Sub<f32> for Matrix3d {
    type Output = Matrix3d;

    fn sub(self, rhs: f32) -> Self::Output {
        Matrix3d { m: {
                [
                    [self.m[0][0] - rhs, self.m[0][1] - rhs, self.m[0][2] - rhs],
                    [self.m[1][0] - rhs, self.m[1][1] - rhs, self.m[1][2] - rhs],
                    [self.m[2][0] - rhs, self.m[2][1] - rhs, self.m[2][2] - rhs],
                ]
            }
        }
    }
}
