#![no_main]

use std::io::BufReader;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut reader = BufReader::with_capacity(7, data);
    let _ = cbor::reader::read_item(&mut reader);
});
