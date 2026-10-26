//! Known-answer and behavioral tests for `id-MLDSA65-Ed25519-SHA512`.
//!
//! The known-answer vectors are an excerpt of the reference vectors cited by
//! draft-ietf-lamps-pq-composite-sigs-19 (Appendix E points to the
//! `lamps-wg/draft-composite-sigs` repository). The local copy of the draft in
//! `crypto/docs` truncates Appendix E before this algorithm, so the vectors
//! were taken from
//! <https://github.com/lamps-wg/draft-composite-sigs/blob/f0627ab34acfe1aee0abce4bee91ed2b577eab76/src/testvectors.json>,
//! keeping only the `id-MLDSA65-Ed25519-SHA512` case and the `m`/`ctx` values
//! it is signed over.

use base64::{Alphabet, decode};
use serde::Deserialize;

use super::*;
use crate::{Hasher, sha2::Sha512};

const VECTORS: &str = include_str!("../../testdata/mldsa/composite_mldsa65_ed25519_sha512.json");

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test vectors
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
struct Vectors {
    m: String,
    ctx: String,
    tests: Vec<TestCase>,
}

#[derive(Deserialize)]
struct TestCase {
    #[serde(rename = "tcId")]
    tc_id: String,
    pk: String,
    sk: String,
    s: String,
    #[serde(rename = "sWithContext")]
    s_with_context: String,
}

struct Kat {
    tc_id: String,
    message: Vec<u8>,
    ctx: Vec<u8>,
    public_key: [u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE],
    private_key: [u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE],
    signature: [u8; ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE],
    signature_with_context: [u8; ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE],
}

fn base64_decode(value: &str) -> Vec<u8> {
    decode(value, Alphabet::Standard).expect("test vector is valid base64")
}

fn to_array<const N: usize>(bytes: Vec<u8>) -> [u8; N] {
    bytes
        .try_into()
        .unwrap_or_else(|bytes: Vec<u8>| panic!("expected {N} bytes, got {}", bytes.len()))
}

fn load_kat() -> Kat {
    let vectors: Vectors = serde_json::from_str(VECTORS).expect("test vectors are valid JSON");

    assert_eq!(
        vectors.tests.len(),
        1,
        "the vector file must contain exactly the id-MLDSA65-Ed25519-SHA512 test case"
    );
    let test = &vectors.tests[0];

    Kat {
        tc_id: test.tc_id.clone(),
        message: base64_decode(&vectors.m),
        ctx: base64_decode(&vectors.ctx),
        public_key: to_array(base64_decode(&test.pk)),
        private_key: to_array(base64_decode(&test.sk)),
        signature: to_array(base64_decode(&test.s)),
        signature_with_context: to_array(base64_decode(&test.s_with_context)),
    }
}

/// Computes the message representative exactly as the spec describes, without
/// going through the implementation helper.
fn reference_message_representative(message: &[u8], ctx: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"CompositeAlgorithmSignatures2025");
    out.extend_from_slice(b"COMPSIG-MLDSA65-Ed25519-SHA512");
    out.push(ctx.len() as u8);
    out.extend_from_slice(ctx);
    out.extend_from_slice(&Sha512::hash(message));
    out
}

fn message_representative_bytes(message: &[u8], ctx: &[u8]) -> (Vec<u8>, usize) {
    let mut buffer = [0u8; MESSAGE_REPRESENTATIVE_MAX_SIZE];
    let len = message_representative(message, ctx, &mut buffer).expect("context is valid");
    (buffer[..len].to_vec(), len)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Known-answer tests
////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn kat_has_expected_identifiers_and_sizes() {
    let kat = load_kat();
    assert_eq!(kat.tc_id, "id-MLDSA65-Ed25519-SHA512");
    assert_eq!(kat.public_key.len(), ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE);
    assert_eq!(kat.private_key.len(), ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE);
    assert_eq!(kat.signature.len(), ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE);
    assert_eq!(kat.signature_with_context.len(), ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE);

    assert_eq!(ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE, 1984);
    assert_eq!(ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE, 64);
    assert_eq!(ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE, 3373);
    assert_eq!(ML_DSA_65_ED25519_SHA512_CONTEXT_MAX_LEN, 255);
    assert_eq!(MESSAGE_REPRESENTATIVE_MAX_SIZE, 382);
}

#[test]
fn kat_key_expansion_matches_public_key() {
    let kat = load_kat();
    let key = MlDsa65Ed25519Sha512SecretKey::new(&kat.private_key);

    assert_eq!(key.to_bytes(), kat.private_key);
    assert_eq!(key.public_key().to_bytes(), kat.public_key);

    // The key can be rebuilt from its serialized form.
    let rebuilt = MlDsa65Ed25519Sha512SecretKey::new(&key.to_bytes());
    assert_eq!(rebuilt.public_key().to_bytes(), kat.public_key);
}

#[test]
fn kat_verify_empty_context() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);

    public_key
        .verify(&kat.message, &kat.signature, b"")
        .expect("known-answer signature must verify with an empty context");
}

#[test]
fn kat_verify_with_context() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);

    public_key
        .verify(&kat.message, &kat.signature_with_context, &kat.ctx)
        .expect("known-answer signature must verify with the vector context");
}

#[test]
fn kat_sign_roundtrip() {
    let kat = load_kat();
    let key = MlDsa65Ed25519Sha512SecretKey::new(&kat.private_key);
    let public_key = key.public_key();

    // ML-DSA signing is hedged, so the generated signature does not have to
    // match the vector byte-for-byte; it must verify.
    let signature = key.sign_derand(&kat.message, &kat.ctx, &[0u8; 32]).unwrap();
    public_key.verify(&kat.message, &signature, &kat.ctx).unwrap();

    let empty_ctx_signature = key.sign_derand(&kat.message, b"", &[0u8; 32]).unwrap();
    public_key.verify(&kat.message, &empty_ctx_signature, b"").unwrap();
}

#[test]
fn kat_component_breakdown() {
    let kat = load_kat();
    let (m_prime, _) = message_representative_bytes(&kat.message, b"");
    assert_eq!(m_prime, reference_message_representative(&kat.message, b""));

    let mldsa_public_key: &[u8; ML_DSA_65_PUBLIC_KEY_SIZE] =
        kat.public_key[..ML_DSA_65_PUBLIC_KEY_SIZE].try_into().unwrap();
    let mldsa_signature: &[u8; ML_DSA_65_SIGNATURE_SIZE] =
        kat.signature[..ML_DSA_65_SIGNATURE_SIZE].try_into().unwrap();
    let ed25519_public_key: &[u8; ed25519::PUBLIC_KEY_SIZE] =
        kat.public_key[ML_DSA_65_PUBLIC_KEY_SIZE..].try_into().unwrap();
    let ed25519_signature: &[u8; ed25519::SIGNATURE_SIZE] =
        kat.signature[ML_DSA_65_SIGNATURE_SIZE..].try_into().unwrap();

    MlDsa65PublicKey::from_bytes(mldsa_public_key)
        .verify(&m_prime, mldsa_signature, LABEL)
        .expect("ML-DSA component must verify with the label as context");
    assert!(
        MlDsa65PublicKey::from_bytes(mldsa_public_key)
            .verify(&m_prime, mldsa_signature, b"")
            .is_err(),
        "the ML-DSA component must be bound to the composite label"
    );

    ed25519::PublicKey::from_bytes(ed25519_public_key)
        .unwrap()
        .verify(&m_prime, ed25519_signature)
        .expect("Ed25519 component must verify in pure mode");

    // Ed25519 is deterministic: re-signing the same representative gives the
    // exact same component signature.
    let mut ed25519_seed = [0u8; ed25519::SECRET_KEY_SIZE];
    ed25519_seed.copy_from_slice(&kat.private_key[ML_DSA_SEED_SIZE..]);
    let ed25519_secret_key = ed25519::SecretKey::from_bytes(&ed25519_seed);
    assert_eq!(&ed25519_secret_key.sign(&m_prime), ed25519_signature);
}

#[test]
fn message_representative_golden_values() {
    // From Appendix D of the draft: the SHA-512 digest of 00 01 .. 09 is
    // 0f89ee...3533, so the representative for that message with an empty
    // context must be the prefix, our label, a zero length byte and that
    // digest.
    let (m_prime, len) = message_representative_bytes(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], b"");
    assert_eq!(len, 127);
    assert_eq!(
        hex::encode(&m_prime),
        concat!(
            "436f6d706f73697465416c676f726974686d5369676e61747572657332303235",
            "434f4d505349472d4d4c44534136352d456432353531392d534841353132",
            "00",
            "0f89ee1fcb7b0a4f7809d1267a029719004c5a5e5ec323a7c3523a20974f9a3f2",
            "02f56fadba4cd9e8d654ab9f2e96dc5c795ea176fa20ede8d854c342f903533",
        )
    );

    let kat = load_kat();

    let (m_prime, len) = message_representative_bytes(&kat.message, b"");
    assert_eq!(len, 127);
    assert_eq!(
        hex::encode(&m_prime),
        concat!(
            "436f6d706f73697465416c676f726974686d5369676e61747572657332303235",
            "434f4d505349472d4d4c44534136352d456432353531392d534841353132",
            "00",
            "91ea1245f20d46ae9a037a989f54f1f790f0a47607eeb8a14d12890cea77a1bbc",
            "6c7ed9cf205e67b7f2b8fd4c7dfd3a7a8617e45f3c463d481c7e586c39ac1ed",
        )
    );

    let (m_prime, len) = message_representative_bytes(&kat.message, &kat.ctx);
    assert_eq!(len, 198);
    assert_eq!(
        hex::encode(&m_prime),
        concat!(
            "436f6d706f73697465416c676f726974686d5369676e61747572657332303235",
            "434f4d505349472d4d4c44534136352d456432353531392d534841353132",
            "47",
            "546865206c65746861726769632c20636f6c6f726c65737320646f672073617420",
            "62656e656174682074686520656e657267657469632c2073746174696f6e617279",
            "20666f782e",
            "91ea1245f20d46ae9a037a989f54f1f790f0a47607eeb8a14d12890cea77a1bbc",
            "6c7ed9cf205e67b7f2b8fd4c7dfd3a7a8617e45f3c463d481c7e586c39ac1ed",
        )
    );

    // The accelerated helper and the straightforward reference construction
    // must agree on every context length boundary.
    for ctx_len in [0usize, 1, 31, 32, 255] {
        let ctx = vec![0xab; ctx_len];
        let (m_prime, _) = message_representative_bytes(&kat.message, &ctx);
        assert_eq!(m_prime, reference_message_representative(&kat.message, &ctx));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Context handling
////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn context_bounds() {
    let key = MlDsa65Ed25519Sha512SecretKey::new(&[7u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]);
    let public_key = key.public_key();
    let message = b"context boundary";

    for ctx_len in [0usize, 1, 255] {
        let ctx = vec![0x5a; ctx_len];
        let signature = key.sign_derand(message, &ctx, &[0u8; 32]).unwrap();
        public_key.verify(message, &signature, &ctx).unwrap();
    }

    let ctx = vec![0u8; ML_DSA_65_ED25519_SHA512_CONTEXT_MAX_LEN + 1];
    assert_eq!(
        key.sign_derand(message, &ctx, &[0u8; 32]).unwrap_err(),
        MlDsa65Ed25519Error::ContextTooLong
    );

    let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
    assert_eq!(
        public_key.verify(message, &signature, &ctx).unwrap_err(),
        MlDsa65Ed25519Error::ContextTooLong
    );

    let mut buffer = [0u8; MESSAGE_REPRESENTATIVE_MAX_SIZE];
    assert_eq!(
        message_representative(message, &ctx, &mut buffer).unwrap_err(),
        MlDsa65Ed25519Error::ContextTooLong
    );
}

#[test]
fn signature_is_bound_to_context() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);

    assert!(public_key.verify(&kat.message, &kat.signature, &kat.ctx).is_err());
    assert!(
        public_key
            .verify(&kat.message, &kat.signature_with_context, b"")
            .is_err()
    );
    assert!(
        public_key
            .verify(&kat.message, &kat.signature_with_context, b"other context")
            .is_err()
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Behavioral tests
////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn roundtrip_and_determinism() {
    let key = MlDsa65Ed25519Sha512SecretKey::new(&[3u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]);
    let public_key = key.public_key();
    let message = b"composite roundtrip";
    let ctx = b"my-application";

    let mut rnd = [0u8; 32];
    for (i, byte) in rnd.iter_mut().enumerate() {
        *byte = (i * 7 + 3) as u8;
    }

    let signature = key.sign_derand(message, ctx, &rnd).unwrap();
    public_key.verify(message, &signature, ctx).unwrap();
    assert_eq!(signature, key.sign_derand(message, ctx, &rnd).unwrap());

    let mut other_rnd = rnd;
    other_rnd[0] ^= 1;
    let other_signature = key.sign_derand(message, ctx, &other_rnd).unwrap();
    assert_ne!(signature, other_signature);
    public_key.verify(message, &other_signature, ctx).unwrap();

    assert!(public_key.verify(b"wrong message", &signature, ctx).is_err());
    assert!(public_key.verify(message, &signature, b"wrong context").is_err());

    let other_key = MlDsa65Ed25519Sha512SecretKey::new(&[4u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]);
    assert!(other_key.public_key().verify(message, &signature, ctx).is_err());
}

#[test]
fn empty_and_long_messages() {
    let key = MlDsa65Ed25519Sha512SecretKey::new(&[5u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]);
    let public_key = key.public_key();

    let messages: Vec<Vec<u8>> = vec![Vec::new(), vec![0x41u8; 10_000], vec![0x5au8; 1 << 20]];
    for message in &messages {
        let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
        public_key.verify(message, &signature, b"").unwrap();

        let mut tampered = message.to_vec();
        if tampered.is_empty() {
            tampered.push(0);
        } else {
            tampered[0] ^= 1;
        }
        assert!(public_key.verify(&tampered, &signature, b"").is_err());
    }
}

#[test]
fn tamper_signature_sampled() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);
    let mut signature = kat.signature;

    let mut positions: Vec<usize> = (0..32).collect();
    positions.extend((0..ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE).step_by(13));
    positions.sort_unstable();
    positions.dedup();

    for position in positions {
        signature[position] ^= 1;
        assert!(
            public_key.verify(&kat.message, &signature, b"").is_err(),
            "tampered signature byte {position} must fail"
        );
        signature[position] ^= 1;
    }
}

#[test]
fn tamper_ed25519_component_exhaustive() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);
    let mut signature = kat.signature;

    for position in ML_DSA_65_SIGNATURE_SIZE..ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE {
        signature[position] ^= 1;
        assert!(
            public_key.verify(&kat.message, &signature, b"").is_err(),
            "tampered Ed25519 signature byte {position} must fail"
        );
        signature[position] ^= 1;
    }
}

#[test]
fn malformed_ed25519_public_key_is_rejected() {
    let kat = load_kat();

    // The identity point is a valid curve encoding but has small order, which
    // would allow trivial forgeries if it were accepted.
    let mut public_key_bytes = kat.public_key;
    public_key_bytes[ML_DSA_65_PUBLIC_KEY_SIZE..].fill(0);
    public_key_bytes[ML_DSA_65_PUBLIC_KEY_SIZE] = 1;

    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&public_key_bytes);
    assert_eq!(
        public_key.verify(&kat.message, &kat.signature, b"").unwrap_err(),
        MlDsa65Ed25519Error::InvalidPublicKey
    );
}

#[test]
fn component_failures_are_reported_as_invalid_signature() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);

    let mut tampered_mldsa = kat.signature;
    tampered_mldsa[0] ^= 1;
    assert_eq!(
        public_key.verify(&kat.message, &tampered_mldsa, b"").unwrap_err(),
        MlDsa65Ed25519Error::InvalidSignature
    );

    let mut tampered_ed25519 = kat.signature;
    tampered_ed25519[ML_DSA_65_SIGNATURE_SIZE] ^= 1;
    assert_eq!(
        public_key.verify(&kat.message, &tampered_ed25519, b"").unwrap_err(),
        MlDsa65Ed25519Error::InvalidSignature
    );
}

#[test]
fn wrong_component_keys_are_rejected() {
    let kat = load_kat();
    let message = &kat.message;

    // A valid signature under a different ML-DSA key must not verify.
    let other = MlDsa65Ed25519Sha512SecretKey::new(&[9u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]);
    let other_public_key = other.public_key().to_bytes();
    let mut swapped_mldsa_key = kat.public_key;
    swapped_mldsa_key[..ML_DSA_65_PUBLIC_KEY_SIZE].copy_from_slice(&other_public_key[..ML_DSA_65_PUBLIC_KEY_SIZE]);
    assert!(
        MlDsa65Ed25519Sha512PublicKey::from_bytes(&swapped_mldsa_key)
            .verify(message, &kat.signature, b"")
            .is_err()
    );

    // A valid signature under a different Ed25519 key must not verify.
    let mut swapped_ed25519_key = kat.public_key;
    swapped_ed25519_key[ML_DSA_65_PUBLIC_KEY_SIZE..].copy_from_slice(&other_public_key[ML_DSA_65_PUBLIC_KEY_SIZE..]);
    assert!(
        MlDsa65Ed25519Sha512PublicKey::from_bytes(&swapped_ed25519_key)
            .verify(message, &kat.signature, b"")
            .is_err()
    );
}

#[test]
fn swapped_key_and_signature_halves_are_rejected() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);

    let mut swapped_signature = kat.signature;
    swapped_signature.rotate_left(ML_DSA_65_SIGNATURE_SIZE);
    assert!(public_key.verify(&kat.message, &swapped_signature, b"").is_err());

    let mut swapped_key_bytes = kat.public_key;
    swapped_key_bytes.rotate_left(ML_DSA_65_PUBLIC_KEY_SIZE);
    let swapped_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&swapped_key_bytes);
    assert!(swapped_key.verify(&kat.message, &kat.signature, b"").is_err());
}

#[test]
fn standalone_components_do_not_form_a_composite_signature() {
    let kat = load_kat();
    let composite_public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&kat.public_key);

    let mut mldsa_seed = [0u8; ML_DSA_SEED_SIZE];
    mldsa_seed.copy_from_slice(&kat.private_key[..ML_DSA_SEED_SIZE]);
    let mldsa_secret_key = MlDsa65SecretKey::new(&mldsa_seed);

    // A standalone ML-DSA signature over the raw message (rather than M', and
    // with an empty context) must not be accepted where the composite
    // signature belongs.
    let standalone_signature = mldsa_secret_key.sign_derand(&kat.message, b"", &[0u8; 32]).unwrap();
    let mut forged = kat.signature;
    forged[..ML_DSA_65_SIGNATURE_SIZE].copy_from_slice(&standalone_signature);
    assert!(composite_public_key.verify(&kat.message, &forged, b"").is_err());

    // Same for a component signed over a different message representative.
    let other_m_prime = reference_message_representative(b"other message", b"");
    let other_signature = mldsa_secret_key.sign_derand(&other_m_prime, LABEL, &[0u8; 32]).unwrap();
    let mut forged = kat.signature;
    forged[..ML_DSA_65_SIGNATURE_SIZE].copy_from_slice(&other_signature);
    assert!(composite_public_key.verify(&kat.message, &forged, b"").is_err());
}

#[test]
fn key_encodings_are_checked() {
    let kat = load_kat();

    assert_eq!(
        MlDsa65Ed25519Sha512PublicKey::try_from(&kat.public_key[..])
            .unwrap()
            .to_bytes(),
        kat.public_key
    );
    assert_eq!(
        MlDsa65Ed25519Sha512SecretKey::try_from(&kat.private_key[..])
            .unwrap()
            .to_bytes(),
        kat.private_key
    );

    assert_eq!(
        MlDsa65Ed25519Sha512PublicKey::try_from(&kat.public_key[..ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE - 1])
            .unwrap_err(),
        MlDsa65Ed25519Error::InvalidPublicKey
    );
    let mut too_long_public_key = kat.public_key.to_vec();
    too_long_public_key.push(0);
    assert_eq!(
        MlDsa65Ed25519Sha512PublicKey::try_from(&too_long_public_key[..]).unwrap_err(),
        MlDsa65Ed25519Error::InvalidPublicKey
    );

    assert_eq!(
        MlDsa65Ed25519Sha512SecretKey::try_from(&kat.private_key[..ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE - 1])
            .err(),
        Some(MlDsa65Ed25519Error::InvalidPrivateKey)
    );
    let mut too_long_private_key = kat.private_key.to_vec();
    too_long_private_key.push(0);
    assert_eq!(
        MlDsa65Ed25519Sha512SecretKey::try_from(&too_long_private_key[..]).err(),
        Some(MlDsa65Ed25519Error::InvalidPrivateKey)
    );
}

#[test]
fn from_arrays() {
    let kat = load_kat();
    let public_key = MlDsa65Ed25519Sha512PublicKey::from(&kat.public_key);
    let secret_key = MlDsa65Ed25519Sha512SecretKey::from(&kat.private_key);

    assert_eq!(public_key.to_bytes(), kat.public_key);
    assert_eq!(secret_key.public_key(), public_key);
}

#[test]
#[cfg(feature = "random")]
fn generate_and_randomized_sign() {
    let key = MlDsa65Ed25519Sha512SecretKey::generate().unwrap();
    let other = MlDsa65Ed25519Sha512SecretKey::generate().unwrap();

    assert_ne!(key.to_bytes(), other.to_bytes());
    assert_ne!(key.public_key(), other.public_key());
    assert_eq!(
        MlDsa65Ed25519Sha512SecretKey::new(&key.to_bytes()).public_key(),
        key.public_key()
    );

    let message = b"randomized signing";
    let signature = key.sign(message, b"ctx").unwrap();
    key.public_key().verify(message, &signature, b"ctx").unwrap();

    // Hedged ML-DSA signing should not repeat itself across calls.
    let another_signature = key.sign(message, b"ctx").unwrap();
    assert_ne!(signature, another_signature);
}

#[test]
#[cfg(feature = "zeroize")]
fn secret_key_zeroize_clears_key_material() {
    use zeroize::Zeroize;

    let kat = load_kat();
    let mut key = MlDsa65Ed25519Sha512SecretKey::new(&kat.private_key);
    assert_ne!(key.to_bytes(), [0u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]);

    key.zeroize();
    assert_eq!(key.to_bytes(), [0u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]);
}

#[test]
fn error_display() {
    assert_eq!(
        MlDsa65Ed25519Error::ContextTooLong.to_string(),
        "context length exceeds 255 bytes"
    );
    assert_eq!(MlDsa65Ed25519Error::InvalidPrivateKey.to_string(), "private key is not valid");
    assert_eq!(MlDsa65Ed25519Error::InvalidPublicKey.to_string(), "public key is not valid");
    assert_eq!(MlDsa65Ed25519Error::InvalidSignature.to_string(), "signature is not valid");
    assert_eq!(
        MlDsa65Ed25519Error::RetryLimitExceeded.to_string(),
        "signing retry limit exceeded"
    );
    assert_eq!(
        MlDsa65Ed25519Error::from(MlDsaError::InvalidSignatureLength),
        MlDsa65Ed25519Error::InvalidSignature
    );
}

#[test]
fn expanded_secret_key_size_is_reported() {
    assert!(ML_DSA_65_ED25519_SHA512_EXPANDED_SECRET_KEY_SIZE >= core::mem::size_of::<MlDsa65SecretKey>());
}
