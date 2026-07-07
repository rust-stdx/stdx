// SAFETY: all intrinsics are only called from functions that have #[target_feature(enable = "neon")]
#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::aarch64::*;

use crate::{Alphabet, DecodeError, EncodeError, decode_into_constant_time, encode_into_constant_time};

const ALPHABET_STANDARD: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const ALPHABET_URL: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

const ENCODE_NEON_STANDARD: [u8; 64] = {
    let src = ALPHABET_STANDARD;
    let mut dst = [0u8; 64];
    let mut i = 0;
    while i < 16 {
        dst[4 * i] = src[i];
        dst[4 * i + 1] = src[16 + i];
        dst[4 * i + 2] = src[32 + i];
        dst[4 * i + 3] = src[48 + i];
        i += 1;
    }
    dst
};

const ENCODE_NEON_URL_SAFE: [u8; 64] = {
    let src = ALPHABET_URL;
    let mut dst = [0u8; 64];
    let mut i = 0;
    while i < 16 {
        dst[4 * i] = src[i];
        dst[4 * i + 1] = src[16 + i];
        dst[4 * i + 2] = src[32 + i];
        dst[4 * i + 3] = src[48 + i];
        i += 1;
    }
    dst
};

#[target_feature(enable = "neon")]
pub unsafe fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    let table_bytes: &[u8; 64] = match alphabet {
        Alphabet::Standard | Alphabet::StandardNoPadding => &ENCODE_NEON_STANDARD,
        Alphabet::Url | Alphabet::UrlNoPadding => &ENCODE_NEON_URL_SAFE,
    };

    let lut_q = vld4q_u8(table_bytes.as_ptr());
    let v3f = vdupq_n_u8(0x3F);

    let mut inp = data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = data.len();

    while len >= 48 {
        let p = vld3q_u8(inp);

        let a = vshrq_n_u8::<2>(p.0);
        let b = vandq_u8(vorrq_u8(vshlq_n_u8::<4>(p.0), vshrq_n_u8::<4>(p.1)), v3f);
        let c = vandq_u8(vorrq_u8(vshlq_n_u8::<2>(p.1), vshrq_n_u8::<6>(p.2)), v3f);
        let d = vandq_u8(p.2, v3f);

        let r = uint8x16x4_t(
            vqtbl4q_u8(lut_q, a),
            vqtbl4q_u8(lut_q, b),
            vqtbl4q_u8(lut_q, c),
            vqtbl4q_u8(lut_q, d),
        );

        vst4q_u8(out, r);

        inp = inp.add(48);
        out = out.add(64);
        len -= 48;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let out_len = match len % 3 {
            0 => len / 3 * 4,
            1 => {
                len / 3 * 4
                    + if matches!(alphabet, Alphabet::Standard | Alphabet::Url) {
                        4
                    } else {
                        2
                    }
            }
            _ => {
                len / 3 * 4
                    + if matches!(alphabet, Alphabet::Standard | Alphabet::Url) {
                        4
                    } else {
                        3
                    }
            }
        };
        let out_slice = core::slice::from_raw_parts_mut(out, out_len);
        return encode_into_constant_time(out_slice, data_slice, alphabet);
    }

    Ok(())
}

#[target_feature(enable = "neon")]
pub unsafe fn decode_into(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let url = matches!(alphabet, Alphabet::Url | Alphabet::UrlNoPadding);
    let mut inp = encoded_data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = encoded_data.len();

    while len >= 32 {
        let chunk = vld4_u8(inp);

        let (sa, ea) = lookup_byte_blend(chunk.0, url);
        let (sb, eb) = lookup_byte_blend(chunk.1, url);
        let (sc, ec) = lookup_byte_blend(chunk.2, url);
        let (sd, ed) = lookup_byte_blend(chunk.3, url);

        let e0 = vorr_u8(ea, eb);
        let e1 = vorr_u8(ec, ed);
        let err = vorr_u8(e0, e1);
        let mut buf = [0u8; 8];
        vst1_u8(buf.as_mut_ptr(), err);
        if u64::from_ne_bytes(buf) != 0 {
            return Err(DecodeError::InvalidInput);
        }

        let va = vadd_u8(chunk.0, sa);
        let vb = vadd_u8(chunk.1, sb);
        let vc = vadd_u8(chunk.2, sc);
        let vd = vadd_u8(chunk.3, sd);

        let r0 = vorr_u8(vshl_n_u8::<2>(va), vshr_n_u8::<4>(vb));
        let r1 = vorr_u8(vshl_n_u8::<4>(vb), vshr_n_u8::<2>(vc));
        let r2 = vorr_u8(vshl_n_u8::<6>(vc), vd);

        vst3_u8(out, uint8x8x3_t(r0, r1, r2));

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

#[inline]
unsafe fn lookup_byte_blend(input: uint8x8_t, url: bool) -> (uint8x8_t, uint8x8_t) {
    let z = vdup_n_u8(0);

    let ge_a_upper = vcgt_u8(input, vdup_n_u8(b'A' - 1));
    let le_z_upper = vclt_u8(input, vdup_n_u8(b'Z' + 1));
    let m_az_upper = vand_u8(ge_a_upper, le_z_upper);
    let sh_az_upper = vand_u8(m_az_upper, vdup_n_u8((-65i8) as u8));

    let ge_a = vcgt_u8(input, vdup_n_u8(b'a' - 1));
    let le_z = vclt_u8(input, vdup_n_u8(b'z' + 1));
    let m_az = vand_u8(ge_a, le_z);
    let sh_az = vand_u8(m_az, vdup_n_u8((-71i8) as u8));

    let ge_0 = vcgt_u8(input, vdup_n_u8(b'0' - 1));
    let le_9 = vclt_u8(input, vdup_n_u8(b'9' + 1));
    let m_09 = vand_u8(ge_0, le_9);
    let sh_09 = vand_u8(m_09, vdup_n_u8(4u8));

    let mut s = vbsl_u8(m_az, sh_az, sh_az_upper);
    s = vorr_u8(s, sh_09);

    let eq_plus = vceq_u8(input, vdup_n_u8(b'+'));
    s = vbsl_u8(eq_plus, vdup_n_u8(19u8), s);
    let eq_slash = vceq_u8(input, vdup_n_u8(b'/'));
    s = vbsl_u8(eq_slash, vdup_n_u8(16u8), s);

    if url {
        let eq_dash = vceq_u8(input, vdup_n_u8(b'-'));
        s = vbsl_u8(eq_dash, vdup_n_u8(17u8), s);
        let eq_under = vceq_u8(input, vdup_n_u8(b'_'));
        s = vbsl_u8(eq_under, vdup_n_u8((-32i8) as u8), s);
    }

    let err = vceq_u8(s, z);
    (s, err)
}
