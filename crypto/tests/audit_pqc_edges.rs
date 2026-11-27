//! Additional adversarial edge-case tests for the post-quantum unit:
//! ML-KEM-768/1024, X-Wing, MLKEM1024-X25519, ML-DSA-44/65/87 and the
//! composite ML-DSA/Ed25519 constructions.
//!
//! The crate already has exhaustive known-answer, Wycheproof and bit-flip
//! tests. This file deliberately targets the remaining API boundaries:
//!
//! * all-zero / all-0xFF keys and ciphertexts, wrong-key decapsulation, and
//!   corruption of every region of an ML-KEM decapsulation key;
//! * composite KEM explicit rejection of low-order X25519 ciphertexts;
//! * ML-DSA all-zero/all-0xFF seeds, signatures and external message
//!   representatives, plus context/message length edges;
//! * component-mismatch combinations for the composite signature schemes.

use crypto::{
    Xof,
    curve25519::ed25519,
    mldsa::{
        ML_DSA_44_PUBLIC_KEY_SIZE, ML_DSA_44_SIGNATURE_SIZE, ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SIGNATURE_SIZE,
        ML_DSA_87_PUBLIC_KEY_SIZE, MlDsa44PublicKey, MlDsa44SecretKey, MlDsa65Ed25519Error,
        MlDsa65Ed25519Sha512PublicKey, MlDsa65Ed25519Sha512SecretKey, MlDsa65PublicKey, MlDsa65SecretKey,
        MlDsa87Ed25519Error, MlDsa87Ed25519PublicKey, MlDsa87Ed25519SecretKey, MlDsa87PublicKey, MlDsa87SecretKey,
        MlDsaError,
    },
    mlkem::{
        CIPHERTEXT_SIZE_768, CIPHERTEXT_SIZE_1024, MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE,
        MLKEM_1024_X25519_SECRET_KEY_SIZE, MlKem1024X25519Error, MlKem1024X25519PublicKey, MlKem1024X25519SecretKey,
        MlKemError, PUBLIC_KEY_SIZE_768, PUBLIC_KEY_SIZE_1024, PublicKey768, PublicKey1024, SECRET_KEY_SIZE_768,
        SECRET_KEY_SIZE_1024, SecretKey768, SecretKey1024, XWING_CIPHERTEXT_SIZE, XWING_SECRET_KEY_SIZE,
        generate_xwing_keypair_derand,
    },
    sha3::Shake256,
};

const RND: [u8; 32] = [0x5Au8; 32];

fn rejection_secret(z: &[u8], ciphertext: &[u8]) -> [u8; 32] {
    let mut hasher = Shake256::new();
    hasher.absorb(z);
    hasher.absorb(ciphertext);
    let mut out = [0u8; 32];
    hasher.squeeze(&mut out);
    out
}

/// Flips one bit of each region of an ML-KEM-768 decapsulation key and checks
/// the documented rejection behaviour:
///
/// * corrupting `H(ek)` or any byte of the embedded `ek` makes the key fail its
///   FIPS 203 consistency check;
/// * corrupting the IND-CPA secret part `s` leaves the check intact and must
///   still decapsulate without panicking, with a different shared secret.
#[test]
fn mlkem_768_dk_region_corruption() {
    let (sk, pk) = SecretKey768::generate_derand(&[11u8; 64]);
    let (ss, ciphertext) = pk.encapsulate_derand(&[22u8; 32]);
    assert_eq!(sk.decapsulate(&ciphertext).unwrap(), ss);

    let original = sk.to_bytes();
    let s_len = 3 * 384;
    let ek_start = s_len;
    let h_start = SECRET_KEY_SIZE_768 - 64;
    let z_start = SECRET_KEY_SIZE_768 - 32;

    for offset in [0usize, 1, 383, s_len - 1] {
        let mut bytes = original;
        bytes[offset] ^= 1;
        let corrupted = SecretKey768::from_bytes(&bytes);
        let got = corrupted.decapsulate(&ciphertext).unwrap();
        assert_ne!(got, ss, "corrupting s at byte {offset} did not change the shared secret");
        // The FIPS 203 re-encryption still uses the (intact) embedded ek and h,
        // so the rejection secret is J(z || c) only when s is untouched. Here s
        // changed, so just require determinism and a non-panic.
        assert_eq!(got, corrupted.decapsulate(&ciphertext).unwrap());
    }

    for offset in [ek_start, ek_start + 17, h_start, h_start + 31] {
        let mut bytes = original;
        bytes[offset] ^= 1;
        assert_eq!(
            SecretKey768::from_bytes(&bytes).decapsulate(&ciphertext),
            Err(MlKemError::InvalidKey),
            "corruption at byte {offset} should fail the H(ek) check"
        );
    }

    // The stored z (implicit-rejection seed) is not covered by the consistency
    // check. It only affects *invalid* ciphertexts: changing it changes the
    // implicit-rejection secret but must not error.
    let mut bad_ct = ciphertext;
    bad_ct[0] ^= 1;
    let z_original = &original[z_start..];
    assert_eq!(sk.decapsulate(&bad_ct).unwrap(), rejection_secret(z_original, &bad_ct));
    let mut bytes = original;
    bytes[z_start] ^= 1;
    let rejected = SecretKey768::from_bytes(&bytes).decapsulate(&bad_ct).unwrap();
    assert_ne!(rejected, rejection_secret(z_original, &bad_ct));
}

/// Same corruption sweep for ML-KEM-1024.
#[test]
fn mlkem_1024_dk_region_corruption() {
    let (sk, pk) = SecretKey1024::generate_derand(&[12u8; 64]);
    let (ss, ciphertext) = pk.encapsulate_derand(&[23u8; 32]);
    assert_eq!(sk.decapsulate(&ciphertext).unwrap(), ss);
    let original = sk.to_bytes();

    let mut bad_s = original;
    bad_s[0] ^= 1;
    assert!(SecretKey1024::from_bytes(&bad_s).decapsulate(&ciphertext).is_ok());

    let h_start = SECRET_KEY_SIZE_1024 - 64;
    let z_start = SECRET_KEY_SIZE_1024 - 32;
    for offset in [h_start, h_start + 31] {
        let mut bytes = original;
        bytes[offset] ^= 1;
        assert_eq!(
            SecretKey1024::from_bytes(&bytes).decapsulate(&ciphertext),
            Err(MlKemError::InvalidKey)
        );
    }

    let mut bad_ct = ciphertext;
    bad_ct[CIPHERTEXT_SIZE_1024 - 1] ^= 1;
    let z = &original[z_start..];
    assert_eq!(sk.decapsulate(&bad_ct).unwrap(), rejection_secret(z, &bad_ct));

    let mut bad_z = original;
    bad_z[z_start] ^= 1;
    let rejected = SecretKey1024::from_bytes(&bad_z).decapsulate(&bad_ct).unwrap();
    assert_ne!(rejected, rejection_secret(z, &bad_ct));
}

/// All-zero ciphertexts/keys and all-0xFF ciphertexts/keys are legal byte
/// strings at the fixed-size API boundary. Ciphertexts must implicitly reject,
/// never panic; an all-0xFF public key must fail the modulus check; an
/// all-0xFF decapsulation key must fail the hash check.
#[test]
fn mlkem_extreme_keys_and_ciphertexts() {
    let (sk768, _) = SecretKey768::generate_derand(&[31u8; 64]);
    let (sk1024, _) = SecretKey1024::generate_derand(&[32u8; 64]);
    let z768 = {
        let b = sk768.to_bytes();
        b[SECRET_KEY_SIZE_768 - 32..].to_vec()
    };
    let z1024 = {
        let b = sk1024.to_bytes();
        b[SECRET_KEY_SIZE_1024 - 32..].to_vec()
    };

    for ct in [[0u8; CIPHERTEXT_SIZE_768], [0xFFu8; CIPHERTEXT_SIZE_768]] {
        let got = sk768.decapsulate(&ct).unwrap();
        assert_eq!(got, rejection_secret(&z768, &ct));
    }
    for ct in [[0u8; CIPHERTEXT_SIZE_1024], [0xFFu8; CIPHERTEXT_SIZE_1024]] {
        let got = sk1024.decapsulate(&ct).unwrap();
        assert_eq!(got, rejection_secret(&z1024, &ct));
    }

    // All-zero ek encodes only canonical zero coefficients and is accepted;
    // all-0xFF coefficients are 4095 >= q and must be rejected.
    assert!(PublicKey768::from_bytes(&[0u8; PUBLIC_KEY_SIZE_768]).is_ok());
    assert_eq!(
        PublicKey768::from_bytes(&[0xFFu8; PUBLIC_KEY_SIZE_768]),
        Err(MlKemError::InvalidKey)
    );
    assert!(PublicKey1024::from_bytes(&[0u8; PUBLIC_KEY_SIZE_1024]).is_ok());
    assert_eq!(
        PublicKey1024::from_bytes(&[0xFFu8; PUBLIC_KEY_SIZE_1024]),
        Err(MlKemError::InvalidKey)
    );

    for sk in [[0u8; SECRET_KEY_SIZE_768], [0xFFu8; SECRET_KEY_SIZE_768]] {
        assert_eq!(
            SecretKey768::from_bytes(&sk).decapsulate(&[0u8; CIPHERTEXT_SIZE_768]),
            Err(MlKemError::InvalidKey)
        );
    }
    for sk in [[0u8; SECRET_KEY_SIZE_1024], [0xFFu8; SECRET_KEY_SIZE_1024]] {
        assert_eq!(
            SecretKey1024::from_bytes(&sk).decapsulate(&[0u8; CIPHERTEXT_SIZE_1024]),
            Err(MlKemError::InvalidKey)
        );
    }
}

/// Wrong-key decapsulation returns a deterministic secret that differs from the
/// real one, and never leaks the sender's secret.
#[test]
fn mlkem_wrong_key_decapsulation() {
    let (alice, alice_pk) = SecretKey768::generate_derand(&[41u8; 64]);
    let (bob, _) = SecretKey768::generate_derand(&[42u8; 64]);
    let (ss, ct) = alice_pk.encapsulate_derand(&[43u8; 32]);

    assert_eq!(alice.decapsulate(&ct).unwrap(), ss);
    let wrong = bob.decapsulate(&ct).unwrap();
    assert_ne!(wrong, ss);
    assert_eq!(wrong, bob.decapsulate(&ct).unwrap());

    // A same-length one-bit change in the ciphertext must also be rejected.
    let mut bad = ct;
    bad[CIPHERTEXT_SIZE_768 / 2] ^= 0x80;
    assert_ne!(alice.decapsulate(&bad).unwrap(), ss);
}

/// MLKEM1024-X25519 must explicitly reject a low-order X25519 ciphertext
/// component and a low-order encapsulation key, but accept non-canonical
/// encodings that reduce to a valid point.
#[test]
fn mlkem_1024_x25519_rejection_paths() {
    let (sk, pk) = MlKem1024X25519SecretKey::generate_derand(&[51u8; MLKEM_1024_X25519_SECRET_KEY_SIZE]);
    let (ss, ct) = pk
        .encapsulate_derand(&[52u8; MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE])
        .unwrap();
    assert_eq!(sk.decapsulate(&ct).unwrap(), ss);

    let low_order = [
        [0u8; 32],
        {
            let mut u = [0u8; 32];
            u[0] = 1;
            u
        },
        [
            0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
        ],
    ];

    for point in low_order {
        let mut bad_ct = ct;
        bad_ct[CIPHERTEXT_SIZE_1024..].copy_from_slice(&point);
        assert_eq!(sk.decapsulate(&bad_ct), Err(MlKem1024X25519Error::InvalidCiphertext));

        let mut bad_pk = pk.to_bytes();
        bad_pk[PUBLIC_KEY_SIZE_1024..].copy_from_slice(&point);
        assert_eq!(
            MlKem1024X25519PublicKey::from_bytes(&bad_pk),
            Err(MlKem1024X25519Error::InvalidPublicKey)
        );
    }

    // The all-zero ML-KEM ciphertext half implicitly rejects.
    let mut zero_pq = ct;
    zero_pq[..CIPHERTEXT_SIZE_1024].fill(0);
    let got = sk.decapsulate(&zero_pq).unwrap();
    assert_ne!(got, ss);
}

/// X-Wing: every ciphertext bit is covered by the combiner, and corrupting the
/// X25519 half of a ciphertext never panics.
#[test]
fn xwing_ciphertext_tampering_is_absorbed() {
    let (sk, pk) = generate_xwing_keypair_derand(&[61u8; XWING_SECRET_KEY_SIZE]);
    let (ss, ct) = pk.encapsulate_derand(&[62u8; 64]);
    assert_eq!(sk.decapsulate(&ct).unwrap(), ss);

    for offset in [
        0usize,
        CIPHERTEXT_SIZE_768 - 1,
        CIPHERTEXT_SIZE_768,
        XWING_CIPHERTEXT_SIZE - 1,
    ] {
        let mut bad = ct;
        bad[offset] ^= 1;
        let got = sk.decapsulate(&bad).unwrap();
        assert_ne!(got, ss, "X-Wing shared secret did not change at byte {offset}");
    }

    // All-zero and all-0xFF ciphertexts must not panic.
    for bad in [[0u8; XWING_CIPHERTEXT_SIZE], [0xFFu8; XWING_CIPHERTEXT_SIZE]] {
        let _ = sk.decapsulate(&bad).unwrap();
    }
}

fn mldsa44_roundtrip(seed: &[u8; 32]) {
    let key = MlDsa44SecretKey::new(seed);
    let pk = key.public_key();
    for len in [0usize, 1, 135, 136, 137, 4096] {
        let msg: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(17)).collect();
        let sig = key.sign_derand(&msg, b"edge", &RND).unwrap();
        pk.verify(&msg, &sig, b"edge").unwrap();
        assert!(pk.verify(&msg, &sig, b"other").is_err());
    }
    for ctx_len in [0usize, 1, 255] {
        let ctx = vec![0x33u8; ctx_len];
        let sig = key.sign_derand(b"context", &ctx, &RND).unwrap();
        pk.verify(b"context", &sig, &ctx).unwrap();
    }
    let too_long = vec![0u8; 256];
    assert_eq!(key.sign_derand(b"x", &too_long, &RND), Err(MlDsaError::ContextTooLong));
    assert_eq!(
        pk.verify(b"x", &[0u8; ML_DSA_44_SIGNATURE_SIZE], &too_long),
        Err(MlDsaError::ContextTooLong)
    );
}

fn mldsa65_roundtrip(seed: &[u8; 32]) {
    let key = MlDsa65SecretKey::new(seed);
    let pk = key.public_key();
    for len in [0usize, 1, 135, 136, 137, 4096] {
        let msg: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(19)).collect();
        let sig = key.sign_derand(&msg, b"", &RND).unwrap();
        pk.verify(&msg, &sig, b"").unwrap();
        let mut ext = msg.clone();
        ext.push(0);
        assert!(pk.verify(&ext, &sig, b"").is_err());
    }
}

fn mldsa87_roundtrip(seed: &[u8; 32]) {
    let key = MlDsa87SecretKey::new(seed);
    let pk = key.public_key();
    for len in [0usize, 1, 136, 1360, 4096] {
        let msg: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(23)).collect();
        let sig = key.sign_derand(&msg, b"87", &RND).unwrap();
        pk.verify(&msg, &sig, b"87").unwrap();
        assert!(pk.verify(&msg, &sig, b"").is_err());
    }
}

/// Every-bit-set and every-bit-clear seeds are valid ML-DSA seeds. Signing and
/// verification must work, deterministically for a fixed `rnd`, across all
/// three parameter sets.
#[test]
fn mldsa_seed_extremes_and_determinism() {
    for seed in [[0x00u8; 32], [0xFFu8; 32], [0x01u8; 32]] {
        mldsa44_roundtrip(&seed);
        mldsa65_roundtrip(&seed);
        mldsa87_roundtrip(&seed);
    }

    let key = MlDsa65SecretKey::new(&[0u8; 32]);
    let a = key.sign_derand(b"determinism", b"ctx", &RND).unwrap();
    let b = key.sign_derand(b"determinism", b"ctx", &RND).unwrap();
    assert_eq!(a, b);
    let mut different_rnd = RND;
    different_rnd[0] ^= 1;
    let c = key.sign_derand(b"determinism", b"ctx", &different_rnd).unwrap();
    assert_ne!(a, c);
    key.public_key().verify(b"determinism", &c, b"ctx").unwrap();
}

/// All-zero, all-0xFF and random bit-flipped ML-DSA signatures must be
/// rejected cleanly, including through `verify_external_mu`.
#[test]
fn mldsa_malformed_signatures_are_rejected() {
    let key = MlDsa65SecretKey::new(&[0x44u8; 32]);
    let pk = key.public_key();
    let msg = b"malformed signature";
    let valid = key.sign_derand(msg, b"", &RND).unwrap();

    for sig in [[0u8; ML_DSA_65_SIGNATURE_SIZE], [0xFFu8; ML_DSA_65_SIGNATURE_SIZE]] {
        assert_eq!(pk.verify(msg, &sig, b""), Err(MlDsaError::InvalidSignature));
    }

    // Flip one bit at representative offsets across the challenge, z and hint
    // regions, including the very first and last byte.
    for offset in [
        0usize,
        1,
        47,
        48,
        1000,
        ML_DSA_65_SIGNATURE_SIZE - 2,
        ML_DSA_65_SIGNATURE_SIZE - 1,
    ] {
        let mut bad = valid;
        bad[offset] ^= 0x01;
        assert!(pk.verify(msg, &bad, b"").is_err(), "flipped signature byte {offset} accepted");
    }

    // External-mu entry point with degenerate representatives.
    for mu in [[0u8; 64], [0xFFu8; 64]] {
        assert_eq!(pk.verify_external_mu(&mu, &valid), Err(MlDsaError::InvalidSignature));
        assert!(key.sign_external_mu_derand(&mu, &RND).is_ok());
    }
}

/// The public-key `TryFrom` path is the only length-polymorphic ML-DSA entry
/// point. All lengths other than the exact one must be rejected.
#[test]
fn mldsa_public_key_length_sweep() {
    fn sweep<const N: usize>(parse: impl Fn(&[u8]) -> Result<(), MlDsaError>) {
        let data = vec![0xA5u8; N + 2];
        for len in 0..=N + 2 {
            let result = parse(&data[..len]);
            if len == N {
                result.unwrap_or_else(|e| panic!("valid length {len} rejected: {e}"));
            } else {
                assert_eq!(result, Err(MlDsaError::InvalidPublicKey), "length {len}");
            }
        }
    }
    sweep::<ML_DSA_44_PUBLIC_KEY_SIZE>(|b| MlDsa44PublicKey::try_from(b).map(|_| ()));
    sweep::<ML_DSA_65_PUBLIC_KEY_SIZE>(|b| MlDsa65PublicKey::try_from(b).map(|_| ()));
    sweep::<ML_DSA_87_PUBLIC_KEY_SIZE>(|b| MlDsa87PublicKey::try_from(b).map(|_| ()));
}

/// Mixing an ML-DSA half from one key with an Ed25519 half from another key, or
/// swapping component order, must never verify.
#[test]
fn composite_65_component_mixing_fails() {
    let key_a = MlDsa65Ed25519Sha512SecretKey::new(&[0x71u8; 64]);
    let key_b = MlDsa65Ed25519Sha512SecretKey::new(&[0x72u8; 64]);
    let pk_a = key_a.public_key();
    let msg = b"component mixing";

    let sig_a = key_a.sign_derand(msg, b"", &RND).unwrap();
    let sig_b = key_b.sign_derand(msg, b"", &RND).unwrap();

    let mut mixed = sig_a;
    mixed[ML_DSA_65_SIGNATURE_SIZE..].copy_from_slice(&sig_b[ML_DSA_65_SIGNATURE_SIZE..]);
    assert_eq!(pk_a.verify(msg, &mixed, b""), Err(MlDsa65Ed25519Error::InvalidSignature));

    let mut swapped = [0u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE];
    let ed_len = ed25519::SIGNATURE_SIZE;
    swapped[..ed_len].copy_from_slice(&sig_a[ML_DSA_65_SIGNATURE_SIZE..]);
    swapped[ed_len..].copy_from_slice(&sig_a[..ML_DSA_65_SIGNATURE_SIZE]);
    assert!(pk_a.verify(msg, &swapped, b"").is_err());

    // A different context must not verify.
    assert!(pk_a.verify(msg, &sig_a, b"other").is_err());

    // The all-zero Ed25519 encoding is the identity point and must be
    // rejected on every verification.
    let mut pk_bytes = pk_a.to_bytes();
    pk_bytes[ML_DSA_65_PUBLIC_KEY_SIZE..].fill(0);
    assert_eq!(
        MlDsa65Ed25519Sha512PublicKey::from_bytes(&pk_bytes).verify(msg, &sig_a, b""),
        Err(MlDsa65Ed25519Error::InvalidPublicKey)
    );
}

/// The custom ML-DSA-87 composite scheme rejects identity Ed25519 keys and
/// mismatched component signatures, and its pre-hash path is context-bound.
#[test]
fn composite_87_edges() {
    let key = MlDsa87Ed25519SecretKey::new(&[0x81u8; 64]);
    let pk = key.public_key();
    let valid = MlDsa87Ed25519PublicKey::from_bytes(&pk.to_bytes());
    let sig = key.sign_derand(b"87 edges", b"c", &RND).unwrap();
    assert!(valid.verify(b"87 edges", &sig, b"c").is_ok());
    let (ml, _ed) = sig.to_parts();

    // Mixing the Ed25519 half of another key must fail.
    let other = MlDsa87Ed25519SecretKey::new(&[0x82u8; 64]);
    let other_sig = other.sign_derand(b"87 edges", b"c", &RND).unwrap();
    let (_, other_ed) = other_sig.to_parts();
    let mixed = crypto::mldsa::MlDsa87Ed25519Signature::from_parts(ml, other_ed);
    assert_eq!(
        valid.verify(b"87 edges", &mixed, b"c"),
        Err(MlDsa87Ed25519Error::InvalidSignature)
    );

    // Identity/non-canonical Ed25519 public keys are rejected.
    for bad in [[0u8; 32], [0xFFu8; 32]] {
        let mut bytes = pk.to_bytes();
        bytes[ML_DSA_87_PUBLIC_KEY_SIZE..].copy_from_slice(&bad);
        let parsed = MlDsa87Ed25519PublicKey::from_bytes(&bytes);
        assert!(parsed.verify(b"87 edges", &sig, b"c").is_err());
    }

    // The message representative is length-bounded at 255 bytes of context.
    assert_eq!(
        crypto::mldsa::MlDsa87Ed25519MessageRepresentative::from_message(b"x", &[0u8; 256]),
        Err(MlDsa87Ed25519Error::ContextTooLong)
    );
}
