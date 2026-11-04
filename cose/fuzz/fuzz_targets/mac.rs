#![no_main]

use cose::{Algorithm, Error, Mac, Mac0, Macer, VerifyOptions};
use libfuzzer_sys::fuzz_target;

struct Reject;

impl Macer for Reject {
    fn algorithm(&self) -> Algorithm {
        Algorithm::Hmac256_256
    }

    fn tag_len(&self) -> usize {
        32
    }

    fn mac_into(&self, _message: &[u8], _out: &mut [u8]) -> cose::Result<usize> {
        Err(Error::InvalidTag)
    }

    fn verify(&self, _message: &[u8], _tag: &[u8]) -> cose::Result<()> {
        Err(Error::InvalidTag)
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(message) = Mac0::from_slice(data) {
        let mut scratch = vec![0u8; 4096];
        let _ = message.verify_with_scratch(&Reject, data, &VerifyOptions::default(), &mut scratch);
    }
    if let Ok(message) = Mac::from_slice(data) {
        let mut scratch = vec![0u8; 4096];
        let _ = message.verify_with_scratch(&Reject, data, &VerifyOptions::default(), &mut scratch);
    }
});
