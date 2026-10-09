//! Minimal parser and evaluator for POSIX `TZ` strings.
//!
//! TZif footers (and the `TZ` environment variable) encode future time zone
//! transitions using the POSIX `TZ` grammar. The library needs
//! this to answer "what is the offset in the future?" once the explicit
//! transitions in a TZif file are exhausted.

use crate::{
    civil::{SECS_PER_DAY, days_from_civil, is_leap},
    tzif::ZoneInfo,
};

/// A daylight saving window relative to standard time: an explicit
/// daylight-time offset east of UTC plus the rules for entering and leaving
/// daylight saving time.
#[derive(Clone, Copy)]
pub(crate) struct DstRule<'a> {
    /// The daylight-saving abbreviation, such as `EDT`.
    pub(crate) name: &'a str,
    /// Daylight-saving offset east of UTC, in seconds.
    pub(crate) east: i32,
    /// `(rule, local time)` at which daylight saving time starts.
    pub(crate) start: (Rule, i32),
    /// `(rule, local time)` at which daylight saving time ends.
    pub(crate) end: (Rule, i32),
}

/// A whole-year daylight saving transition rule day of the POSIX `TZ`
/// grammar, that is one of the `Jn` / `n` / `Mm.w.d` day forms.
///
/// # Packed encoding
///
/// The embedded database stores day rules as one `u16` each; the encoding is
/// duplicated by `tools/gen_tzdb.rs` and must remain in sync with it:
///
/// * bits 14..16 select the rule kind (the [`Rule`] variants in order
///   `JulianNoLeap`, `Julian`, `MonthWeekDay`; any other kind decodes as
///   `JulianNoLeap` and is simply never stored),
/// * for the two Julian forms the day number occupies the low bits, and
/// * for `Mm.w.d` the month occupies bits 10..14, the week (1..=5, where 5
///   means "last of its weekday") bits 7..10 and the weekday (Sunday = 0)
///   bits 4..7.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Rule {
    /// `Jn`: day `n` of the year, not counting February 29.
    JulianNoLeap(u16),
    /// `n`: zero-based day of the year, counting February 29.
    Julian(u16),
    /// `Mm.w.d`: the `d`-th weekday of the `w`-th week of month `m`.
    MonthWeekDay { month: u8, week: u8, day: u8 },
}

/// Decodes the packed day encoding documented on [`Rule`].
#[allow(dead_code)]
pub(crate) fn daylight_rule_from_packed_day(day: u16) -> Rule {
    match day >> 14 {
        1 => Rule::Julian(day & 0x3FFF),
        2 => Rule::MonthWeekDay {
            month: ((day >> 10) & 0xF) as u8,
            week: ((day >> 7) & 0x7) as u8,
            day: ((day >> 4) & 0x7) as u8,
        },
        _ => Rule::JulianNoLeap(day & 0x3FFF),
    }
}

/// Evaluates a daylight saving rule at `seconds`.
///
/// This mirrors what a POSIX `TZ` footer means: standard time applies
/// except between the two rule instants, each computed in the local time of
/// the zone that takes effect on the other side of the transition. The
/// returned abbreviation is `rule.name` while daylight saving time is in
/// effect and `standard_name` otherwise; both abbreviations must share the
/// returned lifetime (callers pass two `'static` names or the same
/// underlying string slice, as [`PosixTz::lookup`] does with its footer).
pub(crate) fn evaluate_daylight<'a>(
    rule: DstRule<'a>,
    standard_east: i32,
    standard_name: &'a str,
    seconds: i64,
) -> ZoneInfo<'a> {
    let approx_local = seconds + i64::from(standard_east);
    let days = approx_local.div_euclid(SECS_PER_DAY);
    let year = crate::civil::civil_from_days(days).0;
    let jan1 = days_from_civil(year, 1, 1);
    let start = transition_instant_fast(year, jan1, rule.start.0, rule.start.1, standard_east);
    let end = transition_instant_fast(year, jan1, rule.end.0, rule.end.1, rule.east);

    let in_dst = if start <= end {
        seconds >= start && seconds < end
    } else {
        seconds >= start || seconds < end
    };

    if in_dst {
        ZoneInfo {
            offset: rule.east,
            is_dst: true,
            abbrev: rule.name,
        }
    } else {
        ZoneInfo {
            offset: standard_east,
            is_dst: false,
            abbrev: standard_name,
        }
    }
}

/// A parsed POSIX `TZ` string.
pub(crate) struct PosixTz<'a> {
    std_name: &'a str,
    /// Standard-time offset east of UTC, in seconds.
    std_east: i32,
    dst: Option<DstRule<'a>>,
}

fn parse_name(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    if bytes[0] == b'<' {
        let end = bytes.iter().position(|&b| b == b'>')?;
        return Some((&s[1..end], &s[end + 1..]));
    }
    let end = bytes
        .iter()
        .position(|b| !b.is_ascii_alphabetic())
        .unwrap_or(bytes.len());
    if end == 0 {
        return None;
    }
    Some((&s[..end], &s[end..]))
}

/// Parses `[+-]?hh[:mm[:ss]]` into signed seconds.
fn parse_hms(s: &str) -> Option<(i32, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut sign = 1i32;
    if i < bytes.len() && bytes[i] == b'+' {
        i += 1;
    } else if i < bytes.len() && bytes[i] == b'-' {
        sign = -1;
        i += 1;
    }
    let start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == start {
        return None;
    }
    let hours: i32 = s[start..i].parse().ok()?;
    if hours > 167 {
        return None;
    }
    let mut total = hours * 3600;
    if i < bytes.len() && bytes[i] == b':' {
        i += 1;
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i - start != 2 {
            return None;
        }
        let minutes: i32 = s[start..i].parse().ok()?;
        if minutes > 59 {
            return None;
        }
        total += minutes * 60;
        if i < bytes.len() && bytes[i] == b':' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i - start != 2 {
                return None;
            }
            let seconds: i32 = s[start..i].parse().ok()?;
            if seconds > 59 {
                return None;
            }
            total += seconds;
        }
    }
    Some((sign * total, &s[i..]))
}

fn parse_rule(s: &str) -> Option<(Rule, i32, &str)> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let (rule, rest) = if bytes[0] == b'J' {
        let (n, rest) = parse_digits(&s[1..])?;
        if !(1..=365).contains(&n) {
            return None;
        }
        (Rule::JulianNoLeap(n as u16), rest)
    } else if bytes[0] == b'M' {
        let (month, rest) = parse_digits(&s[1..])?;
        let rest = rest.strip_prefix('.')?;
        let (week, rest) = parse_digits(rest)?;
        let rest = rest.strip_prefix('.')?;
        let (day, rest) = parse_digits(rest)?;
        if !(1..=12).contains(&month) || !(1..=5).contains(&week) || day > 6 {
            return None;
        }
        (
            Rule::MonthWeekDay {
                month: month as u8,
                week: week as u8,
                day: day as u8,
            },
            rest,
        )
    } else {
        let (n, rest) = parse_digits(s)?;
        if n > 365 {
            return None;
        }
        (Rule::Julian(n as u16), rest)
    };
    if let Some(rest) = rest.strip_prefix('/') {
        let (time, rest) = parse_hms(rest)?;
        Some((rule, time, rest))
    } else {
        Some((rule, 2 * 3600, rest))
    }
}

fn parse_digits(s: &str) -> Option<(i32, &str)> {
    let bytes = s.as_bytes();
    let end = bytes.iter().position(|b| !b.is_ascii_digit()).unwrap_or(bytes.len());
    if end == 0 {
        return None;
    }
    Some((s[..end].parse().ok()?, &s[end..]))
}

impl<'a> PosixTz<'a> {
    /// Parses a POSIX `TZ` string.
    pub(crate) fn parse(s: &'a str) -> Option<PosixTz<'a>> {
        let (std_name, rest) = parse_name(s)?;
        let (std_west, rest) = parse_hms(rest)?;
        let std_east = -std_west;

        let bytes = rest.as_bytes();
        if rest.is_empty() || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'<') {
            return Some(PosixTz {
                std_name,
                std_east,
                dst: None,
            });
        }

        let (dst_name, rest) = parse_name(rest)?;
        let bytes = rest.as_bytes();
        let (dst_east, rest) =
            if !rest.is_empty() && (bytes[0].is_ascii_digit() || bytes[0] == b'+' || bytes[0] == b'-') {
                let (dst_west, rest) = parse_hms(rest)?;
                (-dst_west, rest)
            } else {
                (std_east + 3600, rest)
            };

        let (start, end) = if let Some(rest) = rest.strip_prefix(',') {
            let (start_rule, start_time, rest) = parse_rule(rest)?;
            let (end_rule, end_time, rest) = parse_rule(rest.strip_prefix(',')?)?;
            if !rest.is_empty() {
                return None;
            }
            ((start_rule, start_time), (end_rule, end_time))
        } else if rest.is_empty() {
            // The POSIX standard leaves the default rules implementation
            // defined; the common (US) rules are a reasonable choice.
            (
                (
                    Rule::MonthWeekDay {
                        month: 3,
                        week: 2,
                        day: 0,
                    },
                    2 * 3600,
                ),
                (
                    Rule::MonthWeekDay {
                        month: 11,
                        week: 1,
                        day: 0,
                    },
                    2 * 3600,
                ),
            )
        } else {
            return None;
        };

        Some(PosixTz {
            std_name,
            std_east,
            dst: Some(DstRule {
                name: dst_name,
                east: dst_east,
                start,
                end,
            }),
        })
    }

    /// Returns the local time in effect at the given Unix timestamp, assuming
    /// the POSIX rule applies.
    pub(crate) fn lookup(&self, secs: i64) -> ZoneInfo<'a> {
        let Some(dst) = &self.dst else {
            return ZoneInfo {
                offset: self.std_east,
                is_dst: false,
                abbrev: self.std_name,
            };
        };
        evaluate_daylight(
            DstRule {
                name: dst.name,
                east: dst.east,
                start: dst.start,
                end: dst.end,
            },
            self.std_east,
            self.std_name,
            secs,
        )
    }
}

/// Days before the first of each month in a non-leap year.
const DAYS_BEFORE_MONTH: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// Days per month, common and leap years.
const DAYS_PER_MONTH: [[u8; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

/// The zero-based day of the year on which `rule` falls in `year`.
fn rule_day_of_year(rule: Rule, year: i32, jan1: i64) -> i32 {
    match rule {
        Rule::JulianNoLeap(n) => {
            let mut ordinal = i32::from(n) - 1;
            if is_leap(year) && n >= 60 {
                ordinal += 1;
            }
            ordinal
        }
        Rule::Julian(n) => i32::from(n),
        Rule::MonthWeekDay {
            month,
            week,
            day,
        } => {
            let month0 = usize::from(month) - 1;
            let leap = is_leap(year);
            let first_doy = DAYS_BEFORE_MONTH[month0] + i32::from(leap && month > 2);
            // `weekday_from_days` inlined: Monday = 0.
            let first_weekday = (jan1 + i64::from(first_doy) + 3).rem_euclid(7) as i32;
            let target = (i32::from(day) + 6) % 7; // POSIX Sunday = 0 -> Monday = 0
            let mut dom = 1 + (target - first_weekday).rem_euclid(7) + (i32::from(week) - 1) * 7;
            let last = i32::from(DAYS_PER_MONTH[usize::from(leap)][month0]);
            if week == 5 && dom > last {
                dom -= 7;
            }
            first_doy + dom - 1
        }
    }
}

/// The transition instant of `rule`, given the first day of `year` as `jan1`
/// (that is, `days_from_civil(year, 1, 1)`), without converting through civil
/// fields again.
fn transition_instant_fast(year: i32, jan1: i64, rule: Rule, time: i32, offset_before: i32) -> i64 {
    let doy = rule_day_of_year(rule, year, jan1);
    (jan1 + i64::from(doy)) * SECS_PER_DAY + i64::from(time) - i64::from(offset_before)
}
