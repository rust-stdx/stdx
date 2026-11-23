//! Tests for ML-DSA: encoding canonicality, hint-decoder rejection,
//! length parsing, message/context edge cases, and the composite ML-DSA-65 /
//! Ed25519 construction.
//!
//! These complement `crypto/src/mldsa/tests.rs` (ACVP + NIST KAT + Wycheproof)
//! and `crypto/tests/pqc_hardening.rs` (random malformed signatures): this file
//! targets the boundary values, canonicality rules and API length parsing that
//! the vector suites do not enumerate.

use crypto::{
    Xof,
    curve25519::ed25519,
    mldsa::{
        ML_DSA_44_PUBLIC_KEY_SIZE, ML_DSA_44_SIGNATURE_SIZE, ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SIGNATURE_SIZE,
        ML_DSA_87_PUBLIC_KEY_SIZE, ML_DSA_87_SIGNATURE_SIZE, MlDsa44PublicKey, MlDsa44SecretKey, MlDsa65Ed25519Error,
        MlDsa65Ed25519Sha512PublicKey, MlDsa65Ed25519Sha512SecretKey, MlDsa65PublicKey, MlDsa65SecretKey,
        MlDsa87Ed25519Error, MlDsa87Ed25519MessageRepresentative, MlDsa87Ed25519SecretKey, MlDsa87Ed25519Signature,
        MlDsa87PublicKey, MlDsa87SecretKey, MlDsaError,
    },
    sha3::Shake256,
};

const SEED: [u8; 32] = [0x3Cu8; 32];
const RND: [u8; 32] = [0x5Au8; 32];

////////////////////////////////////////////////////////////////////////////////////////////////////
// Message and context sweeps
////////////////////////////////////////////////////////////////////////////////////////////////////

fn deterministic_message(len: usize, byte: u8) -> Vec<u8> {
    let mut msg = Vec::with_capacity(len);
    for i in 0..len {
        msg.push(byte.wrapping_add((i as u8).wrapping_mul(31)));
    }
    msg
}

/// Message lengths around the SHAKE-256 (136) and SHAKE-128 (168) rate
/// boundaries, the empty message, and a message larger than one rate block.
fn message_lengths() -> Vec<usize> {
    vec![
        0, 1, 2, 3, 31, 32, 33, 63, 64, 65, 127, 128, 135, 136, 137, 167, 168, 169, 255, 256, 257, 271, 272, 273, 1024,
        4096,
    ]
}

fn message_roundtrip<const PK: usize, const SIG: usize>(
    name: &str,
    pk: &[u8; PK],
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
    sign: impl Fn(&[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
) {
    for len in message_lengths() {
        for byte in [0x00u8, 0xFF, 0x80] {
            let msg = deterministic_message(len, byte);
            let sig = sign(&msg, b"", &RND).unwrap_or_else(|e| panic!("{name}: sign failed at len={len}: {e}"));
            verify(pk, &msg, &sig, b"")
                .unwrap_or_else(|e| panic!("{name}: verify failed at len={len} byte={byte:#x}: {e}"));

            // Changing the last byte (or appending one) must invalidate it.
            if len > 0 {
                let mut other = msg.clone();
                *other.last_mut().unwrap() ^= 1;
                assert!(
                    verify(pk, &other, &sig, b"").is_err(),
                    "{name}: signature verified a modified message (len={len})"
                );
            }
            let mut longer = msg.clone();
            longer.push(0);
            assert!(
                verify(pk, &longer, &sig, b"").is_err(),
                "{name}: signature verified an extended message (len={len})"
            );
        }
    }
}

#[test]
fn mldsa44_message_lengths_roundtrip() {
    let key = MlDsa44SecretKey::new(&SEED);
    let pk = key.public_key().to_bytes();
    message_roundtrip(
        "mldsa44",
        &pk,
        |pk, msg, sig, ctx| MlDsa44PublicKey::from_bytes(pk).verify(msg, sig, ctx),
        |msg, ctx, rnd| key.sign_derand(msg, ctx, rnd),
    );
}

#[test]
fn mldsa65_message_lengths_roundtrip() {
    let key = MlDsa65SecretKey::new(&SEED);
    let pk = key.public_key().to_bytes();
    message_roundtrip(
        "mldsa65",
        &pk,
        |pk, msg, sig, ctx| MlDsa65PublicKey::from_bytes(pk).verify(msg, sig, ctx),
        |msg, ctx, rnd| key.sign_derand(msg, ctx, rnd),
    );
}

#[test]
fn mldsa87_message_lengths_roundtrip() {
    let key = MlDsa87SecretKey::new(&SEED);
    let pk = key.public_key().to_bytes();
    message_roundtrip(
        "mldsa87",
        &pk,
        |pk, msg, sig, ctx| MlDsa87PublicKey::from_bytes(pk).verify(msg, sig, ctx),
        |msg, ctx, rnd| key.sign_derand(msg, ctx, rnd),
    );
}

/// All-zero, all-ones and empty messages are distinct inputs and must not
/// collide in the message representative.
#[test]
fn degenerate_messages_are_distinct() {
    let key = MlDsa65SecretKey::new(&SEED);
    let pk = key.public_key();

    let empty = key.sign_derand(&[], b"", &RND).unwrap();
    let zero = key.sign_derand(&[0u8; 64], b"", &RND).unwrap();
    let ones = key.sign_derand(&[0xFFu8; 64], b"", &RND).unwrap();

    assert_ne!(empty, zero);
    assert_ne!(zero, ones);
    assert_ne!(empty, ones);

    for (msg, sig) in [(&[][..], &empty), (&[0u8; 64][..], &zero), (&[0xFFu8; 64][..], &ones)] {
        pk.verify(msg, sig, b"").unwrap();
    }

    assert!(pk.verify(&[0u8; 64], &empty, b"").is_err());
    assert!(pk.verify(&[0u8; 63], &zero, b"").is_err());
}

#[test]
fn context_length_boundaries() {
    let key = MlDsa65SecretKey::new(&SEED);
    let pk = key.public_key();
    let msg = b"context boundary";

    for len in [0usize, 1, 2, 254, 255] {
        let ctx = vec![0xA5u8; len];
        let sig = key
            .sign_derand(msg, &ctx, &RND)
            .unwrap_or_else(|e| panic!("ctx len {len} should be accepted: {e}"));
        pk.verify(msg, &sig, &ctx)
            .unwrap_or_else(|e| panic!("ctx len {len} should verify: {e}"));
    }

    // 256 bytes is one too many, on every entry point.
    let ctx256 = vec![0xA5u8; 256];
    assert_eq!(key.sign_derand(msg, &ctx256, &RND), Err(MlDsaError::ContextTooLong));
    let sig = key.sign_derand(msg, &[], &RND).unwrap();
    assert_eq!(pk.verify(msg, &sig, &ctx256), Err(MlDsaError::ContextTooLong));
    // The context check happens before signature verification.
    assert_eq!(
        pk.verify(msg, &[0u8; ML_DSA_65_SIGNATURE_SIZE], &ctx256),
        Err(MlDsaError::ContextTooLong)
    );

    // Empty and zero-filled contexts are different domain separators.
    let ctx_none = key.sign_derand(msg, &[], &RND).unwrap();
    let ctx_zero = key.sign_derand(msg, &[0u8], &RND).unwrap();
    let ctx_two_zero = key.sign_derand(msg, &[0u8, 0u8], &RND).unwrap();
    assert_ne!(ctx_none, ctx_zero);
    assert_ne!(ctx_zero, ctx_two_zero);
    pk.verify(msg, &ctx_none, &[]).unwrap();
    pk.verify(msg, &ctx_zero, &[0u8]).unwrap();
    pk.verify(msg, &ctx_two_zero, &[0u8, 0u8]).unwrap();
    assert!(pk.verify(msg, &ctx_zero, &[]).is_err());
    // 255 zeros and 255 0xFF are different contexts.
    let all_zero = vec![0u8; 255];
    let all_ones = vec![0xFFu8; 255];
    let sig_zero = key.sign_derand(msg, &all_zero, &RND).unwrap();
    let sig_ones = key.sign_derand(msg, &all_ones, &RND).unwrap();
    assert_ne!(sig_zero, sig_ones);
    pk.verify(msg, &sig_zero, &all_zero).unwrap();
    pk.verify(msg, &sig_ones, &all_ones).unwrap();
    assert!(pk.verify(msg, &sig_zero, &all_ones).is_err());
}

/// `sign_external_mu` must be exactly the second half of `sign` once the
/// FIPS 204 message representative has been computed by the caller.
#[test]
fn external_mu_matches_message_signing() {
    fn mu(pk: &[u8], ctx: &[u8], msg: &[u8]) -> [u8; 64] {
        let mut shake = Shake256::new();
        shake.absorb(pk);
        let mut tr = [0u8; 64];
        shake.squeeze(&mut tr);

        let mut shake = Shake256::new();
        shake.absorb(&tr);
        shake.absorb(&[0u8]);
        shake.absorb(&[ctx.len() as u8]);
        shake.absorb(ctx);
        shake.absorb(msg);
        let mut out = [0u8; 64];
        shake.squeeze(&mut out);
        out
    }

    for ctx in [b"".as_slice(), b"ctx".as_slice(), &[0u8; 255][..]] {
        let key = MlDsa65SecretKey::new(&SEED);
        let pk = key.public_key();
        let msg = b"external mu";
        let mu = mu(&pk.to_bytes(), ctx, msg);

        let from_msg = key.sign_derand(msg, ctx, &RND).unwrap();
        let from_mu = key.sign_external_mu_derand(&mu, &RND).unwrap();
        assert_eq!(from_msg, from_mu, "external-mu signing diverged for ctx len {}", ctx.len());
        pk.verify_external_mu(&mu, &from_mu).unwrap();

        let mut wrong_mu = mu;
        wrong_mu[0] ^= 1;
        assert_eq!(pk.verify_external_mu(&wrong_mu, &from_mu), Err(MlDsaError::InvalidSignature));
    }
}

#[test]
fn deterministic_and_hedged_signing() {
    let key = MlDsa65SecretKey::new(&SEED);
    let pk = key.public_key();
    let msg = b"hedged signing";

    let zero = [0u8; 32];
    let a = key.sign_derand(msg, b"", &zero).unwrap();
    let b = key.sign_derand(msg, b"", &zero).unwrap();
    assert_eq!(a, b, "signing with the same rnd must be deterministic");

    let mut rnd = RND;
    let c = key.sign_derand(msg, b"", &rnd).unwrap();
    rnd[31] ^= 1;
    let d = key.sign_derand(msg, b"", &rnd).unwrap();
    assert_ne!(c, d, "different rnd must give a different signature");
    pk.verify(msg, &c, b"").unwrap();
    pk.verify(msg, &d, b"").unwrap();

    // All-zero rnd is the deterministic variant; it must differ from any other
    // rnd and from the randomized convenience wrapper.
    assert_ne!(a, c);

    #[cfg(feature = "random")]
    {
        let r1 = key.sign(msg, b"").unwrap();
        let r2 = key.sign(msg, b"").unwrap();
        assert_ne!(r1, r2, "randomized signing should not repeat");
        pk.verify(msg, &r1, b"").unwrap();
        pk.verify(msg, &r2, b"").unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Encoding and parsing
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Public keys are fixed-size types; every other length must be rejected by the
/// fallible parser instead of being silently truncated or zero-padded.
fn public_key_length_sweep<const PK: usize>(name: &str, parse: impl Fn(&[u8]) -> Result<(), MlDsaError>) {
    let buffer = vec![0xABu8; PK + 2];
    for len in 0..=PK + 2 {
        let result = parse(&buffer[..len]);
        if len == PK {
            result.unwrap_or_else(|e| panic!("{name}: valid length {len} rejected: {e}"));
        } else {
            assert_eq!(
                result,
                Err(MlDsaError::InvalidPublicKey),
                "{name}: length {len} must be rejected"
            );
        }
    }
}

#[test]
fn public_key_length_parsing() {
    public_key_length_sweep::<ML_DSA_44_PUBLIC_KEY_SIZE>("mldsa44", |b| MlDsa44PublicKey::try_from(b).map(|_| ()));
    public_key_length_sweep::<ML_DSA_65_PUBLIC_KEY_SIZE>("mldsa65", |b| MlDsa65PublicKey::try_from(b).map(|_| ()));
    public_key_length_sweep::<ML_DSA_87_PUBLIC_KEY_SIZE>("mldsa87", |b| MlDsa87PublicKey::try_from(b).map(|_| ()));
}

/// An arbitrary (but correctly sized) public key must not panic the verifier and
/// must reject a signature made under a different key.
#[test]
fn zero_and_max_public_keys_fail_cleanly() {
    let key = MlDsa65SecretKey::new(&SEED);
    let msg = b"garbage keys";
    let sig = key.sign_derand(msg, b"", &RND).unwrap();

    for pk in [[0x00u8; ML_DSA_65_PUBLIC_KEY_SIZE], [0xFFu8; ML_DSA_65_PUBLIC_KEY_SIZE]] {
        assert_eq!(
            MlDsa65PublicKey::from_bytes(&pk).verify(msg, &sig, b""),
            Err(MlDsaError::InvalidSignature)
        );
    }

    // A single flipped bit in the public key must be detected.
    let mut pk = key.public_key().to_bytes();
    for offset in [0usize, 31, 32, 100, ML_DSA_65_PUBLIC_KEY_SIZE - 1] {
        let saved = pk[offset];
        pk[offset] ^= 1;
        assert!(
            MlDsa65PublicKey::from_bytes(&pk).verify(msg, &sig, b"").is_err(),
            "modified public key at byte {offset} was accepted"
        );
        pk[offset] = saved;
    }
    MlDsa65PublicKey::from_bytes(&pk).verify(msg, &sig, b"").unwrap();
}

/// Parameters for the hint-encoding section of a signature.
struct Layout {
    name: &'static str,
    hint_start: usize,
    omega: usize,
    k: usize,
    signature_size: usize,
}

const LAYOUT_44: Layout = Layout {
    name: "mldsa44",
    hint_start: 32 + 4 * (18 * 256 / 8),
    omega: 80,
    k: 4,
    signature_size: ML_DSA_44_SIGNATURE_SIZE,
};
const LAYOUT_65: Layout = Layout {
    name: "mldsa65",
    hint_start: 48 + 5 * (20 * 256 / 8),
    omega: 55,
    k: 6,
    signature_size: ML_DSA_65_SIGNATURE_SIZE,
};
const LAYOUT_87: Layout = Layout {
    name: "mldsa87",
    hint_start: 64 + 7 * (20 * 256 / 8),
    omega: 75,
    k: 8,
    signature_size: ML_DSA_87_SIGNATURE_SIZE,
};

impl Layout {
    fn challenge_len(&self) -> usize {
        // lambda/4: 32, 48, 64 for ML-DSA-44/65/87.
        match self.name {
            "mldsa44" => 32,
            "mldsa65" => 48,
            _ => 64,
        }
    }
}

fn hint_encoding_cases(layout: &Layout, valid: &[u8]) -> Vec<(&'static str, Vec<u8>)> {
    let base = layout.hint_start;
    let omega = layout.omega;
    let k = layout.k;
    assert_eq!(valid.len(), layout.signature_size);
    assert_eq!(base + omega + k, valid.len());

    let mut cases = Vec::new();

    // Duplicate hint index inside one polynomial: FIPS 204 Algorithm 24 step 8.
    let mut duplicate = valid.to_vec();
    duplicate[base] = 5;
    duplicate[base + 1] = 5;
    for i in 0..k {
        duplicate[base + omega + i] = 2;
    }
    cases.push(("duplicate hint index", duplicate));

    // Decreasing hint index inside one polynomial.
    let mut decreasing = valid.to_vec();
    decreasing[base] = 9;
    decreasing[base + 1] = 4;
    for i in 0..k {
        decreasing[base + omega + i] = 2;
    }
    cases.push(("decreasing hint index", decreasing));

    // A cumulative count that exceeds omega.
    let mut too_many = valid.to_vec();
    too_many[base + omega + k - 1] = (omega + 1) as u8;
    cases.push(("hint count exceeds omega", too_many));

    // Non-monotone cumulative counts: row 0 claims 5 hints, row 1 claims 1.
    let mut non_monotone = valid.to_vec();
    non_monotone[base] = 5;
    non_monotone[base + omega] = 5;
    non_monotone[base + omega + 1] = 1;
    cases.push(("non-monotone hint counts", non_monotone));

    // Non-zero padding after the last used hint index.
    let mut padding = valid.to_vec();
    for i in 0..k {
        padding[base + omega + i] = 1;
    }
    padding[base + 1] = 0xEE;
    cases.push(("non-zero hint padding", padding));

    cases
}

fn hint_encoding_rejection<const SIG: usize>(
    layout: &Layout,
    valid: [u8; SIG],
    verify: impl Fn(&[u8; SIG]) -> Result<(), MlDsaError>,
) {
    verify(&valid).unwrap_or_else(|e| panic!("{}: baseline signature rejected: {e}", layout.name));

    // The last row's count equals the total number of hints used, so the
    // trailing bytes of the hint section must be zero in every valid signature.
    let total = valid[layout.hint_start + layout.omega + layout.k - 1] as usize;
    assert!(total <= layout.omega);
    for i in total..layout.omega {
        assert_eq!(
            valid[layout.hint_start + i],
            0,
            "{}: valid signature has non-zero hint padding at {i}",
            layout.name
        );
    }

    for (what, mutated) in hint_encoding_cases(layout, &valid) {
        let mutated: [u8; SIG] = mutated.try_into().expect("case keeps the signature length");
        assert_eq!(
            verify(&mutated),
            Err(MlDsaError::InvalidSignature),
            "{}: malformed hint encoding ({what}) was accepted",
            layout.name
        );
    }

    // Exhaustively flip every byte of the hint section.
    for offset in layout.hint_start..layout.signature_size {
        let mut mutated = valid;
        mutated[offset] ^= 0xFF;
        assert!(
            verify(&mutated).is_err(),
            "{}: flipping hint byte {offset} was accepted",
            layout.name
        );
    }

    // Exhaustively flip every byte of the challenge prefix.
    for offset in 0..layout.challenge_len() {
        let mut mutated = valid;
        mutated[offset] ^= 0x01;
        assert!(
            verify(&mutated).is_err(),
            "{}: flipping challenge byte {offset} was accepted",
            layout.name
        );
    }
}

#[test]
fn hint_encoding_canonicality() {
    let key = MlDsa44SecretKey::new(&SEED);
    let pk = key.public_key();
    let sig = key.sign_derand(b"hints", b"", &RND).unwrap();
    hint_encoding_rejection(&LAYOUT_44, sig, |sig| pk.verify(b"hints", sig, b""));

    let key = MlDsa65SecretKey::new(&SEED);
    let pk = key.public_key();
    let sig = key.sign_derand(b"hints", b"", &RND).unwrap();
    hint_encoding_rejection(&LAYOUT_65, sig, |sig| pk.verify(b"hints", sig, b""));

    let key = MlDsa87SecretKey::new(&SEED);
    let pk = key.public_key();
    let sig = key.sign_derand(b"hints", b"", &RND).unwrap();
    hint_encoding_rejection(&LAYOUT_87, sig, |sig| pk.verify(b"hints", sig, b""));
}

/// Every byte of an ML-DSA-44 signature must be covered by the encoding: a
/// one-bit change anywhere is rejected. (ML-DSA-44 is small enough to do this
/// exhaustively; the z and challenge regions of the larger sets are sampled.)
#[test]
fn signature_bits_are_all_significant() {
    let key = MlDsa44SecretKey::new(&SEED);
    let pk = key.public_key();
    let msg = b"bit significance";
    let sig = key.sign_derand(msg, b"", &RND).unwrap();

    // ML-DSA-44: 32-byte challenge + 4*576-byte z + 84-byte hint section.
    for offset in 32..32 + 4 * 576 {
        let mut mutated = sig;
        mutated[offset] ^= 0x01;
        assert!(
            pk.verify(msg, &mutated, b"").is_err(),
            "mldsa44: one-bit change to z byte {offset} was accepted"
        );
    }

    // Sampled sweep over the full signature (every 17th byte, all bits).
    for offset in (0..ML_DSA_44_SIGNATURE_SIZE).step_by(17) {
        for bit in 0..8 {
            let mut mutated = sig;
            mutated[offset] ^= 1 << bit;
            assert!(
                pk.verify(msg, &mutated, b"").is_err(),
                "mldsa44: bit {bit} of byte {offset} is not covered"
            );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Composite ML-DSA-65 + Ed25519
////////////////////////////////////////////////////////////////////////////////////////////////////

const COMPOSITE_SEED: [u8; MlDsa65Ed25519Sha512SecretKey::SIZE] = [0x77u8; 64];

#[test]
fn composite_65_message_and_context_edges() {
    let key = MlDsa65Ed25519Sha512SecretKey::new(&COMPOSITE_SEED);
    let pk = key.public_key();
    assert_eq!(pk.to_bytes(), key.public_key().to_bytes());

    for len in [0usize, 1, 135, 136, 137, 4096] {
        let msg = deterministic_message(len, 0x5A);
        let sig = key.sign_derand(&msg, b"", &RND).unwrap();
        pk.verify(&msg, &sig, b"").unwrap();
    }

    let msg = b"composite context";
    for len in [0usize, 1, 254, 255] {
        let ctx = vec![0x11u8; len];
        let sig = key.sign_derand(msg, &ctx, &RND).unwrap();
        pk.verify(msg, &sig, &ctx).unwrap();
        if len > 0 {
            assert!(pk.verify(msg, &sig, b"").is_err());
        }
    }

    let ctx256 = vec![0x11u8; 256];
    assert_eq!(key.sign_derand(msg, &ctx256, &RND), Err(MlDsa65Ed25519Error::ContextTooLong));
    let sig = key.sign_derand(msg, b"", &RND).unwrap();
    assert_eq!(pk.verify(msg, &sig, &ctx256), Err(MlDsa65Ed25519Error::ContextTooLong));

    // Deterministic ML-DSA component: same rnd, same signature.
    let a = key.sign_derand(msg, b"c", &RND).unwrap();
    let b = key.sign_derand(msg, b"c", &RND).unwrap();
    assert_eq!(a, b);
}

#[test]
fn composite_65_component_tampering() {
    let key = MlDsa65Ed25519Sha512SecretKey::new(&COMPOSITE_SEED);
    let pk = key.public_key();
    let msg = b"component tampering";
    let sig = key.sign_derand(msg, b"", &RND).unwrap();

    // Every Ed25519 signature byte matters.
    let ed_start = ML_DSA_65_SIGNATURE_SIZE;
    for offset in ed_start..ed_start + ed25519::SIGNATURE_SIZE {
        let mut mutated = sig;
        mutated[offset] ^= 0x01;
        assert_eq!(
            pk.verify(msg, &mutated, b""),
            Err(MlDsa65Ed25519Error::InvalidSignature),
            "composite65: tampered Ed25519 byte {offset} accepted"
        );
    }

    // Sampled sweep over the ML-DSA component.
    for offset in (0..ML_DSA_65_SIGNATURE_SIZE).step_by(37) {
        let mut mutated = sig;
        mutated[offset] ^= 0x80;
        assert!(
            pk.verify(msg, &mutated, b"").is_err(),
            "composite65: tampered ML-DSA byte {offset} accepted"
        );
    }

    // Zeroing either half must fail.
    let mut zero_ed = sig;
    zero_ed[ed_start..].fill(0);
    assert!(pk.verify(msg, &zero_ed, b"").is_err());
    let mut zero_ml = sig;
    zero_ml[..ed_start].fill(0);
    assert!(pk.verify(msg, &zero_ml, b"").is_err());

    // Swapping the two halves cannot form a valid signature.
    let (ml, ed) = sig.split_at(ed_start);
    let mut swapped = [0u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE];
    swapped[..ed.len()].copy_from_slice(ed);
    swapped[ed.len()..].copy_from_slice(ml);
    assert!(pk.verify(msg, &swapped, b"").is_err());
}

#[test]
fn composite_65_public_key_validation() {
    let key = MlDsa65Ed25519Sha512SecretKey::new(&COMPOSITE_SEED);
    let msg = b"public key validation";
    let sig = key.sign_derand(msg, b"", &RND).unwrap();

    let mut pk = key.public_key().to_bytes();
    let ed_start = ML_DSA_65_PUBLIC_KEY_SIZE;

    // Non-canonical y (all 0xFF is >= p) and the identity / small-order point.
    for bad in [[0xFFu8; 32], {
        let mut b = [0u8; 32];
        b[0] = 1;
        b
    }] {
        let mut bytes = pk;
        bytes[ed_start..].copy_from_slice(&bad);
        assert_eq!(
            MlDsa65Ed25519Sha512PublicKey::from_bytes(&bytes).verify(msg, &sig, b""),
            Err(MlDsa65Ed25519Error::InvalidPublicKey),
            "composite65: malformed Ed25519 public key was accepted"
        );
    }

    // A bit flip in either public-key half invalidates the signature.
    for offset in [0usize, 31, 32, ML_DSA_65_PUBLIC_KEY_SIZE - 1, ed_start, ed_start + 31] {
        let saved = pk[offset];
        pk[offset] ^= 1;
        assert!(
            MlDsa65Ed25519Sha512PublicKey::from_bytes(&pk)
                .verify(msg, &sig, b"")
                .is_err(),
            "composite65: modified public key byte {offset} accepted"
        );
        pk[offset] = saved;
    }
    MlDsa65Ed25519Sha512PublicKey::from_bytes(&pk)
        .verify(msg, &sig, b"")
        .unwrap();
}

#[test]
fn composite_65_length_parsing() {
    // Public key: exact length only.
    let good = vec![0x01u8; MlDsa65Ed25519Sha512PublicKey::SIZE + 2];
    for len in 0..=good.len() {
        let parsed = MlDsa65Ed25519Sha512PublicKey::try_from(&good[..len]);
        if len == MlDsa65Ed25519Sha512PublicKey::SIZE {
            parsed.unwrap();
        } else {
            assert_eq!(parsed.unwrap_err(), MlDsa65Ed25519Error::InvalidPublicKey);
        }
    }

    // Private key: exact length only.
    let good = [0x02u8; MlDsa65Ed25519Sha512SecretKey::SIZE + 2];
    for len in 0..=good.len() {
        match MlDsa65Ed25519Sha512SecretKey::try_from(&good[..len]) {
            Ok(_) => assert_eq!(
                len,
                MlDsa65Ed25519Sha512SecretKey::SIZE,
                "composite65: short private key was accepted"
            ),
            Err(err) => {
                assert!(
                    len != MlDsa65Ed25519Sha512SecretKey::SIZE,
                    "composite65: valid private key length rejected: {err}"
                );
                assert_eq!(err, MlDsa65Ed25519Error::InvalidPrivateKey);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Composite ML-DSA-87 + Ed25519 + ParallelHash256
////////////////////////////////////////////////////////////////////////////////////////////////////

const COMPOSITE87_SEED: [u8; MlDsa87Ed25519SecretKey::SIZE] = [0x33u8; 64];

fn prehash(message: &[u8]) -> [u8; 64] {
    let mut out = [0u8; 64];
    crypto::sha3::ParallelHash256::hash(message, 4096, MlDsa87Ed25519MessageRepresentative::LABEL, &mut out).unwrap();
    out
}

#[test]
fn composite_87_prehashed_and_streaming_paths_agree() {
    let key = MlDsa87Ed25519SecretKey::new(&COMPOSITE87_SEED);
    let pk = key.public_key();
    let msg = deterministic_message(9000, 0xC3);
    let ctx = b"streamed";

    let one_shot = key.sign_derand(&msg, ctx, &RND).unwrap();
    pk.verify(&msg, &one_shot, ctx).unwrap();

    // The pre-hash helper must match the one-shot pre-hash.
    let mut hasher = MlDsa87Ed25519MessageRepresentative::hasher();
    for chunk in msg.chunks(1000) {
        hasher.update(chunk);
    }
    let mut streamed = [0u8; 64];
    hasher.finalize_into(&mut streamed);
    assert_eq!(streamed, prehash(&msg));

    let from_prehash = key.sign_prehashed_derand(&streamed, ctx, &RND).unwrap();
    assert_eq!(from_prehash.to_bytes(), one_shot.to_bytes());
    pk.verify_prehashed(&streamed, &from_prehash, ctx).unwrap();
    assert!(pk.verify_prehashed(&[0u8; 64], &from_prehash, ctx).is_err());

    // The message representative is ctx-bound and has the documented length:
    // prefix (32) || label (39) || len(ctx) (1) || ctx (0) || PH (64).
    let empty = MlDsa87Ed25519MessageRepresentative::from_message(b"", b"").unwrap();
    assert_eq!(empty.as_bytes().len(), 32 + 39 + 1 + 64);
    let full = MlDsa87Ed25519MessageRepresentative::from_message(b"", &[0u8; 255]).unwrap();
    assert_eq!(full.as_bytes().len(), MlDsa87Ed25519MessageRepresentative::MAX_SIZE);

    assert_eq!(
        MlDsa87Ed25519MessageRepresentative::from_message(b"", &[0u8; 256]).unwrap_err(),
        MlDsa87Ed25519Error::ContextTooLong
    );
    assert_eq!(
        MlDsa87Ed25519MessageRepresentative::from_prehash(&[0u8; 64], &[0u8; 256]).unwrap_err(),
        MlDsa87Ed25519Error::ContextTooLong
    );
    assert_eq!(
        key.sign_derand(b"msg", &[0u8; 256], &RND),
        Err(MlDsa87Ed25519Error::ContextTooLong)
    );
}

#[test]
fn composite_87_signature_and_component_tampering() {
    let key = MlDsa87Ed25519SecretKey::new(&COMPOSITE87_SEED);
    let pk = key.public_key();
    let msg = b"composite87 tampering";
    let sig = key.sign_derand(msg, b"", &RND).unwrap();

    let (ml, ed) = sig.to_parts();
    assert_eq!(MlDsa87Ed25519Signature::from_parts(ml, ed), sig);

    // Ed25519 half.
    let mut bytes = sig.to_bytes();
    let ed_start = ML_DSA_87_SIGNATURE_SIZE;
    for offset in ed_start..ed_start + ed25519::SIGNATURE_SIZE {
        let mut mutated = sig.to_bytes();
        mutated[offset] ^= 0x01;
        assert_eq!(
            pk.verify(msg, &MlDsa87Ed25519Signature::from_bytes(&mutated), b""),
            Err(MlDsa87Ed25519Error::InvalidSignature),
            "composite87: tampered Ed25519 byte {offset} accepted"
        );
    }

    // ML-DSA-87 half (sampled).
    for offset in (0..ML_DSA_87_SIGNATURE_SIZE).step_by(53) {
        bytes[offset] ^= 0x40;
        assert!(
            pk.verify(msg, &MlDsa87Ed25519Signature::from_bytes(&bytes), b"")
                .is_err(),
            "composite87: tampered ML-DSA byte {offset} accepted"
        );
        bytes[offset] ^= 0x40;
    }

    // Length parsing.
    let padded = {
        let mut v = bytes.to_vec();
        v.extend_from_slice(&[0u8; 2]);
        v
    };
    for len in 0..=padded.len() {
        let parsed = MlDsa87Ed25519Signature::try_from(&padded[..len]);
        if len == MlDsa87Ed25519Signature::SIZE {
            parsed.unwrap();
        } else {
            assert_eq!(parsed.unwrap_err(), MlDsa87Ed25519Error::InvalidSignature);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Input-slice offsets / guard canaries
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Signing and verifying a message slice that sits at a non-zero offset inside a
/// larger allocation must not depend on the neighbouring bytes, and must not
/// modify them.
#[test]
fn message_and_context_slices_at_offsets() {
    let key = MlDsa65SecretKey::new(&SEED);
    let pk = key.public_key();

    for len in [0usize, 1, 135, 136, 137, 255, 256] {
        for offset in 0..8 {
            let mut backing = vec![0xC7u8; offset + len + 8];
            for (i, b) in backing.iter_mut().enumerate() {
                *b = if i < offset || i >= offset + len {
                    0xEE
                } else {
                    (i as u8).wrapping_mul(7)
                };
            }
            let before = backing.clone();

            let msg = &backing[offset..offset + len];
            let mut ctx_backing = [0x5Au8; 40];
            let ctx = &mut ctx_backing[3..3 + 5];
            let sig = key.sign_derand(msg, ctx, &RND).unwrap();
            pk.verify(msg, &sig, ctx).unwrap();

            // Identical message content in a fresh allocation gives the same
            // signature, so no out-of-slice bytes influenced the hash.
            let copy = msg.to_vec();
            let sig_copy = key.sign_derand(&copy, &ctx_backing[3..8], &RND).unwrap();
            assert_eq!(sig, sig_copy, "signature depends on out-of-slice bytes (offset {offset})");

            assert_eq!(backing, before, "signing modified the input buffer (offset {offset})");
            assert_eq!(ctx_backing[3..8], [0x5Au8; 5]);
            assert_eq!(ctx_backing[0], 0x5A);
        }
    }
}
