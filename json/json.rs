#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! A fast, strict JSON parser and generator.
//!
//! This crate implements RFC 8259 with a native `Encode`/`Decode` derive API.
//! It does **not** depend on serde. Parsing is zero-copy for strings without
//! escapes, and encoding can write directly into a fixed buffer or any byte
//! sink.
//!
//! # Feature flags
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `std` | yes | Runtime SIMD detection, `std::io` helpers and `HashMap` support |
//! | `alloc` | via `std` | `Value`, `String`, `Vec`, `RawValueBuf` and the owned APIs |
//! | `derive` | no | `#[derive(json::Encode, json::Decode)]` |
//! | `heapless` | no | Implementations for `heapless` containers |
//! | `small_collections` | no | Implementations for `SmallVec`/`SmallString` |
//! | `bytes` | no | `bytes::Bytes`/`BytesMut` support |
//!
//! # Quick start
//!
//! ```
//! # #[cfg(feature = "derive")] {
//! use json::{Decode, Encode};
//!
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! let point = Point { x: 1.5, y: -2.0 };
//! let text = json::to_string(&point);
//! let decoded: Point = json::from_str(&text).unwrap();
//! assert_eq!(point, decoded);
//! # }
//! ```
//!
//! # no_std
//!
//! With `default-features = false`, the crate is `no_std` and has no heap
//! dependency. Decoding into borrowed types, encoding into a fixed buffer,
//! `validate`, `skip` and [`RawValue`] work without an allocator. Enable
//! `alloc` for [`Value`], `String`, `Vec` and the `to_vec` family.
//!
//! # Relationship to serde
//!
//! This crate deliberately does not depend on serde. Types are serialized with
//! `#[derive(json::Encode, json::Decode)]`, and dynamic data uses [`Value`].
//! This keeps the dependency tree small and makes the derive available in
//! `no_std` builds.
//!
//! # SIMD
//!
//! String scanning uses AVX2 on x86_64 when available and NEON on AArch64.
//! With the `std` feature the best kernel is selected at runtime; without
//! `std` it is selected at compile time from the enabled target features.
//! UTF-8 validation uses the full SIMD validator from the `utf8` crate on
//! AVX2, NEON, and WASM SIMD128 targets.
//!
//! # Current limitations
//!
//! * `#[json(flatten)]` is not implemented yet; it returns a compile-time
//!   error.
//! * Internally tagged enums require the tag member to be the first member of
//!   the object.
//! * The allocation-free bitmask-ring stage-1 parser is experimental and
//!   disabled by default; enable it with `Options::use_simd_masks` after
//!   benchmarking your workload.

#[cfg(feature = "alloc")]
extern crate alloc;

mod decode;
mod document;
mod encode;
mod error;
mod float_lemire;
#[cfg(feature = "heapless")]
mod heapless_impl;
mod mask;
mod number;
mod raw;
mod scan;
#[cfg(feature = "small_collections")]
mod small_collections_impl;
mod stream;
mod string;
#[cfg(feature = "alloc")]
mod value;

#[cfg(feature = "derive")]
pub use json_derive::{Decode, Encode};

#[cfg(feature = "std")]
pub use crate::encode::IoWriter;
#[cfg(feature = "std")]
pub use crate::encode::to_io_writer;
#[cfg(feature = "alloc")]
pub use crate::encode::{to_string, to_string_pretty, to_vec, to_vec_into, to_vec_with_capacity};
#[cfg(feature = "alloc")]
pub use crate::raw::RawValueBuf;
#[cfg(feature = "std")]
pub use crate::stream::{JsonLines, from_reader};
#[cfg(feature = "alloc")]
pub use crate::value::{Map, OwnedValue, Value};
pub use crate::{
    decode::{
        Decode, DecodeOwned, Decoder, DuplicateKeys, Key, Options, from_slice, from_slice_owned,
        from_slice_with_options, from_str, from_str_owned, from_str_with_options,
    },
    document::{ArrayIter, Document, Element, ObjectIter},
    encode::{Count, Encode, EncodeOptions, Encoder, Write, encoded_len, to_slice},
    error::{Error, ErrorKind, Result},
    number::{BigNumberPolicy, Number},
    raw::{RawNumber, RawValue},
    scan::{minify, validate, validate_with_options},
    stream::StreamDeserializer,
};

/// Re-exports used by the `json!` macro.
#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "alloc")]
    pub use alloc::borrow::Cow;
    #[cfg(feature = "alloc")]
    pub use alloc::vec::Vec;
}
