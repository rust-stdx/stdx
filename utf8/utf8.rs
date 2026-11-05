#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Fast UTF-8 validation and safe `&str` construction.
//!
//! This crate provides two operations:
//!
//! - [`validate`] returns `true` if a byte slice is valid UTF-8. It never
//!   allocates and does not construct a `&str`.
//! - [`from_utf8`] is a safe, drop-in replacement for
//!   [`core::str::from_utf8`]. On success it returns a `&str`; on failure it
//!   returns the same [`Utf8Error`] that the standard library would return.
//!
//! On supported targets, long ASCII inputs are checked with SIMD instructions
//! before the `&str` is constructed with
//! [`core::str::from_utf8_unchecked`]. If a block contains a non-ASCII byte,
//! the standard library validates the input instead. Inputs shorter than 64
//! bytes and targets without a SIMD kernel also use
//! [`core::str::from_utf8`] directly.
//!
//! # Supported acceleration
//!
//! | Target | Kernel |
//! |--------|--------|
//! | `aarch64` | NEON ASCII fast path |
//! | `x86_64` | AVX2 ASCII fast path |
//! | `wasm32` with `simd128` | WASM SIMD128 ASCII fast path |
//!
//! There is deliberately no SSE kernel. Targets without one of the kernels
//! above use the standard library implementation. Non-ASCII input is always
//! validated by the standard library in this version.
//!
//! With the default `std` feature, the fastest available kernel is selected at
//! runtime. Without `std` (`default-features = false`), the kernel is selected
//! at compile time from the enabled target features.
//!
//! # Safety
//!
//! This crate contains unsafe code in two places only:
//!
//! 1. SIMD intrinsic calls inside functions with the corresponding
//!    `#[target_feature]` attribute.
//! 2. The single `core::str::from_utf8_unchecked` call in [`from_utf8`], made
//!    only after [`validate`] has returned `true`.
//!
//! The public API is safe. A false positive from [`validate`] would be a
//! soundness bug; the SIMD kernels are tested against the standard library
//! over exhaustive, boundary, random, and fuzzed inputs.
//!
//! # Errors
//!
//! [`from_utf8`] returns the same error as [`core::str::from_utf8`]. Invalid
//! input is revalidated by the standard library so that `valid_up_to` and
//! `error_len` are exact.
//!
//! # Examples
//!
//! ```
//! assert!(utf8::validate("hello".as_bytes()));
//! assert!(!utf8::validate(&[0xff]));
//!
//! assert_eq!(utf8::from_utf8("héllo".as_bytes()).unwrap(), "héllo");
//! assert!(utf8::from_utf8(&[0xff]).is_err());
//! ```

pub use core::str::Utf8Error;

#[cfg(test)]
extern crate std;

#[cfg(target_arch = "aarch64")]
mod utf8_neon;

#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
mod utf8_avx2;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod utf8_wasm_simd128;

/// Inputs shorter than this use the standard library implementation.
///
/// The standard library has a good word-at-a-time ASCII fast path. Below this
/// size, SIMD setup and dispatch cost more than they save.
const SIMD_THRESHOLD: usize = 64;

/// Returns `true` if `bytes` is valid UTF-8.
///
/// This never allocates and never constructs a `&str`.
///
/// # Examples
///
/// ```
/// assert!(utf8::validate(b"abc"));
/// assert!(!utf8::validate(&[0x80]));
/// ```
#[inline]
pub fn validate(bytes: &[u8]) -> bool {
    if bytes.len() < SIMD_THRESHOLD {
        return scalar_validate(bytes);
    }
    validate_impl(bytes)
}

/// Converts a byte slice to a `&str`, validating it as UTF-8.
///
/// This is a safe replacement for [`core::str::from_utf8`]. On valid input it
/// may use a SIMD kernel and then construct the string once without a second
/// validation pass. On invalid input it delegates to the standard library so
/// the returned [`Utf8Error`] is exact.
///
/// # Errors
///
/// Returns [`Utf8Error`] if `bytes` is not valid UTF-8. The error values match
/// [`core::str::from_utf8`] exactly.
///
/// # Examples
///
/// ```
/// assert_eq!(utf8::from_utf8(b"abc").unwrap(), "abc");
///
/// let error = utf8::from_utf8(&[b'a', 0xff]).unwrap_err();
/// assert_eq!(error.valid_up_to(), 1);
/// ```
#[inline]
#[allow(unsafe_code)]
pub fn from_utf8(bytes: &[u8]) -> Result<&str, Utf8Error> {
    if bytes.len() < SIMD_THRESHOLD {
        return core::str::from_utf8(bytes);
    }
    if validate_impl(bytes) {
        // SAFETY: `validate_impl` returned `true`, so the bytes are valid
        // UTF-8, which is the only requirement of `from_utf8_unchecked`.
        Ok(unsafe { core::str::from_utf8_unchecked(bytes) })
    } else {
        // Revalidate invalid input so the caller gets std's exact error
        // position and error length.
        core::str::from_utf8(bytes)
    }
}

#[inline]
fn scalar_validate(bytes: &[u8]) -> bool {
    core::str::from_utf8(bytes).is_ok()
}

#[inline]
#[allow(unreachable_code)]
#[allow(unsafe_code)]
fn validate_impl(bytes: &[u8]) -> bool {
    #[cfg(all(feature = "std", target_arch = "x86_64"))]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: the runtime check above guarantees AVX2.
            return unsafe { utf8_avx2::validate(bytes) };
        }
    }

    #[cfg(all(not(feature = "std"), target_arch = "x86_64", target_feature = "avx2"))]
    {
        // SAFETY: the compile-time target feature guarantees AVX2.
        return unsafe { utf8_avx2::validate(bytes) };
    }

    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(feature = "std")]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                // SAFETY: the runtime check above guarantees NEON.
                return unsafe { utf8_neon::validate(bytes) };
            }
        }

        #[cfg(not(feature = "std"))]
        {
            // SAFETY: NEON is mandatory on AArch64.
            return unsafe { utf8_neon::validate(bytes) };
        }
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        // SAFETY: the compile-time target feature guarantees SIMD128.
        return unsafe { utf8_wasm_simd128::validate(bytes) };
    }

    scalar_validate(bytes)
}

#[cfg(test)]
mod tests {
    use std::{vec, vec::Vec};

    use super::*;

    fn assert_agrees(bytes: &[u8]) {
        let expected = core::str::from_utf8(bytes);
        assert_eq!(validate(bytes), expected.is_ok(), "validate disagreed for {bytes:02x?}");
        match (from_utf8(bytes), expected) {
            (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
            (Err(actual), Err(expected)) => {
                assert_eq!(actual.valid_up_to(), expected.valid_up_to(), "valid_up_to for {bytes:02x?}");
                assert_eq!(actual.error_len(), expected.error_len(), "error_len for {bytes:02x?}");
            }
            (actual, expected) => panic!("from_utf8 disagreed for {bytes:02x?}: {actual:?} vs {expected:?}"),
        }
    }

    #[test]
    fn empty_input_is_valid() {
        assert!(validate(b""));
        assert_eq!(from_utf8(b"").unwrap(), "");
    }

    #[test]
    fn one_byte_exhaustive() {
        for byte in 0..=u8::MAX {
            assert_agrees(&[byte]);
        }
    }

    #[test]
    fn two_byte_exhaustive() {
        for first in 0..=u8::MAX {
            for second in 0..=u8::MAX {
                assert_agrees(&[first, second]);
            }
        }
    }

    #[test]
    fn three_byte_exhaustive() {
        for first in 0xe0..=0xef {
            for second in 0..=u8::MAX {
                for third in 0..=u8::MAX {
                    assert_agrees(&[first, second, third]);
                }
            }
        }
    }

    #[test]
    fn four_byte_targeted() {
        for first in 0xf0..=0xf4 {
            for second in 0x80..=0xbf {
                for third in 0x80..=0xbf {
                    for fourth in 0x80..=0xbf {
                        assert_agrees(&[first, second, third, fourth]);
                    }
                }
            }
            // Exercise the rejected boundaries around the continuation ranges.
            for second in [0x7f, 0x80, 0x8f, 0x90, 0xbf, 0xc0] {
                for third in [0x7f, 0x80, 0xbf, 0xc0] {
                    for fourth in [0x7f, 0x80, 0xbf, 0xc0] {
                        assert_agrees(&[first, second, third, fourth]);
                    }
                }
            }
        }
    }

    #[test]
    fn all_unicode_scalar_values() {
        for codepoint in 0..=0x10ffff {
            let Some(character) = char::from_u32(codepoint) else {
                continue;
            };
            let mut buffer = [0u8; 4];
            let encoded = character.encode_utf8(&mut buffer);
            assert!(validate(encoded.as_bytes()), "codepoint U+{codepoint:04X}");
            assert_eq!(from_utf8(encoded.as_bytes()).unwrap(), encoded);
        }
    }

    #[test]
    fn invalid_sequences() {
        let cases: &[&[u8]] = &[
            &[0x80],
            &[0xbf],
            &[0xc0, 0x80],             // overlong NUL
            &[0xc1, 0xbf],             // overlong
            &[0xe0, 0x80, 0x80],       // overlong
            &[0xe0, 0x9f, 0xbf],       // overlong
            &[0xed, 0xa0, 0x80],       // surrogate
            &[0xed, 0xbf, 0xbf],       // surrogate
            &[0xf0, 0x80, 0x80, 0x80], // overlong
            &[0xf0, 0x8f, 0xbf, 0xbf], // overlong
            &[0xf4, 0x90, 0x80, 0x80], // > U+10FFFF
            &[0xf5, 0x80, 0x80, 0x80], // invalid lead
            &[0xfe],
            &[0xff],
            &[0xc2],
            &[0xe2, 0x82],
            &[0xf0, 0x9f, 0x92],
            &[0xc2, b'a'],
            &[0xe2, 0x82, b'a'],
            &[0xf0, 0x9f, 0x92, b'a'],
            b"hello\xffworld",
        ];
        for bytes in cases {
            assert_agrees(bytes);
            assert!(!validate(bytes));
        }
    }

    #[test]
    fn valid_sequences() {
        let cases: &[&[u8]] = &[
            b"",
            b"hello",
            "é".as_bytes(),
            "€".as_bytes(),
            "😀".as_bytes(),
            "aé€😀z".as_bytes(),
            "\u{10FFFF}".as_bytes(),
        ];
        for bytes in cases {
            assert_agrees(bytes);
            assert!(validate(bytes));
        }
    }

    #[test]
    fn sequences_across_block_boundaries() {
        let characters = ['a', 'é', '€', '😀', '\u{10FFFF}'];
        for character in characters {
            let mut buffer = [0u8; 4];
            let encoded = character.encode_utf8(&mut buffer).as_bytes();
            for prefix in 0..=64 {
                for suffix in 0..=64 {
                    let mut bytes = vec![b'x'; prefix];
                    bytes.extend_from_slice(encoded);
                    bytes.extend(core::iter::repeat_n(b'y', suffix));
                    assert_agrees(&bytes);
                }
            }
            for cut in 1..encoded.len() {
                let mut bytes = vec![b'x'; 63];
                bytes.extend_from_slice(&encoded[..cut]);
                assert!(!validate(&bytes), "truncated {:?} at {cut}", character);
                assert_agrees(&bytes);
            }
        }
    }

    #[test]
    fn threshold_boundary() {
        for length in 0..=128 {
            let ascii = vec![b'a'; length];
            assert!(validate(&ascii));
            assert_eq!(from_utf8(&ascii).unwrap().len(), length);

            let mut mixed = ascii;
            mixed.push(0xc3);
            mixed.push(0xa9);
            assert!(validate(&mixed));
            assert_eq!(from_utf8(&mixed).unwrap().len(), mixed.len());

            let mut invalid = vec![b'a'; length];
            invalid.push(0xff);
            assert!(!validate(&invalid));
            assert!(from_utf8(&invalid).is_err());
        }
    }

    #[test]
    fn random_agreement() {
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };

        for _ in 0..200_000 {
            let length = (next() as usize) % 129;
            let mut bytes = Vec::with_capacity(length);
            for _ in 0..length {
                bytes.push(next() as u8);
            }
            assert_agrees(&bytes);
        }
    }
}
