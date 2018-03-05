//! SBET file format.


use {Error, Result};

use byteorder;
use byteorder::{LittleEndian, ReadBytesExt};
use point::Point;
use source::Source;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::IntoIterator;
use std::path::Path;
use units::Radians;

/// An SBET reader.
#[derive(Debug)]
pub struct Reader<R: Read> {
    reader: R,
}

impl Reader<BufReader<File>> {
    /// Opens a reader for a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::sbet::Reader;
    /// let reader = Reader::from_path("data/2-points.sbet").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>> {
        Ok(Reader { reader: BufReader::new(File::open(path)?) })
    }
}

impl<R: Read> Reader<R> {
    /// Reads a point from this reader.
    ///
    /// Returns none if the file is at its end when this reader starts reading. We have to do it
    /// this way since sbet files don't have a point count.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::sbet::Reader;
    /// let mut reader = Reader::from_path("data/2-points.sbet").unwrap();
    /// let point = reader.read_point().unwrap().unwrap();
    /// ```
    pub fn read_point(&mut self) -> Result<Option<Point>> {
        let time = match self.reader.read_f64::<LittleEndian>() {
            Ok(time) => time,
            Err(byteorder::Error::UnexpectedEOF) => return Ok(None),
            Err(err) => return Err(Error::from(err)),
        };
        Ok(Some(Point {
            time: time,
            latitude: Radians(self.reader.read_f64::<LittleEndian>()?),
            longitude: Radians(self.reader.read_f64::<LittleEndian>()?),
            altitude: self.reader.read_f64::<LittleEndian>()?,
            x_velocity: Some(self.reader.read_f64::<LittleEndian>()?),
            y_velocity: Some(self.reader.read_f64::<LittleEndian>()?),
            z_velocity: Some(self.reader.read_f64::<LittleEndian>()?),
            roll: Radians(self.reader.read_f64::<LittleEndian>()?),
            pitch: Radians(self.reader.read_f64::<LittleEndian>()?),
            yaw: Radians(self.reader.read_f64::<LittleEndian>()?),
            wander_angle: Some(Radians(self.reader.read_f64::<LittleEndian>()?)),
            x_acceleration: Some(self.reader.read_f64::<LittleEndian>()?),
            y_acceleration: Some(self.reader.read_f64::<LittleEndian>()?),
            z_acceleration: Some(self.reader.read_f64::<LittleEndian>()?),
            x_angular_rate: Some(Radians(self.reader.read_f64::<LittleEndian>()?)),
            y_angular_rate: Some(Radians(self.reader.read_f64::<LittleEndian>()?)),
            z_angular_rate: Some(Radians(self.reader.read_f64::<LittleEndian>()?)),
            ..Default::default()
        }))
    }
}

impl<R: Read> IntoIterator for Reader<R> {
    type Item = Point;
    type IntoIter = ReaderIterator<R>;
    fn into_iter(self) -> Self::IntoIter {
        ReaderIterator { reader: self }
    }
}

/// An iterator over an sbet reader.
#[derive(Debug)]
pub struct ReaderIterator<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> Iterator for ReaderIterator<R> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_point().unwrap()
    }
}

impl<R: Debug + Read> Source for Reader<R> {
    fn source(&mut self) -> Result<Option<Point>> {
        self.read_point()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let reader = Reader::from_path("data/2-points.sbet").unwrap();
        let points: Vec<_> = reader.into_iter().collect();
        assert_eq!(2, points.len());
        let point = points[0];
        assert!((1.5163100e5 - point.time).abs() < 1e-2, "{}", point.time);
        assert!(
            (0.5680211 - point.latitude.0).abs() < 1e-7,
            "{:?}",
            point.latitude
        );
        assert!(
            (1.5163110e5 - points[1].time).abs() < 1e-1,
            "{}",
            points[1].time
        );
    }
}
