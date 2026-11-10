#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = json::from_slice::<json::Value>(data) {
        let encoded = json::to_string(&value);
        let decoded: json::Value = json::from_str(&encoded).unwrap();
        assert_eq!(value, decoded);
    }
});
