//! Platform clock access.
//!
//! Native targets read the wall clock from [`std::time::SystemTime`] and the
//! monotonic clock from [`std::time::Instant`]. On `wasm32-unknown-unknown` the
//! `wasm` feature reads the wall clock from the browser's `Date.now()`. That
//! target has no monotonic clock, so [`monotonic`] never returns a reading
//! there and elapsed-time measurement falls back to the wall clock.

/// Returns the current wall-clock time as nanoseconds since the Unix epoch.
///
/// On `wasm32-unknown-unknown` this requires the `wasm` feature; without it
/// the call panics with an explanatory message.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) fn unix_nanos() -> i128 {
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
    i128::from(seconds) * 1_000_000_000 + i128::from(nanos)
}

/// Returns the current wall-clock time as nanoseconds since the Unix epoch,
/// read from the browser's `Date.now()`.
#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "wasm"))]
pub(crate) fn unix_nanos() -> i128 {
    millis_to_unix_nanos(js_sys::Date::now())
}

/// Panics because the target has no clock and the `wasm` feature is disabled.
#[cfg(all(target_arch = "wasm32", target_os = "unknown", not(feature = "wasm")))]
pub(crate) fn unix_nanos() -> i128 {
    panic!("reading the current time on wasm32-unknown-unknown requires the `wasm` feature");
}

/// Returns a monotonic clock reading, when the platform provides one.
///
/// `wasm32-unknown-unknown` has no `std` monotonic clock, so values obtained
/// there never carry a reading and elapsed-time measurement uses the wall
/// clock instead.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) fn monotonic() -> Option<std::time::Instant> {
    Some(std::time::Instant::now())
}

/// Returns no reading: `wasm32-unknown-unknown` has no monotonic clock.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) const fn monotonic() -> Option<std::time::Instant> {
    None
}

/// Converts a `Date.now()`-style timestamp in milliseconds into nanoseconds
/// since the Unix epoch.
///
/// Sub-millisecond digits are rounded to the nearest nanosecond. Non-finite
/// values map to zero, and values beyond the `i64` millisecond range saturate;
/// both are far outside the supported calendar range and are rejected by
/// [`DateTime::from_unix_nanos`](crate::DateTime::from_unix_nanos).
#[cfg(any(all(target_arch = "wasm32", target_os = "unknown", feature = "wasm"), test))]
pub(crate) fn millis_to_unix_nanos(millis: f64) -> i128 {
    if !millis.is_finite() {
        return 0;
    }
    let whole_millis = millis.trunc();
    let sub_nanos = ((millis - whole_millis) * 1_000_000.0).round() as i128;
    (whole_millis as i64 as i128) * 1_000_000 + sub_nanos
}

/// Converts `Date.prototype.getTimezoneOffset()` minutes, which are positive
/// west of Greenwich, into seconds east of Greenwich.
///
/// Fractional minutes are truncated. Returns `None` for non-finite values and
/// for values whose seconds representation does not fit in an `i32`.
#[cfg(any(
    all(
        target_arch = "wasm32",
        target_os = "unknown",
        feature = "wasm",
        feature = "timezone-system"
    ),
    test
))]
pub(crate) fn seconds_east_from_minutes(minutes: f64) -> Option<i32> {
    if !minutes.is_finite() || minutes.abs() > f64::from(i32::MAX) {
        return None;
    }
    i32::try_from(-(minutes.trunc() as i64) * 60).ok()
}

/// Returns the IANA time zone name reported by the browser, such as
/// `"America/New_York"`, when `Intl.DateTimeFormat` is available.
///
/// Every step is a fallible reflection call, so a host without `Intl` (or
/// with a broken implementation) yields `None` instead of trapping.
#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "wasm",
    feature = "timezone-system"
))]
pub(crate) fn local_zone_name() -> Option<alloc::string::String> {
    use js_sys::{
        Function, Reflect,
        wasm_bindgen::{JsCast, JsValue},
    };

    let global: JsValue = js_sys::global().into();
    let intl = Reflect::get(&global, &JsValue::from_str("Intl")).ok()?;
    let constructor = Reflect::get(&intl, &JsValue::from_str("DateTimeFormat")).ok()?;
    let constructor = constructor.dyn_into::<Function>().ok()?;
    let formatter = constructor.call0(&JsValue::UNDEFINED).ok()?;
    let resolved_options = Reflect::get(&formatter, &JsValue::from_str("resolvedOptions")).ok()?;
    let resolved_options = resolved_options.dyn_into::<Function>().ok()?;
    let resolved = resolved_options.call0(&formatter).ok()?;
    let zone = Reflect::get(&resolved, &JsValue::from_str("timeZone")).ok()?;
    zone.as_string()
}

/// Returns the browser's current UTC offset in seconds east of Greenwich, as
/// reported by `Date.prototype.getTimezoneOffset()`.
#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "wasm",
    feature = "timezone-system"
))]
pub(crate) fn local_offset_seconds() -> Option<i32> {
    seconds_east_from_minutes(js_sys::Date::new_0().get_timezone_offset())
}

#[cfg(test)]
mod tests {
    use super::{millis_to_unix_nanos, seconds_east_from_minutes};

    #[test]
    fn millis_to_unix_nanos_whole_and_fractional() {
        assert_eq!(millis_to_unix_nanos(0.0), 0);
        assert_eq!(millis_to_unix_nanos(-0.0), 0);
        assert_eq!(millis_to_unix_nanos(1.0), 1_000_000);
        assert_eq!(millis_to_unix_nanos(1.5), 1_500_000);
        assert_eq!(millis_to_unix_nanos(0.001), 1_000);
        assert_eq!(millis_to_unix_nanos(0.000_001), 1);
        assert_eq!(millis_to_unix_nanos(1_700_000_000_123.0), 1_700_000_000_123_000_000);
    }

    #[test]
    fn millis_to_unix_nanos_negative() {
        assert_eq!(millis_to_unix_nanos(-1.0), -1_000_000);
        assert_eq!(millis_to_unix_nanos(-1.5), -1_500_000);
        assert_eq!(millis_to_unix_nanos(-0.5), -500_000);
        // A negative instant before the epoch keeps its sign.
        assert_eq!(millis_to_unix_nanos(-2_208_988_800_000.0), -2_208_988_800_000_000_000);
    }

    #[test]
    fn millis_to_unix_nanos_rounds_sub_millisecond_values() {
        assert_eq!(millis_to_unix_nanos(0.000_000_4), 0);
        assert_eq!(millis_to_unix_nanos(0.000_000_6), 1);
        assert_eq!(millis_to_unix_nanos(-0.000_000_4), 0);
        assert_eq!(millis_to_unix_nanos(-0.000_000_6), -1);
        // Rounding the fraction may carry into the next millisecond.
        assert_eq!(millis_to_unix_nanos(1.999_999_9), 2_000_000);
        assert_eq!(millis_to_unix_nanos(-1.999_999_9), -2_000_000);
    }

    #[test]
    fn millis_to_unix_nanos_rejects_non_finite_values() {
        assert_eq!(millis_to_unix_nanos(f64::NAN), 0);
        assert_eq!(millis_to_unix_nanos(f64::INFINITY), 0);
        assert_eq!(millis_to_unix_nanos(f64::NEG_INFINITY), 0);
    }

    #[test]
    fn millis_to_unix_nanos_saturates_out_of_range_values() {
        // `i64::MAX` milliseconds is far beyond the supported calendar range.
        assert_eq!(millis_to_unix_nanos(f64::MAX), (i64::MAX as i128) * 1_000_000);
        assert_eq!(millis_to_unix_nanos(f64::MIN), (i64::MIN as i128) * 1_000_000);
    }

    #[test]
    fn millis_to_unix_nanos_matches_the_supported_range_boundaries() {
        use crate::{
            DateTime,
            datetime::{MAX_SECS, MIN_SECS},
        };

        // Last supported instant, `9999-12-31T23:59:59.999Z`.
        let max_millis = (MAX_SECS as f64) * 1000.0 + 999.0;
        let dt = DateTime::from_unix_nanos(millis_to_unix_nanos(max_millis)).unwrap();
        assert_eq!(dt.unix(), MAX_SECS);
        assert_eq!(dt.millisecond(), 999);
        assert!(DateTime::from_unix_nanos(millis_to_unix_nanos(max_millis + 1.0)).is_err());

        // First supported instant, `-9999-01-01T00:00:00Z`.
        let min_millis = (MIN_SECS as f64) * 1000.0;
        let dt = DateTime::from_unix_nanos(millis_to_unix_nanos(min_millis)).unwrap();
        assert_eq!(dt.unix(), MIN_SECS);
        assert!(DateTime::from_unix_nanos(millis_to_unix_nanos(min_millis - 1.0)).is_err());
    }

    #[test]
    fn seconds_east_from_minutes_maps_sign_and_scale() {
        assert_eq!(seconds_east_from_minutes(0.0), Some(0));
        assert_eq!(seconds_east_from_minutes(-0.0), Some(0));
        // UTC-5 is 300 minutes west of Greenwich.
        assert_eq!(seconds_east_from_minutes(300.0), Some(-18_000));
        // UTC+5:30 is 330 minutes east of Greenwich.
        assert_eq!(seconds_east_from_minutes(-330.0), Some(19_800));
        assert_eq!(seconds_east_from_minutes(840.0), Some(-50_400));
        assert_eq!(seconds_east_from_minutes(-840.0), Some(50_400));
    }

    #[test]
    fn seconds_east_from_minutes_truncates_fractional_minutes() {
        assert_eq!(seconds_east_from_minutes(1.5), Some(-60));
        assert_eq!(seconds_east_from_minutes(-1.5), Some(60));
        assert_eq!(seconds_east_from_minutes(0.9), Some(0));
    }

    #[test]
    fn seconds_east_from_minutes_rejects_non_finite_values() {
        assert_eq!(seconds_east_from_minutes(f64::NAN), None);
        assert_eq!(seconds_east_from_minutes(f64::INFINITY), None);
        assert_eq!(seconds_east_from_minutes(f64::NEG_INFINITY), None);
    }

    #[test]
    fn seconds_east_from_minutes_rejects_i32_overflow() {
        assert_eq!(seconds_east_from_minutes(f64::from(i32::MAX)), None);
        assert_eq!(seconds_east_from_minutes(-f64::from(i32::MAX)), None);
        assert_eq!(seconds_east_from_minutes(f64::from(i32::MAX) + 1.0), None);
        assert_eq!(seconds_east_from_minutes(f64::from(i32::MIN)), None);
        // The largest minute value whose seconds still fit in an `i32`.
        assert_eq!(seconds_east_from_minutes(35_791_394.0), Some(-2_147_483_640));
        assert_eq!(seconds_east_from_minutes(35_791_395.0), None);
    }
}
