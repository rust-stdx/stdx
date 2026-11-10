#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let valid = json::validate(data).is_ok();
    if valid {
        // Some grammar-valid numbers are outside the representable `f64`
        // range, so parsing may still fail; the fuzzer only cares that it
        // never panics.
        let _ = json::from_slice::<json::Value>(data);
    }
});
