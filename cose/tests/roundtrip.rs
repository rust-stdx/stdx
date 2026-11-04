#![cfg(all(feature = "alloc", feature = "random"))]

use cose::{
    Algorithm, CoseKey, Curve, Header, KeyBuilder, KeyOp, KeySetBuilder, KeyType, Mac0, Mac0Builder, MacBuilder, Macer,
    Sign1, Sign1Builder, SignBuilder, SymmetricKey, VerifyOptions,
};
use crypto::{curve25519::ed25519, mldsa::*, p256, p384, p521};

fn sign_verify(signer: &dyn cose::Signer, verifier: &dyn cose::Verifier, alg: Algorithm) {
    let protected = Header::builder().algorithm(alg).build_protected().unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"hello world")
        .sign(signer)
        .unwrap();

    let message = Sign1::from_slice(&token).unwrap();
    message.verify(verifier, b"", &VerifyOptions::default()).unwrap();
    assert_eq!(message.payload(), Some(&b"hello world"[..]));
}

#[test]
fn ed25519_roundtrip() {
    let secret = ed25519::SecretKey::generate().unwrap();
    let public = secret.public_key();
    sign_verify(&secret, &public, Algorithm::Ed25519);
}

#[test]
fn p256_roundtrip() {
    let secret = p256::SecretKey::generate().unwrap();
    let public = secret.public_key();
    sign_verify(&secret, &public, Algorithm::ESP256);
    sign_verify(&secret, &public, Algorithm::ES256);
}

#[test]
fn p384_roundtrip() {
    let secret = p384::SecretKey::generate().unwrap();
    let public = secret.public_key();
    sign_verify(&secret, &public, Algorithm::ESP384);
}

#[test]
fn p521_roundtrip() {
    let secret = p521::SecretKey::generate().unwrap();
    let public = secret.public_key();
    sign_verify(&secret, &public, Algorithm::ESP512);
}

#[test]
fn mldsa44_roundtrip() {
    let secret = MlDsa44SecretKey::new(&[1u8; 32]);
    let public = secret.public_key();
    sign_verify(&secret, &public, Algorithm::MlDsa44);
}

#[test]
fn mldsa65_roundtrip() {
    let secret = MlDsa65SecretKey::new(&[2u8; 32]);
    let public = secret.public_key();
    sign_verify(&secret, &public, Algorithm::MlDsa65);
}

#[test]
fn mldsa87_roundtrip() {
    let secret = MlDsa87SecretKey::new(&[3u8; 32]);
    let public = secret.public_key();
    sign_verify(&secret, &public, Algorithm::MlDsa87);
}

#[test]
fn detached_payload_and_untagged() {
    let secret = ed25519::SecretKey::generate().unwrap();
    let public = secret.public_key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();

    let token = Sign1Builder::new()
        .protected(protected)
        .detached_payload(b"detached")
        .external_aad(b"ctx")
        .tagged(false)
        .sign(&secret)
        .unwrap();

    let message = Sign1::from_slice(&token).unwrap();
    assert!(!message.is_tagged());
    assert_eq!(message.payload(), None);
    message
        .verify_detached(b"detached", &public, b"ctx", &VerifyOptions::default())
        .unwrap();
}

#[test]
fn sign_with_multiple_signers() {
    let ed = ed25519::SecretKey::generate().unwrap();
    let ed_public = ed.public_key();
    let ec = p256::SecretKey::generate().unwrap();
    let ec_public = ec.public_key();

    let body = Header::builder()
        .content_type("application/json")
        .build_protected()
        .unwrap();
    let token = SignBuilder::new()
        .protected(body)
        .payload(b"{}")
        .add_signature(
            &ed,
            Header::builder()
                .algorithm(Algorithm::Ed25519)
                .build_protected()
                .unwrap(),
            Header::empty_unprotected(),
        )
        .unwrap()
        .add_signature(
            &ec,
            Header::builder()
                .algorithm(Algorithm::ESP256)
                .build_protected()
                .unwrap(),
            Header::empty_unprotected(),
        )
        .unwrap()
        .sign()
        .unwrap();

    let message = cose::Sign::from_slice(&token).unwrap();
    assert_eq!(message.signature_count(), 2);
    assert_eq!(message.verify_any(&ed_public, b"", &VerifyOptions::default()).unwrap(), 0);
    message.verify_any(&ec_public, b"", &VerifyOptions::default()).unwrap();
}

#[test]
fn mac0_roundtrip() {
    let secret = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret).unwrap();
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let token = Mac0Builder::new()
        .protected(protected)
        .payload(b"hello")
        .mac(&key)
        .unwrap();

    let message = Mac0::from_slice(&token).unwrap();
    message.verify(&key, b"", &VerifyOptions::default()).unwrap();
}

#[test]
fn mac_direct_recipient_roundtrip() {
    let secret = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret).unwrap();
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let token = MacBuilder::new()
        .protected(protected)
        .payload(b"hello")
        .key_id(b"our-secret")
        .mac(&key)
        .unwrap();

    let message = cose::Mac::from_slice(&token).unwrap();
    assert_eq!(message.recipient_count(), 1);
    message.verify(&key, b"", &VerifyOptions::default()).unwrap();
}

#[test]
fn cose_key_roundtrip() {
    let secret = p256::SecretKey::generate().unwrap();
    let public = secret.public_key();
    let cose_key = CoseKey::from(&public);
    let bytes = cose_key.to_vec();
    let parsed = CoseKey::from_slice(&bytes).unwrap();
    assert_eq!(parsed.kty().unwrap(), KeyType::Ec2);
    assert_eq!(parsed.alg(), Some(Algorithm::ESP256));
    let recovered = p256::PublicKey::try_from(&parsed).unwrap();
    assert_eq!(recovered.to_bytes(), public.to_bytes());

    let secret_key = CoseKey::from(&secret);
    let secret_key_bytes = secret_key.to_vec();
    let parsed = CoseKey::from_slice(&secret_key_bytes).unwrap();
    let recovered = p256::SecretKey::try_from(&parsed).unwrap();
    assert_eq!(recovered.to_bytes(), secret.to_bytes());
}

#[test]
fn ed25519_key_roundtrip() {
    let secret = ed25519::SecretKey::generate().unwrap();
    let public = secret.public_key();
    let bytes = CoseKey::from(&public).to_vec();
    let parsed = CoseKey::from_slice(&bytes).unwrap();
    assert_eq!(parsed.kty().unwrap(), KeyType::Okp);
    assert_eq!(parsed.okp().unwrap().crv, Curve::Ed25519);
    let recovered = ed25519::PublicKey::try_from(&parsed).unwrap();
    assert_eq!(recovered.to_bytes(), public.to_bytes());
}

#[test]
fn mldsa_key_roundtrip() {
    let secret = MlDsa44SecretKey::new(&[1u8; 32]);
    let public = secret.public_key();
    let bytes = CoseKey::from(&public).to_vec();
    let parsed = CoseKey::from_slice(&bytes).unwrap();
    assert_eq!(parsed.kty().unwrap(), KeyType::Akp);
    assert_eq!(parsed.alg(), Some(Algorithm::MlDsa44));
    let recovered = MlDsa44PublicKey::try_from(&parsed).unwrap();
    assert_eq!(recovered.to_bytes(), public.to_bytes());

    let secret_bytes = CoseKey::from(&secret).to_vec();
    let parsed = CoseKey::from_slice(&secret_bytes).unwrap();
    let recovered = MlDsa44SecretKey::try_from(&parsed).unwrap();
    assert_eq!(recovered.seed(), secret.seed());
}

#[test]
fn key_builder_no_alloc() {
    let public = ed25519::SecretKey::generate().unwrap().public_key();
    let mut buffer = [0u8; 128];
    let key = KeyBuilder::new(KeyType::Okp)
        .algorithm(Algorithm::Ed25519)
        .okp(Curve::Ed25519, &public.to_bytes())
        .build_into(&mut buffer)
        .unwrap();
    let recovered = ed25519::PublicKey::try_from(&key).unwrap();
    assert_eq!(recovered.to_bytes(), public.to_bytes());
}

#[test]
fn key_set_lookup() {
    let public = ed25519::SecretKey::generate().unwrap().public_key();
    let mut key_buf = [0u8; 128];
    let key = KeyBuilder::new(KeyType::Okp)
        .algorithm(Algorithm::Ed25519)
        .key_id(b"key-1")
        .okp(Curve::Ed25519, &public.to_bytes())
        .build_into(&mut key_buf)
        .unwrap();

    let key_set = KeySetBuilder::new().push(CoseKey::from(&public)).build().unwrap();
    assert!(key_set.find_by_kid(b"key-1").is_none());
    let _ = key;
    let _ = KeyOp::Verify;
}

#[test]
fn sign1_official_vector() {
    // cose-wg/Examples/sign1-tests/sign-pass-01.json
    let message = hex::decode(
        "D28441A0A201260442313154546869732069732074686520636F6E74656E742E584087DB0D2E5571843B78AC33ECB2830DF7B6E0A4D5B7376DE336B23C591C90C425317E56127FBE04370097CE347087B233BF722B64072BEB4486BDA4031D27244F",
    )
    .unwrap();
    let x = hex::decode("bac5b11cad8f99f9c72b05cf4b9e26d244dc189f745228255a219a86d6a09eff").unwrap();
    let y = hex::decode("20138bf82dc1b6d562be0fa54ab7804a3a64b6d72ccfed6b6fb6ed28bbfc117e").unwrap();
    let public = p256::PublicKey::from_x_y(x.as_slice().try_into().unwrap(), y.as_slice().try_into().unwrap()).unwrap();

    let parsed = Sign1::from_slice(&message).unwrap();
    parsed.verify(&public, b"", &VerifyOptions::default()).unwrap();
    assert_eq!(parsed.to_vec(), message);
}

#[test]
fn mac0_official_vector() {
    // cose-wg/Examples/mac0-tests/mac-pass-01.json
    let message = hex::decode(
        "D18441A0A1010554546869732069732074686520636F6E74656E742E5820176DCE14C1E57430C13658233F41DC89AA4FA0FF9B8783F23B0EF51CA6B026BC",
    )
    .unwrap();
    let key_bytes = hex::decode("849B57219DAE48DE646D07DBB533566E976686457C1491BE3A76DCEA6C427188").unwrap();
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &key_bytes).unwrap();
    let parsed = Mac0::from_slice(&message).unwrap();
    parsed.verify(&key, b"", &VerifyOptions::default()).unwrap();
    assert_eq!(parsed.to_vec(), message);
}

#[test]
fn duplicate_header_label_is_rejected() {
    // COSE_Sign1 with unprotected {1: -7, 1: -8}
    let mut message = hex::decode(
        "D28441A0A201260442313154546869732069732074686520636F6E74656E742E584087DB0D2E5571843B78AC33ECB2830DF7B6E0A4D5B7376DE336B23C591C90C425317E56127FBE04370097CE347087B233BF722B64072BEB4486BDA4031D27244F",
    )
    .unwrap();
    // Replace a2 01 26 04 42 31 31 with a2 01 26 01 27 (same length) and drop the extra byte.
    let marker = hex::decode("A2012604423131").unwrap();
    let pos = message.windows(marker.len()).position(|w| w == marker).unwrap();
    let replacement = hex::decode("A201260127").unwrap();
    message.splice(pos..pos + marker.len(), replacement);
    assert!(Sign1::from_slice(&message).unwrap().headers().is_err());
}

#[test]
fn no_alloc_sign_and_verify() {
    let secret = ed25519::SecretKey::generate().unwrap();
    let public = secret.public_key();

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

    let parsed = Sign1::from_slice(&message_buf[..len]).unwrap();
    let mut scratch = [0u8; 256];
    parsed
        .verify_with_scratch(&public, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn verify_with_key_set_alloc() {
    let secret = ed25519::SecretKey::generate().unwrap();
    let public = secret.public_key();

    let mut key_buf = [0u8; 128];
    let key = KeyBuilder::new(KeyType::Okp)
        .algorithm(Algorithm::Ed25519)
        .key_id(b"key-1")
        .okp(Curve::Ed25519, &public.to_bytes())
        .build_into(&mut key_buf)
        .unwrap();
    let key_set = KeySetBuilder::new()
        .push(CoseKey::from_slice(key.as_bytes()).unwrap())
        .build()
        .unwrap();

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

    let message = Sign1::from_slice(&token).unwrap();
    message
        .verify_with_key_set(&key_set, b"", &VerifyOptions::default())
        .unwrap();

    let mut scratch = [0u8; 128];
    message
        .verify_with_key_set_scratch(&key_set, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn mac0_verify_with_key_set() {
    let secret = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret).unwrap();
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let token = Mac0Builder::new()
        .protected(protected)
        .payload(b"hello")
        .mac(&key)
        .unwrap();

    let mut key_buf = [0u8; 128];
    let key = KeyBuilder::new(KeyType::Symmetric)
        .algorithm(Algorithm::Hmac256_256)
        .symmetric(&secret)
        .build_into(&mut key_buf)
        .unwrap();
    let set = KeySetBuilder::new()
        .push(CoseKey::from_slice(key.as_bytes()).unwrap())
        .build()
        .unwrap();

    let message = Mac0::from_slice(&token).unwrap();
    let mut scratch = [0u8; 128];
    message
        .verify_with_key_set_scratch(&set, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn symmetric_cose_key_roundtrip() {
    let secret = [0x42u8; 32];
    let key = SymmetricKey::new(Algorithm::Hmac256_256, &secret).unwrap();
    let cose_key = CoseKey::from(&key);
    let bytes = cose_key.to_vec();
    let parsed = CoseKey::from_slice(&bytes).unwrap();
    assert_eq!(parsed.kty().unwrap(), KeyType::Symmetric);
    let recovered = parsed.symmetric_key(Algorithm::Hmac256_256, KeyOp::MacVerify).unwrap();
    assert!(recovered.verify(b"message", &key.mac(b"message").unwrap()).is_ok());
}
