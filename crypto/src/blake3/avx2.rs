//! AVX2 (8-way) BLAKE3 kernels for `x86_64`.
//!
//! The wide `hash_many` path hashes up to eight chunks in parallel, one lane
//! per chunk. The chaining values are kept transposed in vector registers
//! across the blocks of a chunk, which is where most of the throughput on
//! large inputs comes from.
//!
//! The `xof_many` path uses the same eight lanes, but one per extendable-output
//! block (sharing the message and varying the counter), so several 64-byte
//! output blocks are produced per call.
//!
//! Single-block compression is delegated to the portable kernel: a lone
//! compression only fills a 128-bit vector register and gains little from
//! AVX2.

use core::arch::x86_64::*;

use super::{
    platform::{BLOCK_LEN, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, counter_high, counter_low},
    scalar,
};

/// Number of chunks (or parents) hashed in parallel.
pub(crate) const DEGREE: usize = 8;

#[inline(always)]
unsafe fn loadu(src: *const u8) -> __m256i {
    // SAFETY: caller guarantees `src` is valid for 32 bytes.
    unsafe { _mm256_loadu_si256(src as *const __m256i) }
}

#[inline(always)]
unsafe fn storeu(src: __m256i, dest: *mut u8) {
    // SAFETY: caller guarantees `dest` is valid for 32 bytes.
    unsafe { _mm256_storeu_si256(dest as *mut __m256i, src) }
}

#[inline(always)]
unsafe fn add(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_add_epi32(a, b) }
}

#[inline(always)]
unsafe fn xor(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_xor_si256(a, b) }
}

#[inline(always)]
unsafe fn set1(x: u32) -> __m256i {
    unsafe { _mm256_set1_epi32(x as i32) }
}

#[inline(always)]
unsafe fn set8(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32) -> __m256i {
    unsafe { _mm256_setr_epi32(a as i32, b as i32, c as i32, d as i32, e as i32, f as i32, g as i32, h as i32) }
}

/// Byte shuffle mask that rotates every 32-bit lane right by 16 bits:
/// `out[i] = in[(i + 2) % 4]` within each lane. Repeated per 128-bit half.
#[rustfmt::skip]
const ROT16_MASK: [u8; 32] = [
    2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13,
    2, 3, 0, 1, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13,
];

/// Byte shuffle mask that rotates every 32-bit lane right by 8 bits:
/// `out[i] = in[(i + 1) % 4]` within each lane.
#[rustfmt::skip]
const ROT8_MASK: [u8; 32] = [
    1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12,
    1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12,
];

/// Rotate each 32-bit lane right by 16 bits with a byte shuffle (`vpshufb`).
#[inline(always)]
unsafe fn rot16(x: __m256i) -> __m256i {
    // SAFETY: `ROT16_MASK` is a 32-byte constant and `loadu` reads 32 bytes.
    unsafe { _mm256_shuffle_epi8(x, loadu(ROT16_MASK.as_ptr())) }
}

#[inline(always)]
unsafe fn rot12(x: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(_mm256_srli_epi32(x, 12), _mm256_slli_epi32(x, 32 - 12)) }
}

/// Rotate each 32-bit lane right by 8 bits with a byte shuffle (`vpshufb`).
#[inline(always)]
unsafe fn rot8(x: __m256i) -> __m256i {
    // SAFETY: `ROT8_MASK` is a 32-byte constant and `loadu` reads 32 bytes.
    unsafe { _mm256_shuffle_epi8(x, loadu(ROT8_MASK.as_ptr())) }
}

#[inline(always)]
unsafe fn rot7(x: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(_mm256_srli_epi32(x, 7), _mm256_slli_epi32(x, 32 - 7)) }
}

#[inline(always)]
#[allow(clippy::too_many_lines)]
unsafe fn round(v: &mut [__m256i; 16], m: &[__m256i; 16], r: usize) {
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

#[inline(always)]
unsafe fn interleave128(a: __m256i, b: __m256i) -> (__m256i, __m256i) {
    unsafe { (_mm256_permute2x128_si256(a, b, 0x20), _mm256_permute2x128_si256(a, b, 0x31)) }
}

/// Transpose an 8x8 matrix of 32-bit words held in eight vectors.
#[inline(always)]
unsafe fn transpose_vecs(vecs: &mut [__m256i; DEGREE]) {
    unsafe {
        let ab_0145 = _mm256_unpacklo_epi32(vecs[0], vecs[1]);
        let ab_2367 = _mm256_unpackhi_epi32(vecs[0], vecs[1]);
        let cd_0145 = _mm256_unpacklo_epi32(vecs[2], vecs[3]);
        let cd_2367 = _mm256_unpackhi_epi32(vecs[2], vecs[3]);
        let ef_0145 = _mm256_unpacklo_epi32(vecs[4], vecs[5]);
        let ef_2367 = _mm256_unpackhi_epi32(vecs[4], vecs[5]);
        let gh_0145 = _mm256_unpacklo_epi32(vecs[6], vecs[7]);
        let gh_2367 = _mm256_unpackhi_epi32(vecs[6], vecs[7]);

        let abcd_04 = _mm256_unpacklo_epi64(ab_0145, cd_0145);
        let abcd_15 = _mm256_unpackhi_epi64(ab_0145, cd_0145);
        let abcd_26 = _mm256_unpacklo_epi64(ab_2367, cd_2367);
        let abcd_37 = _mm256_unpackhi_epi64(ab_2367, cd_2367);
        let efgh_04 = _mm256_unpacklo_epi64(ef_0145, gh_0145);
        let efgh_15 = _mm256_unpackhi_epi64(ef_0145, gh_0145);
        let efgh_26 = _mm256_unpacklo_epi64(ef_2367, gh_2367);
        let efgh_37 = _mm256_unpackhi_epi64(ef_2367, gh_2367);

        let (abcdefgh_0, abcdefgh_4) = interleave128(abcd_04, efgh_04);
        let (abcdefgh_1, abcdefgh_5) = interleave128(abcd_15, efgh_15);
        let (abcdefgh_2, abcdefgh_6) = interleave128(abcd_26, efgh_26);
        let (abcdefgh_3, abcdefgh_7) = interleave128(abcd_37, efgh_37);

        vecs[0] = abcdefgh_0;
        vecs[1] = abcdefgh_1;
        vecs[2] = abcdefgh_2;
        vecs[3] = abcdefgh_3;
        vecs[4] = abcdefgh_4;
        vecs[5] = abcdefgh_5;
        vecs[6] = abcdefgh_6;
        vecs[7] = abcdefgh_7;
    }
}

#[inline(always)]
unsafe fn transpose_msg_vecs(inputs: &[*const u8; DEGREE], block_offset: usize) -> [__m256i; 16] {
    unsafe {
        let mut vecs = [
            loadu(inputs[0].add(block_offset)),
            loadu(inputs[1].add(block_offset)),
            loadu(inputs[2].add(block_offset)),
            loadu(inputs[3].add(block_offset)),
            loadu(inputs[4].add(block_offset)),
            loadu(inputs[5].add(block_offset)),
            loadu(inputs[6].add(block_offset)),
            loadu(inputs[7].add(block_offset)),
            loadu(inputs[0].add(block_offset + 32)),
            loadu(inputs[1].add(block_offset + 32)),
            loadu(inputs[2].add(block_offset + 32)),
            loadu(inputs[3].add(block_offset + 32)),
            loadu(inputs[4].add(block_offset + 32)),
            loadu(inputs[5].add(block_offset + 32)),
            loadu(inputs[6].add(block_offset + 32)),
            loadu(inputs[7].add(block_offset + 32)),
        ];
        let (square0, square1) = vecs.split_at_mut(DEGREE);
        transpose_vecs(square0.try_into().unwrap());
        transpose_vecs(square1.try_into().unwrap());
        vecs
    }
}

#[inline(always)]
unsafe fn load_counters(counter: u64, increment_counter: IncrementCounter) -> (__m256i, __m256i) {
    let mask = if increment_counter.yes() { !0 } else { 0 };
    unsafe {
        (
            set8(
                counter_low(counter),
                counter_low(counter.wrapping_add(mask & 1)),
                counter_low(counter.wrapping_add(mask & 2)),
                counter_low(counter.wrapping_add(mask & 3)),
                counter_low(counter.wrapping_add(mask & 4)),
                counter_low(counter.wrapping_add(mask & 5)),
                counter_low(counter.wrapping_add(mask & 6)),
                counter_low(counter.wrapping_add(mask & 7)),
            ),
            set8(
                counter_high(counter),
                counter_high(counter.wrapping_add(mask & 1)),
                counter_high(counter.wrapping_add(mask & 2)),
                counter_high(counter.wrapping_add(mask & 3)),
                counter_high(counter.wrapping_add(mask & 4)),
                counter_high(counter.wrapping_add(mask & 5)),
                counter_high(counter.wrapping_add(mask & 6)),
                counter_high(counter.wrapping_add(mask & 7)),
            ),
        )
    }
}

/// Hash exactly [`DEGREE`] inputs of the same number of blocks.
///
/// # Safety
/// AVX2 must be available on the running CPU, and each input must have the
/// same whole number of 64-byte blocks (at least one).
#[target_feature(enable = "avx2")]
unsafe fn hash8(
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

            // Inlining the compression function manually here improves compile
            // times and codegen compared to factoring it out.
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

        transpose_vecs(&mut h_vecs);
        let ptr = out.as_mut_ptr();
        for (i, vec) in h_vecs.iter().enumerate() {
            storeu(*vec, ptr.add(i * 32));
        }
    }
}

/// Write exactly [`DEGREE`] extended-output blocks of a single root node.
///
/// Lane `i` compresses the same message block with counter `counter + i` and
/// writes the resulting 64-byte output block.
///
/// # Safety
/// AVX2 must be available on the running CPU.
#[target_feature(enable = "avx2")]
unsafe fn xof8(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8, out: &mut [u8]) {
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

        // After transposing each half, `v[i]` holds the first eight words of
        // output block `i` and `v[i + DEGREE]` holds the second eight.
        let (first, second) = v.split_at_mut(DEGREE);
        transpose_vecs(first.try_into().unwrap());
        transpose_vecs(second.try_into().unwrap());
        let ptr = out.as_mut_ptr();
        for i in 0..DEGREE {
            storeu(v[i], ptr.add(i * BLOCK_LEN));
            storeu(v[i + DEGREE], ptr.add(i * BLOCK_LEN + OUT_LEN));
        }
    }
}

/// Compress a single block into `cv` (truncated output).
///
/// # Safety
/// AVX2 must be available on the running CPU.
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn compress_in_place(
    cv: &mut [u32; 8],
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) {
    scalar::compress_in_place(cv, block, block_len, counter, flags);
}

/// Compress a single block and return the 64-byte extended output.
///
/// # Safety
/// AVX2 must be available on the running CPU.
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn compress_xof(
    cv: &[u32; 8],
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) -> [u8; 64] {
    scalar::compress_xof(cv, block, block_len, counter, flags)
}

/// Hash several equal-length inputs.
///
/// Every input must be a whole number of 64-byte blocks with the same block
/// count; `out` must hold `inputs.len() * OUT_LEN` bytes.
///
/// # Safety
/// AVX2 must be available on the running CPU.
#[target_feature(enable = "avx2")]
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
        let blocks = if inputs.is_empty() {
            0
        } else {
            inputs[0].len() / BLOCK_LEN
        };
        let input_ptrs: [*const u8; DEGREE] = [
            inputs[0].as_ptr(),
            inputs[1].as_ptr(),
            inputs[2].as_ptr(),
            inputs[3].as_ptr(),
            inputs[4].as_ptr(),
            inputs[5].as_ptr(),
            inputs[6].as_ptr(),
            inputs[7].as_ptr(),
        ];
        // SAFETY: AVX2 is available (target feature / runtime check) and the
        // eight inputs each have `blocks * BLOCK_LEN` readable bytes.
        unsafe {
            hash8(
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

    // Tail: fewer than DEGREE inputs remain.
    scalar::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out);
}

/// Fill `out` with consecutive extendable-output blocks.
///
/// `out` must be a whole number of 64-byte blocks and may be empty, in which
/// case this is a no-op.
///
/// # Safety
/// AVX2 must be available on the running CPU.
#[target_feature(enable = "avx2")]
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
        // SAFETY: AVX2 is available and `out` has DEGREE whole blocks.
        unsafe { xof8(cv, block, block_len, counter, flags, &mut out[..DEGREE * BLOCK_LEN]) };
        counter = counter.wrapping_add(DEGREE as u64);
        out = &mut out[DEGREE * BLOCK_LEN..];
    }

    scalar::xof_many(cv, block, block_len, counter, flags, out);
}

#[cfg(test)]
mod tests {
    use rand::{RngExt, SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn transpose_roundtrips() {
        if !is_x86_feature_detected!("avx2") {
            return;
        }

        #[target_feature(enable = "avx2")]
        unsafe fn transpose_wrapper(vecs: &mut [__m256i; DEGREE]) {
            unsafe { transpose_vecs(vecs) };
        }

        let mut matrix = [[0u32; DEGREE]; DEGREE];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = (i * DEGREE + j) as u32;
            }
        }

        unsafe {
            let mut vecs: [__m256i; DEGREE] = core::mem::transmute(matrix);
            transpose_wrapper(&mut vecs);
            matrix = core::mem::transmute::<[__m256i; 8], [[u32; 8]; 8]>(vecs);
        }

        for (i, row) in matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                assert_eq!(*cell, (j * DEGREE + i) as u32);
            }
        }
    }

    use crate::blake3::platform::{
        CHUNK_END, CHUNK_LEN, CHUNK_START, DERIVE_KEY_CONTEXT, DERIVE_KEY_MATERIAL, KEYED_HASH, PARENT, ROOT,
    };

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Differential tests against the portable/scalar kernels.
    //
    // These run in the backend's own configuration: natively on aarch64 (NEON),
    // under `wasm32-wasip1` with `-C target-feature=+simd128`, and on `x86_64`
    // with a CPU that actually exposes AVX2 / AVX-512F (or under qemu). The
    // guard below skips the test when the kernel cannot legally be executed.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Deterministic seeded `StdRng` stream: reproducible.
    fn pattern(len: usize, seed: u64) -> Vec<u8> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut out = vec![0u8; len];
        rng.fill(&mut out);
        out
    }

    fn words(bytes: &[u8]) -> [u32; 8] {
        let mut out = [0u32; 8];
        for (word, chunk) in out.iter_mut().zip(bytes.as_chunks::<4>().0) {
            *word = u32::from_le_bytes(*chunk);
        }
        out
    }

    fn kernel_available() -> bool {
        is_x86_feature_detected!("avx2")
    }

    /// Buffer whose `[pad..pad + len]` window holds deterministic data and whose
    /// trailing `trail` bytes are untouched canaries. `trail == 0` puts the
    /// window at the very end of the allocation, so an over-read of a whole
    /// SIMD block is a heap-buffer-overflow (visible under ASan).
    fn input_with_pad(len: usize, pad: usize, trail: usize, seed: u64) -> (Vec<u8>, usize) {
        let data = pattern(len, seed);
        let mut buf = vec![0xC3u8; pad + len + trail];
        buf[pad..pad + len].copy_from_slice(&data);
        (buf, pad)
    }

    const FLAGS: [u8; 10] = [
        0,
        CHUNK_START,
        CHUNK_END,
        PARENT,
        ROOT,
        KEYED_HASH,
        DERIVE_KEY_CONTEXT,
        DERIVE_KEY_MATERIAL,
        CHUNK_START | CHUNK_END | ROOT,
        CHUNK_START | CHUNK_END | ROOT | KEYED_HASH,
    ];

    const COUNTERS: [u64; 8] = [
        0,
        1,
        (1 << 31) - 1,
        u32::MAX as u64 - 1,
        u32::MAX as u64,
        1 << 32,
        (1 << 32) + 1,
        u64::MAX,
    ];

    /// Boundary-covering input counts: both sides of every SIMD group size.
    fn input_counts(degree: usize) -> Vec<usize> {
        let mut ns = vec![
            1,
            2,
            3,
            5,
            7,
            9,
            15,
            16,
            17,
            24,
            31,
            32,
            33,
            degree - 1,
            degree,
            degree + 1,
            2 * degree,
            2 * degree + 1,
        ];
        ns.retain(|n| (1..=2 * degree + 1).contains(n));
        ns.sort_unstable();
        ns.dedup();
        ns
    }

    /// `hash_many` compared against the scalar kernel, with output canaries and
    /// a variant where `out` is one block longer than needed (the kernel must
    /// not write past `inputs.len() * OUT_LEN` even when the slice is longer).
    #[allow(clippy::too_many_arguments)]
    fn check_hash_many(
        n: usize,
        len: usize,
        counter: u64,
        increment_counter: IncrementCounter,
        flags: u8,
        flags_start: u8,
        flags_end: u8,
        seed: u64,
    ) {
        const CANARY: u8 = 0x5A;
        const OUT_PAD: usize = 5;

        let key = words(&pattern(32, seed ^ 0xF00D));
        let mut bufs = Vec::with_capacity(n);
        let mut pads = Vec::with_capacity(n);
        for lane in 0..n {
            // Loads must work at every alignment and at the end of the allocation.
            let pad = [0usize, 1, 3, 7, 13, 31, 33, 63][lane % 8];
            let trail = if lane % 3 == 0 { 0 } else { [1usize, 3, 17][lane % 3] };
            let (buf, p) = input_with_pad(len, pad, trail, seed ^ (lane as u64) << 8);
            bufs.push(buf);
            pads.push(p);
        }
        let inputs: Vec<&[u8]> = (0..n).map(|lane| &bufs[lane][pads[lane]..pads[lane] + len]).collect();

        for extra in [0usize, OUT_LEN] {
            let mut expected = vec![CANARY; OUT_PAD + n * OUT_LEN + extra];
            let mut actual = expected.clone();

            scalar::hash_many(
                &inputs,
                &key,
                counter,
                increment_counter,
                flags,
                flags_start,
                flags_end,
                &mut expected[OUT_PAD..OUT_PAD + n * OUT_LEN],
            );
            // SAFETY: the backend was checked to be available by the caller.
            unsafe {
                hash_many(
                    &inputs,
                    &key,
                    counter,
                    increment_counter,
                    flags,
                    flags_start,
                    flags_end,
                    &mut actual[OUT_PAD..OUT_PAD + n * OUT_LEN + extra],
                );
            }

            assert_eq!(
                &expected[OUT_PAD..OUT_PAD + n * OUT_LEN],
                &actual[OUT_PAD..OUT_PAD + n * OUT_LEN],
                "hash_many mismatch (n {n}, len {len}, counter {counter}, \
                 increment {increment_counter:?}, flags {flags}, start {flags_start}, end {flags_end}, extra {extra})"
            );
            assert!(
                actual[..OUT_PAD].iter().all(|b| *b == CANARY),
                "hash_many wrote before `out` (n {n}, len {len})"
            );
            assert!(
                actual[OUT_PAD + n * OUT_LEN..].iter().all(|b| *b == CANARY),
                "hash_many wrote past inputs.len() * OUT_LEN (n {n}, len {len}, extra {extra})"
            );
        }
    }

    #[test]
    fn hash_many_matches_scalar() {
        if !kernel_available() {
            return;
        }

        let lengths = [BLOCK_LEN, 3 * BLOCK_LEN, CHUNK_LEN, CHUNK_LEN + BLOCK_LEN];
        let counters = [0u64, u32::MAX as u64 - 1, u32::MAX as u64, 1 << 32];
        let flag_sets = [(0u8, CHUNK_START, CHUNK_END), (PARENT, 0, 0)];

        for n in input_counts(DEGREE) {
            for &len in &lengths {
                for &counter in &counters {
                    for increment_counter in [IncrementCounter::Yes, IncrementCounter::No] {
                        for &(flags, flags_start, flags_end) in &flag_sets {
                            check_hash_many(
                                n,
                                len,
                                counter,
                                increment_counter,
                                flags,
                                flags_start,
                                flags_end,
                                0x1234 + n as u64 * 7 + len as u64,
                            );
                        }
                    }
                }
            }
        }

        // Counter wrap-around at the top of the u64 range, including a carry
        // across the 32-bit low word in the middle of a SIMD group.
        for &len in &[BLOCK_LEN, CHUNK_LEN] {
            for increment_counter in [IncrementCounter::Yes, IncrementCounter::No] {
                check_hash_many(DEGREE + 1, len, u64::MAX, increment_counter, 0, CHUNK_START, CHUNK_END, 0x99);
            }
        }
    }

    /// `xof_many` compared against the scalar kernel: every output block count
    /// on both sides of the SIMD degree, every block length, extreme counters,
    /// and canaries around (and after) the output blocks.
    fn check_xof_many(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, blocks: usize, counter: u64, flags: u8) {
        const CANARY: u8 = 0x3C;
        const OUT_PAD: usize = 7;

        let mut expected = vec![CANARY; OUT_PAD + blocks * BLOCK_LEN + BLOCK_LEN];
        let mut actual = expected.clone();
        scalar::xof_many(
            cv,
            block,
            block_len,
            counter,
            flags,
            &mut expected[OUT_PAD..OUT_PAD + blocks * BLOCK_LEN],
        );
        // SAFETY: the backend was checked to be available by the caller.
        unsafe {
            xof_many(
                cv,
                block,
                block_len,
                counter,
                flags,
                &mut actual[OUT_PAD..OUT_PAD + blocks * BLOCK_LEN],
            );
        }

        assert_eq!(
            &expected[OUT_PAD..OUT_PAD + blocks * BLOCK_LEN],
            &actual[OUT_PAD..OUT_PAD + blocks * BLOCK_LEN],
            "xof_many mismatch (blocks {blocks}, block_len {block_len}, counter {counter}, flags {flags})"
        );
        assert!(
            actual[..OUT_PAD].iter().all(|b| *b == CANARY),
            "xof_many wrote before `out` (blocks {blocks})"
        );
        assert!(
            actual[OUT_PAD + blocks * BLOCK_LEN..].iter().all(|b| *b == CANARY),
            "xof_many wrote past blocks * BLOCK_LEN (blocks {blocks})"
        );
    }

    #[test]
    fn xof_many_matches_scalar() {
        if !kernel_available() {
            return;
        }

        let cv = words(&pattern(32, 0x9A9A));
        let block_bytes = pattern(BLOCK_LEN, 0xBEEF);
        let block: [u8; BLOCK_LEN] = block_bytes.as_slice().try_into().unwrap();
        let counters = [0u64, u32::MAX as u64, 1 << 32, u64::MAX];
        let flag_sets = [ROOT | CHUNK_END, ROOT | CHUNK_END | KEYED_HASH];

        for blocks in 0..=2 * DEGREE + 1 {
            for &block_len in &[0u8, 1, 32, 63, 64] {
                for &counter in &counters {
                    for &flags in &flag_sets {
                        check_xof_many(&cv, &block, block_len, blocks, counter, flags);
                    }
                }
            }
        }

        // Exhaustive block-length sweep at the SIMD boundary.
        for block_len in 0..=BLOCK_LEN as u8 {
            for &blocks in &[DEGREE, DEGREE + 1] {
                check_xof_many(&cv, &block, block_len, blocks, 0, ROOT | CHUNK_END);
            }
        }

        // An empty output must be a no-op, not a write of a whole SIMD group.
        let mut empty: [u8; 0] = [];
        // SAFETY: the backend was checked to be available by the caller.
        unsafe { xof_many(&cv, &block, BLOCK_LEN as u8, 0, ROOT, &mut empty) };
    }

    /// Single-block compression compared against the scalar kernel for every
    /// block length 0..=64, extreme counters and all flag bits.
    #[test]
    fn compress_matches_scalar() {
        if !kernel_available() {
            return;
        }

        let block_bytes = pattern(BLOCK_LEN, 0xA11CE);
        let block: [u8; BLOCK_LEN] = block_bytes.as_slice().try_into().unwrap();
        let cv = words(&pattern(32, 0xB0B));

        for block_len in 0..=BLOCK_LEN as u8 {
            for &counter in &COUNTERS {
                for &flags in &FLAGS {
                    let mut expected = cv;
                    let mut actual = cv;
                    scalar::compress_in_place(&mut expected, &block, block_len, counter, flags);
                    // SAFETY: the backend was checked to be available by the caller.
                    unsafe { compress_in_place(&mut actual, &block, block_len, counter, flags) };
                    assert_eq!(
                        expected, actual,
                        "compress_in_place mismatch (block_len {block_len}, counter {counter}, flags {flags})"
                    );

                    let expected_xof = scalar::compress_xof(&cv, &block, block_len, counter, flags);
                    // SAFETY: the backend was checked to be available by the caller.
                    let actual_xof = unsafe { compress_xof(&cv, &block, block_len, counter, flags) };
                    assert_eq!(
                        expected_xof, actual_xof,
                        "compress_xof mismatch (block_len {block_len}, counter {counter}, flags {flags})"
                    );
                }
            }
        }
    }
}
