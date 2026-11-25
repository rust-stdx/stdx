//! JSON support for [`DateTime`].
//!
//! Date/times use the RFC 3339 / RFC 9557 textual representation, for example
//! `"2024-01-01T12:34:56.789Z"` or
//! `"2024-01-01T12:34:56-05:00[America/New_York]"`. Decoding accepts any value
//! that [`DateTime::from_str`](core::str::FromStr::from_str) accepts.

use alloc::string::ToString;

use ::json::{Decode, Decoder, Encode, Encoder, ErrorKind, Write};

use crate::DateTime;

impl Encode for DateTime {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.string(&self.to_string())
    }
}

impl<'a> Decode<'a> for DateTime {
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        let text = decoder.decode_string()?;
        text.parse()
            .map_err(|_| decoder.error(ErrorKind::Message("invalid date/time")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_utc() {
        let value: DateTime = "2024-01-01T12:34:56.789Z".parse().unwrap();
        let text = ::json::to_string(&value);
        assert_eq!(text, "\"2024-01-01T12:34:56.789Z\"");
        let decoded: DateTime = ::json::from_str(&text).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn roundtrip_with_zone() {
        let value: DateTime = "2024-01-01T12:34:56-05:00[America/New_York]".parse().unwrap();
        let text = ::json::to_string(&value);
        let decoded: DateTime = ::json::from_str(&text).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn rejects_garbage() {
        assert!(::json::from_str::<DateTime>("\"not a date\"").is_err());
    }
}
