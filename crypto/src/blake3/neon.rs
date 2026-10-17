//! NEON (4-way) BLAKE3 kernel for `aarch64`.
//!
//! NEON is mandatory on AArch64, so no runtime detection is needed. Only the
//! wide `hash_many` and `xof_many` paths are implemented: a single 64-byte
//! compression fills a 128-bit register but does not parallelize, and the
//! portable kernel is just as fast there.

use core::arch::aarch64::*;

use super::{
    platform::{BLOCK_LEN, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, counter_high, counter_low},
    scalar,
};

/// Number of chunks (or parents) hashed in parallel.
pub(crate) const DEGREE: usize = 4;

#[inline(always)]
unsafe fn loadu(src: *const u8) -> uint32x4_t {
    // SAFETY: caller guarantees `src` is valid for 16 bytes.
    unsafe { vreinterpretq_u32_u8(vld1q_u8(src)) }
}

#[inline(always)]
unsafe fn storeu(src: uint32x4_t, dest: *mut u8) {
    // SAFETY: caller guarantees `dest` is valid for 16 bytes.
    unsafe { vst1q_u8(dest, vreinterpretq_u8_u32(src)) }
}

#[inline(always)]
unsafe fn add(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    unsafe { vaddq_u32(a, b) }
}

#[inline(always)]
unsafe fn xor(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    unsafe { veorq_u32(a, b) }
}

#[inline(always)]
unsafe fn set1(x: u32) -> uint32x4_t {
    unsafe { vdupq_n_u32(x) }
}

#[inline(always)]
unsafe fn set4(a: u32, b: u32, c: u32, d: u32) -> uint32x4_t {
    let array = [a, b, c, d];
    unsafe { vld1q_u32(array.as_ptr()) }
}

// Byte-aligned rotations use the cheaper reversing/shift-insert forms; the
// plain shift+or version is measurably slower (see BLAKE3 PR #319).
#[inline(always)]
unsafe fn rot16(x: uint32x4_t) -> uint32x4_t {
    unsafe { vreinterpretq_u32_u16(vrev32q_u16(vreinterpretq_u16_u32(x))) }
}

#[inline(always)]
unsafe fn rot12(x: uint32x4_t) -> uint32x4_t {
    unsafe { vsriq_n_u32(vshlq_n_u32(x, 32 - 12), x, 12) }
}

#[inline(always)]
unsafe fn rot8(x: uint32x4_t) -> uint32x4_t {
    // Byte-shuffle [1,2,3,0] within each 32-bit lane (one `tbl` instruction instead of shift+insert).
    #[rustfmt::skip]
    let idx: uint8x16_t = unsafe {
        core::mem::transmute([1u8, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12])
    };
    unsafe { vreinterpretq_u32_u8(vqtbl1q_u8(vreinterpretq_u8_u32(x), idx)) }
}

#[inline(always)]
unsafe fn rot7(x: uint32x4_t) -> uint32x4_t {
    unsafe { vsriq_n_u32(vshlq_n_u32(x, 32 - 7), x, 7) }
}

#[inline(always)]
#[allow(clippy::too_many_lines)]
unsafe fn round(v: &mut [uint32x4_t; 16], m: &[uint32x4_t; 16], r: usize) {
    unsafe {
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
}

/// Transpose a 4x4 matrix of 32-bit words held in four vectors.
#[inline(always)]
unsafe fn transpose_vecs(vecs: &mut [uint32x4_t; DEGREE]) {
    unsafe {
        let rows01 = vtrnq_u32(vecs[0], vecs[1]);
        let rows23 = vtrnq_u32(vecs[2], vecs[3]);

        vecs[0] = vcombine_u32(vget_low_u32(rows01.0), vget_low_u32(rows23.0));
        vecs[1] = vcombine_u32(vget_low_u32(rows01.1), vget_low_u32(rows23.1));
        vecs[2] = vcombine_u32(vget_high_u32(rows01.0), vget_high_u32(rows23.0));
        vecs[3] = vcombine_u32(vget_high_u32(rows01.1), vget_high_u32(rows23.1));
    }
}

#[inline(always)]
unsafe fn transpose_msg_vecs(inputs: &[*const u8; DEGREE], block_offset: usize) -> [uint32x4_t; 16] {
    unsafe {
        let mut vecs = [
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
        ];
        let (square0, rest) = vecs.split_at_mut(DEGREE);
        let (square1, rest) = rest.split_at_mut(DEGREE);
        let (square2, square3) = rest.split_at_mut(DEGREE);
        transpose_vecs(square0.try_into().unwrap());
        transpose_vecs(square1.try_into().unwrap());
        transpose_vecs(square2.try_into().unwrap());
        transpose_vecs(square3.try_into().unwrap());
        vecs
    }
}

#[inline(always)]
unsafe fn load_counters(counter: u64, increment_counter: IncrementCounter) -> (uint32x4_t, uint32x4_t) {
    let mask = if increment_counter.yes() { !0 } else { 0 };
    unsafe {
        (
            set4(
                counter_low(counter),
                counter_low(counter.wrapping_add(mask & 1)),
                counter_low(counter.wrapping_add(mask & 2)),
                counter_low(counter.wrapping_add(mask & 3)),
            ),
            set4(
                counter_high(counter),
                counter_high(counter.wrapping_add(mask & 1)),
                counter_high(counter.wrapping_add(mask & 2)),
                counter_high(counter.wrapping_add(mask & 3)),
            ),
        )
    }
}

/// Hash exactly [`DEGREE`] inputs of the same number of blocks.
///
/// # Safety
/// NEON must be available (always true on AArch64), and each input must have
/// the same whole number of 64-byte blocks (at least one).
#[target_feature(enable = "neon")]
unsafe fn hash4(
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
    unsafe {
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
            let msg_vecs = transpose_msg_vecs(inputs, block * BLOCK_LEN);

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
        // The first four vectors now hold the first half of each output and the
        // second four hold the second half.
        let ptr = out.as_mut_ptr();
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

/// Write exactly [`DEGREE`] extended-output blocks of a single root node.
///
/// Lane `i` compresses the same message block with counter `counter + i` and
/// writes the resulting 64-byte output block.
///
/// # Safety
/// NEON must be available (always true on AArch64).
#[target_feature(enable = "neon")]
unsafe fn xof4(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8, out: &mut [u8]) {
    unsafe {
        let block_words = scalar::words_from_le_bytes_64(block);
        let mut msg_vecs = [set1(0); 16];
        for (vec, word) in msg_vecs.iter_mut().zip(block_words) {
            *vec = set1(word);
        }

        let h_vecs = [
            set1(cv[0]),
            set1(cv[1]),
            set1(cv[2]),
            set1(cv[3]),
            set1(cv[4]),
            set1(cv[5]),
            set1(cv[6]),
            set1(cv[7]),
        ];
        let (counter_low_vec, counter_high_vec) = load_counters(counter, IncrementCounter::Yes);

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
            set1(block_len as u32),
            set1(flags as u32),
        ];
        round(&mut v, &msg_vecs, 0);
        round(&mut v, &msg_vecs, 1);
        round(&mut v, &msg_vecs, 2);
        round(&mut v, &msg_vecs, 3);
        round(&mut v, &msg_vecs, 4);
        round(&mut v, &msg_vecs, 5);
        round(&mut v, &msg_vecs, 6);

        for i in 0..8 {
            v[i] = xor(v[i], v[i + 8]);
            v[i + 8] = xor(v[i + 8], h_vecs[i]);
        }

        // Four 4x4 transposes, one per group of four words: after them,
        // `v[4 * group + i]` holds words `4 * group .. 4 * group + 4` of output
        // block `i`.
        for group in v.as_chunks_mut::<DEGREE>().0 {
            transpose_vecs(group);
        }
        let ptr = out.as_mut_ptr();
        for i in 0..DEGREE {
            storeu(v[i], ptr.add(i * BLOCK_LEN));
            storeu(v[i + DEGREE], ptr.add(i * BLOCK_LEN + 16));
            storeu(v[i + 2 * DEGREE], ptr.add(i * BLOCK_LEN + 32));
            storeu(v[i + 3 * DEGREE], ptr.add(i * BLOCK_LEN + 48));
        }
    }
}

/// Hash several equal-length inputs.
///
/// Every input must be a whole number of 64-byte blocks with the same block
/// count; `out` must hold `inputs.len() * OUT_LEN` bytes.
///
/// # Safety
/// NEON must be available on the running CPU (always true on AArch64).
#[target_feature(enable = "neon")]
pub(crate) unsafe fn hash_many(
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
        // SAFETY: NEON is available and the four inputs each have
        // `blocks * BLOCK_LEN` readable bytes.
        unsafe {
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
        }
        if increment_counter.yes() {
            counter = counter.wrapping_add(DEGREE as u64);
        }
        inputs = &inputs[DEGREE..];
        out = &mut out[DEGREE * OUT_LEN..];
    }

    scalar::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out);
}

/// Fill `out` with consecutive extendable-output blocks.
///
/// `out` must be a whole number of 64-byte blocks and may be empty, in which
/// case this is a no-op.
///
/// # Safety
/// NEON must be available (always true on AArch64).
#[target_feature(enable = "neon")]
pub(crate) unsafe fn xof_many(
    cv: &[u32; 8],
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    mut counter: u64,
    flags: u8,
    mut out: &mut [u8],
) {
    debug_assert_eq!(out.len() % BLOCK_LEN, 0, "out must be whole blocks");
    while out.len() >= DEGREE * BLOCK_LEN {
        // SAFETY: NEON is available and `out` has DEGREE whole blocks.
        unsafe { xof4(cv, block, block_len, counter, flags, &mut out[..DEGREE * BLOCK_LEN]) };
        counter = counter.wrapping_add(DEGREE as u64);
        out = &mut out[DEGREE * BLOCK_LEN..];
    }

    scalar::xof_many(cv, block, block_len, counter, flags, out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_roundtrips() {
        #[target_feature(enable = "neon")]
        unsafe fn transpose_wrapper(vecs: &mut [uint32x4_t; DEGREE]) {
            unsafe { transpose_vecs(vecs) };
        }

        let mut matrix = [[0u32; DEGREE]; DEGREE];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = (i * DEGREE + j) as u32;
            }
        }

        unsafe {
            let mut vecs: [uint32x4_t; DEGREE] = core::mem::transmute(matrix);
            transpose_wrapper(&mut vecs);
            matrix = core::mem::transmute::<[std::arch::aarch64::uint32x4_t; 4], [[u32; 4]; 4]>(vecs);
        }

        for (i, row) in matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                assert_eq!(*cell, (j * DEGREE + i) as u32);
            }
        }
    }
}
