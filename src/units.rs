//! Unit newtypes.

use std::f64::consts::PI;
use std::ops::{Add, Mul, Sub};

/// Newtype wrapper around a radian value.
///
/// It's so easy to forget if you're using radians or degrees.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Radians<T>(pub T);

impl Radians<f64> {
    /// Create a new radian value from a degree value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::units::Radians;
    /// let radians = Radians::from_degrees(180.0);
    /// ```
    pub fn from_degrees(degrees: f64) -> Radians<f64> {
        Radians(degrees * PI / 180.0)
    }

    /// Converts this radians value to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::units::Radians;
    /// let degrees = Radians(3.14).to_degrees();
    /// ```
    pub fn to_degrees(self) -> f64 {
        self.0 * 180.0 / PI
    }
}

impl Add for Radians<f64> {
    type Output = Radians<f64>;
    fn add(self, other: Radians<f64>) -> Radians<f64> {
        Radians(self.0 + other.0)
    }
}

impl Sub for Radians<f64> {
    type Output = Radians<f64>;
    fn sub(self, other: Radians<f64>) -> Radians<f64> {
        Radians(self.0 - other.0)
    }
}

impl Mul<Radians<f64>> for f64 {
    type Output = Radians<f64>;
    fn mul(self, other: Radians<f64>) -> Radians<f64> {
        Radians(self * other.0)
    }
}
