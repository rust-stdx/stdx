#![cfg(all(feature = "alloc", feature = "random"))]

use cose::{
    Algorithm, Curve, Error, Header, HeaderValue, KeyBuilder, KeyType, Label, Sign1, Sign1Builder, VerifyOptions,
    YCoordinate,
};
use crypto::{curve25519::ed25519, p256};

fn key() -> (ed25519::SecretKey, ed25519::PublicKey) {
    let secret = ed25519::SecretKey::generate().unwrap();
    let public = secret.public_key();
    (secret, public)
}

#[test]
fn crit_must_be_understood() {
    let (secret, public) = key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .critical(Label::from(100))
        .value(Label::from(100), HeaderValue::Text("private"))
        .build_protected()
        .unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"hello")
        .sign(&secret)
        .unwrap();
    let message = Sign1::from_slice(&token).unwrap();

    assert!(matches!(
        message.verify(&public, b"", &VerifyOptions::default()),
        Err(Error::UnknownCriticalHeader)
    ));

    let understood = [Label::from(100)];
    message
        .verify(
            &public,
            b"",
            &VerifyOptions {
                understood_header_labels: &understood,
                ..Default::default()
            },
        )
        .unwrap();
}

#[test]
fn crit_without_matching_parameter_is_rejected() {
    let result = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .critical(Label::from(100))
        .build_protected();
    assert!(matches!(result, Err(Error::InvalidHeader)));
}

#[test]
fn crit_in_unprotected_is_rejected() {
    let result = Header::builder().critical(Label::from(100)).build_unprotected();
    assert!(matches!(result, Err(Error::CriticalHeaderNotProtected)));
}

#[test]
fn duplicate_across_buckets_is_rejected() {
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();
    let unprotected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_unprotected()
        .unwrap();
    assert!(matches!(cose::Headers::new(protected, unprotected), Err(Error::DuplicateLabel)));
}

#[test]
fn require_protected_alg() {
    let (secret, public) = key();
    let unprotected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_unprotected()
        .unwrap();
    let token = Sign1Builder::new()
        .unprotected(unprotected)
        .payload(b"hello")
        .sign(&secret)
        .unwrap();
    let message = Sign1::from_slice(&token).unwrap();
    message.verify(&public, b"", &VerifyOptions::default()).unwrap();
    assert!(matches!(
        message.verify(
            &public,
            b"",
            &VerifyOptions {
                require_protected_alg: true,
                ..Default::default()
            }
        ),
        Err(Error::AlgorithmNotProtected)
    ));
}

#[test]
fn allowed_algorithms() {
    let (secret, public) = key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"hello")
        .sign(&secret)
        .unwrap();
    let message = Sign1::from_slice(&token).unwrap();
    let allowed = [Algorithm::ES256];
    assert!(matches!(
        message.verify(
            &public,
            b"",
            &VerifyOptions {
                allowed_algorithms: Some(&allowed),
                ..Default::default()
            }
        ),
        Err(Error::AlgorithmNotAllowed(_))
    ));
}

#[test]
fn wrong_tag_and_trailing_bytes() {
    let (secret, _) = key();
    let protected = Header::builder()
        .algorithm(Algorithm::Ed25519)
        .build_protected()
        .unwrap();
    let token = Sign1Builder::new()
        .protected(protected)
        .payload(b"hello")
        .sign(&secret)
        .unwrap();

    // Change the tag from 18 (0xd2) to 98 (0xd8 0x62) by replacing the first byte.
    let mut wrong = token.clone();
    wrong[0] = 0xd8;
    wrong.insert(1, 0x62);
    assert!(matches!(Sign1::from_slice(&wrong), Err(Error::WrongTag { .. })));

    let mut trailing = token;
    trailing.push(0);
    assert!(matches!(Sign1::from_slice(&trailing), Err(Error::TrailingBytes)));
}

#[test]
fn compressed_and_private_only_ec2_keys() {
    let secret = p256::SecretKey::generate().unwrap();
    let public = secret.public_key();
    let (x, y) = public.x_y();
    let sign = y[31] & 1 == 1;
    let mut key_buf = [0u8; 128];
    let key = KeyBuilder::new(KeyType::Ec2)
        .algorithm(Algorithm::ESP256)
        .ec2(Curve::P256, &x, YCoordinate::Sign(sign))
        .build_into(&mut key_buf)
        .unwrap();
    let recovered = p256::PublicKey::try_from(&key).unwrap();
    assert_eq!(recovered.to_bytes(), public.to_bytes());

    let mut key_buf = [0u8; 128];
    let key = KeyBuilder::new(KeyType::Ec2)
        .algorithm(Algorithm::ESP256)
        .ec2_private(Curve::P256, &secret.to_bytes())
        .build_into(&mut key_buf)
        .unwrap();
    let recovered = p256::PublicKey::try_from(&key).unwrap();
    assert_eq!(recovered.to_bytes(), public.to_bytes());
    let recovered = p256::SecretKey::try_from(&key).unwrap();
    assert_eq!(recovered.to_bytes(), secret.to_bytes());
}

#[test]
fn empty_cose_key_is_rejected() {
    assert!(matches!(CoseKey::from_slice(&[]), Err(Error::InvalidStructure)));
}

use cose::CoseKey;

#[test]
fn header_builder_bytes() {
    let header = Header::builder()
        .algorithm(Algorithm::ES256)
        .key_id(b"11")
        .build_unprotected()
        .unwrap();
    assert_eq!(header.as_bytes(), hex::decode("A2012604423131").unwrap());
}
