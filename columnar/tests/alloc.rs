//! Allocation regression tests for the decode paths.
//!
//! These tests use [`alloc_counter`] so they only measure the calling thread.
//! The thresholds are deliberately loose: they catch an accidental extra
//! intermediate vector, not allocator bookkeeping differences.

use alloc_counter::measure;
use columnar::Columnar;

#[derive(Columnar, Debug, PartialEq)]
struct Numeric {
    #[columnar(key = 1)]
    a: u64,
    #[columnar(key = 2)]
    b: i64,
    #[columnar(key = 3)]
    c: u32,
    #[columnar(key = 4)]
    d: i32,
    #[columnar(key = 5)]
    e: f64,
    #[columnar(key = 6)]
    f: bool,
    #[columnar(key = 7)]
    g: u16,
    #[columnar(key = 8)]
    h: i16,
}

#[test]
fn numeric_decode_does_not_allocate_intermediate_columns() {
    let rows: Vec<Numeric> = (0..1_000)
        .map(|index| Numeric {
            a: index,
            b: -(index as i64),
            c: index as u32 % 1_000,
            d: -(index as i32 % 1_000),
            e: index as f64 / 7.0,
            f: index % 2 == 0,
            g: index as u16 % 500,
            h: -(index as i16 % 500),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let (decoded, counters) = measure(|| columnar::from_slice::<Numeric>(&bytes).unwrap());
    assert_eq!(decoded, rows);
    assert!(
        counters.calls <= 12,
        "numeric decode allocated {} times, expected one vector per column plus the row vector",
        counters.calls
    );
}

#[derive(Columnar, Debug, PartialEq)]
struct OneLongList {
    #[columnar(key = 1)]
    values: Vec<u64>,
}

#[test]
fn list_decode_streams_child_values_without_a_flat_vector() {
    const ELEMENTS: usize = 10_000;
    let rows = vec![OneLongList {
        values: (0..ELEMENTS as u64).collect(),
    }];
    let bytes = columnar::to_vec(&rows).unwrap();
    let (decoded, counters) = measure(|| columnar::from_slice::<OneLongList>(&bytes).unwrap());
    assert_eq!(decoded, rows);
    assert!(
        counters.calls <= 4,
        "list decode allocated {} times; the flat child vector is still present",
        counters.calls
    );
    assert!(
        counters.bytes < (ELEMENTS as u64 * 16),
        "list decode allocated {} bytes; expected only the row vector, not a second element buffer",
        counters.bytes
    );
}

#[derive(Columnar, Debug, PartialEq)]
struct OneOption {
    #[columnar(key = 1)]
    value: Option<u64>,
}

#[test]
fn option_decode_streams_inner_values() {
    let rows: Vec<OneOption> = (0..1_000)
        .map(|index| OneOption {
            value: (index % 3 != 0).then_some(index as u64),
        })
        .collect();
    let bytes = columnar::to_vec(&rows).unwrap();
    let (decoded, counters) = measure(|| columnar::from_slice::<OneOption>(&bytes).unwrap());
    assert_eq!(decoded, rows);
    assert!(
        counters.calls <= 3,
        "option decode allocated {} times; expected the output vector only",
        counters.calls
    );
}
