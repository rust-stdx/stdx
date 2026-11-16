//! Validated pass-through CBOR items.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{
    canonical::validate_prefix,
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::{Error, Result},
};

/// A borrowed CBOR item that has been validated as exactly one well-formed
/// data item, including UTF-8 validity for text strings.
///
/// Encoding a `RawValue` splices its bytes into the output unchanged, which
/// is useful for signature payloads and protocol fields that must not be
/// re-encoded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RawValue<'b> {
    bytes: &'b [u8],
}

impl<'b> RawValue<'b> {
    /// Validate `bytes` as exactly one well-formed CBOR item.
    ///
    /// The check includes text UTF-8 validity. It does not check
    /// deterministic encoding.
    pub fn new(bytes: &'b [u8]) -> Result<Self> {
        let used = validate_prefix(bytes)?;
        if used != bytes.len() {
            return Err(Error::trailing_data().at(used));
        }
        Ok(RawValue {
            bytes,
        })
    }

    /// Return the encoded bytes.
    #[inline]
    pub const fn as_bytes(&self) -> &'b [u8] {
        self.bytes
    }

    /// Decode the contained item as a typed value.
    #[inline]
    pub fn deserialize<T: Decode<'b>>(&self) -> Result<T> {
        crate::decode::from_slice(self.bytes)
    }
}

impl Encode for RawValue<'_> {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.raw(self.bytes)
    }
}

impl<'b, 'a> Decode<'b> for RawValue<'a>
where
    'b: 'a,
{
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let start = d.position();
        d.skip()?;
        let end = d.position();
        Ok(RawValue {
            bytes: &d.input()[start..end],
        })
    }
}

/// An owned, validated CBOR item.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RawValueBuf {
    bytes: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl RawValueBuf {
    /// Validate `bytes` as exactly one well-formed CBOR item and take
    /// ownership of them.
    pub fn new(bytes: Vec<u8>) -> Result<Self> {
        let used = validate_prefix(&bytes)?;
        if used != bytes.len() {
            return Err(Error::trailing_data().at(used));
        }
        Ok(RawValueBuf {
            bytes,
        })
    }

    /// Return the encoded bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the value and return the encoded bytes.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Decode the contained item as a typed value.
    #[inline]
    pub fn deserialize<T: for<'b> Decode<'b>>(&self) -> Result<T> {
        crate::decode::from_slice(&self.bytes)
    }
}

#[cfg(feature = "alloc")]
impl Encode for RawValueBuf {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.raw(&self.bytes)
    }
}

#[cfg(feature = "alloc")]
impl<'b> Decode<'b> for RawValueBuf {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let start = d.position();
        d.skip()?;
        let end = d.position();
        Ok(RawValueBuf {
            bytes: Vec::from(&d.input()[start..end]),
        })
    }
}
