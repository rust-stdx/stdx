// SAFETY: all intrinsics are only called from functions that have #[target_feature(enable = "avx2")]
#![allow(unsafe_op_in_unsafe_fn)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{Alphabet, DecodeError, EncodeError};

/// Encode `data` as hex into `output` using AVX2.
///
/// Processes 32 input bytes per iteration using `_mm256_shuffle_epi8`
/// table lookups, then encodes any remaining bytes (< 32) via
/// [`encode_into_constant_time`].
///
/// # Safety
///
/// The caller must ensure AVX2 is available on the current CPU
/// (e.g. via `is_x86_feature_detected!("avx2")`). Calling this on a
/// CPU without AVX2 support will cause an illegal instruction fault.
#[target_feature(enable = "avx2")]
pub unsafe fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    debug_assert!(output.len() == data.len() * 2);

    let table = _mm256_broadcastsi128_si256(_mm_loadu_si128(
        match alphabet {
            Alphabet::Lower => super::ALPHABET_LOWER.as_ptr(),
            Alphabet::Upper => super::ALPHABET_UPPER.as_ptr(),
        }
        .cast(),
    ));
    let nibble_mask = _mm256_set1_epi8(0x0F);

    let mut i = 0;
    let len = data.len();

    while i + 32 <= len {
        let chunk = _mm256_loadu_si256(data.as_ptr().add(i).cast());

        let lo = _mm256_and_si256(chunk, nibble_mask);
        let hi = _mm256_and_si256(_mm256_srli_epi16(chunk, 4), nibble_mask);

        let lo_hex = _mm256_shuffle_epi8(table, lo);
        let hi_hex = _mm256_shuffle_epi8(table, hi);

        let tmp0 = _mm256_unpacklo_epi8(hi_hex, lo_hex);
        let tmp1 = _mm256_unpackhi_epi8(hi_hex, lo_hex);

        let result0 = _mm256_permute2x128_si256(tmp0, tmp1, 0x20);
        let result1 = _mm256_permute2x128_si256(tmp0, tmp1, 0x31);

        let o = i * 2;
        _mm256_storeu_si256(output.as_mut_ptr().add(o).cast(), result0);
        _mm256_storeu_si256(output.as_mut_ptr().add(o + 32).cast(), result1);

        i += 32;
    }

    if i < len {
        return crate::encode_into_constant_time(&mut output[i * 2..], &data[i..], alphabet);
    }

    Ok(())
}

/// Decode hex `input` into `output` using AVX2.
///
/// Processes 32 hex chars (16 output bytes) per iteration using range
/// comparison for character classification and nibble extraction, then
/// decodes any remaining hex chars (< 32) via
/// [`decode_into_constant_time`].
///
/// # Safety
///
/// The caller must ensure AVX2 is available on the current CPU
/// (e.g. via `is_x86_feature_detected!("avx2")`). Calling this on a
/// CPU without AVX2 support will cause an illegal instruction fault.
#[target_feature(enable = "avx2")]
pub unsafe fn decode_into(output: &mut [u8], input: &[u8]) -> Result<(), DecodeError> {
    debug_assert!(input.len().is_multiple_of(2));
    debug_assert!(output.len() == input.len() / 2);

    let zero = _mm256_setzero_si256();

    let digit_min = _mm256_set1_epi8(b'0' as i8);
    let digit_max = _mm256_set1_epi8(b'9' as i8);
    let upper_min = _mm256_set1_epi8(b'A' as i8);
    let upper_max = _mm256_set1_epi8(b'F' as i8);
    let lower_min = _mm256_set1_epi8(b'a' as i8);
    let lower_max = _mm256_set1_epi8(b'f' as i8);
    let digit_base = _mm256_set1_epi8(b'0' as i8);
    let upper_base = _mm256_set1_epi8((b'A' - 10) as i8);
    let lower_base = _mm256_set1_epi8((b'a' - 10) as i8);
    let mult = _mm256_set1_epi16(0x0110);

    let mut i = 0;
    let in_len = input.len();

    while i + 32 <= in_len {
        let c = _mm256_loadu_si256(input.as_ptr().add(i).cast());

        let is_digit_ge = _mm256_cmpeq_epi8(_mm256_max_epu8(c, digit_min), c);
        let is_digit_le = _mm256_cmpeq_epi8(_mm256_min_epu8(c, digit_max), c);
        let is_digit = _mm256_and_si256(is_digit_ge, is_digit_le);

        let is_upper_ge = _mm256_cmpeq_epi8(_mm256_max_epu8(c, upper_min), c);
        let is_upper_le = _mm256_cmpeq_epi8(_mm256_min_epu8(c, upper_max), c);
        let is_upper = _mm256_and_si256(is_upper_ge, is_upper_le);

        let is_lower_ge = _mm256_cmpeq_epi8(_mm256_max_epu8(c, lower_min), c);
        let is_lower_le = _mm256_cmpeq_epi8(_mm256_min_epu8(c, lower_max), c);
        let is_lower = _mm256_and_si256(is_lower_ge, is_lower_le);

        let valid = _mm256_or_si256(is_digit, _mm256_or_si256(is_upper, is_lower));

        if _mm256_movemask_epi8(valid) != -1i32 {
            return Err(DecodeError::InvalidInput);
        }

        let nd = _mm256_sub_epi8(c, digit_base);
        let nu = _mm256_sub_epi8(c, upper_base);
        let nl = _mm256_sub_epi8(c, lower_base);

        let nibble = _mm256_or_si256(
            _mm256_or_si256(_mm256_and_si256(is_digit, nd), _mm256_and_si256(is_upper, nu)),
            _mm256_and_si256(is_lower, nl),
        );

        let packed16 = _mm256_maddubs_epi16(nibble, mult);

        let packed8 = _mm256_packus_epi16(packed16, zero);

        let ordered = _mm256_permute4x64_epi64(packed8, 0xD8);

        _mm_storeu_si128(output.as_mut_ptr().add(i / 2).cast(), _mm256_castsi256_si128(ordered));

        i += 32;
    }

    if i < in_len {
        return crate::decode_into_constant_time(&mut output[i / 2..], &input[i..]);
    }

    Ok(())
}
