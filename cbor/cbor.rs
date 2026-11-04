//! A fast, low-level CBOR (RFC 8949) codec.
//!
//! This crate is organised around four pieces:
//!
//! - [`Encoder`] and [`Decoder`] for direct, type-directed encoding and
//!   decoding without an intermediate representation.
//! - The [`Encode`] and [`Decode`] traits, which can be derived with
//!   `#[derive(Encode, Decode)]` (feature `derive`).
//! - [`Value`] and [`RawValue`] for dynamic data and validated pass-through
//!   bytes.
//! - Optional serde support (feature `serde`) built on the same encoder and
//!   decoder.
//!
//! # Quick start
//!
//! ```
//! # #[cfg(all(feature = "derive", feature = "alloc"))]
//! # {
//! use cbor::{Decode, Encode};
//!
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! let point = Point { x: 1.5, y: -2.0 };
//! let bytes = cbor::to_vec(&point);
//! let decoded: Point = cbor::from_slice(&bytes).unwrap();
//! assert_eq!(point, decoded);
//! # }
//! ```
//!
//! # Wire format
//!
//! Derived structs are encoded as CBOR maps with text keys by default. Use
//! `#[cbor(array)]` for positional arrays and `#[cbor(key = N)]` for
//! integer-keyed maps (COSE style). Enums use serde-compatible external
//! tagging by default.
//!
//! `Vec<T>` and `&[T]` are encoded as CBOR arrays. To encode a `Vec<u8>` or
//! `&[u8]` field as a CBOR byte string, annotate it with `#[cbor(bytes)]`.
//! The optional `bytes` feature integrates `bytes::Bytes` and
//! `bytes::BytesMut`, which are always encoded as byte strings.
//!
//! # Features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `std` | yes | Standard library, `std::io` helpers and `HashMap` support. |
//! | `alloc` | via `std` | `Vec`, `String`, `Value`, `RawValueBuf` and the owned APIs. |
//! | `derive` | no | `#[derive(Encode, Decode)]`. |
//! | `serde` | no | serde `Serializer`/`Deserializer` integration (requires `alloc`). |
//! | `heapless` | no | Implementations for `heapless` containers. |
//! | `small_collections` | no | Implementations for `SmallVec`/`SmallString`. |
//! | `bytes` | no | Implementations for `bytes::Bytes`/`BytesMut`. |
//!
//! # no_std
//!
//! With `default-features = false`, the crate is `no_std` and has no heap
//! dependency. Encoding into a fixed buffer, decoding borrowed types,
//! `encoded_len`, `skip`, `validate` and `RawValue` all work without an
//! allocator. Enable `alloc` for `Value`, `String`, `Vec` and `to_vec`.
//!
//! # Deterministic encoding
//!
//! [`to_canonical_vec`] and [`to_canonical_slice`] enable deterministic
//! mode. Derived maps are emitted in RFC 8949 core-deterministic key order at
//! compile time, so they are deterministic even in the normal mode; the
//! canonical mode additionally normalizes NaNs and dynamic maps.
//!
//! # Struct and enum encoding
//!
//! The default representation is a text-key map, matching serde/ciborium:
//!
//! ```
//! # #[cfg(all(feature = "derive", feature = "alloc"))]
//! # {
//! use cbor::{Decode, Encode};
//!
//! #[derive(Encode, Decode)]
//! struct Point { x: f64, y: f64 }
//!
//! let bytes = cbor::to_vec(&Point { x: 1.0, y: 2.0 });
//! let value: cbor::Value = cbor::from_slice(&bytes).unwrap();
//! assert!(value.get(&"x".into()).is_some());
//! # }
//! ```
//!
//! Positional arrays are smaller and faster:
//!
//! ```
//! # #[cfg(all(feature = "derive", feature = "alloc"))]
//! # {
//! use cbor::{Decode, Encode};
//!
//! #[derive(Encode, Decode)]
//! #[cbor(array)]
//! struct Vec3(f64, f64, f64);
//!
//! let bytes = cbor::to_vec(&Vec3(1.0, 2.0, 3.0));
//! assert_eq!(bytes[0], 0x83);
//! # }
//! ```
//!
//! Integer keys are intended for protocols such as COSE (RFC 9052):
//!
//! ```
//! # #[cfg(all(feature = "derive", feature = "alloc"))]
//! # {
//! use cbor::{Decode, Encode};
//!
//! #[derive(Encode, Decode)]
//! #[cbor(tag = 18)]
//! struct Sign1 {
//!     #[cbor(key = 1)]
//!     protected: Vec<u8>,
//!     #[cbor(key = 4)]
//!     kid: Vec<u8>,
//! }
//! # }
//! ```
//!
//! Enums use serde-compatible external tagging by default. Use
//! `#[cbor(array)]` for a compact `[variant_index, fields...]` form,
//! `#[cbor(index_only)]` for a bare integer variant index,
//! `#[cbor(tagged)]` for `[variant_index, {field_key: value, ...}]`, or
//! `#[cbor(key = N)]` on variants for an integer-keyed map.
//!
//! # Byte strings
//!
//! `Vec<T>` and `&[T]` are CBOR arrays. Annotate a `Vec<u8>`, `&[u8]` or
//! `Option<Vec<u8>>` field with `#[cbor(bytes)]` to encode it as a byte string
//! (`None` becomes CBOR `null`), or use `bytes::Bytes`/`bytes::BytesMut` with
//! the `bytes` feature. The same rule applies to `Vec<u8>` in serde
//! (`serde_bytes` is needed there).
//!
//! # serde
//!
//! The `serde` feature provides a streaming serializer and deserializer over
//! the same [`Encoder`]/[`Decoder`]. Definite-length text and byte strings are
//! borrowed from the input when the target type asks for them:
//!
//! ```
//! # #[cfg(feature = "serde")]
//! # {
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Packet<'a> {
//!     #[serde(borrow)]
//!     name: &'a str,
//!     #[serde(with = "serde_bytes")]
//!     data: &'a [u8],
//! }
//!
//! let bytes = cbor::serde::to_vec(&Packet { name: "hi", data: b"abc" }).unwrap();
//! let packet: Packet<'_> = cbor::serde::from_slice(&bytes).unwrap();
//! assert_eq!(packet.name, "hi");
//! # }
//! ```
//!
//! # Security
//!
//! Decoding is bounded: [`Options::max_depth`] limits recursion in `skip`,
//! `Value`, derived types, serde and validation. Byte/text/array/map lengths
//! are checked against the remaining input before use, and collection
//! preallocation is capped so a forged length cannot trigger a huge allocation.
//! Some helpers use their own fixed limits: `reader::read_item` uses 64 and
//! diagnostic formatting uses 128.
//!
//! Duplicate keys are rejected by default for the native typed maps
//! (`BTreeMap`, `HashMap`, `Map`) and for derived maps. The serde adapter
//! rejects keys with byte-identical wire encodings by default, but it cannot
//! detect semantically equal keys that use different encodings; use
//! [`validate_canonical`] or the native typed API when that distinction matters.
//!
//! [`RawValue`] obtained from a decoder is validated as one well-formed item,
//! including UTF-8 text validity. [`validate_canonical`] additionally checks the
//! strict deterministic form, including preferred bignum serialization.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod bytes;
#[cfg(feature = "bytes")]
mod bytes_impl;
mod canonical;
mod data;
mod decode;
mod diag;
mod encode;
mod error;
mod float;
#[cfg(feature = "heapless")]
mod heapless_impl;
mod raw;
#[cfg(feature = "small_collections")]
mod small_collections_impl;
mod tagged;
#[cfg(feature = "alloc")]
mod value;

#[cfg(feature = "alloc")]
pub use crate::diag::to_diagnostic_string;
#[cfg(feature = "alloc")]
pub use crate::encode::{to_canonical_vec, to_vec, to_vec_with_capacity};
#[cfg(feature = "std")]
pub use crate::encode::{to_canonical_writer, to_writer};
#[cfg(feature = "alloc")]
pub use crate::raw::RawValueBuf;
#[cfg(feature = "alloc")]
pub use crate::value::{Bignum, Map, Value};
pub use crate::{
    bytes::DecodeBytes,
    canonical::{
        KeyOrder, validate, validate_canonical, validate_canonical_with, validate_prefix, validate_prefix_with,
        validate_sequence,
    },
    data::{Integer, Simple, Tag, Type, tags},
    decode::{
        BytesIter, Decode, Decoder, DuplicateKeys, Options, TextIter, from_slice, from_slice_prefix,
        from_slice_with_options,
    },
    diag::to_diagnostic,
    encode::{
        Count, Encode, Encoder, Write, encoded_len, f64_to_half, half_to_f64, shortest_float, to_canonical_slice,
        to_slice,
    },
    error::{Error, ErrorKind, Result},
    raw::RawValue,
    tagged::{Tagged, TaggedValue},
};

#[cfg(all(feature = "std", feature = "serde"))]
mod reader;
#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "derive")]
pub use cbor_derive::{Decode, Encode};
