//! JSON support for [`Uuid`].
//!
//! UUIDs are encoded as their canonical 8-4-4-4-12 lowercase string, for
//! example `"f47ac10b-58cc-4372-a567-0e02b2c3d479"`. Decoding accepts the same
//! form and is strict: only the canonical grouping and hexadecimal digits are
//! accepted.

use ::json::{Decode, Decoder, Encode, Encoder, ErrorKind, Write};

use crate::Uuid;

impl Encode for Uuid {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.to_heapless_string().as_str())
    }
}

impl<'a> Decode<'a> for Uuid {
    fn decode(decoder: &mut Decoder<'a>) -> json::Result<Self> {
        let text = decoder.decode_string()?;
        Uuid::parse(&text).map_err(|_| decoder.error(ErrorKind::Message("invalid UUID")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let uuid = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
        let text = ::json::to_string(&uuid);
        assert_eq!(text, "\"f47ac10b-58cc-4372-a567-0e02b2c3d479\"");
        let decoded: Uuid = ::json::from_str(&text).unwrap();
        assert_eq!(decoded, uuid);
    }

    #[test]
    fn rejects_invalid_uuid() {
        assert!(::json::from_str::<Uuid>("\"not-a-uuid\"").is_err());
    }
}
