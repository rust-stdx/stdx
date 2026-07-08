use core::arch::wasm32::*;

use crate::{Alphabet, DecodeError, EncodeError, decode_into_constant_time, encode_into_constant_time};

pub fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    let url = matches!(alphabet, Alphabet::Url | Alphabet::UrlNoPadding);

    let mut inp = data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = data.len();

    let splat_25 = i8x16_splat(25);
    let splat_51 = i8x16_splat(51);
    let splat_62 = i8x16_splat(62);
    let splat_63 = i8x16_splat(63);
    let splat_65 = i8x16_splat(65);
    let splat_6 = i8x16_splat(6);
    let splat_n75 = i8x16_splat((-75i8) as u8 as i8);
    let splat_241 = i8x16_splat((-15i8) as u8 as i8);
    let splat_n12 = i8x16_splat((-12i8) as u8 as i8);
    let splat_0x3F = i8x16_splat(0x3F);
    let zero = i8x16_splat(0);

    while len >= 12 {
        let chunk = unsafe { v128_load(inp.cast()) };

        let lanes = i8x16_shuffle::<2, 1, 0, 16, 5, 4, 3, 17, 8, 7, 6, 18, 11, 10, 9, 19>(chunk, zero);

        let c3 = v128_and(lanes, splat_0x3F);
        let c2 = v128_and(u32x4_shr(lanes, 6), splat_0x3F);
        let c1 = v128_and(u32x4_shr(lanes, 12), splat_0x3F);
        let c0 = u32x4_shr(lanes, 18);

        let bytes = v128_or(c0, v128_or(u32x4_shl(c1, 8), v128_or(u32x4_shl(c2, 16), u32x4_shl(c3, 24))));

        let mut result = i8x16_add(bytes, splat_65);
        let gt_25 = u8x16_gt(bytes, splat_25);
        result = i8x16_add(result, v128_and(gt_25, splat_6));
        let gt_51 = u8x16_gt(bytes, splat_51);
        result = i8x16_add(result, v128_and(gt_51, splat_n75));
        let eq_62 = i8x16_eq(bytes, splat_62);
        result = i8x16_add(result, v128_and(eq_62, splat_241));
        let eq_63 = i8x16_eq(bytes, splat_63);
        result = i8x16_add(result, v128_and(eq_63, splat_n12));

        if url {
            let eq_62 = i8x16_eq(bytes, splat_62);
            result = v128_bitselect(i8x16_splat(b'-' as i8), result, eq_62);
            let eq_63 = i8x16_eq(bytes, splat_63);
            result = v128_bitselect(i8x16_splat(b'_' as i8), result, eq_63);
        }

        unsafe {
            v128_store(out.cast(), result);
            inp = inp.add(12);
            out = out.add(16);
        }
        len -= 12;
    }

    if len > 0 {
        let data_slice = unsafe { core::slice::from_raw_parts(inp, len) };
        let padded = alphabet.is_padded();
        let out_len = match len % 3 {
            0 => (len / 3) * 4,
            1 => (len / 3) * 4 + if padded { 4 } else { 2 },
            _ => (len / 3) * 4 + if padded { 4 } else { 3 },
        };
        let out_slice = unsafe { core::slice::from_raw_parts_mut(out, out_len) };
        return encode_into_constant_time(out_slice, data_slice, alphabet);
    }

    Ok(())
}

pub fn decode_into(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let url = matches!(alphabet, Alphabet::Url | Alphabet::UrlNoPadding);

    let mut inp = encoded_data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = encoded_data.len();

    let zero = i8x16_splat(0);
    let splat_63 = i8x16_splat(63);

    while len >= 16 {
        let chunk = unsafe { v128_load(inp.cast()) };

        let v0 = i8x16_shuffle::<0, 4, 8, 12, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16>(chunk, zero);
        let v1 = i8x16_shuffle::<1, 5, 9, 13, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16>(chunk, zero);
        let v2 = i8x16_shuffle::<2, 6, 10, 14, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16>(chunk, zero);
        let v3 = i8x16_shuffle::<3, 7, 11, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16>(chunk, zero);

        let sextets = combine_sextets(v0, v1, v2, v3, url);

        let err = u8x16_gt(sextets.0, splat_63);
        let err_bits = i8x16_bitmask(err);
        if err_bits != 0 {
            return Err(DecodeError::InvalidInput);
        }

        let b0 = v128_or(i8x16_shl(sextets.0, 2), u8x16_shr(sextets.1, 4));
        let b1 = v128_or(i8x16_shl(sextets.1, 4), u8x16_shr(sextets.2, 2));
        let b2 = v128_or(i8x16_shl(sextets.2, 6), sextets.3);

        let z01 = i8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(b0, b1);
        let result = i8x16_shuffle::<0, 1, 16, 2, 3, 17, 4, 5, 18, 6, 7, 19, 8, 9, 10, 11>(z01, b2);

        unsafe {
            core::ptr::copy_nonoverlapping(&result as *const v128 as *const u8, out, 12);
            inp = inp.add(16);
            out = out.add(12);
        }
        len -= 16;
    }

    if len > 0 {
        let data_slice = unsafe { core::slice::from_raw_parts(inp, len) };
        let out_len = len / 4 * 3
            + match len % 4 {
                0 => 0,
                2 => 1,
                3 => 2,
                _ => 0,
            };
        let out_slice = unsafe { core::slice::from_raw_parts_mut(out, out_len) };
        decode_into_constant_time(out_slice, data_slice, alphabet)?;
    }

    Ok(())
}

#[inline]
fn combine_sextets(v0: v128, v1: v128, v2: v128, v3: v128, url: bool) -> (v128, v128, v128, v128) {
    let s0 = classify_sextet(v0, url);
    let s1 = classify_sextet(v1, url);
    let s2 = classify_sextet(v2, url);
    let s3 = classify_sextet(v3, url);
    (s0, s1, s2, s3)
}

#[inline]
fn classify_sextet(c: v128, url: bool) -> v128 {
    let ge_A = u8x16_ge(c, i8x16_splat(b'A' as i8));
    let le_Z = u8x16_le(c, i8x16_splat(b'Z' as i8));
    let m_AZ = v128_and(ge_A, le_Z);
    let sh_AZ = v128_and(m_AZ, i8x16_splat((-65i8) as u8 as i8));

    let ge_a = u8x16_ge(c, i8x16_splat(b'a' as i8));
    let le_z = u8x16_le(c, i8x16_splat(b'z' as i8));
    let m_az = v128_and(ge_a, le_z);
    let sh_az = v128_and(m_az, i8x16_splat((-71i8) as u8 as i8));

    let ge_0 = u8x16_ge(c, i8x16_splat(b'0' as i8));
    let le_9 = u8x16_le(c, i8x16_splat(b'9' as i8));
    let m_09 = v128_and(ge_0, le_9);
    let sh_09 = v128_and(m_09, i8x16_splat(4i8));

    let mut s = v128_bitselect(sh_az, sh_AZ, m_az);
    s = v128_or(s, sh_09);

    let eq_plus = i8x16_eq(c, i8x16_splat(b'+' as i8));
    s = v128_bitselect(i8x16_splat(19i8), s, eq_plus);

    let eq_slash = i8x16_eq(c, i8x16_splat(b'/' as i8));
    s = v128_bitselect(i8x16_splat(16i8), s, eq_slash);

    if url {
        let eq_dash = i8x16_eq(c, i8x16_splat(b'-' as i8));
        s = v128_bitselect(i8x16_splat(17i8), s, eq_dash);
        let eq_under = i8x16_eq(c, i8x16_splat(b'_' as i8));
        s = v128_bitselect(i8x16_splat((-32i8) as u8 as i8), s, eq_under);
    }

    i8x16_add(c, s)
}
