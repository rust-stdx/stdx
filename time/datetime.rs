//! The [`DateTime`] type.

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

use crate::{
    civil,
    error::{self, Error},
    format::{self, Format, Formatted},
    parse,
    timezone::{Disambiguation, TimeZone},
};

/// The earliest supported calendar year.
pub(crate) const MIN_YEAR: i32 = -9999;
/// The latest supported calendar year.
pub(crate) const MAX_YEAR: i32 = 9999;

pub(crate) const MIN_SECS: i64 = civil::days_from_civil(MIN_YEAR, 1, 1) * civil::SECS_PER_DAY;
pub(crate) const MAX_SECS: i64 =
    civil::days_from_civil(MAX_YEAR, 12, 31) * civil::SECS_PER_DAY + (civil::SECS_PER_DAY - 1);

/// Validates civil fields and returns the local wall-clock time as seconds
/// since the Unix epoch.
///
/// A leap second (`second == 60`) is accepted and represented as `:59` of the
/// same minute; the library never produces `:60` on output.
pub(crate) fn local_seconds_checked(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> Result<i64, Error> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(error::out_of_range("year is outside the supported range"));
    }
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
    if second > 60 {
        return Err(error::invalid("second is not in 0..=60"));
    }
    // A leap second is accepted but mapped onto `:59`.
    let second = if second == 60 { 59 } else { second };
    Ok(civil::days_from_civil(year, month, day) * civil::SECS_PER_DAY
        + i64::from(hour) * 3600
        + i64::from(minute) * 60
        + i64::from(second))
}

/// A day of the week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weekday {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

impl Weekday {
    pub(crate) const fn from_index(index: u8) -> Weekday {
        match index {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            _ => Weekday::Sunday,
        }
    }

    /// Returns the English name of the weekday.
    #[must_use]
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }

    /// Returns the three-letter English abbreviation of the weekday.
    #[must_use]
    #[inline]
    pub const fn short_name(self) -> &'static str {
        match self {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        }
    }

    /// Returns the ISO day number, `Monday = 1 ..= Sunday = 7`.
    #[must_use]
    #[inline]
    pub const fn number_from_monday(self) -> u8 {
        match self {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        }
    }

    /// Returns the day number with `Sunday = 0 ..= Saturday = 6`.
    #[must_use]
    #[inline]
    pub const fn number_from_sunday(self) -> u8 {
        match self {
            Weekday::Sunday => 0,
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
        }
    }
}

/// A precise instant in time together with a [`TimeZone`].
///
/// A `DateTime` is always absolute: it is represented as nanoseconds since the
/// Unix epoch (with nanosecond precision) plus the time zone used to display
/// it. Comparisons and subtraction operate on the instant only.
///
/// The supported UTC calendar range is `-9999-01-01` through `9999-12-31`.
/// (A value near the boundary can display a year one outside this range when a
/// large UTC offset is applied.)
///
/// # Examples
///
/// ```
/// use time::DateTime;
///
/// let dt: DateTime = "2024-02-29T12:00:00Z".parse().unwrap();
/// assert_eq!(dt.year(), 2024);
/// assert_eq!(dt.month(), 2);
/// assert_eq!(dt.day(), 29);
/// ```
#[derive(Clone, Copy)]
pub struct DateTime {
    pub(crate) secs: i64,
    pub(crate) nanos: u32,
    pub(crate) offset: i32,
    pub(crate) zone: TimeZone,
    /// When both values being compared were obtained from `now`, differences
    /// are measured with the monotonic clock.
    #[cfg(feature = "std")]
    pub(crate) monotonic: Option<std::time::Instant>,
}

impl DateTime {
    /// The Unix epoch, `1970-01-01T00:00:00Z`.
    pub const UNIX_EPOCH: DateTime = DateTime {
        secs: 0,
        nanos: 0,
        offset: 0,
        zone: TimeZone::UTC,
        #[cfg(feature = "std")]
        monotonic: None,
    };

    pub(crate) const fn from_raw(secs: i64, nanos: u32, offset: i32, zone: TimeZone) -> DateTime {
        DateTime {
            secs,
            nanos,
            offset,
            zone,
            #[cfg(feature = "std")]
            monotonic: None,
        }
    }

    /// Creates a `DateTime` from whole seconds since the Unix epoch, in UTC.
    ///
    /// # Errors
    ///
    /// Returns an error if the instant is outside the supported range.
    #[inline]
    pub fn from_unix(seconds: i64) -> Result<DateTime, Error> {
        DateTime::from_unix_nanos(i128::from(seconds) * 1_000_000_000)
    }

    /// Creates a `DateTime` from nanoseconds since the Unix epoch, in UTC.
    ///
    /// # Errors
    ///
    /// Returns an error if the instant is outside the supported range.
    #[inline]
    pub fn from_unix_nanos(nanoseconds: i128) -> Result<DateTime, Error> {
        let seconds = i64::try_from(nanoseconds.div_euclid(1_000_000_000))
            .map_err(|_| error::out_of_range("instant is outside the supported range"))?;
        let nanos = nanoseconds.rem_euclid(1_000_000_000) as u32;
        DateTime::from_raw_checked(seconds, nanos, TimeZone::UTC)
    }

    /// Creates a `DateTime` from civil fields in the given time zone.
    ///
    /// A civil time that is skipped by a DST transition is shifted forward; a
    /// civil time that occurs twice selects the earlier instant. Use
    /// [`DateTime::from_parts_with`] to choose another strategy, or
    /// [`TimeZone::ambiguity`] to inspect the mapping. A leap second
    /// (`second == 60`) is accepted and represented as `:59` of the same
    /// minute; the library never produces `:60` on output.
    ///
    /// # Errors
    ///
    /// Returns an error if any field is out of range, or if the resulting
    /// instant is outside the supported range.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
        zone: TimeZone,
    ) -> Result<DateTime, Error> {
        DateTime::from_parts_with(
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
            zone,
            Disambiguation::Compatible,
        )
    }

    /// Creates a `DateTime` from civil fields, resolving any ambiguity or gap
    /// with the given [`Disambiguation`] strategy.
    ///
    /// # Errors
    ///
    /// Returns an error if any field is out of range, if the resulting instant
    /// is outside the supported range, or if `disambiguation` is
    /// [`Disambiguation::Reject`] and the civil time is ambiguous or does not
    /// exist (see [`ErrorKind::Ambiguous`](crate::ErrorKind::Ambiguous)).
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts_with(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
        zone: TimeZone,
        disambiguation: Disambiguation,
    ) -> Result<DateTime, Error> {
        if nanosecond >= civil::NANOS_PER_SEC {
            return Err(error::invalid("nanosecond is not in 0..1_000_000_000"));
        }
        let local = local_seconds_checked(year, month, day, hour, minute, second)?;
        let instant = zone.resolve_with(local, disambiguation)?;
        DateTime::from_raw_checked(instant, nanosecond, zone)
    }

    #[inline]
    pub(crate) fn from_raw_checked(secs: i64, nanos: u32, zone: TimeZone) -> Result<DateTime, Error> {
        if !(MIN_SECS..=MAX_SECS).contains(&secs) {
            return Err(error::out_of_range("instant is outside the supported range"));
        }
        let offset = zone.offset_at(secs);
        Ok(DateTime {
            secs,
            nanos,
            offset,
            zone,
            #[cfg(feature = "std")]
            monotonic: None,
        })
    }

    /// Returns the current time in the given time zone.
    #[cfg(feature = "std")]
    #[must_use]
    pub fn now_in(zone: TimeZone) -> DateTime {
        let now = std::time::SystemTime::now();
        let (seconds, nanos) = match now.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => (duration.as_secs() as i64, duration.subsec_nanos()),
            Err(err) => {
                let duration = err.duration();
                let seconds = duration.as_secs() as i64;
                if duration.subsec_nanos() == 0 {
                    (-seconds, 0)
                } else {
                    (-seconds - 1, 1_000_000_000 - duration.subsec_nanos())
                }
            }
        };
        let utc = DateTime::from_unix_nanos(i128::from(seconds) * 1_000_000_000 + i128::from(nanos))
            .unwrap_or(DateTime::UNIX_EPOCH);
        let mut result = utc.in_timezone(zone);
        result.monotonic = Some(std::time::Instant::now());
        result
    }

    /// Returns the current time in the system's local time zone.
    ///
    /// With the `timezone-system` feature the system zone is used, falling back
    /// to UTC if it cannot be determined. Without that feature this is UTC.
    #[cfg(feature = "std")]
    #[must_use]
    pub fn now() -> DateTime {
        #[cfg(feature = "timezone-system")]
        {
            match TimeZone::system() {
                Ok(zone) => DateTime::now_in(zone),
                Err(_) => DateTime::now_in(TimeZone::UTC),
            }
        }
        #[cfg(not(feature = "timezone-system"))]
        {
            DateTime::now_in(TimeZone::UTC)
        }
    }

    /// Parses a string, interpreting zone-less input in `zone`.
    ///
    /// This is [`str::parse`] with an explicit default zone. An offset or zone
    /// annotation in the input always takes precedence.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or is out of range.
    pub fn parse_with_timezone(input: &str, zone: TimeZone) -> Result<DateTime, Error> {
        parse::parse(input, Some(zone))
    }

    /// Parses a byte slice, interpreting zone-less input as UTC.
    ///
    /// This accepts the same grammar as [`str::parse`] and never allocates;
    /// non-UTF-8 input is rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or is out of range.
    pub fn parse_bytes(input: &[u8]) -> Result<DateTime, Error> {
        parse::parse_bytes(input, None)
    }

    /// Parses a byte slice, interpreting zone-less input in `zone`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or is out of range.
    pub fn parse_bytes_with_timezone(input: &[u8], zone: TimeZone) -> Result<DateTime, Error> {
        parse::parse_bytes(input, Some(zone))
    }

    /// Parses a byte slice using a specific [`Format`].
    ///
    /// This accepts the same grammar as [`DateTime::parse_with_format`] and
    /// never allocates; non-UTF-8 input is rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or is out of range.
    pub fn parse_bytes_with_format(input: &[u8], format: Format) -> Result<DateTime, Error> {
        parse::parse_bytes_with_format(input, format)
    }

    /// Parses a string using a specific [`Format`].
    ///
    /// `TimeOnly` input is interpreted on the Unix epoch date (`1970-01-01`).
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or is out of range.
    pub fn parse_with_format(input: &str, format: Format) -> Result<DateTime, Error> {
        parse::parse_with_format(input, format)
    }

    /// Returns a best-effort [`Display`](core::fmt::Display) adapter for
    /// `format`.
    ///
    /// This never fails. Formats with a restricted domain can emit a
    /// non-conforming string for out-of-domain values; see [`Format`] and use
    /// [`DateTime::try_format`] for strict output.
    #[must_use]
    pub fn format(&self, format: Format) -> Formatted<'_> {
        Formatted::new(self, format)
    }

    /// Returns a [`Display`](core::fmt::Display) adapter, checking up front
    /// that the value is representable in `format`.
    ///
    /// Unlike [`DateTime::format`], this rejects values that the format cannot
    /// express conformantly: a year outside `0000..=9999` for every format
    /// except [`Format::TimeOnly`], and a UTC offset with a seconds component
    /// for the RFC 3339, RFC 9557, ISO 8601 and RFC 2822 formats.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::OutOfRange`](crate::ErrorKind::OutOfRange) if the
    /// value cannot be represented in the requested format.
    pub fn try_format(&self, format: Format) -> Result<Formatted<'_>, Error> {
        if !matches!(format, Format::TimeOnly) && !(0..=9999).contains(&self.year()) {
            return Err(error::out_of_range("value cannot be represented in the requested format"));
        }
        if matches!(format, Format::Rfc3339 | Format::Rfc9557 | Format::Iso8601 | Format::Rfc2822)
            && self.offset() % 60 != 0
        {
            return Err(error::out_of_range("offset cannot be represented in the requested format"));
        }
        Ok(Formatted::new(self, format))
    }

    /// Writes this value in `format` into `buffer` without allocating, and
    /// returns the number of bytes written.
    ///
    /// This is the allocation-free companion of [`DateTime::format`]: it
    /// applies the same strict representability checks as
    /// [`DateTime::try_format`], so it works in `no_std` environments without
    /// `alloc`.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::OutOfRange`](crate::ErrorKind::OutOfRange) if the
    /// value cannot be represented in `format`, or
    /// [`ErrorKind::BufferTooSmall`](crate::ErrorKind::BufferTooSmall) if
    /// `buffer` is too small.
    pub fn format_to(&self, buffer: &mut [u8], format: Format) -> Result<usize, Error> {
        self.try_format(format)?.write_to(buffer)
    }

    /// Returns the number of whole seconds since the Unix epoch.
    #[must_use]
    #[inline]
    pub const fn unix(&self) -> i64 {
        self.secs
    }

    /// Returns the number of nanoseconds since the Unix epoch.
    #[must_use]
    #[inline]
    pub const fn unix_nanos(&self) -> i128 {
        self.secs as i128 * 1_000_000_000 + self.nanos as i128
    }

    /// Returns the UTC offset in seconds east of Greenwich.
    #[must_use]
    #[inline]
    pub const fn offset(&self) -> i32 {
        self.offset
    }

    /// Returns the time zone associated with this value.
    #[must_use]
    #[inline]
    pub const fn timezone(&self) -> TimeZone {
        self.zone
    }

    /// Returns the canonical name of the time zone, if it has one.
    #[must_use]
    #[inline]
    pub const fn timezone_name(&self) -> Option<&'static str> {
        self.zone.annotation_name()
    }

    /// Returns `true` if daylight saving time is in effect.
    #[must_use]
    pub fn is_dst(&self) -> bool {
        self.zone.is_dst_at(self.secs)
    }

    /// Returns the local time at this instant in `zone`.
    #[must_use]
    #[inline]
    pub fn in_timezone(&self, zone: TimeZone) -> DateTime {
        let offset = zone.offset_at(self.secs);
        DateTime {
            secs: self.secs,
            nanos: self.nanos,
            offset,
            zone,
            #[cfg(feature = "std")]
            monotonic: self.monotonic,
        }
    }

    /// Returns the local civil fields: `(year, month, day, hour, minute,
    /// second, nanosecond)`.
    #[inline]
    pub(crate) fn parts(&self) -> (i32, u8, u8, u8, u8, u8, u32) {
        let local = self.secs + i64::from(self.offset);
        let days = local.div_euclid(civil::SECS_PER_DAY);
        let seconds = local.rem_euclid(civil::SECS_PER_DAY);
        let (year, month, day) = civil::civil_from_days(days);
        let hour = (seconds / 3600) as u8;
        let minute = ((seconds % 3600) / 60) as u8;
        let second = (seconds % 60) as u8;
        (year, month, day, hour, minute, second, self.nanos)
    }

    /// Returns the calendar year.
    #[must_use]
    #[inline]
    pub fn year(&self) -> i32 {
        self.parts().0
    }

    /// Returns the month (`1..=12`).
    #[must_use]
    #[inline]
    pub fn month(&self) -> u8 {
        self.parts().1
    }

    /// Returns the day of the month (`1..=31`).
    #[must_use]
    #[inline]
    pub fn day(&self) -> u8 {
        self.parts().2
    }

    /// Returns the hour (`0..=23`).
    #[must_use]
    #[inline]
    pub fn hour(&self) -> u8 {
        self.parts().3
    }

    /// Returns the minute (`0..=59`).
    #[must_use]
    #[inline]
    pub fn minute(&self) -> u8 {
        self.parts().4
    }

    /// Returns the second (`0..=59`).
    #[must_use]
    #[inline]
    pub fn second(&self) -> u8 {
        self.parts().5
    }

    /// Returns the nanosecond (`0..1_000_000_000`).
    #[must_use]
    #[inline]
    pub fn nanosecond(&self) -> u32 {
        self.nanos
    }

    /// Returns the millisecond (`0..1000`).
    #[must_use]
    #[inline]
    pub fn millisecond(&self) -> u32 {
        self.nanos / 1_000_000
    }

    /// Returns the microsecond (`0..1_000_000`).
    #[must_use]
    #[inline]
    pub fn microsecond(&self) -> u32 {
        self.nanos / 1_000
    }

    /// Returns the day of the week.
    #[must_use]
    #[inline]
    pub fn weekday(&self) -> Weekday {
        let days = (self.secs + i64::from(self.offset)).div_euclid(civil::SECS_PER_DAY);
        Weekday::from_index(civil::weekday_from_days(days))
    }

    /// Returns the day of the year (`1..=366`).
    #[must_use]
    #[inline]
    pub fn ordinal(&self) -> u16 {
        let (year, month, day, ..) = self.parts();
        civil::ordinal(year, month, day)
    }

    /// Returns the ISO 8601 week-based year and week number.
    #[must_use]
    #[inline]
    pub fn iso_week(&self) -> (i32, u8) {
        let (year, month, day, ..) = self.parts();
        civil::iso_week(year, month, day)
    }

    /// Adds a [`Duration`](core::time::Duration) to this instant.
    ///
    /// # Errors
    ///
    /// Returns an error if the result is out of range.
    pub fn checked_add(&self, duration: core::time::Duration) -> Result<DateTime, Error> {
        let total = self
            .unix_nanos()
            .checked_add(i128::from(duration.as_secs()) * 1_000_000_000)
            .and_then(|v| v.checked_add(i128::from(duration.subsec_nanos())))
            .ok_or_else(|| error::out_of_range("result is outside the supported range"))?;
        #[allow(unused_mut)]
        let mut result = DateTime::from_unix_nanos(total)?.in_timezone_checked(self.zone)?;
        #[cfg(feature = "std")]
        {
            result.monotonic = self.monotonic.and_then(|reading| reading.checked_add(duration));
        }
        Ok(result)
    }

    /// Subtracts a [`Duration`](core::time::Duration) from this instant.
    ///
    /// # Errors
    ///
    /// Returns an error if the result is out of range.
    pub fn checked_sub(&self, duration: core::time::Duration) -> Result<DateTime, Error> {
        let total = self
            .unix_nanos()
            .checked_sub(i128::from(duration.as_secs()) * 1_000_000_000)
            .and_then(|v| v.checked_sub(i128::from(duration.subsec_nanos())))
            .ok_or_else(|| error::out_of_range("result is outside the supported range"))?;
        #[allow(unused_mut)]
        let mut result = DateTime::from_unix_nanos(total)?.in_timezone_checked(self.zone)?;
        #[cfg(feature = "std")]
        {
            result.monotonic = self.monotonic.and_then(|reading| reading.checked_sub(duration));
        }
        Ok(result)
    }

    /// Adds a duration, saturating at the supported range boundaries.
    #[must_use]
    pub fn saturating_add(&self, duration: core::time::Duration) -> DateTime {
        match self.checked_add(duration) {
            Ok(dt) => dt,
            Err(_) => DateTime::from_raw_checked(MAX_SECS, 999_999_999, self.zone)
                .unwrap_or_else(|_| DateTime::from_raw(MAX_SECS, 999_999_999, self.offset, self.zone)),
        }
    }

    /// Subtracts a duration, saturating at the supported range boundaries.
    #[must_use]
    pub fn saturating_sub(&self, duration: core::time::Duration) -> DateTime {
        match self.checked_sub(duration) {
            Ok(dt) => dt,
            Err(_) => DateTime::from_raw_checked(MIN_SECS, 0, self.zone)
                .unwrap_or_else(|_| DateTime::from_raw(MIN_SECS, 0, self.offset, self.zone)),
        }
    }

    /// Returns the duration from `earlier` to `self`.
    ///
    /// When both values were produced by [`DateTime::now`] or
    /// [`DateTime::now_in`], the measurement uses the monotonic clock and is
    /// therefore robust against wall-clock adjustments (for example NTP
    /// steps). Otherwise the wall clock is used.
    ///
    /// # Errors
    ///
    /// Returns an error if `earlier` is later than `self`.
    pub fn duration_since(&self, earlier: DateTime) -> Result<core::time::Duration, Error> {
        let (seconds, nanos) = self.signed_duration_since(earlier);
        if seconds < 0 {
            return Err(error::out_of_range("earlier is later than self"));
        }
        Ok(core::time::Duration::new(seconds as u64, nanos))
    }

    /// Returns the duration from `self` to `later`.
    ///
    /// This is the inverse of [`DateTime::duration_since`] and follows the
    /// same monotonic-clock rules.
    ///
    /// # Errors
    ///
    /// Returns an error if `later` is earlier than `self`.
    pub fn duration_until(&self, later: DateTime) -> Result<core::time::Duration, Error> {
        later.duration_since(*self)
    }

    /// Returns the signed difference `self - other` as
    /// `(seconds, subsecond_nanoseconds)`, where the sign is carried by
    /// `seconds` and the nanosecond component is always in
    /// `0..=999_999_999`.
    ///
    /// The monotonic clock is used when both values carry a reading (see
    /// [`DateTime::duration_since`]).
    #[must_use]
    pub fn signed_duration_since(&self, other: DateTime) -> (i64, u32) {
        #[cfg(feature = "std")]
        if let (Some(left), Some(right)) = (self.monotonic, other.monotonic) {
            return difference_between(left, right);
        }
        let mut seconds = self.secs - other.secs;
        let mut nanos = i64::from(self.nanos) - i64::from(other.nanos);
        if nanos < 0 {
            seconds -= 1;
            nanos += 1_000_000_000;
        }
        (seconds, nanos as u32)
    }

    /// Returns the time elapsed since this value was created.
    ///
    /// # Errors
    ///
    /// Returns an error if the system clock reports a value before this one
    /// (only possible without a monotonic reading).
    #[cfg(feature = "std")]
    pub fn elapsed(&self) -> Result<core::time::Duration, Error> {
        DateTime::now_in(self.zone).duration_since(*self)
    }

    /// Returns `true` if this value carries a monotonic clock reading, which
    /// [`DateTime::now`] and [`DateTime::now_in`] add so that elapsed-time
    /// measurement is robust against wall-clock adjustments.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn has_monotonic(&self) -> bool {
        self.monotonic.is_some()
    }

    /// Returns the time zone abbreviation in effect, such as `EST`, when the
    /// zone has one (UTC reports `UTC`; fixed offsets have none).
    #[must_use]
    pub fn abbreviation(&self) -> Option<&'static str> {
        self.zone
            .info_at(self.secs)
            .map(|info| info.abbrev)
            .filter(|abbrev| !abbrev.is_empty())
    }

    /// Returns the bounds of the constant-offset period that contains this
    /// instant, each as an optional [`DateTime`] where `None` means unbounded
    /// in that direction.
    ///
    /// Bounds are reported for embedded-database zones and only span explicit
    /// transitions. The database materializes future daylight-saving
    /// transitions up to a rolling horizon (the IANA release year plus 50), so
    /// dates beyond that horizon have an unbounded end even while a DST rule
    /// applies; fixed offsets and UTC are always unbounded.
    #[must_use]
    pub fn zone_bounds(&self) -> (Option<DateTime>, Option<DateTime>) {
        let (start, end) = self.zone.bounds_at(self.secs);
        (
            start.and_then(|secs| DateTime::from_raw_checked(secs, 0, self.zone).ok()),
            end.and_then(|secs| DateTime::from_raw_checked(secs, 0, self.zone).ok()),
        )
    }

    /// Adds whole calendar days, preserving the local time of day across DST
    /// transitions.
    ///
    /// # Errors
    ///
    /// Returns an error if the result is out of range.
    pub fn add_days(&self, days: i64) -> Result<DateTime, Error> {
        let (year, month, day, hour, minute, second, nanos) = self.parts();
        let target = civil::days_from_civil(year, month, day)
            .checked_add(days)
            .ok_or_else(|| error::out_of_range("result is outside the supported range"))?;
        let (year, month, day) = civil::civil_from_days(target);
        DateTime::from_parts(year, month, day, hour, minute, second, nanos, self.zone)
    }

    /// Adds whole calendar months, clamping the day to the end of the target
    /// month.
    ///
    /// # Errors
    ///
    /// Returns an error if the result is out of range.
    pub fn add_months(&self, months: i32) -> Result<DateTime, Error> {
        let (year, month, day, hour, minute, second, nanos) = self.parts();
        let total = i64::from(year) * 12 + i64::from(month) - 1 + i64::from(months);
        let new_year = i32::try_from(total.div_euclid(12))
            .map_err(|_| error::out_of_range("result is outside the supported range"))?;
        let new_month = (total.rem_euclid(12) + 1) as u8;
        let new_day = day.min(civil::days_in_month(new_year, new_month));
        DateTime::from_parts(new_year, new_month, new_day, hour, minute, second, nanos, self.zone)
    }

    /// Adds whole calendar years, clamping February 29 to February 28 in
    /// non-leap years.
    ///
    /// # Errors
    ///
    /// Returns an error if the result is out of range.
    pub fn add_years(&self, years: i32) -> Result<DateTime, Error> {
        let (year, month, day, hour, minute, second, nanos) = self.parts();
        let new_year = year
            .checked_add(years)
            .ok_or_else(|| error::out_of_range("result is outside the supported range"))?;
        let new_day = day.min(civil::days_in_month(new_year, month));
        DateTime::from_parts(new_year, month, new_day, hour, minute, second, nanos, self.zone)
    }

    fn in_timezone_checked(&self, zone: TimeZone) -> Result<DateTime, Error> {
        if self.secs < MIN_SECS || self.secs > MAX_SECS {
            return Err(error::out_of_range("instant is outside the supported range"));
        }
        Ok(self.in_timezone(zone))
    }
}

/// The signed difference between two monotonic readings, normalized so the
/// nanosecond component is non-negative.
#[cfg(feature = "std")]
fn difference_between(left: std::time::Instant, right: std::time::Instant) -> (i64, u32) {
    if let Some(duration) = left.checked_duration_since(right) {
        (duration.as_secs() as i64, duration.subsec_nanos())
    } else if let Some(duration) = right.checked_duration_since(left) {
        let seconds = duration.as_secs() as i64;
        if duration.subsec_nanos() == 0 {
            (-seconds, 0)
        } else {
            (-seconds - 1, 1_000_000_000 - duration.subsec_nanos())
        }
    } else {
        (0, 0)
    }
}

impl Default for DateTime {
    #[inline]
    fn default() -> DateTime {
        DateTime::UNIX_EPOCH
    }
}

impl PartialEq for DateTime {
    #[inline]
    fn eq(&self, other: &DateTime) -> bool {
        self.secs == other.secs && self.nanos == other.nanos
    }
}

impl Eq for DateTime {}

impl PartialOrd for DateTime {
    #[inline]
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    #[inline]
    fn cmp(&self, other: &DateTime) -> Ordering {
        (self.secs, self.nanos).cmp(&(other.secs, other.nanos))
    }
}

impl Hash for DateTime {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.secs.hash(state);
        self.nanos.hash(state);
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format::write_datetime(f, self)
    }
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl core::str::FromStr for DateTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<DateTime, Error> {
        parse::parse(s, None)
    }
}

impl core::ops::Add<core::time::Duration> for DateTime {
    type Output = DateTime;

    /// Adds a duration, panicking if the result is out of range.
    fn add(self, rhs: core::time::Duration) -> DateTime {
        self.checked_add(rhs).expect("DateTime + Duration overflowed")
    }
}

impl core::ops::Sub<core::time::Duration> for DateTime {
    type Output = DateTime;

    /// Subtracts a duration, panicking if the result is out of range.
    fn sub(self, rhs: core::time::Duration) -> DateTime {
        self.checked_sub(rhs).expect("DateTime - Duration overflowed")
    }
}

impl core::ops::AddAssign<core::time::Duration> for DateTime {
    fn add_assign(&mut self, rhs: core::time::Duration) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign<core::time::Duration> for DateTime {
    fn sub_assign(&mut self, rhs: core::time::Duration) {
        *self = *self - rhs;
    }
}

#[cfg(feature = "std")]
impl TryFrom<std::time::SystemTime> for DateTime {
    type Error = Error;

    /// Converts a [`SystemTime`](std::time::SystemTime) into a `DateTime`.
    ///
    /// # Errors
    ///
    /// Returns an error if the instant is outside the supported range.
    fn try_from(value: std::time::SystemTime) -> Result<DateTime, Error> {
        let (seconds, nanos) = match value.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let seconds = i64::try_from(duration.as_secs())
                    .map_err(|_| error::out_of_range("instant is outside the supported range"))?;
                (seconds, duration.subsec_nanos())
            }
            Err(err) => {
                let duration = err.duration();
                let seconds = i64::try_from(duration.as_secs())
                    .map_err(|_| error::out_of_range("instant is outside the supported range"))?;
                if duration.subsec_nanos() == 0 {
                    (-seconds, 0)
                } else {
                    (-seconds - 1, 1_000_000_000 - duration.subsec_nanos())
                }
            }
        };
        DateTime::from_unix_nanos(i128::from(seconds) * 1_000_000_000 + i128::from(nanos))
    }
}

#[cfg(feature = "std")]
impl From<DateTime> for std::time::SystemTime {
    /// Converts this value into a [`SystemTime`](std::time::SystemTime).
    ///
    /// The supported calendar range fits within `SystemTime` on common
    /// platforms; on platforms with a narrower `SystemTime` this saturates to
    /// the Unix epoch rather than panicking.
    fn from(dt: DateTime) -> std::time::SystemTime {
        let nanos = dt.unix_nanos();
        if nanos >= 0 {
            let seconds = (nanos / 1_000_000_000) as u64;
            let subsec = (nanos % 1_000_000_000) as u32;
            std::time::UNIX_EPOCH
                .checked_add(core::time::Duration::new(seconds, subsec))
                .unwrap_or(std::time::UNIX_EPOCH)
        } else {
            let magnitude = -nanos;
            let seconds = (magnitude / 1_000_000_000) as u64;
            let subsec = (magnitude % 1_000_000_000) as u32;
            std::time::UNIX_EPOCH
                .checked_sub(core::time::Duration::new(seconds, subsec))
                .unwrap_or(std::time::UNIX_EPOCH)
        }
    }
}
