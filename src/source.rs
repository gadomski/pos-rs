//! Sources of position points.

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Seek, Read};
use std::iter::IntoIterator;
use std::path::Path;

use Result;
use pof;
use poq;
use point::{Accuracy, Point};

/// A source of points.
pub trait Source: Debug {
    /// Reads one point from the source.
    fn source(&mut self) -> Result<Option<Point>>;
}

impl IntoIterator for Box<Source> {
    type Item = Point;
    type IntoIter = SourceIterator;
    fn into_iter(self) -> Self::IntoIter {
        SourceIterator { source: self }
    }
}

/// An iterator over a boxed point source.
#[derive(Debug)]
pub struct SourceIterator {
    source: Box<Source>,
}

impl Iterator for SourceIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        self.source.source().unwrap()
    }
}

/// A source of accuracy information.
pub trait AccuracySource: Debug {
    /// Reads an accuracy reading from this accuracy source.
    fn source(&mut self) -> Result<Option<Accuracy>>;
}

impl<R: Debug + Seek + Read> AccuracySource for poq::Reader<R> {
    fn source(&mut self) -> Result<Option<Accuracy>> {
        self.read_accuracy()
    }
}

/// A source of points that is based in a file.
pub trait FileSource {
    /// Open a new file source from a file.
    fn open_file_source<P: AsRef<Path>>(path: P) -> Result<Box<Source>>;
}

impl FileSource for pof::Reader<BufReader<File>> {
    fn open_file_source<P: AsRef<Path>>(path: P) -> Result<Box<Source>> {
        Ok(Box::new(try!(pof::Reader::from_path(path))))
    }
}

/// A source of accuracy information
pub trait FileAccuracySource {
    /// Opens a new accuracy source from a file.
    fn open_file_accuracy_source<P: AsRef<Path>>(path: P) -> Result<Box<AccuracySource>>;
}

impl FileAccuracySource for poq::Reader<BufReader<File>> {
    fn open_file_accuracy_source<P: AsRef<Path>>(path: P) -> Result<Box<AccuracySource>> {
        Ok(Box::new(try!(poq::Reader::from_path(path))))
    }
}

/// A source of points that includes accuracy information.
#[derive(Debug)]
pub struct CombinedSource {
    source: Box<Source>,
    accuracy_source: Box<AccuracySource>,
    accuracies: (Option<Accuracy>, Option<Accuracy>),
}

impl CombinedSource {
    /// Creates a new combined source from two boxes.
    pub fn new(source: Box<Source>,
               mut accuracy_source: Box<AccuracySource>)
               -> Result<CombinedSource> {
        let accuracies = (try!(accuracy_source.source()),
                          try!(accuracy_source.source()));
        Ok(CombinedSource {
            source: source,
            accuracy_source: accuracy_source,
            accuracies: accuracies,
        })
    }
}

impl Source for CombinedSource {
    fn source(&mut self) -> Result<Option<Point>> {
        let mut point = match try!(self.source.source()) {
            Some(point) => point,
            None => return Ok(None),
        };
        // Since we populate the accuracies on create, if these are none we've run out of
        // accuracies.
        if self.accuracies.0.is_none() || self.accuracies.1.is_none() ||
           point.time < self.accuracies.0.unwrap().time {
            return Ok(Some(point));
        }
        loop {
            if point.time > self.accuracies.1.unwrap().time {
                self.accuracies.0 = self.accuracies.1;
                self.accuracies.1 = try!(self.accuracy_source.source());
            } else {
                break;
            }
            if self.accuracies.1.is_none() {
                return Ok(Some(point));
            }
        }
        point.accuracy = Some(self.accuracies
                                  .0
                                  .unwrap()
                                  .interpolate(&self.accuracies.1.unwrap(), point.time));
        Ok(Some(point))
    }
}

impl IntoIterator for CombinedSource {
    type Item = Point;
    type IntoIter = CombinedSourceIterator;
    fn into_iter(self) -> Self::IntoIter {
        CombinedSourceIterator { source: self }
    }
}

/// Iterator over a combined source.
#[derive(Debug)]
pub struct CombinedSourceIterator {
    source: CombinedSource,
}

impl Iterator for CombinedSourceIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        self.source.source().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pof;
    use poq;

    #[test]
    fn read_pof() {
        let source = pof::Reader::open_file_source("data/sbet_mission_1.pof").unwrap();
        let points: Vec<_> = source.into_iter().collect();
        assert_eq!(1114521, points.len());
    }

    #[test]
    fn read_pof_with_poq() {
        let source = pof::Reader::open_file_source("data/sbet_mission_1.pof").unwrap();
        let accuracy_source = poq::Reader::open_file_accuracy_source("data/sbet_mission_1.poq")
                                  .unwrap();
        let accuracies: Vec<_> = CombinedSource::new(source, accuracy_source)
                                     .unwrap()
                                     .into_iter()
                                     .take(20)
                                     .collect();
        assert_eq!(20, accuracies.len());
    }
}
