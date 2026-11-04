#![no_main]

use cose::{CoseKey, CoseKeySet, Header, HeaderRole};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(header) = Header::from_slice(data) {
        let _ = header.validate(HeaderRole::Protected);
        let _ = header.validate(HeaderRole::Unprotected);
        let _ = header.algorithm();
        let _ = header.kid();
        let _ = header.content_type();
        let _ = header.crit();
        for item in header.iter() {
            let _ = item;
        }
    }

    if let Ok(key) = CoseKey::from_slice(data) {
        let _ = key.validate();
        let _ = key.kty();
        let _ = key.kid();
        let _ = key.alg();
        let _ = key.ec2();
        let _ = key.okp();
        let _ = key.rsa();
        let _ = key.akp();
        let _ = key.symmetric();
        let _ = key.key_ops();
        for item in key.iter() {
            let _ = item;
        }
    }

    if let Ok(set) = CoseKeySet::from_slice(data) {
        let _ = set.len();
        let _ = set.iter().count();
        let _ = set.find(None, None);
        let _ = set.find_by_kid(b"fuzz");
    }
});
