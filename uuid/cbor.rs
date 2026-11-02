//! CBOR serialization with [`minicbor`](https://docs.rs/minicbor).
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

use minicbor::{
    Decode, Encode,
    data::{Tag, Type},
    decode::{Decoder, Error as DecodeError},
    encode::{Encoder, Error as EncodeError, Write},
};

use crate::Uuid;

/// The IANA CBOR tag for a binary UUID (RFC 9562 §4).
pub const UUID_TAG: u64 = 37;

impl<C> Encode<C> for Uuid {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>, _: &mut C) -> Result<(), EncodeError<W::Error>> {
        encoder.tag(Tag::new(UUID_TAG))?.bytes(&self.as_bytes())?;
        Ok(())
    }
}

impl<'b, C> Decode<'b, C> for Uuid {
    fn decode(decoder: &mut Decoder<'b>, _: &mut C) -> Result<Self, DecodeError> {
        match decoder.datatype()? {
            Type::Tag => {
                let tag = decoder.tag()?;
                if tag.as_u64() != UUID_TAG {
                    return Err(DecodeError::tag_mismatch(tag));
                }
                decode_bytes(decoder.bytes()?)
            }
            Type::Bytes => decode_bytes(decoder.bytes()?),
            Type::String => decode_text(decoder.str()?),
            other => Err(DecodeError::type_mismatch(other)),
        }
    }
}

fn decode_bytes(bytes: &[u8]) -> Result<Uuid, DecodeError> {
    Uuid::from_slice(bytes).map_err(DecodeError::custom)
}

fn decode_text(text: &str) -> Result<Uuid, DecodeError> {
    Uuid::parse(text).map_err(DecodeError::custom)
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
        assert_eq!(minicbor::to_vec(sample()).unwrap(), expected);
    }

    #[test]
    fn round_trips() {
        let encoded = minicbor::to_vec(sample()).unwrap();
        let decoded: Uuid = minicbor::decode(&encoded).unwrap();
        assert_eq!(decoded, sample());
    }

    #[test]
    fn accepts_bare_byte_string() {
        let mut encoded = vec![0x50];
        encoded.extend_from_slice(&sample().as_bytes());
        let decoded: Uuid = minicbor::decode(&encoded).unwrap();
        assert_eq!(decoded, sample());
    }

    #[test]
    fn accepts_text() {
        let encoded = minicbor::to_vec(TEXT).unwrap();
        let decoded: Uuid = minicbor::decode(&encoded).unwrap();
        assert_eq!(decoded, sample());
    }

    #[test]
    fn rejects_wrong_tag() {
        let mut encoded = vec![0xd8, 0x26, 0x50]; // tag 38
        encoded.extend_from_slice(&sample().as_bytes());
        assert!(minicbor::decode::<Uuid>(&encoded).is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        let encoded = vec![0xd8, 0x25, 0x48, 0, 1, 2, 3, 4, 5, 6, 7];
        assert!(minicbor::decode::<Uuid>(&encoded).is_err());
    }

    #[test]
    fn rejects_invalid_text() {
        let encoded = minicbor::to_vec("not a uuid").unwrap();
        assert!(minicbor::decode::<Uuid>(&encoded).is_err());
    }

    #[test]
    fn optional_uuid_round_trips() {
        let encoded = minicbor::to_vec(Some(sample())).unwrap();
        let decoded: Option<Uuid> = minicbor::decode(&encoded).unwrap();
        assert_eq!(decoded, Some(sample()));

        let encoded = minicbor::to_vec(Option::<Uuid>::None).unwrap();
        let decoded: Option<Uuid> = minicbor::decode(&encoded).unwrap();
        assert_eq!(decoded, None);
    }
}
