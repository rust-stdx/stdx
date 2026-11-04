#![no_main]

use cose::{Algorithm, Error, Mac, Mac0, Sign, Sign1, Verifier, VerifyOptions};
use libfuzzer_sys::fuzz_target;

struct Reject;

impl Verifier for Reject {
    fn algorithm(&self) -> Algorithm {
        Algorithm::Ed25519
    }

    fn verify(&self, _message: &[u8], _signature: &[u8]) -> cose::Result<()> {
        Err(Error::InvalidSignature)
    }
}

fn exercise(data: &[u8]) {
    if let Ok(message) = Sign1::from_slice(data) {
        let _ = message.headers();
        let _ = message.encoded_len();
        let _ = message.to_vec();
        let mut scratch = vec![0u8; 4096];
        let _ = message.verify_with_scratch(&Reject, data, &VerifyOptions::default(), &mut scratch);
    }
    if let Ok(message) = Sign::from_slice(data) {
        let _ = message.headers();
        let _ = message.encoded_len();
        let _ = message.to_vec();
        for signature in message.signatures() {
            if let Ok(signature) = signature {
                let _ = signature.headers();
            }
        }
        let mut scratch = vec![0u8; 4096];
        let _ = message.verify_any_with_scratch(&Reject, data, &VerifyOptions::default(), &mut scratch);
    }
    if let Ok(message) = Mac0::from_slice(data) {
        let _ = message.headers();
        let _ = message.encoded_len();
        let _ = message.to_vec();
    }
    if let Ok(message) = Mac::from_slice(data) {
        let _ = message.headers();
        let _ = message.encoded_len();
        let _ = message.to_vec();
        for recipient in message.recipients() {
            if let Ok(recipient) = recipient {
                let _ = recipient.headers();
                let _ = recipient.algorithm();
                let _ = recipient.kid();
            }
        }
    }
}

fuzz_target!(|data: &[u8]| exercise(data));
