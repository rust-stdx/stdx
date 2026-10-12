use core::arch::aarch64::*;

use crate::{Alphabet, DecodeError, EncodeError};

/// Encode `data` as hex into `output` using NEON.
///
/// Processes 16 input bytes per iteration using `vqtbl1q_u8` table
/// lookups with `vzip1q_u8`/`vzip2q_u8` interleaving, then encodes any
/// remaining bytes (< 16) via [`encode_into_constant_time`].
///
/// # Safety
///
/// The caller must ensure NEON is available on the current CPU
/// (e.g. via `is_aarch64_feature_detected!("neon")`). Calling this on a
/// CPU without NEON support will cause an illegal instruction fault.
#[target_feature(enable = "neon")]
pub unsafe fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    debug_assert!(output.len() >= data.len() * 2);

    let table = unsafe {
        vld1q_u8(match alphabet {
            Alphabet::Lower => super::ALPHABET_LOWER.as_ptr(),
            Alphabet::Upper => super::ALPHABET_UPPER.as_ptr(),
        })
    };
    let nibble_mask = vdupq_n_u8(0x0F);

    let mut i = 0;
    let len = data.len();

    while i + 16 <= len {
        let chunk = unsafe { vld1q_u8(data.as_ptr().add(i)) };

        let lo = vandq_u8(chunk, nibble_mask);
        let hi = vshrq_n_u8(chunk, 4);

        let lo_hex = vqtbl1q_u8(table, lo);
        let hi_hex = vqtbl1q_u8(table, hi);

        let result0 = vzip1q_u8(hi_hex, lo_hex);
        let result1 = vzip2q_u8(hi_hex, lo_hex);

        let o = i * 2;
        unsafe {
            vst1q_u8(output.as_mut_ptr().add(o), result0);
            vst1q_u8(output.as_mut_ptr().add(o + 16), result1);
        }

        i += 16;
    }

    if i < len {
        return crate::encode_into_constant_time(&mut output[i * 2..], &data[i..], alphabet);
    }

    Ok(())
}

/// Decode hex `input` into `output` using NEON.
///
/// Processes 32 hex chars (16 output bytes) per iteration using range
/// comparison for character classification and nibble extraction, then
/// decodes any remaining hex chars (< 32) via
/// [`decode_into_constant_time`].
///
/// # Safety
///
/// The caller must ensure NEON is available on the current CPU
/// (e.g. via `is_aarch64_feature_detected!("neon")`). Calling this on a
/// CPU without NEON support will cause an illegal instruction fault.
#[allow(non_snake_case)]
#[target_feature(enable = "neon")]
pub unsafe fn decode_into(output: &mut [u8], input: &[u8]) -> Result<(), DecodeError> {
    debug_assert!(input.len().is_multiple_of(2));
    debug_assert!(output.len() >= input.len() / 2);

    let zero_u16 = vdupq_n_u16(0);

    let ge_0 = vdupq_n_u8(47);
    let le_9 = vdupq_n_u8(57);
    let ge_A = vdupq_n_u8(64);
    let le_F = vdupq_n_u8(70);
    let ge_a = vdupq_n_u8(96);
    let le_f = vdupq_n_u8(102);
    let digit_base = vdupq_n_u8(48);
    let upper_base = vdupq_n_u8(55);
    let lower_base = vdupq_n_u8(87);

    let mut i = 0;
    let in_len = input.len();

    while i + 32 <= in_len {
        let c0 = unsafe { vld1q_u8(input.as_ptr().add(i)) };
        let c1 = unsafe { vld1q_u8(input.as_ptr().add(i + 16)) };

        for (j, c) in [c0, c1].into_iter().enumerate() {
            let is_digit = vandq_u8(vcgtq_u8(c, ge_0), vcleq_u8(c, le_9));
            let is_upper = vandq_u8(vcgtq_u8(c, ge_A), vcleq_u8(c, le_F));
            let is_lower = vandq_u8(vcgtq_u8(c, ge_a), vcleq_u8(c, le_f));

            let valid = vorrq_u8(vorrq_u8(is_digit, is_upper), is_lower);

            let m0 = vandq_u32(
                vreinterpretq_u32_u8(valid),
                vextq_u32(vreinterpretq_u32_u8(valid), vreinterpretq_u32_u8(valid), 2),
            );
            let m1 = vandq_u32(m0, vextq_u32(m0, m0, 1));
            if vgetq_lane_u32(m1, 0) != 0xFFFF_FFFF {
                return Err(DecodeError::InvalidInput);
            }

            let nd = vsubq_u8(c, digit_base);
            let nu = vsubq_u8(c, upper_base);
            let nl = vsubq_u8(c, lower_base);

            let nibble_byte =
                vorrq_u8(vorrq_u8(vandq_u8(is_digit, nd), vandq_u8(is_upper, nu)), vandq_u8(is_lower, nl));

            let w_lo = vmovl_u8(vget_low_u8(nibble_byte));
            let w_hi = vmovl_u8(vget_high_u8(nibble_byte));

            let mult = vreinterpretq_u8_u16(vdupq_n_u16(0x0110));
            let w_m_lo = vmovl_u8(vget_low_u8(mult));
            let w_m_hi = vmovl_u8(vget_high_u8(mult));

            let p_lo = vmulq_u16(w_lo, w_m_lo);
            let p_hi = vmulq_u16(w_hi, w_m_hi);

            let s_lo = vpaddq_u16(p_lo, zero_u16);
            let s_hi = vpaddq_u16(p_hi, zero_u16);

            let packed = vcombine_u16(vget_low_u16(s_lo), vget_low_u16(s_hi));
            let out_bytes = vmovn_u16(packed);

            unsafe {
                vst1_u8(output.as_mut_ptr().add(i / 2 + j * 8), out_bytes);
            }
        }

        i += 32;
    }

    if i < in_len {
        return crate::decode_into_constant_time(&mut output[i / 2..], &input[i..]);
    }

    Ok(())
}
