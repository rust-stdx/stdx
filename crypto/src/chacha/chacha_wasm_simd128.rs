use core::arch::wasm32::*;

use super::{BLOCK_SIZE, STATE_WORDS};

// https://doc.rust-lang.org/stable/core/arch/wasm32

/// how many ChaCha blocks we compute in parallel (depends on the size of the SIMD vectors, here 128 / 32 = 4)
pub const SIMD_LANES: usize = 4;

pub fn chacha_wasm_simd128<const ROUNDS: usize, const IS_IETF: bool>(
    state: &mut [u32; STATE_WORDS],
    input: &mut [u8],
    keystream_leftover: &mut [u8; BLOCK_SIZE - 1],
) {
    let mut counter = if IS_IETF {
        state[12] as u64
    } else {
        ((state[13] as u64) << 32) | (state[12] as u64)
    };
    let mut keystream = [0u8; SIMD_LANES * BLOCK_SIZE];

    let w13 = if IS_IETF { state[13] as i32 } else { 0 };

    let mut initial_state: [v128; STATE_WORDS] = [
        // constant
        i32x4_splat(state[0] as i32),
        i32x4_splat(state[1] as i32),
        i32x4_splat(state[2] as i32),
        i32x4_splat(state[3] as i32),
        // key
        i32x4_splat(state[4] as i32),
        i32x4_splat(state[5] as i32),
        i32x4_splat(state[6] as i32),
        i32x4_splat(state[7] as i32),
        i32x4_splat(state[8] as i32),
        i32x4_splat(state[9] as i32),
        i32x4_splat(state[10] as i32),
        i32x4_splat(state[11] as i32),
        // counter, set it to 0 for now, it is injected later during each iteration of the loop
        i32x4_splat(0),
        // word 13: nonce low for IETF, counter high for DJB (set to 0 and injected per-lane)
        i32x4_splat(w13),
        // nonce
        i32x4_splat(state[14] as i32),
        i32x4_splat(state[15] as i32),
    ];

    // process input by chunks of 4 * 64 bytes
    // The final chunk may contain fewer than `SIMD_LANES` blocks. Unused
    // lanes compute wrapped or reserved counter values, but are never stored.
    for input_blocks in input.chunks_mut(BLOCK_SIZE * SIMD_LANES) {
        // inject counter as 32-bit little-endian words for each lane
        let mut counter_lane_low = [0u32; SIMD_LANES];
        let mut counter_lane_high = [0u32; SIMD_LANES];
        for i in 0..SIMD_LANES {
            if IS_IETF {
                counter_lane_low[i] = (counter as u32).wrapping_add(i as u32);
            } else {
                let counter_lane = counter.wrapping_add(i as u64);
                counter_lane_low[i] = counter_lane as u32;
                counter_lane_high[i] = (counter_lane >> 32) as u32;
            }
        }

        // SAFETY: both arrays contain exactly `SIMD_LANES` u32 elements, and
        // `v128_load` reads one full 16-byte vector from each.
        unsafe {
            initial_state[12] = v128_load(counter_lane_low.as_ptr() as *const v128);
            if !IS_IETF {
                initial_state[13] = v128_load(counter_lane_high.as_ptr() as *const v128);
            }
        }

        // compute 4 64-byte ChaCha blocks in parallel
        chacha20_wasm_4blocks::<ROUNDS>(initial_state, &mut keystream);

        // XOR plaintext with keystream
        input_blocks
            .iter_mut()
            .zip(keystream)
            .for_each(|(plaintext, keystream)| *plaintext ^= keystream);

        counter = counter.wrapping_add((input_blocks.len() as u64).div_ceil(BLOCK_SIZE as u64));
    }

    state[12] = counter as u32;
    if !IS_IETF {
        state[13] = (counter >> 32) as u32;
    }

    if !input.len().is_multiple_of(BLOCK_SIZE) {
        let last_keystream_block_index = ((input.len() - 1) / BLOCK_SIZE) % SIMD_LANES;
        let last_keystream_block_offset = last_keystream_block_index * BLOCK_SIZE;
        // copy the last 63 bytes of the leftover keystream block
        keystream_leftover
            .copy_from_slice(&keystream[last_keystream_block_offset + 1..last_keystream_block_offset + BLOCK_SIZE]);
    }

    #[cfg(feature = "zeroize")]
    zeroize::Zeroize::zeroize(&mut keystream);
}

#[inline(always)]
fn rotate_left(a: v128, n: u32) -> v128 {
    v128_or(u32x4_shl(a, n), u32x4_shr(a, 32 - n))
}

/// Compute 4 64-byte ChaCha blocks in parallel using WASM simd128 vectors.
/// The keystream is the 4 64-byte blocks computed in parallel.
/// [ block1 (64 bytes) || block2 (64 bytes) || block3 (64 bytes) || block4 (64 bytes) ... ]
#[inline(always)]
fn chacha20_wasm_4blocks<const ROUNDS: usize>(
    initial_state: [v128; STATE_WORDS],
    keystream: &mut [u8; SIMD_LANES * BLOCK_SIZE],
) {
    let keystream_ptr = keystream.as_mut_ptr();

    // SAFETY: SIMD128 is enabled for this module; all accesses are to
    // fixed-size local state and the `keystream` array.
    unsafe {
        let mut working_state = initial_state;

        macro_rules! quarter_round {
            ($a:expr, $b:expr, $c:expr, $d:expr) => {
                // a += b; d ^= a; d <<<= 16
                $a = i32x4_add($a, $b);
                $d = v128_xor($d, $a);
                $d = rotate_left($d, 16);

                // c += d; b ^= c; b <<<= 12
                $c = i32x4_add($c, $d);
                $b = v128_xor($b, $c);
                $b = rotate_left($b, 12);

                // a += b; d ^= a; d <<<= 8
                $a = i32x4_add($a, $b);
                $d = v128_xor($d, $a);
                $d = rotate_left($d, 8);

                // c += d; b ^= c; b <<<= 7
                $c = i32x4_add($c, $d);
                $b = v128_xor($b, $c);
                $b = rotate_left($b, 7);
            };
        }

        for _ in 0..ROUNDS / 2 {
            // column rounds
            quarter_round!(working_state[0], working_state[4], working_state[8], working_state[12]);
            quarter_round!(working_state[1], working_state[5], working_state[9], working_state[13]);
            quarter_round!(working_state[2], working_state[6], working_state[10], working_state[14]);
            quarter_round!(working_state[3], working_state[7], working_state[11], working_state[15]);

            // diagonal rounds
            quarter_round!(working_state[0], working_state[5], working_state[10], working_state[15]);
            quarter_round!(working_state[1], working_state[6], working_state[11], working_state[12]);
            quarter_round!(working_state[2], working_state[7], working_state[8], working_state[13]);
            quarter_round!(working_state[3], working_state[4], working_state[9], working_state[14]);
        }

        // Each iteration of the loop writes a 32-bit word for each block (lane) into keystream.
        // The first iteration writes the following bytes: block1[0..4], block2[0..4], block3[0..4], block4[0..4]
        // the second iteration writes block1[4..8], block2[4..8], block3[4..8], block4[4..8]
        // the third iteration writes block1[4..8], block2[8..12], block3[8..12], block4[8..12]
        // and so on, for the 16 32-bit words of the ChaCha state
        for word_index in 0..STATE_WORDS {
            // first we add the working state to the initial state to get the keystream
            working_state[word_index] = i32x4_add(working_state[word_index], initial_state[word_index]);

            // then we convert the SIMD lanes into the keystream bytes
            let mut lanes = [0u32; SIMD_LANES];
            v128_store(lanes.as_mut_ptr() as *mut v128, working_state[word_index]);

            // each lane is a 32-bit little-endian word
            for block in 0..SIMD_LANES {
                let byte_offset = (block * STATE_WORDS * 4) + (word_index * 4);
                core::ptr::copy_nonoverlapping(lanes[block].to_le_bytes().as_ptr(), keystream_ptr.add(byte_offset), 4);
            }
        }
    }
}

/// Kernel tests: exercise the private kernel directly, without
/// going through the dispatch in `chacha::ChaCha::xor_keystream`, and validate
/// the keystream, the advanced counter and the 63-byte `keystream_leftover`
/// tail against an independent reference implementation.
#[cfg(all(test, feature = "alloc"))]
mod kernel_tests {
    use alloc::{vec, vec::Vec};

    use super::{super::simd_test_reference as reference, *};

    fn available() -> bool {
        true
    }

    fn key() -> [u8; 32] {
        let mut k = [0u8; 32];
        for (i, b) in k.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(13).wrapping_add(7);
        }
        k
    }

    fn nonce12() -> [u8; 12] {
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc]
    }

    fn nonce8() -> [u8; 8] {
        [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]
    }

    fn plaintext(len: usize) -> Vec<u8> {
        let mut v = vec![0u8; len];
        let mut x = 0x1234_5678_9abc_def0u64 ^ len as u64;
        for b in &mut v {
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            *b = x.wrapping_mul(0x2545_f491_4f6c_dd1d) as u8;
        }
        v
    }

    fn check<const ROUNDS: usize, const IS_IETF: bool>(mut state: [u32; STATE_WORDS], start_counter: u64, len: usize) {
        let padded = len.div_ceil(BLOCK_SIZE) * BLOCK_SIZE;
        let full_keystream = reference::ref_stream_from::<ROUNDS>(&state, IS_IETF, padded);

        let mut buf = plaintext(len);
        let expected: Vec<u8> = buf.iter().zip(&full_keystream).map(|(p, k)| p ^ k).collect();

        let mut leftover = [0x5cu8; BLOCK_SIZE - 1];
        chacha_wasm_simd128::<ROUNDS, IS_IETF>(&mut state, &mut buf, &mut leftover);

        assert_eq!(buf, expected, "keystream mismatch, len={len}, ietf={IS_IETF}");

        let blocks = len.div_ceil(BLOCK_SIZE) as u64;
        if IS_IETF {
            assert_eq!(
                state[12],
                (start_counter as u32).wrapping_add(blocks as u32),
                "IETF counter not advanced correctly, len={len}"
            );
        } else {
            let counter = ((state[13] as u64) << 32) | state[12] as u64;
            assert_eq!(
                counter,
                start_counter.wrapping_add(blocks),
                "DJB counter not advanced correctly, len={len}"
            );
        }

        if !len.is_multiple_of(BLOCK_SIZE) {
            let last_block = padded - BLOCK_SIZE;
            assert_eq!(
                &leftover[..],
                &full_keystream[last_block + 1..last_block + BLOCK_SIZE],
                "leftover tail mismatch, len={len}, ietf={IS_IETF}"
            );
        } else {
            assert_eq!(leftover, [0x5cu8; BLOCK_SIZE - 1], "leftover written for a block-aligned input");
        }
    }

    #[test]
    fn kernel_matches_reference() {
        if !available() {
            eprintln!("skipping: required CPU feature not available at runtime");
            return;
        }
        let key = key();
        for &len in &[
            0usize, 1, 63, 64, 65, 127, 128, 129, 130, 191, 192, 193, 255, 256, 257, 320, 511, 512, 513, 600, 777,
            1023, 1024, 1025,
        ] {
            check::<20, true>(reference::ietf_state(&key, &nonce12(), 7), 7, len);
            check::<20, false>(reference::djb_state(&key, &nonce8(), 3), 3, len);
        }
    }

    #[test]
    fn kernel_high_counters() {
        if !available() {
            eprintln!("skipping: required CPU feature not available at runtime");
            return;
        }
        let key = key();
        let counter = u32::MAX as u64 - 4;
        check::<20, true>(reference::ietf_state(&key, &nonce12(), counter as u32), counter, 128);
        let counter = u64::MAX - 4;
        check::<20, false>(reference::djb_state(&key, &nonce8(), counter), counter, 128);
        let counter = (1u64 << 32) - 2;
        check::<20, false>(reference::djb_state(&key, &nonce8(), counter), counter, 512);
    }

    #[test]
    fn kernel_round_variants() {
        if !available() {
            eprintln!("skipping: required CPU feature not available at runtime");
            return;
        }
        let key = key();
        for &len in &[128usize, 129, 257, 512] {
            check::<8, true>(reference::ietf_state(&key, &nonce12(), 1), 1, len);
            check::<12, true>(reference::ietf_state(&key, &nonce12(), 1), 1, len);
            check::<8, false>(reference::djb_state(&key, &nonce8(), 1), 1, len);
            check::<12, false>(reference::djb_state(&key, &nonce8(), 1), 1, len);
        }
    }
}
