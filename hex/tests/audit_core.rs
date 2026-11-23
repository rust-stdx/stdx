//! Tests for `hex/hex.rs` safe core.
//!
//! Covers the constant-time encoder/decoder, the `encode_into` / `decode_into`
//! dispatch (which runs the NEON/AVX2/wasm kernels when the corresponding
//! target features are enabled, so the SIMD paths are exercised through the
//! safe API),
//! `nibble_to_hex` / `nibble_from_hex` semantics, length checking, the
//! `String::from_utf8_unchecked` preconditions, `encode_into_string`, and the
//! `const fn` array APIs.
//!
//! Everything is compared against an independent reference implementation that
//! uses table lookups and a plain `match`, never the crate's bit tricks.

use hex::{Alphabet, DecodeError, EncodeError};
use rand::{Rng, RngExt, SeedableRng, rngs::StdRng};

const LOWER: &[u8; 16] = b"0123456789abcdef";
const UPPER: &[u8; 16] = b"0123456789ABCDEF";
const CANARY: u8 = 0xA5;

// ---------------------------------------------------------------------------
// Reference implementation
// ---------------------------------------------------------------------------

fn ref_encode(data: &[u8], alphabet: Alphabet) -> Vec<u8> {
    let table = match alphabet {
        Alphabet::Lower => LOWER,
        Alphabet::Upper => UPPER,
    };
    let mut out = Vec::with_capacity(data.len() * 2);
    for &b in data {
        out.push(table[(b >> 4) as usize]);
        out.push(table[(b & 0x0F) as usize]);
    }
    out
}

fn ref_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn ref_decode(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
    if !data.len().is_multiple_of(2) {
        return Err(DecodeError::InvalidInputLength);
    }
    let mut out = Vec::with_capacity(data.len() / 2);
    for pair in data.as_chunks::<2>().0 {
        let hi = ref_nibble(pair[0]).ok_or(DecodeError::InvalidInput)?;
        let lo = ref_nibble(pair[1]).ok_or(DecodeError::InvalidInput)?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Deterministic seeded `StdRng`, fixed seed.
// ---------------------------------------------------------------------------

/// A deterministic, non-uniform test pattern for `len` bytes.
fn pattern(len: usize, salt: u64) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(0x9E37_79B9_7F4A_7C15 ^ salt.rotate_left(17));
    {
        let mut v = vec![0u8; len];
        rng.fill(&mut v);
        v
    }
}

// ---------------------------------------------------------------------------
// Canary helpers
// ---------------------------------------------------------------------------

/// Runs `f` with a slice of `len` canary-filled bytes inside a larger
/// allocation, then asserts that the bytes before and after `len` were not
/// touched. Returns the slice contents and whatever `f` returned.
fn with_canary<R>(len: usize, f: impl FnOnce(&mut [u8]) -> R) -> (Vec<u8>, R) {
    const PRE: usize = 7;
    const POST: usize = 9;
    let mut buf = vec![CANARY; PRE + len + POST];
    let result = {
        let slice = &mut buf[PRE..PRE + len];
        f(slice)
    };
    assert!(
        buf[..PRE].iter().all(|&b| b == CANARY),
        "write before the output slice ({len} bytes)"
    );
    assert!(
        buf[PRE + len..].iter().all(|&b| b == CANARY),
        "write past the end of the output slice ({len} bytes)"
    );
    (buf[PRE..PRE + len].to_vec(), result)
}

// ---------------------------------------------------------------------------
// 1. nibble_to_hex: all 16 nibbles in both alphabets, via every byte value.
// ---------------------------------------------------------------------------

#[test]
fn nibble_to_hex_all_nibbles_both_alphabets() {
    for b in 0..=255u8 {
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let expected = ref_encode(&[b], alphabet);
            let mut out = [0u8; 2];
            hex::encode_into_constant_time(&mut out, &[b], alphabet).unwrap();
            assert_eq!(&out, &expected[..], "byte {b:#04x} {alphabet:?}");
        }
    }
    // Every nibble value appears at least once in each position for a full
    // 0..=255 sweep, but pin the exact table entries explicitly too.
    for n in 0..16u8 {
        let b = (n << 4) | n;
        assert_eq!(hex::encode([b]).as_bytes()[0], LOWER[n as usize]);
        assert_eq!(hex::encode([b]).as_bytes()[1], LOWER[n as usize]);
        assert_eq!(hex::encode_with_alphabet([b], Alphabet::Upper).as_bytes()[0], UPPER[n as usize]);
    }
}

// ---------------------------------------------------------------------------
// 2. nibble_from_hex: exhaustive accept/reject over all 256 byte values.
// ---------------------------------------------------------------------------

#[test]
fn scalar_decode_accepts_exactly_hex_chars() {
    // Put the candidate byte in each position of an even-length input so the
    // high-nibble and low-nibble code paths are both covered.
    for pos in 0..2usize {
        for c in 0..=255u8 {
            let mut input = *b"00";
            input[pos] = c;
            let expected_ok = ref_nibble(c).is_some();

            let mut out = [0u8; 1];
            let ct = hex::decode_into_constant_time(&mut out, &input);
            assert_eq!(
                ct.is_ok(),
                expected_ok,
                "constant-time decoder wrong decision for {c:#04x} at {pos}: {ct:?}"
            );
            if expected_ok {
                assert_eq!(out[0], ref_decode(&input).unwrap()[0], "wrong value for {c:#04x} at {pos}");
            } else {
                assert_eq!(ct, Err(DecodeError::InvalidInput));
            }
        }
    }
}

#[test]
fn dispatch_decode_accepts_exactly_hex_chars() {
    // Same sweep through the public dispatcher, including lengths that cross
    // the NEON threshold (>= 32) and leave a scalar tail (33..=63).
    for len in [2usize, 4, 32, 34, 48, 64, 65, 96, 97] {
        for pos in 0..len {
            for c in 0..=255u8 {
                let mut input = vec![b'0'; len];
                // Make the baseline valid but non-uniform.
                for (k, slot) in input.iter_mut().enumerate() {
                    *slot = LOWER[k % 16];
                }
                input[pos] = c;
                let expected = ref_decode(&input);

                let (got, res) = with_canary(len / 2, |out| hex::decode_into(out, &input));
                match expected {
                    Ok(want) => {
                        assert!(res.is_ok(), "len={len} pos={pos} c={c:#04x}: {res:?}");
                        assert_eq!(got, want, "len={len} pos={pos} c={c:#04x}");
                    }
                    Err(e) => {
                        assert_eq!(res, Err(e), "len={len} pos={pos} c={c:#04x}");
                    }
                }
            }
        }
    }
}

#[test]
fn decode_rejects_non_hex_bytes_explicitly() {
    let rejected: &[u8] = b"\x00 \t\n\r\x0b\x0cgGxX=+-/_ :;,.@`[]{}~!?#$%^&*()|\\\"'\x7f\x80\xff\x01\xfe";
    for &c in rejected {
        assert!(ref_nibble(c).is_none());
        for alphabet_pad in 0..2 {
            let input = if alphabet_pad == 0 { [c, b'0'] } else { [b'0', c] };
            // Scalar.
            let mut out = [0u8; 1];
            assert_eq!(
                hex::decode_into_constant_time(&mut out, &input),
                Err(DecodeError::InvalidInput),
                "scalar accepted {c:#04x}"
            );
            // Dispatcher (short input).
            let mut out = [0u8; 1];
            assert_eq!(
                hex::decode_into(&mut out, &input),
                Err(DecodeError::InvalidInput),
                "dispatch accepted short {c:#04x}"
            );
            // A length that forces the SIMD path (when available) and also
            // leaves a scalar tail.
            for len in [32usize, 34, 64, 66] {
                let mut long = vec![b'a'; len];
                long[0] = b'0';
                long[1] = b'0';
                long[len - 2] = c;
                let mut out = vec![0u8; len / 2];
                assert_eq!(
                    hex::decode_into(&mut out, &long),
                    Err(DecodeError::InvalidInput),
                    "dispatch accepted {c:#04x} in len={len}"
                );
            }
        }
    }
    // No padding in hex: '=' and related are invalid, and a padded string is
    // odd-length anyway.
    assert_eq!(hex::decode(b"00="), Err(DecodeError::InvalidInputLength));
    assert_eq!(hex::decode(b"00=="), Err(DecodeError::InvalidInput));
    assert_eq!(hex::decode(b"0\x00"), Err(DecodeError::InvalidInput));
    assert_eq!(hex::decode(b"\x000"), Err(DecodeError::InvalidInput));
}

#[test]
fn odd_lengths_reject_every_byte_at_every_position() {
    // Odd-length inputs must fail on the length check alone, for every byte
    // value at every position, on both the scalar and dispatched paths, and
    // must never write output.
    for len in [1usize, 3, 5, 31, 33, 35, 63, 65] {
        for pos in 0..len {
            for c in 0..=255u8 {
                let mut input = vec![b'0'; len];
                input[pos] = c;
                assert_eq!(
                    hex::decode_into_constant_time(&mut [], &input),
                    Err(DecodeError::InvalidInputLength),
                    "scalar len={len} pos={pos} c={c:#04x}"
                );
                let (out, res) = with_canary(len / 2, |out| hex::decode_into(out, &input));
                assert_eq!(
                    res,
                    Err(DecodeError::InvalidInputLength),
                    "dispatch len={len} pos={pos} c={c:#04x}"
                );
                assert!(out.iter().all(|&b| b == CANARY));
            }
        }
    }
}

#[test]
fn scalar_and_dispatch_agree_on_random_invalid_strings() {
    // Multi-character invalid inputs, not just single-byte mutations.
    const POOL: &[u8] = b"0123456789abcdefABCDEFgGxX=+-/_ \x00\x7f\x80\xff@`[]{}";
    let mut rng = StdRng::seed_from_u64(0x1234_5678_9ABC_DEF0);
    for len in 0..=64usize {
        for _ in 0..40 {
            let input: Vec<u8> = (0..len).map(|_| POOL[rng.next_u64() as usize % POOL.len()]).collect();
            let expected = ref_decode(&input);
            let (scalar_out, scalar_res) = with_canary(len / 2, |out| hex::decode_into_constant_time(out, &input));
            let (dispatch_out, dispatch_res) = with_canary(len / 2, |out| hex::decode_into(out, &input));
            assert_eq!(
                scalar_res.is_ok(),
                expected.is_ok(),
                "scalar accept/reject mismatch for {:?}",
                String::from_utf8_lossy(&input)
            );
            assert_eq!(
                dispatch_res.is_ok(),
                expected.is_ok(),
                "dispatch accept/reject mismatch for {:?}",
                String::from_utf8_lossy(&input)
            );
            if let Ok(want) = expected {
                assert_eq!(scalar_out, want);
                assert_eq!(dispatch_out, want);
            }
        }
    }
}

#[test]
fn decode_is_case_insensitive_both_alphabets() {
    let data: Vec<u8> = (0..=255).collect();
    for alphabet in [Alphabet::Lower, Alphabet::Upper] {
        let encoded = hex::encode_with_alphabet(&data, alphabet);
        assert_eq!(hex::decode(&encoded).unwrap(), data);
        // Flip the case of every letter.
        let flipped: Vec<u8> = encoded
            .bytes()
            .map(|b| {
                if b.is_ascii_lowercase() {
                    b.to_ascii_uppercase()
                } else if b.is_ascii_uppercase() {
                    b.to_ascii_lowercase()
                } else {
                    b
                }
            })
            .collect();
        assert_eq!(hex::decode(&flipped).unwrap(), data);
    }
}

// ---------------------------------------------------------------------------
// 3. Length sweeps 0..=512: encode and decode against the reference.
// ---------------------------------------------------------------------------

#[test]
fn scalar_encode_matches_reference_all_lengths() {
    for len in 0..=512usize {
        let data = pattern(len, len as u64);
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let want = ref_encode(&data, alphabet);
            let (got, res) = with_canary(len * 2, |out| hex::encode_into_constant_time(out, &data, alphabet));
            assert_eq!(res, Ok(()), "len={len}");
            assert_eq!(got, want, "constant-time encode, len={len}, {alphabet:?}");
        }
    }
}

#[test]
fn scalar_decode_matches_reference_all_lengths() {
    for len in 0..=512usize {
        let raw = pattern(len, 0xC0FFEE ^ len as u64);
        let encoded = ref_encode(&raw, Alphabet::Lower);
        let (got, res) = with_canary(len, |out| hex::decode_into_constant_time(out, &encoded));
        assert_eq!(res, Ok(()), "len={len}");
        assert_eq!(got, raw, "constant-time decode, len={len}");
    }
}

#[test]
fn dispatch_encode_matches_reference_all_lengths() {
    for len in 0..=512usize {
        let data = pattern(len, 0x1234_5678 ^ len as u64);
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let want = ref_encode(&data, alphabet);
            let (got, res) = with_canary(len * 2, |out| hex::encode_into(out, &data, alphabet));
            assert_eq!(res, Ok(()), "len={len}");
            assert_eq!(got, want, "dispatch encode, len={len}, {alphabet:?}");
        }
    }
}

#[test]
fn dispatch_decode_matches_reference_all_lengths() {
    for len in 0..=512usize {
        let raw = pattern(len, 0xDEAD_BEEF ^ len as u64);
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let encoded = ref_encode(&raw, alphabet);
            let (got, res) = with_canary(len, |out| hex::decode_into(out, &encoded));
            assert_eq!(res, Ok(()), "len={len}");
            assert_eq!(got, raw, "dispatch decode, len={len}, {alphabet:?}");
        }
    }
}

#[test]
fn roundtrip_both_directions_all_lengths() {
    for len in 0..=512usize {
        let data = pattern(len, 0x5EED ^ len as u64);
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let encoded = hex::encode_with_alphabet(&data, alphabet);
            assert_eq!(encoded.len(), len * 2);
            assert_eq!(hex::decode(&encoded).unwrap(), data, "len={len}");
        }
    }
}

#[test]
fn dispatch_handles_unaligned_input_and_output() {
    // Loads/stores are unaligned (vld1q_u8 / vld1_u8); exercise non-zero
    // offsets for both the input and the output so a kernel that assumed
    // alignment would fault or corrupt neighbouring bytes.
    for len in 0..=128usize {
        let data = pattern(len, 0xA11C ^ len as u64);
        for offset in 1..4usize {
            // --- encode: offset input and offset output ---
            let mut src_backing = vec![CANARY; offset + len + 2];
            src_backing[offset..offset + len].copy_from_slice(&data);
            let mut out_backing = vec![CANARY; offset + len * 2 + 3];
            let res = hex::encode_into(
                &mut out_backing[offset..offset + len * 2],
                &src_backing[offset..offset + len],
                Alphabet::Lower,
            );
            assert_eq!(res, Ok(()), "len={len} off={offset}");
            assert_eq!(&out_backing[offset..offset + len * 2], &ref_encode(&data, Alphabet::Lower)[..]);
            assert_eq!(&out_backing[..offset], &vec![CANARY; offset][..]);
            assert_eq!(&out_backing[offset + len * 2..], &[CANARY; 3][..]);
            // The input is never modified.
            assert_eq!(&src_backing[offset..offset + len], &data[..]);
            assert_eq!(&src_backing[..offset], &vec![CANARY; offset][..]);
            assert_eq!(&src_backing[offset + len..], &[CANARY; 2][..]);

            // --- decode: offset input and offset output ---
            let encoded = ref_encode(&data, Alphabet::Upper);
            let mut enc_backing = vec![CANARY; offset + encoded.len() + 2];
            enc_backing[offset..offset + encoded.len()].copy_from_slice(&encoded);
            let mut dec_backing = vec![CANARY; offset + len + 5];
            let res = hex::decode_into(
                &mut dec_backing[offset..offset + len],
                &enc_backing[offset..offset + encoded.len()],
            );
            assert_eq!(res, Ok(()), "len={len} off={offset}");
            assert_eq!(&dec_backing[offset..offset + len], &data[..]);
            assert_eq!(&dec_backing[..offset], &vec![CANARY; offset][..]);
            assert_eq!(&dec_backing[offset + len..], &[CANARY; 5][..]);
            assert_eq!(
                &enc_backing[offset..offset + encoded.len()],
                &encoded[..],
                "decoder modified its input"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Odd-length inputs.
// ---------------------------------------------------------------------------

#[test]
fn odd_length_rejected_every_length() {
    for len in (1..=129usize).step_by(2) {
        let input = vec![b'0'; len];
        // Scalar.
        let (out, res) = with_canary(len / 2, |out| hex::decode_into_constant_time(out, &input));
        assert_eq!(res, Err(DecodeError::InvalidInputLength), "len={len}");
        assert!(out.iter().all(|&b| b == CANARY), "scalar wrote output on a length error");
        // Dispatcher.
        let (out, res) = with_canary(len / 2, |out| hex::decode_into(out, &input));
        assert_eq!(res, Err(DecodeError::InvalidInputLength), "len={len}");
        assert!(out.iter().all(|&b| b == CANARY), "dispatch wrote output on a length error");
        // `decode` convenience API.
        assert_eq!(hex::decode(&input), Err(DecodeError::InvalidInputLength));
        // Odd length takes precedence over a wrong output length.
        let mut wrong = [0u8; 3];
        assert_eq!(hex::decode_into(&mut wrong, &input), Err(DecodeError::InvalidInputLength));
    }
}

// ---------------------------------------------------------------------------
// 5. Output length exactness for `decode_into` / `decode_array`.
// ---------------------------------------------------------------------------

#[test]
fn decode_output_length_must_be_exact() {
    for in_len in (0..=64usize).step_by(2) {
        let input = vec![b'0'; in_len];
        for out_len in 0..=34usize {
            if out_len == in_len / 2 {
                continue;
            }
            // Scalar.
            let (out, res) = with_canary(out_len, |out| hex::decode_into_constant_time(out, &input));
            assert_eq!(
                res,
                Err(DecodeError::InvalidOutputLength),
                "scalar in_len={in_len} out_len={out_len}"
            );
            assert!(out.iter().all(|&b| b == CANARY), "scalar wrote on output-length error");
            // Dispatcher.
            let (out, res) = with_canary(out_len, |out| hex::decode_into(out, &input));
            assert_eq!(
                res,
                Err(DecodeError::InvalidOutputLength),
                "dispatch in_len={in_len} out_len={out_len}"
            );
            assert!(out.iter().all(|&b| b == CANARY), "dispatch wrote on output-length error");
        }
    }
}

#[test]
fn decode_array_wrong_out_parameter() {
    macro_rules! check_case {
        ($n:literal, $input:expr, $want:expr) => {{
            let res: Result<[u8; $n], DecodeError> = hex::decode_array::<$n>($input);
            assert_eq!(res, $want, "decode_array::<{}>({:?})", $n, $input);
        }};
    }

    // Correct OUT.
    check_case!(0, b"", Ok([]));
    check_case!(1, b"00", Ok([0x00]));
    check_case!(2, b"00ff", Ok([0x00, 0xff]));
    check_case!(4, b"deadbeef", Ok([0xDE, 0xAD, 0xBE, 0xEF]));
    // OUT too small and too large, including for empty input.
    check_case!(1, b"", Err(DecodeError::InvalidOutputLength));
    check_case!(0, b"00", Err(DecodeError::InvalidOutputLength));
    check_case!(1, b"0000", Err(DecodeError::InvalidOutputLength));
    check_case!(3, b"0000", Err(DecodeError::InvalidOutputLength));
    check_case!(2, b"00", Err(DecodeError::InvalidOutputLength));
    check_case!(33, b"0000", Err(DecodeError::InvalidOutputLength));
    // Odd length wins over an otherwise-correct OUT.
    check_case!(0, b"0", Err(DecodeError::InvalidInputLength));
    check_case!(1, b"abc", Err(DecodeError::InvalidInputLength));
    // Invalid characters.
    check_case!(1, b"0g", Err(DecodeError::InvalidInput));
    check_case!(1, b"@0", Err(DecodeError::InvalidInput));
    check_case!(1, b"`0", Err(DecodeError::InvalidInput));
    // Const-evaluated cases (this is the documented use case).
    const OK: Result<[u8; 4], DecodeError> = hex::decode_array::<4>(b"DEADBEEF");
    const BAD_LEN: Result<[u8; 0], DecodeError> = hex::decode_array::<0>(b"0");
    const BAD_OUT: Result<[u8; 2], DecodeError> = hex::decode_array::<2>(b"00");
    const BAD_CH: Result<[u8; 1], DecodeError> = hex::decode_array::<1>(b"zz");
    assert_eq!(OK, Ok([0xDE, 0xAD, 0xBE, 0xEF]));
    assert_eq!(BAD_LEN, Err(DecodeError::InvalidInputLength));
    assert_eq!(BAD_OUT, Err(DecodeError::InvalidOutputLength));
    assert_eq!(BAD_CH, Err(DecodeError::InvalidInput));
}

// ---------------------------------------------------------------------------
// 6. Output length exactness for `encode_into` / `encode_array`.
// ---------------------------------------------------------------------------

#[test]
fn encode_output_length_must_be_exact() {
    for in_len in 0..=64usize {
        let data = vec![0x5Au8; in_len];
        for out_len in 0..=131usize {
            if out_len == in_len * 2 {
                continue;
            }
            let (out, res) = with_canary(out_len, |out| hex::encode_into_constant_time(out, &data, Alphabet::Lower));
            assert_eq!(
                res,
                Err(EncodeError::InvalidOutputLength),
                "scalar in_len={in_len} out_len={out_len}"
            );
            assert!(out.iter().all(|&b| b == CANARY), "scalar encode wrote on error");
            let (out, res) = with_canary(out_len, |out| hex::encode_into(out, &data, Alphabet::Lower));
            assert_eq!(
                res,
                Err(EncodeError::InvalidOutputLength),
                "dispatch in_len={in_len} out_len={out_len}"
            );
            assert!(out.iter().all(|&b| b == CANARY), "dispatch encode wrote on error");
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn encode_array_wrong_out_parameter_panics() {
    macro_rules! panics {
        ($n:literal, $input:expr) => {{
            let result = std::panic::catch_unwind(|| hex::encode_array::<$n>($input, Alphabet::Lower));
            assert!(result.is_err(), "encode_array::<{}>({:?}) did not panic", $n, $input);
        }};
    }
    // Correct OUT does not panic.
    let ok = std::panic::catch_unwind(|| hex::encode_array::<2>(b"\xAB", Alphabet::Lower));
    assert_eq!(ok.unwrap(), *b"ab");
    // Wrong OUT panics, both directions and for empty input.
    panics!(0, b"\x00");
    panics!(1, b"\x00");
    panics!(3, b"\x00");
    panics!(1, b"");
    panics!(1, b"ab");
    panics!(8, b"hello");
}

// ---------------------------------------------------------------------------
// 7. Canaries around the exact output region on success and on error.
// ---------------------------------------------------------------------------

#[test]
fn decode_success_writes_exactly_output_region() {
    // The constant-time decoder is documented to write `output` in full, so
    // check it writes *exactly* `output.len()` bytes and nothing beyond, for
    // inputs that cross every internal block boundary.
    for len in 0..=200usize {
        let input: Vec<u8> = (0..len * 2).map(|i| LOWER[i % 16]).collect();
        with_canary(len, |out| {
            hex::decode_into_constant_time(out, &input).unwrap();
        });
        with_canary(len, |out| {
            hex::decode_into(out, &input).unwrap();
        });
    }
}

#[test]
fn decode_error_constant_time_writes_only_within_output_region() {
    // Documented: the constant-time decoder intentionally writes `output`
    // even on error, but never beyond `input.len() / 2`.
    for len in [4usize, 32, 34, 64, 66, 128, 130] {
        for bad_pos in [0usize, 1, len / 2 - 1, len / 2, len - 2, len - 1] {
            let mut input: Vec<u8> = (0..len).map(|i| LOWER[i % 16]).collect();
            input[bad_pos] = b'g';
            let (out, res) = with_canary(len / 2, |out| hex::decode_into_constant_time(out, &input));
            assert_eq!(res, Err(DecodeError::InvalidInput), "len={len} pos={bad_pos}");
            // The whole output region is written even on error (this is the
            // constant-time contract), and it must not be all canary.
            assert!(
                out.iter().any(|&b| b != CANARY),
                "constant-time decoder did not write output on error, len={len} pos={bad_pos}"
            );
        }
        // The dispatcher must also not write out of bounds on error.
        for bad_pos in [0usize, len - 1] {
            let mut input: Vec<u8> = (0..len).map(|i| LOWER[i % 16]).collect();
            input[bad_pos] = b'g';
            with_canary(len / 2, |out| {
                assert_eq!(
                    hex::decode_into(out, &input),
                    Err(DecodeError::InvalidInput),
                    "dispatch len={len} pos={bad_pos}"
                );
            });
        }
    }
}

#[test]
fn encode_writes_exactly_output_region() {
    for len in 0..=200usize {
        let data = pattern(len, 0xBEEF ^ len as u64);
        with_canary(len * 2, |out| {
            hex::encode_into_constant_time(out, &data, Alphabet::Upper).unwrap();
        });
        with_canary(len * 2, |out| {
            hex::encode_into(out, &data, Alphabet::Upper).unwrap();
        });
    }
}

// ---------------------------------------------------------------------------
// 8. `String::from_utf8_unchecked` preconditions.
// ---------------------------------------------------------------------------

#[test]
fn encoded_output_is_always_ascii_utf8() {
    // The unchecked conversions in `encode_with_alphabet` and
    // `encode_into_string` rely on the output being pure ASCII hex.
    for len in 0..=300usize {
        let data = pattern(len, 0xFACE ^ len as u64);
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let s = hex::encode_with_alphabet(&data, alphabet);
            let bytes = s.as_bytes();
            assert!(bytes.iter().all(u8::is_ascii), "non-ASCII output at len={len}");
            assert!(core::str::from_utf8(bytes).is_ok());
            assert_eq!(bytes, &ref_encode(&data, alphabet)[..]);

            let mut appended = String::from("prefix:");
            hex::encode_into_string(&mut appended, &data, alphabet);
            assert!(appended.bytes().all(|b| b.is_ascii()));
            assert_eq!(
                appended.as_bytes(),
                &[b"prefix:".as_slice(), &ref_encode(&data, alphabet)[..]].concat()[..]
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 9. `encode_into_string` stack/alloc boundary.
// ---------------------------------------------------------------------------

#[test]
fn encode_into_string_boundary() {
    // `encoded_length <= 256` uses a stack buffer; 258 and above heap.
    for len in [0usize, 1, 127, 128, 129, 130, 255, 256, 1024, 4096] {
        let data = pattern(len, 0x2468 ^ len as u64);
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let mut s = String::new();
            hex::encode_into_string(&mut s, &data, alphabet);
            assert_eq!(s.as_bytes(), &ref_encode(&data, alphabet)[..], "len={len}");
        }
    }
    // Appending to a non-empty string keeps the pre-existing prefix intact.
    let mut s = String::from("keep");
    hex::encode_into_string(&mut s, b"", Alphabet::Lower);
    assert_eq!(s, "keep");
    for chunk in [&b"\x00"[..], &b"\xff\x00"[..], &[0x11; 200][..]] {
        let before = s.clone();
        hex::encode_into_string(&mut s, chunk, Alphabet::Upper);
        let mut expected = before.into_bytes();
        expected.extend_from_slice(&ref_encode(chunk, Alphabet::Upper));
        assert_eq!(s.as_bytes(), &expected[..]);
    }
}

// ---------------------------------------------------------------------------
// 10. const APIs and array encode/decode.
// ---------------------------------------------------------------------------

#[test]
fn encode_array_covers_all_bytes_both_alphabets() {
    for b in 0..=255u8 {
        let lower: [u8; 2] = hex::encode_array::<2>(&[b], Alphabet::Lower);
        let upper: [u8; 2] = hex::encode_array::<2>(&[b], Alphabet::Upper);
        assert_eq!(&lower[..], &ref_encode(&[b], Alphabet::Lower)[..]);
        assert_eq!(&upper[..], &ref_encode(&[b], Alphabet::Upper)[..]);
    }
    const EMPTY: [u8; 0] = hex::encode_array::<0>(b"", Alphabet::Lower);
    const TWO: [u8; 4] = hex::encode_array::<4>(b"\xde\xad", Alphabet::Upper);
    assert_eq!(EMPTY.len(), 0);
    assert_eq!(&TWO, b"DEAD");
}

#[test]
fn const_and_runtime_agree_all_small_lengths() {
    // Round-trip through the const array APIs for every small length.
    macro_rules! case {
        ($n:literal) => {{
            let data = pattern($n, $n as u64);
            let encoded: [u8; $n * 2] = hex::encode_array::<{ $n * 2 }>(&data, Alphabet::Lower);
            assert_eq!(&encoded[..], &ref_encode(&data, Alphabet::Lower)[..]);
            let decoded: [u8; $n] = hex::decode_array::<$n>(&encoded).unwrap();
            assert_eq!(&decoded[..], &data[..]);
        }};
    }
    case!(0);
    case!(1);
    case!(2);
    case!(3);
    case!(4);
    case!(8);
    case!(16);
    case!(32);
    case!(64);
}

// ---------------------------------------------------------------------------
// 11. Length arithmetic / overflow.
// ---------------------------------------------------------------------------

#[test]
fn length_arithmetic_cannot_overflow_for_real_slices() {
    // A slice's length is at most `isize::MAX`, so `len * 2` fits in `usize`
    // for every slice that can ever be passed to these APIs. This is the
    // invariant that keeps `check_encode_output_length`'s `data_length * 2`
    // (and the same expression in `encode_with_alphabet` /
    // `encode_into_string`) from overflowing under release-mode wrapping.
    let max_slice_len = isize::MAX as usize;
    assert_eq!(max_slice_len, usize::MAX / 2);
    assert!(max_slice_len.checked_mul(2).is_some());
    assert_eq!(max_slice_len * 2, usize::MAX - 1);
}

// ---------------------------------------------------------------------------
// 12. Misuse of the safe API that must stay memory-safe.
// ---------------------------------------------------------------------------

#[test]
fn empty_inputs_are_consistent() {
    let mut empty: [u8; 0] = [];
    assert_eq!(hex::encode_into(&mut empty, b"", Alphabet::Lower), Ok(()));
    assert_eq!(hex::encode_into_constant_time(&mut empty, b"", Alphabet::Upper), Ok(()));
    assert_eq!(hex::decode_into(&mut empty, b""), Ok(()));
    assert_eq!(hex::decode_into_constant_time(&mut empty, b""), Ok(()));
    assert_eq!(hex::encode(b""), "");
    assert_eq!(hex::decode(b"").unwrap(), b"");
    // Empty output with non-empty input is rejected, never written.
    assert_eq!(hex::decode_into(&mut empty, b"00"), Err(DecodeError::InvalidOutputLength));
    assert_eq!(
        hex::decode_into_constant_time(&mut empty, b"00"),
        Err(DecodeError::InvalidOutputLength)
    );
}

#[test]
fn miri_smoke() {
    // Scalar-only smoke test, cheap enough to run under Miri (which cannot
    // execute the NEON / AVX2 / wasm SIMD intrinsics, so it deliberately
    // avoids `encode_into` / `decode_into`). Covers both unrolled loops
    // (16-byte encode blocks, 32-byte decode blocks), the tails, the canary
    // helper and the `const` APIs.
    for len in 0..=40usize {
        let data = pattern(len, 0xE6 ^ len as u64);
        for alphabet in [Alphabet::Lower, Alphabet::Upper] {
            let (encoded, res) = with_canary(len * 2, |out| hex::encode_into_constant_time(out, &data, alphabet));
            assert_eq!(res, Ok(()));
            assert_eq!(encoded, ref_encode(&data, alphabet));
            let (decoded, res) = with_canary(len, |out| hex::decode_into_constant_time(out, &encoded));
            assert_eq!(res, Ok(()));
            assert_eq!(decoded, data);
        }
    }
    assert_eq!(hex::decode_array::<1>(b"0g"), Err(DecodeError::InvalidInput));
    assert_eq!(hex::encode_array::<2>(b"\xab", Alphabet::Upper), *b"AB");
}

#[test]
fn decode_output_may_be_larger_allocation_but_exact_slice() {
    // Callers commonly pass a sub-slice of a larger buffer; only the exact
    // slice length is written (checked above), and the tail stays intact.
    let input: Vec<u8> = (0..64).map(|i| LOWER[i % 16]).collect();
    let mut backing = [CANARY; 100];
    hex::decode_into(&mut backing[10..42], &input).unwrap();
    assert_eq!(&backing[10..42], &ref_decode(&input).unwrap()[..]);
    assert!(backing[..10].iter().all(|&b| b == CANARY));
    assert!(backing[42..].iter().all(|&b| b == CANARY));
}
