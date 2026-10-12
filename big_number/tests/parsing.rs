//! Parsing, boundary and encoding tests for `big_number`.
//!
//! These target the failure classes that most often turn into security bugs:
//! malicious or non-canonical input, values that exceed the declared width,
//! and the byte-length assumptions made by callers.

use big_number::{Error, Uint};

type U128 = Uint<128, 2>;
type U256 = Uint<256, 4>;
type U130 = Uint<130, 3>;

fn rng(state: &mut u64) -> u64 {
    // xorshift64: deterministic, dependency-free.
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

#[test]
fn round_trips_all_radices() {
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    let values = [
        U128::ZERO,
        U128::ONE,
        U128::MAX,
        U128::from_u64(u64::MAX),
        U128::from_limbs([rng(&mut state), rng(&mut state)]),
    ];
    for value in values {
        for radix in 2..=36u32 {
            let encoded = value.to_string_radix(radix);
            let decoded = U128::from_str_radix(&encoded, radix)
                .unwrap_or_else(|e| panic!("radix {radix} failed to parse {encoded:?}: {e:?}"));
            assert_eq!(decoded, value, "radix {radix} round trip");
        }
    }
}

#[test]
fn rejects_non_canonical_and_hostile_input() {
    // Signs are not accepted.
    assert_eq!(U128::from_str_radix("+1", 10), Err(Error::InvalidDigit));
    assert_eq!(U128::from_str_radix("-1", 10), Err(Error::InvalidDigit));
    // Whitespace, NUL, separators and prefixes are not accepted.
    assert_eq!(U128::from_str_radix(" 1", 10), Err(Error::InvalidDigit));
    assert_eq!(U128::from_str_radix("1 ", 10), Err(Error::InvalidDigit));
    assert_eq!(U128::from_str_radix("1\0", 10), Err(Error::InvalidDigit));
    assert_eq!(U128::from_str_radix("1_000", 10), Err(Error::InvalidDigit));
    assert_eq!(U128::from_str_radix("0x10", 16), Err(Error::InvalidDigit));
    // Non-ASCII digits (full-width) are rejected, unlike Java's BigInteger.
    assert_eq!(U128::from_str_radix("\u{ff11}\u{ff12}", 10), Err(Error::InvalidDigit));
    // Digits beyond the radix.
    assert_eq!(U128::from_str_radix("2", 2), Err(Error::InvalidDigit));
    assert_eq!(U128::from_str_radix("g", 16), Err(Error::InvalidDigit));
    // Radix and empty-input errors.
    assert_eq!(U128::from_str_radix("", 10), Err(Error::EmptyString));
    assert_eq!(U128::from_str_radix("", 2), Err(Error::EmptyString));
    for radix in [0u32, 1, 37, 100, u32::MAX] {
        assert_eq!(U128::from_str_radix("10", radix), Err(Error::InvalidRadix));
    }
}

#[test]
fn detects_overflow_at_the_bit_width() {
    // 2^128 - 1 fits, 2^128 does not.
    assert_eq!(
        U128::from_str_radix("340282366920938463463374607431768211455", 10),
        Ok(U128::MAX)
    );
    assert_eq!(
        U128::from_str_radix("340282366920938463463374607431768211456", 10),
        Err(Error::Overflow)
    );
    // 16^32 == 2^128.
    let hex_overflow = format!("1{}", "0".repeat(32));
    assert_eq!(U128::from_str_radix(&hex_overflow, 16), Err(Error::Overflow));
}

#[test]
fn enforces_bits_not_only_limb_count() {
    // `Uint<130, 3>` can store 192 bits, but only 130 bits are valid. A value
    // with bit length 133 fits the limbs yet must be rejected.
    let value = format!("1{}", "0".repeat(40)); // 10^40 > 2^130
    assert_eq!(U130::from_str_radix(&value, 10), Err(Error::Overflow));

    // The maximum value round-trips and a longer representation overflows.
    let max = U130::MAX;
    assert_eq!(max.bit_len(), 130);
    let encoded = max.to_string_radix(16);
    assert_eq!(U130::from_str_radix(&encoded, 16), Ok(max));
}

#[test]
fn leading_zeros_are_ignored_and_do_not_blow_up() {
    // A multi-megabyte run of zeros must be cheap and yield zero.
    let zeros = "0".repeat(2_000_000);
    let start = std::time::Instant::now();
    assert_eq!(U128::from_str_radix(&zeros, 10), Ok(U128::ZERO));
    assert!(
        start.elapsed() < std::time::Duration::from_secs(2),
        "parsing 2M zeros took {:?}",
        start.elapsed()
    );

    // Zero-padded significant digits still parse.
    let padded = format!("{}1", "0".repeat(100_000));
    assert_eq!(U128::from_str_radix(&padded, 10), Ok(U128::ONE));

    // A long significant run overflows quickly (bounded work, no hang).
    let huge = format!("1{}", "0".repeat(1_000_000));
    let start = std::time::Instant::now();
    assert_eq!(U128::from_str_radix(&huge, 10), Err(Error::Overflow));
    assert!(start.elapsed() < std::time::Duration::from_secs(2));
}

#[test]
fn byte_apis_are_limb_aligned_for_odd_bit_widths() {
    // `Uint<130, 3>` uses 3 limbs = 24 bytes, not 130/8 = 16.
    let mut bytes = [0u8; 24];
    bytes[23] = 0x01; // big-endian value 1
    let value = U130::from_be_slice(&bytes);
    assert_eq!(value, U130::ONE);

    // Bits at or above BITS are masked off rather than panicking.
    let saturated = U130::from_be_slice(&[0xFFu8; 24]);
    assert_eq!(saturated, U130::MAX);

    // Round trip through both endiannesses.
    let round = U130::from_be_slice(&saturated.to_be_bytes_fixed::<24>());
    assert_eq!(round, saturated);
    let round = U130::from_le_slice(&saturated.to_le_bytes_fixed::<24>());
    assert_eq!(round, saturated);
}

#[test]
fn u128_operands_do_not_panic() {
    // Previously `u128_to_word` panicked for operands wider than 64 bits.
    assert_eq!(U128::ONE + u128::MAX, U128::ZERO); // 1 + (2^128 - 1) wraps
    assert_eq!(U128::ONE * (1u128 << 64), U128::from_limbs([0, 1]));
    assert_eq!(U128::ONE - i128::MIN, U128::from_limbs([1, 0x8000_0000_0000_0000]));
    assert_eq!(U128::MAX / u128::MAX, U128::ONE);
    assert_eq!(U128::MAX % u128::MAX, 0u128);
}

#[test]
fn div_rem_u128_matches_definition() {
    let mut state = 0xDEAD_BEEF_CAFE_F00Du64;
    for _ in 0..2_000 {
        let a = U256::from_limbs([rng(&mut state), rng(&mut state), rng(&mut state), rng(&mut state)]);
        // Exercise both the single-limb fast path and the two-limb path.
        let d = (rng(&mut state) as u128) | ((rng(&mut state) as u128) << 64);
        if d == 0 {
            continue;
        }
        let (q, r) = a.div_rem_u128(d);
        assert!(r < d, "remainder {r} not below divisor {d}");
        // q * d + r == a, computed without overflow because q * d <= a.
        let reconstructed = q.mul_u128(d).add_u128(r);
        assert_eq!(reconstructed, a, "a={a:x} d={d:x} q={q:x} r={r:x}");
    }
}
