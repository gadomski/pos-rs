//! Interpolate between two position points.

use {Error, Result};
use source::Source;
use point::Point;

/// Structure that handles the interpolation.
#[derive(Debug)]
pub struct Interpolator<S: Source> {
    index: usize,
    source: Box<S>,
    points: Vec<Point>,
}

impl<S: Source> Interpolator<S> {
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
    pub fn new(mut source: Box<S>) -> Result<Interpolator<S>> {
        let mut points = Vec::with_capacity(2);
        for _ in 0..2 {
            points.push(match try!(source.source()) {
                Some(point) => point,
                None => {
                    return Err(Error::Extrapolation("Source must have at least two points for \
                                                     interpolation"
                                                        .to_string()))
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
    pub fn interpolate(&mut self, time: f64) -> Result<Point> {
        loop {
            assert!(self.index != 0 && self.index != self.points.len());
            if time < self.points[self.index - 1].time {
                if self.index == 1 {
                    return Err(Error::Extrapolation(format!("Time {} is below the minimum time \
                                                             of this source",
                                                            time)));
                } else {
                    self.index -= 1;
                }
            } else if time > self.points[self.index].time {
                if self.index < self.points.len() - 1 {
                    self.index += 1;
                } else {
                    match try!(self.source.source()) {
                        Some(point) => {
                            self.points.push(point);
                            self.index += 1;
                        }
                        None => {
                            return Err(Error::Extrapolation(format!("Time {} is above the \
                                                                     maximum time of this source",
                                                                    time)))
                        }
                    }
                }
            } else {
                break;
            }
        }
        Ok(self.points[self.index - 1].interpolate(&self.points[self.index], time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sbet;

    #[test]
    fn interp_sbet() {
        let mut interpolator = Interpolator::new(Box::new(sbet::Reader::from_path("data/2-point\
                                                                                   s.sbet")
                                                              .unwrap()))
                                   .unwrap();
        let time = 1.516310048360710e5;
        let point = interpolator.interpolate(time).unwrap();
        assert_eq!(time, point.time);
        assert!(interpolator.interpolate(0.0).is_err());
    }
}
