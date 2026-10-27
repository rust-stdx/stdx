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
    public_key: [u8; MlDsa65Ed25519Sha512PublicKey::SIZE],
    private_key: [u8; MlDsa65Ed25519Sha512SecretKey::SIZE],
    signature: [u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE],
    signature_with_context: [u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE],
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
    let mut buffer = [0u8; MlDsa65Ed25519Sha512PublicKey::MESSAGE_REPRESENTATIVE_MAX_SIZE];
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
    assert_eq!(kat.public_key.len(), MlDsa65Ed25519Sha512PublicKey::SIZE);
    assert_eq!(kat.private_key.len(), MlDsa65Ed25519Sha512SecretKey::SIZE);
    assert_eq!(kat.signature.len(), MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE);
    assert_eq!(kat.signature_with_context.len(), MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE);

    assert_eq!(MlDsa65Ed25519Sha512PublicKey::SIZE, 1984);
    assert_eq!(MlDsa65Ed25519Sha512SecretKey::SIZE, 64);
    assert_eq!(MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE, 3373);
    assert_eq!(MlDsa65Ed25519Sha512PublicKey::CONTEXT_MAX_LEN, 255);
    assert_eq!(MlDsa65Ed25519Sha512PublicKey::MESSAGE_REPRESENTATIVE_MAX_SIZE, 382);
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
        .verify(&m_prime, mldsa_signature, MlDsa65Ed25519Sha512PublicKey::LABEL)
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
    let key = MlDsa65Ed25519Sha512SecretKey::new(&[7u8; MlDsa65Ed25519Sha512SecretKey::SIZE]);
    let public_key = key.public_key();
    let message = b"context boundary";

    for ctx_len in [0usize, 1, 255] {
        let ctx = vec![0x5a; ctx_len];
        let signature = key.sign_derand(message, &ctx, &[0u8; 32]).unwrap();
        public_key.verify(message, &signature, &ctx).unwrap();
    }

    let ctx = vec![0u8; MlDsa65Ed25519Sha512PublicKey::CONTEXT_MAX_LEN + 1];
    assert_eq!(
        key.sign_derand(message, &ctx, &[0u8; 32]).unwrap_err(),
        MlDsa65Ed25519Error::ContextTooLong
    );

    let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
    assert_eq!(
        public_key.verify(message, &signature, &ctx).unwrap_err(),
        MlDsa65Ed25519Error::ContextTooLong
    );

    let mut buffer = [0u8; MlDsa65Ed25519Sha512PublicKey::MESSAGE_REPRESENTATIVE_MAX_SIZE];
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
    let key = MlDsa65Ed25519Sha512SecretKey::new(&[3u8; MlDsa65Ed25519Sha512SecretKey::SIZE]);
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

    let other_key = MlDsa65Ed25519Sha512SecretKey::new(&[4u8; MlDsa65Ed25519Sha512SecretKey::SIZE]);
    assert!(other_key.public_key().verify(message, &signature, ctx).is_err());
}

#[test]
fn empty_and_long_messages() {
    let key = MlDsa65Ed25519Sha512SecretKey::new(&[5u8; MlDsa65Ed25519Sha512SecretKey::SIZE]);
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
    positions.extend((0..MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE).step_by(13));
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

    for position in ML_DSA_65_SIGNATURE_SIZE..MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE {
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
    let other = MlDsa65Ed25519Sha512SecretKey::new(&[9u8; MlDsa65Ed25519Sha512SecretKey::SIZE]);
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
    let other_signature = mldsa_secret_key
        .sign_derand(&other_m_prime, MlDsa65Ed25519Sha512PublicKey::LABEL, &[0u8; 32])
        .unwrap();
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
        MlDsa65Ed25519Sha512PublicKey::try_from(&kat.public_key[..MlDsa65Ed25519Sha512PublicKey::SIZE - 1])
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
        MlDsa65Ed25519Sha512SecretKey::try_from(&kat.private_key[..MlDsa65Ed25519Sha512SecretKey::SIZE - 1]).err(),
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
    assert_ne!(key.to_bytes(), [0u8; MlDsa65Ed25519Sha512SecretKey::SIZE]);

    key.zeroize();
    assert_eq!(key.to_bytes(), [0u8; MlDsa65Ed25519Sha512SecretKey::SIZE]);
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
    assert!(MlDsa65Ed25519Sha512SecretKey::EXPANDED_SIZE >= core::mem::size_of::<MlDsa65SecretKey>());
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA-87 + Ed25519 + ParallelHash256
//////////////////////////////////////////////////////////////////////////////////////////////////

mod ml_dsa87_ed25519_parallelhash256 {
    //! Behavioral tests for the local ML-DSA-87 + Ed25519 + ParallelHash256
    //! composite scheme.
    //!
    //! Draft-19 does not define this pairing or a ParallelHash256 pre-hash, so
    //! there are no published known-answer vectors. These tests instead validate
    //! the construction from its components: the pre-hash is recomputed with the
    //! independent [`ParallelHash256`] implementation, and the two signatures are
    //! checked directly with the ML-DSA-87 and Ed25519 primitives.

    use super::super::*;
    use crate::sha3::ParallelHash256;

    /// Independent reference parameters for the scheme's ParallelHash256.
    const BLOCK_SIZE: usize = 4096;
    const PREHASH_SIZE: usize = 64;

    fn seed() -> [u8; MlDsa87Ed25519SecretKey::SIZE] {
        [7u8; MlDsa87Ed25519SecretKey::SIZE]
    }

    fn reference_prehash(message: &[u8]) -> [u8; PREHASH_SIZE] {
        let mut out = [0u8; PREHASH_SIZE];
        ParallelHash256::hash(message, BLOCK_SIZE, MlDsa87Ed25519MessageRepresentative::LABEL, &mut out);
        out
    }

    fn reference_message_representative(message: &[u8], ctx: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"CompositeAlgorithmSignatures2025");
        out.extend_from_slice(MlDsa87Ed25519MessageRepresentative::LABEL);
        out.push(ctx.len() as u8);
        out.extend_from_slice(ctx);
        out.extend_from_slice(&reference_prehash(message));
        out
    }

    fn message_representative_bytes(message: &[u8], ctx: &[u8]) -> (Vec<u8>, usize) {
        let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, ctx).expect("context is valid");
        (m_prime.as_bytes().to_vec(), m_prime.as_bytes().len())
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // Constants and construction
    //////////////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn scheme_constants() {
        assert_eq!(MlDsa87Ed25519PublicKey::SIZE, 2624);
        assert_eq!(MlDsa87Ed25519SecretKey::SIZE, 64);
        assert_eq!(MlDsa87Ed25519Signature::SIZE, 4691);
        assert_eq!(MlDsa87Ed25519MessageRepresentative::CONTEXT_MAX_LEN, 255);
        assert_eq!(
            MlDsa87Ed25519MessageRepresentative::LABEL,
            b"COMPSIG-MLDSA87-Ed25519-PARALLELHASH256"
        );
        assert_eq!(MlDsa87Ed25519MessageRepresentative::MAX_SIZE, 391);
    }

    #[test]
    fn message_representative_matches_reference() {
        let message = b"the quick brown fox";

        let (m_prime, len) = message_representative_bytes(message, b"");
        assert_eq!(len, 136);
        assert_eq!(m_prime, reference_message_representative(message, b""));
        assert_eq!(&m_prime[..32], b"CompositeAlgorithmSignatures2025");
        assert_eq!(
            &m_prime[32..32 + MlDsa87Ed25519MessageRepresentative::LABEL.len()],
            MlDsa87Ed25519MessageRepresentative::LABEL
        );
        assert_eq!(m_prime[32 + MlDsa87Ed25519MessageRepresentative::LABEL.len()], 0);
        assert_eq!(
            &m_prime[33 + MlDsa87Ed25519MessageRepresentative::LABEL.len()..],
            reference_prehash(message).as_slice()
        );

        let ctx = b"my-application";
        let (m_prime, len) = message_representative_bytes(message, ctx);
        assert_eq!(len, 136 + ctx.len());
        assert_eq!(m_prime, reference_message_representative(message, ctx));

        // Every context length boundary must agree with the reference construction.
        for ctx_len in [0usize, 1, 31, 32, 255] {
            let ctx = vec![0xab; ctx_len];
            let (m_prime, _) = message_representative_bytes(message, &ctx);
            assert_eq!(m_prime, reference_message_representative(message, &ctx));
        }
    }

    #[test]
    fn streaming_prehash_matches_one_shot() {
        let cases: &[(usize, &[usize])] = &[
            (0, &[1, 4096]),
            (1, &[1, 4096]),
            (4095, &[1, 4096, 4097]),
            (4096, &[1, 4096, 4097]),
            (4097, &[1, 4096, 4097]),
            (8192, &[63, 4096, 8192]),
            (10_000, &[63, 4096, 65536]),
            (1 << 20, &[4096, 65536]),
        ];

        for &(len, chunk_sizes) in cases {
            let message = vec![0xa5u8; len];
            let expected = reference_prehash(&message);

            for &chunk_size in chunk_sizes {
                let mut hasher = MlDsa87Ed25519MessageRepresentative::hasher();
                for chunk in message.chunks(chunk_size) {
                    hasher.update(chunk);
                }
                let mut prehash = [0u8; PREHASH_SIZE];
                hasher.finalize_into(&mut prehash);
                assert_eq!(prehash, expected, "message length {len}, chunk size {chunk_size}");
            }

            // The secret key exposes the same configured hasher.
            let mut hasher = MlDsa87Ed25519MessageRepresentative::hasher();
            hasher.update(&message);
            let mut prehash = [0u8; PREHASH_SIZE];
            hasher.finalize_into(&mut prehash);
            assert_eq!(prehash, expected);
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // Key handling
    //////////////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn key_expansion_and_roundtrip() {
        let seed = seed();
        let key = MlDsa87Ed25519SecretKey::new(&seed);
        assert_eq!(key.to_bytes(), seed);

        let public_key = key.public_key();
        assert_eq!(public_key.to_bytes().len(), MlDsa87Ed25519PublicKey::SIZE);
        assert_eq!(MlDsa87Ed25519PublicKey::from_bytes(&public_key.to_bytes()), public_key);

        let rebuilt = MlDsa87Ed25519SecretKey::new(&key.to_bytes());
        assert_eq!(rebuilt.public_key(), public_key);

        let (mldsa_seed, ed25519_seed) = seed.split_at(ML_DSA_SEED_SIZE);
        let from_seeds =
            MlDsa87Ed25519SecretKey::from_seeds(mldsa_seed.try_into().unwrap(), ed25519_seed.try_into().unwrap());
        assert_eq!(from_seeds.to_bytes(), seed);
        assert_eq!(from_seeds.public_key(), public_key);
    }

    #[test]
    fn key_encodings_are_checked() {
        let seed = seed();
        let key = MlDsa87Ed25519SecretKey::new(&seed);
        let public_key = key.public_key().to_bytes();

        assert_eq!(
            MlDsa87Ed25519PublicKey::try_from(&public_key[..]).unwrap().to_bytes(),
            public_key
        );
        assert_eq!(MlDsa87Ed25519SecretKey::try_from(&seed[..]).unwrap().to_bytes(), seed);

        assert_eq!(
            MlDsa87Ed25519PublicKey::try_from(&public_key[..MlDsa87Ed25519PublicKey::SIZE - 1]).unwrap_err(),
            MlDsa87Ed25519Error::InvalidPublicKey
        );
        let mut too_long_public_key = public_key.to_vec();
        too_long_public_key.push(0);
        assert_eq!(
            MlDsa87Ed25519PublicKey::try_from(&too_long_public_key[..]).unwrap_err(),
            MlDsa87Ed25519Error::InvalidPublicKey
        );

        assert_eq!(
            MlDsa87Ed25519SecretKey::try_from(&seed[..MlDsa87Ed25519SecretKey::SIZE - 1]).err(),
            Some(MlDsa87Ed25519Error::InvalidPrivateKey)
        );
        let mut too_long_private_key = seed.to_vec();
        too_long_private_key.push(0);
        assert_eq!(
            MlDsa87Ed25519SecretKey::try_from(&too_long_private_key[..]).err(),
            Some(MlDsa87Ed25519Error::InvalidPrivateKey)
        );
    }

    #[test]
    fn from_arrays() {
        let seed = seed();
        let key = MlDsa87Ed25519SecretKey::new(&seed);
        let public_key = MlDsa87Ed25519PublicKey::from(&key.public_key().to_bytes());
        let secret_key = MlDsa87Ed25519SecretKey::from(&seed);

        assert_eq!(public_key, key.public_key());
        assert_eq!(secret_key.public_key(), public_key);
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // Signing and verification
    //////////////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn roundtrip_and_determinism() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
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

        let other_key = MlDsa87Ed25519SecretKey::new(&[4u8; MlDsa87Ed25519SecretKey::SIZE]);
        assert!(other_key.public_key().verify(message, &signature, ctx).is_err());
    }

    #[test]
    fn prehashed_signing_matches_one_shot() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let public_key = key.public_key();
        let message = b"prehashed signing";
        let ctx = b"ctx";
        let rnd = [9u8; 32];

        let prehash = reference_prehash(message);
        let signature = key.sign_prehashed_derand(&prehash, ctx, &rnd).unwrap();
        assert_eq!(signature, key.sign_derand(message, ctx, &rnd).unwrap());
        public_key.verify_prehashed(&prehash, &signature, ctx).unwrap();
        public_key.verify(message, &signature, ctx).unwrap();

        // A signature over a different pre-hash only verifies against that
        // pre-hash, never against the real message.
        let mut wrong = [0u8; PREHASH_SIZE];
        wrong[0] = 1;
        let wrong_signature = key.sign_prehashed_derand(&wrong, ctx, &rnd).unwrap();
        public_key.verify_prehashed(&wrong, &wrong_signature, ctx).unwrap();
        assert!(public_key.verify_prehashed(&prehash, &wrong_signature, ctx).is_err());
        assert!(public_key.verify(message, &wrong_signature, ctx).is_err());

        // The block size and customization string are part of the domain
        // separation, so digests computed with different parameters must fail.
        let mut other_block_size = [0u8; PREHASH_SIZE];
        ParallelHash256::hash(message, 8192, MlDsa87Ed25519MessageRepresentative::LABEL, &mut other_block_size);
        let signature = key.sign_prehashed_derand(&other_block_size, ctx, &rnd).unwrap();
        assert!(public_key.verify(message, &signature, ctx).is_err());

        let mut empty_customization = [0u8; PREHASH_SIZE];
        ParallelHash256::hash(message, BLOCK_SIZE, b"", &mut empty_customization);
        let signature = key.sign_prehashed_derand(&empty_customization, ctx, &rnd).unwrap();
        assert!(public_key.verify(message, &signature, ctx).is_err());
    }

    #[test]
    fn message_representative_public_api() {
        let message = b"the quick brown fox";

        let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, b"").unwrap();
        assert_eq!(m_prime.as_bytes(), reference_message_representative(message, b"").as_slice());
        let as_ref: &[u8] = m_prime.as_ref();
        assert_eq!(as_ref, m_prime.as_bytes());

        let ctx = b"my-application";
        let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, ctx).unwrap();
        assert_eq!(m_prime.as_bytes(), reference_message_representative(message, ctx).as_slice());

        let prehash = reference_prehash(message);
        let from_prehash = MlDsa87Ed25519MessageRepresentative::from_prehash(&prehash, ctx).unwrap();
        assert_eq!(from_prehash, m_prime);

        for ctx_len in [0usize, 1, 31, 32, 255] {
            let ctx = vec![0xab; ctx_len];
            let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, &ctx).unwrap();
            assert_eq!(m_prime.as_bytes(), reference_message_representative(message, &ctx).as_slice());
        }

        let ctx = vec![0u8; MlDsa87Ed25519MessageRepresentative::CONTEXT_MAX_LEN + 1];
        assert_eq!(
            MlDsa87Ed25519MessageRepresentative::from_message(message, &ctx).unwrap_err(),
            MlDsa87Ed25519Error::ContextTooLong
        );
        assert_eq!(
            MlDsa87Ed25519MessageRepresentative::from_prehash(&prehash, &ctx).unwrap_err(),
            MlDsa87Ed25519Error::ContextTooLong
        );
    }

    #[test]
    fn component_helpers_match_full_sign() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let message = b"component helpers";
        let ctx = b"ctx";
        let rnd = [5u8; 32];
        let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, ctx).unwrap();

        let mldsa_signature = key.sign_mldsa_component(&m_prime, &rnd).unwrap();
        let ed25519_signature = key.sign_ed25519_component(&m_prime);
        let combined = MlDsa87Ed25519Signature::from_parts(&mldsa_signature, &ed25519_signature);

        assert_eq!(combined, key.sign_derand(message, ctx, &rnd).unwrap());
        key.public_key().verify(message, &combined, ctx).unwrap();

        // Each component is a plain signature over M' with the expected domain
        // separation, so external modules can reproduce it byte for byte.
        let public_key_bytes = key.public_key().to_bytes();
        let mldsa_public_key: &[u8; ML_DSA_87_PUBLIC_KEY_SIZE] =
            public_key_bytes[..ML_DSA_87_PUBLIC_KEY_SIZE].try_into().unwrap();
        MlDsa87PublicKey::from_bytes(mldsa_public_key)
            .verify(m_prime.as_bytes(), &mldsa_signature, MlDsa87Ed25519MessageRepresentative::LABEL)
            .unwrap();

        let ed25519_public_key: &[u8; ed25519::PUBLIC_KEY_SIZE] =
            public_key_bytes[ML_DSA_87_PUBLIC_KEY_SIZE..].try_into().unwrap();
        ed25519::PublicKey::from_bytes(ed25519_public_key)
            .unwrap()
            .verify(m_prime.as_bytes(), &ed25519_signature)
            .unwrap();
    }

    #[test]
    fn external_components_reconstruct_signature() {
        // Simulates a secure enclave (ML-DSA-87) and a second module (Ed25519)
        // that each hold one component key and only expose "sign this message".
        let seed = seed();
        let key = MlDsa87Ed25519SecretKey::new(&seed);
        let message = b"externally signed";
        let ctx = b"my-application";

        let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, ctx).unwrap();

        let (mldsa_seed, ed25519_seed) = seed.split_at(ML_DSA_SEED_SIZE);
        let mldsa_signing_key: &[u8; ML_DSA_SEED_SIZE] = mldsa_seed.try_into().unwrap();
        let ed25519_signing_key: &[u8; ed25519::SECRET_KEY_SIZE] = ed25519_seed.try_into().unwrap();

        let mldsa_signature = MlDsa87SecretKey::new(mldsa_signing_key)
            .sign_derand(m_prime.as_bytes(), MlDsa87Ed25519MessageRepresentative::LABEL, &[3u8; 32])
            .unwrap();
        let ed25519_signature = ed25519::SecretKey::from_bytes(ed25519_signing_key).sign(m_prime.as_bytes());

        let signature = MlDsa87Ed25519Signature::from_parts(&mldsa_signature, &ed25519_signature);
        key.public_key().verify(message, &signature, ctx).unwrap();
        assert!(key.public_key().verify(message, &signature, b"other context").is_err());
    }

    #[test]
    fn hybrid_local_and_external_components() {
        let seed = seed();
        let key = MlDsa87Ed25519SecretKey::new(&seed);
        let message = b"hybrid";
        let ctx = b"ctx";
        let rnd = [1u8; 32];
        let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, ctx).unwrap();
        let (mldsa_seed, ed25519_seed) = seed.split_at(ML_DSA_SEED_SIZE);

        // External ML-DSA-87, local Ed25519.
        let mldsa_signing_key: &[u8; ML_DSA_SEED_SIZE] = mldsa_seed.try_into().unwrap();
        let external_mldsa = MlDsa87SecretKey::new(mldsa_signing_key)
            .sign_derand(m_prime.as_bytes(), MlDsa87Ed25519MessageRepresentative::LABEL, &rnd)
            .unwrap();
        let signature = MlDsa87Ed25519Signature::from_parts(&external_mldsa, &key.sign_ed25519_component(&m_prime));
        key.public_key().verify(message, &signature, ctx).unwrap();

        // Local ML-DSA-87, external Ed25519.
        let ed25519_signing_key: &[u8; ed25519::SECRET_KEY_SIZE] = ed25519_seed.try_into().unwrap();
        let external_ed25519 = ed25519::SecretKey::from_bytes(ed25519_signing_key).sign(m_prime.as_bytes());
        let signature =
            MlDsa87Ed25519Signature::from_parts(&key.sign_mldsa_component(&m_prime, &rnd).unwrap(), &external_ed25519);
        key.public_key().verify(message, &signature, ctx).unwrap();

        assert_eq!(signature, key.sign_derand(message, ctx, &rnd).unwrap());
    }

    #[test]
    fn signature_parts_roundtrip() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();

        assert_eq!(MlDsa87Ed25519Signature::SIZE, 4691);
        assert_eq!(signature.as_bytes().len(), MlDsa87Ed25519Signature::SIZE);
        assert_eq!(MlDsa87Ed25519Signature::from_bytes(&signature.to_bytes()), signature);
        assert_eq!(MlDsa87Ed25519Signature::from(&signature.to_bytes()), signature);

        let (mldsa_signature, ed25519_signature) = signature.to_parts();
        assert_eq!(mldsa_signature.len(), ML_DSA_87_SIGNATURE_SIZE);
        assert_eq!(ed25519_signature.len(), ed25519::SIGNATURE_SIZE);
        assert_eq!(
            MlDsa87Ed25519Signature::from_parts(mldsa_signature, ed25519_signature),
            signature
        );

        let bytes = signature.to_bytes();
        assert_eq!(MlDsa87Ed25519Signature::try_from(&bytes[..]).unwrap(), signature);
        assert_eq!(
            MlDsa87Ed25519Signature::try_from(&bytes[..MlDsa87Ed25519Signature::SIZE - 1]).unwrap_err(),
            MlDsa87Ed25519Error::InvalidSignature
        );
        let mut too_long = bytes.to_vec();
        too_long.push(0);
        assert_eq!(
            MlDsa87Ed25519Signature::try_from(&too_long[..]).unwrap_err(),
            MlDsa87Ed25519Error::InvalidSignature
        );
    }

    #[test]
    fn context_bounds() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let public_key = key.public_key();
        let message = b"context boundary";
        let prehash = reference_prehash(message);

        for ctx_len in [0usize, 1, 255] {
            let ctx = vec![0x5a; ctx_len];
            let signature = key.sign_derand(message, &ctx, &[0u8; 32]).unwrap();
            public_key.verify(message, &signature, &ctx).unwrap();

            let signature = key.sign_prehashed_derand(&prehash, &ctx, &[0u8; 32]).unwrap();
            public_key.verify_prehashed(&prehash, &signature, &ctx).unwrap();
        }

        let ctx = vec![0u8; MlDsa87Ed25519MessageRepresentative::CONTEXT_MAX_LEN + 1];
        assert_eq!(
            key.sign_derand(message, &ctx, &[0u8; 32]).unwrap_err(),
            MlDsa87Ed25519Error::ContextTooLong
        );
        assert_eq!(
            key.sign_prehashed_derand(&prehash, &ctx, &[0u8; 32]).unwrap_err(),
            MlDsa87Ed25519Error::ContextTooLong
        );

        let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
        assert_eq!(
            public_key.verify(message, &signature, &ctx).unwrap_err(),
            MlDsa87Ed25519Error::ContextTooLong
        );
        assert_eq!(
            public_key.verify_prehashed(&prehash, &signature, &ctx).unwrap_err(),
            MlDsa87Ed25519Error::ContextTooLong
        );
    }

    #[test]
    fn empty_and_long_messages() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let public_key = key.public_key();

        let messages: Vec<Vec<u8>> = vec![Vec::new(), vec![0x41u8; 10_000], vec![0x5au8; 1 << 20]];
        for message in &messages {
            let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
            public_key.verify(message, &signature, b"").unwrap();

            let mut tampered = message.clone();
            if tampered.is_empty() {
                tampered.push(0);
            } else {
                tampered[0] ^= 1;
            }
            assert!(public_key.verify(&tampered, &signature, b"").is_err());
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // Component binding and malformed input
    //////////////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn component_breakdown() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let message = b"component breakdown";
        let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
        let public_key_bytes = key.public_key().to_bytes();

        let (m_prime, _) = message_representative_bytes(message, b"");
        assert_eq!(m_prime, reference_message_representative(message, b""));

        let mldsa_public_key: &[u8; ML_DSA_87_PUBLIC_KEY_SIZE] =
            public_key_bytes[..ML_DSA_87_PUBLIC_KEY_SIZE].try_into().unwrap();
        let (mldsa_signature, ed25519_signature) = signature.to_parts();
        let ed25519_public_key: &[u8; ed25519::PUBLIC_KEY_SIZE] =
            public_key_bytes[ML_DSA_87_PUBLIC_KEY_SIZE..].try_into().unwrap();

        MlDsa87PublicKey::from_bytes(mldsa_public_key)
            .verify(&m_prime, mldsa_signature, MlDsa87Ed25519MessageRepresentative::LABEL)
            .expect("ML-DSA component must verify with the label as context");
        assert!(
            MlDsa87PublicKey::from_bytes(mldsa_public_key)
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
        ed25519_seed.copy_from_slice(&key.to_bytes()[ML_DSA_SEED_SIZE..]);
        let ed25519_secret_key = ed25519::SecretKey::from_bytes(&ed25519_seed);
        assert_eq!(&ed25519_secret_key.sign(&m_prime), ed25519_signature);
    }

    #[test]
    fn malformed_ed25519_public_key_is_rejected() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();

        // The identity point is a valid curve encoding but has small order, which
        // would allow trivial forgeries if it were accepted.
        let mut public_key_bytes = key.public_key().to_bytes();
        public_key_bytes[ML_DSA_87_PUBLIC_KEY_SIZE..].fill(0);
        public_key_bytes[ML_DSA_87_PUBLIC_KEY_SIZE] = 1;

        let public_key = MlDsa87Ed25519PublicKey::from_bytes(&public_key_bytes);
        assert_eq!(
            public_key.verify(b"message", &signature, b"").unwrap_err(),
            MlDsa87Ed25519Error::InvalidPublicKey
        );
    }

    #[test]
    fn component_failures_are_reported_as_invalid_signature() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let public_key = key.public_key();
        let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();

        let mut bytes = signature.to_bytes();
        bytes[0] ^= 1;
        let tampered_mldsa = MlDsa87Ed25519Signature::from_bytes(&bytes);
        assert_eq!(
            public_key.verify(b"message", &tampered_mldsa, b"").unwrap_err(),
            MlDsa87Ed25519Error::InvalidSignature
        );

        let mut bytes = signature.to_bytes();
        bytes[ML_DSA_87_SIGNATURE_SIZE] ^= 1;
        let tampered_ed25519 = MlDsa87Ed25519Signature::from_bytes(&bytes);
        assert_eq!(
            public_key.verify(b"message", &tampered_ed25519, b"").unwrap_err(),
            MlDsa87Ed25519Error::InvalidSignature
        );
    }

    #[test]
    fn wrong_component_keys_are_rejected() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let message = b"wrong component keys";
        let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();

        let other = MlDsa87Ed25519SecretKey::new(&[9u8; MlDsa87Ed25519SecretKey::SIZE]);
        let other_public_key = other.public_key().to_bytes();

        let mut swapped_mldsa_key = key.public_key().to_bytes();
        swapped_mldsa_key[..ML_DSA_87_PUBLIC_KEY_SIZE].copy_from_slice(&other_public_key[..ML_DSA_87_PUBLIC_KEY_SIZE]);
        assert!(
            MlDsa87Ed25519PublicKey::from_bytes(&swapped_mldsa_key)
                .verify(message, &signature, b"")
                .is_err()
        );

        let mut swapped_ed25519_key = key.public_key().to_bytes();
        swapped_ed25519_key[ML_DSA_87_PUBLIC_KEY_SIZE..]
            .copy_from_slice(&other_public_key[ML_DSA_87_PUBLIC_KEY_SIZE..]);
        assert!(
            MlDsa87Ed25519PublicKey::from_bytes(&swapped_ed25519_key)
                .verify(message, &signature, b"")
                .is_err()
        );
    }

    #[test]
    fn swapped_key_and_signature_halves_are_rejected() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let message = b"swapped halves";
        let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
        let public_key = MlDsa87Ed25519PublicKey::from_bytes(&key.public_key().to_bytes());

        let mut bytes = signature.to_bytes();
        bytes.rotate_left(ML_DSA_87_SIGNATURE_SIZE);
        let swapped_signature = MlDsa87Ed25519Signature::from_bytes(&bytes);
        assert!(public_key.verify(message, &swapped_signature, b"").is_err());

        let mut swapped_key_bytes = key.public_key().to_bytes();
        swapped_key_bytes.rotate_left(ML_DSA_87_PUBLIC_KEY_SIZE);
        let swapped_key = MlDsa87Ed25519PublicKey::from_bytes(&swapped_key_bytes);
        assert!(swapped_key.verify(message, &signature, b"").is_err());
    }

    #[test]
    fn tamper_signature_sampled() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let public_key = key.public_key();
        let message = b"tamper";
        let signature = key.sign_derand(message, b"", &[0u8; 32]).unwrap();
        let mut bytes = signature.to_bytes();

        let mut positions: Vec<usize> = (0..32).collect();
        positions.extend((0..MlDsa87Ed25519Signature::SIZE).step_by(13));
        positions.extend((0..ed25519::SIGNATURE_SIZE).map(|i| ML_DSA_87_SIGNATURE_SIZE + i));
        positions.sort_unstable();
        positions.dedup();

        for position in positions {
            bytes[position] ^= 1;
            let tampered = MlDsa87Ed25519Signature::from_bytes(&bytes);
            assert!(
                public_key.verify(message, &tampered, b"").is_err(),
                "tampered signature byte {position} must fail"
            );
            bytes[position] ^= 1;
        }
    }

    #[test]
    fn standalone_components_do_not_form_a_composite_signature() {
        let key = MlDsa87Ed25519SecretKey::new(&seed());
        let composite_public_key = key.public_key();
        let message = b"standalone components";

        let mut mldsa_seed = [0u8; ML_DSA_SEED_SIZE];
        mldsa_seed.copy_from_slice(&key.to_bytes()[..ML_DSA_SEED_SIZE]);
        let mldsa_secret_key = MlDsa87SecretKey::new(&mldsa_seed);

        // A standalone ML-DSA signature over the raw message (rather than M', and
        // with an empty context) must not be accepted where the composite
        // signature belongs.
        let standalone_signature = mldsa_secret_key.sign_derand(message, b"", &[0u8; 32]).unwrap();
        let mut forged = key.sign_derand(message, b"", &[0u8; 32]).unwrap().to_bytes();
        forged[..ML_DSA_87_SIGNATURE_SIZE].copy_from_slice(&standalone_signature);
        let forged = MlDsa87Ed25519Signature::from_bytes(&forged);
        assert!(composite_public_key.verify(message, &forged, b"").is_err());

        // Same for a component signed over a different message representative.
        let other_m_prime = reference_message_representative(b"other message", b"");
        let other_signature = mldsa_secret_key
            .sign_derand(&other_m_prime, MlDsa87Ed25519MessageRepresentative::LABEL, &[0u8; 32])
            .unwrap();
        let mut forged = key.sign_derand(message, b"", &[0u8; 32]).unwrap().to_bytes();
        forged[..ML_DSA_87_SIGNATURE_SIZE].copy_from_slice(&other_signature);
        let forged = MlDsa87Ed25519Signature::from_bytes(&forged);
        assert!(composite_public_key.verify(message, &forged, b"").is_err());
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // Randomness, zeroization and errors
    //////////////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    #[cfg(feature = "random")]
    fn generate_and_randomized_sign() {
        let key = MlDsa87Ed25519SecretKey::generate().unwrap();
        let other = MlDsa87Ed25519SecretKey::generate().unwrap();

        assert_ne!(key.to_bytes(), other.to_bytes());
        assert_ne!(key.public_key(), other.public_key());
        assert_eq!(MlDsa87Ed25519SecretKey::new(&key.to_bytes()).public_key(), key.public_key());

        let message = b"randomized signing";
        let signature = key.sign(message, b"ctx").unwrap();
        key.public_key().verify(message, &signature, b"ctx").unwrap();

        let prehash = reference_prehash(message);
        let signature = key.sign_prehashed(&prehash, b"ctx").unwrap();
        key.public_key().verify_prehashed(&prehash, &signature, b"ctx").unwrap();

        // Hedged ML-DSA signing should not repeat itself across calls.
        let another_signature = key.sign(message, b"ctx").unwrap();
        assert_ne!(signature, another_signature);
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn secret_key_zeroize_clears_key_material() {
        use zeroize::Zeroize;

        let mut key = MlDsa87Ed25519SecretKey::new(&seed());
        assert_ne!(key.to_bytes(), [0u8; MlDsa87Ed25519SecretKey::SIZE]);

        key.zeroize();
        assert_eq!(key.to_bytes(), [0u8; MlDsa87Ed25519SecretKey::SIZE]);
    }

    #[test]
    fn error_display() {
        assert_eq!(
            MlDsa87Ed25519Error::ContextTooLong.to_string(),
            "context length exceeds 255 bytes"
        );
        assert_eq!(MlDsa87Ed25519Error::InvalidPrivateKey.to_string(), "private key is not valid");
        assert_eq!(MlDsa87Ed25519Error::InvalidPublicKey.to_string(), "public key is not valid");
        assert_eq!(MlDsa87Ed25519Error::InvalidSignature.to_string(), "signature is not valid");
        assert_eq!(
            MlDsa87Ed25519Error::RetryLimitExceeded.to_string(),
            "signing retry limit exceeded"
        );
        assert_eq!(
            MlDsa87Ed25519Error::from(MlDsaError::InvalidSignatureLength),
            MlDsa87Ed25519Error::InvalidSignature
        );
    }
}
