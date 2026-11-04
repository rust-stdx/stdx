#![cfg(feature = "alloc")]

use alloc_counter::measure;
use cose::{
    Algorithm, CoseKey, CoseKeySet, Header, KeyBuilder, KeySetBuilder, KeyType, Mac0, Mac0Builder, MacBuilder, Sign1,
    Sign1Builder, SymmetricKey, VerifyOptions,
};
use crypto::curve25519::ed25519;

fn key() -> (ed25519::SecretKey, ed25519::PublicKey) {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let public = secret.public_key();
    (secret, public)
}

fn assert_allocs<F: FnMut()>(expected: u64, name: &str, mut f: F) {
    f();
    let (_, counters) = measure(|| {
        for _ in 0..100 {
            f();
        }
    });
    assert_eq!(counters.calls, expected * 100, "{name}: {counters:?}");
}

#[test]
fn parse_verify_encode_allocations() {
    let (secret, public) = key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .key_id(b"key-1")
        .build_protected()
        .unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"hello")
        .sign(&secret)
        .unwrap();

    assert_allocs(0, "parse + headers + verify_with_scratch", || {
        let message = Sign1::from_slice(&token).unwrap();
        let _ = message.headers().unwrap();
        let mut scratch = [0u8; 256];
        message
            .verify_with_scratch(&public, b"", &VerifyOptions::default(), &mut scratch)
            .unwrap();
    });

    assert_allocs(1, "to_vec", || {
        let message = Sign1::from_slice(&token).unwrap();
        let _ = message.to_vec();
    });

    assert_allocs(1, "verify (alloc scratch)", || {
        let message = Sign1::from_slice(&token).unwrap();
        message.verify(&public, b"", &VerifyOptions::default()).unwrap();
    });

    assert_allocs(2, "header build + sign (Ed25519)", || {
        let protected = Header::builder()
            .algorithm(Algorithm::Ed25519)
            .build_protected()
            .unwrap();
        let _ = Sign1Builder::new()
            .protected(protected)
            .payload(b"hello")
            .sign(&secret)
            .unwrap();
    });
}

#[test]
fn zero_alloc_signing_and_mac() {
    let (secret, _) = key();
    let protected = {
        let mut buffer = [0u8; 64];
        let header = Header::builder()
            .algorithm(Algorithm::Ed25519)
            .build_protected_into(&mut buffer)
            .unwrap();
        header.to_vec()
    };

    // Rebuild inside the measured closure, since the returned Header borrows a stack buffer.
    assert_allocs(0, "sign_into", || {
        let mut protected_buf = [0u8; 64];
        let protected = Header::builder()
            .algorithm(Algorithm::Ed25519)
            .build_protected_into(&mut protected_buf)
            .unwrap();
        let mut message = [0u8; 256];
        Sign1Builder::new()
            .protected(protected)
            .payload(b"hello")
            .sign_into(&secret, &mut message)
            .unwrap();
    });
    let _ = protected;

    let secret_bytes = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret_bytes).unwrap();
    assert_allocs(0, "mac0 mac_into", || {
        let mut protected_buf = [0u8; 32];
        let protected = Header::builder()
            .algorithm(Algorithm::Hmac256_256)
            .build_protected_into(&mut protected_buf)
            .unwrap();
        let mut message = [0u8; 128];
        Mac0Builder::new()
            .protected(protected)
            .payload(b"hello")
            .mac_into(&key, &mut message)
            .unwrap();
    });
    assert_allocs(0, "mac mac_into", || {
        let mut protected_buf = [0u8; 32];
        let protected = Header::builder()
            .algorithm(Algorithm::Hmac256_256)
            .build_protected_into(&mut protected_buf)
            .unwrap();
        let mut message = [0u8; 160];
        MacBuilder::new()
            .protected(protected)
            .payload(b"hello")
            .key_id(b"our-secret")
            .mac_into(&key, &mut message)
            .unwrap();
    });
}

#[test]
fn key_allocations() {
    let (_, public) = key();
    assert_allocs(0, "KeyBuilder::build_into", || {
        let mut buffer = [0u8; 128];
        KeyBuilder::new(KeyType::Okp)
            .algorithm(Algorithm::Ed25519)
            .okp(cose::Curve::P256, &[0u8; 32])
            .build_into(&mut buffer)
            .ok();
    });

    let cose_key = CoseKey::from(&public);
    let bytes = cose_key.to_vec();
    assert_allocs(0, "CoseKey::from_slice + accessors", || {
        let key = CoseKey::from_slice(&bytes).unwrap();
        let _ = key.kty().unwrap();
        let _ = key.kid();
        let _ = key.alg();
    });
    assert_allocs(0, "CoseKeySet::encode_into", || {
        let mut buffer = [0u8; 256];
        CoseKeySet::encode_into(&mut buffer, &[CoseKey::from_slice(&bytes).unwrap()]).unwrap();
    });
    assert_allocs(4, "CoseKey::from + KeySetBuilder push + build", || {
        let _ = KeySetBuilder::new().push(CoseKey::from(&public)).build().unwrap();
    });
}

#[test]
fn mac_verify_allocations() {
    let secret_bytes = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret_bytes).unwrap();
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let token = Mac0Builder::new()
        .protected(protected)
        .payload(b"hello")
        .mac(&key)
        .unwrap();

    assert_allocs(0, "mac0 parse + verify_with_scratch", || {
        let message = Mac0::from_slice(&token).unwrap();
        let mut scratch = [0u8; 128];
        message
            .verify_with_scratch(&key, b"", &VerifyOptions::default(), &mut scratch)
            .unwrap();
    });
}
