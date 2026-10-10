//! Decoding PostgreSQL binary values into Rust values.

use postgresql_protocol::oid::{
    BOOLOID, BPCHAROID, BYTEAOID, CIDROID, DATEOID, FLOAT4OID, FLOAT8OID, INETOID, INT2OID, INT4OID, INT8OID,
    INTERVALOID, JSONBOID, JSONOID, NAMEOID, Oid, TEXTOID, TIMEOID, TIMESTAMPOID, TIMESTAMPTZOID, TIMETZOID, UUIDOID,
    VARCHAROID, array_element,
};

use crate::{encode::PG_EPOCH_UNIX, error::Error, interval::Interval, json::Json};

/// A value that can be decoded from the PostgreSQL binary format.
pub trait FromSql: Sized {
    /// Decodes a non-null value of type `oid`.
    ///
    /// Returns an error if `buf` is too short or malformed for the type.
    fn from_sql(oid: Oid, buf: &[u8]) -> Result<Self, Error>;

    /// Decodes a non-null value, using `zone` to interpret zone-less temporal
    /// values (`timestamp`, `date`, `time`).
    ///
    /// The default implementation ignores `zone`.
    fn from_sql_zoned(oid: Oid, buf: &[u8], zone: time::TimeZone) -> Result<Self, Error> {
        let _ = zone;
        Self::from_sql(oid, buf)
    }

    /// Decodes SQL `NULL`.
    ///
    /// The default implementation errors; [`Option<T>`] overrides it to return
    /// `None`.
    fn from_sql_null() -> Result<Self, Error> {
        Err(Error::UnexpectedNull(String::new()))
    }

    /// Whether this type can be decoded from a column of type `oid`.
    ///
    /// Used to report a type mismatch ("column `x` is `int4`, cannot decode as
    /// `bool`") instead of silently reinterpreting the bytes of another type.
    /// The default accepts every type.
    fn accepts(oid: Oid) -> bool {
        let _ = oid;
        true
    }
}

fn need(buf: &[u8], n: usize, what: &str) -> Result<(), Error> {
    if buf.len() < n {
        Err(Error::Decode(format!("{what}: buffer of {} bytes is too short", buf.len())))
    } else {
        Ok(())
    }
}

macro_rules! impl_int {
    ($ty:ty, $n:expr, $name:expr, $oid:expr) => {
        impl FromSql for $ty {
            fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
                need(buf, $n, $name)?;
                let arr: [u8; $n] = buf[..$n].try_into().unwrap();
                Ok(<$ty>::from_be_bytes(arr))
            }

            fn accepts(oid: Oid) -> bool {
                oid == $oid
            }
        }
    };
}

impl_int!(i16, 2, "int2", INT2OID);
impl_int!(i32, 4, "int4", INT4OID);
impl_int!(i64, 8, "int8", INT8OID);

impl FromSql for f32 {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        need(buf, 4, "float4")?;
        Ok(f32::from_be_bytes(buf[..4].try_into().unwrap()))
    }

    fn accepts(oid: Oid) -> bool {
        oid == FLOAT4OID
    }
}

impl FromSql for f64 {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        need(buf, 8, "float8")?;
        Ok(f64::from_be_bytes(buf[..8].try_into().unwrap()))
    }

    fn accepts(oid: Oid) -> bool {
        oid == FLOAT8OID
    }
}

impl FromSql for bool {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        need(buf, 1, "bool")?;
        Ok(buf[0] != 0)
    }

    fn accepts(oid: Oid) -> bool {
        oid == BOOLOID
    }
}

impl FromSql for String {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        Ok(std::str::from_utf8(buf)?.to_owned())
    }

    fn accepts(oid: Oid) -> bool {
        matches!(oid, TEXTOID | VARCHAROID | BPCHAROID | NAMEOID)
    }
}

impl FromSql for Vec<u8> {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        Ok(buf.to_vec())
    }

    fn accepts(oid: Oid) -> bool {
        oid == BYTEAOID
    }
}

impl FromSql for uuid::Uuid {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        uuid::Uuid::from_slice(buf).map_err(Error::Uuid)
    }

    fn accepts(oid: Oid) -> bool {
        oid == UUIDOID
    }
}

fn from_pg_micros(micros: i64) -> Result<time::DateTime, Error> {
    let nanos = (PG_EPOCH_UNIX as i128) * 1_000_000_000 + (micros as i128) * 1_000;
    Ok(time::DateTime::from_unix_nanos(nanos)?)
}

/// Rejects PostgreSQL's `infinity` / `-infinity` timestamp sentinels.
///
/// PostgreSQL encodes them as the extreme `i64` values, which are not a real
/// instant and cannot be represented as a `time::DateTime`. Reporting them as a
/// clear error is better than silently decoding a nonsensical far-future date
/// (or overflowing).
fn reject_timestamp_infinity(micros: i64) -> Result<(), Error> {
    if micros == i64::MAX || micros == i64::MIN {
        return Err(Error::Decode(
            "PostgreSQL `infinity` / `-infinity` timestamps are not supported".into(),
        ));
    }
    Ok(())
}

/// Rejects PostgreSQL's `infinity` / `-infinity` date sentinels (extreme `i32`).
fn reject_date_infinity(days: i32) -> Result<(), Error> {
    if days == i32::MAX || days == i32::MIN {
        return Err(Error::Decode(
            "PostgreSQL `infinity` / `-infinity` dates are not supported".into(),
        ));
    }
    Ok(())
}

impl FromSql for time::DateTime {
    fn from_sql(oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        match oid {
            TIMESTAMPTZOID | TIMESTAMPOID => {
                need(buf, 8, "timestamp")?;
                let micros = i64::from_be_bytes(buf[..8].try_into().unwrap());
                reject_timestamp_infinity(micros)?;
                from_pg_micros(micros)
            }
            DATEOID => {
                need(buf, 4, "date")?;
                let days = i32::from_be_bytes(buf[..4].try_into().unwrap());
                reject_date_infinity(days)?;
                let unix = PG_EPOCH_UNIX + days as i64 * 86_400;
                Ok(time::DateTime::from_unix(unix)?)
            }
            TIMEOID => {
                need(buf, 8, "time")?;
                let micros = i64::from_be_bytes(buf[..8].try_into().unwrap());
                // PostgreSQL's `time` counts microseconds since midnight,
                // `0..=86_400_000_000` (where the maximum is `24:00:00`). A
                // value outside that range is not a time of day; rejecting it
                // avoids decoding `24:00:00` as the next day's midnight and
                // bounds a hostile value.
                if !(0..86_400_000_000).contains(&micros) {
                    return Err(Error::Decode(format!("time value {micros} is outside a single day")));
                }
                Ok(time::DateTime::from_unix_nanos((micros as i128) * 1_000)?)
            }
            TIMETZOID => {
                need(buf, 12, "timetz")?;
                let micros = i64::from_be_bytes(buf[..8].try_into().unwrap());
                let offset = i32::from_be_bytes(buf[8..12].try_into().unwrap());
                if !(0..86_400_000_000).contains(&micros) {
                    return Err(Error::Decode(format!("timetz value {micros} is outside a single day")));
                }
                let nanos = (micros as i128 - (offset as i128) * 1_000_000) * 1_000;
                Ok(time::DateTime::from_unix_nanos(nanos)?)
            }
            other => Err(Error::Decode(format!("cannot decode OID {other} as DateTime"))),
        }
    }

    fn accepts(oid: Oid) -> bool {
        matches!(oid, TIMESTAMPTZOID | TIMESTAMPOID | DATEOID | TIMEOID | TIMETZOID)
    }

    fn from_sql_zoned(oid: Oid, buf: &[u8], zone: time::TimeZone) -> Result<Self, Error> {
        match oid {
            // Absolute instants: the configured zone is irrelevant.
            TIMESTAMPTZOID | TIMETZOID => Self::from_sql(oid, buf),
            // Zone-less values: interpret the stored wall clock in `zone`.
            TIMESTAMPOID => {
                need(buf, 8, "timestamp")?;
                let micros = i64::from_be_bytes(buf[..8].try_into().unwrap());
                reject_timestamp_infinity(micros)?;
                civil_in_zone(from_pg_micros(micros)?, zone)
            }
            DATEOID => {
                need(buf, 4, "date")?;
                let days = i32::from_be_bytes(buf[..4].try_into().unwrap());
                reject_date_infinity(days)?;
                civil_in_zone(time::DateTime::from_unix(PG_EPOCH_UNIX + days as i64 * 86_400)?, zone)
            }
            TIMEOID => {
                need(buf, 8, "time")?;
                let micros = i64::from_be_bytes(buf[..8].try_into().unwrap());
                civil_in_zone(time::DateTime::from_unix_nanos((micros as i128) * 1_000)?, zone)
            }
            other => Err(Error::Decode(format!("cannot decode OID {other} as DateTime"))),
        }
    }
}

/// Reinterprets the civil fields of `naive` (a UTC instant) as a wall clock in
/// `zone`, returning the corresponding instant.
fn civil_in_zone(naive: time::DateTime, zone: time::TimeZone) -> Result<time::DateTime, Error> {
    Ok(time::DateTime::from_parts_with(
        naive.year(),
        naive.month(),
        naive.day(),
        naive.hour(),
        naive.minute(),
        naive.second(),
        naive.nanosecond(),
        zone,
        time::Disambiguation::Compatible,
    )?)
}

/// Decodes a `json` / `jsonb` payload into any `serde` type.
///
/// Returns an error if `oid` is not a JSON type, if a `jsonb` payload is
/// missing its version byte or carries an unsupported one, or if the payload
/// is not valid JSON for `T`.
pub fn json_from_sql<T: serde::de::DeserializeOwned>(oid: Oid, buf: &[u8]) -> Result<T, Error> {
    match oid {
        JSONBOID => {
            need(buf, 1, "jsonb")?;
            if buf[0] != 1 {
                return Err(Error::Decode(format!("unsupported jsonb format version {}", buf[0])));
            }
            Ok(serde_json::from_slice(&buf[1..])?)
        }
        JSONOID => Ok(serde_json::from_slice(buf)?),
        other => Err(Error::Decode(format!("cannot decode OID {other} as JSON"))),
    }
}

/// Whether `oid` is a JSON type (`json` or `jsonb`).
pub fn json_accepts(oid: Oid) -> bool {
    matches!(oid, JSONOID | JSONBOID)
}

impl FromSql for serde_json::Value {
    fn from_sql(oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        json_from_sql(oid, buf)
    }

    fn accepts(oid: Oid) -> bool {
        json_accepts(oid)
    }
}

impl<T: serde::de::DeserializeOwned> FromSql for Json<T> {
    fn from_sql(oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        json_from_sql(oid, buf).map(Json)
    }

    fn accepts(oid: Oid) -> bool {
        json_accepts(oid)
    }
}

impl FromSql for ipnetwork::IpNetwork {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        need(buf, 4, "inet")?;
        let family = buf[0];
        let prefix = buf[1];
        let len = buf[3] as usize;
        need(buf, 4 + len, "inet")?;
        let addr = &buf[4..4 + len];
        let network = match family {
            2 => {
                let octets: [u8; 4] = addr
                    .try_into()
                    .map_err(|_| Error::Decode("invalid IPv4 length".into()))?;
                ipnetwork::IpNetwork::new(std::net::IpAddr::from(octets), prefix)?
            }
            3 => {
                let octets: [u8; 16] = addr
                    .try_into()
                    .map_err(|_| Error::Decode("invalid IPv6 length".into()))?;
                ipnetwork::IpNetwork::new(std::net::IpAddr::from(octets), prefix)?
            }
            other => return Err(Error::Decode(format!("unknown inet family {other}"))),
        };
        Ok(network)
    }

    fn accepts(oid: Oid) -> bool {
        matches!(oid, INETOID | CIDROID)
    }
}

impl FromSql for Interval {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        need(buf, 16, "interval")?;
        Ok(Interval {
            micros: i64::from_be_bytes(buf[0..8].try_into().unwrap()),
            days: i32::from_be_bytes(buf[8..12].try_into().unwrap()),
            months: i32::from_be_bytes(buf[12..16].try_into().unwrap()),
        })
    }

    fn accepts(oid: Oid) -> bool {
        oid == INTERVALOID
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_sql(oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        T::from_sql(oid, buf).map(Some)
    }

    fn from_sql_zoned(oid: Oid, buf: &[u8], zone: time::TimeZone) -> Result<Self, Error> {
        T::from_sql_zoned(oid, buf, zone).map(Some)
    }

    fn from_sql_null() -> Result<Self, Error> {
        Ok(None)
    }

    fn accepts(oid: Oid) -> bool {
        T::accepts(oid)
    }
}

impl<T: FromSql> FromSql for Vec<T> {
    fn from_sql(_oid: Oid, buf: &[u8]) -> Result<Self, Error> {
        decode_array(buf, time::TimeZone::UTC)
    }

    fn from_sql_zoned(_oid: Oid, buf: &[u8], zone: time::TimeZone) -> Result<Self, Error> {
        decode_array(buf, zone)
    }

    fn accepts(oid: Oid) -> bool {
        array_element(oid).is_some_and(|elem| T::accepts(elem.oid))
    }
}

fn decode_array<T: FromSql>(buf: &[u8], zone: time::TimeZone) -> Result<Vec<T>, Error> {
    need(buf, 12, "array")?;
    let ndim = i32::from_be_bytes(buf[0..4].try_into().unwrap());
    let element_oid = u32::from_be_bytes(buf[8..12].try_into().unwrap());

    if ndim == 0 {
        return Ok(Vec::new());
    }
    if ndim != 1 {
        return Err(Error::Decode("only one-dimensional arrays are supported".into()));
    }
    need(buf, 20, "array")?;

    let raw_count = i32::from_be_bytes(buf[12..16].try_into().unwrap());
    let count = usize::try_from(raw_count).map_err(|_| Error::Decode("array has a negative element count".into()))?;
    // Every element costs at least its 4-byte length prefix, so a count that
    // cannot fit in the payload is malformed (or hostile) and must not be used
    // to size an allocation.
    if count > (buf.len() - 20) / 4 {
        return Err(Error::Decode(format!(
            "array claims {count} elements but the payload holds at most {}",
            (buf.len() - 20) / 4
        )));
    }
    // The element OID comes from the payload and must be one the target type
    // can actually hold; otherwise a server could have an `int4[]` column
    // decode as, say, `text` with a different element type.
    if !T::accepts(element_oid) {
        return Err(Error::Decode(format!(
            "array element type OID {element_oid} cannot be decoded as the requested type"
        )));
    }

    let mut pos = 20;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        need(&buf[pos..], 4, "array element")?;
        let raw_len = i32::from_be_bytes(buf[pos..pos + 4].try_into().unwrap());
        pos += 4;
        if raw_len == -1 {
            out.push(T::from_sql_null()?);
        } else {
            let len = usize::try_from(raw_len)
                .map_err(|_| Error::Decode(format!("array element has an invalid length {raw_len}")))?;
            need(&buf[pos..], len, "array element")?;
            out.push(T::from_sql_zoned(element_oid, &buf[pos..pos + len], zone)?);
            pos += len;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use postgresql_protocol::oid::{INT4_ARRAY_OID, INT8_ARRAY_OID, JSONBOID, JSONOID, TEXT_ARRAY_OID};

    use super::*;

    #[test]
    fn jsonb_version_byte_is_validated() {
        // Version 1 is the only supported `jsonb` wire format.
        assert_eq!(
            json_from_sql::<serde_json::Value>(JSONBOID, b"\x01{}").unwrap(),
            serde_json::json!({})
        );
        // A different version must be rejected, not decoded.
        assert!(json_from_sql::<serde_json::Value>(JSONBOID, b"\x02{}").is_err());
        // A missing version byte is rejected too.
        assert!(json_from_sql::<serde_json::Value>(JSONBOID, b"").is_err());
        // `json` (not `jsonb`) has no version byte.
        assert_eq!(
            json_from_sql::<serde_json::Value>(JSONOID, b"{}").unwrap(),
            serde_json::json!({})
        );
    }

    #[test]
    fn decode_basic() {
        assert_eq!(i32::from_sql(23, &42i32.to_be_bytes()).unwrap(), 42);
        assert!(bool::from_sql(16, &[1]).unwrap());
        assert_eq!(String::from_sql(25, b"hi").unwrap(), "hi");
        assert_eq!(Option::<i32>::from_sql_null().unwrap(), None);
    }

    #[test]
    fn decode_array() {
        // 3 elements: [1, NULL, 3]
        let mut buf = Vec::new();
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&23u32.to_be_bytes());
        buf.extend_from_slice(&3i32.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&4i32.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&(-1i32).to_be_bytes());
        buf.extend_from_slice(&4i32.to_be_bytes());
        buf.extend_from_slice(&3i32.to_be_bytes());

        assert_eq!(
            Vec::<i32>::from_sql(1007, &buf)
                .unwrap_err()
                .to_string()
                .contains("NULL"),
            true
        );
        assert_eq!(Vec::<Option<i32>>::from_sql(1007, &buf).unwrap(), vec![Some(1), None, Some(3)]);
    }

    #[test]
    fn array_with_impossible_count_is_rejected() {
        // 2^31-1 claimed elements in a 24-byte payload must not size a
        // multi-gigabyte allocation.
        let mut buf = Vec::new();
        buf.extend_from_slice(&1i32.to_be_bytes()); // ndim
        buf.extend_from_slice(&0i32.to_be_bytes()); // has nulls
        buf.extend_from_slice(&23u32.to_be_bytes()); // element oid
        buf.extend_from_slice(&i32::MAX.to_be_bytes()); // count
        buf.extend_from_slice(&1i32.to_be_bytes()); // lower bound
        let err = Vec::<Option<i32>>::from_sql(1007, &buf).unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");

        // Negative counts are rejected rather than reinterpreted.
        let mut buf = Vec::new();
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&23u32.to_be_bytes());
        buf.extend_from_slice(&(-1i32).to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        assert!(Vec::<Option<i32>>::from_sql(1007, &buf).is_err());
    }

    #[test]
    fn array_with_invalid_element_length_is_rejected() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&23u32.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&(-2i32).to_be_bytes()); // not NULL (-1), not a length
        assert!(Vec::<Option<i32>>::from_sql(1007, &buf).is_err());
    }

    #[test]
    fn short_array_headers_are_rejected() {
        // Less than the 12-byte fixed header.
        for len in 0..12 {
            assert!(
                Vec::<Option<i32>>::from_sql(1007, &vec![0u8; len]).is_err(),
                "len {len} must be rejected"
            );
        }
        // 12..=19 bytes: a dimension count of 1 promises the 20-byte header.
        for len in 12..20 {
            let mut buf = vec![0u8; len];
            buf[0..4].copy_from_slice(&1i32.to_be_bytes());
            assert!(Vec::<Option<i32>>::from_sql(1007, &buf).is_err(), "len {len} must be rejected");
        }
        // Exactly 12 zero bytes is a valid empty array (ndim = 0).
        assert!(Vec::<Option<i32>>::from_sql(1007, &vec![0u8; 12]).unwrap().is_empty());
    }

    #[test]
    fn type_mismatches_are_rejected_before_decoding() {
        assert!(i32::accepts(INT4OID));
        assert!(!i32::accepts(INT8OID));
        assert!(!bool::accepts(INT4OID));
        assert!(String::accepts(VARCHAROID));
        assert!(!String::accepts(JSONOID));
        assert!(time::DateTime::accepts(TIMESTAMPOID));
        assert!(Vec::<i32>::accepts(INT4_ARRAY_OID));
        assert!(!Vec::<i32>::accepts(INT8_ARRAY_OID));
        assert!(Vec::<Option<String>>::accepts(TEXT_ARRAY_OID));
    }

    #[test]
    fn array_element_oid_must_match_the_target_type() {
        // An int4[] column whose payload declares the elements are `text`:
        // the element OID comes from the payload and must be validated.
        let mut buf = Vec::new();
        buf.extend_from_slice(&1i32.to_be_bytes()); // ndim
        buf.extend_from_slice(&0i32.to_be_bytes()); // has nulls
        buf.extend_from_slice(&TEXTOID.to_be_bytes()); // element oid = text
        buf.extend_from_slice(&1i32.to_be_bytes()); // count
        buf.extend_from_slice(&1i32.to_be_bytes()); // lower bound
        buf.extend_from_slice(&4i32.to_be_bytes());
        buf.extend_from_slice(&42i32.to_be_bytes());
        let err = Vec::<i32>::from_sql(INT4_ARRAY_OID, &buf).unwrap_err();
        assert!(matches!(err, Error::Decode(_)), "{err}");

        // An unknown element OID is rejected too, instead of being ignored.
        let mut buf = Vec::new();
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&999_999u32.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.extend_from_slice(&2i32.to_be_bytes());
        buf.extend_from_slice(b"hi");
        assert!(Vec::<String>::from_sql(TEXT_ARRAY_OID, &buf).is_err());
    }

    #[test]
    fn infinity_timestamps_and_dates_are_rejected() {
        for micros in [i64::MAX, i64::MIN] {
            assert!(time::DateTime::from_sql(TIMESTAMPTZOID, &micros.to_be_bytes()).is_err());
            assert!(time::DateTime::from_sql(TIMESTAMPOID, &micros.to_be_bytes()).is_err());
            assert!(time::DateTime::from_sql_zoned(TIMESTAMPOID, &micros.to_be_bytes(), time::TimeZone::UTC).is_err());
        }
        for days in [i32::MAX, i32::MIN] {
            assert!(time::DateTime::from_sql(DATEOID, &days.to_be_bytes()).is_err());
            assert!(time::DateTime::from_sql_zoned(DATEOID, &days.to_be_bytes(), time::TimeZone::UTC).is_err());
        }
    }

    #[test]
    fn decode_timestamptz() {
        let dt = time::DateTime::from_unix(PG_EPOCH_UNIX + 5).unwrap();
        assert_eq!(time::DateTime::from_sql(1184, &5_000_000i64.to_be_bytes()).unwrap(), dt);
    }

    #[test]
    fn time_values_are_range_checked() {
        // Midnight and the last representable time before 24:00 decode.
        assert!(time::DateTime::from_sql(TIMEOID, &0i64.to_be_bytes()).is_ok());
        assert!(time::DateTime::from_sql(TIMEOID, &86_399_999_999i64.to_be_bytes()).is_ok());
        // 24:00:00 and hostile values are rejected, never panic.
        for micros in [86_400_000_000i64, i64::MAX, i64::MIN, -1] {
            assert!(
                time::DateTime::from_sql(TIMEOID, &micros.to_be_bytes()).is_err(),
                "time {micros} must be rejected"
            );
        }
        // timetz: an in-day time decodes; hostile values error rather than panic.
        let mut buf = 5_000_000i64.to_be_bytes().to_vec();
        buf.extend_from_slice(&3_600i32.to_be_bytes());
        assert!(time::DateTime::from_sql(TIMETZOID, &buf).is_ok());
        let mut buf = i64::MAX.to_be_bytes().to_vec();
        buf.extend_from_slice(&i32::MAX.to_be_bytes());
        assert!(time::DateTime::from_sql(TIMETZOID, &buf).is_err());
    }

    #[test]
    fn inet_mismatched_family_and_lengths_are_rejected() {
        // family 2 (IPv4) with a 16-byte address.
        let mut buf = vec![2u8, 32u8, 0u8, 16u8];
        buf.extend_from_slice(&[0u8; 16]);
        assert!(ipnetwork::IpNetwork::from_sql(INETOID, &buf).is_err());
        // family 2 with a zero-length address.
        assert!(ipnetwork::IpNetwork::from_sql(INETOID, &[2u8, 8u8, 0u8, 0u8]).is_err());
        // unknown address family.
        assert!(ipnetwork::IpNetwork::from_sql(INETOID, &[9u8, 8u8, 0u8, 4u8, 1, 2, 3, 4]).is_err());
        // truncated header.
        assert!(ipnetwork::IpNetwork::from_sql(INETOID, &[2u8, 8u8, 0u8]).is_err());
        // a valid IPv4 value still decodes.
        assert!(ipnetwork::IpNetwork::from_sql(INETOID, &[2u8, 24u8, 1u8, 4u8, 10, 0, 0, 0]).is_ok());
    }
}
