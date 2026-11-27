//! Audit tests for the vendored `zmij` crate.
//!
//! Covers:
//!   * the upstream `tests/test.rs` cases (with the `ryu` oracle replaced by
//!     exact expected strings / round-trip + shortest-digit checks),
//!   * the upstream `tests/generated/boundary_bits.rs` rounding-boundary list
//!     (expected shortest decimal strings taken from the file's comments),
//!   * an exponent x significand sweep for every f64 and f32 exponent,
//!   * powers of ten, subnormal endpoints, fixed/exponential edges,
//!   * the 24-byte `Buffer` length boundary (f64::MIN/MAX),
//!   * NaN / infinity for the safe `format` API and `format_finite` on every
//!     non-finite class (the latter is documented as unspecified but must stay
//!     memory-safe),
//!   * a deterministic random differential sweep with round-trip and shortest
//!     checks.
//!
//! Under Miri the big sweeps are skipped/reduced; the edge/special-value tests
//! still run and are what check raw-pointer provenance, table bounds, and
//! uninitialized reads.

use std::fmt::Write as _;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Deterministic SplitMix64, so failures are reproducible and no dev-deps are
/// needed.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn dtoa(value: f64) -> String {
    zmij::Buffer::new().format(value).to_owned()
}

fn ftoa(value: f32) -> String {
    zmij::Buffer::new().format(value).to_owned()
}

fn dtoa_finite(value: f64) -> String {
    zmij::Buffer::new().format_finite(value).to_owned()
}

fn ftoa_finite(value: f32) -> String {
    zmij::Buffer::new().format_finite(value).to_owned()
}

/// Canonical shortest-decimal form: (negative, significant digits with leading
/// and trailing zeros removed, decimal exponent of the last digit).
///
/// Examples:
///   "1.234e-6" -> (false, 1234, -9)
///   "10000.0"  -> (false, 1, 4)
///   "0.001234" -> (false, 1234, -6)
fn canonical(s: &str) -> (bool, String, i64) {
    let (neg, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s),
    };
    let (mant, exp) = match s.split_once(['e', 'E']) {
        Some((m, e)) => (m, e.parse::<i64>().expect("exponent")),
        None => (s, 0),
    };
    let mut digits = String::new();
    let mut frac = 0i64;
    let mut seen_point = false;
    for ch in mant.chars() {
        if ch == '.' {
            assert!(!seen_point);
            seen_point = true;
        } else {
            assert!(ch.is_ascii_digit(), "not a decimal digit: {ch:?} in {s:?}");
            digits.push(ch);
            if seen_point {
                frac += 1;
            }
        }
    }
    let mut value: u64 = digits.parse().expect("digits");
    let mut e = exp - frac;
    if value == 0 {
        return (neg, String::new(), 0);
    }
    while value.is_multiple_of(10) {
        value /= 10;
        e += 1;
    }
    (neg, value.to_string(), e)
}

/// Check that `s` is exactly the shortest correctly-rounded decimal for `x`.
///
/// Apart from round-tripping, this removes the final significand digit and
/// tries both one-shorter neighbours (`prefix` and `prefix + 1` at one decimal
/// place coarser). Neither may round-trip to `x`.
fn shortest_f64(x: f64, s: &str) -> bool {
    if x == 0.0 {
        return s == "0.0" || s == "-0.0" || s == "0" || s == "-0";
    }
    let (neg, digits, e) = canonical(s);
    if neg != x.is_sign_negative() {
        return false;
    }
    let d: u64 = if digits.is_empty() { 0 } else { digits.parse().unwrap() };
    if d < 10 {
        // One significant digit: no shorter non-zero decimal can exist at this
        // decimal exponent.
        return true;
    }
    let prefix = d / 10;
    for cand in [prefix, prefix + 1] {
        let mut text = String::new();
        write!(&mut text, "{}e{}", cand, e + 1).unwrap();
        if text.parse::<f64>().map(f64::to_bits) == Ok(x.to_bits()) {
            return false;
        }
    }
    true
}

fn shortest_f32(x: f32, s: &str) -> bool {
    if x == 0.0 {
        return s == "0.0" || s == "-0.0" || s == "0" || s == "-0";
    }
    let (neg, digits, e) = canonical(s);
    if neg != x.is_sign_negative() {
        return false;
    }
    let d: u64 = if digits.is_empty() { 0 } else { digits.parse().unwrap() };
    if d < 10 {
        return true;
    }
    let prefix = d / 10;
    for cand in [prefix, prefix + 1] {
        let mut text = String::new();
        write!(&mut text, "{}e{}", cand, e + 1).unwrap();
        if text.parse::<f32>().map(f32::to_bits) == Ok(x.to_bits()) {
            return false;
        }
    }
    true
}

fn check_f64(x: f64) {
    let s = dtoa_finite(x);
    assert!(s.len() <= 24, "output too long for {x:?}: {s:?} ({} bytes)", s.len());
    assert!(s.is_ascii(), "non-ASCII output for {x:?} bits=0x{:016x}: {s:?}", x.to_bits());
    assert_eq!(
        s.parse::<f64>().map(f64::to_bits),
        Ok(x.to_bits()),
        "round-trip failed: bits=0x{:016x} zmij={s:?}",
        x.to_bits(),
    );
    assert!(
        shortest_f64(x, &s),
        "not shortest: bits=0x{:016x} x={x:?} zmij={s:?}",
        x.to_bits(),
    );
}

fn check_f32(x: f32) {
    let s = ftoa_finite(x);
    assert!(s.len() <= 24, "output too long for {x:?}: {s:?} ({} bytes)", s.len());
    assert!(s.is_ascii(), "non-ASCII output for {x:?} bits=0x{:08x}: {s:?}", x.to_bits());
    assert_eq!(
        s.parse::<f32>().map(f32::to_bits),
        Ok(x.to_bits()),
        "round-trip failed: bits=0x{:08x} zmij={s:?}",
        x.to_bits(),
    );
    assert!(
        shortest_f32(x, &s),
        "not shortest: bits=0x{:08x} x={x:?} zmij={s:?}",
        x.to_bits(),
    );
}

// ---------------------------------------------------------------------------
// Upstream tests/test.rs, with ryu replaced by exact expectations / properties
// ---------------------------------------------------------------------------

#[test]
fn upstream_normal() {
    assert_eq!(dtoa(6.62607015e-34), "6.62607015e-34");
    assert_eq!(dtoa(5.444310685350916e+14), "544431068535091.6");
}

#[test]
fn upstream_subnormal() {
    assert_eq!(dtoa(0.0f64.next_up()), "5e-324");
    assert_eq!(dtoa(1e-323), "1e-323");
    assert_eq!(dtoa(1.2e-322), "1.2e-322");
    assert_eq!(dtoa(1.24e-322), "1.24e-322");
    assert_eq!(dtoa(1.234e-320), "1.234e-320");
    assert_eq!(dtoa(2.2250738585072004e-308), "2.2250738585072004e-308");
}

#[test]
fn upstream_all_irregular() {
    if cfg!(miri) {
        return;
    }
    // exp << 52 with a zero mantissa: powers of two, the irregular path.
    for exp in 1..0x7ffu64 {
        let bits = exp << 52;
        let value = f64::from_bits(bits);
        check_f64(value);
        check_f64(-value);
    }
}

#[test]
fn upstream_all_exponents() {
    if cfg!(miri) {
        return;
    }
    // Smallest mantissa for every finite exponent.
    for exp in 0..=0x7feu64 {
        for sig in [
            1u64,
            2,
            3,
            0x000F_FFFF_FFFF_FFFF,
            0x0008_0000_0000_0000,
            0x000A_AAAA_AAAA_AAAA,
            0x0005_5555_5555_5555,
        ] {
            let bits = (exp << 52) | sig;
            let value = f64::from_bits(bits);
            check_f64(value);
            check_f64(-value);
        }
    }
}

#[test]
fn upstream_small_int_zero_inf_nan() {
    assert_eq!(dtoa(1.0), "1.0");
    assert_eq!(dtoa(0.0), "0.0");
    assert_eq!(dtoa(-0.0), "-0.0");
    assert_eq!(dtoa(f64::INFINITY), "inf");
    assert_eq!(dtoa(f64::NEG_INFINITY), "-inf");
    assert_eq!(dtoa(f64::NAN.copysign(-1.0)), "NaN");
}

#[test]
fn upstream_shorter_single_candidate_boundary() {
    assert_eq!(dtoa(-4.932096661796888e-226), "-4.932096661796888e-226");
    assert_eq!(dtoa(3.439070283483335e+35), "3.439070283483335e+35");
    assert_eq!(dtoa(6.606854224493745e-17), "6.606854224493745e-17");
    assert_eq!(dtoa(6.079537928711555e+61), "6.079537928711555e+61");
    assert_eq!(dtoa(1.3588129002659584e-245), "1.3588129002659584e-245");
    assert_eq!(dtoa(2.9802322387695312e-08), "2.9802322387695312e-8");
    assert_eq!(dtoa(5.960464477539063e-08), "5.960464477539063e-8");
    assert_eq!(dtoa(1.3076622631878654e+65), "1.3076622631878654e+65");
    assert_eq!(dtoa(9.03725590277404e+159), "9.03725590277404e+159");
    assert_eq!(dtoa(9.03725590277404e+160), "9.03725590277404e+160");
    assert_eq!(dtoa(9.03725590277404e+161), "9.03725590277404e+161");
    assert_eq!(dtoa(9.03725590277404e+162), "9.03725590277404e+162");
    assert_eq!(dtoa(5.960464477539062e-07), "5.960464477539062e-7");
}

#[test]
fn upstream_fixed_with_zeros() {
    assert_eq!(dtoa(43210.0), "43210.0");
    assert_eq!(dtoa(43210.1), "43210.1");
    assert_eq!(dtoa(10000.0), "10000.0");
    assert_eq!(dtoa(-5942736479622170.0), "-5942736479622170.0");
}

#[test]
fn upstream_ftoa() {
    assert_eq!(ftoa(43210.0), "43210.0");
    assert_eq!(ftoa(43210.1), "43210.1");
    assert_eq!(ftoa(10000.0), "10000.0");
    assert_eq!(ftoa(6.62607e-34), "6.62607e-34");
    assert_eq!(ftoa(1.342178e+08), "134217800.0");
    assert_eq!(ftoa(1.3421781e+08), "134217810.0");
    assert_eq!(ftoa(0.0f32.next_up()), "1e-45");
    assert_eq!(ftoa(f32::INFINITY), "inf");
    assert_eq!(ftoa(f32::NEG_INFINITY), "-inf");
    assert_eq!(ftoa(f32::NAN.copysign(-1.0)), "NaN");
}

// ---------------------------------------------------------------------------
// Buffer length boundary: BUFFER_SIZE is 24.
// ---------------------------------------------------------------------------

#[test]
fn buffer_exact_boundary() {
    // The longest f64 output: sign + 17 digits + decimal point + e±ddd.
    let s = dtoa_finite(-f64::MAX);
    assert_eq!(s, "-1.7976931348623157e+308");
    assert_eq!(s.len(), 24, "longest output must fill the buffer exactly");

    let s = dtoa_finite(f64::MAX);
    assert_eq!(s, "1.7976931348623157e+308");
    assert_eq!(s.len(), 23);

    // Negative minimum subnormal / normal.
    for &x in &[
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        f64::from_bits(0x000F_FFFF_FFFF_FFFF),
    ] {
        let s = dtoa_finite(x);
        assert!(s.len() <= 24, "{s:?}");
        assert_eq!(s.parse::<f64>().unwrap().to_bits(), x.to_bits());
    }

    // The fixed-notation branch with dec_exp == -5 and 17 significant digits
    // is the other layout that can fill all 24 bytes: "-0.0000" + 17 digits +
    // sign = 24.
    let x: f64 = "1.2345678901234567e-5".parse().unwrap();
    let s = dtoa_finite(-x);
    assert_eq!(s.len(), 24, "unexpected fixed-boundary output {s:?}");

    // Longest f32 output.
    let s = ftoa_finite(-f32::MAX);
    assert_eq!(s, "-3.4028235e+38");
    assert_eq!(s.len(), 14);
}

// ---------------------------------------------------------------------------
// Powers of ten and fixed/exponential notation edges
// ---------------------------------------------------------------------------

#[test]
fn powers_of_ten() {
    if cfg!(miri) {
        return;
    }
    for e in -323..=308i32 {
        let text = format!("1e{e}");
        let x: f64 = text.parse().unwrap();
        check_f64(x);
        check_f64(-x);
        // The neighbour on either side of a power of ten is where the decimal
        // exponent crosses a boundary.
        check_f64(f64::from_bits(x.to_bits() - 1));
        check_f64(f64::from_bits(x.to_bits() + 1));
    }
}

#[test]
fn fixed_notation_edges_f64() {
    // FIXED_DEC_EXP = -5..=15 in the Rust port.
    for e in -8..=18i32 {
        let x: f64 = format!("1e{e}").parse().unwrap();
        for v in [
            x,
            f64::from_bits(x.to_bits().wrapping_sub(1)),
            f64::from_bits(x.to_bits().wrapping_add(1)),
            x * 1.234_567_890_123_45,
            x * 9.999_999_999_999_99,
        ] {
            if v.is_finite() {
                check_f64(v);
                check_f64(-v);
            }
        }
    }
    // Values straddling the fixed range explicitly.
    for &v in &[
        0.000_009_999_999_999_999,
        0.000_01,
        0.000_1,
        0.001,
        0.01,
        0.1,
        1.0,
        10.0,
        1e12,
        1e13,
        1e14,
        1e15,
        1e16,
        1e17,
        9007199254740991.0,
        9007199254740992.0,
        9007199254740994.0,
    ] {
        check_f64(v);
    }
}

#[test]
fn fixed_notation_edges_f32() {
    // FIXED_DEC_EXP = -6..=12 in the Rust port.
    for e in -9..=15i32 {
        let x: f32 = format!("1e{e}").parse().unwrap();
        for v in [
            x,
            f32::from_bits(x.to_bits().wrapping_sub(1)),
            f32::from_bits(x.to_bits().wrapping_add(1)),
            x * 1.234_567,
            x * 9.999_999,
        ] {
            if v.is_finite() {
                check_f32(v);
                check_f32(-v);
            }
        }
    }
}

#[test]
fn f32_all_exponents() {
    if cfg!(miri) {
        return;
    }
    for exp in 0..=0xffu32 {
        for sig in [0u32, 1, 2, 3, 0x7F_FFFF, 0x40_0000, 0x55_5555, 0x2A_AAAA] {
            let bits = (exp << 23) | sig;
            let x = f32::from_bits(bits);
            if x.is_finite() {
                check_f32(x);
                check_f32(-x);
            } else {
                // Safe API must handle every non-finite pattern.
                let s = ftoa(x);
                assert_eq!(s, if sig == 0 { "inf" } else { "NaN" });
            }
        }
    }
    // All subnormal boundaries.
    for &bits in &[1u32, 2, 0x7F_FFFF, 0x80_0000 - 1] {
        check_f32(f32::from_bits(bits));
        check_f32(-f32::from_bits(bits));
    }
}

// ---------------------------------------------------------------------------
// NaN and infinity: safe `format` exact, `format_finite` memory-safe
// ---------------------------------------------------------------------------

#[test]
fn nonfinite_format_finite_is_memory_safe() {
    // Every NaN payload class plus infinities and both signs. `format_finite`
    // documents the value as unspecified; the invariant audited here is that
    // it must not panic, overrun the 24-byte buffer, read uninitialized bytes,
    // or index a table out of bounds (Miri checks the last two).
    let f64_patterns = [
        0x7FF0_0000_0000_0000u64, // +inf
        0xFFF0_0000_0000_0000,    // -inf
        0x7FF8_0000_0000_0000,    // quiet +NaN
        0xFFF8_0000_0000_0000,    // quiet -NaN
        0x7FF0_0000_0000_0001,    // signaling +NaN, min payload
        0xFFF0_0000_0000_0001,    // signaling -NaN
        0x7FFF_FFFF_FFFF_FFFF,    // all payload bits
        0x7FF4_0000_0000_0000,
        0x7FFA_AAAA_AAAA_AAAA,
        0x7FFC_CCCC_CCCC_CCCC,
    ];
    for bits in f64_patterns {
        let x = f64::from_bits(bits);
        assert_eq!(
            dtoa(x),
            if bits & 0xF_FFFF_FFFF_FFFF != 0 {
                "NaN"
            } else if bits >> 63 != 0 {
                "-inf"
            } else {
                "inf"
            }
        );
        let s = dtoa_finite(x);
        assert!(s.len() <= 24, "bits=0x{bits:016x} s={s:?}");
        assert!(s.is_ascii(), "bits=0x{bits:016x} s={s:?}");
    }

    let f32_patterns = [
        0x7F80_0000u32,
        0xFF80_0000,
        0x7FC0_0000,
        0xFFC0_0000,
        0x7F80_0001,
        0xFF80_0001,
        0x7FFF_FFFF,
        0x7FA0_0000,
        0x7FD5_5555,
        0x7FEA_AAAA,
    ];
    for bits in f32_patterns {
        let x = f32::from_bits(bits);
        assert_eq!(
            ftoa(x),
            if bits & 0x7F_FFFF != 0 {
                "NaN"
            } else if bits >> 31 != 0 {
                "-inf"
            } else {
                "inf"
            }
        );
        let s = ftoa_finite(x);
        assert!(s.len() <= 24, "bits=0x{bits:08x} s={s:?}");
        assert!(s.is_ascii(), "bits=0x{bits:08x} s={s:?}");
    }

    // All 2^13 combinations of exponent-all-ones, sign and high mantissa bits
    // are too many for Miri; sample a deterministic spread instead.
    let mut rng = Rng::new(0x5EED_1234_ABCD_0001);
    for _ in 0..256 {
        let payload = rng.next_u64() & 0x000F_FFFF_FFFF_FFFF;
        let bits = 0x7FF0_0000_0000_0000 | payload | ((rng.next_u64() & 1) << 63);
        let s = dtoa_finite(f64::from_bits(bits));
        assert!(s.len() <= 24);
    }
    for _ in 0..256 {
        let payload = (rng.next_u64() as u32) & 0x007F_FFFF;
        let bits = 0x7F80_0000 | payload | (((rng.next_u64() as u32) & 1) << 31);
        let s = ftoa_finite(f32::from_bits(bits));
        assert!(s.len() <= 24);
    }
}

// ---------------------------------------------------------------------------
// Upstream rounding-boundary corpus (independent ryu/dragonbox strings)
// ---------------------------------------------------------------------------

#[test]
fn upstream_boundary_bits() {
    let text = include_str!("generated/boundary_bits.rs");
    let mut count = 0usize;
    for line in text.lines() {
        let line = line.trim();
        let Some(hex) = line.strip_prefix("0x") else {
            continue;
        };
        let (digits, rest) = match hex.split_once(',') {
            Some(parts) => parts,
            None => (hex, ""),
        };
        let bits = u64::from_str_radix(digits.trim(), 16).expect("hex bits");
        let expected = rest
            .split_once("//")
            .map(|(_, e)| e.trim())
            .expect("expected decimal comment");
        let value = f64::from_bits(bits);
        // The comment is the shortest representation from the independent
        // dragonbox/ryu reference. Compare digits and exponent, not notation.
        assert_eq!(
            canonical(&dtoa_finite(value)),
            canonical(expected),
            "bits=0x{bits:016x} zmij={:?} expected={expected:?}",
            dtoa_finite(value),
        );
        assert_eq!(value.to_bits(), expected.parse::<f64>().unwrap().to_bits());
        count += 1;

        if cfg!(miri) && count >= 64 {
            break;
        }
    }
    let min_count = if cfg!(miri) { 64 } else { 5000 };
    assert!(count >= min_count, "boundary corpus unexpectedly small: {count}");
}

// ---------------------------------------------------------------------------
// Random differential sweep
// ---------------------------------------------------------------------------

#[test]
fn random_differential() {
    // The x86_64 binary is run under qemu (to exercise the SSE2/SSE4.1
    // backends); keep that run short. Native aarch64/NEON gets the full sweep.
    let n: usize = if cfg!(miri) {
        2_000
    } else if cfg!(target_arch = "x86_64") {
        300_000
    } else {
        20_000_000
    };
    let mut rng = Rng::new(0xD1FF_3E7A_5EED_2026);

    for _ in 0..n {
        let bits = rng.next_u64();
        let x = f64::from_bits(bits);
        if x.is_finite() {
            check_f64(x);
        }
    }

    for _ in 0..n {
        let bits = rng.next_u64() as u32;
        let x = f32::from_bits(bits);
        if x.is_finite() {
            check_f32(x);
        }
    }

    // Targeted f64 sweeps: subnormals (raw exponent 0) and the top finite
    // binade (raw exponent 0x7fe), where the table indices and the subnormal
    // normalization loop are most extreme.
    let m: usize = if cfg!(miri) {
        200
    } else if cfg!(target_arch = "x86_64") {
        20_000
    } else {
        2_000_000
    };
    for _ in 0..m {
        let bits = rng.next_u64() & 0x800F_FFFF_FFFF_FFFF; // sign | exp 0
        check_f64(f64::from_bits(bits));
    }
    for _ in 0..m {
        let bits = (0x7FEu64 << 52) | (rng.next_u64() & 0x000F_FFFF_FFFF_FFFF);
        check_f64(f64::from_bits(bits));
        check_f64(f64::from_bits(bits | (1 << 63)));
    }
    for _ in 0..m {
        let bits = (rng.next_u64() as u32) & 0x807F_FFFF; // sign | exp 0
        check_f32(f32::from_bits(bits));
    }
}

// ---------------------------------------------------------------------------
// Buffer underrun / overrun sentinel (upstream issue #86 class)
// ---------------------------------------------------------------------------

#[repr(C)]
struct Sentinelled {
    pre: [u8; 16],
    buf: zmij::Buffer,
    post: [u8; 16],
}

#[test]
fn no_underrun_or_overrun() {
    // The upstream C++ implementation once wrote one byte before the supplied
    // buffer in the SSE fixed path (vitaut/zmij issue #86, reproducer
    // 9.061488e+15). The Rust fixed path uses ptr::copy instead, but check the
    // property directly with poisoned bytes on both sides of the 24-byte
    // Buffer for all layout branches.
    let mut values = vec![
        9.061488e+15f64,
        1.2345678901234567e-5,
        -1.2345678901234567e-5,
        f64::MAX,
        -f64::MAX,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        1e-5,
        1e-4,
        1e15,
        1e16,
        43210.0,
        -5942736479622170.0,
    ];
    let mut rng = Rng::new(0x0BAD_F00D_1234_5678);
    for _ in 0..1000 {
        let x = f64::from_bits(rng.next_u64());
        if x.is_finite() {
            values.push(x);
        }
    }

    for x in values {
        let mut s = Sentinelled {
            pre: [0xAA; 16],
            buf: zmij::Buffer::new(),
            post: [0xBB; 16],
        };
        let len;
        {
            let out = s.buf.format_finite(x);
            assert_eq!(out.parse::<f64>().unwrap().to_bits(), x.to_bits());
            len = out.len();
        }
        assert!(len <= 24);
        assert_eq!(s.pre, [0xAA; 16], "underrun for bits=0x{:016x}", x.to_bits());
        assert_eq!(s.post, [0xBB; 16], "overrun for bits=0x{:016x}", x.to_bits());
    }

    let f32_values = [
        9.061488e+15f32,
        1.234567e-5,
        -1.234567e-5,
        f32::MAX,
        -f32::MAX,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        -f32::from_bits(1),
        1e-5,
        1e-4,
        1e12,
        1e13,
        43210.0,
    ];
    for x in f32_values {
        let mut s = Sentinelled {
            pre: [0xCC; 16],
            buf: zmij::Buffer::new(),
            post: [0xDD; 16],
        };
        let len;
        {
            let out = s.buf.format_finite(x);
            assert_eq!(out.parse::<f32>().unwrap().to_bits(), x.to_bits());
            len = out.len();
        }
        assert!(len <= 24);
        assert_eq!(s.pre, [0xCC; 16], "underrun for bits=0x{:08x}", x.to_bits());
        assert_eq!(s.post, [0xDD; 16], "overrun for bits=0x{:08x}", x.to_bits());
    }
}

// ---------------------------------------------------------------------------
// Full 2^32 f32 sweep (opt-in; run with `--ignored` in release)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "full 2^32 f32 sweep; run with --ignored --release"]
fn exhaustive_f32_bits() {
    // Round-trip and buffer-length invariants for every f32 bit pattern,
    // including every signaling/quiet NaN payload and infinity. The shortest
    // property is covered by `random_differential` and the exponent sweeps;
    // it is skipped here to keep the sweep within the runtime budget.
    const THREADS: u64 = 3;
    let mut workers = Vec::new();
    for t in 0..THREADS {
        workers.push(std::thread::spawn(move || {
            let start = t * (1u64 << 32) / THREADS;
            let end = (t + 1) * (1u64 << 32) / THREADS;
            let mut buf = zmij::Buffer::new();
            let mut bits = start;
            while bits < end {
                let b = bits as u32;
                let x = f32::from_bits(b);
                if x.is_finite() {
                    let s = buf.format_finite(x);
                    assert!(s.len() <= 24, "bits=0x{b:08x} s={s:?}");
                    assert_eq!(s.parse::<f32>().map(f32::to_bits), Ok(b), "bits=0x{b:08x} s={s:?}");
                } else {
                    let s = buf.format(x);
                    let expected = if b & 0x007F_FFFF != 0 {
                        "NaN"
                    } else if b >> 31 != 0 {
                        "-inf"
                    } else {
                        "inf"
                    };
                    assert_eq!(s, expected, "bits=0x{b:08x}");
                }
                bits += 1;
            }
        }));
    }
    for worker in workers {
        worker.join().unwrap();
    }
}

#[test]
fn f32_exponential_exact() {
    // Exact bytes for the f32 exponential path (NEON and SSE4.1 use the SIMD
    // shuffle table; SSE2/scalar use a different writer). These must agree.
    for &(x, expected) in &[
        (1.1754944e-38f32, "1.1754944e-38"),
        (1.1754942e-38, "1.1754942e-38"),
        (5.877472e-39, "5.877472e-39"),
        (1.2345678e-38, "1.2345678e-38"),
        (1.2345679e-38, "1.2345679e-38"),
        (9.999999e-38, "9.999999e-38"),
        (1e-30, "1e-30"),
        (1.2345678e-30, "1.2345678e-30"),
        (9.999999e-30, "9.999999e-30"),
        (1.0000001e-20, "1.0000001e-20"),
        (1e30, "1e+30"),
        (1.2345678e30, "1.2345678e+30"),
        (1e38, "1e+38"),
        (3.4028235e38, "3.4028235e+38"),
        (1e-6, "0.000001"),
        (1e-7, "1e-7"),
        (1e12, "1000000000000.0"),
        (1e13, "1e+13"),
        (f32::from_bits(1), "1e-45"),
    ] {
        assert_eq!(ftoa_finite(x), expected, "bits=0x{:08x}", x.to_bits());
        check_f32(x);
    }
}

// ---------------------------------------------------------------------------
// Buffer reuse: the returned &str must remain valid across subsequent calls
// (this mainly exercises provenance under Miri).
// ---------------------------------------------------------------------------

#[test]
fn buffer_reuse_and_provenance() {
    let mut buf = zmij::Buffer::new();
    let values = [
        0.0f64,
        -0.0,
        1.0,
        -1.0,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::MAX,
        -f64::MAX,
        1.234_567_890_123_456_7e-300,
        9.999_999_999_999_999e300,
    ];
    for &v in &values {
        let s = buf.format_finite(v);
        assert_eq!(s.parse::<f64>().unwrap().to_bits(), v.to_bits());
        // Reuse must overwrite, and the reference must still be in-bounds.
        assert!(s.len() <= 24);
    }
    for &v in &[0.0f32, -0.0, 1.0, f32::MAX, f32::from_bits(1)] {
        let s = buf.format_finite(v);
        assert_eq!(s.parse::<f32>().unwrap().to_bits(), v.to_bits());
        assert!(s.len() <= 24);
    }
}
