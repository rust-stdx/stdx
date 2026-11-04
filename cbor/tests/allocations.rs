#![cfg(all(feature = "alloc", feature = "derive"))]

use alloc_counter::measure;
use cbor::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Borrowed<'a> {
    #[cbor(borrow)]
    name: &'a str,
    #[cbor(bytes)]
    data: &'a [u8],
    value: u64,
}

#[test]
fn borrowed_decode_is_allocation_free() {
    let value = Borrowed {
        name: "hello",
        data: b"abc",
        value: 42,
    };
    let bytes = cbor::to_vec(&value);
    let (decoded, counters) = measure(|| cbor::from_slice::<Borrowed<'_>>(&bytes).unwrap());
    assert_eq!(decoded, value);
    assert_eq!(counters.calls, 0, "borrowed decode allocated");
}

#[test]
fn fixed_buffer_encode_is_allocation_free() {
    let value = Borrowed {
        name: "hello",
        data: b"abc",
        value: 42,
    };
    let mut buffer = [0u8; 128];
    let (used, counters) = measure(|| cbor::to_slice(&value, &mut buffer).unwrap().len());
    assert!(used > 0);
    assert_eq!(counters.calls, 0, "fixed-buffer encode allocated");
}

#[test]
fn encoded_len_validate_and_skip_are_allocation_free() {
    let value = ("hello", 42u64, vec![1u8, 2, 3]);
    let bytes = cbor::to_vec(&value);
    let (_, counters) = measure(|| {
        let len = cbor::encoded_len(&value);
        assert_eq!(len, bytes.len());
        cbor::validate(&bytes).unwrap();
        cbor::validate_canonical(&bytes).unwrap();
        let mut decoder = cbor::Decoder::new(&bytes);
        decoder.skip().unwrap();
    });
    assert_eq!(counters.calls, 0);
}

#[test]
fn raw_value_is_allocation_free() {
    let bytes = [0x82, 0x01, 0x02];
    let (raw, counters) = measure(|| cbor::RawValue::new(&bytes).unwrap());
    assert_eq!(raw.as_bytes(), &bytes);
    assert_eq!(counters.calls, 0);

    let (decoded, counters) = measure(|| {
        let mut decoder = cbor::Decoder::new(&bytes);
        decoder.decode::<cbor::RawValue<'_>>().unwrap()
    });
    assert_eq!(decoded.as_bytes(), &bytes);
    assert_eq!(counters.calls, 0);
}

#[test]
fn to_vec_allocates_once_for_small_values() {
    let value = ("hello", 42u64);
    let (bytes, counters) = measure(|| cbor::to_vec(&value));
    assert!(!bytes.is_empty());
    assert_eq!(counters.calls, 1, "expected one allocation, got {}", counters.calls);
}

#[test]
fn vec_decode_uses_a_bounded_initial_allocation() {
    let values: Vec<u64> = (0..1000).collect();
    let bytes = cbor::to_vec(&values);
    let (decoded, counters) = measure(|| cbor::from_slice::<Vec<u64>>(&bytes).unwrap());
    assert_eq!(decoded, values);
    assert!(
        counters.calls <= 2,
        "expected a bounded initial allocation, got {}",
        counters.calls
    );
    assert!(counters.bytes <= 64 * 1024);
}

#[test]
fn vec_decode_does_not_amplify_large_elements() {
    let n: u16 = 4096;
    let mut bytes = Vec::new();
    bytes.push(0x99);
    bytes.extend_from_slice(&n.to_be_bytes());
    bytes.extend(std::iter::repeat_n(0u8, n as usize));

    let (result, counters) = measure(|| cbor::from_slice::<Vec<[u8; 1024]>>(&bytes));
    assert!(result.is_err());
    assert!(counters.bytes <= 64 * 1024, "preallocated {} bytes", counters.bytes);
}
