#![allow(unsafe_op_in_unsafe_fn)]

//! AArch64 NEON implementation of Argon2's compression function `G`.
//!
//! The 64-bit words are processed two at a time in `uint64x2_t` lanes, which
//! halves the number of rotation/multiply/add instructions compared to the
//! scalar kernel. The `NeonSha3` variant is compiled with the `sha3` target
//! feature so LLVM can fuse the rotate-xor steps into a single `xar`
//! instruction.

use core::arch::aarch64::*;

use super::Block;

/// Rotate each 64-bit lane right by 32 bits (swap the two 32-bit halves).
#[inline(always)]
unsafe fn rotr32(x: uint64x2_t) -> uint64x2_t {
    vreinterpretq_u64_u32(vrev64q_u32(vreinterpretq_u32_u64(x)))
}

/// Rotate each 64-bit lane right by 16 bits.
#[inline(always)]
unsafe fn rotr16(x: uint64x2_t) -> uint64x2_t {
    vorrq_u64(vshrq_n_u64::<16>(x), vshlq_n_u64::<48>(x))
}

/// Rotate each 64-bit lane right by 24 bits.
#[inline(always)]
unsafe fn rotr24(x: uint64x2_t) -> uint64x2_t {
    vorrq_u64(vshrq_n_u64::<24>(x), vshlq_n_u64::<40>(x))
}

/// Rotate each 64-bit lane right by 63 bits.
#[inline(always)]
unsafe fn rotr63(x: uint64x2_t) -> uint64x2_t {
    vorrq_u64(vshrq_n_u64::<63>(x), vshlq_n_u64::<1>(x))
}

/// Argon2's `fBlaMka`: `a + b + 2 * low32(a) * low32(b)`, lane-wise.
#[inline(always)]
unsafe fn fblamka(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    // Gather the low 32 bits of each 64-bit lane into a 32-bit vector and use
    // the widening multiply, which is the closest NEON analogue of x86's
    // `_mm_mul_epu32`.
    let alo = vuzp1q_u32(vreinterpretq_u32_u64(a), vreinterpretq_u32_u64(a));
    let blo = vuzp1q_u32(vreinterpretq_u32_u64(b), vreinterpretq_u32_u64(b));
    let prod = vmull_u32(vget_low_u32(alo), vget_low_u32(blo));
    vaddq_u64(a, vaddq_u64(b, vshlq_n_u64::<1>(prod)))
}

/// The GB mixing function applied lane-wise to four pairs of words.
macro_rules! gbv {
    ($a:ident, $b:ident, $c:ident, $d:ident) => {{
        $a = fblamka($a, $b);
        $d = rotr32(veorq_u64($d, $a));
        $c = fblamka($c, $d);
        $b = rotr24(veorq_u64($b, $c));
        $a = fblamka($a, $b);
        $d = rotr16(veorq_u64($d, $a));
        $c = fblamka($c, $d);
        $b = rotr63(veorq_u64($b, $c));
    }};
}

/// Apply the permutation P to the 16 words stored at the eight pair offsets
/// `o0..o7` (each offset points at two adjacent words).
macro_rules! round_at {
    ($p:expr, $o0:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr) => {{
        let p = $p;
        let mut a0 = vld1q_u64(p.add($o0));
        let mut a1 = vld1q_u64(p.add($o1));
        let mut b0 = vld1q_u64(p.add($o2));
        let mut b1 = vld1q_u64(p.add($o3));
        let mut c0 = vld1q_u64(p.add($o4));
        let mut c1 = vld1q_u64(p.add($o5));
        let mut d0 = vld1q_u64(p.add($o6));
        let mut d1 = vld1q_u64(p.add($o7));

        // Column step.
        gbv!(a0, b0, c0, d0);
        gbv!(a1, b1, c1, d1);

        // Diagonalize: rotate B left by 1, swap the halves of C, rotate D
        // left by 3 (relative to the 16-word round).
        let nb0 = vextq_u64::<1>(b0, b1);
        let nb1 = vextq_u64::<1>(b1, b0);
        let nc0 = c1;
        let nc1 = c0;
        let nd0 = vextq_u64::<1>(d1, d0);
        let nd1 = vextq_u64::<1>(d0, d1);
        b0 = nb0;
        b1 = nb1;
        c0 = nc0;
        c1 = nc1;
        d0 = nd0;
        d1 = nd1;

        // Diagonal step.
        gbv!(a0, b0, c0, d0);
        gbv!(a1, b1, c1, d1);

        // Undiagonalize (inverse of the swaps above).
        let ub0 = vextq_u64::<1>(b1, b0);
        let ub1 = vextq_u64::<1>(b0, b1);
        let uc0 = c1;
        let uc1 = c0;
        let ud0 = vextq_u64::<1>(d0, d1);
        let ud1 = vextq_u64::<1>(d1, d0);
        b0 = ub0;
        b1 = ub1;
        c0 = uc0;
        c1 = uc1;
        d0 = ud0;
        d1 = ud1;

        vst1q_u64(p.add($o0), a0);
        vst1q_u64(p.add($o1), a1);
        vst1q_u64(p.add($o2), b0);
        vst1q_u64(p.add($o3), b1);
        vst1q_u64(p.add($o4), c0);
        vst1q_u64(p.add($o5), c1);
        vst1q_u64(p.add($o6), d0);
        vst1q_u64(p.add($o7), d1);
    }};
}

/// Shared body of the NEON compression function. Kept separate from the two
/// entry points so it can be inlined into both the baseline and the
/// SHA3-enabled variants.
#[inline(always)]
unsafe fn fill_block_impl(prev: &Block, reference: &Block, cur: *mut Block, with_xor: bool) {
    let mut r = [0u64; 128];
    let mut xy = [0u64; 128];

    let prev_ptr = prev.v.as_ptr();
    let reference_ptr = reference.v.as_ptr();
    let r_ptr = r.as_mut_ptr();
    let xy_ptr = xy.as_mut_ptr();

    if with_xor {
        let cur_ptr = (*cur).v.as_ptr();
        for i in (0..128).step_by(2) {
            let x = veorq_u64(vld1q_u64(prev_ptr.add(i)), vld1q_u64(reference_ptr.add(i)));
            vst1q_u64(r_ptr.add(i), x);
            vst1q_u64(xy_ptr.add(i), veorq_u64(x, vld1q_u64(cur_ptr.add(i))));
        }
    } else {
        for i in (0..128).step_by(2) {
            let x = veorq_u64(vld1q_u64(prev_ptr.add(i)), vld1q_u64(reference_ptr.add(i)));
            vst1q_u64(r_ptr.add(i), x);
            vst1q_u64(xy_ptr.add(i), x);
        }
    }

    // Rows: 16 contiguous words per round.
    let mut b = 0usize;
    while b < 128 {
        round_at!(r_ptr, b, b + 2, b + 4, b + 6, b + 8, b + 10, b + 12, b + 14);
        b += 16;
    }

    // Columns: the same eight strides of 16 words.
    let mut b = 0usize;
    while b < 16 {
        round_at!(r_ptr, b, b + 16, b + 32, b + 48, b + 64, b + 80, b + 96, b + 112);
        b += 2;
    }

    let cur_ptr = (*cur).v.as_mut_ptr();
    for i in (0..128).step_by(2) {
        vst1q_u64(cur_ptr.add(i), veorq_u64(vld1q_u64(r_ptr.add(i)), vld1q_u64(xy_ptr.add(i))));
    }
}

/// Baseline NEON kernel (always available on `aarch64`).
#[inline(always)]
pub(crate) unsafe fn fill_block(prev: &Block, reference: &Block, cur: *mut Block, with_xor: bool) {
    fill_block_impl(prev, reference, cur, with_xor)
}

/// NEON kernel compiled with the `sha3` extension, which lets LLVM fuse the
/// rotate-xor steps into `xar`. Only call this when the CPU advertises `sha3`.
#[target_feature(enable = "sha3")]
pub(crate) unsafe fn fill_block_sha3(prev: &Block, reference: &Block, cur: *mut Block, with_xor: bool) {
    fill_block_impl(prev, reference, cur, with_xor)
}
