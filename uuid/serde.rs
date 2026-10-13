//! Serialization and deserialization with [`serde`](https://docs.rs/serde).
//!
//! Enabled via the `serde` feature flag.
//!
//! [`Uuid`]s are serialized as their canonical 8-4-4-4-12 lowercase hex string
//! (e.g. `"f47ac10b-58cc-4372-a567-0e02b2c3d479"`).
//!
//! Deserialization accepts the canonical string form (JSON, YAML, TOML, and
//! other human-readable formats) as well as a raw 16-byte buffer (bincode,
//! MessagePack, and other binary formats). Human-readable formats route string
//! input through [`Visitor::visit_bytes`] or [`Visitor::visit_str`] depending on
//! the format; both are handled.

use core::fmt;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

use crate::Uuid;

impl Serialize for Uuid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

struct UuidVisitor;

impl<'de> Visitor<'de> for UuidVisitor {
    type Value = Uuid;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a UUID as a 36-character hex string or 16-byte buffer")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Uuid, E> {
        Uuid::parse(v).map_err(de::Error::custom)
    }

    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Uuid, E> {
        // serde_json forwards string content here as 36 raw bytes;
        // binary formats (bincode, msgpack) pass the raw 16-byte buffer.
        match v.len() {
            16 => Uuid::from_slice(v).map_err(de::Error::custom),
            36 => Uuid::parse(v).map_err(de::Error::custom),
            _ => Err(de::Error::custom("expected 16 bytes (binary) or 36 bytes (hex string)")),
        }
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Uuid, D::Error> {
        deserializer.deserialize_bytes(UuidVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::de::{
        IntoDeserializer,
        value::{BorrowedBytesDeserializer, StrDeserializer},
    };

    use super::*;

    const S: &str = "f47ac10b-58cc-4372-a567-0e02b2c3d479";

    #[test]
    fn round_trip() {
        let uuid = Uuid::parse(S).unwrap();
        let json = serde_json::to_string(&uuid).unwrap();
        assert_eq!(json, format!("\"{S}\""));
        let got: Uuid = serde_json::from_str(&json).unwrap();
        assert_eq!(got, uuid);
    }

    #[test]
    fn deserialize_from_str() {
        let json = format!("\"{S}\"");
        let uuid: Uuid = serde_json::from_str(&json).unwrap();
        assert_eq!(uuid.to_string(), S);
    }

    #[test]
    fn deserialize_uppercase_normalizes_to_lowercase() {
        let json = format!("\"{}\"", S.to_uppercase());
        let uuid: Uuid = serde_json::from_str(&json).unwrap();
        assert_eq!(uuid.to_string(), S);
    }

    #[test]
    fn deserialize_array_rejected() {
        let json = "[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]";
        let result: Result<Uuid, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_errors() {
        for bad in [
            "\"nope\"",
            "\"f47ac10b-58cc-4372-a567-0e02b2c3d47\"",
            "\"00112233-4455-6677-8899-aabbccddeeff0\"",
            "123",
            "null",
            "true",
            "{}",
        ] {
            let result: Result<Uuid, _> = serde_json::from_str(bad);
            assert!(result.is_err(), "should reject {bad}, got {result:?}");
        }
    }

    #[test]
    fn nil_round_trip() {
        let uuid = Uuid::nil();
        let json = serde_json::to_string(&uuid).unwrap();
        assert_eq!(json, "\"00000000-0000-0000-0000-000000000000\"");
        let got: Uuid = serde_json::from_str(&json).unwrap();
        assert_eq!(got, uuid);
    }

    #[test]
    fn max_round_trip() {
        let uuid = Uuid::max();
        let json = serde_json::to_string(&uuid).unwrap();
        assert_eq!(json, "\"ffffffff-ffff-ffff-ffff-ffffffffffff\"");
        let got: Uuid = serde_json::from_str(&json).unwrap();
        assert_eq!(got, uuid);
    }

    #[test]
    fn deserialize_binary_16_byte_buffer() {
        let uuid = Uuid::parse(S).unwrap();
        let bytes = uuid.as_bytes();
        let de: BorrowedBytesDeserializer<'_, serde::de::value::Error> = BorrowedBytesDeserializer::new(&bytes);
        assert_eq!(Uuid::deserialize(de).unwrap(), uuid);
    }

    #[test]
    fn deserialize_36_byte_hex_buffer() {
        let uuid = Uuid::parse(S).unwrap();
        let de: BorrowedBytesDeserializer<'_, serde::de::value::Error> = BorrowedBytesDeserializer::new(S.as_bytes());
        assert_eq!(Uuid::deserialize(de).unwrap(), uuid);
    }

    #[test]
    fn deserialize_wrong_lengths_error() {
        for len in [0usize, 1, 15, 17, 35, 37, 64] {
            let buf = vec![0u8; len];
            let de: BorrowedBytesDeserializer<'_, serde::de::value::Error> = BorrowedBytesDeserializer::new(&buf);
            assert!(Uuid::deserialize(de).is_err(), "length {len} must error");
        }
    }

    #[test]
    fn deserialize_visit_str_directly() {
        let de: StrDeserializer<'_, serde::de::value::Error> = S.into_deserializer();
        assert_eq!(Uuid::deserialize(de).unwrap(), Uuid::parse(S).unwrap());

        let de: StrDeserializer<'_, serde::de::value::Error> = "zzz".into_deserializer();
        assert!(Uuid::deserialize(de).is_err());
    }

    #[test]
    fn serialize_does_not_allocate_extra() {
        let uuid = Uuid::parse(S).unwrap();
        // Writing to a sink must not allocate at all.
        let (_, c) = alloc_counter::measure(|| serde_json::to_writer(std::io::sink(), &uuid).unwrap());
        assert_eq!(c.calls, 0, "to_writer must not allocate");

        // `to_string` allocates only the returned `String`.
        let (_, c) = alloc_counter::measure(|| serde_json::to_string(&uuid).unwrap());
        assert!(
            c.calls <= 1,
            "to_string should allocate at most the output string, got {}",
            c.calls
        );
    }

    #[test]
    fn serialize_matches_display() {
        let uuid = Uuid::parse(S).unwrap();
        assert_eq!(serde_json::to_string(&uuid).unwrap(), format!("\"{uuid}\""));
    }
}
