//! Tests for the base64 NEON kernel and its dispatch.
//!
//! When the target enables NEON, [`base64::encode_into`] uses the NEON kernel
//! for inputs of at least 48 bytes and [`base64::decode_into`] uses it for
//! content of at least 32 bytes. Everything else falls back to the
//! constant-time scalar path, which is used here as the oracle. A second,
//! fully independent encoder (`ref_encode`) generates valid vectors and
//! expected encodings.
//!
//! Run with:
//!
//! ```text
//! cargo test -p base64 --test audit_neon
//! ```
#![allow(unsafe_op_in_unsafe_fn)]

use base64::Alphabet;
use random::{RngExt, SeedableRng, rngs::ChaChaRng};

const ALL: [Alphabet; 4] = [
    Alphabet::Standard,
    Alphabet::StandardNoPadding,
    Alphabet::Url,
    Alphabet::UrlNoPadding,
];

const STD: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

const CANARY: usize = 64;
const CANARY_BYTE: u8 = 0xC7;
const SENTINEL: u8 = 0x5A;

fn table(alphabet: Alphabet) -> &'static [u8; 64] {
    match alphabet {
        Alphabet::Standard | Alphabet::StandardNoPadding => STD,
        Alphabet::Url | Alphabet::UrlNoPadding => URL,
    }
}

fn padded(alphabet: Alphabet) -> bool {
    matches!(alphabet, Alphabet::Standard | Alphabet::Url)
}

/// Independent reference encoder (no shared code with the crate).
fn ref_encode(data: &[u8], alphabet: Alphabet) -> Vec<u8> {
    let t = table(alphabet);
    let pad = padded(alphabet);
    let mut out = Vec::with_capacity(data.len() / 3 * 4 + 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(t[(b0 >> 2) as usize]);
        out.push(t[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize]);
        if chunk.len() >= 2 {
            out.push(t[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize]);
        } else if pad {
            out.push(b'=');
        }
        if chunk.len() >= 3 {
            out.push(t[(b2 & 0x3F) as usize]);
        } else if pad {
            out.push(b'=');
        }
    }
    out
}

/// Deterministic seeded `ChaChaRng` byte stream. Fixed seed per call site.
fn pattern(len: usize, seed: u64) -> Vec<u8> {
    let mut rng = ChaChaRng::seed_from_u64(seed);
    let mut out = vec![0u8; len];
    rng.fill(&mut out);
    out
}

fn seed_for(alphabet: Alphabet, salt: u64) -> u64 {
    let a = match alphabet {
        Alphabet::Standard => 0x1111_1111_1111_1111,
        Alphabet::StandardNoPadding => 0x2222_2222_2222_2222,
        Alphabet::Url => 0x3333_3333_3333_3333,
        Alphabet::UrlNoPadding => 0x4444_4444_4444_4444,
    };
    a ^ salt
}

fn canary_buffer(payload_len: usize) -> Vec<u8> {
    vec![CANARY_BYTE; payload_len + 2 * CANARY]
}

fn payload(buf: &mut [u8]) -> &mut [u8] {
    let end = buf.len() - CANARY;
    &mut buf[CANARY..end]
}

fn check_canaries(buf: &[u8]) {
    assert_eq!(&buf[..CANARY], &[CANARY_BYTE; CANARY][..], "leading canary overwritten");
    assert_eq!(
        &buf[buf.len() - CANARY..],
        &[CANARY_BYTE; CANARY][..],
        "trailing canary overwritten"
    );
}

fn assert_same_decode(alphabet: Alphabet, input: &[u8], out_capacity: usize, context: &str) {
    let mut simd_out = vec![0u8; out_capacity];
    let mut ct_out = vec![0u8; out_capacity];
    let simd = base64::decode_into(&mut simd_out, input, alphabet);
    let ct = base64::decode_into_constant_time(&mut ct_out, input, alphabet);
    assert_eq!(
        simd, ct,
        "dispatch/scalar result divergence ({context}): alphabet={alphabet:?} input={input:?}"
    );
    if let Ok(n) = simd {
        assert!(n <= out_capacity);
        assert_eq!(
            &simd_out[..n],
            &ct_out[..n],
            "decoded bytes divergence ({context}): alphabet={alphabet:?} input={input:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Encoding
// ---------------------------------------------------------------------------

/// Every length 0..=400, every alphabet: dispatched encoder (NEON for len>=48)
/// must agree with the independent reference and with the constant-time path.
/// Output buffers are guarded by canaries.
#[test]
fn encode_differential_all_lengths_all_alphabets() {
    for alphabet in ALL {
        for len in 0..=400usize {
            let data = pattern(len, seed_for(alphabet, len as u64 + 1));
            let expected = ref_encode(&data, alphabet);
            let expected_len = base64::encoded_length(len, padded(alphabet)).unwrap();
            assert_eq!(expected.len(), expected_len, "reference length mismatch");

            let mut buf = canary_buffer(expected_len);
            for b in payload(&mut buf) {
                *b = SENTINEL;
            }
            let res = base64::encode_into(payload(&mut buf), &data, alphabet);
            assert!(res.is_ok(), "encode_into failed: alphabet={alphabet:?} len={len} -> {res:?}");
            assert_eq!(
                &payload(&mut buf)[..],
                &expected[..],
                "NEON encoding mismatch: alphabet={alphabet:?} len={len}"
            );
            check_canaries(&buf);

            let mut ct = vec![0u8; expected_len];
            let ct_res = base64::encode_into_constant_time(&mut ct, &data, alphabet);
            assert!(ct_res.is_ok());
            assert_eq!(ct, expected, "constant-time encoding mismatch: alphabet={alphabet:?} len={len}");
        }
    }
}

/// NEON-block boundary lengths with high-entropy and degenerate data.
#[test]
fn encode_boundary_lengths_fixed_patterns() {
    let lengths = [
        0usize, 1, 2, 3, 4, 15, 16, 17, 23, 24, 25, 31, 32, 33, 46, 47, 48, 49, 50, 62, 63, 64, 65, 66, 94, 95, 96, 97,
        98, 126, 127, 128, 129, 143, 144, 145, 191, 192, 193, 255, 256, 257, 383, 384, 385, 399, 400,
    ];
    for alphabet in ALL {
        for &len in &lengths {
            for (tag, data) in [
                ("zeros", vec![0u8; len]),
                ("ones", vec![0xFFu8; len]),
                ("counter", (0..len).map(|i| (i * 37 + 11) as u8).collect()),
                ("pattern", pattern(len, seed_for(alphabet, len as u64))),
            ] {
                let expected = ref_encode(&data, alphabet);
                let mut out = vec![0u8; expected.len()];
                base64::encode_into(&mut out, &data, alphabet)
                    .unwrap_or_else(|e| panic!("encode failed: {alphabet:?} len={len} data={tag} err={e:?}"));
                assert_eq!(out, expected, "encoding mismatch: alphabet={alphabet:?} len={len} data={tag}");
            }
        }
    }
}

/// The input pointer must not matter: exercise slices at non-zero offsets in
/// a larger allocation, which also rules out any alignment assumption.
#[test]
fn encode_unaligned_input_slices() {
    let lengths = [
        47usize, 48, 49, 63, 64, 65, 95, 96, 97, 127, 128, 129, 191, 192, 193, 400,
    ];
    for alphabet in ALL {
        for &len in &lengths {
            for offset in [0usize, 1, 2, 3, 7, 15, 16, 17, 31] {
                let backing = pattern(len + offset + 8, seed_for(alphabet, offset as u64 + 7));
                let data = &backing[offset..offset + len];
                let expected = ref_encode(data, alphabet);
                let mut out = vec![0u8; expected.len()];
                base64::encode_into(&mut out, data, alphabet)
                    .unwrap_or_else(|e| panic!("offset={offset} len={len}: {e:?}"));
                assert_eq!(out, expected, "offset={offset} len={len} alphabet={alphabet:?}");
            }
        }
    }
}

/// `encode_into` requires an exactly sized output buffer. Verify the
/// off-by-one rejection on both sides of the expected length, including the
/// SIMD threshold.
#[test]
fn encode_rejects_wrong_output_length() {
    for alphabet in ALL {
        for len in 0..=400usize {
            let data = pattern(len, seed_for(alphabet, len as u64 + 99));
            let expected_len = base64::encoded_length(len, padded(alphabet)).unwrap();

            if expected_len > 0 {
                let mut too_small = vec![0u8; expected_len - 1];
                assert_eq!(
                    base64::encode_into(&mut too_small, &data, alphabet),
                    Err(base64::EncodeError::InvalidOutputLength),
                    "accepted short buffer: alphabet={alphabet:?} len={len}"
                );
            }
            let mut too_large = vec![0u8; expected_len + 1];
            assert_eq!(
                base64::encode_into(&mut too_large, &data, alphabet),
                Err(base64::EncodeError::InvalidOutputLength),
                "accepted oversized buffer: alphabet={alphabet:?} len={len}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Decoding
// ---------------------------------------------------------------------------

/// Every raw length 0..=400, every alphabet: decode of an independently
/// encoded vector must reproduce the raw bytes and never write outside the
/// exact output buffer (canaries).
#[test]
fn decode_differential_all_lengths_all_alphabets() {
    for alphabet in ALL {
        for raw_len in 0..=400usize {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 3));
            let encoded = ref_encode(&raw, alphabet);

            let mut buf = canary_buffer(raw_len);
            for b in payload(&mut buf) {
                *b = SENTINEL;
            }
            let res = base64::decode_into(payload(&mut buf), &encoded, alphabet);
            assert_eq!(
                res,
                Ok(raw_len),
                "decode_into: alphabet={alphabet:?} raw_len={raw_len} encoded={encoded:?}"
            );
            assert_eq!(
                &payload(&mut buf)[..],
                &raw[..],
                "decoded bytes: alphabet={alphabet:?} raw_len={raw_len}"
            );
            check_canaries(&buf);

            let mut ct = vec![0u8; raw_len];
            assert_eq!(base64::decode_into_constant_time(&mut ct, &encoded, alphabet), Ok(raw_len));
            assert_eq!(ct, raw, "constant-time decode: alphabet={alphabet:?} raw_len={raw_len}");
        }
    }
}

/// Output buffers larger than required are allowed for decoding. Only the
/// first `computed_output` bytes may be written.
#[test]
fn decode_oversized_output_leaves_tail_untouched() {
    let raw_lengths = [
        1usize, 2, 3, 23, 24, 25, 31, 32, 33, 46, 47, 48, 49, 63, 64, 65, 95, 96, 97, 127, 128, 129, 191, 192, 300, 400,
    ];
    for alphabet in ALL {
        for &raw_len in &raw_lengths {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 21));
            let encoded = ref_encode(&raw, alphabet);
            let extra = 73;
            let mut buf = canary_buffer(raw_len + extra);
            for b in payload(&mut buf) {
                *b = SENTINEL;
            }
            let res = base64::decode_into(payload(&mut buf), &encoded, alphabet);
            assert_eq!(res, Ok(raw_len));
            assert_eq!(&payload(&mut buf)[..raw_len], &raw[..]);
            assert!(
                payload(&mut buf)[raw_len..].iter().all(|&b| b == SENTINEL),
                "wrote past computed output: alphabet={alphabet:?} raw_len={raw_len}"
            );
            check_canaries(&buf);
        }
    }
}

/// An output buffer one byte too small must be rejected before any SIMD work.
#[test]
fn decode_rejects_short_output() {
    let raw_lengths = [1usize, 2, 3, 31, 32, 33, 47, 48, 49, 63, 64, 65, 95, 96, 97, 300, 400];
    for alphabet in ALL {
        for &raw_len in &raw_lengths {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 55));
            let encoded = ref_encode(&raw, alphabet);
            let mut buf = vec![0u8; raw_len - 1];
            assert_eq!(
                base64::decode_into(&mut buf, &encoded, alphabet),
                Err(base64::DecodeError::InvalidOutputLength),
                "accepted short output: alphabet={alphabet:?} raw_len={raw_len}"
            );
        }
    }
}

/// Input slices at non-zero offsets (alignment / over-read assumptions).
#[test]
fn decode_unaligned_input_slices() {
    let raw_lengths = [31usize, 32, 33, 47, 48, 49, 63, 64, 65, 95, 96, 97, 128, 300];
    for alphabet in ALL {
        for &raw_len in &raw_lengths {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 8));
            let encoded = ref_encode(&raw, alphabet);
            for offset in [0usize, 1, 2, 3, 7, 15, 16, 17, 31] {
                let mut backing = vec![0u8; encoded.len() + offset + 8];
                backing[offset..offset + encoded.len()].copy_from_slice(&encoded);
                let input = &backing[offset..offset + encoded.len()];
                let mut out = vec![0u8; raw_len];
                let n = base64::decode_into(&mut out, input, alphabet)
                    .unwrap_or_else(|e| panic!("offset={offset} raw_len={raw_len}: {e:?}"));
                assert_eq!(n, raw_len);
                assert_eq!(out, raw, "offset={offset} alphabet={alphabet:?}");
            }
        }
    }
}

/// Substitute every possible byte at every position of representative
/// encodings. The dispatched path must agree with the constant-time path on
/// acceptance and on decoded bytes: every invalid byte has to be rejected,
/// and no valid byte may be rejected.
#[test]
fn decode_every_byte_substitution_matches_scalar() {
    // Raw lengths chosen so the padded encodings straddle several 32-byte
    // chunks and leave non-empty tails:
    //   31 -> 44 chars, 34 -> 48 chars, 50 -> 68 chars, 100 -> 136 chars.
    for alphabet in ALL {
        for &raw_len in &[31usize, 34, 50, 100] {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 13));
            let encoded = ref_encode(&raw, alphabet);
            assert!(encoded.len() >= 32);
            let capacity = raw_len + 8;
            for pos in 0..encoded.len() {
                for value in 0u16..=255 {
                    let value = value as u8;
                    if value == encoded[pos] {
                        continue;
                    }
                    let mut mutated = encoded.clone();
                    mutated[pos] = value;
                    let context = format!("pos={pos} value=0x{value:02X} raw_len={raw_len}");
                    assert_same_decode(alphabet, &mutated, capacity, &context);
                }
            }
        }
    }
}

/// Every truncation of representative encodings, including lengths that end
/// inside a NEON chunk, exactly on a chunk boundary, and in the scalar tail.
#[test]
fn decode_truncations_match_scalar() {
    for alphabet in ALL {
        for &raw_len in &[0usize, 1, 2, 31, 32, 33, 47, 48, 49, 63, 64, 65, 95, 96, 97, 128, 300] {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 17));
            let encoded = ref_encode(&raw, alphabet);
            for n in 0..=encoded.len() {
                let capacity = raw_len + 8;
                assert_same_decode(
                    alphabet,
                    &encoded[..n],
                    capacity,
                    &format!("truncated to {n}/{} raw_len={raw_len}", encoded.len()),
                );
            }
        }
    }
}

/// Handcrafted malformed inputs (padding, length, internal `=`) must behave
/// identically on both paths, including the exact error variant.
#[test]
fn decode_malformed_inputs_match_scalar() {
    let cases: &[&[u8]] = &[
        b"",
        b"=",
        b"==",
        b"===",
        b"====",
        b"A",
        b"AA",
        b"AAA",
        b"AAAA",
        b"AAAA=",
        b"AA==",
        b"A===",
        b"A=A=",
        b"A=AA",
        b"AA=A",
        b"AAAA====",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA===",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA====",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=AAAA",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA+AAA",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA/AAA",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA-AAA",
        b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA_AAA",
    ];
    for alphabet in ALL {
        for &case in cases {
            assert_same_decode(alphabet, case, case.len() + 8, &format!("malformed {case:?}"));
        }
    }
}

/// Non-canonical trailing bits (unused low bits set) must be rejected by the
/// scalar tail, both for padded and unpadded alphabets, including when the
/// non-canonical character sits in the tail after a NEON chunk.
#[test]
fn decode_rejects_non_canonical_trailing_bits() {
    for alphabet in ALL {
        let pad = padded(alphabet);
        // raw_len % 3 == 1 -> 4 unused bits in the last content sextet.
        // raw_len % 3 == 2 -> 2 unused bits in the last content sextet.
        for &raw_len in &[1usize, 2, 4, 5, 7, 8, 34, 35, 37, 38, 100, 101] {
            let rem = raw_len % 3;
            assert!(rem == 1 || rem == 2);
            let mut raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 31));
            // Force the unused bits of the final sextet to zero so the last
            // content character is 'A'.
            if rem == 1 {
                raw[raw_len - 1] &= 0xFC;
            } else {
                raw[raw_len - 1] &= 0xF0;
            }
            let encoded = ref_encode(&raw, alphabet);
            let content_len = encoded.iter().rposition(|&b| b != b'=').map_or(0, |i| i + 1);
            assert_eq!(encoded[content_len - 1], b'A', "setup failed");

            // Canonical version decodes.
            assert_same_decode(alphabet, &encoded, raw_len, "canonical baseline");
            let mut decoded = vec![0u8; raw_len];
            assert_eq!(base64::decode_into(&mut decoded, &encoded, alphabet), Ok(raw_len));
            assert_eq!(decoded, raw, "canonical decode mismatch");

            // 'A' -> 'B' sets the lowest unused bit.
            let mut mutated = encoded.clone();
            mutated[content_len - 1] = b'B';
            let mut out = vec![0u8; raw_len];
            assert_eq!(
                base64::decode_into(&mut out, &mutated, alphabet),
                Err(base64::DecodeError::InvalidInput),
                "non-canonical trailing bits accepted: alphabet={alphabet:?} raw_len={raw_len} pad={pad}"
            );
            assert_same_decode(alphabet, &mutated, raw_len, "non-canonical trailing bits");
        }
    }
}

/// Regression test for the URL-alphabet separator bug: the NEON chunk path
/// used to accept `+` and `/` for the URL alphabet (and the scalar path does
/// not). Check every separator at many positions in a multi-chunk input.
#[test]
fn url_alphabet_separators_are_alphabet_specific() {
    let raw = pattern(96, 0xA5A5_1234);
    for alphabet in [Alphabet::Url, Alphabet::UrlNoPadding] {
        let encoded = ref_encode(&raw, alphabet); // 128 content chars
        assert_eq!(encoded.len(), 128);
        for pos in [0usize, 1, 30, 31, 32, 33, 62, 63, 64, 94, 126, 127] {
            for (ch, allowed) in [(b'+', false), (b'/', false), (b'-', true), (b'_', true)] {
                let mut mutated = encoded.clone();
                mutated[pos] = ch;
                let mut out = vec![0u8; raw.len() + 4];
                let res = base64::decode_into(&mut out, &mutated, alphabet);
                if allowed {
                    assert!(res.is_ok(), "rejected valid URL char 0x{ch:02X} at {pos}");
                    assert_same_decode(alphabet, &mutated, raw.len(), &format!("valid URL char 0x{ch:02X} at {pos}"));
                } else {
                    assert_eq!(
                        res,
                        Err(base64::DecodeError::InvalidInput),
                        "URL alphabet accepted standard-only char 0x{ch:02X} at {pos}"
                    );
                }
            }
        }
    }

    // And the standard alphabet must reject the URL-only separators, both in
    // chunks and in the tail.
    for alphabet in [Alphabet::Standard, Alphabet::StandardNoPadding] {
        let encoded = ref_encode(&pattern(96, 0x5A5A_9999), alphabet);
        for pos in [0usize, 31, 32, 63, 64, 126, 127] {
            for ch in *b"-_" {
                let mut mutated = encoded.clone();
                mutated[pos] = ch;
                let mut out = vec![0u8; 100];
                assert_eq!(
                    base64::decode_into(&mut out, &mutated, alphabet),
                    Err(base64::DecodeError::InvalidInput),
                    "standard alphabet accepted 0x{ch:02X} at {pos}"
                );
            }
        }
    }
}

/// `+`/`/` must remain valid for the standard alphabet in the chunk path.
#[test]
fn standard_alphabet_separators_accepted_in_chunks() {
    // Encode 96 raw bytes such that the value 62 and 63 appear, then swap the
    // characters that carry them for '+' and '/' where the alphabet allows.
    for alphabet in [Alphabet::Standard, Alphabet::StandardNoPadding] {
        let mut raw = pattern(96, 0xDEAD_BEEF);
        // Make the first two sextets 62 and 63: b0 = 0b111110_11 = 0xFB,
        // b1 high nibble 1111 -> values 62,63,63,63.
        raw[0] = 0xFB;
        raw[1] = 0xFF;
        let encoded = ref_encode(&raw, alphabet);
        assert_eq!(encoded[0], b'+');
        assert_eq!(encoded[1], b'/');
        let mut out = vec![0u8; raw.len()];
        assert_eq!(base64::decode_into(&mut out, &encoded, alphabet), Ok(raw.len()));
        assert_eq!(out, raw);
    }
}

// ---------------------------------------------------------------------------
// Page-guard checks (reads and writes must stop exactly at the slice end)
// ---------------------------------------------------------------------------

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mod page_guard {
    pub const PAGE: usize = 4096;

    /// Two anonymous pages; the second is `PROT_NONE`. Exposes a slice that
    /// ends exactly at the guard page, so any access one byte past the slice
    /// faults instead of silently reading/writing adjacent memory.
    pub struct Guarded {
        data: *mut u8,
        len: usize,
    }

    // The mapping is intentionally leaked for the lifetime of the test.
    unsafe fn sys_mmap(len: usize) -> *mut u8 {
        let ret: u64;
        core::arch::asm!(
            "svc 0",
            in("x8") 222u64,        // mmap
            inout("x0") 0u64 => ret,
            in("x1") len as u64,
            in("x2") 3u64,          // PROT_READ | PROT_WRITE
            in("x3") 0x22u64,       // MAP_PRIVATE | MAP_ANONYMOUS
            in("x4") (-1i64) as u64,
            in("x5") 0u64,
            options(nostack),
        );
        let signed = ret as i64;
        assert!(signed > 0, "mmap failed: {signed}");
        ret as *mut u8
    }

    unsafe fn sys_mprotect(addr: *mut u8, len: usize, prot: u64) {
        let ret: u64;
        core::arch::asm!(
            "svc 0",
            in("x8") 226u64,        // mprotect
            inout("x0") addr as u64 => ret,
            in("x1") len as u64,
            in("x2") prot,
            options(nostack),
        );
        assert_eq!(ret as i64, 0, "mprotect failed");
    }

    impl Guarded {
        pub fn new(len: usize) -> Self {
            assert!(len <= PAGE);
            // SAFETY: raw mmap/mprotect syscalls on freshly mapped anonymous
            // memory owned by this test.
            unsafe {
                let base = sys_mmap(2 * PAGE);
                sys_mprotect(base.add(PAGE), PAGE, 0); // PROT_NONE
                Self {
                    data: base.add(PAGE - len),
                    len,
                }
            }
        }

        pub fn as_slice(&self) -> &[u8] {
            // SAFETY: `data..data+len` is inside the writable first page.
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }

        pub fn as_mut_slice(&mut self) -> &mut [u8] {
            // SAFETY: as above; `&mut self` guarantees exclusivity.
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
#[test]
fn neon_input_reads_stop_at_slice_end() {
    use page_guard::Guarded;

    for alphabet in ALL {
        // Encoding: the NEON kernel does 48-byte loads.
        for &len in &[48usize, 49, 63, 64, 65, 95, 96, 97, 127, 128, 129, 191, 192, 193, 400] {
            let data = pattern(len, seed_for(alphabet, len as u64 + 401));
            let mut guard = Guarded::new(len);
            guard.as_mut_slice().copy_from_slice(&data);
            let expected = ref_encode(&data, alphabet);
            let mut out = vec![0u8; expected.len()];
            base64::encode_into(&mut out, guard.as_slice(), alphabet).unwrap();
            assert_eq!(out, expected, "guarded encode: alphabet={alphabet:?} len={len}");
        }

        // Decoding: the NEON kernel does 32-byte loads.
        for &raw_len in &[
            24usize, 25, 26, 31, 32, 33, 47, 48, 49, 63, 64, 65, 95, 96, 97, 128, 300,
        ] {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 402));
            let encoded = ref_encode(&raw, alphabet);
            if encoded.len() > page_guard::PAGE {
                continue;
            }
            let mut guard = Guarded::new(encoded.len());
            guard.as_mut_slice().copy_from_slice(&encoded);
            let mut out = vec![0u8; raw_len];
            let n = base64::decode_into(&mut out, guard.as_slice(), alphabet).unwrap();
            assert_eq!(n, raw_len);
            assert_eq!(out, raw, "guarded decode: alphabet={alphabet:?} raw_len={raw_len}");
        }
    }
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
#[test]
fn neon_output_writes_stop_at_slice_end() {
    use page_guard::Guarded;

    for alphabet in ALL {
        // Encoding writes 64-byte stores; the output ends at the guard page.
        for &len in &[
            48usize, 49, 50, 63, 64, 65, 95, 96, 97, 127, 128, 129, 191, 192, 193, 400,
        ] {
            let data = pattern(len, seed_for(alphabet, len as u64 + 403));
            let expected = ref_encode(&data, alphabet);
            let mut guard = Guarded::new(expected.len());
            base64::encode_into(guard.as_mut_slice(), &data, alphabet).unwrap();
            assert_eq!(
                guard.as_slice(),
                &expected[..],
                "guarded encode output: alphabet={alphabet:?} len={len}"
            );
        }

        // Decoding writes 24-byte stores with an exact-length output.
        for &raw_len in &[
            24usize, 25, 26, 31, 32, 33, 47, 48, 49, 63, 64, 65, 95, 96, 97, 128, 300,
        ] {
            let raw = pattern(raw_len, seed_for(alphabet, raw_len as u64 + 404));
            let encoded = ref_encode(&raw, alphabet);
            let mut guard = Guarded::new(raw_len);
            let n = base64::decode_into(guard.as_mut_slice(), &encoded, alphabet).unwrap();
            assert_eq!(n, raw_len);
            assert_eq!(
                guard.as_slice(),
                &raw[..],
                "guarded decode output: alphabet={alphabet:?} raw_len={raw_len}"
            );
        }
    }
}
