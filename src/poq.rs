//! Position and orientation quality files.

use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use point::{Accuracy, SatelliteCount};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::iter::IntoIterator;
use std::path::Path;
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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>, Error> {
        let reader = BufReader::new(File::open(path)?);
        Reader::new(reader)
    }
}

impl<R: Seek + Read> Reader<R> {
    // TODO can I make this just an io error on return?
    fn new(mut reader: R) -> Result<Reader<R>, Error> {
        let mut preamble = [0; 35];
        reader.read_exact(&mut preamble)?;

        let major = reader.read_u16::<LittleEndian>()?;
        let minor = reader.read_u16::<LittleEndian>()?;
        let version = Version::new(major, minor);
        let avgint = reader.read_f64::<LittleEndian>()?;
        let maxint = reader.read_f64::<LittleEndian>()?;
        let devint = reader.read_f64::<LittleEndian>()?;

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
    pub fn read_accuracy(&mut self) -> Result<Option<Accuracy>, Error> {
        use std::io::ErrorKind;

        let time = match self.reader.read_f64::<LittleEndian>() {
            Ok(time) => time,
            Err(err) => {
                match err.kind() {
                    ErrorKind::UnexpectedEof => return Ok(None),
                    _ => return Err(err.into()),
                }
            }
        };
        let north = self.reader.read_f64::<LittleEndian>()?;
        let east = self.reader.read_f64::<LittleEndian>()?;
        let down = self.reader.read_f64::<LittleEndian>()?;
        let roll = self.reader.read_f64::<LittleEndian>()?;
        let pitch = self.reader.read_f64::<LittleEndian>()?;
        let yaw = self.reader.read_f64::<LittleEndian>()?;
        let pdop = self.reader.read_f64::<LittleEndian>()?;
        let satellite_count = if self.version.specifies_satellite_count() {
            let ngps = self.reader.read_u16::<LittleEndian>()?;
            let nglonass = self.reader.read_u16::<LittleEndian>()?;
            SatelliteCount::Specified {
                gps: ngps,
                glonass: nglonass,
            }
        } else {
            SatelliteCount::Unspecified(self.reader.read_u16::<LittleEndian>()?)
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
