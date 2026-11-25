//! A YAML parser and generator with native `Encode`/`Decode` derive macros.
//!
//! The crate is organised around three pieces:
//!
//! - [`Encode`] and [`Decode`], which convert Rust types to and from a
//!   [`Value`]. They can be derived with `#[derive(yaml::Encode, yaml::Decode)]`
//!   (feature `derive`).
//! - [`Value`], [`Mapping`] and [`Number`] for dynamic YAML documents.
//! - [`to_string`], [`to_vec`], [`from_str`], and [`from_slice`] for text
//!   conversion.
//!
//! # Quick start
//!
//! ```
//! # #[cfg(feature = "derive")] {
//! use yaml::{Decode, Encode};
//!
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! let point = Point { x: 1.5, y: -2.0 };
//! let text = yaml::to_string(&point).unwrap();
//! let decoded: Point = yaml::from_str(&text).unwrap();
//! assert_eq!(point, decoded);
//! # }
//! ```
//!
//! # Feature flags
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `std` | yes | Standard library and the `std::io` helpers |
//! | `alloc` | always | Accepted as a no-op; the crate always uses `alloc` |
//! | `derive` | no | `#[derive(Encode, Decode)]` |
//!
//! # no_std
//!
//! With `default-features = false, features = ["alloc"]`, the crate works
//! without the standard library. Parsing and emission use the bundled
//! `libyaml_unsafe` parser and emitter but expose a safe, owning API.
//!
//! # Supported YAML
//!
//! Parsing supports the YAML 1.2 core schema: nulls, booleans, decimal,
//! hexadecimal, octal, and binary integers, floating-point numbers (including
//! `.inf` and `.nan`), strings, sequences, mappings, anchors/aliases, merge
//! keys (`<<`), and explicit tags. Input must be UTF-8.
//!
//! `from_str`/`from_slice` return the first document. Use
//! [`from_str_multi`]/[`from_slice_multi`] for every document in a stream, and
//! [`from_reader`] for a `std::io::Read` source.
//!
//! Emission writes one document. Plain scalars are used only when they cannot
//! be mistaken for another type; multiline strings use literal blocks and
//! control characters use double quotes.
//!
//! # Limits and duplicate keys
//!
//! Parsing applies conservative limits by default: a maximum nesting depth of
//! 128, one million values per document, 10,000 aliases, and 1,024 documents.
//! Duplicate mapping keys are rejected by default. Use [`Options`] and the
//! `*_with_options` functions to relax these limits, or
//! [`DuplicateKeys::First`]/[`DuplicateKeys::Last`] to choose a policy.
//!
//! # Safety
//!
//! The public API is safe Rust. Unsafe code is confined to the vendored
//! libyaml parser/emitter wrapper (`yaml::libyaml`, crate-private).

#![no_std]
#![deny(unsafe_code)]

pub extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod de;
mod decode;
mod encode;
mod error;
#[allow(unsafe_code)]
mod libyaml;
mod options;
mod ser;
mod value;

#[cfg(feature = "derive")]
pub use yaml_derive::{Decode, Encode};

#[cfg(feature = "std")]
pub use crate::{de::from_reader, ser::to_writer};
pub use crate::{
    de::{
        from_slice, from_slice_multi, from_slice_multi_with_options, from_slice_with_options, from_str, from_str_multi,
        from_str_multi_with_options, from_str_with_options,
    },
    decode::{Decode, from_value},
    encode::{Encode, to_value},
    error::{Error, Result},
    options::{DuplicateKeys, Options},
    ser::{to_string, to_vec},
    value::{Mapping, Number, TaggedValue, Value},
};
