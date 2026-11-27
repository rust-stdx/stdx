//! Edge and boundary tests added by the unsafe audit of `crypto::aes`.
//!
//! The pre-existing `audit_aes_*` files already cover the NIST vectors,
//! software/hardware parity over 0..=256 bytes, canaries at non-zero offsets,
//! tag tampering and the published length limits. This file adds the cases the
//! audit found missing:
//!
//! * the **CTR counter state machine near 2^128**: every split of a message,
//!   with an independent model of when `CounterExhausted` must be returned and
//!   that a failed call must not write to the buffer;
//! * **all-zero / all-0xff keys and nonces**, and a deterministic
//!   key/nonce/AAD/plaintext fuzz-style differential against an independent
//!   GCM oracle;
//! * **every GHASH block count 0..=20** (AAD and ciphertext, aligned and
//!   partial) through the public API;
//! * a **1 MiB AAD** so the GHASH tail path runs at scale;
//! * **guard-page** tests: payloads that start or end exactly on a `PROT_NONE`
//!   boundary, so any 16/32/64-byte load or store outside `in_out` faults
//!   instead of merely corrupting a canary;
//! * a hardening assertion that a rejected tag never leaves the decrypted
//!   plaintext in the caller's buffer (see RUSTSEC-2023-0096). The ARMv8 fused
//!   decrypt restores the ciphertext before returning an error, so it passes on
//!   every backend.
//!
//! The GCM oracle here is independent of the crate's GHASH: it uses the
//! textbook bit-serial `GF(2^128)` multiplication from NIST SP 800-38D
//! Appendix B, while AES comes from the crate's public `Aes*Block` API (which
//! the block-cipher audit verifies independently).

use crypto::{
    Aead, AeadError, StreamCipher, StreamCipherError,
    aes::{Aes128Block, Aes128Ctr, Aes128Gcm, Aes256Block, Aes256Ctr, Aes256Gcm},
};
use random::{Rng, RngExt, SeedableRng, rngs::ChaChaRng};

const KEY128: [u8; 16] = [
    0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
];
const KEY256: [u8; 32] = [
    0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81, 0x1f, 0x35, 0x2c,
    0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
];

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

// ── Independent GCM oracle ───────────────────────────────────────────────────
//
// Bit-serial GF(2^128) multiplication in GCM (big-endian) bit order, exactly
// the algorithm in NIST SP 800-38D Appendix B. Deliberately branchy and
// table-free so it shares nothing with the constant-time kernel.

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

/// GCM `inc32`: increment only the last four bytes of the counter block.
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

/// Independent AES-GCM encryption: returns `(ciphertext, tag)`.
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
    ghash_update_ref(&mut state, &h, aad);
    ghash_update_ref(&mut state, &h, &ciphertext);

    let mut length_block = [0u8; 16];
    length_block[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    length_block[8..].copy_from_slice(&((ciphertext.len() as u64) * 8).to_be_bytes());
    ghash_update_ref(&mut state, &h, &length_block);

    for i in 0..16 {
        state[i] ^= ej0[i];
    }

    (ciphertext, state)
}

/// Encrypt through the public API and compare ciphertext and tag against the
/// oracle, then decrypt both through the public API.
fn check_gcm<B: BlockCipher, C: Aead>(block: &B, cipher: &C, nonce: &[u8; 12], aad: &[u8], pt: &[u8], label: &str) {
    let (want_ct, want_tag) = gcm_ref(block, nonce, aad, pt);

    let mut got = pt.to_vec();
    let tag = cipher.encrypt_in_place(&mut got, nonce, aad).unwrap();
    assert_eq!(got, want_ct, "{label}: ciphertext");
    assert_eq!(tag.as_ref(), &want_tag[..], "{label}: tag");

    let mut back = want_ct.clone();
    cipher
        .decrypt_in_place(&mut back, nonce, aad, &want_tag)
        .unwrap_or_else(|e| panic!("{label}: decrypt: {e}"));
    assert_eq!(back, pt, "{label}: roundtrip");
}

// ── CTR counter state machine near 2^128 ─────────────────────────────────────

/// Common driver for the two CTR instantiations.
trait Ctr {
    fn set_counter(&mut self, counter: &[u8; 16]);
    fn xor(&mut self, data: &mut [u8]) -> Result<(), StreamCipherError>;
}

impl Ctr for Aes128Ctr {
    fn set_counter(&mut self, counter: &[u8; 16]) {
        Aes128Ctr::set_counter(self, counter);
    }
    fn xor(&mut self, data: &mut [u8]) -> Result<(), StreamCipherError> {
        self.xor_keystream(data)
    }
}

impl Ctr for Aes256Ctr {
    fn set_counter(&mut self, counter: &[u8; 16]) {
        Aes256Ctr::set_counter(self, counter);
    }
    fn xor(&mut self, data: &mut [u8]) -> Result<(), StreamCipherError> {
        self.xor_keystream(data)
    }
}

/// Model of the documented CTR contract for one call sequence.
///
/// After `cum` bytes have been consumed, `ceil(cum / 16)` keystream blocks have
/// been generated, so the next block counter is `base + ceil(cum / 16)`. A call
/// is accepted iff that value does not overflow `u128`; the implementation
/// deliberately reserves the final counter value. On the first rejected call
/// nothing is written.
fn assert_ctr_calls<C: Ctr>(base: u128, cuts: &[usize], mut cipher: C, block: &impl BlockCipher) {
    let total: usize = cuts.iter().sum();
    let pt = pattern(total, 37, 11);
    let mut buf = pt.clone();
    cipher.set_counter(&base.to_be_bytes());

    let mut model = pt.clone();
    let mut cum = 0usize;
    let mut off = 0usize;
    let mut failed = false;
    for (idx, &n) in cuts.iter().enumerate() {
        cum += n;
        let allowed = base.checked_add(cum.div_ceil(16) as u128).is_some();

        let mut chunk = buf[off..off + n].to_vec();
        let before = chunk.clone();
        let result = cipher.xor(&mut chunk);

        if allowed {
            assert_eq!(result, Ok(()), "call {idx} should succeed (cuts={cuts:?})");
            for (j, byte) in model[off..off + n].iter_mut().enumerate() {
                let pos = off + j;
                let ctr = base.wrapping_add((pos / 16) as u128).to_be_bytes();
                *byte ^= block.encrypt(&ctr)[pos % 16];
            }
            buf[off..off + n].copy_from_slice(&chunk);
        } else {
            assert_eq!(
                result,
                Err(StreamCipherError::CounterExhausted),
                "call {idx} should report CounterExhausted (cuts={cuts:?})"
            );
            assert_eq!(chunk, before, "rejected call {idx} modified the buffer");
            failed = true;
            break;
        }
        off += n;
    }

    if failed {
        assert_eq!(&buf[..off], &model[..off], "successful prefix differs (cuts={cuts:?})");
    } else {
        assert_eq!(buf, model, "ciphertext differs (cuts={cuts:?})");
    }
}

/// Exhaustive near-exhaustion sweep: every 1- and 2-way split of short messages
/// for counters a few blocks below `2^128 - 1`.
#[test]
fn ctr_counter_exhaustion_state_machine() {
    let block128 = Aes128Block::new(&KEY128);
    let block256 = Aes256Block::new(&KEY256);

    let mut starts = vec![0u128, 1, 2, 0xff, 0xffff_ffff];
    // The last 26 counter values: the reserve-one-block rule makes the exact
    // accepted length change at each of them.
    for k in 0..=25u128 {
        starts.push(u128::MAX - k);
    }

    for &base in &starts {
        for len in 0..=33usize {
            assert_ctr_calls(base, &[len], Aes128Ctr::new(&KEY128), &block128);
            assert_ctr_calls(base, &[len], Aes256Ctr::new(&KEY256), &block256);
        }
        for len in 2..=40usize {
            for first in 0..=len {
                assert_ctr_calls(base, &[first, len - first], Aes128Ctr::new(&KEY128), &block128);
                assert_ctr_calls(base, &[first, len - first], Aes256Ctr::new(&KEY256), &block256);
            }
        }
        // A three-way split, including empty calls, around block boundaries.
        for &cuts in &[
            &[0usize, 16, 0][..],
            &[0usize, 1, 15][..],
            &[5usize, 11, 1][..],
            &[16usize, 0, 17][..],
            &[7usize, 9, 16, 1][..],
        ] {
            assert_ctr_calls(base, cuts, Aes128Ctr::new(&KEY128), &block128);
            assert_ctr_calls(base, cuts, Aes256Ctr::new(&KEY256), &block256);
        }
    }
}

/// A rejected call must leave the object usable from the same counter, and
/// `set_counter` must recover it after `CounterExhausted`.
#[test]
fn ctr_counter_exhaustion_is_not_sticky() {
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&[0xff; 16]);
    let mut rejected = [0xa5u8; 21];
    assert_eq!(c.xor_keystream(&mut rejected), Err(StreamCipherError::CounterExhausted));
    assert_eq!(rejected, [0xa5u8; 21]);

    // The same (rejected) counter can still be replaced by a usable one.
    c.set_counter(&[0u8; 16]);
    let mut ok = [0u8; 16];
    c.xor_keystream(&mut ok).unwrap();
    assert_eq!(
        ok,
        Aes128Block::new(&KEY128).encrypt_block(&[0u8; 16]),
        "counter after a rejected call is corrupted"
    );

    // One block below the reserved value still works, then the next call fails.
    let mut near = [0xffu8; 16];
    near[15] = 0xfe;
    let mut c = Aes256Ctr::new(&KEY256);
    c.set_counter(&near);
    let mut one = [0u8; 16];
    c.xor_keystream(&mut one).unwrap();
    let mut two = [0xa5u8; 16];
    assert_eq!(c.xor_keystream(&mut two), Err(StreamCipherError::CounterExhausted));
    assert_eq!(two, [0xa5u8; 16]);
}

// ── GCM key / nonce edge values and randomized differential ──────────────────

#[test]
fn gcm_all_zero_and_all_ff_key_and_nonce_matrix() {
    let lengths = [
        0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257,
    ];
    let keys128: [[u8; 16]; 3] = [[0u8; 16], [0xffu8; 16], [0x80u8; 16]];
    let keys256: [[u8; 32]; 3] = [[0u8; 32], [0xffu8; 32], [0x80u8; 32]];
    let nonces: [[u8; 12]; 3] = [[0u8; 12], [0xffu8; 12], [0x80u8; 12]];

    for key in keys128 {
        let cipher = Aes128Gcm::new(&key);
        let block = Aes128Block::new(&key);
        for nonce in nonces {
            for &len in &lengths {
                let pt = pattern(len, 91, 3);
                // An AAD length with a different partial-block shape.
                let aad = pattern((len * 7 + 13) % 67, 53, 7);
                check_gcm(&block, &cipher, &nonce, &aad, &pt, "aes128 edge matrix");
            }
        }
    }

    for key in keys256 {
        let cipher = Aes256Gcm::new(&key);
        let block = Aes256Block::new(&key);
        for nonce in nonces {
            for &len in &lengths {
                let pt = pattern(len, 29, 5);
                let aad = pattern((len * 5 + 17) % 71, 43, 9);
                check_gcm(&block, &cipher, &nonce, &aad, &pt, "aes256 edge matrix");
            }
        }
    }
}

#[test]
fn gcm_deterministic_random_differential() {
    let mut rng = ChaChaRng::seed_from_u64(0xa5a5_5a5a_1234_9876);

    for i in 0..400 {
        let pt_len = (rng.next_u64() as usize) % 301;
        let aad_len = (rng.next_u64() as usize) % 41;
        // Bias towards block boundaries but keep the boundaries themselves.
        let pt_len = match i % 4 {
            0 => pt_len,
            1 => (pt_len / 16) * 16,
            2 => (pt_len / 16) * 16 + 1,
            _ => pt_len.saturating_sub(1),
        };
        let pt = {
            let mut v = vec![0u8; pt_len];
            rng.fill(&mut v);
            v
        };
        let aad = {
            let mut v = vec![0u8; aad_len];
            rng.fill(&mut v);
            v
        };
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce);

        if i % 2 == 0 {
            let mut key = [0u8; 16];
            rng.fill(&mut key);
            check_gcm(
                &Aes128Block::new(&key),
                &Aes128Gcm::new(&key),
                &nonce,
                &aad,
                &pt,
                &format!("random aes128 case {i}"),
            );
        } else {
            let mut key = [0u8; 32];
            rng.fill(&mut key);
            check_gcm(
                &Aes256Block::new(&key),
                &Aes256Gcm::new(&key),
                &nonce,
                &aad,
                &pt,
                &format!("random aes256 case {i}"),
            );
        }
    }
}

/// GHASH block counts 0..=20 through the public API: aligned and partial AAD
/// and ciphertext, independently for each stream (their tails are padded
/// separately). `H` itself is AES_K(0) and can never be chosen by the caller,
/// so an all-zero H case is not reachable through the public API.
#[test]
fn gcm_ghash_block_counts_0_to_20() {
    let key = [0x3cu8; 32];
    let cipher = Aes256Gcm::new(&key);
    let block = Aes256Block::new(&key);
    let nonce = [0x5au8; 12];

    for blocks in 0..=20usize {
        let aligned = blocks * 16;
        let cases: [(usize, usize); 5] = [
            (aligned, 0),
            (0, aligned),
            (aligned, aligned),
            (aligned + 1, aligned.saturating_sub(1)),
            (aligned.saturating_sub(1), aligned + 1),
        ];
        for (pt_len, aad_len) in cases {
            let pt = pattern(pt_len, 37, 11);
            let aad = pattern(aad_len, 53, 7);
            check_gcm(
                &block,
                &cipher,
                &nonce,
                &aad,
                &pt,
                &format!("ghash blocks={blocks} pt={pt_len} aad={aad_len}"),
            );
        }
    }
}

/// A 1 MiB + 3 AAD exercises the GHASH multi-block loop and final partial
/// padding far beyond the short-length sweeps.
#[test]
fn gcm_large_aad_matches_reference() {
    let key = [0x11u8; 16];
    let cipher = Aes128Gcm::new(&key);
    let block = Aes128Block::new(&key);
    let nonce = [0xa5u8; 12];
    let aad = pattern(1024 * 1024 + 3, 101, 17);

    for pt_len in [0usize, 1, 16, 17, 129] {
        let pt = pattern(pt_len, 43, 9);
        check_gcm(&block, &cipher, &nonce, &aad, &pt, "large aad");
    }
}

// ── Decrypt-failure leakage (RUSTSEC-2023-0096 class) ────────────────────────

/// A rejected tag must not leave unauthenticated plaintext in the caller's
/// buffer. The `Aead` contract allows *unspecified* contents, but exposing the
/// plaintext enables chosen-ciphertext attacks for callers that inspect the
/// buffer after an error (RUSTSEC-2023-0096 / CVE-2023-42811).
///
/// The software and x86 paths verify-then-decrypt and pass this. The ARMv8
/// fused path decrypts while authenticating, but re-applies the CTR keystream
/// before returning the error, so it passes as well.
#[test]
fn gcm_decrypt_failure_does_not_expose_plaintext() {
    let key = [0x33u8; 32];
    let nonce = [0x44u8; 12];
    let aad = pattern(21, 11, 2);

    for len in [0usize, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 255, 256, 257] {
        let pt = pattern(len, 23, 5);
        let cipher = Aes256Gcm::new(&key);
        let mut buf = pt.clone();
        let tag: [u8; 16] = cipher
            .encrypt_in_place(&mut buf, &nonce, &aad)
            .unwrap()
            .as_ref()
            .try_into()
            .unwrap();
        let ciphertext = buf.clone();

        // A single bit flip in the tag, and (for non-empty input) one in the
        // ciphertext, both after the tag is known.
        let mut bad_tag = tag;
        bad_tag[len % 16] ^= 1 << (len % 8);
        let mut tampered = ciphertext.clone();
        assert_eq!(
            cipher.decrypt_in_place(&mut tampered, &nonce, &aad, &bad_tag),
            Err(AeadError::InvalidCiphertext)
        );
        assert!(
            tampered == ciphertext,
            "len={len}: rejected tag left {} in the buffer (RUSTSEC-2023-0096 class)",
            if tampered == pt {
                "the full plaintext"
            } else {
                "modified data"
            }
        );

        if len != 0 {
            let mut tampered_ct = ciphertext.clone();
            tampered_ct[len / 2] ^= 0x40;
            let before = tampered_ct.clone();
            assert_eq!(
                cipher.decrypt_in_place(&mut tampered_ct, &nonce, &aad, &tag),
                Err(AeadError::InvalidCiphertext)
            );
            assert!(
                tampered_ct == before,
                "len={len}: rejected ciphertext left {} in the buffer",
                if tampered_ct == pt {
                    "a plaintext"
                } else {
                    "modified data"
                }
            );
        }
    }
}

// ── Guard-page pointer tests ─────────────────────────────────────────────────

#[cfg(target_os = "linux")]
mod guard {
    use std::{ffi::c_void, ptr};

    const PROT_NONE: i32 = 0;
    const PROT_READ: i32 = 1;
    const PROT_WRITE: i32 = 2;
    const MAP_PRIVATE: i32 = 2;
    const MAP_ANONYMOUS: i32 = 0x20;
    const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;
    const SC_PAGESIZE: i32 = 30;

    unsafe extern "C" {
        fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut c_void;
        fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
        fn munmap(addr: *mut c_void, len: usize) -> i32;
        fn sysconf(name: i32) -> i64;
    }

    /// Three pages: `[PROT_NONE][read/write][PROT_NONE]`.
    ///
    /// A slice placed at the start of the middle page has a guard immediately
    /// before it; a slice ending at the end of the middle page has one
    /// immediately after it. Either over-read or over-write by one byte faults.
    pub struct Guarded {
        base: *mut u8,
        len: usize,
        page: usize,
    }

    impl Guarded {
        pub fn new() -> Guarded {
            let page = {
                let value = unsafe { sysconf(SC_PAGESIZE) };
                assert!(value > 0, "sysconf(_SC_PAGESIZE) failed");
                value as usize
            };
            let len = 3 * page;
            let base =
                unsafe { mmap(ptr::null_mut(), len, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0) };
            assert_ne!(base, MAP_FAILED, "mmap failed");
            let base = base.cast::<u8>();
            unsafe {
                assert_eq!(mprotect(base.cast(), page, PROT_NONE), 0, "mprotect prefix failed");
                assert_eq!(
                    mprotect(base.add(2 * page).cast(), page, PROT_NONE),
                    0,
                    "mprotect suffix failed"
                );
            }
            Guarded {
                base,
                len,
                page,
            }
        }

        pub fn page(&self) -> usize {
            self.page
        }

        /// Start of the writable page.
        pub fn data(&self) -> *mut u8 {
            unsafe { self.base.add(self.page) }
        }

        /// Safe slice over `[offset, offset + len)` of the writable page.
        ///
        /// # Safety
        ///
        /// The caller must keep `offset + len <= page()` and must not hold a
        /// second live reference into the same region.
        pub unsafe fn slice(&mut self, offset: usize, len: usize) -> &mut [u8] {
            assert!(offset + len <= self.page);
            unsafe { std::slice::from_raw_parts_mut(self.data().add(offset), len) }
        }
    }

    impl Drop for Guarded {
        fn drop(&mut self) {
            unsafe {
                munmap(self.base.cast(), self.len);
            }
        }
    }
}

/// Run `f` with a buffer of `len` bytes that ends exactly at a guard page, and
/// again with one that starts immediately after a guard page.
#[cfg(target_os = "linux")]
fn with_guard_edges(mut f: impl FnMut(&mut [u8], &str)) {
    use guard::Guarded;

    let lengths = [
        0usize, 1, 2, 15, 16, 17, 31, 32, 33, 47, 48, 49, 63, 64, 65, 79, 80, 81, 127, 128, 129, 191, 192, 193, 255,
        256, 257, 383, 384, 385, 511, 512, 513, 1023, 1024, 1025, 2047, 2048, 2049,
    ];

    let mut region = Guarded::new();
    let page = region.page();
    for &len in &lengths {
        assert!(len <= page);
        let end = page - len;
        unsafe {
            region.slice(end, len).fill(0xa5);
        }
        f(unsafe { region.slice(end, len) }, "ends-at-guard");
        f(unsafe { region.slice(0, len) }, "starts-after-guard");
    }
}

#[cfg(target_os = "linux")]
#[cfg_attr(miri, ignore = "mmap/mprotect are not available under Miri")]
#[test]
fn ctr_guard_page_edges() {
    use guard::Guarded;

    with_guard_edges(|buf, where_| {
        let len = buf.len();
        let mut c = Aes128Ctr::new(&KEY128);
        c.set_counter(&[0x42u8; 16]);
        c.xor_keystream(buf)
            .unwrap_or_else(|e| panic!("aes128 len={len} {where_}: {e}"));

        let mut c = Aes256Ctr::new(&KEY256);
        c.set_counter(&[0x24u8; 16]);
        c.xor_keystream(buf)
            .unwrap_or_else(|e| panic!("aes256 len={len} {where_}: {e}"));
    });

    // AAD and plaintext each end exactly at their own guard page.
    let mut aad_region = Guarded::new();
    let page = aad_region.page();
    with_guard_edges(|buf, where_| {
        let len = buf.len();
        let aad_len = 37usize.min(page);
        let aad = unsafe { aad_region.slice(page - aad_len, aad_len) };
        aad.fill(0x5a);
        let cipher = Aes128Gcm::new(&KEY128);
        let tag = cipher
            .encrypt_in_place(buf, &[0x99u8; 12], aad)
            .unwrap_or_else(|e| panic!("aes128-gcm len={len} {where_}: {e}"));
        let mut back = buf.to_vec();
        cipher
            .decrypt_in_place(&mut back, &[0x99u8; 12], aad, tag.as_ref())
            .unwrap_or_else(|e| panic!("aes128-gcm decrypt len={len} {where_}: {e}"));
    });
}

#[cfg(target_os = "linux")]
#[cfg_attr(miri, ignore = "mmap/mprotect are not available under Miri")]
#[test]
fn gcm_guard_page_edges() {
    use guard::Guarded;

    let mut region = Guarded::new();
    let page = region.page();
    let nonce = [0x77u8; 12];
    let aad = pattern(29, 13, 1);

    for &len in &[
        0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 191, 192, 193, 255, 256, 257, 383, 384, 385, 511,
        512, 513, 1023, 1024, 1025, 2047, 2048, 2049,
    ] {
        for &offset in &[page - len, 0usize] {
            let buf = unsafe { region.slice(offset, len) };
            buf.fill(0xa5);

            let cipher = Aes256Gcm::new(&KEY256);
            let tag = cipher
                .encrypt_in_place(buf, &nonce, &aad)
                .unwrap_or_else(|e| panic!("encrypt len={len} offset={offset}: {e}"));
            let ct = buf.to_vec();
            cipher
                .decrypt_in_place(buf, &nonce, &aad, tag.as_ref())
                .unwrap_or_else(|e| panic!("decrypt len={len} offset={offset}: {e}"));
            if len != 0 {
                assert_ne!(buf, &ct[..], "decrypt did not change the buffer");
            }
        }
    }

    // AAD at a guard edge with an empty / partial plaintext at another edge.
    let mut aad_region = Guarded::new();
    for &aad_len in &[1usize, 15, 16, 17, 63, 64, 65, 255, 256, 257, 1024] {
        let aad = unsafe { aad_region.slice(page - aad_len, aad_len) };
        aad.fill(0x3c);
        for &pt_len in &[0usize, 1, 16, 17, 129] {
            let buf = unsafe { region.slice(page - pt_len, pt_len) };
            buf.fill(0xa5);
            let cipher = Aes128Gcm::new(&KEY128);
            let tag = cipher
                .encrypt_in_place(buf, &nonce, aad)
                .unwrap_or_else(|e| panic!("aad={aad_len} pt={pt_len}: {e}"));
            cipher
                .decrypt_in_place(buf, &nonce, aad, tag.as_ref())
                .unwrap_or_else(|e| panic!("aad={aad_len} pt={pt_len} decrypt: {e}"));
        }
    }
}
