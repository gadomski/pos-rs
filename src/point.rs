//! Points.

use units::Radians;

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
            time: self.time + factor * (other.time - self.time),
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            z: self.z + factor * (other.z - self.z),
            roll: self.roll + factor * (other.roll - self.roll),
            pitch: self.pitch + factor * (other.pitch - self.pitch),
            yaw: self.yaw + factor * (other.yaw - self.yaw),
            pdop: self.pdop + factor * (other.pdop - self.pdop),
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
