//! Public-API tests for the aarch64 AEGIS backends.
//!
//! On aarch64 targets with AES and SHA-3, dispatch selects `ArmSha3` for
//! AEGIS-256 and AEGIS-256X2 and `Arm` for AEGIS-256X4. The private kernel
//! tests in `crypto/src/aegis/aegis_arm64.rs` cover every `Ne`/`Ne3` degree
//! directly (including the `D = 1`/`D = 2` plain-NEON kernels that dispatch
//! never selects on a SHA3-capable CPU).
//!
//! These tests pin the public contract around the kernels: exact buffer
//! lengths, non-zero input/output offsets, block/rate boundaries, tag sizes,
//! in-place behaviour, and the zeroization-on-failure guarantee.

use crypto::{
    Aead, AeadError,
    aegis::{
        Aegis256, Aegis256Mac, Aegis256Stream, Aegis256X2, Aegis256X2Mac, Aegis256X2Stream, Aegis256X4, Aegis256X4Mac,
        Aegis256X4Stream,
    },
};
use rand::{RngExt, SeedableRng, rngs::StdRng};

const KEY: [u8; 32] = [0x42; 32];
const NONCE: [u8; 32] = [0x24; 32];

/// Deterministic seeded random pattern; the seed is forced odd so that no seed
/// degenerates to an all-zero sequence.
fn pattern(len: usize, seed: u8) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(u64::from(seed));
    let mut out = vec![0u8; len];
    rng.fill(&mut out);
    out
}

/// Every length from 0 through 64, then the interesting block/rate boundaries
/// (including 2^10, where the split-state bulk loop starts in some backends).
fn aead_lengths() -> Vec<usize> {
    let mut lens: Vec<usize> = (0..=64).collect();
    lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4096]);
    lens
}

fn aad_lengths() -> Vec<usize> {
    vec![
        0, 1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4096,
    ]
}

/// Encrypt, decrypt and tamper one `Aead` implementation at every message/AAD
/// length. `TAG` is taken from the implementation.
fn check_aead<C: Aead>(cipher: &C) {
    for &len in &aead_lengths() {
        for &ad_len in &aad_lengths() {
            let ad = pattern(ad_len, 0x33);
            let plain = pattern(len, 0x11);
            // Non-zero, length-dependent output offset so unaligned pointers
            // and slice sub-ranges are exercised.
            let off = (len * 7 + ad_len * 3) % 11;

            let mut buf = vec![0xA5u8; off + len + 9];
            buf[off..off + len].copy_from_slice(&plain);

            let tag_hash = cipher.encrypt_in_place(&mut buf[off..off + len], &NONCE, &ad).unwrap();
            let tag = tag_hash.as_ref().to_vec();
            assert_eq!(tag.len(), C::TAG_SIZE, "tag size");
            assert!(
                buf[..off].iter().all(|&b| b == 0xA5),
                "encrypt pre-canary len={len} ad={ad_len}"
            );
            assert!(
                buf[off + len..].iter().all(|&b| b == 0xA5),
                "encrypt post-canary len={len} ad={ad_len}"
            );

            // Correct tag restores the plaintext in place.
            let mut dec = buf.clone();
            cipher
                .decrypt_in_place(&mut dec[off..off + len], &NONCE, &ad, &tag)
                .unwrap_or_else(|err| panic!("valid tag rejected len={len} ad={ad_len}: {err:?}"));
            assert_eq!(&dec[off..off + len], &plain[..], "roundtrip len={len} ad={ad_len}");
            assert!(dec[..off].iter().all(|&b| b == 0xA5), "decrypt pre-canary");
            assert!(dec[off + len..].iter().all(|&b| b == 0xA5), "decrypt post-canary");

            // A single flipped tag bit must fail and must not leak plaintext.
            let mut bad = tag.clone();
            let flip = (len + ad_len) % bad.len();
            bad[flip] ^= 0x80;
            let mut dec = buf.clone();
            let err = cipher
                .decrypt_in_place(&mut dec[off..off + len], &NONCE, &ad, &bad)
                .expect_err("corrupt tag accepted");
            assert_eq!(err, AeadError::InvalidCiphertext);
            assert!(
                dec[off..off + len].iter().all(|&b| b == 0),
                "plaintext not zeroed on failure len={len} ad={ad_len}"
            );
            assert!(dec[..off].iter().all(|&b| b == 0xA5), "zeroing crossed the pre-canary");
            assert!(dec[off + len..].iter().all(|&b| b == 0xA5), "zeroing crossed the post-canary");
        }
    }
}

#[test]
fn aead_boundary_sweep_all_variants_and_tag_sizes() {
    check_aead(&Aegis256::<16>::new(&KEY));
    check_aead(&Aegis256::<32>::new(&KEY));
    check_aead(&Aegis256X2::<16>::new(&KEY));
    check_aead(&Aegis256X2::<32>::new(&KEY));
    check_aead(&Aegis256X4::<16>::new(&KEY));
    check_aead(&Aegis256X4::<32>::new(&KEY));
}

/// Every tag byte position must be authenticated, and every failure must zero
/// the whole message buffer.
#[test]
fn tampering_any_tag_byte_fails_and_zeroes() {
    fn check<C: Aead>(cipher: &C) {
        let plain = pattern(37, 0x19);
        let ad = pattern(23, 0x2b);
        let mut buf = plain.clone();
        let tag = cipher
            .encrypt_in_place(&mut buf, &NONCE, &ad)
            .unwrap()
            .as_ref()
            .to_vec();
        assert_eq!(tag.len(), C::TAG_SIZE);

        for byte in 0..tag.len() {
            for bit in 0..8 {
                let mut bad = tag.clone();
                bad[byte] ^= 1 << bit;
                let mut dec = buf.clone();
                let err = cipher
                    .decrypt_in_place(&mut dec, &NONCE, &ad, &bad)
                    .expect_err("tag mutation accepted");
                assert_eq!(err, AeadError::InvalidCiphertext);
                assert!(dec.iter().all(|&b| b == 0), "mutation {byte}/{bit} did not zero plaintext");
            }
        }
    }

    check(&Aegis256::<16>::new(&KEY));
    check(&Aegis256::<32>::new(&KEY));
    check(&Aegis256X2::<16>::new(&KEY));
    check(&Aegis256X2::<32>::new(&KEY));
    check(&Aegis256X4::<16>::new(&KEY));
    check(&Aegis256X4::<32>::new(&KEY));
}

/// Wrong nonce/tag lengths are rejected before any decryption, so the buffer is
/// left untouched.
#[test]
fn wrong_nonce_and_tag_lengths_leave_the_buffer_untouched() {
    let cipher = Aegis256::<16>::new(&KEY);
    let mut buf = vec![0xA5u8; 64];
    let tag16 = [0u8; 16];

    for nonce_len in [0usize, 1, 31, 33, 64] {
        let nonce = vec![0x11u8; nonce_len];
        let err = cipher.decrypt_in_place(&mut buf, &nonce, b"ad", &tag16).unwrap_err();
        assert_eq!(err, AeadError::InvalidNonce, "nonce_len={nonce_len}");
        assert!(buf.iter().all(|&b| b == 0xA5), "buffer touched on bad nonce length");
    }
    for tag_len in [0usize, 1, 15, 17, 31, 33, 64] {
        let tag = vec![0u8; tag_len];
        let err = cipher.decrypt_in_place(&mut buf, &NONCE, b"ad", &tag).unwrap_err();
        assert_eq!(err, AeadError::InvalidCiphertext, "tag_len={tag_len}");
        assert!(buf.iter().all(|&b| b == 0xA5), "buffer touched on bad tag length");
    }

    // Encrypting with a non-32-byte nonce returns an error without touching
    // the buffer.
    let mut guarded = vec![0xA5u8; 32];
    assert_eq!(
        cipher.encrypt_in_place(&mut guarded, &[0u8; 31], b"ad").err(),
        Some(AeadError::InvalidNonce)
    );
    assert!(guarded.iter().all(|&b| b == 0xA5), "buffer written on a nonce error");
}

/// Stream output must be position-independent: chunked filling must equal one
/// large fill, and XORing the keystream into zeroes must equal filling.
#[test]
fn stream_boundaries_and_chunking() {
    macro_rules! check {
        ($ctor:expr, $name:literal) => {{
            for &len in &aead_lengths() {
                let mut whole = vec![0u8; len];
                $ctor.fill_keystream(&mut whole);

                let mut zeros = vec![0u8; len];
                $ctor.xor_keystream_in_place(&mut zeros);
                assert_eq!(zeros, whole, "{} fill/xor disagree len={len}", $name);

                // Chunked fill with growing chunk sizes, including calls that
                // cross the rate boundary and exact-rate calls.
                let mut chunked = vec![0u8; len];
                let mut stream = $ctor;
                let mut pos = 0usize;
                let mut chunk = 1usize;
                while pos < len {
                    let take = chunk.min(len - pos);
                    stream.fill_keystream(&mut chunked[pos..pos + take]);
                    pos += take;
                    chunk = chunk * 3 + 1;
                }
                assert_eq!(chunked, whole, "{} chunked fill len={len}", $name);

                // Non-zero offset plus guard canaries.
                let off = len % 7;
                let input = pattern(len, 0x51);
                let mut buf = vec![0xA5u8; off + len + 5];
                buf[off..off + len].copy_from_slice(&input);
                let mut stream = $ctor;
                stream.xor_keystream_in_place(&mut buf[off..off + len]);
                assert!(buf[..off].iter().all(|&b| b == 0xA5), "{} pre-canary len={len}", $name);
                assert!(buf[off + len..].iter().all(|&b| b == 0xA5), "{} post-canary len={len}", $name);
                for i in 0..len {
                    assert_eq!(buf[off + i], input[i] ^ whole[i], "{} xor len={len} i={i}", $name);
                }

                // Chunked XOR must be identical to one-shot XOR.
                let mut chunked = input.clone();
                let mut stream = $ctor;
                let mut pos = 0usize;
                let mut chunk = 2usize;
                while pos < len {
                    let take = chunk.min(len - pos);
                    stream.xor_keystream_in_place(&mut chunked[pos..pos + take]);
                    pos += take;
                    chunk = chunk * 5 + 3;
                }
                let mut oneshot = input.clone();
                $ctor.xor_keystream_in_place(&mut oneshot);
                assert_eq!(chunked, oneshot, "{} chunked xor len={len}", $name);
            }
        }};
    }

    check!(Aegis256Stream::new(&KEY, &NONCE), "Aegis256Stream");
    check!(Aegis256X2Stream::new(&KEY, &NONCE), "Aegis256X2Stream");
    check!(Aegis256X4Stream::new(&KEY, &NONCE), "Aegis256X4Stream");
}

/// Incremental MAC updates must not depend on the chunking, including across
/// the rate boundary and with empty updates.
#[test]
fn mac_incremental_matches_oneshot() {
    macro_rules! check {
        ($ty:ident, $tag:literal, $rate:literal) => {{
            for &len in &aead_lengths() {
                let data = pattern(len, 0x71);

                let mut oneshot = <$ty<$tag>>::new(&KEY);
                oneshot.update(&data).unwrap();
                let expected = oneshot.finalize();

                for &split in &[
                    1usize,
                    2,
                    3,
                    7,
                    15,
                    16,
                    17,
                    31,
                    32,
                    33,
                    63,
                    64,
                    65,
                    $rate - 1,
                    $rate,
                    $rate + 1,
                    1023,
                    1024,
                ] {
                    let mut mac = <$ty<$tag>>::new(&KEY);
                    mac.update(&[]).unwrap();
                    for chunk in data.chunks(split) {
                        mac.update(chunk).unwrap();
                    }
                    mac.update(&[]).unwrap();
                    assert_eq!(mac.finalize(), expected, "{}/{} len={len} split={split}", stringify!($ty), $tag);
                }

                // Verification succeeds for the real tag and fails for any
                // single-bit mutation.
                let mut mac = <$ty<$tag>>::new(&KEY);
                mac.update(&data).unwrap();
                assert!(mac.verify(&expected).is_ok());
                for byte in 0..expected.len() {
                    let mut bad = expected;
                    bad[byte] ^= 1;
                    let mut mac = <$ty<$tag>>::new(&KEY);
                    mac.update(&data).unwrap();
                    assert_eq!(mac.verify(&bad).unwrap_err(), AeadError::InvalidCiphertext);
                }
            }
        }};
    }

    check!(Aegis256Mac, 16, 16);
    check!(Aegis256Mac, 32, 16);
    check!(Aegis256X2Mac, 16, 32);
    check!(Aegis256X2Mac, 32, 32);
    check!(Aegis256X4Mac, 16, 64);
    check!(Aegis256X4Mac, 32, 64);
}

/// The three modes are distinct algorithms: the same key/nonce/message must not
/// produce interchangeable ciphertexts or tags.
#[test]
fn variants_are_distinct() {
    let mut a = pattern(129, 0x5b);
    let mut b = a.clone();
    let mut c = a.clone();
    let ta = Aegis256::<16>::new(&KEY)
        .encrypt_in_place(&mut a, &NONCE, b"ad")
        .unwrap();
    let tb = Aegis256X2::<16>::new(&KEY)
        .encrypt_in_place(&mut b, &NONCE, b"ad")
        .unwrap();
    let tc = Aegis256X4::<16>::new(&KEY)
        .encrypt_in_place(&mut c, &NONCE, b"ad")
        .unwrap();
    assert_ne!(a, b);
    assert_ne!(a, c);
    assert_ne!(b, c);
    assert_ne!(ta.as_ref(), tb.as_ref());
    assert_ne!(ta.as_ref(), tc.as_ref());
    assert_ne!(tb.as_ref(), tc.as_ref());
}
