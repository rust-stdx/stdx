//! The embedded IANA time zone database.
//!
//! The database is compiled into a *pre-parsed* static form by
//! `tools/gen_tzdb.rs`, from the IANA `tzdata.zi` source run through
//! `zic -b slim`. Lookups are allocation-free binary searches over static
//! arrays, so named zones work without `std` and without `alloc`, and the
//! work per lookup is far smaller than parsing a TZif header each time.
//!
//! The generated file (included below when the `timezone-db` feature is
//! enabled) defines everything documented here:
//!
//! * [`TZDB_VERSION`] — the IANA release the data was compiled from,
//! * [`ZONES`] — one [`ZoneEntry`] per IANA zone name (canonical names and
//!   historic aliases alike, with aliases sharing one [`ZoneData`]), sorted
//!   case-insensitively for [`lookup`],
//! * `ABBREVIATIONS` — interned abbreviation strings (`"LMT"`, `"EST"`,
//!   `"EET"`, `"-05"`, …) referenced from records by a `u8` index,
//! * one `ZONE_n` static per unique set of rules, and — only when the crate is
//!   compiled for tests — `ZONES_LEGACY` and the raw `tzdb.bin` blob, kept so
//!   the test builds can assert that the pre-parsed engine and the TZif reader
//!   agree.
//!
//! # Encoding (keep in sync with `tools/gen_tzdb.rs`)
//!
//! [`ZoneData`] describes a zone as an `initial` record — the local mean
//! time before the first explicit transition — the ascending transition
//! `starts`, a small `record_table` of the distinct [`Record`]s used by this
//! zone plus one `records` index per transition, and a [`Future`] rule that
//! governs from the last transition onward. The split table keeps long
//! transition lists compact without repeating the record for every entry.
//! Together these express exactly what a TZif file encodes plus its POSIX
//! `TZ` footer. The generator also materializes the footer rule as explicit
//! transitions for a rolling horizon (the release year plus 50), so lookups
//! in the near future are a plain binary search; the footer still governs
//! beyond it. For a zone without a usable footer, the record of the last
//! transition is stored as the fixed future, which matches the fallback the
//! runtime TZif reader applies.
//!
//! # Future rules (keep in sync with `tools/gen_tzdb.rs`)
//!
//! [`Future::DaylightSaving`] is the pre-parsed POSIX footer: standard and
//! daylight records plus `(packed day, local seconds)` pairs for entering
//! and leaving daylight saving time. The packed encoding is documented on
//! `crate::posix::Rule`, which is the same encoding the generator emits.

#[cfg(feature = "timezone-db")]
use crate::posix::{DstRule, daylight_rule_from_packed_day, evaluate_daylight};
#[cfg(feature = "timezone-db")]
use crate::tzif::ZoneInfo;

#[cfg(feature = "timezone-db")]
include!("tzdb_data.rs");

/// Without the embedded database there is no release to report.
#[cfg(not(feature = "timezone-db"))]
pub static TZDB_VERSION: &str = "unknown";

/// The rules of one pre-parsed IANA zone.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ZoneData {
    /// The record in effect before the first explicit transition.
    pub(crate) initial: Record,
    /// Explicit transition instants (Unix seconds) in ascending order. Each
    /// record applies from its start until the next one, and
    /// [`ZoneData::future`] applies from the last transition onward.
    pub(crate) starts: &'static [i64],
    /// For each transition, the index into [`ZoneData::record_table`] of the
    /// record that takes effect there. Stored separately from the instants so
    /// that the (few) distinct records are not repeated for every transition.
    pub(crate) records: &'static [u8],
    /// The distinct records used by this zone's transitions.
    pub(crate) record_table: &'static [Record],
    /// The rule that governs from the last transition onward, or from the
    /// beginning for zones without explicit transitions.
    pub(crate) future: Future,
}

/// One entry of the embedded database: a zone name and its compiled rules.
#[allow(dead_code)]
pub struct ZoneEntry {
    /// The zone name; historic aliases keep their own name and share the
    /// rules of the canonical zone.
    pub(crate) name: &'static str,
    /// The compiled rules; identical zones share one static.
    pub(crate) rules: ZoneRules,
}

/// Backing store of a zone's rules: either the embedded database or a TZif
/// image provided at runtime (the system zone or a caller-supplied image).
#[allow(dead_code)]
pub enum ZoneRules {
    /// Rules pre-parsed into the embedded database.
    Embedded(&'static ZoneData),
    /// Rules kept as a parsed TZif image. The image is validated once and then
    /// kept for the process (system detection leaks the parsed value), so time
    /// zone values stay `Copy`.
    Runtime(&'static crate::tzif::Tzif<'static>),
}

/// One stretch of constant UTC offset and daylight saving state.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Record {
    /// Offset from UTC in seconds, east of Greenwich.
    pub(crate) offset: i32,
    /// Whether daylight saving time is in effect during this stretch.
    pub(crate) dst: bool,
    /// Index into the generated abbreviation table.
    pub(crate) abbrev: u8,
}

/// The rules that govern from the last explicit transition onward, mirroring
/// the POSIX `TZ` footer of a TZif file.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Future {
    /// The same record applies without end.
    Fixed { record: Record },
    /// Daylight saving time, evaluated per POSIX transition rules.
    DaylightSaving {
        /// The standard-time record.
        standard: Record,
        /// The daylight-time record.
        daylight: Record,
        /// `(packed day, local seconds)` at which daylight saving starts.
        start: (u16, i32),
        /// `(packed day, local seconds)` at which daylight saving ends.
        end: (u16, i32),
    },
}

/// Resolves an abbreviation index into the shared string table.
#[cfg(feature = "timezone-db")]
pub(crate) fn abbreviation(index: u8) -> Option<&'static str> {
    ABBREVIATIONS.get(usize::from(index)).copied()
}

#[cfg(feature = "timezone-db")]
pub(crate) fn record_info(record: Record) -> ZoneInfo<'static> {
    ZoneInfo {
        offset: record.offset,
        is_dst: record.dst,
        abbrev: abbreviation(record.abbrev).unwrap_or(""),
    }
}

impl ZoneData {
    /// Returns the record that takes effect at the transition `index`.
    ///
    /// Kept outside the `timezone-db` gate because the ambiguity classifier
    /// needs it even in builds that exclude the embedded database (where no
    /// `ZoneData` value exists, but the code must still compile).
    pub(crate) fn record_at(self, index: usize) -> Record {
        self.record_table[usize::from(self.records[index])]
    }
}

#[cfg(feature = "timezone-db")]
impl ZoneData {
    /// Returns the local time in effect at `seconds`: the UTC offset, the
    /// daylight saving flag, and the abbreviation in effect.
    pub fn lookup(self, seconds: i64) -> ZoneInfo<'static> {
        let starts = self.starts;
        if starts.is_empty() {
            return self.future_info(seconds);
        }
        let past = starts.partition_point(|&start| start <= seconds);
        if past == 0 {
            // Before the first explicit transition: the initial record, per
            // RFC 8536 §3.2.
            return record_info(self.initial);
        }
        if past == starts.len() {
            // From the last transition onward the future rule applies.
            return self.future_info(seconds);
        }
        record_info(self.record_at(past - 1))
    }

    /// The local time implied by the future rule at `seconds`.
    fn future_info(self, seconds: i64) -> ZoneInfo<'static> {
        match self.future {
            Future::Fixed {
                record,
            } => record_info(record),
            Future::DaylightSaving {
                standard,
                daylight,
                start,
                end,
            } => evaluate_daylight(
                DstRule {
                    name: abbreviation(daylight.abbrev).unwrap_or(""),
                    east: daylight.offset,
                    start: (daylight_rule_from_packed_day(start.0), start.1),
                    end: (daylight_rule_from_packed_day(end.0), end.1),
                },
                standard.offset,
                abbreviation(standard.abbrev).unwrap_or(""),
                seconds,
            ),
        }
    }
}

/// Looks up the entry for the given zone name.
///
/// Matching is case-insensitive, mirroring the IANA database conventions;
/// the table is sorted lowercased by the generator for this purpose.
#[cfg(feature = "timezone-db")]
pub(crate) fn lookup(name: &str) -> Option<&'static ZoneEntry> {
    let index = ZONES.binary_search_by(|zone| compare_lowercase(zone.name, name)).ok()?;
    Some(&ZONES[index])
}

/// Legacy blob-based lookup, kept while the pre-parsed engine is verified in
/// tests; test builds assert that both agree (see `timezone.rs`).
#[cfg(all(test, feature = "timezone-db"))]
pub(crate) fn lookup_legacy(name: &str) -> Option<(&'static str, &'static [u8])> {
    if let Ok(index) = ZONES_LEGACY.binary_search_by(|entry| entry.0.cmp(name)) {
        let (canonical, offset, len) = ZONES_LEGACY[index];
        return Some((canonical, &BLOB[offset as usize..(offset + len) as usize]));
    }
    for &(candidate, offset, len) in ZONES_LEGACY {
        if candidate.eq_ignore_ascii_case(name) {
            return Some((candidate, &BLOB[offset as usize..(offset + len) as usize]));
        }
    }
    None
}

/// Returns the names of every embedded time zone, sorted.
#[cfg(feature = "timezone-db")]
pub fn all_timezones() -> impl Iterator<Item = &'static str> {
    ZONES.iter().map(|zone| zone.name)
}

/// Stub for builds without the embedded database: no named zones exist.
#[cfg(not(feature = "timezone-db"))]
pub(crate) fn lookup(_name: &str) -> Option<&'static ZoneEntry> {
    None
}

#[cfg(feature = "timezone-db")]
fn compare_lowercase(left: &str, right: &str) -> core::cmp::Ordering {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let mut index = 0;
    while index < left.len() && index < right.len() {
        let a = left[index].to_ascii_lowercase();
        let b = right[index].to_ascii_lowercase();
        if a != b {
            return a.cmp(&b);
        }
        index += 1;
    }
    left.len().cmp(&right.len())
}
