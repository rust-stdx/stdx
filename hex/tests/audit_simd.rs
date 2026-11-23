//! Tests for the hex SIMD kernels (NEON, AVX2, wasm SIMD128) reached
//! through the public dispatch in `hex::encode_into` / `hex::decode_into`.
//!
//! Covered here:
//!
//! * differential encode/decode against the constant-time path and an
//!   independent reference for every length 0..=512, both alphabets;
//! * invalid-input sweeps over every byte value at every position of a
//!   32/33/48/64-byte input, comparing the dispatched result to the scalar
//!   result and checking guard canaries around the output buffer;
//! * input slices at non-zero offsets and output buffers with canaries;
//! * output-length mismatches (both codecs require the exact length);
//! * mixed-case decoding.
//!
//! The test binary is built with whatever SIMD target features are enabled
//! for the target; on aarch64 the NEON dispatch is active by default, for
//! wasm32 pass `-C target-feature=+simd128`, for x86_64 AVX2 can be run
//! under emulation.

use hex::{Alphabet, DecodeError, EncodeError};
use rand::{RngExt, SeedableRng, rngs::StdRng};

const LOW: &[u8; 16] = b"0123456789abcdef";
const UPP: &[u8; 16] = b"0123456789ABCDEF";
const CANARY: u8 = 0xA5;
const PAD: usize = 32;

/// Independent, obviously-correct reference encoder.
fn ref_encode(data: &[u8], alphabet: Alphabet) -> Vec<u8> {
    let table = match alphabet {
        Alphabet::Lower => LOW,
        Alphabet::Upper => UPP,
    };
    let mut out = Vec::with_capacity(data.len() * 2);
    for &b in data {
        out.push(table[(b >> 4) as usize]);
        out.push(table[(b & 0x0f) as usize]);
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

/// Independent reference decoder mirroring the documented error precedence.
fn ref_decode(input: &[u8]) -> Result<Vec<u8>, DecodeError> {
    if !input.len().is_multiple_of(2) {
        return Err(DecodeError::InvalidInputLength);
    }
    let mut out = Vec::with_capacity(input.len() / 2);
    for pair in input.as_chunks::<2>().0 {
        match (ref_nibble(pair[0]), ref_nibble(pair[1])) {
            (Some(h), Some(l)) => out.push((h << 4) | l),
            _ => return Err(DecodeError::InvalidInput),
        }
    }
    Ok(out)
}

/// Runs `f` on a sub-slice of a padded buffer, then checks that bytes outside
/// the sub-slice were not touched.
fn guarded_out(out_len: usize, f: impl FnOnce(&mut [u8])) -> Vec<u8> {
    let mut buf = vec![CANARY; out_len + 2 * PAD];
    {
        let out = &mut buf[PAD..PAD + out_len];
        f(out);
    }
    assert!(buf[..PAD].iter().all(|&b| b == CANARY), "write before start of output buffer");
    assert!(
        buf[PAD + out_len..].iter().all(|&b| b == CANARY),
        "write after end of output buffer"
    );
    buf[PAD..PAD + out_len].to_vec()
}

const BOUNDARIES: &[usize] = &[0, 1, 2, 15, 16, 17, 31, 32, 33, 47, 48, 49, 63, 64, 65];

/// Deterministic pseudo-random byte stream (seeded `StdRng`).
fn test_data(max: usize) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(0x1234_5678_9abc_def0);
    (0..max).map(|_| rng.random::<u8>()).collect()
}

#[test]
fn encode_matches_reference_all_lengths() {
    let data = test_data(512);
    for len in 0..=512 {
        for &alphabet in &[Alphabet::Lower, Alphabet::Upper] {
            let src = &data[..len];
            let expected = ref_encode(src, alphabet);

            let dispatched = guarded_out(expected.len(), |out| {
                hex::encode_into(out, src, alphabet).unwrap();
            });
            assert_eq!(dispatched, expected, "dispatch len={len} {alphabet:?}");

            let scalar = guarded_out(expected.len(), |out| {
                hex::encode_into_constant_time(out, src, alphabet).unwrap();
            });
            assert_eq!(scalar, expected, "scalar len={len} {alphabet:?}");
        }
    }
    // Explicit threshold boundary names, so a failure points at the exact
    // kernel threshold rather than a sweep index.
    for &len in BOUNDARIES {
        let src = &data[..len];
        let mut out = vec![0u8; len * 2];
        hex::encode_into(&mut out, src, Alphabet::Lower).unwrap();
        assert_eq!(out, ref_encode(src, Alphabet::Lower), "boundary len={len}");
    }
}

#[test]
fn decode_matches_reference_all_lengths() {
    let data = test_data(512);
    for len in 0..=512 {
        let encoded = ref_encode(&data[..len], Alphabet::Lower);
        let expected = data[..len].to_vec();

        let dispatched = guarded_out(len, |out| {
            hex::decode_into(out, &encoded).unwrap();
        });
        assert_eq!(dispatched, expected, "dispatch len={len}");

        let scalar = guarded_out(len, |out| {
            hex::decode_into_constant_time(out, &encoded).unwrap();
        });
        assert_eq!(scalar, expected, "scalar len={len}");

        // Upper-case input decodes identically.
        let upper = ref_encode(&data[..len], Alphabet::Upper);
        let mut out = vec![0u8; len];
        hex::decode_into(&mut out, &upper).unwrap();
        assert_eq!(out, expected, "upper len={len}");
    }
    for &len in BOUNDARIES {
        let encoded = ref_encode(&data[..len], Alphabet::Upper);
        let mut out = vec![0u8; len];
        hex::decode_into(&mut out, &encoded).unwrap();
        assert_eq!(out, &data[..len], "boundary len={len}");
    }
}

#[test]
fn decode_mixed_case_matches_reference() {
    // Flip the case of a deterministic subset of characters.
    let data = test_data(300);
    let lower = ref_encode(&data, Alphabet::Lower);
    let mut mixed = lower.clone();
    for i in (0..mixed.len()).step_by(3) {
        mixed[i] = mixed[i].to_ascii_uppercase();
    }
    let mut out = vec![0u8; data.len()];
    hex::decode_into(&mut out, &mixed).unwrap();
    assert_eq!(out, data);
}

#[test]
fn encode_output_length_mismatch() {
    let data = test_data(80);
    for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 64] {
        let src = &data[..len];
        for delta in [-1isize, 1] {
            let out_len = (len as isize * 2 + delta).max(0) as usize;
            if out_len == len * 2 {
                continue;
            }
            let mut out = vec![0u8; out_len];
            let got = hex::encode_into(&mut out, src, Alphabet::Lower);
            let want = hex::encode_into_constant_time(&mut out, src, Alphabet::Lower);
            assert_eq!(got, Err(EncodeError::InvalidOutputLength), "len={len} out={out_len}");
            assert_eq!(got, want, "len={len}");
        }
    }
}

#[test]
fn decode_output_length_mismatch() {
    let data = test_data(80);
    for len in [0usize, 1, 15, 16, 17, 31, 32, 33, 64] {
        let encoded = ref_encode(&data[..len], Alphabet::Lower);
        for delta in [-1isize, 1] {
            let out_len = (len as isize + delta).max(0) as usize;
            if out_len == len {
                continue;
            }
            let mut out = vec![0u8; out_len];
            let got = hex::decode_into(&mut out, &encoded);
            let want = hex::decode_into_constant_time(&mut out, &encoded);
            assert_eq!(got, Err(DecodeError::InvalidOutputLength), "len={len} out={out_len}");
            assert_eq!(got, want, "len={len}");
        }
    }
}

#[test]
fn decode_odd_length_all_values() {
    // Odd lengths are rejected before any SIMD work regardless of content.
    for len in (1usize..=65).step_by(2) {
        for pos in [0, len / 2, len - 1] {
            for val in 0u16..=255 {
                let mut input = vec![b'a'; len];
                input[pos] = val as u8;
                let mut out = vec![0u8; len / 2];
                assert_eq!(
                    hex::decode_into(&mut out, &input),
                    Err(DecodeError::InvalidInputLength),
                    "len={len} pos={pos} val={val}"
                );
            }
        }
    }
}

/// The core invalid-input sweep: every byte value at every position of an
/// otherwise-valid input. Verifies the dispatched path agrees with both the
/// scalar path and the reference decoder, and that no bytes outside the
/// output slice are written.
#[test]
fn decode_invalid_every_position_every_value() {
    for len in [32usize, 33, 48, 64] {
        for pos in 0..len {
            for val in 0u16..=255 {
                let mut input = vec![b'f'; len];
                input[pos] = val as u8;

                let want = ref_decode(&input);
                let want_result = want.as_ref().map(|_| ()).map_err(|e| *e);

                let mut scalar_out = vec![0u8; len / 2];
                let scalar = hex::decode_into_constant_time(&mut scalar_out, &input);

                // Padded output buffer: catches OOB writes for this exact
                // combination, including the error paths.
                let mut dispatched = None;
                let dispatch_out = guarded_out(len / 2, |out| {
                    dispatched = Some(hex::decode_into(out, &input));
                });
                let dispatched = dispatched.unwrap();

                assert_eq!(dispatched, want_result, "result mismatch len={len} pos={pos} val={val:#04x}");
                assert_eq!(dispatched, scalar, "dispatch != scalar len={len} pos={pos} val={val:#04x}");
                if dispatched.is_ok() {
                    assert_eq!(dispatch_out, want.unwrap(), "content len={len} pos={pos} val={val:#04x}");
                }
            }
        }
    }
}

/// Same sweep, but the input slice starts at a non-zero offset inside a larger
/// allocation so an under/over-read of the load would be more visible to ASan-
/// style detectors and to the canaries below.
#[test]
fn decode_invalid_sweep_at_input_offset() {
    for len in [32usize, 48, 64] {
        for &prefix in &[1usize, 7, 15] {
            for pos in 0..len {
                for val in [0x00u8, 0x2f, 0x3a, 0x40, 0x47, 0x60, 0x67, 0x80, 0xff] {
                    let mut input = vec![CANARY; prefix + len + PAD];
                    input[prefix..prefix + len].fill(b'0');
                    input[prefix + pos] = val;
                    let slice = &input[prefix..prefix + len];

                    let want = ref_decode(slice).map(|_| ());
                    let mut out = vec![0u8; len / 2];
                    let got = hex::decode_into(&mut out, slice);
                    assert_eq!(got, want, "len={len} prefix={prefix} pos={pos} val={val:#04x}");
                }
            }
        }
    }
}

#[test]
fn encode_input_at_offset_matches_reference() {
    let data = test_data(300);
    for len in 0..=300 {
        for &prefix in &[1usize, 15, 16] {
            let mut backing = vec![CANARY; prefix + len + PAD];
            backing[prefix..prefix + len].copy_from_slice(&data[..len]);
            let src = &backing[prefix..prefix + len];
            let expected = ref_encode(src, Alphabet::Upper);
            let mut out = vec![0u8; len * 2];
            hex::encode_into(&mut out, src, Alphabet::Upper).unwrap();
            assert_eq!(out, expected, "len={len} prefix={prefix}");
        }
    }
}

/// Targeted regression for the wasm SIMD decoder: bytes outside `0-9A-Fa-f`
/// must be rejected, including bytes matching no class (which used to decode
/// as nibble 0) and the range boundaries `'@'` (0x40) / `` '`' `` (0x60).
#[test]
fn decode_rejects_invalid_boundary_characters() {
    for bad in [0x00u8, 0x2f, 0x40, 0x47, 0x60, 0x67, 0x80, 0xff] {
        let mut input = *b"00000000000000000000000000000000";
        input[5] = bad;
        let mut out = [0u8; 16];
        assert_eq!(
            hex::decode_into(&mut out, &input),
            Err(DecodeError::InvalidInput),
            "dispatched, bad={bad:#04x}"
        );
        assert_eq!(
            hex::decode_into_constant_time(&mut out, &input),
            Err(DecodeError::InvalidInput),
            "scalar, bad={bad:#04x}"
        );
    }
}

/// A single invalid byte in the tail (below the 32-byte SIMD block size) must
/// also be rejected: the tail is delegated to the constant-time path.
#[test]
fn decode_invalid_in_tail() {
    for len in [32usize, 34, 48, 50, 64, 66] {
        for pos in 0..len {
            let mut input = vec![b'a'; len];
            input[pos] = b'g';
            let mut out = vec![0u8; len / 2];
            assert_eq!(
                hex::decode_into(&mut out, &input),
                Err(DecodeError::InvalidInput),
                "len={len} pos={pos}"
            );
        }
    }
}
