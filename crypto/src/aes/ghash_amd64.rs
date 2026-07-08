#![allow(unsafe_op_in_unsafe_fn)]
/// x86-64 GHASH using PCLMULQDQ + SSSE3 intrinsics.
///
/// GCM uses a big-endian polynomial ring; PCLMULQDQ uses little-endian.
/// To bridge this, every GCM block is transformed by reversing bits in each byte
/// with `_mm_shuffle_epi8` nibble lookups before carry-less multiplication.
use core::arch::x86_64::*;

/// Reverses bits in each byte of a 128-bit SSE value using SSSE3 `pshufb`.
#[target_feature(enable = "ssse3")]
#[inline]
pub(crate) unsafe fn bitreverse_per_byte(x: __m128i) -> __m128i {
    let reverse4 = _mm_setr_epi8(0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15);
    let low_nibble_mask = _mm_set1_epi8(0x0f_u8 as i8);

    let lo = _mm_and_si128(x, low_nibble_mask);
    let hi = _mm_and_si128(_mm_srli_epi16(x, 4), low_nibble_mask);

    let rev_lo = _mm_shuffle_epi8(reverse4, lo);
    let rev_hi = _mm_shuffle_epi8(reverse4, hi);
    _mm_or_si128(_mm_slli_epi16(rev_lo, 4), rev_hi)
}

/// Reduce a 256-bit carry-less product `(lo, hi)` modulo P = x^128+x^7+x^2+x+1.
#[target_feature(enable = "pclmulqdq,sse2")]
#[inline]
pub(crate) unsafe fn gcm_reduce(lo: __m128i, hi: __m128i) -> __m128i {
    let poly = _mm_set_epi64x(0, 0x87_i64);
    let t1 = _mm_clmulepi64_si128(hi, poly, 0x00);
    let t2 = _mm_clmulepi64_si128(_mm_shuffle_epi32(hi, 0x4e), poly, 0x00);
    let t2_lo = _mm_slli_si128(t2, 8);
    let t2_hi = _mm_srli_si128(t2, 8);
    let t3 = _mm_clmulepi64_si128(t2_hi, poly, 0x00);
    _mm_xor_si128(_mm_xor_si128(lo, t1), _mm_xor_si128(t2_lo, t3))
}

/// Multiply two GCM elements (both already byte-swapped) using Karatsuba + PCLMULQDQ.
#[target_feature(enable = "pclmulqdq,sse2")]
#[inline]
pub(crate) unsafe fn clmul_gcm(a: __m128i, b: __m128i) -> __m128i {
    let lo = _mm_clmulepi64_si128(a, b, 0x00);
    let hi = _mm_clmulepi64_si128(a, b, 0x11);
    let a_swap = _mm_shuffle_epi32(a, 0x4e);
    let b_swap = _mm_shuffle_epi32(b, 0x4e);
    let mid = _mm_clmulepi64_si128(_mm_xor_si128(a, a_swap), _mm_xor_si128(b, b_swap), 0x00);
    let mid_true = _mm_xor_si128(mid, _mm_xor_si128(lo, hi));
    let product_lo = _mm_xor_si128(lo, _mm_slli_si128(mid_true, 8));
    let product_hi = _mm_xor_si128(hi, _mm_srli_si128(mid_true, 8));
    gcm_reduce(product_lo, product_hi)
}

/// Feed a byte slice into the running GHASH state (single-block loop).
#[target_feature(enable = "pclmulqdq,ssse3,sse2")]
pub(crate) unsafe fn ghash_update(mut state: __m128i, h: __m128i, data: &[u8]) -> __m128i {
    let n = data.len();
    let mut i = 0;
    while i + 16 <= n {
        let block = bitreverse_per_byte(_mm_loadu_si128(data.as_ptr().add(i).cast()));
        state = clmul_gcm(_mm_xor_si128(state, block), h);
        i += 16;
    }
    if i < n {
        let mut buf = [0u8; 16];
        buf[..n - i].copy_from_slice(&data[i..]);
        let block = bitreverse_per_byte(_mm_loadu_si128(buf.as_ptr().cast()));
        state = clmul_gcm(_mm_xor_si128(state, block), h);
    }
    state
}

/// Process 4 successive GHASH blocks in one aggregated step.
///
///   S' = S·H⁴ ⊕ B₁·H⁴ ⊕ B₂·H³ ⊕ B₃·H² ⊕ B₄·H
#[target_feature(enable = "pclmulqdq,ssse3,sse2")]
#[inline]
pub(crate) unsafe fn ghash_4blocks(
    state: __m128i,
    b1: __m128i,
    b2: __m128i,
    b3: __m128i,
    b4: __m128i,
    h_powers: &[__m128i; 8],
) -> __m128i {
    let b1 = bitreverse_per_byte(b1);
    let b2 = bitreverse_per_byte(b2);
    let b3 = bitreverse_per_byte(b3);
    let b4 = bitreverse_per_byte(b4);

    let h4 = h_powers[3];
    let h3 = h_powers[2];
    let h2 = h_powers[1];
    let h1 = h_powers[0];

    let t0 = clmul_gcm(state, h4);
    let t1 = clmul_gcm(b1, h4);
    let t2 = clmul_gcm(b2, h3);
    let t3 = clmul_gcm(b3, h2);
    let t4 = clmul_gcm(b4, h1);

    let left = _mm_xor_si128(t0, t1);
    let right = _mm_xor_si128(_mm_xor_si128(t2, t3), t4);
    _mm_xor_si128(left, right)
}

/// Process 8 successive GHASH blocks in one aggregated step.
///
///   S' = S·H⁸ ⊕ B₁·H⁸ ⊕ B₂·H⁷ ⊕ B₃·H⁶ ⊕ B₄·H⁵
///      ⊕ B₅·H⁴ ⊕ B₆·H³ ⊕ B₇·H² ⊕ B₈·H
#[target_feature(enable = "pclmulqdq,ssse3,sse2")]
#[inline]
pub(crate) unsafe fn ghash_8blocks(
    state: __m128i,
    b1: __m128i,
    b2: __m128i,
    b3: __m128i,
    b4: __m128i,
    b5: __m128i,
    b6: __m128i,
    b7: __m128i,
    b8: __m128i,
    h_powers: &[__m128i; 8],
) -> __m128i {
    let b1 = bitreverse_per_byte(b1);
    let b2 = bitreverse_per_byte(b2);
    let b3 = bitreverse_per_byte(b3);
    let b4 = bitreverse_per_byte(b4);
    let b5 = bitreverse_per_byte(b5);
    let b6 = bitreverse_per_byte(b6);
    let b7 = bitreverse_per_byte(b7);
    let b8 = bitreverse_per_byte(b8);

    let h8 = h_powers[7];
    let h7 = h_powers[6];
    let h6 = h_powers[5];
    let h5 = h_powers[4];
    let h4 = h_powers[3];
    let h3 = h_powers[2];
    let h2 = h_powers[1];
    let h1 = h_powers[0];

    let t0 = clmul_gcm(state, h8);
    let t1 = clmul_gcm(b1, h8);
    let t2 = clmul_gcm(b2, h7);
    let t3 = clmul_gcm(b3, h6);
    let t4 = clmul_gcm(b4, h5);
    let t5 = clmul_gcm(b5, h4);
    let t6 = clmul_gcm(b6, h3);
    let t7 = clmul_gcm(b7, h2);
    let t8 = clmul_gcm(b8, h1);

    let l0 = _mm_xor_si128(t0, t1);
    let l1 = _mm_xor_si128(t2, t3);
    let l2 = _mm_xor_si128(t4, t5);
    let l3 = _mm_xor_si128(t6, t7);
    let m0 = _mm_xor_si128(l0, l1);
    let m1 = _mm_xor_si128(l2, l3);
    _mm_xor_si128(m0, _mm_xor_si128(m1, t8))
}
