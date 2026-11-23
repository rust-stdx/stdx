//! Tests for the public AES-GCM API, dispatch, and the
//! scalar GHASH kernel.
//!
//! The oracle used here is fully independent of the crate's GHASH: GCM blocks
//! are converted to a little-endian polynomial coefficient vector, multiplied
//! with a left-shifting `0x87`-reducing bit-serial routine, and converted back.
//! AES itself is taken from the crate's public block-cipher API, so this file
//! tests GCM construction (J0, inc32, padding, length block, tag mask) rather
//! than re-testing AES.
//!
//! Everything is deterministic (fixed seeds) and every output buffer is placed
//! at a non-zero offset between guard canaries, so over-reads and over-writes
//! at block/SIMD thresholds are visible.

use crypto::{
    Aead, AeadError,
    aes::{Aes128Block, Aes128Gcm, Aes256Block, Aes256Gcm},
};
use rand::{RngExt, SeedableRng, rngs::StdRng};

// ── deterministic pseudo-randomness ──────────────────────────────────────────

// ── guard canaries ───────────────────────────────────────────────────────────

const GUARD: usize = 32;
const CANARY: u8 = 0xa5;

fn guarded(data: &[u8]) -> Vec<u8> {
    let mut buf = vec![CANARY; data.len() + 2 * GUARD];
    buf[GUARD..GUARD + data.len()].copy_from_slice(data);
    buf
}

fn assert_guards(buf: &[u8], len: usize) {
    assert_eq!(buf.len(), len + 2 * GUARD);
    assert!(buf[..GUARD].iter().all(|&b| b == CANARY), "guard before buffer was clobbered");
    assert!(
        buf[GUARD + len..].iter().all(|&b| b == CANARY),
        "guard after buffer was clobbered"
    );
}

// ── independent GF(2^128) / GCM reference ────────────────────────────────────

/// GCM bit 0 (leftmost bit of byte 0) is the coefficient of x^0.
fn to_poly(block: &[u8; 16]) -> u128 {
    let mut bytes = *block;
    for byte in &mut bytes {
        *byte = byte.reverse_bits();
    }
    u128::from_le_bytes(bytes)
}

fn from_poly(value: u128) -> [u8; 16] {
    let mut bytes = value.to_le_bytes();
    for byte in &mut bytes {
        *byte = byte.reverse_bits();
    }
    bytes
}

/// Polynomial-orientation multiplication modulo x^128 + x^7 + x^2 + x + 1.
fn gf128_mul(x: &[u8; 16], h: &[u8; 16]) -> [u8; 16] {
    let xv = to_poly(x);
    let mut v = to_poly(h);
    let mut z = 0u128;
    for i in 0..128 {
        if (xv >> i) & 1 == 1 {
            z ^= v;
        }
        let carry = v >> 127;
        v <<= 1;
        if carry != 0 {
            v ^= 0x87;
        }
    }
    from_poly(z)
}

/// GHASH one padded input (AAD or ciphertext) into `state`.
fn ghash_update(state: &mut [u8; 16], h: &[u8; 16], data: &[u8]) {
    let (chunks, rem) = data.as_chunks::<16>();
    for block in chunks {
        for i in 0..16 {
            state[i] ^= block[i];
        }
        *state = gf128_mul(state, h);
    }
    if !rem.is_empty() {
        let mut padded = [0u8; 16];
        padded[..rem.len()].copy_from_slice(rem);
        for i in 0..16 {
            state[i] ^= padded[i];
        }
        *state = gf128_mul(state, h);
    }
}

/// Increment the low 32 bits of a GCM counter block (inc32).
fn inc32(counter: &mut [u8; 16]) {
    let mut value = u32::from_be_bytes(counter[12..16].try_into().unwrap());
    value = value.wrapping_add(1);
    counter[12..16].copy_from_slice(&value.to_be_bytes());
}

trait BlockCipher {
    fn encrypt(&self, block: &[u8; 16]) -> [u8; 16];
}

impl BlockCipher for Aes128Block {
    fn encrypt(&self, block: &[u8; 16]) -> [u8; 16] {
        self.encrypt_block(block)
    }
}

impl BlockCipher for Aes256Block {
    fn encrypt(&self, block: &[u8; 16]) -> [u8; 16] {
        self.encrypt_block(block)
    }
}

/// Independent AES-GCM encryption. Returns `(ciphertext, tag)`.
fn gcm_ref<B: BlockCipher>(block: &B, nonce: &[u8; 12], aad: &[u8], plaintext: &[u8]) -> (Vec<u8>, [u8; 16]) {
    let h = block.encrypt(&[0u8; 16]);

    let mut j0 = [0u8; 16];
    j0[..12].copy_from_slice(nonce);
    j0[15] = 1;
    let ej0 = block.encrypt(&j0);

    let mut counter = j0;
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    for chunk in plaintext.chunks(16) {
        inc32(&mut counter);
        let keystream = block.encrypt(&counter);
        for (byte, key) in chunk.iter().zip(keystream.iter()) {
            ciphertext.push(*byte ^ *key);
        }
    }

    let mut state = [0u8; 16];
    ghash_update(&mut state, &h, aad);
    ghash_update(&mut state, &h, &ciphertext);

    let mut length_block = [0u8; 16];
    length_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    length_block[8..].copy_from_slice(&((ciphertext.len() as u64) * 8).to_be_bytes());
    ghash_update(&mut state, &h, &length_block);

    for i in 0..16 {
        state[i] ^= ej0[i];
    }

    (ciphertext, state)
}

// ── shared helpers ───────────────────────────────────────────────────────────

/// Encrypt/decrypt one case against the independent reference, with the input
/// and output at a non-zero offset inside canary-guarded allocations.
fn check_case<B: BlockCipher, C: Aead>(
    block: &B,
    cipher: &C,
    nonce: &[u8; 12],
    aad: &[u8],
    plaintext: &[u8],
    label: &str,
) {
    let (ref_ct, ref_tag) = gcm_ref(block, nonce, aad, plaintext);

    // AAD must be read-only from the API's point of view.
    let aad_buf = guarded(aad);
    let aad_slice = &aad_buf[GUARD..GUARD + aad.len()];

    let mut buf = guarded(plaintext);
    let tag = cipher
        .encrypt_in_place(&mut buf[GUARD..GUARD + plaintext.len()], nonce, aad_slice)
        .unwrap();
    assert_guards(&buf, plaintext.len());
    assert_guards(&aad_buf, aad.len());
    assert_eq!(tag.len(), 16, "{label}: tag length");
    assert_eq!(
        &buf[GUARD..GUARD + plaintext.len()],
        &ref_ct[..],
        "{label}: ciphertext mismatch"
    );
    assert_eq!(tag.as_ref(), &ref_tag[..], "{label}: tag mismatch");

    // The independent ciphertext + tag must decrypt through the public API.
    let mut back = guarded(&ref_ct);
    cipher
        .decrypt_in_place(&mut back[GUARD..GUARD + ref_ct.len()], nonce, aad_slice, &ref_tag)
        .unwrap_or_else(|err| panic!("{label}: reference decrypt failed: {err:?}"));
    assert_guards(&back, ref_ct.len());
    assert_eq!(&back[GUARD..GUARD + plaintext.len()], plaintext, "{label}: plaintext mismatch");

    // Decrypting our own output must round-trip too.
    let mut roundtrip = buf.clone();
    cipher
        .decrypt_in_place(&mut roundtrip[GUARD..GUARD + plaintext.len()], nonce, aad_slice, &ref_tag)
        .unwrap_or_else(|err| panic!("{label}: roundtrip decrypt failed: {err:?}"));
    assert_guards(&roundtrip, plaintext.len());
    assert_eq!(&roundtrip[GUARD..GUARD + plaintext.len()], plaintext);

    // A tag that authenticates but a ciphertext that was modified must fail.
    // (Buffer contents on failure are explicitly unspecified by `Aead`; an
    // aarch64 fused backend may have decrypted the tampered ciphertext. We
    // only require the failure, no out-of-bounds write, and no panic.)
    if !ref_ct.is_empty() {
        let mut tampered = guarded(&ref_ct);
        tampered[GUARD + ref_ct.len() / 2] ^= 0x40;
        assert_eq!(
            cipher
                .decrypt_in_place(&mut tampered[GUARD..GUARD + ref_ct.len()], nonce, aad_slice, &ref_tag)
                .unwrap_err(),
            AeadError::InvalidCiphertext,
            "{label}: modified ciphertext accepted"
        );
        assert_guards(&tampered, ref_ct.len());
    }
}

/// On failure a backend may either leave the ciphertext untouched (verify then
/// decrypt, as x86_64 and the software path do) or replace it with the full
/// plaintext (the aarch64 fused path decrypts while authenticating). What it
/// must never do is leave a partial or mixed buffer behind.
fn assert_failure_buffer_is_consistent(buffer: &[u8], ciphertext: &[u8], plaintext: &[u8], label: &str) {
    if buffer == ciphertext {
        return;
    }
    assert_eq!(
        buffer, plaintext,
        "{label}: buffer after tag failure is neither the ciphertext nor the plaintext"
    );
}

fn reference_fixtures() -> [(Aes128Block, [u8; 16], [u8; 12]); 3] {
    [
        (Aes128Block::new(&[0u8; 16]), [0u8; 16], [0u8; 12]),
        (Aes128Block::new(&[0x2b; 16]), [0x2b; 16], [0x9c; 12]),
        (Aes128Block::new(&[0xfe; 16]), [0xfe; 16], [0x01; 12]),
    ]
}

fn reference_fixtures_256() -> [(Aes256Block, [u8; 32], [u8; 12]); 3] {
    [
        (Aes256Block::new(&[0u8; 32]), [0u8; 32], [0u8; 12]),
        (Aes256Block::new(&[0x3c; 32]), [0x3c; 32], [0x5a; 12]),
        (Aes256Block::new(&[0xfe; 32]), [0xfe; 32], [0x01; 12]),
    ]
}

// ── tests ────────────────────────────────────────────────────────────────────

#[test]
fn independent_reference_matches_nist_kat() {
    let block = Aes128Block::new(&[0u8; 16]);
    let (ct, tag) = gcm_ref(&block, &[0u8; 12], &[], &[]);
    assert!(ct.is_empty());
    assert_eq!(hex::encode(tag), "58e2fccefa7e3061367f1d57a4e7455a");

    let (ct, tag) = gcm_ref(&block, &[0u8; 12], &[], &[0u8; 16]);
    assert_eq!(hex::encode(ct), "0388dace60b6a392f328c2b971b2fe78");
    assert_eq!(hex::encode(tag), "ab6e47d42cec13bdf53a67b21257bddf");
}

#[test]
fn gcm_matches_independent_reference_sweep() {
    let mut rng = StdRng::seed_from_u64(0xc3_0001);
    let aad_lengths = [0usize, 1, 7, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129];

    for (block, key, nonce) in reference_fixtures() {
        let cipher = Aes128Gcm::new(&key);
        for pt_len in 0..=160usize {
            let plaintext = {
                let mut v = vec![0u8; pt_len];
                rng.fill(&mut v);
                v
            };
            for aad_len in aad_lengths {
                let aad = {
                    let mut v = vec![0u8; aad_len];
                    rng.fill(&mut v);
                    v
                };
                check_case(
                    &block,
                    &cipher,
                    &nonce,
                    &aad,
                    &plaintext,
                    &format!("aes128 pt={pt_len} aad={aad_len}"),
                );
            }
        }
    }

    for (block, key, nonce) in reference_fixtures_256() {
        let cipher = Aes256Gcm::new(&key);
        for pt_len in 0..=160usize {
            let plaintext = {
                let mut v = vec![0u8; pt_len];
                rng.fill(&mut v);
                v
            };
            for aad_len in aad_lengths {
                let aad = {
                    let mut v = vec![0u8; aad_len];
                    rng.fill(&mut v);
                    v
                };
                check_case(
                    &block,
                    &cipher,
                    &nonce,
                    &aad,
                    &plaintext,
                    &format!("aes256 pt={pt_len} aad={aad_len}"),
                );
            }
        }
    }
}

#[test]
fn gcm_matches_reference_at_block_and_simd_boundaries() {
    let mut rng = StdRng::seed_from_u64(0xc3_0002);
    // 1-byte increments around every 16/32/64/128/192/256/512/1024 boundary.
    let lengths: &[usize] = &[
        0, 1, 15, 16, 17, 31, 32, 33, 47, 48, 49, 63, 64, 65, 95, 96, 97, 127, 128, 129, 130, 191, 192, 193, 255, 256,
        257, 383, 384, 385, 511, 512, 513, 1023, 1024, 1025,
    ];
    let aad_lengths: &[usize] = &[0, 1, 15, 16, 17, 31, 32, 33, 64, 127, 128, 129, 256];

    for (block, key, nonce) in reference_fixtures() {
        let cipher = Aes128Gcm::new(&key);
        for &pt_len in lengths {
            let plaintext = {
                let mut v = vec![0u8; pt_len];
                rng.fill(&mut v);
                v
            };
            for &aad_len in aad_lengths {
                let aad = {
                    let mut v = vec![0u8; aad_len];
                    rng.fill(&mut v);
                    v
                };
                check_case(
                    &block,
                    &cipher,
                    &nonce,
                    &aad,
                    &plaintext,
                    &format!("aes128 boundary pt={pt_len} aad={aad_len}"),
                );
            }
        }
    }

    for (block, key, nonce) in reference_fixtures_256() {
        let cipher = Aes256Gcm::new(&key);
        for &pt_len in lengths {
            let plaintext = {
                let mut v = vec![0u8; pt_len];
                rng.fill(&mut v);
                v
            };
            for &aad_len in aad_lengths {
                let aad = {
                    let mut v = vec![0u8; aad_len];
                    rng.fill(&mut v);
                    v
                };
                check_case(
                    &block,
                    &cipher,
                    &nonce,
                    &aad,
                    &plaintext,
                    &format!("aes256 boundary pt={pt_len} aad={aad_len}"),
                );
            }
        }
    }
}

#[test]
fn empty_plaintext_and_empty_aad() {
    let mut rng = StdRng::seed_from_u64(0xc3_0003);
    let no_aad: &[u8] = &[];

    for (block, key, nonce) in reference_fixtures() {
        let cipher = Aes128Gcm::new(&key);

        // Both empty: the tag is E(J0) because GHASH(0 || len 0) = 0.
        check_case(&block, &cipher, &nonce, no_aad, &[], "aes128 empty/empty");

        // Empty plaintext, non-empty AAD.
        let aad = {
            let mut v = vec![0u8; 37];
            rng.fill(&mut v);
            v
        };
        check_case(&block, &cipher, &nonce, &aad, &[], "aes128 empty-pt");

        // Non-empty plaintext, empty AAD.
        let pt = {
            let mut v = vec![0u8; 37];
            rng.fill(&mut v);
            v
        };
        check_case(&block, &cipher, &nonce, no_aad, &pt, "aes128 empty-aad");

        // Exactly one block / one block + 1.
        for len in [16usize, 17] {
            let pt = {
                let mut v = vec![0u8; len];
                rng.fill(&mut v);
                v
            };
            check_case(&block, &cipher, &nonce, no_aad, &pt, &format!("aes128 pt={len}"));
        }
    }

    for (block, key, nonce) in reference_fixtures_256() {
        let cipher = Aes256Gcm::new(&key);
        check_case(&block, &cipher, &nonce, no_aad, &[], "aes256 empty/empty");
        let aad = {
            let mut v = vec![0u8; 37];
            rng.fill(&mut v);
            v
        };
        check_case(&block, &cipher, &nonce, &aad, &[], "aes256 empty-pt");
        let pt = {
            let mut v = vec![0u8; 37];
            rng.fill(&mut v);
            v
        };
        check_case(&block, &cipher, &nonce, no_aad, &pt, "aes256 empty-aad");
        for len in [16usize, 17] {
            let pt = {
                let mut v = vec![0u8; len];
                rng.fill(&mut v);
                v
            };
            check_case(&block, &cipher, &nonce, no_aad, &pt, &format!("aes256 pt={len}"));
        }
    }
}

fn check_tag_handling<C: Aead>(cipher: &C, nonce: [u8; 12], pt: &[u8], label: &str) {
    let aad = b"authenticated but not encrypted";
    let mut buf = pt.to_vec();
    let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad).unwrap();
    let tag: [u8; 16] = tag.as_ref().try_into().unwrap();
    let ciphertext = buf.clone();

    // Exact 16-byte tag verifies.
    let mut decrypted = ciphertext.clone();
    cipher.decrypt_in_place(&mut decrypted, &nonce, aad, &tag).unwrap();
    assert_eq!(decrypted, pt);

    // Every single-bit flip of the tag fails and leaves the buffer alone.
    for bit in 0..128 {
        let mut bad = tag;
        bad[bit / 8] ^= 1 << (bit % 8);
        let mut untouched = ciphertext.clone();
        assert_eq!(
            cipher.decrypt_in_place(&mut untouched, &nonce, aad, &bad).unwrap_err(),
            AeadError::InvalidCiphertext,
            "{label}: tag bit {bit} accepted"
        );
        assert_failure_buffer_is_consistent(&untouched, &ciphertext, pt, &format!("{label}: tag bit {bit}"));
    }

    // Truncated or over-long tags are rejected; there is no implicit
    // truncation to the first 16 bytes.
    for len in (0..16).chain(17..=48) {
        let bad = vec![0u8; len];
        let mut untouched = ciphertext.clone();
        assert_eq!(
            cipher.decrypt_in_place(&mut untouched, &nonce, aad, &bad).unwrap_err(),
            AeadError::InvalidCiphertext,
            "{label}: tag length {len} accepted"
        );
        assert_failure_buffer_is_consistent(&untouched, &ciphertext, pt, &format!("{label}: tag length {len}"));
    }

    // A 17-byte tag formed by appending a zero byte to the real tag is
    // still rejected.
    let mut long = tag.to_vec();
    long.push(0);
    let mut untouched = ciphertext.clone();
    assert_eq!(
        cipher.decrypt_in_place(&mut untouched, &nonce, aad, &long).unwrap_err(),
        AeadError::InvalidCiphertext
    );
    assert_failure_buffer_is_consistent(&untouched, &ciphertext, pt, &format!("{label}: 17-byte tag"));

    // An all-zero tag of the correct size is rejected for this message.
    let mut untouched = ciphertext.clone();
    assert_eq!(
        cipher
            .decrypt_in_place(&mut untouched, &nonce, aad, &[0u8; 16])
            .unwrap_err(),
        AeadError::InvalidCiphertext
    );
    assert_failure_buffer_is_consistent(&untouched, &ciphertext, pt, &format!("{label}: zero tag"));
}

#[test]
fn tag_handling_exact_tampered_and_truncated() {
    check_tag_handling(&Aes128Gcm::new(&[0x11; 16]), [0x22; 12], &[0x33; 45], "aes128");
    check_tag_handling(&Aes256Gcm::new(&[0x44; 32]), [0x55; 12], &[0x66; 45], "aes256");
    // Also exercise an empty ciphertext: the tag is still mandatory.
    check_tag_handling(&Aes128Gcm::new(&[0x11; 16]), [0x22; 12], &[], "aes128-empty");
}

#[test]
fn aad_bit_length_arithmetic_boundary() {
    // NIST/RustCrypto A_MAX = 2^61 - 1 bytes; its bit length fits in u64,
    // while one more byte would overflow the length block. The runtime case is
    // unreachable (2 EiB), so this pins the arithmetic contract.
    const A_MAX: u64 = (1u64 << 61) - 1;
    assert_eq!(A_MAX.checked_mul(8), Some(u64::MAX - 7));
    assert_eq!((A_MAX + 1).checked_mul(8), None);
}

#[test]
fn decrypt_failure_buffer_semantics() {
    let mut rng = StdRng::seed_from_u64(0xc3_0004);
    for len in [0usize, 1, 15, 16, 17, 64, 65, 128, 129, 300] {
        let pt = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        let cipher = Aes256Gcm::new(&[0x77; 32]);
        let nonce = [0x88; 12];
        let aad = {
            let mut v = vec![0u8; 23];
            rng.fill(&mut v);
            v
        };

        let mut buf = guarded(&pt);
        let tag = cipher
            .encrypt_in_place(&mut buf[GUARD..GUARD + len], &nonce, &aad)
            .unwrap();
        let ciphertext = buf[GUARD..GUARD + len].to_vec();
        let mut bad: [u8; 16] = tag.as_ref().try_into().unwrap();
        bad[15] ^= 0x01;

        assert_eq!(
            cipher
                .decrypt_in_place(&mut buf[GUARD..GUARD + len], &nonce, &aad, &bad)
                .unwrap_err(),
            AeadError::InvalidCiphertext
        );
        assert_guards(&buf, len);
        // The Aead contract leaves the contents unspecified on failure. Every
        // backend must at least return an error without writing out of bounds,
        // and must not leave a partial plaintext/ciphertext mix behind.
        assert_failure_buffer_is_consistent(&buf[GUARD..GUARD + len], &ciphertext, &pt, &format!("len={len}"));
    }
}

/// On backends that verify the tag before writing plaintext, a failed
/// decryption leaves the caller's buffer untouched.
///
/// The aarch64 fused kernel intentionally writes plaintext before verifying the
/// tag, so this stronger-than-contract property is only asserted on other
/// targets. `decrypt_failure_buffer_semantics` covers the all-backend contract.
#[cfg(not(target_arch = "aarch64"))]
#[test]
fn decrypt_failure_leaves_ciphertext_unchanged() {
    let cipher = Aes128Gcm::new(&[0x99; 16]);
    let nonce = [0x12; 12];
    let pt = vec![0x34u8; 77];
    let aad = b"aad";

    let mut buf = pt.clone();
    let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad).unwrap();
    let mut bad: [u8; 16] = tag.as_ref().try_into().unwrap();
    bad[0] ^= 0x80;
    let ciphertext = buf.clone();

    assert_eq!(
        cipher.decrypt_in_place(&mut buf, &nonce, aad, &bad).unwrap_err(),
        AeadError::InvalidCiphertext
    );
    assert_eq!(buf, ciphertext, "plaintext was written before tag verification");
}

#[test]
fn nonce_length_contract() {
    let cipher = Aes128Gcm::new(&[0x01; 16]);
    let valid_nonce = [0x02u8; 12];

    // Sanity: the documented nonce length works.
    let mut buf = vec![0u8; 8];
    let _ = cipher.encrypt_in_place(&mut buf, &valid_nonce, &[]).unwrap();

    // `encrypt_in_place` returns InvalidNonce for a wrong-length nonce, before
    // touching the output buffer (no partial write can escape).
    for len in [0usize, 1, 11, 13, 16, 32] {
        let nonce = vec![0u8; len];
        let mut victim = b"do not touch".to_vec();
        let before = victim.clone();
        assert_eq!(
            cipher.encrypt_in_place(&mut victim, &nonce, &[]).err(),
            Some(AeadError::InvalidNonce),
            "encrypt accepted a {len}-byte nonce"
        );
        assert_eq!(victim, before, "output buffer written on a nonce error");
    }

    // `decrypt_in_place` returns InvalidNonce instead of panicking and does
    // not modify the buffer.
    for len in [0usize, 1, 11, 13, 16, 32] {
        let nonce = vec![0u8; len];
        let mut victim = b"do not touch".to_vec();
        let before = victim.clone();
        assert_eq!(
            cipher
                .decrypt_in_place(&mut victim, &nonce, &[], &[0u8; 16])
                .unwrap_err(),
            AeadError::InvalidNonce,
            "wrong nonce length {len}"
        );
        assert_eq!(victim, before);
    }
}

#[test]
fn aead_default_helpers_and_short_ciphertext() {
    let cipher = Aes256Gcm::new(&[0xab; 32]);
    let nonce = [0xcd; 12];
    let aad = b"header";

    for len in [0usize, 1, 15, 16, 17, 127, 128, 129, 1000] {
        let pt = vec![0x5au8; len];
        let sealed = cipher.encrypt(&pt, &nonce, aad).unwrap();
        assert_eq!(sealed.len(), len + 16, "combined ciphertext length");
        let opened = cipher.decrypt(&sealed, &nonce, aad).expect("decrypt");
        assert_eq!(opened, pt);
    }

    // A combined input shorter than the 16-byte tag is rejected before any
    // decryption, and never accepted as an empty plaintext.
    for len in 0..16 {
        let short = vec![0u8; len];
        assert_eq!(
            cipher.decrypt(&short, &nonce, aad).unwrap_err(),
            AeadError::InvalidCiphertext,
            "combined ciphertext of length {len} accepted"
        );
    }

    // Exactly a tag, with empty plaintext, is valid.
    let mut empty: [u8; 0] = [];
    let tag = cipher.encrypt_in_place(&mut empty, &nonce, aad).unwrap();
    assert_eq!(cipher.decrypt(tag.as_ref(), &nonce, aad).unwrap(), Vec::<u8>::new());

    // The tag appended to a ciphertext is not accepted when the plaintext part
    // is truncated (the length block does not match).
    let mut sealed = cipher.encrypt(b"0123456789", &nonce, aad).unwrap();
    sealed.drain(..1);
    assert_eq!(cipher.decrypt(&sealed, &nonce, aad).unwrap_err(), AeadError::InvalidCiphertext);
}

#[test]
fn aad_is_read_only_and_cipher_reuse_is_deterministic() {
    let cipher = Aes128Gcm::new(&[0x5a; 16]);
    let nonce = [0xa5; 12];
    let aad = b"aad that must not be copied or modified";
    let pt = b"deterministic plaintext";

    let mut first = pt.to_vec();
    let tag1 = cipher.encrypt_in_place(&mut first, &nonce, aad).unwrap();
    assert_eq!(aad, b"aad that must not be copied or modified");

    // Re-using the same instance with the same nonce is deterministic (the
    // cipher keeps no per-call state).
    let mut second = pt.to_vec();
    let tag2 = cipher.encrypt_in_place(&mut second, &nonce, aad).unwrap();
    assert_eq!(first, second);
    assert_eq!(tag1.as_ref(), tag2.as_ref());

    // AAD in a guarded, offset allocation is neither read out of bounds nor
    // written.
    let aad_buf = guarded(aad);
    let mut third = pt.to_vec();
    let _ = cipher
        .encrypt_in_place(&mut third, &nonce, &aad_buf[GUARD..GUARD + aad.len()])
        .unwrap();
    assert_guards(&aad_buf, aad.len());
    assert_eq!(&aad_buf[GUARD..GUARD + aad.len()], aad);
}

#[test]
fn aead_constants_match_the_api() {
    assert_eq!(<Aes128Gcm as Aead>::KEY_SIZE, 16);
    assert_eq!(<Aes256Gcm as Aead>::KEY_SIZE, 32);
    assert_eq!(<Aes128Gcm as Aead>::TAG_SIZE, 16);
    assert_eq!(<Aes256Gcm as Aead>::TAG_SIZE, 16);
    assert_eq!(<Aes128Gcm as Aead>::NONCE_SIZE, 12);
    assert_eq!(<Aes256Gcm as Aead>::NONCE_SIZE, 12);
    assert_eq!(Aes128Gcm::KEY_SIZE, 16);
    assert_eq!(Aes256Gcm::KEY_SIZE, 32);
}
