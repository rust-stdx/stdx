//! Calendar-aware truncation of [`DateTime`] values.

use crate::{
    civil::{self, SECS_PER_DAY},
    datetime::{DateTime, MAX_DAYS, MAX_SECS, MAX_YEAR, MIN_DAYS, MIN_SECS, MIN_YEAR},
    timezone::{Ambiguity, TimeZone},
};

const NANOS_PER_SEC: i128 = 1_000_000_000;
const NANOS_PER_MINUTE: i128 = 60 * NANOS_PER_SEC;
const NANOS_PER_HOUR: i128 = 60 * NANOS_PER_MINUTE;
const NANOS_PER_DAY: i128 = 24 * NANOS_PER_HOUR;

/// The maximum UTC offset is less than two days, so a local date can differ
/// from the supported UTC date range by at most two days.
const LOCAL_DAY_MARGIN: i64 = 2;

/// A calendar-aware amount used to truncate a [`DateTime`].
///
/// A `Period` is a count of one unit. It is not a
/// [`Duration`](core::time::Duration): a month or a year has no fixed length,
/// and a local day can be shorter or longer than 24 hours.
///
/// Counts are unsigned. A zero count is a no-op. Composite periods such as
/// "1 month and 15 days" are not represented.
///
/// The bucket origins and the exact behavior are documented on
/// [`DateTime::truncate`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Period {
    count: u64,
    unit: Unit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Unit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

impl Period {
    /// Returns a period of `count` nanoseconds.
    ///
    /// This is a wall-clock period; see [`DateTime::truncate`].
    #[must_use]
    pub const fn nanoseconds(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Nanosecond,
        }
    }

    /// Returns a period of `count` microseconds.
    ///
    /// This is a wall-clock period; see [`DateTime::truncate`].
    #[must_use]
    pub const fn microseconds(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Microsecond,
        }
    }

    /// Returns a period of `count` milliseconds.
    ///
    /// This is a wall-clock period; see [`DateTime::truncate`].
    #[must_use]
    pub const fn milliseconds(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Millisecond,
        }
    }

    /// Returns a period of `count` seconds.
    ///
    /// This is a wall-clock period, not an elapsed-time duration; see
    /// [`DateTime::truncate`].
    #[must_use]
    pub const fn seconds(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Second,
        }
    }

    /// Returns a period of `count` minutes.
    ///
    /// This is a wall-clock period, not an elapsed-time duration; see
    /// [`DateTime::truncate`].
    #[must_use]
    pub const fn minutes(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Minute,
        }
    }

    /// Returns a period of `count` hours.
    ///
    /// This is a wall-clock period, not an elapsed-time duration; see
    /// [`DateTime::truncate`].
    #[must_use]
    pub const fn hours(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Hour,
        }
    }

    /// Returns a period of `count` calendar days.
    ///
    /// Day buckets are anchored at the local date `1970-01-01`. There is no
    /// week unit: `Period::days(7)` is anchored at `1970-01-01`, which was a
    /// Thursday, so seven-day buckets start on Thursdays rather than Mondays.
    #[must_use]
    pub const fn days(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Day,
        }
    }

    /// Returns a period of `count` calendar months.
    ///
    /// Month buckets are anchored at January 1970. Six months therefore run
    /// January through June and July through December. Counts that do not
    /// divide 12 (for example five months) are still continuous, but their
    /// boundaries move through the year instead of aligning with January.
    #[must_use]
    pub const fn months(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Month,
        }
    }

    /// Returns a period of `count` calendar years.
    ///
    /// Year buckets are anchored at 1970 and start on January 1.
    #[must_use]
    pub const fn years(count: u64) -> Period {
        Period {
            count,
            unit: Unit::Year,
        }
    }
}

impl DateTime {
    /// Truncates this value to the start of the containing period.
    ///
    /// The result is never later than `self`. The value's [`TimeZone`] is
    /// preserved and its UTC offset is recomputed for the result.
    ///
    /// # Calendar periods
    ///
    /// `Day`, `Month` and `Year` periods are calendar-aware and operate on the
    /// local calendar fields:
    ///
    /// * `Period::days(1)` is the start of the local day, not a fixed
    ///   24-hour interval. Across a DST transition a local day can be 23 or
    ///   25 hours long.
    /// * `Period::months(6)` is a six-month calendar period, not 180 days.
    /// * Month buckets are anchored at January 1970. Six-month buckets
    ///   therefore run January-June and July-December. Counts that do not
    ///   divide 12, such as five months, do not align with year boundaries and
    ///   their starts move through the calendar.
    /// * Year buckets are anchored at 1970 and start on January 1.
    ///
    /// There is deliberately no week unit. `Period::days(7)` does not mean an
    /// ISO week: day buckets are anchored at 1970-01-01, which was a Thursday,
    /// so seven-day buckets start on Thursdays.
    ///
    /// # Clock periods
    ///
    /// Periods smaller than a day are wall-clock periods anchored at local
    /// `1970-01-01T00:00:00`. They are not UTC or elapsed-time periods:
    ///
    /// * `Period::hours(6)` aligns with local 00:00, 06:00, 12:00 and 18:00
    ///   because six hours divide a day. `Period::hours(5)` drifts across
    ///   local days, so its boundaries are not always the same local time.
    /// * `Period::hours(24)` is a wall-clock period anchored at local
    ///   midnight, but it is still a clock period. Use `Period::days(1)` when
    ///   you mean a calendar day.
    /// * Clock periods use the latest matching boundary that is not after
    ///   `self`. In a DST fold, a repeated local hour or minute therefore
    ///   stays in the same occurrence instead of jumping back to the earlier
    ///   occurrence.
    ///
    /// # Daylight saving transitions
    ///
    /// Local boundaries can be skipped (a spring-forward gap) or repeated (a
    /// fall-back fold):
    ///
    /// * Calendar periods return the earliest valid instant of the local
    ///   period. A skipped midnight resolves to the transition instant. A
    ///   repeated midnight selects the earlier occurrence, unless the value is
    ///   already aligned to that boundary and is therefore returned unchanged.
    ///   A fold at midnight thus normally remains part of the same calendar
    ///   day.
    /// * Clock periods return the latest boundary not after `self`, or the
    ///   transition instant when the nominal boundary does not exist. Around
    ///   a gap, a clock bucket can therefore be shorter than nominal, and
    ///   adjacent nominal boundaries can resolve to the same instant.
    ///
    /// # Surprising behavior
    ///
    /// * Truncating in a non-UTC zone is not the same as truncating in UTC
    ///   and converting back. For example, a day boundary in `+05:30` is
    ///   18:30 UTC on the previous day.
    /// * The result is not necessarily the same as subtracting a fixed
    ///   duration. `Period::days(1)` and 24 elapsed hours differ across DST.
    /// * Truncation is always downwards. Rounding up to the next boundary is
    ///   not provided.
    /// * A count larger than the distance from the 1970 anchor selects the
    ///   anchor itself. For example, `Period::years(u64::MAX)` on a 2024 value
    ///   truncates to `1970-01-01`; for a value before 1970 it saturates to the
    ///   earliest representable instant.
    /// * A zero-count period returns the value unchanged, including its
    ///   monotonic reading.
    /// * If truncation changes the value, the monotonic clock reading is
    ///   dropped because the result is a different wall-clock instant. If the
    ///   value is already aligned to the period, it is returned unchanged.
    /// * This method is infallible. If the computed boundary is before the
    ///   earliest representable instant, the result saturates to that instant.
    ///   Near the upper boundary, where a large positive UTC offset can make
    ///   the local year `10000`, the result may be the input unchanged if no
    ///   representable bucket boundary exists. At such extremes the result may
    ///   not be an exact period boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use time::{DateTime, Period};
    ///
    /// let dt: DateTime = "2024-08-15T12:00:00Z".parse().unwrap();
    ///
    /// // Six-month calendar bucket: January-June or July-December.
    /// assert_eq!(
    ///     dt.truncate(Period::months(6)).to_string(),
    ///     "2024-07-01T00:00:00Z"
    /// );
    ///
    /// // Local calendar day.
    /// assert_eq!(
    ///     dt.truncate(Period::days(1)).to_string(),
    ///     "2024-08-15T00:00:00Z"
    /// );
    /// ```
    #[must_use]
    pub fn truncate(&self, period: Period) -> DateTime {
        if period.count == 0 {
            return *self;
        }

        let original = self.parts();
        let (year, month, day, hour, minute, second, nanosecond) = original;

        let clock =
            |unit_nanos| clock_bucket(period.count, unit_nanos, year, month, day, hour, minute, second, nanosecond);

        let bucket = match period.unit {
            Unit::Nanosecond => clock(1),
            Unit::Microsecond => clock(1_000),
            Unit::Millisecond => clock(1_000_000),
            Unit::Second => clock(NANOS_PER_SEC),
            Unit::Minute => clock(NANOS_PER_MINUTE),
            Unit::Hour => clock(NANOS_PER_HOUR),
            Unit::Day => day_bucket(period.count, year, month, day),
            Unit::Month => month_bucket(period.count, year, month),
            Unit::Year => year_bucket(period.count, year),
        };

        let (year, month, day, hour, minute, second, nanosecond) = match bucket {
            Bucket::Low => return saturated_low(self.zone),
            // A bucket at the very end of the supported range may be
            // unrepresentable. Returning the input keeps the result at or
            // before `self`; this is documented on the method.
            Bucket::High => return *self,
            Bucket::Local {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nanosecond,
            } => (year, month, day, hour, minute, second, nanosecond),
        };

        if (year, month, day, hour, minute, second, nanosecond) == original {
            return *self;
        }

        let resolution = match period.unit {
            Unit::Day | Unit::Month | Unit::Year => Resolution::Start,
            Unit::Nanosecond | Unit::Microsecond | Unit::Millisecond | Unit::Second | Unit::Minute | Unit::Hour => {
                Resolution::Value
            }
        };

        self.resolve_bucket(year, month, day, hour, minute, second, nanosecond, resolution)
    }

    fn resolve_bucket(
        &self,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
        resolution: Resolution,
    ) -> DateTime {
        // Build the wall-clock instant directly. `civil` supports the one-year
        // margin that a large UTC offset can produce at the supported range
        // edges, and `from_raw_checked` below is the authority on whether the
        // resulting instant is representable.
        let local = civil::days_from_civil(year, month, day) * SECS_PER_DAY
            + i64::from(hour) * 3_600
            + i64::from(minute) * 60
            + i64::from(second);

        let ambiguity = self.zone.classify(local);

        let (instant, nanosecond) = match ambiguity {
            Ambiguity::Exact {
                offset,
            } => (local - i64::from(offset), nanosecond),
            Ambiguity::Fold {
                earlier,
                later,
            } => {
                let early = local - i64::from(earlier);
                let late = local - i64::from(later);
                let instant = match resolution {
                    Resolution::Start => early,
                    Resolution::Value if late <= self.secs => late,
                    Resolution::Value => early,
                };
                (instant, nanosecond)
            }
            // The nominal boundary was skipped. The first instant at or after
            // it is the transition instant; sub-second fields cannot survive
            // a gap.
            Ambiguity::Gap {
                before,
                after,
            } => (gap_transition(self.zone, local, before, after), 0),
        };

        if instant < MIN_SECS {
            return saturated_low(self.zone);
        }
        if instant > MAX_SECS {
            return *self;
        }

        match DateTime::from_raw_checked(instant, nanosecond, self.zone) {
            Ok(result) => {
                // Truncation is never allowed to move forward, even if a
                // pathological time-zone transition makes the resolution
                // non-monotonic.
                if result > *self {
                    return *self;
                }
                #[cfg(feature = "std")]
                if result.secs == self.secs && result.nanos == self.nanos {
                    let mut result = result;
                    result.monotonic = self.monotonic;
                    return result;
                }
                result
            }
            Err(_) => {
                if instant < MIN_SECS {
                    saturated_low(self.zone)
                } else {
                    *self
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Resolution {
    Start,
    Value,
}

enum Bucket {
    Local {
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    },
    Low,
    High,
}

#[allow(clippy::too_many_arguments)]
fn clock_bucket(
    count: u64,
    unit_nanos: i128,
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
) -> Bucket {
    let days = i128::from(civil::days_from_civil(year, month, day));
    let local = days * NANOS_PER_DAY
        + i128::from(hour) * NANOS_PER_HOUR
        + i128::from(minute) * NANOS_PER_MINUTE
        + i128::from(second) * NANOS_PER_SEC
        + i128::from(nanosecond);

    let period_nanos = i128::from(count) * unit_nanos;
    let bucket = local.div_euclid(period_nanos) * period_nanos;

    let bucket_day = bucket.div_euclid(NANOS_PER_DAY);
    if bucket_day < i128::from(MIN_DAYS - LOCAL_DAY_MARGIN) {
        return Bucket::Low;
    }
    if bucket_day > i128::from(MAX_DAYS + LOCAL_DAY_MARGIN) {
        return Bucket::High;
    }

    let remainder = bucket.rem_euclid(NANOS_PER_DAY);
    let hour = (remainder / NANOS_PER_HOUR) as u8;
    let minute = ((remainder % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
    let second = ((remainder % NANOS_PER_MINUTE) / NANOS_PER_SEC) as u8;
    let nanosecond = (remainder % NANOS_PER_SEC) as u32;
    let (year, month, day) = civil::civil_from_days(bucket_day as i64);

    Bucket::Local {
        year,
        month,
        day,
        hour,
        minute,
        second,
        nanosecond,
    }
}

fn day_bucket(count: u64, year: i32, month: u8, day: u8) -> Bucket {
    let days = i128::from(civil::days_from_civil(year, month, day));
    let bucket = days.div_euclid(i128::from(count)) * i128::from(count);

    if bucket < i128::from(MIN_DAYS - LOCAL_DAY_MARGIN) {
        return Bucket::Low;
    }
    if bucket > i128::from(MAX_DAYS + LOCAL_DAY_MARGIN) {
        return Bucket::High;
    }

    let (year, month, day) = civil::civil_from_days(bucket as i64);
    Bucket::Local {
        year,
        month,
        day,
        hour: 0,
        minute: 0,
        second: 0,
        nanosecond: 0,
    }
}

fn month_bucket(count: u64, year: i32, month: u8) -> Bucket {
    let index = (i128::from(year) - 1970) * 12 + i128::from(month) - 1;
    let bucket = index.div_euclid(i128::from(count)) * i128::from(count);
    let year = 1970 + bucket.div_euclid(12);
    let month = (bucket.rem_euclid(12) + 1) as u8;

    if year < i128::from(MIN_YEAR - 1) {
        return Bucket::Low;
    }
    if year > i128::from(MAX_YEAR + 1) {
        return Bucket::High;
    }

    Bucket::Local {
        year: year as i32,
        month,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        nanosecond: 0,
    }
}

fn year_bucket(count: u64, year: i32) -> Bucket {
    let index = i128::from(year) - 1970;
    let bucket = index.div_euclid(i128::from(count)) * i128::from(count);
    let year = 1970 + bucket;

    if year < i128::from(MIN_YEAR - 1) {
        return Bucket::Low;
    }
    if year > i128::from(MAX_YEAR + 1) {
        return Bucket::High;
    }

    Bucket::Local {
        year: year as i32,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        nanosecond: 0,
    }
}

/// Returns the first instant at or after a skipped local time.
///
/// For a gap, `local` lies in
/// `[transition + before, transition + after)`, so `local - after` is before
/// the transition and `local - before` is at or after it. The offset changes
/// exactly once between those bounds, which makes the transition the first
/// second whose offset is `after`.
fn gap_transition(zone: TimeZone, local: i64, before: i32, after: i32) -> i64 {
    let mut low = local - i64::from(after);
    let mut high = local - i64::from(before);
    if high <= low {
        return zone.resolve_compatible(local);
    }

    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if zone.offset_at(middle) == before {
            low = middle;
        } else {
            high = middle;
        }
    }

    high
}

fn saturated_low(zone: TimeZone) -> DateTime {
    DateTime::from_raw_checked(MIN_SECS, 0, zone)
        .unwrap_or_else(|_| DateTime::from_raw(MIN_SECS, 0, zone.offset_at(MIN_SECS), zone))
}
