//! Points.

use crate::units::Radians;

macro_rules! interpolate {
    ($lhs:ident, $rhs:ident, $factor:ident, $var:ident) => {{
        $lhs.$var + $factor * ($rhs.$var - $lhs.$var)
    }};
}

macro_rules! interpolate_optional {
    ($lhs:ident, $rhs:ident, $factor:ident, $var:ident) => {{
        if let Some(l) = $lhs.$var {
            if let Some(r) = $rhs.$var {
                Some(l + $factor * (r - l))
            } else {
                None
            }
        } else {
            None
        }
    }};
}

/// A position point.
///
/// This must contain position and attidue information, and may contain error information.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(missing_docs)]
pub struct Point {
    pub time: f64,
    pub longitude: Radians<f64>,
    pub latitude: Radians<f64>,
    pub altitude: f64,
    pub roll: Radians<f64>,
    pub pitch: Radians<f64>,
    pub yaw: Radians<f64>,
    pub distance: Option<f64>,
    pub x_velocity: Option<f64>,
    pub y_velocity: Option<f64>,
    pub z_velocity: Option<f64>,
    pub wander_angle: Option<Radians<f64>>,
    pub x_acceleration: Option<f64>,
    pub y_acceleration: Option<f64>,
    pub z_acceleration: Option<f64>,
    pub x_angular_rate: Option<Radians<f64>>,
    pub y_angular_rate: Option<Radians<f64>>,
    pub z_angular_rate: Option<Radians<f64>>,
    pub accuracy: Option<Accuracy>,
}

impl Point {
    /// Linearly interpolate a new point between these two.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::point::Point;
    /// let mut p1: Point = Default::default();
    /// p1.time = 10.0;
    /// let mut p2: Point = Default::default();
    /// p2.time = 20.0;
    /// let p3 = p1.interpolate(&p2, 15.0);
    /// ```
    pub fn interpolate(&self, other: &Point, time: f64) -> Point {
        let factor = (time - self.time) / (other.time - self.time);
        Point {
            time: interpolate!(self, other, factor, time),
            longitude: interpolate!(self, other, factor, longitude),
            latitude: interpolate!(self, other, factor, latitude),
            altitude: interpolate!(self, other, factor, altitude),
            roll: interpolate!(self, other, factor, roll),
            pitch: interpolate!(self, other, factor, pitch),
            yaw: interpolate!(self, other, factor, yaw),
            distance: interpolate_optional!(self, other, factor, distance),
            x_velocity: interpolate_optional!(self, other, factor, x_velocity),
            y_velocity: interpolate_optional!(self, other, factor, y_velocity),
            z_velocity: interpolate_optional!(self, other, factor, z_velocity),
            wander_angle: interpolate_optional!(self, other, factor, wander_angle),
            x_acceleration: interpolate_optional!(self, other, factor, x_acceleration),
            y_acceleration: interpolate_optional!(self, other, factor, y_acceleration),
            z_acceleration: interpolate_optional!(self, other, factor, z_acceleration),
            x_angular_rate: interpolate_optional!(self, other, factor, x_angular_rate),
            y_angular_rate: interpolate_optional!(self, other, factor, y_angular_rate),
            z_angular_rate: interpolate_optional!(self, other, factor, z_angular_rate),
            accuracy: if let Some(a1) = self.accuracy {
                if let Some(a2) = other.accuracy {
                    Some(a1.interpolate(&a2, time))
                } else {
                    None
                }
            } else {
                None
            },
        }
    }
}

/// The accuracy of a position.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(missing_docs)]
pub struct Accuracy {
    pub time: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub roll: Radians<f64>,
    pub pitch: Radians<f64>,
    pub yaw: Radians<f64>,
    pub pdop: f64,
    pub satellite_count: Option<SatelliteCount>,
}

impl Accuracy {
    /// Linearly interpolate a new accuracy between these two.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::point::Accuracy;
    /// let mut accuracy1: Accuracy = Default::default();
    /// accuracy1.time = 10.0;
    /// let mut accuracy2: Accuracy = Default::default();
    /// accuracy2.time = 20.0;
    /// let accuracy3 = accuracy1.interpolate(&accuracy2, 15.0);
    /// ```
    pub fn interpolate(&self, other: &Accuracy, time: f64) -> Accuracy {
        let factor = (time - self.time) / (other.time - self.time);
        Accuracy {
            time: interpolate!(self, other, factor, time),
            x: interpolate!(self, other, factor, x),
            y: interpolate!(self, other, factor, y),
            z: interpolate!(self, other, factor, z),
            roll: interpolate!(self, other, factor, roll),
            pitch: interpolate!(self, other, factor, pitch),
            yaw: interpolate!(self, other, factor, yaw),
            pdop: interpolate!(self, other, factor, pdop),
            satellite_count: None,
        }
    }
}

/// A count of the number of satellites.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SatelliteCount {
    /// The type of the satellites being counted is unspecified.
    Unspecified(u16),
    /// The type of the satellites being countes has been specified.
    Specified {
        /// GPS satellties (US).
        gps: u16,
        /// GLONASS satellites (Russia).
        glonass: u16,
    },
}

impl Default for SatelliteCount {
    fn default() -> SatelliteCount {
        SatelliteCount::Unspecified(0)
    }
}
