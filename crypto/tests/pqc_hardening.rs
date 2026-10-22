//! Malformed-input and boundary regression tests for ML-KEM and ML-DSA.
//!
//! These tests deliberately exercise every ciphertext byte, every public-key
//! coefficient, and random malformed signatures. They complement the
//! known-answer and Wycheproof vectors by checking properties that are easy to
//! regress without changing any valid vector.

use crypto::{
    Xof,
    mldsa::{MlDsa44SecretKey, MlDsa65SecretKey, MlDsa87SecretKey, MlDsaError},
    mlkem::{
        CIPHERTEXT_SIZE_768, CIPHERTEXT_SIZE_1024, MlKemError, PublicKey768, PublicKey1024, SECRET_KEY_SIZE_768,
        SECRET_KEY_SIZE_1024, SecretKey768, SecretKey1024,
    },
    sha3::{Shake128, Shake256},
};

const Q: u16 = 3329;

fn rejection_secret(z: &[u8], ciphertext: &[u8]) -> [u8; 32] {
    let mut hasher = Shake256::new();
    hasher.absorb(z);
    hasher.absorb(ciphertext);
    let mut out = [0u8; 32];
    hasher.squeeze(&mut out);
    out
}

#[test]
fn ml_kem_768_implicit_rejection_covers_every_ciphertext_byte() {
    let (sk, pk) = SecretKey768::generate().unwrap();
    let (_valid_secret, ciphertext) = pk.encapsulate().unwrap();
    let sk_bytes = sk.to_bytes();
    let z = &sk_bytes[SECRET_KEY_SIZE_768 - 32..];

    for offset in 0..CIPHERTEXT_SIZE_768 {
        let mut bad = ciphertext;
        bad[offset] ^= 1;
        let secret = sk.decapsulate(&bad).unwrap();
        assert_eq!(
            secret,
            rejection_secret(z, &bad),
            "ML-KEM-768 implicit rejection mismatch at ciphertext byte {offset}"
        );
    }
}

#[test]
fn ml_kem_1024_implicit_rejection_covers_every_ciphertext_byte() {
    let (sk, pk) = SecretKey1024::generate().unwrap();
    let (_valid_secret, ciphertext) = pk.encapsulate().unwrap();
    let sk_bytes = sk.to_bytes();
    let z = &sk_bytes[SECRET_KEY_SIZE_1024 - 32..];

    for offset in 0..CIPHERTEXT_SIZE_1024 {
        let mut bad = ciphertext;
        bad[offset] ^= 1;
        let secret = sk.decapsulate(&bad).unwrap();
        assert_eq!(
            secret,
            rejection_secret(z, &bad),
            "ML-KEM-1024 implicit rejection mismatch at ciphertext byte {offset}"
        );
    }
}

fn set_coefficient(bytes: &mut [u8], coefficient: usize, value: u16) {
    let offset = coefficient * 3 / 2;
    if coefficient.is_multiple_of(2) {
        bytes[offset] = value as u8;
        bytes[offset + 1] = (bytes[offset + 1] & 0xf0) | ((value >> 8) as u8 & 0x0f);
    } else {
        bytes[offset] = (bytes[offset] & 0x0f) | ((value as u8 & 0x0f) << 4);
        bytes[offset + 1] = (value >> 4) as u8;
    }
}

#[test]
fn ml_kem_modulus_check_rejects_q_at_every_coefficient_position() {
    let (_, pk768) = SecretKey768::generate_derand(&[7u8; 64]);
    let base768 = pk768.to_bytes();
    for coefficient in 0..3 * 256 {
        let mut bytes = base768;
        set_coefficient(&mut bytes, coefficient, Q);
        assert!(
            PublicKey768::from_bytes(&bytes).is_err(),
            "ML-KEM-768 accepted q at coefficient {coefficient}"
        );
    }
    // The in-range boundary must remain accepted.
    let mut bytes = base768;
    set_coefficient(&mut bytes, 0, Q - 1);
    assert!(PublicKey768::from_bytes(&bytes).is_ok());

    let (_, pk1024) = SecretKey1024::generate_derand(&[7u8; 64]);
    let base1024 = pk1024.to_bytes();
    for coefficient in 0..4 * 256 {
        let mut bytes = base1024;
        set_coefficient(&mut bytes, coefficient, Q);
        assert!(
            PublicKey1024::from_bytes(&bytes).is_err(),
            "ML-KEM-1024 accepted q at coefficient {coefficient}"
        );
    }
    let mut bytes = base1024;
    set_coefficient(&mut bytes, 0, Q - 1);
    assert!(PublicKey1024::from_bytes(&bytes).is_ok());
}

#[test]
fn ml_kem_random_ciphertexts_follow_implicit_rejection() {
    let (sk768, _) = SecretKey768::generate_derand(&[1u8; 64]);
    let (sk1024, _) = SecretKey1024::generate_derand(&[2u8; 64]);
    let z768 = {
        let bytes = sk768.to_bytes();
        bytes[SECRET_KEY_SIZE_768 - 32..].to_vec()
    };
    let z1024 = {
        let bytes = sk1024.to_bytes();
        bytes[SECRET_KEY_SIZE_1024 - 32..].to_vec()
    };

    let mut rng = Shake128::new();
    rng.absorb(b"ml-kem-random-ciphertexts");
    for _ in 0..256 {
        let mut ct768 = [0u8; CIPHERTEXT_SIZE_768];
        let mut ct1024 = [0u8; CIPHERTEXT_SIZE_1024];
        rng.squeeze(&mut ct768);
        rng.squeeze(&mut ct1024);

        assert_eq!(sk768.decapsulate(&ct768).unwrap(), rejection_secret(&z768, &ct768));
        assert_eq!(sk1024.decapsulate(&ct1024).unwrap(), rejection_secret(&z1024, &ct1024));
    }
}

#[test]
fn ml_kem_secret_key_length_parsing_is_fallible() {
    assert!(matches!(
        SecretKey768::try_from(&[0u8; SECRET_KEY_SIZE_768 - 1][..]),
        Err(MlKemError::InvalidKey)
    ));
    assert!(matches!(
        SecretKey768::try_from(&[0u8; SECRET_KEY_SIZE_768 + 1][..]),
        Err(MlKemError::InvalidKey)
    ));
    assert!(matches!(
        SecretKey1024::try_from(&[0u8; SECRET_KEY_SIZE_1024 - 1][..]),
        Err(MlKemError::InvalidKey)
    ));
    assert!(matches!(
        SecretKey1024::try_from(&[0u8; SECRET_KEY_SIZE_1024 + 1][..]),
        Err(MlKemError::InvalidKey)
    ));
}

fn random_signature_rejected<const SIG: usize>(rng: &mut Shake128, verify: impl Fn(&[u8; SIG]) -> bool) {
    for _ in 0..128 {
        let mut sig = [0u8; SIG];
        rng.squeeze(&mut sig);
        assert!(!verify(&sig), "random signature was accepted");
    }
}

#[test]
fn ml_dsa_random_signatures_do_not_panic() {
    let seed = [3u8; 32];
    let msg = b"random signature hardening";
    let k44 = MlDsa44SecretKey::new(&seed);
    let k65 = MlDsa65SecretKey::new(&seed);
    let k87 = MlDsa87SecretKey::new(&seed);
    let pk44 = k44.public_key();
    let pk65 = k65.public_key();
    let pk87 = k87.public_key();

    let mut rng = Shake128::new();
    rng.absorb(b"ml-dsa-random-signatures");
    random_signature_rejected(&mut rng, |sig| pk44.verify(msg, sig, b"").is_ok());
    random_signature_rejected(&mut rng, |sig| pk65.verify(msg, sig, b"").is_ok());
    random_signature_rejected(&mut rng, |sig| pk87.verify(msg, sig, b"").is_ok());

    // Corrupted valid signatures must also be rejected without panicking.
    for _ in 0..128 {
        let mut sig = k65.sign_derand(msg, b"", &[1u8; 32]).unwrap();
        let mut index = [0u8; 1];
        rng.squeeze(&mut index);
        sig[index[0] as usize % sig.len()] ^= 1;
        assert!(pk65.verify(msg, &sig, b"").is_err());
    }
}

#[test]
fn ml_dsa_verify_enforces_context_limit() {
    let key = MlDsa65SecretKey::new(&[5u8; 32]);
    let pk = key.public_key();
    let msg = b"context limit";
    let ctx255 = [0xa5u8; 255];
    let ctx256 = [0xa5u8; 256];

    let sig = key.sign_derand(msg, &ctx255, &[1u8; 32]).unwrap();
    assert!(pk.verify(msg, &sig, &ctx255).is_ok());
    assert_eq!(pk.verify(msg, &sig, &ctx256), Err(MlDsaError::ContextTooLong));
    assert_eq!(key.sign_derand(msg, &ctx256, &[1u8; 32]), Err(MlDsaError::ContextTooLong));
}
