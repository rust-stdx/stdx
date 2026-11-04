//! Zero-allocation API tests. These run with or without the `alloc` feature.

use cose::{
    Algorithm, CoseKey, CoseKeySet, Curve, Header, KeyBuilder, KeyType, Mac, Mac0, Mac0Builder, MacBuilder, Sign,
    Sign1, Sign1Builder, SignatureSlot, SymmetricKey, VerifyOptions,
};
use crypto::{curve25519::ed25519, p256};

fn ed25519_key() -> (ed25519::SecretKey, ed25519::PublicKey) {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let public = secret.public_key();
    (secret, public)
}

#[test]
fn sign1_end_to_end_without_alloc() {
    let (secret, public) = ed25519_key();

    let mut protected_buf = [0u8; 64];
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .key_id(b"key-1")
        .build_protected_into(&mut protected_buf)
        .unwrap();

    let mut message_buf = [0u8; 256];
    let len = Sign1Builder::new()
        .protected(protected)
        .payload(b"hello")
        .sign_into(&secret, &mut message_buf)
        .unwrap();

    let message = Sign1::from_slice(&message_buf[..len]).unwrap();
    let mut scratch = [0u8; 256];
    message
        .verify_with_scratch(&public, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
    assert_eq!(message.payload(), Some(&b"hello"[..]));
}

#[cfg(feature = "random")]
#[test]
fn sign1_ml_dsa_with_signature_buffer() {
    let secret = crypto::mldsa::MlDsa44SecretKey::new(&[1u8; 32]);
    let public = secret.public_key();

    let mut protected_buf = [0u8; 32];
    let protected = Header::builder()
        .algorithm(Algorithm::MlDsa44)
        .build_protected_into(&mut protected_buf)
        .unwrap();

    let mut message_buf = [0u8; 4096];
    let mut signature_buf = [0u8; 2420];
    let len = Sign1Builder::new()
        .protected(protected)
        .payload(b"post-quantum")
        .sign_into_with_signature(&secret, &mut message_buf, &mut signature_buf)
        .unwrap();

    let message = Sign1::from_slice(&message_buf[..len]).unwrap();
    let mut scratch = [0u8; 512];
    message
        .verify_with_scratch(&public, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn mac0_without_alloc() {
    let secret = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret).unwrap();

    let mut protected_buf = [0u8; 32];
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected_into(&mut protected_buf)
        .unwrap();

    let mut message_buf = [0u8; 128];
    let len = Mac0Builder::new()
        .protected(protected)
        .payload(b"hello")
        .mac_into(&key, &mut message_buf)
        .unwrap();

    let message = Mac0::from_slice(&message_buf[..len]).unwrap();
    let mut scratch = [0u8; 128];
    message
        .verify_with_scratch(&key, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn mac_direct_without_alloc() {
    let secret = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret).unwrap();

    let mut protected_buf = [0u8; 32];
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected_into(&mut protected_buf)
        .unwrap();

    let mut message_buf = [0u8; 160];
    let len = MacBuilder::new()
        .protected(protected)
        .payload(b"hello")
        .key_id(b"our-secret")
        .mac_into(&key, &mut message_buf)
        .unwrap();

    let message = Mac::from_slice(&message_buf[..len]).unwrap();
    let mut scratch = [0u8; 160];
    message
        .verify_with_scratch(&key, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn multi_signer_without_alloc() {
    let (ed_secret, ed_public) = ed25519_key();
    let p256_secret = p256::SecretKey::from_bytes(&[1u8; 32]).unwrap();
    let p256_public = p256_secret.public_key();

    let mut p1_buf = [0u8; 32];
    let p1 = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected_into(&mut p1_buf)
        .unwrap();
    let mut p2_buf = [0u8; 32];
    let p2 = Header::builder()
        .algorithm(Algorithm::ESP256)
        .build_protected_into(&mut p2_buf)
        .unwrap();

    let mut sig1 = [0u8; 64];
    let mut sig2 = [0u8; 64];
    let mut slots = [
        SignatureSlot::new(&ed_secret, p1, Header::empty_unprotected(), &mut sig1),
        SignatureSlot::new(&p256_secret, p2, Header::empty_unprotected(), &mut sig2),
    ];

    let mut message_buf = [0u8; 4096];
    let len = Sign::sign_into(
        Header::empty_protected(),
        Header::empty_unprotected(),
        b"payload",
        b"",
        true,
        &mut slots,
        &mut message_buf,
    )
    .unwrap();

    let message = Sign::from_slice(&message_buf[..len]).unwrap();
    let mut scratch = [0u8; 512];
    assert_eq!(
        message
            .verify_any_with_scratch(&ed_public, b"", &VerifyOptions::default(), &mut scratch)
            .unwrap(),
        0
    );
    message
        .verify_with_scratch(1, &p256_public, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn key_and_key_set_without_alloc() {
    let (_, public) = ed25519_key();

    let mut key_buf = [0u8; 128];
    let key = KeyBuilder::new(KeyType::Okp)
        .algorithm(Algorithm::Ed25519)
        .okp(Curve::Ed25519, &public.to_bytes())
        .build_into(&mut key_buf)
        .unwrap();
    let recovered = ed25519::PublicKey::try_from(&key).unwrap();
    assert_eq!(recovered.to_bytes(), public.to_bytes());

    assert_eq!(CoseKey::from_slice(key.as_bytes()).unwrap().kty().unwrap(), KeyType::Okp);

    let mut set_buf = [0u8; 256];
    let set = CoseKeySet::encode_into(&mut set_buf, &[key]).unwrap();
    assert_eq!(set.iter().count(), 1);
}
