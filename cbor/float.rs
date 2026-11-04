//! IEEE 754 half-precision conversion helpers.
//!
//! These functions avoid a dependency on the `half` crate while remaining
//! exact: `f16_to_f64` is lossless, and `f64_to_f16` rounds to nearest, ties
//! to even. NaN values are canonicalized to the positive quiet NaN
//! `0x7e00` by `f64_to_f16`.

/// Convert the bits of an IEEE 754 binary16 value to an `f64`.
#[inline]
fn pow2(exp: i32) -> f64 {
    f64::from_bits(((1023 + exp) as u64) << 52)
}

#[inline]
pub fn f16_to_f64(bits: u16) -> f64 {
    let sign = if bits & 0x8000 == 0 { 1.0 } else { -1.0 };
    let exponent = i32::from((bits >> 10) & 0x1f);
    let fraction = f64::from(bits & 0x03ff);
    match exponent {
        0 => sign * fraction * pow2(-24),
        31 if fraction == 0.0 => sign * f64::INFINITY,
        31 => f64::NAN,
        _ => sign * (1.0 + fraction / 1024.0) * pow2(exponent - 15),
    }
}

/// Convert an `f64` to the bits of an IEEE 754 binary16 value.
///
/// The conversion rounds to nearest, ties to even. Values outside the binary16
/// range become infinities; values that are too small become zero. All NaNs
/// are canonicalized to `0x7e00`.
#[inline]
pub fn f64_to_f16(value: f64) -> u16 {
    let bits = value.to_bits();
    let sign = ((bits >> 63) & 1) as u16;
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let mantissa = bits & 0x000f_ffff_ffff_ffff;

    if exponent == 0x7ff {
        if mantissa == 0 {
            return (sign << 15) | 0x7c00;
        }
        // Canonical positive quiet NaN. A sign or payload would make the
        // encoding non-deterministic across implementations.
        return 0x7e00;
    }

    if exponent == 0 {
        // f64 subnormals are far below the binary16 minimum; they round to
        // zero.
        return sign << 15;
    }

    let exp = exponent - 1023;
    let sig = (1u64 << 52) | mantissa;

    if exp > 15 {
        return (sign << 15) | 0x7c00;
    }

    if exp >= -14 {
        // Normal binary16.
        const SHIFT: u32 = 42;
        let mut m = (sig >> SHIFT) as u32;
        let rem = sig & ((1u64 << SHIFT) - 1);
        let half = 1u64 << (SHIFT - 1);
        if rem > half || (rem == half && (m & 1) == 1) {
            m += 1;
        }
        let mut exp = exp;
        if m == 0x0800 {
            m = 0x0400;
            exp += 1;
            if exp > 15 {
                return (sign << 15) | 0x7c00;
            }
        }
        return (sign << 15) | (((exp + 15) as u16) << 10) | ((m as u16) & 0x03ff);
    }

    // Subnormal binary16 or zero. The value is sig * 2^(exp - 52), and the
    // binary16 subnormal unit is 2^-24, so the result is sig >> (28 - exp).
    let shift = 28 - exp;
    if shift >= 64 {
        return sign << 15;
    }
    let shift = shift as u32;
    let mut m = (sig >> shift) as u32;
    let rem = sig & ((1u64 << shift) - 1);
    let half = 1u64 << (shift - 1);
    if rem > half || (rem == half && (m & 1) == 1) {
        m += 1;
    }
    if m >= 0x0400 {
        // Rounded up into the smallest normal.
        return (sign << 15) | 0x0400;
    }
    (sign << 15) | m as u16
}

/// Return the shortest float encoding that round-trips exactly.
///
/// The returned value is one of `(0xf9, u16)`, `(0xfa, u32)` or `(0xfb, u64)`.
#[inline]
pub fn shortest_f64(value: f64) -> (u8, u64) {
    if value.is_nan() {
        return (0xf9, 0x7e00);
    }
    let bits16 = f64_to_f16(value);
    if f16_to_f64(bits16).to_bits() == value.to_bits() {
        return (0xf9, u64::from(bits16));
    }
    let bits32 = value as f32;
    if f64::from(bits32).to_bits() == value.to_bits() {
        return (0xfa, u64::from(bits32.to_bits()));
    }
    (0xfb, value.to_bits())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values() {
        let cases: &[(u16, f64)] = &[
            (0x0000, 0.0),
            (0x8000, -0.0),
            (0x3c00, 1.0),
            (0x3e00, 1.5),
            (0x7bff, 65504.0),
            (0x0001, 5.960_464_477_539_063e-8),
            (0x0400, 6.103_515_625e-5),
            (0x7c00, f64::INFINITY),
            (0xfc00, f64::NEG_INFINITY),
        ];
        for &(bits, value) in cases {
            assert_eq!(f16_to_f64(bits).to_bits(), value.to_bits(), "bits {bits:#06x}");
        }
    }

    #[test]
    fn every_f16_roundtrips() {
        for bits in 0u16..=0xffff {
            let value = f16_to_f64(bits);
            if value.is_nan() {
                assert_eq!(f64_to_f16(value), 0x7e00);
            } else {
                assert_eq!(f64_to_f16(value), bits, "bits {bits:#06x} value {value}");
            }
        }
    }

    #[test]
    fn rounds_to_nearest_even() {
        // 2049.0 is exactly between 2048 and 2050 in binary16; the even
        // mantissa is 2048.
        assert_eq!(f16_to_f64(f64_to_f16(2049.0)), 2048.0);
        // 2051.0 rounds to 2052.0 (the even mantissa).
        assert_eq!(f16_to_f64(f64_to_f16(2051.0)), 2052.0);
        // 2^-25 exactly ties with zero, so it rounds to zero.
        assert_eq!(f64_to_f16(pow2(-25)), 0);
        // Just above 2^-25 rounds to the smallest subnormal.
        assert_eq!(f64_to_f16(pow2(-25) * 1.5), 1);
    }

    #[test]
    fn shortest_uses_the_smallest_form() {
        assert_eq!(shortest_f64(1.5), (0xf9, 0x3e00));
        assert_eq!(shortest_f64(100000.0), (0xfa, 0x47c35000));
        assert_eq!(shortest_f64(1.0e300), (0xfb, 1.0e300f64.to_bits()));
        assert_eq!(shortest_f64(f64::NAN), (0xf9, 0x7e00));
    }
}
