//! Proleptic Gregorian calendar math.
//!
//! All routines use astronomical year numbering (year `0` exists and denotes
//! `1 BCE`). Day `0` is `1970-01-01` (the Unix epoch). The conversion
//! algorithms are the well-known ones by Howard Hinnant.

/// Number of seconds in a day.
pub(crate) const SECS_PER_DAY: i64 = 86_400;

/// Number of nanoseconds in a second.
pub(crate) const NANOS_PER_SEC: u32 = 1_000_000_000;

/// Number of days in a 400-year Gregorian cycle.
const DAYS_PER_CYCLE: i64 = 146_097;

/// Days from `0000-03-01` to the Unix epoch.
const EPOCH_DAYS: i64 = 719_468;

/// Returns `true` if `year` is a leap year in the proleptic Gregorian calendar.
#[must_use]
pub(crate) const fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Returns the number of days in the given month, or `0` if `month` is invalid.
#[must_use]
pub(crate) const fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Converts a civil date to the number of days since the Unix epoch.
///
/// The inputs are assumed to have already been validated.
#[must_use]
pub(crate) const fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let year = year as i64 - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400; // [0, 399]
    let month = month as i64;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * DAYS_PER_CYCLE + doe - EPOCH_DAYS
}

/// Converts a number of days since the Unix epoch back to a civil date.
#[must_use]
pub(crate) const fn civil_from_days(days: i64) -> (i32, u8, u8) {
    let days = days + EPOCH_DAYS;
    let era = if days >= 0 { days } else { days - (DAYS_PER_CYCLE - 1) } / DAYS_PER_CYCLE;
    let doe = days - era * DAYS_PER_CYCLE; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let mut year = (yoe + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let month = (mp + if mp < 10 { 3 } else { -9 }) as u8;
    if month <= 2 {
        year += 1;
    }
    (year, month, day)
}

/// Returns the day of the year (`1..=366`).
#[must_use]
pub(crate) const fn ordinal(year: i32, month: u8, day: u8) -> u16 {
    (days_from_civil(year, month, day) - days_from_civil(year, 1, 1) + 1) as u16
}

/// Returns the ISO weekday (`Monday = 0 ..= Sunday = 6`).
#[must_use]
pub(crate) const fn weekday_from_days(days: i64) -> u8 {
    (days + 3).rem_euclid(7) as u8
}

/// Returns the number of ISO weeks in the given year.
#[must_use]
pub(crate) const fn iso_weeks_in_year(year: i32) -> u8 {
    let jan1 = weekday_from_days(days_from_civil(year, 1, 1));
    // A year has 53 ISO weeks when it starts on a Thursday, or on a Wednesday
    // in a leap year.
    if jan1 == 3 || (is_leap(year) && jan1 == 2) {
        53
    } else {
        52
    }
}

/// Returns the ISO 8601 week-based year and week number.
#[must_use]
pub(crate) const fn iso_week(year: i32, month: u8, day: u8) -> (i32, u8) {
    let days = days_from_civil(year, month, day);
    let weekday = weekday_from_days(days) as i32 + 1; // Monday = 1 ..= Sunday = 7
    let week = (ordinal(year, month, day) as i32 - weekday + 10) / 7;
    if week < 1 {
        (year - 1, iso_weeks_in_year(year - 1))
    } else if week > iso_weeks_in_year(year) as i32 {
        (year + 1, 1)
    } else {
        (year, week as u8)
    }
}
