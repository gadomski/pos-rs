//! Pos files are ASCII position files.

use failure::Error;
use point::Point;
use source::Source;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use units::Radians;

/// A pos reader.
#[derive(Debug)]
pub struct Reader<R: BufRead> {
    reader: R,
}

impl Reader<BufReader<File>> {
    /// Creates a new reader from a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::pos::Reader;
    /// let reader = Reader::from_path("data/0916_2014_ie.pos").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>, Error> {
        let mut reader = BufReader::new(File::open(path)?);
        let ref mut header: String = String::new();
        let _ = reader.read_line(header)?;
        Ok(Reader { reader: reader })
    }
}

impl<R: BufRead> Reader<R> {
    /// Reads a point from the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::pos::Reader;
    /// let mut reader = Reader::from_path("data/0916_2014_ie.pos").unwrap();
    /// let point = reader.read_point().unwrap();
    /// ```
    pub fn read_point(&mut self) -> Result<Option<Point>, Error> {
        let mut line = String::new();
        let _ = self.reader.read_line(&mut line)?;
        let values: Vec<_> = line.split_whitespace().collect();
        if values.is_empty() {
            return Ok(None);
        }
        Ok(Some(Point {
            time: values[0].parse()?,
            latitude: Radians::from_degrees(values[1].parse()?),
            longitude: Radians::from_degrees(values[2].parse()?),
            altitude: values[3].parse()?,
            roll: Radians::from_degrees(values[4].parse()?),
            pitch: Radians::from_degrees(values[5].parse()?),
            yaw: Radians::from_degrees(values[6].parse()?),
            ..Default::default()
        }))
    }
}

impl<R: BufRead> IntoIterator for Reader<R> {
    type Item = Point;
    type IntoIter = ReaderIterator<R>;
    fn into_iter(self) -> Self::IntoIter {
        ReaderIterator { reader: self }
    }
}

/// An iterator over a pos reader.
#[derive(Debug)]
pub struct ReaderIterator<R: BufRead> {
    reader: Reader<R>,
}

impl<R: BufRead> Iterator for ReaderIterator<R> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_point().unwrap()
    }
}

impl<R: Debug + BufRead> Source for Reader<R> {
    fn source(&mut self) -> Result<Option<Point>, Error> {
        self.read_point()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_count() {
        let points: Vec<_> = Reader::from_path("data/0916_2014_ie.pos")
            .unwrap()
            .into_iter()
            .collect();
        assert_eq!(722800, points.len());
    }
}
