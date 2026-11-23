//! Differential and boundary tests for `base64/base64.rs`.
//!
//! Covered here:
//!
//! * every input length `0..=512` for all four alphabets through
//!   `encode_into_constant_time` / `decode_into_constant_time`,
//! * the same sweep through the public dispatching API (`encode_into`,
//!   `decode_into`, `encode`, `decode`, `encode_into_string`),
//! * differential comparison against an independent reference encoder/decoder
//!   written below (no call into the crate's own scalar path),
//! * padding acceptance/rejection (missing, extra, interior, 1/2/3 pads,
//!   padding in no-padding mode, lengths not a multiple of 4, empty input,
//!   only-padding input),
//! * non-canonical trailing-bit rejection (exhaustive over the unused bits),
//! * output-buffer behaviour: exact/larger/smaller buffers, guard canaries
//!   before/after the decoded region (also on the `Err` path), no writes past
//!   the documented decoded length, slices at a non-zero offset,
//! * `encoded_length` overflow next to `usize::MAX` via `const` evaluation,
//! * all 256 byte values at every position of representative inputs,
//! * documentation claims that matter for security.
//!
//! Run with:
//!
//! ```text
//! cargo test -p base64 --test audit_core
//! ```

use base64::{Alphabet, DecodeError, EncodeError};
use rand::{Rng, RngExt, SeedableRng, rngs::StdRng};

const ALL_ALPHABETS: [Alphabet; 4] = [
    Alphabet::Standard,
    Alphabet::StandardNoPadding,
    Alphabet::Url,
    Alphabet::UrlNoPadding,
];

// Guard canary layout: `PRE` bytes before, `POST` bytes after the region.
const PRE: usize = 8;
const POST: usize = 16;
const CANARY: u8 = 0x5A;
const FILL: u8 = 0xCC;

////////////////////////////////////////////////////////////////////////////////
// Independent reference implementation
////////////////////////////////////////////////////////////////////////////////
fn is_padded(alphabet: Alphabet) -> bool {
    matches!(alphabet, Alphabet::Standard | Alphabet::Url)
}

fn alphabet_bytes(alphabet: Alphabet) -> &'static [u8; 64] {
    match alphabet {
        Alphabet::Standard | Alphabet::StandardNoPadding => {
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        }
        Alphabet::Url | Alphabet::UrlNoPadding => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
    }
}

fn alphabet_map(alphabet: Alphabet) -> [u8; 256] {
    let mut map = [0xFFu8; 256];
    for (value, &ch) in alphabet_bytes(alphabet).iter().enumerate() {
        map[ch as usize] = value as u8;
    }
    map
}

/// Straightforward reference encoder. Padding only when the alphabet asks for it.
fn ref_encode(data: &[u8], alphabet: Alphabet) -> Vec<u8> {
    let table = alphabet_bytes(alphabet);
    let pad = is_padded(alphabet);
    let mut out = Vec::with_capacity(base64::encoded_length(data.len(), pad).unwrap());
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(table[((n >> 18) & 0x3F) as usize]);
        out.push(table[((n >> 12) & 0x3F) as usize]);
        match chunk.len() {
            1 => {
                if pad {
                    out.push(b'=');
                    out.push(b'=');
                }
            }
            2 => {
                out.push(table[((n >> 6) & 0x3F) as usize]);
                if pad {
                    out.push(b'=');
                }
            }
            _ => {
                out.push(table[((n >> 6) & 0x3F) as usize]);
                out.push(table[(n & 0x3F) as usize]);
            }
        }
    }
    out
}

/// Reference decoded length for a padding-stripped content length.
fn ref_decoded_length(content_len: usize) -> Result<usize, DecodeError> {
    let full = content_len / 4;
    let base = full.checked_mul(3).ok_or(DecodeError::InvalidInputLength)?;
    match content_len % 4 {
        0 => Ok(base),
        2 => base.checked_add(1).ok_or(DecodeError::InvalidInputLength),
        3 => base.checked_add(2).ok_or(DecodeError::InvalidInputLength),
        _ => Err(DecodeError::InvalidInputLength),
    }
}

/// Reference padding scan. Mirrors the crate's *documented* rejection rules
/// (padding must be a suffix, at most two pads, total length a multiple of 4
/// when padded, pad count consistent with the content length) but is written
/// independently from scratch.
fn ref_strip(encoded: &[u8], alphabet: Alphabet) -> Result<(usize, usize), DecodeError> {
    let n = encoded.len();
    let padded = is_padded(alphabet);

    if !padded {
        if n > 0 && encoded[n - 1] == b'=' {
            return Err(DecodeError::InvalidPadding);
        }
        return Ok((n, 0));
    }

    let last = |k: usize| -> bool { n > k && encoded[n - 1 - k] == b'=' };
    let p0 = last(0);
    let p1 = p0 && last(1);
    let p2 = p1 && last(2);
    let pad_count = p0 as usize + p1 as usize + p2 as usize;
    let content_len = n - pad_count;

    if pad_count == 0 {
        return match n & 3 {
            0 => Ok((n, 0)),
            1 => Err(DecodeError::InvalidInputLength),
            _ => Err(DecodeError::InvalidPadding),
        };
    }

    if pad_count != 0 && !n.is_multiple_of(4) {
        return Err(DecodeError::InvalidInputLength);
    }
    if pad_count > 2 {
        return Err(DecodeError::InvalidPadding);
    }
    if pad_count != 0 && content_len % 4 != 4 - pad_count {
        return Err(DecodeError::InvalidPadding);
    }
    Ok((content_len, pad_count))
}

/// Full reference decoder. Returns exactly the byte string the crate *should*
/// produce, or the error variant the crate's documented ordering implies.
fn ref_decode(encoded: &[u8], alphabet: Alphabet) -> Result<Vec<u8>, DecodeError> {
    let (content_len, _) = ref_strip(encoded, alphabet)?;
    if content_len == 0 {
        return Ok(Vec::new());
    }
    let out_len = ref_decoded_length(content_len)?;

    let map = alphabet_map(alphabet);
    let mut sextets = [0u8; 4];
    for (k, &ch) in encoded[..content_len].iter().enumerate() {
        let v = map[ch as usize];
        if v == 0xFF {
            return Err(DecodeError::InvalidInput);
        }
        sextets[k % 4] = v;
    }

    // Unused trailing bits must be zero.
    match content_len % 4 {
        2 if sextets[1] & 0x0F != 0 => return Err(DecodeError::InvalidInput),
        3 if sextets[2] & 0x03 != 0 => return Err(DecodeError::InvalidInput),
        _ => {}
    }

    let mut out = Vec::with_capacity(out_len);
    let mut i = 0;
    while i + 4 <= content_len {
        let (v0, v1, v2, v3) = (
            map[encoded[i] as usize],
            map[encoded[i + 1] as usize],
            map[encoded[i + 2] as usize],
            map[encoded[i + 3] as usize],
        );
        out.push((v0 << 2) | (v1 >> 4));
        out.push((v1 << 4) | (v2 >> 2));
        out.push((v2 << 6) | v3);
        i += 4;
    }
    match content_len - i {
        0 => {}
        2 => {
            let v0 = map[encoded[i] as usize];
            let v1 = map[encoded[i + 1] as usize];
            out.push((v0 << 2) | (v1 >> 4));
        }
        3 => {
            let v0 = map[encoded[i] as usize];
            let v1 = map[encoded[i + 1] as usize];
            let v2 = map[encoded[i + 2] as usize];
            out.push((v0 << 2) | (v1 >> 4));
            out.push((v1 << 4) | (v2 >> 2));
        }
        _ => return Err(DecodeError::InvalidInputLength),
    }
    assert_eq!(out.len(), out_len, "reference decoder length mismatch");
    Ok(out)
}

////////////////////////////////////////////////////////////////////////////////
// Test helpers
////////////////////////////////////////////////////////////////////////////////
/// Deterministic seeded `StdRng` generator.
fn patterned(len: usize, pattern: usize) -> Vec<u8> {
    match pattern {
        0 => vec![0x00; len],
        1 => vec![0xFF; len],
        2 => (0..len).map(|i| (i % 256) as u8).collect(),
        3 => {
            let mut rng = StdRng::seed_from_u64(0x1234_5678_9ABC_DEF0 ^ len as u64);
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        }
        _ => unreachable!(),
    }
}

/// Allocate `PRE + len + POST` canary bytes and return the whole buffer plus
/// the index range of the usable middle region.
fn canary_buffer(len: usize) -> (Vec<u8>, usize) {
    let mut buf = vec![CANARY; PRE + len + POST];
    buf[PRE..PRE + len].fill(FILL);
    (buf, PRE)
}

fn check_canaries(buf: &[u8], len: usize, what: &str) {
    assert!(
        buf[..PRE].iter().all(|&b| b == CANARY),
        "guard canary before output region was modified: {what}"
    );
    assert!(
        buf[PRE + len..].iter().all(|&b| b == CANARY),
        "guard canary after output region was modified (write past decoded region): {what}"
    );
}

/// Exercise the constant-time encoder and decoder for one `(data, alphabet)`
/// pair using exact-size output buffers surrounded by guard canaries.
fn check_encode_scalar(data: &[u8], alphabet: Alphabet, what: &str) {
    let padded = is_padded(alphabet);
    let expected = ref_encode(data, alphabet);
    assert_eq!(
        expected.len(),
        base64::encoded_length(data.len(), padded).unwrap(),
        "encoded_length disagrees with reference encoding: {what}"
    );

    let (mut buf, pre) = canary_buffer(expected.len());
    let r = base64::encode_into_constant_time(&mut buf[pre..pre + expected.len()], data, alphabet);
    assert_eq!(r, Ok(()), "scalar encode failed: {what}");
    assert_eq!(
        &buf[pre..pre + expected.len()],
        &expected[..],
        "scalar encode output mismatch: {what}"
    );
    check_canaries(&buf, expected.len(), what);

    // Every emitted byte must be ASCII base64/pad; this is what makes the
    // `from_utf8_unchecked` call in `encode`/`encode_into_string` sound.
    for &b in &expected {
        assert!(b.is_ascii(), "non-ASCII byte {b:#04x} in encoded output: {what}");
    }

    // Exact-size decode buffer.
    let (mut dec, dpre) = canary_buffer(data.len());
    let r = base64::decode_into_constant_time(&mut dec[dpre..dpre + data.len()], &expected, alphabet);
    assert_eq!(r, Ok(data.len()), "scalar decode failed: {what}");
    assert_eq!(&dec[dpre..dpre + data.len()], data, "scalar decode output mismatch: {what}");
    check_canaries(&dec, data.len(), what);
}

/// Exercise the public dispatching encoder/decoder plus the allocating helpers.
fn check_encode_dispatched(data: &[u8], alphabet: Alphabet, what: &str) {
    let expected = ref_encode(data, alphabet);

    // `encode_into` (SIMD dispatch when the target feature is available).
    let (mut buf, pre) = canary_buffer(expected.len());
    let r = base64::encode_into(&mut buf[pre..pre + expected.len()], data, alphabet);
    assert_eq!(r, Ok(()), "dispatched encode failed: {what}");
    assert_eq!(
        &buf[pre..pre + expected.len()],
        &expected[..],
        "dispatched encode mismatch: {what}"
    );
    check_canaries(&buf, expected.len(), what);

    // `encode` allocates the string via `String::from_utf8_unchecked`; compare
    // against the reference and parse it back as UTF-8 to be sure.
    let encoded = base64::encode(data, alphabet);
    assert_eq!(encoded.as_bytes(), &expected[..], "encode() mismatch: {what}");
    assert!(
        core::str::from_utf8(encoded.as_bytes()).is_ok(),
        "encode() produced non-UTF-8: {what}"
    );

    // `decode` (allocating) roundtrip.
    let decoded = base64::decode(&encoded, alphabet).unwrap_or_else(|e| panic!("decode() failed: {e:?}: {what}"));
    assert_eq!(decoded, data, "decode() roundtrip mismatch: {what}");

    // `decode_into` (dispatch) with an over-sized buffer and canaries past the
    // decoded region.
    let slack = 37;
    let (mut dec, dpre) = canary_buffer(data.len() + slack);
    let r = base64::decode_into(&mut dec[dpre..dpre + data.len() + slack], &expected, alphabet);
    assert_eq!(r, Ok(data.len()), "dispatched decode failed: {what}");
    assert_eq!(&dec[dpre..dpre + data.len()], data, "dispatched decode mismatch: {what}");
    assert!(
        dec[dpre + data.len()..dpre + data.len() + slack]
            .iter()
            .all(|&b| b == FILL),
        "dispatched decode wrote past the decoded length: {what}"
    );
    check_canaries(&dec, data.len() + slack, what);
}

/// Compare the constant-time decoder against the reference for an arbitrary
/// (possibly invalid) encoded input, using an exact-size output buffer with
/// guard canaries. Checks that nothing is written past the decoded length even
/// when an error is returned.
fn check_decode_case(encoded: &[u8], alphabet: Alphabet, what: &str) {
    let stripped = ref_strip(encoded, alphabet);
    let (cap, expected) = match stripped {
        Err(e) => (0usize, Err(e)),
        Ok((content_len, _)) => match ref_decoded_length(content_len) {
            Err(e) => (0usize, Err(e)),
            Ok(cap) => (cap, ref_decode(encoded, alphabet)),
        },
    };

    let (mut buf, pre) = canary_buffer(cap);
    let r = base64::decode_into_constant_time(&mut buf[pre..pre + cap], encoded, alphabet);
    match expected {
        Ok(bytes) => {
            assert_eq!(bytes.len(), cap);
            assert_eq!(r, Ok(cap), "scalar decode result mismatch: {what}");
            assert_eq!(&buf[pre..pre + cap], &bytes[..], "scalar decode bytes mismatch: {what}");
        }
        Err(e) => {
            // On `InvalidInput` the crate may leave a partially decoded prefix
            // inside the output region (documented in the report), but it must
            // never touch the canaries.
            assert_eq!(r.map(|_| ()), Err(e), "scalar decode error mismatch: {what}");
        }
    }
    check_canaries(&buf, cap, what);
}

////////////////////////////////////////////////////////////////////////////////
// 1. Exhaustive length sweep, constant-time path
////////////////////////////////////////////////////////////////////////////////
#[test]
fn scalar_encode_decode_all_lengths_0_to_512() {
    for len in 0..=512usize {
        for pattern in 0..4 {
            let data = patterned(len, pattern);
            for alphabet in ALL_ALPHABETS {
                let what = format!("len={len} pattern={pattern} alphabet={alphabet:?}");
                check_encode_scalar(&data, alphabet, &what);
            }
        }
    }
}

#[test]
fn dispatch_and_public_api_all_lengths_0_to_512() {
    for len in 0..=512usize {
        let data = patterned(len, 3);
        for alphabet in ALL_ALPHABETS {
            let what = format!("len={len} alphabet={alphabet:?}");
            check_encode_dispatched(&data, alphabet, &what);
        }
    }
}

#[test]
fn encode_into_string_all_lengths_0_to_300() {
    // 192 bytes -> exactly 256 encoded bytes, the stack-buffer boundary.
    for len in 0..=300usize {
        let data = patterned(len, 2);
        for alphabet in ALL_ALPHABETS {
            let expected = ref_encode(&data, alphabet);
            let mut s = String::from("prefix:");
            base64::encode_into_string(&mut s, &data, alphabet);
            assert_eq!(
                s.as_bytes(),
                [b"prefix:".as_slice(), &expected[..]].concat(),
                "len={len} alphabet={alphabet:?}"
            );
            assert!(core::str::from_utf8(s.as_bytes()).is_ok());
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// 2. Exact buffer sizes and sub-slice offsets
////////////////////////////////////////////////////////////////////////////////
#[test]
fn exact_buffers_at_nonzero_offset() {
    for len in [
        0usize, 1, 2, 3, 4, 5, 23, 24, 25, 31, 32, 33, 47, 48, 49, 64, 65, 127, 128, 129,
    ] {
        let mut raw = vec![0x11u8; len + 9];
        let mut rng = StdRng::seed_from_u64(0xDEAD_BEEF_0000_0001 ^ len as u64);
        rng.fill(&mut raw[3..3 + len]);
        let input = &raw[3..3 + len];

        for alphabet in ALL_ALPHABETS {
            let expected = ref_encode(input, alphabet);
            let encoded_len = expected.len();

            // Encode into a slice at a non-zero offset inside a larger allocation.
            let mut enc = vec![0x5Au8; encoded_len + 11];
            let r = base64::encode_into_constant_time(&mut enc[7..7 + encoded_len], input, alphabet);
            assert_eq!(r, Ok(()));
            assert_eq!(&enc[7..7 + encoded_len], &expected[..], "offset encode len={len}");
            assert!(enc[..7].iter().all(|&b| b == 0x5A));
            assert!(enc[7 + encoded_len..].iter().all(|&b| b == 0x5A));

            // Decode into a slice at a non-zero offset.
            let mut dec = vec![0x5Au8; len + 13];
            let r = base64::decode_into_constant_time(&mut dec[5..5 + len], &enc[7..7 + encoded_len], alphabet);
            assert_eq!(r, Ok(len));
            assert_eq!(&dec[5..5 + len], input, "offset decode len={len}");
            assert!(dec[..5].iter().all(|&b| b == 0x5A));
            assert!(dec[5 + len..].iter().all(|&b| b == 0x5A));
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// 3. Padding acceptance / rejection matrix
////////////////////////////////////////////////////////////////////////////////
/// Padded alphabets require canonical padding; the no-padding alphabets accept
/// the unpadded forms.
#[test]
fn padding_matrix_strict_behaviour() {
    use DecodeError::{InvalidInput, InvalidInputLength, InvalidPadding};

    // (input, Standard, StandardNoPadding, Url, UrlNoPadding)
    let cases: &[(&[u8], Result<&[u8], DecodeError>, Result<&[u8], DecodeError>)] = &[
        // RFC 4648 vectors.
        (b"", Ok(b""), Ok(b"")),
        (b"Zg==", Ok(b"f"), Err(InvalidPadding)),
        (b"Zm8=", Ok(b"fo"), Err(InvalidPadding)),
        (b"Zm9v", Ok(b"foo"), Ok(b"foo")),
        // Missing padding on a padded alphabet is rejected; the no-padding
        // alphabets still accept the same unpadded input.
        (b"Zg", Err(InvalidPadding), Ok(b"f")),
        (b"Zm8", Err(InvalidPadding), Ok(b"fo")),
        (b"Zm9vYmE", Err(InvalidPadding), Ok(b"fooba")),
        // Total length not a multiple of 4, no pads: rejected by the padded
        // alphabets, accepted by the no-padding alphabets.
        (b"AAAAAA", Err(InvalidPadding), Ok(&[0, 0, 0, 0])),
        (b"AAAAAAA", Err(InvalidPadding), Ok(&[0, 0, 0, 0, 0])),
        // Length congruent 1 mod 4 is always a length error.
        (b"A", Err(InvalidInputLength), Err(InvalidInputLength)),
        (b"AAAAA", Err(InvalidInputLength), Err(InvalidInputLength)),
        (b"AA=", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"A==", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"AA===", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"Zg===", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"Zg====", Err(InvalidInputLength), Err(InvalidPadding)),
        // Only padding.
        (b"=", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"==", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"===", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"====", Err(InvalidPadding), Err(InvalidPadding)),
        (b"=====", Err(InvalidInputLength), Err(InvalidPadding)),
        // More than two pads after valid content.
        (b"Z===", Err(InvalidPadding), Err(InvalidPadding)),
        (b"A===", Err(InvalidPadding), Err(InvalidPadding)),
        (b"AAAA====", Err(InvalidPadding), Err(InvalidPadding)),
        // Padded inputs whose total length is not a multiple of 4 are length
        // errors, even when the pad count itself is plausible.
        (b"AAAA==", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"AAAAAAAA==", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"AAAAA=", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"AAAAAA=", Err(InvalidInputLength), Err(InvalidPadding)),
        // 8 chars: one pad with a 7-char content (3 mod 4) is canonical.
        (b"AAAAAAA=", Ok(&[0, 0, 0, 0, 0]), Err(InvalidPadding)),
        (b"AAAA=", Err(InvalidInputLength), Err(InvalidPadding)),
        // Interior '=' can never be padding; it is an invalid character.
        (b"A=AA", Err(InvalidInput), Err(InvalidInput)),
        (b"AA=A", Err(InvalidInput), Err(InvalidInput)),
        (b"=AAA", Err(InvalidInput), Err(InvalidInput)),
        (b"A=A=", Err(InvalidInput), Err(InvalidPadding)),
        (b"AA=A=", Err(InvalidInputLength), Err(InvalidPadding)),
        (b"AAA=A", Err(InvalidInputLength), Err(InvalidInputLength)),
        (b"=A==", Err(InvalidInput), Err(InvalidPadding)),
        (b"AA==AAAA", Err(InvalidInput), Err(InvalidInput)),
        // Padding is only legal at the very end in no-padding mode.
        (b"Zg=", Err(InvalidInputLength), Err(InvalidPadding)),
        // Overflow-ish trailing pads.
        (b"AAA==", Err(InvalidInputLength), Err(InvalidPadding)),
    ];

    for (input, std_result, nopad_result) in cases {
        let text = String::from_utf8_lossy(input).into_owned();
        let expect = |r: &Result<&[u8], DecodeError>, alphabet: Alphabet| match r {
            Ok(bytes) => assert_eq!(
                base64::decode(input, alphabet),
                Ok(bytes.to_vec()),
                "input={text:?} alphabet={alphabet:?}"
            ),
            Err(e) => assert_eq!(base64::decode(input, alphabet), Err(*e), "input={text:?} alphabet={alphabet:?}"),
        };
        expect(std_result, Alphabet::Standard);
        expect(std_result, Alphabet::Url);
        expect(nopad_result, Alphabet::StandardNoPadding);
        expect(nopad_result, Alphabet::UrlNoPadding);

        // The scalar decoder must agree with the public wrapper on every case.
        for alphabet in ALL_ALPHABETS {
            check_decode_case(input, alphabet, &format!("padding matrix input={text:?} alphabet={alphabet:?}"));
        }
    }
}

/// Regression test: `Alphabet::Standard`/`Alphabet::Url` require canonical
/// padding and reject unpadded input, including lengths that are not a multiple
/// of 4.
#[test]
fn padded_alphabet_requires_canonical_padding() {
    assert_eq!(base64::decode(b"Zg", Alphabet::Standard), Err(DecodeError::InvalidPadding));
    assert_eq!(base64::decode(b"Zm8", Alphabet::Standard), Err(DecodeError::InvalidPadding));
    assert_eq!(base64::decode(b"Zg", Alphabet::Url), Err(DecodeError::InvalidPadding));
    assert_eq!(base64::decode(b"AAAAAA", Alphabet::Standard), Err(DecodeError::InvalidPadding));
    assert_eq!(base64::decode(b"AAAAAAA", Alphabet::Standard), Err(DecodeError::InvalidPadding));
    assert_eq!(
        base64::decode(b"AAAAAA", Alphabet::StandardNoPadding),
        Ok(vec![0, 0, 0, 0]),
        "unpadded alphabet still accepts unpadded input"
    );
}

/// Long unpadded inputs must be rejected by the padded alphabets on both the
/// scalar and dispatched (SIMD-threshold) paths.
#[test]
fn long_padded_alphabets_reject_unpadded_input() {
    for alphabet in [Alphabet::Standard, Alphabet::Url] {
        for len in 1..=512usize {
            let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(37)).collect();
            let mut encoded = base64::encode(&data, alphabet).into_bytes();
            while encoded.last() == Some(&b'=') {
                encoded.pop();
            }

            let expected = if data.len().is_multiple_of(3) {
                Ok(data.clone())
            } else {
                Err(DecodeError::InvalidPadding)
            };
            assert_eq!(
                base64::decode(&encoded, alphabet),
                expected,
                "len={len} alphabet={alphabet:?} stripped_len={}",
                encoded.len()
            );

            // The dispatch path must agree; use an output buffer large enough
            // for the canonical form so only the padding decision differs.
            let mut out = vec![0u8; data.len()];
            let result = base64::decode_into(&mut out, &encoded, alphabet);
            match expected {
                Ok(bytes) => {
                    assert_eq!(result, Ok(bytes.len()), "len={len} alphabet={alphabet:?}");
                    assert_eq!(&out[..bytes.len()], &bytes[..]);
                }
                Err(err) => assert_eq!(result, Err(err), "len={len} alphabet={alphabet:?}"),
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// 4. Non-canonical trailing bits
////////////////////////////////////////////////////////////////////////////////
#[test]
fn non_canonical_trailing_bits_two_char_tail() {
    // Encode two canonical sextets (v0, v1) into a 2-char tail. Only the high
    // two bits of v1 are used, so decoding must succeed iff `v1 & 0x0F == 0`.
    for alphabet in [Alphabet::StandardNoPadding, Alphabet::UrlNoPadding] {
        let table = alphabet_bytes(alphabet);
        for v0 in 0..64usize {
            for v1 in 0..64usize {
                let input = [table[v0], table[v1]];
                let expected = if v1 & 0x0F == 0 {
                    Ok(vec![((v0 as u8) << 2) | ((v1 as u8) >> 4)])
                } else {
                    Err(DecodeError::InvalidInput)
                };
                let mut out = [0u8; 1];
                let r = base64::decode_into_constant_time(&mut out, &input, alphabet).map(|n| {
                    assert_eq!(n, 1);
                    out.to_vec()
                });
                assert_eq!(r, expected, "v0={v0} v1={v1} alphabet={alphabet:?}");
                check_decode_case(&input, alphabet, "two-char tail");
            }
        }
    }

    // Same, but with the two `=` pads present on a padded alphabet.
    for alphabet in [Alphabet::Standard, Alphabet::Url] {
        let table = alphabet_bytes(alphabet);
        for v1 in 0..64usize {
            let input = [table[1], table[v1], b'=', b'='];
            let expected = if v1 & 0x0F == 0 {
                Ok(vec![(1u8 << 2) | ((v1 as u8) >> 4)])
            } else {
                Err(DecodeError::InvalidInput)
            };
            let mut out = [0u8; 1];
            let r = base64::decode_into_constant_time(&mut out, &input, alphabet).map(|_| out.to_vec());
            assert_eq!(r, expected, "padded v1={v1} alphabet={alphabet:?}");
        }
    }
}

#[test]
fn non_canonical_trailing_bits_three_char_tail() {
    // Three-char tail: only the high four bits of v2 are used.
    for alphabet in [Alphabet::StandardNoPadding, Alphabet::UrlNoPadding] {
        let table = alphabet_bytes(alphabet);
        for v0 in [0usize, 1, 17, 61, 62, 63] {
            for v1 in [0usize, 1, 31, 52, 62, 63] {
                for v2 in 0..64usize {
                    let input = [table[v0], table[v1], table[v2]];
                    let expected = if v2 & 0x03 == 0 {
                        Ok(vec![
                            ((v0 as u8) << 2) | ((v1 as u8) >> 4),
                            ((v1 as u8) << 4) | ((v2 as u8) >> 2),
                        ])
                    } else {
                        Err(DecodeError::InvalidInput)
                    };
                    let mut out = [0u8; 2];
                    let r = base64::decode_into_constant_time(&mut out, &input, alphabet).map(|n| {
                        assert_eq!(n, 2);
                        out.to_vec()
                    });
                    assert_eq!(r, expected, "v0={v0} v1={v1} v2={v2}");
                }
            }
        }
    }

    for alphabet in [Alphabet::Standard, Alphabet::Url] {
        let table = alphabet_bytes(alphabet);
        for v2 in 0..64usize {
            let input = [table[5], table[9], table[v2], b'='];
            let expected = if v2 & 0x03 == 0 {
                Ok(())
            } else {
                Err(DecodeError::InvalidInput)
            };
            let mut out = [0u8; 2];
            let r = base64::decode_into_constant_time(&mut out, &input, alphabet).map(|_| ());
            assert_eq!(r, expected, "padded v2={v2} alphabet={alphabet:?}");
        }
    }
}

#[test]
fn non_canonical_trailing_bits_after_full_block() {
    // 32 valid characters (>= the NEON/AVX2 decode threshold) followed by a
    // non-canonical 2- or 3-character tail. The tail check must still run after
    // the block loop and in the dispatching path.
    for alphabet in ALL_ALPHABETS {
        let table = alphabet_bytes(alphabet);
        let pad = is_padded(alphabet);
        let base = vec![table[10]; 32];

        for bad in 1..16u8 {
            let mut input = base.clone();
            input.push(table[0]);
            input.push(table[bad as usize]);
            if pad {
                input.push(b'=');
                input.push(b'=');
            }
            assert_eq!(
                base64::decode_into_constant_time(&mut [0u8; 32], &input, alphabet).map(|_| ()),
                Err(DecodeError::InvalidInput),
                "2-char tail after block, bad={bad} alphabet={alphabet:?}"
            );
            assert_eq!(
                base64::decode_into(&mut [0u8; 32], &input, alphabet).map(|_| ()),
                Err(DecodeError::InvalidInput),
                "2-char tail after block (dispatched), bad={bad} alphabet={alphabet:?}"
            );
        }

        for bad in 1..4u8 {
            let mut input = base.clone();
            input.push(table[0]);
            input.push(table[0]);
            input.push(table[bad as usize]);
            if pad {
                input.push(b'=');
            }
            assert_eq!(
                base64::decode_into_constant_time(&mut [0u8; 32], &input, alphabet).map(|_| ()),
                Err(DecodeError::InvalidInput),
                "3-char tail after block, bad={bad} alphabet={alphabet:?}"
            );
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// 5. All 256 byte values at every position
////////////////////////////////////////////////////////////////////////////////
#[test]
fn all_byte_values_at_every_position() {
    // Start from canonical encodings of known data so a single mutation is the
    // only reason for an error.
    for len in [4usize, 8, 32, 36, 64] {
        let data = patterned(len, 2);
        for alphabet in ALL_ALPHABETS {
            let valid = ref_encode(&data, alphabet);
            for pos in 0..valid.len() {
                for byte in 0u16..=255 {
                    let mut input = valid.clone();
                    input[pos] = byte as u8;
                    let what = format!("len={len} pos={pos} byte={:#04x} alphabet={alphabet:?}", byte as u8);
                    check_decode_case(&input, alphabet, &what);

                    // The dispatching decoder must agree with the scalar path
                    // for every byte value at every position.
                    let mut scalar_out = vec![0u8; len + 8];
                    let scalar = base64::decode_into_constant_time(&mut scalar_out, &input, alphabet);
                    let mut dispatch_out = vec![0u8; len + 8];
                    let dispatch = base64::decode_into(&mut dispatch_out, &input, alphabet);
                    assert_eq!(dispatch, scalar, "dispatch/scalar disagreement: {what}");
                    if let (Ok(n), Ok(m)) = (scalar, dispatch) {
                        assert_eq!(m, n);
                        assert_eq!(&dispatch_out[..m], &scalar_out[..n], "{what}");
                    }
                }
            }
        }
    }
}

#[test]
fn exhaustive_two_byte_inputs() {
    for alphabet in ALL_ALPHABETS {
        let mut input = [0u8; 2];
        for a in 0u16..=255 {
            for b in 0u16..=255 {
                input[0] = a as u8;
                input[1] = b as u8;
                check_decode_case(&input, alphabet, "exhaustive 2-byte");
            }
        }
    }
}

#[test]
fn structured_four_byte_inputs() {
    // Characters that exercise every validation branch: alphabet members,
    // padding, the other alphabet's separators, whitespace, invalid ASCII, and
    // non-ASCII bytes.
    let interesting: [u8; 12] = [b'A', b'Z', b'a', b'z', b'0', b'9', b'+', b'/', b'-', b'_', b'=', 0x80];
    for alphabet in ALL_ALPHABETS {
        for &a in &interesting {
            for &b in &interesting {
                for &c in &interesting {
                    for &d in &interesting {
                        let input = [a, b, c, d];
                        check_decode_case(&input, alphabet, "structured 4-byte");
                    }
                }
            }
        }
    }
}

#[test]
fn deterministic_fuzz_arbitrary_encoded_bytes() {
    let mut rng = StdRng::seed_from_u64(0xA5A5_5A5A_1234_5678);
    for case in 0..20_000usize {
        let len = (rng.next_u64() % 65) as usize;
        let mut input = vec![0u8; len];
        rng.fill(&mut input);
        // Bias some cases towards plausible base64.
        if case % 3 == 0 {
            for b in &mut input {
                *b = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/-_= "[(*b as usize) % 66];
            }
        }
        let alphabet = ALL_ALPHABETS[case % 4];
        check_decode_case(&input, alphabet, "fuzz arbitrary bytes");

        // The allocating public decoder must agree with the scalar path,
        // including the error variant.
        let mut out = vec![0u8; 256];
        let r = base64::decode_into_constant_time(&mut out, &input, alphabet);
        let scalar = r.map(|n| out[..n].to_vec());
        assert_eq!(
            base64::decode(&input, alphabet),
            scalar,
            "decode() vs scalar mismatch for {input:02x?} alphabet={alphabet:?}"
        );
    }

    // Longer inputs, including the SIMD block boundaries.
    let mut rng = StdRng::seed_from_u64(0x0F0F_1111_2222_3333);
    for case in 0..8_000usize {
        let len = (rng.next_u64() % 201) as usize;
        let mut input = vec![0u8; len];
        rng.fill(&mut input);
        if case % 2 == 0 {
            for b in &mut input {
                *b = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_+/="[(*b as usize) % 65];
            }
        }
        let alphabet = ALL_ALPHABETS[case % 4];
        check_decode_case(&input, alphabet, "long fuzz");
        let mut out = vec![0u8; 256];
        let r = base64::decode_into_constant_time(&mut out, &input, alphabet);
        let scalar = r.map(|n| out[..n].to_vec());
        assert_eq!(base64::decode(&input, alphabet), scalar, "long fuzz decode mismatch");
    }
}

#[test]
fn exhaustive_short_padding_positions() {
    // Exhaustively enumerate all inputs of length 1..=6 over a tiny alphabet
    // containing valid characters, `=`, an invalid character, and the other
    // alphabet's separator. This covers every padding position/count and every
    // length congruence for short inputs.
    let chars = *b"AB=!+";
    for len in 1..=6usize {
        let total = chars.len().pow(len as u32);
        for k in 0..total {
            let mut input = vec![0u8; len];
            let mut x = k;
            for slot in &mut input {
                *slot = chars[x % chars.len()];
                x /= chars.len();
            }
            for alphabet in ALL_ALPHABETS {
                check_decode_case(&input, alphabet, "exhaustive short padding positions");
            }
        }
    }
}

#[test]
fn encode_all_byte_values() {
    // Every byte value as a one-byte input.
    for byte in 0u16..=255 {
        let data = [byte as u8];
        for alphabet in ALL_ALPHABETS {
            check_encode_scalar(&data, alphabet, "single byte");
            check_encode_dispatched(&data, alphabet, "single byte");
        }
    }

    // Every byte value at every position of a one-block (3/24/48 byte) input.
    for len in [1usize, 2, 3, 24, 25, 48, 49] {
        let mut data = vec![0xA5u8; len];
        for pos in 0..len {
            for byte in 0u16..=255 {
                data[pos] = byte as u8;
                for alphabet in ALL_ALPHABETS {
                    let expected = ref_encode(&data, alphabet);
                    assert_eq!(
                        base64::encode(&data, alphabet).as_bytes(),
                        &expected[..],
                        "len={len} pos={pos} byte={byte:#04x} alphabet={alphabet:?}"
                    );
                    let mut buf = vec![0u8; expected.len()];
                    base64::encode_into_constant_time(&mut buf, &data, alphabet).unwrap();
                    assert_eq!(buf, expected);
                }
            }
            data[pos] = 0xA5;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// 6. Output buffer behaviour and error paths
////////////////////////////////////////////////////////////////////////////////
#[test]
fn encode_rejects_wrong_output_length_without_writing() {
    let data = b"hello world";
    for alphabet in ALL_ALPHABETS {
        let expected = base64::encoded_length(data.len(), is_padded(alphabet)).unwrap();
        for delta in [-2isize, -1, 1, 2, 8] {
            let len = expected.checked_add_signed(delta).unwrap();
            let (mut buf, pre) = canary_buffer(len);
            let r = base64::encode_into_constant_time(&mut buf[pre..pre + len], data, alphabet);
            assert_eq!(r, Err(EncodeError::InvalidOutputLength), "len={len}");
            assert!(buf[pre..pre + len].iter().all(|&b| b == FILL), "encode wrote on error");
            check_canaries(&buf, len, "encode wrong length");

            let (mut buf, pre) = canary_buffer(len);
            let r = base64::encode_into(&mut buf[pre..pre + len], data, alphabet);
            assert_eq!(r, Err(EncodeError::InvalidOutputLength), "dispatched len={len}");
            assert!(buf[pre..pre + len].iter().all(|&b| b == FILL));
            check_canaries(&buf, len, "dispatched encode wrong length");
        }
    }
}

#[test]
fn decode_rejects_short_output_without_writing() {
    for (encoded, decoded_len) in [
        (&b"Zm9v"[..], 3usize),
        (&b"aGVsbG8gd29ybGQ="[..], 11),
        (&b"AAAA"[..], 3),
    ] {
        for alphabet in ALL_ALPHABETS {
            if encoded.ends_with(b"=") && !is_padded(alphabet) {
                continue;
            }
            if decoded_len == 0 {
                continue;
            }
            let len = decoded_len - 1;
            let (mut buf, pre) = canary_buffer(len);
            let r = base64::decode_into_constant_time(&mut buf[pre..pre + len], encoded, alphabet);
            assert_eq!(r, Err(DecodeError::InvalidOutputLength));
            assert!(buf[pre..pre + len].iter().all(|&b| b == FILL), "decode wrote on length error");
            check_canaries(&buf, len, "decode short buffer");

            let (mut buf, pre) = canary_buffer(len);
            let r = base64::decode_into(&mut buf[pre..pre + len], encoded, alphabet);
            assert_eq!(r, Err(DecodeError::InvalidOutputLength), "dispatched");
            assert!(buf[pre..pre + len].iter().all(|&b| b == FILL));
            check_canaries(&buf, len, "dispatched decode short buffer");
        }
    }
}

#[test]
fn decode_into_larger_buffer_does_not_touch_tail() {
    for len in [0usize, 1, 2, 3, 7, 23, 24, 25, 47, 48, 49, 100] {
        let data = patterned(len, 3);
        for alphabet in ALL_ALPHABETS {
            let encoded = ref_encode(&data, alphabet);
            let (mut buf, pre) = canary_buffer(len + 64);
            buf[pre + len..pre + len + 64].fill(FILL);
            let r = base64::decode_into_constant_time(&mut buf[pre..pre + len + 64], &encoded, alphabet);
            assert_eq!(r, Ok(len));
            assert_eq!(&buf[pre..pre + len], &data[..]);
            assert!(buf[pre + len..pre + len + 64].iter().all(|&b| b == FILL));
            check_canaries(&buf, len + 64, "large decode buffer");
        }
    }
}

/// Finding **F3** (Low): `decode_into_constant_time` / `decode_into` leave a
/// partially decoded plaintext prefix in the caller's buffer when they return
/// `Err(DecodeError::InvalidInput)`. The public documentation does not say that
/// the output buffer is unspecified on error, so a caller that ignores the
/// result (or logs it and reuses the buffer) observes a prefix of the plaintext.
#[test]
fn decode_error_may_leave_partial_plaintext() {
    let mut out = [0xAAu8; 8];
    let r = base64::decode_into_constant_time(&mut out, b"AAAA!!!!", Alphabet::Standard);
    assert_eq!(r, Err(DecodeError::InvalidInput));
    assert_ne!(out, [0xAAu8; 8], "documented finding: output is modified on error");
    assert_eq!(&out[..3], &[0, 0, 0], "first block was decoded before the error");
}

////////////////////////////////////////////////////////////////////////////////
// 7. Length arithmetic near `usize::MAX`
////////////////////////////////////////////////////////////////////////////////
#[test]
fn encoded_length_overflow_boundary() {
    // `encoded_length` is public and `const`, so the boundary can be checked
    // without allocating anything.
    const CHUNKS: usize = usize::MAX / 4; // usize::MAX == 4*CHUNKS + 3
    const BASE: usize = CHUNKS * 4; // == usize::MAX - 3
    const D0: usize = CHUNKS * 3; // complete_chunks == CHUNKS, rem == 0

    const D0_PADDED: Option<usize> = base64::encoded_length(D0, true);
    const D0_UNPADDED: Option<usize> = base64::encoded_length(D0, false);
    const D1: usize = D0 + 1;
    const D1_PADDED: Option<usize> = base64::encoded_length(D1, true);
    const D1_UNPADDED: Option<usize> = base64::encoded_length(D1, false);
    const D2: usize = D0 + 2;
    const D2_PADDED: Option<usize> = base64::encoded_length(D2, true);
    const D2_UNPADDED: Option<usize> = base64::encoded_length(D2, false);
    const D3: usize = D0 + 3;
    const D3_UNPADDED: Option<usize> = base64::encoded_length(D3, false);

    assert_eq!(BASE, usize::MAX - 3);
    assert_eq!(D0_PADDED, Some(BASE));
    assert_eq!(D0_UNPADDED, Some(BASE));
    // One leftover byte with padding needs BASE + 4, which overflows.
    assert_eq!(D1_PADDED, None);
    assert_eq!(D1_UNPADDED, Some(BASE + 2));
    // Two leftover bytes with padding needs BASE + 4, which overflows.
    assert_eq!(D2_PADDED, None);
    assert_eq!(D2_UNPADDED, Some(BASE + 3));
    // A further complete chunk needs BASE + 4, which overflows.
    assert_eq!(D3_UNPADDED, None);

    // The documented examples.
    assert_eq!(base64::encoded_length(3, true), Some(4));
    assert_eq!(base64::encoded_length(1, false), Some(2));
    assert_eq!(base64::encoded_length(usize::MAX, true), None);
    assert_eq!(base64::encoded_length(usize::MAX, false), None);

    // Cross-check the reference length function for a range of ordinary sizes.
    for len in 0..=4096usize {
        for pad in [false, true] {
            assert_eq!(base64::encoded_length(len, pad), Some(ref_encode_len(len, pad)));
        }
    }
}

fn ref_encode_len(len: usize, pad: bool) -> usize {
    let full = len / 3;
    let rem = len % 3;
    if rem == 0 {
        full * 4
    } else if pad {
        full * 4 + 4
    } else if rem == 1 {
        full * 4 + 2
    } else {
        full * 4 + 3
    }
}

/// `decoded_length` is crate-private, so it cannot be called from an
/// integration test. Its overflow behaviour is instead pinned by a `const`
/// replica of the same arithmetic, and the reachable public behaviour is
/// checked through `decode`/`decode_into_constant_time`.
#[test]
fn decoded_length_overflow_replica() {
    const fn decoded_length_replica(n: usize) -> Option<usize> {
        let full = n / 4;
        let base = match full.checked_mul(3) {
            Some(v) => v,
            None => return None,
        };
        match n % 4 {
            0 => Some(base),
            2 => match base.checked_add(1) {
                Some(v) => Some(v),
                None => return None,
            },
            3 => match base.checked_add(2) {
                Some(v) => Some(v),
                None => return None,
            },
            _ => None,
        }
    }

    // No `usize` input overflows the multiplication: full <= usize::MAX/4 and
    // 3 * (usize::MAX/4) + 2 < usize::MAX.
    const MAX_INPUT: Option<usize> = decoded_length_replica(usize::MAX);
    assert_eq!(MAX_INPUT, Some((usize::MAX / 4) * 3 + 2));
    const { assert!((usize::MAX / 4) * 3 + 2 < usize::MAX) };
    assert_eq!(decoded_length_replica(usize::MAX - 3), Some((usize::MAX - 3) / 4 * 3));

    // Lengths congruent to 1 mod 4 are rejected, and the accepted lengths agree
    // with the reference decoder for all reachable inputs.
    for n in 0..=4096usize {
        assert_eq!(decoded_length_replica(n), ref_decoded_length(n).ok());
    }
    assert_eq!(decoded_length_replica(1), None);
    assert_eq!(decoded_length_replica(5), None);
}

/// `const fn` entry points must work at compile time (no allocation).
#[test]
fn const_entry_points() {
    const ENC_FOO: [u8; 4] = base64::encode_array::<4>(b"foo", Alphabet::Standard);
    const ENC_FOO_URL: [u8; 4] = base64::encode_array::<4>(b"foo", Alphabet::UrlNoPadding);
    const ENC_EMPTY: [u8; 0] = base64::encode_array::<0>(b"", Alphabet::Url);
    const DEC_FOO: Result<[u8; 3], DecodeError> = base64::decode_array::<3>(b"Zm9v", Alphabet::Standard);
    const DEC_EMPTY: Result<[u8; 0], DecodeError> = base64::decode_array::<0>(b"", Alphabet::Url);
    const DEC_WRONG_SIZE: Result<[u8; 2], DecodeError> = base64::decode_array::<2>(b"Zm9v", Alphabet::Standard);
    const DEC_INVALID: Result<[u8; 3], DecodeError> = base64::decode_array::<3>(b"!!!!", Alphabet::Standard);

    assert_eq!(&ENC_FOO, b"Zm9v");
    assert_eq!(&ENC_FOO_URL, b"Zm9v");
    assert_eq!(ENC_EMPTY.len(), 0);
    assert_eq!(DEC_FOO.unwrap(), *b"foo");
    assert_eq!(DEC_EMPTY.unwrap().len(), 0);
    assert_eq!(DEC_WRONG_SIZE, Err(DecodeError::InvalidOutputLength));
    assert_eq!(DEC_INVALID, Err(DecodeError::InvalidInput));
}

////////////////////////////////////////////////////////////////////////////////
// 8. Documentation claims that matter
////////////////////////////////////////////////////////////////////////////////
/// The `Display` text for `EncodeError::InvalidOutputLength` names the
/// encoder's length function, not the decoder's.
#[test]
fn encode_error_message_names_encoded_length() {
    let message = format!("{}", EncodeError::InvalidOutputLength);
    assert_eq!(message, "output buffer size must be exactly equal to encoded_length(input)");
}

/// Padding placement is observable through the returned error variant:
/// `strip_padding_info` inspects the trailing bytes and branches on whether
/// they are `=`. This test records that behavior; it does not measure timing.
#[test]
fn padding_error_variant_depends_on_input_bytes() {
    assert_eq!(
        base64::decode(b"Zg==", Alphabet::Standard).map(|_| ()),
        Ok(()),
        "well-padded input is accepted"
    );
    assert_eq!(
        base64::decode(b"Zg=", Alphabet::Standard).map(|_| ()),
        Err(DecodeError::InvalidInputLength),
        "a truncated pad run selects the length error"
    );
    assert_eq!(
        base64::decode(b"Z===", Alphabet::Standard).map(|_| ()),
        Err(DecodeError::InvalidPadding),
        "an over-long pad run selects the padding error"
    );
}

////////////////////////////////////////////////////////////////////////////////
// 9. RFC 4648 vectors (independent of the crate's own unit tests)
////////////////////////////////////////////////////////////////////////////////
/// Inputs strictly below every SIMD dispatch threshold, so this test exercises
/// only the scalar core. This is the Miri-friendly subset (the dispatched SIMD
/// paths use NEON/AVX2 intrinsics that Miri cannot execute) and it is the test
/// that validates the `from_utf8_unchecked` call sites under Miri.
#[test]
fn short_inputs_scalar_path_miri_friendly() {
    for len in 0..=23usize {
        let data = patterned(len, 2);
        for alphabet in ALL_ALPHABETS {
            let expected = ref_encode(&data, alphabet);

            let encoded = base64::encode(&data, alphabet);
            assert_eq!(encoded.as_bytes(), &expected[..]);
            assert!(core::str::from_utf8(encoded.as_bytes()).is_ok());

            let mut appended = String::from("~~");
            base64::encode_into_string(&mut appended, &data, alphabet);
            assert_eq!(appended.as_bytes(), [b"~~".as_slice(), &expected[..]].concat());

            assert_eq!(base64::decode(&encoded, alphabet), Ok(data.clone()));

            let mut out = vec![0u8; len];
            assert_eq!(base64::decode_into(&mut out, encoded.as_bytes(), alphabet), Ok(len));
            assert_eq!(out, data);

            // Constant-time path as well.
            assert_eq!(base64::decode_into_constant_time(&mut out, &expected, alphabet), Ok(len));
            assert_eq!(out, data);
        }
    }
}

#[test]
fn rfc4648_vectors() {
    let vectors: &[(&[u8], &str)] = &[
        (b"", ""),
        (b"f", "Zg=="),
        (b"fo", "Zm8="),
        (b"foo", "Zm9v"),
        (b"foob", "Zm9vYg=="),
        (b"fooba", "Zm9vYmE="),
        (b"foobar", "Zm9vYmFy"),
    ];
    for (raw, encoded) in vectors {
        for alphabet in [Alphabet::Standard, Alphabet::Url] {
            assert_eq!(base64::encode(raw, alphabet), *encoded);
            assert_eq!(base64::decode(encoded.as_bytes(), alphabet), Ok(raw.to_vec()));
            assert_eq!(ref_encode(raw, alphabet), encoded.as_bytes());
        }
    }
    // RFC 4648 section 10 example.
    let raw = [0x14, 0xfb, 0x9c, 0x03, 0xd9, 0x7e];
    assert_eq!(base64::encode(raw, Alphabet::Standard), "FPucA9l+");
    assert_eq!(base64::decode(b"FPucA9l+", Alphabet::Standard), Ok(raw.to_vec()));
}
