//! CBOR serialization with the workspace [`cbor`](https://docs.rs/cbor) crate.
//!
//! Enabled via the `cbor` feature flag.
//!
//! Encoding uses the IANA CBOR tag `37` ("Binary UUID", RFC 9562 §4) over a
//! 16-byte byte string, e.g. `d8 25 50 <16 bytes>`. The tag makes the value
//! self-describing, which is what lets a decoder distinguish a UUID from any
//! other 16-byte blob.
//!
//! Decoding accepts three representations:
//!
//! - the canonical `tag(37) : bstr(16)` form,
//! - a bare `bstr(16)`,
//! - the canonical 36-character text form.
//!
//! The text form goes through [`Uuid::parse`], so it stays strict: only the
//! 8-4-4-4-12 lowercase or uppercase form is accepted.

use ::cbor::{Decode, Decoder, Encode, Encoder, Error as DecodeError, Tag, Type, Write};

use crate::Uuid;

/// The IANA CBOR tag for a binary UUID (RFC 9562 §4).
pub const UUID_TAG: u64 = 37;

impl Encode for Uuid {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.tag(Tag::new(UUID_TAG))?;
        encoder.bytes(&self.as_bytes())?;
        Ok(())
    }
}

impl<'b> Decode<'b> for Uuid {
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, DecodeError> {
        match decoder.datatype()? {
            Type::Tag => {
                let tag = decoder.tag()?;
                if tag.get() != UUID_TAG {
                    return Err(DecodeError::message("unexpected UUID tag").at(decoder.position()));
                }
                decode_bytes(decoder.bytes()?)
            }
            Type::Bytes => decode_bytes(decoder.bytes()?),
            Type::Text => decode_text(decoder.text()?),
            other => Err(DecodeError::type_mismatch(Type::Tag, other).at(decoder.position())),
        }
    }
}

fn decode_bytes(bytes: &[u8]) -> Result<Uuid, DecodeError> {
    Uuid::from_slice(bytes).map_err(|_| DecodeError::message("invalid UUID byte string"))
}

fn decode_text(text: &str) -> Result<Uuid, DecodeError> {
    Uuid::parse(text).map_err(|_| DecodeError::message("invalid UUID text"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "f47ac10b-58cc-4372-a567-0e02b2c3d479";

    fn sample() -> Uuid {
        Uuid::parse(TEXT).unwrap()
    }

    #[test]
    fn encodes_tagged_byte_string() {
        let mut expected = vec![0xd8, 0x25, 0x50];
        expected.extend_from_slice(&sample().as_bytes());
        assert_eq!(::cbor::to_vec(&sample()), expected);
    }

    #[test]
    fn round_trips() {
        let encoded = ::cbor::to_vec(&sample());
        let decoded: Uuid = ::cbor::from_slice(&encoded).unwrap();
        assert_eq!(decoded, sample());
    }

    #[test]
    fn accepts_bare_byte_string() {
        let mut encoded = vec![0x50];
        encoded.extend_from_slice(&sample().as_bytes());
        let decoded: Uuid = ::cbor::from_slice(&encoded).unwrap();
        assert_eq!(decoded, sample());
    }

    #[test]
    fn accepts_text() {
        let encoded = ::cbor::to_vec(TEXT);
        let decoded: Uuid = ::cbor::from_slice(&encoded).unwrap();
        assert_eq!(decoded, sample());
    }

    #[test]
    fn rejects_wrong_tag() {
        let mut encoded = vec![0xd8, 0x26, 0x50]; // tag 38
        encoded.extend_from_slice(&sample().as_bytes());
        assert!(::cbor::from_slice::<Uuid>(&encoded).is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        let encoded = vec![0xd8, 0x25, 0x48, 0, 1, 2, 3, 4, 5, 6, 7];
        assert!(::cbor::from_slice::<Uuid>(&encoded).is_err());
    }

    #[test]
    fn rejects_invalid_text() {
        let encoded = ::cbor::to_vec("not a uuid");
        assert!(::cbor::from_slice::<Uuid>(&encoded).is_err());
    }

    #[test]
    fn optional_uuid_round_trips() {
        let encoded = ::cbor::to_vec(&Some(sample()));
        let decoded: Option<Uuid> = ::cbor::from_slice(&encoded).unwrap();
        assert_eq!(decoded, Some(sample()));

        let encoded = ::cbor::to_vec(&Option::<Uuid>::None);
        let decoded: Option<Uuid> = ::cbor::from_slice(&encoded).unwrap();
        assert_eq!(decoded, None);
    }
}
