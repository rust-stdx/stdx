#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let valid = cbor::validate(data);
    let _ = cbor::validate_canonical(data);
    let mut decoder = cbor::Decoder::new(data);
    let _ = decoder.skip();
    let _ = cbor::to_diagnostic_string(data);

    // `RawValue` decoding must agree with the validator for the whole input.
    match cbor::from_slice::<cbor::RawValue<'_>>(data) {
        Ok(raw) => {
            assert!(valid.is_ok(), "RawValue accepted an invalid item: {data:02x?}");
            assert_eq!(raw.as_bytes(), data);
        }
        Err(_) => assert!(valid.is_err(), "RawValue rejected a valid item: {data:02x?}"),
    }
});
