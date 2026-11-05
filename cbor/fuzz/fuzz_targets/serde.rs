#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = cbor::serde::from_slice::<cbor::Value>(data) {
        let encoded = cbor::serde::to_vec(&value).unwrap();
        let decoded: cbor::Value = cbor::serde::from_slice(&encoded).unwrap();
        // Compare encodings rather than `Value` equality: floating-point NaN
        // is never equal to itself, even after a lossless roundtrip.
        assert_eq!(cbor::serde::to_vec(&decoded).unwrap(), encoded);
    }
});
