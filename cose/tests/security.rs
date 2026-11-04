#![cfg(feature = "alloc")]

use cose::{
    Algorithm, CoseKey, CoseKeySet, Curve, Error, Header, KeyBuilder, KeyOp, KeyType, Mac, MacBuilder, Sign1Builder,
    SymmetricKey, VerifyOptions,
};
use crypto::{curve25519::ed25519, p256};

fn symmetric_key() -> SymmetricKey<'static> {
    SymmetricKey::new(Algorithm::Hmac256_256, &[0x42u8; 32]).unwrap()
}

fn mac_token() -> Vec<u8> {
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    MacBuilder::new()
        .protected(protected)
        .payload(b"hello")
        .key_id(b"k")
        .mac(&symmetric_key())
        .unwrap()
}

/// Returns the offset of the outer recipients array count and the offset of
/// the single recipient array in a message produced by `mac_token`.
fn recipient_offsets(token: &[u8]) -> (usize, usize) {
    let outer = token
        .windows(1)
        .rposition(|window| window == [0x81])
        .expect("recipients array");
    assert_eq!(token[outer + 1], 0x83, "single recipient");
    (outer, outer + 1)
}

#[test]
fn mac_accepts_a_valid_direct_recipient() {
    let token = mac_token();
    let message = Mac::from_slice(&token).unwrap();
    message
        .verify(&symmetric_key(), b"", &VerifyOptions::default())
        .unwrap();
}

#[test]
fn mac_rejects_multiple_recipients() {
    let token = mac_token();
    let (outer, _) = recipient_offsets(&token);
    let mut evil = token.clone();
    evil[outer] = 0x82;
    // Add a second, non-direct recipient.
    evil.extend_from_slice(&[0x83, 0x40, 0xa1, 0x01, 0x22, 0x41, 0x00]);

    let message = Mac::from_slice(&evil).unwrap();
    assert!(matches!(
        message.verify(&symmetric_key(), b"", &VerifyOptions::default()),
        Err(Error::InvalidStructure)
    ));
}

#[test]
fn mac_rejects_nonempty_protected_recipient() {
    let token = mac_token();
    let (_, recipient) = recipient_offsets(&token);
    let mut evil = token.clone();
    // Replace the empty protected bstr with a non-empty private header.
    evil.splice(recipient + 1..recipient + 2, [0x43, 0xa1, 0x08, 0x00]);

    let message = Mac::from_slice(&evil).unwrap();
    assert!(matches!(
        message.verify(&symmetric_key(), b"", &VerifyOptions::default()),
        Err(Error::InvalidStructure)
    ));
}

#[test]
fn mac_rejects_malformed_recipient_headers() {
    let token = mac_token();
    let (_, recipient) = recipient_offsets(&token);
    let mut evil = token.clone();
    // Replace protected with {1: -6, 1: -6}.
    evil.splice(recipient + 1..recipient + 2, [0x45, 0xa2, 0x01, 0x25, 0x01, 0x25]);

    let message = Mac::from_slice(&evil).unwrap();
    assert!(matches!(
        message.verify(&symmetric_key(), b"", &VerifyOptions::default()),
        Err(Error::DuplicateLabel)
    ));
}

#[test]
fn mac_rejects_cross_bucket_duplicate_kid() {
    let token = mac_token();
    let (_, recipient) = recipient_offsets(&token);
    let mut evil = token.clone();
    // Protected {4: h'a'}, unprotected already has {4: h'k'}.
    evil.splice(recipient + 1..recipient + 2, [0x44, 0xa1, 0x04, 0x41, 0x61]);

    let message = Mac::from_slice(&evil).unwrap();
    assert!(matches!(
        message.verify(&symmetric_key(), b"", &VerifyOptions::default()),
        Err(Error::DuplicateLabel)
    ));
}

#[test]
fn mac_rejects_nonempty_and_nil_ciphertext() {
    let token = mac_token();
    let end = token.len();

    let mut nonempty = token.clone();
    nonempty.splice(end - 1..end, [0x41, 0x00]);
    let message = Mac::from_slice(&nonempty).unwrap();
    assert!(matches!(
        message.verify(&symmetric_key(), b"", &VerifyOptions::default()),
        Err(Error::InvalidStructure)
    ));

    let mut nil = token;
    nil[end - 1] = 0xf6;
    let message = Mac::from_slice(&nil).unwrap();
    assert!(matches!(
        message.verify(&symmetric_key(), b"", &VerifyOptions::default()),
        Err(Error::InvalidStructure)
    ));
}

#[test]
fn mac_builder_rejects_empty_recipient_kid() {
    let protected = Header::builder()
        .algorithm(Algorithm::Hmac256_256)
        .build_protected()
        .unwrap();
    let result = MacBuilder::new()
        .protected(protected)
        .payload(b"hello")
        .key_id(b"")
        .mac(&symmetric_key());
    assert!(matches!(result, Err(Error::InvalidKey)));
}

#[test]
fn mac_rejects_nested_recipients() {
    let token = mac_token();
    let (_, recipient) = recipient_offsets(&token);
    let mut evil = token.clone();
    evil[recipient] = 0x84;
    evil.push(0x80);

    let message = Mac::from_slice(&evil).unwrap();
    assert!(matches!(
        message.verify(&symmetric_key(), b"", &VerifyOptions::default()),
        Err(Error::InvalidStructure)
    ));
}

#[test]
fn cose_key_validate_rejects_bad_parameter_types() {
    // {1: 4, -1: h'00' * 16, 2: 123}
    let mut key = vec![0xa3, 0x01, 0x04, 0x20, 0x50];
    key.extend_from_slice(&[0u8; 16]);
    key.extend_from_slice(&[0x02, 0x18, 0x7b]);
    assert!(matches!(CoseKey::from_slice(&key).unwrap().validate(), Err(Error::InvalidKey)));

    // {1: 4, -1: h'00' * 16, 4: 5}
    let mut key = vec![0xa3, 0x01, 0x04, 0x20, 0x50];
    key.extend_from_slice(&[0u8; 16]);
    key.extend_from_slice(&[0x04, 0x05]);
    assert!(matches!(CoseKey::from_slice(&key).unwrap().validate(), Err(Error::InvalidKey)));
}

#[test]
fn cose_key_validate_rejects_weak_rsa_key() {
    // {1: 3, -1: h'01', -2: h'03'}
    let key = [0xa3, 0x01, 0x03, 0x20, 0x41, 0x01, 0x21, 0x41, 0x03];
    let key = CoseKey::from_slice(&key).unwrap();
    assert!(matches!(key.validate(), Err(Error::InvalidKey)));
}

#[test]
fn cose_key_validate_rejects_invalid_ec_point() {
    // {1: 2, -1: 1, -2: h'01'*32, -3: h'01'*32}
    let mut key = vec![0xa4, 0x01, 0x02, 0x20, 0x01, 0x21, 0x58, 0x20];
    key.extend_from_slice(&[1u8; 32]);
    key.extend_from_slice(&[0x22, 0x58, 0x20]);
    key.extend_from_slice(&[1u8; 32]);
    let key = CoseKey::from_slice(&key).unwrap();
    assert!(matches!(key.validate(), Err(Error::InvalidKey)));
}

#[test]
fn conversions_honor_key_ops_and_algorithm() {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let public = secret.public_key();

    let key = KeyBuilder::new(KeyType::Okp)
        .algorithm(Algorithm::Ed25519)
        .okp(Curve::Ed25519, &public.to_bytes())
        .key_op(KeyOp::Sign)
        .build()
        .unwrap();
    assert!(matches!(
        key.validate_for(Algorithm::Ed25519, KeyOp::Verify),
        Err(Error::UnsupportedKeyOperation(_))
    ));
    assert!(ed25519::PublicKey::try_from(&key).is_err());

    let key = KeyBuilder::new(KeyType::Okp)
        .algorithm(Algorithm::Ed25519)
        .okp(Curve::Ed25519, &public.to_bytes())
        .key_op(KeyOp::Verify)
        .build()
        .unwrap();
    assert!(ed25519::PublicKey::try_from(&key).is_ok());

    let wrong_alg = KeyBuilder::new(KeyType::Okp)
        .algorithm(Algorithm::ESP256)
        .okp(Curve::Ed25519, &public.to_bytes())
        .build();
    assert!(wrong_alg.is_err());
}

#[test]
fn key_set_verification_attempt_limit() {
    let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
    let public = secret.public_key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"hello")
        .sign(&secret)
        .unwrap();
    let message = cose::Sign1::from_slice(&token).unwrap();

    let mut builder = cose::KeySetBuilder::new();
    for i in 0..10u8 {
        let other = ed25519::SecretKey::from_bytes(&[i + 1; 32]).public_key();
        builder = builder.push(CoseKey::from(&other));
    }
    let key_set = builder.build().unwrap();

    let mut scratch = [0u8; 256];
    let options = VerifyOptions {
        max_key_candidates: Some(2),
        ..Default::default()
    };
    assert!(matches!(
        message.verify_with_key_set_scratch(&key_set, b"", &options, &mut scratch),
        Err(Error::TooManyAttempts {
            limit: 2
        })
    ));

    // The key is present but beyond the default budget only when no kid is
    // supplied; a matching key in the first slots still verifies.
    let mut builder = cose::KeySetBuilder::new().push(CoseKey::from(&public));
    for i in 0..10u8 {
        let other = ed25519::SecretKey::from_bytes(&[i + 1; 32]).public_key();
        builder = builder.push(CoseKey::from(&other));
    }
    let key_set = builder.build().unwrap();
    message
        .verify_with_key_set_scratch(&key_set, b"", &VerifyOptions::default(), &mut scratch)
        .unwrap();
}

#[test]
fn default_verification_budget_is_conservative() {
    let options = VerifyOptions::default();
    assert_eq!(options.max_key_candidates, Some(cose::DEFAULT_MAX_KEY_CANDIDATES));
    assert_eq!(options.max_signature_attempts, Some(cose::DEFAULT_MAX_SIGNATURE_ATTEMPTS));
}

#[test]
fn p256_conversion_rejects_incompatible_algorithm() {
    let secret = p256::SecretKey::from_bytes(&[7u8; 32]).unwrap();
    let public = secret.public_key();

    // A valid EC2 key whose declared algorithm cannot be used with EC2.
    let (x, y) = public.x_y();
    let key = KeyBuilder::new(KeyType::Ec2)
        .algorithm(Algorithm::Ed25519)
        .ec2(Curve::P256, &x, cose::YCoordinate::Bytes(&y))
        .build();
    assert!(key.is_err());
}

#[test]
fn key_set_from_slice_is_lazy_about_key_contents() {
    // A one-element key set containing an empty key map.
    let raw = [0x81, 0xa0];
    let set = CoseKeySet::from_slice(&raw).unwrap();
    assert_eq!(set.as_bytes(), &raw);
    assert_eq!(set.len(), 1);
    assert!(!set.is_empty());
    let key = set.iter().next().unwrap().unwrap();
    assert!(matches!(key.validate(), Err(Error::InvalidKey)));
}
