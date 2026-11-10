#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let options = json::Options {
        use_simd_masks: true,
        ..json::Options::default()
    };
    let plain = json::from_slice::<json::Value>(data);
    let masked = json::from_slice_with_options::<json::Value>(data, options);
    match (plain, masked) {
        (Ok(a), Ok(b)) => assert_eq!(a, b),
        (Err(_), Err(_)) => {}
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => panic!("masked and scalar parsers disagreed"),
    }
    assert_eq!(json::validate(data).is_ok(), json::validate_with_options(data, options).is_ok());
});
