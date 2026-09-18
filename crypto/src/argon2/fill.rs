//! Portable scalar implementation of Argon2's compression function `G`.
//!
//! This is the reference-shaped kernel: it keeps the 16 words of each
//! permutation in registers and operates on `u64`s. It is the fallback for
//! targets without a SIMD backend, and is also used for the small,
//! data-independent address-generation blocks.

use super::Block;

/// The GB mixing function for Argon2.
///
/// Unlike BLAKE2b's G, this adds a 32x32->64 multiplication for extra
/// sequential memory hardness.
macro_rules! gb {
    ($a:ident, $b:ident, $c:ident, $d:ident) => {{
        $a = $a
            .wrapping_add($b)
            .wrapping_add(2u64.wrapping_mul(($a as u32 as u64).wrapping_mul($b as u32 as u64)));
        $d = ($d ^ $a).rotate_right(32);
        $c = $c
            .wrapping_add($d)
            .wrapping_add(2u64.wrapping_mul(($c as u32 as u64).wrapping_mul($d as u32 as u64)));
        $b = ($b ^ $c).rotate_right(24);
        $a = $a
            .wrapping_add($b)
            .wrapping_add(2u64.wrapping_mul(($a as u32 as u64).wrapping_mul($b as u32 as u64)));
        $d = ($d ^ $a).rotate_right(16);
        $c = $c
            .wrapping_add($d)
            .wrapping_add(2u64.wrapping_mul(($c as u32 as u64).wrapping_mul($d as u32 as u64)));
        $b = ($b ^ $c).rotate_right(63);
    }};
}

/// Apply the permutation P to the 16 words of `$b` addressed by the given
/// indices, keeping the words in registers between mixing steps.
macro_rules! round_nomsg {
    ($b:expr, $i0:expr, $i1:expr, $i2:expr, $i3:expr, $i4:expr, $i5:expr, $i6:expr, $i7:expr, $i8:expr, $i9:expr,
     $i10:expr, $i11:expr, $i12:expr, $i13:expr, $i14:expr, $i15:expr) => {{
        let mut v0 = $b[$i0];
        let mut v1 = $b[$i1];
        let mut v2 = $b[$i2];
        let mut v3 = $b[$i3];
        let mut v4 = $b[$i4];
        let mut v5 = $b[$i5];
        let mut v6 = $b[$i6];
        let mut v7 = $b[$i7];
        let mut v8 = $b[$i8];
        let mut v9 = $b[$i9];
        let mut v10 = $b[$i10];
        let mut v11 = $b[$i11];
        let mut v12 = $b[$i12];
        let mut v13 = $b[$i13];
        let mut v14 = $b[$i14];
        let mut v15 = $b[$i15];

        gb!(v0, v4, v8, v12);
        gb!(v1, v5, v9, v13);
        gb!(v2, v6, v10, v14);
        gb!(v3, v7, v11, v15);
        gb!(v0, v5, v10, v15);
        gb!(v1, v6, v11, v12);
        gb!(v2, v7, v8, v13);
        gb!(v3, v4, v9, v14);

        $b[$i0] = v0;
        $b[$i1] = v1;
        $b[$i2] = v2;
        $b[$i3] = v3;
        $b[$i4] = v4;
        $b[$i5] = v5;
        $b[$i6] = v6;
        $b[$i7] = v7;
        $b[$i8] = v8;
        $b[$i9] = v9;
        $b[$i10] = v10;
        $b[$i11] = v11;
        $b[$i12] = v12;
        $b[$i13] = v13;
        $b[$i14] = v14;
        $b[$i15] = v15;
    }};
}

/// Compression function G(X, Y) -> Z XOR R, written in place into `cur`.
///
/// When `with_xor` is set (any pass after the first), the previous contents of
/// `cur` are XORed in, as required by the Argon2 specification.
///
/// # Safety
///
/// `cur` must be valid for writes. When `with_xor` is set it must also be
/// initialized (which is the case for every pass after the first).
#[inline(always)]
pub(crate) unsafe fn fill_block(prev: &Block, reference: &Block, cur: *mut Block, with_xor: bool) {
    // R = X XOR Y
    let mut r = Block::zero();
    for i in 0..128 {
        r.v[i] = prev.v[i] ^ reference.v[i];
    }

    // T = R, optionally XORed with the block being overwritten.
    let mut t = r.clone();
    if with_xor {
        // SAFETY: with_xor is only used from the second pass onwards, where the
        // destination block has already been written.
        t.xor_with(unsafe { &*cur });
    }

    // Apply P to each of the 8 rows of 16 words.
    for i in 0..8 {
        let b = i * 16;
        round_nomsg!(
            r.v,
            b,
            b + 1,
            b + 2,
            b + 3,
            b + 4,
            b + 5,
            b + 6,
            b + 7,
            b + 8,
            b + 9,
            b + 10,
            b + 11,
            b + 12,
            b + 13,
            b + 14,
            b + 15
        );
    }

    // Apply P to each of the 8 columns.
    for i in 0..8 {
        let b = 2 * i;
        round_nomsg!(
            r.v,
            b,
            b + 1,
            b + 16,
            b + 17,
            b + 32,
            b + 33,
            b + 48,
            b + 49,
            b + 64,
            b + 65,
            b + 80,
            b + 81,
            b + 96,
            b + 97,
            b + 112,
            b + 113
        );
    }

    // Z = T XOR R
    for i in 0..128 {
        // SAFETY: `cur` is valid for writes.
        unsafe { (*cur).v[i] = t.v[i] ^ r.v[i] };
    }
}

/// Safe wrapper around [`fill_block`] for initialized stack blocks.
///
/// Used for the small address-generation blocks, which never touch the arena.
#[inline(always)]
pub(crate) fn fill_block_ref(prev: &Block, reference: &Block, cur: &mut Block, with_xor: bool) {
    // SAFETY: `cur` points at a valid, initialized `Block`, and the call does
    // not alias `prev` or `reference`.
    unsafe { fill_block(prev, reference, cur, with_xor) }
}

/// Permutation P applied to a standalone 16-word vector (used by tests).
#[cfg(test)]
pub(crate) fn permutation_p(v: &mut [u64; 16]) {
    round_nomsg!(v, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
}

/// Compression G(X, Y) computed into a fresh block (used by tests).
#[cfg(test)]
pub(crate) fn compress(x: &Block, y: &Block) -> Block {
    let mut out = Block::zero();
    // SAFETY: `out` is an initialized stack value.
    unsafe { fill_block(x, y, &mut out, false) };
    out
}
