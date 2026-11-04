//! CBOR serialization with the workspace [`cbor`](https://docs.rs/cbor) crate.
//!
//! Enabled via the `cbor` feature flag.
//!
//! Encoding uses IANA CBOR tag `1` ("Epoch-Based Date/Time", RFC 8949 §3.4.2)
//! with an integer number of seconds since `1970-01-01T00:00:00Z`. Subsecond
//! precision is discarded: the fractional part is rounded down to the
//! containing second, so a value encoded from `12:34:56.789Z` decodes as
//! `12:34:56Z`.
//!
//! Decoding accepts tag `1` with integer content and, for interoperability with
//! other CBOR producers, half-, single- and double-precision floating-point
//! content. Fractional seconds in a float are also discarded by rounding down
//! to the containing second. Non-finite floats and values outside the supported
//! calendar range are rejected.
//!
//! Decoding also accepts IANA tag `0` ("Standard Date/Time String", RFC 8949
//! §3.4.1), whose content is an RFC 3339 text string. The encoded UTC offset
//! and subsecond precision are preserved. The string must be a complete RFC
//! 3339 date-time with an offset; date-only strings, zone-less strings and RFC
//! 9557 time zone annotations are rejected.
//!
//! Half-precision floats are decoded directly by the shared codec.

use ::cbor::{Decode, Decoder, Encode, Encoder, Error as DecodeError, Tag, Type, Write};

use crate::{DateTime, Format};

/// The IANA CBOR tag for a standard date/time string (RFC 8949 §3.4.1).
pub const DATE_TIME_STRING_TAG: u64 = 0;

/// The IANA CBOR tag for an epoch-based date/time (RFC 8949 §3.4.2).
pub const EPOCH_TAG: u64 = 1;

impl Encode for DateTime {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.tag(Tag::new(EPOCH_TAG))?;
        encoder.i64(self.unix())?;
        Ok(())
    }
}

impl<'b> Decode<'b> for DateTime {
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, DecodeError> {
        let datatype = decoder.datatype()?;
        if datatype != Type::Tag {
            return Err(DecodeError::message("expected date/time tag 0 or 1").at(decoder.position()));
        }
        let tag = decoder.tag()?;
        match tag.get() {
            DATE_TIME_STRING_TAG => decode_string(decoder),
            EPOCH_TAG => decode_epoch_content(decoder),
            _ => Err(DecodeError::message("unsupported date/time tag").at(decoder.position())),
        }
    }
}

/// Decodes an RFC 3339 date/time text string (tag 0).
///
/// Returns an error if the content is not a definite-length UTF-8 text string,
/// or if it is not a valid RFC 3339 date-time within the supported calendar
/// range.
fn decode_string(decoder: &mut Decoder<'_>) -> Result<DateTime, DecodeError> {
    let text = decoder.text()?;
    DateTime::parse_with_format(text, Format::Rfc3339).map_err(|_| DecodeError::message("invalid RFC 3339 date/time"))
}

fn decode_epoch_content(decoder: &mut Decoder<'_>) -> Result<DateTime, DecodeError> {
    let seconds = match decoder.datatype()? {
        Type::F16 | Type::F32 | Type::F64 => decoder.f64()?,
        _ => {
            return DateTime::from_unix(decoder.i64()?)
                .map_err(|_| DecodeError::message("timestamp is outside the supported range"));
        }
    };
    let seconds =
        floor_to_i64(seconds).ok_or_else(|| DecodeError::message("timestamp is outside the supported range"))?;
    DateTime::from_unix(seconds).map_err(|_| DecodeError::message("timestamp is outside the supported range"))
}

/// Rounds a finite `f64` down to the containing integer without using `std`
/// floating-point helpers.
fn floor_to_i64(value: f64) -> Option<i64> {
    if !value.is_finite() || value < i64::MIN as f64 || value > i64::MAX as f64 {
        return None;
    }
    let truncated = value as i64;
    Some(if (truncated as f64) > value {
        truncated - 1
    } else {
        truncated
    })
}

#[cfg(test)]
mod tests {
    use std::vec::Vec;

    use super::*;
    use crate::{Format, Period};

    const ISO: &str = "2026-10-27T12:34:56.789Z";

    fn parse(value: &str) -> DateTime {
        DateTime::parse_with_format(value, Format::Rfc3339).unwrap()
    }

    fn encode_tagged(f: impl FnOnce(&mut Encoder<Vec<u8>>)) -> Vec<u8> {
        let mut encoder = Encoder::new(Vec::new());
        encoder.tag(Tag::new(EPOCH_TAG)).unwrap();
        f(&mut encoder);
        encoder.into_inner()
    }

    fn encode_tag0(f: impl FnOnce(&mut Encoder<Vec<u8>>)) -> Vec<u8> {
        let mut encoder = Encoder::new(Vec::new());
        encoder.tag(Tag::new(DATE_TIME_STRING_TAG)).unwrap();
        f(&mut encoder);
        encoder.into_inner()
    }

    fn encode_string(value: &str) -> Vec<u8> {
        encode_tag0(|encoder| {
            encoder.text(value).unwrap();
        })
    }

    fn encode_integer(seconds: i64) -> Vec<u8> {
        encode_tagged(|encoder| {
            encoder.i64(seconds).unwrap();
        })
    }

    fn decode(bytes: &[u8]) -> Result<DateTime, DecodeError> {
        ::cbor::from_slice(bytes)
    }

    #[test]
    fn encodes_tagged_integer_seconds() {
        let value = parse(ISO);
        assert_eq!(::cbor::to_vec(&value), encode_integer(value.unix()));
    }

    #[test]
    fn truncates_subseconds() {
        let value = parse(ISO);
        let decoded = decode(&::cbor::to_vec(&value)).unwrap();
        assert_eq!(decoded, parse("2026-10-27T12:34:56Z"));
        assert_eq!(decoded.nanosecond(), 0);
    }

    #[test]
    fn round_trips_whole_seconds() {
        let value = parse("2026-10-27T12:34:56Z");
        let encoded = ::cbor::to_vec(&value);
        assert_eq!(decode(&encoded).unwrap(), value);
    }

    #[test]
    fn decodes_negative_values() {
        let encoded = encode_integer(-1);
        assert_eq!(decode(&encoded).unwrap(), DateTime::from_unix(-1).unwrap());
    }

    #[test]
    fn decodes_f64_and_truncates() {
        let encoded = encode_tagged(|encoder| {
            encoder.f64(1_700_000_000.999).unwrap();
        });
        assert_eq!(decode(&encoded).unwrap(), DateTime::from_unix(1_700_000_000).unwrap());
    }

    #[test]
    fn decodes_negative_f64_by_rounding_down() {
        let encoded = encode_tagged(|encoder| {
            encoder.f64(-0.5).unwrap();
        });
        assert_eq!(decode(&encoded).unwrap(), DateTime::from_unix(-1).unwrap());
    }

    #[test]
    fn decodes_f32() {
        let encoded = encode_tagged(|encoder| {
            encoder.f32(1_700_000_000.0).unwrap();
        });
        assert_eq!(decode(&encoded).unwrap(), DateTime::from_unix(1_700_000_000).unwrap());
    }

    #[test]
    fn decodes_f16() {
        // 1.5 in binary16 is 0x3e00.
        let encoded = [0xc1, 0xf9, 0x3e, 0x00];
        assert_eq!(decode(&encoded).unwrap(), DateTime::from_unix(1).unwrap());

        // 1.0 in binary16 is 0x3c00.
        let encoded = [0xc1, 0xf9, 0x3c, 0x00];
        assert_eq!(decode(&encoded).unwrap(), DateTime::from_unix(1).unwrap());
    }

    #[test]
    fn decodes_tag0_string_with_fractional_seconds() {
        let decoded = decode(&encode_string(ISO)).unwrap();
        assert_eq!(decoded, parse(ISO));
        assert_eq!(decoded.nanosecond(), 789_000_000);
    }

    #[test]
    fn decodes_tag0_string_with_offset() {
        let decoded = decode(&encode_string("2026-10-27T12:34:56-08:00")).unwrap();
        assert_eq!(decoded.unix(), parse("2026-10-27T20:34:56Z").unix());
        assert_eq!(decoded.offset(), -8 * 3600);
        assert_eq!(decoded.hour(), 12);
    }

    #[test]
    fn rejects_tag0_non_utf8_or_non_text_content() {
        let encoded = encode_tag0(|encoder| {
            encoder.i64(0).unwrap();
        });
        assert!(decode(&encoded).is_err());

        // Tag 0 followed by a text string with invalid UTF-8.
        assert!(decode(&[0xc0, 0x62, 0xff, 0xfe]).is_err());
    }

    #[test]
    fn rejects_tag0_non_rfc3339_strings() {
        // A zone-less date-time is not a complete RFC 3339 date-time.
        assert!(decode(&encode_string("2026-10-27T12:34:56")).is_err());
        // Date-only strings and RFC 9557 annotations are not RFC 3339.
        assert!(decode(&encode_string("2026-10-27")).is_err());
        assert!(decode(&encode_string("2026-10-27T12:34:56Z[UTC]")).is_err());
    }

    #[test]
    fn decodes_range_boundaries() {
        use crate::datetime::{MAX_SECS, MIN_SECS};

        assert_eq!(
            decode(&encode_integer(MIN_SECS)).unwrap(),
            DateTime::from_unix(MIN_SECS).unwrap()
        );
        assert_eq!(
            decode(&encode_integer(MAX_SECS)).unwrap(),
            DateTime::from_unix(MAX_SECS).unwrap()
        );
    }

    #[test]
    fn rejects_values_beyond_range() {
        use crate::datetime::{MAX_SECS, MIN_SECS};

        assert!(decode(&encode_integer(MIN_SECS - 1)).is_err());
        assert!(decode(&encode_integer(MAX_SECS + 1)).is_err());
    }

    #[test]
    fn rejects_non_finite_floats() {
        let encoded = encode_tagged(|encoder| {
            encoder.f64(f64::NAN).unwrap();
        });
        assert!(decode(&encoded).is_err());

        let encoded = encode_tagged(|encoder| {
            encoder.f64(f64::INFINITY).unwrap();
        });
        assert!(decode(&encoded).is_err());

        // Half-precision infinity and NaN.
        assert!(decode(&[0xc1, 0xf9, 0x7c, 0x00]).is_err());
        assert!(decode(&[0xc1, 0xf9, 0x7e, 0x00]).is_err());
    }

    #[test]
    fn rejects_unsupported_tag() {
        let mut bytes = encode_integer(0);
        bytes[0] = 0xc2; // tag 2
        assert!(decode(&bytes).is_err());
    }

    #[test]
    fn rejects_untagged_integers() {
        assert!(decode(&::cbor::to_vec(&0i64)).is_err());
    }

    #[test]
    fn rejects_wrong_content_type() {
        let mut bytes = std::vec![0xc1];
        bytes.extend_from_slice(&::cbor::to_vec("2026-10-27T12:34:56Z"));
        assert!(decode(&bytes).is_err());
    }

    #[test]
    fn rejects_out_of_range_values() {
        let encoded = encode_integer(i64::MAX);
        assert!(decode(&encoded).is_err());

        let encoded = encode_tagged(|encoder| {
            encoder.f64(1e300).unwrap();
        });
        assert!(decode(&encoded).is_err());
    }

    #[test]
    fn optional_round_trips() {
        let value = parse("2026-10-27T12:34:56Z");
        let encoded = ::cbor::to_vec(&Some(value));
        assert_eq!(::cbor::from_slice::<Option<DateTime>>(&encoded).unwrap(), Some(value));

        let encoded = ::cbor::to_vec(&Option::<DateTime>::None);
        assert_eq!(::cbor::from_slice::<Option<DateTime>>(&encoded).unwrap(), None);
    }

    #[test]
    fn truncation_is_idempotent() {
        let value = parse(ISO).truncate(Period::seconds(1));
        let encoded = ::cbor::to_vec(&value);
        assert_eq!(decode(&encoded).unwrap(), value);
    }
}
