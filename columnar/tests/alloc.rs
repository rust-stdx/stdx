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

#[derive(Columnar, Debug, PartialEq)]
struct OneU64 {
    #[columnar(key = 1)]
    value: u64,
}

#[test]
fn encode_does_not_overallocate_or_use_column_temporaries() {
    let mut state = 0x1234_5678_9abc_def0u64;
    let rows: Vec<OneU64> = (0..100_000)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            OneU64 {
                value: state,
            }
        })
        .collect();
    let (bytes, counters) = measure(|| columnar::to_vec(&rows).unwrap());
    assert_eq!(bytes.len(), 800_017);
    assert!(
        counters.calls <= 2,
        "encode allocated {} times; expected the output buffer only",
        counters.calls
    );
    assert!(
        counters.bytes < 1 << 20,
        "encode allocated {} bytes for {} encoded bytes",
        counters.bytes,
        bytes.len()
    );
}

#[test]
fn encode_into_reuses_its_buffer() {
    let mut state = 0x0ddc_0ffe_e15e_beefu64;
    let rows: Vec<OneU64> = (0..10_000)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            OneU64 {
                value: state,
            }
        })
        .collect();
    let mut buffer = Vec::new();
    columnar::to_vec_into(&rows, &mut buffer).unwrap();
    let (_, counters) = measure(|| columnar::to_vec_into(&rows, &mut buffer).unwrap());
    assert_eq!(
        counters.calls, 0,
        "encoding into a large enough buffer allocated {} times",
        counters.calls
    );
}

#[derive(Columnar, Debug, PartialEq)]
struct Delegated {
    #[columnar(key = 1)]
    index: usize,
    #[columnar(key = 2)]
    character: char,
}

#[test]
fn delegated_scalar_encode_does_not_build_conversion_vectors() {
    let rows: Vec<Delegated> = (0..100_000)
        .map(|index| Delegated {
            index,
            character: char::from_u32(0x20 + (index as u32 % 90)).unwrap(),
        })
        .collect();
    let (_, counters) = measure(|| columnar::to_vec(&rows).unwrap());
    // One output buffer plus the internal run/length vectors that Auto may
    // choose for this data.
    assert!(
        counters.calls <= 4,
        "delegated scalar encode allocated {} times",
        counters.calls
    );
}
