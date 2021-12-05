use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
/// # `Matrix2`
/// A good ol' 2x2 Matrix
pub struct Matrix2 {
    pub c11: f32,
    pub c12: f32,
    pub c21: f32,
    pub c22: f32,
}

impl Matrix2 {
    /// # `identity`
    /// Creates an identity 2x2 matrix
    pub fn identity() -> Matrix2 {
        Matrix2 {
            c11: 1.0,
            c12: 0.0,
            c21: 0.0,
            c22: 1.0
        }
    }

    /// # `rotation`
    /// Creates a rotation matrix using the given angle `f32` in radians
    pub fn rotation(radians: f32) -> Matrix2 {
        Matrix2 {
            c11: radians.cos(),
            c21: radians.sin(),
            c12: -radians.sin(),
            c22: radians.cos()
        }
    }

    /// # `determinate`
    /// Returns the determinate of the matrix as an `f32`
    pub fn determinate(&self) -> f32 {
        self.c11 * self.c22 - self.c12 * self.c21
    }

    /// # `inverse`
    /// Returns the inverse of the matrix
    pub fn inverse(&self) -> Option<Matrix2> {
        let det = self.determinate();

        if det != 0.0 {
            return Some((1.0 / det) * Matrix2 {
                c11: self.c22,
                c12: -self.c12,
                c21: -self.c21,
                c22: self.c11 });
        }
        None
    }
}

/// Implement scalar * Matrix multiplication
impl ops::Mul<Matrix2> for f32 {
    type Output = Matrix2;

    fn mul(self, _rhs: Matrix2) -> Matrix2 {
        Matrix2 {
            c11: _rhs.c11 * self,
            c12: _rhs.c12 * self,
            c21: _rhs.c21 * self,
            c22: _rhs.c22 * self
        }
    }
}

/// Implement Matrix * Matrix multiplication
impl ops::Mul<Matrix2> for Matrix2 {
    type Output = Matrix2;

    fn mul(self, _rhs: Matrix2) -> Matrix2{
        Matrix2 {
            c11: self.c11 * _rhs.c11 + self.c12 * _rhs.c21,
            c12: self.c11 * _rhs.c12 + self.c12 * _rhs.c22,
            c21: self.c21 * _rhs.c11 + self.c22 * _rhs.c21,
            c22: self.c21 * _rhs.c12 + self.c22 * _rhs.c22
        }
    }
}