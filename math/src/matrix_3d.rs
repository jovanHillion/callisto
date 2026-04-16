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

    pub fn new_from_float(
        a0: f32,
        a1: f32,
        a2: f32,
        b0: f32,
        b1: f32,
        b2: f32,
        c0: f32,
        c1: f32,
        c2: f32,
    ) -> Self {
        Self {
            m: [[a0, a1, a2], [b0, b1, b2], [c0, c1, c2]],
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
                    self.m[0][0] * rhs[0][2] + self.m[0][1] * rhs[1][2] + self.m[0][2] * rhs[2][2],
                ],
                [
                    self.m[1][0] * rhs[0][0] + self.m[1][1] * rhs[1][0] + self.m[1][2] * rhs[2][0],
                    self.m[1][0] * rhs[0][1] + self.m[1][1] * rhs[1][1] + self.m[1][2] * rhs[2][1],
                    self.m[1][0] * rhs[0][2] + self.m[1][1] * rhs[1][2] + self.m[1][2] * rhs[2][2],
                ],
                [
                    self.m[2][0] * rhs[0][0] + self.m[2][1] * rhs[1][0] + self.m[2][2] * rhs[2][0],
                    self.m[2][0] * rhs[0][1] + self.m[2][1] * rhs[1][1] + self.m[2][2] * rhs[2][1],
                    self.m[2][0] * rhs[0][2] + self.m[2][1] * rhs[1][2] + self.m[2][2] * rhs[2][2],
                ],
            ],
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
        Matrix3d {
            m: {
                [
                    [self.m[0][0] * rhs, self.m[0][1] * rhs, self.m[0][2] * rhs],
                    [self.m[1][0] * rhs, self.m[1][1] * rhs, self.m[1][2] * rhs],
                    [self.m[2][0] * rhs, self.m[2][1] * rhs, self.m[2][2] * rhs],
                ]
            },
        }
    }
}

impl ops::Add<f32> for Matrix3d {
    type Output = Matrix3d;

    fn add(self, rhs: f32) -> Self::Output {
        Matrix3d {
            m: {
                [
                    [self.m[0][0] + rhs, self.m[0][1] + rhs, self.m[0][2] + rhs],
                    [self.m[1][0] + rhs, self.m[1][1] + rhs, self.m[1][2] + rhs],
                    [self.m[2][0] + rhs, self.m[2][1] + rhs, self.m[2][2] + rhs],
                ]
            },
        }
    }
}

impl ops::Sub<f32> for Matrix3d {
    type Output = Matrix3d;

    fn sub(self, rhs: f32) -> Self::Output {
        Matrix3d {
            m: {
                [
                    [self.m[0][0] - rhs, self.m[0][1] - rhs, self.m[0][2] - rhs],
                    [self.m[1][0] - rhs, self.m[1][1] - rhs, self.m[1][2] - rhs],
                    [self.m[2][0] - rhs, self.m[2][1] - rhs, self.m[2][2] - rhs],
                ]
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector_3d::Vector3d;

    // Helpers ------------------------------------------------------------
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6 // We are cheking if flaots are equal with a precision of "1e-6" -> 0.000001
    }

    fn mat_approx_eq(a: Matrix3d, b: Matrix3d) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !approx_eq(a[i][j], b[i][j]) {
                    return false;
                }
            }
        }
        true
    }

    fn vec_approx_eq(a: Vector3d, b: Vector3d) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    fn vec(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d { x, y, z }
    }

    // Constructors ------------------------------------------------------------
    #[test]
    fn test_default_is_all_zeros() {
        let m = Matrix3d::default();
        assert!(mat_approx_eq(
            m,
            Matrix3d::new_from_float(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
        ));
    }

    #[test]
    fn test_new_rows_match_vectors() {
        let m = Matrix3d::new(vec(1.0, 2.0, 3.0), vec(4.0, 5.0, 6.0), vec(7.0, 8.0, 9.0));
        assert!(mat_approx_eq(
            m,
            Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,)
        ));
    }

    #[test]
    fn test_identity_diagonal_is_one() {
        let m = Matrix3d::identity();
        assert!(approx_eq(m[0][0], 1.0));
        assert!(approx_eq(m[1][1], 1.0));
        assert!(approx_eq(m[2][2], 1.0));
    }

    #[test]
    fn test_identity_off_diagonal_is_zero() {
        let m = Matrix3d::identity();
        assert!(approx_eq(m[0][1], 0.0));
        assert!(approx_eq(m[0][2], 0.0));
        assert!(approx_eq(m[1][0], 0.0));
        assert!(approx_eq(m[1][2], 0.0));
        assert!(approx_eq(m[2][0], 0.0));
        assert!(approx_eq(m[2][1], 0.0));
    }

    // Index ------------------------------------------------------------
    #[test]
    fn test_index_row_0() {
        let m = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m[0], [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_index_row_1() {
        let m = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m[1], [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_index_row_2() {
        let m = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m[2], [7.0, 8.0, 9.0]);
    }

    #[test]
    fn test_index_element_access() {
        let m = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert!(approx_eq(m[1][2], 6.0));
    }

    #[test]
    #[should_panic(expected = "Index out of range")]
    fn test_index_out_of_range() {
        let m = Matrix3d::identity();
        let _ = m[3];
    }

    // Index Mut ------------------------------------------------------------
    #[test]
    fn test_index_mut_row_0() {
        let mut m = Matrix3d::default();
        m[0][1] = 42.0;
        assert!(approx_eq(m[0][1], 42.0));
    }

    #[test]
    fn test_index_mut_row_1() {
        let mut m = Matrix3d::default();
        m[1][0] = 7.0;
        assert!(approx_eq(m[1][0], 7.0));
    }

    #[test]
    fn test_index_mut_row_2() {
        let mut m = Matrix3d::default();
        m[2][2] = 99.0;
        assert!(approx_eq(m[2][2], 99.0));
    }

    #[test]
    #[should_panic(expected = "Index out of range")]
    fn test_index_mut_out_of_range() {
        let mut m = Matrix3d::identity();
        m[3][0] = 1.0;
    }

    // MulAssign<f32> ------------------------------------------------------------
    #[test]
    fn test_mul_assign_scalar_basic() {
        let mut m = Matrix3d::identity();
        m *= 2.0;
        assert!(mat_approx_eq(
            m,
            Matrix3d::new_from_float(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0,)
        ));
    }

    #[test]
    fn test_mul_assign_scalar_zero() {
        let mut m = Matrix3d::identity();
        m *= 0.0;
        assert!(mat_approx_eq(m, Matrix3d::default()));
    }

    #[test]
    fn test_mul_assign_scalar_one() {
        let mut m = Matrix3d::identity();
        m *= 1.0;
        assert!(mat_approx_eq(m, Matrix3d::identity()));
    }

    #[test]
    fn test_mul_assign_scalar_negative() {
        let mut m = Matrix3d::identity();
        m *= -1.0;
        assert!(mat_approx_eq(
            m,
            Matrix3d::new_from_float(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0,)
        ));
    }

    // AddAssign<f32> ------------------------------------------------------------
    #[test]
    fn test_add_assign_scalar_basic() {
        let mut m = Matrix3d::default();
        m += 1.0;
        assert!(mat_approx_eq(
            m,
            Matrix3d::new_from_float(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,)
        ));
    }

    #[test]
    fn test_add_assign_scalar_zero() {
        let mut m = Matrix3d::identity();
        m += 0.0;
        assert!(mat_approx_eq(m, Matrix3d::identity()));
    }

    #[test]
    fn test_add_assign_scalar_negative() {
        let mut m = Matrix3d::default();
        m += -3.0;
        assert!(mat_approx_eq(
            m,
            Matrix3d::new_from_float(-3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0,)
        ));
    }

    // SubAssign<f32> ------------------------------------------------------------
    #[test]
    fn test_sub_assign_scalar_basic() {
        let mut m = Matrix3d::new_from_float(5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0);
        m -= 2.0;
        assert!(mat_approx_eq(
            m,
            Matrix3d::new_from_float(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,)
        ));
    }

    #[test]
    fn test_sub_assign_scalar_zero() {
        let mut m = Matrix3d::identity();
        m -= 0.0;
        assert!(mat_approx_eq(m, Matrix3d::identity()));
    }

    #[test]
    fn test_sub_assign_scalar_cancels_add() {
        let mut m = Matrix3d::identity();
        m += 5.0;
        m -= 5.0;
        assert!(mat_approx_eq(m, Matrix3d::identity()));
    }

    // Mul<f32> ------------------------------------------------------------
    #[test]
    fn test_mul_matrix_identity_left() {
        let a = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let result = Matrix3d::identity() * a;
        assert!(mat_approx_eq(result, a));
    }

    #[test]
    fn test_mul_matrix_identity_right() {
        let a = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let result = a * Matrix3d::identity();
        assert!(mat_approx_eq(result, a));
    }

    #[test]
    fn test_mul_matrix_zero_left() {
        let a = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let result = Matrix3d::default() * a;
        assert!(mat_approx_eq(result, Matrix3d::default()));
    }

    #[test]
    fn test_mul_matrix_zero_right() {
        let a = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let result = a * Matrix3d::default();
        assert!(mat_approx_eq(result, Matrix3d::default()));
    }

    #[test]
    fn test_mul_matrix_known_result() {
        // [1 2]   [5 6]   [1 * 5 + 2 * 7  1 * 6 + 2 * 8]   [19 22]
        // [3 4] * [7 8] = [3 * 5 + 4 * 7  3 * 6 + 4 * 8] = [43 50]
        // Extended to 3x3 with zeros on third row/col
        let a = Matrix3d::new_from_float(1.0, 2.0, 0.0, 3.0, 4.0, 0.0, 0.0, 0.0, 1.0);
        let b = Matrix3d::new_from_float(5.0, 6.0, 0.0, 7.0, 8.0, 0.0, 0.0, 0.0, 1.0);
        let result = a * b;
        assert!(mat_approx_eq(
            result,
            Matrix3d::new_from_float(19.0, 22.0, 0.0, 43.0, 50.0, 0.0, 0.0, 0.0, 1.0,)
        ));
    }

    #[test]
    fn test_mul_matrix_not_commutative() {
        let a = Matrix3d::new_from_float(1.0, 2.0, 0.0, 3.0, 4.0, 0.0, 0.0, 0.0, 1.0);
        let b = Matrix3d::new_from_float(0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let ab = a * b;
        let ba = b * a;
        // AB ≠ BA in general
        assert!(!mat_approx_eq(ab, ba));
    }

     #[test]
    fn test_mul_scalar_basic() {
        let m = Matrix3d::identity();
        let result = m * 3.0;
        assert!(mat_approx_eq(
            result,
            Matrix3d::new_from_float(3.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 3.0,)
        ));
    }

    #[test]
    fn test_mul_scalar_zero() {
        let m = Matrix3d::identity();
        let result = m * 0.0;
        assert!(mat_approx_eq(result, Matrix3d::default()));
    }

    #[test]
    fn test_mul_scalar_one() {
        let m = Matrix3d::identity();
        let result = m * 1.0;
        assert!(mat_approx_eq(result, Matrix3d::identity()));
    }

    #[test]
    fn test_mul_scalar_negative() {
        let m = Matrix3d::identity();
        let result = m * -2.0;
        assert!(mat_approx_eq(
            result,
            Matrix3d::new_from_float(-2.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, -2.0,)
        ));
    }

    #[test]
    fn test_mul_scalar_does_not_mutate_original() {
        let m = Matrix3d::identity();
        let _ = m * 5.0;
        assert!(mat_approx_eq(m, Matrix3d::identity()));
    }

    // Mul<Vector3d> ------------------------------------------------------------
    #[test]
    fn test_mul_vector_identity() {
        let v = vec(1.0, 2.0, 3.0);
        let result = Matrix3d::identity() * v;
        assert!(vec_approx_eq(result, v));
    }

    #[test]
    fn test_mul_vector_zero_matrix() {
        let v = vec(1.0, 2.0, 3.0);
        let result = Matrix3d::default() * v;
        assert!(vec_approx_eq(result, vec(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_mul_vector_scale() {
        // Diagonal matrix scales each component
        let m = Matrix3d::new_from_float(2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0);
        let v = vec(1.0, 1.0, 1.0);
        let result = m * v;
        assert!(vec_approx_eq(result, vec(2.0, 3.0, 4.0)));
    }

    #[test]
    fn test_mul_vector_known_result() {
        let m = Matrix3d::new_from_float(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let v = vec(1.0, 0.0, 0.0);
        // First column of m
        let result = m * v;
        assert!(vec_approx_eq(result, vec(1.0, 4.0, 7.0)));
    }

    #[test]
    fn test_mul_vector_rotation_90_around_z() {
        // 90° rotation around Z maps (1,0,0) → (0,1,0)
        let m = Matrix3d::new_from_float(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let v = vec(1.0, 0.0, 0.0);
        let result = m * v;
        assert!(vec_approx_eq(result, vec(0.0, 1.0, 0.0)));
    }

    // Add<f32> ------------------------------------------------------------
    #[test]
    fn test_add_scalar_basic() {
        let m = Matrix3d::default();
        let result = m + 1.0;
        assert!(mat_approx_eq(
            result,
            Matrix3d::new_from_float(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,)
        ));
    }

    #[test]
    fn test_add_scalar_zero() {
        let m = Matrix3d::identity();
        let result = m + 0.0;
        assert!(mat_approx_eq(result, Matrix3d::identity()));
    }

    #[test]
    fn test_add_scalar_negative() {
        let m = Matrix3d::default();
        let result = m + (-2.0);
        assert!(mat_approx_eq(
            result,
            Matrix3d::new_from_float(-2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0,)
        ));
    }

    #[test]
    fn test_add_scalar_does_not_mutate_original() {
        let m = Matrix3d::identity();
        let _ = m + 5.0;
        assert!(mat_approx_eq(m, Matrix3d::identity()));
    }

    // Sub<f32> ------------------------------------------------------------
    #[test]
    fn test_sub_scalar_basic() {
        let m = Matrix3d::new_from_float(5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0);
        let result = m - 2.0;
        assert!(mat_approx_eq(
            result,
            Matrix3d::new_from_float(3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,)
        ));
    }

    #[test]
    fn test_sub_scalar_zero() {
        let m = Matrix3d::identity();
        let result = m - 0.0;
        assert!(mat_approx_eq(result, Matrix3d::identity()));
    }

    #[test]
    fn test_sub_scalar_cancels_add() {
        let m = Matrix3d::identity();
        let result = (m + 5.0) - 5.0;
        assert!(mat_approx_eq(result, Matrix3d::identity()));
    }

    #[test]
    fn test_sub_scalar_does_not_mutate_original() {
        let m = Matrix3d::identity();
        let _ = m - 1.0;
        assert!(mat_approx_eq(m, Matrix3d::identity()));
    }
}
