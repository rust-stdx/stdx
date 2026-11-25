//! Differential and boundary tests for the `base32` safe core.
//!
//! Everything here is differential: an independent, deliberately naive reference
//! implementation (`ref_encode` / `ref_decode`, written straight from RFC 4648)
//! is compared against the crate for every alphabet, every length in `0..=512`,
//! and every possible single-byte input at every position of representative
//! inputs.
//!
//! Run with:
//!
//! ```text
//! cargo test -p base32 --test audit_core
//! ```

#![cfg(feature = "alloc")]

use base32::{Alphabet, DecodeError, EncodeError};
use random::{Rng, RngExt, SeedableRng, rngs::ChaChaRng};

/// All ten alphabets, including the no-padding variants that the crate's own
/// unit tests never exercise.
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

/// The `}` sentinel used by the canary tests; also deliberately a byte that is
/// never produced by any alphabet or by valid UTF-8 continuation handling.
const CANARY: u8 = 0xA5;

fn alphabet_table(a: Alphabet) -> &'static [u8; 32] {
    match a {
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => b"abcdefghijklmnopqrstuvwxyz234567",
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => b"0123456789ABCDEFGHIJKLMNOPQRSTUV",
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => b"0123456789abcdefghijklmnopqrstuv",
        Alphabet::Crockford => b"0123456789ABCDEFGHJKMNPQRSTVWXYZ",
        Alphabet::Z32 => b"ybndrfg8ejkmcpqxot1uwisza345h769",
    }
}

/// Padding policy as documented in the crate-level table.
fn is_padded(a: Alphabet) -> bool {
    matches!(
        a,
        Alphabet::Rfc4648 | Alphabet::Rfc4648Lower | Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexLower
    )
}

/// The no-padding variant of `a`.
///
/// Tests that deliberately pass an unpadded content fragment (char mapping,
/// trailing-bit sweeps, SIMD tails) must use this, because padded alphabets now
/// reject fragments that are not a whole number of 8-character blocks.
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

////////////////////////////////////////////////////////////////////////////////
// Independent reference implementation (RFC 4648, bit-at-a-time)
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RefError {
    Input,
    Length,
    Padding,
}

fn ref_error(e: DecodeError) -> RefError {
    match e {
        DecodeError::InvalidInput => RefError::Input,
        DecodeError::InvalidLength => RefError::Length,
        DecodeError::InvalidPadding => RefError::Padding,
    }
}

/// Straightforward RFC 4648 encoder: accumulate up to 5 input bytes in the top
/// of a `u64`, then pull 5-bit groups out of the most significant end.
fn ref_encode(data: &[u8], a: Alphabet) -> Vec<u8> {
    let table = alphabet_table(a);
    let mut out = Vec::with_capacity(ref_encoded_length(data.len(), is_padded(a)));
    for chunk in data.chunks(5) {
        let mut acc = 0u64;
        for (i, &b) in chunk.iter().enumerate() {
            acc |= (b as u64) << (56 - 8 * i);
        }
        let nchars = (chunk.len() * 8).div_ceil(5);
        for j in 0..nchars {
            out.push(table[((acc >> (59 - 5 * j)) & 0x1F) as usize]);
        }
        if is_padded(a) {
            out.resize(out.len() + 8 - nchars, b'=');
        }
    }
    out
}

/// Independent decoder: strip padding, validate the shape exactly as the crate
/// documents, map characters through a linear table search, then shift bits into
/// an accumulator. Rejects non-canonical trailing bits (`acc != 0` after the
/// final full byte has been emitted).
fn ref_decode(input: &[u8], a: Alphabet) -> Result<Vec<u8>, RefError> {
    let mut content_end = input.len();

    if is_padded(a) {
        let mut pad = 0usize;
        while content_end > 0 && input[content_end - 1] == b'=' {
            content_end -= 1;
            pad += 1;
        }
        let allowed_pad = matches!(pad, 0 | 1 | 3 | 4 | 6);
        if !allowed_pad || pad > 6 {
            return Err(RefError::Padding);
        }
        if pad == 0 {
            match input.len() % 8 {
                0 => {}
                1 | 3 | 6 => return Err(RefError::Length),
                _ => return Err(RefError::Padding),
            }
        }
        if pad > 0 {
            if !input.len().is_multiple_of(8) {
                return Err(RefError::Padding);
            }
            // 1 pad => 7 content chars, 3 => 5, 4 => 4, 6 => 2.
            if content_end % 8 != (8 - pad) % 8 {
                return Err(RefError::Padding);
            }
        }
    } else if input.last() == Some(&b'=') {
        // No-padding modes reject a trailing '=' as a padding error; an interior
        // '=' is caught later as an invalid character.
        return Err(RefError::Padding);
    }

    let content = &input[..content_end];
    let rem = content.len() % 8;
    let extra = match rem {
        0 => 0,
        2 => 1,
        4 => 2,
        5 => 3,
        7 => 4,
        _ => return Err(RefError::Length),
    };

    let table = alphabet_table(a);
    let mut out = Vec::with_capacity(content.len() / 8 * 5 + extra);
    let mut acc = 0u64;
    let mut nbits = 0u32;
    for &c in content {
        let Some(v) = table.iter().position(|&x| x == c) else {
            return Err(RefError::Input);
        };
        acc = (acc << 5) | v as u64;
        nbits += 5;
        if nbits >= 8 {
            nbits -= 8;
            out.push((acc >> nbits) as u8);
            acc &= (1u64 << nbits) - 1;
        }
    }
    // Leftover bits are the unused bits of the last character and must be zero.
    if acc != 0 {
        return Err(RefError::Input);
    }
    if nbits as usize != (5 * rem) % 8 {
        return Err(RefError::Length);
    }
    Ok(out)
}

fn ref_encoded_length(bytes_len: usize, padding: bool) -> usize {
    if bytes_len == 0 {
        return 0;
    }
    let chunks = bytes_len / 5;
    let rem = bytes_len % 5;
    if rem == 0 {
        chunks * 8
    } else if padding {
        chunks * 8 + 8
    } else {
        (bytes_len * 8).div_ceil(5)
    }
}

/// Checked (u128) reference used by the overflow tests.
fn ref_encoded_length_checked(bytes_len: usize, padding: bool) -> Option<usize> {
    let n = bytes_len as u128;
    if n == 0 {
        return Some(0);
    }
    let chunks = n / 5;
    let rem = n % 5;
    let value = if rem == 0 {
        chunks * 8
    } else if padding {
        chunks * 8 + 8
    } else {
        (n * 8).div_ceil(5)
    };
    if value <= usize::MAX as u128 {
        Some(value as usize)
    } else {
        None
    }
}

////////////////////////////////////////////////////////////////////////////////
// Deterministic PRNG
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
// Differential helpers
////////////////////////////////////////////////////////////////////////////////
/// SIMD dispatch kicks in at 40 bytes (encode) / 64 chars (decode) content.
/// Miri does not model NEON intrinsics, so skip only the dispatched variants
/// for larger inputs when running under Miri.
fn miri_skip_encode(data: &[u8]) -> bool {
    cfg!(miri) && data.len() >= 40
}

fn miri_skip_decode(encoded: &[u8]) -> bool {
    let content_len = encoded.iter().position(|&c| c == b'=').unwrap_or(encoded.len());
    cfg!(miri) && content_len >= 64
}

fn check_encode(data: &[u8], a: Alphabet) {
    let expected = ref_encode(data, a);
    assert!(expected.iter().all(u8::is_ascii), "reference produced non-ASCII");

    assert_eq!(
        base32::encoded_length(data.len(), is_padded(a)),
        Some(expected.len()),
        "encoded_length mismatch: len={} alphabet={a:?}",
        data.len()
    );

    // Constant-time core: exact buffer.
    let mut buf = vec![0u8; expected.len()];
    base32::encode_into_constant_time(&mut buf, data, a).expect("ct encode with exact buffer");
    assert_eq!(buf, expected, "ct encode len={} alphabet={a:?}", data.len());
    // The `from_utf8_unchecked` precondition in `encode`/`encode_into_string` is
    // that every produced byte is ASCII.
    assert!(std::str::from_utf8(&buf).is_ok());

    // Constant-time core: oversized buffer must not be scribbled past the output.
    let mut buf = vec![CANARY; expected.len() + 16];
    base32::encode_into_constant_time(&mut buf, data, a).expect("ct encode with oversized buffer");
    assert_eq!(
        &buf[..expected.len()],
        &expected[..],
        "ct encode prefix len={} alphabet={a:?}",
        data.len()
    );
    assert!(
        buf[expected.len()..].iter().all(|&b| b == CANARY),
        "ct encode wrote past expected length: len={} alphabet={a:?}",
        data.len()
    );

    if miri_skip_encode(data) {
        return;
    }

    // Public API (may take the SIMD path).
    let got = base32::encode(data, a);
    assert_eq!(got.as_bytes(), &expected[..], "encode len={} alphabet={a:?}", data.len());
    assert!(std::str::from_utf8(got.as_bytes()).is_ok());

    let mut buf = vec![CANARY; expected.len() + 16];
    base32::encode_into(&mut buf, data, a).expect("encode_into with oversized buffer");
    assert_eq!(
        &buf[..expected.len()],
        &expected[..],
        "encode_into prefix len={} alphabet={a:?}",
        data.len()
    );
    assert!(
        buf[expected.len()..].iter().all(|&b| b == CANARY),
        "encode_into wrote past expected length: len={} alphabet={a:?}",
        data.len()
    );
}

fn check_decode_ct(input: &[u8], a: Alphabet) {
    let expected = ref_decode(input, a);
    // `decoded_len <= 5/8 * input_len`, so `input.len()` + canaries is a safe
    // upper bound for any write the decoder may perform.
    let mut out = vec![CANARY; input.len() + 8];
    let result = base32::decode_into_constant_time(&mut out, input, a);
    match (result, &expected) {
        (Ok(()), Ok(bytes)) => assert_eq!(
            &out[..bytes.len()],
            &bytes[..],
            "ct decode bytes mismatch input={input:?} alphabet={a:?}"
        ),
        (Err(e), Err(re)) => assert_eq!(
            ref_error(e),
            *re,
            "ct decode error class mismatch input={input:?} alphabet={a:?}"
        ),
        (result, expected) => {
            panic!("ct decode mismatch input={input:?} alphabet={a:?}: got {result:?}, reference {expected:?}")
        }
    }
    assert!(
        out[input.len()..].iter().all(|&b| b == CANARY),
        "ct decode wrote past decoded-length upper bound: input={input:?} alphabet={a:?}"
    );
}

fn check_decode_into(input: &[u8], a: Alphabet) {
    let expected = ref_decode(input, a);
    let mut out = vec![CANARY; input.len() + 8];
    let result = base32::decode_into(&mut out, input, a);
    match (result, &expected) {
        (Ok(()), Ok(bytes)) => assert_eq!(
            &out[..bytes.len()],
            &bytes[..],
            "decode_into bytes mismatch input={input:?} alphabet={a:?}"
        ),
        (Err(e), Err(re)) => assert_eq!(
            ref_error(e),
            *re,
            "decode_into error class mismatch input={input:?} alphabet={a:?}"
        ),
        (result, expected) => {
            panic!("decode_into mismatch input={input:?} alphabet={a:?}: got {result:?}, reference {expected:?}")
        }
    }
    assert!(
        out[input.len()..].iter().all(|&b| b == CANARY),
        "decode_into wrote past decoded-length upper bound: input={input:?} alphabet={a:?}"
    );
}

fn check_decode_alloc(input: &[u8], a: Alphabet) {
    let expected = ref_decode(input, a);
    match (base32::decode(input, a), expected) {
        (Ok(got), Ok(want)) => assert_eq!(got, want, "decode bytes mismatch input={input:?} alphabet={a:?}"),
        (Err(e), Err(re)) => {
            assert_eq!(ref_error(e), re, "decode error class mismatch input={input:?} alphabet={a:?}")
        }
        (got, want) => panic!("decode mismatch input={input:?} alphabet={a:?}: got {got:?}, reference {want:?}"),
    }
}

/// Run every public decode entry point on `input`.
fn check_decode_all(input: &[u8], a: Alphabet) {
    check_decode_ct(input, a);
    if miri_skip_decode(input) {
        return;
    }
    check_decode_into(input, a);
    check_decode_alloc(input, a);
}

////////////////////////////////////////////////////////////////////////////////
// Length arithmetic
////////////////////////////////////////////////////////////////////////////////
#[test]
fn encoded_length_sweep_and_overflow() {
    // Exhaustive small lengths.
    for len in 0..=512usize {
        for padding in [false, true] {
            assert_eq!(
                base32::encoded_length(len, padding),
                Some(ref_encoded_length(len, padding)),
                "encoded_length({len}, {padding})"
            );
        }
    }

    // Large values, including the whole boundary region near usize::MAX.
    let mut values = vec![usize::MAX, usize::MAX - 1, usize::MAX - 2];
    let m = (usize::MAX / 8) * 5;
    for d in 0..=32usize {
        values.push(m - d);
        if let Some(v) = m.checked_add(d) {
            values.push(v);
        }
        // The no-padding length does not overflow until `5 * usize::MAX / 8`,
        // far above the point where `bytes_len * 8` overflows.
        values.push(usize::MAX / 8 - d);
        values.push(usize::MAX / 8 + d);
    }
    for n in (usize::MAX - 64)..=usize::MAX {
        values.push(n);
    }
    for &n in &values {
        for padding in [false, true] {
            assert_eq!(
                base32::encoded_length(n, padding),
                ref_encoded_length_checked(n, padding),
                "encoded_length({n}, {padding})"
            );
        }
    }

    // Regression: `1 << 61` is representable as a slice length on 64-bit and its
    // unpadded encoding is only ~3.7e18 bytes, so it must not be reported as an
    // overflow. The old code computed `bytes_len * 8` first and returned `None`.
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(base32::encoded_length(1 << 61, false), Some(3_689_348_814_741_910_324));
    }

    // Largest representable length for each mode, by binary search, then the
    // first rejected value above it.
    for padding in [false, true] {
        let mut lo = 0usize;
        let mut hi = usize::MAX;
        while lo < hi {
            let mid = lo + (hi - lo) / 2 + 1;
            if base32::encoded_length(mid, padding).is_some() {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        assert!(base32::encoded_length(lo, padding).is_some(), "padding={padding} max={lo}");
        if lo < usize::MAX {
            assert_eq!(base32::encoded_length(lo + 1, padding), None, "padding={padding} above max");
        }
    }

    // Documented examples.
    assert_eq!(base32::encoded_length(0, true), Some(0));
    assert_eq!(base32::encoded_length(1, false), Some(2));
    assert_eq!(base32::encoded_length(usize::MAX, true), None);
    assert_eq!(base32::encoded_length(usize::MAX, false), None);

    // Random full-range values against the u128 reference.
    let mut rng = ChaChaRng::seed_from_u64(0xE4E4_1111_2222_3333);
    for _ in 0..5000 {
        let n = rng.next_u64() as usize;
        for padding in [false, true] {
            assert_eq!(
                base32::encoded_length(n, padding),
                ref_encoded_length_checked(n, padding),
                "random encoded_length({n}, {padding})"
            );
        }
    }

    // Monotonicity: encoding never shrinks as the input grows.
    let mut previous = [0usize; 2];
    for n in 1..=10_000usize {
        for (i, padding) in [false, true].into_iter().enumerate() {
            let value = base32::encoded_length(n, padding).unwrap();
            assert!(value >= previous[i], "encoded_length not monotonic at {n} padding={padding}");
            previous[i] = value;
        }
    }
}

/// `decoded_length` is private; its checked arithmetic is verified indirectly by
/// the exhaustive length sweep below (every accepted content length must be
/// reflected in exactly-sized output buffers, and invalid remainders must be
/// rejected before any write). The `usize::MAX` boundary is unreachable from a
/// real slice on this platform and is covered by the crate's own unit test
/// `test_decoded_length_does_not_overflow` plus source inspection.
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn decoded_length_remainder_classes() {
    // Inputs made of a valid character: acceptance is determined purely by the
    // content-length remainder, for both padding modes.
    for a in [
        Alphabet::Rfc4648NoPadding,
        Alphabet::Rfc4648LowerNoPadding,
        Alphabet::Z32,
        Alphabet::Crockford,
    ] {
        let fill = alphabet_table(a)[0];
        for len in 0..=520usize {
            let input = vec![fill; len];
            let expected_ok = matches!(len % 8, 0 | 2 | 4 | 5 | 7);
            let mut out = vec![CANARY; len + 8];
            let got = base32::decode_into_constant_time(&mut out, &input, a);
            if expected_ok {
                assert_eq!(got, Ok(()), "len={len} alphabet={a:?} should decode");
                let expected_len = len / 8 * 5
                    + match len % 8 {
                        0 => 0,
                        2 => 1,
                        4 => 2,
                        5 => 3,
                        7 => 4,
                        _ => unreachable!(),
                    };
                // Everything decoded is the all-zero symbol.
                assert!(out[..expected_len].iter().all(|&b| b == 0));
            } else {
                assert_eq!(
                    got,
                    Err(DecodeError::InvalidLength),
                    "len={len} alphabet={a:?} must be InvalidLength"
                );
            }
            assert!(out[len..].iter().all(|&b| b == CANARY));
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Encode / decode sweeps
////////////////////////////////////////////////////////////////////////////////
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn encode_all_lengths_all_alphabets() {
    let mut rng = ChaChaRng::seed_from_u64(0xE4_0123_4567_89AB);
    for len in 0..=512usize {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        for a in ALL_ALPHABETS {
            check_encode(&data, a);
        }
    }
}

#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn roundtrip_all_lengths_all_alphabets() {
    let mut rng = ChaChaRng::seed_from_u64(0xC0FF_EE00_1234_5678);
    for len in 0..=512usize {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        for a in ALL_ALPHABETS {
            let encoded = ref_encode(&data, a);
            check_decode_all(&encoded, a);

            // Also compare the crate's own encoder output against the reference
            // at the decode boundary.
            let crate_encoded = if miri_skip_encode(&data) {
                let mut buf = vec![0u8; encoded.len()];
                base32::encode_into_constant_time(&mut buf, &data, a).unwrap();
                buf
            } else {
                base32::encode(&data, a).into_bytes()
            };
            assert_eq!(crate_encoded, encoded, "encode mismatch len={len} alphabet={a:?}");
            assert_eq!(ref_decode(&crate_encoded, a).as_deref(), Ok(&data[..]));
        }
    }
}

/// Padded alphabets require canonical padding and reject unpadded input.
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn padded_alphabets_reject_unpadded_input() {
    for a in ALL_ALPHABETS.iter().copied().filter(|a| is_padded(*a)) {
        for len in 1..=40usize {
            let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(29)).collect();
            let padded = ref_encode(&data, a);
            let mut unpadded = padded.clone();
            while unpadded.last() == Some(&b'=') {
                unpadded.pop();
            }
            if padded.len() == unpadded.len() {
                // `len % 5 == 0`: a full block, padding was never required.
                assert_eq!(base32::decode(&unpadded, a).unwrap(), data, "len={len} alphabet={a:?}");
            } else {
                assert!(
                    base32::decode(&unpadded, a).is_err(),
                    "padded alphabet accepted unpadded input {unpadded:?} alphabet={a:?}"
                );
            }
            check_decode_all(&unpadded, a);
        }
    }
}

/// Long unpadded inputs must be rejected by every padded alphabet, including
/// lengths that cross the SIMD dispatch thresholds.
#[test]
fn long_padded_alphabets_reject_unpadded_input() {
    let padded = [
        Alphabet::Rfc4648,
        Alphabet::Rfc4648Lower,
        Alphabet::Rfc4648Hex,
        Alphabet::Rfc4648HexLower,
    ];
    for alphabet in padded {
        for len in 1..=512usize {
            let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(41)).collect();
            let mut encoded = base32::encode(&data, alphabet).into_bytes();
            while encoded.last() == Some(&b'=') {
                encoded.pop();
            }

            let expected = match encoded.len() % 8 {
                0 => Ok(data.clone()),
                1 | 3 | 6 => Err(DecodeError::InvalidLength),
                _ => Err(DecodeError::InvalidPadding),
            };
            assert_eq!(
                base32::decode(&encoded, alphabet),
                expected,
                "len={len} alphabet={alphabet:?} stripped_len={}",
                encoded.len()
            );

            let mut out = vec![0u8; data.len()];
            let result = base32::decode_into(&mut out, &encoded, alphabet);
            match expected {
                Ok(bytes) => {
                    assert_eq!(result, Ok(()), "len={len} alphabet={alphabet:?}");
                    assert_eq!(&out[..bytes.len()], &bytes[..]);
                }
                Err(err) => assert_eq!(result, Err(err), "len={len} alphabet={alphabet:?}"),
            }
        }
    }
}

#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn large_roundtrip_and_tails() {
    let mut rng = ChaChaRng::seed_from_u64(0xB16B_00B5_1234_5678);
    for &len in &[640usize, 641, 642, 1000, 4096, 8191, 65536, 65537] {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        for a in ALL_ALPHABETS {
            let expected = ref_encode(&data, a);
            let encoded = base32::encode(&data, a);
            assert_eq!(encoded.as_bytes(), &expected[..], "large encode len={len} alphabet={a:?}");
            assert_eq!(
                base32::decode(&encoded, a).unwrap(),
                data,
                "large roundtrip len={len} alphabet={a:?}"
            );
            check_decode_ct(&expected, a);

            // A non-canonical last character must be rejected no matter how many
            // SIMD blocks precede the scalar tail.
            let content_len = expected.iter().position(|&c| c == b'=').unwrap_or(expected.len());
            if content_len % 8 != 0 {
                let table = alphabet_table(a);
                let canonical_last = expected[content_len - 1];
                let last_v = table.iter().position(|&c| c == canonical_last).unwrap();
                let unused = match content_len % 8 {
                    2 => 2,
                    4 => 4,
                    5 => 1,
                    7 => 3,
                    other => panic!("unexpected remainder {other}"),
                };
                // First alphabet entry whose value has non-zero unused bits.
                let bad = table
                    .iter()
                    .find(|&&c| {
                        let v = table.iter().position(|&x| x == c).unwrap();
                        v & ((1 << unused) - 1) != 0
                    })
                    .copied()
                    .unwrap();
                let mut input = expected.clone();
                input[content_len - 1] = bad;
                assert_eq!(
                    base32::decode(&input, a),
                    Err(DecodeError::InvalidInput),
                    "non-canonical tail len={len} alphabet={a:?} (canonical value {last_v})"
                );
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Padding rules
////////////////////////////////////////////////////////////////////////////////
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn padding_matrix() {
    // only-'=' inputs of every short length
    for len in 0..=20usize {
        let input = vec![b'='; len];
        check_decode_all(&input, Alphabet::Rfc4648);
        check_decode_all(&input, Alphabet::Rfc4648NoPadding);
        // Empty input is the only successful case.
        assert_eq!(base32::decode(&input, Alphabet::Rfc4648).is_ok(), len == 0);
        assert_eq!(base32::decode(&input, Alphabet::Rfc4648NoPadding).is_ok(), len == 0);
    }

    // Canonical encodings mutated by adding/removing/replacing/inserting '='.
    for padding_mode in [Alphabet::Rfc4648, Alphabet::Rfc4648NoPadding] {
        for len in 0..=16usize {
            let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(0x9D).wrapping_add(3)).collect();
            let canonical = ref_encode(&data, padding_mode);

            check_decode_all(&canonical, padding_mode);

            let mut cases: Vec<Vec<u8>> = Vec::new();
            // append one '='
            let mut c = canonical.clone();
            c.push(b'=');
            cases.push(c);
            // prepend one '='
            let mut c = canonical.clone();
            c.insert(0, b'=');
            cases.push(c);
            // remove the last byte (may remove padding or content)
            if !canonical.is_empty() {
                cases.push(canonical[..canonical.len() - 1].to_vec());
            }
            // replace every position with '='
            for pos in 0..canonical.len() {
                let mut c = canonical.clone();
                c[pos] = b'=';
                cases.push(c);
            }
            // insert '=' at every position
            for pos in 0..=canonical.len() {
                let mut c = canonical.clone();
                c.insert(pos, b'=');
                cases.push(c);
            }
            for case in &cases {
                check_decode_all(case, padding_mode);
            }
        }
    }

    // Lengths that are not multiples of 8 but carry padding.
    for input in [
        b"AA=====" as &[u8],
        b"AA=======",
        b"AAA=====",
        b"AAAAAAA=",
        b"AAAAAAAAA=",
        b"A=======",
        b"========",
        b"================",
        b"AAAAAAAA========",
        b"AAAAAAAAAA======",
        b"AAAAAAAAAA=====",
        b"AAAAAAAAAA======A",
    ] {
        check_decode_all(input, Alphabet::Rfc4648);
    }
    for input in [
        b"AA" as &[u8],
        b"AAA",
        b"AAAAAA",
        b"AAAAAAAA",
        b"A=A",
        b"AA=",
        b"=AA",
        b"AAAAAAAA=AAAAAAAA",
    ] {
        check_decode_all(input, Alphabet::Rfc4648NoPadding);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Non-canonical trailing bits
////////////////////////////////////////////////////////////////////////////////
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn non_canonical_trailing_bits() {
    // data lengths 1..=9 cover every content remainder class (2, 4, 5, 7 and 0
    // once blocks are full) with and without padding.
    for a in ALL_ALPHABETS {
        let ua = unpadded(a);
        let table = alphabet_table(a);
        for len in 1..=9usize {
            let data: Vec<u8> = (0..len).map(|i| (i as u8).wrapping_mul(31).wrapping_add(7)).collect();
            let canonical = ref_encode(&data, a);
            let content_len = canonical.iter().position(|&c| c == b'=').unwrap_or(canonical.len());
            let content = &canonical[..content_len];
            let rem = content_len % 8;
            let unused = match rem {
                2 => 2,
                4 => 4,
                5 => 1,
                7 => 3,
                0 => 0,
                other => panic!("unexpected content remainder {other}"),
            };

            // Mutate the last content character through the whole alphabet.
            let mut accepted = 0usize;
            for &ch in table {
                let mut mutated = content.to_vec();
                mutated[content_len - 1] = ch;
                let expected = ref_decode(&mutated, ua);
                if expected.is_ok() {
                    accepted += 1;
                }
                check_decode_all(&mutated, ua);

                // A rejected mutation must be rejected as InvalidInput (the
                // trailing-bits check), not by accident.
                if rem != 0 && expected == Err(RefError::Input) {
                    let mut out = [0u8; 64];
                    assert_eq!(
                        base32::decode_into_constant_time(&mut out, &mutated, ua),
                        Err(DecodeError::InvalidInput),
                        "input={mutated:?} alphabet={ua:?}"
                    );
                }
            }
            assert_eq!(
                accepted,
                1 << (5 - unused),
                "accepted last-char variants len={len} alphabet={a:?} rem={rem}"
            );

            // Every second-to-last character uses all five bits, so all 32
            // substitutions of a full-byte position must decode successfully
            // (they change the data but remain canonical).
            if content_len >= 2 {
                let idx = content_len - 2;
                // Only positions whose five bits are fully consumed. For rem
                // classes the second-to-last char is always fully used.
                for &ch in table {
                    let mut mutated = content.to_vec();
                    mutated[idx] = ch;
                    assert!(
                        ref_decode(&mutated, ua).is_ok(),
                        "full-byte position rejected: {mutated:?} alphabet={ua:?}"
                    );
                    check_decode_all(&mutated, ua);
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Character mapping
////////////////////////////////////////////////////////////////////////////////
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn char_to_quintet_all_bytes_all_alphabets() {
    for a in ALL_ALPHABETS {
        let a = unpadded(a);
        let table = alphabet_table(a);
        let zero_char = table[0];
        for c in 0..=255u8 {
            // First character is a fully-consumed quintet; the second is the
            // zero symbol, so the result byte is `value << 3`.
            let input = [c, zero_char];
            let mut out = [CANARY; 4];
            let result = base32::decode_into_constant_time(&mut out, &input, a);
            match table.iter().position(|&x| x == c) {
                Some(v) => {
                    assert_eq!(result, Ok(()), "byte {c:#04x} should be valid for {a:?}");
                    assert_eq!(out[0], (v as u8) << 3, "byte {c:#04x} mapping for {a:?}");
                }
                None => {
                    assert_eq!(
                        result,
                        Err(DecodeError::InvalidInput),
                        "byte {c:#04x} should be invalid for {a:?}"
                    );
                }
            }
            // Other 3 bytes of the scratch buffer are past the decoded length.
            assert_eq!(&out[1..], &[CANARY; 3]);
            if !miri_skip_decode(&input) {
                check_decode_into(&input, a);
                check_decode_alloc(&input, a);
            }
        }
    }
}

/// The crate documents Crockford as `0-9 A-H J-K M-N P-Z` (no `I`, `L`, `O`,
/// `U`) and Z32 as its own lowercase alphabet. Neither is case-insensitive: the
/// canonical Crockford spec's lowercase/`I`/`L`/`O` aliases are intentionally
/// not supported. Pin this so an accidental alias change is caught, and so the
/// documentation stays honest.
#[test]
fn crockford_z32_case_and_aliases() {
    let crockford = alphabet_table(Alphabet::Crockford);
    for &c in b"ILOUilou" {
        assert!(!crockford.contains(&c), "Crockford must not contain {}", c as char);
        let mut out = [0u8; 1];
        assert_eq!(
            base32::decode_into_constant_time(&mut out, &[c, b'0'], Alphabet::Crockford),
            Err(DecodeError::InvalidInput),
            "Crockford must reject {}",
            c as char
        );
    }
    // All lowercase letters are rejected (uppercase-only alphabet).
    for c in b'a'..=b'z' {
        let mut out = [0u8; 1];
        assert_eq!(
            base32::decode_into_constant_time(&mut out, &[c, b'0'], Alphabet::Crockford),
            Err(DecodeError::InvalidInput),
            "Crockford must reject lowercase {}",
            c as char
        );
    }
    // Every documented Crockford character maps back to its quintet.
    for (v, &c) in crockford.iter().enumerate() {
        let mut out = [0u8; 1];
        base32::decode_into_constant_time(&mut out, &[c, b'0'], Alphabet::Crockford).unwrap();
        assert_eq!(out[0], (v as u8) << 3);
    }

    // Z32 is lowercase-only.
    let z32 = alphabet_table(Alphabet::Z32);
    for c in b'A'..=b'Z' {
        let mut out = [0u8; 1];
        assert_eq!(
            base32::decode_into_constant_time(&mut out, &[c, b'y'], Alphabet::Z32),
            Err(DecodeError::InvalidInput),
            "Z32 must reject uppercase {}",
            c as char
        );
    }
    for (v, &c) in z32.iter().enumerate() {
        let mut out = [0u8; 1];
        base32::decode_into_constant_time(&mut out, &[c, b'y'], Alphabet::Z32).unwrap();
        assert_eq!(out[0], (v as u8) << 3, "Z32 mapping for {}", c as char);
    }
}

/// RFC 4648 alphabets must reject the opposite case (no implicit case folding).
#[test]
fn rfc_alphabets_are_case_sensitive() {
    let upper = alphabet_table(Alphabet::Rfc4648);
    let lower = alphabet_table(Alphabet::Rfc4648Lower);
    for c in b'a'..=b'z' {
        let mut out = [0u8; 1];
        assert_eq!(
            base32::decode_into_constant_time(&mut out, &[c, b'A'], Alphabet::Rfc4648NoPadding),
            Err(DecodeError::InvalidInput)
        );
    }
    for c in b'A'..=b'Z' {
        let mut out = [0u8; 1];
        assert_eq!(
            base32::decode_into_constant_time(&mut out, &[c, b'a'], Alphabet::Rfc4648LowerNoPadding),
            Err(DecodeError::InvalidInput)
        );
    }
    assert_ne!(upper, lower);
}

////////////////////////////////////////////////////////////////////////////////
// Exhaustive single-byte input-space sweep
////////////////////////////////////////////////////////////////////////////////
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn every_byte_at_every_position() {
    let mut rng = ChaChaRng::seed_from_u64(0xDEAD_BEEF_1234_5678);
    // 8 chars = one full quad, 16 chars = two quads plus padding semantics.
    for &data_len in &[5usize, 10] {
        let data = {
            let mut v = vec![0u8; data_len];
            rng.fill(&mut v);
            v
        };
        for a in ALL_ALPHABETS {
            let canonical = ref_encode(&data, a);
            assert_eq!(canonical.len(), if data_len == 5 { 8 } else { 16 });
            for pos in 0..canonical.len() {
                for byte in 0..=255u8 {
                    let mut input = canonical.clone();
                    input[pos] = byte;
                    check_decode_ct(&input, a);
                    if !miri_skip_decode(&input) {
                        check_decode_into(&input, a);
                        check_decode_alloc(&input, a);
                    }
                }
            }
        }
    }
}

/// Same sweep across the SIMD/scalar boundary, so the tail handling that runs
/// after the vector kernels is checked with every possible trailing byte.
#[test]
fn dispatch_boundary_tail_sweep() {
    if cfg!(miri) {
        // NEON intrinsics are not modelled by Miri.
        return;
    }
    let mut rng = ChaChaRng::seed_from_u64(0x5EED_0BAD_F00D_1234);
    for &data_len in &[25usize, 32, 33, 40, 41, 42, 43, 64, 65, 66, 67, 68, 69, 70, 71, 72, 80] {
        let data = {
            let mut v = vec![0u8; data_len];
            rng.fill(&mut v);
            v
        };
        for a in ALL_ALPHABETS {
            let canonical = ref_encode(&data, a);
            let content_len = canonical.iter().position(|&c| c == b'=').unwrap_or(canonical.len());
            // Mutate every position within the last 9 chars (the scalar tail
            // after a 64-char SIMD block) across the alphabets.
            let start = content_len.saturating_sub(9);
            for pos in start..canonical.len() {
                for &ch in alphabet_table(a).iter().chain([b'=', b'!', 0x80].iter()) {
                    let mut input = canonical.clone();
                    input[pos] = ch;
                    check_decode_all(&input, a);
                }
            }
            check_decode_all(&canonical, a);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Output buffers, canaries, decode_array
////////////////////////////////////////////////////////////////////////////////
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn output_buffers_and_canaries() {
    let mut rng = ChaChaRng::seed_from_u64(0x1234_ABCD_5678_EF90);
    for a in ALL_ALPHABETS {
        for len in 0..=80usize {
            let data = {
                let mut v = vec![0u8; len];
                rng.fill(&mut v);
                v
            };
            let encoded = ref_encode(&data, a);

            // Too-small encode buffer: rejected before any write.
            if !encoded.is_empty() {
                let mut ring = vec![CANARY; encoded.len() - 1 + 2 * 16];
                let before = ring.clone();
                let result = base32::encode_into(&mut ring[16..16 + encoded.len() - 1], &data, a);
                assert_eq!(result, Err(EncodeError::InvalidOutputLength));
                assert_eq!(ring, before, "undersized encode_into wrote to the buffer");
                let result = base32::encode_into_constant_time(&mut ring[16..16 + encoded.len() - 1], &data, a);
                assert_eq!(result, Err(EncodeError::InvalidOutputLength));
                assert_eq!(ring, before, "undersized ct encode wrote to the buffer");
            }

            // Exact buffer placed inside a canary ring.
            let mut ring = vec![CANARY; encoded.len() + 2 * 16];
            base32::encode_into(&mut ring[16..16 + encoded.len()], &data, a).unwrap();
            assert_eq!(&ring[16..16 + encoded.len()], &encoded[..]);
            assert!(ring[..16].iter().all(|&b| b == CANARY), "encode_into underflow canary");
            assert!(
                ring[16 + encoded.len()..].iter().all(|&b| b == CANARY),
                "encode_into overflow canary"
            );

            // Oversized slice: bytes after `encoded.len()` must stay untouched.
            let mut ring = vec![CANARY; encoded.len() + 64];
            base32::encode_into(&mut ring, &data, a).unwrap();
            assert_eq!(&ring[..encoded.len()], &encoded[..]);
            assert!(ring[encoded.len()..].iter().all(|&b| b == CANARY));

            // Decode into an exact buffer inside a canary ring.
            let decoded = ref_decode(&encoded, a).unwrap();
            let mut ring = vec![CANARY; decoded.len() + 2 * 16];
            base32::decode_into(&mut ring[16..16 + decoded.len()], &encoded, a).unwrap();
            assert_eq!(&ring[16..16 + decoded.len()], &decoded[..]);
            assert!(ring[..16].iter().all(|&b| b == CANARY), "decode_into underflow canary");
            assert!(
                ring[16 + decoded.len()..].iter().all(|&b| b == CANARY),
                "decode_into overflow canary"
            );

            // Oversized decode slice.
            let mut ring = vec![CANARY; decoded.len() + 64];
            base32::decode_into(&mut ring, &encoded, a).unwrap();
            assert_eq!(&ring[..decoded.len()], &decoded[..]);
            assert!(ring[decoded.len()..].iter().all(|&b| b == CANARY));

            // Too-small decode buffer: rejected without writing.
            if !decoded.is_empty() {
                let mut ring = vec![CANARY; decoded.len() - 1 + 2 * 16];
                let before = ring.clone();
                let result = base32::decode_into(&mut ring[16..16 + decoded.len() - 1], &encoded, a);
                assert_eq!(result, Err(DecodeError::InvalidLength));
                assert_eq!(ring, before, "undersized decode_into wrote to the buffer");
                let result = base32::decode_into_constant_time(&mut ring[16..16 + decoded.len() - 1], &encoded, a);
                assert_eq!(result, Err(DecodeError::InvalidLength));
                assert_eq!(ring, before, "undersized ct decode wrote to the buffer");
            }
        }
    }
}

macro_rules! check_decode_array {
    ($($n:literal),* $(,)?) => {
        $({
            // Round-trip for a payload that is exactly `$n` bytes.
            let data: Vec<u8> = (0usize..$n).map(|i| (i as u8).wrapping_mul(37).wrapping_add(11)).collect();
            for a in ALL_ALPHABETS {
                let encoded = ref_encode(&data, a);
                let got: [u8; $n] = base32::decode_array::<$n>(&encoded, a).unwrap();
                assert_eq!(&got[..], &data[..], "decode_array::<{}> alphabet={a:?}", $n);

                // Appending one full 5-byte block (8 chars) must be rejected as
                // a length mismatch rather than silently truncated.
                let mut longer = encoded.clone();
                longer.extend_from_slice(&ref_encode(&[0u8; 5], a));
                assert_eq!(
                    base32::decode_array::<$n>(&longer, a),
                    Err(DecodeError::InvalidLength),
                    "decode_array::<{}> must reject a longer input (alphabet={a:?})",
                    $n
                );
            }
        })*
    };
}

#[test]
fn decode_array_exact_length() {
    check_decode_array!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    check_decode_array!(20, 32, 33, 40, 41, 64);

    // Wrong-sized array for a known 5-byte payload.
    const TOO_LONG: Result<[u8; 6], DecodeError> = base32::decode_array::<6>(b"NBSWY3DP", Alphabet::Rfc4648);
    assert_eq!(TOO_LONG, Err(DecodeError::InvalidLength));
    const TOO_SHORT: Result<[u8; 4], DecodeError> = base32::decode_array::<4>(b"NBSWY3DP", Alphabet::Rfc4648);
    assert_eq!(TOO_SHORT, Err(DecodeError::InvalidLength));

    // Non-canonical trailing bits are rejected by the const path too.
    const NON_CANONICAL: Result<[u8; 4], DecodeError> =
        base32::decode_array::<4>(b"MZXW6YR", Alphabet::Rfc4648NoPadding);
    assert_eq!(NON_CANONICAL, Err(DecodeError::InvalidInput));

    // Empty input, zero-length array.
    const EMPTY: Result<[u8; 0], DecodeError> = base32::decode_array::<0>(b"", Alphabet::Z32);
    assert_eq!(EMPTY, Ok([0u8; 0]));

    // Interior '=' is an invalid character even though the padding is applied
    // at the end of the block.
    const BAD_CHAR: Result<[u8; 2], DecodeError> = base32::decode_array::<2>(b"=AAA====", Alphabet::Rfc4648);
    assert_eq!(BAD_CHAR, Err(DecodeError::InvalidInput));
}

/// Small, scalar-only suite: this is the test to run under Miri
/// (`cargo +nightly miri test -p base32 --test audit_core miri_smoke`).
/// It exercises every remainder class, both padding modes, all ten alphabets,
/// and each public entry point while staying strictly below the SIMD dispatch
/// thresholds (40 input bytes to encode, 64 content characters to decode), so no
/// intrinsic is ever executed.
#[test]
fn miri_smoke() {
    let mut rng = ChaChaRng::seed_from_u64(0x1122_3344_5566_7788);
    for a in ALL_ALPHABETS {
        for len in 0..=12usize {
            let data = {
                let mut v = vec![0u8; len];
                rng.fill(&mut v);
                v
            };
            check_encode(&data, a);
            let encoded = ref_encode(&data, a);
            check_decode_ct(&encoded, a);
            check_decode_into(&encoded, a);
            check_decode_alloc(&encoded, a);
        }
        // Largest inputs that still take the scalar path.
        let data = {
            let mut v = vec![0u8; 39];
            rng.fill(&mut v);
            v
        };
        check_encode(&data, a);
        let data = {
            let mut v = vec![0u8; 30];
            rng.fill(&mut v);
            v
        };
        let encoded = ref_encode(&data, a);
        assert!(encoded.iter().position(|&c| c == b'=').unwrap_or(encoded.len()) < 64);
        check_decode_all(&encoded, a);
    }
}

/// Randomised (deterministic) garbage inputs, so the decoder is driven through
/// combinations of interior `=`, boundary `=`, invalid bytes and truncations that
/// canonical-encoding mutations do not reach.
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn random_garbage_decode() {
    let mut rng = ChaChaRng::seed_from_u64(0xA11C_E5EE_D00D_5EED);
    let extras: [u8; 10] = [b'=', b'!', 0x00, 0x7F, 0x80, 0xFF, b'0', b'a', b'A', b'y'];
    for a in ALL_ALPHABETS {
        let table = alphabet_table(a);
        for len in 0..=200usize {
            for _ in 0..4 {
                let mut input = Vec::with_capacity(len);
                for _ in 0..len {
                    let roll = rng.random::<u8>();
                    if roll & 3 != 0 {
                        input.push(table[(roll >> 2) as usize % 32]);
                    } else {
                        input.push(extras[(roll >> 2) as usize % extras.len()]);
                    }
                }
                check_decode_ct(&input, a);
                if !miri_skip_decode(&input) {
                    check_decode_into(&input, a);
                    check_decode_alloc(&input, a);
                }
            }
        }
    }
}

#[test]
fn single_and_double_byte_exhaustive() {
    // Every 1-byte input for every alphabet (2-character remainder class).
    for a in ALL_ALPHABETS {
        for b in 0..=255u8 {
            check_encode(&[b], a);
            let encoded = base32::encode([b], a);
            assert_eq!(base32::decode(&encoded, a).unwrap(), vec![b], "byte={b:#04x} alphabet={a:?}");
        }
    }
    // Every 2-byte input for a padded and an unpadded alphabet (4-character
    // remainder class, all 16-bit patterns).
    for a in [
        Alphabet::Rfc4648,
        Alphabet::Rfc4648NoPadding,
        Alphabet::Crockford,
        Alphabet::Z32,
    ] {
        for hi in 0..=255u8 {
            for lo in 0..=255u8 {
                let data = [hi, lo];
                let expected = ref_encode(&data, a);
                let encoded = base32::encode(data, a);
                assert_eq!(encoded.as_bytes(), &expected[..], "data={data:?} alphabet={a:?}");
                assert_eq!(base32::decode(&encoded, a).unwrap(), data.to_vec());
            }
        }
    }
}

#[test]
fn partial_output_on_decode_error() {
    // `decode_into*` decodes full blocks and only checks the accumulated error at
    // the end, so a later invalid character leaves the already-decoded prefix in
    // the caller's buffer. This is not a memory-safety issue and `decode` (which
    // owns its buffer) never exposes it, but callers must not consume an output
    // buffer after an `Err`.
    let mut out = [0u8; 10];
    let input = b"NBSWY3DP!!!!!!!!";
    assert_eq!(
        base32::decode_into_constant_time(&mut out, input, Alphabet::Rfc4648),
        Err(DecodeError::InvalidInput)
    );
    assert_eq!(
        &out[..5],
        b"hello",
        "the first (valid) block was written before the error was reported"
    );
    assert_eq!(base32::decode(input, Alphabet::Rfc4648), Err(DecodeError::InvalidInput));
}

////////////////////////////////////////////////////////////////////////////////
// encode_into_string / encode_array
////////////////////////////////////////////////////////////////////////////////
#[test]
#[cfg_attr(miri, ignore = "exhaustive sweep; run miri_smoke under Miri instead")]
fn encode_into_string_both_branches() {
    let mut rng = ChaChaRng::seed_from_u64(0x0BAD_C0DE_FEED_1234);
    // 160 bytes is the largest input whose padded encoding still fits the
    // 256-byte stack buffer; go past it to exercise the heap branch too.
    for len in 0..=300usize {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        for a in ALL_ALPHABETS {
            let expected = ref_encode(&data, a);
            let mut s = String::from("prefix:");
            base32::encode_into_string(&mut s, &data, a);
            let full = format!("prefix:{}", String::from_utf8(expected.clone()).unwrap());
            assert_eq!(s, full, "encode_into_string len={len} alphabet={a:?}");
            assert!(s.is_char_boundary(7));

            // Repeated appends keep a valid String and never lose the prefix.
            let mut t = String::new();
            base32::encode_into_string(&mut t, &data, a);
            base32::encode_into_string(&mut t, &data, a);
            let mut twice = expected.clone();
            twice.extend_from_slice(&expected);
            assert_eq!(t.as_bytes(), &twice[..]);
        }
    }
}

#[test]
fn encode_array_matches_reference() {
    // Spot-check every alphabet at each remainder class, including the const
    // evaluation itself.
    const RFC5: [u8; 8] = base32::encode_array::<8>(b"hello", Alphabet::Rfc4648);
    const RFC1: [u8; 8] = base32::encode_array::<8>(b"h", Alphabet::Rfc4648);
    const RFC2: [u8; 8] = base32::encode_array::<8>(b"he", Alphabet::Rfc4648);
    const RFC3: [u8; 8] = base32::encode_array::<8>(b"hel", Alphabet::Rfc4648);
    const RFC4: [u8; 8] = base32::encode_array::<8>(b"hell", Alphabet::Rfc4648);
    const NOPAD1: [u8; 2] = base32::encode_array::<2>(b"h", Alphabet::Rfc4648NoPadding);
    const NOPAD2: [u8; 4] = base32::encode_array::<4>(b"he", Alphabet::Rfc4648NoPadding);
    const NOPAD3: [u8; 5] = base32::encode_array::<5>(b"hel", Alphabet::Rfc4648NoPadding);
    const NOPAD4: [u8; 7] = base32::encode_array::<7>(b"hell", Alphabet::Rfc4648NoPadding);
    const LOWER: [u8; 2] = base32::encode_array::<2>(b"h", Alphabet::Rfc4648LowerNoPadding);
    const HEXL: [u8; 2] = base32::encode_array::<2>(b"h", Alphabet::Rfc4648HexLowerNoPadding);
    const CROCK: [u8; 2] = base32::encode_array::<2>(b"h", Alphabet::Crockford);
    const Z32: [u8; 2] = base32::encode_array::<2>(b"h", Alphabet::Z32);

    assert_eq!(&RFC5, &ref_encode(b"hello", Alphabet::Rfc4648)[..]);
    assert_eq!(&RFC1, &ref_encode(b"h", Alphabet::Rfc4648)[..]);
    assert_eq!(&RFC2, &ref_encode(b"he", Alphabet::Rfc4648)[..]);
    assert_eq!(&RFC3, &ref_encode(b"hel", Alphabet::Rfc4648)[..]);
    assert_eq!(&RFC4, &ref_encode(b"hell", Alphabet::Rfc4648)[..]);
    assert_eq!(&NOPAD1, &ref_encode(b"h", Alphabet::Rfc4648NoPadding)[..]);
    assert_eq!(&NOPAD2, &ref_encode(b"he", Alphabet::Rfc4648NoPadding)[..]);
    assert_eq!(&NOPAD3, &ref_encode(b"hel", Alphabet::Rfc4648NoPadding)[..]);
    assert_eq!(&NOPAD4, &ref_encode(b"hell", Alphabet::Rfc4648NoPadding)[..]);
    assert_eq!(&LOWER, &ref_encode(b"h", Alphabet::Rfc4648LowerNoPadding)[..]);
    assert_eq!(&HEXL, &ref_encode(b"h", Alphabet::Rfc4648HexLowerNoPadding)[..]);
    assert_eq!(&CROCK, &ref_encode(b"h", Alphabet::Crockford)[..]);
    assert_eq!(&Z32, &ref_encode(b"h", Alphabet::Z32)[..]);

    const EMPTY: [u8; 0] = base32::encode_array::<0>(b"", Alphabet::Z32);
    assert_eq!(EMPTY.len(), 0);
}
