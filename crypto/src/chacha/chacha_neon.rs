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
//
// The ChaCha state is manipulated in a "word-major" layout (each vector holds word *i* of
// the 4 blocks). Before the keystream can be XORed with the input, the word-major vectors
// are transposed into "block-major" vectors (each holding 16 bytes of a single block) using
// only VUZP1/VUZP2, so that a single vector load/xor/store can stream the keystream into
// the input without materializing an intermediate byte buffer.
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
    // only used for the final, partial chunk (see below)
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

    // number of bytes that can be processed as full 4-block chunks
    let full_len = (input.len() / (SIMD_LANES * BLOCK_SIZE)) * (SIMD_LANES * BLOCK_SIZE);

    // process the full chunks by computing 4 blocks in parallel and XORing them directly
    // into input (transpose + direct emit, one word group at a time to keep the register
    // pressure low)
    let mut offset = 0;
    while offset < full_len {
        inject_counter(&mut state_simd, counter, IS_IETF);

        // SAFETY: `offset` is always a multiple of the block batch size and stays within `input`
        chacha_neon_emit::<ROUNDS>(state_simd, unsafe { input.as_mut_ptr().add(offset) });

        counter = counter.wrapping_add(SIMD_LANES as u64);
        offset += SIMD_LANES * BLOCK_SIZE;
    }

    // the final chunk (< 4 * 64 bytes) is XORed with a keystream materialized in a buffer:
    // this preserves the unconsumed part of the last block for `keystream_leftover`.
    if offset < input.len() {
        inject_counter(&mut state_simd, counter, IS_IETF);

        let mut keystream_vectors = chacha_neon_rounds::<ROUNDS>(state_simd);
        // add the initial state to the working state to get the keystream
        for i in 0..STATE_WORDS {
            keystream_vectors[i] = vaddq_u32(keystream_vectors[i], state_simd[i]);
        }
        serialize_keystream(&keystream_vectors, &mut keystream);

        input[offset..]
            .iter_mut()
            .zip(keystream)
            .for_each(|(plaintext, keystream)| *plaintext ^= keystream);

        counter = counter.wrapping_add(((input.len() - offset) as u64).div_ceil(BLOCK_SIZE as u64));
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

/// Injects the counter into words 12 (and 13 for DJB) of the SIMD state.
///
/// Word 12 receives `counter..counter+SIMD_LANES` (one value per block). For the DJB
/// variant, word 13 receives the high 32 bits of the 64-bit counter per block.
#[inline(always)]
fn inject_counter(state_simd: &mut [uint32x4_t; STATE_WORDS], counter: u64, is_ietf: bool) {
    let mut counter_lane_low = [0u32; SIMD_LANES];
    let mut counter_lane_high = [0u32; SIMD_LANES];
    for i in 0..SIMD_LANES {
        if is_ietf {
            counter_lane_low[i] = (counter as u32).wrapping_add(i as u32);
        } else {
            let counter_lane = counter.wrapping_add(i as u64);
            counter_lane_low[i] = counter_lane as u32;
            counter_lane_high[i] = (counter_lane >> 32) as u32;
        }
    }
    unsafe {
        state_simd[12] = vld1q_u32(counter_lane_low.as_ptr());
        if !is_ietf {
            state_simd[13] = vld1q_u32(counter_lane_high.as_ptr());
        }
    }
}

/// Computes 4 64-byte ChaCha blocks in parallel and XORs them with the 4 input blocks at
/// `input` (256 bytes) in place.
///
/// The state is kept in "word-major" layout (word *i* of all 4 blocks in a single vector).
/// After the rounds, each group of 4 words is added to the initial state, transposed with
/// `transpose4` into 4 block-major vectors (16 bytes of a single block), and XORed into
/// the input with a single vector load/xor/store each. Processing one word group at a time
/// keeps the number of live vectors low enough to avoid spilling keystream vectors to the
/// stack. No intermediate keystream buffer is involved.
#[inline(always)]
fn chacha_neon_emit<const ROUNDS: usize>(state: [uint32x4_t; STATE_WORDS], input: *mut u8) {
    let working = chacha_neon_rounds::<ROUNDS>(state);

    // word group g: words 4g..4g+4 of every block -> bytes 16g..16g+16 of every block
    for g in 0..4 {
        let k0 = unsafe { vaddq_u32(working[4 * g], state[4 * g]) };
        let k1 = unsafe { vaddq_u32(working[4 * g + 1], state[4 * g + 1]) };
        let k2 = unsafe { vaddq_u32(working[4 * g + 2], state[4 * g + 2]) };
        let k3 = unsafe { vaddq_u32(working[4 * g + 3], state[4 * g + 3]) };
        let (t0, t1, t2, t3) = transpose4(k0, k1, k2, k3);
        unsafe {
            xor_store(input.add(g * 16), t0);
            xor_store(input.add(BLOCK_SIZE + g * 16), t1);
            xor_store(input.add(2 * BLOCK_SIZE + g * 16), t2);
            xor_store(input.add(3 * BLOCK_SIZE + g * 16), t3);
        }
    }
}

/// Loads the 16 bytes at `ptr`, XORs them with `v`, and stores the result back.
#[inline(always)]
unsafe fn xor_store(ptr: *mut u8, v: uint32x4_t) {
    let loaded = unsafe { vld1q_u32(ptr.cast::<u32>()) };
    let result = unsafe { veorq_u32(loaded, v) };
    unsafe { vst1q_u32(ptr.cast::<u32>(), result) };
}

/// Turns 4 word-major vectors (each holding word *i* of blocks `counter..counter+3`)
/// into 4 block-major vectors (each holding 4 consecutive words of one block), using
/// only VUZP1/VUZP2 (deinterleave even/odd).
#[inline(always)]
fn transpose4(
    a: uint32x4_t,
    b: uint32x4_t,
    c: uint32x4_t,
    d: uint32x4_t,
) -> (uint32x4_t, uint32x4_t, uint32x4_t, uint32x4_t) {
    let u0 = unsafe { vuzp1q_u32(a, b) };
    let u1 = unsafe { vuzp2q_u32(a, b) };
    let u2 = unsafe { vuzp1q_u32(c, d) };
    let u3 = unsafe { vuzp2q_u32(c, d) };
    unsafe { (vuzp1q_u32(u0, u2), vuzp1q_u32(u1, u3), vuzp2q_u32(u0, u2), vuzp2q_u32(u1, u3)) }
}

/// Computes the 4 64-byte ChaCha working states in parallel using NEON vectors.
///
/// Returns the state in word-major layout: `result[i]` holds word *i* of all 4 blocks,
/// *before* the initial state is added back. The caller is responsible for adding the
/// initial state and for reordering (`chacha_neon_emit`) or serializing
/// (`serialize_keystream`) the resulting keystream into a byte stream.
#[inline(always)]
fn chacha_neon_rounds<const ROUNDS: usize>(state: [uint32x4_t; STATE_WORDS]) -> [uint32x4_t; STATE_WORDS] {
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

    tmp_state
}

/// Serializes the word-major keystream vectors into a block-major byte buffer:
/// `block1 || block2 || block3 || block4`.
///
/// Each iteration of the loop writes a 32-bit word for each block into the buffer.
/// The first iteration writes block1[0..4], block2[0..4], block3[0..4], block4[0..4],
/// the second writes block1[4..8], block2[4..8], block3[4..8], block4[4..8], and so
/// on, for the 16 32-bit words of the ChaCha state.
#[inline(always)]
fn serialize_keystream(keystream_vectors: &[uint32x4_t; STATE_WORDS], keystream: &mut [u8; SIMD_LANES * BLOCK_SIZE]) {
    let keystream_ptr = keystream.as_mut_ptr();

    for word_index in 0..STATE_WORDS {
        let mut lanes = [0u32; SIMD_LANES];
        unsafe { vst1q_u32(lanes.as_mut_ptr(), keystream_vectors[word_index]) };

        // each lane is a 32-bit little-endian word
        for block in 0..SIMD_LANES {
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
