//! Utilities for reading and writing


use byteorder::{Error, Result};
use std::io;

/// Reads enough data to fill the buffer, or errors.
///
/// This is taken from the byteorder souce, where it is a private method.
pub fn read_full<R: io::Read + ?Sized>(rdr: &mut R, buf: &mut [u8]) -> Result<()> {
    let mut nread = 0usize;
    while nread < buf.len() {
        match rdr.read(&mut buf[nread..]) {
            Ok(0) => return Err(Error::UnexpectedEOF),
            Ok(n) => nread += n,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(From::from(e)),
        }
    }
    Ok(())
}
