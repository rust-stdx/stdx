#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = cbor::from_slice::<cbor::Value>(data) {
        let encoded = cbor::to_vec(&value);
        let decoded: cbor::Value = cbor::from_slice(&encoded).unwrap();
        // Compare encodings rather than `Value` equality: floating-point NaN
        // is never equal to itself, even after a lossless roundtrip.
        assert_eq!(encoded, cbor::to_vec(&decoded));
    }
});
