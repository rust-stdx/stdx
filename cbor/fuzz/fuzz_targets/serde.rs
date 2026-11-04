#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = cbor::serde::from_slice::<cbor::Value>(data) {
        let encoded = cbor::serde::to_vec(&value).unwrap();
        let decoded: cbor::Value = cbor::serde::from_slice(&encoded).unwrap();
        assert_eq!(value, decoded);
    }
});
