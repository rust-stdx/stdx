//! Serde `serialize`/`deserialize` helpers for base32-encoded byte buffers.
//!
//! Use with `#[serde(with = "base32")]` on byte fields. In human-readable
//! formats (e.g. JSON) the field is serialized as an RFC 4648 base32 string;
//! in binary formats the raw bytes are used directly.
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Foo {
//!     #[serde(with = "base32")]
//!     key: [u8; 32],
//! }
//! ```
//!
//! # Feature-dependent target types
//!
//! * `serde`: `[u8; N]`
//! * `serde` + `alloc`: `[u8; N]` and `Vec<u8>`
//! * `small_collections`: `[u8; N]`, `Vec<u8>`, and
//!   [`SmallVec<u8, N>`](small_collections::SmallVec) (implies `serde` and
//!   `alloc`)
//!
//! `&[u8]` and `Cow<[u8]>` can borrow decoded bytes from binary formats.
//! Human-readable formats cannot borrow decoded bytes out of encoded text, so
//! `&[u8]` returns an error there; use `Cow<[u8]>` or an owned container instead.
//! Borrowed fields must opt in with `#[serde(borrow, with = "base32")]` so
//! serde adds the required lifetime bound.
//!
//! Values whose encoded form fits in 256 bytes are encoded into a stack
//! buffer, so serialization does not allocate. Larger values use one
//! allocation when `alloc` is enabled, or stream through
//! [`Serializer::collect_str`] when it is not.

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, vec::Vec};
use core::{fmt, marker::PhantomData};

use serde::{
    Deserializer, Serializer,
    de::{self, Visitor},
};
#[cfg(feature = "small_collections")]
use small_collections::SmallVec;

use crate::{Alphabet, DecodeError, decode_into, decoded_length, encode_into, strip_padding_info};
#[cfg(feature = "alloc")]
use crate::{decode, encode};

/// Number of encoded bytes that fit in the stack fast path.
const STACK_BUFFER_SIZE: usize = 256;

/// Input bytes processed per streaming chunk.
#[cfg(any(not(feature = "alloc"), test))]
const STREAM_INPUT_CHUNK: usize = STACK_BUFFER_SIZE / 8 * 5;

/// Error returned by [`FromEncoded`] implementations.
///
/// This is an implementation detail of the serde helpers.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromEncodedError {
    /// Decoding the input failed.
    Decode(DecodeError),
    /// The target requires borrowed bytes, but the format only provides an
    /// encoded string or an ephemeral byte slice.
    BorrowedNotSupported,
}

impl From<DecodeError> for FromEncodedError {
    fn from(error: DecodeError) -> Self {
        Self::Decode(error)
    }
}

impl fmt::Display for FromEncodedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(error) => error.fmt(f),
            Self::BorrowedNotSupported => f.write_str(
                "borrowed byte slices can only be deserialized from binary formats; use Vec<u8> or Cow<[u8]> instead",
            ),
        }
    }
}

impl core::error::Error for FromEncodedError {}

/// Serializes `data` to a base32 string (human-readable) or raw bytes (binary
/// formats).
///
/// Typically used via `#[serde(with = "base32")]`. `Vec<u8>`, `[u8; N]`, and
/// `SmallVec<u8, N>` values coerce to `&[u8]`, so no additional setup is
/// required. See the module documentation for an example.
pub fn serialize<S: Serializer>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    if serializer.is_human_readable() {
        serialize_human_readable(data, serializer)
    } else {
        serializer.serialize_bytes(data)
    }
}

fn serialize_human_readable<S: Serializer>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    if let Some(len) = encoded_len(data.len())
        && len <= STACK_BUFFER_SIZE
    {
        let mut buffer = [0u8; STACK_BUFFER_SIZE];
        encode_into(&mut buffer[..len], data, Alphabet::Rfc4648)
            .map_err(|_| serde::ser::Error::custom("failed to encode base32"))?;
        let text = core::str::from_utf8(&buffer[..len])
            .map_err(|_| serde::ser::Error::custom("base32 encoder produced non-UTF-8"))?;
        return serializer.serialize_str(text);
    }

    serialize_fallback(data, serializer)
}

#[cfg(feature = "alloc")]
fn serialize_fallback<S: Serializer>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&encode(data, Alphabet::Rfc4648))
}

#[cfg(not(feature = "alloc"))]
fn serialize_fallback<S: Serializer>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&StreamDisplay(data))
}

/// Deserializes a base32 string (human-readable formats) or raw bytes (binary
/// formats) into `T`.
///
/// `T` is inferred from the field type when used via
/// `#[serde(with = "base32")]`. Supported types depend on the enabled
/// features; see the module documentation.
///
/// # Errors
///
/// Returns an error if the input is not valid base32, or if a fixed-size target
/// does not have the expected length.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromEncoded<'de>,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(BytesVisitor(PhantomData))
    } else {
        deserializer.deserialize_bytes(BytesVisitor(PhantomData))
    }
}

struct BytesVisitor<T>(PhantomData<T>);

impl<'de, T: FromEncoded<'de>> Visitor<'de> for BytesVisitor<T> {
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a base32-encoded string or raw bytes")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<T, E> {
        T::from_encoded(v.as_bytes()).map_err(de::Error::custom)
    }

    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<T, E> {
        T::from_decoded(v).map_err(de::Error::custom)
    }

    fn visit_borrowed_bytes<E: de::Error>(self, v: &'de [u8]) -> Result<T, E> {
        T::from_borrowed(v).map_err(de::Error::custom)
    }
}

/// Byte containers that the serde helpers can deserialize into.
///
/// This trait is an implementation detail of the serde helpers; it does not
/// need to be imported to use `#[serde(with = "base32")]`. The target type is
/// inferred from the field.
///
/// It is implemented for `Vec<u8>`, `[u8; N]`, `&[u8]`, and
/// [`SmallVec<u8, N>`](small_collections::SmallVec), depending on the enabled
/// features. Some care is required for borrowed slices:
///
/// * `&[u8]` and `Cow<[u8]>` can only borrow from binary formats that provide
///   borrowed bytes. Human-readable formats return
///   [`FromEncodedError::BorrowedNotSupported`] for `&[u8]`.
/// * `Cow<[u8]>` owns the decoded data in human-readable formats and can borrow
///   it in binary formats.
#[doc(hidden)]
pub trait FromEncoded<'de>: Sized {
    /// Creates a value from a base32-encoded string.
    ///
    /// # Errors
    ///
    /// Returns an error if `encoded` is not valid base32, if `Self` has a
    /// fixed length that does not match the decoded input, or if `Self` can
    /// only be built from borrowed bytes.
    fn from_encoded(encoded: &[u8]) -> Result<Self, FromEncodedError>;

    /// Creates a value from already-decoded bytes (binary formats).
    ///
    /// # Errors
    ///
    /// Returns an error if `Self` has a fixed length that does not match
    /// `decoded`, or if `Self` can only be built from borrowed bytes.
    fn from_decoded(decoded: &[u8]) -> Result<Self, FromEncodedError>;

    /// Creates a value from bytes borrowed from the deserializer.
    ///
    /// The default implementation delegates to [`Self::from_decoded`], which
    /// copies the input for owned containers.
    ///
    /// # Errors
    ///
    /// Returns an error if the decoded input does not match the target type.
    fn from_borrowed(decoded: &'de [u8]) -> Result<Self, FromEncodedError> {
        Self::from_decoded(decoded)
    }
}

#[cfg(feature = "alloc")]
impl<'de> FromEncoded<'de> for Vec<u8> {
    fn from_encoded(encoded: &[u8]) -> Result<Self, FromEncodedError> {
        Ok(decode(encoded, Alphabet::Rfc4648)?)
    }

    fn from_decoded(decoded: &[u8]) -> Result<Self, FromEncodedError> {
        Ok(decoded.to_vec())
    }
}

impl<'de, const N: usize> FromEncoded<'de> for [u8; N] {
    fn from_encoded(encoded: &[u8]) -> Result<Self, FromEncodedError> {
        if decoded_len(encoded)? != N {
            return Err(DecodeError::InvalidLength.into());
        }
        let mut output = [0u8; N];
        decode_into(&mut output, encoded, Alphabet::Rfc4648)?;
        Ok(output)
    }

    fn from_decoded(decoded: &[u8]) -> Result<Self, FromEncodedError> {
        decoded.try_into().map_err(|_| DecodeError::InvalidLength.into())
    }
}

#[cfg(feature = "small_collections")]
impl<'de, const N: usize> FromEncoded<'de> for SmallVec<u8, N> {
    fn from_encoded(encoded: &[u8]) -> Result<Self, FromEncodedError> {
        let len = decoded_len(encoded)?;
        let mut output = SmallVec::with_capacity(len);
        output.resize(len, 0);
        decode_into(output.as_mut_slice(), encoded, Alphabet::Rfc4648)?;
        Ok(output)
    }

    fn from_decoded(decoded: &[u8]) -> Result<Self, FromEncodedError> {
        Ok(SmallVec::from_slice(decoded))
    }
}

impl<'de> FromEncoded<'de> for &'de [u8] {
    fn from_encoded(_: &[u8]) -> Result<Self, FromEncodedError> {
        Err(FromEncodedError::BorrowedNotSupported)
    }

    fn from_decoded(_: &[u8]) -> Result<Self, FromEncodedError> {
        Err(FromEncodedError::BorrowedNotSupported)
    }

    fn from_borrowed(decoded: &'de [u8]) -> Result<Self, FromEncodedError> {
        Ok(decoded)
    }
}

#[cfg(feature = "alloc")]
impl<'de> FromEncoded<'de> for Cow<'de, [u8]> {
    fn from_encoded(encoded: &[u8]) -> Result<Self, FromEncodedError> {
        Ok(Cow::Owned(decode(encoded, Alphabet::Rfc4648)?))
    }

    fn from_decoded(decoded: &[u8]) -> Result<Self, FromEncodedError> {
        Ok(Cow::Owned(decoded.to_vec()))
    }

    fn from_borrowed(decoded: &'de [u8]) -> Result<Self, FromEncodedError> {
        Ok(Cow::Borrowed(decoded))
    }
}

fn decoded_len(encoded: &[u8]) -> Result<usize, DecodeError> {
    let (content_len, _) = strip_padding_info(encoded, Alphabet::Rfc4648.is_padded())?;
    decoded_length(content_len)
}

fn encoded_len(input_len: usize) -> Option<usize> {
    crate::encoded_length(input_len, Alphabet::Rfc4648.is_padded())
}

/// Streaming encoder used when `alloc` is disabled and the value does not fit
/// in the stack buffer. It is also compiled for tests so both encoding paths
/// can be compared with `alloc` enabled.
#[cfg(any(not(feature = "alloc"), test))]
struct StreamDisplay<'a>(&'a [u8]);

#[cfg(any(not(feature = "alloc"), test))]
impl fmt::Display for StreamDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = [0u8; STACK_BUFFER_SIZE];
        for chunk in self.0.chunks(STREAM_INPUT_CHUNK) {
            let len = encoded_len(chunk.len()).ok_or(fmt::Error)?;
            encode_into(&mut buffer[..len], chunk, Alphabet::Rfc4648).map_err(|_| fmt::Error)?;
            let text = core::str::from_utf8(&buffer[..len]).map_err(|_| fmt::Error)?;
            f.write_str(text)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// Inputs around the 256-byte stack boundary and every streaming chunk
    /// boundary.
    fn sizes() -> Vec<usize> {
        let mut sizes = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 95, 96, 97, 127, 128, 129, 159, 160, 161,
            191, 192, 193, 255, 256, 257,
        ];
        for multiple in 1..=8 {
            let boundary = multiple * STREAM_INPUT_CHUNK;
            sizes.extend([boundary - 1, boundary, boundary + 1]);
        }
        sizes.extend([2047, 2048, 2049, 4095, 4096, 4097]);
        sizes.sort_unstable();
        sizes.dedup();
        sizes
    }

    fn payload(size: usize) -> Vec<u8> {
        (0..size).map(|i| (i % 251) as u8).collect()
    }

    fn stack_encode(data: &[u8]) -> Option<Vec<u8>> {
        let len = encoded_len(data.len())?;
        if len > STACK_BUFFER_SIZE {
            return None;
        }

        let mut buffer = [0u8; STACK_BUFFER_SIZE];
        encode_into(&mut buffer[..len], data, Alphabet::Rfc4648).unwrap();
        Some(buffer[..len].to_vec())
    }

    #[test]
    fn encoded_boundaries() {
        assert_eq!(encoded_len(0), Some(0));
        assert_eq!(encoded_len(158), Some(256));
        assert_eq!(encoded_len(159), Some(256));
        assert_eq!(encoded_len(160), Some(256));
        assert_eq!(encoded_len(161), Some(264));

        assert!(stack_encode(&payload(158)).is_some());
        assert!(stack_encode(&payload(159)).is_some());
        assert!(stack_encode(&payload(160)).is_some());
        assert!(stack_encode(&payload(161)).is_none());
    }

    #[test]
    fn stack_stream_and_encode_are_equivalent() {
        for size in sizes() {
            let data = payload(size);
            let expected = crate::encode(&data, Alphabet::Rfc4648);

            if let Some(stack) = stack_encode(&data) {
                assert_eq!(stack, expected.as_bytes(), "stack encoding differs at size {size}");
            }

            let streamed = format!("{}", StreamDisplay(&data));
            assert_eq!(streamed, expected, "stream encoding differs at size {size}");
        }
    }

    #[test]
    fn json_boundary_roundtrips() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Data<const N: usize>(#[serde(with = "crate")] [u8; N]);

        fn check<const N: usize>() {
            let data = Data(core::array::from_fn(|i| (i % 251) as u8));
            let json = serde_json::to_string(&data).unwrap();
            let expected = format!("\"{}\"", crate::encode(data.0.as_slice(), Alphabet::Rfc4648));
            assert_eq!(json, expected, "JSON differs at N = {N}");

            let deserialized: Data<N> = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, data, "round-trip differs at N = {N}");
        }

        check::<0>();
        check::<1>();
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<6>();
        check::<7>();
        check::<8>();
        check::<15>();
        check::<16>();
        check::<17>();
        check::<31>();
        check::<32>();
        check::<33>();
        check::<63>();
        check::<64>();
        check::<65>();
        check::<95>();
        check::<96>();
        check::<97>();
        check::<127>();
        check::<128>();
        check::<129>();
        check::<159>();
        check::<160>();
        check::<161>();
        check::<191>();
        check::<192>();
        check::<193>();
        check::<255>();
        check::<256>();
        check::<257>();
        check::<383>();
        check::<384>();
        check::<385>();
        check::<511>();
        check::<512>();
        check::<513>();
        check::<767>();
        check::<768>();
        check::<769>();
        check::<1023>();
        check::<1024>();
        check::<1025>();
        check::<2048>();
        check::<4096>();
    }

    #[test]
    fn binary_roundtrip() {
        use serde_test::{Configure, Token, assert_tokens};

        static LARGE: [u8; 1024] = [0xAB; 1024];

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Data<const N: usize>(#[serde(with = "crate")] [u8; N]);

        assert_tokens(
            &Data(*b"abcd").compact(),
            &[
                Token::NewtypeStruct {
                    name: "Data",
                },
                Token::Bytes(b"abcd"),
            ],
        );
        assert_tokens(
            &Data::<1024>(LARGE).compact(),
            &[
                Token::NewtypeStruct {
                    name: "Data",
                },
                Token::Bytes(&LARGE),
            ],
        );
    }

    #[test]
    fn borrowed_slice_supports_binary_only() {
        use serde_test::{Configure, Token, assert_tokens};

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Borrowed<'a>(#[serde(borrow, with = "crate")] &'a [u8]);

        let json = serde_json::to_string(&Borrowed(b"abcd")).unwrap();
        assert_eq!(json, "\"MFRGGZA=\"");

        let error = serde_json::from_str::<Borrowed>(&json).unwrap_err().to_string();
        assert!(error.contains("borrowed byte slices"), "{error}");

        let borrowed = <&[u8] as FromEncoded<'_>>::from_borrowed(b"abcd").unwrap();
        assert_eq!(borrowed.as_ptr(), b"abcd".as_ptr());
        assert!(<&[u8] as FromEncoded<'_>>::from_encoded(b"MFRGGZA=").is_err());
        assert!(<&[u8] as FromEncoded<'_>>::from_decoded(b"abcd").is_err());

        assert_tokens(
            &Borrowed(b"abcd").compact(),
            &[
                Token::NewtypeStruct {
                    name: "Borrowed",
                },
                Token::BorrowedBytes(b"abcd"),
            ],
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn cow_borrows_in_binary_and_owns_in_json() {
        use std::borrow::Cow;

        use serde_test::{Configure, Token, assert_tokens};

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Data<'a>(#[serde(borrow, with = "crate")] Cow<'a, [u8]>);

        let json = serde_json::to_string(&Data(Cow::Borrowed(b"abcd"))).unwrap();
        assert_eq!(json, "\"MFRGGZA=\"");
        let decoded: Data = serde_json::from_str(&json).unwrap();
        assert!(matches!(decoded.0, Cow::Owned(_)));
        assert_eq!(&*decoded.0, b"abcd");

        let borrowed = <Cow<'_, [u8]> as FromEncoded<'_>>::from_borrowed(b"abcd").unwrap();
        assert!(matches!(borrowed, Cow::Borrowed(_)));
        assert_eq!(borrowed.as_ptr(), b"abcd".as_ptr());

        let owned = <Cow<'_, [u8]> as FromEncoded<'_>>::from_encoded(b"MFRGGZA=").unwrap();
        assert!(matches!(owned, Cow::Owned(_)));
        assert_eq!(&*owned, b"abcd");

        let owned = <Cow<'_, [u8]> as FromEncoded<'_>>::from_decoded(b"abcd").unwrap();
        assert!(matches!(owned, Cow::Owned(_)));
        assert_eq!(&*owned, b"abcd");

        assert_tokens(
            &Data(Cow::Borrowed(b"abcd")).compact(),
            &[
                Token::NewtypeStruct {
                    name: "Data",
                },
                Token::BorrowedBytes(b"abcd"),
            ],
        );
        assert_tokens(
            &Data(Cow::Borrowed(b"abcd")).compact(),
            &[
                Token::NewtypeStruct {
                    name: "Data",
                },
                Token::Bytes(b"abcd"),
            ],
        );
    }

    #[test]
    fn small_serialization_does_not_allocate() {
        use std::io;

        use alloc_counter::measure;

        #[derive(Serialize)]
        struct Data<const N: usize>(#[serde(with = "crate")] [u8; N]);

        let data = Data::<32>([7u8; 32]);
        serde_json::to_writer(io::sink(), &data).unwrap();

        let (_, counters) = measure(|| {
            serde_json::to_writer(io::sink(), &data).unwrap();
        });
        assert_eq!(counters.calls, 0);
        assert_eq!(counters.bytes, 0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn large_serialization_allocates_once() {
        use std::io;

        use alloc_counter::measure;

        #[derive(Serialize)]
        struct Data<const N: usize>(#[serde(with = "crate")] [u8; N]);

        const N: usize = 1024;
        let data = Data::<N>([7u8; N]);
        serde_json::to_writer(io::sink(), &data).unwrap();

        let (_, counters) = measure(|| {
            serde_json::to_writer(io::sink(), &data).unwrap();
        });
        assert_eq!(counters.calls, 1);
        assert_eq!(counters.bytes, encoded_len(N).unwrap() as u64);
    }

    #[cfg(not(feature = "alloc"))]
    #[test]
    fn large_serialization_without_alloc_does_not_allocate() {
        use std::io;

        use alloc_counter::measure;

        #[derive(Serialize)]
        struct Data<const N: usize>(#[serde(with = "crate")] [u8; N]);

        const N: usize = 1024;
        let data = Data::<N>([7u8; N]);
        serde_json::to_writer(io::sink(), &data).unwrap();

        let (_, counters) = measure(|| {
            serde_json::to_writer(io::sink(), &data).unwrap();
        });
        assert_eq!(counters.calls, 0);
        assert_eq!(counters.bytes, 0);
    }
}
