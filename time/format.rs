//! Formatting of [`DateTime`] values.

use core::fmt;

use crate::{datetime::DateTime, timezone::TimeZone};

/// Stack buffer size for the allocation-free fast path used by the `Display`
/// implementations.
///
/// The longest well-formed value this crate can produce is about 74 bytes:
/// a six-byte out-of-range year, the date and time, nine fractional digits,
/// a seconds-bearing offset and a 32-byte IANA annotation (`[` + name + `]`).
/// 96 leaves headroom while staying small; only a custom zone name longer
/// than this falls back to the `fmt`-based path.
const MAX_FORMATTED_LENGTH: usize = 96;

/// A well-known datetime representation.
///
/// # Representability
///
/// [`DateTime::format`] is infallible and best effort: it can emit a
/// non-conforming string for out-of-domain values (for example `-0001` for a
/// negative year, or `+00:19:32` for a historical offset with a seconds
/// component). Use [`DateTime::try_format`] or [`DateTime::format_to`] when
/// the value must be strictly valid for the requested format; both reject a
/// year outside `0000..=9999` (except for `TimeOnly`) and, for `Rfc3339`,
/// `Rfc9557`, `Iso8601` and `Rfc2822`, an offset with a seconds component.
/// `TimeOnly` is always valid.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[non_exhaustive]
pub enum Format {
    /// RFC 3339 / ISO 8601, for example `2006-01-02T15:04:05Z` or
    /// `2006-01-02T15:04:05+07:00`.
    Rfc3339,
    /// RFC 3339 plus an RFC 9557 time zone annotation, for example
    /// `2006-01-02T15:04:05-05:00[America/New_York]`.
    Rfc9557,
    /// RFC 3339, except that a zero offset is written as `+00:00` rather than
    /// `Z`.
    Iso8601,
    /// RFC 2822 (email) date, for example
    /// `Mon, 02 Jan 2006 15:04:05 +0000`.
    Rfc2822,
    /// HTTP `IMF-fixdate`, for example `Mon, 02 Jan 2006 15:04:05 GMT`.
    ///
    /// The value is always converted to UTC.
    HttpDate,
    /// Just the date, for example `2006-01-02`.
    DateOnly,
    /// Just the time, for example `15:04:05` (with fractional seconds when
    /// they are non-zero).
    TimeOnly,
}

/// A [`Display`](core::fmt::Display) adapter returned by [`DateTime::format`].
pub struct Formatted<'a> {
    dt: &'a DateTime,
    format: Format,
}

impl<'a> Formatted<'a> {
    pub(crate) fn new(dt: &'a DateTime, format: Format) -> Formatted<'a> {
        Formatted {
            dt,
            format,
        }
    }

    /// Writes the formatted value into `buffer` without allocating, returning
    /// the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::BufferTooSmall`](crate::ErrorKind::BufferTooSmall)
    /// if `buffer` cannot hold the result.
    pub fn write_to(&self, buffer: &mut [u8]) -> Result<usize, crate::Error> {
        format_into(buffer, self.dt, self.format)
            .ok_or_else(|| crate::error::buffer_too_small("buffer is too small for the formatted value"))
    }
}

/// A fixed-buffer writer used by the allocation-free rendering fast path.
/// Every method returns `false` when the buffer is full.
struct Buf<'a> {
    bytes: &'a mut [u8],
    len: usize,
}

impl Buf<'_> {
    fn byte(&mut self, byte: u8) -> bool {
        if self.len < self.bytes.len() {
            self.bytes[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }

    fn bytes(&mut self, bytes: &[u8]) -> bool {
        if self.len + bytes.len() <= self.bytes.len() {
            self.bytes[self.len..self.len + bytes.len()].copy_from_slice(bytes);
            self.len += bytes.len();
            true
        } else {
            false
        }
    }

    /// Writes `value` as exactly `width` zero-padded decimal digits.
    fn pad(&mut self, mut value: u32, width: usize) -> bool {
        let mut scratch = [b'0'; 10];
        let mut index = scratch.len();
        loop {
            index -= 1;
            scratch[index] = b'0' + (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        let digits = scratch.len() - index;
        if digits > width {
            return false;
        }
        let padding = [b'0'; 10];
        let pad = width - digits;
        self.bytes(&padding[..pad]) && self.bytes(&scratch[index..])
    }

    /// Writes a year exactly as [`write_year`] does.
    fn year(&mut self, year: i32) -> bool {
        if (0..=9999).contains(&year) {
            self.pad(year as u32, 4)
        } else if year < 0 {
            self.byte(b'-') && self.pad((-i64::from(year)) as u32, 4)
        } else {
            self.byte(b'+') && self.pad(year as u32, 1)
        }
    }

    /// Writes a UTC offset as `+HH:MM` or `+HH:MM:SS`.
    fn offset(&mut self, offset: i32) -> bool {
        let sign = if offset < 0 { b'-' } else { b'+' };
        let abs = offset.unsigned_abs();
        let seconds = abs % 60;
        self.byte(sign)
            && self.pad(abs / 3600, 2)
            && self.byte(b':')
            && self.pad((abs % 3600) / 60, 2)
            && (seconds == 0 || (self.byte(b':') && self.pad(seconds, 2)))
    }

    /// Writes a UTC offset as `+HHMM` (the RFC 2822 form).
    fn offset_compact(&mut self, offset: i32) -> bool {
        let sign = if offset < 0 { b'-' } else { b'+' };
        let abs = offset.unsigned_abs();
        self.byte(sign) && self.pad(abs / 3600, 2) && self.pad((abs % 3600) / 60, 2)
    }

    /// Writes fractional seconds exactly as [`write_fraction`] does.
    fn fraction(&mut self, nanos: u32) -> bool {
        if nanos == 0 {
            true
        } else if nanos % 1_000_000 == 0 {
            self.byte(b'.') && self.pad(nanos / 1_000_000, 3)
        } else if nanos % 1_000 == 0 {
            self.byte(b'.') && self.pad(nanos / 1_000, 6)
        } else {
            self.byte(b'.') && self.pad(nanos, 9)
        }
    }
}

fn month_short_name(month: u8) -> &'static str {
    month_short(month)
}

fn format_iso(buf: &mut Buf<'_>, dt: &DateTime, z_suffix: bool, zone: bool) -> bool {
    let (year, month, day, hour, minute, second, nanos) = dt.parts();
    let base = buf.year(year)
        && buf.byte(b'-')
        && buf.pad(u32::from(month), 2)
        && buf.byte(b'-')
        && buf.pad(u32::from(day), 2)
        && buf.byte(b'T')
        && buf.pad(u32::from(hour), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(minute), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(second), 2)
        && buf.fraction(nanos)
        && {
            let offset = dt.offset();
            if z_suffix && offset == 0 {
                buf.byte(b'Z')
            } else {
                buf.offset(offset)
            }
        };
    if !base || !zone {
        return base;
    }
    match dt.timezone_name() {
        Some(name) => buf.byte(b'[') && buf.bytes(name.as_bytes()) && buf.byte(b']'),
        None => true,
    }
}

fn format_rfc2822(buf: &mut Buf<'_>, dt: &DateTime) -> bool {
    let (year, month, day, hour, minute, second, _) = dt.parts();
    buf.bytes(dt.weekday().short_name().as_bytes())
        && buf.bytes(b", ")
        && buf.pad(u32::from(day), 2)
        && buf.byte(b' ')
        && buf.bytes(month_short_name(month).as_bytes())
        && buf.byte(b' ')
        && buf.year(year)
        && buf.byte(b' ')
        && buf.pad(u32::from(hour), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(minute), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(second), 2)
        && buf.byte(b' ')
        && buf.offset_compact(dt.offset())
}

fn format_http(buf: &mut Buf<'_>, dt: &DateTime) -> bool {
    let utc = dt.in_timezone(TimeZone::UTC);
    let (year, month, day, hour, minute, second, _) = utc.parts();
    buf.bytes(utc.weekday().short_name().as_bytes())
        && buf.bytes(b", ")
        && buf.pad(u32::from(day), 2)
        && buf.byte(b' ')
        && buf.bytes(month_short_name(month).as_bytes())
        && buf.byte(b' ')
        && buf.year(year)
        && buf.byte(b' ')
        && buf.pad(u32::from(hour), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(minute), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(second), 2)
        && buf.bytes(b" GMT")
}

fn format_date(buf: &mut Buf<'_>, dt: &DateTime) -> bool {
    let (year, month, day, ..) = dt.parts();
    buf.year(year) && buf.byte(b'-') && buf.pad(u32::from(month), 2) && buf.byte(b'-') && buf.pad(u32::from(day), 2)
}

fn format_time(buf: &mut Buf<'_>, dt: &DateTime) -> bool {
    let (_, _, _, hour, minute, second, nanos) = dt.parts();
    buf.pad(u32::from(hour), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(minute), 2)
        && buf.byte(b':')
        && buf.pad(u32::from(second), 2)
        && buf.fraction(nanos)
}

/// Renders `dt` in `format` into `out`, returning the number of bytes written,
/// or `None` when `out` is too small.
///
/// This is the allocation-free fast path. It can only fail for a buffer that is
/// too small (including the internal stack buffer used by `Display` when a time
/// zone name is unusually long); callers fall back to the `fmt`-based
/// [`write_format`] in that case.
pub(crate) fn format_into(out: &mut [u8], dt: &DateTime, format: Format) -> Option<usize> {
    let mut buf = Buf {
        bytes: out,
        len: 0,
    };
    let ok = match format {
        Format::Rfc3339 => format_iso(&mut buf, dt, true, false),
        Format::Rfc9557 => format_iso(&mut buf, dt, true, true),
        Format::Iso8601 => format_iso(&mut buf, dt, false, false),
        Format::Rfc2822 => format_rfc2822(&mut buf, dt),
        Format::HttpDate => format_http(&mut buf, dt),
        Format::DateOnly => format_date(&mut buf, dt),
        Format::TimeOnly => format_time(&mut buf, dt),
    };
    if ok { Some(buf.len) } else { None }
}

impl fmt::Display for Formatted<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = [0u8; MAX_FORMATTED_LENGTH];
        match format_into(&mut buffer, self.dt, self.format) {
            Some(len) => f.write_str(core::str::from_utf8(&buffer[..len]).map_err(|_| fmt::Error)?),
            None => write_format(f, self.dt, self.format),
        }
    }
}

impl fmt::Debug for Formatted<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub(crate) fn write_year(f: &mut fmt::Formatter<'_>, year: i32) -> fmt::Result {
    if (0..=9999).contains(&year) {
        write!(f, "{year:04}")
    } else if year < 0 {
        write!(f, "-{:04}", -i64::from(year))
    } else {
        write!(f, "+{year}")
    }
}

/// Writes a UTC offset as `+HH:MM` or `+HH:MM:SS`.
pub(crate) fn write_offset(f: &mut fmt::Formatter<'_>, offset: i32) -> fmt::Result {
    let sign = if offset < 0 { '-' } else { '+' };
    let abs = offset.unsigned_abs();
    let hours = abs / 3600;
    let minutes = (abs % 3600) / 60;
    let seconds = abs % 60;
    if seconds == 0 {
        write!(f, "{sign}{hours:02}:{minutes:02}")
    } else {
        write!(f, "{sign}{hours:02}:{minutes:02}:{seconds:02}")
    }
}

/// Writes a UTC offset as `+HHMM` (the RFC 2822 form).
fn write_offset_compact(f: &mut fmt::Formatter<'_>, offset: i32) -> fmt::Result {
    let sign = if offset < 0 { '-' } else { '+' };
    let abs = offset.unsigned_abs();
    let hours = abs / 3600;
    let minutes = (abs % 3600) / 60;
    write!(f, "{sign}{hours:02}{minutes:02}")
}

fn write_fraction(f: &mut fmt::Formatter<'_>, nanos: u32) -> fmt::Result {
    if nanos == 0 {
        Ok(())
    } else if nanos % 1_000_000 == 0 {
        write!(f, ".{:03}", nanos / 1_000_000)
    } else if nanos % 1_000 == 0 {
        write!(f, ".{:06}", nanos / 1_000)
    } else {
        write!(f, ".{nanos:09}")
    }
}

fn month_short(month: u8) -> &'static str {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    (month as usize)
        .checked_sub(1)
        .and_then(|index| MONTHS.get(index))
        .copied()
        .unwrap_or("???")
}

pub(crate) fn write_datetime(f: &mut fmt::Formatter<'_>, dt: &DateTime) -> fmt::Result {
    let mut buffer = [0u8; MAX_FORMATTED_LENGTH];
    if let Some(len) = format_into(&mut buffer, dt, Format::Rfc9557) {
        return f.write_str(core::str::from_utf8(&buffer[..len]).map_err(|_| fmt::Error)?);
    }
    // Fallback for an unusually long time zone name.
    let (year, month, day, hour, minute, second, nanos) = dt.parts();
    write_year(f, year)?;
    write!(f, "-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}")?;
    write_fraction(f, nanos)?;
    if dt.offset() == 0 {
        f.write_str("Z")?;
    } else {
        write_offset(f, dt.offset())?;
    }
    if let Some(name) = dt.timezone_name() {
        write!(f, "[{name}]")?;
    }
    Ok(())
}

fn write_format(f: &mut fmt::Formatter<'_>, dt: &DateTime, format: Format) -> fmt::Result {
    match format {
        Format::Rfc3339 => write_iso(f, dt, true, false),
        Format::Rfc9557 => write_iso(f, dt, true, true),
        Format::Iso8601 => write_iso(f, dt, false, false),
        Format::Rfc2822 => write_rfc2822(f, dt),
        Format::HttpDate => write_http_date(f, dt),
        Format::DateOnly => write_date(f, dt),
        Format::TimeOnly => write_time(f, dt),
    }
}

fn write_iso(f: &mut fmt::Formatter<'_>, dt: &DateTime, z_suffix: bool, zone: bool) -> fmt::Result {
    let (year, month, day, hour, minute, second, nanos) = dt.parts();
    write_year(f, year)?;
    write!(f, "-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}")?;
    write_fraction(f, nanos)?;
    let offset = dt.offset();
    if z_suffix && offset == 0 {
        f.write_str("Z")?;
    } else {
        write_offset(f, offset)?;
    }
    if zone {
        if let Some(name) = dt.timezone_name() {
            write!(f, "[{name}]")?;
        }
    }
    Ok(())
}

fn write_date(f: &mut fmt::Formatter<'_>, dt: &DateTime) -> fmt::Result {
    let (year, month, day, ..) = dt.parts();
    write_year(f, year)?;
    write!(f, "-{month:02}-{day:02}")
}

fn write_time(f: &mut fmt::Formatter<'_>, dt: &DateTime) -> fmt::Result {
    let (_, _, _, hour, minute, second, nanos) = dt.parts();
    write!(f, "{hour:02}:{minute:02}:{second:02}")?;
    write_fraction(f, nanos)
}

fn write_rfc2822(f: &mut fmt::Formatter<'_>, dt: &DateTime) -> fmt::Result {
    let (year, month, day, hour, minute, second, _) = dt.parts();
    write!(f, "{}, {day:02} {} ", dt.weekday().short_name(), month_short(month))?;
    write_year(f, year)?;
    write!(f, " {hour:02}:{minute:02}:{second:02} ")?;
    write_offset_compact(f, dt.offset())
}

fn write_http_date(f: &mut fmt::Formatter<'_>, dt: &DateTime) -> fmt::Result {
    let utc = dt.in_timezone(TimeZone::UTC);
    let (year, month, day, hour, minute, second, _) = utc.parts();
    write!(f, "{}, {day:02} {} ", utc.weekday().short_name(), month_short(month))?;
    write_year(f, year)?;
    write!(f, " {hour:02}:{minute:02}:{second:02} GMT")
}
