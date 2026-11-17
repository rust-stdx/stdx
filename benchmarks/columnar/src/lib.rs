//! Reproducible fixtures for the columnar benchmarks.
//!
//! Each encoding is applied through its own row type because the encoding is
//! selected by a field attribute, not at runtime.

use columnar::Columnar;

/// Number of values generated for every data set.
pub const ROWS: usize = 100_000;

#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct AutoRow {
    #[columnar(key = 1)]
    pub value: u64,
}

#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct RleRow {
    #[columnar(key = 1, encoding = "rle")]
    pub value: u64,
}

#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct DeltaRleRow {
    #[columnar(key = 1, encoding = "delta_rle")]
    pub value: u64,
}

#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct DeltaOfDeltaRow {
    #[columnar(key = 1, encoding = "delta_of_delta")]
    pub value: u64,
}

/// Deterministic pseudo-random numbers, so every run sees the same data.
fn next_random(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}

/// Strictly increasing identifiers.
pub fn sequential() -> Vec<u64> {
    (0..ROWS as u64).collect()
}

/// Timestamps with a fixed step.
pub fn regular_timestamps() -> Vec<u64> {
    (0..ROWS as u64)
        .map(|index| 1_700_000_000_000 + index * 1_000)
        .collect()
}

/// Timestamps with small, frequently changing jitter.
pub fn jittered_timestamps() -> Vec<u64> {
    let mut state = 0x1234_5678_9abc_def0;
    let mut value = 1_700_000_000_000u64;
    (0..ROWS)
        .map(|_| {
            let jitter = (next_random(&mut state) % 21) as i64 - 10;
            value = (value as i64 + 1_000 + jitter) as u64;
            value
        })
        .collect()
}

/// Values that repeat in long blocks.
pub fn blocky() -> Vec<u64> {
    (0..ROWS as u64).map(|index| index / 1_000).collect()
}

/// Uniformly random values.
pub fn random() -> Vec<u64> {
    let mut state = 0x0ddc_0ffe_e15e_beef;
    (0..ROWS).map(|_| next_random(&mut state)).collect()
}
