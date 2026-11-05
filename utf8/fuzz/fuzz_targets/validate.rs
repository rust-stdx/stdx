#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let expected = core::str::from_utf8(data);
    assert_eq!(utf8::validate(data), expected.is_ok(), "validate disagreed with std");

    match (utf8::from_utf8(data), expected) {
        (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
        (Err(actual), Err(expected)) => {
            assert_eq!(actual.valid_up_to(), expected.valid_up_to());
            assert_eq!(actual.error_len(), expected.error_len());
        }
        (actual, expected) => panic!("from_utf8 disagreed with std: {actual:?} vs {expected:?}"),
    }
});
