//! Validated raw JSON slices and raw numbers.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt;

use crate::error::{Error, ErrorKind, Result};

/// A validated JSON value borrowed from the input.
///
/// The bytes are guaranteed to contain exactly one well-formed JSON value.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RawValue<'a> {
    bytes: &'a [u8],
}

impl<'a> RawValue<'a> {
    /// Validates and wraps one JSON value.
    pub fn new(bytes: &'a [u8]) -> Result<Self> {
        utf8::from_utf8(bytes).map_err(|error| Error::at(ErrorKind::InvalidUtf8, bytes, error.valid_up_to()))?;
        let mut decoder = crate::decode::Decoder::from_validated_utf8(bytes, crate::decode::Options::default());
        decoder.skip_ws()?;
        let start = decoder.position();
        decoder.skip_value().map_err(|error| decoder.locate_error(error))?;
        let end = decoder.position();
        decoder.finish()?;
        Ok(Self {
            bytes: &bytes[start..end],
        })
    }

    /// Creates a value from bytes that have already been validated.
    #[inline]
    pub(crate) fn from_validated(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
        }
    }

    /// Returns the raw bytes.
    #[inline]
    pub const fn get(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the raw bytes as a string.
    pub fn as_str(&self) -> Result<&'a str> {
        core::str::from_utf8(self.bytes).map_err(|_| Error::new(ErrorKind::InvalidUtf8, 0))
    }

    /// Decodes the raw value as `T`.
    pub fn deserialize<T: crate::Decode<'a>>(&self) -> Result<T> {
        crate::from_slice(self.bytes)
    }

    /// Copies the raw value into an owned buffer.
    #[cfg(feature = "alloc")]
    pub fn into_owned(self) -> RawValueBuf {
        RawValueBuf {
            bytes: self.bytes.to_vec(),
        }
    }
}

impl fmt::Debug for RawValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RawValue")
            .field(&self.as_str().unwrap_or("<invalid>"))
            .finish()
    }
}

impl fmt::Display for RawValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str().unwrap_or("<invalid>"))
    }
}

impl crate::encode::Encode for RawValue<'_> {
    fn encode<W: crate::encode::Write>(
        &self,
        encoder: &mut crate::encode::Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        encoder.raw_validated(self.bytes)
    }
}

/// An owned, validated raw JSON value.
#[cfg(feature = "alloc")]
#[derive(Clone, PartialEq, Eq)]
pub struct RawValueBuf {
    bytes: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl RawValueBuf {
    /// Validates and copies one JSON value.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Result<Self> {
        let mut bytes = bytes.into();
        utf8::from_utf8(&bytes).map_err(|error| Error::at(ErrorKind::InvalidUtf8, &bytes, error.valid_up_to()))?;
        let mut decoder = crate::decode::Decoder::from_validated_utf8(&bytes, crate::decode::Options::default());
        decoder.skip_ws()?;
        let start = decoder.position();
        decoder.skip_value()?;
        let end = decoder.position();
        decoder.finish()?;
        if start != 0 || end != bytes.len() {
            bytes.copy_within(start..end, 0);
            bytes.truncate(end - start);
        }
        Ok(Self {
            bytes,
        })
    }

    /// Copies one already validated JSON value from a slice.
    pub(crate) fn from_validated_slice(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }

    /// Validates and copies one JSON value from a slice.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
        Ok(crate::RawValue::new(bytes)?.into_owned())
    }

    /// Returns the raw bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the raw bytes as a string.
    pub fn as_str(&self) -> Result<&str> {
        core::str::from_utf8(&self.bytes).map_err(|_| Error::new(ErrorKind::InvalidUtf8, 0))
    }

    /// Consumes the value and returns its bytes.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Decodes the raw value as `T`.
    pub fn deserialize<T: for<'a> crate::Decode<'a>>(&self) -> Result<T> {
        crate::from_slice(&self.bytes)
    }
}

#[cfg(feature = "alloc")]
impl fmt::Debug for RawValueBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RawValueBuf")
            .field(&self.as_str().unwrap_or("<invalid>"))
            .finish()
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for RawValueBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str().unwrap_or("<invalid>"))
    }
}

#[cfg(feature = "alloc")]
impl crate::encode::Encode for RawValueBuf {
    fn encode<W: crate::encode::Write>(
        &self,
        encoder: &mut crate::encode::Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        encoder.raw_validated(&self.bytes)
    }
}

/// A JSON number whose exact decimal representation is preserved.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RawNumber<'a> {
    text: &'a str,
}

impl<'a> RawNumber<'a> {
    /// Validates and wraps one JSON number.
    pub fn new(text: &'a str) -> Result<Self> {
        let bytes = text.as_bytes();
        let (end, _) = crate::number::scan(bytes, 0)?;
        if end != bytes.len() {
            return Err(Error::new(ErrorKind::TrailingData, end));
        }
        Ok(Self {
            text,
        })
    }

    #[inline]
    pub(crate) fn from_slice(text: &'a str) -> Self {
        Self {
            text,
        }
    }

    /// Returns the exact decimal representation.
    #[inline]
    pub const fn as_str(&self) -> &'a str {
        self.text
    }

    /// Returns the number as an `f64`.
    pub fn as_f64(&self) -> Result<f64> {
        crate::number::parse_f64(self.text, 0)
    }

    /// Returns the number as a `u64`.
    pub fn as_u64(&self) -> Result<u64> {
        crate::number::parse_u64(self.text, 0)
    }

    /// Returns the number as an `i64`.
    pub fn as_i64(&self) -> Result<i64> {
        crate::number::parse_i64(self.text, 0)
    }

    /// Returns the number as a `u128`.
    pub fn as_u128(&self) -> Result<u128> {
        crate::number::parse_u128(self.text, 0)
    }

    /// Returns the number as an `i128`.
    pub fn as_i128(&self) -> Result<i128> {
        crate::number::parse_i128(self.text, 0)
    }
}

impl fmt::Debug for RawNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RawNumber").field(&self.text).finish()
    }
}

impl fmt::Display for RawNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.text)
    }
}

impl crate::encode::Encode for RawNumber<'_> {
    fn encode<W: crate::encode::Write>(
        &self,
        encoder: &mut crate::encode::Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        encoder.raw_validated(self.text.as_bytes())
    }
}
