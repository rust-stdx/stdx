#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Fast base32 encoding and decoding with SIMD acceleration, constant-time
//! operations, and `const fn` support.
//!
//! Ten alphabet variants are available via [`Alphabet`]:
//!
//! | Variant               | Characters              | Padding | Description                |
//! |-----------------------|-------------------------|---------|----------------------------|
//! | `Rfc4648`             | `A-Z 2-7`               | `=`     | RFC 4648 (standard)        |
//! | `Rfc4648NoPadding`    | `A-Z 2-7`               | none    | RFC 4648 without padding   |
//! | `Rfc4648Lower`        | `a-z 2-7`               | `=`     | RFC 4648 lowercase         |
//! | `Rfc4648LowerNoPadding`| `a-z 2-7`              | none    | RFC 4648 lowercase no pad  |
//! | `Rfc4648Hex`          | `0-9 A-V`               | `=`     | RFC 4648 extended hex      |
//! | `Rfc4648HexNoPadding` | `0-9 A-V`               | none    | RFC 4648 extended hex no pad|
//! | `Rfc4648HexLower`     | `0-9 a-v`               | `=`     | RFC 4648 extended hex lower|
//! | `Rfc4648HexLowerNoPadding`| `0-9 a-v`           | none    | RFC 4648 extended hex lower no pad|
//! | `Crockford`           | `0-9 A-H J-K M-N P-Z`   | none    | Crockford (no I L O U)     |
//! | `Z32`                 | `ybndrfg8ejkmcpqxot1uwisza345h769` | none | Z-base-32 (zooko) |
//!
//! # Feature flags
//!
//! | Flag    | Description                                             |
//! |---------|---------------------------------------------------------|
//! | `std`   | [`std::error::Error`] trait impls (enabled by default)  |
//! | `alloc` | `String`/`Vec`-returning convenience APIs               |
//! | `serde` | Serde [`serialize`](crate::serde::serialize)/[`deserialize`](crate::serde::deserialize) helpers  |
//!
//! # Performance
//!
//! The [`encode_into`] and [`decode_into`] functions
//! automatically dispatch to SIMD-accelerated paths (AVX2 on x86/x86_64,
//! NEON on aarch64). When a constant-time guarantee is required, use
//! [`encode_into_constant_time`] or [`decode_into_constant_time`].
//!
//! # `const fn` support
//!
//! [`encode_array`] and [`decode_array`] are `const fn`, enabling base32
//! encoding and decoding at compile time.
//!
//! # Examples
//!
//! ```rust
//! let encoded = base32::encode(b"hello", base32::Alphabet::Rfc4648);
//! assert_eq!(encoded, "NBSWY3DP");
//!
//! let decoded = base32::decode(b"NBSWY3DP", base32::Alphabet::Rfc4648).unwrap();
//! assert_eq!(decoded, b"hello");
//!
//! let url = base32::encode(b"hello", base32::Alphabet::Crockford);
//! assert_eq!(url, "D1JPRV3F");
//!
//! let z32 = base32::encode(b"hello", base32::Alphabet::Z32);
//! assert_eq!(z32, "pb1sa5dx");
//! ```

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(all(feature = "serde", any(feature = "alloc", test)))]
mod serde;

#[cfg(target_arch = "aarch64")]
mod base32_neon;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod base32_avx2;

const PAD: u8 = b'=';

const Z32_DECODE_TABLE: [u8; 256] = [
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x12, 0x20, 0x19, 0x1a, 0x1b, 0x1e, 0x1d, 0x07,
    0x1f, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x18, 0x01, 0x0c, 0x03, 0x08, 0x05, 0x06, 0x1c, 0x15, 0x09, 0x0a, 0x20, 0x0b, 0x02, 0x10, 0x0d, 0x0e,
    0x04, 0x16, 0x11, 0x13, 0x20, 0x14, 0x0f, 0x00, 0x17, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
];

const Z32_ENCODE_TABLE: [u8; 32] = *b"ybndrfg8ejkmcpqxot1uwisza345h769";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alphabet {
    Crockford,
    Rfc4648,
    Rfc4648NoPadding,
    Rfc4648Lower,
    Rfc4648LowerNoPadding,
    Rfc4648Hex,
    Rfc4648HexNoPadding,
    Rfc4648HexLower,
    Rfc4648HexLowerNoPadding,
    Z32,
}

impl Alphabet {
    #[inline]
    const fn is_padded(&self) -> bool {
        match self {
            Alphabet::Crockford => false,
            Alphabet::Rfc4648 => true,
            Alphabet::Rfc4648NoPadding => false,
            Alphabet::Rfc4648Lower => true,
            Alphabet::Rfc4648LowerNoPadding => false,
            Alphabet::Rfc4648Hex => true,
            Alphabet::Rfc4648HexNoPadding => false,
            Alphabet::Rfc4648HexLower => true,
            Alphabet::Rfc4648HexLowerNoPadding => false,
            Alphabet::Z32 => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    InvalidOutputLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidInput,
    InvalidLength,
    InvalidPadding,
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidOutputLength => f.write_str("output buffer size is not valid"),
        }
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInput => f.write_str("invalid base32 character"),
            Self::InvalidLength => f.write_str("invalid base32 length"),
            Self::InvalidPadding => f.write_str("invalid base32 padding"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EncodeError {}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// Returns the size in bytes of the input data after base32 encoding.
///
/// Returns `None` if the output size overflows `usize`.
///
/// # Example
///
/// ```rust
/// assert_eq!(base32::encoded_length(5, true), Some(8));
/// assert_eq!(base32::encoded_length(1, false), Some(2));
/// assert_eq!(base32::encoded_length(usize::MAX, true), None);
/// ```
pub const fn encoded_length(bytes_len: usize, padding: bool) -> Option<usize> {
    if bytes_len == 0 {
        return Some(0);
    }
    let complete_chunks = bytes_len / 5;
    let base = match complete_chunks.checked_mul(8) {
        Some(v) => v,
        None => return None,
    };
    let rem = bytes_len % 5;
    if rem == 0 {
        Some(base)
    } else if padding {
        base.checked_add(8)
    } else {
        let bits = match bytes_len.checked_mul(8) {
            Some(v) => v,
            None => return None,
        };
        match bits.checked_add(4) {
            Some(v) => Some(v / 5),
            None => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Encode
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Encodes bytes to a base32 string using the given [`Alphabet`].
///
/// # Example
///
/// ```rust
/// let encoded = base32::encode(b"hello", base32::Alphabet::Rfc4648);
/// assert_eq!(encoded, "NBSWY3DP");
/// ```
#[cfg(feature = "alloc")]
pub fn encode(data: impl AsRef<[u8]>, alphabet: Alphabet) -> alloc::string::String {
    let data = data.as_ref();
    let padding = alphabet.is_padded();
    let len = encoded_length(data.len(), padding).expect("encoded length overflow");
    let mut output = alloc::vec![0u8; len];
    encode_into(&mut output, data, alphabet).expect("output buffer sized correctly");
    unsafe { alloc::string::String::from_utf8_unchecked(output) }
}

/// Encodes `data` into a fixed-size array at compile time.
///
/// The generic parameter `OUT` is the output array length. It must be exactly
/// the encoded length of `data` or a compile-time panic is raised.
///
/// # Example
///
/// ```rust
/// const DATA: [u8; 5] = [0x68, 0x65, 0x6C, 0x6C, 0x6F];
/// const B32: [u8; 8] = base32::encode_array::<8>(&DATA, base32::Alphabet::Rfc4648);
/// assert_eq!(&B32, b"NBSWY3DP");
/// ```
pub const fn encode_array<const OUT: usize>(data: &[u8], alphabet: Alphabet) -> [u8; OUT] {
    match encoded_length(data.len(), alphabet.is_padded()) {
        Some(len) if len == OUT => {}
        _ => panic!("encode_array: output array length is invalid"),
    }
    let mut result = [0u8; OUT];
    match encode_into_constant_time(&mut result, data, alphabet) {
        Ok(()) => result,
        Err(_) => panic!("encode_array: output array length is invalid"),
    }
}

/// Encodes bytes into an existing buffer.
///
/// Dispatches to a SIMD-accelerated implementation (AVX2 or NEON) when
/// the target feature is available.
///
/// See [`encode_into_constant_time`] for security-sensitive and cryptographic operations.
///
/// # Errors
///
/// Returns [`EncodeError`] if `output.len()` is less than the expected encoded
/// length.
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 8];
/// base32::encode_into(&mut buf, b"hello", base32::Alphabet::Rfc4648).unwrap();
/// assert_eq!(&buf, b"NBSWY3DP");
/// ```
pub fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    let padding = alphabet.is_padded();
    let expected = encoded_length(data.len(), padding).expect("encoded length overflow");
    if output.len() < expected {
        return Err(EncodeError::InvalidOutputLength);
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    if data.len() >= 40 {
        return unsafe { base32_neon::encode_into(output, data, alphabet) };
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    if data.len() >= 40 {
        return unsafe { base32_avx2::encode_into(output, data, alphabet) };
    }

    encode_into_constant_time(output, data, alphabet)
}

/// Constant-time base32 encoding. Processes all input data without
/// secret-dependent branches or memory accesses, making it suitable
/// for cryptographic applications.
///
/// Consumers may prefer the faster [`encode_into`] which dispatches to
/// a SIMD-accelerated path when available (non constant-time).
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 8];
/// base32::encode_into_constant_time(&mut buf, b"hello", base32::Alphabet::Rfc4648).unwrap();
/// assert_eq!(&buf, b"NBSWY3DP");
/// ```
pub const fn encode_into_constant_time(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    let padding = alphabet.is_padded();
    let expected = encoded_length(data.len(), padding).expect("encoded length overflow");
    if output.len() < expected {
        return Err(EncodeError::InvalidOutputLength);
    }

    let len = data.len();
    let mut i = 0;

    while i + 40 <= len {
        encode_8blocks(output, alphabet, data, i);
        i += 40;
    }

    while i + 5 <= len {
        let b0 = data[i];
        let b1 = data[i + 1];
        let b2 = data[i + 2];
        let b3 = data[i + 3];
        let b4 = data[i + 4];

        let q0 = b0 >> 3;
        let q1 = ((b0 & 0x07) << 2) | (b1 >> 6);
        let q2 = (b1 >> 1) & 0x1F;
        let q3 = ((b1 & 0x01) << 4) | (b2 >> 4);
        let q4 = ((b2 & 0x0F) << 1) | (b3 >> 7);
        let q5 = (b3 >> 2) & 0x1F;
        let q6 = ((b3 & 0x03) << 3) | (b4 >> 5);
        let q7 = b4 & 0x1F;

        let o = (i / 5) * 8;
        output[o] = quintet_to_char(q0, alphabet);
        output[o + 1] = quintet_to_char(q1, alphabet);
        output[o + 2] = quintet_to_char(q2, alphabet);
        output[o + 3] = quintet_to_char(q3, alphabet);
        output[o + 4] = quintet_to_char(q4, alphabet);
        output[o + 5] = quintet_to_char(q5, alphabet);
        output[o + 6] = quintet_to_char(q6, alphabet);
        output[o + 7] = quintet_to_char(q7, alphabet);

        i += 5;
    }

    let rem = len - i;
    if rem > 0 {
        let o = (i / 5) * 8;
        let b0 = data[i];
        let q0 = b0 >> 3;
        let q1 = (b0 & 0x07) << 2;
        output[o] = quintet_to_char(q0, alphabet);
        output[o + 1] = quintet_to_char(q1, alphabet);

        if rem >= 2 {
            let b1 = data[i + 1];
            let q1 = ((b0 & 0x07) << 2) | (b1 >> 6);
            let q2 = (b1 >> 1) & 0x1F;
            let q3 = (b1 & 0x01) << 4;
            output[o + 1] = quintet_to_char(q1, alphabet);
            output[o + 2] = quintet_to_char(q2, alphabet);
            output[o + 3] = quintet_to_char(q3, alphabet);

            if rem >= 3 {
                let b2 = data[i + 2];
                let q3 = ((b1 & 0x01) << 4) | (b2 >> 4);
                let q4 = (b2 & 0x0F) << 1;
                output[o + 3] = quintet_to_char(q3, alphabet);
                output[o + 4] = quintet_to_char(q4, alphabet);

                if rem == 4 {
                    let b3 = data[i + 3];
                    let q4 = ((b2 & 0x0F) << 1) | (b3 >> 7);
                    let q5 = (b3 >> 2) & 0x1F;
                    let q6 = (b3 & 0x03) << 3;
                    output[o + 4] = quintet_to_char(q4, alphabet);
                    output[o + 5] = quintet_to_char(q5, alphabet);
                    output[o + 6] = quintet_to_char(q6, alphabet);
                }
            }
        }

        if padding {
            let pad_start = match rem {
                1 => o + 2,
                2 => o + 4,
                3 => o + 5,
                4 => o + 7,
                _ => unreachable!(),
            };
            let pad_end = o + 8;
            let mut p = pad_start;
            while p < pad_end {
                output[p] = PAD;
                p += 1;
            }
        }
    }
    Ok(())
}

/// Appends the base32-encoded representation of `data` to a [`String`].
///
/// # Example
///
/// ```rust
/// let mut s = String::from("tag: ");
/// base32::encode_into_string(&mut s, b"hello", base32::Alphabet::Rfc4648);
/// assert_eq!(s, "tag: NBSWY3DP");
/// ```
#[cfg(feature = "alloc")]
pub fn encode_into_string(output: &mut alloc::string::String, data: &[u8], alphabet: Alphabet) {
    let encoded_length = encoded_length(data.len(), alphabet.is_padded()).expect("output length overflow");
    if encoded_length <= 256 {
        // zero-alloc version for small data
        let mut buf = [0u8; 256];
        let mut buf = &mut buf[..encoded_length];
        encode_into(&mut buf, data, alphabet).unwrap();
        // SAFETY: base64 only produces ASCII characters, which are valid UTF-8.
        output.push_str(unsafe { core::str::from_utf8_unchecked(&buf) });
    } else {
        let mut buf = alloc::vec![0u8; encoded_length];
        encode_into(&mut buf, data, alphabet).unwrap();
        // SAFETY: base64 only produces ASCII characters, which are valid UTF-8.
        output.push_str(unsafe { core::str::from_utf8_unchecked(&buf) });
    }
}

/// Returns 0x00 if lo <= v <= hi, 0xFF otherwise.
/// Uses sign-bit propagation for branchless range checking.
#[inline]
const fn not_in_range(v: u8, lo: u8, hi: u8) -> u8 {
    (((v.wrapping_sub(lo) as i8) | (hi.wrapping_sub(v) as i8)) >> 7) as u8
}

/// Returns 0x20 if the lower `max_pad` bits of `value` are non-zero (invalid trailing bits),
/// 0x00 otherwise. Used for branchless non-canonical encoding rejection.
#[inline]
const fn check_trailing_bits(value: u8, max_pad: u8) -> u8 {
    let mask = (1u8 << max_pad).wrapping_sub(1);
    let pad_bits = value & mask;
    (!not_in_range(pad_bits, 1, mask)) & 0x20
}

#[inline]
const fn encode_8blocks(output: &mut [u8], alphabet: Alphabet, data: &[u8], start: usize) {
    let mut n = 0;
    while n < 8 {
        let i = start + n * 5;
        let b0 = data[i];
        let b1 = data[i + 1];
        let b2 = data[i + 2];
        let b3 = data[i + 3];
        let b4 = data[i + 4];

        let q0 = b0 >> 3;
        let q1 = ((b0 & 0x07) << 2) | (b1 >> 6);
        let q2 = (b1 >> 1) & 0x1F;
        let q3 = ((b1 & 0x01) << 4) | (b2 >> 4);
        let q4 = ((b2 & 0x0F) << 1) | (b3 >> 7);
        let q5 = (b3 >> 2) & 0x1F;
        let q6 = ((b3 & 0x03) << 3) | (b4 >> 5);
        let q7 = b4 & 0x1F;

        let o = (start / 5) * 8 + n * 8;
        output[o] = quintet_to_char(q0, alphabet);
        output[o + 1] = quintet_to_char(q1, alphabet);
        output[o + 2] = quintet_to_char(q2, alphabet);
        output[o + 3] = quintet_to_char(q3, alphabet);
        output[o + 4] = quintet_to_char(q4, alphabet);
        output[o + 5] = quintet_to_char(q5, alphabet);
        output[o + 6] = quintet_to_char(q6, alphabet);
        output[o + 7] = quintet_to_char(q7, alphabet);

        n += 1;
    }
}

/// Constant-time mapping: 5-bit value (0-31) to base32 character.
/// No secret-dependent branches or memory accesses.
#[inline]
const fn quintet_to_char(v: u8, alphabet: Alphabet) -> u8 {
    match alphabet {
        Alphabet::Crockford => quintet_to_crockford(v),
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => {
            let not_upper = not_in_range(v, 0, 25);
            let not_digit = not_in_range(v, 26, 31);
            (v + b'A') & !not_upper | (v.wrapping_sub(26).wrapping_add(b'2')) & !not_digit
        }
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => {
            let not_lower = not_in_range(v, 0, 25);
            let not_digit = not_in_range(v, 26, 31);
            (v + b'a') & !not_lower | (v.wrapping_sub(26).wrapping_add(b'2')) & !not_digit
        }
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => {
            let not_digit = not_in_range(v, 0, 9);
            let not_upper = not_in_range(v, 10, 31);
            (v + b'0') & !not_digit | (v.wrapping_sub(10).wrapping_add(b'A')) & !not_upper
        }
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => {
            let not_digit = not_in_range(v, 0, 9);
            let not_lower = not_in_range(v, 10, 31);
            (v + b'0') & !not_digit | (v.wrapping_sub(10).wrapping_add(b'a')) & !not_lower
        }
        Alphabet::Z32 => Z32_ENCODE_TABLE[v as usize],
    }
}

/// Crockford quintet-to-character: 6 non-contiguous ranges.
#[inline]
const fn quintet_to_crockford(v: u8) -> u8 {
    let not_0_9 = not_in_range(v, 0, 9);
    let not_10_17 = not_in_range(v, 10, 17);
    let not_18_19 = not_in_range(v, 18, 19);
    let not_20_21 = not_in_range(v, 20, 21);
    let not_22_26 = not_in_range(v, 22, 26);
    let not_27_31 = not_in_range(v, 27, 31);
    (v + b'0') & !not_0_9
        | (v + 55) & !not_10_17
        | (v + 56) & !not_18_19
        | (v + 57) & !not_20_21
        | (v + 58) & !not_22_26
        | (v + 59) & !not_27_31
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Decode
////////////////////////////////////////////////////////////////////////////////////////////////////

#[inline]
const fn decoded_length(encoded_content_len: usize) -> Result<usize, DecodeError> {
    let full_blocks = encoded_content_len / 8;
    let rem = encoded_content_len % 8;

    let base = full_blocks * 5;

    match rem {
        0 => Ok(base),
        2 => Ok(base + 1),
        4 => Ok(base + 2),
        5 => Ok(base + 3),
        7 => Ok(base + 4),
        _ => Err(DecodeError::InvalidLength),
    }
}

/// Decodes a base32 string into bytes.
///
/// # Errors
///
/// Returns [`DecodeError`] if any character is invalid for the chosen
/// [`Alphabet`], the input length is not valid, or padding is incorrect.
///
/// # Example
///
/// ```rust
/// let decoded = base32::decode(b"NBSWY3DP", base32::Alphabet::Rfc4648).unwrap();
/// assert_eq!(decoded, b"hello");
/// ```
#[cfg(feature = "alloc")]
pub fn decode(data: impl AsRef<[u8]>, alphabet: Alphabet) -> Result<alloc::vec::Vec<u8>, DecodeError> {
    let data = data.as_ref();
    let padding = alphabet.is_padded();
    let (content_len, _) = strip_padding_info(data, padding)?;
    let output_len = decoded_length(content_len)?;
    let mut output = alloc::vec![0u8; output_len];
    decode_into(&mut output, data, alphabet)?;
    Ok(output)
}

/// Decodes a base32 string into a fixed-size array at compile time.
///
/// The generic parameter `OUT` is the output array length. It must be exactly
/// the decoded length of the input or an error is returned.
///
/// # Example
///
/// ```rust
/// const RESULT: Result<[u8; 5], base32::DecodeError> =
///     base32::decode_array::<5>(b"NBSWY3DP", base32::Alphabet::Rfc4648);
/// assert_eq!(RESULT.unwrap(), *b"hello");
/// ```
pub const fn decode_array<const OUT: usize>(encoded_data: &[u8], alphabet: Alphabet) -> Result<[u8; OUT], DecodeError> {
    let mut result = [0u8; OUT];
    match decode_into_constant_time(&mut result, encoded_data, alphabet) {
        Ok(()) => Ok(result),
        Err(err) => Err(err),
    }
}

/// Decodes a base32 string into an existing buffer.
///
/// Dispatches to a SIMD-accelerated implementation (AVX2 or NEON) when
/// the target feature is available.
///
/// See [`decode_into_constant_time`] for security-sensitive and cryptographic operations.
///
/// # Errors
///
/// Returns [`DecodeError`] if any character is invalid for the chosen
/// [`Alphabet`], if the input length is not valid, if padding is incorrect,
/// or if `output.len()` is too small to hold the decoded data.
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 5];
/// base32::decode_into(&mut buf, b"NBSWY3DP", base32::Alphabet::Rfc4648).unwrap();
/// assert_eq!(&buf, b"hello");
/// ```
pub fn decode_into(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let padding = alphabet.is_padded();
    let (content_len, _) = strip_padding_info(encoded_data, padding)?;
    let computed_output = decoded_length(content_len)?;
    if output.len() < computed_output {
        return Err(DecodeError::InvalidLength);
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    if content_len >= 64 {
        let content = &encoded_data[..content_len];
        return unsafe { base32_neon::decode_into(output, content, alphabet) };
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    if content_len >= 32 {
        let content = &encoded_data[..content_len];
        return unsafe { base32_avx2::decode_into(output, content, alphabet) };
    }

    decode_into_constant_time(output, encoded_data, alphabet)
}

/// Constant-time base32 decoding. Processes all input data without
/// secret-dependent branches or memory accesses, making it suitable
/// for cryptographic applications.
///
/// Consumers may prefer the faster [`decode_into`] which dispatches to
/// a SIMD-accelerated path when available (non constant-time).
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 5];
/// base32::decode_into_constant_time(&mut buf, b"NBSWY3DP", base32::Alphabet::Rfc4648).unwrap();
/// assert_eq!(&buf, b"hello");
/// ```
pub const fn decode_into_constant_time(
    output: &mut [u8],
    encoded_data: &[u8],
    alphabet: Alphabet,
) -> Result<(), DecodeError> {
    let in_len = encoded_data.len();
    let padding = alphabet.is_padded();

    if in_len == 0 {
        return Ok(());
    }

    let (content_len, _padding_len) = match strip_padding_info(encoded_data, padding) {
        Ok(info) => info,
        Err(e) => return Err(e),
    };

    if content_len == 0 {
        return Ok(());
    }

    let computed_output = match decoded_length(content_len) {
        Ok(len) => len,
        Err(e) => return Err(e),
    };

    if output.len() < computed_output {
        return Err(DecodeError::InvalidLength);
    }

    let mut err: u8 = 0;
    let mut i = 0;
    let mut o = 0;

    while i + 64 <= content_len {
        decode_8quads(output, alphabet, encoded_data, &mut i, &mut o, &mut err);
    }

    while i + 8 <= content_len {
        decode_1quad(output, alphabet, encoded_data, &mut i, &mut o, &mut err);
    }

    if i < content_len {
        let remaining = content_len - i;
        match remaining {
            2 => {
                let v0 = char_to_quintet(encoded_data[i], alphabet);
                let v1 = char_to_quintet(encoded_data[i + 1], alphabet);
                err |= v0 | v1;
                err |= check_trailing_bits(v1, 2);
                output[o] = (v0 << 3) | (v1 >> 2);
            }
            4 => {
                let v0 = char_to_quintet(encoded_data[i], alphabet);
                let v1 = char_to_quintet(encoded_data[i + 1], alphabet);
                let v2 = char_to_quintet(encoded_data[i + 2], alphabet);
                let v3 = char_to_quintet(encoded_data[i + 3], alphabet);
                err |= v0 | v1 | v2 | v3;
                err |= check_trailing_bits(v3, 4);
                output[o] = (v0 << 3) | (v1 >> 2);
                output[o + 1] = (v1.wrapping_shl(6)) | (v2 << 1) | (v3 >> 4);
            }
            5 => {
                let v0 = char_to_quintet(encoded_data[i], alphabet);
                let v1 = char_to_quintet(encoded_data[i + 1], alphabet);
                let v2 = char_to_quintet(encoded_data[i + 2], alphabet);
                let v3 = char_to_quintet(encoded_data[i + 3], alphabet);
                let v4 = char_to_quintet(encoded_data[i + 4], alphabet);
                err |= v0 | v1 | v2 | v3 | v4;
                err |= check_trailing_bits(v4, 1);
                output[o] = (v0 << 3) | (v1 >> 2);
                output[o + 1] = (v1.wrapping_shl(6)) | (v2 << 1) | (v3 >> 4);
                output[o + 2] = (v3.wrapping_shl(4)) | (v4 >> 1);
            }
            7 => {
                let v0 = char_to_quintet(encoded_data[i], alphabet);
                let v1 = char_to_quintet(encoded_data[i + 1], alphabet);
                let v2 = char_to_quintet(encoded_data[i + 2], alphabet);
                let v3 = char_to_quintet(encoded_data[i + 3], alphabet);
                let v4 = char_to_quintet(encoded_data[i + 4], alphabet);
                let v5 = char_to_quintet(encoded_data[i + 5], alphabet);
                let v6 = char_to_quintet(encoded_data[i + 6], alphabet);
                err |= v0 | v1 | v2 | v3 | v4 | v5 | v6;
                err |= check_trailing_bits(v6, 3);
                output[o] = (v0 << 3) | (v1 >> 2);
                output[o + 1] = (v1.wrapping_shl(6)) | (v2 << 1) | (v3 >> 4);
                output[o + 2] = (v3.wrapping_shl(4)) | (v4 >> 1);
                output[o + 3] = (v4.wrapping_shl(7)) | (v5 << 2) | (v6 >> 3);
            }
            _ => return Err(DecodeError::InvalidLength),
        }
    }

    if err >= 32 {
        return Err(DecodeError::InvalidInput);
    }

    Ok(())
}

#[inline]
const fn decode_1quad(output: &mut [u8], alphabet: Alphabet, data: &[u8], i: &mut usize, o: &mut usize, err: &mut u8) {
    let v0 = char_to_quintet(data[*i], alphabet);
    let v1 = char_to_quintet(data[*i + 1], alphabet);
    let v2 = char_to_quintet(data[*i + 2], alphabet);
    let v3 = char_to_quintet(data[*i + 3], alphabet);
    let v4 = char_to_quintet(data[*i + 4], alphabet);
    let v5 = char_to_quintet(data[*i + 5], alphabet);
    let v6 = char_to_quintet(data[*i + 6], alphabet);
    let v7 = char_to_quintet(data[*i + 7], alphabet);
    *err |= v0 | v1 | v2 | v3 | v4 | v5 | v6 | v7;
    output[*o] = (v0 << 3) | (v1 >> 2);
    output[*o + 1] = (v1.wrapping_shl(6)) | (v2 << 1) | (v3 >> 4);
    output[*o + 2] = (v3.wrapping_shl(4)) | (v4 >> 1);
    output[*o + 3] = (v4.wrapping_shl(7)) | (v5 << 2) | (v6 >> 3);
    output[*o + 4] = (v6.wrapping_shl(5)) | v7;
    *i += 8;
    *o += 5;
}

#[inline]
const fn decode_8quads(output: &mut [u8], alphabet: Alphabet, data: &[u8], i: &mut usize, o: &mut usize, err: &mut u8) {
    let mut n = 0;
    while n < 8 {
        let v0 = char_to_quintet(data[*i], alphabet);
        let v1 = char_to_quintet(data[*i + 1], alphabet);
        let v2 = char_to_quintet(data[*i + 2], alphabet);
        let v3 = char_to_quintet(data[*i + 3], alphabet);
        let v4 = char_to_quintet(data[*i + 4], alphabet);
        let v5 = char_to_quintet(data[*i + 5], alphabet);
        let v6 = char_to_quintet(data[*i + 6], alphabet);
        let v7 = char_to_quintet(data[*i + 7], alphabet);
        *err |= v0 | v1 | v2 | v3 | v4 | v5 | v6 | v7;
        output[*o] = (v0 << 3) | (v1 >> 2);
        output[*o + 1] = (v1.wrapping_shl(6)) | (v2 << 1) | (v3 >> 4);
        output[*o + 2] = (v3.wrapping_shl(4)) | (v4 >> 1);
        output[*o + 3] = (v4.wrapping_shl(7)) | (v5 << 2) | (v6 >> 3);
        output[*o + 4] = (v6.wrapping_shl(5)) | v7;
        *i += 8;
        *o += 5;
        n += 1;
    }
}

#[inline]
const fn strip_padding_info(data: &[u8], expect_padding: bool) -> Result<(usize, usize), DecodeError> {
    let in_len = data.len();

    if expect_padding {
        if in_len == 0 {
            return Ok((0, 0));
        }

        let count = count_trailing_padding(data);
        let content_len = in_len - count;

        let err = (count > 0 && in_len % 8 != 0)
            || count > 6
            || (count > 0
                && match count {
                    6 => content_len % 8 != 2,
                    4 => content_len % 8 != 4,
                    3 => content_len % 8 != 5,
                    1 => content_len % 8 != 7,
                    _ => true,
                });

        if err {
            return Err(DecodeError::InvalidPadding);
        }

        Ok((content_len, count))
    } else {
        if in_len > 0 && data[in_len - 1] == PAD {
            return Err(DecodeError::InvalidPadding);
        }
        Ok((in_len, 0))
    }
}

/// Count trailing `=` padding characters in constant time.
/// Scans at most 7 bytes from the end (max valid padding is 6).
/// The loop always runs exactly `min(len, 7)` iterations.
const fn count_trailing_padding(data: &[u8]) -> usize {
    let len = data.len();
    if len == 0 {
        return 0;
    }
    let max_check = if len < 7 { len } else { 7 };
    let mut count: usize = 0;
    let mut all_pad: u8 = 0xFF;

    let mut k = 0;
    while k < max_check {
        let idx = len - 1 - k;
        let is_pad = if data[idx] == PAD { 0xFFu8 } else { 0x00u8 };
        all_pad = all_pad & is_pad;
        let all_pad_ext = (all_pad as i8 >> 7) as usize;
        count = ((k + 1) as usize) & all_pad_ext | count & !all_pad_ext;
        k += 1;
    }

    count
}

/// Constant-time mapping: base32 character to 5-bit value.
/// Valid characters return 0-31. Invalid characters return a value with bit 5 set (>= 32).
#[inline]
const fn char_to_quintet(c: u8, alphabet: Alphabet) -> u8 {
    match alphabet {
        Alphabet::Crockford => crockford_to_quintet(c),
        Alphabet::Rfc4648 | Alphabet::Rfc4648NoPadding => {
            let not_upper = not_in_range(c, b'A', b'Z');
            let not_digit = not_in_range(c, b'2', b'7');
            let value = (c.wrapping_sub(b'A')) & !not_upper | (c.wrapping_sub(b'2').wrapping_add(26)) & !not_digit;
            let invalid = not_upper & not_digit;
            value | (invalid & 0x20)
        }
        Alphabet::Rfc4648Lower | Alphabet::Rfc4648LowerNoPadding => {
            let not_lower = not_in_range(c, b'a', b'z');
            let not_digit = not_in_range(c, b'2', b'7');
            let value = (c.wrapping_sub(b'a')) & !not_lower | (c.wrapping_sub(b'2').wrapping_add(26)) & !not_digit;
            let invalid = not_lower & not_digit;
            value | (invalid & 0x20)
        }
        Alphabet::Rfc4648Hex | Alphabet::Rfc4648HexNoPadding => {
            let not_digit = not_in_range(c, b'0', b'9');
            let not_upper = not_in_range(c, b'A', b'V');
            let value = (c.wrapping_sub(b'0')) & !not_digit | (c.wrapping_sub(b'A').wrapping_add(10)) & !not_upper;
            let invalid = not_digit & not_upper;
            value | (invalid & 0x20)
        }
        Alphabet::Rfc4648HexLower | Alphabet::Rfc4648HexLowerNoPadding => {
            let not_digit = not_in_range(c, b'0', b'9');
            let not_lower = not_in_range(c, b'a', b'v');
            let value = (c.wrapping_sub(b'0')) & !not_digit | (c.wrapping_sub(b'a').wrapping_add(10)) & !not_lower;
            let invalid = not_digit & not_lower;
            value | (invalid & 0x20)
        }
        Alphabet::Z32 => Z32_DECODE_TABLE[c as usize],
    }
}

/// Crockford character-to-quintet: 6 non-contiguous ranges.
#[inline]
const fn crockford_to_quintet(c: u8) -> u8 {
    let not_0_9 = not_in_range(c, b'0', b'9');
    let not_a_h = not_in_range(c, b'A', b'H');
    let not_j_k = not_in_range(c, b'J', b'K');
    let not_m_n = not_in_range(c, b'M', b'N');
    let not_p_t = not_in_range(c, b'P', b'T');
    let not_v_z = not_in_range(c, b'V', b'Z');
    let value = (c.wrapping_sub(b'0')) & !not_0_9
        | (c.wrapping_sub(b'A').wrapping_add(10)) & !not_a_h
        | (c.wrapping_sub(b'J').wrapping_add(18)) & !not_j_k
        | (c.wrapping_sub(b'M').wrapping_add(20)) & !not_m_n
        | (c.wrapping_sub(b'P').wrapping_add(22)) & !not_p_t
        | (c.wrapping_sub(b'V').wrapping_add(27)) & !not_v_z;
    let invalid = not_0_9 & not_a_h & not_j_k & not_m_n & not_p_t & not_v_z;
    value | (invalid & 0x20)
}

#[cfg(test)]
mod tests {
    use super::*;

    // (input_bytes, alphabet, expected_encoded_str, description)
    const ENCODE_VECTORS: &[(&[u8], Alphabet, &str, &str)] = &[
        (b"", Alphabet::Rfc4648, "", "RFC4648 padded: empty"),
        (b"", Alphabet::Rfc4648NoPadding, "", "RFC4648 unpadded: empty"),
        (b"\x00", Alphabet::Rfc4648, "AA======", "RFC4648 padded: 0x00"),
        (b"\xFF", Alphabet::Rfc4648, "74======", "RFC4648 padded: 0xFF"),
        (b"\xAB", Alphabet::Rfc4648, "VM======", "RFC4648 padded: 0xAB"),
        (b"fo", Alphabet::Rfc4648, "MZXQ====", "RFC4648 padded: 'fo'"),
        (b"foo", Alphabet::Rfc4648, "MZXW6===", "RFC4648 padded: 'foo'"),
        (b"foob", Alphabet::Rfc4648, "MZXW6YQ=", "RFC4648 padded: 'foob'"),
        (b"fooba", Alphabet::Rfc4648, "MZXW6YTB", "RFC4648 padded: 'fooba'"),
        (b"foobar", Alphabet::Rfc4648, "MZXW6YTBOI======", "RFC4648 padded: 'foobar'"),
        (b"hello", Alphabet::Rfc4648, "NBSWY3DP", "RFC4648 padded: 'hello'"),
        (b"hello", Alphabet::Rfc4648NoPadding, "NBSWY3DP", "RFC4648 unpadded: 'hello'"),
        (b"h", Alphabet::Rfc4648NoPadding, "NA", "RFC4648 unpadded: 'h'"),
        (b"he", Alphabet::Rfc4648NoPadding, "NBSQ", "RFC4648 unpadded: 'he'"),
        (b"hel", Alphabet::Rfc4648NoPadding, "NBSWY", "RFC4648 unpadded: 'hel'"),
        (b"hell", Alphabet::Rfc4648NoPadding, "NBSWY3A", "RFC4648 unpadded: 'hell'"),
        (b"hello", Alphabet::Rfc4648Lower, "nbswy3dp", "RFC4648 lower: 'hello'"),
        (b"hello", Alphabet::Rfc4648Hex, "D1IMOR3F", "RFC4648 hex: 'hello'"),
        (b"hello", Alphabet::Rfc4648HexLower, "d1imor3f", "RFC4648 hex lower: 'hello'"),
        (b"hello", Alphabet::Crockford, "D1JPRV3F", "Crockford: 'hello'"),
        // RFC 4648 Section 10 hex test vectors
        (b"f", Alphabet::Rfc4648Hex, "CO======", "RFC4648 hex: 'f'"),
        (b"fo", Alphabet::Rfc4648Hex, "CPNG====", "RFC4648 hex: 'fo'"),
        (b"foo", Alphabet::Rfc4648Hex, "CPNMU===", "RFC4648 hex: 'foo'"),
        (b"foob", Alphabet::Rfc4648Hex, "CPNMUOG=", "RFC4648 hex: 'foob'"),
        (b"fooba", Alphabet::Rfc4648Hex, "CPNMUOJ1", "RFC4648 hex: 'fooba'"),
        (b"foobar", Alphabet::Rfc4648Hex, "CPNMUOJ1E8======", "RFC4648 hex: 'foobar'"),
        // Z32
        (b"", Alphabet::Z32, "", "Z32: empty"),
        (b"\x00", Alphabet::Z32, "yy", "Z32: 0x00"),
        (b"\xff", Alphabet::Z32, "9h", "Z32: 0xFF"),
        (b"\xab", Alphabet::Z32, "ic", "Z32: 0xAB"),
        (b"fo", Alphabet::Z32, "c3zo", "Z32: fo"),
        (b"foo", Alphabet::Z32, "c3zs6", "Z32: foo"),
        (b"foob", Alphabet::Z32, "c3zs6ao", "Z32: foob"),
        (b"fooba", Alphabet::Z32, "c3zs6aub", "Z32: fooba"),
        (b"foobar", Alphabet::Z32, "c3zs6aubqe", "Z32: foobar"),
        (b"hello", Alphabet::Z32, "pb1sa5dx", "Z32: hello"),
        (b"h", Alphabet::Z32, "py", "Z32: h"),
        (b"he", Alphabet::Z32, "pb1o", "Z32: he"),
        (b"hel", Alphabet::Z32, "pb1sa", "Z32: hel"),
        (b"hell", Alphabet::Z32, "pb1sa5y", "Z32: hell"),
    ];

    // (encoded_str, alphabet, expected_bytes, description)
    const DECODE_VECTORS: &[(&[u8], Alphabet, &[u8], &str)] = &[
        (b"", Alphabet::Rfc4648, b"", "RFC4648 padded: empty"),
        (b"AA======", Alphabet::Rfc4648, b"\x00", "RFC4648 padded: 0x00"),
        (b"AE======", Alphabet::Rfc4648, b"\x01", "RFC4648 padded: 0x01"),
        (b"MZXQ====", Alphabet::Rfc4648, b"fo", "RFC4648 padded: 'fo'"),
        (b"MZXW6===", Alphabet::Rfc4648, b"foo", "RFC4648 padded: 'foo'"),
        (b"MZXW6YQ=", Alphabet::Rfc4648, b"foob", "RFC4648 padded: 'foob'"),
        (b"MZXW6YTB", Alphabet::Rfc4648, b"fooba", "RFC4648 padded: 'fooba'"),
        (b"NA", Alphabet::Rfc4648NoPadding, b"h", "RFC4648 unpadded: 'h'"),
        (b"NBSQ", Alphabet::Rfc4648NoPadding, b"he", "RFC4648 unpadded: 'he'"),
        (b"NBSWY", Alphabet::Rfc4648NoPadding, b"hel", "RFC4648 unpadded: 'hel'"),
        (b"NBSWY3A", Alphabet::Rfc4648NoPadding, b"hell", "RFC4648 unpadded: 'hell'"),
        (b"nbswy3dp", Alphabet::Rfc4648Lower, b"hello", "RFC4648 lower: 'hello'"),
        (b"D1IMOR3F", Alphabet::Rfc4648Hex, b"hello", "RFC4648 hex: 'hello'"),
        (b"D1JPRV3F", Alphabet::Crockford, b"hello", "Crockford: 'hello'"),
        // Z32
        (b"", Alphabet::Z32, b"", "Z32: empty"),
        (b"yy", Alphabet::Z32, b"\x00", "Z32: 0x00"),
        (b"9h", Alphabet::Z32, b"\xff", "Z32: 0xFF"),
        (b"ic", Alphabet::Z32, b"\xab", "Z32: 0xAB"),
        (b"c3zo", Alphabet::Z32, b"fo", "Z32: fo"),
        (b"c3zs6", Alphabet::Z32, b"foo", "Z32: foo"),
        (b"c3zs6ao", Alphabet::Z32, b"foob", "Z32: foob"),
        (b"c3zs6aub", Alphabet::Z32, b"fooba", "Z32: fooba"),
        (b"c3zs6aubqe", Alphabet::Z32, b"foobar", "Z32: foobar"),
        (b"pb1sa5dx", Alphabet::Z32, b"hello", "Z32: hello"),
        (b"py", Alphabet::Z32, b"h", "Z32: h"),
        (b"pb1o", Alphabet::Z32, b"he", "Z32: he"),
        (b"pb1sa", Alphabet::Z32, b"hel", "Z32: hel"),
        (b"pb1sa5y", Alphabet::Z32, b"hell", "Z32: hell"),
    ];

    // (encoded_str, alphabet, expected_error, description)
    const DECODE_ERROR_VECTORS: &[(&[u8], Alphabet, DecodeError, &str)] = &[
        (b"!!!!====", Alphabet::Rfc4648, DecodeError::InvalidInput, "4 invalid chars"),
        (
            b"AAA=====",
            Alphabet::Rfc4648,
            DecodeError::InvalidPadding,
            "wrong padding position",
        ),
        (
            b"AA==========",
            Alphabet::Rfc4648,
            DecodeError::InvalidPadding,
            "too many padding chars",
        ),
        (
            b"AA======",
            Alphabet::Rfc4648NoPadding,
            DecodeError::InvalidPadding,
            "no-pad rejects padding",
        ),
        (b"A", Alphabet::Rfc4648, DecodeError::InvalidLength, "single char"),
        (
            b"D1JPRV!!",
            Alphabet::Crockford,
            DecodeError::InvalidInput,
            "Crockford invalid chars",
        ),
        (b"!!!!", Alphabet::Z32, DecodeError::InvalidInput, "Z32 invalid chars"),
    ];

    // (data_length, padding, expected_result, description)
    const ENCODED_LENGTH_VECTORS: &[(usize, bool, Option<usize>, &str)] = &[
        (0, true, Some(0), "empty padded"),
        (1, true, Some(8), "1 byte padded"),
        (5, true, Some(8), "5 bytes padded"),
        (6, true, Some(16), "6 bytes padded"),
        (10, true, Some(16), "10 bytes padded"),
        (0, false, Some(0), "empty unpadded"),
        (1, false, Some(2), "1 byte unpadded"),
        (5, false, Some(8), "5 bytes unpadded"),
        (6, false, Some(10), "6 bytes unpadded"),
    ];

    // (initial_string, input_bytes, alphabet, expected_output, description)
    const ENCODE_INTO_STRING_VECTORS: &[(&str, &[u8], Alphabet, &str, &str)] = &[
        ("", b"", Alphabet::Rfc4648, "", "empty"),
        ("prefix", b"", Alphabet::Rfc4648, "prefix", "empty data with prefix"),
        (
            "",
            b"hello world",
            Alphabet::Rfc4648,
            "NBSWY3DPEB3W64TMMQ======",
            "hello world padded",
        ),
        (
            "",
            b"hello world",
            Alphabet::Rfc4648NoPadding,
            "NBSWY3DPEB3W64TMMQ",
            "hello world unpadded",
        ),
        (
            "data: ",
            b"hello world",
            Alphabet::Rfc4648,
            "data: NBSWY3DPEB3W64TMMQ======",
            "append to prefix",
        ),
        ("", b"foobar", Alphabet::Rfc4648Hex, "CPNMUOJ1E8======", "hex alphabet"),
    ];

    const ALL_ALPHABETS: &[Alphabet] = &[
        Alphabet::Rfc4648,
        Alphabet::Rfc4648NoPadding,
        Alphabet::Rfc4648Lower,
        Alphabet::Rfc4648Hex,
        Alphabet::Rfc4648HexLower,
        Alphabet::Crockford,
        Alphabet::Z32,
    ];

    const ROUNDTRIP_SIZES: &[usize] = &[
        0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129,
    ];

    const SIMD_BOUNDARY_SIZES: &[usize] = &[38, 39, 40, 41, 42, 45, 50, 62, 63, 64, 65, 66, 84, 85, 100];

    #[test]
    fn test_encode() {
        for &(input, alphabet, expected, desc) in ENCODE_VECTORS {
            let result = encode(input, alphabet);
            assert_eq!(result, expected, "encode: {desc}");
        }
    }

    #[test]
    fn test_decode() {
        for &(encoded, alphabet, expected, desc) in DECODE_VECTORS {
            let result = decode(encoded, alphabet).unwrap();
            assert_eq!(&result, expected, "decode: {desc}");
        }
    }

    #[test]
    fn test_decode_error() {
        for &(encoded, alphabet, expected_err, desc) in DECODE_ERROR_VECTORS {
            let result = decode(encoded, alphabet);
            assert_eq!(result, Err(expected_err), "decode error: {desc}");
        }

        // Large buffer with trailing invalid char
        let mut data = alloc::vec![b'A'; 256];
        data[255] = b'!';
        assert_eq!(decode(&data, Alphabet::Rfc4648), Err(DecodeError::InvalidInput));
    }

    #[test]
    fn test_encoded_length() {
        for &(data_len, padding, expected, desc) in ENCODED_LENGTH_VECTORS {
            let result = encoded_length(data_len, padding);
            assert_eq!(result, expected, "encoded_length: {desc}");
        }
    }

    #[test]
    fn test_encode_into_string() {
        for &(initial, input, alphabet, expected, desc) in ENCODE_INTO_STRING_VECTORS {
            let mut s = alloc::string::String::from(initial);
            encode_into_string(&mut s, input, alphabet);
            assert_eq!(s, expected, "encode_into_string: {desc}");
        }

        // Multi-append
        let mut s = alloc::string::String::from("~~");
        encode_into_string(&mut s, b"hello", Alphabet::Rfc4648);
        assert_eq!(s, "~~NBSWY3DP");
        encode_into_string(&mut s, b"foo", Alphabet::Rfc4648);
        assert_eq!(s, "~~NBSWY3DPMZXW6===");

        // All alphabets
        for alphabet in ALL_ALPHABETS {
            let expected = encode(b"hello world", *alphabet);
            let mut s = alloc::string::String::new();
            encode_into_string(&mut s, b"hello world", *alphabet);
            assert_eq!(s, expected, "encode_into_string alphabet {alphabet:?}");
        }
    }

    #[test]
    fn test_roundtrip() {
        for &len in ROUNDTRIP_SIZES {
            let data: Vec<u8> = (0..len as u8).collect();
            for alphabet in ALL_ALPHABETS {
                let encoded = encode(&data, *alphabet);
                let decoded = decode(encoded.as_bytes(), *alphabet).unwrap();
                assert_eq!(decoded, data, "roundtrip len={len} alphabet={alphabet:?}");
            }
        }
    }

    #[test]
    fn test_roundtrip_large() {
        let size = 4096;

        let data = alloc::vec![0x00u8; size];
        let elen = encoded_length(size, true).expect("encoded_len overflow");
        let mut encoded = alloc::vec![0u8; elen];
        encode_into_constant_time(&mut encoded, &data, Alphabet::Rfc4648).unwrap();
        let mut decoded = alloc::vec![0u8; size];
        decode_into_constant_time(&mut decoded, &encoded, Alphabet::Rfc4648).unwrap();
        assert_eq!(decoded, data, "4096 zeroes constant-time");

        let data = alloc::vec![0xFFu8; size];
        let mut encoded = alloc::vec![0u8; elen];
        encode_into_constant_time(&mut encoded, &data, Alphabet::Rfc4648).unwrap();
        decode_into_constant_time(&mut decoded, &encoded, Alphabet::Rfc4648).unwrap();
        assert_eq!(decoded, data, "4096 0xFF constant-time");

        let data: Vec<u8> = (0..=255).cycle().take(size).collect();
        let encoded = encode(&data, Alphabet::Rfc4648);
        let decoded = decode(encoded.as_bytes(), Alphabet::Rfc4648).unwrap();
        assert_eq!(decoded, data, "4096 cycle dispatch");

        let data: Vec<u8> = (0..=255).collect();
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Rfc4648);
        let decoded = decode(s.as_bytes(), Alphabet::Rfc4648).unwrap();
        assert_eq!(decoded, data, "256-byte encode_into_string roundtrip");

        let data: Vec<u8> = (0..255).cycle().take(4096).collect();
        let expected = encode(&data, Alphabet::Rfc4648);
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Rfc4648);
        assert_eq!(s, expected, "4096-byte encode_into_string");
    }

    #[test]
    fn test_encode_all_single_bytes() {
        for byte in 0..=255u8 {
            for alphabet in &[
                Alphabet::Rfc4648,
                Alphabet::Rfc4648Lower,
                Alphabet::Rfc4648Hex,
                Alphabet::Rfc4648HexLower,
                Alphabet::Crockford,
                Alphabet::Z32,
            ] {
                let padding = alphabet.is_padded();
                let elen = encoded_length(1, padding).unwrap();
                let mut encoded = alloc::vec![0u8; elen];
                encode_into_constant_time(&mut encoded, &[byte], *alphabet).unwrap();
                let mut decoded = [0u8; 1];
                decode_into_constant_time(&mut decoded, &encoded, *alphabet).unwrap();
                assert_eq!(decoded[0], byte, "single byte roundtrip {byte:#04x} alphabet={alphabet:?}");
            }
        }
    }

    #[test]
    fn test_decode_invalid_char_every_position() {
        let mut out = [0u8; 128];

        // 8 chars (1 full quad), invalid at positions 0..7
        for pos in 0..8 {
            let mut input = [b'A'; 8];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Rfc4648),
                Err(DecodeError::InvalidInput),
                "invalid char at position {pos} in 8-char input"
            );
        }

        // 64 chars (8 quads), invalid at positions 0..63
        for pos in 0..64 {
            let mut input = [b'A'; 64];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Rfc4648),
                Err(DecodeError::InvalidInput),
                "invalid char at position {pos} in 64-char input"
            );
        }

        // 72 chars (8 quads + 1 more quad), invalid at positions 0..71
        for pos in 0..72 {
            let mut input = [b'A'; 72];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Rfc4648),
                Err(DecodeError::InvalidInput),
                "invalid char at position {pos} in 72-char input"
            );
        }

        // Lower, hex, hex_lower, crockford alphabets
        for pos in 0..8 {
            let mut input = [b'a'; 8];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Rfc4648Lower),
                Err(DecodeError::InvalidInput)
            );
        }
        for pos in 0..8 {
            let mut input = [b'0'; 8];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Rfc4648Hex),
                Err(DecodeError::InvalidInput)
            );
        }
        for pos in 0..8 {
            let mut input = [b'0'; 8];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Rfc4648HexLower),
                Err(DecodeError::InvalidInput)
            );
        }
        for pos in 0..8 {
            let mut input = [b'0'; 8];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Crockford),
                Err(DecodeError::InvalidInput)
            );
        }
        for pos in 0..8 {
            let mut input = [b'y'; 8];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Z32),
                Err(DecodeError::InvalidInput)
            );
        }
    }

    #[test]
    fn test_decode_non_canonical_trailing_bits() {
        let mut out = [0u8; 8];

        // remaining == 2: bottom 2 bits of v1 must be zero
        for &(input, expected) in &[
            (b"AA======" as &[u8], Ok(())),
            (b"AB======" as &[u8], Err(DecodeError::InvalidInput)),
            (b"AC======" as &[u8], Err(DecodeError::InvalidInput)),
            (b"AD======" as &[u8], Err(DecodeError::InvalidInput)),
        ] {
            assert_eq!(
                decode_into_constant_time(&mut out, input, Alphabet::Rfc4648),
                expected,
                "non-canonical (rem=2): {:?}",
                core::str::from_utf8(input)
            );
        }

        // remaining == 4: bottom 4 bits of v3 must be zero
        for &(input, expected) in &[
            (b"MZXQ====" as &[u8], Ok(())),
            (b"MZXR====" as &[u8], Err(DecodeError::InvalidInput)),
        ] {
            assert_eq!(
                decode_into_constant_time(&mut out, input, Alphabet::Rfc4648),
                expected,
                "non-canonical (rem=4): {:?}",
                core::str::from_utf8(input)
            );
        }

        // remaining == 5: bottom 1 bit of v4 must be zero
        for &(input, expected) in &[
            (b"MZXW6===" as &[u8], Ok(())),
            (b"MZXW7===" as &[u8], Err(DecodeError::InvalidInput)),
        ] {
            assert_eq!(
                decode_into_constant_time(&mut out, input, Alphabet::Rfc4648),
                expected,
                "non-canonical (rem=5): {:?}",
                core::str::from_utf8(input)
            );
        }

        // remaining == 7: bottom 3 bits of v6 must be zero
        assert_eq!(decode_into_constant_time(&mut out, b"NBSWY3DP", Alphabet::Rfc4648), Ok(()));
    }

    #[test]
    fn test_decode_rejects_interior_padding() {
        let mut out = [0u8; 8];
        assert_eq!(
            decode_into_constant_time(&mut out, b"=AAA====", Alphabet::Rfc4648),
            Err(DecodeError::InvalidInput)
        );
        assert_eq!(
            decode_into_constant_time(&mut out, b"A=AA====", Alphabet::Rfc4648),
            Err(DecodeError::InvalidInput)
        );
        assert_eq!(
            decode_into_constant_time(&mut out, b"AA=A====", Alphabet::Rfc4648),
            Err(DecodeError::InvalidInput)
        );
        assert_eq!(
            decode_into_constant_time(&mut out, b"AAA=====", Alphabet::Rfc4648),
            Err(DecodeError::InvalidPadding)
        );
    }

    #[test]
    fn test_roundtrip_simd_boundary_sizes() {
        let mut data_buf = Vec::new();
        let mut enc_buf = Vec::new();

        for &input_len in SIMD_BOUNDARY_SIZES {
            data_buf.clear();
            for b in 0..input_len {
                data_buf.push(b as u8);
            }

            for alphabet in &[Alphabet::Rfc4648, Alphabet::Rfc4648NoPadding] {
                let padding = alphabet.is_padded();
                let elen = encoded_length(input_len, padding).expect("encoded_len overflow");
                enc_buf.resize(elen, 0);
                encode_into_constant_time(&mut enc_buf, &data_buf, *alphabet).unwrap();

                let mut decoded = alloc::vec![0u8; input_len];
                assert_eq!(
                    decode_into_constant_time(&mut decoded, &enc_buf, *alphabet),
                    Ok(()),
                    "decode failed len={input_len} alphabet={alphabet:?}"
                );
                assert_eq!(&decoded, &data_buf, "roundtrip mismatch len={input_len} alphabet={alphabet:?}");
            }
        }
    }

    #[test]
    fn test_const_encode() {
        const RESULT: [u8; 8] = encode_array::<8>(b"hello", Alphabet::Rfc4648);
        assert_eq!(&RESULT, b"NBSWY3DP");

        const RESULT_EMPTY: [u8; 0] = encode_array::<0>(b"", Alphabet::Rfc4648);
        assert_eq!(RESULT_EMPTY.len(), 0);

        const RESULT_CROCKFORD: [u8; 8] = encode_array::<8>(b"hello", Alphabet::Crockford);
        assert_eq!(&RESULT_CROCKFORD, b"D1JPRV3F");

        const RESULT_Z32: [u8; 8] = encode_array::<8>(b"hello", Alphabet::Z32);
        assert_eq!(&RESULT_Z32, b"pb1sa5dx");
    }

    #[test]
    fn test_const_decode() {
        const RESULT: Result<[u8; 5], DecodeError> = decode_array::<5>(b"NBSWY3DP", Alphabet::Rfc4648);
        assert_eq!(RESULT.unwrap(), *b"hello");

        const RESULT_EMPTY: Result<[u8; 0], DecodeError> = decode_array::<0>(b"", Alphabet::Rfc4648);
        assert_eq!(RESULT_EMPTY.unwrap().len(), 0);

        const RESULT_Z32: Result<[u8; 5], DecodeError> = decode_array::<5>(b"pb1sa5dx", Alphabet::Z32);
        assert_eq!(RESULT_Z32.unwrap(), *b"hello");
    }

    #[test]
    fn test_const_decode_error() {
        const ERR_INVALID: Result<[u8; 5], DecodeError> = decode_array::<5>(b"D1JPRV!!", Alphabet::Crockford);
        assert_eq!(ERR_INVALID, Err(DecodeError::InvalidInput));

        const ERR_Z32: Result<[u8; 5], DecodeError> = decode_array::<5>(b"pb1sa!!x", Alphabet::Z32);
        assert_eq!(ERR_Z32, Err(DecodeError::InvalidInput));
    }

    #[test]
    fn test_buffer_management() {
        let mut out = [0u8; 1];
        assert_eq!(
            encode_into(&mut out, b"hello", Alphabet::Rfc4648),
            Err(EncodeError::InvalidOutputLength)
        );

        let mut out = [0u8; 5];
        decode_into(&mut out, b"NBSWY3DP", Alphabet::Rfc4648).unwrap();
        assert_eq!(&out, b"hello");

        let mut out = [0u8; 1];
        assert_eq!(
            decode_into(&mut out, b"NBSWY3DP", Alphabet::Rfc4648),
            Err(DecodeError::InvalidLength)
        );

        // Exact output size decode
        let mut remainders = [0u8; 80];
        let mut output = [0u8; 80];
        for len in 1..=80 {
            for i in 0..len {
                remainders[i] = (i * 7 + 3) as u8;
            }
            let encoded = encode(&remainders[..len], Alphabet::Rfc4648);
            let expected_output_len = len;
            let r = decode_into(&mut output[..expected_output_len], encoded.as_bytes(), Alphabet::Rfc4648);
            assert!(r.is_ok(), "decode_into failed at len {}", len);
            assert_eq!(&output[..expected_output_len], &remainders[..len], "mismatch at len {}", len);
        }
    }

    #[test]
    fn test_display_error() {
        assert_eq!(format!("{}", DecodeError::InvalidInput), "invalid base32 character");
        assert_eq!(format!("{}", DecodeError::InvalidLength), "invalid base32 length");
        assert_eq!(format!("{}", DecodeError::InvalidPadding), "invalid base32 padding");
        assert_eq!(
            format!("{}", EncodeError::InvalidOutputLength),
            "output buffer size is not valid"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        struct Data(#[serde(with = "crate::serde")] Vec<u8>);

        let data = Data(b"hello world".to_vec());
        let json = ::serde_json::to_string(&data).unwrap();
        assert_eq!(json, "\"NBSWY3DPEB3W64TMMQ======\"");
        let deserialized: Data = ::serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.0, b"hello world");
    }
}
