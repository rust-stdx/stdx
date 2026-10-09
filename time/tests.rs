//! Tests for the `time` crate.
//!
//! These tests use `std` (the test harness needs it, and `to_string` uses
//! `alloc`); on a `no_std` build the module compiles to nothing.
#![cfg(feature = "std")]

use alloc::{format, string::ToString, vec};
use core::time::Duration;

use crate::{
    Ambiguity, Disambiguation, ErrorKind, Format, TimeZone, civil,
    datetime::{DateTime, Weekday},
};

fn parse(s: &str) -> DateTime {
    s.parse().expect("parse failed")
}

#[test]
fn unix_epoch() {
    let dt = parse("1970-01-01T00:00:00Z");
    assert_eq!(dt.unix_seconds(), 0);
    assert_eq!(dt.unix_nanos(), 0);
    assert_eq!(dt, DateTime::UNIX_EPOCH);
}

#[test]
fn epoch_boundaries() {
    assert_eq!(parse("1969-12-31T23:59:59Z").unix_seconds(), -1);
    assert_eq!(parse("2001-09-09T01:46:40Z").unix_seconds(), 1_000_000_000);
    assert_eq!(DateTime::from_unix_seconds(-1).unwrap().to_string(), "1969-12-31T23:59:59Z");
}

#[test]
fn calendar_math() {
    assert_eq!(civil::days_from_civil(1970, 1, 1), 0);
    assert_eq!(civil::civil_from_days(0), (1970, 1, 1));
    assert_eq!(civil::civil_from_days(-1), (1969, 12, 31));
    assert!(civil::is_leap(2000));
    assert!(!civil::is_leap(1900));
    assert!(civil::is_leap(2024));
    assert_eq!(civil::days_in_month(2024, 2), 29);
    assert_eq!(civil::days_in_month(2023, 2), 28);
}

#[test]
fn accessors() {
    let dt = parse("2024-02-29T13:45:06.123456789Z");
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 2);
    assert_eq!(dt.day(), 29);
    assert_eq!(dt.hour(), 13);
    assert_eq!(dt.minute(), 45);
    assert_eq!(dt.second(), 6);
    assert_eq!(dt.nanosecond(), 123_456_789);
    assert_eq!(dt.millisecond(), 123);
    assert_eq!(dt.microsecond(), 123_456);
    assert_eq!(dt.weekday(), Weekday::Thursday);
    assert_eq!(dt.ordinal(), 60);
}

#[test]
fn weekday_and_iso_week() {
    assert_eq!(parse("1970-01-01T00:00:00Z").weekday(), Weekday::Thursday);
    assert_eq!(parse("2024-01-01T00:00:00Z").weekday(), Weekday::Monday);
    assert_eq!(parse("2023-01-01T00:00:00Z").weekday(), Weekday::Sunday);
    assert_eq!(parse("2021-01-01T00:00:00Z").iso_week(), (2020, 53));
    assert_eq!(parse("2020-12-31T00:00:00Z").iso_week(), (2020, 53));
    assert_eq!(parse("2019-01-01T00:00:00Z").iso_week(), (2019, 1));
    assert_eq!(parse("2024-12-31T00:00:00Z").ordinal(), 366);
    assert_eq!(parse("2023-12-31T00:00:00Z").ordinal(), 365);
}

#[test]
fn parse_offsets() {
    let base = civil::days_from_civil(2024, 1, 1) * 86_400;
    assert_eq!(parse("2024-01-01T00:00:00+05:30").unix_seconds(), base - (5 * 3600 + 30 * 60));
    assert_eq!(parse("2024-01-01T00:00:00-08:00").unix_seconds(), base + 8 * 3600);
    assert_eq!(parse("2024-01-01T00:00:00+0530").offset(), 5 * 3600 + 30 * 60);
    assert_eq!(parse("2024-01-01T00:00:00-08").offset(), -8 * 3600);
    assert_eq!(parse("2024-01-01t00:00:00z").offset(), 0);
    assert_eq!(parse("2024-01-01 00:00:00Z").offset(), 0);
}

#[test]
fn parse_fractions() {
    assert_eq!(parse("2024-01-01T00:00:00.5Z").nanosecond(), 500_000_000);
    assert_eq!(parse("2024-01-01T00:00:00,123Z").nanosecond(), 123_000_000);
    assert_eq!(parse("2024-01-01T00:00:00.123456789123Z").nanosecond(), 123_456_789);
}

#[test]
fn parse_date_only() {
    let dt = parse("2024-01-01");
    assert_eq!(dt.unix_seconds(), civil::days_from_civil(2024, 1, 1) * 86_400);
    assert_eq!(dt.offset(), 0);
}

#[test]
fn parse_leap_second_is_clamped() {
    let dt = parse("2016-12-31T23:59:60Z");
    assert_eq!(dt.second(), 59);
}

#[test]
fn parse_with_timezone_uses_default_zone() {
    #[cfg(feature = "timezone-db")]
    {
        let zone = TimeZone::named("America/New_York").unwrap();
        let dt = DateTime::parse_with_timezone("2024-01-01T09:00", zone).unwrap();
        // 09:00 EST is 14:00 UTC.
        assert_eq!(dt.unix_seconds(), civil::days_from_civil(2024, 1, 1) * 86_400 + 14 * 3600);
        assert_eq!(dt.to_string(), "2024-01-01T09:00:00-05:00[America/New_York]");
    }
}

#[test]
fn parse_errors() {
    for bad in [
        "",
        "2024-13-01T00:00:00Z",
        "2024-02-30T00:00:00Z",
        "2023-02-29T00:00:00Z",
        "2024-01-01T24:00:00Z",
        "2024-01-01T00:60:00Z",
        "2024-01-01T00:00:61Z",
        "2024-01-01T00:00:00+25:00",
        "2024-01-01T00:00:00x",
        "2024-1-1",
        "24-01-01T00:00:00Z",
        "2024-01-01T00:00:00[Bad/Zone/Name]",
    ] {
        assert!(bad.parse::<DateTime>().is_err(), "expected {bad:?} to fail");
    }
}

#[test]
fn equality_ignores_zone() {
    let utc = parse("2024-01-01T00:00:00Z");
    let fixed = utc.in_timezone(TimeZone::fixed(-5 * 3600).unwrap());
    assert_eq!(utc, fixed);
    assert_eq!(fixed.to_string(), "2023-12-31T19:00:00-05:00");
}

#[test]
fn arithmetic() {
    let dt = parse("2024-01-01T00:00:00Z");
    assert_eq!(
        dt.checked_add(Duration::from_secs(3600)).unwrap().to_string(),
        "2024-01-01T01:00:00Z"
    );
    assert_eq!(
        dt.checked_sub(Duration::from_secs(1)).unwrap().to_string(),
        "2023-12-31T23:59:59Z"
    );

    let later = parse("2024-01-01T00:00:10.5Z");
    assert_eq!(later.duration_since(&dt).unwrap(), Duration::new(10, 500_000_000));
    assert!(dt.duration_since(&later).is_err());
}

#[test]
fn calendar_arithmetic() {
    assert_eq!(
        parse("2024-01-31T12:00:00Z").add_months(1).unwrap().to_string(),
        "2024-02-29T12:00:00Z"
    );
    assert_eq!(
        parse("2023-01-31T12:00:00Z").add_months(1).unwrap().to_string(),
        "2023-02-28T12:00:00Z"
    );
    assert_eq!(
        parse("2024-02-29T12:00:00Z").add_years(1).unwrap().to_string(),
        "2025-02-28T12:00:00Z"
    );
    assert_eq!(
        parse("2024-01-01T00:00:00Z").add_days(-1).unwrap().to_string(),
        "2023-12-31T00:00:00Z"
    );
}

#[test]
fn display_roundtrip() {
    for text in [
        "1970-01-01T00:00:00Z",
        "2024-02-29T13:45:06.123Z",
        "2024-12-31T23:59:59.999999999Z",
        "1969-12-31T23:59:59Z",
    ] {
        assert_eq!(parse(text).to_string(), text);
    }
}

#[test]
fn fixed_offset_bounds() {
    assert!(TimeZone::fixed(0).is_ok());
    assert!(TimeZone::fixed(93_599).is_ok());
    assert!(TimeZone::fixed(93_600).is_err());
    assert!(TimeZone::fixed(-93_600).is_err());
}

#[cfg(feature = "timezone-db")]
mod zones {
    use super::*;

    #[test]
    fn utc_const_and_convenience_alias() {
        assert_eq!(TimeZone::UTC.name(), Some("UTC"));
        // The bare `z` alias is the only name that maps to the const.
        assert_eq!(TimeZone::named("z").unwrap(), TimeZone::UTC);
        assert_eq!(TimeZone::named("Z").unwrap(), TimeZone::UTC);
        assert!(TimeZone::named("zz").is_err());
    }

    #[test]
    fn utc_database_names_are_preserved() {
        // IANA UTC names are ordinary zones, so the abbreviation and the
        // RFC 9557 annotation survive a round trip.
        let gmt = TimeZone::named("GMT").unwrap();
        assert_eq!(gmt.name(), Some("GMT"));
        let dt = parse("2024-01-01T12:00:00Z").in_timezone(gmt);
        assert_eq!(dt.abbreviation(), Some("GMT"));
        assert_eq!(dt.to_string(), "2024-01-01T12:00:00Z[GMT]");
        assert_eq!(dt.format(Format::Rfc9557).to_string(), "2024-01-01T12:00:00Z[GMT]");

        let round = parse("2024-01-01T12:00:00Z[GMT]");
        assert_eq!(round.timezone_name(), Some("GMT"));
        assert_eq!(round.abbreviation(), Some("GMT"));
        assert_eq!(round.to_string(), "2024-01-01T12:00:00Z[GMT]");

        for name in ["UTC", "Etc/UTC", "Etc/GMT", "UCT", "Universal", "Zulu"] {
            assert_eq!(TimeZone::named(name).unwrap().name(), Some(name), "{name}");
        }
        // Named UTC zones are distinct values from the `UTC` constant; the
        // name-aware equality is intentional.
        assert_ne!(TimeZone::named("UTC").unwrap(), TimeZone::UTC);
    }

    #[test]
    fn known_zone() {
        let zone = TimeZone::named("America/New_York").unwrap();
        assert_eq!(zone.name(), Some("America/New_York"));
        assert!(TimeZone::named("Mars/Olympus").is_err());
        // Case insensitive lookup.
        assert!(TimeZone::named("america/new_york").is_ok());
    }

    #[test]
    fn convert_to_zone() {
        let dt = parse("2024-07-11T01:14:00Z");
        let ny = TimeZone::named("America/New_York").unwrap();
        let dt = dt.in_timezone(ny);
        assert_eq!(dt.hour(), 21);
        assert_eq!(dt.day(), 10);
        assert_eq!(dt.offset(), -4 * 3600);
        assert!(dt.is_dst());
        assert_eq!(dt.to_string(), "2024-07-10T21:14:00-04:00[America/New_York]");
    }

    #[test]
    fn gap_is_shifted_forward() {
        let zone = TimeZone::named("America/New_York").unwrap();
        let dt = DateTime::from_parts(2024, 3, 10, 2, 30, 0, 0, zone).unwrap();
        assert_eq!(dt.hour(), 3);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.offset(), -4 * 3600);
    }

    #[test]
    fn fold_selects_earlier() {
        let zone = TimeZone::named("America/New_York").unwrap();
        let dt = DateTime::from_parts(2024, 11, 3, 1, 30, 0, 0, zone).unwrap();
        assert_eq!(dt.hour(), 1);
        assert_eq!(dt.offset(), -4 * 3600);
    }

    #[test]
    fn add_days_across_dst() {
        let zone = TimeZone::named("America/New_York").unwrap();
        let dt = DateTime::from_parts(2024, 3, 9, 21, 0, 0, 0, zone).unwrap();
        assert_eq!(
            dt.add_days(1).unwrap().to_string(),
            "2024-03-10T21:00:00-04:00[America/New_York]"
        );
    }

    #[test]
    fn rfc9557_roundtrip() {
        let dt = parse("2024-03-10T01:59:59-05:00[America/New_York]");
        assert_eq!(dt.to_string(), "2024-03-10T01:59:59-05:00[America/New_York]");
    }

    #[test]
    fn annotation_without_offset() {
        let dt = parse("2024-01-01T09:00[America/New_York]");
        assert_eq!(dt.unix_seconds(), civil::days_from_civil(2024, 1, 1) * 86_400 + 14 * 3600);
    }

    #[test]
    fn offset_conflict_is_rejected() {
        let err = "2024-01-01T12:00:00+00:00[America/New_York]"
            .parse::<DateTime>()
            .unwrap_err();
        assert_eq!(err.kind(), ErrorKind::OffsetConflict);
    }

    #[test]
    fn future_uses_posix_footer() {
        // Beyond the last explicit transition, the footer rule applies.
        let dt = parse("2099-07-01T12:00:00Z");
        let ny = TimeZone::named("America/New_York").unwrap();
        assert_eq!(dt.in_timezone(ny).offset(), -4 * 3600);
        let dt = parse("2099-01-01T12:00:00Z");
        assert_eq!(dt.in_timezone(ny).offset(), -5 * 3600);
    }

    #[test]
    fn southern_hemisphere() {
        let zone = TimeZone::named("Australia/Sydney").unwrap();
        // January is summer (DST, +11), July is winter (+10).
        assert_eq!(parse("2024-01-15T00:00:00Z").in_timezone(zone).offset(), 11 * 3600);
        assert_eq!(parse("2024-07-15T00:00:00Z").in_timezone(zone).offset(), 10 * 3600);
    }
}

#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let dt = parse("2024-01-01T12:34:56.789Z");
        let json = serde_json::to_string(&dt).unwrap();
        assert_eq!(json, "\"2024-01-01T12:34:56.789Z\"");
        let back: DateTime = serde_json::from_str(&json).unwrap();
        assert_eq!(back, dt);
    }

    #[cfg(feature = "timezone-db")]
    #[test]
    fn roundtrip_with_zone() {
        let dt = parse("2024-01-01T12:34:56-05:00[America/New_York]");
        let json = serde_json::to_string(&dt).unwrap();
        assert_eq!(json, "\"2024-01-01T12:34:56-05:00[America/New_York]\"");
        let back: DateTime = serde_json::from_str(&json).unwrap();
        assert_eq!(back, dt);
        assert_eq!(back.timezone_name(), Some("America/New_York"));
    }
}

#[test]
fn weekday_numbers() {
    assert_eq!(Weekday::Monday.number_from_monday(), 1);
    assert_eq!(Weekday::Sunday.number_from_monday(), 7);
    assert_eq!(Weekday::Sunday.number_from_sunday(), 0);
    assert_eq!(Weekday::Saturday.number_from_sunday(), 6);
    assert_eq!(Weekday::Thursday.name(), "Thursday");
}

#[test]
fn negative_unix_normalization() {
    let dt = DateTime::from_unix(-1, 500_000_000).unwrap();
    assert_eq!(dt.unix_seconds(), -1);
    assert_eq!(dt.unix_nanos(), -500_000_000);
    assert_eq!(dt.unix_seconds(), -1);
    // `-1s + 0.5s` is `1969-12-31T23:59:59.5Z`.
    assert_eq!(dt.to_string(), "1969-12-31T23:59:59.500Z");
}

#[test]
fn negative_year_roundtrip() {
    let dt = parse("-0001-01-01T00:00:00Z");
    assert_eq!(dt.year(), -1);
    assert_eq!(dt.to_string(), "-0001-01-01T00:00:00Z");
}

#[test]
fn offset_with_seconds() {
    let dt = parse("2024-01-01T00:00:00+00:00:30");
    assert_eq!(dt.offset(), 30);
    assert_eq!(dt.unix_seconds(), civil::days_from_civil(2024, 1, 1) * 86_400 - 30);
    assert!("2024-01-01T00:00:00+00:00:60".parse::<DateTime>().is_err());
    assert!("2024-01-01T00:00:00+23:60".parse::<DateTime>().is_err());
}

#[test]
fn invalid_civil_fields() {
    assert!(DateTime::from_parts(2023, 2, 29, 0, 0, 0, 0, TimeZone::UTC).is_err());
    assert!(DateTime::from_parts(2024, 0, 1, 0, 0, 0, 0, TimeZone::UTC).is_err());
    assert!(DateTime::from_parts(2024, 1, 1, 24, 0, 0, 0, TimeZone::UTC).is_err());
    assert!(DateTime::from_parts(2024, 1, 1, 0, 0, 0, 1_000_000_000, TimeZone::UTC).is_err());
}

#[test]
fn extreme_years_are_bounded() {
    assert!(DateTime::from_parts(9999, 12, 31, 23, 59, 59, 999_999_999, TimeZone::UTC).is_ok());
    assert!(DateTime::from_parts(-9999, 1, 1, 0, 0, 0, 0, TimeZone::UTC).is_ok());
    assert!(DateTime::from_parts(10000, 1, 1, 0, 0, 0, 0, TimeZone::UTC).is_err());
    assert!(DateTime::from_parts(-10000, 1, 1, 0, 0, 0, 0, TimeZone::UTC).is_err());
}

#[test]
fn fixed_offset_is_never_dst() {
    let dt = parse("2024-07-01T12:00:00Z").in_timezone(TimeZone::fixed(-4 * 3600).unwrap());
    assert!(!dt.is_dst());
    assert_eq!(dt.timezone_name(), None);
}

#[test]
fn add_months_and_years_negative() {
    assert_eq!(
        parse("2024-03-31T00:00:00Z").add_months(-1).unwrap().to_string(),
        "2024-02-29T00:00:00Z"
    );
    assert_eq!(
        parse("2024-01-01T00:00:00Z").add_months(-1).unwrap().to_string(),
        "2023-12-01T00:00:00Z"
    );
    assert_eq!(
        parse("2024-02-29T00:00:00Z").add_years(-1).unwrap().to_string(),
        "2023-02-28T00:00:00Z"
    );
}

#[test]
fn saturating_arithmetic() {
    let max = DateTime::from_parts(9999, 12, 31, 23, 59, 59, 0, TimeZone::UTC).unwrap();
    assert!(max.checked_add(Duration::from_secs(1)).is_err());
    let _ = max.saturating_add(Duration::from_secs(86_400 * 365));
    let min = DateTime::from_parts(-9999, 1, 1, 0, 0, 0, 0, TimeZone::UTC).unwrap();
    assert!(min.checked_sub(Duration::from_secs(1)).is_err());
    let _ = min.saturating_sub(Duration::from_secs(1));
}

#[test]
fn tzif_malformed_input_is_safe() {
    use crate::tzif::Tzif;

    // Empty, truncated and garbage input must never panic.
    assert!(Tzif::parse(&[]).is_none());
    assert!(Tzif::parse(b"TZif").is_none());
    let mut state = 0x1234_5678u32;
    let mut buffer = [0u8; 256];
    for _ in 0..2000 {
        for byte in &mut buffer {
            state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            *byte = (state >> 16) as u8;
        }
        let _ = Tzif::parse(&buffer);
    }
}

/// Builds a minimal version-1 TZif image for validation tests.
#[cfg(test)]
fn minimal_tzif(
    transitions: &[i32],
    indices: &[u8],
    typecnt: u32,
    charcnt: u32,
    designation: u8,
) -> alloc::vec::Vec<u8> {
    let mut data = alloc::vec::Vec::new();
    data.extend_from_slice(b"TZif");
    data.push(0);
    data.extend_from_slice(&[0u8; 15]);
    data.extend_from_slice(&0u32.to_be_bytes()); // isutcnt
    data.extend_from_slice(&0u32.to_be_bytes()); // isstdcnt
    data.extend_from_slice(&0u32.to_be_bytes()); // leapcnt
    data.extend_from_slice(&(transitions.len() as u32).to_be_bytes());
    data.extend_from_slice(&typecnt.to_be_bytes());
    data.extend_from_slice(&charcnt.to_be_bytes());
    for transition in transitions {
        data.extend_from_slice(&transition.to_be_bytes());
    }
    data.extend_from_slice(indices);
    for index in 0..typecnt {
        data.extend_from_slice(&(index as i32 * 3600).to_be_bytes());
        data.push(0);
        data.push(designation);
    }
    for index in 0..charcnt {
        // Alternate a letter with its NUL terminator so each designation
        // index is followed by a terminator.
        if index % 2 == 1 {
            data.push(0);
        } else {
            data.push(b'A' + (index / 2) as u8);
        }
    }
    data
}

#[test]
fn tzif_rejects_out_of_range_indices() {
    use crate::tzif::Tzif;

    // Valid: one transition, type index 0.
    let valid = minimal_tzif(&[0], &[0], 1, 2, 0);
    assert!(Tzif::parse(&valid).is_some());

    // A transition pointing at a non-existent type is rejected.
    let bad_type = minimal_tzif(&[0], &[7], 1, 2, 0);
    assert!(Tzif::parse(&bad_type).is_none());

    // Transitions that are not strictly increasing are rejected.
    let unsorted = minimal_tzif(&[100, 50], &[0, 0], 1, 2, 0);
    assert!(Tzif::parse(&unsorted).is_none());

    // A designation index outside the abbreviation block is rejected.
    let bad_designation = minimal_tzif(&[0], &[0], 1, 2, 9);
    assert!(Tzif::parse(&bad_designation).is_none());
}

#[cfg(feature = "std")]
#[test]
fn now_is_recent() {
    let now = DateTime::now();
    assert!(now.year() >= 2024, "unexpected year: {}", now.year());
    assert!(now.unix_seconds() > 1_700_000_000);
}

#[cfg(feature = "timezone-system")]
#[test]
fn system_timezone_does_not_panic() {
    let _ = TimeZone::system();
}

#[cfg(feature = "timezone-db")]
#[test]
fn historical_offset_with_seconds() {
    // Before 1937, Europe/Amsterdam used LMT +00:19:32.
    let zone = TimeZone::named("Europe/Amsterdam").unwrap();
    let dt = parse("1900-06-01T00:00:00Z").in_timezone(zone);
    assert_eq!(dt.offset(), 19 * 60 + 32);
}

#[test]
fn format_parse_fuzz() {
    // Sweep a wide range of instants, formatting and re-parsing each one.
    let mut seconds = -3_000_000_000i64;
    let mut nanos = 0u32;
    while seconds < 4_000_000_000 {
        let dt = DateTime::from_unix(seconds, i64::from(nanos)).unwrap();
        let text = dt.to_string();
        let back: DateTime = text.parse().unwrap_or_else(|err| panic!("{text:?}: {err}"));
        assert_eq!(back, dt, "round trip failed for {text}");
        seconds += 999_983;
        nanos = (nanos + 123_456_789) % 1_000_000_000;
    }
}

#[test]
fn parse_arbitrary_strings_do_not_panic() {
    let mut state = 0x9e37_79b9u32;
    let mut buffer = [0u8; 48];
    for _ in 0..20_000 {
        let len = (state % buffer.len() as u32) as usize;
        for byte in &mut buffer[..len] {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *byte = (state >> 16) as u8;
        }
        if let Ok(text) = core::str::from_utf8(&buffer[..len]) {
            let _ = text.parse::<DateTime>();
        }
    }
}

#[test]
fn parse_valid_corpus() {
    let cases: &[(&str, i32, u32)] = &[
        ("2024-01-01T00:00Z", 0, 0),
        ("2024-01-01T00:00:00Z", 0, 0),
        ("2024-01-01T00:00:00.123456789012Z", 0, 123_456_789),
        ("2024-01-01T00:00:00.1Z", 0, 100_000_000),
        ("0000-01-01T00:00:00Z", 0, 0),
        ("-0001-12-31T23:59:59Z", 0, 0),
    ];
    for (text, offset, nanos) in cases {
        let dt: DateTime = text.parse().unwrap_or_else(|err| panic!("{text:?}: {err}"));
        assert_eq!(dt.offset(), *offset, "{text}");
        assert_eq!(dt.nanosecond(), *nanos, "{text}");
    }
    // `+10000` is out of range for our calendar.
    assert!("+10000-01-01T00:00:00Z".parse::<DateTime>().is_err());
}

#[test]
fn parse_invalid_corpus() {
    for bad in [
        "2024-01-01T00:00:00.",
        "2024-01-01T00:00:00+",
        "2024-01-01T00:00:00Z[",
        "2024-01-01T00:00:00Z extra",
        "2024-01-01T0:00:00Z",
        "2024-01-01T00:00:00.123Z ",
        "2024-02-30T00:00:00Z",
        "2024-00-01T00:00:00Z",
        "2024-01-00T00:00:00Z",
        "2024-01-32T00:00:00Z",
        "2024-01-01T23:59:61Z",
        "2024-01-01T00:00:00+24:00",
        "2024-01-01T00:00:00+00:60",
        "2024-01-01T00:00:00[UTC]extra",
        "20240101T000000Z",
    ] {
        assert!(bad.parse::<DateTime>().is_err(), "expected {bad:?} to fail");
    }
}

#[cfg(feature = "timezone-db")]
#[test]
fn zone_offset_table() {
    // (zone, UTC instant, expected offset in seconds)
    let cases: &[(&str, &str, i32)] = &[
        ("Europe/London", "2024-01-15T12:00:00Z", 0),
        ("Europe/London", "2024-07-15T12:00:00Z", 3600),
        ("Asia/Kolkata", "2024-01-15T12:00:00Z", 19_800),
        ("Asia/Kolkata", "2024-07-15T12:00:00Z", 19_800),
        ("Asia/Kathmandu", "2024-01-15T12:00:00Z", 20_700),
        ("Asia/Tokyo", "2024-07-15T12:00:00Z", 32_400),
        ("Pacific/Chatham", "2024-01-15T00:00:00Z", 49_500),
        ("Pacific/Chatham", "2024-07-15T00:00:00Z", 45_900),
        ("America/St_Johns", "2024-01-15T12:00:00Z", -12_600),
        ("America/St_Johns", "2024-07-15T12:00:00Z", -9_000),
        ("America/Sao_Paulo", "2024-01-15T12:00:00Z", -10_800),
        ("America/Sao_Paulo", "2024-07-15T12:00:00Z", -10_800),
        ("Africa/Casablanca", "2024-07-15T12:00:00Z", 3600),
        ("Etc/GMT+5", "2024-01-15T12:00:00Z", -18_000),
        ("Etc/GMT-5", "2024-01-15T12:00:00Z", 18_000),
    ];
    for (zone, instant, offset) in cases {
        let zone = TimeZone::named(zone).unwrap_or_else(|err| panic!("{zone}: {err}"));
        let dt = parse(instant).in_timezone(zone);
        assert_eq!(dt.offset(), *offset, "{zone} at {instant}");
    }
}

#[test]
fn year_zero_roundtrip() {
    let dt = parse("0000-01-01T00:00:00Z");
    assert_eq!(dt.year(), 0);
    assert_eq!(dt.to_string(), "0000-01-01T00:00:00Z");
}

#[test]
fn format_known_values() {
    let dt = parse("2024-01-01T12:34:56Z");
    assert_eq!(dt.format(Format::Rfc3339).to_string(), "2024-01-01T12:34:56Z");
    assert_eq!(dt.format(Format::Rfc9557).to_string(), "2024-01-01T12:34:56Z");
    assert_eq!(dt.format(Format::Iso8601).to_string(), "2024-01-01T12:34:56+00:00");
    assert_eq!(dt.format(Format::DateOnly).to_string(), "2024-01-01");
    assert_eq!(dt.format(Format::TimeOnly).to_string(), "12:34:56");
    assert_eq!(dt.format(Format::Rfc2822).to_string(), "Mon, 01 Jan 2024 12:34:56 +0000");
    assert_eq!(dt.format(Format::HttpDate).to_string(), "Mon, 01 Jan 2024 12:34:56 GMT");
}

#[test]
fn format_with_offset() {
    let dt = parse("2024-01-01T07:34:56-05:00");
    assert_eq!(dt.format(Format::Rfc3339).to_string(), "2024-01-01T07:34:56-05:00");
    assert_eq!(dt.format(Format::Rfc2822).to_string(), "Mon, 01 Jan 2024 07:34:56 -0500");
    // HTTP dates are always UTC.
    assert_eq!(dt.format(Format::HttpDate).to_string(), "Mon, 01 Jan 2024 12:34:56 GMT");
}

#[test]
fn format_rfc9557_with_zone() {
    #[cfg(feature = "timezone-db")]
    {
        let zone = TimeZone::named("America/New_York").unwrap();
        let dt = parse("2024-07-11T01:14:00Z").in_timezone(zone);
        assert_eq!(
            dt.format(Format::Rfc9557).to_string(),
            "2024-07-10T21:14:00-04:00[America/New_York]"
        );
        // RFC 3339 does not include the zone annotation.
        assert_eq!(dt.format(Format::Rfc3339).to_string(), "2024-07-10T21:14:00-04:00");
    }
}

#[test]
fn format_out_of_domain_is_best_effort() {
    let dt = parse("-0001-01-01T00:00:00Z");
    assert!(dt.try_format(Format::Rfc3339).is_err());
    assert!(dt.try_format(Format::HttpDate).is_err());
    assert_eq!(dt.format(Format::Rfc3339).to_string(), "-0001-01-01T00:00:00Z");
    // `TimeOnly` has no year and is always representable.
    assert!(dt.try_format(Format::TimeOnly).is_ok());
    assert_eq!(dt.format(Format::TimeOnly).to_string(), "00:00:00");
}

#[test]
fn parse_http_dates() {
    let expected = parse("1994-11-06T08:49:37Z");
    for text in [
        "Sun, 06 Nov 1994 08:49:37 GMT",  // IMF-fixdate
        "Sunday, 06-Nov-94 08:49:37 GMT", // RFC 850
        "Sun Nov  6 08:49:37 1994",       // asctime
    ] {
        let dt = DateTime::parse_with_format(text, Format::HttpDate).unwrap();
        assert_eq!(dt, expected, "{text}");
        assert_eq!(dt.unix_seconds(), 784_111_777, "{text}");
    }
    for bad in [
        "Sun, 06 Nov 1994 08:49:37",      // missing GMT
        "Sun Nov 10 08*00:00 2000",       // bad separator
        "Sunday, 06-Nov-94 08+49:37 GMT", // bad separator
        "Xyz, 06 Nov 1994 08:49:37 GMT",  // bad weekday
        "Sun, 30 Feb 1994 08:49:37 GMT",  // impossible date
    ] {
        assert!(
            DateTime::parse_with_format(bad, Format::HttpDate).is_err(),
            "expected {bad:?} to fail"
        );
    }
}

#[test]
fn parse_http_uses_rfc2822_year_pivot() {
    // Two-digit years 50..99 map to 1950..1999, 00..49 to 2000..2049.
    let dt = DateTime::parse_with_format("Sunday, 06-Nov-94 08:49:37 GMT", Format::HttpDate).unwrap();
    assert_eq!(dt.year(), 1994);
    let dt = DateTime::parse_with_format("Monday, 01-Jan-24 00:00:00 GMT", Format::HttpDate).unwrap();
    assert_eq!(dt.year(), 2024);
    let dt = DateTime::parse_with_format("Saturday, 01-Jan-00 00:00:00 GMT", Format::HttpDate).unwrap();
    assert_eq!(dt.year(), 2000);
    let dt = DateTime::parse_with_format("Monday, 01-Jan-50 00:00:00 GMT", Format::HttpDate).unwrap();
    assert_eq!(dt.year(), 1950);
}

#[test]
fn parse_rfc2822_dates() {
    let dt = DateTime::parse_with_format("Mon, 01 Jan 2024 12:34:56 -0500", Format::Rfc2822).unwrap();
    assert_eq!(dt, parse("2024-01-01T17:34:56Z"));
    // Weekday and seconds are optional; a missing zone means UTC.
    let dt = DateTime::parse_with_format("01 Jan 2024 12:34 +0000", Format::Rfc2822).unwrap();
    assert_eq!(dt, parse("2024-01-01T12:34:00Z"));
    let dt = DateTime::parse_with_format("Mon, 01 Jan 2024 12:34:56 GMT", Format::Rfc2822).unwrap();
    assert_eq!(dt, parse("2024-01-01T12:34:56Z"));
    assert!(DateTime::parse_with_format("not a date", Format::Rfc2822).is_err());
}

#[test]
fn parse_date_and_time_only() {
    assert_eq!(
        DateTime::parse_with_format("2024-01-01", Format::DateOnly).unwrap(),
        parse("2024-01-01T00:00:00Z")
    );
    assert!(DateTime::parse_with_format("2024-01-01T00:00:00Z", Format::DateOnly).is_err());
    // Time-only values are placed on the Unix epoch date.
    assert_eq!(
        DateTime::parse_with_format("12:34:56", Format::TimeOnly).unwrap(),
        parse("1970-01-01T12:34:56Z")
    );
    assert_eq!(
        DateTime::parse_with_format("12:34:56.5", Format::TimeOnly)
            .unwrap()
            .nanosecond(),
        500_000_000
    );
    assert!(DateTime::parse_with_format("12:34:56Z", Format::TimeOnly).is_err());
}

#[test]
fn parse_with_format_iso() {
    assert_eq!(
        DateTime::parse_with_format("2024-01-01T12:34:56+00:00", Format::Iso8601).unwrap(),
        parse("2024-01-01T12:34:56Z")
    );
    assert_eq!(
        DateTime::parse_with_format("2024-01-01T12:34:56Z", Format::Rfc3339).unwrap(),
        parse("2024-01-01T12:34:56Z")
    );
}

#[cfg(feature = "std")]
#[test]
fn system_time_roundtrip() {
    use std::time::SystemTime;

    for seconds in [0i64, 1, -1, 1_700_000_000, -1_000_000, -62_135_596_800] {
        let dt = DateTime::from_unix_seconds(seconds).unwrap();
        let system: SystemTime = dt.into();
        let back = DateTime::try_from(system).unwrap();
        assert_eq!(back, dt, "{seconds}");
    }

    let dt = DateTime::from_unix(1, 500_000_000).unwrap();
    let system: SystemTime = dt.into();
    assert_eq!(DateTime::try_from(system).unwrap(), dt);
}

// --- Phase A: monotonic clock, leap seconds, disambiguation, strict I/O ------

#[test]
fn leap_second_is_accepted_and_never_emitted() {
    let leap = DateTime::from_parts(2016, 12, 31, 23, 59, 60, 0, TimeZone::UTC).unwrap();
    let floor = DateTime::from_parts(2016, 12, 31, 23, 59, 59, 0, TimeZone::UTC).unwrap();
    assert_eq!(leap, floor);
    assert_eq!(leap.second(), 59);
    // Parsing agrees with `from_parts`, and `:60` is never produced.
    let parsed = parse("2016-12-31T23:59:60Z");
    assert_eq!(parsed, floor);
    assert_eq!(parsed.format(Format::Rfc3339).to_string(), "2016-12-31T23:59:59Z");
    assert!(!parsed.to_string().contains(":60"));
}

#[test]
fn signed_duration_since_is_normalized() {
    let later = parse("2024-01-01T00:00:00.250Z");
    let earlier = parse("2023-12-31T23:59:59.500Z");
    assert_eq!(later.signed_duration_since(&earlier), (0, 750_000_000));
    assert_eq!(earlier.signed_duration_since(&later), (-1, 250_000_000));
    assert_eq!(later.duration_since(&earlier).unwrap(), Duration::new(0, 750_000_000));
    assert_eq!(earlier.duration_until(&later).unwrap(), Duration::new(0, 750_000_000));
}

#[test]
fn strict_format_checks() {
    // A historical offset with a seconds component is not representable in a
    // conformant RFC 3339 / ISO 8601 / RFC 9557 / RFC 2822 string.
    let base = DateTime::from_unix_seconds(-2_200_000_000).unwrap();
    let dt = base.in_timezone(TimeZone::fixed(1172).unwrap());
    assert_eq!(dt.offset(), 1172);
    assert!(dt.format(Format::Rfc3339).to_string().contains("+00:19:32"));
    assert!(dt.try_format(Format::Rfc3339).is_err());
    assert!(dt.try_format(Format::Rfc9557).is_err());
    assert!(dt.try_format(Format::Iso8601).is_err());
    assert!(dt.try_format(Format::Rfc2822).is_err());
    assert!(dt.try_format(Format::HttpDate).is_ok());

    // A year outside `0000..=9999` (reachable at the range boundary with a
    // large offset) is not representable either.
    let max = DateTime::from_parts(9999, 12, 31, 23, 59, 59, 0, TimeZone::UTC).unwrap();
    let shifted = max.in_timezone(TimeZone::fixed(93_599).unwrap());
    assert_eq!(shifted.year(), 10000);
    assert!(shifted.try_format(Format::Rfc3339).is_err());
    assert!(shifted.try_format(Format::DateOnly).is_err());
}

#[test]
fn format_to_buffer() {
    let dt = parse("2024-07-11T01:14:00.5Z");
    let mut buffer = [0u8; 64];
    let written = dt.format_to(&mut buffer, Format::Rfc3339).unwrap();
    assert_eq!(&buffer[..written], b"2024-07-11T01:14:00.500Z");

    let mut small = [0u8; 8];
    let err = dt.format_to(&mut small, Format::Rfc3339).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BufferTooSmall);

    // `Formatted::write_to` is the adapter-level equivalent.
    let mut buffer = [0u8; 10];
    let written = dt.format(Format::DateOnly).write_to(&mut buffer).unwrap();
    assert_eq!(&buffer[..written], b"2024-07-11");
}

#[test]
fn strict_parse_with_format() {
    // The RFC 3339 family is strict under `parse_with_format`; the lenient
    // superset remains available through `str::parse`.
    assert!(DateTime::parse_with_format("2024-01-01T12:34:56Z", Format::Rfc3339).is_ok());
    assert!(DateTime::parse_with_format("2024-01-01T12:34:56+00:00", Format::Iso8601).is_ok());
    for bad in [
        "2024-01-01",
        "2024-01-01 12:34:56Z",
        "2024-01-01T12:34:56",
        "2024-01-01T12:34:56+00:00:30",
        "2024-01-01T12:34:56.5Z extra",
    ] {
        assert!(
            DateTime::parse_with_format(bad, Format::Rfc3339).is_err(),
            "expected {bad:?} to be rejected"
        );
    }
    // Lenient parsing still accepts these.
    assert!("2024-01-01".parse::<DateTime>().is_ok());
    assert!("2024-01-01 12:34:56Z".parse::<DateTime>().is_ok());
}

#[test]
fn parse_bytes_slice() {
    let dt = DateTime::parse_bytes(b"2024-01-01T00:00:00Z").unwrap();
    assert_eq!(dt.unix_seconds(), civil::days_from_civil(2024, 1, 1) * 86_400);
    assert!(DateTime::parse_bytes(&[0xff, 0xfe, 0xfd]).is_err());
}

#[cfg(feature = "std")]
#[test]
fn monotonic_measurement() {
    let start = DateTime::now();
    assert!(start.has_monotonic());
    let end = DateTime::now();
    let elapsed = end.duration_since(&start).unwrap();
    assert!(elapsed < Duration::from_secs(60));
    assert!(start.elapsed().is_ok());

    // Wall-clock values have no monotonic reading, and the reading is
    // preserved through instant-preserving operations but not calendar ones.
    let parsed = parse("2024-01-01T00:00:00Z");
    assert!(!parsed.has_monotonic());
    assert!(start.in_timezone(TimeZone::UTC).has_monotonic());
    let rounded = start.checked_add(Duration::from_secs(1)).unwrap();
    assert!(rounded.has_monotonic());
    let calendar = start.add_days(1).unwrap();
    assert!(!calendar.has_monotonic());
}

#[cfg(feature = "timezone-db")]
#[test]
fn disambiguation_strategies() {
    let zone = TimeZone::named("America/New_York").unwrap();

    // Fold: 2024-11-03 01:30 occurs twice (EDT then EST).
    assert_eq!(
        zone.ambiguity(2024, 11, 3, 1, 30, 0).unwrap(),
        Ambiguity::Fold {
            earlier: -4 * 3600,
            later: -5 * 3600,
        }
    );
    let earlier = DateTime::from_parts_with(2024, 11, 3, 1, 30, 0, 0, zone, Disambiguation::Earlier).unwrap();
    let later = DateTime::from_parts_with(2024, 11, 3, 1, 30, 0, 0, zone, Disambiguation::Later).unwrap();
    assert_eq!(earlier.offset(), -4 * 3600);
    assert_eq!(later.offset(), -5 * 3600);
    assert!(later > earlier);
    let rejected = DateTime::from_parts_with(2024, 11, 3, 1, 30, 0, 0, zone, Disambiguation::Reject);
    assert_eq!(rejected.unwrap_err().kind(), ErrorKind::Ambiguous);

    // Gap: 2024-03-10 02:30 does not exist; every strategy but `Reject`
    // shifts forward.
    assert_eq!(
        zone.ambiguity(2024, 3, 10, 2, 30, 0).unwrap(),
        Ambiguity::Gap {
            before: -5 * 3600,
            after: -4 * 3600,
        }
    );
    let shifted = DateTime::from_parts_with(2024, 3, 10, 2, 30, 0, 0, zone, Disambiguation::Compatible).unwrap();
    assert_eq!(shifted.hour(), 3);
    assert!(DateTime::from_parts_with(2024, 3, 10, 2, 30, 0, 0, zone, Disambiguation::Reject).is_err());

    // An unambiguous time is classified as such.
    assert_eq!(
        zone.ambiguity(2024, 6, 1, 12, 0, 0).unwrap(),
        Ambiguity::Exact {
            offset: -4 * 3600
        }
    );
}

#[cfg(feature = "timezone-db")]
#[test]
fn zone_abbreviation() {
    let zone = TimeZone::named("America/New_York").unwrap();
    let winter = parse("2024-01-15T12:00:00Z").in_timezone(zone);
    let summer = parse("2024-07-15T12:00:00Z").in_timezone(zone);
    assert_eq!(winter.abbreviation(), Some("EST"));
    assert_eq!(summer.abbreviation(), Some("EDT"));
    assert_eq!(parse("2024-01-15T12:00:00Z").abbreviation(), Some("UTC"));
    let fixed = parse("2024-01-15T12:00:00Z").in_timezone(TimeZone::fixed(3600).unwrap());
    assert_eq!(fixed.abbreviation(), None);
}

#[cfg(feature = "timezone-db")]
#[test]
fn zone_bounds() {
    let utc = parse("2024-01-15T12:00:00Z");
    assert_eq!(utc.zone_bounds(), (None, None));

    let zone = TimeZone::named("America/New_York").unwrap();

    // Between two explicit transitions both bounds are known.
    let historical = parse("2000-06-01T12:00:00Z").in_timezone(zone);
    let (start, end) = historical.zone_bounds();
    let start = start.expect("a start bound");
    let end = end.expect("an end bound");
    assert!(start < historical && historical < end);

    // The generator materializes the footer as explicit transitions up to a
    // rolling horizon (release year + 50), so dates inside it have both
    // bounds known.
    let modern = parse("2024-01-15T12:00:00Z").in_timezone(zone);
    let (start, end) = modern.zone_bounds();
    let start = start.expect("a start bound");
    let end = end.expect("an end bound");
    assert!(start < modern && modern < end);

    // Beyond the horizon the POSIX footer governs, so the end is unbounded.
    let far = parse("2090-01-15T12:00:00Z").in_timezone(zone);
    let (start, end) = far.zone_bounds();
    assert!(start.is_some());
    assert!(end.is_none());
}

#[test]
fn strict_rfc3339_corpus() {
    for good in [
        "2024-01-01T12:34:56Z",
        "2024-01-01t12:34:56z",
        "2024-01-01T12:34:56.5Z",
        "2024-01-01T12:34:56+00:00",
        "2024-01-01T12:34:56-08:00",
        "2024-12-31T23:59:60Z",
    ] {
        assert!(
            DateTime::parse_with_format(good, Format::Rfc3339).is_ok(),
            "expected {good:?} to be accepted"
        );
    }
    for bad in [
        "2024-01-01 12:34:56Z",
        "2024-01-01T12:34:56",
        "2024-01-01T12:34+00:00",
        "2024-01-01",
        "2024-01-01T12:34:56+0000",
        "2024-01-01T12:34:56+00:00:30",
        "+2024-01-01T12:34:56Z",
        "2024-01-01T12:34:56.5",
    ] {
        assert!(
            DateTime::parse_with_format(bad, Format::Rfc3339).is_err(),
            "expected {bad:?} to be rejected"
        );
    }
}

#[test]
fn format_to_exact_buffer() {
    let formats = [
        Format::Rfc3339,
        Format::Rfc9557,
        Format::Iso8601,
        Format::Rfc2822,
        Format::HttpDate,
        Format::DateOnly,
        Format::TimeOnly,
    ];
    let dt = parse("2024-07-11T01:14:00.5Z");
    for format in formats {
        let mut big = [0u8; 128];
        let len = dt.format_to(&mut big, format).unwrap();
        let mut exact = vec![0u8; len];
        assert_eq!(dt.format_to(&mut exact, format).unwrap(), len);
        assert_eq!(&exact, &big[..len]);
        if len > 0 {
            let mut small = vec![0u8; len - 1];
            assert_eq!(dt.format_to(&mut small, format).unwrap_err().kind(), ErrorKind::BufferTooSmall);
        }
    }
}

#[cfg(feature = "timezone-db")]
#[test]
fn disambiguation_matches_classification() {
    // Walk every explicit transition of a few zones with unusual DST shapes
    // (30- and 45-minute shifts, negative DST) and check that the classifier
    // agrees with every disambiguation strategy around each transition.
    for name in [
        "America/New_York",
        "Australia/Lord_Howe",
        "Pacific/Chatham",
        "Europe/Dublin",
    ] {
        let zone = TimeZone::named(name).unwrap();
        let limit = parse("2026-01-01T00:00:00Z").unix_seconds();
        let mut cursor = parse("2024-01-01T00:00:00Z").in_timezone(zone);
        let mut folds = 0usize;
        let mut gaps = 0usize;
        while let (_, Some(end)) = cursor.zone_bounds() {
            if end.unix_seconds() > limit {
                break;
            }
            let instant = end.unix_seconds();
            let before = zone.offset_at(instant - 1);
            let after = zone.offset_at(instant);
            if before != after {
                let low = instant + i64::from(before.min(after));
                let high = instant + i64::from(before.max(after));
                for local in [low - 1, low, low + 1, (low + high) / 2, high - 1, high] {
                    let (year, month, day, hour, minute, second, _) =
                        DateTime::from_unix_seconds(local).unwrap().parts();
                    let ambiguity = zone.ambiguity(year, month, day, hour, minute, second).unwrap();
                    let resolve =
                        |strategy| DateTime::from_parts_with(year, month, day, hour, minute, second, 0, zone, strategy);
                    let compatible = resolve(Disambiguation::Compatible).unwrap();
                    let earlier = resolve(Disambiguation::Earlier);
                    let later = resolve(Disambiguation::Later);
                    let reject = resolve(Disambiguation::Reject);
                    let label = format!("{name} {year}-{month}-{day} {hour}:{minute}:{second}");
                    match ambiguity {
                        Ambiguity::Exact {
                            offset,
                        } => {
                            assert_eq!(compatible.offset(), offset, "{label}");
                            assert!(earlier.is_ok() && later.is_ok() && reject.is_ok(), "{label}");
                        }
                        Ambiguity::Fold {
                            earlier: first,
                            later: second_offset,
                        } => {
                            assert_eq!(compatible.offset(), first, "{label}");
                            assert_eq!(earlier.unwrap().offset(), first, "{label}");
                            assert_eq!(later.unwrap().offset(), second_offset, "{label}");
                            assert!(reject.is_err(), "{label}");
                            folds += 1;
                        }
                        Ambiguity::Gap {
                            after, ..
                        } => {
                            assert_eq!(compatible.offset(), after, "{label}");
                            assert_eq!(earlier.unwrap(), compatible, "{label}");
                            assert_eq!(later.unwrap(), compatible, "{label}");
                            assert!(reject.is_err(), "{label}");
                            gaps += 1;
                        }
                    }
                }
            }
            cursor = end;
        }
        assert!(folds + gaps > 0, "{name} produced no ambiguous civil time");
    }
}
