use core::arch::aarch64::*;

use super::{BLOCK_SIZE, STATE_WORDS};

// https://doc.rust-lang.org/stable/core/arch/aarch64

/// how many ChaCha blocks we compute in parallel (depends on the size of the SIMD vectors, here 128 / 32 = 4)
pub const SIMD_LANES: usize = 4;

// NEON instructions use 128-bit wide vectors, thus we compute 128 / 32 = 4 ChaCha blocks
// in parallel.
// Each vector can be seen as a 4 lanes, where each lane is 32-bit wide.
// Thus, in a single vector we will get the follwing state:
// [ block1 (32-bits) || block2 (32-bits) || block3 (32-bits) || block4 (32-bits) ]
// then we perform the normal ChaCha operations on these vectors, meaning that we compute
// 4 ChaCha blocks in parallel for every operation on these vectors.
#[target_feature(enable = "neon")]
pub fn chacha_neon<const ROUNDS: usize, const IS_IETF: bool>(
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

    let w13 = if IS_IETF { state[13] } else { 0 };

    // process 4 blocks of 64 bytes (4 * 16) in parallel
    let mut state_simd: [uint32x4_t; STATE_WORDS] = unsafe {
        [
            // constant
            vdupq_n_u32(state[0]),
            vdupq_n_u32(state[1]),
            vdupq_n_u32(state[2]),
            vdupq_n_u32(state[3]),
            // key
            vdupq_n_u32(state[4]),
            vdupq_n_u32(state[5]),
            vdupq_n_u32(state[6]),
            vdupq_n_u32(state[7]),
            vdupq_n_u32(state[8]),
            vdupq_n_u32(state[9]),
            vdupq_n_u32(state[10]),
            vdupq_n_u32(state[11]),
            // counter, set to 0, it will be injected later
            vld1q_u32([0, 0, 0, 0].as_ptr()),
            // word 13: nonce low for IETF, counter high for DJB (set to 0 and injected per-lane)
            vdupq_n_u32(w13),
            // nonce
            vdupq_n_u32(state[14]),
            vdupq_n_u32(state[15]),
        ]
    };

    for input_blocks in input.chunks_mut(BLOCK_SIZE * SIMD_LANES) {
        // inject counters
        // TODO: there should be a better / faster way
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
        unsafe {
            state_simd[12] = vld1q_u32(counter_lane_low.as_ptr());
            if !IS_IETF {
                state_simd[13] = vld1q_u32(counter_lane_high.as_ptr());
            }
        }

        // compute 4 blocks in parallel
        chacha_neon_4blocks::<ROUNDS>(state_simd, &mut keystream);

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

    if input.len() % BLOCK_SIZE != 0 {
        let last_keystream_block_index = ((input.len() - 1) / BLOCK_SIZE) % SIMD_LANES;
        let last_keystream_block_offset = last_keystream_block_index * BLOCK_SIZE;
        // copy the last 63 bytes of the leftover keystream block
        keystream_leftover
            .copy_from_slice(&keystream[last_keystream_block_offset + 1..last_keystream_block_offset + BLOCK_SIZE]);
    }
}

/// Compute 4 64-byte ChaCha blocks in parallel using NEON vectors.
#[inline(always)]
fn chacha_neon_4blocks<const ROUNDS: usize>(
    state: [uint32x4_t; STATE_WORDS],
    keystream: &mut [u8; SIMD_LANES * BLOCK_SIZE],
) {
    let keystream_ptr = keystream.as_mut_ptr();

    // tmp_state is the "working state" where we perform the ChaCha operations
    let mut tmp_state = state;

    for _ in 0..ROUNDS / 2 {
        // column rounds
        quarter_round(&mut tmp_state, 0, 4, 8, 12);
        quarter_round(&mut tmp_state, 1, 5, 9, 13);
        quarter_round(&mut tmp_state, 2, 6, 10, 14);
        quarter_round(&mut tmp_state, 3, 7, 11, 15);

        // diagonal rounds
        quarter_round(&mut tmp_state, 0, 5, 10, 15);
        quarter_round(&mut tmp_state, 1, 6, 11, 12);
        quarter_round(&mut tmp_state, 2, 7, 8, 13);
        quarter_round(&mut tmp_state, 3, 4, 9, 14);
    }

    // serialize the keystream as follow:
    // block1 || block2 || block3 || block4

    // Each iteration of the loop writes a 32-bit word for each block into keystream.
    // The first iteration writes block1[0], block2[0], block3[0], block4[0]
    // the second iterations writes block1[1], block2[1], block3[1], block4[1]
    // and so on, for the 16 32-bit words of the ChaCha state
    for word_index in 0..STATE_WORDS {
        // add working state to initial state to get the keystream
        let keystream_simd = unsafe { vaddq_u32(tmp_state[word_index], state[word_index]) };
        let mut lanes = [0u32; SIMD_LANES];
        unsafe { vst1q_u32(lanes.as_mut_ptr(), keystream_simd) };

        // TODO: there should be a fast way to directly XOR input with keystream SIMD here
        for block in 0..SIMD_LANES {
            // keystream[(block * STATE_WORDS) + word_index] = tmp[block].to_le();
            let byte_offset = (block * STATE_WORDS * 4) + (word_index * 4);
            unsafe {
                core::ptr::copy_nonoverlapping(lanes[block].to_le_bytes().as_ptr(), keystream_ptr.add(byte_offset), 4);
            }
        }
    }
}

#[inline(always)]
fn quarter_round(state: &mut [uint32x4_t; STATE_WORDS], a: usize, b: usize, c: usize, d: usize) {
    // optimized rotate_left for NEON
    macro_rules! rotate_left {
        ($v:expr, 8) => {{
            let mask_bytes = [3u8, 0, 1, 2, 7, 4, 5, 6, 11, 8, 9, 10, 15, 12, 13, 14];
            let mask = vld1q_u8(mask_bytes.as_ptr());

            $v = vreinterpretq_u32_u8(vqtbl1q_u8(vreinterpretq_u8_u32($v), mask))
        }};
        ($v:expr, 16) => {
            $v = vreinterpretq_u32_u16(vrev32q_u16(vreinterpretq_u16_u32($v)))
        };
        ($v:expr, $r:literal) => {
            $v = vorrq_u32(vshlq_n_u32($v, $r), vshrq_n_u32($v, 32 - $r))
        };
    }

    unsafe {
        // a += b; d ^= a; d <<<= 16
        state[a] = vaddq_u32(state[a], state[b]);
        state[d] = veorq_u32(state[d], state[a]);
        rotate_left!(state[d], 16);

        // c += d; b ^= c; b <<<= 12
        state[c] = vaddq_u32(state[c], state[d]);
        state[b] = veorq_u32(state[b], state[c]);
        rotate_left!(state[b], 12);

        // a += b; d ^= a; d <<<= 8
        state[a] = vaddq_u32(state[a], state[b]);
        state[d] = veorq_u32(state[d], state[a]);
        rotate_left!(state[d], 8);

        // c += d; b ^= c; b <<<= 7
        state[c] = vaddq_u32(state[c], state[d]);
        state[b] = veorq_u32(state[b], state[c]);
        rotate_left!(state[b], 7);
    }
}
