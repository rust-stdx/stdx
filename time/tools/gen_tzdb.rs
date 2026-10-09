// Standalone generator for the embedded IANA time zone database.
//
// Reads a directory of compiled TZif files (for example the output of
// `zic -b slim tzdata.zi`), deduplicates identical files, and writes:
//
// * a raw `tzdb.bin` blob containing every unique TZif file, and
// * a `tzdb_data.rs` source file holding the pre-parsed runtime rules used by
//   the `time` crate (see the module documentation in `time/tzdb.rs` for the
//   encoding; it is duplicated here by necessity).
//
// The blob and the legacy index are compiled into the library only in test
// builds, where `timezone.rs::parity_check` cross-checks the pre-parsed engine
// against the blob-based evaluation on every zone-offset lookup, so a
// regeneration that disagrees in any way fails loudly during tests.
//
// The generator also materializes each zone's POSIX footer rule as explicit
// transitions up to a rolling horizon (the release year plus 50), so lookups
// in the near future are a plain binary search; the footer still governs
// beyond the horizon.
//
// The release version in the output is taken from the `# version <release>`
// header comment of the `tzdata.zi` input, so the compiled database and the
// advertised release cannot drift apart.
//
// Usage (run from the repository root):
//
//     rustc -O -o /tmp/stdx-gen-tzdb time/tools/gen_tzdb.rs
//     /tmp/stdx-gen-tzdb tzdata.zi /dir/with/zic/output time/tzdb.bin time/tzdb_data.rs

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Calendar helpers (mirrors of `time/src/civil.rs`)
// ---------------------------------------------------------------------------

const SECS_PER_DAY: i64 = 86_400;

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let year = year as i64 - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month as i64;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(days: i64) -> (i32, u8, u8) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - (146_097 - 1) } / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
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

// ---------------------------------------------------------------------------
// POSIX TZ footer rules (a mirror of `time/src/posix.rs`; the day-rule
// packing must match the encoding that `time/src/posix.rs` documents)
// ---------------------------------------------------------------------------

const RULE_JNOLEAP: u16 = 0 << 14;
const RULE_JULIAN: u16 = 1 << 14;
const RULE_MWD: u16 = 2 << 14;

/// A parsed footer rule.
struct FooterRule {
    /// Standard-time offset east of UTC, in seconds.
    standard: i32,
    standard_abbrev: String,
    /// The daylight part: `(daylight offset, abbreviation, window)`.
    dst: Option<(i32, String, DstWindow)>,
}

#[derive(Clone, Copy)]
struct DstWindow {
    start: (u16, i32),
    end: (u16, i32),
}

fn parse_footer(text: &str) -> Option<FooterRule> {
    let (standard_name, rest) = parse_name(text)?;
    let (standard_west, rest) = parse_hms(rest)?;
    let standard = -standard_west;

    let bytes = rest.as_bytes();
    if rest.is_empty() || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'<') {
        return Some(FooterRule {
            standard,
            standard_abbrev: standard_name.to_owned(),
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
            // The POSIX default: daylight saving time is one hour ahead of
            // standard time.
            (standard + 3600, rest)
        };

    if let Some(rest) = rest.strip_prefix(',') {
        let (start, rest) = parse_transition(rest)?;
        let rest = rest.strip_prefix(',')?;
        let (end, rest) = parse_transition(rest)?;
        if !rest.is_empty() {
            return None;
        }
        return Some(FooterRule {
            standard,
            standard_abbrev: standard_name.to_owned(),
            dst: Some((dst_east, dst_name.to_owned(), DstWindow { start, end })),
        });
    }
    if !rest.is_empty() {
        return None;
    }

    // The POSIX standard leaves the default rules implementation defined;
    // the common (US) rules are a reasonable choice, mirroring
    // `time/src/posix.rs`.
    Some(FooterRule {
        standard,
        standard_abbrev: standard_name.to_owned(),
        dst: Some((
            dst_east,
            dst_name.to_owned(),
            DstWindow {
                start: (rule_mwd(3, 2, 0), 2 * 3600),
                end: (rule_mwd(11, 1, 0), 2 * 3600),
            },
        )),
    })
}

fn rule_mwd(month: u8, week: u8, day: u8) -> u16 {
    RULE_MWD | u16::from(month) << 10 | u16::from(week) << 7 | u16::from(day) << 4
}

fn parse_name(input: &str) -> Option<(&str, &str)> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    if bytes[0] == b'<' {
        let end = bytes.iter().position(|&byte| byte == b'>')?;
        return Some((&input[1..end], &input[end + 1..]));
    }
    let end = bytes
        .iter()
        .position(|&byte| !byte.is_ascii_alphabetic())
        .unwrap_or(bytes.len());
    if end == 0 {
        return None;
    }
    Some((&input[..end], &input[end..]))
}

fn parse_hms(input: &str) -> Option<(i32, &str)> {
    let bytes = input.as_bytes();
    let mut index = 0usize;
    let mut sign = 1i32;
    match bytes.first() {
        Some(b'+') => index = 1,
        Some(b'-') => {
            sign = -1;
            index = 1;
        }
        _ => {}
    }
    let start = index;
    while index < bytes.len() && bytes[index].is_ascii_digit() {
        index += 1;
    }
    if index == start {
        return None;
    }
    let hours: i32 = input[start..index].parse().ok()?;
    if hours > 167 {
        return None;
    }
    let mut total = hours * 3600;
    if bytes.get(index) == Some(&b':') {
        index += 1;
        let start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index - start != 2 {
            return None;
        }
        let minutes: i32 = input[start..index].parse().ok()?;
        if minutes > 59 {
            return None;
        }
        total += minutes * 60;
        if bytes.get(index) == Some(&b':') {
            index += 1;
            let start = index;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
            if index - start != 2 {
                return None;
            }
            let seconds: i32 = input[start..index].parse().ok()?;
            if seconds > 59 {
                return None;
            }
            total += seconds;
        }
    }
    Some((sign * total, &input[index..]))
}

/// Parses a transition-day rule (`Jn`, `n` or `Mm.w.d`) plus an optional
/// local time (defaulting to 02:00:00 when no `/time` follows).
fn parse_transition(input: &str) -> Option<((u16, i32), &str)> {
    let day;
    let rest;
    match input.as_bytes().first() {
        Some(b'J') => {
            let (number, remainder) = parse_digits(&input[1..])?;
            if !(1..=365).contains(&number) {
                return None;
            }
            day = RULE_JNOLEAP | number as u16;
            rest = remainder;
        }
        Some(b'M') => {
            let (month, remainder) = parse_digits(&input[1..])?;
            let remainder = remainder.strip_prefix('.')?;
            let (week, remainder) = parse_digits(remainder)?;
            let remainder = remainder.strip_prefix('.')?;
            let (day_of_week, remainder) = parse_digits(remainder)?;
            if !(1..=12).contains(&month) || !(1..=5).contains(&week) || day_of_week > 6 {
                return None;
            }
            day = RULE_MWD
                | (month as u16) << 10
                | (week as u16) << 7
                | (day_of_week as u16) << 4;
            rest = remainder;
        }
        Some(byte) if byte.is_ascii_digit() => {
            let (number, remainder) = parse_digits(input)?;
            if number > 365 {
                return None;
            }
            day = RULE_JULIAN | number as u16;
            rest = remainder;
        }
        _ => return None,
    }
    if let Some(remainder) = rest.strip_prefix('/') {
        let (time, remainder) = parse_hms(remainder)?;
        Some(((day, time), remainder))
    } else {
        Some(((day, 2 * 3600), rest))
    }
}

fn parse_digits(input: &str) -> Option<(i32, &str)> {
    let bytes = input.as_bytes();
    let end = bytes
        .iter()
        .position(|&byte| !byte.is_ascii_digit())
        .unwrap_or(bytes.len());
    if end == 0 {
        return None;
    }
    Some((input[..end].parse().ok()?, &input[end..]))
}

// ---------------------------------------------------------------------------
// TZif reading (RFC 8536), mirroring `time/src/tzif.rs`
// ---------------------------------------------------------------------------

struct ZoneType {
    offset: i32,
    is_dst: bool,
    abbreviation: String,
}

struct RawZone {
    transitions: Vec<i64>,
    indices: Vec<u8>,
    types: Vec<ZoneType>,
    footer: String,
}

fn be_u32(data: &[u8], offset: usize) -> Option<u32> {
    data.get(offset..offset.checked_add(4)?)?
        .try_into()
        .ok()
        .map(u32::from_be_bytes)
}

fn be_i32(data: &[u8], offset: usize) -> Option<i32> {
    be_u32(data, offset).map(|value| value as i32)
}

fn be_i64(data: &[u8], offset: usize) -> Option<i64> {
    data.get(offset..offset.checked_add(8)?)?
        .try_into()
        .ok()
        .map(i64::from_be_bytes)
}

struct Header {
    leapcnt: usize,
    timecnt: usize,
    typecnt: usize,
    charcnt: usize,
    isstdcnt: usize,
    isutcnt: usize,
}

impl Header {
    fn block_len(&self, time_size: usize) -> Option<usize> {
        Some(
            self.timecnt
                .checked_mul(time_size)?
                .checked_add(self.timecnt)?
                .checked_add(self.typecnt.checked_mul(6)?)?
                .checked_add(self.charcnt)?
                .checked_add(self.leapcnt.checked_mul(time_size.checked_add(4)?)?)?
                .checked_add(self.isstdcnt)?
                .checked_add(self.isutcnt)?,
        )
    }
}

fn read_header(data: &[u8], offset: usize) -> Option<Header> {
    Some(Header {
        isutcnt: be_u32(data, offset.checked_add(20)?)? as usize,
        isstdcnt: be_u32(data, offset.checked_add(24)?)? as usize,
        leapcnt: be_u32(data, offset.checked_add(28)?)? as usize,
        timecnt: be_u32(data, offset.checked_add(32)?)? as usize,
        typecnt: be_u32(data, offset.checked_add(36)?)? as usize,
        charcnt: be_u32(data, offset.checked_add(40)?)? as usize,
    })
}

fn parse_tzif(data: &[u8]) -> Option<RawZone> {
    if data.len() < 44 || &data[..4] != b"TZif" {
        return None;
    }
    let version = data[4];

    // For version 2+ files, skip the legacy 32-bit block and use the 64-bit
    // block that follows.
    let (header_offset, time_size, version) = if version >= b'2' {
        let first = read_header(data, 0)?;
        let first_end = first
            .block_len(4)
            .and_then(|length| 44usize.checked_add(length))?;
        if data.len() < first_end.checked_add(44)? {
            return None;
        }
        if &data[first_end..first_end + 4] != b"TZif" {
            return None;
        }
        (first_end, 8usize, version)
    } else {
        (0usize, 4usize, b'1')
    };

    let header = read_header(data, header_offset)?;
    if header.typecnt == 0 || header.charcnt == 0 {
        return None;
    }
    let block_offset = header_offset.checked_add(44)?;
    let block_len = header.block_len(time_size)?;
    let block_end = block_offset.checked_add(block_len)?;
    if data.len() < block_end {
        return None;
    }
    let block = &data[block_offset..block_end];

    let mut transitions = Vec::with_capacity(header.timecnt);
    for index in 0..header.timecnt {
        let position = index.checked_mul(time_size)?;
        let value = if time_size == 8 {
            be_i64(block, position)?
        } else {
            i64::from(be_i32(block, position)?)
        };
        transitions.push(value);
    }
    let indices_start = header.timecnt.checked_mul(time_size)?;
    let indices = block
        .get(indices_start..indices_start.checked_add(header.timecnt)?)?
        .to_vec();
    let types_start = indices_start + header.timecnt;
    let types_end = types_start + header.typecnt * 6;
    let types_block = block.get(types_start..types_end)?;
    let designations = block.get(types_end..types_end.checked_add(header.charcnt)?)?;

    let mut types = Vec::with_capacity(header.typecnt);
    for index in 0..header.typecnt {
        let base = index * 6;
        let offset = be_i32(types_block, base)?;
        let is_dst = types_block[base + 4] != 0;
        let designation = types_block[base + 5] as usize;
        let bytes = designations.get(designation..)?;
        let end = bytes.iter().position(|&byte| byte == 0)?;
        types.push(ZoneType {
            offset,
            is_dst,
            abbreviation: String::from_utf8_lossy(&bytes[..end]).into_owned(),
        });
    }

    // Keep these validation checks in sync with the library's `tzif.rs`: the
    // transitions must be strictly increasing and every transition type index
    // must point at a real type, so the compiled encoding is always total.
    for pair in transitions.windows(2) {
        if pair[0] >= pair[1] {
            return None;
        }
    }
    for &type_index in &indices {
        if usize::from(type_index) >= types.len() {
            return None;
        }
    }

    // The footer is present in version 2+ files: `\n<TZ string>\n`.
    let footer = if version >= b'2' {
        let rest = data.get(block_end..)?;
        let content = rest.strip_prefix(b"\n")?;
        let end = content.iter().position(|&byte| byte == b'\n')?;
        String::from_utf8_lossy(&content[..end]).into_owned()
    } else {
        String::new()
    };

    Some(RawZone {
        transitions,
        indices,
        types,
        footer,
    })
}

// ---------------------------------------------------------------------------
// Compilation into the library's encoding
// ---------------------------------------------------------------------------

/// A zone in the library's encoding, ready to be emitted.
#[derive(Clone)]
struct CompiledZone {
    initial: CompiledRecord,
    fences: Vec<CompiledFence>,
    future: CompiledFuture,
}

#[derive(Clone)]
struct CompiledRecord {
    offset: i32,
    is_dst: bool,
    abbreviation: String,
}

#[derive(Clone)]
struct CompiledFence {
    start: i64,
    record: CompiledRecord,
}

#[derive(Clone)]
enum CompiledFuture {
    Fixed(CompiledRecord),
    Dst {
        standard: CompiledRecord,
        daylight: CompiledRecord,
        start: (u16, i32),
        end: (u16, i32),
    },
}

/// Local-time information at one instant (the legacy evaluation result).
#[derive(Clone, PartialEq, Eq, Debug)]
struct Reference {
    offset: i32,
    is_dst: bool,
    abbreviation: String,
}

/// Resolves a TZif file into compiled rules.
fn compile_zone(raw: &RawZone) -> Result<CompiledZone, String> {
    // Transitions must be ascending for the binary search to be sound.
    for pair in raw.transitions.windows(2) {
        if pair[0] >= pair[1] {
            return Err("transitions are not strictly increasing".to_owned());
        }
    }

    let footer = if raw.footer.is_empty() {
        None
    } else {
        match parse_footer(&raw.footer) {
            Some(rule) => Some(rule),
            None => {
                eprintln!("warning: unparseable POSIX footer {:?}", raw.footer);
                None
            }
        }
    };

    // Fences encode every transition; the record is the transition's
    // destination type.
    let fence_count = raw.transitions.len();
    let mut fences = Vec::with_capacity(fence_count);
    for (index, &start) in raw.transitions.iter().enumerate() {
        let record = &raw.types[raw.indices[index] as usize];
        fences.push(CompiledFence {
            start,
            record: CompiledRecord {
                offset: record.offset,
                is_dst: record.is_dst,
                abbreviation: record.abbreviation.clone(),
            },
        });
    }

    // The future rule mirrors the runtime TZif reader: on and after the last
    // fence the footer governs; when there is no usable footer, the record
    // of the last fence applies (or the initial type, for fence-less zones).
    let future = match footer {
        Some(FooterRule {
            standard,
            standard_abbrev,
            dst: None,
        }) => CompiledFuture::Fixed(CompiledRecord {
            offset: standard,
            is_dst: false,
            abbreviation: standard_abbrev,
        }),
        Some(FooterRule {
            standard,
            standard_abbrev,
            dst: Some((daylight, daylight_abbrev, window)),
        }) => CompiledFuture::Dst {
            standard: CompiledRecord {
                offset: standard,
                is_dst: false,
                abbreviation: standard_abbrev,
            },
            daylight: CompiledRecord {
                offset: daylight,
                is_dst: true,
                abbreviation: daylight_abbrev,
            },
            start: window.start,
            end: window.end,
        },
        None => {
            let remaining = if fence_count >= 1 {
                fences[fence_count - 1].record.clone()
            } else {
                let first = raw.types.first().unwrap();
                CompiledRecord {
                    offset: first.offset,
                    is_dst: first.is_dst,
                    abbreviation: first.abbreviation.clone(),
                }
            };
            CompiledFuture::Fixed(remaining)
        }
    };

    let first = raw.types.first().unwrap();
    Ok(CompiledZone {
        initial: CompiledRecord {
            offset: first.offset,
            is_dst: first.is_dst,
            abbreviation: first.abbreviation.clone(),
        },
        fences,
        future,
    })
}

// ---------------------------------------------------------------------------
// Reference evaluation (the algorithms of `time/src/tzif.rs` and
// `time/src/posix.rs`), used to cross-check the compiled form
// ---------------------------------------------------------------------------

fn reference_lookup(raw: &RawZone, seconds: i64) -> Option<Reference> {
    let footer_dst = if raw.footer.is_empty() {
        None
    } else {
        parse_footer(&raw.footer)
    };

    if raw.transitions.is_empty() {
        if let Some(rule) = &footer_dst {
            return Some(reference_dst(rule, seconds));
        }
        let first = raw.types.first()?;
        return Some(Reference {
            offset: first.offset,
            is_dst: first.is_dst,
            abbreviation: first.abbreviation.clone(),
        });
    }

    if seconds < raw.transitions[0] {
        let first = raw.types.first()?;
        return Some(Reference {
            offset: first.offset,
            is_dst: first.is_dst,
            abbreviation: first.abbreviation.clone(),
        });
    }

    let count = raw.transitions.len();
    let mut low = 0usize;
    let mut high = count;
    while low + 1 < high {
        let middle = (low + high) / 2;
        if raw.transitions[middle] <= seconds {
            low = middle;
        } else {
            high = middle;
        }
    }

    if low == count - 1 {
        if let Some(rule) = &footer_dst {
            return Some(reference_dst(rule, seconds));
        }
    }

    let index = raw.indices[low] as usize;
    let kind = raw.types.get(index)?;
    Some(Reference {
        offset: kind.offset,
        is_dst: kind.is_dst,
        abbreviation: kind.abbreviation.clone(),
    })
}

/// Evaluates a parsed footer rule the way `posix::PosixTz::lookup` does.
fn reference_dst(rule: &FooterRule, seconds: i64) -> Reference {
    let Some((daylight, daylight_abbrev, window)) = &rule.dst else {
        return Reference {
            offset: rule.standard,
            is_dst: false,
            abbreviation: rule.standard_abbrev.clone(),
        };
    };

    let approximate_local = seconds + i64::from(rule.standard);
    let year = civil_from_days(approximate_local.div_euclid(SECS_PER_DAY)).0;
    let jan1 = days_from_civil(year, 1, 1);
    let start = transition_instant_fast(year, jan1, window.start.0, window.start.1, rule.standard);
    let end = transition_instant_fast(year, jan1, window.end.0, window.end.1, *daylight);

    let inside = if start <= end {
        seconds >= start && seconds < end
    } else {
        seconds >= start || seconds < end
    };

    if inside {
        Reference {
            offset: *daylight,
            is_dst: true,
            abbreviation: daylight_abbrev.clone(),
        }
    } else {
        Reference {
            offset: rule.standard,
            is_dst: false,
            abbreviation: rule.standard_abbrev.clone(),
        }
    }
}

/// Days before the first of each month in a non-leap year.
const DAYS_BEFORE_MONTH: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// Days per month, common and leap years.
const DAYS_PER_MONTH: [[u8; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

/// The zero-based day of the year on which a packed footer rule falls; mirrors
/// the implementation in `time/src/posix.rs`.
fn rule_day_of_year(packed_day: u16, year: i32, jan1: i64) -> i32 {
    match packed_day >> 14 {
        0 => {
            let number = (packed_day & 0x3FFF) as i32;
            let mut ordinal = number - 1;
            if is_leap(year) && number >= 60 {
                ordinal += 1;
            }
            ordinal
        }
        1 => (packed_day & 0x3FFF) as i32,
        _ => {
            let month = ((packed_day >> 10) & 0xF) as usize;
            let week = ((packed_day >> 7) & 0x7) as i32;
            let day_of_week = ((packed_day >> 4) & 0x7) as i32;
            let month0 = month - 1;
            let leap = is_leap(year);
            let first_doy = DAYS_BEFORE_MONTH[month0] + i32::from(leap && month > 2);
            let first_weekday = (jan1 + i64::from(first_doy) + 3).rem_euclid(7) as i32;
            let target = (day_of_week + 6) % 7;
            let mut dom = 1 + (target - first_weekday).rem_euclid(7) + (week - 1) * 7;
            let last = i32::from(DAYS_PER_MONTH[usize::from(leap)][month0]);
            if week == 5 && dom > last {
                dom -= 7;
            }
            first_doy + dom - 1
        }
    }
}

/// The transition instant of a packed rule; `jan1` must be
/// `days_from_civil(year, 1, 1)`.
fn transition_instant_fast(year: i32, jan1: i64, packed_day: u16, time: i32, offset_before: i32) -> i64 {
    let doy = rule_day_of_year(packed_day, year, jan1);
    (jan1 + i64::from(doy)) * SECS_PER_DAY + i64::from(time) - i64::from(offset_before)
}

// ---------------------------------------------------------------------------
// Cross-checking
// ---------------------------------------------------------------------------

/// Evaluates a compiled zone exactly as the library's static engine will.
fn compiled_lookup(zone: &CompiledZone, seconds: i64) -> Reference {
    let fences = &zone.fences;
    if fences.is_empty() {
        return compiled_future(&zone.future, seconds);
    }
    // The last fence with start <= seconds.
    let mut low = 0usize;
    let mut high = fences.len();
    while low + 1 < high {
        let middle = (low + high) / 2;
        if fences[middle].start <= seconds {
            low = middle;
        } else {
            high = middle;
        }
    }
    if fences[low].start > seconds {
        return from_record(&zone.initial);
    }
    if low == fences.len() - 1 {
        // From the last fence onward the future rule applies.
        return compiled_future(&zone.future, seconds);
    }
    from_record(&fences[low].record)
}

fn compiled_future(future: &CompiledFuture, seconds: i64) -> Reference {
    match future {
        CompiledFuture::Fixed(record) => from_record(record),
        CompiledFuture::Dst {
            standard,
            daylight,
            start,
            end,
        } => {
            let approx_local = seconds + i64::from(standard.offset);
            let year = civil_from_days(approx_local.div_euclid(SECS_PER_DAY)).0;
            let jan1 = days_from_civil(year, 1, 1);
            let start_instant =
                transition_instant_fast(year, jan1, start.0, start.1, standard.offset);
            let end_instant =
                transition_instant_fast(year, jan1, end.0, end.1, daylight.offset);
            let inside = if start_instant <= end_instant {
                seconds >= start_instant && seconds < end_instant
            } else {
                seconds >= start_instant || seconds < end_instant
            };
            if inside {
                from_record(daylight)
            } else {
                from_record(standard)
            }
        }
    }
}

fn from_record(record: &CompiledRecord) -> Reference {
    Reference {
        offset: record.offset,
        is_dst: record.is_dst,
        abbreviation: record.abbreviation.clone(),
    }
}

/// The year encoded in an IANA release string such as `"2026b"`.
fn release_year(release: &str) -> i32 {
    let digits: String = release.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits
        .parse()
        .unwrap_or_else(|_| panic!("cannot parse a year from release {release:?}"))
}

/// Appends explicit transitions generated from the POSIX footer rule up to
/// `horizon_year`, so near-future lookups are a plain binary search instead of
/// re-evaluating the rule. Beyond the horizon the footer still governs.
fn extend_horizon(zone: &mut CompiledZone, horizon_year: i32) {
    let (standard, daylight, start, end) = match &zone.future {
        CompiledFuture::Dst { standard, daylight, start, end } => {
            (standard.clone(), daylight.clone(), *start, *end)
        }
        CompiledFuture::Fixed(_) => return,
    };
    let last = zone.fences.last().map(|fence| fence.start);
    let first_year = last
        .map(|start| civil_from_days(start.div_euclid(SECS_PER_DAY)).0)
        .unwrap_or(horizon_year);
    let mut extra: Vec<CompiledFence> = Vec::new();
    for year in first_year..=horizon_year {
        // `transition_instant_fast` subtracts the offset in effect just before
        // the change, exactly as `posix::evaluate_daylight` does.
        let jan1 = days_from_civil(year, 1, 1);
        let starts = transition_instant_fast(year, jan1, start.0, start.1, standard.offset);
        let ends = transition_instant_fast(year, jan1, end.0, end.1, daylight.offset);
        for (instant, record) in [(starts, &daylight), (ends, &standard)] {
            if last.is_none_or(|last| instant > last) {
                extra.push(CompiledFence {
                    start: instant,
                    record: record.clone(),
                });
            }
        }
    }
    extra.sort_by_key(|fence| fence.start);
    extra.dedup_by_key(|fence| fence.start);
    zone.fences.extend(extra);
}

/// Asserts that the compiled encoding evaluates identically to the raw one.
fn self_check(raw: &RawZone, compiled: &CompiledZone, name: &str) {
    let check = |seconds: i64| {
        let expected = reference_lookup(raw, seconds)
            .unwrap_or_else(|| panic!("reference lookup failed for {name} at {seconds}"));
        let actual = compiled_lookup(compiled, seconds);
        if expected != actual {
            panic!("self-check failed for {name} at {seconds}: {expected:?} != {actual:?}");
        }
    };

    // Every explicit transition, one second either side, including the
    // boundary where the footer takes over.
    for &transition in &raw.transitions {
        check(transition - 1);
        check(transition);
        check(transition + 1);
    }

    // Sampled years, covering the early rules and the future footer.
    let years = [1830, 1880, 1900, 1920, 1950, 1970, 1990, 2010, 2030, 2050, 2075, 2100, 2200];
    for year in years {
        for day in [1i64, 100, 200, 300] {
            check(days_from_civil(year, 1, 1) * SECS_PER_DAY + day * SECS_PER_DAY);
        }
    }
}

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

fn walk(dir: &Path, root: &Path, out: &mut Vec<(String, Vec<u8>)>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, root, out);
        } else {
            let bytes = fs::read(&path).unwrap();
            if bytes.len() < 4 || &bytes[..4] != b"TZif" {
                continue;
            }
            let relative = path.strip_prefix(root).unwrap();
            let name = relative
                .components()
                .map(|component| component.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<_>>()
                .join("/");
            out.push((name, bytes));
        }
    }
}

/// Extracts the release version from the `tzdata.zi` source.
fn parse_release(path: &Path) -> String {
    let text = fs
        ::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read {path:?}: {error}"));
    for line in text.lines() {
        if let Some(rest) = line.trim().strip_prefix("# version") {
            let version = rest.trim();
            if !version.is_empty() {
                return version.to_owned();
            }
        }
    }
    panic!("could not find a `# version <release>` comment in {path:?}");
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 5 {
        eprintln!("usage: gen_tzdb <tzdata.zi> <zic-output-dir> <tzdb.bin> <tzdb_data.rs>");
        std::process::exit(2);
    }
    let release = parse_release(Path::new(&arguments[1]));
    let horizon_year = release_year(&release) + 50;
    let tzdir = PathBuf::from(&arguments[2]);
    let bin_path = PathBuf::from(&arguments[3]);
    let data_path = PathBuf::from(&arguments[4]);

    let mut files = Vec::new();
    walk(&tzdir, &tzdir, &mut files);
    if files.is_empty() {
        panic!("no TZif files found under {tzdir:?}");
    }

    // Deduplicate identical zone rules by TZif content; identical content
    // compiles identically, so aliases share one emitted static.
    let mut blob: Vec<u8> = Vec::new();
    let mut offsets: Vec<u32> = Vec::new();
    let mut lengths: Vec<u32> = Vec::new();
    let mut by_content: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut zones: Vec<(String, CompiledZone)> = Vec::new();
    let mut legacy: Vec<(String, u32, u32)> = Vec::new();
    let mut entries: Vec<(String, usize)> = Vec::new();
    for (name, bytes) in files {
        let index = match by_content.get(&bytes) {
            Some(&index) => index,
            None => {
                let raw = parse_tzif(&bytes)
                    .unwrap_or_else(|| panic!("cannot parse {name:?}: the TZif file is malformed"));
                let compiled = compile_zone(&raw)
                    .unwrap_or_else(|error| panic!("cannot compile {name:?}: {error}"));
                let mut compiled = compiled;
                extend_horizon(&mut compiled, horizon_year);
                self_check(&raw, &compiled, &name);
                let offset = blob.len() as u32;
                blob.extend_from_slice(&bytes);
                let index = zones.len();
                offsets.push(offset);
                lengths.push(bytes.len() as u32);
                zones.push((name.clone(), compiled));
                by_content.insert(bytes, index);
                index
            }
        };
        // Aliases share the unique zone's blob range, so the recorded length
        // is the zone's own TZif size rather than the rest of the blob.
        legacy.push((name.clone(), offsets[index], lengths[index]));
        entries.push((name, index));
    }

    // The shared abbreviation table: every record's abbreviation gets a
    // stable `u8` id.
    let abbreviations = AbbrevTable::new(&zones);
    let generated = emit(&data_path, &release, &zones, &entries, &legacy, &abbreviations);
    fs::write(&bin_path, &blob).unwrap();

    let fence_total: usize = zones
        .iter()
        .map(|(_, zone)| zone.fences.len())
        .sum();
    eprintln!(
        "wrote {} zones ({} unique, {} fences, {} abbreviations); blob {} bytes, data {} bytes (release {})",
        legacy.len(),
        zones.len(),
        fence_total,
        abbreviations.names.len(),
        blob.len(),
        generated,
        release,
    );
}

/// The interning table of abbreviation strings shared by every record.
struct AbbrevTable {
    names: Vec<String>,
    ids: HashMap<String, u8>,
}

impl AbbrevTable {
    fn new(zones: &[(String, CompiledZone)]) -> AbbrevTable {
        let mut table = AbbrevTable {
            names: Vec::new(),
            ids: HashMap::new(),
        };
        for (_, zone) in zones {
            table.intern(&zone.initial.abbreviation);
            for fence in &zone.fences {
                table.intern(&fence.record.abbreviation);
            }
            match &zone.future {
                CompiledFuture::Fixed(record) => {
                    table.intern(&record.abbreviation);
                }
                CompiledFuture::Dst { standard, daylight, .. } => {
                    table.intern(&standard.abbreviation);
                    table.intern(&daylight.abbreviation);
                }
            }
        }
        table
    }

    fn intern(&mut self, abbreviation: &str) -> u8 {
        if let Some(&id) = self.ids.get(abbreviation) {
            return id;
        }
        let id = u8::try_from(self.ids.len())
            .expect("more than 255 distinct abbreviations");
        self.ids.insert(abbreviation.to_owned(), id);
        self.names.push(abbreviation.to_owned());
        id
    }

    fn id(&self, abbreviation: &str) -> u8 {
        self.ids[abbreviation]
    }
}

/// Writes `tzdb_data.rs`; returns the number of bytes written.
fn emit(
    data_path: &Path,
    release: &str,
    zones: &[(String, CompiledZone)],
    entries: &[(String, usize)],
    legacy: &[(String, u32, u32)],
    abbreviations: &AbbrevTable,
) -> usize {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "// @generated by tools/gen_tzdb.rs. Do not edit by hand.\n//\n// The IANA Time Zone Database, release {release}: every named zone's rules\n// pre-parsed from its TZif file. The encoding is documented in\n// `time/src/tzdb.rs` and duplicated by the generator; `ZONES_LEGACY` and\n// `tzdb.bin` are compiled in tests only, so test builds cross-check the\n// pre-parsed engine against the TZif reader.\n"
    );
    let _ = writeln!(out, "/// The IANA Time Zone Database release the embedded data was compiled from.\npub static TZDB_VERSION: &str = {release:?};\n");

    let _ = writeln!(out, "#[cfg(all(test, feature = \"timezone-db\"))]\nstatic BLOB: &[u8] = include_bytes!(\"tzdb.bin\");\n");

    let _ = writeln!(out, "/// Legacy index from zone name to a range of `BLOB` (kept for the test\n/// parity check; see `time/src/tzdb.rs`).\n#[cfg(all(test, feature = \"timezone-db\"))]\npub static ZONES_LEGACY: &[(&str, u32, u32)] = &[");
    for (name, offset, length) in legacy {
        let _ = writeln!(out, "    ({name:?}, {offset}, {length}),");
    }
    let _ = writeln!(out, "];\n");

    let _ = writeln!(out, "static ABBREVIATIONS: &[&str] = &[");
    for name in &abbreviations.names {
        let _ = writeln!(out, "    {name:?},");
    }
    let _ = writeln!(out, "];\n");

    // Unique zone data, in first-appearance order.
    for (index, (name, zone)) in zones.iter().enumerate() {
        let _ = writeln!(out, "static ZONE_{index}: ZoneData = ZoneData {{");
        let _ = writeln!(out, "    initial: Record {{ offset: {}, dst: {}, abbrev: {} }},", zone.initial.offset, zone.initial.is_dst, abbreviations.id(&zone.initial.abbreviation));

        // Intern this zone's transition records so each distinct record is
        // stored once and referenced by a `u8` index.
        let mut table: Vec<CompiledRecord> = Vec::new();
        let mut ids: HashMap<(i32, bool, u8), u8> = HashMap::new();
        let mut record_indices: Vec<u8> = Vec::with_capacity(zone.fences.len());
        for fence in &zone.fences {
            let key = (
                fence.record.offset,
                fence.record.is_dst,
                abbreviations.id(&fence.record.abbreviation),
            );
            let id = match ids.get(&key) {
                Some(&id) => id,
                None => {
                    let id = u8::try_from(table.len())
                        .expect("more than 255 distinct records in one zone");
                    ids.insert(key, id);
                    table.push(fence.record.clone());
                    id
                }
            };
            record_indices.push(id);
        }

        let _ = writeln!(out, "    starts: &[");
        for chunk in zone.fences.chunks(8) {
            let values: Vec<String> = chunk.iter().map(|fence| fence.start.to_string()).collect();
            let _ = writeln!(out, "        {},", values.join(", "));
        }
        let _ = writeln!(out, "    ],");

        let _ = writeln!(out, "    record_table: &[");
        for record in &table {
            let _ = writeln!(
                out,
                "        Record {{ offset: {}, dst: {}, abbrev: {} }},",
                record.offset,
                record.is_dst,
                abbreviations.id(&record.abbreviation)
            );
        }
        let _ = writeln!(out, "    ],");

        let _ = writeln!(out, "    records: &[");
        for chunk in record_indices.chunks(24) {
            let values: Vec<String> = chunk.iter().map(u8::to_string).collect();
            let _ = writeln!(out, "        {},", values.join(", "));
        }
        let _ = writeln!(out, "    ],");
        match &zone.future {
            CompiledFuture::Fixed(record) => {
                let _ = writeln!(
                    out,
                    "    future: Future::Fixed {{ record: Record {{ offset: {}, dst: {}, abbrev: {} }} }},",
                    record.offset,
                    record.is_dst,
                    abbreviations.id(&record.abbreviation)
                );
            }
            CompiledFuture::Dst { standard, daylight, start, end } => {
                let _ = writeln!(out, "    future: Future::DaylightSaving {{");
                let _ = writeln!(
                    out,
                    "        standard: Record {{ offset: {}, dst: {}, abbrev: {} }},",
                    standard.offset,
                    standard.is_dst,
                    abbreviations.id(&standard.abbreviation)
                );
                let _ = writeln!(
                    out,
                    "        daylight: Record {{ offset: {}, dst: {}, abbrev: {} }},",
                    daylight.offset,
                    daylight.is_dst,
                    abbreviations.id(&daylight.abbreviation)
                );
                let _ = writeln!(out, "        start: ({}, {}),", start.0, start.1);
                let _ = writeln!(out, "        end: ({}, {}),", end.0, end.1);
                let _ = writeln!(out, "    }},");
            }
        }
        let _ = writeln!(out, "}};\n");
        let _ = name;
    }

    // The named-zone table: every IANA name, sorted by lowercase name for
    // binary search; aliases share one `ZONE_n` static.
    let mut rows: Vec<&(String, usize)> = entries.iter().collect();
    rows.sort_by(|left, right| left.0.to_lowercase().cmp(&right.0.to_lowercase()));
    let _ = writeln!(out, "/// Every named zone, sorted by lowercase name for binary search.\npub static ZONES: &[ZoneEntry] = &[");
    for (name, zone_index) in &rows {
        let _ = writeln!(
            out,
            "    ZoneEntry {{ name: {name:?}, rules: ZoneRules::Embedded(&ZONE_{zone_index}) }},"
        );
    }
    let _ = writeln!(out, "];\n");

    fs::write(data_path, out.as_bytes()).unwrap();
    out.len()
}
