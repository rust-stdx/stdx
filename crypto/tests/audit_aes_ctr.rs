//! Tests for `crypto::aes::{Aes128Ctr, Aes256Ctr}`.
//!
//! The reference implementation below is deliberately independent of the CTR
//! code under test: it drives the plain block cipher (`Aes128Block` /
//! `Aes256Block`, which always use the bitsliced software path) and advances a
//! 128-bit big-endian counter with `u128` arithmetic. That exercises the
//! hardware dispatch against a scalar model.
//!
//! Coverage:
//! - NIST SP 800-38A F.5.1 (CTR-AES128) and F.5.5 (CTR-AES256) vectors.
//! - Differential sweep for every length 0..=512 (and larger boundaries) over
//!   counters that stress the 32-bit, 64-bit and 128-bit carry positions.
//! - Non-zero input slice offsets and canary bytes around the output slice.
//! - In-place encryption/decryption, reset with `set_counter`, and repeated
//!   calls.
//! - Block-aligned chunking equivalence and arbitrary (including partial-block)
//!   chunking equivalence, which must match a one-shot call.
//! - Observable counter state after a call (probed through the next call).

use crypto::{
    StreamCipher, StreamCipherError,
    aes::{Aes128Block, Aes128Ctr, Aes256Block, Aes256Ctr},
};
use random::{Rng, SeedableRng, rngs::ChaChaRng};

const KEY128: [u8; 16] = [
    0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
];
const KEY256: [u8; 32] = [
    0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81, 0x1f, 0x35, 0x2c,
    0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
];

/// Deterministic non-secret test pattern.
fn pattern(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i as u8).wrapping_mul(37).wrapping_add(11)).collect()
}

/// Counters that put the carry at every interesting byte position: the low
/// 32-bit word, the low 64-bit half, and the full 128-bit wrap.
fn counters() -> Vec<[u8; 16]> {
    [
        0u128,
        1,
        0xff,
        0xffff_ffff,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff_0000_0000_0000_0000,
        0x5a5a_5a5a_5a5a_5a5a_5a5a_5a5a_5a5a_5a5a,
    ]
    .into_iter()
    .map(u128::to_be_bytes)
    .collect()
}

/// Independent AES-128-CTR model: block cipher + `u128` big-endian counter.
fn reference_128(key: &[u8; 16], counter: [u8; 16], data: &mut [u8]) {
    let block = Aes128Block::new(key);
    let base = u128::from_be_bytes(counter);
    for (i, chunk) in data.chunks_mut(16).enumerate() {
        let ctr = base.wrapping_add(i as u128).to_be_bytes();
        let ks = block.encrypt_block(&ctr);
        for (j, b) in chunk.iter_mut().enumerate() {
            *b ^= ks[j];
        }
    }
}

/// Independent AES-256-CTR model.
fn reference_256(key: &[u8; 32], counter: [u8; 16], data: &mut [u8]) {
    let block = Aes256Block::new(key);
    let base = u128::from_be_bytes(counter);
    for (i, chunk) in data.chunks_mut(16).enumerate() {
        let ctr = base.wrapping_add(i as u128).to_be_bytes();
        let ks = block.encrypt_block(&ctr);
        for (j, b) in chunk.iter_mut().enumerate() {
            *b ^= ks[j];
        }
    }
}

fn keystream_128(key: &[u8; 16], counter: u128, len: usize) -> Vec<u8> {
    let block = Aes128Block::new(key);
    let mut out = Vec::with_capacity(len);
    let mut i = 0u128;
    while out.len() < len {
        out.extend_from_slice(&block.encrypt_block(&(counter.wrapping_add(i)).to_be_bytes()));
        i = i.wrapping_add(1);
    }
    out.truncate(len);
    out
}

fn keystream_256(key: &[u8; 32], counter: u128, len: usize) -> Vec<u8> {
    let block = Aes256Block::new(key);
    let mut out = Vec::with_capacity(len);
    let mut i = 0u128;
    while out.len() < len {
        out.extend_from_slice(&block.encrypt_block(&(counter.wrapping_add(i)).to_be_bytes()));
        i = i.wrapping_add(1);
    }
    out.truncate(len);
    out
}

// ── NIST SP 800-38A vectors ────────────────────────────────────────────────────

const NIST_COUNTER: [u8; 16] = [
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];

const NIST_PLAINTEXT: &str = concat!(
    "6bc1bee22e409f96e93d7e117393172a",
    "ae2d8a571e03ac9c9eb76fac45af8e51",
    "30c81c46a35ce411e5fbc1191a0a52ef",
    "f69f2445df4f9b17ad2b417be66c3710",
);

const NIST_CTR128_CIPHERTEXT: &str = concat!(
    "874d6191b620e3261bef6864990db6ce",
    "9806f66b7970fdff8617187bb9fffdff",
    "5ae4df3edbd5d35e5b4f09020db03eab",
    "1e031dda2fbe03d1792170a0f3009cee",
);

// F.5.5 CTR-AES256.Encrypt
const NIST_CTR256_CIPHERTEXT: &str = concat!(
    "601ec313775789a5b7a7f504bbf3d228",
    "f443e3ca4d62b59aca84e990cacaf5c5",
    "2b0930daa23de94ce87017ba2d84988d",
    "dfc9c58db67aada613c2dd08457941a6",
);

#[test]
fn nist_sp800_38a_f51_ctr_aes128() {
    let pt = hex::decode(NIST_PLAINTEXT).unwrap();
    let want = hex::decode(NIST_CTR128_CIPHERTEXT).unwrap();
    assert_eq!(pt.len(), 64);

    let mut buf = pt.clone();
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&NIST_COUNTER);
    c.xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, want, "F.5.1 CTR-AES128.Encrypt");

    // Decryption is the same operation with a freshly reset counter.
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&NIST_COUNTER);
    c.xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, pt, "F.5.1 CTR-AES128.Decrypt");

    // Per-block counters (last byte increments from feff): the keystream block
    // XORed with the plaintext block must give the expected ciphertext block.
    let base = u128::from_be_bytes(NIST_COUNTER);
    for (block, (pt_block, ct_block)) in pt.chunks(16).zip(want.chunks(16)).enumerate() {
        let mut ks = [0u8; 16];
        let mut c = Aes128Ctr::new(&KEY128);
        c.set_counter(&(base.wrapping_add(block as u128)).to_be_bytes());
        c.xor_keystream(&mut ks).unwrap();
        let got: Vec<u8> = ks.iter().zip(pt_block).map(|(k, p)| k ^ p).collect();
        assert_eq!(got, ct_block, "F.5.1 block {block}");
    }

    // Independent model agrees as well.
    let mut model = pt.clone();
    reference_128(&KEY128, NIST_COUNTER, &mut model);
    assert_eq!(model, want);
}

#[test]
fn nist_sp800_38a_f55_ctr_aes256() {
    let pt = hex::decode(NIST_PLAINTEXT).unwrap();
    let want = hex::decode(NIST_CTR256_CIPHERTEXT).unwrap();

    let mut buf = pt.clone();
    let mut c = Aes256Ctr::new(&KEY256);
    c.set_counter(&NIST_COUNTER);
    c.xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, want, "F.5.5 CTR-AES256.Encrypt");

    let mut c = Aes256Ctr::new(&KEY256);
    c.set_counter(&NIST_COUNTER);
    c.xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, pt, "F.5.5 CTR-AES256.Decrypt");

    let mut model = pt.clone();
    reference_256(&KEY256, NIST_COUNTER, &mut model);
    assert_eq!(model, want);
}

// ── Differential sweeps ───────────────────────────────────────────────────────

/// For a block-aligned call (`consumed % 16 == 0`) the stream buffer is empty
/// and the counter must be left at `base + consumed/16`; a following 16-byte
/// call reveals it. Non-aligned calls buffer the rest of the block instead, so
/// the continuation is checked by the chunking tests.
fn probe_counter_128(key: &[u8; 16], c: &mut Aes128Ctr, base: [u8; 16], consumed: usize) {
    let expected = u128::from_be_bytes(base).wrapping_add((consumed / 16) as u128);
    let mut probe = [0u8; 16];
    c.xor_keystream(&mut probe).unwrap();
    assert_eq!(
        probe.to_vec(),
        keystream_128(key, expected, 16),
        "counter state after {consumed} bytes (want counter {expected:#034x})"
    );
}

fn probe_counter_256(key: &[u8; 32], c: &mut Aes256Ctr, base: [u8; 16], consumed: usize) {
    let expected = u128::from_be_bytes(base).wrapping_add((consumed / 16) as u128);
    let mut probe = [0u8; 16];
    c.xor_keystream(&mut probe).unwrap();
    assert_eq!(
        probe.to_vec(),
        keystream_256(key, expected, 16),
        "counter state after {consumed} bytes (want counter {expected:#034x})"
    );
}

#[test]
fn differential_every_length_0_to_512_aes128() {
    for counter in counters() {
        for len in 0..=512usize {
            let pt = pattern(len);
            let mut got = pt.clone();
            let mut c = Aes128Ctr::new(&KEY128);
            c.set_counter(&counter);
            c.xor_keystream(&mut got).unwrap();

            let mut want = pt.clone();
            reference_128(&KEY128, counter, &mut want);
            assert_eq!(got, want, "len={len} counter={counter:?}");

            // After a block-aligned call the buffer is empty and the counter
            // points at the next block, which a 16-byte probe reveals.
            if len % 16 == 0 {
                probe_counter_128(&KEY128, &mut c, counter, len);
            }
        }
    }
}

#[test]
fn differential_every_length_0_to_512_aes256() {
    for counter in counters() {
        for len in 0..=512usize {
            let pt = pattern(len);
            let mut got = pt.clone();
            let mut c = Aes256Ctr::new(&KEY256);
            c.set_counter(&counter);
            c.xor_keystream(&mut got).unwrap();

            let mut want = pt.clone();
            reference_256(&KEY256, counter, &mut want);
            assert_eq!(got, want, "len={len} counter={counter:?}");

            if len % 16 == 0 {
                probe_counter_256(&KEY256, &mut c, counter, len);
            }
        }
    }
}

#[test]
fn differential_simd_and_block_boundaries() {
    // Lengths around 1, 2, 4, 8 and 16 AES blocks, plus off-by-one cases.
    let lengths = [
        0usize, 1, 15, 16, 17, 31, 32, 33, 47, 48, 49, 63, 64, 65, 95, 96, 97, 127, 128, 129, 191, 192, 193, 255, 256,
        257, 383, 384, 385, 511, 512, 513, 1023, 1024, 1025, 2047, 2048, 2049, 4096, 4097,
    ];
    for counter in counters() {
        for &len in &lengths {
            let pt = pattern(len);
            let mut got = pt.clone();
            let mut c = Aes128Ctr::new(&KEY128);
            c.set_counter(&counter);
            c.xor_keystream(&mut got).unwrap();
            let mut want = pt.clone();
            reference_128(&KEY128, counter, &mut want);
            assert_eq!(got, want, "aes128 len={len} counter={counter:?}");

            let mut got = pt.clone();
            let mut c = Aes256Ctr::new(&KEY256);
            c.set_counter(&counter);
            c.xor_keystream(&mut got).unwrap();
            let mut want = pt.clone();
            reference_256(&KEY256, counter, &mut want);
            assert_eq!(got, want, "aes256 len={len} counter={counter:?}");
        }
    }
}

// ── Canaries and non-zero offsets ─────────────────────────────────────────────

#[test]
fn canaries_and_nonzero_slice_offsets() {
    const CANARY: u8 = 0xa5;
    for len in 0..=512usize {
        for offset in [1usize, 15, 16, 17, 63] {
            let pt = pattern(len);
            let mut buf = vec![CANARY; offset + len + 32];
            buf[offset..offset + len].copy_from_slice(&pt);

            let mut c = Aes128Ctr::new(&KEY128);
            c.set_counter(&NIST_COUNTER);
            c.xor_keystream(&mut buf[offset..offset + len]).unwrap();

            let mut want = pt.clone();
            reference_128(&KEY128, NIST_COUNTER, &mut want);
            assert_eq!(&buf[offset..offset + len], &want[..], "len={len} offset={offset}");
            assert!(
                buf[..offset].iter().all(|&b| b == CANARY),
                "prefix clobbered len={len} offset={offset}"
            );
            assert!(
                buf[offset + len..].iter().all(|&b| b == CANARY),
                "suffix clobbered len={len} offset={offset}"
            );
        }
    }
}

#[test]
fn exact_allocation_no_overflow() {
    // Process buffers that end exactly at the end of their allocation; an
    // over-read/over-write of the final partial block would touch the guard.
    for len in 0..=64usize {
        let pt = pattern(len);
        let mut buf = pt.clone();
        let mut c = Aes128Ctr::new(&KEY128);
        c.set_counter(&NIST_COUNTER);
        c.xor_keystream(&mut buf).unwrap();
        let mut want = pt;
        reference_128(&KEY128, NIST_COUNTER, &mut want);
        assert_eq!(buf, want, "len={len}");
    }
}

// ── In-place / repeated calls / reset ────────────────────────────────────────

#[test]
fn in_place_roundtrip_and_reset() {
    let pt = pattern(1024);
    let counter = [0x42u8; 16];

    let mut buf = pt.clone();
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&counter);
    c.xor_keystream(&mut buf).unwrap();
    assert_ne!(buf, pt);

    // Reset the same object with set_counter and decrypt in place.
    c.set_counter(&counter);
    c.xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, pt);

    let mut buf = pt.clone();
    let mut c = Aes256Ctr::new(&KEY256);
    c.set_counter(&counter);
    c.xor_keystream(&mut buf).unwrap();
    assert_ne!(buf, pt);
    c.set_counter(&counter);
    c.xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, pt);
}

#[test]
fn empty_input_does_not_advance_counter() {
    for _ in 0..3 {
        let mut c = Aes128Ctr::new(&KEY128);
        c.set_counter(&NIST_COUNTER);
        let mut empty: [u8; 0] = [];
        c.xor_keystream(&mut empty).unwrap();
        let mut probe = [0u8; 16];
        c.xor_keystream(&mut probe).unwrap();
        assert_eq!(probe.to_vec(), keystream_128(&KEY128, u128::from_be_bytes(NIST_COUNTER), 16));
    }
}

#[test]
fn successive_calls_advance_by_blocks() {
    // Consecutive 16-byte calls must equal one big call.
    let pt = pattern(256);
    let mut chunks = pt.clone();
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&NIST_COUNTER);
    for chunk in chunks.chunks_mut(16) {
        c.xor_keystream(chunk).unwrap();
    }
    let mut one = pt.clone();
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&NIST_COUNTER);
    c.xor_keystream(&mut one).unwrap();
    assert_eq!(chunks, one);
}

#[test]
fn block_aligned_chunking_is_equivalent() {
    for total in [16usize, 32, 48, 64, 128, 256, 512, 1024] {
        let pt = pattern(total);
        let mut one = pt.clone();
        let mut c = Aes128Ctr::new(&KEY128);
        c.set_counter(&NIST_COUNTER);
        c.xor_keystream(&mut one).unwrap();

        for chunk_size in [16usize, 32, 64, 128, 256] {
            if chunk_size > total {
                continue;
            }
            let mut got = pt.clone();
            let mut c = Aes128Ctr::new(&KEY128);
            c.set_counter(&NIST_COUNTER);
            for chunk in got.chunks_mut(chunk_size) {
                // Every chunk, including a short last one, stays block aligned.
                assert_eq!(chunk.len() % 16, 0);
                c.xor_keystream(chunk).unwrap();
            }
            assert_eq!(got, one, "total={total} chunk_size={chunk_size}");
            let mut want = pt.clone();
            reference_128(&KEY128, NIST_COUNTER, &mut want);
            assert_eq!(want, one);
            assert_eq!(got, want);
        }
    }
}

/// XOR `pt` under `counter` with a fresh AES-128-CTR cipher, feeding the input
/// in the given chunk sizes (which must sum to `pt.len()`).
fn encrypt_chunks_128(pt: &[u8], counter: [u8; 16], chunks: &[usize]) -> Vec<u8> {
    let mut buf = pt.to_vec();
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&counter);
    let mut off = 0;
    for &n in chunks {
        c.xor_keystream(&mut buf[off..off + n]).unwrap();
        off += n;
    }
    assert_eq!(off, pt.len(), "chunk sizes must cover the input");
    buf
}

fn encrypt_chunks_256(pt: &[u8], counter: [u8; 16], chunks: &[usize]) -> Vec<u8> {
    let mut buf = pt.to_vec();
    let mut c = Aes256Ctr::new(&KEY256);
    c.set_counter(&counter);
    let mut off = 0;
    for &n in chunks {
        c.xor_keystream(&mut buf[off..off + n]).unwrap();
        off += n;
    }
    assert_eq!(off, pt.len(), "chunk sizes must cover the input");
    buf
}

/// Regression test for the two-time-pad bug: a call ending mid-block must not
/// restart the block keystream on the next call. Shapes 5+11, 15+1, 1+15,
/// 17+15 and 16+17 cover a partial block, both sides of a block boundary, and
/// whole-block prefixes.
#[test]
fn partial_block_chunks_do_not_reuse_keystream() {
    let cases: &[(&str, &[usize])] = &[
        ("5+11", &[5, 11]),
        ("15+1", &[15, 1]),
        ("1+15", &[1, 15]),
        ("17+15", &[17, 15]),
        ("16+17", &[16, 17]),
    ];

    for &(label, chunks) in cases {
        let total: usize = chunks.iter().sum();
        let pt = pattern(total);

        let mut want128 = pt.clone();
        reference_128(&KEY128, NIST_COUNTER, &mut want128);
        let got128 = encrypt_chunks_128(&pt, NIST_COUNTER, chunks);
        assert_eq!(got128, want128, "AES-128 CTR {label}");

        let mut want256 = pt.clone();
        reference_256(&KEY256, NIST_COUNTER, &mut want256);
        let got256 = encrypt_chunks_256(&pt, NIST_COUNTER, chunks);
        assert_eq!(got256, want256, "AES-256 CTR {label}");
    }

    // Explicitly pin the byte position: after a 5-byte call, the next 11 bytes
    // must be keystream bytes 5..16 of block 0, not 0..11.
    let got = encrypt_chunks_128(&[0u8; 16], NIST_COUNTER, &[5, 11]);
    let ks = keystream_128(&KEY128, u128::from_be_bytes(NIST_COUNTER), 16);
    assert_eq!(&got[5..16], &ks[5..16], "second chunk must continue at offset 5");
    assert_ne!(&got[5..16], &ks[..11], "second chunk must not reuse keystream bytes 0..11");
}

#[test]
fn one_byte_chunks_match_one_shot() {
    // Keep enough room for the 257-byte maximum below the reserved final
    // counter value; the overflow cases are covered separately.
    for counter in [u128::from_be_bytes(NIST_COUNTER), u128::MAX - 512, 0] {
        let ctr = counter.to_be_bytes();
        for len in 0..=257usize {
            let pt = pattern(len);

            let mut want = pt.clone();
            reference_128(&KEY128, ctr, &mut want);
            let got = encrypt_chunks_128(&pt, ctr, &vec![1; len]);
            assert_eq!(got, want, "aes128 len={len} counter={counter:#034x}");

            let mut want = pt.clone();
            reference_256(&KEY256, ctr, &mut want);
            let got = encrypt_chunks_256(&pt, ctr, &vec![1; len]);
            assert_eq!(got, want, "aes256 len={len} counter={counter:#034x}");
        }
    }
}

/// Deterministic randomized split sweep: every length up to 512 bytes is split
/// at pseudo-random boundaries (including zero-length chunks) and must equal a
/// one-shot encryption under both key sizes, across counter-wrap boundaries.
#[test]
fn randomized_split_sweep_matches_one_shot() {
    let mut rng = ChaChaRng::seed_from_u64(0x1234_5678_9abc_def0);

    for counter in counters() {
        for len in 0..=512usize {
            let mut chunks = vec![0usize];
            let mut remaining = len;
            while remaining > 0 {
                let max = remaining.min(53);
                let mut n = (rng.next_u64() as usize) % (max + 1);
                if n == 0 {
                    // Keep an occasional explicit zero-length call, but make
                    // progress so the loop terminates.
                    if rng.next_u64() & 1 == 0 {
                        chunks.push(0);
                    }
                    n = 1;
                }
                chunks.push(n);
                remaining -= n;
            }

            let pt = pattern(len);
            let mut want = pt.clone();
            reference_128(&KEY128, counter, &mut want);
            let got = encrypt_chunks_128(&pt, counter, &chunks);
            assert_eq!(got, want, "aes128 len={len} chunks={chunks:?}");

            let mut want = pt.clone();
            reference_256(&KEY256, counter, &mut want);
            let got = encrypt_chunks_256(&pt, counter, &chunks);
            assert_eq!(got, want, "aes256 len={len} chunks={chunks:?}");
        }
    }
}

/// `set_counter` must discard keystream buffered from a partial call.
#[test]
fn set_counter_discards_buffered_keystream() {
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&NIST_COUNTER);
    let mut first = [0u8; 5];
    c.xor_keystream(&mut first).unwrap();
    assert_eq!(first.to_vec(), keystream_128(&KEY128, u128::from_be_bytes(NIST_COUNTER), 5));

    let other = u128::from_be_bytes(NIST_COUNTER).wrapping_add(100);
    c.set_counter(&other.to_be_bytes());
    let mut next = [0u8; 11];
    c.xor_keystream(&mut next).unwrap();
    assert_eq!(next.to_vec(), keystream_128(&KEY128, other, 11));

    let mut c = Aes256Ctr::new(&KEY256);
    c.set_counter(&NIST_COUNTER);
    let mut first = [0u8; 15];
    c.xor_keystream(&mut first).unwrap();
    c.set_counter(&other.to_be_bytes());
    let mut next = [0u8; 1];
    c.xor_keystream(&mut next).unwrap();
    assert_eq!(next.to_vec(), keystream_256(&KEY256, other, 1));
}

#[test]
fn chunked_in_place_decrypt_roundtrip() {
    let pt = pattern(300);
    let counter = [0x42u8; 16];

    let mut ct = pt.clone();
    let mut enc = Aes128Ctr::new(&KEY128);
    enc.set_counter(&counter);
    for chunk in ct.chunks_mut(7) {
        enc.xor_keystream(chunk).unwrap();
    }
    assert_ne!(ct, pt);

    let mut dec = Aes128Ctr::new(&KEY128);
    dec.set_counter(&counter);
    for chunk in ct.chunks_mut(13) {
        dec.xor_keystream(chunk).unwrap();
    }
    assert_eq!(ct, pt);
}

// ── Counter carry / wrap through the public API ──────────────────────────────

#[test]
fn full_128_bit_counter_carry_and_wrap() {
    let cases: [(u128, &str); 5] = [
        (0x0000_0000_0000_0000_ffff_ffff_ffff_ffff, "carry out of low 64 bits"),
        (0x0000_0000_0000_0000_0000_0000_ffff_ffff, "carry out of low 32 bits"),
        (0xffff_ffff_ffff_ffff_ffff_ffff_ffff_fffe, "carry across the 128-bit wrap"),
        (0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff, "full wrap looks like 0"),
        (0x00ff_00ff_00ff_00ff_00ff_00ff_00ff_00fe, "carry through two nonzero bytes"),
    ];

    for (start, what) in cases {
        // Four blocks so the carry propagates into the next counter word.
        if start.checked_add(3).is_none() {
            let mut buf = [0xa5u8; 64];
            let mut c = Aes128Ctr::new(&KEY128);
            c.set_counter(&start.to_be_bytes());
            assert_eq!(c.xor_keystream(&mut buf), Err(StreamCipherError::CounterExhausted), "{what}");
            assert_eq!(buf, [0xa5u8; 64], "{what}: failed call must not touch the buffer");

            let mut buf = [0xa5u8; 64];
            let mut c = Aes256Ctr::new(&KEY256);
            c.set_counter(&start.to_be_bytes());
            assert_eq!(
                c.xor_keystream(&mut buf),
                Err(StreamCipherError::CounterExhausted),
                "{what} (aes256)"
            );
            assert_eq!(buf, [0xa5u8; 64], "{what}: failed call must not touch the buffer");
            continue;
        }

        let ks = keystream_128(&KEY128, start, 64);
        let mut buf = [0u8; 64];
        let mut c = Aes128Ctr::new(&KEY128);
        c.set_counter(&start.to_be_bytes());
        c.xor_keystream(&mut buf).unwrap();
        assert_eq!(buf.to_vec(), ks, "{what}");

        let mut buf = [0u8; 64];
        let mut c = Aes256Ctr::new(&KEY256);
        c.set_counter(&start.to_be_bytes());
        c.xor_keystream(&mut buf).unwrap();
        assert_eq!(buf.to_vec(), keystream_256(&KEY256, start, 64), "{what} (aes256)");
    }
}

#[test]
fn counter_exhaustion_with_buffered_state_returns_error_without_writing() {
    // Using the reserved final counter value is rejected even for one block.
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&[0xff; 16]);
    let mut buf = [0xa5u8; 16];
    assert_eq!(
        c.xor_keystream(&mut buf),
        Err(StreamCipherError::CounterExhausted),
        "final counter value must be reserved"
    );
    assert_eq!(buf, [0xa5u8; 16], "failed call must not touch the buffer");

    // One block below the reserved value is usable; the next call fails.
    let mut counter = [0xffu8; 16];
    counter[15] = 0xfe;
    let mut c = Aes128Ctr::new(&KEY128);
    c.set_counter(&counter);
    let mut first = [0u8; 16];
    c.xor_keystream(&mut first).unwrap();
    let mut second = [0xa5u8; 1];
    assert_eq!(
        c.xor_keystream(&mut second),
        Err(StreamCipherError::CounterExhausted),
        "counter must not wrap after the last usable block"
    );
    assert_eq!(second, [0xa5u8; 1], "failed call must not touch the buffer");
}

#[test]
fn new_starts_at_zero_counter() {
    // AES-128 of an all-zero block under the all-zero key.
    let mut buf = [0u8; 16];
    let mut c = Aes128Ctr::new(&[0u8; 16]);
    c.xor_keystream(&mut buf).unwrap();
    assert_eq!(
        buf.to_vec(),
        hex::decode("66e94bd4ef8a2c3b884cfa59ca342b2e").unwrap(),
        "default counter must be zero"
    );
}

#[test]
fn distinct_counters_give_distinct_keystream() {
    let mut a = [0u8; 16];
    let mut b = [0u8; 16];
    let mut ca = Aes128Ctr::new(&KEY128);
    let mut cb = Aes128Ctr::new(&KEY128);
    ca.set_counter(&[0u8; 16]);
    cb.set_counter(&{
        let mut x = [0u8; 16];
        x[15] = 1;
        x
    });
    ca.xor_keystream(&mut a).unwrap();
    cb.xor_keystream(&mut b).unwrap();
    assert_ne!(a, b);
}
