//! Byte-string helpers.
//!
//! `Vec<T>` and `&[T]` are CBOR arrays. A field annotated with
//! `#[cbor(bytes)]` uses the [`DecodeBytes`] trait instead, which constructs
//! the target from a CBOR byte string.

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{
    decode::{Decode, Decoder},
    error::Result,
};

/// Construct a value from a CBOR byte string.
///
/// This is used by `#[cbor(bytes)]` fields. It is implemented for the common
/// byte containers; users can implement it for their own types.
pub trait DecodeBytes<'b>: Sized {
    /// Decode a definite- or indefinite-length byte string.
    fn decode_bytes(decoder: &mut Decoder<'b>) -> Result<Self>;
}

#[cfg(feature = "alloc")]
impl<'b> DecodeBytes<'b> for Vec<u8> {
    #[inline]
    fn decode_bytes(decoder: &mut Decoder<'b>) -> Result<Self> {
        decoder.bytes_owned()
    }
}

impl<'b, 'a> DecodeBytes<'b> for &'a [u8]
where
    'b: 'a,
{
    #[inline]
    fn decode_bytes(decoder: &mut Decoder<'b>) -> Result<Self> {
        decoder.bytes()
    }
}

#[cfg(feature = "alloc")]
impl<'b> DecodeBytes<'b> for Box<[u8]> {
    #[inline]
    fn decode_bytes(decoder: &mut Decoder<'b>) -> Result<Self> {
        Ok(decoder.bytes_owned()?.into_boxed_slice())
    }
}

impl<'b, T: DecodeBytes<'b>> DecodeBytes<'b> for Option<T> {
    #[inline]
    fn decode_bytes(decoder: &mut Decoder<'b>) -> Result<Self> {
        if decoder.peek_byte()? == 0xf6 {
            decoder.null()?;
            Ok(None)
        } else {
            Ok(Some(T::decode_bytes(decoder)?))
        }
    }
}

#[cfg(feature = "alloc")]
impl<'b, 'a> DecodeBytes<'b> for Cow<'a, [u8]>
where
    'b: 'a,
{
    fn decode_bytes(decoder: &mut Decoder<'b>) -> Result<Self> {
        let head = decoder.peek_byte()?;
        if head & 0xe0 == 0x40 && head & 0x1f == 31 {
            Ok(Cow::Owned(decoder.bytes_owned()?))
        } else {
            Ok(Cow::Borrowed(decoder.bytes()?))
        }
    }
}

/// Encode a value as a CBOR byte string.
///
/// This is a convenience for encoders outside a `#[cbor(bytes)]` field.
#[inline]
pub fn encode_bytes<W: crate::Write>(
    value: &impl AsRef<[u8]>,
    encoder: &mut crate::Encoder<W>,
) -> core::result::Result<(), W::Error> {
    encoder.bytes(value.as_ref())
}

/// Decode a CBOR byte string.
#[inline]
pub fn decode_bytes<'b, T: DecodeBytes<'b>>(decoder: &mut Decoder<'b>) -> Result<T> {
    T::decode_bytes(decoder)
}

impl<'b, 'a> Decode<'b> for &'a [u8]
where
    'b: 'a,
{
    #[inline]
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self> {
        decoder.bytes()
    }
}

#[cfg(feature = "alloc")]
impl<'b, 'a> Decode<'b> for Cow<'a, [u8]>
where
    'b: 'a,
{
    #[inline]
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self> {
        <Cow<'a, [u8]> as DecodeBytes<'b>>::decode_bytes(decoder)
    }
}
