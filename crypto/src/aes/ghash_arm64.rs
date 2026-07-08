#![allow(unsafe_op_in_unsafe_fn)]
/// aarch64 GHASH using ARMv8 Crypto extension PMULL instructions.
///
/// Uses `vmull_p64` intrinsics via inline asm for carry-less multiplication,
/// and `vrbitq_u8` for per-byte bit reversal (mapping between GCM's big-endian
/// polynomial representation and the little-endian PMULL domain).
use core::arch::aarch64::*;

/// Single-block GHASH feed (tail processing).
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn ghash_update(mut state: uint8x16_t, h: uint8x16_t, data: &[u8]) -> uint8x16_t {
    let n = data.len();
    let mut i = 0usize;

    while i + 16 <= n {
        let block = vrbitq_u8(vld1q_u8(data.as_ptr().add(i)));
        state = clmul_gcm_pmull(veorq_u8(state, block), h);
        i += 16;
    }

    if i < n {
        let mut padded = [0u8; 16];
        padded[..n - i].copy_from_slice(&data[i..]);
        let block = vrbitq_u8(vld1q_u8(padded.as_ptr()));
        state = clmul_gcm_pmull(veorq_u8(state, block), h);
    }

    state
}

/// 4-block aggregated GHASH.
///
///   state' = state·H⁴ ⊕ B₁·H⁴ ⊕ B₂·H³ ⊕ B₃·H² ⊕ B₄·H
#[inline]
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn ghash_4blocks(
    state: uint8x16_t,
    b1: uint8x16_t,
    b2: uint8x16_t,
    b3: uint8x16_t,
    b4: uint8x16_t,
    h_powers: &[uint8x16_t; 8],
) -> uint8x16_t {
    let b1 = vrbitq_u8(b1);
    let b2 = vrbitq_u8(b2);
    let b3 = vrbitq_u8(b3);
    let b4 = vrbitq_u8(b4);

    let h1 = h_powers[0];
    let h2 = h_powers[1];
    let h3 = h_powers[2];
    let h4 = h_powers[3];

    let t0 = clmul_gcm_pmull(state, h4);
    let t1 = clmul_gcm_pmull(b1, h4);
    let t2 = clmul_gcm_pmull(b2, h3);
    let t3 = clmul_gcm_pmull(b3, h2);
    let t4 = clmul_gcm_pmull(b4, h1);

    let left = veorq_u8(t0, t1);
    let right = veorq_u8(veorq_u8(t2, t3), t4);
    veorq_u8(left, right)
}

/// 8-block aggregated GHASH.
///
///   state' = state·H⁸ ⊕ B₁·H⁸ ⊕ B₂·H⁷ ⊕ ... ⊕ B₈·H
#[inline]
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn ghash_8blocks(
    state: uint8x16_t,
    b1: uint8x16_t,
    b2: uint8x16_t,
    b3: uint8x16_t,
    b4: uint8x16_t,
    b5: uint8x16_t,
    b6: uint8x16_t,
    b7: uint8x16_t,
    b8: uint8x16_t,
    h_powers: &[uint8x16_t; 8],
) -> uint8x16_t {
    let b1 = vrbitq_u8(b1);
    let b2 = vrbitq_u8(b2);
    let b3 = vrbitq_u8(b3);
    let b4 = vrbitq_u8(b4);
    let b5 = vrbitq_u8(b5);
    let b6 = vrbitq_u8(b6);
    let b7 = vrbitq_u8(b7);
    let b8 = vrbitq_u8(b8);

    let h8 = h_powers[7];
    let h7 = h_powers[6];
    let h6 = h_powers[5];
    let h5 = h_powers[4];
    let h4 = h_powers[3];
    let h3 = h_powers[2];
    let h2 = h_powers[1];
    let h1 = h_powers[0];

    let t0 = clmul_gcm_pmull(state, h8);
    let t1 = clmul_gcm_pmull(b1, h8);
    let t2 = clmul_gcm_pmull(b2, h7);
    let t3 = clmul_gcm_pmull(b3, h6);
    let t4 = clmul_gcm_pmull(b4, h5);
    let t5 = clmul_gcm_pmull(b5, h4);
    let t6 = clmul_gcm_pmull(b6, h3);
    let t7 = clmul_gcm_pmull(b7, h2);
    let t8 = clmul_gcm_pmull(b8, h1);

    let l0 = veorq_u8(t0, t1);
    let l1 = veorq_u8(t2, t3);
    let l2 = veorq_u8(t4, t5);
    let l3 = veorq_u8(t6, t7);
    let m0 = veorq_u8(l0, l1);
    let m1 = veorq_u8(l2, l3);
    veorq_u8(m0, veorq_u8(m1, t8))
}

/// Multiply two GCM elements using PMULL + 3-step reduction.
#[inline]
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn clmul_gcm_pmull(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    let poly = vld1q_u8([0x87, 0, 0, 0, 0, 0, 0, 0, 0x87, 0, 0, 0, 0, 0, 0, 0].as_ptr());
    let zero = vdupq_n_u8(0);

    let a_p128 = vreinterpretq_p128_u8(a);
    let b_p128 = vreinterpretq_p128_u8(b);
    let poly_p128 = vreinterpretq_p128_u8(poly);
    let poly_p64x2 = vreinterpretq_p64_p128(poly_p128);

    let a_p64x2 = vreinterpretq_p64_p128(a_p128);
    let b_p64x2 = vreinterpretq_p64_p128(b_p128);

    // lo = pmull   v18.1q, a.d[0], b.d[0]
    let lo_p128 = vmull_p64(vgetq_lane_p64::<0>(a_p64x2), vgetq_lane_p64::<0>(b_p64x2));
    // hi = pmull2  v19.1q, a.2d, b.2d
    let hi_p128 = vmull_high_p64(a_p64x2, b_p64x2);

    // a_swap = ext a, a, #8; b_swap = ext b, b, #8
    let a_swap = vextq_u8(a, a, 8);
    let b_swap = vextq_u8(b, b, 8);
    // a_xor = a_swap ^ a; b_xor = b_swap ^ b
    let a_xor = veorq_u8(a_swap, a);
    let b_xor = veorq_u8(b_swap, b);
    // mid = pmull a_xor.d[0], b_xor.d[0]
    let mid_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(a_xor))),
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(b_xor))),
    );

    // mid ^= lo ^ hi
    let mut lo = vreinterpretq_u8_p128(lo_p128);
    let mut hi = vreinterpretq_u8_p128(hi_p128);
    let mid = vreinterpretq_u8_p128(mid_p128);
    let mid = veorq_u8(veorq_u8(mid, lo), hi);

    // ext zero, mid, #8 → lo ^= (mid shifted right by 8)
    lo = veorq_u8(lo, vextq_u8(zero, mid, 8));
    // ext mid, zero, #8 → hi ^= (mid shifted left by 8)
    hi = veorq_u8(hi, vextq_u8(mid, zero, 8));

    // Reduction step
    // r1 = pmull hi.d[0], poly.d[0]
    let r1_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(hi))),
        vgetq_lane_p64::<0>(poly_p64x2),
    );
    // hi_swap = ext hi, hi, #8
    let hi_swap = vextq_u8(hi, hi, 8);
    // r2 = pmull hi_swap.d[0], poly.d[0]
    let r2_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(hi_swap))),
        vgetq_lane_p64::<0>(poly_p64x2),
    );

    let r2 = vreinterpretq_u8_p128(r2_p128);
    // ext zero, r2, #8 → r2_lo
    let r2_lo = vextq_u8(zero, r2, 8);
    // ext r2, zero, #8 → r2_hi
    let r2_hi = vextq_u8(r2, zero, 8);
    // r3 = pmull r2_hi.d[0], poly.d[0]
    let r3_p128 = vmull_p64(
        vgetq_lane_p64::<0>(vreinterpretq_p64_p128(vreinterpretq_p128_u8(r2_hi))),
        vgetq_lane_p64::<0>(poly_p64x2),
    );

    // result = lo ^ r1 ^ r2_lo ^ r3
    veorq_u8(
        veorq_u8(veorq_u8(lo, vreinterpretq_u8_p128(r1_p128)), r2_lo),
        vreinterpretq_u8_p128(r3_p128),
    )
}
