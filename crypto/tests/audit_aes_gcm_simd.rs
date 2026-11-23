//! Tests for the AES-GCM / GHASH SIMD kernels.
//!
//! Scope: `aes/aes_gcm_amd64.rs`, `aes/aes_gcm_arm64.rs`,
//! `aes/ghash_amd64.rs`, `aes/ghash_arm64.rs`.
//!
//! The tests here drive the public dispatch and compare *both* ciphertext and
//! tag against an independent
//! implementation of GCM built from:
//!
//! * a branchy bit-serial GF(2^128) multiplier (the textbook
//!   McGrew–Viega SP 800-38D algorithm, not the crate's constant-time
//!   multiplier), and
//! * the crate's public AES block cipher (`Aes128Block` / `Aes256Block`) for
//!   `E(K, ·)` only.
//!
//! That gives a second, independent oracle for GHASH, padding, length-block
//! encoding, counter handling and the partial final block.
//!
//! The x86-64 kernels are compile-checked separately; the tests below exercise
//! whichever backend dispatch selects for the current target.

use crypto::{
    Aead, AeadError,
    aes::{Aes128Block, Aes128Gcm, Aes256Block, Aes256Gcm},
};

// ── Independent GCM reference ────────────────────────────────────────────────

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

/// Bit-serial GF(2^128) multiplication, GCM bit order (big-endian).
///
/// This is deliberately *not* the implementation under test: it uses a data
/// dependent branch and processes one bit at a time.
fn gf_mul_ref(x: &[u8; 16], h: &[u8; 16]) -> [u8; 16] {
    let xv = u128::from_be_bytes(*x);
    let mut v = u128::from_be_bytes(*h);
    let mut z = 0u128;
    for k in (0..128u32).rev() {
        if (xv >> k) & 1 == 1 {
            z ^= v;
        }
        let carry = v & 1;
        v >>= 1;
        if carry != 0 {
            v ^= 0xe1u128 << 120;
        }
    }
    z.to_be_bytes()
}

/// Spec GHASH feed: zero-pad the final partial block, then multiply by H.
fn ghash_update_ref(state: &mut [u8; 16], h: &[u8; 16], data: &[u8]) {
    for chunk in data.chunks(16) {
        let mut block = [0u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        for i in 0..16 {
            state[i] ^= block[i];
        }
        *state = gf_mul_ref(state, h);
    }
}

/// Increment the last 32 bits of a big-endian counter block (GCM inc32).
fn inc32(counter: &mut [u8; 16]) {
    let mut carry = 1u16;
    for byte in counter[12..].iter_mut().rev() {
        let sum = *byte as u16 + carry;
        *byte = sum as u8;
        carry = sum >> 8;
    }
    assert_eq!(carry, 0, "GCM inc32 overflowed after 2^32 blocks");
}

struct RefGcm {
    encrypt_block: Box<dyn Fn(&[u8; 16]) -> [u8; 16]>,
}

impl RefGcm {
    fn aes128(key: &[u8; 16]) -> Self {
        let cipher = Aes128Block::new(key);
        Self {
            encrypt_block: Box::new(move |block| cipher.encrypt_block(block)),
        }
    }

    fn aes256(key: &[u8; 32]) -> Self {
        let cipher = Aes256Block::new(key);
        Self {
            encrypt_block: Box::new(move |block| cipher.encrypt_block(block)),
        }
    }

    /// Encrypt `plaintext`, returning `(ciphertext, tag)`.
    fn encrypt(&self, nonce: &[u8; 12], aad: &[u8], plaintext: &[u8]) -> (Vec<u8>, [u8; 16]) {
        let h = (self.encrypt_block)(&[0u8; 16]);

        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;
        let ej0 = (self.encrypt_block)(&j0);

        let mut counter = j0;
        inc32(&mut counter);

        let mut ciphertext = Vec::with_capacity(plaintext.len());
        for chunk in plaintext.chunks(16) {
            let keystream = (self.encrypt_block)(&counter);
            let mut block = [0u8; 16];
            for (i, byte) in chunk.iter().enumerate() {
                block[i] = byte ^ keystream[i];
            }
            ciphertext.extend_from_slice(&block[..chunk.len()]);
            inc32(&mut counter);
        }

        let mut state = [0u8; 16];
        ghash_update_ref(&mut state, &h, aad);
        ghash_update_ref(&mut state, &h, &ciphertext);

        let mut len_block = [0u8; 16];
        len_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
        len_block[8..].copy_from_slice(&((plaintext.len() as u64) * 8).to_be_bytes());
        ghash_update_ref(&mut state, &h, &len_block);

        let mut tag = [0u8; 16];
        for i in 0..16 {
            tag[i] = state[i] ^ ej0[i];
        }

        (ciphertext, tag)
    }
}

// ── Published vectors cross-check the reference itself ──────────────────────

#[test]
fn gcm_length_block_arithmetic_boundary() {
    // The ciphertext limit is (2^32 - 2) * 16 bytes, and its bit length fits.
    const GCM_MAX_LEN: u64 = (u32::MAX as u64 - 1) * 16;
    assert_eq!(GCM_MAX_LEN.checked_mul(8), Some(GCM_MAX_LEN * 8));
    assert_eq!((1u64 << 61).checked_mul(8), None);
}

/// The independent reference must reproduce NIST SP 800-38D AES-256-GCM
/// test cases 13–16 before it can be used as an oracle.
#[test]
fn reference_matches_nist_vectors() {
    let key = hex::decode_array::<32>(("feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308").as_bytes())
        .unwrap();
    let nonce = hex::decode_array::<12>(("cafebabefacedbaddecaf888").as_bytes()).unwrap();
    let zero_nonce = [0u8; 12];
    let reference = RefGcm::aes256(&key);
    let zero_key_ref = RefGcm::aes256(&[0u8; 32]);

    let cases: [(&RefGcm, &[u8; 12], &str, &str, &str, &str); 4] = [
        // tc13: empty
        (&zero_key_ref, &zero_nonce, "", "", "", "530f8afbc74536b9a963b4f1c4cb738b"),
        // tc14: one zero block
        (
            &zero_key_ref,
            &zero_nonce,
            "00000000000000000000000000000000",
            "",
            "cea7403d4d606b6e074ec5d3baf39d18",
            "d0d1c8a799996bf0265b98b5d48ab919",
        ),
        // tc15: 64-byte plaintext, no AAD
        (
            &reference,
            &nonce,
            "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255",
            "",
            "522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662898015ad",
            "b094dac5d93471bdec1a502270e3cc6c",
        ),
        // tc16: 60-byte plaintext, 20-byte AAD
        (
            &reference,
            &nonce,
            "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39",
            "feedfacedeadbeeffeedfacedeadbeefabaddad2",
            "522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662",
            "76fc6ece0f4e1768cddf8853bb2d551b",
        ),
    ];

    for (idx, (reference, nonce, pt, aad, want_ct, want_tag)) in cases.iter().enumerate() {
        let pt = hex::decode(pt).unwrap();
        let aad = hex::decode(aad).unwrap();
        let (ct, tag) = reference.encrypt(nonce, &aad, &pt);
        assert_eq!(
            ct,
            hex::decode(want_ct).unwrap(),
            "reference ciphertext mismatch for vector {idx}"
        );
        assert_eq!(
            tag,
            hex::decode_array::<16>((want_tag).as_bytes()).unwrap(),
            "reference tag mismatch for vector {idx}"
        );
    }
}

// ── Dispatch (ARMv8 here) vs independent reference ──────────────────────────

fn check_128(reference: &RefGcm, cipher: &Aes128Gcm, nonce: &[u8; 12], aad: &[u8], pt: &[u8], label: &str) {
    let (want_ct, want_tag) = reference.encrypt(nonce, aad, pt);

    let mut got = pt.to_vec();
    let tag = cipher.encrypt_in_place(&mut got, nonce, aad).unwrap();
    assert_eq!(got, want_ct, "AES-128-GCM ciphertext mismatch ({label})");
    assert_eq!(tag.as_ref(), &want_tag[..], "AES-128-GCM tag mismatch ({label})");

    // Decrypt through the hardware path and check the plaintext.
    let mut back = want_ct.clone();
    cipher
        .decrypt_in_place(&mut back, nonce, aad, &want_tag)
        .unwrap_or_else(|e| panic!("AES-128-GCM decrypt failed ({label}): {e}"));
    assert_eq!(back, pt, "AES-128-GCM roundtrip mismatch ({label})");
}

fn check_256(reference: &RefGcm, cipher: &Aes256Gcm, nonce: &[u8; 12], aad: &[u8], pt: &[u8], label: &str) {
    let (want_ct, want_tag) = reference.encrypt(nonce, aad, pt);

    let mut got = pt.to_vec();
    let tag = cipher.encrypt_in_place(&mut got, nonce, aad).unwrap();
    assert_eq!(got, want_ct, "AES-256-GCM ciphertext mismatch ({label})");
    assert_eq!(tag.as_ref(), &want_tag[..], "AES-256-GCM tag mismatch ({label})");

    let mut back = want_ct.clone();
    cipher
        .decrypt_in_place(&mut back, nonce, aad, &want_tag)
        .unwrap_or_else(|e| panic!("AES-256-GCM decrypt failed ({label}): {e}"));
    assert_eq!(back, pt, "AES-256-GCM roundtrip mismatch ({label})");
}

/// Every plaintext length 0..=256 against a spread of AAD lengths, including
/// lengths that are not multiples of 16 and lengths that differ from the
/// plaintext length. 0..=256 covers the 16/64/128-byte single-, four- and
/// eight-block loop boundaries and their ±1 neighbours.
#[test]
fn gcm_matches_reference_all_short_lengths() {
    let key128 = [0x3cu8; 16];
    let key256 = [0x3cu8; 32];
    let nonce = [0x5au8; 12];
    let ref128 = RefGcm::aes128(&key128);
    let ref256 = RefGcm::aes256(&key256);
    let cipher128 = Aes128Gcm::new(&key128);
    let cipher256 = Aes256Gcm::new(&key256);

    let aad_lengths = [0usize, 1, 2, 15, 16, 17, 31, 32, 33];

    for pt_len in 0..=256usize {
        let pt = pattern(pt_len, 37, 11);
        for &aad_len in &aad_lengths {
            let aad = pattern(aad_len, 53, 7);
            let label = format!("pt_len={pt_len} aad_len={aad_len}");
            check_128(&ref128, &cipher128, &nonce, &aad, &pt, &label);
            check_256(&ref256, &cipher256, &nonce, &aad, &pt, &label);
        }
    }
}

/// Boundary sweep across the multi-block loop thresholds: 511/512/513 (the
/// 4-block loop's 64-byte stride nearby), 1023/1024/1025 (8-block and 16-block
/// stride neighbourhoods) plus the exact 16-byte boundaries.
#[test]
fn gcm_matches_reference_boundary_lengths() {
    let key128 = [0x11u8; 16];
    let key256 = [0x22u8; 32];
    let nonce = [0xa5u8; 12];
    let ref128 = RefGcm::aes128(&key128);
    let ref256 = RefGcm::aes256(&key256);
    let cipher128 = Aes128Gcm::new(&key128);
    let cipher256 = Aes256Gcm::new(&key256);

    let mut pt_lengths = vec![
        31usize, 32, 33, 47, 48, 49, 63, 64, 65, 79, 80, 81, 127, 128, 129, 143, 144, 145, 191, 192, 193, 255, 256,
        257, 383, 384, 385, 511, 512, 513, 767, 768, 769, 1023, 1024, 1025,
    ];
    // Every exact multiple of 16 in the interesting range, so a partial block
    // is never accidentally the only shape tested.
    for multiple in [16usize, 32, 48, 64, 80, 96, 112, 128, 240, 256, 512, 1024] {
        pt_lengths.push(multiple);
    }
    pt_lengths.sort_unstable();
    pt_lengths.dedup();

    let aad_lengths = [
        0usize, 1, 15, 16, 17, 20, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257,
    ];

    for &pt_len in &pt_lengths {
        let pt = pattern(pt_len, 41, 5);
        for &aad_len in &aad_lengths {
            let aad = pattern(aad_len, 17, 9);
            let label = format!("pt_len={pt_len} aad_len={aad_len}");
            check_128(&ref128, &cipher128, &nonce, &aad, &pt, &label);
            check_256(&ref256, &cipher256, &nonce, &aad, &pt, &label);
        }
    }
}

/// Empty plaintext, empty AAD and both empty must all be handled and must not
/// leave any output beyond the 16-byte tag.
#[test]
fn gcm_empty_input_combinations() {
    let key256 = [0x77u8; 32];
    let nonce = [0x01u8; 12];
    let reference = RefGcm::aes256(&key256);
    let cipher = Aes256Gcm::new(&key256);

    for aad_len in [0usize, 1, 15, 16, 17, 32] {
        let aad = pattern(aad_len, 29, 3);
        for pt_len in [0usize, 1, 15, 16, 17] {
            let pt = pattern(pt_len, 31, 13);
            check_256(
                &reference,
                &cipher,
                &nonce,
                &aad,
                &pt,
                &format!("empty-combo pt={pt_len} aad={aad_len}"),
            );
        }
    }
}

/// Regression: AAD and ciphertext are padded independently. A non-multiple-of-16
/// AAD followed by a non-multiple-of-16 plaintext exercises two different zero
/// padded tails in the same tag computation.
#[test]
fn gcm_aad_and_ciphertext_partial_blocks_differ() {
    let key128 = [0x5eu8; 16];
    let key256 = [0x6fu8; 32];
    let nonce = [0xc3u8; 12];

    for aad_len in 1..=48usize {
        for pt_len in 1..=48usize {
            let aad = pattern(aad_len, 7, 21);
            let pt = pattern(pt_len, 19, 23);
            check_128(
                &RefGcm::aes128(&key128),
                &Aes128Gcm::new(&key128),
                &nonce,
                &aad,
                &pt,
                &format!("partial aad={aad_len} pt={pt_len}"),
            );
            check_256(
                &RefGcm::aes256(&key256),
                &Aes256Gcm::new(&key256),
                &nonce,
                &aad,
                &pt,
                &format!("partial aad={aad_len} pt={pt_len}"),
            );
        }
    }
}

// ── Tamper / tag-failure behaviour ───────────────────────────────────────────

#[test]
fn decrypt_rejects_every_tag_bit_flip() {
    let key = [0x33u8; 32];
    let nonce = [0x44u8; 12];
    let aad = pattern(37, 11, 2);
    let pt = pattern(300, 23, 5);
    let cipher = Aes256Gcm::new(&key);

    let mut ct = pt.clone();
    let tag: [u8; 16] = cipher
        .encrypt_in_place(&mut ct, &nonce, &aad)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();

    for byte in 0..16 {
        for bit in 0..8 {
            let mut bad = tag;
            bad[byte] ^= 1 << bit;
            let mut buf = ct.clone();
            assert!(
                cipher.decrypt_in_place(&mut buf, &nonce, &aad, &bad).is_err(),
                "accepted forged tag bit {byte}:{bit}"
            );
        }
    }
}

#[test]
fn decrypt_rejects_ciphertext_aad_and_nonce_tampering() {
    let key = [0x33u8; 32];
    let nonce = [0x44u8; 12];
    let aad = pattern(40, 11, 2);
    let pt = pattern(275, 23, 5);
    let cipher = Aes256Gcm::new(&key);

    let mut ct = pt.clone();
    let tag: [u8; 16] = cipher
        .encrypt_in_place(&mut ct, &nonce, &aad)
        .unwrap()
        .as_ref()
        .try_into()
        .unwrap();

    // Every ciphertext byte.
    for i in 0..ct.len() {
        let mut tampered = ct.clone();
        tampered[i] ^= 0x80;
        assert!(
            cipher.decrypt_in_place(&mut tampered, &nonce, &aad, &tag).is_err(),
            "accepted ciphertext byte {i} flip"
        );
    }

    // Every AAD byte.
    for i in 0..aad.len() {
        let mut tampered_aad = aad.clone();
        tampered_aad[i] ^= 0x01;
        let mut buf = ct.clone();
        assert!(
            cipher.decrypt_in_place(&mut buf, &nonce, &tampered_aad, &tag).is_err(),
            "accepted AAD byte {i} flip"
        );
    }

    // Nonce changes.
    for i in 0..12 {
        let mut tampered_nonce = nonce;
        tampered_nonce[i] ^= 0x01;
        let mut buf = ct.clone();
        assert!(
            cipher.decrypt_in_place(&mut buf, &tampered_nonce, &aad, &tag).is_err(),
            "accepted nonce byte {i} flip"
        );
    }

    // Tag length and ciphertext-length changes are rejected by the wrapper.
    let mut buf = ct.clone();
    assert_eq!(
        cipher.decrypt_in_place(&mut buf, &nonce, &aad, &tag[..15]).unwrap_err(),
        AeadError::InvalidCiphertext
    );

    let mut shortened = ct[..ct.len() - 1].to_vec();
    assert!(cipher.decrypt_in_place(&mut shortened, &nonce, &aad, &tag).is_err());

    // The genuine tag still authenticates afterwards.
    let mut good = ct.clone();
    cipher
        .decrypt_in_place(&mut good, &nonce, &aad, &tag)
        .expect("genuine tag rejected");
    assert_eq!(good, pt);
}

// ── Canaries and non-zero slice offsets ──────────────────────────────────────

/// The kernels load the partial final block through a 16-byte stack buffer, so
/// they must never read or write past `in_out`. Put the payload at several
/// offsets inside a larger allocation and check that every byte outside the
/// slice is untouched.
#[test]
fn gcm_in_place_preserves_canaries_at_offsets() {
    let key128 = [0x12u8; 16];
    let nonce = [0x56u8; 12];

    let lengths = [
        0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 511, 512, 513, 1023,
    ];

    for &len in &lengths {
        let plaintext = pattern(len, 43, 9);
        let aad = pattern(37, 13, 1);

        for offset in [0usize, 1, 3, 7, 8, 15] {
            let mut storage = vec![0x5au8; len + offset + 32];

            // Copy the plaintext into the middle region, then encrypt only that.
            storage[offset..offset + len].copy_from_slice(&plaintext);

            let cipher = Aes128Gcm::new(&key128);
            let tag = cipher
                .encrypt_in_place(&mut storage[offset..offset + len], &nonce, &aad)
                .unwrap();

            assert!(
                storage[..offset].iter().all(|&b| b == 0x5a),
                "prefix canary overwritten len={len} offset={offset}"
            );
            assert!(
                storage[offset + len..].iter().all(|&b| b == 0x5a),
                "suffix canary overwritten len={len} offset={offset}"
            );

            // Compare the ciphertext slice against the reference.
            let reference = RefGcm::aes128(&key128);
            let (want_ct, want_tag) = reference.encrypt(&nonce, &aad, &plaintext);
            assert_eq!(&storage[offset..offset + len], &want_ct[..], "len={len} offset={offset}");
            assert_eq!(tag.as_ref(), &want_tag[..], "len={len} offset={offset}");

            // Decrypt in place at the same offset with the same cipher.
            let mut storage = vec![0x5au8; len + offset + 32];
            storage[offset..offset + len].copy_from_slice(&want_ct);
            cipher
                .decrypt_in_place(&mut storage[offset..offset + len], &nonce, &aad, &want_tag)
                .expect("decrypt at offset");
            assert_eq!(&storage[offset..offset + len], &plaintext[..]);
            assert!(storage[..offset].iter().all(|&b| b == 0x5a));
            assert!(storage[offset + len..].iter().all(|&b| b == 0x5a));
        }
    }
}

/// AAD is read-only and may be a subslice at an odd offset. Exercise that
/// together with an odd-offset plaintext.
#[test]
fn gcm_offset_aad_slice() {
    let key = [0x9au8; 32];
    let nonce = [0xbcu8; 12];
    let reference = RefGcm::aes256(&key);
    let cipher = Aes256Gcm::new(&key);

    for aad_len in [0usize, 1, 15, 16, 17, 33, 64, 129] {
        let aad_backing = pattern(aad_len + 23, 57, 3);
        for aad_offset in [0usize, 1, 5, 11] {
            let aad = &aad_backing[aad_offset..aad_offset + aad_len];
            for pt_len in [0usize, 1, 16, 17, 128, 129] {
                let mut backing = pattern(pt_len + 19, 61, 7);
                let initial = backing.clone();
                let pt = backing[5..5 + pt_len].to_vec();
                let tag = cipher
                    .encrypt_in_place(&mut backing[5..5 + pt_len], &nonce, aad)
                    .unwrap();
                let (want_ct, want_tag) = reference.encrypt(&nonce, aad, &pt);
                assert_eq!(&backing[5..5 + pt_len], &want_ct[..]);
                assert_eq!(tag.as_ref(), &want_tag[..]);
                // Bytes outside the processed slice are untouched.
                assert_eq!(&backing[..5], &initial[..5], "prefix canary");
                assert_eq!(&backing[5 + pt_len..], &initial[5 + pt_len..], "suffix canary");
            }
        }
    }
}

// ── Length/limit sanity ─────────────────────────────────────────────────────

/// The GHASH power tables are fixed-size and indexed only by constant block
/// positions (0..8 for ARMv8, 0..16 for x86-64), never by data length. This test
/// simply confirms the hardware path still agrees with the reference at the
/// largest lengths we can afford to allocate, including many loop iterations.
#[test]
fn gcm_long_input_matches_reference() {
    let key = [0x7eu8; 32];
    let nonce = [0x1fu8; 12];
    let aad = pattern(257, 3, 4);
    let plaintext = pattern(64 * 1024 + 3, 101, 17);

    let reference = RefGcm::aes256(&key);
    let cipher = Aes256Gcm::new(&key);

    let (want_ct, want_tag) = reference.encrypt(&nonce, &aad, &plaintext);
    let mut got = plaintext.clone();
    let tag = cipher.encrypt_in_place(&mut got, &nonce, &aad).unwrap();
    assert_eq!(got, want_ct);
    assert_eq!(tag.as_ref(), &want_tag[..]);

    cipher
        .decrypt_in_place(&mut got, &nonce, &aad, &want_tag)
        .expect("long decrypt");
    assert_eq!(got, plaintext);
}
