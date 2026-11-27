//! Adversarial parameter-boundary tests for the symmetric KDF/MAC/hash unit:
//! Ascon-AEAD128, BLAKE2b, HMAC, HKDF and PBKDF2.
//!
//! These complement the RFC/NIST/Wycheproof known-answer tests by exercising
//! the lengths and parameter values that a network-facing caller can control:
//! nonce/tag lengths, zero iterations, maximum HKDF output, block boundaries,
//! empty keys/salts, and failure-path buffer handling.

use crypto::{
    Aead, AeadError, Hasher, HkdfError,
    ascon::AsconAead128,
    blake2::Blake2b,
    hkdf,
    hmac::Hmac,
    pbkdf2,
    pbkdf2::Pbkdf2Error,
    sha2::{Sha256, Sha512},
    sha3::Sha3_256,
};

/// Ascon-AEAD128 must reject non-16-byte nonces and tags, and must not write
/// plaintext into the buffer when authentication fails.
#[test]
fn ascon_nonce_tag_and_failure_paths() {
    let key = [0x42u8; 16];
    let cipher = AsconAead128::new(&key);
    let nonce = [0x24u8; 16];

    let mut pt = b"ascon boundary test".to_vec();
    let tag = cipher.encrypt_in_place(&mut pt, &nonce, b"aad").unwrap();
    assert_eq!(tag.as_ref().len(), 16);

    // Wrong nonce lengths: 0..15 and 17 must error before touching the buffer.
    for len in 0..=17 {
        if len == 16 {
            continue;
        }
        let bad_nonce = vec![0u8; len];
        let mut buf = pt.clone();
        let before = buf.clone();
        assert_eq!(
            cipher.decrypt_in_place(&mut buf, &bad_nonce, b"aad", tag.as_ref()),
            Err(AeadError::InvalidNonce)
        );
        assert_eq!(buf, before, "nonce length {len} modified the buffer");

        let mut enc = pt.clone();
        assert!(matches!(
            cipher.encrypt_in_place(&mut enc, &bad_nonce, b"aad"),
            Err(AeadError::InvalidNonce)
        ));
        assert_eq!(enc, pt, "encrypt with nonce length {len} modified the buffer");
    }

    // Wrong tag lengths must error; only 16 bytes is considered.
    for len in 0..=17 {
        if len == 16 {
            continue;
        }
        let bad_tag = vec![0u8; len];
        let mut buf = pt.clone();
        assert_eq!(
            cipher.decrypt_in_place(&mut buf, &nonce, b"aad", &bad_tag),
            Err(AeadError::InvalidCiphertext)
        );
    }

    // A one-bit tag change must fail and zeroize the output buffer.
    let mut bad_tag = [0u8; 16];
    bad_tag.copy_from_slice(tag.as_ref());
    bad_tag[15] ^= 1;
    let mut buf = pt.clone();
    assert_eq!(
        cipher.decrypt_in_place(&mut buf, &nonce, b"aad", &bad_tag),
        Err(AeadError::InvalidCiphertext)
    );
    assert_eq!(buf, vec![0u8; pt.len()], "failed decryption left plaintext/ciphertext");

    // All-zero/all-0xFF nonces and tags with a valid ciphertext must fail
    // cleanly for every data length around the 16-byte rate boundary.
    for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 255] {
        let mut msg = vec![0x5Au8; len];
        let tag = cipher.encrypt_in_place(&mut msg, &nonce, b"").unwrap();
        let mut good = msg.clone();
        cipher.decrypt_in_place(&mut good, &nonce, b"", tag.as_ref()).unwrap();
        assert_eq!(good, vec![0x5Au8; len]);

        for bad_nonce in [[0u8; 16], [0xFFu8; 16]] {
            let mut buf = msg.clone();
            assert!(
                cipher
                    .decrypt_in_place(&mut buf, &bad_nonce, b"", tag.as_ref())
                    .is_err()
            );
        }
        for bad_tag in [[0u8; 16], [0xFFu8; 16]] {
            let mut buf = msg.clone();
            assert!(cipher.decrypt_in_place(&mut buf, &nonce, b"", &bad_tag).is_err());
            assert_eq!(buf, vec![0u8; len]);
        }
    }
}

/// Ascon accepts empty and maximum-key values, and AAD length boundaries do not
/// change the round trip.
#[test]
fn ascon_key_and_aad_boundaries() {
    for key in [[0u8; 16], [0xFFu8; 16]] {
        let cipher = AsconAead128::new(&key);
        for aad_len in [0usize, 1, 15, 16, 17, 32, 33] {
            let aad = vec![0xA5u8; aad_len];
            for pt_len in [0usize, 1, 15, 16, 17, 64] {
                let mut buf = vec![0x3Cu8; pt_len];
                let tag = cipher.encrypt_in_place(&mut buf, &[0x11u8; 16], &aad).unwrap();
                cipher
                    .decrypt_in_place(&mut buf, &[0x11u8; 16], &aad, tag.as_ref())
                    .unwrap();
                assert_eq!(buf, vec![0x3Cu8; pt_len]);
            }
        }
    }
}

/// HKDF output and PRK length boundaries, including the RFC 5869 255·HashLen
/// limit and the `n == 0` fast path.
#[test]
fn hkdf_output_and_prk_boundaries() {
    let ikm = b"hkdf boundary input key material";
    let prk = hkdf::extract::<Sha256>(ikm, Some(b"salt"));
    assert_eq!(prk.as_ref().len(), 32);

    let max = 255 * 32;
    for len in [0usize, 1, 31, 32, 33, 255, max - 1, max] {
        let mut okm = vec![0xA5u8; len];
        hkdf::expand::<Sha256>(&mut okm, prk.as_ref(), b"info")
            .unwrap_or_else(|e| panic!("okm len {len} rejected: {e:?}"));
        // The zero-length output must stay untouched.
        if len == 0 {
            assert!(okm.is_empty());
        }
    }
    let mut too_long = vec![0u8; max + 1];
    assert_eq!(
        hkdf::expand::<Sha256>(&mut too_long, prk.as_ref(), b""),
        Err(HkdfError::OutputIsTooLong)
    );

    for prk_len in [0usize, 1, 31] {
        let short = vec![0u8; prk_len];
        let mut okm = [0u8; 32];
        assert!(matches!(
            hkdf::expand::<Sha256>(&mut okm, &short, b""),
            Err(HkdfError::PrkIsTooShort(32))
        ));
    }
    // Longer PRKs are accepted (only the first HashLen bytes are used).
    for prk_len in [32usize, 33, 64, 200] {
        let long = vec![0x11u8; prk_len];
        let mut okm = [0u8; 32];
        hkdf::expand::<Sha256>(&mut okm, &long, b"").unwrap();
        let mut expected = [0u8; 32];
        hkdf::expand::<Sha256>(&mut expected, &long[..32], b"").unwrap();
        assert_eq!(okm, expected);
    }

    // `derive_key` is extract-then-expand and respects the same limit.
    let one = hkdf::derive_key::<Sha256, 1>(ikm, b"info", Some(b"salt")).unwrap();
    let mut manual = [0u8; 1];
    hkdf::expand::<Sha256>(&mut manual, prk.as_ref(), b"info").unwrap();
    assert_eq!(one, manual);

    // Empty and absent salts agree (both mean HashLen zero bytes).
    let none = hkdf::extract::<Sha256>(ikm, None);
    let empty = hkdf::extract::<Sha256>(ikm, Some(b""));
    assert_eq!(none.as_ref(), empty.as_ref());

    // SHA-512 has its own 255·64 maximum.
    let prk512 = hkdf::extract::<Sha512>(ikm, None);
    let max512 = 255 * 64;
    let mut okm = vec![0u8; max512];
    hkdf::expand::<Sha512>(&mut okm, prk512.as_ref(), b"").unwrap();
    let mut too_long = vec![0u8; max512 + 1];
    assert_eq!(
        hkdf::expand::<Sha512>(&mut too_long, prk512.as_ref(), b""),
        Err(HkdfError::OutputIsTooLong)
    );
}

/// PBKDF2 rejects zero iterations (including with empty password/salt), handles
/// zero-length output, and crosses the hash/block boundaries without overflow.
#[test]
fn pbkdf2_parameter_boundaries() {
    assert_eq!(
        pbkdf2::derive::<Sha256, 32>(b"password", b"salt", 0),
        Err(Pbkdf2Error::ZeroIterations)
    );
    assert_eq!(pbkdf2::derive::<Sha256, 0>(b"", b"", 0), Err(Pbkdf2Error::ZeroIterations));
    assert_eq!(pbkdf2::derive::<Sha256, 0>(b"", b"", 1).unwrap(), [] as [u8; 0]);

    for iterations in [1u32, 2, 3, 16] {
        let derived = pbkdf2::derive::<Sha256, 65>(b"pw", b"salt", iterations).unwrap();
        assert_eq!(derived.len(), 65);
        // The first 32 bytes must equal the shorter derivation.
        let short = pbkdf2::derive::<Sha256, 32>(b"pw", b"salt", iterations).unwrap();
        assert_eq!(&derived[..32], &short[..]);
    }

    // Empty and very long passwords/salts must not overflow the HMAC key path.
    for pw_len in [0usize, 1, 63, 64, 65, 200] {
        for salt_len in [0usize, 1, 64, 200] {
            let pw = vec![0x11u8; pw_len];
            let salt = vec![0x22u8; salt_len];
            let out = pbkdf2::derive::<Sha256, 16>(&pw, &salt, 1).unwrap();
            assert_eq!(out.len(), 16);
        }
    }

    // Block index boundaries: 64-byte hash means output 63/64/65 exercises the
    // final partial block.
    let a = pbkdf2::derive::<Sha256, 64>(b"p", b"s", 2).unwrap();
    let b = pbkdf2::derive::<Sha256, 65>(b"p", b"s", 2).unwrap();
    assert_eq!(&a[..], &b[..64]);
}

/// HMAC accepts empty, block-sized, over-block, and very long keys; streaming
/// and one-shot results agree across block boundaries.
#[test]
fn hmac_key_and_data_boundaries() {
    for key_len in [0usize, 1, 63, 64, 65, 127, 128, 129, 1000] {
        let key = vec![0xA5u8; key_len];
        for data_len in [0usize, 1, 63, 64, 65, 128, 1000] {
            let data = vec![0x5Au8; data_len];
            let one_shot = Hmac::<Sha256>::mac(&key, &data);

            let mut mac = Hmac::<Sha256>::new(&key);
            for chunk in data.chunks(7) {
                mac.update(chunk);
            }
            assert_eq!(mac.finalize().as_ref(), one_shot.as_ref(), "key {key_len} data {data_len}");
        }
    }

    // Changing the key or data must change the tag.
    let base = Hmac::<Sha256>::mac(b"key", b"data");
    assert_ne!(base.as_ref(), Hmac::<Sha256>::mac(b"key", b"datb").as_ref());
    assert_ne!(base.as_ref(), Hmac::<Sha256>::mac(b"kez", b"data").as_ref());

    // SHA-3 HMAC also supports block-sized and over-block keys.
    for key_len in [0usize, 136, 137, 500] {
        let key = vec![0x11u8; key_len];
        let tag = Hmac::<Sha3_256>::mac(&key, b"message");
        assert_eq!(tag.as_ref().len(), 32);
    }
}

/// BLAKE2b output/key length validation and streaming across the 128-byte block
/// boundary.
#[test]
fn blake2b_parameter_boundaries() {
    for outlen in [0usize, 65, 128] {
        assert!(Blake2b::new_keyed(b"", outlen).is_err());
    }
    for key_len in [65usize, 128] {
        let key = vec![0u8; key_len];
        assert!(Blake2b::new_keyed(&key, 32).is_err());
    }

    for outlen in [1usize, 20, 32, 48, 64] {
        let mut h = Blake2b::new_keyed(&[], outlen).unwrap();
        h.update(b"");
        assert_eq!(h.sum().as_ref().len(), outlen);

        let mut h = Blake2b::new_keyed(&[0x42u8; 64], outlen).unwrap();
        h.update(b"keyed");
        assert_eq!(h.sum().as_ref().len(), outlen);
    }

    // Streaming in 1-byte chunks must match one-shot around the block boundary.
    for len in [0usize, 1, 127, 128, 129, 255, 256, 257, 1024] {
        let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(31)).collect();

        let mut one = Blake2b::new();
        one.update(&data);
        let one = one.sum();

        let mut streamed = Blake2b::new();
        for byte in &data {
            streamed.update(core::slice::from_ref(byte));
        }
        assert_eq!(streamed.sum().as_ref(), one.as_ref(), "len {len}");
    }

    // Keyed streaming must match keyed one-shot.
    for len in [0usize, 1, 128, 129, 300] {
        let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(7)).collect();
        let mut one = Blake2b::new_keyed(b"k", 64).unwrap();
        one.update(&data);
        let one = one.sum();

        let mut streamed = Blake2b::new_keyed(b"k", 64).unwrap();
        for chunk in data.chunks(13) {
            streamed.update(chunk);
        }
        assert_eq!(streamed.sum().as_ref(), one.as_ref(), "keyed len {len}");
    }
}
