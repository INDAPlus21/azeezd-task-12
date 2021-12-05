use std::ops;
use super::matrix::Matrix2;

#[derive(Debug, PartialEq, Copy, Clone)]
/// # `Vector`
/// Structure that stores a 2D vector
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    /// # `new`
    /// Takes a `f32` x and y coordinate and returns a Vector with those coordinates
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x,
            y: y
        }
    }

    /// # `from_usize`
    /// Takes a `(isize, isize)` coordinates and returns a Vector with those coordinates
    pub fn from_isize(coordinates: (isize, isize)) -> Vector2 {
        Vector2::new(coordinates.0 as f32, coordinates.1 as f32)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Implement vector + vector
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + _rhs.x, 
            y: self.y + _rhs.y
        }
    }
}

/// Implement vector - vector
impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - _rhs.x, 
            y: self.y - _rhs.y
        }
    }
}

/// Implement Vector * scalar multiplication
impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, _rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * _rhs,
            y: self.y * _rhs 
        }
    }
}

/// Implement scalar * vector multiplication
impl ops::Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul (self, _rhs: Vector2) -> Vector2 {
        _rhs * self
    }
}

/// Implement Vector / Scalar division
impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, _rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x / _rhs ,
            y: self.y / _rhs 
        }
    } 
}

/// Implement Matrix * Vector multiplication
impl ops::Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, _rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.c11 * _rhs.x + self.c12 * _rhs.y,
            y: self.c21 * _rhs.x + self.c22 * _rhs.y
        }
    }
}

/// Implement -vector
impl ops::Neg<> for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y
        }
    }
}