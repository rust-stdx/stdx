//! Tests for the base32 SIMD kernels.
//!
//! These tests exercise the public dispatch (`encode_into` / `decode_into`)
//! against the crate's constant-time scalar implementation and against an
//! independent, bit-level reference implementation written from RFC 4648.
//!
//! Every output buffer is a sub-slice of a larger allocation with guard bytes
//! before and after it (non-zero offset, so `from_raw_parts[_mut]` offsets are
//! exercised), a sentinel-filled "slack" region after the documented output
//! length, and an optional extra-visible region inside the provided slice. That
//! makes writes past the expected encoded/decoded length observable without a
//! sanitizer.
//!
//! Inputs are likewise placed at non-zero offsets inside a larger allocation.

use base32::{
    Alphabet, DecodeError, EncodeError, decode_into, decode_into_constant_time, encode_into, encode_into_constant_time,
    encoded_length,
};
use random::{RngExt, SeedableRng, rngs::ChaChaRng};

const ALL_ALPHABETS: [Alphabet; 10] = [
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

// Alphabets whose decode goes through `decode_simd` (two character ranges).
const SIMD_ALPHABETS: [Alphabet; 8] = [
    Alphabet::Rfc4648,
    Alphabet::Rfc4648NoPadding,
    Alphabet::Rfc4648Lower,
    Alphabet::Rfc4648LowerNoPadding,
    Alphabet::Rfc4648Hex,
    Alphabet::Rfc4648HexNoPadding,
    Alphabet::Rfc4648HexLower,
    Alphabet::Rfc4648HexLowerNoPadding,
];

// Alphabets whose decode goes through `decode_scalar` in both kernels.
const SCALAR_ALPHABETS: [Alphabet; 2] = [Alphabet::Crockford, Alphabet::Z32];

const GUARD: u8 = 0xC7;
const FILL: u8 = 0xA5;
const PAD_BEFORE: usize = 23;
const PAD_AFTER: usize = 31;
const INPUT_OFFSET: usize = 17;

fn table(a: Alphabet) -> &'static [u8; 32] {
    match a {
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => b"abcdefghijklmnopqrstuvwxyz234567",
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => b"0123456789ABCDEFGHIJKLMNOPQRSTUV",
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => b"0123456789abcdefghijklmnopqrstuv",
        Alphabet::Crockford => b"0123456789ABCDEFGHJKMNPQRSTVWXYZ",
        Alphabet::Z32 => b"ybndrfg8ejkmcpqxot1uwisza345h769",
    }
}

fn is_padded(a: Alphabet) -> bool {
    matches!(
        a,
        Alphabet::Rfc4648 | Alphabet::Rfc4648Lower | Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexLower
    )
}

/// The no-padding variant of `a`, for tests that pass an unpadded fragment.
fn unpadded(a: Alphabet) -> Alphabet {
    match a {
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => Alphabet::Rfc4648NoPadding,
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => Alphabet::Rfc4648LowerNoPadding,
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => Alphabet::Rfc4648HexNoPadding,
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => Alphabet::Rfc4648HexLowerNoPadding,
        Alphabet::Crockford => Alphabet::Crockford,
        Alphabet::Z32 => Alphabet::Z32,
    }
}

/// Reference `decoded_length` for an unpadded content length.
fn ref_decoded_len(content_len: usize) -> Option<usize> {
    let rem = content_len % 8;
    let extra = match rem {
        0 => 0,
        2 => 1,
        4 => 2,
        5 => 3,
        7 => 4,
        _ => return None,
    };
    Some((content_len / 8) * 5 + extra)
}

/// Pure reference base32 encoder (RFC 4648 bit layout).
fn ref_encode(data: &[u8], a: Alphabet) -> Vec<u8> {
    let tbl = table(a);
    let mut out = Vec::with_capacity(data.len().div_ceil(5) * 8);
    for chunk in data.chunks(5) {
        let n = chunk.len();
        let mut value: u64 = 0;
        for &b in chunk {
            value = (value << 8) | u64::from(b);
        }
        let groups = (n * 8).div_ceil(5); // number of 5-bit groups for a partial block
        let pad_bits = groups * 5 - n * 8;
        let field = value << pad_bits;
        for g in 0..groups {
            let idx = ((field >> (groups * 5 - 5 * (g + 1))) & 0x1F) as usize;
            out.push(tbl[idx]);
        }
    }
    if is_padded(a) {
        while !out.len().is_multiple_of(8) {
            out.push(b'=');
        }
    }
    out
}

fn ref_char_quintet(c: u8, a: Alphabet) -> Option<u8> {
    table(a).iter().position(|&x| x == c).map(|p| p as u8)
}

#[derive(Debug, PartialEq, Eq)]
enum RefError {
    Input,
    Length,
    Padding,
}

/// Reference padding strip, mirroring RFC 4648 rules.
fn ref_strip(input: &[u8], a: Alphabet) -> Result<usize, RefError> {
    let n = input.len();
    if n == 0 {
        return Ok(0);
    }
    if !is_padded(a) {
        if input[n - 1] == b'=' {
            return Err(RefError::Padding);
        }
        return Ok(n);
    }
    let max = n.min(7);
    let mut count = 0;
    while count < max && input[n - 1 - count] == b'=' {
        count += 1;
    }
    let content_len = n - count;
    if count == 0 {
        return match n % 8 {
            0 => Ok(n),
            1 | 3 | 6 => Err(RefError::Length),
            _ => Err(RefError::Padding),
        };
    }
    if count > 0 {
        if !n.is_multiple_of(8) || count > 6 {
            return Err(RefError::Padding);
        }
        let rem = content_len % 8;
        let ok = match count {
            6 => rem == 2,
            4 => rem == 4,
            3 => rem == 5,
            1 => rem == 7,
            _ => false,
        };
        if !ok {
            return Err(RefError::Padding);
        }
    }
    Ok(content_len)
}

/// Pure reference base32 decoder. `Ok` means the input is valid *and*
/// canonical (trailing bits of the final partial group are zero).
fn ref_decode(input: &[u8], a: Alphabet) -> Result<Vec<u8>, RefError> {
    let content_len = ref_strip(input, a)?;
    let expected = ref_decoded_len(content_len).ok_or(RefError::Length)?;
    let mut out = Vec::with_capacity(expected);
    let mut acc: u64 = 0;
    let mut nbits: u32 = 0;
    for &c in &input[..content_len] {
        let q = ref_char_quintet(c, a).ok_or(RefError::Input)?;
        acc = (acc << 5) | u64::from(q);
        nbits += 5;
        if nbits >= 8 {
            nbits -= 8;
            out.push(((acc >> nbits) & 0xFF) as u8);
        }
        acc &= (1u64 << nbits) - 1;
    }
    // Leftover bits are the pad bits of the last byte; they must be zero.
    if nbits != 0 && acc != 0 {
        return Err(RefError::Input);
    }
    debug_assert_eq!(out.len(), expected);
    Ok(out)
}

fn offset_input(input: &[u8]) -> (Vec<u8>, usize) {
    let mut buf = vec![0xEEu8; INPUT_OFFSET];
    buf.extend_from_slice(input);
    buf.extend_from_slice(&[0xEEu8; 23]);
    (buf, INPUT_OFFSET)
}

/// Run `encode_into` or `encode_into_constant_time` into a guarded buffer and
/// return the result plus the first `expected` output bytes.
///
/// - guard bytes before/after the provided slice must stay `GUARD`
/// - if `extra > 0` the provided slice is `expected + extra` long; those
///   `extra` bytes must stay `FILL` on success
/// - `slack` more sentinel bytes are checked as well
fn encode_run(
    data: &[u8],
    a: Alphabet,
    expected: usize,
    extra: usize,
    slack: usize,
    ct: bool,
) -> (Result<(), EncodeError>, Vec<u8>) {
    let slice_len = expected + extra;
    let total = slice_len + slack;
    let mut buf = vec![GUARD; PAD_BEFORE + total + PAD_AFTER];
    buf[PAD_BEFORE..PAD_BEFORE + total].fill(FILL);
    let result = if ct {
        encode_into_constant_time(&mut buf[PAD_BEFORE..PAD_BEFORE + slice_len], data, a)
    } else {
        encode_into(&mut buf[PAD_BEFORE..PAD_BEFORE + slice_len], data, a)
    };
    assert!(
        buf[..PAD_BEFORE].iter().all(|&b| b == GUARD),
        "guard before output corrupted (encode data_len={} extra={extra} slack={slack} alphabet={a:?})",
        data.len()
    );
    assert!(
        buf[PAD_BEFORE + total..].iter().all(|&b| b == GUARD),
        "guard after output corrupted (encode data_len={} extra={extra} slack={slack} alphabet={a:?})",
        data.len()
    );
    if result.is_ok() {
        assert!(
            buf[PAD_BEFORE + expected..PAD_BEFORE + total]
                .iter()
                .all(|&b| b == FILL),
            "wrote past the expected encoded length (encode data_len={} extra={extra} slack={slack} alphabet={a:?})",
            data.len()
        );
    }
    (result, buf[PAD_BEFORE..PAD_BEFORE + expected].to_vec())
}

/// Run `decode_into` or `decode_into_constant_time` into a guarded buffer.
fn decode_run(
    input: &[u8],
    a: Alphabet,
    expected: usize,
    extra: usize,
    slack: usize,
    ct: bool,
) -> (Result<(), DecodeError>, Vec<u8>) {
    let slice_len = expected + extra;
    let total = slice_len + slack;
    let mut buf = vec![GUARD; PAD_BEFORE + total + PAD_AFTER];
    buf[PAD_BEFORE..PAD_BEFORE + total].fill(FILL);
    let (src, off) = offset_input(input);
    let result = if ct {
        decode_into_constant_time(&mut buf[PAD_BEFORE..PAD_BEFORE + slice_len], &src[off..off + input.len()], a)
    } else {
        decode_into(&mut buf[PAD_BEFORE..PAD_BEFORE + slice_len], &src[off..off + input.len()], a)
    };
    assert!(
        buf[..PAD_BEFORE].iter().all(|&b| b == GUARD),
        "guard before output corrupted (decode input_len={} extra={extra} slack={slack} alphabet={a:?})",
        input.len()
    );
    assert!(
        buf[PAD_BEFORE + total..].iter().all(|&b| b == GUARD),
        "guard after output corrupted (decode input_len={} extra={extra} slack={slack} alphabet={a:?})",
        input.len()
    );
    if result.is_ok() {
        assert!(
            buf[PAD_BEFORE + expected..PAD_BEFORE + total]
                .iter()
                .all(|&b| b == FILL),
            "wrote past the expected decoded length (decode input_len={} extra={extra} slack={slack} alphabet={a:?})",
            input.len()
        );
    }
    (result, buf[PAD_BEFORE..PAD_BEFORE + expected].to_vec())
}

/// Size of the buffer a correct decoder must be able to fill, or 0 when the
/// reference considers the input invalid before decoding.
fn expected_decode_len(input: &[u8], a: Alphabet) -> usize {
    match ref_strip(input, a) {
        Ok(content_len) => ref_decoded_len(content_len).unwrap_or(0),
        Err(_) => 0,
    }
}

/// Full differential check for one decode input: dispatch vs constant-time vs
/// the independent reference.
fn compare_decode(input: &[u8], a: Alphabet) -> Result<(), DecodeError> {
    let expected = expected_decode_len(input, a);
    let (r_disp, out_disp) = decode_run(input, a, expected, 7, 13, false);
    let (r_ct, out_ct) = decode_run(input, a, expected, 7, 13, true);

    assert_eq!(
        r_disp, r_ct,
        "dispatch vs constant-time result mismatch (alphabet={a:?}, input={input:?})"
    );
    // On error, the amount of partially written output is not part of the API
    // contract (the SIMD path stops at the offending block, the constant-time
    // path writes the valid prefix), so outputs are only compared on success.
    if r_disp.is_ok() {
        assert_eq!(
            out_disp, out_ct,
            "dispatch vs constant-time output mismatch (alphabet={a:?}, input={input:?})"
        );
    }

    match ref_decode(input, a) {
        Ok(reference) => {
            assert_eq!(
                r_disp,
                Ok(()),
                "reference accepted but crate rejected (alphabet={a:?}, input={input:?})"
            );
            assert_eq!(
                out_disp, reference,
                "decoded bytes differ from reference (alphabet={a:?}, input={input:?})"
            );
        }
        Err(_) => assert!(
            r_disp.is_err(),
            "reference rejected but crate accepted (alphabet={a:?}, input={input:?})"
        ),
    }
    r_disp
}

/// Full differential check for one encode input.
fn compare_encode(data: &[u8], a: Alphabet) -> Vec<u8> {
    let expected = encoded_length(data.len(), is_padded(a)).unwrap();
    let (r_disp, out_disp) = encode_run(data, a, expected, 9, 11, false);
    let (r_ct, out_ct) = encode_run(data, a, expected, 9, 11, true);
    assert_eq!(r_disp, Ok(()), "encode failed for len {} alphabet {a:?}", data.len());
    assert_eq!(r_ct, Ok(()));
    assert_eq!(
        out_disp,
        out_ct,
        "encode dispatch vs constant-time mismatch (len={}, alphabet={a:?})",
        data.len()
    );
    let reference = ref_encode(data, a);
    assert_eq!(
        out_disp,
        reference,
        "encode differs from reference (len={}, alphabet={a:?})",
        data.len()
    );
    out_disp
}

fn pattern(len: usize, seed: u64) -> Vec<u8> {
    let mut rng = ChaChaRng::seed_from_u64(seed);
    let mut out = vec![0u8; len];
    rng.fill(&mut out);
    out
}

// ---------------------------------------------------------------------------
// Thresholds used by the dispatch in base32.rs
// ---------------------------------------------------------------------------

const ENCODE_SIMD_THRESHOLD: usize = 40;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
const DECODE_SIMD_THRESHOLD: usize = 64;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2",
    not(target_arch = "aarch64")
))]
const DECODE_SIMD_THRESHOLD: usize = 32;
#[cfg(not(any(
    all(target_arch = "aarch64", target_feature = "neon"),
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2")
)))]
const DECODE_SIMD_THRESHOLD: usize = usize::MAX;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn encode_differential_all_lengths() {
    let mut checks = 0usize;
    for len in 0..=512usize {
        let data = pattern(len, 0x1234_5678_9ABC_DEF1 ^ len as u64);
        for a in ALL_ALPHABETS {
            compare_encode(&data, a);
            checks += 1;
        }
    }
    println!("encode_differential_all_lengths: {checks} cases");
}

#[test]
fn decode_differential_all_lengths() {
    let mut checks = 0usize;
    for len in 0..=512usize {
        let data = pattern(len, 0xDEAD_BEEF_CAFE_0001 ^ len as u64);
        for a in ALL_ALPHABETS {
            let encoded = ref_encode(&data, a);
            let r = compare_decode(&encoded, a);
            assert_eq!(r, Ok(()), "valid encoding rejected (len={len}, alphabet={a:?})");
            let expected = expected_decode_len(&encoded, a);
            assert_eq!(expected, len, "reference decoded length mismatch");
            checks += 1;
        }
    }
    println!("decode_differential_all_lengths: {checks} cases");
}

/// Content-length boundaries around every SIMD block size used by the kernels
/// (8 chars per NEON block, 32 chars per AVX2 block) and around the dispatch
/// thresholds, with padded and unpadded encodings.
#[test]
fn decode_roundtrip_fixed_boundaries() {
    let lengths = [
        0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 15, 16, 17, 23, 24, 25, 31, 32, 33, 39, 40, 41, 47, 48, 55, 56, 57, 62, 63,
        64, 65, 66, 71, 72, 73, 79, 80, 81, 87, 88, 89, 95, 96, 97, 103, 104, 127, 128, 129, 191, 192, 255, 256, 257,
        319, 320,
    ];
    let mut checks = 0usize;
    for &len in &lengths {
        let data = pattern(len, 0xA5A5_5A5A_0000_0001 ^ len as u64);
        for a in ALL_ALPHABETS {
            let encoded = ref_encode(&data, a);
            let r = compare_decode(&encoded, a);
            assert_eq!(r, Ok(()), "valid encoding rejected (len={len}, alphabet={a:?})");
            checks += 1;
        }
    }
    println!("decode_roundtrip_fixed_boundaries: {checks} cases (decode threshold {DECODE_SIMD_THRESHOLD})");
}

#[test]
fn encode_every_byte_at_every_position() {
    let lengths = [
        ENCODE_SIMD_THRESHOLD - 1,
        ENCODE_SIMD_THRESHOLD,
        ENCODE_SIMD_THRESHOLD + 1,
        ENCODE_SIMD_THRESHOLD + 4,
        ENCODE_SIMD_THRESHOLD + 5,
        ENCODE_SIMD_THRESHOLD + 19,
        ENCODE_SIMD_THRESHOLD + 20,
        ENCODE_SIMD_THRESHOLD + 21,
        64,
        80,
    ];
    let mut checks = 0usize;
    for &len in &lengths {
        let base = pattern(len, 0x1111_2222_3333_4444);
        for a in ALL_ALPHABETS {
            let mut data = base.clone();
            for pos in 0..len {
                let original = data[pos];
                for byte in 0..=255u8 {
                    if byte == original {
                        continue;
                    }
                    data[pos] = byte;
                    compare_encode(&data, a);
                    checks += 1;
                }
                data[pos] = original;
            }
        }
    }
    println!("encode_every_byte_at_every_position: {checks} cases");
}

/// For each position of each length, try every byte value. This checks the
/// SIMD character validation lane-by-lane (all 8 lanes of the NEON block, all
/// 32 lanes of the AVX2 block) and that the dispatch path and scalar path agree
/// on accept/reject and on the decoded bytes.
#[test]
fn decode_every_byte_at_every_position_simd_alphabets() {
    // The 4 distinct character-range mappings. The padded variants use exactly
    // the same mapping (padding is stripped before the kernel runs) and are
    // covered by `decode_every_byte_padded_forms` / `decode_differential_*`.
    let mappings = [
        Alphabet::Rfc4648NoPadding,
        Alphabet::Rfc4648LowerNoPadding,
        Alphabet::Rfc4648HexNoPadding,
        Alphabet::Rfc4648HexLowerNoPadding,
    ];
    let lengths = [8usize, 16, 31, 32, 33, 39, 40, 41, 63, 64, 65, 72, 80, 96, 128];
    let mut checks = 0usize;
    for &len in &lengths {
        for a in mappings {
            checks += sweep_decode_byte_at_every_position(a, len);
        }
    }
    println!("decode_every_byte_at_every_position_simd_alphabets: {checks} cases");
}

/// Same exhaustive byte sweep at the two dispatch thresholds and the NEON/AVX2
/// block boundaries, for *every* SIMD alphabet variant (including padded).
#[test]
fn decode_every_byte_all_simd_alphabets_at_boundaries() {
    let lengths = [8usize, 32, 63, 64, 65];
    let mut checks = 0usize;
    for &len in &lengths {
        for a in SIMD_ALPHABETS {
            checks += sweep_decode_byte_at_every_position(a, len);
        }
    }
    println!("decode_every_byte_all_simd_alphabets_at_boundaries: {checks} cases");
}

#[test]
fn decode_every_byte_at_every_position_scalar_alphabets() {
    // Crockford and Z32 take the scalar kernel path in both kernels.
    let lengths = [8usize, 16, 32, 63, 64, 65, 71, 72, 80, 96, 128];
    let mut checks = 0usize;
    for &len in &lengths {
        for a in SCALAR_ALPHABETS {
            checks += sweep_decode_byte_at_every_position(a, len);
        }
    }
    println!("decode_every_byte_at_every_position_scalar_alphabets: {checks} cases");
}

/// All positions for *all* content lengths 0..=128 with a set of
/// boundary-relevant byte values (every 256-value sweep is done separately at
/// the SIMD boundaries). This catches any length-dependent validation
/// difference.
#[test]
fn decode_interesting_bytes_all_lengths_0_to_128() {
    const INTERESTING: [u8; 16] = [
        0x00, 0x2F, 0x30, 0x31, 0x39, 0x3A, 0x3F, 0x40, 0x41, 0x5A, 0x5B, 0x60, 0x61, 0x7A, 0x80, 0xFF,
    ];
    let mut checks = 0usize;
    for len in 0..=128usize {
        for a in [Alphabet::Rfc4648NoPadding, Alphabet::Crockford, Alphabet::Z32] {
            checks += sweep_decode_selected_bytes(a, len, &INTERESTING);
        }
    }
    println!("decode_interesting_bytes_all_lengths_0_to_128: {checks} cases");
}

fn sweep_decode_selected_bytes(a: Alphabet, len: usize, values: &[u8]) -> usize {
    let tbl = *table(a);
    let mut input: Vec<u8> = (0..len).map(|i| tbl[(i * 7 + 3) % 32]).collect();
    if len > 0 && !len.is_multiple_of(8) {
        input[len - 1] = tbl[0];
    }
    let mut checks = 0usize;
    for pos in 0..len {
        let original = input[pos];
        for &byte in values {
            if byte == original {
                continue;
            }
            input[pos] = byte;
            let _ = compare_decode(&input, unpadded(a));
            checks += 1;
        }
        input[pos] = original;
    }
    checks
}

fn sweep_decode_byte_at_every_position(a: Alphabet, len: usize) -> usize {
    let tbl = *table(a);
    let mut input: Vec<u8> = (0..len).map(|i| tbl[(i * 7 + 3) % 32]).collect();
    if len > 0 && !len.is_multiple_of(8) {
        // Make the trailing partial group canonical.
        input[len - 1] = tbl[0];
    }
    let mut checks = 0usize;
    for pos in 0..len {
        let original = input[pos];
        for byte in 0..=255u8 {
            if byte == original {
                continue;
            }
            input[pos] = byte;
            let _ = compare_decode(&input, unpadded(a));
            checks += 1;
        }
        input[pos] = original;
    }
    checks
}

/// Same sweep, but on *padded* encodings (the '=' region is mutated too).
#[test]
fn decode_every_byte_padded_forms() {
    let content_lens = [2usize, 4, 5, 7, 32, 37, 39, 40, 63, 65, 66, 71];
    let mut checks = 0usize;
    for &content_len in &content_lens {
        if ref_decoded_len(content_len).is_none() {
            continue;
        }
        for a in SIMD_ALPHABETS.into_iter().filter(|a| is_padded(*a)) {
            let tbl = *table(a);
            let mut input: Vec<u8> = (0..content_len).map(|i| tbl[(i * 5 + 1) % 32]).collect();
            if !content_len.is_multiple_of(8) {
                input[content_len - 1] = tbl[0];
            }
            while !input.len().is_multiple_of(8) {
                input.push(b'=');
            }
            for pos in 0..input.len() {
                let original = input[pos];
                for byte in 0..=255u8 {
                    if byte == original {
                        continue;
                    }
                    input[pos] = byte;
                    let _ = compare_decode(&input, a);
                    checks += 1;
                }
                input[pos] = original;
            }
        }
    }
    println!("decode_every_byte_padded_forms: {checks} cases");
}

#[test]
fn decode_invalid_remainder_lengths() {
    let mut checks = 0usize;
    let mut lens = Vec::new();
    for rem in [1usize, 3, 6] {
        for base in [0usize, 1, 2, 4, 8, 12] {
            lens.push(base * 8 + rem);
        }
    }
    for &len in &lens {
        for a in ALL_ALPHABETS {
            let tbl = *table(a);
            let input: Vec<u8> = (0..len).map(|i| tbl[(i * 3) % 32]).collect();
            let r = compare_decode(&input, a);
            assert_eq!(
                r,
                Err(DecodeError::InvalidLength),
                "invalid remainder length {len} should be InvalidLength (alphabet={a:?})"
            );
            checks += 1;
        }
    }
    println!("decode_invalid_remainder_lengths: {checks} cases");
}

#[test]
fn decode_output_too_small_is_rejected_and_untouched() {
    let mut checks = 0usize;
    for len in 1..=100usize {
        let data = pattern(len, 0x7777_0000_1111_2222 ^ len as u64);
        for a in ALL_ALPHABETS {
            let encoded = ref_encode(&data, a);
            let expected = expected_decode_len(&encoded, a);
            if expected == 0 {
                continue;
            }
            for short in 0..expected.min(4) {
                let (r, _) = decode_run(&encoded, a, expected, 0, 0, false);
                assert_eq!(r, Ok(()), "exact-size decode failed");
                let (r, out) = {
                    let slice_len = short;
                    let mut buf = vec![GUARD; PAD_BEFORE + slice_len + PAD_AFTER];
                    buf[PAD_BEFORE..PAD_BEFORE + slice_len].fill(FILL);
                    let (src, off) = offset_input(&encoded);
                    let r =
                        decode_into(&mut buf[PAD_BEFORE..PAD_BEFORE + slice_len], &src[off..off + encoded.len()], a);
                    (r, buf[PAD_BEFORE..PAD_BEFORE + slice_len].to_vec())
                };
                assert_eq!(
                    r,
                    Err(DecodeError::InvalidLength),
                    "short output must be rejected (len={len}, alphabet={a:?})"
                );
                assert!(
                    out.iter().all(|&b| b == FILL),
                    "short-output error path wrote into the buffer (len={len}, alphabet={a:?})"
                );
                checks += 1;
            }
        }
    }
    println!("decode_output_too_small_is_rejected_and_untouched: {checks} cases");
}

#[test]
fn encode_output_too_small_is_rejected_and_untouched() {
    let mut checks = 0usize;
    for len in 0..=100usize {
        let data = pattern(len, 0x3333_9999_0000_AAAA ^ len as u64);
        for a in ALL_ALPHABETS {
            let expected = encoded_length(len, is_padded(a)).unwrap();
            if expected == 0 {
                let mut buf: [u8; 0] = [];
                assert_eq!(encode_into(&mut buf, &data, a), Ok(()));
                continue;
            }
            let (r, out) = {
                let slice_len = expected - 1;
                let mut buf = vec![GUARD; PAD_BEFORE + slice_len + PAD_AFTER];
                buf[PAD_BEFORE..PAD_BEFORE + slice_len].fill(FILL);
                let r = encode_into(&mut buf[PAD_BEFORE..PAD_BEFORE + slice_len], &data, a);
                (r, buf[PAD_BEFORE..PAD_BEFORE + slice_len].to_vec())
            };
            assert_eq!(
                r,
                Err(EncodeError::InvalidOutputLength),
                "short output must be rejected (len={len}, alphabet={a:?})"
            );
            assert!(
                out.iter().all(|&b| b == FILL),
                "short-output error path wrote into the buffer (len={len}, alphabet={a:?})"
            );
            checks += 1;
        }
    }
    println!("encode_output_too_small_is_rejected_and_untouched: {checks} cases");
}

/// Exercise the dispatch thresholds explicitly with every alphabet, at
/// threshold-1 / threshold / threshold+1 and a few sizes around them.
#[test]
fn dispatch_threshold_boundaries() {
    let mut checks = 0usize;

    for delta in [-2i64, -1, 0, 1, 2] {
        let len = (ENCODE_SIMD_THRESHOLD as i64 + delta) as usize;
        let data = pattern(len, 0xBEEF_0000_0000_0001 ^ len as u64);
        for a in ALL_ALPHABETS {
            compare_encode(&data, a);
            // Round-trip through the decoder as well.
            let encoded = ref_encode(&data, a);
            assert_eq!(compare_decode(&encoded, a), Ok(()));
            checks += 2;
        }
    }

    if DECODE_SIMD_THRESHOLD != usize::MAX {
        for delta in [-2i64, -1, 0, 1, 2] {
            let content_len = (DECODE_SIMD_THRESHOLD as i64 + delta) as usize;
            for a in ALL_ALPHABETS {
                let tbl = *table(a);
                let mut input: Vec<u8> = (0..content_len).map(|i| tbl[(i * 11 + 6) % 32]).collect();
                if !content_len.is_multiple_of(8) {
                    input[content_len - 1] = tbl[0];
                }
                let valid = ref_decoded_len(content_len).is_some();
                let r = compare_decode(&input, unpadded(a));
                if valid {
                    assert_eq!(r, Ok(()), "content_len={content_len} alphabet={a:?}");
                } else {
                    assert_eq!(r, Err(DecodeError::InvalidLength), "content_len={content_len} alphabet={a:?}");
                }
                // And with padding where the alphabet allows it.
                if valid && is_padded(a) {
                    let mut padded = input.clone();
                    while !padded.len().is_multiple_of(8) {
                        padded.push(b'=');
                    }
                    assert_eq!(compare_decode(&padded, a), Ok(()));
                }
                checks += 2;
            }
        }
    }

    println!("dispatch_threshold_boundaries: {checks} cases");
}

/// Input placed at several non-zero offsets and lengths exactly at the
/// dispatch/canary-relevant boundaries.
#[test]
fn non_zero_slice_offsets() {
    let mut checks = 0usize;
    for off in [1usize, 3, 7, 15, 17, 31, 33, 63] {
        for len in [0usize, 1, 4, 5, 19, 20, 39, 40, 41, 63, 64, 65, 129] {
            let data = pattern(len, 0xFA0F_0000_0000_0001 ^ (off as u64) << 20 ^ len as u64);
            let mut src = vec![0xCCu8; off];
            src.extend_from_slice(&data);
            src.extend_from_slice(&[0xCCu8; 67]);
            let input = &src[off..off + len];
            for a in ALL_ALPHABETS {
                let expected = encoded_length(len, is_padded(a)).unwrap();
                let mut out = vec![0xDDu8; off + expected + 41];
                let r = encode_into(&mut out[off..off + expected], input, a);
                assert_eq!(r, Ok(()));
                assert_eq!(
                    &out[off..off + expected],
                    &ref_encode(input, a)[..],
                    "encode offset={off} len={len}"
                );
                assert!(out[..off].iter().all(|&b| b == 0xDD));
                assert!(out[off + expected..].iter().all(|&b| b == 0xDD));

                let encoded = ref_encode(input, a);
                let decoded_len = expected_decode_len(&encoded, a);
                let mut dec = vec![0xEEu8; off + decoded_len + 41];
                let mut ebuf = vec![0xCCu8; off];
                ebuf.extend_from_slice(&encoded);
                ebuf.extend_from_slice(&[0xCCu8; 67]);
                let er = decode_into(&mut dec[off..off + decoded_len], &ebuf[off..off + encoded.len()], a);
                assert_eq!(er, Ok(()), "decode offset={off} len={len} alphabet={a:?}");
                assert_eq!(&dec[off..off + decoded_len], input);
                assert!(dec[..off].iter().all(|&b| b == 0xEE));
                assert!(dec[off + decoded_len..].iter().all(|&b| b == 0xEE));
                checks += 2;
            }
        }
    }
    println!("non_zero_slice_offsets: {checks} cases");
}

#[test]
fn empty_and_tiny_inputs() {
    for a in ALL_ALPHABETS {
        assert_eq!(encode_into(&mut [], b"", a), Ok(()));
        assert_eq!(encode_into_constant_time(&mut [], b"", a), Ok(()));
        assert_eq!(decode_into(&mut [], b"", a), Ok(()));
        assert_eq!(decode_into_constant_time(&mut [], b"", a), Ok(()));
        for len in 1..=12usize {
            let data = pattern(len, 0x9999_0000_1111_0000 ^ len as u64);
            compare_encode(&data, a);
            let encoded = ref_encode(&data, a);
            assert_eq!(compare_decode(&encoded, a), Ok(()));
        }
    }
    // 1-byte input has valid 2-char encodings.
    for byte in 0..=255u8 {
        for a in ALL_ALPHABETS {
            let encoded = ref_encode(&[byte], a);
            assert_eq!(compare_decode(&encoded, a), Ok(()));
        }
    }
}
