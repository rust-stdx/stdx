#![allow(unsafe_op_in_unsafe_fn)]

//! x86-64 AVX2 implementation of Argon2's compression function `G`.
//!
//! A port of the Blamka round from the reference implementation
//! (`phc-winner-argon2` / libsodium): the 64-bit words are processed four at a
//! time in `__m256i` lanes.

use core::arch::x86_64::*;

use super::Block;

/// Rotate each 64-bit lane right by 32 bits (swap 32-bit halves in each lane).
#[inline(always)]
unsafe fn rotr32(x: __m256i) -> __m256i {
    // `_MM_SHUFFLE(2, 3, 0, 1)`
    _mm256_shuffle_epi32::<0xB1>(x)
}

/// Rotate each 64-bit lane right by 24 bits.
#[inline(always)]
unsafe fn rotr24(x: __m256i) -> __m256i {
    // `_MM_SHUFFLE`-style byte mask applied independently to each 128-bit lane.
    #[rustfmt::skip]
    let mask = _mm256_setr_epi8(
        3, 4, 5, 6, 7, 0, 1, 2, 11, 12, 13, 14, 15, 8, 9, 10,
        3, 4, 5, 6, 7, 0, 1, 2, 11, 12, 13, 14, 15, 8, 9, 10,
    );
    _mm256_shuffle_epi8(x, mask)
}

/// Rotate each 64-bit lane right by 16 bits.
#[inline(always)]
unsafe fn rotr16(x: __m256i) -> __m256i {
    #[rustfmt::skip]
    let mask = _mm256_setr_epi8(
        2, 3, 4, 5, 6, 7, 0, 1, 10, 11, 12, 13, 14, 15, 8, 9,
        2, 3, 4, 5, 6, 7, 0, 1, 10, 11, 12, 13, 14, 15, 8, 9,
    );
    _mm256_shuffle_epi8(x, mask)
}

/// Rotate each 64-bit lane right by 63 bits.
#[inline(always)]
unsafe fn rotr63(x: __m256i) -> __m256i {
    _mm256_xor_si256(_mm256_srli_epi64::<63>(x), _mm256_add_epi64(x, x))
}

/// Argon2's `fBlaMka`: `a + b + 2 * low32(a) * low32(b)`, lane-wise.
#[inline(always)]
unsafe fn fblamka(a: __m256i, b: __m256i) -> __m256i {
    // `_mm256_mul_epu32` multiplies the low 32 bits of each 64-bit lane.
    let prod = _mm256_mul_epu32(a, b);
    _mm256_add_epi64(a, _mm256_add_epi64(b, _mm256_add_epi64(prod, prod)))
}

/// First half of the Blamka round (rotations by 32 and 24).
macro_rules! g1 {
    ($a0:ident, $a1:ident, $b0:ident, $b1:ident, $c0:ident, $c1:ident, $d0:ident, $d1:ident) => {{
        $a0 = fblamka($a0, $b0);
        $d0 = rotr32(_mm256_xor_si256($d0, $a0));
        $c0 = fblamka($c0, $d0);
        $b0 = rotr24(_mm256_xor_si256($b0, $c0));

        $a1 = fblamka($a1, $b1);
        $d1 = rotr32(_mm256_xor_si256($d1, $a1));
        $c1 = fblamka($c1, $d1);
        $b1 = rotr24(_mm256_xor_si256($b1, $c1));
    }};
}

/// Second half of the Blamka round (rotations by 16 and 63).
macro_rules! g2 {
    ($a0:ident, $a1:ident, $b0:ident, $b1:ident, $c0:ident, $c1:ident, $d0:ident, $d1:ident) => {{
        $a0 = fblamka($a0, $b0);
        $d0 = rotr16(_mm256_xor_si256($d0, $a0));
        $c0 = fblamka($c0, $d0);
        $b0 = rotr63(_mm256_xor_si256($b0, $c0));

        $a1 = fblamka($a1, $b1);
        $d1 = rotr16(_mm256_xor_si256($d1, $a1));
        $c1 = fblamka($c1, $d1);
        $b1 = rotr63(_mm256_xor_si256($b1, $c1));
    }};
}

/// Diagonalize for the row round: rotate B/C/D by 1/2/3 lanes.
macro_rules! diagonalize_1 {
    ($a0:ident, $b0:ident, $c0:ident, $d0:ident, $a1:ident, $b1:ident, $c1:ident, $d1:ident) => {{
        let _ = &$a0;
        let _ = &$a1;
        $b0 = _mm256_permute4x64_epi64::<0x39>($b0);
        $c0 = _mm256_permute4x64_epi64::<0x4E>($c0);
        $d0 = _mm256_permute4x64_epi64::<0x93>($d0);

        $b1 = _mm256_permute4x64_epi64::<0x39>($b1);
        $c1 = _mm256_permute4x64_epi64::<0x4E>($c1);
        $d1 = _mm256_permute4x64_epi64::<0x93>($d1);
    }};
}

/// Undiagonalize for the row round (inverse of [`diagonalize_1`]).
macro_rules! undiagonalize_1 {
    ($a0:ident, $b0:ident, $c0:ident, $d0:ident, $a1:ident, $b1:ident, $c1:ident, $d1:ident) => {{
        let _ = &$a0;
        let _ = &$a1;
        $b0 = _mm256_permute4x64_epi64::<0x93>($b0);
        $c0 = _mm256_permute4x64_epi64::<0x4E>($c0);
        $d0 = _mm256_permute4x64_epi64::<0x39>($d0);

        $b1 = _mm256_permute4x64_epi64::<0x93>($b1);
        $c1 = _mm256_permute4x64_epi64::<0x4E>($c1);
        $d1 = _mm256_permute4x64_epi64::<0x39>($d1);
    }};
}

/// Diagonalize for the column round (crosses the two 4-lane halves).
macro_rules! diagonalize_2 {
    ($a0:ident, $a1:ident, $b0:ident, $b1:ident, $c0:ident, $c1:ident, $d0:ident, $d1:ident) => {{
        let _ = &$a0;
        let _ = &$a1;
        let btmp1 = _mm256_blend_epi32::<0xCC>($b0, $b1);
        let btmp2 = _mm256_blend_epi32::<0x33>($b0, $b1);
        $b1 = _mm256_permute4x64_epi64::<0xB1>(btmp1);
        $b0 = _mm256_permute4x64_epi64::<0xB1>(btmp2);

        core::mem::swap(&mut $c0, &mut $c1);

        let dtmp1 = _mm256_blend_epi32::<0xCC>($d0, $d1);
        let dtmp2 = _mm256_blend_epi32::<0x33>($d0, $d1);
        $d0 = _mm256_permute4x64_epi64::<0xB1>(dtmp1);
        $d1 = _mm256_permute4x64_epi64::<0xB1>(dtmp2);
    }};
}

/// Undiagonalize for the column round (inverse of [`diagonalize_2`]).
macro_rules! undiagonalize_2 {
    ($a0:ident, $a1:ident, $b0:ident, $b1:ident, $c0:ident, $c1:ident, $d0:ident, $d1:ident) => {{
        let _ = &$a0;
        let _ = &$a1;
        let btmp1 = _mm256_blend_epi32::<0xCC>($b0, $b1);
        let btmp2 = _mm256_blend_epi32::<0x33>($b0, $b1);
        $b0 = _mm256_permute4x64_epi64::<0xB1>(btmp1);
        $b1 = _mm256_permute4x64_epi64::<0xB1>(btmp2);

        core::mem::swap(&mut $c0, &mut $c1);

        let dtmp1 = _mm256_blend_epi32::<0x33>($d0, $d1);
        let dtmp2 = _mm256_blend_epi32::<0xCC>($d0, $d1);
        $d0 = _mm256_permute4x64_epi64::<0xB1>(dtmp1);
        $d1 = _mm256_permute4x64_epi64::<0xB1>(dtmp2);
    }};
}

/// One row round (BLAKE2_ROUND_1): two 16-word rows processed together.
#[inline(always)]
unsafe fn round_1(s: *mut __m256i, i: usize) {
    let base = 8 * i;
    let mut a0 = *s.add(base);
    let mut a1 = *s.add(base + 4);
    let mut b0 = *s.add(base + 1);
    let mut b1 = *s.add(base + 5);
    let mut c0 = *s.add(base + 2);
    let mut c1 = *s.add(base + 6);
    let mut d0 = *s.add(base + 3);
    let mut d1 = *s.add(base + 7);

    g1!(a0, a1, b0, b1, c0, c1, d0, d1);
    g2!(a0, a1, b0, b1, c0, c1, d0, d1);
    diagonalize_1!(a0, b0, c0, d0, a1, b1, c1, d1);
    g1!(a0, a1, b0, b1, c0, c1, d0, d1);
    g2!(a0, a1, b0, b1, c0, c1, d0, d1);
    undiagonalize_1!(a0, b0, c0, d0, a1, b1, c1, d1);

    *s.add(base) = a0;
    *s.add(base + 4) = a1;
    *s.add(base + 1) = b0;
    *s.add(base + 5) = b1;
    *s.add(base + 2) = c0;
    *s.add(base + 6) = c1;
    *s.add(base + 3) = d0;
    *s.add(base + 7) = d1;
}

/// One column round (BLAKE2_ROUND_2).
#[inline(always)]
unsafe fn round_2(s: *mut __m256i, i: usize) {
    let mut a0 = *s.add(i);
    let mut a1 = *s.add(4 + i);
    let mut b0 = *s.add(8 + i);
    let mut b1 = *s.add(12 + i);
    let mut c0 = *s.add(16 + i);
    let mut c1 = *s.add(20 + i);
    let mut d0 = *s.add(24 + i);
    let mut d1 = *s.add(28 + i);

    g1!(a0, a1, b0, b1, c0, c1, d0, d1);
    g2!(a0, a1, b0, b1, c0, c1, d0, d1);
    diagonalize_2!(a0, a1, b0, b1, c0, c1, d0, d1);
    g1!(a0, a1, b0, b1, c0, c1, d0, d1);
    g2!(a0, a1, b0, b1, c0, c1, d0, d1);
    undiagonalize_2!(a0, a1, b0, b1, c0, c1, d0, d1);

    *s.add(i) = a0;
    *s.add(4 + i) = a1;
    *s.add(8 + i) = b0;
    *s.add(12 + i) = b1;
    *s.add(16 + i) = c0;
    *s.add(20 + i) = c1;
    *s.add(24 + i) = d0;
    *s.add(28 + i) = d1;
}

/// Shared body of the AVX2 compression function.
#[inline(always)]
unsafe fn fill_block_impl(prev: &Block, reference: &Block, cur: *mut Block, with_xor: bool) {
    let mut state = [_mm256_setzero_si256(); 32];
    let mut block_xy = [_mm256_setzero_si256(); 32];

    let prev_ptr = prev.v.as_ptr() as *const __m256i;
    let reference_ptr = reference.v.as_ptr() as *const __m256i;

    if with_xor {
        let cur_ptr = (*cur).v.as_ptr() as *const __m256i;
        for i in 0..32 {
            let x = _mm256_xor_si256(_mm256_loadu_si256(prev_ptr.add(i)), _mm256_loadu_si256(reference_ptr.add(i)));
            state[i] = x;
            block_xy[i] = _mm256_xor_si256(x, _mm256_loadu_si256(cur_ptr.add(i)));
        }
    } else {
        for i in 0..32 {
            let x = _mm256_xor_si256(_mm256_loadu_si256(prev_ptr.add(i)), _mm256_loadu_si256(reference_ptr.add(i)));
            state[i] = x;
            block_xy[i] = x;
        }
    }

    let sp = state.as_mut_ptr();
    for i in 0..4 {
        round_1(sp, i);
    }
    for i in 0..4 {
        round_2(sp, i);
    }

    let cur_ptr = (*cur).v.as_mut_ptr() as *mut __m256i;
    for i in 0..32 {
        _mm256_storeu_si256(cur_ptr.add(i), _mm256_xor_si256(state[i], block_xy[i]));
    }
}

/// AVX2 kernel. Only call this when the CPU advertises `avx2`.
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn fill_block(prev: &Block, reference: &Block, cur: *mut Block, with_xor: bool) {
    fill_block_impl(prev, reference, cur, with_xor)
}
