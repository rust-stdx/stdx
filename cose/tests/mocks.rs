#![cfg(all(feature = "alloc", feature = "random"))]

use core::cell::Cell;

use cose::{Algorithm, Error, Header, Sign, Sign1Builder, SignBuilder, Signer, Verifier, VerifyOptions};
use crypto::curve25519::ed25519;

struct ZeroSigner {
    written: usize,
}

impl Signer for ZeroSigner {
    fn algorithm(&self) -> Algorithm {
        Algorithm::Ed25519
    }

    fn signature_len(&self) -> usize {
        64
    }

    fn sign_into(&self, _message: &[u8], out: &mut [u8]) -> cose::Result<usize> {
        let len = self.written.min(out.len()).min(self.signature_len());
        out[..len].fill(0);
        Ok(len)
    }
}

struct CountingVerifier {
    calls: Cell<usize>,
}

impl Verifier for CountingVerifier {
    fn algorithm(&self) -> Algorithm {
        Algorithm::Ed25519
    }

    fn verify(&self, _message: &[u8], _signature: &[u8]) -> cose::Result<()> {
        self.calls.set(self.calls.get() + 1);
        Err(Error::InvalidSignature)
    }
}

#[test]
fn sign_builder_signs_with_the_final_context() {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let public = secret.public_key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();

    let builder = SignBuilder::new()
        .payload(b"first")
        .add_signature(&secret, protected, Header::empty_unprotected())
        .unwrap();
    let token = builder.payload(b"second").sign().unwrap();

    let message = Sign::from_slice(&token).unwrap();
    assert_eq!(message.payload(), Some(&b"second"[..]));
    message.verify_any(&public, b"", &VerifyOptions::default()).unwrap();
}

#[test]
fn sign_builder_rejects_short_signatures() {
    let signer = ZeroSigner {
        written: 1,
    };
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();

    let result = SignBuilder::new()
        .payload(b"payload")
        .add_signature(&signer, protected, Header::empty_unprotected())
        .unwrap()
        .sign();
    assert!(matches!(result, Err(Error::InvalidSignature)));
}

#[test]
fn verify_any_honors_the_signature_attempt_budget() {
    let signer = ZeroSigner {
        written: 64,
    };
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();

    let mut builder = SignBuilder::new().payload(b"payload");
    for _ in 0..4 {
        builder = builder
            .add_signature(&signer, protected.clone(), Header::empty_unprotected())
            .unwrap();
    }
    let token = builder.sign().unwrap();
    let message = Sign::from_slice(&token).unwrap();
    let verifier = CountingVerifier {
        calls: Cell::new(0),
    };
    let options = VerifyOptions {
        max_signature_attempts: Some(2),
        ..Default::default()
    };
    let mut scratch = [0u8; 256];

    assert!(matches!(
        message.verify_any_with_scratch(&verifier, b"", &options, &mut scratch),
        Err(Error::TooManyAttempts {
            limit: 2
        })
    ));
    assert_eq!(verifier.calls.get(), 2);
}

#[test]
fn verify_any_tries_signatures_until_the_budget_is_spent() {
    let secret = ed25519::SecretKey::from_bytes(&[9u8; 32]);
    let public = secret.public_key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();

    let token = SignBuilder::new()
        .payload(b"payload")
        .add_signature(&secret, protected, Header::empty_unprotected())
        .unwrap()
        .sign()
        .unwrap();
    let message = Sign::from_slice(&token).unwrap();
    let options = VerifyOptions {
        max_signature_attempts: Some(1),
        ..Default::default()
    };
    let mut scratch = [0u8; 256];
    assert_eq!(
        message
            .verify_any_with_scratch(&public, b"", &options, &mut scratch)
            .unwrap(),
        0
    );
}

#[test]
fn sign1_scratch_too_small_reports_required_size() {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let public = secret.public_key();
    let token = Sign1Builder::new()
        .protected(
            Header::builder()
                .algorithm(Algorithm::Ed25519)
                .build_protected()
                .unwrap(),
        )
        .payload(b"payload")
        .sign(&secret)
        .unwrap();
    let message = cose::Sign1::from_slice(&token).unwrap();
    let mut scratch = [0u8; 4];
    assert!(matches!(
        message.verify_with_scratch(&public, b"", &VerifyOptions::default(), &mut scratch),
        Err(Error::BufferTooSmall {
            required: _,
            available: 4
        })
    ));
}
