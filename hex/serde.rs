//! Serde `serialize`/`deserialize` helpers for hex-encoded byte buffers.
//!
//! Use with `#[serde(with = "hex")]` on [`Vec<u8>`] fields:
//!
//! ```rust,ignore
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Foo {
//!     #[serde(with = "hex")]
//!     bar: Vec<u8>,
//! }
//! ```
//!
//! In human-readable formats (e.g. JSON) the field is serialized as a hex
//! string. In binary formats the raw bytes are used directly.

use alloc::vec::Vec;
use core::fmt;

use serde::{
    Deserializer, Serializer,
    de::{self, Visitor},
};

use crate::{decode, encode};

/// Serializes `data` to a hex string (human-readable) or raw bytes
/// (binary formats).
///
/// Typically used via `#[serde(with = "hex")]`. See the [module-level
/// documentation](self) for a full example.
pub fn serialize<S: Serializer>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    if serializer.is_human_readable() {
        serializer.serialize_str(&encode(data))
    } else {
        serializer.serialize_bytes(data)
    }
}

/// Deserializes a hex string (human-readable) or raw bytes (binary
/// formats) into a [`Vec<u8>`].
///
/// Usually used via `#[serde(with = "hex")]`.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(HexVisitor)
    } else {
        deserializer.deserialize_bytes(HexVisitor)
    }
}

struct HexVisitor;

impl<'de> Visitor<'de> for HexVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a hex string")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Vec<u8>, E> {
        decode(v.as_bytes()).map_err(de::Error::custom)
    }

    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Vec<u8>, E> {
        Ok(v.to_vec())
    }
}
