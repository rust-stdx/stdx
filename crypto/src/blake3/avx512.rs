//! AVX-512 (16-way) BLAKE3 kernels for `x86_64`.
//!
//! The wide [`hash_many`] path hashes up to sixteen chunks in parallel, one
//! lane per chunk, and [`xof_many`] produces up to sixteen extendable-output
//! blocks at once, one lane per output block. As in the AVX2 kernel, the
//! chaining values stay transposed across the blocks of a chunk so that the
//! transpose is paid once per chunk instead of once per block.
//!
//! This module deliberately avoids the
//! AVX-512 VL/BW/DQ extensions: the 512-bit operations use only the AVX-512
//! Foundation instruction set (add, xor, unpack, shuffle and the `vprold`
//! rotation). That matches the `avx512f` feature tested by the platform
//! detection, so CPUs that expose AVX-512F without the other subsets (e.g.
//! Xeon Phi) can run it safely instead of faulting on an illegal instruction.
//! The 256-bit loads and stores the compiler emits alongside are plain
//! AVX/AVX2, which every AVX-512F CPU also implements.
//!
//! Single-block compression and short tails are delegated to the portable
//! kernel (and to AVX2 when it is available), because a lone 64-byte
//! compression only fills one 128-bit register and gains nothing from AVX-512.
//!
//! The extra AVX-512 subsets would not speed up these kernels: the compression
//! loop needs only add, xor, rotate and lane shuffles, all of which are in the
//! Foundation set (`vprold` covers all four rotations, so BW's `vpshufb` has
//! nothing to add, and DQ is unused). The one path where VL would help is
//! single-block compression (`compress_in_place`/`compress_xof`), which
//! handles streaming updates and inputs of at most one chunk.
//! That dispatch tier is deliberately not implemented yet: measure
//! on AVX-512 hardware first.

use core::arch::x86_64::*;

use super::{
    platform::{BLOCK_LEN, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, counter_high, counter_low},
    scalar,
};

/// Number of chunks (or output blocks) processed in parallel.
pub(crate) const DEGREE: usize = 16;

#[inline(always)]
unsafe fn loadu(src: *const u8) -> __m512i {
    // SAFETY: caller guarantees `src` is valid for 64 bytes.
    unsafe { _mm512_loadu_si512(src.cast()) }
}

#[inline(always)]
unsafe fn storeu(src: __m512i, dest: *mut u8) {
    // SAFETY: caller guarantees `dest` is valid for 64 bytes.
    unsafe { _mm512_storeu_si512(dest.cast(), src) }
}

#[inline(always)]
unsafe fn add(a: __m512i, b: __m512i) -> __m512i {
    unsafe { _mm512_add_epi32(a, b) }
}

#[inline(always)]
unsafe fn xor(a: __m512i, b: __m512i) -> __m512i {
    unsafe { _mm512_xor_si512(a, b) }
}

#[inline(always)]
unsafe fn set1(x: u32) -> __m512i {
    unsafe { _mm512_set1_epi32(x as i32) }
}

/// Build a vector from 16 words given in lane order (lane 0 first).
///
/// `_mm512_setr_epi32` is avoided on purpose: its Rust argument order is easy
/// to misread, while an array load is unambiguous and equally cheap.
#[inline(always)]
unsafe fn set16(words: &[u32; DEGREE]) -> __m512i {
    unsafe { loadu(words.as_ptr().cast()) }
}

// All four rotations use `vprold`, which is a 512-bit AVX-512F instruction
// (unlike AVX2, where the byte-aligned ones need a shuffle table).
#[inline(always)]
unsafe fn rot16(x: __m512i) -> __m512i {
    unsafe { _mm512_ror_epi32::<16>(x) }
}

#[inline(always)]
unsafe fn rot12(x: __m512i) -> __m512i {
    unsafe { _mm512_ror_epi32::<12>(x) }
}

#[inline(always)]
unsafe fn rot8(x: __m512i) -> __m512i {
    unsafe { _mm512_ror_epi32::<8>(x) }
}

#[inline(always)]
unsafe fn rot7(x: __m512i) -> __m512i {
    unsafe { _mm512_ror_epi32::<7>(x) }
}

#[inline(always)]
#[allow(clippy::too_many_lines)]
unsafe fn round(v: &mut [__m512i; 16], m: &[__m512i; 16], r: usize) {
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

/// Interleave the low 128-bit quarters of `a` and `b` (`vshufi32x4`).
#[inline(always)]
unsafe fn unpack_lo_128(a: __m512i, b: __m512i) -> __m512i {
    unsafe { _mm512_shuffle_i32x4::<0x88>(a, b) }
}

/// Interleave the high 128-bit quarters of `a` and `b` (`vshufi32x4`).
#[inline(always)]
unsafe fn unpack_hi_128(a: __m512i, b: __m512i) -> __m512i {
    unsafe { _mm512_shuffle_i32x4::<0xdd>(a, b) }
}

/// Transpose a 16x16 matrix of 32-bit words held in sixteen vectors.
#[inline(always)]
#[allow(clippy::too_many_lines)]
unsafe fn transpose_vecs(vecs: &mut [__m512i; DEGREE]) {
    unsafe {
        let ab_0 = _mm512_unpacklo_epi32(vecs[0], vecs[1]);
        let ab_2 = _mm512_unpackhi_epi32(vecs[0], vecs[1]);
        let cd_0 = _mm512_unpacklo_epi32(vecs[2], vecs[3]);
        let cd_2 = _mm512_unpackhi_epi32(vecs[2], vecs[3]);
        let ef_0 = _mm512_unpacklo_epi32(vecs[4], vecs[5]);
        let ef_2 = _mm512_unpackhi_epi32(vecs[4], vecs[5]);
        let gh_0 = _mm512_unpacklo_epi32(vecs[6], vecs[7]);
        let gh_2 = _mm512_unpackhi_epi32(vecs[6], vecs[7]);
        let ij_0 = _mm512_unpacklo_epi32(vecs[8], vecs[9]);
        let ij_2 = _mm512_unpackhi_epi32(vecs[8], vecs[9]);
        let kl_0 = _mm512_unpacklo_epi32(vecs[10], vecs[11]);
        let kl_2 = _mm512_unpackhi_epi32(vecs[10], vecs[11]);
        let mn_0 = _mm512_unpacklo_epi32(vecs[12], vecs[13]);
        let mn_2 = _mm512_unpackhi_epi32(vecs[12], vecs[13]);
        let op_0 = _mm512_unpacklo_epi32(vecs[14], vecs[15]);
        let op_2 = _mm512_unpackhi_epi32(vecs[14], vecs[15]);

        let abcd_0 = _mm512_unpacklo_epi64(ab_0, cd_0);
        let abcd_1 = _mm512_unpackhi_epi64(ab_0, cd_0);
        let abcd_2 = _mm512_unpacklo_epi64(ab_2, cd_2);
        let abcd_3 = _mm512_unpackhi_epi64(ab_2, cd_2);
        let efgh_0 = _mm512_unpacklo_epi64(ef_0, gh_0);
        let efgh_1 = _mm512_unpackhi_epi64(ef_0, gh_0);
        let efgh_2 = _mm512_unpacklo_epi64(ef_2, gh_2);
        let efgh_3 = _mm512_unpackhi_epi64(ef_2, gh_2);
        let ijkl_0 = _mm512_unpacklo_epi64(ij_0, kl_0);
        let ijkl_1 = _mm512_unpackhi_epi64(ij_0, kl_0);
        let ijkl_2 = _mm512_unpacklo_epi64(ij_2, kl_2);
        let ijkl_3 = _mm512_unpackhi_epi64(ij_2, kl_2);
        let mnop_0 = _mm512_unpacklo_epi64(mn_0, op_0);
        let mnop_1 = _mm512_unpackhi_epi64(mn_0, op_0);
        let mnop_2 = _mm512_unpacklo_epi64(mn_2, op_2);
        let mnop_3 = _mm512_unpackhi_epi64(mn_2, op_2);

        let abcdefgh_0 = unpack_lo_128(abcd_0, efgh_0);
        let abcdefgh_1 = unpack_lo_128(abcd_1, efgh_1);
        let abcdefgh_2 = unpack_lo_128(abcd_2, efgh_2);
        let abcdefgh_3 = unpack_lo_128(abcd_3, efgh_3);
        let abcdefgh_4 = unpack_hi_128(abcd_0, efgh_0);
        let abcdefgh_5 = unpack_hi_128(abcd_1, efgh_1);
        let abcdefgh_6 = unpack_hi_128(abcd_2, efgh_2);
        let abcdefgh_7 = unpack_hi_128(abcd_3, efgh_3);
        let ijklmnop_0 = unpack_lo_128(ijkl_0, mnop_0);
        let ijklmnop_1 = unpack_lo_128(ijkl_1, mnop_1);
        let ijklmnop_2 = unpack_lo_128(ijkl_2, mnop_2);
        let ijklmnop_3 = unpack_lo_128(ijkl_3, mnop_3);
        let ijklmnop_4 = unpack_hi_128(ijkl_0, mnop_0);
        let ijklmnop_5 = unpack_hi_128(ijkl_1, mnop_1);
        let ijklmnop_6 = unpack_hi_128(ijkl_2, mnop_2);
        let ijklmnop_7 = unpack_hi_128(ijkl_3, mnop_3);

        vecs[0] = unpack_lo_128(abcdefgh_0, ijklmnop_0);
        vecs[1] = unpack_lo_128(abcdefgh_1, ijklmnop_1);
        vecs[2] = unpack_lo_128(abcdefgh_2, ijklmnop_2);
        vecs[3] = unpack_lo_128(abcdefgh_3, ijklmnop_3);
        vecs[4] = unpack_lo_128(abcdefgh_4, ijklmnop_4);
        vecs[5] = unpack_lo_128(abcdefgh_5, ijklmnop_5);
        vecs[6] = unpack_lo_128(abcdefgh_6, ijklmnop_6);
        vecs[7] = unpack_lo_128(abcdefgh_7, ijklmnop_7);
        vecs[8] = unpack_hi_128(abcdefgh_0, ijklmnop_0);
        vecs[9] = unpack_hi_128(abcdefgh_1, ijklmnop_1);
        vecs[10] = unpack_hi_128(abcdefgh_2, ijklmnop_2);
        vecs[11] = unpack_hi_128(abcdefgh_3, ijklmnop_3);
        vecs[12] = unpack_hi_128(abcdefgh_4, ijklmnop_4);
        vecs[13] = unpack_hi_128(abcdefgh_5, ijklmnop_5);
        vecs[14] = unpack_hi_128(abcdefgh_6, ijklmnop_6);
        vecs[15] = unpack_hi_128(abcdefgh_7, ijklmnop_7);
    }
}

#[inline(always)]
unsafe fn transpose_msg_vecs(inputs: &[*const u8; DEGREE], block_offset: usize) -> [__m512i; 16] {
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
            loadu(inputs[8].add(block_offset)),
            loadu(inputs[9].add(block_offset)),
            loadu(inputs[10].add(block_offset)),
            loadu(inputs[11].add(block_offset)),
            loadu(inputs[12].add(block_offset)),
            loadu(inputs[13].add(block_offset)),
            loadu(inputs[14].add(block_offset)),
            loadu(inputs[15].add(block_offset)),
        ];
        transpose_vecs(&mut vecs);
        vecs
    }
}

#[inline(always)]
unsafe fn load_counters(counter: u64, increment_counter: IncrementCounter) -> (__m512i, __m512i) {
    let mask = if increment_counter.yes() { !0 } else { 0 };
    let mut low = [0u32; DEGREE];
    let mut high = [0u32; DEGREE];
    for lane in 0..DEGREE {
        let lane_counter = counter.wrapping_add(mask & lane as u64);
        low[lane] = counter_low(lane_counter);
        high[lane] = counter_high(lane_counter);
    }
    // SAFETY: both arrays are exactly 64 bytes.
    unsafe { (set16(&low), set16(&high)) }
}

/// Hash exactly [`DEGREE`] inputs of the same number of blocks.
///
/// # Safety
/// AVX-512F must be available on the running CPU, and each input must have the
/// same whole number of 64-byte blocks (at least one).
#[target_feature(enable = "avx512f")]
unsafe fn hash16(
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

        // `transpose_vecs` works on a full 16x16 matrix, but only the eight
        // chaining-value vectors are live. Pushing them through the transpose
        // alongside eight zero vectors plants each output's eight words in the
        // low 256 bits of the corresponding result vector.
        let mut padded = [
            h_vecs[0],
            h_vecs[1],
            h_vecs[2],
            h_vecs[3],
            h_vecs[4],
            h_vecs[5],
            h_vecs[6],
            h_vecs[7],
            set1(0),
            set1(0),
            set1(0),
            set1(0),
            set1(0),
            set1(0),
            set1(0),
            set1(0),
        ];
        transpose_vecs(&mut padded);
        let ptr = out.as_mut_ptr();
        for (i, vec) in padded.iter().enumerate() {
            // SAFETY: `out` holds DEGREE * OUT_LEN bytes and each store writes
            // 32 bytes at a 32-byte stride.
            _mm256_storeu_si256(ptr.add(i * OUT_LEN).cast(), _mm512_castsi512_si256(*vec));
        }
    }
}

/// Hash exactly [`DEGREE`] extended-output blocks of a single root node.
///
/// Lane `i` compresses the same message block with counter `counter + i` and
/// writes the resulting 64-byte output block; the extended output is taken in
/// full, so each lane stores one whole output block.
///
/// # Safety
/// AVX-512F must be available on the running CPU.
#[target_feature(enable = "avx512f")]
unsafe fn xof16(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8, out: &mut [u8]) {
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

        // After the transpose, vector `i` holds the sixteen words of output
        // block `i` in lane order, i.e. its 64 little-endian bytes.
        transpose_vecs(&mut v);
        let ptr = out.as_mut_ptr();
        for (i, vec) in v.iter().enumerate() {
            storeu(*vec, ptr.add(i * BLOCK_LEN));
        }
    }
}

/// Hash several equal-length inputs.
///
/// Every input must be a whole number of 64-byte blocks with the same block
/// count; `out` must hold `inputs.len() * OUT_LEN` bytes.
///
/// # Safety
/// AVX-512F must be available on the running CPU.
#[target_feature(enable = "avx512f")]
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
            inputs[4].as_ptr(),
            inputs[5].as_ptr(),
            inputs[6].as_ptr(),
            inputs[7].as_ptr(),
            inputs[8].as_ptr(),
            inputs[9].as_ptr(),
            inputs[10].as_ptr(),
            inputs[11].as_ptr(),
            inputs[12].as_ptr(),
            inputs[13].as_ptr(),
            inputs[14].as_ptr(),
            inputs[15].as_ptr(),
        ];
        // SAFETY: AVX-512F is available (target feature / runtime check) and
        // the sixteen inputs each have `blocks * BLOCK_LEN` readable bytes.
        unsafe {
            hash16(
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

    // Tail: fewer than DEGREE inputs remain. Delegate to AVX2 when it is
    // available, otherwise 8..15-chunk inputs would hash entirely on the
    // scalar path and be slower than on an AVX2-only build.
    #[cfg(any(feature = "std", target_feature = "avx2"))]
    {
        #[cfg(feature = "std")]
        let avx2_available = is_x86_feature_detected!("avx2");
        #[cfg(not(feature = "std"))]
        let avx2_available = true;
        if avx2_available {
            // SAFETY: AVX2 is available (runtime check or compile-time feature).
            return unsafe {
                super::avx2::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out)
            };
        }
    }
    scalar::hash_many(inputs, key, counter, increment_counter, flags, flags_start, flags_end, out);
}

/// Fill `out` with consecutive extendable-output blocks.
///
/// `out` must be a whole number of 64-byte blocks and may be empty, in which
/// case this is a no-op.
///
/// # Safety
/// AVX-512F must be available on the running CPU.
#[target_feature(enable = "avx512f")]
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
        // SAFETY: AVX-512F is available and `out` has DEGREE whole blocks.
        unsafe { xof16(cv, block, block_len, counter, flags, &mut out[..DEGREE * BLOCK_LEN]) };
        counter = counter.wrapping_add(DEGREE as u64);
        out = &mut out[DEGREE * BLOCK_LEN..];
    }

    // Tail: see the note in `hash_many`.
    #[cfg(any(feature = "std", target_feature = "avx2"))]
    {
        #[cfg(feature = "std")]
        let avx2_available = is_x86_feature_detected!("avx2");
        #[cfg(not(feature = "std"))]
        let avx2_available = true;
        if avx2_available && !out.is_empty() {
            // SAFETY: AVX2 is available (runtime check or compile-time feature).
            return unsafe { super::avx2::xof_many(cv, block, block_len, counter, flags, out) };
        }
    }
    scalar::xof_many(cv, block, block_len, counter, flags, out);
}

/// Compress a single block into `cv` (truncated output).
///
/// # Safety
/// AVX-512F must be available on the running CPU.
#[target_feature(enable = "avx512f")]
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
/// AVX-512F must be available on the running CPU.
#[target_feature(enable = "avx512f")]
pub(crate) unsafe fn compress_xof(
    cv: &[u32; 8],
    block: &[u8; BLOCK_LEN],
    block_len: u8,
    counter: u64,
    flags: u8,
) -> [u8; 64] {
    scalar::compress_xof(cv, block, block_len, counter, flags)
}

#[cfg(test)]
mod tests {
    use rand::{RngExt, SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn transpose_roundtrips() {
        if !is_x86_feature_detected!("avx512f") {
            return;
        }

        #[target_feature(enable = "avx512f")]
        unsafe fn transpose_wrapper(vecs: &mut [__m512i; DEGREE]) {
            unsafe { transpose_vecs(vecs) };
        }

        let mut matrix = [[0u32; DEGREE]; DEGREE];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = (i * DEGREE + j) as u32;
            }
        }

        unsafe {
            let mut vecs: [__m512i; DEGREE] = core::mem::transmute(matrix);
            transpose_wrapper(&mut vecs);
            matrix = core::mem::transmute::<[__m512i; 16], [[u32; 16]; 16]>(vecs);
        }

        for (i, row) in matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                assert_eq!(*cell, (j * DEGREE + i) as u32);
            }
        }
    }

    /// The per-lane counters must match scalar `u64` arithmetic, including a
    /// carry out of the low 32-bit word inside the group (BLAKE3 issue #271).
    #[test]
    fn load_counters_matches_scalar() {
        if !is_x86_feature_detected!("avx512f") {
            return;
        }

        #[target_feature(enable = "avx512f")]
        unsafe fn check(counter: u64, increment: IncrementCounter, lanes: &mut [u32; 2 * DEGREE]) {
            let (low, high) = unsafe { load_counters(counter, increment) };
            unsafe {
                _mm512_storeu_si512(lanes.as_mut_ptr().cast(), low);
                _mm512_storeu_si512(lanes.as_mut_ptr().add(DEGREE).cast(), high);
            }
        }

        for &counter in &[0u64, 7, u32::MAX as u64 - 8, u32::MAX as u64, (1 << 32) | 5] {
            for increment in [IncrementCounter::Yes, IncrementCounter::No] {
                let mut lanes = [0u32; 2 * DEGREE];
                unsafe { check(counter, increment, &mut lanes) };
                let mask = if increment.yes() { u64::MAX } else { 0 };
                for lane in 0..DEGREE {
                    let expected = counter.wrapping_add(mask & lane as u64);
                    assert_eq!(lanes[lane], counter_low(expected), "low lane {lane}");
                    assert_eq!(lanes[DEGREE + lane], counter_high(expected), "high lane {lane}");
                }
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
        is_x86_feature_detected!("avx512f")
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
