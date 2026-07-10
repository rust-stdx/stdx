#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::x86_64::*;

use super::sha512::SHA512_K;

/// Process one or more 128-byte SHA-512 blocks using Intel SHA512+AVX hardware
/// acceleration.
///
/// # Safety
///
/// The caller must ensure the CPU supports `sha512`, `avx`, and `ssse3` features.
#[target_feature(enable = "sha512,avx,ssse3")]
pub(crate) unsafe fn compress(state: &mut [u64; 8], blocks: &[[u8; 128]]) {
    let mut abef = _mm256_setr_epi64x(state[0] as i64, state[1] as i64, state[4] as i64, state[5] as i64);
    let mut cdgh = _mm256_setr_epi64x(state[2] as i64, state[3] as i64, state[6] as i64, state[7] as i64);

    for block in blocks {
        let abef_start = abef;
        let cdgh_start = cdgh;

        let mut w = load_block(block);

        let mut t = 0usize;
        while t < 80 {
            rounds16(&mut abef, &mut cdgh, &w, t);

            if t < 64 {
                schedule_update(&mut w);
            }

            t += 16;
        }

        abef = _mm256_add_epi64(abef, abef_start);
        cdgh = _mm256_add_epi64(cdgh, cdgh_start);
    }

    let mut abef_arr = [0u64; 4];
    let mut cdgh_arr = [0u64; 4];
    _mm256_storeu_si256(abef_arr.as_mut_ptr().cast(), abef);
    _mm256_storeu_si256(cdgh_arr.as_mut_ptr().cast(), cdgh);

    state[0] = abef_arr[0];
    state[1] = abef_arr[1];
    state[2] = cdgh_arr[0];
    state[3] = cdgh_arr[1];
    state[4] = abef_arr[2];
    state[5] = abef_arr[3];
    state[6] = cdgh_arr[2];
    state[7] = cdgh_arr[3];
}

/// Process 16 SHA-512 rounds using the current 4-register message schedule.
#[inline]
unsafe fn rounds16(abef: &mut __m256i, cdgh: &mut __m256i, w: &[__m256i; 4], t: usize) {
    round_pair(abef, cdgh, w[0], &SHA512_K[t..]);
    round_pair(abef, cdgh, w[1], &SHA512_K[t + 4..]);
    round_pair(abef, cdgh, w[2], &SHA512_K[t + 8..]);
    round_pair(abef, cdgh, w[3], &SHA512_K[t + 12..]);
}

/// Perform two SHA-512 compression rounds with the round keys from one
/// 256-bit schedule register and their corresponding K constants.
#[inline]
unsafe fn round_pair(abef: &mut __m256i, cdgh: &mut __m256i, w: __m256i, k: &[u64]) {
    let lo_w = _mm256_castsi256_si128(w);
    let rk0 = _mm_set_epi64x(k[1] as i64, k[0] as i64);
    let wk0 = _mm_add_epi64(lo_w, rk0);
    let prev_abef = *abef;
    *abef = _mm256_sha512rnds2_epi64(*cdgh, *abef, wk0);
    *cdgh = prev_abef;

    let hi_w = _mm256_extracti128_si256(w, 1);
    let rk1 = _mm_set_epi64x(k[3] as i64, k[2] as i64);
    let wk1 = _mm_add_epi64(hi_w, rk1);
    let prev_abef = *abef;
    *abef = _mm256_sha512rnds2_epi64(*cdgh, *abef, wk1);
    *cdgh = prev_abef;
}

/// Compute the next 16 message-schedule words in-place from the current 4
/// register window, using Intel SHA512MSG1 and SHA512MSG2 hardware
/// expansion.
#[inline]
unsafe fn schedule_update(w: &mut [__m256i; 4]) {
    let [w0, w1, w2, w3] = *w;

    let lo_w1 = _mm256_castsi256_si128(w1);
    let t = _mm256_sha512msg1_epi64(w0, lo_w1);
    let t = _mm256_add_epi64(t, extract_w7_xmm(w2, w3));
    let new_w0 = _mm256_sha512msg2_epi64(t, w3);

    let lo_w2 = _mm256_castsi256_si128(w2);
    let t = _mm256_sha512msg1_epi64(w1, lo_w2);
    let t = _mm256_add_epi64(t, extract_w7_xmm(w3, new_w0));
    let new_w1 = _mm256_sha512msg2_epi64(t, new_w0);

    let lo_w3 = _mm256_castsi256_si128(w3);
    let t = _mm256_sha512msg1_epi64(w2, lo_w3);
    let t = _mm256_add_epi64(t, extract_w7_xmm(new_w0, new_w1));
    let new_w2 = _mm256_sha512msg2_epi64(t, new_w1);

    let lo_new_w0 = _mm256_castsi256_si128(new_w0);
    let t = _mm256_sha512msg1_epi64(w3, lo_new_w0);
    let t = _mm256_add_epi64(t, extract_w7_xmm(new_w1, new_w2));
    let new_w3 = _mm256_sha512msg2_epi64(t, new_w2);

    *w = [new_w0, new_w1, new_w2, new_w3];
}

/// Extract the W[i-7] contribution for the schedule update from two adjacent
/// 256-bit registers using 128-bit lane operations.
///
/// Returns a 256-bit value whose 4 × u64 elements are `a[1], a[2], a[3], b[0]`.
#[inline]
unsafe fn extract_w7_xmm(a: __m256i, b: __m256i) -> __m256i {
    let a_lo = _mm256_castsi256_si128(a);
    let a_hi = _mm256_extracti128_si256(a, 1);
    let b_lo = _mm256_castsi256_si128(b);

    let lo = _mm_alignr_epi8(a_hi, a_lo, 8);
    let hi = _mm_alignr_epi8(b_lo, a_hi, 8);
    _mm256_set_m128i(hi, lo)
}

/// Load a 128-byte block into 4 × 256-bit registers, converting from
/// big-endian byte order to native (little-endian) u64 elements.
#[inline]
unsafe fn load_block(block: &[u8; 128]) -> [__m256i; 4] {
    let mask = _mm_set_epi8(8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7);

    let block_ptr: *const __m256i = block.as_ptr().cast();
    core::array::from_fn(|i| {
        let v = _mm256_loadu_si256(block_ptr.add(i));
        let lo = _mm_shuffle_epi8(_mm256_castsi256_si128(v), mask);
        let hi = _mm_shuffle_epi8(_mm256_extracti128_si256(v, 1), mask);
        _mm256_set_m128i(hi, lo)
    })
}
