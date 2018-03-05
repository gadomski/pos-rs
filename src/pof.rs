//! Position and orientation files.
//!
//! These are Riegl-specific GNSS/IMU data files.

use {Error, Result};

use byteorder::{LittleEndian, ReadBytesExt};
use io::read_full;
use point::Point;
use source::Source;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::iter::IntoIterator;
use std::path::Path;
use units::Radians;

/// A pos file reader.
#[derive(Debug)]
pub struct Reader<R: Read + Seek> {
    /// The average time interval between points.
    pub avgint: f64,

    /// The name of the company that produced this file.
    pub company: [u8; 32],

    /// The day this file was written.
    pub day: u16,

    /// The name of the device that collected this file.
    pub device: [u8; 32],

    /// The standard deviation of the time interval between points.
    pub devint: f64,

    /// The number of points in this file.
    pub entries: i64,

    /// The location that this file was collected.
    pub location: [u8; 16],

    /// The maximum altitude in this file.
    pub maxalt: f64,

    /// The maximum time interval between points in this file.
    pub maxint: f64,

    /// The maximum latitude.
    pub maxlat: f64,

    /// The maximum longitude.
    pub maxlon: f64,

    /// The minimum altitude.
    pub minalt: f64,

    /// The minimum latitude.
    pub minlat: f64,

    /// The minimum longitude.
    pub minlon: f64,

    /// The month this file was collected.
    pub month: u16,

    /// The name of this project.
    pub project: [u8; 32],

    /// Information about the time stamps in this file.
    pub timeinfo: TimeInfo,

    /// The units of the times in this file.
    pub timeunit: TimeUnit,

    /// This file's time zone.
    ///
    /// I have no idea if this is a real timezone->string code, or if it is just an arbitrary
    /// string.
    pub timezone: [u8; 16],

    /// The version of this file.
    pub version: Version,

    /// The year this file was collected.
    pub year: u16,

    reader: R,
    position: i64,
}

impl Reader<BufReader<File>> {
    /// Creates a new reader for the given path.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::pof::Reader;
    /// let reader = Reader::from_path("data/sbet_mission_1.pof").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>> {
        let reader = BufReader::new(File::open(path)?);
        Reader::new(reader)
    }
}

impl<R: Read + Seek> Reader<R> {
    fn new(mut reader: R) -> Result<Reader<R>> {
        let mut preamble = [0; 27];
        read_full(&mut reader, &mut preamble)?;

        let major = reader.read_u16::<LittleEndian>()?;
        let minor = reader.read_u16::<LittleEndian>()?;
        let version = Version::new(major, minor);

        let data_offset = reader.read_u32::<LittleEndian>()?;
        let year = reader.read_u16::<LittleEndian>()?;
        let month = reader.read_u16::<LittleEndian>()?;
        let day = reader.read_u16::<LittleEndian>()?;
        let entries = reader.read_i64::<LittleEndian>()?;
        let minlon = reader.read_f64::<LittleEndian>()?;
        let maxlon = reader.read_f64::<LittleEndian>()?;
        let minlat = reader.read_f64::<LittleEndian>()?;
        let maxlat = reader.read_f64::<LittleEndian>()?;
        let minalt = reader.read_f64::<LittleEndian>()?;
        let maxalt = reader.read_f64::<LittleEndian>()?;
        let avgint = reader.read_f64::<LittleEndian>()?;
        let maxint = reader.read_f64::<LittleEndian>()?;
        let devint = reader.read_f64::<LittleEndian>()?;
        let timeunit = TimeUnit::from_u8(reader.read_u8()?)?;
        let timeinfo = TimeInfo::from_u8(reader.read_u8()?)?;

        let mut timezone = [0; 16];
        read_full(&mut reader, &mut timezone)?;
        let mut location = [0; 16];
        read_full(&mut reader, &mut location)?;
        let mut device = [0; 32];
        read_full(&mut reader, &mut device)?;
        let mut reserved = [0; 32];
        read_full(&mut reader, &mut reserved)?;
        let mut project = [0; 32];
        read_full(&mut reader, &mut project)?;
        let mut company = [0; 32];
        read_full(&mut reader, &mut company)?;
        let mut reserved2 = [0; 32];
        read_full(&mut reader, &mut reserved2)?;

        let _ = reader.seek(SeekFrom::Start(data_offset as u64))?;

        Ok(Reader {
            avgint: avgint,
            company: company,
            day: day,
            device: device,
            devint: devint,
            entries: entries,
            location: location,
            maxalt: maxalt,
            maxint: maxint,
            maxlat: maxlat,
            maxlon: maxlon,
            minalt: minalt,
            minlat: minlat,
            minlon: minlon,
            month: month,
            position: 0,
            project: project,
            reader: reader,
            timeinfo: timeinfo,
            timeunit: timeunit,
            timezone: timezone,
            version: version,
            year: year,
        })
    }

    /// Reads a point from the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::pof::Reader;
    /// let mut reader = Reader::from_path("data/sbet_mission_1.pof").unwrap();
    /// let point = reader.read_point().unwrap();
    /// ```
    pub fn read_point(&mut self) -> Result<Option<Point>> {
        if self.position == self.entries {
            return Ok(None);
        }

        let time = self.reader.read_f64::<LittleEndian>()?;
        let longitude = self.reader.read_f64::<LittleEndian>()?;
        let latitude = self.reader.read_f64::<LittleEndian>()?;
        let altitude = self.reader.read_f64::<LittleEndian>()?;
        let roll = self.reader.read_f64::<LittleEndian>()?;
        let pitch = self.reader.read_f64::<LittleEndian>()?;
        let yaw = self.reader.read_f64::<LittleEndian>()?;
        let distance = if self.version.has_distance() {
            Some(self.reader.read_f64::<LittleEndian>()?)
        } else {
            None
        };

        self.position += 1;

        Ok(Some(Point {
            time: time,
            longitude: Radians::from_degrees(longitude),
            latitude: Radians::from_degrees(latitude),
            altitude: altitude,
            roll: Radians::from_degrees(roll),
            pitch: Radians::from_degrees(pitch),
            yaw: Radians::from_degrees(yaw),
            distance: distance,
            ..Default::default()
        }))
    }
}

impl<R: Read + Seek> IntoIterator for Reader<R> {
    type Item = Point;
    type IntoIter = ReaderIterator<R>;
    fn into_iter(self) -> Self::IntoIter {
        ReaderIterator { reader: self }
    }
}

/// An iterator over a pof reader.
#[derive(Debug)]
pub struct ReaderIterator<R: Read + Seek> {
    reader: Reader<R>,
}

impl<R: Read + Seek> Iterator for ReaderIterator<R> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_point().unwrap()
    }
}

/// pof file version.
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
    /// use pos::pof::Version;
    /// Version::new(1, 1);
    /// ```
    pub fn new(major: u16, minor: u16) -> Version {
        Version {
            major: major,
            minor: minor,
        }
    }

    fn has_distance(&self) -> bool {
        self.minor >= 1
    }
}

/// Seconds format.
#[derive(Clone, Copy, Debug)]
pub enum TimeUnit {
    /// Normalized time is referenced to some start point, allowing for higher precision.
    Normalized,
    /// GPS day time.
    Day,
    /// GPS week time.
    Week,
}

impl TimeUnit {
    fn from_u8(n: u8) -> Result<TimeUnit> {
        match n {
            0 => Ok(TimeUnit::Normalized),
            1 => Ok(TimeUnit::Day),
            2 => Ok(TimeUnit::Week),
            _ => Err(Error::InvalidTimeUnit(n)),
        }
    }
}

/// Time format.
#[derive(Clone, Copy, Debug)]
pub enum TimeInfo {
    /// GPS time.
    Gps,
    /// UTC time.
    Utc,
    /// Unknown time information.
    Unknown,
}

impl TimeInfo {
    fn from_u8(n: u8) -> Result<TimeInfo> {
        match n {
            0 => Ok(TimeInfo::Gps),
            1 => Ok(TimeInfo::Utc),
            2 => Ok(TimeInfo::Unknown),
            _ => Err(Error::InvalidTimeInfo(n)),
        }
    }
}

impl<R: Debug + Seek + Read> Source for Reader<R> {
    fn source(&mut self) -> Result<Option<Point>> {
        self.read_point()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header() {
        let reader = Reader::from_path("data/sbet_mission_1.pof").unwrap();
        assert_eq!(Version::new(1, 1), reader.version);
        assert_eq!(2015, reader.year);
        assert_eq!(4, reader.month);
        assert_eq!(29, reader.day);
        assert_eq!(1114521, reader.entries);
    }

    #[test]
    fn point() {
        let mut reader = Reader::from_path("data/sbet_mission_1.pof").unwrap();
        let point = reader.read_point().unwrap().unwrap();
        assert_eq!(5.380900320500246e4, point.time);
        assert_eq!(-107.8941420696491, point.longitude.to_degrees());
        assert_eq!(3.852696630463423e1, point.latitude.to_degrees());
        assert_eq!(1721.1666764324254, point.altitude);
        assert_eq!(-3.5218866203789795e-1, point.roll.to_degrees());
        assert_eq!(2.3209047516182637, point.pitch.to_degrees());
        assert_eq!(359.62872162328546, point.yaw.to_degrees());
        assert_eq!(0.0, point.distance.unwrap());
    }

    #[test]
    fn iter() {
        let reader = Reader::from_path("data/sbet_mission_1.pof").unwrap();
        let points: Vec<_> = reader.into_iter().collect();
        assert_eq!(1114521, points.len());
    }
}
