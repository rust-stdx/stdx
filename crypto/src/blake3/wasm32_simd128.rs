//! WebAssembly `simd128` (4-way) BLAKE3 kernels.
//!
//! Wasm has no runtime feature detection, so this module is only compiled when
//! the target feature is enabled at build time (`-C target-feature=+simd128`).
//! Unlike the AVX2 kernel it also implements single-block compression.

use core::arch::wasm32::*;

use super::{
    platform::{BLOCK_LEN, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, counter_high, counter_low},
    scalar,
};

/// Number of chunks (or parents) hashed in parallel.
pub(crate) const DEGREE: usize = 4;

#[inline(always)]
unsafe fn loadu(src: *const u8) -> v128 {
    // SAFETY: caller guarantees `src` is valid for 16 bytes.
    unsafe { v128_load(src.cast()) }
}

#[inline(always)]
unsafe fn storeu(src: v128, dest: *mut u8) {
    // SAFETY: caller guarantees `dest` is valid for 16 bytes.
    unsafe { v128_store(dest.cast(), src) }
}

#[inline(always)]
fn add(a: v128, b: v128) -> v128 {
    i32x4_add(a, b)
}

#[inline(always)]
fn xor(a: v128, b: v128) -> v128 {
    v128_xor(a, b)
}

#[inline(always)]
fn set1(x: u32) -> v128 {
    i32x4_splat(x as i32)
}

#[inline(always)]
fn set4(a: u32, b: u32, c: u32, d: u32) -> v128 {
    i32x4(a as i32, b as i32, c as i32, d as i32)
}

// rot16 and rot8 use i8x16_shuffle (one instruction) since they are
// byte-aligned; rot12 and rot7 use shift+or.
#[inline(always)]
fn rot16(a: v128) -> v128 {
    i8x16_shuffle::<2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13>(a, a)
}

#[inline(always)]
fn rot12(a: v128) -> v128 {
    v128_or(u32x4_shr(a, 12), u32x4_shl(a, 32 - 12))
}

#[inline(always)]
fn rot8(a: v128) -> v128 {
    i8x16_shuffle::<1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12>(a, a)
}

#[inline(always)]
fn rot7(a: v128) -> v128 {
    v128_or(u32x4_shr(a, 7), u32x4_shl(a, 32 - 7))
}

#[inline(always)]
fn g1(row0: &mut v128, row1: &mut v128, row2: &mut v128, row3: &mut v128, m: v128) {
    *row0 = add(add(*row0, m), *row1);
    *row3 = xor(*row3, *row0);
    *row3 = rot16(*row3);
    *row2 = add(*row2, *row3);
    *row1 = xor(*row1, *row2);
    *row1 = rot12(*row1);
}

#[inline(always)]
fn g2(row0: &mut v128, row1: &mut v128, row2: &mut v128, row3: &mut v128, m: v128) {
    *row0 = add(add(*row0, m), *row1);
    *row3 = xor(*row3, *row0);
    *row3 = rot8(*row3);
    *row2 = add(*row2, *row3);
    *row1 = xor(*row1, *row2);
    *row1 = rot7(*row1);
}

macro_rules! shuffle {
    ($a:expr, $b:expr, $z:expr, $y:expr, $x:expr, $w:expr) => {
        i32x4_shuffle::<{ $w }, { $x }, { $y + 4 }, { $z + 4 }>($a, $b)
    };
}

#[inline(always)]
fn unpacklo_epi64(a: v128, b: v128) -> v128 {
    i64x2_shuffle::<0, 2>(a, b)
}

#[inline(always)]
fn unpackhi_epi64(a: v128, b: v128) -> v128 {
    i64x2_shuffle::<1, 3>(a, b)
}

#[inline(always)]
fn unpacklo_epi32(a: v128, b: v128) -> v128 {
    i32x4_shuffle::<0, 4, 1, 5>(a, b)
}

#[inline(always)]
fn unpackhi_epi32(a: v128, b: v128) -> v128 {
    i32x4_shuffle::<2, 6, 3, 7>(a, b)
}

#[inline(always)]
fn shuffle_epi32<const I3: usize, const I2: usize, const I1: usize, const I0: usize>(a: v128) -> v128 {
    i32x4_shuffle::<I0, I1, I2, I3>(a, a)
}

#[inline(always)]
fn blend_epi16(a: v128, b: v128, imm8: i32) -> v128 {
    let bits = i16x8(0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80);
    let mut mask = i16x8_splat(imm8 as i16);
    mask = v128_and(mask, bits);
    mask = i16x8_eq(mask, bits);
    // Swapped argument order is equivalent to negating the mask.
    v128_bitselect(b, a, mask)
}

// row1 is left unrotated (all message loads below compensate for it); see
// https://github.com/sneves/blake2-avx2/pull/4
#[inline(always)]
fn diagonalize(row0: &mut v128, row2: &mut v128, row3: &mut v128) {
    *row0 = shuffle_epi32::<2, 1, 0, 3>(*row0);
    *row3 = shuffle_epi32::<1, 0, 3, 2>(*row3);
    *row2 = shuffle_epi32::<0, 3, 2, 1>(*row2);
}

#[inline(always)]
fn undiagonalize(row0: &mut v128, row2: &mut v128, row3: &mut v128) {
    *row0 = shuffle_epi32::<0, 3, 2, 1>(*row0);
    *row3 = shuffle_epi32::<1, 0, 3, 2>(*row3);
    *row2 = shuffle_epi32::<2, 1, 0, 3>(*row2);
}

#[inline(always)]
#[allow(clippy::too_many_lines)]
fn compress_pre(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8) -> [v128; 4] {
    // SAFETY: `cv` is [u32; 8] = 32 bytes, loaded as two 16-byte halves.
    let row0 = &mut unsafe { loadu(cv.as_ptr().add(0).cast()) };
    let row1 = &mut unsafe { loadu(cv.as_ptr().add(4).cast()) };
    let row2 = &mut set4(IV[0], IV[1], IV[2], IV[3]);
    let row3 = &mut set4(counter_low(counter), counter_high(counter), block_len as u32, flags as u32);

    // SAFETY: `block` is exactly 64 bytes.
    let mut m0 = unsafe { loadu(block.as_ptr().add(0).cast()) };
    let mut m1 = unsafe { loadu(block.as_ptr().add(16).cast()) };
    let mut m2 = unsafe { loadu(block.as_ptr().add(32).cast()) };
    let mut m3 = unsafe { loadu(block.as_ptr().add(48).cast()) };

    let (mut t0, mut t1, mut t2, mut t3, mut tt);

    // Round 1 permutes the message words into the groups mixed in parallel.
    t0 = shuffle!(m0, m1, 2, 0, 2, 0); //  6  4  2  0
    g1(row0, row1, row2, row3, t0);
    t1 = shuffle!(m0, m1, 3, 1, 3, 1); //  7  5  3  1
    g2(row0, row1, row2, row3, t1);
    diagonalize(row0, row2, row3);
    t2 = shuffle!(m2, m3, 2, 0, 2, 0); // 14 12 10  8
    t2 = shuffle_epi32::<2, 1, 0, 3>(t2); // 12 10  8 14
    g1(row0, row1, row2, row3, t2);
    t3 = shuffle!(m2, m3, 3, 1, 3, 1); // 15 13 11  9
    t3 = shuffle_epi32::<2, 1, 0, 3>(t3); // 13 11  9 15
    g2(row0, row1, row2, row3, t3);
    undiagonalize(row0, row2, row3);
    m0 = t0;
    m1 = t1;
    m2 = t2;
    m3 = t3;

    // Rounds 2..=7: the same permutation applied to the previous round's words.
    for _ in 0..6 {
        t0 = shuffle!(m0, m1, 3, 1, 1, 2);
        t0 = shuffle_epi32::<0, 3, 2, 1>(t0);
        g1(row0, row1, row2, row3, t0);
        t1 = shuffle!(m2, m3, 3, 3, 2, 2);
        tt = shuffle_epi32::<0, 0, 3, 3>(m0);
        t1 = blend_epi16(tt, t1, 0xCC);
        g2(row0, row1, row2, row3, t1);
        diagonalize(row0, row2, row3);
        t2 = unpacklo_epi64(m3, m1);
        tt = blend_epi16(t2, m2, 0xC0);
        t2 = shuffle_epi32::<1, 3, 2, 0>(tt);
        g1(row0, row1, row2, row3, t2);
        t3 = unpackhi_epi32(m1, m3);
        tt = unpacklo_epi32(m2, t3);
        t3 = shuffle_epi32::<0, 1, 3, 2>(tt);
        g2(row0, row1, row2, row3, t3);
        undiagonalize(row0, row2, row3);
        m0 = t0;
        m1 = t1;
        m2 = t2;
        m3 = t3;
    }

    [*row0, *row1, *row2, *row3]
}

/// Compress a single block into `cv` (truncated output).
pub(crate) fn compress_in_place(cv: &mut [u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8) {
    let [row0, row1, row2, row3] = compress_pre(cv, block, block_len, counter, flags);
    // SAFETY: `cv` is [u32; 8] = 32 bytes, stored as two 16-byte halves.
    unsafe {
        storeu(xor(row0, row2), cv.as_mut_ptr().add(0).cast());
        storeu(xor(row1, row3), cv.as_mut_ptr().add(4).cast());
    }
}

/// Compress a single block and return the 64-byte extended output.
pub(crate) fn compress_xof(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8) -> [u8; 64] {
    let [mut row0, mut row1, mut row2, mut row3] = compress_pre(cv, block, block_len, counter, flags);
    row0 = xor(row0, row2);
    row1 = xor(row1, row3);
    // SAFETY: `cv` is [u32; 8] = 32 bytes.
    row2 = xor(row2, unsafe { loadu(cv.as_ptr().add(0).cast()) });
    row3 = xor(row3, unsafe { loadu(cv.as_ptr().add(4).cast()) });
    // SAFETY: four `v128` are exactly 64 bytes.
    unsafe { core::mem::transmute([row0, row1, row2, row3]) }
}

#[inline(always)]
fn round(v: &mut [v128; 16], m: &[v128; 16], r: usize) {
    let s = &MSG_SCHEDULE[r];
    v[0] = add(v[0], m[s[0]]);
    v[1] = add(v[1], m[s[2]]);
    v[2] = add(v[2], m[s[4]]);
    v[3] = add(v[3], m[s[6]]);
    v[0] = add(v[0], v[4]);
    v[1] = add(v[1], v[5]);
    v[2] = add(v[2], v[6]);
    v[3] = add(v[3], v[7]);
    v[12] = xor(v[12], v[0]);
    v[13] = xor(v[13], v[1]);
    v[14] = xor(v[14], v[2]);
    v[15] = xor(v[15], v[3]);
    v[12] = rot16(v[12]);
    v[13] = rot16(v[13]);
    v[14] = rot16(v[14]);
    v[15] = rot16(v[15]);
    v[8] = add(v[8], v[12]);
    v[9] = add(v[9], v[13]);
    v[10] = add(v[10], v[14]);
    v[11] = add(v[11], v[15]);
    v[4] = xor(v[4], v[8]);
    v[5] = xor(v[5], v[9]);
    v[6] = xor(v[6], v[10]);
    v[7] = xor(v[7], v[11]);
    v[4] = rot12(v[4]);
    v[5] = rot12(v[5]);
    v[6] = rot12(v[6]);
    v[7] = rot12(v[7]);
    v[0] = add(v[0], m[s[1]]);
    v[1] = add(v[1], m[s[3]]);
    v[2] = add(v[2], m[s[5]]);
    v[3] = add(v[3], m[s[7]]);
    v[0] = add(v[0], v[4]);
    v[1] = add(v[1], v[5]);
    v[2] = add(v[2], v[6]);
    v[3] = add(v[3], v[7]);
    v[12] = xor(v[12], v[0]);
    v[13] = xor(v[13], v[1]);
    v[14] = xor(v[14], v[2]);
    v[15] = xor(v[15], v[3]);
    v[12] = rot8(v[12]);
    v[13] = rot8(v[13]);
    v[14] = rot8(v[14]);
    v[15] = rot8(v[15]);
    v[8] = add(v[8], v[12]);
    v[9] = add(v[9], v[13]);
    v[10] = add(v[10], v[14]);
    v[11] = add(v[11], v[15]);
    v[4] = xor(v[4], v[8]);
    v[5] = xor(v[5], v[9]);
    v[6] = xor(v[6], v[10]);
    v[7] = xor(v[7], v[11]);
    v[4] = rot7(v[4]);
    v[5] = rot7(v[5]);
    v[6] = rot7(v[6]);
    v[7] = rot7(v[7]);

    v[0] = add(v[0], m[s[8]]);
    v[1] = add(v[1], m[s[10]]);
    v[2] = add(v[2], m[s[12]]);
    v[3] = add(v[3], m[s[14]]);
    v[0] = add(v[0], v[5]);
    v[1] = add(v[1], v[6]);
    v[2] = add(v[2], v[7]);
    v[3] = add(v[3], v[4]);
    v[15] = xor(v[15], v[0]);
    v[12] = xor(v[12], v[1]);
    v[13] = xor(v[13], v[2]);
    v[14] = xor(v[14], v[3]);
    v[15] = rot16(v[15]);
    v[12] = rot16(v[12]);
    v[13] = rot16(v[13]);
    v[14] = rot16(v[14]);
    v[10] = add(v[10], v[15]);
    v[11] = add(v[11], v[12]);
    v[8] = add(v[8], v[13]);
    v[9] = add(v[9], v[14]);
    v[5] = xor(v[5], v[10]);
    v[6] = xor(v[6], v[11]);
    v[7] = xor(v[7], v[8]);
    v[4] = xor(v[4], v[9]);
    v[5] = rot12(v[5]);
    v[6] = rot12(v[6]);
    v[7] = rot12(v[7]);
    v[4] = rot12(v[4]);
    v[0] = add(v[0], m[s[9]]);
    v[1] = add(v[1], m[s[11]]);
    v[2] = add(v[2], m[s[13]]);
    v[3] = add(v[3], m[s[15]]);
    v[0] = add(v[0], v[5]);
    v[1] = add(v[1], v[6]);
    v[2] = add(v[2], v[7]);
    v[3] = add(v[3], v[4]);
    v[15] = xor(v[15], v[0]);
    v[12] = xor(v[12], v[1]);
    v[13] = xor(v[13], v[2]);
    v[14] = xor(v[14], v[3]);
    v[15] = rot8(v[15]);
    v[12] = rot8(v[12]);
    v[13] = rot8(v[13]);
    v[14] = rot8(v[14]);
    v[10] = add(v[10], v[15]);
    v[11] = add(v[11], v[12]);
    v[8] = add(v[8], v[13]);
    v[9] = add(v[9], v[14]);
    v[5] = xor(v[5], v[10]);
    v[6] = xor(v[6], v[11]);
    v[7] = xor(v[7], v[8]);
    v[4] = xor(v[4], v[9]);
    v[5] = rot7(v[5]);
    v[6] = rot7(v[6]);
    v[7] = rot7(v[7]);
    v[4] = rot7(v[4]);
}

#[inline(always)]
fn transpose_vecs(vecs: &mut [v128; DEGREE]) {
    let ab_01 = unpacklo_epi32(vecs[0], vecs[1]);
    let ab_23 = unpackhi_epi32(vecs[0], vecs[1]);
    let cd_01 = unpacklo_epi32(vecs[2], vecs[3]);
    let cd_23 = unpackhi_epi32(vecs[2], vecs[3]);

    let abcd_0 = unpacklo_epi64(ab_01, cd_01);
    let abcd_1 = unpackhi_epi64(ab_01, cd_01);
    let abcd_2 = unpacklo_epi64(ab_23, cd_23);
    let abcd_3 = unpackhi_epi64(ab_23, cd_23);

    vecs[0] = abcd_0;
    vecs[1] = abcd_1;
    vecs[2] = abcd_2;
    vecs[3] = abcd_3;
}

#[inline(always)]
unsafe fn transpose_msg_vecs(inputs: &[*const u8; DEGREE], block_offset: usize) -> [v128; 16] {
    // SAFETY: each input is a whole block and `block_offset` is a multiple of 16.
    let mut vecs = unsafe {
        [
            loadu(inputs[0].add(block_offset)),
            loadu(inputs[1].add(block_offset)),
            loadu(inputs[2].add(block_offset)),
            loadu(inputs[3].add(block_offset)),
            loadu(inputs[0].add(block_offset + 16)),
            loadu(inputs[1].add(block_offset + 16)),
            loadu(inputs[2].add(block_offset + 16)),
            loadu(inputs[3].add(block_offset + 16)),
            loadu(inputs[0].add(block_offset + 32)),
            loadu(inputs[1].add(block_offset + 32)),
            loadu(inputs[2].add(block_offset + 32)),
            loadu(inputs[3].add(block_offset + 32)),
            loadu(inputs[0].add(block_offset + 48)),
            loadu(inputs[1].add(block_offset + 48)),
            loadu(inputs[2].add(block_offset + 48)),
            loadu(inputs[3].add(block_offset + 48)),
        ]
    };
    let (square0, rest) = vecs.split_at_mut(DEGREE);
    let (square1, rest) = rest.split_at_mut(DEGREE);
    let (square2, square3) = rest.split_at_mut(DEGREE);
    transpose_vecs(square0.try_into().unwrap());
    transpose_vecs(square1.try_into().unwrap());
    transpose_vecs(square2.try_into().unwrap());
    transpose_vecs(square3.try_into().unwrap());
    vecs
}

#[inline(always)]
fn load_counters(counter: u64, increment_counter: IncrementCounter) -> (v128, v128) {
    let mask = if increment_counter.yes() { !0 } else { 0 };
    (
        set4(
            counter_low(counter),
            counter_low(counter + (mask & 1)),
            counter_low(counter + (mask & 2)),
            counter_low(counter + (mask & 3)),
        ),
        set4(
            counter_high(counter),
            counter_high(counter + (mask & 1)),
            counter_high(counter + (mask & 2)),
            counter_high(counter + (mask & 3)),
        ),
    )
}

/// Hash exactly [`DEGREE`] inputs of the same number of blocks.
fn hash4(
    inputs: &[*const u8; DEGREE],
    blocks: usize,
    key: &[u32; 8],
    counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    out: &mut [u8],
) {
    let mut h_vecs = [
        set1(key[0]),
        set1(key[1]),
        set1(key[2]),
        set1(key[3]),
        set1(key[4]),
        set1(key[5]),
        set1(key[6]),
        set1(key[7]),
    ];
    let (counter_low_vec, counter_high_vec) = load_counters(counter, increment_counter);
    let mut block_flags = flags | flags_start;

    for block in 0..blocks {
        if block + 1 == blocks {
            block_flags |= flags_end;
        }
        let block_len_vec = set1(BLOCK_LEN as u32);
        let block_flags_vec = set1(block_flags as u32);
        // SAFETY: `inputs` each hold `blocks * BLOCK_LEN` bytes.
        let msg_vecs = unsafe { transpose_msg_vecs(inputs, block * BLOCK_LEN) };

        let mut v = [
            h_vecs[0],
            h_vecs[1],
            h_vecs[2],
            h_vecs[3],
            h_vecs[4],
            h_vecs[5],
            h_vecs[6],
            h_vecs[7],
            set1(IV[0]),
            set1(IV[1]),
            set1(IV[2]),
            set1(IV[3]),
            counter_low_vec,
            counter_high_vec,
            block_len_vec,
            block_flags_vec,
        ];
        round(&mut v, &msg_vecs, 0);
        round(&mut v, &msg_vecs, 1);
        round(&mut v, &msg_vecs, 2);
        round(&mut v, &msg_vecs, 3);
        round(&mut v, &msg_vecs, 4);
        round(&mut v, &msg_vecs, 5);
        round(&mut v, &msg_vecs, 6);
        h_vecs[0] = xor(v[0], v[8]);
        h_vecs[1] = xor(v[1], v[9]);
        h_vecs[2] = xor(v[2], v[10]);
        h_vecs[3] = xor(v[3], v[11]);
        h_vecs[4] = xor(v[4], v[12]);
        h_vecs[5] = xor(v[5], v[13]);
        h_vecs[6] = xor(v[6], v[14]);
        h_vecs[7] = xor(v[7], v[15]);

        block_flags = flags;
    }

    let (square0, square1) = h_vecs.split_at_mut(DEGREE);
    transpose_vecs(square0.try_into().unwrap());
    transpose_vecs(square1.try_into().unwrap());
    // The first four vectors hold the first half of each output and the second
    // four hold the second half.
    let ptr = out.as_mut_ptr();
    unsafe {
        storeu(h_vecs[0], ptr.add(0));
        storeu(h_vecs[4], ptr.add(16));
        storeu(h_vecs[1], ptr.add(32));
        storeu(h_vecs[5], ptr.add(48));
        storeu(h_vecs[2], ptr.add(64));
        storeu(h_vecs[6], ptr.add(80));
        storeu(h_vecs[3], ptr.add(96));
        storeu(h_vecs[7], ptr.add(112));
    }
}

/// Hash several equal-length inputs.
///
/// Every input must be a whole number of 64-byte blocks with the same block
/// count; `out` must hold `inputs.len() * OUT_LEN` bytes.
pub(crate) fn hash_many(
    mut inputs: &[&[u8]],
    key: &[u32; 8],
    mut counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    mut out: &mut [u8],
) {
    while inputs.len() >= DEGREE && out.len() >= DEGREE * OUT_LEN {
        let blocks = inputs[0].len() / BLOCK_LEN;
        let input_ptrs: [*const u8; DEGREE] = [
            inputs[0].as_ptr(),
            inputs[1].as_ptr(),
            inputs[2].as_ptr(),
            inputs[3].as_ptr(),
        ];
        hash4(
            &input_ptrs,
            blocks,
            key,
            counter,
            increment_counter,
            flags,
            flags_start,
            flags_end,
            &mut out[..DEGREE * OUT_LEN],
        );
        if increment_counter.yes() {
            counter = counter.wrapping_add(DEGREE as u64);
        }
        inputs = &inputs[DEGREE..];
        out = &mut out[DEGREE * OUT_LEN..];
    }

    scalar::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_roundtrips() {
        let mut matrix = [[0u32; DEGREE]; DEGREE];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = (i * DEGREE + j) as u32;
            }
        }

        unsafe {
            let mut vecs: [v128; DEGREE] = core::mem::transmute(matrix);
            transpose_vecs(&mut vecs);
            matrix = core::mem::transmute(vecs);
        }

        for (i, row) in matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                assert_eq!(*cell, (j * DEGREE + i) as u32);
            }
        }
    }
}
