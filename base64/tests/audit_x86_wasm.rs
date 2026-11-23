//! Tests for the base64 AVX2 and wasm SIMD kernels and their dispatch.
//!
//! The file is written so that it compiles on any target architecture: backend-specific
//! tests are behind `cfg(target_feature)` gates, everything else compares the dispatched
//! implementation against the crate's own scalar constant-time implementation.

#![allow(clippy::needless_range_loop)]

use base64::{
    Alphabet, decode_into, decode_into_constant_time, encode_into, encode_into_constant_time, encoded_length,
};
use rand::{Rng, RngExt, SeedableRng, rngs::StdRng};

const ALPHABETS: [Alphabet; 4] = [
    Alphabet::Standard,
    Alphabet::StandardNoPadding,
    Alphabet::Url,
    Alphabet::UrlNoPadding,
];

/// True when the dispatch in `base64.rs` selects one of the SIMD kernels
/// (AVX2 on x86/x86_64, wasm SIMD on wasm32). Used to gate tests that assert
/// kernel-specific strictness rather than cross-backend agreement.
const SIMD_BACKEND: bool = cfg!(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"),
    all(target_arch = "wasm32", target_feature = "simd128"),
));

fn is_padded(alphabet: Alphabet) -> bool {
    matches!(alphabet, Alphabet::Standard | Alphabet::Url)
}

/// Deterministic seeded `StdRng` so failures are reproducible.
fn fill_random(buf: &mut [u8], seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.fill(buf);
}

fn scalar_encode(data: &[u8], alphabet: Alphabet) -> Vec<u8> {
    let len = encoded_length(data.len(), is_padded(alphabet)).expect("encoded length overflow");
    let mut out = vec![0u8; len];
    encode_into_constant_time(&mut out, data, alphabet).expect("scalar encode");
    out
}

/// Encoding must not write outside the exact expected output length.
#[test]
fn encode_dispatch_respects_output_canaries_and_matches_scalar() {
    const CANARY: usize = 16;
    let mut seed = 0x5eed_0001_u64;
    let mut data = vec![0u8; 400];

    for len in 0..=400 {
        fill_random(&mut data[..len], seed ^ len as u64);
        for alphabet in ALPHABETS {
            let expected = scalar_encode(&data[..len], alphabet);
            let mut buf = vec![0xAAu8; expected.len() + 2 * CANARY];
            encode_into(&mut buf[CANARY..CANARY + expected.len()], &data[..len], alphabet)
                .unwrap_or_else(|e| panic!("encode len={len} alphabet={alphabet:?}: {e:?}"));
            assert_eq!(
                &buf[CANARY..CANARY + expected.len()],
                &expected[..],
                "encode mismatch len={len} alphabet={alphabet:?}"
            );
            assert!(
                buf[..CANARY].iter().all(|&b| b == 0xAA),
                "encode wrote before output len={len} alphabet={alphabet:?}"
            );
            assert!(
                buf[CANARY + expected.len()..].iter().all(|&b| b == 0xAA),
                "encode wrote after output len={len} alphabet={alphabet:?}"
            );
        }
        seed = StdRng::seed_from_u64(seed).next_u64();
    }
}

/// Decoding of valid data must match the scalar path byte-for-byte and must not write
/// outside the reported decoded length.
#[test]
fn decode_dispatch_respects_output_canaries_and_matches_scalar() {
    const CANARY: usize = 16;
    let mut seed = 0x5eed_0002_u64;
    let mut data = vec![0u8; 400];

    for len in 0..=400 {
        fill_random(&mut data[..len], seed ^ (len as u64).wrapping_mul(3));
        for alphabet in ALPHABETS {
            let encoded = scalar_encode(&data[..len], alphabet);

            let mut buf = vec![0x55u8; len + 2 * CANARY];
            let n = decode_into(&mut buf[CANARY..CANARY + len], &encoded, alphabet)
                .unwrap_or_else(|e| panic!("decode len={len} alphabet={alphabet:?}: {e:?}"));
            assert_eq!(n, len, "decode length len={len} alphabet={alphabet:?}");
            assert_eq!(
                &buf[CANARY..CANARY + len],
                &data[..len],
                "decode mismatch len={len} alphabet={alphabet:?}"
            );
            assert!(
                buf[..CANARY].iter().all(|&b| b == 0x55),
                "decode wrote before output len={len} alphabet={alphabet:?}"
            );
            assert!(
                buf[CANARY + len..].iter().all(|&b| b == 0x55),
                "decode wrote after output len={len} alphabet={alphabet:?}"
            );

            let mut scalar_out = vec![0u8; len];
            let scalar_n = decode_into_constant_time(&mut scalar_out, &encoded, alphabet).unwrap();
            assert_eq!(scalar_n, n);
            assert_eq!(&scalar_out[..], &buf[CANARY..CANARY + len]);
        }
        seed = StdRng::seed_from_u64(seed).next_u64();
    }
}

/// Exhaustively substitute every byte value at every position of representative
/// valid inputs and require the dispatched decoder to agree with the scalar decoder
/// (both on the error and, when both accept, on the decoded bytes).
#[test]
fn decode_every_byte_value_at_every_position_matches_scalar() {
    if !SIMD_BACKEND {
        // The AVX2/wasm kernels are not compiled in; the dispatch is the scalar path
        // and this exhaustive sweep would only compare the scalar path with itself.
        return;
    }

    // All lengths are multiples of 4 so the scalar path has no partial-quad tail
    // (the SIMD kernels process 32 bytes for AVX2 and 16 for wasm SIMD).
    for &len in &[16usize, 32, 64, 96] {
        for alphabet in ALPHABETS {
            let base = vec![b'A'; len];

            let mut out_dispatch = vec![0u8; len];
            let mut out_scalar = vec![0u8; len];
            let mut input = base.clone();

            for pos in 0..len {
                for value in 0..=255u8 {
                    input.copy_from_slice(&base);
                    input[pos] = value;

                    out_dispatch.fill(0);
                    out_scalar.fill(0);

                    let dispatched = decode_into(&mut out_dispatch, &input, alphabet);
                    let scalar = decode_into_constant_time(&mut out_scalar, &input, alphabet);

                    match (dispatched, scalar) {
                        (Ok(dn), Ok(sn)) => {
                            assert_eq!(
                                dn, sn,
                                "decoded length mismatch len={len} alphabet={alphabet:?} pos={pos} value={value:#04x}"
                            );
                            assert_eq!(
                                &out_dispatch[..dn],
                                &out_scalar[..sn],
                                "decoded bytes mismatch len={len} alphabet={alphabet:?} pos={pos} value={value:#04x}"
                            );
                        }
                        (Err(de), Err(se)) => {
                            assert_eq!(
                                de, se,
                                "error mismatch len={len} alphabet={alphabet:?} pos={pos} value={value:#04x}"
                            );
                        }
                        (d, s) => panic!(
                            "dispatch/scalar disagreement len={len} alphabet={alphabet:?} pos={pos} \
                             value={value:#04x}: dispatched={d:?} scalar={s:?}"
                        ),
                    }
                }
            }
        }
    }
}

/// Deterministic random (mostly invalid) inputs: dispatched decoding must agree with
/// the scalar decoder, including on which inputs are rejected.
#[test]
fn decode_random_inputs_match_scalar() {
    if !SIMD_BACKEND {
        return;
    }

    let mut seed = 0x5eed_0003_u64;
    for len in 12..=200usize {
        let mut input = vec![0u8; len];
        fill_random(&mut input, seed ^ len as u64);

        for alphabet in ALPHABETS {
            let mut out_dispatch = vec![0u8; len];
            let mut out_scalar = vec![0u8; len];
            let dispatched = decode_into(&mut out_dispatch, &input, alphabet);
            let scalar = decode_into_constant_time(&mut out_scalar, &input, alphabet);
            match (dispatched, scalar) {
                (Ok(dn), Ok(sn)) => {
                    assert_eq!(dn, sn, "random len={len} alphabet={alphabet:?} input={input:02x?}");
                    assert_eq!(&out_dispatch[..dn], &out_scalar[..sn]);
                }
                (Err(de), Err(se)) => assert_eq!(
                    de, se,
                    "random error mismatch len={len} alphabet={alphabet:?} input={input:02x?}"
                ),
                (d, s) => panic!(
                    "random disagreement len={len} alphabet={alphabet:?} input={input:02x?}: \
                     dispatched={d:?} scalar={s:?}"
                ),
            }
        }
        seed = StdRng::seed_from_u64(seed).next_u64();
    }
}

/// The standard alphabet must reject `-`/`_` and the URL alphabet must reject `+`/`/`.
/// This is an alphabet-strictness requirement: the SIMD kernels must not accept
/// characters that belong to the other alphabet.
#[test]
fn decode_rejects_chars_from_the_other_alphabet() {
    if !SIMD_BACKEND {
        return;
    }

    // Lengths large enough to enter the SIMD loop for the compiled backend.
    for &len in &[16usize, 32, 64, 96] {
        for pos in [0usize, 1, 2, 3, len / 2, len - 1] {
            for (alphabet, ch) in [
                (Alphabet::Url, b'+'),
                (Alphabet::UrlNoPadding, b'+'),
                (Alphabet::Url, b'/'),
                (Alphabet::UrlNoPadding, b'/'),
                (Alphabet::Standard, b'-'),
                (Alphabet::StandardNoPadding, b'-'),
                (Alphabet::Standard, b'_'),
                (Alphabet::StandardNoPadding, b'_'),
            ] {
                let mut input = vec![b'A'; len];
                input[pos] = ch;
                let mut out = vec![0u8; len];
                let dispatched = decode_into(&mut out, &input, alphabet);
                let scalar = decode_into_constant_time(&mut out, &input, alphabet);
                assert_eq!(
                    dispatched, scalar,
                    "alphabet {alphabet:?} accepted foreign char {:?} at pos {pos} of {len}",
                    ch as char
                );
                assert_eq!(
                    dispatched,
                    Err(base64::DecodeError::InvalidInput),
                    "alphabet {alphabet:?} must reject foreign char {:?} at pos {pos} of {len}",
                    ch as char
                );
            }
        }
    }
}

/// Output buffers that are exactly as small as the API allows must be accepted, and
/// buffers that are one byte too small must be rejected before any write happens.
#[test]
fn output_length_boundaries_are_checked() {
    let mut seed = 0x5eed_0004_u64;
    for len in 0..=120usize {
        let mut data = vec![0u8; len];
        fill_random(&mut data, seed ^ len as u64);
        for alphabet in ALPHABETS {
            let expected = scalar_encode(&data, alphabet);
            let mut exact = vec![0u8; expected.len()];
            encode_into(&mut exact, &data, alphabet).unwrap();
            assert_eq!(exact, expected, "encode len={len} alphabet={alphabet:?}");

            if !expected.is_empty() {
                let mut short = vec![0xCCu8; expected.len() - 1];
                assert_eq!(
                    encode_into(&mut short, &data, alphabet),
                    Err(base64::EncodeError::InvalidOutputLength),
                    "short encode buffer len={len} alphabet={alphabet:?}"
                );
                assert!(short.iter().all(|&b| b == 0xCC), "short encode buffer was written");
            }

            // Decode: one byte too small must be rejected without writing.
            let mut exact_decoded = vec![0u8; len];
            assert_eq!(decode_into(&mut exact_decoded, &expected, alphabet), Ok(len));
            if len > 0 {
                let mut short_decoded = vec![0xCCu8; len - 1];
                assert_eq!(
                    decode_into(&mut short_decoded, &expected, alphabet),
                    Err(base64::DecodeError::InvalidOutputLength),
                    "short decode buffer len={len} alphabet={alphabet:?}"
                );
                assert!(short_decoded.iter().all(|&b| b == 0xCC), "short decode buffer was written");
            }
        }
        seed = StdRng::seed_from_u64(seed).next_u64();
    }
}

// ======================================================================================
// wasm linear-memory boundary tests: placing the input flush against the end of the
// wasm linear memory makes any 16-byte `v128_load` that reads past the slice trap with
// "memory access out of bounds", giving the same signal as a guard page.
// ======================================================================================
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod wasm_memory_boundary {
    use core::arch::wasm32::{memory_grow, memory_size};

    use super::*;

    /// Inputs smaller than 16 bytes must not be passed to `v128_load`. The kernel used
    /// to enter its vector loop for 12..=15 byte inputs, which read up to 4 bytes past
    /// the end of the slice (here: past the end of linear memory, which traps).
    #[test]
    fn encode_never_reads_past_linear_memory() {
        let mut data = vec![0u8; 64];
        fill_random(&mut data, 0x9003);

        // Compute every expected output before growing the memory: allocating after the
        // growth would move the end of linear memory and mask an over-read.
        let lengths = [12usize, 13, 14, 15, 16, 17, 24, 27, 28, 31, 32, 33, 48, 64];
        let mut expected = Vec::new();
        for &len in &lengths {
            for alphabet in ALPHABETS {
                expected.push((len, alphabet, scalar_encode(&data[..len], alphabet)));
            }
        }
        let mut out = vec![0u8; 96];

        let grown = memory_grow::<0>(1);
        assert_ne!(grown, usize::MAX, "cannot grow wasm linear memory");
        let end = memory_size::<0>() * 65536;

        for (len, alphabet, want) in expected {
            let start = (end - len) as *mut u8;
            // SAFETY: `start..start+len` is inside the page that was just grown.
            unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), start, len) };
            // SAFETY: as above; the memory is initialised and not used elsewhere.
            let input = unsafe { core::slice::from_raw_parts(start, len) };

            encode_into(&mut out[..want.len()], input, alphabet)
                .unwrap_or_else(|e| panic!("boundary encode len={len} alphabet={alphabet:?}: {e:?}"));
            assert_eq!(
                &out[..want.len()],
                &want[..],
                "boundary encode mismatch len={len} alphabet={alphabet:?}"
            );
        }
    }

    /// The decoder's input is always a multiple of 16 in the vector loop, so placing it
    /// at the end of linear memory must not trap.
    #[test]
    fn decode_never_reads_past_linear_memory() {
        let lengths = [16usize, 32, 48, 64, 96];
        let mut expected = Vec::new();
        for &len in &lengths {
            let plain = vec![len as u8; len / 4 * 3];
            let encoded = scalar_encode(&plain, Alphabet::Standard);
            assert_eq!(encoded.len(), len);
            expected.push((len, encoded, plain));
        }
        let mut out = vec![0u8; 96];

        let grown = memory_grow::<0>(1);
        assert_ne!(grown, usize::MAX, "cannot grow wasm linear memory");
        let end = memory_size::<0>() * 65536;

        for (len, encoded, plain) in expected {
            let start = (end - len) as *mut u8;
            // SAFETY: `start..start+len` is inside the page that was just grown.
            unsafe { core::ptr::copy_nonoverlapping(encoded.as_ptr(), start, len) };
            // SAFETY: as above; the memory is initialised and not used elsewhere.
            let input = unsafe { core::slice::from_raw_parts(start, len) };

            let n = decode_into(&mut out[..plain.len()], input, Alphabet::Standard)
                .unwrap_or_else(|e| panic!("boundary decode len={len}: {e:?}"));
            assert_eq!(n, plain.len());
            assert_eq!(&out[..n], &plain[..], "boundary decode mismatch len={len}");
        }
    }
}

// ======================================================================================
// Guard-page tests (x86_64 AVX2 only): put the input slice flush against an unreadable
// page so any over-read of the vector loads faults immediately.
// ======================================================================================
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod guard_page {
    use super::*;

    const PAGE: usize = 4096;

    unsafe fn sys_mmap(len: usize) -> *mut u8 {
        let minus_one: isize = -1;
        let ret: isize;
        // SAFETY: raw mmap syscall with a private anonymous mapping.
        unsafe {
            core::arch::asm!(
                "syscall",
                inlateout("rax") 9isize => ret, // SYS_mmap
                in("rdi") 0usize,
                in("rsi") len,
                in("rdx") 0b011usize,          // PROT_READ | PROT_WRITE
                in("r10") 0x22usize,           // MAP_PRIVATE | MAP_ANONYMOUS
                in("r8") minus_one,
                in("r9") 0usize,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        assert!(ret > 0, "mmap failed: {ret}");
        ret as *mut u8
    }

    unsafe fn sys_mprotect(addr: *mut u8, len: usize, prot: usize) -> isize {
        let ret: isize;
        // SAFETY: raw mprotect syscall on the mapping returned by `sys_mmap`.
        unsafe {
            core::arch::asm!(
                "syscall",
                inlateout("rax") 10isize => ret, // SYS_mprotect
                in("rdi") addr,
                in("rsi") len,
                in("rdx") prot,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        ret
    }

    /// Returns a writable page followed by a PROT_NONE page.
    fn guarded_pages() -> (*mut u8, *mut u8) {
        // SAFETY: mmap/mprotect of two pages; the second is made inaccessible.
        unsafe {
            let base = sys_mmap(2 * PAGE);
            assert_eq!(sys_mprotect(base.add(PAGE), PAGE, 0), 0, "mprotect failed");
            (base, base.add(PAGE))
        }
    }

    #[test]
    fn encode_input_end_of_page() {
        for &len in &[24usize, 27, 28, 32, 40, 52, 64, 128] {
            let (base, guard) = guarded_pages();
            let mut data = vec![0u8; len];
            fill_random(&mut data, 0x9001 ^ len as u64);

            let start = unsafe { guard.sub(len) };
            // SAFETY: `start..start+len` lies in the writable page.
            unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), start, len) };
            let input = unsafe { core::slice::from_raw_parts(start, len) };

            for alphabet in ALPHABETS {
                let expected = scalar_encode(&data, alphabet);
                let mut out = vec![0u8; expected.len()];
                encode_into(&mut out, input, alphabet)
                    .unwrap_or_else(|e| panic!("guard encode len={len} alphabet={alphabet:?}: {e:?}"));
                assert_eq!(out, expected, "guard encode mismatch len={len} alphabet={alphabet:?}");
            }

            // SAFETY: unmapping memory we allocated; no references remain.
            unsafe {
                let _ = sys_mprotect(base, PAGE, 0b011);
                let _ = sys_mprotect(base, 2 * PAGE, 0);
            }
        }
    }

    #[test]
    fn decode_input_end_of_page() {
        for &len in &[32usize, 64, 96, 128] {
            let (base, guard) = guarded_pages();
            let mut data = vec![0u8; len / 4 * 3];
            fill_random(&mut data, 0x9002 ^ len as u64);

            for alphabet in ALPHABETS {
                let encoded = scalar_encode(&data, alphabet);
                assert_eq!(encoded.len(), len);

                let start = unsafe { guard.sub(len) };
                // SAFETY: `start..start+len` lies in the writable page.
                unsafe { core::ptr::copy_nonoverlapping(encoded.as_ptr(), start, len) };
                let input = unsafe { core::slice::from_raw_parts(start, len) };

                let mut out = vec![0u8; data.len()];
                let n = decode_into(&mut out, input, alphabet)
                    .unwrap_or_else(|e| panic!("guard decode len={len} alphabet={alphabet:?}: {e:?}"));
                assert_eq!(n, data.len());
                assert_eq!(out, data, "guard decode mismatch len={len} alphabet={alphabet:?}");
            }

            // SAFETY: unmapping memory we allocated; no references remain.
            unsafe {
                let _ = sys_mprotect(base, PAGE, 0b011);
                let _ = sys_mprotect(base, 2 * PAGE, 0);
            }
        }
    }
}
