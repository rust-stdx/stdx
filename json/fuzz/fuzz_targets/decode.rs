#![no_main]

use json::DecodeOwned;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = json::from_slice::<json::Value>(data);
    let _ = json::OwnedValue::decode_owned(data);
    let _ = json::validate(data);
});
