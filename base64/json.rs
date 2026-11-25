//! JSON helpers for base64-encoded byte buffers.
//!
//! Use with the stdx `json` derive:
//!
//! ```ignore
//! #[derive(json::Encode, json::Decode)]
//! struct Foo {
//!     #[json(with = "base64")]
//!     key: [u8; 32],
//! }
//! ```
//!
//! The field is encoded as a standard, padded base64 string. Decoding accepts
//! the same form and rejects anything that [`decode`] rejects.

use alloc::{borrow::Cow, vec::Vec};

use ::json::{Decoder, Encoder, ErrorKind, Write};

use crate::{Alphabet, DecodeError, decode, decode_array};

/// A type that can be decoded from a base64 string.
pub trait FromEncoded: Sized {
    /// Decodes `text` into `Self`.
    fn from_encoded(text: &str) -> Result<Self, DecodeError>;
}

impl<const N: usize> FromEncoded for [u8; N] {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode_array::<N>(text.as_bytes(), Alphabet::Standard)
    }
}

impl FromEncoded for Vec<u8> {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode(text.as_bytes(), Alphabet::Standard)
    }
}

impl<'a> FromEncoded for Cow<'a, [u8]> {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode(text.as_bytes(), Alphabet::Standard).map(Cow::Owned)
    }
}

#[cfg(feature = "small_collections")]
impl<const N: usize> FromEncoded for small_collections::SmallVec<u8, N> {
    fn from_encoded(text: &str) -> Result<Self, DecodeError> {
        decode(text.as_bytes(), Alphabet::Standard).map(|bytes| small_collections::SmallVec::from_slice(&bytes))
    }
}

/// Encodes a byte buffer as a base64 string.
pub fn serialize<T, W>(value: &T, encoder: &mut Encoder<W>) -> Result<(), W::Error>
where
    T: AsRef<[u8]> + ?Sized,
    W: Write,
{
    encoder.string(&crate::encode(value.as_ref(), Alphabet::Standard))
}

/// Decodes a base64 string into `T`.
pub fn deserialize<'a, T: FromEncoded>(decoder: &mut Decoder<'a>) -> ::json::Result<T> {
    let text = decoder.decode_string()?;
    T::from_encoded(&text).map_err(|_| decoder.error(ErrorKind::Message("invalid base64")))
}
