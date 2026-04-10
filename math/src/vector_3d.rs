use std::ops;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// From
impl From<[f32; 3]> for Vector3d {
    fn from(value: [f32; 3]) -> Self {
        Vector3d { x: value[0], y: value[1], z: value[2] }
    }
}

impl From<&mut [f32; 3]> for Vector3d {
    fn from(value: &mut [f32; 3]) -> Self {
        Vector3d { x: value[0], y: value[1], z: value[2] }
    }
}

// Operators
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
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f32> for Vector3d {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
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

impl ops::Mul<f32> for Vector3d {
    type Output = Vector3d;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vector3d {
    type Output = Vector3d;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    #[inline]
    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    #[inline]
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
    #[inline]
    fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    #[inline]
    fn normalize(&self) -> Vector3d {
        self.clone() / self.magnitude()
    }

    #[inline]
    fn dot_product(a: Vector3d, b: Vector3d) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z as f32
    }
}

// Tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Helpers ------------------------------------------------------------
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6 // We are cheking if flaots are equal with a precision of "1e-6" -> 0.000001
    }

    fn vec_approx_eq(a: Vector3d, b: Vector3d) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    // Index ------------------------------------------------------------
    #[test]
    fn test_index_x() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v[0], 1.0);
    }

    #[test]
    fn test_index_y() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v[1], 2.0);
    }

    #[test]
    fn test_index_z() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v[2], 3.0);
    }

    #[test]
    #[should_panic(expected = "Index out of range")]
    fn test_index_out_of_range() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let _ = v[3];
    }

    // Mutable Index ------------------------------------------------------------
    #[test]
    fn test_index_mut_x() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v[0] = 10.0;
        assert_eq!(v.x, 10.0);
    }

    #[test]
    fn test_index_mut_y() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v[1] = 20.0;
        assert_eq!(v.y, 20.0);
    }

    #[test]
    fn test_index_mut_z() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v[2] = 30.0;
        assert_eq!(v.z, 30.0);
    }

    #[test]
    #[should_panic(expected = "Index out of range")]
    fn test_index_mut_out_of_range() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v[3] = 99.0;
    }

    // Mul Assign ------------------------------------------------------------
    #[test]
    fn test_mul_assign_positive_scalar() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v *= 2.0;
        assert!(vec_approx_eq(v, Vector3d { x: 2.0, y: 4.0, z: 6.0 }));
    }

    #[test]
    fn test_mul_assign_zero() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v *= 0.0;
        assert!(vec_approx_eq(v, Vector3d { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_mul_assign_negative_scalar() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v *= -1.0;
        assert!(vec_approx_eq(v, Vector3d { x: -1.0, y: -2.0, z: -3.0 }));
    }

    #[test]
    fn test_mul_assign_one() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v *= 1.0;
        assert!(vec_approx_eq(v, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    // Div Assign ------------------------------------------------------------
    #[test]
    fn test_div_assign_positive_scalar() {
        let mut v = Vector3d { x: 2.0, y: 4.0, z: 6.0 };
        v /= 2.0;
        assert!(vec_approx_eq(v, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    #[test]
    fn test_div_assign_one() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v /= 1.0;
        assert!(vec_approx_eq(v, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    #[test]
    fn test_div_assign_negative_scalar() {
        let mut v = Vector3d { x: 2.0, y: 4.0, z: 6.0 };
        v /= -2.0;
        assert!(vec_approx_eq(v, Vector3d { x: -1.0, y: -2.0, z: -3.0 }));
    }

    #[test]
    fn test_div_assign_by_zero_produces_infinity() {
        // Rust f32 division by zero yields infinity, not a panic
        let mut v = Vector3d { x: 1.0, y: 1.0, z: 1.0 };
        v /= 0.0;
        assert!(v.x.is_infinite() && v.y.is_infinite() && v.z.is_infinite());
    }

    // Add Assign ------------------------------------------------------------
    #[test]
    fn test_add_assign_basic() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v += Vector3d { x: 4.0, y: 5.0, z: 6.0 };
        assert!(vec_approx_eq(v, Vector3d { x: 5.0, y: 7.0, z: 9.0 }));
    }

    #[test]
    fn test_add_assign_zero_vector() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v += Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        assert!(vec_approx_eq(v, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    #[test]
    fn test_add_assign_negative() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v += Vector3d { x: -1.0, y: -2.0, z: -3.0 };
        assert!(vec_approx_eq(v, Vector3d { x: 0.0, y: 0.0, z: 0.0 }));
    }

    // Sub Assign ------------------------------------------------------------
    #[test]
    fn test_sub_assign_basic() {
        let mut v = Vector3d { x: 5.0, y: 7.0, z: 9.0 };
        v -= Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        assert!(vec_approx_eq(v, Vector3d { x: 4.0, y: 5.0, z: 6.0 }));
    }

    #[test]
    fn test_sub_assign_zero_vector() {
        let mut v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        v -= Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        assert!(vec_approx_eq(v, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    #[test]
    fn test_sub_assign_self_gives_zero() {
        let mut v = Vector3d { x: 3.0, y: 5.0, z: 7.0 };
        v -= Vector3d { x: 3.0, y: 5.0, z: 7.0 };
        assert!(vec_approx_eq(v, Vector3d { x: 0.0, y: 0.0, z: 0.0 }));
    }

    // Mul ------------------------------------------------------------
    #[test]
    fn test_mul_ref_basic() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = v * 3.0;
        assert!(vec_approx_eq(result, Vector3d { x: 3.0, y: 6.0, z: 9.0 }));
    }

    #[test]
    fn test_mul_ref_zero() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = v * 0.0;
        assert!(vec_approx_eq(result, Vector3d { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_mul_ref_negative() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = v * -2.0;
        assert!(vec_approx_eq(result, Vector3d { x: -2.0, y: -4.0, z: -6.0 }));
    }

    #[test]
    fn test_mul_ref_does_not_mutate_original() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let _ = v * 5.0;
        assert!(vec_approx_eq(v, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    // Div ------------------------------------------------------------
    #[test]
    fn test_div_ref_basic() {
        let v = Vector3d { x: 2.0, y: 4.0, z: 6.0 };
        let result = v / 2.0;
        assert!(vec_approx_eq(result, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    #[test]
    fn test_div_ref_one() {
        let v = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = v / 1.0;
        assert!(vec_approx_eq(result, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    #[test]
    fn test_div_ref_negative() {
        let v = Vector3d { x: 2.0, y: 4.0, z: 6.0 };
        let result = v / -2.0;
        assert!(vec_approx_eq(result, Vector3d { x: -1.0, y: -2.0, z: -3.0 }));
    }

    #[test]
    fn test_div_ref_by_zero_produces_infinity() {
        let v = Vector3d { x: 1.0, y: 1.0, z: 1.0 };
        let result = v / 0.0;
        assert!(result.x.is_infinite() && result.y.is_infinite() && result.z.is_infinite());
    }

    #[test]
    fn test_div_ref_does_not_mutate_original() {
        let v = Vector3d { x: 4.0, y: 4.0, z: 4.0 };
        let _ = v / 2.0;
        assert!(vec_approx_eq(v, Vector3d { x: 4.0, y: 4.0, z: 4.0 }));
    }

    // Add ------------------------------------------------------------
    #[test]
    fn test_add_ref_basic() {
        let a = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vector3d { x: 4.0, y: 5.0, z: 6.0 };
        let result = a + b;
        assert!(vec_approx_eq(result, Vector3d { x: 5.0, y: 7.0, z: 9.0 }));
    }

    #[test]
    fn test_add_ref_zero_vector() {
        let a = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = a + Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        assert!(vec_approx_eq(result, a));
    }

    #[test]
    fn test_add_ref_negative() {
        let a = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = a + Vector3d { x: -1.0, y: -2.0, z: -3.0 };
        assert!(vec_approx_eq(result, Vector3d { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_add_ref_does_not_mutate_original() {
        let a = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let _ = a + Vector3d { x: 10.0, y: 10.0, z: 10.0 };
        assert!(vec_approx_eq(a, Vector3d { x: 1.0, y: 2.0, z: 3.0 }));
    }

    // Sub ------------------------------------------------------------
    #[test]
    fn test_sub_ref_basic() {
        let a = Vector3d { x: 5.0, y: 7.0, z: 9.0 };
        let b = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = a - b;
        assert!(vec_approx_eq(result, Vector3d { x: 4.0, y: 5.0, z: 6.0 }));
    }

    #[test]
    fn test_sub_ref_zero_vector() {
        let a = Vector3d { x: 1.0, y: 2.0, z: 3.0 };
        let result = a - Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        assert!(vec_approx_eq(result, a));
    }

    #[test]
    fn test_sub_ref_self_gives_zero() {
        let a = Vector3d { x: 3.0, y: 5.0, z: 7.0 };
        let result = a - Vector3d { x: 3.0, y: 5.0, z: 7.0 };
        assert!(vec_approx_eq(result, Vector3d { x: 0.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_sub_ref_does_not_mutate_original() {
        let a = Vector3d { x: 5.0, y: 5.0, z: 5.0 };
        let _ = a - Vector3d { x: 1.0, y: 1.0, z: 1.0 };
        assert!(vec_approx_eq(a, Vector3d { x: 5.0, y: 5.0, z: 5.0 }));
    }

    // Magnitude ------------------------------------------------------------
    #[test]
    fn test_magnitude_unit_x() {
        let v = Vector3d { x: 1.0, y: 0.0, z: 0.0 };
        assert!(approx_eq(v.magnitude(), 1.0));
    }

    #[test]
    fn test_magnitude_unit_y() {
        let v = Vector3d { x: 0.0, y: 1.0, z: 0.0 };
        assert!(approx_eq(v.magnitude(), 1.0));
    }

    #[test]
    fn test_magnitude_unit_z() {
        let v = Vector3d { x: 0.0, y: 0.0, z: 1.0 };
        assert!(approx_eq(v.magnitude(), 1.0));
    }

    #[test]
    fn test_magnitude_known_value() {
        // sqrt(1² + 2² + 2²) = sqrt(9) = 3
        let v = Vector3d { x: 1.0, y: 2.0, z: 2.0 };
        assert!(approx_eq(v.magnitude(), 3.0));
    }

    #[test]
    fn test_magnitude_zero_vector() {
        let v = Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        assert!(approx_eq(v.magnitude(), 0.0));
    }

    #[test]
    fn test_magnitude_negative_components() {
        // sqrt((-3)² + 4² + 0²) = sqrt(25) = 5
        let v = Vector3d { x: -3.0, y: 4.0, z: 0.0 };
        assert!(approx_eq(v.magnitude(), 5.0));
    }

    #[test]
    fn test_magnitude_all_same() {
        // sqrt(3 * 2²) = sqrt(12) ≈ 3.4641
        let v = Vector3d { x: 2.0, y: 2.0, z: 2.0 };
        assert!(approx_eq(v.magnitude(), 12f32.sqrt()));
    }

    // Normalize ------------------------------------------------------------
    #[test]
    fn test_normalize_unit_x() {
        let v = Vector3d { x: 5.0, y: 0.0, z: 0.0 };
        let n = v.normalize();
        assert!(vec_approx_eq(n, Vector3d { x: 1.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_normalize_unit_y() {
        let v = Vector3d { x: 0.0, y: 3.0, z: 0.0 };
        let n = v.normalize();
        assert!(vec_approx_eq(n, Vector3d { x: 0.0, y: 1.0, z: 0.0 }));
    }

    #[test]
    fn test_normalize_unit_z() {
        let v = Vector3d { x: 0.0, y: 0.0, z: 7.0 };
        let n = v.normalize();
        assert!(vec_approx_eq(n, Vector3d { x: 0.0, y: 0.0, z: 1.0 }));
    }

    #[test]
    fn test_normalize_magnitude_is_one() {
        let v = Vector3d { x: 3.0, y: 4.0, z: 0.0 };
        let n = v.normalize();
        assert!(approx_eq(n.magnitude(), 1.0));
    }

    #[test]
    fn test_normalize_already_unit_vector() {
        let v = Vector3d { x: 1.0, y: 0.0, z: 0.0 };
        let n = v.normalize();
        assert!(vec_approx_eq(n, Vector3d { x: 1.0, y: 0.0, z: 0.0 }));
    }

    #[test]
    fn test_normalize_negative_components() {
        let v = Vector3d { x: -3.0, y: 4.0, z: 0.0 };
        let n = v.normalize();
        assert!(approx_eq(n.magnitude(), 1.0));
        assert!(approx_eq(n.x, -0.6));
        assert!(approx_eq(n.y, 0.8));
    }

    #[test]
    fn test_normalize_does_not_mutate_original() {
        let v = Vector3d { x: 3.0, y: 4.0, z: 0.0 };
        let _ = v.normalize();
        assert!(vec_approx_eq(v, Vector3d { x: 3.0, y: 4.0, z: 0.0 }));
    }

    #[test]
    fn test_normalize_zero_vector_produces_nan() {
        // Dividing by zero magnitude yields NaN — expected behavior for f32
        let v = Vector3d { x: 0.0, y: 0.0, z: 0.0 };
        let n = v.normalize();
        assert!(n.x.is_nan() && n.y.is_nan() && n.z.is_nan());
    }
}

