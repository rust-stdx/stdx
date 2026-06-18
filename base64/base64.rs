#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Fast base64 encoding and decoding with SIMD acceleration, constant-time
//! operations, and `const fn` support.
//!
//! Four alphabet variants are available via [`Alphabet`]:
//!
//! | Variant             | Characters       | Padding |
//! |---------------------|------------------|---------|
//! | `Standard`          | `A-Za-z0-9+/`    | `=`     |
//! | `StandardNoPadding` | `A-Za-z0-9+/`    | none    |
//! | `Url`               | `A-Za-z0-9-_`    | `=`     |
//! | `UrlNoPadding`      | `A-Za-z0-9-_`    | none    |
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
//! [`encode_array`] and [`decode_array`] are `const fn`, enabling base64
//! encoding and decoding at compile time.
//!
//! # Examples
//!
//! ```rust
//! use base64::{Alphabet, encode, decode};
//!
//! let encoded = encode(b"hello world", Alphabet::Standard);
//! assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
//!
//! let decoded = decode(b"aGVsbG8gd29ybGQ=", Alphabet::Standard).unwrap();
//! assert_eq!(decoded, b"hello world");
//!
//! let url = encode(b"hello world", Alphabet::Url);
//! assert_eq!(url, "aGVsbG8gd29ybGQ=");
//! ```

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(all(feature = "serde", any(feature = "alloc", test)))]
mod serde;

#[cfg(target_arch = "aarch64")]
mod base64_neon;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod base64_avx2;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod base64_wasm_simd128;

const PAD: u8 = b'=';

/// The base64 alphabet used for encoding and decoding.
///
/// | Variant             | Characters       | Padding |
/// |---------------------|------------------|---------|
/// | `Standard`          | `A-Za-z0-9+/`    | `=`     |
/// | `StandardNoPadding` | `A-Za-z0-9+/`    | none    |
/// | `Url`               | `A-Za-z0-9-_`    | `=`     |
/// | `UrlNoPadding`      | `A-Za-z0-9-_`    | none    |
///
/// # Example
///
/// ```rust
/// use base64::Alphabet;
///
/// let encoded = base64::encode(b"hello", Alphabet::Url);
/// assert_eq!(encoded, "aGVsbG8=");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alphabet {
    Standard,
    StandardNoPadding,
    Url,
    UrlNoPadding,
}

impl Alphabet {
    #[inline]
    const fn is_padded(&self) -> bool {
        matches!(self, Alphabet::Standard | Alphabet::Url)
    }
}

/// Errors that can occur during base64 encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    /// The output buffer length does not match the expected encoded length.
    InvalidOutputLength,
    /// The encoded output length overflows `usize`.
    OutputOverflow,
}

/// Errors that can occur during base64 decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// The input contains a character that is not valid for the chosen
    /// [`Alphabet`].
    InvalidInput,
    /// The input length is not valid for base64 decoding.
    InvalidInputLength,
    /// The input has invalid padding (e.g. missing `=` when expected,
    /// unexpected `=`, or wrong number of padding characters).
    InvalidPadding,
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidOutputLength => f.write_str("output buffer size must be exactly equal to decoded_len(input)"),
            Self::OutputOverflow => f.write_str("output length overflows usize::MAX"),
        }
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInput => f.write_str("invalid base64 character"),
            Self::InvalidInputLength => f.write_str("invalid base64 length"),
            Self::InvalidPadding => f.write_str("invalid base64 padding"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EncodeError {}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Encode
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Returns the size in bytes of the input data after base64 encoding.
///
/// Returns `None` if the output size overflows `usize`.
///
/// # Example
///
/// ```rust
/// assert_eq!(base64::encoded_length(3, true), Some(4));
/// assert_eq!(base64::encoded_length(1, false), Some(2));
/// assert_eq!(base64::encoded_length(usize::MAX, true), None);
/// ```
pub const fn encoded_length(data_length: usize, padding: bool) -> Option<usize> {
    let complete_chunks = data_length / 3;
    let remaining = data_length % 3;
    let base = match complete_chunks.checked_mul(4) {
        Some(v) => v,
        None => return None,
    };
    if remaining == 0 {
        Some(base)
    } else if padding {
        base.checked_add(4)
    } else if remaining == 1 {
        Some(base + 2)
    } else {
        Some(base + 3)
    }
}

/// Encodes bytes to a base64 string using the given [`Alphabet`].
///
/// # Example
///
/// ```rust
/// let encoded = base64::encode(b"hello world", base64::Alphabet::Standard);
/// assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
/// ```
#[cfg(feature = "alloc")]
pub fn encode(data: impl AsRef<[u8]>, alphabet: Alphabet) -> alloc::string::String {
    let data = data.as_ref();
    let len = encoded_length(data.len(), alphabet.is_padded()).expect("encoded length overflow");
    let mut output = alloc::vec![0u8; len];
    encode_into(&mut output, data, alphabet).unwrap();
    // SAFETY: base64 only produces ASCII characters, which are valid UTF-8.
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
/// const DATA: [u8; 3] = [0x66, 0x6F, 0x6F];
/// const B64: [u8; 4] = base64::encode_array::<4>(&DATA, base64::Alphabet::Standard);
/// assert_eq!(&B64, b"Zm9v");
/// ```
pub const fn encode_array<const OUT: usize>(data: &[u8], alphabet: Alphabet) -> [u8; OUT] {
    let mut out_buffer = [0u8; OUT];
    match encode_into_constant_time(&mut out_buffer, data, alphabet) {
        Ok(_) => {}
        Err(_) => panic!("output buffer size is not valid"),
    };
    out_buffer
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
/// Returns [`EncodeError`] if `output.len()` does not match the expected encoded
/// length or if the encoded length overflows `usize`.
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 16];
/// base64::encode_into(&mut buf, b"hello world", base64::Alphabet::Standard).unwrap();
/// assert_eq!(&buf, b"aGVsbG8gd29ybGQ=");
/// ```
pub fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    if data.len() >= 48 {
        check_encode_output_length(output.len(), data.len(), alphabet)?;
        return unsafe { base64_neon::encode_into(output, data, alphabet) };
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    if data.len() >= 24 {
        check_encode_output_length(output.len(), data.len(), alphabet)?;
        return unsafe { base64_avx2::encode_into(output, data, alphabet) };
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    if data.len() >= 12 {
        check_encode_output_length(output.len(), data.len(), alphabet)?;
        return base64_wasm_simd128::encode_into(output, data, alphabet);
    }

    return encode_into_constant_time(output, data, alphabet);
}

/// Constant-time base64 encoding. Processes all input data without
/// secret-dependent branches or memory accesses, making it suitable
/// for cryptographic applications.
///
/// Consumers may prefer the faster [`encode_into`] which dispatches to
/// a SIMD-accelerated path when available (non constant-time).
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 4];
/// base64::encode_into_constant_time(&mut buf, b"foo", base64::Alphabet::Standard).unwrap();
/// assert_eq!(&buf, b"Zm9v");
/// ```
pub const fn encode_into_constant_time(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    match check_encode_output_length(output.len(), data.len(), alphabet) {
        Ok(_) => {}
        Err(err) => return Err(err),
    };

    let padding = alphabet.is_padded();
    let len = data.len();
    let mut i = 0;

    while i + 24 <= len {
        let b0 = data[i];
        let b1 = data[i + 1];
        let b2 = data[i + 2];
        let b3 = data[i + 3];
        let b4 = data[i + 4];
        let b5 = data[i + 5];
        let b6 = data[i + 6];
        let b7 = data[i + 7];
        let b8 = data[i + 8];
        let b9 = data[i + 9];
        let b10 = data[i + 10];
        let b11 = data[i + 11];
        let b12 = data[i + 12];
        let b13 = data[i + 13];
        let b14 = data[i + 14];
        let b15 = data[i + 15];
        let b16 = data[i + 16];
        let b17 = data[i + 17];
        let b18 = data[i + 18];
        let b19 = data[i + 19];
        let b20 = data[i + 20];
        let b21 = data[i + 21];
        let b22 = data[i + 22];
        let b23 = data[i + 23];

        let o = (i / 3) * 4;
        output[o] = sextet_to_base64_char(b0 >> 2, alphabet);
        output[o + 1] = sextet_to_base64_char(((b0 & 0x03) << 4) | (b1 >> 4), alphabet);
        output[o + 2] = sextet_to_base64_char(((b1 & 0x0F) << 2) | (b2 >> 6), alphabet);
        output[o + 3] = sextet_to_base64_char(b2 & 0x3F, alphabet);

        output[o + 4] = sextet_to_base64_char(b3 >> 2, alphabet);
        output[o + 5] = sextet_to_base64_char(((b3 & 0x03) << 4) | (b4 >> 4), alphabet);
        output[o + 6] = sextet_to_base64_char(((b4 & 0x0F) << 2) | (b5 >> 6), alphabet);
        output[o + 7] = sextet_to_base64_char(b5 & 0x3F, alphabet);

        output[o + 8] = sextet_to_base64_char(b6 >> 2, alphabet);
        output[o + 9] = sextet_to_base64_char(((b6 & 0x03) << 4) | (b7 >> 4), alphabet);
        output[o + 10] = sextet_to_base64_char(((b7 & 0x0F) << 2) | (b8 >> 6), alphabet);
        output[o + 11] = sextet_to_base64_char(b8 & 0x3F, alphabet);

        output[o + 12] = sextet_to_base64_char(b9 >> 2, alphabet);
        output[o + 13] = sextet_to_base64_char(((b9 & 0x03) << 4) | (b10 >> 4), alphabet);
        output[o + 14] = sextet_to_base64_char(((b10 & 0x0F) << 2) | (b11 >> 6), alphabet);
        output[o + 15] = sextet_to_base64_char(b11 & 0x3F, alphabet);

        output[o + 16] = sextet_to_base64_char(b12 >> 2, alphabet);
        output[o + 17] = sextet_to_base64_char(((b12 & 0x03) << 4) | (b13 >> 4), alphabet);
        output[o + 18] = sextet_to_base64_char(((b13 & 0x0F) << 2) | (b14 >> 6), alphabet);
        output[o + 19] = sextet_to_base64_char(b14 & 0x3F, alphabet);

        output[o + 20] = sextet_to_base64_char(b15 >> 2, alphabet);
        output[o + 21] = sextet_to_base64_char(((b15 & 0x03) << 4) | (b16 >> 4), alphabet);
        output[o + 22] = sextet_to_base64_char(((b16 & 0x0F) << 2) | (b17 >> 6), alphabet);
        output[o + 23] = sextet_to_base64_char(b17 & 0x3F, alphabet);

        output[o + 24] = sextet_to_base64_char(b18 >> 2, alphabet);
        output[o + 25] = sextet_to_base64_char(((b18 & 0x03) << 4) | (b19 >> 4), alphabet);
        output[o + 26] = sextet_to_base64_char(((b19 & 0x0F) << 2) | (b20 >> 6), alphabet);
        output[o + 27] = sextet_to_base64_char(b20 & 0x3F, alphabet);

        output[o + 28] = sextet_to_base64_char(b21 >> 2, alphabet);
        output[o + 29] = sextet_to_base64_char(((b21 & 0x03) << 4) | (b22 >> 4), alphabet);
        output[o + 30] = sextet_to_base64_char(((b22 & 0x0F) << 2) | (b23 >> 6), alphabet);
        output[o + 31] = sextet_to_base64_char(b23 & 0x3F, alphabet);

        i += 24;
    }

    while i + 3 <= len {
        let b0 = data[i];
        let b1 = data[i + 1];
        let b2 = data[i + 2];
        let o = (i / 3) * 4;
        output[o] = sextet_to_base64_char(b0 >> 2, alphabet);
        output[o + 1] = sextet_to_base64_char(((b0 & 0x03) << 4) | (b1 >> 4), alphabet);
        output[o + 2] = sextet_to_base64_char(((b1 & 0x0F) << 2) | (b2 >> 6), alphabet);
        output[o + 3] = sextet_to_base64_char(b2 & 0x3F, alphabet);
        i += 3;
    }

    let remaining = len - i;
    if remaining > 0 {
        let o = (i / 3) * 4;
        let b0 = data[i];
        let b1 = if i + 1 < len { data[i + 1] } else { 0 };

        let rem1 = (remaining == 1) as u8;
        let rem2 = (remaining == 2) as u8;
        let m1 = 0u8.wrapping_sub(rem1);
        let m2 = 0u8.wrapping_sub(rem2);

        output[o] = sextet_to_base64_char(b0 >> 2, alphabet);

        let o1_rem1 = sextet_to_base64_char((b0 & 0x03) << 4, alphabet);
        let o1_rem2 = sextet_to_base64_char(((b0 & 0x03) << 4) | (b1 >> 4), alphabet);
        output[o + 1] = (o1_rem1 & m1) | (o1_rem2 & m2);

        if padding {
            let o2_rem1 = PAD;
            let o2_rem2 = sextet_to_base64_char((b1 & 0x0F) << 2, alphabet);
            output[o + 2] = (o2_rem1 & m1) | (o2_rem2 & m2);
            output[o + 3] = PAD;
        } else {
            if remaining == 2 {
                output[o + 2] = sextet_to_base64_char((b1 & 0x0F) << 2, alphabet);
            }
        }
    }

    Ok(())
}

/// Appends the base64-encoded representation of `data` to a [`String`].
///
/// # Example
///
/// ```rust
/// let mut s = String::from("tag: ");
/// base64::encode_into_string(&mut s, b"hello", base64::Alphabet::Standard);
/// assert_eq!(s, "tag: aGVsbG8=");
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

/// Checks that `output_length == encoded_length(data_length, padding)`
#[inline]
const fn check_encode_output_length(
    output_length: usize,
    data_length: usize,
    alphabet: Alphabet,
) -> Result<(), EncodeError> {
    let padding = alphabet.is_padded();

    let expected_output_length = match encoded_length(data_length, padding) {
        Some(length) => length,
        None => return Err(EncodeError::OutputOverflow),
    };
    if output_length != expected_output_length {
        return Err(EncodeError::InvalidOutputLength);
    }

    return Ok(());
}

/// Returns 0x00 if lo <= v <= hi, 0xFF otherwise.
/// Uses sign-bit propagation for branchless range checking.
#[inline]
const fn not_in_range(v: u8, lo: u8, hi: u8) -> u8 {
    (((v.wrapping_sub(lo) as i8) | (hi.wrapping_sub(v) as i8)) >> 7) as u8
}

/// Constant-time mapping: 6-bit value (0-63) to base64 character.
/// No secret-dependent branches or memory accesses.
#[inline]
const fn sextet_to_base64_char(v: u8, alphabet: Alphabet) -> u8 {
    let v = v & 0x3F;

    let not_upper = not_in_range(v, 0, 25);
    let not_lower = not_in_range(v, 26, 51);
    let not_digit = not_in_range(v, 52, 61);
    let not_62 = not_in_range(v, 62, 62);
    let not_63 = not_in_range(v, 63, 63);

    let upper_val = v + b'A';
    let lower_val = v.wrapping_sub(26).wrapping_add(b'a');
    let digit_val = v.wrapping_sub(52).wrapping_add(b'0');

    let (ch_62, ch_63) = match alphabet {
        Alphabet::Standard | Alphabet::StandardNoPadding => (b'+', b'/'),
        Alphabet::Url | Alphabet::UrlNoPadding => (b'-', b'_'),
    };

    (upper_val & !not_upper)
        | (lower_val & !not_lower)
        | (digit_val & !not_digit)
        | (ch_62 & !not_62)
        | (ch_63 & !not_63)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Decode
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Decodes a base64 string into bytes.
///
/// # Errors
///
/// Returns [`DecodeError`] if any character is invalid for the chosen
/// [`Alphabet`], the input length is not valid, or padding is incorrect.
///
/// # Example
///
/// ```rust
/// let decoded = base64::decode(b"aGVsbG8=", base64::Alphabet::Standard).unwrap();
/// assert_eq!(decoded, b"hello");
/// ```
#[cfg(feature = "alloc")]
pub fn decode(data: impl AsRef<[u8]>, alphabet: Alphabet) -> Result<alloc::vec::Vec<u8>, DecodeError> {
    let data = data.as_ref();
    let (content_len, _) = strip_padding_info(data, alphabet.is_padded())?;
    let output_len = decoded_length(content_len)?;
    let mut output = alloc::vec![0u8; output_len];
    decode_into(&mut output, data, alphabet)?;
    Ok(output)
}

/// Decodes a base64 string into a fixed-size array at compile time.
///
/// The generic parameter `OUT` is the output array length. It must be exactly
/// the decoded length of the input or a an error is returned.
///
/// # Example
///
/// ```rust
/// const RESULT: Result<[u8; 5], base64::DecodeError> =
///     base64::decode_array::<5>(b"aGVsbG8=", base64::Alphabet::Standard);
/// assert_eq!(RESULT.unwrap(), *b"hello");
/// ```
pub const fn decode_array<const OUT: usize>(encoded_data: &[u8], alphabet: Alphabet) -> Result<[u8; OUT], DecodeError> {
    let mut result = [0u8; OUT];
    match decode_into_constant_time(&mut result, encoded_data, alphabet) {
        Ok(()) => Ok(result),
        Err(err) => Err(err),
    }
}

/// Decodes a base64 string into an existing buffer.
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
/// base64::decode_into(&mut buf, b"aGVsbG8=", base64::Alphabet::Standard).unwrap();
/// assert_eq!(&buf, b"hello");
/// ```
pub fn decode_into(output: &mut [u8], encoded_data: &[u8], alphabet: Alphabet) -> Result<(), DecodeError> {
    let (content_len, _) = strip_padding_info(encoded_data, alphabet.is_padded())?;
    let computed_output = decoded_length(content_len)?;
    if output.len() < computed_output {
        return Err(DecodeError::InvalidInputLength);
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    if content_len >= 32 {
        let content = &encoded_data[..content_len];
        return unsafe { base64_neon::decode_into(output, content, alphabet) };
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    if content_len >= 32 {
        let content = &encoded_data[..content_len];
        return unsafe { base64_avx2::decode_into(output, content, alphabet) };
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    if content_len >= 16 {
        let content = &encoded_data[..content_len];
        return base64_wasm_simd128::decode_into(output, content, alphabet);
    }

    decode_into_constant_time(output, encoded_data, alphabet)
}

/// Constant-time base64 decoding. Processes all input data without
/// secret-dependent branches or memory accesses, making it suitable
/// for cryptographic applications.
///
/// Consumers may prefer the faster [`decode_into`] which dispatches to
/// a SIMD-accelerated path when available (non constant-time).
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 3];
/// base64::decode_into_constant_time(&mut buf, b"Zm9v", base64::Alphabet::Standard).unwrap();
/// assert_eq!(&buf, b"foo");
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
        return Err(DecodeError::InvalidInputLength);
    }

    let mut err: u8 = 0;
    let mut i = 0;
    let mut o = 0;

    while i + 32 <= content_len {
        let v0 = base64_char_to_sextet(encoded_data[i], alphabet);
        let v1 = base64_char_to_sextet(encoded_data[i + 1], alphabet);
        let v2 = base64_char_to_sextet(encoded_data[i + 2], alphabet);
        let v3 = base64_char_to_sextet(encoded_data[i + 3], alphabet);
        err |= v0 | v1 | v2 | v3;
        output[o] = (v0 << 2) | (v1 >> 4);
        output[o + 1] = (v1 << 4) | (v2 >> 2);
        output[o + 2] = (v2 << 6) | v3;

        let v4 = base64_char_to_sextet(encoded_data[i + 4], alphabet);
        let v5 = base64_char_to_sextet(encoded_data[i + 5], alphabet);
        let v6 = base64_char_to_sextet(encoded_data[i + 6], alphabet);
        let v7 = base64_char_to_sextet(encoded_data[i + 7], alphabet);
        err |= v4 | v5 | v6 | v7;
        output[o + 3] = (v4 << 2) | (v5 >> 4);
        output[o + 4] = (v5 << 4) | (v6 >> 2);
        output[o + 5] = (v6 << 6) | v7;

        let v8 = base64_char_to_sextet(encoded_data[i + 8], alphabet);
        let v9 = base64_char_to_sextet(encoded_data[i + 9], alphabet);
        let v10 = base64_char_to_sextet(encoded_data[i + 10], alphabet);
        let v11 = base64_char_to_sextet(encoded_data[i + 11], alphabet);
        err |= v8 | v9 | v10 | v11;
        output[o + 6] = (v8 << 2) | (v9 >> 4);
        output[o + 7] = (v9 << 4) | (v10 >> 2);
        output[o + 8] = (v10 << 6) | v11;

        let v12 = base64_char_to_sextet(encoded_data[i + 12], alphabet);
        let v13 = base64_char_to_sextet(encoded_data[i + 13], alphabet);
        let v14 = base64_char_to_sextet(encoded_data[i + 14], alphabet);
        let v15 = base64_char_to_sextet(encoded_data[i + 15], alphabet);
        err |= v12 | v13 | v14 | v15;
        output[o + 9] = (v12 << 2) | (v13 >> 4);
        output[o + 10] = (v13 << 4) | (v14 >> 2);
        output[o + 11] = (v14 << 6) | v15;

        let v16 = base64_char_to_sextet(encoded_data[i + 16], alphabet);
        let v17 = base64_char_to_sextet(encoded_data[i + 17], alphabet);
        let v18 = base64_char_to_sextet(encoded_data[i + 18], alphabet);
        let v19 = base64_char_to_sextet(encoded_data[i + 19], alphabet);
        err |= v16 | v17 | v18 | v19;
        output[o + 12] = (v16 << 2) | (v17 >> 4);
        output[o + 13] = (v17 << 4) | (v18 >> 2);
        output[o + 14] = (v18 << 6) | v19;

        let v20 = base64_char_to_sextet(encoded_data[i + 20], alphabet);
        let v21 = base64_char_to_sextet(encoded_data[i + 21], alphabet);
        let v22 = base64_char_to_sextet(encoded_data[i + 22], alphabet);
        let v23 = base64_char_to_sextet(encoded_data[i + 23], alphabet);
        err |= v20 | v21 | v22 | v23;
        output[o + 15] = (v20 << 2) | (v21 >> 4);
        output[o + 16] = (v21 << 4) | (v22 >> 2);
        output[o + 17] = (v22 << 6) | v23;

        let v24 = base64_char_to_sextet(encoded_data[i + 24], alphabet);
        let v25 = base64_char_to_sextet(encoded_data[i + 25], alphabet);
        let v26 = base64_char_to_sextet(encoded_data[i + 26], alphabet);
        let v27 = base64_char_to_sextet(encoded_data[i + 27], alphabet);
        err |= v24 | v25 | v26 | v27;
        output[o + 18] = (v24 << 2) | (v25 >> 4);
        output[o + 19] = (v25 << 4) | (v26 >> 2);
        output[o + 20] = (v26 << 6) | v27;

        let v28 = base64_char_to_sextet(encoded_data[i + 28], alphabet);
        let v29 = base64_char_to_sextet(encoded_data[i + 29], alphabet);
        let v30 = base64_char_to_sextet(encoded_data[i + 30], alphabet);
        let v31 = base64_char_to_sextet(encoded_data[i + 31], alphabet);
        err |= v28 | v29 | v30 | v31;
        output[o + 21] = (v28 << 2) | (v29 >> 4);
        output[o + 22] = (v29 << 4) | (v30 >> 2);
        output[o + 23] = (v30 << 6) | v31;

        i += 32;
        o += 24;
    }

    while i + 4 <= content_len {
        let v0 = base64_char_to_sextet(encoded_data[i], alphabet);
        let v1 = base64_char_to_sextet(encoded_data[i + 1], alphabet);
        let v2 = base64_char_to_sextet(encoded_data[i + 2], alphabet);
        let v3 = base64_char_to_sextet(encoded_data[i + 3], alphabet);
        err |= v0 | v1 | v2 | v3;
        output[o] = (v0 << 2) | (v1 >> 4);
        output[o + 1] = (v1 << 4) | (v2 >> 2);
        output[o + 2] = (v2 << 6) | v3;
        i += 4;
        o += 3;
    }

    let remaining = content_len - i;
    let rem2 = (remaining == 2) as u8;
    let rem3 = (remaining == 3) as u8;
    let rem0 = (remaining == 0) as u8;
    let valid_rem = rem0 | rem2 | rem3;
    err |= (1 - valid_rem) << 6;

    let rem2_mask = 0u8.wrapping_sub(rem2);
    let rem3_mask = 0u8.wrapping_sub(rem3);

    if remaining > 0 {
        let c0 = encoded_data[i];
        let c1 = if i + 1 < content_len { encoded_data[i + 1] } else { b'A' };
        let c2 = if i + 2 < content_len { encoded_data[i + 2] } else { b'A' };

        let v0 = base64_char_to_sextet(c0, alphabet);
        let v1 = base64_char_to_sextet(c1, alphabet);
        let v2 = base64_char_to_sextet(c2, alphabet);

        let m0 = 0u8.wrapping_sub((i < content_len) as u8);
        let m1 = 0u8.wrapping_sub((i + 1 < content_len) as u8);
        let m2 = 0u8.wrapping_sub((i + 2 < content_len) as u8);
        err |= (v0 & m0) | (v1 & m1) | (v2 & m2);

        // Reject non-canonical trailing bits
        // remaining == 2: bottom 4 bits of v1 are unused -> must be zero
        // remaining == 3: bottom 2 bits of v2 are unused -> must be zero
        let v1_trailing = v1 & 0x0F;
        let v2_trailing = v2 & 0x03;
        let trailing = (v1_trailing & rem2_mask) | (v2_trailing & rem3_mask);
        err |= ((trailing != 0) as u8) << 6;

        let out0 = (v0 << 2) | (v1 >> 4);
        let out1 = (v1 << 4) | (v2 >> 2);

        output[o] = out0;
        if remaining == 3 {
            output[o + 1] = out1;
        }
    }

    if err >= 64 {
        return Err(DecodeError::InvalidInput);
    }

    Ok(())
}

/// Constant-time mapping: base64 character to 6-bit value.
/// Valid characters return 0-63. Invalid characters return a value with bit 6 set (>= 64).
#[inline]
const fn base64_char_to_sextet(c: u8, alphabet: Alphabet) -> u8 {
    let not_upper = not_in_range(c, b'A', b'Z');
    let not_lower = not_in_range(c, b'a', b'z');
    let not_digit = not_in_range(c, b'0', b'9');

    let upper_val = c.wrapping_sub(b'A');
    let lower_val = c.wrapping_sub(b'a').wrapping_add(26);
    let digit_val = c.wrapping_sub(b'0').wrapping_add(52);

    let (ch_62, ch_63) = match alphabet {
        Alphabet::Standard | Alphabet::StandardNoPadding => (b'+', b'/'),
        Alphabet::Url | Alphabet::UrlNoPadding => (b'-', b'_'),
    };
    let not_62 = not_in_range(c, ch_62, ch_62);
    let not_63 = not_in_range(c, ch_63, ch_63);

    let value = (upper_val & !not_upper)
        | (lower_val & !not_lower)
        | (digit_val & !not_digit)
        | (62 & !not_62)
        | (63 & !not_63);

    let invalid = not_upper & not_lower & not_digit & not_62 & not_63;
    value | (invalid & 0x40)
}

pub(crate) const fn strip_padding_info(data: &[u8], expect_padding: bool) -> Result<(usize, usize), DecodeError> {
    let in_len = data.len();

    if !expect_padding {
        let last_is_pad = if in_len > 0 { (data[in_len - 1] == PAD) as u8 } else { 0 };
        if last_is_pad != 0 {
            return Err(DecodeError::InvalidPadding);
        }
        return Ok((in_len, 0));
    }

    // For padded input, examine up to the last 3 bytes branchlessly.
    // Valid base64 has at most 2 trailing '=' characters.
    let b0 = if in_len > 0 { data[in_len - 1] } else { 0 };
    let b1 = if in_len > 1 { data[in_len - 2] } else { 0 };
    let b2 = if in_len > 2 { data[in_len - 3] } else { 0 };

    let p0 = (b0 == PAD) as usize;
    let p1 = (b1 == PAD) as usize;
    let p2 = (b2 == PAD) as usize;

    // pad_count is the number of trailing PADs (0..3).
    let pad_count = p0 + (p0 & p1) + (p0 & p1 & p2);
    let content_len = in_len - pad_count;

    let mut err_len: u8 = 0;
    let mut err_pad: u8 = 0;

    // If padding is present, total length must be a multiple of 4.
    let has_pads = (pad_count != 0) as u8;
    let len_mod4_ok = ((in_len & 3) == 0) as u8;
    err_len |= has_pads & (1 - len_mod4_ok);

    // More than 2 padding characters is invalid.
    err_pad |= (pad_count > 2) as u8;

    if err_len != 0 {
        return Err(DecodeError::InvalidInputLength);
    }

    // Padding count must match the content length modulo 4.
    // 2 pads => content_len % 4 == 2
    // 1 pad  => content_len % 4 == 3
    let content_mod4 = content_len & 3;
    let expected_mod4 = 4usize.wrapping_sub(pad_count);
    let mod4_ok = (content_mod4 == expected_mod4) as u8;
    err_pad |= has_pads & (1 - mod4_ok);

    if err_pad != 0 {
        return Err(DecodeError::InvalidPadding);
    }

    Ok((content_len, pad_count))
}

/// Returns the size in bytes of the data after decoding from Base64.
/// Returns `None` if the length overflows `usize`.
pub(crate) const fn decoded_length(encoded_data_length: usize) -> Result<usize, DecodeError> {
    let full_blocks = encoded_data_length / 4;
    let rem = encoded_data_length % 4;

    let base = match full_blocks.checked_mul(3) {
        Some(v) => v,
        None => return Err(DecodeError::InvalidInputLength),
    };

    match rem {
        0 => Ok(base),
        2 => match base.checked_add(1) {
            Some(v) => Ok(v),
            None => Err(DecodeError::InvalidInputLength),
        },
        3 => match base.checked_add(2) {
            Some(v) => Ok(v),
            None => Err(DecodeError::InvalidInputLength),
        },
        _ => Err(DecodeError::InvalidInputLength),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // (input_bytes, alphabet, expected_encoded_str, description)
    const ENCODE_VECTORS: &[(&[u8], Alphabet, &str, &str)] = &[
        // RFC 4648 test vectors
        (b"", Alphabet::Standard, "", "RFC4648: empty"),
        (b"f", Alphabet::Standard, "Zg==", "RFC4648: 'f'"),
        (b"fo", Alphabet::Standard, "Zm8=", "RFC4648: 'fo'"),
        (b"foo", Alphabet::Standard, "Zm9v", "RFC4648: 'foo'"),
        (b"foob", Alphabet::Standard, "Zm9vYg==", "RFC4648: 'foob'"),
        (b"fooba", Alphabet::Standard, "Zm9vYmE=", "RFC4648: 'fooba'"),
        (b"foobar", Alphabet::Standard, "Zm9vYmFy", "RFC4648: 'foobar'"),
        // RFC 4648 Section 9 illustration vectors
        (
            &[0x14, 0xfb, 0x9c, 0x03, 0xd9, 0x7e],
            Alphabet::Standard,
            "FPucA9l+",
            "illustration: 6 bytes",
        ),
        (
            &[0x14, 0xfb, 0x9c, 0x03, 0xd9],
            Alphabet::Standard,
            "FPucA9k=",
            "illustration: 5 bytes",
        ),
        (
            &[0x14, 0xfb, 0x9c, 0x03],
            Alphabet::Standard,
            "FPucAw==",
            "illustration: 4 bytes",
        ),
        (b"Man", Alphabet::Standard, "TWFu", "illustration: 'Man'"),
        (b"Ma", Alphabet::Standard, "TWE=", "illustration: 'Ma'"),
        (b"M", Alphabet::Standard, "TQ==", "illustration: 'M'"),
        // Single bytes
        (b"\x00", Alphabet::Standard, "AA==", "single byte 0x00"),
        (b"\xFF", Alphabet::Standard, "/w==", "single byte 0xFF"),
        (b"\xAB", Alphabet::Standard, "qw==", "single byte 0xAB"),
        (b"\xFF", Alphabet::Url, "_w==", "single byte 0xFF URL"),
        // Two bytes
        (b"\x00\x00", Alphabet::Standard, "AAA=", "two bytes 0x00"),
        (b"\xFF\xFF", Alphabet::Standard, "//8=", "two bytes 0xFF"),
        // Three bytes
        (b"bar", Alphabet::Standard, "YmFy", "three bytes 'bar'"),
        // No padding
        (b"f", Alphabet::StandardNoPadding, "Zg", "no-pad: 'f'"),
        (b"fo", Alphabet::StandardNoPadding, "Zm8", "no-pad: 'fo'"),
        (b"foo", Alphabet::StandardNoPadding, "Zm9v", "no-pad: 'foo'"),
        // URL-safe
        (b"\xFF\xEC\x20\x55\x00", Alphabet::Url, "_-wgVQA=", "URL padded"),
        (b"\xFF\xEC\x20\x55\x00", Alphabet::UrlNoPadding, "_-wgVQA", "URL no-pad"),
    ];

    // (encoded_str, alphabet, expected_bytes, description)
    const DECODE_VECTORS: &[(&[u8], Alphabet, &[u8], &str)] = &[
        (b"", Alphabet::Standard, b"", "RFC4648: empty"),
        (b"Zg==", Alphabet::Standard, b"f", "RFC4648: 'f'"),
        (b"Zm8=", Alphabet::Standard, b"fo", "RFC4648: 'fo'"),
        (b"Zm9v", Alphabet::Standard, b"foo", "RFC4648: 'foo'"),
        (b"Zm9vYg==", Alphabet::Standard, b"foob", "RFC4648: 'foob'"),
        (b"Zm9vYmE=", Alphabet::Standard, b"fooba", "RFC4648: 'fooba'"),
        (b"Zm9vYmFy", Alphabet::Standard, b"foobar", "RFC4648: 'foobar'"),
        (b"AA==", Alphabet::Standard, b"\x00", "single byte 0x00"),
        (b"/w==", Alphabet::Standard, b"\xFF", "single byte 0xFF"),
        (b"qw==", Alphabet::Standard, b"\xAB", "single byte 0xAB"),
        (b"AAA=", Alphabet::Standard, b"\x00\x00", "two bytes 0x00"),
        (b"//8=", Alphabet::Standard, b"\xFF\xFF", "two bytes 0xFF"),
        (b"Zg", Alphabet::StandardNoPadding, b"f", "no-pad: 'f'"),
        (b"Zm8", Alphabet::StandardNoPadding, b"fo", "no-pad: 'fo'"),
        (b"Zm9v", Alphabet::StandardNoPadding, b"foo", "no-pad: 'foo'"),
        (b"_-wgVQA=", Alphabet::Url, b"\xFF\xEC\x20\x55\x00", "URL padded"),
        (b"_-wgVQA", Alphabet::UrlNoPadding, b"\xFF\xEC\x20\x55\x00", "URL no-pad"),
    ];

    // (encoded_str, alphabet, expected_error, description)
    const DECODE_ERROR_VECTORS: &[(&[u8], Alphabet, DecodeError, &str)] = &[
        (b"!!", Alphabet::Standard, DecodeError::InvalidInput, "two invalid chars"),
        (
            b"Zg!!",
            Alphabet::Standard,
            DecodeError::InvalidInput,
            "valid prefix + invalid suffix",
        ),
        (b"!A==", Alphabet::Standard, DecodeError::InvalidInput, "invalid first char"),
        (b"A", Alphabet::Standard, DecodeError::InvalidInputLength, "single char"),
        (b"AAAAA", Alphabet::Standard, DecodeError::InvalidInputLength, "5 chars"),
        (b"Z===", Alphabet::Standard, DecodeError::InvalidPadding, "1 content + 3 pads"),
        (
            b"Zg=A",
            Alphabet::Standard,
            DecodeError::InvalidInput,
            "interior '=' before valid char",
        ),
        (
            b"Zg===",
            Alphabet::Standard,
            DecodeError::InvalidInputLength,
            "valid + 3 pads (invalid length)",
        ),
        (
            b"Zg==",
            Alphabet::StandardNoPadding,
            DecodeError::InvalidPadding,
            "no-pad rejects padding",
        ),
        (
            b"Zm8=",
            Alphabet::StandardNoPadding,
            DecodeError::InvalidPadding,
            "no-pad rejects padding 2 bytes",
        ),
        (b"=", Alphabet::Standard, DecodeError::InvalidInputLength, "single pad only"),
        (b"==", Alphabet::Standard, DecodeError::InvalidInputLength, "double pad only"),
        (b"A===", Alphabet::Standard, DecodeError::InvalidPadding, "1 content + 3 pads"),
    ];

    // (data_length, padding, expected_result, description)
    const ENCODED_LENGTH_VECTORS: &[(usize, bool, Option<usize>, &str)] = &[
        (0, true, Some(0), "empty padded"),
        (1, true, Some(4), "1 byte padded"),
        (2, true, Some(4), "2 bytes padded"),
        (3, true, Some(4), "3 bytes padded"),
        (4, true, Some(8), "4 bytes padded"),
        (5, true, Some(8), "5 bytes padded"),
        (0, false, Some(0), "empty unpadded"),
        (1, false, Some(2), "1 byte unpadded"),
        (2, false, Some(3), "2 bytes unpadded"),
        (3, false, Some(4), "3 bytes unpadded"),
        (4, false, Some(6), "4 bytes unpadded"),
        (5, false, Some(7), "5 bytes unpadded"),
        (usize::MAX, true, None, "overflow padded"),
        (usize::MAX, false, None, "overflow unpadded"),
        (usize::MAX / 4 * 3 + 3, true, None, "overflow at boundary"),
    ];

    // (initial_string, input_bytes, alphabet, expected_output, description)
    const ENCODE_INTO_STRING_VECTORS: &[(&str, &[u8], Alphabet, &str, &str)] = &[
        ("", b"", Alphabet::Standard, "", "empty"),
        ("prefix", b"", Alphabet::Standard, "prefix", "empty data with prefix"),
        ("", b"\x00", Alphabet::Standard, "AA==", "single null byte"),
        ("", b"fo", Alphabet::Standard, "Zm8=", "two bytes 'fo'"),
        ("", b"foo", Alphabet::Standard, "Zm9v", "three bytes 'foo'"),
        ("", b"f", Alphabet::StandardNoPadding, "Zg", "no-pad: 'f'"),
        ("", b"fo", Alphabet::StandardNoPadding, "Zm8", "no-pad: 'fo'"),
        ("", b"foo", Alphabet::StandardNoPadding, "Zm9v", "no-pad: 'foo'"),
        ("", b"\xFF\xEC\x20\x55\x00", Alphabet::Url, "_-wgVQA=", "URL padded"),
        ("", b"\xFF\xEC\x20\x55\x00", Alphabet::UrlNoPadding, "_-wgVQA", "URL no-pad"),
    ];

    const ALL_ALPHABETS: &[Alphabet] = &[
        Alphabet::Standard,
        Alphabet::StandardNoPadding,
        Alphabet::Url,
        Alphabet::UrlNoPadding,
    ];

    const ROUNDTRIP_SIZES: &[usize] = &[
        0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 23, 24, 25, 31, 32, 33, 47, 48, 49, 63, 64, 65, 127, 128, 129,
    ];

    const SIMD_BOUNDARY_SIZES: &[usize] = &[
        22, 23, 24, 25, 26, 30, 31, 32, 33, 34, 46, 47, 48, 49, 50, 62, 63, 64, 65, 66,
    ];

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

        // 32-byte input with invalid char at position 31 (SIMD boundary via dispatch)
        let mut input = alloc::vec![b'A'; 32];
        input[31] = b'!';
        assert_eq!(decode(&input, Alphabet::Standard), Err(DecodeError::InvalidInput));
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
        encode_into_string(&mut s, b"foo", Alphabet::Standard);
        assert_eq!(s, "~~Zm9v");
        encode_into_string(&mut s, b"bar", Alphabet::Standard);
        assert_eq!(s, "~~Zm9vYmFy");

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
        encode_into_constant_time(&mut encoded, &data, Alphabet::Standard).unwrap();
        let mut decoded = alloc::vec![0u8; size];
        decode_into_constant_time(&mut decoded, &encoded, Alphabet::Standard).unwrap();
        assert_eq!(decoded, data, "4096 zeroes constant-time");

        let data = alloc::vec![0xFFu8; size];
        let mut encoded = alloc::vec![0u8; elen];
        encode_into_constant_time(&mut encoded, &data, Alphabet::Standard).unwrap();
        decode_into_constant_time(&mut decoded, &encoded, Alphabet::Standard).unwrap();
        assert_eq!(decoded, data, "4096 0xFF constant-time");

        let data: Vec<u8> = (0..=255).cycle().take(size).collect();
        let encoded = encode(&data, Alphabet::Standard);
        let decoded = decode(encoded.as_bytes(), Alphabet::Standard).unwrap();
        assert_eq!(decoded, data, "4096 cycle dispatch");

        let data: Vec<u8> = (0..=255).collect();
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Standard);
        let decoded = decode(s.as_bytes(), Alphabet::Standard).unwrap();
        assert_eq!(decoded, data, "256-byte encode_into_string roundtrip");

        let data: Vec<u8> = (0..255).cycle().take(4096).collect();
        let expected = encode(&data, Alphabet::Standard);
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Standard);
        assert_eq!(s, expected, "4096-byte encode_into_string");
    }

    #[test]
    fn test_encode_all_single_bytes() {
        for byte in 0..=255u8 {
            for alphabet in &[Alphabet::Standard, Alphabet::Url] {
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
        let mut out = [0u8; 32];

        for pos in 0..32 {
            let mut input = [b'A'; 32];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Standard),
                Err(DecodeError::InvalidInput),
                "invalid char at position {pos} in 32-byte block"
            );
        }

        for pos in 0..36 {
            let mut input = [b'A'; 36];
            input[pos] = b'!';
            assert_eq!(
                decode_into_constant_time(&mut out, &input, Alphabet::Standard),
                Err(DecodeError::InvalidInput),
                "invalid char at position {pos} in 36-byte block"
            );
        }
    }

    #[test]
    fn test_decode_non_canonical_trailing_bits() {
        let mut out = [0u8; 2];

        // remaining == 2: bottom 4 bits of v1 must be zero
        for &(input, expected) in &[
            (b"/w==" as &[u8], Ok(())),
            (b"/x==" as &[u8], Err(DecodeError::InvalidInput)),
            (b"/y==" as &[u8], Err(DecodeError::InvalidInput)),
            (b"/z==" as &[u8], Err(DecodeError::InvalidInput)),
        ] {
            assert_eq!(
                decode_into_constant_time(&mut out, input, Alphabet::Standard).map(|_| ()),
                expected,
                "non-canonical (rem=2): {:?}",
                core::str::from_utf8(input)
            );
        }

        // remaining == 3: bottom 2 bits of v2 must be zero
        for &(input, expected) in &[
            (b"iYU=" as &[u8], Ok(())),
            (b"iYV=" as &[u8], Err(DecodeError::InvalidInput)),
            (b"iYW=" as &[u8], Err(DecodeError::InvalidInput)),
            (b"iYX=" as &[u8], Err(DecodeError::InvalidInput)),
        ] {
            assert_eq!(
                decode_into_constant_time(&mut out, input, Alphabet::Standard).map(|_| ()),
                expected,
                "non-canonical (rem=3): {:?}",
                core::str::from_utf8(input)
            );
        }
    }

    #[test]
    fn test_decode_rejects_interior_padding() {
        let mut out = [0u8; 4];
        assert_eq!(
            decode_into_constant_time(&mut out, b"A=AA", Alphabet::Standard),
            Err(DecodeError::InvalidInput)
        );
        assert_eq!(
            decode_into_constant_time(&mut out, b"AA=A", Alphabet::Standard),
            Err(DecodeError::InvalidInput)
        );
        assert_eq!(
            decode_into_constant_time(&mut out, b"AA==", Alphabet::StandardNoPadding),
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

            for alphabet in ALL_ALPHABETS {
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
        const DATA: [u8; 3] = [0x66, 0x6F, 0x6F];
        const B64: [u8; 4] = encode_array::<4>(&DATA, Alphabet::Standard);
        assert_eq!(&B64, b"Zm9v");

        const B64_URL: [u8; 4] = encode_array::<4>(&DATA, Alphabet::Url);
        assert_eq!(&B64_URL, b"Zm9v");

        const B64_EMPTY: [u8; 0] = encode_array::<0>(b"", Alphabet::Standard);
        assert_eq!(B64_EMPTY.len(), 0);

        const NOPAD_DATA: [u8; 2] = [0x66, 0x6F];
        const B64_NOPAD: [u8; 3] = encode_array::<3>(&NOPAD_DATA, Alphabet::StandardNoPadding);
        assert_eq!(&B64_NOPAD, b"Zm8");
    }

    #[test]
    fn test_const_decode() {
        const RESULT: Result<[u8; 3], DecodeError> = decode_array::<3>(b"Zm9v", Alphabet::Standard);
        assert_eq!(RESULT.unwrap(), [0x66, 0x6F, 0x6F]);

        const RESULT_EMPTY: Result<[u8; 0], DecodeError> = decode_array::<0>(b"", Alphabet::Standard);
        assert_eq!(RESULT_EMPTY.unwrap().len(), 0);

        const RESULT_NOPAD: Result<[u8; 2], DecodeError> = decode_array::<2>(b"Zm8", Alphabet::StandardNoPadding);
        assert_eq!(RESULT_NOPAD.unwrap(), [0x66, 0x6F]);
    }

    #[test]
    fn test_const_decode_error() {
        const ERR_INVALID: Result<[u8; 1], DecodeError> = decode_array::<1>(b"!!", Alphabet::Standard);
        assert_eq!(ERR_INVALID, Err(DecodeError::InvalidInput));

        const ERR_SIZE: Result<[u8; 0], DecodeError> = decode_array::<0>(b"Zg==", Alphabet::Standard);
        assert_eq!(ERR_SIZE, Err(DecodeError::InvalidInputLength));
    }

    #[test]
    fn test_buffer_management() {
        let mut out = [0u8; 1];
        assert_eq!(
            encode_into(&mut out, b"hello", Alphabet::Standard),
            Err(EncodeError::InvalidOutputLength)
        );

        let mut out = [0u8; 3];
        decode_into(&mut out, b"Zm9v", Alphabet::Standard).unwrap();
        assert_eq!(&out, b"foo");

        let mut out = [0u8; 2];
        assert_eq!(
            decode_into(&mut out, b"Zm9v", Alphabet::Standard),
            Err(DecodeError::InvalidInputLength)
        );
    }

    #[test]
    fn test_display_error() {
        assert_eq!(format!("{}", DecodeError::InvalidInput), "invalid base64 character");
        assert_eq!(format!("{}", DecodeError::InvalidInputLength), "invalid base64 length");
        assert_eq!(format!("{}", DecodeError::InvalidPadding), "invalid base64 padding");
        assert_eq!(
            format!("{}", EncodeError::InvalidOutputLength),
            "output buffer size must be exactly equal to decoded_len(input)"
        );
        assert_eq!(format!("{}", EncodeError::OutputOverflow), "output length overflows usize::MAX");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        struct Data(#[serde(with = "crate::serde")] Vec<u8>);

        let data = Data(b"hello world".to_vec());
        let json = ::serde_json::to_string(&data).unwrap();
        let deserialized: Data = ::serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.0, b"hello world");
    }
}
