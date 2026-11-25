//! JSON helpers for hex-encoded byte buffers.
//!
//! Use with the stdx `json` derive:
//!
//! ```ignore
//! #[derive(json::Encode, json::Decode)]
//! struct Foo {
//!     #[json(with = "hex")]
//!     key: [u8; 32],
//! }
//! ```
//!
//! The field is encoded as lowercase hexadecimal. Decoding accepts upper- or
//! lowercase digits and rejects anything that [`decode`]
//! rejects.

use alloc::{borrow::Cow, vec::Vec};

use ::json::{Decoder, Encoder, ErrorKind, Write};

use crate::{DecodeError, decode, decode_array};

/// A type that can be decoded from a hex string.
pub trait FromEncoded: Sized {
    /// Decodes `text` into `Self`.
    fn from_encoded(text: &str) -> Result<Self, DecodeError>;
}

impl<const N: usize> FromEncoded for [u8; N] {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode_array::<N>(text.as_bytes())
    }
}

impl FromEncoded for Vec<u8> {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode(text)
    }
}

impl<'a> FromEncoded for Cow<'a, [u8]> {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode(text).map(Cow::Owned)
    }
}

#[cfg(feature = "small_collections")]
impl<const N: usize> FromEncoded for small_collections::SmallVec<u8, N> {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode(text).map(|bytes| small_collections::SmallVec::from_slice(&bytes))
    }
}

/// Encodes a byte buffer as a lowercase hex string.
pub fn serialize<T, W>(value: &T, encoder: &mut Encoder<W>) -> Result<(), W::Error>
where
    T: AsRef<[u8]> + ?Sized,
    W: Write,
{
    encoder.string(&crate::encode(value.as_ref()))
}

/// Decodes a hex string into `T`.
pub fn deserialize<'a, T: FromEncoded>(decoder: &mut Decoder<'a>) -> ::json::Result<T> {
    let text = decoder.decode_string()?;
    T::from_encoded(&text).map_err(|_| decoder.error(ErrorKind::Message("invalid hex")))
}
