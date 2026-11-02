//! Serialization and deserialization with [`serde`](https://docs.rs/serde).
//!
//! Enabled via the `serde` feature flag.
//!
//! Human-readable formats (JSON, YAML, TOML, and `serde-wasm-bindgen`) receive
//! the canonical 8-4-4-4-12 lowercase hex string, e.g.
//! `"f47ac10b-58cc-4372-a567-0e02b2c3d479"`.
//!
//! Binary formats (CBOR, MessagePack, bincode, postcard) receive the raw
//! 16-byte buffer, which avoids 20 bytes of encoding per identifier.
//!
//! Deserialization accepts both shapes regardless of the format: a string, a
//! raw 16-byte buffer, or a 36-byte hex buffer are all valid.

use core::fmt;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

use crate::Uuid;

impl Serialize for Uuid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Binary formats carry the identifier as 16 raw bytes: a CBOR byte
        // string, a MessagePack `bin`, and so on. Text formats keep the
        // canonical form because it is what a human reading the document
        // expects, and what the webapp's TypeScript interfaces declare.
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            serializer.serialize_bytes(&self.as_bytes())
        }
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
        // Human-readable formats expose the canonical string. Asking for bytes
        // would fail for formats such as `serde-wasm-bindgen`, whose
        // `deserialize_bytes` only accepts typed arrays.
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(UuidVisitor)
        } else {
            deserializer.deserialize_bytes(UuidVisitor)
        }
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

    #[test]
    fn serialize_binary_emits_16_raw_bytes() {
        let uuid = Uuid::parse(S).unwrap();
        assert_eq!(BinarySerializer::encode(&uuid), Encoded::Bytes(uuid.as_bytes().to_vec()));
    }

    #[test]
    fn deserialize_after_binary_serialize_round_trips() {
        let uuid = Uuid::parse(S).unwrap();
        let bytes = uuid.as_bytes();
        let de: BorrowedBytesDeserializer<'_, serde::de::value::Error> = BorrowedBytesDeserializer::new(&bytes);
        assert_eq!(Uuid::deserialize(de).unwrap(), uuid);
    }

    /// Collects what a non-human-readable `Serializer` is asked to emit.
    #[derive(Debug, PartialEq)]
    enum Encoded {
        Bytes(Vec<u8>),
        Str(String),
    }

    #[derive(Debug)]
    struct Unsupported;

    impl core::fmt::Display for Unsupported {
        fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            formatter.write_str("unsupported value")
        }
    }

    impl serde::ser::Error for Unsupported {
        fn custom<T: core::fmt::Display>(_: T) -> Self {
            Unsupported
        }
    }

    impl std::error::Error for Unsupported {}

    /// A serializer that reports itself as a binary (non-human-readable) format.
    struct BinarySerializer;

    impl BinarySerializer {
        fn encode<T: Serialize>(value: &T) -> Encoded {
            value.serialize(BinarySerializer).unwrap()
        }
    }

    impl Serializer for BinarySerializer {
        type Ok = Encoded;
        type Error = Unsupported;
        type SerializeSeq = serde::ser::Impossible<Encoded, Unsupported>;
        type SerializeTuple = serde::ser::Impossible<Encoded, Unsupported>;
        type SerializeTupleStruct = serde::ser::Impossible<Encoded, Unsupported>;
        type SerializeTupleVariant = serde::ser::Impossible<Encoded, Unsupported>;
        type SerializeMap = serde::ser::Impossible<Encoded, Unsupported>;
        type SerializeStruct = serde::ser::Impossible<Encoded, Unsupported>;
        type SerializeStructVariant = serde::ser::Impossible<Encoded, Unsupported>;

        fn is_human_readable(&self) -> bool {
            false
        }

        fn serialize_bytes(self, value: &[u8]) -> Result<Encoded, Unsupported> {
            Ok(Encoded::Bytes(value.to_vec()))
        }

        fn serialize_str(self, value: &str) -> Result<Encoded, Unsupported> {
            Ok(Encoded::Str(value.to_owned()))
        }

        fn serialize_bool(self, _: bool) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_i8(self, _: i8) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_i16(self, _: i16) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_i32(self, _: i32) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_i64(self, _: i64) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_u8(self, _: u8) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_u16(self, _: u16) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_u32(self, _: u32) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_u64(self, _: u64) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_f32(self, _: f32) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_f64(self, _: f64) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_char(self, _: char) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_none(self) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_unit(self) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: &T,
        ) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<Encoded, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Unsupported> {
            Err(Unsupported)
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, Unsupported> {
            Err(Unsupported)
        }
    }
}
