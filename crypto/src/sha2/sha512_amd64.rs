#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::x86_64::*;

use super::sha512::SHA512_K;

#[target_feature(enable = "sha512,avx")]
pub(crate) unsafe fn compress(state: &mut [u64; 8], block: &[u8; 128]) {
    let mut w = [0u64; 80];

    let mut i = 0usize;
    while i < 16 {
        let offset = i * 8;
        w[i] = u64::from_be_bytes([
            block[offset],
            block[offset + 1],
            block[offset + 2],
            block[offset + 3],
            block[offset + 4],
            block[offset + 5],
            block[offset + 6],
            block[offset + 7],
        ]);
        i += 1;
    }

    while i < 80 {
        let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
        let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        i += 1;
    }

    let mut abef = _mm256_setr_epi64x(state[0] as i64, state[1] as i64, state[4] as i64, state[5] as i64);
    let mut cdgh = _mm256_setr_epi64x(state[2] as i64, state[3] as i64, state[6] as i64, state[7] as i64);
    let abef_start = abef;
    let cdgh_start = cdgh;

    i = 0;
    while i < 80 {
        let wk = _mm_set_epi64x(
            w[i + 1].wrapping_add(SHA512_K[i + 1]) as i64,
            w[i].wrapping_add(SHA512_K[i]) as i64,
        );

        let prev_abef = abef;
        abef = _mm256_sha512rnds2_epi64(cdgh, abef, wk);
        cdgh = prev_abef;

        i += 2;
    }

    abef = _mm256_add_epi64(abef, abef_start);
    cdgh = _mm256_add_epi64(cdgh, cdgh_start);

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
