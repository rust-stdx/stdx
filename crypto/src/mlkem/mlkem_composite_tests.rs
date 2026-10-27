//! Known-answer and behavioral tests for `MLKEM1024-X25519`.
//!
//! The vectors in `testdata/mlkem/mlkem1024_x25519_vectors.json` use the
//! ML-KEM-1024 component values from the first three vectors of Appendix B.3
//! of draft-irtf-cfrg-concrete-hybrid-kems-04. The X25519 outputs and the
//! universal-combiner digest were computed independently from that draft and
//! from Python's SHAKE256/SHA3-256 implementations.

use serde::Deserialize;

use super::*;

const LOW_ORDER_POINTS: [[u8; x25519::KEY_SIZE]; 5] = [
    // u = 0
    [0u8; 32],
    // u = 1
    {
        let mut u = [0u8; 32];
        u[0] = 1;
        u
    },
    // u = p - 1
    [
        0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
    ],
    // order-8 point
    [
        0xe0, 0xeb, 0x7a, 0x7c, 0x3b, 0x41, 0xb8, 0xae, 0x16, 0x56, 0xe3, 0xfa, 0xf1, 0x9f, 0xc4, 0x6a, 0xda, 0x09,
        0x8d, 0xeb, 0x9c, 0x32, 0xb1, 0xfd, 0x86, 0x62, 0x05, 0x16, 0x5f, 0x49, 0xb8, 0x00,
    ],
    // order-8 point
    [
        0x5f, 0x9c, 0x95, 0xbc, 0xa3, 0x50, 0x8c, 0x24, 0xb1, 0xd0, 0xb1, 0x55, 0x9c, 0x83, 0xef, 0x5b, 0x04, 0x44,
        0x5c, 0xc4, 0x58, 0x1c, 0x8e, 0x86, 0xd8, 0x22, 0x4e, 0xdd, 0xd0, 0x9f, 0x11, 0x57,
    ],
];

fn hex_array<const N: usize>(value: &str) -> [u8; N] {
    hex::decode(value)
        .expect("test vector is valid hex")
        .try_into()
        .expect("test vector has the expected length")
}

#[test]
fn constants_match_the_specification() {
    assert_eq!(MLKEM_1024_X25519_SEED_SIZE, 32);
    assert_eq!(MLKEM_1024_X25519_SECRET_KEY_SIZE, 32);
    assert_eq!(MLKEM_1024_X25519_PUBLIC_KEY_SIZE, 1600);
    assert_eq!(MLKEM_1024_X25519_CIPHERTEXT_SIZE, 1600);
    assert_eq!(MLKEM_1024_X25519_SHARED_SECRET_SIZE, 32);
    assert_eq!(MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE, 64);
    assert_eq!(MLKEM_1024_X25519_LABEL, b"MLKEM1024-X25519");
    assert_eq!(MLKEM_1024_X25519_LABEL.len(), 16);
}

#[derive(Deserialize)]
struct Vector {
    seed: String,
    randomness: String,
    decapsulation_key_pq: String,
    decapsulation_key_t: String,
    encapsulation_key: String,
    ciphertext: String,
    shared_secret: String,
}

#[test]
fn draft_derived_known_answer_vectors() {
    let vectors: Vec<Vector> = serde_json::from_str(include_str!("../../testdata/mlkem/mlkem1024_x25519_vectors.json"))
        .expect("test vectors are valid JSON");
    assert_eq!(vectors.len(), 3, "the vector file must contain three vectors");

    for (index, vector) in vectors.iter().enumerate() {
        let seed: [u8; MLKEM_1024_X25519_SEED_SIZE] = hex_array(&vector.seed);
        let randomness: [u8; MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE] = hex_array(&vector.randomness);
        let expected_dk_pq: [u8; 64] = hex_array(&vector.decapsulation_key_pq);
        let expected_dk_t: [u8; x25519::KEY_SIZE] = hex_array(&vector.decapsulation_key_t);
        let expected_ek: [u8; MLKEM_1024_X25519_PUBLIC_KEY_SIZE] = hex_array(&vector.encapsulation_key);
        let expected_ct: [u8; MLKEM_1024_X25519_CIPHERTEXT_SIZE] = hex_array(&vector.ciphertext);
        let expected_ss: [u8; MLKEM_1024_X25519_SHARED_SECRET_SIZE] = hex_array(&vector.shared_secret);

        // The PRG splits SHAKE256(seed) into the ML-KEM-1024 seed and the
        // X25519 scalar seed, PQ first.
        let mut expanded = [0u8; EXPANDED_SEED_SIZE_FOR_TESTS];
        Shake256::hash(&seed, &mut expanded);
        assert_eq!(&expanded[..64], &expected_dk_pq, "vector {index}: PQ seed mismatch");
        assert_eq!(&expanded[64..], &expected_dk_t, "vector {index}: T seed mismatch");

        let (secret_key, public_key) = MlKem1024X25519SecretKey::generate_derand(&seed);
        assert_eq!(secret_key.to_bytes(), seed, "vector {index}: seed mismatch");
        assert_eq!(
            secret_key.x25519_secret_key.to_bytes(),
            expected_dk_t,
            "vector {index}: X25519 secret key mismatch"
        );

        // The ML-KEM-1024 half must expand from the same seed.
        let (_, pq_public_key) = SecretKey1024::generate_derand(&expected_dk_pq);
        assert_eq!(
            &pq_public_key.to_bytes()[..],
            &expected_ek[..PUBLIC_KEY_SIZE_1024],
            "vector {index}: ML-KEM-1024 public key mismatch"
        );

        assert_eq!(public_key.to_bytes(), expected_ek, "vector {index}: ek mismatch");

        let (ss, ct) = public_key
            .encapsulate_derand(&randomness)
            .unwrap_or_else(|err| panic!("vector {index}: encapsulation failed: {err}"));
        assert_eq!(ct, expected_ct, "vector {index}: ciphertext mismatch");
        assert_eq!(ss, expected_ss, "vector {index}: shared secret mismatch");
        assert_eq!(
            secret_key.decapsulate(&ct).unwrap(),
            expected_ss,
            "vector {index}: decapsulation mismatch"
        );
    }
}

/// Size of the SHAKE256 PRG output; kept local to make the test independent of
/// the private production constant.
const EXPANDED_SEED_SIZE_FOR_TESTS: usize = 96;

#[test]
fn round_trip() {
    let (secret_key, public_key) = MlKem1024X25519SecretKey::generate().unwrap();
    let (shared_secret, ciphertext) = public_key.encapsulate().unwrap();
    let decapsulated = secret_key.decapsulate(&ciphertext).unwrap();
    assert_eq!(shared_secret, decapsulated);
}

#[test]
fn round_trip_many() {
    for _ in 0..20 {
        let (secret_key, public_key) = MlKem1024X25519SecretKey::generate().unwrap();
        let (shared_secret, ciphertext) = public_key.encapsulate().unwrap();
        assert_eq!(secret_key.decapsulate(&ciphertext).unwrap(), shared_secret);
    }
}

#[test]
fn derandomized_keygen_is_deterministic() {
    let seed = [7u8; MLKEM_1024_X25519_SEED_SIZE];
    let (secret_key_1, public_key_1) = MlKem1024X25519SecretKey::generate_derand(&seed);
    let (secret_key_2, public_key_2) = MlKem1024X25519SecretKey::generate_derand(&seed);

    assert_eq!(secret_key_1.to_bytes(), seed);
    assert!(secret_key_1 == secret_key_2);
    assert_eq!(public_key_1, public_key_2);
    assert_eq!(public_key_1.to_bytes(), public_key_2.to_bytes());
}

#[test]
fn derandomized_encapsulation_is_deterministic() {
    let (_, public_key) = MlKem1024X25519SecretKey::generate_derand(&[1u8; 32]);
    let randomness = [2u8; MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE];

    let first = public_key.encapsulate_derand(&randomness).unwrap();
    let second = public_key.encapsulate_derand(&randomness).unwrap();
    assert_eq!(first, second);
}

#[test]
fn public_key_encoding_round_trips() {
    let (secret_key, public_key) = MlKem1024X25519SecretKey::generate_derand(&[3u8; 32]);
    let encoded = public_key.to_bytes();

    assert_eq!(MlKem1024X25519PublicKey::from_bytes(&encoded).unwrap(), public_key);
    assert_eq!(secret_key.public_key(), public_key);
}

#[test]
fn tampered_mlkem_ciphertext_changes_the_shared_secret() {
    let (secret_key, public_key) = MlKem1024X25519SecretKey::generate_derand(&[4u8; 32]);
    let (shared_secret, mut ciphertext) = public_key.encapsulate_derand(&[5u8; 64]).unwrap();

    ciphertext[0] ^= 0x80;
    let tampered = secret_key.decapsulate(&ciphertext).unwrap();
    assert_ne!(shared_secret, tampered);
}

#[test]
fn tampered_x25519_ciphertext_changes_the_shared_secret() {
    let (secret_key, public_key) = MlKem1024X25519SecretKey::generate_derand(&[4u8; 32]);
    let (shared_secret, mut ciphertext) = public_key.encapsulate_derand(&[5u8; 64]).unwrap();

    // Replace ct_T with a different, non-low-order point.
    let replacement = x25519::SecretKey::from_bytes(&[11u8; 32]).public_key().to_bytes();
    ciphertext[CIPHERTEXT_SIZE_1024..].copy_from_slice(&replacement);

    let tampered = secret_key.decapsulate(&ciphertext).unwrap();
    assert_ne!(shared_secret, tampered);
}

#[test]
fn a_different_secret_key_does_not_decapsulate_to_the_same_secret() {
    let (alice_secret, alice_public) = MlKem1024X25519SecretKey::generate_derand(&[6u8; 32]);
    let (bob_secret, _) = MlKem1024X25519SecretKey::generate_derand(&[7u8; 32]);
    let (alice_shared, ciphertext) = alice_public.encapsulate_derand(&[8u8; 64]).unwrap();

    assert_ne!(bob_secret.decapsulate(&ciphertext).unwrap(), alice_shared);
    assert_eq!(alice_secret.decapsulate(&ciphertext).unwrap(), alice_shared);
}

#[test]
fn low_order_x25519_ciphertext_is_rejected() {
    let (secret_key, public_key) = MlKem1024X25519SecretKey::generate_derand(&[9u8; 32]);
    let (_, ciphertext) = public_key.encapsulate_derand(&[10u8; 64]).unwrap();

    for point in LOW_ORDER_POINTS {
        let mut tampered = ciphertext;
        tampered[CIPHERTEXT_SIZE_1024..].copy_from_slice(&point);
        assert_eq!(
            secret_key.decapsulate(&tampered),
            Err(MlKem1024X25519Error::InvalidCiphertext),
            "low-order ct_T={} must be rejected",
            hex::encode(point)
        );
    }
}

#[test]
fn non_canonical_low_order_x25519_ciphertext_is_rejected() {
    let (secret_key, public_key) = MlKem1024X25519SecretKey::generate_derand(&[9u8; 32]);
    let (_, ciphertext) = public_key.encapsulate_derand(&[10u8; 64]).unwrap();

    // u = p and u = p + 1 reduce to the low-order points 0 and 1.
    for point in [
        hex_array::<32>("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
        hex_array::<32>("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
    ] {
        let mut tampered = ciphertext;
        tampered[CIPHERTEXT_SIZE_1024..].copy_from_slice(&point);
        assert_eq!(secret_key.decapsulate(&tampered), Err(MlKem1024X25519Error::InvalidCiphertext));
    }
}

#[test]
fn non_canonical_x25519_ciphertext_encoding_is_bound_into_the_combiner() {
    let (secret_key, public_key) = MlKem1024X25519SecretKey::generate_derand(&[17u8; 32]);
    let (_, mut canonical_ct) = public_key.encapsulate_derand(&[18u8; 64]).unwrap();

    // u = 2 and its non-canonical encoding u = p + 2 decode to the same
    // point, so both decapsulate successfully. The combiner binds the exact
    // ciphertext bytes, so the shared secrets must differ.
    let mut reduced = [0u8; 32];
    reduced[0] = 2;
    canonical_ct[CIPHERTEXT_SIZE_1024..].copy_from_slice(&reduced);

    let mut non_canonical_ct = canonical_ct;
    non_canonical_ct[CIPHERTEXT_SIZE_1024..].copy_from_slice(&hex_array::<32>(
        "efffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
    ));

    let reduced_ss = secret_key.decapsulate(&canonical_ct).unwrap();
    let non_canonical_ss = secret_key.decapsulate(&non_canonical_ct).unwrap();
    assert_ne!(reduced_ss, non_canonical_ss);
}

#[test]
fn low_order_x25519_public_key_is_rejected() {
    let (_, public_key) = MlKem1024X25519SecretKey::generate_derand(&[12u8; 32]);
    let valid = public_key.to_bytes();

    for point in LOW_ORDER_POINTS {
        let mut bytes = valid;
        bytes[PUBLIC_KEY_SIZE_1024..].copy_from_slice(&point);
        assert_eq!(
            MlKem1024X25519PublicKey::from_bytes(&bytes),
            Err(MlKem1024X25519Error::InvalidPublicKey),
            "low-order ek_T={} must be rejected",
            hex::encode(point)
        );
    }
}

#[test]
fn non_canonical_low_order_x25519_public_key_is_rejected() {
    let (_, public_key) = MlKem1024X25519SecretKey::generate_derand(&[12u8; 32]);
    let valid = public_key.to_bytes();

    for point in [
        hex_array::<32>("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
        hex_array::<32>("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
    ] {
        let mut bytes = valid;
        bytes[PUBLIC_KEY_SIZE_1024..].copy_from_slice(&point);
        assert_eq!(
            MlKem1024X25519PublicKey::from_bytes(&bytes),
            Err(MlKem1024X25519Error::InvalidPublicKey)
        );
    }
}

#[test]
fn non_canonical_x25519_public_key_is_normalized() {
    let (_, public_key) = MlKem1024X25519SecretKey::generate_derand(&[13u8; 32]);
    let mut bytes = public_key.to_bytes();

    // u = p + 2 reduces to u = 2.
    bytes[PUBLIC_KEY_SIZE_1024..].copy_from_slice(&hex_array::<32>(
        "efffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
    ));

    let parsed = MlKem1024X25519PublicKey::from_bytes(&bytes).unwrap();
    let normalized = parsed.to_bytes();
    let mut expected = [0u8; 32];
    expected[0] = 2;
    assert_eq!(&normalized[PUBLIC_KEY_SIZE_1024..], &expected);
}

#[test]
fn malformed_mlkem_public_key_is_rejected() {
    let (_, public_key) = MlKem1024X25519SecretKey::generate_derand(&[14u8; 32]);
    let mut bytes = public_key.to_bytes();

    // The ML-KEM-1024 modulus is 3329; encode a non-canonical first
    // coefficient (0x0fff).
    bytes[0] = 0xff;
    bytes[1] = (bytes[1] & 0xf0) | 0x0f;
    assert_eq!(
        MlKem1024X25519PublicKey::from_bytes(&bytes),
        Err(MlKem1024X25519Error::InvalidPublicKey)
    );
}

#[test]
fn from_slices_checks_lengths() {
    assert_eq!(
        MlKem1024X25519SecretKey::try_from([0u8; 31].as_slice()).err(),
        Some(MlKem1024X25519Error::InvalidSecretKey)
    );
    assert_eq!(
        MlKem1024X25519PublicKey::try_from([0u8; 1599].as_slice()).err(),
        Some(MlKem1024X25519Error::InvalidPublicKey)
    );
}

#[test]
fn all_zero_and_all_ones_ciphertexts_do_not_panic() {
    let (secret_key, _) = MlKem1024X25519SecretKey::generate_derand(&[15u8; 32]);
    let _ = secret_key.decapsulate(&[0u8; MLKEM_1024_X25519_CIPHERTEXT_SIZE]);
    let _ = secret_key.decapsulate(&[0xffu8; MLKEM_1024_X25519_CIPHERTEXT_SIZE]);
}

#[cfg(feature = "zeroize")]
#[test]
fn secret_key_zeroize_clears_the_seed() {
    use zeroize::Zeroize;

    let (mut secret_key, _) = MlKem1024X25519SecretKey::generate_derand(&[16u8; 32]);
    assert_ne!(secret_key.to_bytes(), [0u8; 32]);
    secret_key.zeroize();
    assert_eq!(secret_key.to_bytes(), [0u8; 32]);
}
