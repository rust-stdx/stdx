//! Strict parsing of datetime strings.

use crate::{
    civil,
    datetime::{DateTime, MAX_YEAR, MIN_YEAR},
    error::{self, Error},
    format::Format,
    timezone::TimeZone,
};

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser {
            bytes: input.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        Some(byte)
    }

    fn eat(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, byte: u8) -> Result<(), Error> {
        if self.eat(byte) {
            Ok(())
        } else {
            Err(error::parse("unexpected character"))
        }
    }

    fn fixed(&mut self, count: usize) -> Result<i64, Error> {
        let mut value = 0i64;
        for _ in 0..count {
            match self.bump() {
                Some(byte) if byte.is_ascii_digit() => {
                    value = value * 10 + i64::from(byte - b'0');
                }
                _ => return Err(error::parse("expected a digit")),
            }
        }
        Ok(value)
    }

    fn number(&self, start: usize, end: usize) -> Result<i64, Error> {
        let mut value = 0i64;
        for &byte in &self.bytes[start..end] {
            if !byte.is_ascii_digit() {
                return Err(error::parse("expected a digit"));
            }
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(i64::from(byte - b'0')))
                .ok_or_else(|| error::parse("number is too large"))?;
        }
        Ok(value)
    }

    fn done(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

/// Parses `input` using a specific [`Format`].
pub(crate) fn parse_with_format(input: &str, format: Format) -> Result<DateTime, Error> {
    match format {
        Format::Rfc3339 | Format::Iso8601 => parse_rfc3339_strict(input),
        Format::Rfc9557 => parse_rfc9557_strict(input),
        Format::DateOnly => parse_date_only(input),
        Format::TimeOnly => parse_time_only(input),
        Format::Rfc2822 => parse_rfc2822(input),
        Format::HttpDate => parse_http(input),
    }
}

/// Parses `input` as raw bytes, using `default_zone` for zone-less input.
pub(crate) fn parse_bytes(input: &[u8], default_zone: Option<TimeZone>) -> Result<DateTime, Error> {
    match core::str::from_utf8(input) {
        Ok(text) => parse(text, default_zone),
        Err(_) => Err(error::parse("input is not valid UTF-8")),
    }
}

/// Parses `input` as raw bytes using a specific [`Format`].
pub(crate) fn parse_bytes_with_format(input: &[u8], format: Format) -> Result<DateTime, Error> {
    match core::str::from_utf8(input) {
        Ok(text) => parse_with_format(text, format),
        Err(_) => Err(error::parse("input is not valid UTF-8")),
    }
}

/// Strictly parses an RFC 3339 date-time.
fn parse_rfc3339_strict(input: &str) -> Result<DateTime, Error> {
    if !rfc3339_shape(input) {
        return Err(error::parse("input is not an RFC 3339 date-time"));
    }
    parse(input, None)
}

/// Strictly parses RFC 9557: an RFC 3339 date-time plus at most one trailing
/// `[...]` time zone annotation.
fn parse_rfc9557_strict(input: &str) -> Result<DateTime, Error> {
    let datetime = match input.rfind('[') {
        Some(index) if input.ends_with(']') => &input[..index],
        _ if !input.contains('[') => input,
        _ => return Err(error::parse("malformed RFC 9557 time zone annotation")),
    };
    if !rfc3339_shape(datetime) {
        return Err(error::parse("input is not an RFC 9557 date-time"));
    }
    // The lenient parser validates and resolves the annotation itself.
    parse(input, None)
}

/// Returns `true` if `input` matches the strict RFC 3339 date-time grammar:
/// a four-digit year, mandatory seconds, and an offset of `Z` or `±HH:MM`
/// (no seconds), with `T`/`t` as the separator and `Z`/`z` accepted.
fn rfc3339_shape(input: &str) -> bool {
    let bytes = input.as_bytes();
    let digits = |range: core::ops::Range<usize>| {
        range
            .clone()
            .all(|index| bytes.get(index).is_some_and(u8::is_ascii_digit))
    };
    if bytes.len() < 20
        || !digits(0..4)
        || bytes[4] != b'-'
        || !digits(5..7)
        || bytes[7] != b'-'
        || !digits(8..10)
        || !matches!(bytes[10], b'T' | b't')
        || !digits(11..13)
        || bytes[13] != b':'
        || !digits(14..16)
        || bytes[16] != b':'
        || !digits(17..19)
    {
        return false;
    }
    let mut index = 19;
    if bytes[index] == b'.' {
        index += 1;
        let start = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        if index == start {
            return false;
        }
    }
    match bytes.get(index) {
        Some(b'Z' | b'z') => index + 1 == bytes.len(),
        Some(b'+' | b'-') => {
            index + 6 == bytes.len()
                && digits(index + 1..index + 3)
                && bytes[index + 3] == b':'
                && digits(index + 4..index + 6)
        }
        _ => false,
    }
}

/// Parses `input`, using `default_zone` for zone-less input.
pub(crate) fn parse(input: &str, default_zone: Option<TimeZone>) -> Result<DateTime, Error> {
    let mut parser = Parser::new(input);
    let (year, month, day) = parse_year_month_day(&mut parser)?;

    // Date-only input means midnight.
    if parser.done() {
        return assemble(year, month, day, 0, 0, 0, 0, None, None, default_zone);
    }

    // Date/time separator: `T`, `t` or a space.
    match parser.bump() {
        Some(b'T' | b't' | b' ') => {}
        _ => return Err(error::parse("expected a date/time separator")),
    }

    let (hour, minute, second, nanos) = parse_clock(&mut parser)?;
    let offset = parse_offset(&mut parser)?;
    let zone = parse_zone_annotation(&mut parser)?;

    if !parser.done() {
        return Err(error::parse("trailing characters"));
    }

    assemble(year, month, day, hour, minute, second, nanos, zone, offset, default_zone)
}

fn parse_year_month_day(parser: &mut Parser<'_>) -> Result<(i32, u8, u8), Error> {
    // Year: an optional sign followed by at least four digits.
    let negative_year = parser.eat(b'-');
    parser.eat(b'+');
    let year_start = parser.pos;
    let mut digits = 0usize;
    while parser.peek().is_some_and(|b| b.is_ascii_digit()) {
        parser.bump();
        digits += 1;
    }
    if digits < 4 {
        return Err(error::parse("expected a four digit year"));
    }
    let mut year = parser.number(year_start, parser.pos)?;
    if negative_year {
        year = -year;
    }
    if year < i64::from(MIN_YEAR) || year > i64::from(MAX_YEAR) {
        return Err(error::out_of_range("year is outside the supported range"));
    }
    let year = year as i32;

    parser.expect(b'-')?;
    let month = parser.fixed(2)? as u8;
    parser.expect(b'-')?;
    let day = parser.fixed(2)? as u8;
    Ok((year, month, day))
}

fn parse_clock(parser: &mut Parser<'_>) -> Result<(u8, u8, u8, u32), Error> {
    let hour = parser.fixed(2)? as u8;
    parser.expect(b':')?;
    let minute = parser.fixed(2)? as u8;
    let second = if parser.eat(b':') { parser.fixed(2)? as u8 } else { 0 };

    let mut nanos = 0u32;
    if parser.peek().is_some_and(|b| b == b'.' || b == b',') {
        parser.bump();
        let start = parser.pos;
        let mut significant = 0usize;
        let mut scale = 100_000_000u32;
        while parser.peek().is_some_and(|b| b.is_ascii_digit()) {
            let digit = (parser.bump().unwrap() - b'0') as u32;
            if significant < 9 {
                nanos += digit * scale;
                scale /= 10;
                significant += 1;
            }
        }
        if parser.pos == start {
            return Err(error::parse("expected fractional digits"));
        }
    }
    Ok((hour, minute, second, nanos))
}

fn parse_offset(parser: &mut Parser<'_>) -> Result<Option<i32>, Error> {
    match parser.peek() {
        Some(b'Z' | b'z') => {
            parser.bump();
            Ok(Some(0))
        }
        Some(b'+' | b'-') => {
            let negative = parser.bump() == Some(b'-');
            let hours = parser.fixed(2)? as i32;
            let mut minutes = 0i32;
            let mut seconds = 0i32;
            if parser.eat(b':') {
                minutes = parser.fixed(2)? as i32;
                if parser.eat(b':') {
                    seconds = parser.fixed(2)? as i32;
                }
            } else if parser.peek().is_some_and(|b| b.is_ascii_digit()) {
                minutes = parser.fixed(2)? as i32;
                if parser.peek().is_some_and(|b| b.is_ascii_digit()) {
                    seconds = parser.fixed(2)? as i32;
                }
            }
            let magnitude = hours * 3600 + minutes * 60 + seconds;
            if hours > 23 || minutes > 59 || seconds > 59 {
                return Err(error::parse("offset is out of range"));
            }
            let offset = if negative { -magnitude } else { magnitude };
            TimeZone::fixed(offset)?;
            Ok(Some(offset))
        }
        _ => Ok(None),
    }
}

fn parse_zone_annotation(parser: &mut Parser<'_>) -> Result<Option<TimeZone>, Error> {
    if parser.peek() != Some(b'[') {
        return Ok(None);
    }
    parser.bump();
    let start = parser.pos;
    while parser.peek().is_some_and(|b| b != b']') {
        parser.bump();
    }
    if !parser.eat(b']') {
        return Err(error::parse("unterminated time zone annotation"));
    }
    let name = core::str::from_utf8(&parser.bytes[start..parser.pos - 1])
        .map_err(|_| error::parse("invalid time zone name"))?;
    Ok(Some(TimeZone::named(name)?))
}

#[allow(clippy::too_many_arguments)]
fn assemble(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    nanos: u32,
    zone: Option<TimeZone>,
    offset: Option<i32>,
    default_zone: Option<TimeZone>,
) -> Result<DateTime, Error> {
    if !(1..=12).contains(&month) {
        return Err(error::invalid("month is not in 1..=12"));
    }
    if day < 1 || day > civil::days_in_month(year, month) {
        return Err(error::invalid("day is not valid for the given month"));
    }
    if hour > 23 {
        return Err(error::invalid("hour is not in 0..=23"));
    }
    if minute > 59 {
        return Err(error::invalid("minute is not in 0..=59"));
    }
    // Leap seconds are not representable; constrain `:60` to `:59`.
    let second = if second == 60 { 59 } else { second };
    if second > 59 {
        return Err(error::invalid("second is not in 0..=60"));
    }
    if nanos >= civil::NANOS_PER_SEC {
        return Err(error::invalid("fractional seconds are out of range"));
    }

    let local = civil::days_from_civil(year, month, day) * civil::SECS_PER_DAY
        + i64::from(hour) * 3600
        + i64::from(minute) * 60
        + i64::from(second);

    let (zone, instant) = match (zone, offset) {
        (Some(zone), Some(offset)) => {
            let instant = local - i64::from(offset);
            if zone.offset_at(instant) != offset {
                return Err(error::offset_conflict("offset does not match the time zone"));
            }
            (zone, instant)
        }
        (Some(zone), None) => (zone, zone.resolve_compatible(local)),
        (None, Some(offset)) => {
            let zone = if offset == 0 {
                TimeZone::UTC
            } else {
                TimeZone::fixed(offset)?
            };
            (zone, local - i64::from(offset))
        }
        (None, None) => {
            let zone = default_zone.unwrap_or(TimeZone::UTC);
            let instant = zone.resolve_compatible(local);
            (zone, instant)
        }
    };

    DateTime::from_raw_checked(instant, nanos, zone)
}

fn parse_date_only(input: &str) -> Result<DateTime, Error> {
    let mut parser = Parser::new(input);
    let (year, month, day) = parse_year_month_day(&mut parser)?;
    if !parser.done() {
        return Err(error::parse("trailing characters"));
    }
    assemble(year, month, day, 0, 0, 0, 0, None, None, None)
}

/// Parses a time of day, placing it on the Unix epoch date (`1970-01-01`).
fn parse_time_only(input: &str) -> Result<DateTime, Error> {
    let mut parser = Parser::new(input);
    let (hour, minute, second, nanos) = parse_clock(&mut parser)?;
    if !parser.done() {
        return Err(error::parse("trailing characters"));
    }
    assemble(1970, 1, 1, hour, minute, second, nanos, None, None, None)
}

// --- RFC 2822 -----------------------------------------------------------------

fn parse_rfc2822(input: &str) -> Result<DateTime, Error> {
    let mut tokens = input.split_ascii_whitespace();
    let first = tokens.next().ok_or_else(|| error::parse("empty RFC 2822 date"))?;

    let day_token = if let Some(weekday) = first.strip_suffix(',') {
        validate_weekday_long(weekday)?;
        tokens.next().ok_or_else(|| error::parse("missing day"))?
    } else {
        first
    };

    let day = parse_decimal(day_token, 1, 2)? as u8;
    let month = month_from_abbrev(tokens.next().ok_or_else(|| error::parse("missing month"))?)?;
    let year_token = tokens.next().ok_or_else(|| error::parse("missing year"))?;
    let year = parse_rfc2822_year(year_token)?;
    let time = tokens.next().ok_or_else(|| error::parse("missing time"))?;
    let (hour, minute, second) = parse_hms(time)?;
    let offset = match tokens.next() {
        None => 0,
        Some(zone) => parse_rfc2822_zone(zone)?,
    };
    if tokens.next().is_some() {
        return Err(error::parse("trailing characters"));
    }

    assemble(year, month, day, hour, minute, second, 0, None, Some(offset), None)
}

fn parse_rfc2822_year(token: &str) -> Result<i32, Error> {
    let value = parse_decimal(token, 2, 4)?;
    if token.len() == 2 {
        Ok(two_digit_year(value as u16))
    } else {
        Ok(value)
    }
}

fn parse_rfc2822_zone(token: &str) -> Result<i32, Error> {
    if token.eq_ignore_ascii_case("gmt")
        || token.eq_ignore_ascii_case("ut")
        || token.eq_ignore_ascii_case("utc")
        || token.eq_ignore_ascii_case("z")
    {
        return Ok(0);
    }
    let bytes = token.as_bytes();
    if bytes.len() != 5 || !matches!(bytes[0], b'+' | b'-') {
        return Err(error::parse("invalid RFC 2822 time zone"));
    }
    let hours = two_digits(&bytes[1..3])? as i32;
    let minutes = two_digits(&bytes[3..5])? as i32;
    if hours > 23 || minutes > 59 {
        return Err(error::parse("RFC 2822 time zone is out of range"));
    }
    let magnitude = hours * 3600 + minutes * 60;
    Ok(if bytes[0] == b'-' { -magnitude } else { magnitude })
}

fn parse_hms(token: &str) -> Result<(u8, u8, u8), Error> {
    let mut parts = token.split(':');
    let hour = parse_decimal(parts.next().ok_or_else(|| error::parse("missing hour"))?, 1, 2)? as u8;
    let minute = parse_decimal(parts.next().ok_or_else(|| error::parse("missing minute"))?, 2, 2)? as u8;
    let second = match parts.next() {
        Some(part) => parse_decimal(part, 2, 2)? as u8,
        None => 0,
    };
    if parts.next().is_some() {
        return Err(error::parse("invalid time"));
    }
    Ok((hour, minute, second))
}

fn parse_decimal(token: &str, min: usize, max: usize) -> Result<i32, Error> {
    let bytes = token.as_bytes();
    if bytes.len() < min || bytes.len() > max {
        return Err(error::parse("expected a decimal number"));
    }
    let mut value = 0i32;
    for &byte in bytes {
        let digit = byte.wrapping_sub(b'0');
        if digit >= 10 {
            return Err(error::parse("expected a decimal number"));
        }
        value = value * 10 + i32::from(digit);
    }
    Ok(value)
}

// --- HTTP dates ---------------------------------------------------------------

fn parse_http(input: &str) -> Result<DateTime, Error> {
    let input = input.trim();
    if !input.is_ascii() {
        return Err(error::parse("date is not ASCII"));
    }
    if let Ok(datetime) = parse_imf_fixdate(input) {
        return Ok(datetime);
    }
    if let Ok(datetime) = parse_rfc850_date(input) {
        return Ok(datetime);
    }
    if let Ok(datetime) = parse_asctime(input) {
        return Ok(datetime);
    }
    Err(error::parse("input is not an IMF-fixdate (RFC 9110), RFC 850 or asctime date"))
}

fn parse_imf_fixdate(s: &str) -> Result<DateTime, Error> {
    // Example: `Sun, 06 Nov 1994 08:49:37 GMT`
    let b = s.as_bytes();
    if b.len() != 29
        || b[3] != b','
        || b[4] != b' '
        || b[7] != b' '
        || b[11] != b' '
        || b[16] != b' '
        || b[19] != b':'
        || b[22] != b':'
        || &b[25..29] != b" GMT"
    {
        return Err(error::parse("invalid IMF-fixdate"));
    }
    validate_weekday_short(ascii_str(&b[0..3])?)?;
    let day = two_digits(&b[5..7])?;
    let month = month_from_abbrev(ascii_str(&b[8..11])?)?;
    let year = four_digits(&b[12..16])?;
    let hour = two_digits(&b[17..19])?;
    let minute = two_digits(&b[20..22])?;
    let second = two_digits(&b[23..25])?;
    assemble(year, month, day, hour, minute, second, 0, None, Some(0), None)
}

fn parse_rfc850_date(s: &str) -> Result<DateTime, Error> {
    // Example: `Sunday, 06-Nov-94 08:49:37 GMT`
    let rest = strip_weekday_long(s)?;
    let b = rest.as_bytes();
    if b.len() != 22
        || b[2] != b'-'
        || b[6] != b'-'
        || b[9] != b' '
        || b[12] != b':'
        || b[15] != b':'
        || &b[18..22] != b" GMT"
    {
        return Err(error::parse("invalid RFC 850 date"));
    }
    let day = two_digits(&b[0..2])?;
    let month = month_from_abbrev(ascii_str(&b[3..6])?)?;
    let year = two_digit_year(two_digits(&b[7..9])? as u16);
    let hour = two_digits(&b[10..12])?;
    let minute = two_digits(&b[13..15])?;
    let second = two_digits(&b[16..18])?;
    assemble(year, month, day, hour, minute, second, 0, None, Some(0), None)
}

fn parse_asctime(s: &str) -> Result<DateTime, Error> {
    // Example: `Sun Nov  6 08:49:37 1994`
    let b = s.as_bytes();
    if b.len() != 24 || b[3] != b' ' || b[7] != b' ' || b[10] != b' ' || b[13] != b':' || b[16] != b':' || b[19] != b' '
    {
        return Err(error::parse("invalid asctime date"));
    }
    validate_weekday_short(ascii_str(&b[0..3])?)?;
    let month = month_from_abbrev(ascii_str(&b[4..7])?)?;
    let day = {
        let field = &b[8..10];
        if field[0] == b' ' {
            one_digit(field[1])?
        } else {
            two_digits(field)?
        }
    };
    let hour = two_digits(&b[11..13])?;
    let minute = two_digits(&b[14..16])?;
    let second = two_digits(&b[17..19])?;
    let year = four_digits(&b[20..24])?;
    assemble(year, month, day, hour, minute, second, 0, None, Some(0), None)
}

// --- small helpers ------------------------------------------------------------

fn ascii_str(bytes: &[u8]) -> Result<&str, Error> {
    core::str::from_utf8(bytes).map_err(|_| error::parse("expected ASCII"))
}

fn one_digit(byte: u8) -> Result<u8, Error> {
    let value = byte.wrapping_sub(b'0');
    if value < 10 {
        Ok(value)
    } else {
        Err(error::parse("expected a digit"))
    }
}

fn two_digits(bytes: &[u8]) -> Result<u8, Error> {
    if bytes.len() != 2 {
        return Err(error::parse("expected two digits"));
    }
    Ok(one_digit(bytes[0])? * 10 + one_digit(bytes[1])?)
}

fn four_digits(bytes: &[u8]) -> Result<i32, Error> {
    if bytes.len() != 4 {
        return Err(error::parse("expected four digits"));
    }
    let mut value = 0i32;
    for &byte in bytes {
        value = value * 10 + i32::from(one_digit(byte)?);
    }
    Ok(value)
}

/// Maps a two-digit year using the RFC 2822 / HTTP convention: `00..49` become
/// `2000..2049` and `50..99` become `1950..1999`.
fn two_digit_year(year: u16) -> i32 {
    if year < 50 {
        2000 + i32::from(year)
    } else {
        1900 + i32::from(year)
    }
}

fn month_from_abbrev(name: &str) -> Result<u8, Error> {
    let month = match name {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return Err(error::parse("invalid month name")),
    };
    Ok(month)
}

fn validate_weekday_short(name: &str) -> Result<(), Error> {
    match name {
        "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun" => Ok(()),
        _ => Err(error::parse("invalid weekday name")),
    }
}

fn validate_weekday_long(name: &str) -> Result<(), Error> {
    match name {
        "Mon" | "Monday" | "Tue" | "Tuesday" | "Wed" | "Wednesday" | "Thu" | "Thursday" | "Fri" | "Friday" | "Sat"
        | "Saturday" | "Sun" | "Sunday" => Ok(()),
        _ => Err(error::parse("invalid weekday name")),
    }
}

fn strip_weekday_long(s: &str) -> Result<&str, Error> {
    for name in [
        "Monday, ",
        "Tuesday, ",
        "Wednesday, ",
        "Thursday, ",
        "Friday, ",
        "Saturday, ",
        "Sunday, ",
    ] {
        if let Some(rest) = s.strip_prefix(name) {
            return Ok(rest);
        }
    }
    Err(error::parse("invalid weekday name"))
}
