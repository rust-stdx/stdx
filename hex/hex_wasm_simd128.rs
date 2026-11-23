use core::arch::wasm32::*;

use crate::{Alphabet, DecodeError, EncodeError};

pub fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    debug_assert!(output.len() >= data.len() * 2);

    let table_vec = unsafe {
        v128_load(
            match alphabet {
                Alphabet::Lower => super::ALPHABET_LOWER.as_ptr(),
                Alphabet::Upper => super::ALPHABET_UPPER.as_ptr(),
            }
            .cast(),
        )
    };
    let nibble_mask = i8x16_splat(0x0F);

    let mut i = 0;
    let len = data.len();

    while i + 16 <= len {
        let chunk = unsafe { v128_load(data.as_ptr().add(i).cast()) };

        let lo = v128_and(chunk, nibble_mask);
        let hi = u8x16_shr(chunk, 4);

        let hi_hex = i8x16_swizzle(table_vec, hi);
        let lo_hex = i8x16_swizzle(table_vec, lo);

        let result0 = i8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(hi_hex, lo_hex);
        let result1 = i8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31>(hi_hex, lo_hex);

        let o = i * 2;
        unsafe {
            v128_store(output.as_mut_ptr().add(o).cast(), result0);
            v128_store(output.as_mut_ptr().add(o + 16).cast(), result1);
        }

        i += 16;
    }

    if i < len {
        return crate::encode_into_constant_time(&mut output[i * 2..], &data[i..], alphabet);
    }

    Ok(())
}

pub fn decode_into(output: &mut [u8], input: &[u8]) -> Result<(), DecodeError> {
    debug_assert!(input.len().is_multiple_of(2));
    debug_assert!(output.len() >= input.len() / 2);

    let ge_0 = i8x16_splat(48);
    let le_9 = i8x16_splat(57);
    let ge_upper = i8x16_splat(65);
    let le_upper = i8x16_splat(70);
    let ge_a = i8x16_splat(97);
    let le_f = i8x16_splat(102);
    let digit_base = i8x16_splat(48);
    let upper_base = i8x16_splat(55);
    let lower_base = i8x16_splat(87);
    let nibble_max = i8x16_splat(15);
    let zero = i8x16_splat(0);

    let mut i = 0;
    let in_len = input.len();

    while i + 32 <= in_len {
        let c0 = unsafe { v128_load(input.as_ptr().add(i).cast()) };
        let c1 = unsafe { v128_load(input.as_ptr().add(i + 16).cast()) };

        let nibble0 = classify_nibble(
            c0, ge_0, le_9, ge_upper, le_upper, ge_a, le_f, digit_base, upper_base, lower_base,
        );
        let nibble1 = classify_nibble(
            c1, ge_0, le_9, ge_upper, le_upper, ge_a, le_f, digit_base, upper_base, lower_base,
        );

        let err0 = u8x16_gt(nibble0, nibble_max);
        let err1 = u8x16_gt(nibble1, nibble_max);
        let err = v128_or(err0, err1);
        if has_any_byte(err, zero) {
            return Err(DecodeError::InvalidInput);
        }

        let even = i8x16_shuffle::<0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30>(nibble0, nibble1);
        let odd = i8x16_shuffle::<1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31>(nibble0, nibble1);

        let result = v128_or(i8x16_shl(even, 4), odd);

        unsafe {
            v128_store(output.as_mut_ptr().add(i / 2).cast(), result);
        }

        i += 32;
    }

    if i < in_len {
        return crate::decode_into_constant_time(&mut output[i / 2..], &input[i..]);
    }

    Ok(())
}

#[cfg(test)]
mod kernel_tests {
    use super::*;

    /// Regression: every byte outside `0-9A-Fa-f` must be rejected, including
    /// the range boundaries `'@'` (0x40), `` '`' `` (0x60) and `/` (0x2f), and
    /// bytes that match no character class at all (which previously decoded as
    /// nibble 0 and were accepted).
    #[test]
    fn decode_rejects_every_invalid_byte_at_every_lane() {
        for pos in 0..32 {
            for val in 0u16..=255 {
                let byte = val as u8;
                if matches!(byte, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') {
                    continue;
                }
                let mut input = [b'0'; 32];
                input[pos] = byte;
                let mut out = [0u8; 16];
                assert_eq!(
                    decode_into(&mut out, &input),
                    Err(DecodeError::InvalidInput),
                    "pos={pos} val={val:#04x}"
                );
            }
        }
    }

    #[test]
    fn encode_matches_constant_time() {
        let data: [u8; 80] = core::array::from_fn(|i| (i * 37) as u8);
        for len in 0..=data.len() {
            for alphabet in [Alphabet::Lower, Alphabet::Upper] {
                let mut simd = [0u8; 160];
                let mut scalar = [0u8; 160];
                encode_into(&mut simd[..len * 2], &data[..len], alphabet).unwrap();
                crate::encode_into_constant_time(&mut scalar[..len * 2], &data[..len], alphabet).unwrap();
                assert_eq!(simd[..len * 2], scalar[..len * 2], "len={len}");
            }
        }
    }
}

#[inline]
fn classify_nibble(
    c: v128,
    ge_0: v128,
    le_9: v128,
    ge_upper: v128,
    le_upper: v128,
    ge_a: v128,
    le_f: v128,
    digit_base: v128,
    upper_base: v128,
    lower_base: v128,
) -> v128 {
    let is_digit = v128_and(u8x16_ge(c, ge_0), u8x16_le(c, le_9));
    let is_upper = v128_and(u8x16_ge(c, ge_upper), u8x16_le(c, le_upper));
    let is_lower = v128_and(u8x16_ge(c, ge_a), u8x16_le(c, le_f));

    let nd = i8x16_sub(c, digit_base);
    let nu = i8x16_sub(c, upper_base);
    let nl = i8x16_sub(c, lower_base);

    let valid = v128_or(v128_or(is_digit, is_upper), is_lower);
    let nibble = v128_or(v128_or(v128_and(is_digit, nd), v128_and(is_upper, nu)), v128_and(is_lower, nl));

    // Every classification mask is zero for a character outside `0-9A-Fa-f`,
    // so `nibble` would be zero and the caller's `nibble > 15` check would
    // accept it. Mark invalid bytes with 0xFF so the check rejects them.
    let invalid = v128_xor(valid, i8x16_splat(-1));
    v128_or(nibble, invalid)
}

#[inline]
fn has_any_byte(v: v128, zero: v128) -> bool {
    let nonzero_mask = u8x16_gt(v, zero);
    i8x16_bitmask(nonzero_mask) != 0
}
