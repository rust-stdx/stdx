//! Time zone rules and named-zone lookup.

use core::fmt;

use crate::{
    civil::SECS_PER_DAY,
    error::{self, Error},
    tzdb::{self, ZoneEntry, ZoneRules},
    tzif::ZoneInfo,
};

/// How a civil (wall-clock) time that does not map cleanly to an instant
/// should be resolved.
///
/// This is the [`TimeZone::ambiguity`] companion used by
/// [`DateTime::from_parts_with`](crate::DateTime::from_parts_with).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[non_exhaustive]
pub enum Disambiguation {
    /// The default: in a gap shift the wall time forward, in a fold select
    /// the earlier of the two instants.
    Compatible,
    /// Select the earliest instant. In a gap there is only the shifted
    /// forward instant, so this is the same as `Compatible` there.
    Earlier,
    /// Select the latest instant. In a gap there is only the shifted
    /// forward instant, so this is the same as `Compatible` there.
    Later,
    /// Reject civil times that are ambiguous or that do not exist with
    /// [`ErrorKind::Ambiguous`](crate::ErrorKind::Ambiguous).
    Reject,
}

/// Describes how a civil time maps to UTC offsets in a time zone.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Ambiguity {
    /// Exactly one offset applies.
    Exact {
        /// The offset in effect, in seconds east of Greenwich.
        offset: i32,
    },
    /// A fold: the civil time occurs twice.
    Fold {
        /// The offset of the earlier instant.
        earlier: i32,
        /// The offset of the later instant.
        later: i32,
    },
    /// A gap: the civil time does not exist.
    Gap {
        /// The offset in effect before the transition.
        before: i32,
        /// The offset in effect after the transition.
        after: i32,
    },
}

/// The maximum magnitude of a UTC offset, in seconds (25:59:59).
pub(crate) const OFFSET_LIMIT: i32 = 93_599;

/// Number of seconds used when probing for offset transitions.
const PROBE: i64 = 2 * SECS_PER_DAY;

/// A time zone: UTC, a fixed offset, or a named IANA zone.
///
/// Values are cheap to copy. Named zones are backed by the embedded database
/// (or, with the `timezone-system` feature, by data read from the system) and
/// never require allocation to use.
///
/// # Invariants
///
/// A value can only be constructed through [`TimeZone::UTC`],
/// [`TimeZone::fixed`], [`TimeZone::named`] or [`TimeZone::system`], so:
///
/// * a fixed offset is always validated to lie within
///   `-25:59:59 ..= 25:59:59`, and
/// * a named zone always carries rule data that parsed successfully; a
///   lookup or a read of malformed data fails with
///   [`ErrorKind::UnknownTimeZone`](crate::ErrorKind::UnknownTimeZone) or
///   [`ErrorKind::UnknownSystemTimeZone`](crate::ErrorKind::UnknownSystemTimeZone)
///   instead of inventing offsets.
#[derive(Clone, Copy)]
pub struct TimeZone {
    zone: ZoneKind,
}

#[derive(Clone, Copy)]
enum ZoneKind {
    /// Coordinated Universal Time: [`TimeZone::UTC`], the `"z"` alias, and
    /// zero offsets without an annotation.
    Utc,
    /// A fixed offset from UTC, in seconds east of Greenwich.
    Fixed {
        /// Offset in seconds east of UTC.
        offset: i32,
    },
    /// A named IANA zone from the embedded database, or the detected system
    /// zone (see [`TimeZone::system`]).
    Named(&'static ZoneEntry),
}

impl TimeZone {
    /// The UTC time zone.
    pub const UTC: TimeZone = TimeZone {
        zone: ZoneKind::Utc,
    };

    /// Returns a fixed-offset time zone.
    ///
    /// # Errors
    ///
    /// Returns an error if `offset` is outside `-25:59:59 ..= 25:59:59`.
    pub const fn fixed(offset: i32) -> Result<TimeZone, Error> {
        if offset < -OFFSET_LIMIT || offset > OFFSET_LIMIT {
            return Err(error::out_of_range("UTC offset is outside the supported range"));
        }
        Ok(TimeZone {
            zone: ZoneKind::Fixed {
                offset,
            },
        })
    }

    /// Looks up an IANA time zone by name.
    ///
    /// `"UTC"`, `"GMT"`, `"Etc/UTC"` and the other UTC names in the database
    /// are returned as ordinary named zones, so their canonical name,
    /// abbreviation and RFC 9557 annotation are preserved. The bare `"z"`
    /// convenience alias resolves to [`TimeZone::UTC`]. `"local"` resolves to
    /// the system time zone when the `timezone-system` feature is enabled.
    /// Matching is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::UnknownTimeZone`](crate::ErrorKind::UnknownTimeZone)
    /// if `name` is not present in the database.
    pub fn named(name: &str) -> Result<TimeZone, Error> {
        if is_utc_convenience(name) {
            return Ok(TimeZone::UTC);
        }
        if name.eq_ignore_ascii_case("local") {
            return TimeZone::system();
        }
        match tzdb::lookup(name) {
            Some(entry) => Ok(TimeZone {
                zone: ZoneKind::Named(entry),
            }),
            None => Err(error::unknown_time_zone("unknown time zone")),
        }
    }

    /// Returns the name used in an RFC 9557 annotation, which only named
    /// IANA zones have (UTC and fixed offsets are written as offsets).
    pub(crate) const fn annotation_name(&self) -> Option<&'static str> {
        match self.zone {
            ZoneKind::Named(entry) => Some(entry.name),
            _ => None,
        }
    }

    /// Returns the canonical name of this time zone, if it has one.
    #[must_use]
    pub const fn name(&self) -> Option<&'static str> {
        match self.zone {
            ZoneKind::Utc => Some("UTC"),
            ZoneKind::Fixed {
                ..
            } => None,
            ZoneKind::Named(entry) => Some(entry.name),
        }
    }

    /// Returns the UTC offset of this time zone at the given instant, in
    /// seconds east of Greenwich.
    pub(crate) fn offset_at(self, secs: i64) -> i32 {
        match self.info_at(secs) {
            Some(info) => info.offset,
            None => 0,
        }
    }

    /// Returns the local-time information at the given instant.
    pub(crate) fn info_at(self, secs: i64) -> Option<ZoneInfo<'static>> {
        Some(match self.zone {
            ZoneKind::Utc => ZoneInfo {
                offset: 0,
                is_dst: false,
                abbrev: "UTC",
            },
            ZoneKind::Fixed {
                offset,
            } => ZoneInfo {
                offset,
                is_dst: false,
                abbrev: "",
            },
            ZoneKind::Named(entry) => match &entry.rules {
                ZoneRules::Embedded(zone) => {
                    let info = embedded_at(zone, secs);
                    #[cfg(all(test, feature = "timezone-db"))]
                    parity_check(entry.name, secs, &info);
                    info
                }
                ZoneRules::Runtime(tzif) => tzif.lookup(secs)?,
            },
        })
    }

    /// Returns `true` if daylight saving time is in effect at `secs`.
    pub(crate) fn is_dst_at(self, secs: i64) -> bool {
        self.info_at(secs).is_some_and(|info| info.is_dst)
    }

    /// Resolves a civil (wall-clock) time expressed as seconds since the Unix
    /// epoch to the corresponding instant, using the "compatible"
    /// disambiguation strategy: gaps are shifted forward, folds select the
    /// earlier instant.
    pub(crate) fn resolve_compatible(self, local_secs: i64) -> i64 {
        match self.resolve_with(local_secs, Disambiguation::Compatible) {
            Ok(instant) => instant,
            // `Compatible` never rejects a civil time.
            Err(_) => local_secs,
        }
    }

    /// Returns the pair of UTC offsets that can map to the civil time
    /// `local_secs`.
    ///
    /// For a named zone this uses the two offsets around the nearest explicit
    /// transition when one is close enough, which is exact even when two
    /// transitions are less than the two-day probe apart. When no explicit
    /// transition is near (for example the future governed by a POSIX footer
    /// rule, whose transitions are always months apart), it falls back to
    /// probing two days either side.
    fn candidate_offsets(self, local_secs: i64) -> (i32, i32) {
        if let ZoneKind::Named(entry) = self.zone {
            let nearby = match &entry.rules {
                ZoneRules::Embedded(zone) => embedded_nearby(zone, local_secs),
                ZoneRules::Runtime(tzif) => tzif.nearby_offsets(local_secs),
            };
            if let Some(pair) = nearby {
                return pair;
            }
        }
        (self.offset_at(local_secs - PROBE), self.offset_at(local_secs + PROBE))
    }

    /// Resolves a civil (wall-clock) time, expressed as seconds since the
    /// Unix epoch, to the corresponding instant following `disambiguation`.
    pub(crate) fn resolve_with(self, local_secs: i64, disambiguation: Disambiguation) -> Result<i64, Error> {
        match self.zone {
            ZoneKind::Utc => Ok(local_secs),
            ZoneKind::Fixed {
                offset,
            } => Ok(local_secs - i64::from(offset)),
            ZoneKind::Named(..) => {
                let (offset_before, offset_after) = self.candidate_offsets(local_secs);
                if offset_before == offset_after {
                    return Ok(local_secs - i64::from(offset_before));
                }
                let instant_before = local_secs - i64::from(offset_before);
                let instant_after = local_secs - i64::from(offset_after);
                let before_ok = self.offset_at(instant_before) == offset_before;
                let after_ok = self.offset_at(instant_after) == offset_after;
                match (before_ok, after_ok) {
                    // A fold: two instants map to the same wall time.
                    (true, true) => match disambiguation {
                        Disambiguation::Later => Ok(instant_after),
                        Disambiguation::Reject => Err(error::ambiguous("civil time occurs twice in this time zone")),
                        _ => Ok(instant_before),
                    },
                    (true, false) => Ok(instant_before),
                    (false, true) => Ok(instant_after),
                    // A gap: the wall time does not exist. Shift forward.
                    (false, false) => match disambiguation {
                        Disambiguation::Reject => Err(error::ambiguous("civil time does not exist in this time zone")),
                        _ => Ok(instant_before),
                    },
                }
            }
        }
    }

    /// Classifies how the civil time `local_secs` maps to offsets in this
    /// zone. `local_secs` is measured as seconds since the Unix epoch and is
    /// interpreted as if it were an instant, which is sound because every
    /// offset is smaller than the probe window.
    pub(crate) fn classify(self, local_secs: i64) -> Ambiguity {
        match self.zone {
            ZoneKind::Utc => Ambiguity::Exact {
                offset: 0,
            },
            ZoneKind::Fixed {
                offset,
            } => Ambiguity::Exact {
                offset,
            },
            ZoneKind::Named(..) => {
                let (offset_before, offset_after) = self.candidate_offsets(local_secs);
                if offset_before == offset_after {
                    return Ambiguity::Exact {
                        offset: offset_before,
                    };
                }
                let instant_before = local_secs - i64::from(offset_before);
                let instant_after = local_secs - i64::from(offset_after);
                let before_ok = self.offset_at(instant_before) == offset_before;
                let after_ok = self.offset_at(instant_after) == offset_after;
                match (before_ok, after_ok) {
                    // A fold: two instants map to the same wall time.
                    (true, true) => Ambiguity::Fold {
                        earlier: offset_before,
                        later: offset_after,
                    },
                    // Exactly one candidate is self-consistent (a transition
                    // inside the probe window): the civil time exists once.
                    (true, false) => Ambiguity::Exact {
                        offset: offset_before,
                    },
                    (false, true) => Ambiguity::Exact {
                        offset: offset_after,
                    },
                    // A gap: the wall time does not exist.
                    (false, false) => Ambiguity::Gap {
                        before: offset_before,
                        after: offset_after,
                    },
                }
            }
        }
    }

    /// Classifies how the given civil time maps to offsets in this zone.
    ///
    /// # Errors
    ///
    /// Returns an error if a field is out of range.
    pub fn ambiguity(
        self,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Ambiguity, Error> {
        let local = crate::datetime::local_seconds_checked(year, month, day, hour, minute, second)?;
        Ok(self.classify(local))
    }

    /// Returns the bounds of the constant-offset period that contains `secs`,
    /// as `(start, end)` instants where an absent bound means unbounded.
    pub(crate) fn bounds_at(self, secs: i64) -> (Option<i64>, Option<i64>) {
        match self.zone {
            ZoneKind::Named(entry) => match &entry.rules {
                ZoneRules::Embedded(zone) => {
                    let starts = zone.starts;
                    if starts.is_empty() {
                        return (None, None);
                    }
                    let past = starts.partition_point(|&start| start <= secs);
                    if past == 0 {
                        return (None, Some(starts[0]));
                    }
                    if past == starts.len() {
                        return (Some(starts[starts.len() - 1]), None);
                    }
                    (Some(starts[past - 1]), Some(starts[past]))
                }
                // Runtime TZif zones do not expose their transition table.
                ZoneRules::Runtime(_) => (None, None),
            },
            _ => (None, None),
        }
    }

    /// Determines the system's local time zone.
    ///
    /// This reads the `TZ` environment variable, then `/etc/localtime`, then
    /// the platform zoneinfo directory. It never calls `localtime_r`. The
    /// time zone data that a detected value uses must parse; otherwise the
    /// lookup fails. The detected zone is determined once per process and
    /// cached, so a later change of `TZ` (or of `/etc/localtime`) is not
    /// observed, and the process leaks one small allocation of detected data
    /// at most.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::UnknownSystemTimeZone`](crate::ErrorKind::UnknownSystemTimeZone)
    /// when no system zone can be determined, or
    /// [`ErrorKind::UnknownTimeZone`](crate::ErrorKind::UnknownTimeZone) when
    /// `TZ` names a zone that cannot be found.
    #[cfg(feature = "timezone-system")]
    pub fn system() -> Result<TimeZone, Error> {
        static SYSTEM_ZONE: std::sync::OnceLock<Result<TimeZone, Error>> = std::sync::OnceLock::new();
        *SYSTEM_ZONE.get_or_init(system::detect)
    }

    /// Determines the system's local time zone.
    ///
    /// Without the `timezone-system` feature this always returns `UTC`.
    ///
    /// # Errors
    ///
    /// With `timezone-system`, returns an error when the system zone cannot be
    /// determined.
    #[cfg(not(feature = "timezone-system"))]
    pub fn system() -> Result<TimeZone, Error> {
        Ok(TimeZone::UTC)
    }
}

/// Returns `true` for a name that maps directly to [`TimeZone::UTC`] instead of
/// going through the database.
///
/// The bare `"z"` alias always does. Without the `timezone-db` feature the
/// database is absent, so the canonical UTC names fall back to the constant
/// too; with it they are ordinary named zones that preserve their abbreviation
/// and RFC 9557 annotation.
fn is_utc_convenience(name: &str) -> bool {
    if name.eq_ignore_ascii_case("z") {
        return true;
    }
    #[cfg(not(feature = "timezone-db"))]
    {
        ["utc", "gmt", "zulu", "universal", "uct", "etc/utc", "etc/gmt"]
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
    }
    #[cfg(feature = "timezone-db")]
    {
        false
    }
}

/// Evaluates a zone from the embedded database.
#[cfg(feature = "timezone-db")]
fn embedded_at(zone: &'static tzdb::ZoneData, secs: i64) -> ZoneInfo<'static> {
    zone.lookup(secs)
}

/// Without the embedded database, no `ZoneRules::Embedded` value can be
/// constructed; this arm exists only to keep the match exhaustive in builds
/// that exclude the database.
#[cfg(not(feature = "timezone-db"))]
fn embedded_at(_zone: &'static tzdb::ZoneData, _secs: i64) -> ZoneInfo<'static> {
    ZoneInfo {
        offset: 0,
        is_dst: false,
        abbrev: "",
    }
}

/// Returns the offsets immediately before and after the explicit transition
/// nearest to `local` in an embedded zone, or `None` when no transition is
/// close enough to affect it.
fn embedded_nearby(zone: &'static tzdb::ZoneData, local: i64) -> Option<(i32, i32)> {
    let starts = zone.starts;
    if starts.is_empty() {
        return None;
    }
    let past = starts.partition_point(|&start| start <= local);
    let mut best_index = 0usize;
    let mut best_distance = i64::MAX;
    let mut consider = |index: usize| {
        if index < starts.len() {
            let distance = (starts[index] - local).abs();
            if distance < best_distance {
                best_distance = distance;
                best_index = index;
            }
        }
    };
    if let Some(index) = past.checked_sub(1) {
        consider(index);
    }
    consider(past);
    if best_distance > 2 * i64::from(OFFSET_LIMIT) {
        return None;
    }
    let after = zone.record_at(best_index).offset;
    let before = if best_index == 0 {
        zone.initial.offset
    } else {
        zone.record_at(best_index - 1).offset
    };
    Some((before, after))
}

/// Test-only cross-check: the pre-parsed static engine must produce the same
/// answer as evaluating the original TZif bytes (the legacy path).
#[cfg(all(test, feature = "timezone-db"))]
fn parity_check(name: &'static str, secs: i64, evaluated: &ZoneInfo<'static>) {
    if let Some((_, data)) = tzdb::lookup_legacy(name) {
        if let Some(expected) = crate::tzif::Tzif::parse(data).and_then(|tz| tz.lookup(secs)) {
            assert!(
                expected.offset == evaluated.offset
                    && expected.is_dst == evaluated.is_dst
                    && expected.abbrev == evaluated.abbrev,
                "zone engine mismatch for {name} at {secs}: {expected:?} != {evaluated:?}",
            );
        }
    }
}

impl fmt::Debug for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.zone {
            ZoneKind::Utc => f.write_str("Utc"),
            ZoneKind::Fixed {
                offset,
            } => write!(f, "Fixed({offset})"),
            ZoneKind::Named(entry) => write!(f, "Named({})", entry.name),
        }
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.zone {
            ZoneKind::Utc => f.write_str("UTC"),
            ZoneKind::Fixed {
                offset,
            } => crate::format::write_offset(f, offset),
            ZoneKind::Named(entry) => f.write_str(entry.name),
        }
    }
}

impl PartialEq for TimeZone {
    fn eq(&self, other: &TimeZone) -> bool {
        match (self.zone, other.zone) {
            (ZoneKind::Utc, ZoneKind::Utc) => true,
            (
                ZoneKind::Fixed {
                    offset: left,
                },
                ZoneKind::Fixed {
                    offset: right,
                },
            ) => left == right,
            (ZoneKind::Named(left), ZoneKind::Named(right)) => left.name == right.name,
            _ => false,
        }
    }
}

impl Eq for TimeZone {}

impl core::hash::Hash for TimeZone {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self.zone {
            ZoneKind::Utc => 0u8.hash(state),
            ZoneKind::Fixed {
                offset,
            } => {
                1u8.hash(state);
                offset.hash(state);
            }
            ZoneKind::Named(entry) => {
                2u8.hash(state);
                entry.name.hash(state);
            }
        }
    }
}

#[cfg(feature = "timezone-system")]
mod system {
    use alloc::{borrow::ToOwned, boxed::Box, vec::Vec};

    use small_collections::SmallString;

    use super::{TimeZone, ZoneEntry, ZoneKind, ZoneRules};
    use crate::{
        error::{self, Error},
        tzif::Tzif,
    };

    /// Reads the system TZif data, parses it once, and returns a zone value
    /// whose name and rules are heap-allocated once and never freed (the
    /// caller caches the zone, so the process leaks at most one entry).
    ///
    /// The data is validated before anything is released, so a malformed image
    /// does not leak on the error path.
    fn leak_named(name: &str, data: Vec<u8>) -> Result<TimeZone, Error> {
        Tzif::parse(&data).ok_or_else(|| error::unknown_system_time_zone("cannot parse system time zone data"))?;
        let data: &'static [u8] = Box::leak(data.into_boxed_slice());
        let tzif: Tzif<'static> =
            Tzif::parse(data).ok_or_else(|| error::unknown_system_time_zone("cannot parse system time zone data"))?;
        let rules: &'static Tzif<'static> = Box::leak(Box::new(tzif));
        let entry: &'static ZoneEntry = Box::leak(Box::new(ZoneEntry {
            name: Box::leak(name.to_owned().into_boxed_str()),
            rules: ZoneRules::Runtime(rules),
        }));
        Ok(TimeZone {
            zone: ZoneKind::Named(entry),
        })
    }

    /// Inline capacity for the path built when probing zone directories.
    ///
    /// A directory prefix plus a typical IANA name fits well within 128 bytes;
    /// longer paths (for example a long `TZDIR`) spill to the heap inside the
    /// `SmallString` rather than being truncated.
    const PATH_INLINE_LENGTH: usize = 128;

    /// Reads `dir/value`, the TZif file for `value` under `dir`.
    fn read_zone_file(dir: &str, value: &str) -> Option<Vec<u8>> {
        let mut path: SmallString<PATH_INLINE_LENGTH> = SmallString::new();
        path.push_str(dir);
        path.push('/');
        path.push_str(value);
        std::fs::read(path.as_str()).ok()
    }

    fn name_from_path(path: &str) -> &str {
        path.find("/zoneinfo/")
            .map(|index| &path[index + "/zoneinfo/".len()..])
            .unwrap_or("localtime")
    }

    pub(super) fn detect() -> Result<TimeZone, Error> {
        if let Ok(value) = std::env::var("TZ") {
            let value = value.strip_prefix(':').unwrap_or(&value);
            if value.is_empty() {
                return Ok(TimeZone::UTC);
            }
            if value.starts_with('/') || value.starts_with('.') {
                let data =
                    std::fs::read(value).map_err(|_| error::unknown_system_time_zone("cannot read TZ time zone"))?;
                return leak_named(value, data);
            }
            if let Ok(zone) = TimeZone::named(value) {
                return Ok(zone);
            }
            let tzdir = std::env::var("TZDIR").ok();
            for dir in tzdir.as_deref().into_iter().chain([
                "/usr/share/zoneinfo",
                "/usr/lib/zoneinfo",
                "/usr/share/lib/zoneinfo",
                "/etc/zoneinfo",
            ]) {
                if let Some(data) = read_zone_file(dir, value) {
                    return leak_named(value, data);
                }
            }
            return Err(error::unknown_time_zone("unknown TZ time zone"));
        }

        let data = std::fs::read("/etc/localtime")
            .map_err(|_| error::unknown_system_time_zone("cannot read /etc/localtime"))?;
        let link = std::fs::read_link("/etc/localtime").ok();
        let name = link
            .as_deref()
            .and_then(|path| path.to_str())
            .map(name_from_path)
            .unwrap_or("localtime");
        leak_named(name, data)
    }
}
