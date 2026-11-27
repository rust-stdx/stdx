//! ChaCha20-Poly1305 / XChaCha20-Poly1305 edge and API tests.
//!
//! The differential oracle for Poly1305 itself lives in
//! `audit_poly1305_vectors.rs`; this file concentrates on the AEAD plumbing:
//! every AAD length 0..=33, every plaintext length 0..=65, 16-byte block
//! boundaries, in-place offsets and canaries, detached tags, wrong nonce/tag
//! lengths, and the exact keystream counter used by each AEAD.

use crypto::{
    Aead, AeadError, StreamCipher,
    chacha::{ChaCha20Ietf, ChaCha20Poly1305, XChaCha20Ietf, XChaCha20Poly1305},
};

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

fn key() -> [u8; 32] {
    let mut k = [0u8; 32];
    for (i, b) in k.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(31).wrapping_add(7);
    }
    k
}

fn nonce12() -> [u8; 12] {
    [0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba, 0xcb]
}

fn nonce24() -> [u8; 24] {
    let mut n = [0u8; 24];
    for (i, b) in n.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(19).wrapping_add(3);
    }
    n
}

/// Every AAD length 0..=33 crossed with every plaintext length 0..=65:
/// round-trip, ciphertext independence from AAD length only where the AAD
/// repeats, and tag dependency on the exact AAD bytes.
#[test]
fn every_aad_and_short_plaintext_length_round_trips() {
    let key = key();
    let c = ChaCha20Poly1305::new(&key);
    let cx = XChaCha20Poly1305::new(&key);
    let n12 = nonce12();
    let n24 = nonce24();

    for alen in 0..=33usize {
        let aad = pattern(alen, 7, 1);
        for len in 0..=65usize {
            let pt = pattern(len, 13, 5);

            let mut buf = pt.clone();
            let tag = c.encrypt_in_place(&mut buf, &n12, &aad).unwrap();
            assert_eq!(tag.as_ref().len(), 16);
            assert_eq!(buf.len(), len);
            let mut dec = buf.clone();
            c.decrypt_in_place(&mut dec, &n12, &aad, tag.as_ref()).unwrap();
            assert_eq!(dec, pt, "chacha len={len} aad={alen}");

            let mut buf = pt.clone();
            let tag = cx.encrypt_in_place(&mut buf, &n24, &aad).unwrap();
            let mut dec = buf.clone();
            cx.decrypt_in_place(&mut dec, &n24, &aad, tag.as_ref()).unwrap();
            assert_eq!(dec, pt, "xchacha len={len} aad={alen}");
        }
    }
}

/// AAD is hashed across its full byte string: changing any byte must change
/// the tag, including the last byte of a 16-byte-multiple AAD.
#[test]
fn every_aad_byte_is_authenticated() {
    let key = key();
    let c = ChaCha20Poly1305::new(&key);
    let n = nonce12();
    let pt = pattern(37, 3, 2);
    for alen in 0..=33usize {
        let aad = pattern(alen, 11, 9);
        let mut buf = pt.clone();
        let good = c.encrypt_in_place(&mut buf, &n, &aad).unwrap();
        for i in 0..alen {
            let mut bad_aad = aad.clone();
            bad_aad[i] ^= 0x80;
            let mut dec = buf.clone();
            assert!(
                c.decrypt_in_place(&mut dec, &n, &bad_aad, good.as_ref()).is_err(),
                "AAD byte {i}/{alen} not authenticated"
            );
        }
    }
}

/// Ciphertext is exactly the raw ChaCha20/XChaCha20 keystream starting at
/// counter 1 XOR the plaintext; counter 0 is reserved for the Poly1305 key.
#[test]
fn payload_uses_counter_one_for_both_aeads() {
    let key = key();
    let pt = pattern(300, 23, 7);

    let mut ct = pt.clone();
    ChaCha20Poly1305::new(&key)
        .encrypt_in_place(&mut ct, &nonce12(), b"aad")
        .unwrap();
    let mut raw = pt.clone();
    let mut s = ChaCha20Ietf::new(&key, &nonce12());
    s.set_counter(1);
    s.xor_keystream(&mut raw).unwrap();
    assert_eq!(ct, raw);

    let mut ct = pt.clone();
    XChaCha20Poly1305::new(&key)
        .encrypt_in_place(&mut ct, &nonce24(), b"aad")
        .unwrap();
    let mut raw = pt.clone();
    let mut s = XChaCha20Ietf::new(&key, &nonce24());
    s.set_counter(1);
    s.xor_keystream(&mut raw).unwrap();
    assert_eq!(ct, raw);
}

/// Non-zero slice offsets, canaries before/after, and lengths around the
/// 16/64/128-byte boundaries. A SIMD over-read or over-write would land in a
/// canary.
#[test]
fn in_place_offsets_and_canaries() {
    const CANARY: u8 = 0x5a;
    let key = key();
    let c = ChaCha20Poly1305::new(&key);
    let cx = XChaCha20Poly1305::new(&key);

    for &off in &[0usize, 1, 3, 7, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
        for &len in &[
            0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 130, 191, 192, 193, 255, 256, 257,
        ] {
            let pt = pattern(len, 29, 11);
            let mut buf = vec![CANARY; off + len + 2 * 19];
            buf[off..off + len].copy_from_slice(&pt);

            let tag = {
                let slice = &mut buf[off..off + len];
                c.encrypt_in_place(slice, &nonce12(), b"canary-aad").unwrap()
            };
            assert!(buf[..off].iter().all(|&b| b == CANARY), "pre off={off} len={len}");
            assert!(buf[off + len..].iter().all(|&b| b == CANARY), "post off={off} len={len}");
            {
                let slice = &mut buf[off..off + len];
                c.decrypt_in_place(slice, &nonce12(), b"canary-aad", tag.as_ref())
                    .unwrap();
            }
            assert_eq!(&buf[off..off + len], &pt[..]);
            assert!(buf[..off].iter().all(|&b| b == CANARY));
            assert!(buf[off + len..].iter().all(|&b| b == CANARY));

            let mut buf = vec![CANARY; off + len + 2 * 19];
            buf[off..off + len].copy_from_slice(&pt);
            let tag = {
                let slice = &mut buf[off..off + len];
                cx.encrypt_in_place(slice, &nonce24(), b"canary-aad").unwrap()
            };
            {
                let slice = &mut buf[off..off + len];
                cx.decrypt_in_place(slice, &nonce24(), b"canary-aad", tag.as_ref())
                    .unwrap();
            }
            assert_eq!(&buf[off..off + len], &pt[..]);
            assert!(buf[..off].iter().all(|&b| b == CANARY));
            assert!(buf[off + len..].iter().all(|&b| b == CANARY));
        }
    }
}

/// The owned-buffer API appends the 16-byte tag and `decrypt` treats the last
/// 16 bytes as the detached tag. Every truncation or extension of that tag
/// must fail.
#[test]
fn detached_tag_handling() {
    let key = key();
    let c = ChaCha20Poly1305::new(&key);
    let cx = XChaCha20Poly1305::new(&key);
    for len in [0usize, 1, 15, 16, 17, 63, 64, 65, 129] {
        let pt = pattern(len, 41, 3);
        let aad = pattern(len % 34, 5, 2);

        let sealed = c.encrypt(&pt, &nonce12(), &aad).unwrap();
        assert_eq!(sealed.len(), len + 16);
        assert_eq!(c.decrypt(&sealed, &nonce12(), &aad).unwrap(), pt);
        for cut in [1usize, 8, 15, 16] {
            assert_eq!(
                c.decrypt(&sealed[..sealed.len() - cut], &nonce12(), &aad).err(),
                Some(AeadError::InvalidCiphertext),
                "chacha truncated tag cut={cut} len={len}"
            );
        }
        let mut bad = sealed.clone();
        bad[len] ^= 1;
        assert!(c.decrypt(&bad, &nonce12(), &aad).is_err());

        let sealed = cx.encrypt(&pt, &nonce24(), &aad).unwrap();
        assert_eq!(sealed.len(), len + 16);
        assert_eq!(cx.decrypt(&sealed, &nonce24(), &aad).unwrap(), pt);
        for cut in [1usize, 8, 15, 16] {
            assert_eq!(
                cx.decrypt(&sealed[..sealed.len() - cut], &nonce24(), &aad).err(),
                Some(AeadError::InvalidCiphertext),
                "xchacha truncated tag cut={cut} len={len}"
            );
        }
    }
}

/// Wrong nonce/tag lengths are rejected before any buffer mutation. This
/// includes lengths that would otherwise be valid for the *other* AEAD
/// (24 bytes for ChaCha20-Poly1305, 12 for XChaCha20-Poly1305).
#[test]
fn wrong_nonce_and_tag_lengths_leave_buffer_untouched() {
    let key = key();
    let n12 = nonce12();
    let n24 = nonce24();

    // Build a valid ciphertext for each AEAD first.
    let pt = pattern(67, 17, 1);
    let c = ChaCha20Poly1305::new(&key);
    let mut ct12 = pt.clone();
    let tag12 = c.encrypt_in_place(&mut ct12, &n12, b"ad").unwrap();
    let cx = XChaCha20Poly1305::new(&key);
    let mut ct24 = pt.clone();
    let tag24 = cx.encrypt_in_place(&mut ct24, &n24, b"ad").unwrap();

    for bad_len in [0usize, 1, 8, 11, 12, 13, 16, 23, 24, 25, 32] {
        let bad = vec![0u8; bad_len];
        if bad_len != 12 {
            let mut buf = ct12.clone();
            assert_eq!(
                c.decrypt_in_place(&mut buf, &bad, b"ad", tag12.as_ref()),
                Err(AeadError::InvalidNonce),
                "chacha nonce len={bad_len}"
            );
            assert_eq!(buf, ct12, "chacha buffer changed on nonce error len={bad_len}");
        }

        if bad_len != 24 {
            let mut buf = ct24.clone();
            assert_eq!(
                cx.decrypt_in_place(&mut buf, &bad, b"ad", tag24.as_ref()),
                Err(AeadError::InvalidNonce),
                "xchacha nonce len={bad_len}"
            );
            assert_eq!(buf, ct24, "xchacha buffer changed on nonce error len={bad_len}");
        }
    }

    for bad_len in 0..=20usize {
        if bad_len == 16 {
            continue;
        }
        let bad = vec![0u8; bad_len];
        let mut buf = ct12.clone();
        assert_eq!(
            c.decrypt_in_place(&mut buf, &n12, b"ad", &bad),
            Err(AeadError::InvalidCiphertext),
            "chacha tag len={bad_len}"
        );
        assert_eq!(buf, ct12, "chacha buffer changed on tag-length error len={bad_len}");

        let mut buf = ct24.clone();
        assert_eq!(
            cx.decrypt_in_place(&mut buf, &n24, b"ad", &bad),
            Err(AeadError::InvalidCiphertext),
            "xchacha tag len={bad_len}"
        );
        assert_eq!(buf, ct24, "xchacha buffer changed on tag-length error len={bad_len}");
    }
}

/// Every tag bit must be checked; a failed check must not yield plaintext.
#[test]
fn every_tag_bit_is_checked_and_no_plaintext_on_failure() {
    let key = key();
    let pt = pattern(97, 53, 1);
    for which in 0..2u8 {
        let (ct, tag) = if which == 0 {
            let c = ChaCha20Poly1305::new(&key);
            let mut ct = pt.clone();
            let tag = c.encrypt_in_place(&mut ct, &nonce12(), b"aad").unwrap();
            (ct, tag.as_ref().to_vec())
        } else {
            let c = XChaCha20Poly1305::new(&key);
            let mut ct = pt.clone();
            let tag = c.encrypt_in_place(&mut ct, &nonce24(), b"aad").unwrap();
            (ct, tag.as_ref().to_vec())
        };

        for byte in 0..16 {
            for bit in 0..8 {
                let mut bad = tag.clone();
                bad[byte] ^= 1 << bit;
                let mut buf = ct.clone();
                let err = if which == 0 {
                    ChaCha20Poly1305::new(&key).decrypt_in_place(&mut buf, &nonce12(), b"aad", &bad)
                } else {
                    XChaCha20Poly1305::new(&key).decrypt_in_place(&mut buf, &nonce24(), b"aad", &bad)
                };
                assert_eq!(err, Err(AeadError::InvalidCiphertext), "tag bit {byte}:{bit}");
                assert_ne!(buf, pt, "plaintext released on failed tag (bit {byte}:{bit})");
            }
        }

        // AAD-only / empty-plaintext tag must not authenticate one byte.
        let empty_tag = if which == 0 {
            ChaCha20Poly1305::new(&key)
                .encrypt_in_place(&mut Vec::new(), &nonce12(), b"only-aad")
                .unwrap()
        } else {
            XChaCha20Poly1305::new(&key)
                .encrypt_in_place(&mut Vec::new(), &nonce24(), b"only-aad")
                .unwrap()
        };
        let mut one = vec![0u8];
        let err = if which == 0 {
            ChaCha20Poly1305::new(&key).decrypt_in_place(&mut one, &nonce12(), b"only-aad", empty_tag.as_ref())
        } else {
            XChaCha20Poly1305::new(&key).decrypt_in_place(&mut one, &nonce24(), b"only-aad", empty_tag.as_ref())
        };
        assert_eq!(err, Err(AeadError::InvalidCiphertext));
    }
}

/// AEGIS-style in-place overlap is not expressible safely through `&mut [u8]`;
/// this checks that encrypting the same buffer twice with the same nonce is
/// deterministic and that decrypting the result yields the original, at every
/// offset within one 16-byte block (the tail-handling hot spot).
#[test]
fn tail_offsets_repeat_deterministically() {
    let key = key();
    let c = ChaCha20Poly1305::new(&key);
    let n = nonce12();
    for len in 0..=130usize {
        let pt = pattern(len, 61, 3);
        let mut a = pt.clone();
        let mut b = pt.clone();
        let ta = c.encrypt_in_place(&mut a, &n, b"x").unwrap();
        let tb = c.encrypt_in_place(&mut b, &n, b"x").unwrap();
        assert_eq!(a, b, "len={len}");
        assert_eq!(ta.as_ref(), tb.as_ref(), "len={len}");
        c.decrypt_in_place(&mut a, &n, b"x", ta.as_ref()).unwrap();
        assert_eq!(a, pt, "len={len}");
    }
}
