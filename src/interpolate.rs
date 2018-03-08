//! Interpolate between two position points.

use failure;
use point::Point;
use source::Source;

/// Errors for interpolation.
#[derive(Clone, Copy, Debug, Fail)]
pub enum Error {
    /// Error returned when trying to extrapolate with only one point in the source.
    #[fail(display = "Cannot interpolate in a source with only one point")]
    OnePoint,

    /// The time value is below the minimum time of the source.
    #[fail(display = "Time value is below minimum of the source: {}", _0)]
    TimeBelowMinimum(f64),

    /// The time value is above the maximum time of the source.
    #[fail(display = "Time value is above the maximum of the source: {}", _0)]
    TimeAboveMaximum(f64),
}

/// Structure that handles the interpolation.
#[derive(Debug)]
pub struct Interpolator {
    index: usize,
    source: Box<Source>,
    points: Vec<Point>,
}

impl Interpolator {
    /// Creates a new interpolator for a given source.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::interpolate::Interpolator;
    /// use pos::sbet;
    /// let reader = sbet::Reader::from_path("data/2-points.sbet").unwrap();
    /// let interpolator = Interpolator::new(Box::new(reader)).unwrap();
    /// ```
    pub fn new(mut source: Box<Source>) -> Result<Interpolator, failure::Error> {
        let mut points = Vec::with_capacity(2);
        for _ in 0..2 {
            points.push(match source.source()? {
                Some(point) => point,
                None => {
                    return Err(Error::OnePoint.into());
                }
            });
        }
        Ok(Interpolator {
            points: points,
            source: source,
            index: 1,
        })
    }

    /// Interpolate a new point for the given time.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::interpolate::Interpolator;
    /// use pos::sbet;
    /// let reader = sbet::Reader::from_path("data/2-points.sbet").unwrap();
    /// let mut interpolator = Interpolator::new(Box::new(reader)).unwrap();
    /// let point = interpolator.interpolate(1.516310048360710e5).unwrap();
    /// ```
    pub fn interpolate(&mut self, time: f64) -> Result<Point, failure::Error> {
        loop {
            assert!(self.index != 0 && self.index != self.points.len());
            if time < self.points[self.index - 1].time {
                if self.index == 1 {
                    return Err(Error::TimeBelowMinimum(time).into());
                } else {
                    self.index -= 1;
                }
            } else if time > self.points[self.index].time {
                if self.index < self.points.len() - 1 {
                    self.index += 1;
                } else {
                    match self.source.source()? {
                        Some(point) => {
                            self.points.push(point);
                            self.index += 1;
                        }
                        None => {
                            return Err(Error::TimeAboveMaximum(time).into());
                        }
                    }
                }
            } else {
                break;
            }
        }
        Ok(self.points[self.index - 1].interpolate(
            &self.points[self.index],
            time,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sbet;

    #[test]
    fn interp_sbet() {
        let mut interpolator = Interpolator::new(Box::new(
            sbet::Reader::from_path(
                "data/2-point\
                                                                                   s.sbet",
            ).unwrap(),
        )).unwrap();
        let time = 1.516310048360710e5;
        let point = interpolator.interpolate(time).unwrap();
        assert_eq!(time, point.time);
        assert!(interpolator.interpolate(0.0).is_err());
    }
}
