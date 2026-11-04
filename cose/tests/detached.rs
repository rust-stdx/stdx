#![cfg(feature = "alloc")]

use cose::{Algorithm, Header, Mac, Mac0, Mac0Builder, MacBuilder, Sign, SignBuilder, SymmetricKey, VerifyOptions};
use crypto::curve25519::ed25519;

fn ed25519_key() -> (ed25519::SecretKey, ed25519::PublicKey) {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let public = secret.public_key();
    (secret, public)
}

fn mac_key() -> SymmetricKey<'static> {
    SymmetricKey::new(Algorithm::Hmac256_256, &[0x42u8; 32]).unwrap()
}

#[test]
fn sign_detached_and_untagged_roundtrip() {
    let (secret, public) = ed25519_key();
    let signer_protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();
    let body_protected = Header::builder()
        .content_type("application/json")
        .build_protected()
        .unwrap();
    let token = SignBuilder::new()
        .protected(body_protected)
        .detached_payload(b"payload")
        .tagged(false)
        .add_signature(&secret, signer_protected, Header::empty_unprotected())
        .unwrap()
        .sign()
        .unwrap();

    let message = Sign::from_slice(&token).unwrap();
    assert!(!message.is_tagged());
    assert_eq!(message.payload(), None);
    let mut scratch = [0u8; 256];
    message
        .verify_detached_with_scratch(0, b"payload", &public, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
    assert_eq!(
        message
            .verify_any_detached_with_scratch(b"payload", &public, b"", &VerifyOptions::default(), &mut scratch)
            .unwrap(),
        0
    );
    message
        .verify_any_detached(b"payload", &public, b"", &VerifyOptions::default())
        .unwrap();
    assert!(matches!(
        message.verify_with_scratch(0, &public, b"", &VerifyOptions::default(), &mut scratch),
        Err(cose::Error::DetachedPayloadMissing)
    ));
}

#[test]
fn mac_detached_roundtrip() {
    let key = mac_key();
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let token = MacBuilder::new()
        .protected(protected)
        .detached_payload(b"hello")
        .tagged(false)
        .key_id(b"k")
        .mac(&key)
        .unwrap();

    let message = Mac::from_slice(&token).unwrap();
    assert!(!message.is_tagged());
    assert_eq!(message.payload(), None);
    let mut scratch = [0u8; 128];
    message
        .verify_detached_with_scratch(b"hello", &key, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
    assert!(matches!(
        message.verify_with_scratch(&key, b"", &VerifyOptions::default(), &mut scratch),
        Err(cose::Error::DetachedPayloadMissing)
    ));
}

#[test]
fn mac0_detached_roundtrip() {
    let key = mac_key();
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let token = Mac0Builder::new()
        .protected(protected)
        .detached_payload(b"hello")
        .tagged(false)
        .mac(&key)
        .unwrap();

    let message = Mac0::from_slice(&token).unwrap();
    assert!(!message.is_tagged());
    assert_eq!(message.payload(), None);
    let mut scratch = [0u8; 128];
    message
        .verify_detached_with_scratch(b"hello", &key, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
    assert!(matches!(
        message.verify_with_scratch(&key, b"", &VerifyOptions::default(), &mut scratch),
        Err(cose::Error::DetachedPayloadMissing)
    ));
}
