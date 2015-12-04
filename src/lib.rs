//! Small rust library for reading IMU/GNSS position data files.
//!
//! These files come in a few sorts:
//!
//! - `pos`: ASCII format
//! - `sbet`: binary format, with optional associated `rmsmsg` accuracy file
//! - `pof`: Riegl's binary format, with optional associated `poq` accuracy file

#![deny(fat_ptr_transmutes, missing_copy_implementations, missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts, unsafe_code, unused_extern_crates, unused_import_braces, unused_qualifications, unused_results, variant_size_differences)]

extern crate byteorder;

mod io;
pub mod error;
pub mod interpolate;
pub mod pof;
pub mod point;
pub mod poq;
pub mod pos;
pub mod sbet;
pub mod source;
pub mod units;

pub use error::Error;
pub use interpolate::Interpolator;
pub use source::{Source, AccuracySource, FileSource, FileAccuracySource, CombinedSource};
pub use point::{Point, Accuracy};
pub use units::Radians;

use std::result;

/// Our custom result type.
pub type Result<T> = result::Result<T, Error>;
