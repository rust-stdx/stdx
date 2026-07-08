// SAFETY: all intrinsics are only called from functions that have #[target_feature(enable = "avx2")]
#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{Alphabet, DecodeError, EncodeError, decode_into_constant_time, encode_into_constant_time};

// Shuffle: rearrange 12 bytes per 128-bit lane into 4×32-bit triplet lanes.
// Input: [b11,b10,b9,b8,b7,b6,b5,b4,b3,b2,b1,b0,  ?,?,?,?]
// Output per 32-bit lane: [b2,b1,b0,?], [b5,b4,b3,?], [b8,b7,b6,?], [b11,b10,b9,?]
#[rustfmt::skip]
const ENCODE_SHUFFLE: [i8; 32] = [
    2, 1, 0, -1,  5, 4, 3, -1,  8, 7, 6, -1,  11, 10, 9, -1,
    2, 1, 0, -1,  5, 4, 3, -1,  8, 7, 6, -1,  11, 10, 9, -1,
];

// Compact: after madd packing, extract 3 bytes per 32-bit lane into 12 bytes per 128-bit half.
// Input per 32-bit lane: [0x00, byte2, byte1, byte0] (3 data bytes at positions 2,1,0)
// Output: [byte2, byte1, byte0,  byte2, byte1, byte0,  ...] per 128-bit half
#[rustfmt::skip]
const DECODE_COMPACT: [i8; 32] = [
    2, 1, 0,  6, 5, 4,  10, 9, 8,  14, 13, 12,  -1, -1, -1, -1,
    2, 1, 0,  6, 5, 4,  10, 9, 8,  14, 13, 12,  -1, -1, -1, -1,
];

// ======================================================================
// Encoding
// ======================================================================

#[target_feature(enable = "avx2")]
pub unsafe fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    let shuffle = _mm256_loadu_si256(ENCODE_SHUFFLE.as_ptr().cast());

    let mask_ac = _mm256_set1_epi32(0x0fc0fc00i32 as i32);
    let mask_bd = _mm256_set1_epi32(0x003f03f0i32 as i32);
    let mul_ac = _mm256_set1_epi32(0x04000040u32 as i32);
    let mul_bd = _mm256_set1_epi32(0x01000010u32 as i32);

    let gt_25 = _mm256_set1_epi8(25i8);
    let gt_51 = _mm256_set1_epi8(51i8);
    let v_62 = _mm256_set1_epi8(62i8);
    let v_63 = _mm256_set1_epi8(63i8);
    let base = _mm256_set1_epi8(b'A' as i8);
    let add_6 = _mm256_set1_epi8(6i8);
    let sub_75 = _mm256_set1_epi8(-75i8);
    let add_241 = _mm256_set1_epi8(-15i8); // 241 as u8 = -15 as i8
    let sub_12 = _mm256_set1_epi8(-12i8);

    let url = matches!(alphabet, Alphabet::Url | Alphabet::UrlNoPadding);
    let v_m62 = _mm256_set1_epi8(62i8);
    let v_m63 = _mm256_set1_epi8(63i8);

    let mut inp = data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = data.len();

    while len >= 24 {
        let lo = _mm_loadu_si128(inp.cast());
        let hi = _mm_loadu_si128(inp.add(12).cast());
        let merged = _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(lo), hi);

        let shuf = _mm256_shuffle_epi8(merged, shuffle);

        let t_ac = _mm256_and_si256(shuf, mask_ac);
        let t_bd = _mm256_and_si256(shuf, mask_bd);
        let v_ac = _mm256_mulhi_epu16(t_ac, mul_ac);
        let v_bd = _mm256_mullo_epi16(t_bd, mul_bd);

        let indices = _mm256_or_si256(v_ac, v_bd);

        let mut result = _mm256_add_epi8(indices, base);
        let c_gt_25 = _mm256_cmpgt_epi8(indices, gt_25);
        result = _mm256_add_epi8(result, _mm256_and_si256(c_gt_25, add_6));
        let c_gt_51 = _mm256_cmpgt_epi8(indices, gt_51);
        result = _mm256_add_epi8(result, _mm256_and_si256(c_gt_51, sub_75));
        let c_eq_62 = _mm256_cmpeq_epi8(indices, v_62);
        result = _mm256_add_epi8(result, _mm256_and_si256(c_eq_62, add_241));
        let c_eq_63 = _mm256_cmpeq_epi8(indices, v_63);
        result = _mm256_add_epi8(result, _mm256_and_si256(c_eq_63, sub_12));

        if url {
            let eq_62 = _mm256_cmpeq_epi8(indices, v_m62);
            result = _mm256_blendv_epi8(result, _mm256_set1_epi8(b'-' as i8), eq_62);
            let eq_63 = _mm256_cmpeq_epi8(indices, v_m63);
            result = _mm256_blendv_epi8(result, _mm256_set1_epi8(b'_' as i8), eq_63);
        }

        _mm256_storeu_si256(out.cast(), result);

        inp = inp.add(24);
        out = out.add(32);
        len -= 24;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let padded = alphabet.is_padded();
        let out_len = match len % 3 {
            0 => (len / 3) * 4,
            1 => (len / 3) * 4 + if padded { 4 } else { 2 },
            _ => (len / 3) * 4 + if padded { 4 } else { 3 },
        };
        let out_slice = core::slice::from_raw_parts_mut(out, out_len);
        return encode_into_constant_time(out_slice, data_slice, alphabet);
    }

    Ok(())
}

// ======================================================================
// Decoding
// ======================================================================

#[target_feature(enable = "avx2")]
pub unsafe fn decode_into(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let url = matches!(alphabet, Alphabet::Url | Alphabet::UrlNoPadding);
    let zero = _mm256_setzero_si256();

    let v_plus = _mm256_set1_epi8(b'+' as i8);
    let v_slash = _mm256_set1_epi8(b'/' as i8);

    let sh_n65 = _mm256_set1_epi8(-65i8);
    let sh_n71 = _mm256_set1_epi8(-71i8);
    let sh_p4 = _mm256_set1_epi8(4i8);
    let sh_p19 = _mm256_set1_epi8(19i8);
    let sh_p16 = _mm256_set1_epi8(16i8);
    let sh_p17 = _mm256_set1_epi8(17i8);
    let sh_n32 = _mm256_set1_epi8(-32i8);

    let maddubs_const = _mm256_set1_epi32(0x01400140u32 as i32);
    let madd_const = _mm256_set1_epi32(0x00011000u32 as i32);
    let compact = _mm256_loadu_si256(DECODE_COMPACT.as_ptr().cast());

    let mut inp = encoded_data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = encoded_data.len();

    while len >= 32 {
        let c = _mm256_loadu_si256(inp.cast());

        // A-Z range
        let ge_A = _mm256_cmpgt_epi8(c, _mm256_set1_epi8((b'A' - 1) as i8));
        let le_Z = _mm256_cmpgt_epi8(_mm256_set1_epi8((b'Z' + 1) as i8), c);
        let m_AZ = _mm256_and_si256(ge_A, le_Z);
        let sh_AZ = _mm256_and_si256(m_AZ, sh_n65);

        // a-z range
        let ge_a = _mm256_cmpgt_epi8(c, _mm256_set1_epi8((b'a' - 1) as i8));
        let le_z = _mm256_cmpgt_epi8(_mm256_set1_epi8((b'z' + 1) as i8), c);
        let m_az = _mm256_and_si256(ge_a, le_z);
        let sh_az = _mm256_and_si256(m_az, sh_n71);

        // 0-9 range
        let ge_0 = _mm256_cmpgt_epi8(c, _mm256_set1_epi8((b'0' - 1) as i8));
        let le_9 = _mm256_cmpgt_epi8(_mm256_set1_epi8((b'9' + 1) as i8), c);
        let m_09 = _mm256_and_si256(ge_0, le_9);
        let sh_09 = _mm256_and_si256(m_09, sh_p4);

        // Combine alphabet group shifts
        let mut shift = _mm256_blendv_epi8(sh_AZ, sh_az, m_az);
        shift = _mm256_or_si256(shift, sh_09);

        // + → 62
        let eq_plus = _mm256_cmpeq_epi8(c, v_plus);
        shift = _mm256_blendv_epi8(shift, sh_p19, eq_plus);

        // / → 63
        let eq_slash = _mm256_cmpeq_epi8(c, v_slash);
        shift = _mm256_blendv_epi8(shift, sh_p16, eq_slash);

        // URL-safe
        if url {
            let eq_dash = _mm256_cmpeq_epi8(c, _mm256_set1_epi8(b'-' as i8));
            shift = _mm256_blendv_epi8(shift, sh_p17, eq_dash);
            let eq_under = _mm256_cmpeq_epi8(c, _mm256_set1_epi8(b'_' as i8));
            shift = _mm256_blendv_epi8(shift, sh_n32, eq_under);
        }

        // Error detection
        let err = _mm256_cmpeq_epi8(shift, zero);
        let mask_lo = _mm_movemask_epi8(_mm256_castsi256_si128(err)) as u32;
        let mask_hi = _mm_movemask_epi8(_mm256_extracti128_si256::<1>(err)) as u32;
        if mask_lo != 0 || mask_hi != 0 {
            return Err(DecodeError::InvalidInput);
        }

        // 6-bit values
        let vals = _mm256_add_epi8(c, shift);

        // Pack 4×6-bit → 3×8-bit using Muła's maddubs+madd formula
        let merged = _mm256_maddubs_epi16(vals, maddubs_const);
        let packed = _mm256_madd_epi16(merged, madd_const);
        let compacted = _mm256_shuffle_epi8(packed, compact);

        let lo = _mm256_castsi256_si128(compacted);
        let hi = _mm256_extracti128_si256::<1>(compacted);

        _mm_storeu_si128(out.cast(), lo);
        core::ptr::copy_nonoverlapping(&hi as *const __m128i as *const u8, out.add(12), 12);

        inp = inp.add(32);
        out = out.add(24);
        len -= 32;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let out_len = len / 4 * 3
            + match len % 4 {
                0 => 0,
                2 => 1,
                3 => 2,
                _ => 0,
            };
        let out_slice = core::slice::from_raw_parts_mut(out, out_len);
        decode_into_constant_time(out_slice, data_slice, alphabet)?;
    }

    Ok(())
}
