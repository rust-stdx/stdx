//! Date and time handling with an embedded IANA time zone database.
//!
//! `time` provides a small, correct, `no_std`-friendly date and time API built
//! around a single value type, [`DateTime`], plus [`TimeZone`] and [`Error`].
//! Durations are the standard [`core::time::Duration`]; there is no dedicated
//! duration type.
//!
//! # Design
//!
//! * A [`DateTime`] always represents a precise instant (seconds and
//!   nanoseconds since the Unix epoch) together with a [`TimeZone`]. There is
//!   no separate "naive" or "floating" datetime.
//! * Equality, ordering, hashing, subtraction and all instant arithmetic are
//!   defined on the instant only, never on the time zone. This avoids the
//!   classic footgun where `==` depends on the zone.
//! * The `offset` reported by a value is always the offset in effect at that
//!   instant, even for civil times that fall in a DST gap.
//! * Zone-less input is parsed as UTC. Use
//!   [`DateTime::parse_with_timezone`] to interpret zone-less input in a
//!   specific zone.
//! * Well-known representations are selected with the [`Format`] enum via
//!   [`DateTime::format`]: RFC 3339, RFC 9557, RFC 2822, HTTP dates, and more.
//!   Strict, allocation-free output is available through
//!   [`DateTime::try_format`] and [`DateTime::format_to`].
//! * The embedded IANA database ([`TimeZone::named`]) is available by default,
//!   is pre-parsed at build time, and works without `alloc`. Future
//!   daylight-saving transitions are materialized up to a rolling horizon (the
//!   IANA release year plus 50) so near-future lookups stay a plain search,
//!   with the POSIX rule governing beyond it. The advertised release is
//!   [`TZDB_VERSION`].
//!
//! # Measuring elapsed time
//!
//! Values from [`DateTime::now`] and [`DateTime::now_in`] also carry a
//! monotonic clock reading. When both values being compared have one,
//! [`DateTime::duration_since`], [`DateTime::duration_until`],
//! [`DateTime::signed_duration_since`] and [`DateTime::elapsed`] measure with
//! the monotonic clock and are robust against wall-clock adjustments (for
//! example NTP steps or manual clock changes). Values that were parsed or
//! built from civil fields use the wall clock, so measuring a duration across
//! a serialization round trip is not monotonic. Use
//! [`DateTime::has_monotonic`] to check.
//!
//! # Leap seconds
//!
//! The supported calendar has no leap seconds. A `:60` second is accepted on
//! input (both in parsing and in [`DateTime::from_parts`]) and is represented
//! as `:59` of the same minute; the library never produces `:60` on output.
//!
//! # Ambiguous and skipped civil times
//!
//! A civil time can be skipped (a DST gap) or repeated (a DST fold).
//! [`DateTime::from_parts`] applies the "compatible" strategy: gaps are
//! shifted forward and folds select the earlier instant. Use
//! [`DateTime::from_parts_with`] with a [`Disambiguation`] to choose another
//! strategy (including rejecting such times with
//! [`ErrorKind::Ambiguous`]) and [`TimeZone::ambiguity`] to inspect the
//! mapping.
//!
//! # Limitations
//!
//! * The supported UTC calendar range is `-9999-01-01` through `9999-12-31`.
//!   A value near the boundary can display a year one outside this range when
//!   a large UTC offset is applied.
//! * There is no leap-second support (see above) and no local-time database
//!   for the hours, minutes and seconds of a zone beyond the abbreviation and
//!   the offset.
//! * RFC 9557 support is partial: a single, bare time zone name annotation is
//!   understood on input; other annotation forms are rejected.
//! * Parsing accepts up to nanosecond precision and truncates further
//!   fractional digits; output uses 3, 6 or 9 fractional digits, which keeps
//!   formatted strings sortable.
//!
//! # Feature flags
//!
//! * `std` (default): enables reading the system clock and the monotonic
//!   clock, and the system time zone. It uses `alloc` internally but exposes no
//!   allocation-requiring API.
//! * `timezone-db` (default): embeds a copy of the IANA time zone database.
//! * `timezone-system`: reads time zone rules from the system database
//!   (implies `std`), detecting and caching the system zone once per process.
//! * `serde`: RFC 3339 / RFC 9557 serialization.
//!
//! # Examples
//!
//! ```
//! use time::{DateTime, TimeZone};
//!
//! let dt: DateTime = "2024-07-11T01:14:00Z".parse().unwrap();
//! assert_eq!(dt.unix(), 1_720_660_440);
//!
//! // Requires the default `timezone-db` feature.
//! if let Ok(ny) = TimeZone::named("America/New_York") {
//!     let dt = dt.in_timezone(ny);
//!     assert_eq!(dt.hour(), 21);
//!     assert_eq!(dt.offset(), -4 * 3600);
//! }
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod civil;
mod datetime;
mod error;
mod format;
mod parse;
mod posix;
#[cfg(feature = "serde")]
mod serde;
mod timezone;
mod tzdb;
mod tzif;

#[cfg(test)]
mod tests;

pub use core::time::Duration;

#[cfg(feature = "timezone-db")]
pub use crate::tzdb::all_timezones;
pub use crate::{
    datetime::{DateTime, Weekday},
    error::{Error, ErrorKind},
    format::{Format, Formatted},
    timezone::{Ambiguity, Disambiguation, TimeZone},
    tzdb::TZDB_VERSION,
};
