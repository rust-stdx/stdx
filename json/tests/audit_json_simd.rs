//! Audit tests for the SIMD string-scanning kernels.
//!
//! The production `simd_avx2.rs` / `simd_neon.rs` sources are included
//! verbatim into this test crate with a local `scalar_find_string_special`
//! shim, so the exact kernel code is compared byte-for-byte against two
//! independent references: a naive per-byte scan and a word-at-a-time scalar
//! scan equivalent to the production fallback.

#![cfg(feature = "alloc")]

// ---------------------------------------------------------------------------
// References
// ---------------------------------------------------------------------------

#[inline]
fn is_special(byte: u8) -> bool {
    byte == b'"' || byte == b'\\' || byte < 0x20
}

/// Naive, obviously correct reference.
fn reference(input: &[u8]) -> usize {
    input.iter().position(|&byte| is_special(byte)).unwrap_or(input.len())
}

#[inline]
fn has_zero_byte(value: u64) -> bool {
    (value.wrapping_sub(0x0101_0101_0101_0101)) & !value & 0x8080_8080_8080_8080 != 0
}

/// Re-implementation of the production scalar fallback (word-at-a-time).
fn scalar_word_at_a_time(input: &[u8]) -> usize {
    let mut i = 0;
    while i + 8 <= input.len() {
        let word = u64::from_le_bytes(input[i..i + 8].try_into().unwrap());
        let quote = word ^ 0x2222_2222_2222_2222;
        let backslash = word ^ 0x5c5c_5c5c_5c5c_5c5c;
        let control = word.wrapping_sub(0x2020_2020_2020_2020) & !word & 0x8080_8080_8080_8080;
        if has_zero_byte(quote) || has_zero_byte(backslash) || control != 0 {
            for offset in 0..8 {
                if is_special(input[i + offset]) {
                    return i + offset;
                }
            }
        }
        i += 8;
    }
    while i < input.len() {
        if is_special(input[i]) {
            return i;
        }
        i += 1;
    }
    input.len()
}

// ---------------------------------------------------------------------------
// Include the real kernels. The production files call
// `super::scalar_find_string_special`, so a root-level shim stands in for the
// crate-private scalar fallback.
// ---------------------------------------------------------------------------

#[allow(dead_code)]
fn scalar_find_string_special(input: &[u8]) -> usize {
    let a = reference(input);
    let b = scalar_word_at_a_time(input);
    assert_eq!(a, b, "scalar references disagree");
    a
}

#[cfg(target_arch = "x86_64")]
#[path = "../simd_avx2.rs"]
mod simd_avx2;

#[cfg(target_arch = "aarch64")]
#[path = "../simd_neon.rs"]
mod simd_neon;

#[allow(dead_code)] // Only used on x86_64.
fn avx2(input: &[u8]) -> usize {
    // SAFETY: AVX2 is guaranteed by the QEMU `-cpu max` target used for
    // x86_64 tests; the kernel itself is what is under test.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        simd_avx2::find_string_special(input)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        reference(input)
    }
}

#[allow(dead_code)] // Only used on aarch64.
fn neon(input: &[u8]) -> usize {
    // SAFETY: NEON is mandatory on AArch64.
    #[cfg(target_arch = "aarch64")]
    unsafe {
        simd_neon::find_string_special(input)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        reference(input)
    }
}

/// Run the native kernel for this target, or the scalar reference otherwise
/// (including under Miri, which cannot execute SIMD intrinsics).
#[cfg(all(not(miri), target_arch = "x86_64"))]
fn kernel(input: &[u8]) -> usize {
    avx2(input)
}

#[cfg(all(not(miri), target_arch = "aarch64"))]
fn kernel(input: &[u8]) -> usize {
    neon(input)
}

#[cfg(any(miri, not(any(target_arch = "x86_64", target_arch = "aarch64"))))]
fn kernel(input: &[u8]) -> usize {
    reference(input)
}

struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

fn check(input: &[u8]) {
    let expected = reference(input);
    let got = kernel(input);
    assert_eq!(
        got,
        expected,
        "kernel mismatch (len={}, expected={expected}, got={got}) input={input:?}",
        input.len()
    );
    assert_eq!(scalar_word_at_a_time(input), expected);
    // Invariants the caller relies on.
    assert!(got <= input.len());
    if got < input.len() {
        assert!(is_special(input[got]));
        assert!(!input[..got].iter().any(|&b| is_special(b)));
    }
}

#[test]
fn empty_and_short_inputs() {
    check(b"");
    for len in 0..=80usize {
        let body = vec![b'a'; len];
        check(&body);
        for &special in &[b'"', b'\\', b'\n', 0u8, 0x1f, b'a', 0x7f, 0x80, 0xff] {
            for at in 0..=len {
                let mut bytes = body.clone();
                bytes.insert(at, special);
                check(&bytes);
            }
        }
    }
}

#[test]
fn every_byte_at_every_block_boundary() {
    // 16/32/64-byte block boundaries and their neighbours, plus the tail.
    let offsets = [
        0usize, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33, 47, 48, 49, 63, 64, 65, 66, 79, 80, 95, 96, 127, 128, 129, 130,
        191, 192, 193,
    ];
    for len in [1usize, 16, 31, 32, 63, 64, 65, 127, 128, 129, 200] {
        for &at in &offsets {
            if at >= len {
                continue;
            }
            for byte in 0..=255u8 {
                let mut bytes = vec![b'a'; len];
                bytes[at] = byte;
                check(&bytes);
            }
        }
    }
}

#[test]
fn special_at_every_offset_with_long_bodies() {
    for len in [16usize, 32, 64, 65, 96, 128, 200, 256] {
        for at in 0..len {
            for &special in &[b'"', b'\\', 0x00, 0x01, 0x1e, 0x1f] {
                let mut bytes = vec![b'x'; len];
                bytes[at] = special;
                check(&bytes);
                // A second special after the first must not change the result.
                if at + 1 < len {
                    bytes[at + 1] = b'"';
                    check(&bytes);
                }
            }
        }
    }
}

#[test]
fn random_buffers_match_reference() {
    let iterations = if cfg!(miri) { 500 } else { 300_000 };
    let mut rng = Rng(0xDEAD_BEEF_CAFE_0001);
    let specials = [b'"', b'\\', 0x00, 0x05, 0x10, 0x1f];
    for iteration in 0..iterations {
        let len = rng.below(260) as usize;
        let mut bytes = vec![0u8; len];
        // Mixture of runs of ordinary bytes and random bytes.
        let ordinary = b'a' + (rng.below(26) as u8);
        let mut i = 0;
        while i < len {
            if rng.below(8) == 0 {
                let run = 1 + rng.below(70) as usize;
                let end = (i + run).min(len);
                bytes[i..end].fill(ordinary);
                i = end;
            } else {
                bytes[i] = rng.below(256) as u8;
                i += 1;
            }
        }
        // Sprinkle exact specials at block boundaries.
        if iteration % 3 == 0 && len > 0 {
            let at = (rng.below(5) as usize * 16).min(len - 1);
            bytes[at] = specials[rng.below(specials.len() as u64) as usize];
        }
        check(&bytes);
    }
}

#[test]
fn misaligned_slices() {
    // Slide a window through a page-ish buffer so the kernel sees every
    // possible alignment while the special sits at varying distances.
    let mut buffer = vec![b'z'; 512];
    for &special in &[b'"', b'\\', 0x00, 0x1f] {
        for at in [0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 200, 511] {
            buffer.fill(b'z');
            buffer[at] = special;
            for start in 0..64 {
                check(&buffer[start..]);
            }
        }
    }
    // Empty windows at every alignment.
    for start in 0..64 {
        check(&buffer[start..start]);
    }
}

#[test]
fn find_returns_len_when_no_special() {
    for len in [0usize, 1, 7, 8, 15, 16, 31, 32, 63, 64, 65, 100, 1000] {
        let bytes = vec![b'A'; len];
        assert_eq!(kernel(&bytes), len);
    }
}

// ---------------------------------------------------------------------------
// Public API scanning across the same boundaries (ASCII only, so the only
// possible source of rejection is the string scanner).
// ---------------------------------------------------------------------------

#[test]
fn public_validate_scans_every_ascii_byte_at_boundaries() {
    let offsets = [
        0usize, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 70, 127, 128, 129,
    ];
    for len in [1usize, 8, 16, 17, 31, 32, 33, 63, 64, 65, 100, 128, 129, 200] {
        for &at in &offsets {
            if at >= len {
                continue;
            }
            for byte in 0..=0x7fu8 {
                let mut body = vec![b'a'; len];
                body[at] = byte;
                let mut literal = Vec::with_capacity(len + 2);
                literal.push(b'"');
                literal.extend_from_slice(&body);
                literal.push(b'"');
                // `"` terminates the string early and `\` starts an invalid
                // escape (`\a`), controls are rejected; everything else is
                // ordinary data. All cases must be rejected except ordinary.
                let expected = !is_special(byte);
                assert_eq!(
                    json::validate(&literal).is_ok(),
                    expected,
                    "validate len={len} at={at} byte=0x{byte:02X}"
                );
                let decoded = json::from_slice::<String>(&literal);
                assert_eq!(
                    decoded.is_ok(),
                    expected,
                    "from_slice<String> len={len} at={at} byte=0x{byte:02X}"
                );
                if expected {
                    assert_eq!(decoded.unwrap().as_bytes(), &body[..]);
                }
                let value = json::from_slice::<json::Value>(&literal);
                assert_eq!(value.is_ok(), expected, "Value len={len} at={at} byte=0x{byte:02X}");
            }
        }
    }
}

#[test]
fn escapes_at_every_offset_across_boundaries() {
    // One escape sequence inserted at every offset of bodies around the SIMD
    // block boundaries: the escaped byte must never terminate the scan and the
    // decoded output must be the literal body with the escape resolved.
    let escapes: &[(&str, &str)] = &[
        (r#"\""#, "\""),
        (r#"\\"#, "\\"),
        (r#"\/"#, "/"),
        (r#"\n"#, "\n"),
        (r#"\t"#, "\t"),
        (r#"\u0041"#, "A"),
        (r#"\uD83D\uDE00"#, "😀"),
    ];
    for len in [0usize, 1, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 100] {
        for at in 0..=len {
            for &(escape, decoded) in escapes {
                let mut body = "a".repeat(at);
                body.push_str(escape);
                body.push_str(&"a".repeat(len - at));
                let literal = format!("\"{body}\"");
                assert!(json::validate(literal.as_bytes()).is_ok(), "{literal:?}");
                let expected = format!("{}{decoded}{}", "a".repeat(at), "a".repeat(len - at));
                let got = json::from_slice::<String>(literal.as_bytes()).unwrap_or_else(|e| {
                    panic!("{literal:?}: {e}");
                });
                assert_eq!(got, expected, "{literal:?}");
                let value: json::Value = json::from_slice(literal.as_bytes()).unwrap();
                assert_eq!(value.as_str(), Some(expected.as_str()));
            }
        }
    }
}

#[test]
fn public_validate_long_strings_with_escapes() {
    // Long bodies with escapes must terminate at the right place and never
    // read past the end; the escaped quote/backslash must be skipped as a pair.
    for len in [0usize, 1, 15, 16, 31, 32, 33, 63, 64, 65, 100, 127, 128, 129, 500] {
        let mut body = String::new();
        let mut expected_a = 0usize;
        let mut expected_newline = 0usize;
        let mut expected_e = 0usize;
        let mut expected_quote = 0usize;
        for i in 0..len {
            match i % 5 {
                0 => {
                    body.push('a');
                    expected_a += 1;
                }
                1 => {
                    body.push_str("\\n");
                    expected_newline += 1;
                }
                2 => {
                    body.push_str("\\\"");
                    expected_quote += 1;
                }
                3 => body.push_str("\\\\"),
                _ => {
                    body.push('é');
                    expected_e += 1;
                }
            }
        }
        let literal = format!("\"{body}\"");
        assert!(json::validate(literal.as_bytes()).is_ok(), "len={len}");
        let decoded = json::from_slice::<String>(literal.as_bytes()).unwrap();
        assert_eq!(decoded.matches('a').count(), expected_a, "len={len}");
        assert_eq!(decoded.matches('é').count(), expected_e, "len={len}");
        assert_eq!(decoded.matches('\n').count(), expected_newline, "len={len}");
        assert_eq!(decoded.matches('"').count(), expected_quote, "len={len}");
    }
    // Truncation at every point of a long escaped string must not panic.
    let body = "ab\\n\\\"c\\\\dé".repeat(20);
    let literal = format!("\"{body}\"");
    for end in 0..literal.len() {
        let _ = json::validate(&literal.as_bytes()[..end]);
        let _ = json::from_slice::<String>(&literal.as_bytes()[..end]);
    }
}
