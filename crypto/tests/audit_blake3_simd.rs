//! Independent tests for the BLAKE3 SIMD backends.
//!
//! Only the public [`Blake3`] API is used here, so these tests exercise whatever
//! backend the build/runtime selects: NEON on aarch64, `simd128` under
//! `wasm32-wasip1`, AVX2 / AVX-512 on `x86_64`.
//!
//! Everything is checked against [`reference`], a small BLAKE3 implementation
//! written directly from the specification (state-based compression plus the
//! recursive tree), so a wrong SIMD kernel, a wrong chunk/tree boundary, or a
//! wrong XOF counter cannot hide behind the crate's own tree logic.
//!
//! Kernel-level differential tests (directly against the scalar kernels, with
//! counters/flags/degrees that the public API cannot reach) live in the
//! `#[cfg(test)]` modules of `blake3/avx2.rs`, `avx512.rs`, `neon.rs` and
//! `wasm32_simd128.rs`.

use crypto::{Hasher, Xof, blake3::Blake3};
use rand::{RngExt, SeedableRng, rngs::StdRng};

/// Official BLAKE3 test vector for the empty input: a 131-byte XOF output, so
/// the final partial word is covered as well. Used to anchor [`reference`].
const EMPTY_XOF_HEX: &str = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262e00f03e7b69af26b7faaf09fcd333050338ddfe085b8cc869ca98b206c08243a26f5487789e8f660afe6c99ef9e0c52b92e7393024a80459cf91f476f9ffdbda7001c22e159b402631f277ca96f2defdf1078282314e763699a31c5363165421cce14d";

/// Official 32-byte digest for a 64-byte input of `test_input` bytes.
const INPUT_64_HASH_HEX: &str = "4eed7141ea4a5cd4b788606bd23f46e212af9cacebacdc7d1f4c6dc7f2511b98";

/// Official 32-byte digest for a 1024-byte input of `test_input` bytes.
const INPUT_1024_HASH_HEX: &str = "42214739f095a406f3fc83deb889744ac00df831c10daa55189b5d121c855af7";

const KEY: &[u8; 32] = b"whats the Elvish word for friend";
const CONTEXT: &str = "stdx blake3 simd test context v1";

/// The deterministic test input used by the official vectors: `i % 251`.
fn test_input(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i % 251) as u8).collect()
}

/// Deterministic seeded `StdRng` bytes (deterministic and reproducible).
fn pattern(len: usize, seed: u64) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = vec![0u8; len];
    rng.fill(&mut out);
    out
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Reference implementation, from the BLAKE3 specification.
////////////////////////////////////////////////////////////////////////////////////////////////////
mod reference {
    pub const BLOCK_LEN: usize = 64;
    pub const CHUNK_LEN: usize = 1024;

    pub const CHUNK_START: u8 = 1 << 0;
    pub const CHUNK_END: u8 = 1 << 1;
    pub const PARENT: u8 = 1 << 2;
    pub const ROOT: u8 = 1 << 3;
    pub const KEYED_HASH: u8 = 1 << 4;
    pub const DERIVE_KEY_CONTEXT: u8 = 1 << 5;
    pub const DERIVE_KEY_MATERIAL: u8 = 1 << 6;

    pub const IV: [u32; 8] = [
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
    ];

    const MSG_SCHEDULE: [[usize; 16]; 7] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8],
        [3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1],
        [10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6],
        [12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4],
        [9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7],
        [11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13],
    ];

    #[allow(clippy::too_many_arguments)]
    fn g(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
        state[a] = state[a].wrapping_add(state[b]).wrapping_add(mx);
        state[d] = (state[d] ^ state[a]).rotate_right(16);
        state[c] = state[c].wrapping_add(state[d]);
        state[b] = (state[b] ^ state[c]).rotate_right(12);
        state[a] = state[a].wrapping_add(state[b]).wrapping_add(my);
        state[d] = (state[d] ^ state[a]).rotate_right(8);
        state[c] = state[c].wrapping_add(state[d]);
        state[b] = (state[b] ^ state[c]).rotate_right(7);
    }

    fn round(state: &mut [u32; 16], m: &[u32; 16], r: usize) {
        let s = &MSG_SCHEDULE[r];
        g(state, 0, 4, 8, 12, m[s[0]], m[s[1]]);
        g(state, 1, 5, 9, 13, m[s[2]], m[s[3]]);
        g(state, 2, 6, 10, 14, m[s[4]], m[s[5]]);
        g(state, 3, 7, 11, 15, m[s[6]], m[s[7]]);
        g(state, 0, 5, 10, 15, m[s[8]], m[s[9]]);
        g(state, 1, 6, 11, 12, m[s[10]], m[s[11]]);
        g(state, 2, 7, 8, 13, m[s[12]], m[s[13]]);
        g(state, 3, 4, 9, 14, m[s[14]], m[s[15]]);
    }

    fn compress(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8) -> [u32; 16] {
        let mut m = [0u32; 16];
        for (word, chunk) in m.iter_mut().zip(block.as_chunks::<4>().0) {
            *word = u32::from_le_bytes(*chunk);
        }
        let mut state = [
            cv[0],
            cv[1],
            cv[2],
            cv[3],
            cv[4],
            cv[5],
            cv[6],
            cv[7],
            IV[0],
            IV[1],
            IV[2],
            IV[3],
            counter as u32,
            (counter >> 32) as u32,
            block_len as u32,
            flags as u32,
        ];
        for r in 0..7 {
            round(&mut state, &m, r);
        }
        state
    }

    fn words_from_le_bytes(bytes: &[u8]) -> [u32; 8] {
        let mut words = [0u32; 8];
        for (word, chunk) in words.iter_mut().zip(bytes.as_chunks::<4>().0) {
            *word = u32::from_le_bytes(*chunk);
        }
        words
    }

    fn le_bytes(words: &[u32]) -> Vec<u8> {
        let mut out = Vec::with_capacity(words.len() * 4);
        for word in words {
            out.extend_from_slice(&word.to_le_bytes());
        }
        out
    }

    /// One node of the tree, before the `ROOT` compression: the chaining value
    /// fed into its final compression, the final block, and its parameters.
    struct Node {
        cv: [u32; 8],
        block: [u8; BLOCK_LEN],
        block_len: u8,
        counter: u64,
        flags: u8,
    }

    fn chaining_value(node: &Node) -> [u32; 8] {
        let state = compress(&node.cv, &node.block, node.block_len, node.counter, node.flags);
        let mut cv = [0u32; 8];
        for i in 0..8 {
            cv[i] = state[i] ^ state[i + 8];
        }
        cv
    }

    /// The split used by BLAKE3: the largest power-of-two number of whole
    /// chunks that still leaves at least one byte on the right.
    fn left_subtree_len(len: u64) -> u64 {
        debug_assert!(len > CHUNK_LEN as u64);
        let full_chunks = (len - 1) / CHUNK_LEN as u64;
        let power_of_two = 1u64 << (63 - full_chunks.leading_zeros());
        power_of_two * CHUNK_LEN as u64
    }

    fn node_for(input: &[u8], chunk_counter: u64, key: &[u32; 8], flags: u8) -> Node {
        if input.len() <= CHUNK_LEN {
            // One chunk. Every full block except the last is compressed as it
            // is; the last block is kept for the (possibly) root compression.
            let block_count = if input.is_empty() {
                1
            } else {
                input.len().div_ceil(BLOCK_LEN)
            };
            let mut cv = *key;
            for block_index in 0..block_count - 1 {
                let block_bytes = &input[block_index * BLOCK_LEN..(block_index + 1) * BLOCK_LEN];
                let block: &[u8; BLOCK_LEN] = block_bytes.try_into().unwrap();
                let block_flags = flags | if block_index == 0 { CHUNK_START } else { 0 };
                let state = compress(&cv, block, BLOCK_LEN as u8, chunk_counter, block_flags);
                for i in 0..8 {
                    cv[i] = state[i] ^ state[i + 8];
                }
            }
            let start = (block_count - 1) * BLOCK_LEN;
            let end = input.len().min(start + BLOCK_LEN);
            let mut block = [0u8; BLOCK_LEN];
            block[..end - start].copy_from_slice(&input[start..end]);
            let block_flags = flags | if block_count == 1 { CHUNK_START } else { 0 } | CHUNK_END;
            Node {
                cv,
                block,
                block_len: (end - start) as u8,
                counter: chunk_counter,
                flags: block_flags,
            }
        } else {
            let left_len = left_subtree_len(input.len() as u64) as usize;
            let (left, right) = input.split_at(left_len);
            let left_node = node_for(left, chunk_counter, key, flags);
            let right_node = node_for(right, chunk_counter + (left_len / CHUNK_LEN) as u64, key, flags);
            let mut block = [0u8; BLOCK_LEN];
            block[..32].copy_from_slice(&le_bytes(&chaining_value(&left_node)));
            block[32..].copy_from_slice(&le_bytes(&chaining_value(&right_node)));
            Node {
                cv: *key,
                block,
                block_len: BLOCK_LEN as u8,
                counter: 0,
                flags: flags | PARENT,
            }
        }
    }

    fn root_state(node: &Node, counter: u64) -> [u32; 16] {
        compress(&node.cv, &node.block, node.block_len, counter, node.flags | ROOT)
    }

    pub fn hash_with_words(input: &[u8], key: &[u32; 8], flags: u8) -> [u8; 32] {
        let state = root_state(&node_for(input, 0, key, flags), 0);
        // The truncated output is the first half of the state folded with the
        // second half (the same feed-forward as a chaining value).
        let mut cv = [0u32; 8];
        for i in 0..8 {
            cv[i] = state[i] ^ state[i + 8];
        }
        le_bytes(&cv).try_into().unwrap()
    }

    pub fn hash_plain(input: &[u8]) -> [u8; 32] {
        hash_with_words(input, &IV, 0)
    }

    pub fn keyed_hash(key: &[u8; 32], input: &[u8]) -> [u8; 32] {
        hash_with_words(input, &words_from_le_bytes(key), KEYED_HASH)
    }

    pub fn derive_key(context: &str, material: &[u8]) -> [u8; 32] {
        let context_key = hash_with_words(context.as_bytes(), &IV, DERIVE_KEY_CONTEXT);
        let material_key = words_from_le_bytes(&context_key);
        hash_with_words(material, &material_key, DERIVE_KEY_MATERIAL)
    }

    pub fn xof_plain(input: &[u8], out: &mut [u8]) {
        let node = node_for(input, 0, &IV, 0);
        for (index, block) in out.chunks_mut(BLOCK_LEN).enumerate() {
            let mut state = root_state(&node, node.counter.wrapping_add(index as u64));
            // Extended output block: first half ^= second half, second half ^= cv.
            for i in 0..8 {
                state[i] ^= state[i + 8];
                state[i + 8] ^= node.cv[i];
            }
            let bytes = le_bytes(&state);
            block.copy_from_slice(&bytes[..block.len()]);
        }
    }

    pub fn xof_bytes(input: &[u8], len: usize) -> Vec<u8> {
        let mut out = vec![0u8; len];
        xof_plain(input, &mut out);
        out
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////////////////////////
/// The reference implementation and the crate must both reproduce the official
/// vectors; this proves the differential tests below compare against a correct
/// oracle rather than a shared misunderstanding.
#[test]
fn known_answer_anchors() {
    let empty_xof = hex::decode(EMPTY_XOF_HEX).unwrap();

    let empty: [u8; 0] = [];
    assert_eq!(&Blake3::hash(&empty).as_ref()[..32], &empty_xof[..32]);

    let mut out = [0u8; 131];
    Blake3::new().finalize_xof().squeeze(&mut out);
    assert_eq!(&out[..], &empty_xof[..]);

    assert_eq!(&reference::hash_plain(&empty)[..], &empty_xof[..32]);
    assert_eq!(&reference::xof_bytes(&empty, 131)[..], &empty_xof[..]);

    let input = test_input(64);
    let expected = hex::decode(INPUT_64_HASH_HEX).unwrap();
    assert_eq!(&Blake3::hash(&input).as_ref()[..32], &expected[..]);
    assert_eq!(&reference::hash_plain(&input)[..], &expected[..]);

    let input = test_input(1024);
    let expected = hex::decode(INPUT_1024_HASH_HEX).unwrap();
    assert_eq!(&Blake3::hash(&input).as_ref()[..32], &expected[..]);
    assert_eq!(&reference::hash_plain(&input)[..], &expected[..]);
}

/// Input lengths 0..=2100 cover every 64/128/1024/2048-byte boundary and both
/// sides of the NEON/wasm (4), AVX2 (8) and AVX-512 (16) chunk thresholds for
/// small inputs; the explicit list covers wide subtrees, where the parent
/// compression also runs `DEGREE` ways.
#[test]
fn one_shot_matches_reference() {
    let mut lengths: Vec<usize> = (0..=2100).collect();
    lengths.extend([
        4095,
        4096,
        4097,
        8191,
        8192,
        8193,
        16_383,
        16_384,
        16_385,
        17 * 1024,
        33 * 1024,
        64 * 1024,
        100 * 1024,
        102400, // the largest official test vector
    ]);
    lengths.sort_unstable();
    lengths.dedup();

    for &len in &lengths {
        let input = pattern(len, 0xA11CE ^ len as u64);

        assert_eq!(
            &Blake3::hash(&input).as_ref()[..32],
            &reference::hash_plain(&input)[..],
            "hash mismatch at input_len {len}"
        );
        assert_eq!(
            &Blake3::keyed_hash(KEY, &input).as_ref()[..32],
            &reference::keyed_hash(KEY, &input)[..],
            "keyed_hash mismatch at input_len {len}"
        );
        assert_eq!(
            Blake3::derive_key(CONTEXT, &input),
            reference::derive_key(CONTEXT, &input),
            "derive_key mismatch at input_len {len}"
        );
    }
}

/// Same data located at every small non-zero offset, and at the very end of the
/// allocation (where an over-read of a full SIMD block crosses the allocation
/// boundary and is caught by ASan builds).
#[test]
fn unaligned_and_end_of_allocation_inputs() {
    for &len in &[0usize, 1, 2, 63, 64, 65, 127, 1023, 1024, 1025, 2048, 4096, 4097] {
        for &pad in &[0usize, 1, 2, 3, 5, 7, 9, 13, 15, 17, 31, 33, 63, 64] {
            let data = pattern(len, 0xBEEF ^ len as u64);
            let mut buf = vec![0xA6u8; pad + len + 1];
            buf[pad..pad + len].copy_from_slice(&data);
            let input = &buf[pad..pad + len];

            assert_eq!(
                &Blake3::hash(input).as_ref()[..32],
                &reference::hash_plain(input)[..],
                "hash mismatch at input_len {len}, offset {pad}"
            );

            let mut out = [0x77u8; 3 + 131 + 5];
            let mut hasher = Blake3::new();
            hasher.update(input);
            hasher.finalize_xof().squeeze(&mut out[3..3 + 131]);
            assert_eq!(
                &out[3..3 + 131],
                &reference::xof_bytes(input, 131)[..],
                "xof mismatch at input_len {len}, offset {pad}"
            );
            assert!(out[..3].iter().all(|b| *b == 0x77), "xof wrote before the output slice");
            assert!(out[3 + 131..].iter().all(|b| *b == 0x77), "xof wrote past the output slice");
        }
    }
}

/// Every output length the caller may ask for in one call (1..=256), plus
/// blocks either side of the 64-byte output-block / SIMD-group boundaries, and
/// the incremental squeeze splits that cross those boundaries.
#[test]
fn xof_squeeze_matches_reference() {
    let mut lengths: Vec<usize> = (1..=256).collect();
    lengths.extend([257, 383, 384, 511, 512, 513, 1023, 1024, 1025, 2048, 4096]);

    for &input_len in &[0usize, 1, 63, 64, 65, 1023, 1024, 1025, 4096, 16_385] {
        let input = pattern(input_len, 0xC0FFEE ^ input_len as u64);
        for &out_len in &lengths {
            let expected = reference::xof_bytes(&input, out_len);

            // One call, with canaries and an unaligned output slice.
            let mut buf = vec![0x5Du8; 9 + out_len + 7];
            let mut hasher = Blake3::new();
            hasher.update(&input);
            hasher.finalize_xof().squeeze(&mut buf[9..9 + out_len]);
            assert_eq!(
                &buf[9..9 + out_len],
                &expected[..],
                "xof mismatch (input_len {input_len}, out_len {out_len})"
            );
            assert!(buf[..9].iter().all(|b| *b == 0x5D), "xof wrote before the output slice");
            assert!(buf[9 + out_len..].iter().all(|b| *b == 0x5D), "xof wrote past the output slice");

            // Two uneven calls: exercises the buffered remainder of the current
            // output block as well as the wide `xof_many` path.
            for &split in &[1usize, 17, 31, 63, 64, 65, out_len / 2] {
                if split >= out_len {
                    continue;
                }
                let mut out = vec![0u8; out_len];
                let mut hasher = Blake3::new();
                hasher.update(&input);
                let mut xof = hasher.finalize_xof();
                xof.squeeze(&mut out[..split]);
                xof.squeeze(&mut out[split..]);
                assert_eq!(
                    &out[..],
                    &expected[..],
                    "xof split mismatch (input_len {input_len}, out_len {out_len}, split {split})"
                );
            }

            // One byte at a time, but only for the smaller outputs.
            if out_len <= 130 {
                let mut out = vec![0u8; out_len];
                let mut hasher = Blake3::new();
                hasher.update(&input);
                let mut xof = hasher.finalize_xof();
                for byte in out.iter_mut() {
                    xof.squeeze(core::slice::from_mut(byte));
                }
                assert_eq!(
                    &out[..],
                    &expected[..],
                    "xof byte-at-a-time mismatch (input_len {input_len}, out_len {out_len})"
                );
            }
        }
    }
}

/// Incremental `update` must agree with the one-shot hash (and the reference)
/// at every split, including splits exactly on SIMD chunk boundaries.
#[test]
fn incremental_updates_match_reference() {
    for &len in &[
        1usize, 63, 64, 65, 128, 1023, 1024, 1025, 1536, 2048, 2049, 4096, 4097, 8192, 8193, 16_385,
    ] {
        let input = pattern(len, 0xFEED ^ len as u64);
        let expected = reference::hash_plain(&input);

        let mut splits = vec![
            0,
            1,
            2,
            63,
            64,
            65,
            127,
            128,
            129,
            len / 2,
            len.saturating_sub(65),
            len - 1,
            len,
        ];
        splits.retain(|split| *split <= len);
        splits.sort_unstable();
        splits.dedup();

        for &split in &splits {
            let mut hasher = Blake3::new();
            hasher.update(&input[..split]);
            hasher.update(&input[split..]);
            assert_eq!(
                &hasher.sum().as_ref()[..32],
                &expected[..],
                "incremental mismatch at input_len {len}, split {split}"
            );
        }

        // One byte at a time for the smaller inputs.
        if len <= 1100 {
            let mut hasher = Blake3::new();
            for byte in &input {
                hasher.update(core::slice::from_ref(byte));
            }
            assert_eq!(
                &hasher.sum().as_ref()[..32],
                &expected[..],
                "byte-at-a-time mismatch at input_len {len}"
            );
        }
    }
}
