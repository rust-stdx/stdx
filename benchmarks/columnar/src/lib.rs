//! Reproducible fixtures for the columnar benchmarks.
//!
//! Each encoding is applied through its own row type because the encoding is
//! selected by a field attribute, not at runtime.

use std::{borrow::Cow, collections::BTreeMap};

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

/// Mixed text columns: identifiers, a low-cardinality column and a unique
/// column. Auto selects RLE for `tag` and length encoding for `name`.
#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct TextRow {
    #[columnar(key = 1)]
    pub id: u64,
    #[columnar(key = 2)]
    pub name: String,
    #[columnar(key = 3)]
    pub tag: String,
}

/// Lists, options and maps in one row.
#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct ContainerRow {
    #[columnar(key = 1)]
    pub id: u64,
    #[columnar(key = 2)]
    pub tags: Vec<u32>,
    #[columnar(key = 3)]
    pub values: Vec<u64>,
    #[columnar(key = 4)]
    pub maybe: Option<i64>,
    #[columnar(key = 5)]
    pub attrs: BTreeMap<u16, u32>,
}

/// Eight numeric fields, close to a typical event row.
#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct WideRow {
    #[columnar(key = 1)]
    pub a: u64,
    #[columnar(key = 2)]
    pub b: i64,
    #[columnar(key = 3)]
    pub c: u32,
    #[columnar(key = 4)]
    pub d: i32,
    #[columnar(key = 5)]
    pub e: f64,
    #[columnar(key = 6)]
    pub f: bool,
    #[columnar(key = 7)]
    pub g: u16,
    #[columnar(key = 8)]
    pub h: i16,
}

#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub enum Kind {
    Empty,
    Count(u64),
    Named { x: i32, y: String },
}

#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct EnumRow {
    #[columnar(key = 1)]
    pub id: u64,
    #[columnar(key = 2)]
    pub kind: Kind,
}

/// Borrowed text: decoding should not allocate per value.
#[derive(Clone, Debug, PartialEq, Columnar)]
#[columnar(iterable)]
pub struct BorrowedRow<'a> {
    #[columnar(key = 1)]
    pub id: u64,
    #[columnar(key = 2)]
    pub text: Cow<'a, str>,
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

pub fn text_rows() -> Vec<TextRow> {
    let mut state = 0x5eed_1234_5678_9abc;
    (0..ROWS)
        .map(|index| TextRow {
            id: index as u64,
            name: format!("user-{}", next_random(&mut state)),
            tag: ["click", "view", "purchase", "signup"][index % 4].into(),
        })
        .collect()
}

pub fn container_rows() -> Vec<ContainerRow> {
    (0..ROWS / 2)
        .map(|index| ContainerRow {
            id: index as u64,
            tags: (0..4).map(|offset| index as u32 + offset).collect(),
            values: (0..8).map(|offset| index as u64 * 10 + offset).collect(),
            maybe: (index % 3 != 0).then_some(index as i64),
            attrs: [(1, index as u32), (2, index as u32 * 2)].into_iter().collect(),
        })
        .collect()
}

pub fn wide_rows() -> Vec<WideRow> {
    (0..ROWS)
        .map(|index| WideRow {
            a: index as u64,
            b: -(index as i64),
            c: index as u32,
            d: -(index as i32),
            e: index as f64 / 7.0,
            f: index % 2 == 0,
            g: index as u16,
            h: -(index as i16),
        })
        .collect()
}

pub fn enum_rows() -> Vec<EnumRow> {
    (0..ROWS / 2)
        .map(|index| EnumRow {
            id: index as u64,
            kind: match index % 3 {
                0 => Kind::Empty,
                1 => Kind::Count(index as u64),
                _ => Kind::Named {
                    x: index as i32,
                    y: format!("name-{}", index % 16),
                },
            },
        })
        .collect()
}

pub fn borrowed_rows() -> Vec<BorrowedRow<'static>> {
    let mut state = 0xfeed_face_cafe_beef;
    (0..ROWS)
        .map(|index| BorrowedRow {
            id: index as u64,
            text: if index % 2 == 0 {
                Cow::Borrowed("a repeated borrowed value")
            } else {
                Cow::Owned(format!("value-{}", next_random(&mut state)))
            },
        })
        .collect()
}
