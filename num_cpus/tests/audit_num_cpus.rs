//! Edge-case checks for logical/physical CPU detection on Linux.
//!
//! The physical count parses `/proc/cpuinfo`; the test compares it with an
//! independent parser and checks the documented fallback when the file has no
//! `physical id`/`cpu cores` fields (common on arm64).

#![cfg(target_os = "linux")]

/// Independent, minimal parser: one core count per distinct `physical id`.
/// Returns `None` when the file does not expose both fields.
fn physical_from_cpuinfo() -> Option<usize> {
    let text = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    let mut map: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new();
    let mut physid: Option<u32> = None;
    let mut cores: Option<usize> = None;
    let mut saw_any = false;
    for line in text.lines().chain(std::iter::once("")) {
        if line.trim().is_empty() {
            if let (Some(id), Some(n)) = (physid.take(), cores.take()) {
                map.insert(id, n);
                saw_any = true;
            }
            physid = None;
            cores = None;
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        match key.trim() {
            "physical id" => physid = value.trim().parse().ok(),
            "cpu cores" => cores = value.trim().parse().ok(),
            _ => {}
        }
    }
    if saw_any { Some(map.values().sum()) } else { None }
}

#[test]
fn logical_count_is_positive_and_matches_std() {
    let n = num_cpus::get();
    assert!(n > 0, "get() must always be at least 1");
    let expected = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    assert_eq!(n, expected);
}

#[test]
fn physical_count_is_positive_and_consistent_with_cpuinfo() {
    let physical = num_cpus::get_physical();
    assert!(physical > 0, "get_physical() must always be at least 1");
    match physical_from_cpuinfo() {
        Some(expected) => assert_eq!(physical, expected, "physical core count differs from /proc/cpuinfo"),
        None => assert_eq!(
            physical,
            num_cpus::get(),
            "without physical id/cpu cores the physical count must fall back to the logical count"
        ),
    }
}

#[test]
fn physical_count_never_exceeds_a_sane_bound() {
    // Guards against integer/parse blowups while staying far above real
    // hardware (the crate's own tests use 236_451 as the bound).
    assert!(num_cpus::get_physical() < 236_451);
}
