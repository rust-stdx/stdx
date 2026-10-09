//! A signed duration type for the expression language.
//!
//! The standard [`core::time::Duration`] is unsigned, but BEL's duration
//! arithmetic (subtraction, negation) needs signed values. This type stores a
//! signed count of nanoseconds in an `i128`, which is far larger than any
//! duration the language needs.

use core::ops::{Add, Mul, Neg, Sub};

use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::char, combinator::opt, multi::many1,
    number::complete::double,
};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

/// A signed duration with nanosecond precision.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Duration {
    nanos: i128,
}

impl Duration {
    /// The zero duration.
    pub const ZERO: Duration = Duration {
        nanos: 0,
    };

    /// Creates a duration from a signed number of nanoseconds.
    #[must_use]
    pub const fn from_nanos(nanos: i128) -> Duration {
        Duration {
            nanos,
        }
    }

    /// Creates a duration from a signed number of nanoseconds.
    #[must_use]
    pub const fn nanoseconds(nanos: i64) -> Duration {
        Duration {
            nanos: nanos as i128,
        }
    }

    /// Creates a duration from a signed number of microseconds.
    #[must_use]
    pub const fn microseconds(micros: i64) -> Duration {
        Duration {
            nanos: micros as i128 * 1_000,
        }
    }

    /// Creates a duration from a signed number of milliseconds.
    #[must_use]
    pub const fn milliseconds(millis: i64) -> Duration {
        Duration {
            nanos: millis as i128 * 1_000_000,
        }
    }

    /// Creates a duration from a signed number of seconds.
    #[must_use]
    pub const fn seconds(seconds: i64) -> Duration {
        Duration {
            nanos: seconds as i128 * NANOS_PER_SECOND,
        }
    }

    /// Creates a duration from a signed number of minutes.
    #[must_use]
    pub const fn minutes(minutes: i64) -> Duration {
        Duration {
            nanos: minutes as i128 * 60 * NANOS_PER_SECOND,
        }
    }

    /// Creates a duration from a signed number of hours.
    #[must_use]
    pub const fn hours(hours: i64) -> Duration {
        Duration {
            nanos: hours as i128 * 3600 * NANOS_PER_SECOND,
        }
    }

    /// Returns the total number of nanoseconds.
    #[must_use]
    pub const fn as_nanos(self) -> i128 {
        self.nanos
    }

    /// Returns the total number of nanoseconds, or `None` if it does not fit in
    /// an `i64`.
    #[must_use]
    pub const fn num_nanoseconds(self) -> Option<i64> {
        if self.nanos >= i64::MIN as i128 && self.nanos <= i64::MAX as i128 {
            Some(self.nanos as i64)
        } else {
            None
        }
    }

    /// Returns the number of whole seconds, truncated toward zero.
    #[must_use]
    pub const fn num_seconds(self) -> i64 {
        (self.nanos / NANOS_PER_SECOND) as i64
    }

    /// Returns `true` if this duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.nanos == 0
    }

    /// Returns `true` if this duration is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.nanos < 0
    }

    /// Adds two durations, returning `None` on overflow.
    #[must_use]
    pub const fn checked_add(self, other: Duration) -> Option<Duration> {
        match self.nanos.checked_add(other.nanos) {
            Some(nanos) => Some(Duration {
                nanos,
            }),
            None => None,
        }
    }

    /// Subtracts two durations, returning `None` on overflow.
    #[must_use]
    pub const fn checked_sub(self, other: Duration) -> Option<Duration> {
        match self.nanos.checked_sub(other.nanos) {
            Some(nanos) => Some(Duration {
                nanos,
            }),
            None => None,
        }
    }

    /// Returns the whole seconds component used for serialization.
    pub(crate) const fn serialized_secs(self) -> i64 {
        (self.nanos / NANOS_PER_SECOND) as i64
    }

    /// Returns the sub-second nanosecond component used for serialization.
    pub(crate) const fn serialized_nanos(self) -> i32 {
        (self.nanos % NANOS_PER_SECOND) as i32
    }

    /// Rebuilds a duration from the serialized `(secs, nanos)` pair.
    pub(crate) const fn from_secs_nanos(secs: i64, nanos: i32) -> Duration {
        Duration {
            nanos: secs as i128 * NANOS_PER_SECOND + nanos as i128,
        }
    }
}

impl core::fmt::Display for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format_duration(self))
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        Duration {
            nanos: self.nanos + rhs.nanos,
        }
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        Duration {
            nanos: self.nanos - rhs.nanos,
        }
    }
}

impl Mul<i32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: i32) -> Duration {
        Duration {
            nanos: self.nanos * rhs as i128,
        }
    }
}

impl Neg for Duration {
    type Output = Duration;

    fn neg(self) -> Duration {
        Duration {
            nanos: -self.nanos,
        }
    }
}

/// Parses a duration string into a [`Duration`]. Duration strings support the
/// following grammar:
///
/// DurationString -> Sign? Number Unit String?
/// Sign           -> '-'
/// Number         -> Digit+ ('.' Digit+)?
/// Digit          -> '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
/// Unit           -> 'h' | 'm' | 's' | 'ms' | 'us' | 'ns'
/// String         -> DurationString
///
/// # Examples
/// - `1h` parses as 1 hour
/// - `1.5h` parses as 1 hour and 30 minutes
/// - `1h30m` parses as 1 hour and 30 minutes
/// - `1h30m1s` parses as 1 hour, 30 minutes, and 1 second
/// - `1ms` parses as 1 millisecond
/// - `1.5ms` parses as 1 millisecond and 500 microseconds
/// - `1ns` parses as 1 nanosecond
/// - `1.5ns` parses as 1 nanosecond (sub-nanosecond durations not supported)
pub fn parse_duration(i: &str) -> IResult<&str, Duration> {
    let (i, neg) = opt(parse_negative).parse(i)?;
    if i == "0" {
        return Ok((i, Duration::ZERO));
    }
    let (i, duration) = many1(parse_number_unit)
        .parse(i)
        .map(|(i, d)| (i, d.iter().fold(Duration::ZERO, |acc, next| acc + *next)))?;
    Ok((i, if neg.is_some() { -duration } else { duration }))
}

enum Unit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
}

impl Unit {
    fn nanos(&self) -> i64 {
        match self {
            Unit::Nanosecond => 1,
            Unit::Microsecond => 1_000,
            Unit::Millisecond => 1_000_000,
            Unit::Second => 1_000_000_000,
            Unit::Minute => 60 * 1_000_000_000,
            Unit::Hour => 60 * 60 * 1_000_000_000,
        }
    }
}

fn parse_number_unit(i: &str) -> IResult<&str, Duration> {
    let (i, num) = double(i)?;
    let (i, unit) = parse_unit(i)?;
    let duration = to_duration(num, unit);
    Ok((i, duration))
}

fn parse_negative(i: &str) -> IResult<&str, ()> {
    let (i, _): (&str, char) = char('-').parse(i)?;
    Ok((i, ()))
}

fn parse_unit(i: &str) -> IResult<&str, Unit> {
    alt((
        tag("ms").map(|_| Unit::Millisecond),
        tag("us").map(|_| Unit::Microsecond),
        tag("ns").map(|_| Unit::Nanosecond),
        char('h').map(|_| Unit::Hour),
        char('m').map(|_| Unit::Minute),
        char('s').map(|_| Unit::Second),
    ))
    .parse(i)
}

fn to_duration(num: f64, unit: Unit) -> Duration {
    Duration::from_nanos((num * unit.nanos() as f64).trunc() as i128)
}

/// Formats a [`Duration`] into a string. The returned string represents the
/// duration in the form "72h3m0.5s". Leading zero units are omitted. As a
/// special case, durations less than one second format use a smaller unit
/// (milli-, micro-, or nanoseconds) to ensure that the leading digit is
/// non-zero. The zero duration formats as `0s`.
///
/// This is a direct port of the Go version of the `time.Duration.String()`
/// function.
pub fn format_duration(d: &Duration) -> String {
    let buf = &mut [0u8; 32];
    let mut w = buf.len();

    let mut neg = false;
    let mut u = d
        .num_nanoseconds()
        .map(|n| {
            if n < 0 {
                neg = true;
            }
            n as u64
        })
        .unwrap_or_else(|| {
            let s = d.num_seconds();
            if s < 0 {
                neg = true;
            }
            s as u64 * SECOND
        });

    if u < SECOND {
        // Special case: if duration is smaller than a second,
        // use smaller units, like 1.2ms
        let mut _prec = 0;
        w -= 1;
        buf[w] = b's';
        w -= 1;

        if u == 0 {
            return "0s".to_string();
        } else if u < MICROSECOND {
            _prec = 0;
            buf[w] = b'n';
        } else if u < MILLISECOND {
            _prec = 3;
            // U+00B5 'µ' micro sign == 0xC2 0xB5
            buf[w] = 0xB5;
            w -= 1;
            buf[w] = 0xC2;
        } else {
            _prec = 6;
            buf[w] = b'm';
        }
        (w, u) = format_float(&mut buf[..w], u, _prec);
        w = format_int(&mut buf[..w], u);
    } else {
        w -= 1;
        buf[w] = b's';
        (w, u) = format_float(&mut buf[..w], u, 9);

        // u is now integer number of seconds
        w = format_int(&mut buf[..w], u % 60);
        u /= 60;

        // u is now integer number of minutes
        if u > 0 {
            w -= 1;
            buf[w] = b'm';
            w = format_int(&mut buf[..w], u % 60);
            u /= 60;

            // u is now integer number of hours
            if u > 0 {
                w -= 1;
                buf[w] = b'h';
                w = format_int(&mut buf[..w], u);
            }
        }
    }

    if neg {
        w -= 1;
        buf[w] = b'-';
    }
    String::from_utf8_lossy(&buf[w..]).into_owned()
}

const SECOND: u64 = 1_000_000_000;
const MILLISECOND: u64 = 1_000_000;
const MICROSECOND: u64 = 1_000;

fn format_float(buf: &mut [u8], mut v: u64, prec: usize) -> (usize, u64) {
    let mut w = buf.len();
    let mut print = false;
    for _ in 0..prec {
        let digit = v % 10;
        print = print || digit != 0;
        if print {
            w -= 1;
            buf[w] = digit as u8 + b'0';
        }
        v /= 10;
    }
    if print {
        w -= 1;
        buf[w] = b'.';
    }
    (w, v)
}

fn format_int(buf: &mut [u8], mut v: u64) -> usize {
    let mut w = buf.len();
    if v == 0 {
        w -= 1;
        buf[w] = b'0';
    } else {
        while v > 0 {
            w -= 1;
            buf[w] = (v % 10) as u8 + b'0';
            v /= 10;
        }
    }
    w
}

#[cfg(test)]
mod tests {
    use crate::duration::{Duration, format_duration, parse_duration};

    fn assert_duration(input: &str, expected: Duration) {
        let (_, duration) = parse_duration(input).unwrap();
        assert_eq!(duration, expected, "{input}");
    }

    fn assert_print_duration(input: Duration, expected: &str) {
        let actual = format_duration(&input);
        assert_eq!(actual, expected, "{input}");
    }

    macro_rules! assert_durations {
        ($($str:expr => $duration:expr),*$(,)?) => {
            #[test]
            fn test_durations() {
                $(
                    assert_duration($str, $duration);
                )*
            }
        };
    }

    macro_rules! assert_duration_format {
        ($($duration:expr => $str:expr),*$(,)?) => {
            #[test]
            fn test_format_durations() {
                $(
                    assert_print_duration($duration, $str);
                )*
            }
        };
    }

    assert_durations! {
        "1s" => Duration::seconds(1),
        "-1s" => Duration::seconds(-1),
        "1.1s" => Duration::seconds(1) + Duration::milliseconds(100),
        "1.5m" => Duration::minutes(1) + Duration::seconds(30),
        "1m1s" => Duration::minutes(1) + Duration::seconds(1),
        "1h1m1s" => Duration::hours(1) + Duration::minutes(1) + Duration::seconds(1),
        "1ms" => Duration::milliseconds(1),
        "1us" => Duration::microseconds(1),
        "1ns" => Duration::nanoseconds(1),
        "1.1ns" => Duration::nanoseconds(1),
        "1.123us" => Duration::microseconds(1) + Duration::nanoseconds(123),
        "0s" => Duration::ZERO,
        "0h0m0s" => Duration::ZERO,
        "0h0m1s" => Duration::seconds(1),
        "0" => Duration::ZERO,
        "-0" => Duration::ZERO,
    }

    assert_duration_format! {
        Duration::ZERO => "0s",
        Duration::nanoseconds(1) => "1ns",
        Duration::nanoseconds(1100) => "1.1µs",
        Duration::microseconds(2200) => "2.2ms",
        Duration::milliseconds(3300) => "3.3s",
        Duration::minutes(4) + Duration::seconds(5) => "4m5s",
        Duration::minutes(4) + Duration::milliseconds(5001) => "4m5.001s",
        Duration::hours(5) + Duration::minutes(6) + Duration::milliseconds(7001) => "5h6m7.001s",
        Duration::minutes(8) + Duration::nanoseconds(1) => "8m0.000000001s",
        Duration::nanoseconds(i64::MAX) => "2562047h47m16.854775807s",
        Duration::nanoseconds(i64::MIN) => "-2562047h47m16.854775808s",
    }
}
