// SAFETY: all intrinsics are only called from functions that have #[target_feature(enable = "avx2")]
#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{Alphabet, DecodeError, EncodeError, decode_into_constant_time, encode_into_constant_time};

const AVX2_MAP_RFC4648: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const AVX2_MAP_RFC4648_LOWER: [u8; 32] = *b"abcdefghijklmnopqrstuvwxyz234567";
const AVX2_MAP_RFC4648_HEX: [u8; 32] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const AVX2_MAP_RFC4648_HEX_LOWER: [u8; 32] = *b"0123456789abcdefghijklmnopqrstuv";
const AVX2_MAP_CROCKFORD: [u8; 32] = *b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const AVX2_MAP_Z32: [u8; 32] = *b"ybndrfg8ejkmcpqxot1uwisza345h769";

// =============================================================================
// Encode
// =============================================================================

#[target_feature(enable = "avx2")]
pub unsafe fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    let tbl_bytes: &[u8; 32] = match alphabet {
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => &AVX2_MAP_RFC4648,
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => &AVX2_MAP_RFC4648_LOWER,
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => &AVX2_MAP_RFC4648_HEX,
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => &AVX2_MAP_RFC4648_HEX_LOWER,
        Alphabet::Crockford => &AVX2_MAP_CROCKFORD,
        Alphabet::Z32 => &AVX2_MAP_Z32,
    };

    let tbl_lo = _mm256_broadcastsi128_si256(_mm_loadu_si128(tbl_bytes.as_ptr().cast()));
    let tbl_hi = _mm256_broadcastsi128_si256(_mm_loadu_si128(tbl_bytes.as_ptr().add(16).cast()));
    let idx_mask = _mm256_set1_epi8(0x0F);
    let gt_15 = _mm256_set1_epi8(15);

    let mut inp = data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = data.len();

    while len >= 40 {
        let mut qbuf = [0u8; 64];
        extract_quintets_8blocks(inp, &mut qbuf);

        let q0 = _mm256_loadu_si256(qbuf.as_ptr().cast());
        let q1 = _mm256_loadu_si256(qbuf.as_ptr().add(32).cast());

        let c0 = simd_vtbl_32(q0, tbl_lo, tbl_hi, idx_mask, gt_15);
        let c1 = simd_vtbl_32(q1, tbl_lo, tbl_hi, idx_mask, gt_15);

        _mm256_storeu_si256(out.cast(), c0);
        _mm256_storeu_si256(out.add(32).cast(), c1);

        inp = inp.add(40);
        out = out.add(64);
        len -= 40;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let out_len = crate::encoded_length(len, alphabet.is_padded()).expect("encoded length overflow");
        let out_slice = core::slice::from_raw_parts_mut(out, out_len);
        return encode_into_constant_time(out_slice, data_slice, alphabet);
    }

    Ok(())
}

#[inline]
unsafe fn extract_quintets_8blocks(inp: *const u8, out: &mut [u8; 64]) {
    let b = |i: usize| inp.add(i).read_unaligned();

    for block in 0..8 {
        let off = block * 5;
        let qoff = block * 8;
        out[qoff + 0] = b(off) >> 3;
        out[qoff + 1] = ((b(off) & 0x07) << 2) | (b(off + 1) >> 6);
        out[qoff + 2] = (b(off + 1) >> 1) & 0x1F;
        out[qoff + 3] = ((b(off + 1) & 0x01) << 4) | (b(off + 2) >> 4);
        out[qoff + 4] = ((b(off + 2) & 0x0F) << 1) | (b(off + 3) >> 7);
        out[qoff + 5] = (b(off + 3) >> 2) & 0x1F;
        out[qoff + 6] = ((b(off + 3) & 0x03) << 3) | (b(off + 4) >> 5);
        out[qoff + 7] = b(off + 4) & 0x1F;
    }
}

/// Map 32 quintets to chars via _mm256_shuffle_epi8.
/// tbl_lo covers indices [0..15], tbl_hi covers [16..31].
#[inline]
unsafe fn simd_vtbl_32(
    indices: __m256i,
    tbl_lo: __m256i,
    tbl_hi: __m256i,
    idx_mask: __m256i,
    gt_15: __m256i,
) -> __m256i {
    let is_lo = _mm256_cmpgt_epi8(gt_15, indices);
    let idx = _mm256_and_si256(indices, idx_mask);
    let chars_lo = _mm256_shuffle_epi8(tbl_lo, idx);
    let chars_hi = _mm256_shuffle_epi8(tbl_hi, idx);
    _mm256_blendv_epi8(chars_hi, chars_lo, is_lo)
}

// =============================================================================
// Decode
// =============================================================================

#[target_feature(enable = "avx2")]
pub unsafe fn decode_into(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    match alphabet {
        Alphabet::Crockford | Alphabet::Z32 => decode_scalar(output, encoded_data, alphabet),
        _ => decode_simd(output, encoded_data, alphabet),
    }
}

/// Scalar decode fallback for Crockford.
unsafe fn decode_scalar(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let mut inp = encoded_data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = encoded_data.len();

    while len >= 8 {
        let c0 = inp.read_unaligned();
        let c1 = inp.add(1).read_unaligned();
        let c2 = inp.add(2).read_unaligned();
        let c3 = inp.add(3).read_unaligned();
        let c4 = inp.add(4).read_unaligned();
        let c5 = inp.add(5).read_unaligned();
        let c6 = inp.add(6).read_unaligned();
        let c7 = inp.add(7).read_unaligned();

        let q0 = super::char_to_quintet(c0, alphabet);
        let q1 = super::char_to_quintet(c1, alphabet);
        let q2 = super::char_to_quintet(c2, alphabet);
        let q3 = super::char_to_quintet(c3, alphabet);
        let q4 = super::char_to_quintet(c4, alphabet);
        let q5 = super::char_to_quintet(c5, alphabet);
        let q6 = super::char_to_quintet(c6, alphabet);
        let q7 = super::char_to_quintet(c7, alphabet);

        if q0 | q1 | q2 | q3 | q4 | q5 | q6 | q7 >= 32 {
            return Err(DecodeError::InvalidInput);
        }

        let b0 = (q0 << 3) | (q1 >> 2);
        let b1 = (q1.wrapping_shl(6)) | (q2 << 1) | (q3 >> 4);
        let b2 = (q3.wrapping_shl(4)) | (q4 >> 1);
        let b3 = (q4.wrapping_shl(7)) | (q5 << 2) | (q6 >> 3);
        let b4 = (q6.wrapping_shl(5)) | q7;

        core::ptr::copy_nonoverlapping([b0, b1, b2, b3, b4].as_ptr(), out, 5);

        inp = inp.add(8);
        out = out.add(5);
        len -= 8;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let out_len = len * 5 / 8;
        let out_slice = core::slice::from_raw_parts_mut(out, out_len.max(1));
        decode_into_constant_time(out_slice, data_slice, alphabet)
    } else {
        Ok(())
    }
}

/// SIMD decode for 2-range alphabets.
///
/// Each alphabet maps input chars to quintets via two contiguous ranges:
///   Range 1: [r1_lo_char, r1_hi_char] → quintet = char - r1_sub
///   Range 2: [r2_lo_char, r2_hi_char] → quintet = char - r2_sub
unsafe fn decode_simd(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let (r1_lo_char, r1_hi_char, r1_sub, r2_lo_char, r2_hi_char, r2_sub) = match alphabet {
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => (b'A', b'Z', b'A', b'2', b'7', b'2' - 26),
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => (b'a', b'z', b'a', b'2', b'7', b'2' - 26),
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => (b'0', b'9', b'0', b'A', b'V', b'A' - 10),
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => (b'0', b'9', b'0', b'a', b'v', b'a' - 10),
        _ => unreachable!(),
    };

    let r1_lo = _mm256_set1_epi8(r1_lo_char as i8);
    let r1_hi = _mm256_set1_epi8(r1_hi_char as i8);
    let r1_sub = _mm256_set1_epi8(r1_sub as i8);
    let r2_lo = _mm256_set1_epi8(r2_lo_char as i8);
    let r2_hi = _mm256_set1_epi8(r2_hi_char as i8);
    let r2_sub = _mm256_set1_epi8(r2_sub as i8);

    let mut inp = encoded_data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = encoded_data.len();

    while len >= 32 {
        let chars = _mm256_loadu_si256(inp.cast());

        let quintets = simd_char_to_quintet_2ranges(chars, r1_lo, r1_hi, r1_sub, r2_lo, r2_hi, r2_sub);

        let invalid = _mm256_cmpgt_epi8(quintets, _mm256_set1_epi8(31));
        if _mm256_movemask_epi8(invalid) != 0 {
            return Err(DecodeError::InvalidInput);
        }

        let mut qbuf = [0u8; 32];
        _mm256_storeu_si256(qbuf.as_mut_ptr().cast(), quintets);

        for block in 0..4 {
            let qoff = block * 8;
            let q = &qbuf[qoff..qoff + 8];
            let b0 = (q[0] << 3) | (q[1] >> 2);
            let b1 = (q[1].wrapping_shl(6)) | (q[2] << 1) | (q[3] >> 4);
            let b2 = (q[3].wrapping_shl(4)) | (q[4] >> 1);
            let b3 = (q[4].wrapping_shl(7)) | (q[5] << 2) | (q[6] >> 3);
            let b4 = (q[6].wrapping_shl(5)) | q[7];
            core::ptr::copy_nonoverlapping([b0, b1, b2, b3, b4].as_ptr(), out.add(block * 5), 5);
        }

        inp = inp.add(32);
        out = out.add(20);
        len -= 32;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let out_len = len * 5 / 8;
        let out_slice = core::slice::from_raw_parts_mut(out, out_len.max(1));
        decode_into_constant_time(out_slice, data_slice, alphabet)
    } else {
        Ok(())
    }
}

/// Vectorized char→quintet for 2 contiguous ranges.
/// Range 1: char in [r1_lo, r1_hi] → quintet = char - r1_sub
/// Range 2: char in [r2_lo, r2_hi] → quintet = char - r2_sub
/// Invalid chars produce a value with bit 5 set (0x20).
#[inline]
unsafe fn simd_char_to_quintet_2ranges(
    chars: __m256i,
    r1_lo: __m256i,
    r1_hi: __m256i,
    r1_sub: __m256i,
    r2_lo: __m256i,
    r2_hi: __m256i,
    r2_sub: __m256i,
) -> __m256i {
    let ge_r1_lo = _mm256_cmpgt_epi8(chars, _mm256_sub_epi8(r1_lo, _mm256_set1_epi8(1)));
    let le_r1_hi = _mm256_xor_si256(_mm256_set1_epi8(-1), _mm256_cmpgt_epi8(chars, r1_hi));
    let in_r1 = _mm256_and_si256(ge_r1_lo, le_r1_hi);

    let ge_r2_lo = _mm256_cmpgt_epi8(chars, _mm256_sub_epi8(r2_lo, _mm256_set1_epi8(1)));
    let le_r2_hi = _mm256_xor_si256(_mm256_set1_epi8(-1), _mm256_cmpgt_epi8(chars, r2_hi));
    let in_r2 = _mm256_and_si256(ge_r2_lo, le_r2_hi);

    let val_r1 = _mm256_sub_epi8(chars, r1_sub);
    let val_r2 = _mm256_sub_epi8(chars, r2_sub);

    let tmp = _mm256_blendv_epi8(_mm256_set1_epi8(0x20), val_r2, in_r2);
    _mm256_blendv_epi8(tmp, val_r1, in_r1)
}
