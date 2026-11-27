//! Safe-API regression tests for CVE-2026-44842.
//!
//! With the default `Options` the wrapper's own `max_depth = 128` stops deep
//! nesting before the underlying scanner becomes expensive. The documented
//! escape hatch for trusted documents (`max_depth: usize::MAX`) removes that
//! protection, so the vendored libyaml scanner's ported `MAX_NESTING_LEVEL`
//! is the remaining hard bound.

use std::time::Instant;

#[test]
fn default_options_reject_deep_nesting_quickly() {
    let input = vec![b'['; 100_000];
    let start = Instant::now();
    let error = yaml::from_slice::<yaml::Value>(&input).unwrap_err().to_string();
    let elapsed = start.elapsed();
    assert!(error.contains("depth"), "{error}");
    // The wrapper stops at 128 frames, so this must be fast regardless of the
    // scanner's super-linear behaviour.
    assert!(elapsed.as_secs() < 5, "took {elapsed:?}");
}

#[test]
fn relaxed_depth_is_bounded_by_the_scanner_limit() {
    // Before the scanner limit was ported, `max_depth: usize::MAX` allowed a
    // deep `Value` to be materialized and then dropped recursively, aborting
    // the process once the nesting exceeded the test thread's stack. The
    // scanner now rejects depth 1001+, so no such value can be built.
    fn relaxed() -> yaml::Options {
        yaml::Options {
            max_depth: usize::MAX,
            max_input_bytes: Some(8 * 1024 * 1024),
            ..yaml::Options::default()
        }
    }

    for depth in [999usize, 1000] {
        let mut input = vec![b'['; depth];
        input.extend(std::iter::repeat_n(b']', depth));
        let parsed = yaml::from_slice_with_options::<yaml::Value>(&input, relaxed());
        assert!(parsed.is_ok(), "depth {depth} should be accepted: {:?}", parsed.err());
    }

    for depth in [1001usize, 2000, 4000, 8000, 16000] {
        let mut input = vec![b'['; depth];
        input.extend(std::iter::repeat_n(b']', depth));
        let error = yaml::from_slice_with_options::<yaml::Value>(&input, relaxed())
            .unwrap_err()
            .to_string();
        assert!(error.contains("depth"), "depth {depth} should be rejected: {error}");
    }
}
