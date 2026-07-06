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

    veorq_u8(t0, veorq_u8(t1, veorq_u8(t2, veorq_u8(t3, t4))))
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

    veorq_u8(
        t0,
        veorq_u8(
            t1,
            veorq_u8(t2, veorq_u8(t3, veorq_u8(t4, veorq_u8(t5, veorq_u8(t6, veorq_u8(t7, t8)))))),
        ),
    )
}

/// Multiply two GCM elements using PMULL + 3-step reduction.
#[inline]
#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn clmul_gcm_pmull(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    let poly = vld1q_u8([0x87, 0, 0, 0, 0, 0, 0, 0, 0x87, 0, 0, 0, 0, 0, 0, 0].as_ptr());
    let result: uint8x16_t;
    core::arch::asm!(
        "movi    v17.16b, #0",
        "pmull   v18.1q, {a:v}.1d, {b:v}.1d",
        "pmull2  v19.1q, {a:v}.2d, {b:v}.2d",
        "ext     v20.16b, {a:v}.16b, {a:v}.16b, #8",
        "ext     v21.16b, {b:v}.16b, {b:v}.16b, #8",
        "eor     v20.16b, v20.16b, {a:v}.16b",
        "eor     v21.16b, v21.16b, {b:v}.16b",
        "pmull   v22.1q, v20.1d, v21.1d",
        "eor     v22.16b, v22.16b, v18.16b",
        "eor     v22.16b, v22.16b, v19.16b",
        "ext     v20.16b, v17.16b, v22.16b, #8",
        "eor     v18.16b, v18.16b, v20.16b",
        "ext     v20.16b, v22.16b, v17.16b, #8",
        "eor     v19.16b, v19.16b, v20.16b",
        "pmull   v22.1q, v19.1d, {poly:v}.1d",
        "ext     v20.16b, v19.16b, v19.16b, #8",
        "pmull   v23.1q, v20.1d, {poly:v}.1d",
        "ext     v20.16b, v17.16b, v23.16b, #8",
        "ext     v21.16b, v23.16b, v17.16b, #8",
        "pmull   v17.1q, v21.1d, {poly:v}.1d",
        "eor     v18.16b, v18.16b, v22.16b",
        "eor     v18.16b, v18.16b, v20.16b",
        "eor     {result:v}.16b, v18.16b, v17.16b",
        a = in(vreg) a,
        b = in(vreg) b,
        poly = in(vreg) poly,
        result = out(vreg) result,
        out("v17") _, out("v18") _, out("v19") _,
        out("v20") _, out("v21") _, out("v22") _, out("v23") _,
        options(nostack),
    );
    result
}
