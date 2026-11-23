// SAFETY: all intrinsics are only called from functions that have #[target_feature(enable = "neon")]
#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::aarch64::*;

use crate::{Alphabet, DecodeError, EncodeError, decode_into_constant_time};

const NEON_MAP_RFC4648: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const NEON_MAP_RFC4648_LOWER: [u8; 32] = *b"abcdefghijklmnopqrstuvwxyz234567";
const NEON_MAP_RFC4648_HEX: [u8; 32] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const NEON_MAP_RFC4648_HEX_LOWER: [u8; 32] = *b"0123456789abcdefghijklmnopqrstuv";
const NEON_MAP_CROCKFORD: [u8; 32] = *b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const NEON_MAP_Z32: [u8; 32] = *b"ybndrfg8ejkmcpqxot1uwisza345h769";

// =============================================================================
// Encode
// =============================================================================

#[target_feature(enable = "neon")]
pub unsafe fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    let tbl_bytes: &[u8; 32] = match alphabet {
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => &NEON_MAP_RFC4648,
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => &NEON_MAP_RFC4648_LOWER,
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => &NEON_MAP_RFC4648_HEX,
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => &NEON_MAP_RFC4648_HEX_LOWER,
        Alphabet::Crockford => &NEON_MAP_CROCKFORD,
        Alphabet::Z32 => &NEON_MAP_Z32,
    };

    let tbl0 = vld1q_u8(tbl_bytes.as_ptr());
    let tbl1 = vld1q_u8(tbl_bytes.as_ptr().add(16));

    let mut inp = data.as_ptr();
    let mut out = output.as_mut_ptr();
    let mut len = data.len();

    // 4 blocks = 20 bytes → 32 chars per iteration
    while len >= 20 {
        let b = |i: usize| inp.add(i).read_unaligned();

        let q01 = simd_quintets_2blocks(b(0), b(1), b(2), b(3), b(4), b(5), b(6), b(7), b(8), b(9));
        let (c0, c1) = simd_vtbl_16(q01, tbl0, tbl1);
        vst1_u8(out, c0);
        vst1_u8(out.add(8), c1);

        let q23 = simd_quintets_2blocks(b(10), b(11), b(12), b(13), b(14), b(15), b(16), b(17), b(18), b(19));
        let (c2, c3) = simd_vtbl_16(q23, tbl0, tbl1);
        vst1_u8(out.add(16), c2);
        vst1_u8(out.add(24), c3);

        inp = inp.add(20);
        out = out.add(32);
        len -= 20;
    }

    // 2 blocks = 10 bytes → 16 chars
    while len >= 10 {
        let b = |i: usize| inp.add(i).read_unaligned();

        let q01 = simd_quintets_2blocks(b(0), b(1), b(2), b(3), b(4), b(5), b(6), b(7), b(8), b(9));
        let (c0, c1) = simd_vtbl_16(q01, tbl0, tbl1);
        vst1_u8(out, c0);
        vst1_u8(out.add(8), c1);

        inp = inp.add(10);
        out = out.add(16);
        len -= 10;
    }

    // 1 block = 5 bytes → 8 chars
    if len >= 5 {
        let b = |i: usize| inp.add(i).read_unaligned();

        let q = simd_quintets_1block(b(0), b(1), b(2), b(3), b(4));
        let (c, _) = simd_vtbl_16(q, tbl0, tbl1);
        vst1_u8(out, c);

        inp = inp.add(5);
        out = out.add(8);
        len -= 5;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let out_len = crate::encoded_length(len, alphabet.is_padded()).expect("encoded length overflow");
        let out_slice = core::slice::from_raw_parts_mut(out, out_len);
        return crate::encode_into_constant_time(out_slice, data_slice, alphabet);
    }

    Ok(())
}

/// Compute 16 quintets from 2 blocks (10 bytes) in a uint8x16_t.
unsafe fn simd_quintets_2blocks(
    b0: u8,
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    b5: u8,
    b6: u8,
    b7: u8,
    b8: u8,
    b9: u8,
) -> uint8x16_t {
    let mut buf = [0u8; 16];

    buf[0] = b0 >> 3;
    buf[1] = ((b0 & 0x07) << 2) | (b1 >> 6);
    buf[2] = (b1 >> 1) & 0x1F;
    buf[3] = ((b1 & 0x01) << 4) | (b2 >> 4);
    buf[4] = ((b2 & 0x0F) << 1) | (b3 >> 7);
    buf[5] = (b3 >> 2) & 0x1F;
    buf[6] = ((b3 & 0x03) << 3) | (b4 >> 5);
    buf[7] = b4 & 0x1F;

    buf[8] = b5 >> 3;
    buf[9] = ((b5 & 0x07) << 2) | (b6 >> 6);
    buf[10] = (b6 >> 1) & 0x1F;
    buf[11] = ((b6 & 0x01) << 4) | (b7 >> 4);
    buf[12] = ((b7 & 0x0F) << 1) | (b8 >> 7);
    buf[13] = (b8 >> 2) & 0x1F;
    buf[14] = ((b8 & 0x03) << 3) | (b9 >> 5);
    buf[15] = b9 & 0x1F;

    vld1q_u8(buf.as_ptr())
}

/// Compute 8 quintets from 1 block (5 bytes) in a uint8x16_t (upper 8 zeroed).
unsafe fn simd_quintets_1block(b0: u8, b1: u8, b2: u8, b3: u8, b4: u8) -> uint8x16_t {
    let mut buf = [0u8; 16];

    buf[0] = b0 >> 3;
    buf[1] = ((b0 & 0x07) << 2) | (b1 >> 6);
    buf[2] = (b1 >> 1) & 0x1F;
    buf[3] = ((b1 & 0x01) << 4) | (b2 >> 4);
    buf[4] = ((b2 & 0x0F) << 1) | (b3 >> 7);
    buf[5] = (b3 >> 2) & 0x1F;
    buf[6] = ((b3 & 0x03) << 3) | (b4 >> 5);
    buf[7] = b4 & 0x1F;

    vld1q_u8(buf.as_ptr())
}

/// Map 16 quintets to chars via vtbl. tbl0 covers indices [0..15], tbl1 covers [16..31].
/// Returns two uint8x8_t with 8 chars each.
#[inline]
unsafe fn simd_vtbl_16(indices: uint8x16_t, tbl0: uint8x16_t, tbl1: uint8x16_t) -> (uint8x8_t, uint8x8_t) {
    let in_range = vandq_u8(indices, vdupq_n_u8(0x0F));
    let use_tbl0 = vcgtq_u8(vdupq_n_u8(0x10), indices);
    let lo = vqtbl1q_u8(tbl0, in_range);
    let hi = vqtbl1q_u8(tbl1, in_range);
    let chars = vbslq_u8(use_tbl0, lo, hi);
    (vget_low_u8(chars), vget_high_u8(chars))
}

// =============================================================================
// Decode
// =============================================================================

#[target_feature(enable = "neon")]
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
        let out_slice = core::slice::from_raw_parts_mut(out, out_len);
        decode_into_constant_time(out_slice, data_slice, alphabet.without_padding())
    } else {
        Ok(())
    }
}

/// SIMD decode for 2-range alphabets.
///
/// Each alphabet maps input chars to quintets via two contiguous ranges:
///   Range 1: [r1_lo_char, r1_hi_char] → quintet = char - r1_sub
///   Range 2: [r2_lo_char, r2_hi_char] → quintet = char - r2_sub
///
/// The subtraction values are precomputed so that:
///   char - r1_sub yields the correct quintet (0-based for range 1)
///   char - r2_sub yields the correct quintet (offset for range 2)
unsafe fn decode_simd(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let (r1_lo_char, r1_hi_char, r1_sub, r2_lo_char, r2_hi_char, r2_sub) = match alphabet {
        // 'A'..'Z' → 0..25,        '2'..'7' → 26..31
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => (b'A', b'Z', b'A', b'2', b'7', b'2' - 26),
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => (b'a', b'z', b'a', b'2', b'7', b'2' - 26),
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => (b'0', b'9', b'0', b'A', b'V', b'A' - 10),
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => (b'0', b'9', b'0', b'a', b'v', b'a' - 10),
        _ => unreachable!(),
    };

    let r1_lo = vdup_n_u8(r1_lo_char);
    let r1_hi = vdup_n_u8(r1_hi_char);
    let r1_sub = vdup_n_u8(r1_sub);
    let r2_lo = vdup_n_u8(r2_lo_char);
    let r2_hi = vdup_n_u8(r2_hi_char);
    let r2_sub = vdup_n_u8(r2_sub);

    let mut inp = encoded_data.as_ptr();
    let mut out_ptr = output.as_mut_ptr();
    let mut len = encoded_data.len();

    while len >= 8 {
        let chars = vld1_u8(inp);

        let quintets = simd_char_to_quintet_2ranges(chars, r1_lo, r1_hi, r1_sub, r2_lo, r2_hi, r2_sub);

        if simd_validate_8(quintets) {
            return Err(DecodeError::InvalidInput);
        }

        pack_and_store(quintets, out_ptr);

        inp = inp.add(8);
        out_ptr = out_ptr.add(5);
        len -= 8;
    }

    if len > 0 {
        let data_slice = core::slice::from_raw_parts(inp, len);
        let out_len = len * 5 / 8;
        let out_slice = core::slice::from_raw_parts_mut(out_ptr, out_len);
        decode_into_constant_time(out_slice, data_slice, alphabet.without_padding())
    } else {
        Ok(())
    }
}

/// Vectorized char→quintet for 2 contiguous ranges.
/// Range 1: char in [r1_lo, r1_hi] → quintet = char - r1_sub
/// Range 2: char in [r2_lo, r2_hi] → quintet = char - r2_sub
/// Invalid chars produce a value with bit 5 set.
#[inline]
unsafe fn simd_char_to_quintet_2ranges(
    chars: uint8x8_t,
    r1_lo: uint8x8_t,
    r1_hi: uint8x8_t,
    r1_sub: uint8x8_t,
    r2_lo: uint8x8_t,
    r2_hi: uint8x8_t,
    r2_sub: uint8x8_t,
) -> uint8x8_t {
    let in_r1 = vand_u8(vcge_u8(chars, r1_lo), vcle_u8(chars, r1_hi));
    let in_r2 = vand_u8(vcge_u8(chars, r2_lo), vcle_u8(chars, r2_hi));

    let val_r1 = vsub_u8(chars, r1_sub);
    let val_r2 = vsub_u8(chars, r2_sub);

    let tmp = vbsl_u8(in_r2, val_r2, vdup_n_u8(0x20));
    vbsl_u8(in_r1, val_r1, tmp)
}

/// Returns true if any lane in the 8-element vector has bit 5 set (invalid).
#[inline]
unsafe fn simd_validate_8(q: uint8x8_t) -> bool {
    let acc = vorr_u8(q, vext_u8(q, q, 4));
    let acc = vorr_u8(acc, vext_u8(acc, acc, 2));
    let acc = vorr_u8(acc, vext_u8(acc, acc, 1));
    vget_lane_u8::<0>(acc) >= 32
}

/// Pack 8 quintets (lanes of uint8x8_t) into 5 bytes and write to output.
#[inline]
unsafe fn pack_and_store(quintets: uint8x8_t, out: *mut u8) {
    let q0 = vget_lane_u8::<0>(quintets);
    let q1 = vget_lane_u8::<1>(quintets);
    let q2 = vget_lane_u8::<2>(quintets);
    let q3 = vget_lane_u8::<3>(quintets);
    let q4 = vget_lane_u8::<4>(quintets);
    let q5 = vget_lane_u8::<5>(quintets);
    let q6 = vget_lane_u8::<6>(quintets);
    let q7 = vget_lane_u8::<7>(quintets);

    let b0 = (q0 << 3) | (q1 >> 2);
    let b1 = (q1.wrapping_shl(6)) | (q2 << 1) | (q3 >> 4);
    let b2 = (q3.wrapping_shl(4)) | (q4 >> 1);
    let b3 = (q4.wrapping_shl(7)) | (q5 << 2) | (q6 >> 3);
    let b4 = (q6.wrapping_shl(5)) | q7;

    core::ptr::copy_nonoverlapping([b0, b1, b2, b3, b4].as_ptr(), out, 5);
}

// =============================================================================
// Kernel tests
// =============================================================================

/// Direct kernel tests. These call the kernels below the public dispatch
/// thresholds so that every loop (20/10/5-byte encode, 8-char decode, tails)
/// is exercised on its own, including lengths the dispatcher never sends here.
#[cfg(test)]
mod kernel_tests {
    use super::*;
    use crate::{decode_into_constant_time, encode_into_constant_time};

    const ALL: [Alphabet; 10] = [
        Alphabet::Rfc4648,
        Alphabet::Rfc4648NoPadding,
        Alphabet::Rfc4648Lower,
        Alphabet::Rfc4648LowerNoPadding,
        Alphabet::Rfc4648Hex,
        Alphabet::Rfc4648HexNoPadding,
        Alphabet::Rfc4648HexLower,
        Alphabet::Rfc4648HexLowerNoPadding,
        Alphabet::Crockford,
        Alphabet::Z32,
    ];

    const SIMD_ALPHABETS: [Alphabet; 4] = [
        Alphabet::Rfc4648NoPadding,
        Alphabet::Rfc4648LowerNoPadding,
        Alphabet::Rfc4648HexNoPadding,
        Alphabet::Rfc4648HexLowerNoPadding,
    ];

    fn pattern(len: usize) -> alloc::vec::Vec<u8> {
        let mut x: u64 = 0x0123_4567_89AB_CDEF ^ len as u64;
        (0..len)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                x as u8
            })
            .collect()
    }

    fn valid_char(a: Alphabet) -> u8 {
        match a {
            Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => b'a',
            Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding | Alphabet::Crockford => b'0',
            Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => b'0',
            Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => b'A',
            Alphabet::Z32 => b'y',
        }
    }

    /// Content length of a correctly padded encoding.
    fn content_len(encoded: &[u8]) -> usize {
        encoded.iter().rposition(|&b| b != b'=').map_or(0, |p| p + 1)
    }

    #[test]
    fn kernel_encode_matches_constant_time_for_every_length() {
        let mut checks = 0usize;
        for len in 0..=200usize {
            let data = pattern(len);
            for a in ALL {
                let expected = crate::encoded_length(len, a.is_padded()).unwrap();
                let mut reference = alloc::vec![0u8; expected];
                encode_into_constant_time(&mut reference, &data, a).unwrap();

                // Exact-size output.
                let mut out = alloc::vec![0x5Au8; expected];
                unsafe { encode_into(&mut out, &data, a) }.unwrap();
                assert_eq!(out, reference, "kernel encode len={len} alphabet={a:?}");

                // Larger output: bytes past `expected` must stay untouched.
                let mut big = alloc::vec![0x5Au8; expected + 19];
                unsafe { encode_into(&mut big, &data, a) }.unwrap();
                assert_eq!(
                    &big[..expected],
                    &reference[..],
                    "kernel encode prefix len={len} alphabet={a:?}"
                );
                assert!(
                    big[expected..].iter().all(|&b| b == 0x5A),
                    "kernel encode wrote past expected length (len={len} alphabet={a:?})"
                );
                checks += 2;
            }
        }
        println!("neon kernel_encode_matches_constant_time_for_every_length: {checks} cases");
    }

    #[test]
    fn kernel_decode_matches_constant_time_for_every_valid_length() {
        let mut checks = 0usize;
        for len in 0..=200usize {
            let data = pattern(len);
            for a in ALL {
                let mut encoded = alloc::vec![0u8; crate::encoded_length(len, a.is_padded()).unwrap()];
                encode_into_constant_time(&mut encoded, &data, a).unwrap();
                let content = &encoded[..content_len(&encoded)];

                let mut out = alloc::vec![0x5Au8; len];
                let r = unsafe { decode_into(&mut out, content, a) };
                assert_eq!(r, Ok(()), "kernel decode len={len} alphabet={a:?}");
                assert_eq!(out, data, "kernel decode bytes len={len} alphabet={a:?}");

                // Larger output: bytes past the decoded length must stay untouched.
                let mut big = alloc::vec![0x5Au8; len + 23];
                let r = unsafe { decode_into(&mut big, content, a) };
                assert_eq!(r, Ok(()));
                assert_eq!(&big[..len], &data[..]);
                assert!(
                    big[len..].iter().all(|&b| b == 0x5A),
                    "kernel decode wrote past decoded length (len={len} alphabet={a:?})"
                );
                checks += 2;
            }
        }
        println!("neon kernel_decode_matches_constant_time_for_every_valid_length: {checks} cases");
    }

    /// Every lane of `simd_validate_8` must be able to reject: an invalid byte
    /// at each of the 8 positions of a block must fail. Also every one of the
    /// 256 byte values at every lane must agree with the scalar decoder.
    #[test]
    fn kernel_decode_every_byte_in_every_lane_of_an_8_char_block() {
        let mut checks = 0usize;
        for a in SIMD_ALPHABETS {
            let valid = valid_char(a);
            for lane in 0..8usize {
                for byte in 0..=255u8 {
                    // A padding byte never reaches the kernels: the dispatch in
                    // base32.rs strips it and validates the padding first. The
                    // kernels require padding-free content, so skip it here.
                    if byte == b'=' {
                        continue;
                    }
                    let mut input = [valid; 8];
                    input[lane] = byte;
                    let mut out = [0u8; 5];
                    let r = unsafe { decode_into(&mut out, &input, a) };
                    let mut ref_out = [0u8; 5];
                    let r_ref = decode_into_constant_time(&mut ref_out, &input, a.without_padding());
                    assert_eq!(
                        r, r_ref,
                        "kernel decode disagreement lane={lane} byte={byte:#04x} alphabet={a:?}"
                    );
                    if r.is_ok() {
                        assert_eq!(out, ref_out, "kernel decode bytes lane={lane} byte={byte:#04x}");
                    } else {
                        assert_eq!(r, Err(DecodeError::InvalidInput));
                    }
                    checks += 1;
                }
            }
        }
        println!("neon kernel_decode_every_byte_in_every_lane_of_an_8_char_block: {checks} cases");
    }

    /// The tail remainder path (including every invalid remainder) must be safe
    /// and match the scalar decoder. Output is over-sized so the kernel's
    /// tail slice is exactly the decoded length and stays inside the allocation.
    #[test]
    fn kernel_decode_tail_remainders() {
        let mut checks = 0usize;
        for a in ALL {
            let valid = valid_char(a);
            for len in 0..32usize {
                let input = alloc::vec![valid; len];
                let mut out = [0x5Au8; 64];
                let r = unsafe { decode_into(&mut out, &input, a) };
                let mut ref_out = [0x5Au8; 64];
                let r_ref = decode_into_constant_time(&mut ref_out, &input, a.without_padding());
                assert_eq!(r, r_ref, "tail len={len} alphabet={a:?}");
                if r.is_ok() {
                    assert_eq!(out, ref_out, "tail len={len} alphabet={a:?}");
                    // Bytes that the scalar path did not write must be untouched.
                    for i in 0..64 {
                        if ref_out[i] == 0x5A {
                            assert_eq!(out[i], 0x5A, "kernel wrote unrequested byte {i} (len={len})");
                        }
                    }
                }
                checks += 1;
            }
        }
        println!("neon kernel_decode_tail_remainders: {checks} cases");
    }

    /// Invalid remainders must be rejected without constructing an output
    /// slice longer than the caller provided, even when the output is empty.
    #[test]
    fn decode_invalid_remainder_exact_buffer() {
        for len in [1usize, 3, 6] {
            let input = alloc::vec![b'A'; len];
            let mut out: [u8; 0] = [];
            // SAFETY: the kernel is compiled with the target feature enabled;
            // the invalid remainder is rejected before any write.
            let result = unsafe { decode_into(&mut out, &input, Alphabet::Rfc4648NoPadding) };
            assert_eq!(result, Err(DecodeError::InvalidLength), "len={len}");
        }
    }
}
