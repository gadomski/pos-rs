//! Position and orientation quality files.

use std::fs::File;
use std::io::{BufReader, Seek, Read};
use std::iter::IntoIterator;
use std::path::Path;

use byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use {Error, Result};
use io::read_full;
use point::{Accuracy, SatelliteCount};
use units::Radians;

/// A poq file reader.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct Reader<R: Read + Seek> {
    pub avgint: f64,
    pub devint: f64,
    pub maxint: f64,
    pub version: Version,
    reader: R,
}

impl Reader<BufReader<File>> {
    /// Creates a new reader for the given path.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::poq::Reader;
    /// let reader = Reader::from_path("data/sbet_mission_1.poq").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>> {
        let reader = BufReader::new(try!(File::open(path)));
        Reader::new(reader)
    }
}

impl<R: Seek + Read> Reader<R> {
    fn new(mut reader: R) -> Result<Reader<R>> {
        let ref mut preamble = [0; 35];
        try!(read_full(&mut reader, preamble));

        let major = try!(reader.read_u16::<LittleEndian>());
        let minor = try!(reader.read_u16::<LittleEndian>());
        let version = Version::new(major, minor);
        let avgint = try!(reader.read_f64::<LittleEndian>());
        let maxint = try!(reader.read_f64::<LittleEndian>());
        let devint = try!(reader.read_f64::<LittleEndian>());

        Ok(Reader {
            avgint: avgint,
            devint: devint,
            maxint: maxint,
            reader: reader,
            version: version,
        })
    }

    /// Reads a record from this reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::poq::Reader;
    /// let mut reader = Reader::from_path("data/sbet_mission_1.poq").unwrap();
    /// let accuracy = reader.read_accuracy().unwrap();
    /// ```
    pub fn read_accuracy(&mut self) -> Result<Option<Accuracy>> {
        let time = match self.reader.read_f64::<LittleEndian>() {
            Ok(time) => time,
            Err(byteorder::Error::UnexpectedEOF) => return Ok(None),
            Err(err) => return Err(Error::from(err)),
        };
        let north = try!(self.reader.read_f64::<LittleEndian>());
        let east = try!(self.reader.read_f64::<LittleEndian>());
        let down = try!(self.reader.read_f64::<LittleEndian>());
        let roll = try!(self.reader.read_f64::<LittleEndian>());
        let pitch = try!(self.reader.read_f64::<LittleEndian>());
        let yaw = try!(self.reader.read_f64::<LittleEndian>());
        let pdop = try!(self.reader.read_f64::<LittleEndian>());
        let satellite_count = if self.version.specifies_satellite_count() {
            let ngps = try!(self.reader.read_u16::<LittleEndian>());
            let nglonass = try!(self.reader.read_u16::<LittleEndian>());
            SatelliteCount::Specified {
                gps: ngps,
                glonass: nglonass,
            }
        } else {
            SatelliteCount::Unspecified(try!(self.reader.read_u16::<LittleEndian>()))
        };

        Ok(Some(Accuracy {
            time: time,
            y: north,
            x: east,
            z: down,
            roll: Radians::from_degrees(roll),
            pitch: Radians::from_degrees(pitch),
            yaw: Radians::from_degrees(yaw),
            pdop: pdop,
            satellite_count: Some(satellite_count),
        }))
    }
}

impl<R: Seek + Read> IntoIterator for Reader<R> {
    type Item = Accuracy;
    type IntoIter = ReaderIterator<R>;
    fn into_iter(self) -> Self::IntoIter {
        ReaderIterator { reader: self }
    }
}

/// An iterator over a poq reader.
#[derive(Debug)]
pub struct ReaderIterator<R: Read + Seek> {
    reader: Reader<R>,
}

impl<R: Read + Seek> Iterator for ReaderIterator<R> {
    type Item = Accuracy;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_accuracy().unwrap()
    }
}

/// poq file version.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Version {
    major: u16,
    minor: u16,
}

impl Version {
    /// Creates a new version.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::poq::Version;
    /// Version::new(1, 1);
    /// ```
    pub fn new(major: u16, minor: u16) -> Version {
        Version {
            major: major,
            minor: minor,
        }
    }

    fn specifies_satellite_count(&self) -> bool {
        self.minor >= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let reader = Reader::from_path("data/sbet_mission_1.poq").unwrap();
        let records: Vec<_> = reader.into_iter().zip(0..5571).map(|(r, _)| r).collect();
        assert_eq!(5571, records.len());
    }
}
