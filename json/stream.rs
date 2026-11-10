//! Streaming and newline-delimited JSON helpers.

use core::marker::PhantomData;

#[cfg(feature = "std")]
use crate::decode::DecodeOwned;
use crate::{
    decode::{Decode, Decoder},
    error::Result,
};

/// An iterator over consecutive JSON values in a byte slice.
///
/// Values may be separated by any JSON whitespace, which covers JSON Lines /
/// NDJSON input.
pub struct StreamDeserializer<'a, T> {
    decoder: Decoder<'a>,
    done: bool,
    marker: PhantomData<T>,
}

impl<'a, T: Decode<'a>> StreamDeserializer<'a, T> {
    /// Creates a stream over `input`.
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            decoder: Decoder::new(input),
            done: false,
            marker: PhantomData,
        }
    }

    /// Returns the byte offset of the next value.
    #[inline]
    pub fn position(&self) -> usize {
        self.decoder.position()
    }
}

impl<'a, T: Decode<'a>> Iterator for StreamDeserializer<'a, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.decoder.skip_ws() {
            Ok(()) => {}
            Err(error) => {
                self.done = true;
                return Some(Err(error));
            }
        }
        if self.decoder.remaining().is_empty() {
            self.done = true;
            return None;
        }
        match self.decoder.decode::<T>() {
            Ok(value) => Some(Ok(value)),
            Err(error) => {
                self.done = true;
                Some(Err(error))
            }
        }
    }
}

/// Deserializes one JSON value from a `std::io::Read`.
///
/// The reader is consumed to EOF and buffered in memory. Apply an external
/// size limit when reading untrusted streams.
#[cfg(feature = "std")]
pub fn from_reader<T: DecodeOwned, R: std::io::Read>(mut reader: R) -> std::io::Result<T> {
    let mut buffer = alloc::vec::Vec::new();
    reader.read_to_end(&mut buffer)?;
    T::decode_owned(&buffer).map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}

/// An iterator over JSON Lines.
///
/// Each line is read into a reusable `String`; a single very long line can use
/// memory proportional to its length. Apply an external line-length limit for
/// untrusted input.
#[cfg(feature = "std")]
pub struct JsonLines<R, T> {
    reader: R,
    buffer: alloc::string::String,
    marker: PhantomData<T>,
}

#[cfg(feature = "std")]
impl<R: std::io::BufRead, T> JsonLines<R, T> {
    /// Creates a JSON Lines iterator.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: alloc::string::String::new(),
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<R: std::io::BufRead, T: DecodeOwned> Iterator for JsonLines<R, T> {
    type Item = std::io::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buffer.clear();
            match self.reader.read_line(&mut self.buffer) {
                Ok(0) => return None,
                Ok(_) => {
                    let line = self.buffer.trim();
                    if line.is_empty() {
                        continue;
                    }
                    return Some(
                        T::decode_owned(line.as_bytes())
                            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
                    );
                }
                Err(error) => return Some(Err(error)),
            }
        }
    }
}
