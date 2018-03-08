//! Our custom errors.

use std::error;
use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError};

/// Our custom error enum.
#[derive(Debug)]
pub enum Error {
    /// This isn't interpolation, this is extrapolation.
    Extrapolation(String),
    /// Cannot convert the u8 into a `TimeInfo`.
    InvalidTimeInfo(u8),
    /// Cannot convert the u8 into a `TimeUnit`.
    InvalidTimeUnit(u8),
    /// Wrapper around `std::io::Error`.
    Io(io::Error),
    /// Wrapper around `std::num::ParseFloatError`.
    ParseFloat(ParseFloatError),
    /// Wrapper around `std::num::ParseIntError`.
    ParseInt(ParseIntError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Extrapolation(_) => "extrapolation",
            Error::InvalidTimeInfo(_) => "invalid time info",
            Error::InvalidTimeUnit(_) => "invalid time unit",
            Error::Io(ref err) => err.description(),
            Error::ParseFloat(ref err) => err.description(),
            Error::ParseInt(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::ParseFloat(ref err) => Some(err),
            Error::ParseInt(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Extrapolation(ref s) => write!(f, "extrapolation error: {}", s),
            Error::InvalidTimeInfo(n) => write!(f, "invalid time info: {}", n),
            Error::InvalidTimeUnit(n) => write!(f, "invalid time unit: {}", n),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::ParseFloat(ref err) => write!(f, "parse float error: {}", err),
            Error::ParseInt(ref err) => write!(f, "parse int error: {}", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}
