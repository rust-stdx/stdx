#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Fast hex encoding and decoding with SIMD acceleration, constant-time
//! operations, and `const fn` support.
//!
//! Encoding supports lowercase (`0-9a-f`) and uppercase (`0-9A-F`)
//! alphabets via [`Alphabet`]. Decoding is case-insensitive.
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
//! [`encode_array`] and [`decode_array`] are `const fn`, enabling hex
//! encoding and decoding at compile time.
//!
//! # Examples
//!
//! ```rust
//! let encoded = hex::encode(b"hello");
//! assert_eq!(encoded, "68656c6c6f");
//!
//! let decoded = hex::decode(b"68656c6c6f").unwrap();
//! assert_eq!(decoded, b"hello");
//!
//! let upper = hex::encode_with_alphabet(b"hello", hex::Alphabet::Upper);
//! assert_eq!(upper, "68656C6C6F");
//! ```

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(all(feature = "serde", any(feature = "alloc", test)))]
mod serde;

#[cfg(target_arch = "aarch64")]
mod hex_neon;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod hex_avx2;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod hex_wasm_simd128;

const ALPHABET_LOWER: [u8; 16] = *b"0123456789abcdef";
const ALPHABET_UPPER: [u8; 16] = *b"0123456789ABCDEF";

/// The hex character set used for encoding.
///
/// | Variant  | Alphabet                    |
/// |----------|-----------------------------|
/// | `Lower`  | `0123456789abcdef`          |
/// | `Upper`  | `0123456789ABCDEF`          |
///
/// Decoding accepts any case regardless of the encoding alphabet.
///
/// # Example
///
/// ```rust
/// let encoded = hex::encode_with_alphabet(b"\xAB", hex::Alphabet::Lower);
/// assert_eq!(encoded, "ab");
///
/// let encoded = hex::encode_with_alphabet(b"\xAB", hex::Alphabet::Upper);
/// assert_eq!(encoded, "AB");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alphabet {
    Lower,
    Upper,
}

/// Errors that can occur during hex decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// The input contains a character that is not a valid hex digit
    /// (`0-9`, `a-f`, `A-F`).
    InvalidInput,
    /// The input length is odd. Hex encoding requires pairs of characters.
    InvalidInputLength,
    /// The output buffer length does not match `input.len() / 2`.
    InvalidOutputLength,
}

/// Errors that can occur during hex encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeError {
    /// The output buffer length does not match `input.len() * 2`.
    InvalidOutputLength,
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInput => f.write_str("invalid hex character"),
            Self::InvalidInputLength => f.write_str("odd number of hex characters"),
            Self::InvalidOutputLength => f.write_str("output buffer size must be equal to input.len() / 2"),
        }
    }
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidOutputLength => f.write_str("output buffer size must be equal to input.len() * 2"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

#[cfg(feature = "std")]
impl std::error::Error for EncodeError {}

/// Encodes bytes to a lowercase hex string.
///
/// This is a convenience wrapper around [`encode_with_alphabet`] using
/// [`Alphabet::Lower`].
///
/// # Example
///
/// ```rust
/// assert_eq!(hex::encode(b"hello"), "68656c6c6f");
/// ```
#[cfg(any(feature = "alloc", test))]
#[inline]
pub fn encode(data: impl AsRef<[u8]>) -> alloc::string::String {
    encode_with_alphabet(data.as_ref(), Alphabet::Lower)
}

/// Encodes bytes to a hex string using the given [`Alphabet`].
///
/// # Example
///
/// ```rust
/// assert_eq!(hex::encode_with_alphabet(b"hello", hex::Alphabet::Upper), "68656C6C6F");
/// ```
#[cfg(any(feature = "alloc", test))]
#[inline]
pub fn encode_with_alphabet(data: impl AsRef<[u8]>, alphabet: Alphabet) -> alloc::string::String {
    let data = data.as_ref();
    let mut output = alloc::vec![0u8; data.len() * 2];
    encode_into(&mut output, data, alphabet).unwrap();
    unsafe { alloc::string::String::from_utf8_unchecked(output) }
}

/// Encodes `data` into a fixed-size array at compile time.
///
/// The generic parameter `OUT` is the output array length. It must be exactly
/// `data.len() * 2` long or a panic is raised.
///
/// # Panics
///
/// This function panics if `OUT != data.len() * 2`.
///
/// # Example
///
/// ```rust
/// const HASH: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
/// const HEX: [u8; 8] = hex::encode_array::<8>(&HASH, hex::Alphabet::Lower);
/// assert_eq!(&HEX, b"deadbeef");
/// ```
pub const fn encode_array<const OUT: usize>(data: &[u8], alphabet: Alphabet) -> [u8; OUT] {
    let mut result = [0u8; OUT];
    match encode_into_constant_time(&mut result, data, alphabet) {
        Ok(_) => {}
        Err(_) => panic!("output buffer size is not valid"),
    };
    result
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
/// Returns [`EncodeError::InvalidOutputLength`] if `output.len() != data.len() * 2`.
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 10];
/// hex::encode_into(&mut buf, b"hello", hex::Alphabet::Lower).unwrap();
/// assert_eq!(&buf, b"68656c6c6f");
/// ```
#[inline]
pub fn encode_into(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    if data.len() >= 16 {
        check_encode_output_length(data.len(), output.len())?;
        return unsafe { hex_neon::encode_into(output, data, alphabet) };
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    if data.len() >= 32 {
        check_encode_output_length(data.len(), output.len())?;
        return unsafe { hex_avx2::encode_into(output, data, alphabet) };
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    if data.len() >= 16 {
        check_encode_output_length(data.len(), output.len())?;
        return hex_wasm_simd128::encode_into(output, data, alphabet);
    }

    return encode_into_constant_time(output, data, alphabet);
}

/// Constant-time hex encoding. Processes all input data without
/// secret-dependent branches or memory accesses, making it suitable
/// for cryptographic applications.
///
/// Consumers may prefer the faster [`encode_into`] which dispatches to
/// a SIMD-accelerated path when available (non constant-time).
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 10];
/// hex::encode_into_constant_time(&mut buf, b"hello", hex::Alphabet::Lower).unwrap();
/// assert_eq!(&buf, b"68656c6c6f");
/// ```
pub const fn encode_into_constant_time(output: &mut [u8], data: &[u8], alphabet: Alphabet) -> Result<(), EncodeError> {
    match check_encode_output_length(data.len(), output.len()) {
        Ok(_) => {}
        Err(err) => return Err(err),
    };

    let mut i = 0;
    let len = data.len();

    while i + 16 <= len {
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

        let o = i * 2;
        output[o] = nibble_to_hex(b0 >> 4, alphabet);
        output[o + 1] = nibble_to_hex(b0 & 0x0F, alphabet);
        output[o + 2] = nibble_to_hex(b1 >> 4, alphabet);
        output[o + 3] = nibble_to_hex(b1 & 0x0F, alphabet);
        output[o + 4] = nibble_to_hex(b2 >> 4, alphabet);
        output[o + 5] = nibble_to_hex(b2 & 0x0F, alphabet);
        output[o + 6] = nibble_to_hex(b3 >> 4, alphabet);
        output[o + 7] = nibble_to_hex(b3 & 0x0F, alphabet);
        output[o + 8] = nibble_to_hex(b4 >> 4, alphabet);
        output[o + 9] = nibble_to_hex(b4 & 0x0F, alphabet);
        output[o + 10] = nibble_to_hex(b5 >> 4, alphabet);
        output[o + 11] = nibble_to_hex(b5 & 0x0F, alphabet);
        output[o + 12] = nibble_to_hex(b6 >> 4, alphabet);
        output[o + 13] = nibble_to_hex(b6 & 0x0F, alphabet);
        output[o + 14] = nibble_to_hex(b7 >> 4, alphabet);
        output[o + 15] = nibble_to_hex(b7 & 0x0F, alphabet);
        output[o + 16] = nibble_to_hex(b8 >> 4, alphabet);
        output[o + 17] = nibble_to_hex(b8 & 0x0F, alphabet);
        output[o + 18] = nibble_to_hex(b9 >> 4, alphabet);
        output[o + 19] = nibble_to_hex(b9 & 0x0F, alphabet);
        output[o + 20] = nibble_to_hex(b10 >> 4, alphabet);
        output[o + 21] = nibble_to_hex(b10 & 0x0F, alphabet);
        output[o + 22] = nibble_to_hex(b11 >> 4, alphabet);
        output[o + 23] = nibble_to_hex(b11 & 0x0F, alphabet);
        output[o + 24] = nibble_to_hex(b12 >> 4, alphabet);
        output[o + 25] = nibble_to_hex(b12 & 0x0F, alphabet);
        output[o + 26] = nibble_to_hex(b13 >> 4, alphabet);
        output[o + 27] = nibble_to_hex(b13 & 0x0F, alphabet);
        output[o + 28] = nibble_to_hex(b14 >> 4, alphabet);
        output[o + 29] = nibble_to_hex(b14 & 0x0F, alphabet);
        output[o + 30] = nibble_to_hex(b15 >> 4, alphabet);
        output[o + 31] = nibble_to_hex(b15 & 0x0F, alphabet);

        i += 16;
    }

    while i < len {
        let b = data[i];
        let o = i * 2;
        output[o] = nibble_to_hex(b >> 4, alphabet);
        output[o + 1] = nibble_to_hex(b & 0x0F, alphabet);
        i += 1;
    }

    Ok(())
}

#[inline]
const fn nibble_to_hex(nibble: u8, alphabet: Alphabet) -> u8 {
    let nibble = nibble & 0x0F;
    let digit_mask = (((nibble as i16) - 10) >> 8) as u8;

    let digit_val = b'0' + nibble;
    let letter_val = b'a' + nibble - 10;
    let upper_val = b'A' + nibble - 10;

    let lower_result = (digit_val & digit_mask) | (letter_val & !digit_mask);
    let upper_result = (digit_val & digit_mask) | (upper_val & !digit_mask);

    match alphabet {
        Alphabet::Lower => lower_result,
        Alphabet::Upper => upper_result,
    }
}

#[inline]
const fn check_encode_output_length(data_length: usize, output_length: usize) -> Result<(), EncodeError> {
    if data_length * 2 != output_length {
        return Err(EncodeError::InvalidOutputLength);
    }
    Ok(())
}

/// Appends the hex-encoded representation of `data` to a [`String`].
///
/// # Example
///
/// ```rust
/// let mut s = String::from("tag: ");
/// hex::encode_into_string(&mut s, b"hello", hex::Alphabet::Lower);
/// assert_eq!(s, "tag: 68656c6c6f");
/// ```
#[cfg(feature = "alloc")]
pub fn encode_into_string(output: &mut alloc::string::String, data: &[u8], alphabet: Alphabet) {
    let encoded_length = data.len() * 2;
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

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Decode
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Decodes a hex string into bytes.
///
/// Accepts any combination of uppercase and lowercase hex characters.
///
/// # Errors
///
/// Returns [`DecodeError`] if any character is not a valid or if the input has an
/// odd number of characters.
///
/// # Example
///
/// ```rust
/// let decoded = hex::decode(b"68656c6c6f").unwrap();
/// assert_eq!(decoded, b"hello");
/// ```
#[cfg(any(feature = "alloc", test))]
pub fn decode(data: impl AsRef<[u8]>) -> Result<alloc::vec::Vec<u8>, DecodeError> {
    let data = data.as_ref();
    let mut output = alloc::vec![0u8; data.len() / 2];
    decode_into(&mut output, data)?;
    Ok(output)
}

/// Decodes a hex string into a fixed-size array at compile time.
///
/// The generic parameter `OUT` is the output array length. It must be exactly
/// `data.len() / 2` bytes long or an error panic is returned.
///
/// # Example
///
/// ```rust
/// const RESULT: Result<[u8; 4], hex::DecodeError> =
///     hex::decode_array::<4>(b"deadbeef");
/// assert_eq!(RESULT.unwrap(), [0xDE, 0xAD, 0xBE, 0xEF]);
/// ```
pub const fn decode_array<const OUT: usize>(encoded_data: &[u8]) -> Result<[u8; OUT], DecodeError> {
    let mut result = [0u8; OUT];
    match decode_into_constant_time(&mut result, encoded_data) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }
    Ok(result)
}

/// Decodes a hex string into an existing buffer.
///
/// Dispatches to a SIMD-accelerated implementation (AVX2 or NEON) when
/// the target feature is available.
///
/// Accepts any combination of uppercase and lowercase hex characters.
///
/// See [`decode_into_constant_time`] for security-sensitive and cryptographic operations.
///
/// # Errors
///
/// Returns [`DecodeError`] if any character is not a valid hex digit or if the input length is odd
/// or if `output.len() != encoded_data.len() / 2`.
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 5];
/// hex::decode_into(&mut buf, b"68656c6c6f").unwrap();
/// assert_eq!(&buf, b"hello");
/// ```
pub fn decode_into(output: &mut [u8], encoded_data: &[u8]) -> Result<(), DecodeError> {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    if encoded_data.len() >= 32 {
        check_decode_input_and_output_length(encoded_data.len(), output.len())?;
        return unsafe { hex_neon::decode_into(output, encoded_data) };
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
    if encoded_data.len() >= 32 {
        check_decode_input_and_output_length(encoded_data.len(), output.len())?;
        return unsafe { hex_avx2::decode_into(output, encoded_data) };
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    if encoded_data.len() >= 32 {
        check_decode_input_and_output_length(encoded_data.len(), output.len())?;
        return hex_wasm_simd128::decode_into(output, encoded_data);
    }

    decode_into_constant_time(output, encoded_data)
}

/// Constant-time hex decoding. Processes all input data without
/// secret-dependent branches or memory accesses, making it suitable
/// for cryptographic applications.
///
/// Note that `output` is written even when `Err` is returned, which is
/// required for constant-time operation. Callers must not rely on the
/// contents of `output` when an error is returned.
///
/// Consumers may prefer the faster [`decode_into`] which dispatches to
/// a SIMD-accelerated path when available (non constant-time).
///
/// # Example
///
/// ```rust
/// let mut buf = [0u8; 5];
/// hex::decode_into_constant_time(&mut buf, b"68656c6c6f").unwrap();
/// assert_eq!(&buf, b"hello");
/// ```
pub const fn decode_into_constant_time(output: &mut [u8], encoded_data: &[u8]) -> Result<(), DecodeError> {
    match check_decode_input_and_output_length(encoded_data.len(), output.len()) {
        Ok(_) => {}
        Err(err) => return Err(err),
    };

    let in_len = encoded_data.len();
    let mut i = 0;
    let mut err: u8 = 0;

    while i + 32 <= in_len {
        let h0 = nibble_from_hex(encoded_data[i]);
        let l0 = nibble_from_hex(encoded_data[i + 1]);
        err |= h0 | l0;
        output[i / 2] = (h0 << 4) | l0;

        let h1 = nibble_from_hex(encoded_data[i + 2]);
        let l1 = nibble_from_hex(encoded_data[i + 3]);
        err |= h1 | l1;
        output[i / 2 + 1] = (h1 << 4) | l1;

        let h2 = nibble_from_hex(encoded_data[i + 4]);
        let l2 = nibble_from_hex(encoded_data[i + 5]);
        err |= h2 | l2;
        output[i / 2 + 2] = (h2 << 4) | l2;

        let h3 = nibble_from_hex(encoded_data[i + 6]);
        let l3 = nibble_from_hex(encoded_data[i + 7]);
        err |= h3 | l3;
        output[i / 2 + 3] = (h3 << 4) | l3;

        let h4 = nibble_from_hex(encoded_data[i + 8]);
        let l4 = nibble_from_hex(encoded_data[i + 9]);
        err |= h4 | l4;
        output[i / 2 + 4] = (h4 << 4) | l4;

        let h5 = nibble_from_hex(encoded_data[i + 10]);
        let l5 = nibble_from_hex(encoded_data[i + 11]);
        err |= h5 | l5;
        output[i / 2 + 5] = (h5 << 4) | l5;

        let h6 = nibble_from_hex(encoded_data[i + 12]);
        let l6 = nibble_from_hex(encoded_data[i + 13]);
        err |= h6 | l6;
        output[i / 2 + 6] = (h6 << 4) | l6;

        let h7 = nibble_from_hex(encoded_data[i + 14]);
        let l7 = nibble_from_hex(encoded_data[i + 15]);
        err |= h7 | l7;
        output[i / 2 + 7] = (h7 << 4) | l7;

        let h8 = nibble_from_hex(encoded_data[i + 16]);
        let l8 = nibble_from_hex(encoded_data[i + 17]);
        err |= h8 | l8;
        output[i / 2 + 8] = (h8 << 4) | l8;

        let h9 = nibble_from_hex(encoded_data[i + 18]);
        let l9 = nibble_from_hex(encoded_data[i + 19]);
        err |= h9 | l9;
        output[i / 2 + 9] = (h9 << 4) | l9;

        let h10 = nibble_from_hex(encoded_data[i + 20]);
        let l10 = nibble_from_hex(encoded_data[i + 21]);
        err |= h10 | l10;
        output[i / 2 + 10] = (h10 << 4) | l10;

        let h11 = nibble_from_hex(encoded_data[i + 22]);
        let l11 = nibble_from_hex(encoded_data[i + 23]);
        err |= h11 | l11;
        output[i / 2 + 11] = (h11 << 4) | l11;

        let h12 = nibble_from_hex(encoded_data[i + 24]);
        let l12 = nibble_from_hex(encoded_data[i + 25]);
        err |= h12 | l12;
        output[i / 2 + 12] = (h12 << 4) | l12;

        let h13 = nibble_from_hex(encoded_data[i + 26]);
        let l13 = nibble_from_hex(encoded_data[i + 27]);
        err |= h13 | l13;
        output[i / 2 + 13] = (h13 << 4) | l13;

        let h14 = nibble_from_hex(encoded_data[i + 28]);
        let l14 = nibble_from_hex(encoded_data[i + 29]);
        err |= h14 | l14;
        output[i / 2 + 14] = (h14 << 4) | l14;

        let h15 = nibble_from_hex(encoded_data[i + 30]);
        let l15 = nibble_from_hex(encoded_data[i + 31]);
        err |= h15 | l15;
        output[i / 2 + 15] = (h15 << 4) | l15;

        i += 32;
    }

    while i < in_len {
        let h = nibble_from_hex(encoded_data[i]);
        let l = nibble_from_hex(encoded_data[i + 1]);
        err |= h | l;
        output[i / 2] = (h << 4) | l;
        i += 2;
    }

    if err & 0xF0 != 0 {
        return Err(DecodeError::InvalidInput);
    }

    Ok(())
}

#[inline]
const fn nibble_from_hex(c: u8) -> u8 {
    let is_digit = ((((c as i16) - (b'0' as i16)) | ((b'9' as i16) - (c as i16))) >> 8) as u8;
    let is_lower = ((((c as i16) - (b'a' as i16)) | ((b'f' as i16) - (c as i16))) >> 8) as u8;
    let is_upper = ((((c as i16) - (b'A' as i16)) | ((b'F' as i16) - (c as i16))) >> 8) as u8;

    let digit_val = c.wrapping_sub(b'0');
    let lower_val = c.wrapping_sub(b'a').wrapping_add(10);
    let upper_val = c.wrapping_sub(b'A').wrapping_add(10);

    let value = (digit_val & !is_digit) | (lower_val & !is_lower) | (upper_val & !is_upper);
    let invalid = is_digit & is_lower & is_upper;

    value | (invalid & 0xF0)
}

#[inline]
const fn check_decode_input_and_output_length(encoded_length: usize, output_length: usize) -> Result<(), DecodeError> {
    if encoded_length % 2 != 0 {
        return Err(DecodeError::InvalidInputLength);
    }
    if output_length != encoded_length / 2 {
        return Err(DecodeError::InvalidOutputLength);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty() {
        assert_eq!(encode(b""), "");
        let mut out = [0u8; 0];
        encode_into(&mut out, b"", Alphabet::Lower).unwrap();
    }

    #[test]
    fn encode_single_byte() {
        assert_eq!(encode(b"\x00"), "00");
        assert_eq!(encode(b"\xFF"), "ff");
        assert_eq!(encode(b"\xAB"), "ab");
        assert_eq!(encode_with_alphabet(b"\xAB", Alphabet::Upper), "AB");
    }

    #[test]
    fn encode_multiple_bytes() {
        assert_eq!(encode(b"hello"), "68656c6c6f");
        assert_eq!(encode_with_alphabet(b"hello", Alphabet::Upper), "68656C6C6F");
        assert_eq!(
            encode(b"\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xAA\xBB\xCC\xDD\xEE\xFF"),
            "00112233445566778899aabbccddeeff"
        );
    }

    #[test]
    fn encode_all_bytes() {
        let data: Vec<u8> = (0..=255).collect();
        let hex = encode(&data);
        assert_eq!(hex.len(), 512);
        for (i, &b) in data.iter().enumerate() {
            let hi = ALPHABET_LOWER[(b >> 4) as usize];
            let lo = ALPHABET_LOWER[(b & 0x0F) as usize];
            assert_eq!(hex.as_bytes()[i * 2], hi);
            assert_eq!(hex.as_bytes()[i * 2 + 1], lo);
        }
    }

    #[test]
    fn encode_into_exact_buffer() {
        let mut out = [0u8; 4];
        encode_into(&mut out, b"\xDE\xAD", Alphabet::Upper).unwrap();
        assert_eq!(&out, b"DEAD");
    }

    #[test]
    fn decode_empty() {
        assert_eq!(decode(b"").unwrap(), b"");
    }

    #[test]
    fn decode_single_byte() {
        assert_eq!(decode(b"00").unwrap(), b"\x00");
        assert_eq!(decode(b"ff").unwrap(), b"\xFF");
        assert_eq!(decode(b"FF").unwrap(), b"\xFF");
        assert_eq!(decode(b"ab").unwrap(), b"\xAB");
        assert_eq!(decode(b"AB").unwrap(), b"\xAB");
    }

    #[test]
    fn decode_multiple_bytes() {
        assert_eq!(decode(b"68656c6c6f").unwrap(), b"hello");
        assert_eq!(
            decode(b"00112233445566778899AABBCCDDEEFF").unwrap(),
            b"\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xAA\xBB\xCC\xDD\xEE\xFF"
        );
    }

    #[test]
    fn decode_into_exact_buffer() {
        let mut out = [0u8; 2];
        decode_into(&mut out, b"DEAD").unwrap();
        assert_eq!(&out, b"\xDE\xAD");
    }

    #[test]
    fn decode_invalid_character() {
        assert_eq!(decode(b"0g"), Err(DecodeError::InvalidInput));
        assert_eq!(decode(b"GG"), Err(DecodeError::InvalidInput));
        assert_eq!(decode(b"  "), Err(DecodeError::InvalidInput));
    }

    #[test]
    fn decode_odd_length() {
        assert_eq!(decode(b"0"), Err(DecodeError::InvalidInputLength));
        assert_eq!(decode(b"abc"), Err(DecodeError::InvalidInputLength));
    }

    #[test]
    fn decode_trailing_invalid_in_large_buffer() {
        let mut input = alloc::vec![b'0'; 64];
        input[63] = b'g';
        assert_eq!(decode(&input), Err(DecodeError::InvalidInput));
    }

    #[test]
    fn roundtrip() {
        let data: Vec<u8> = (0..=255).cycle().take(1024).collect();
        let hex = encode(&data);
        let decoded = decode(hex.as_bytes()).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn roundtrip_upper() {
        let data: Vec<u8> = (0..=255).cycle().take(1024).collect();
        let hex = encode_with_alphabet(&data, Alphabet::Upper);
        let decoded = decode(&hex).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn roundtrip_various_sizes() {
        for len in [0, 1, 2, 3, 4, 5, 15, 16, 17, 31, 32, 33, 63, 64, 65, 95, 96] {
            let data: Vec<u8> = (0..len as u8).collect();
            let hex = encode(&data);
            let decoded = decode(hex.as_bytes()).unwrap();
            assert_eq!(decoded, data, "roundtrip failed for len={}", len);
        }
    }

    #[test]
    fn decode_case_insensitivity() {
        assert_eq!(decode(b"abcdef"), decode(b"ABCDEF"));
        assert_eq!(decode(b"AbCdEf"), decode(b"aBcDeF"));
    }

    #[test]
    fn rfc4648_test_vectors_encode() {
        let vectors = [
            (b"" as &[u8], ""),
            (b"f", "66"),
            (b"fo", "666F"),
            (b"foo", "666F6F"),
            (b"foob", "666F6F62"),
            (b"fooba", "666F6F6261"),
            (b"foobar", "666F6F626172"),
        ];
        for (input, expected) in &vectors {
            assert_eq!(encode_with_alphabet(input, Alphabet::Upper), *expected);
        }
    }

    #[test]
    fn rfc4648_test_vectors_decode() {
        let vectors = [
            ("", b"" as &[u8]),
            ("66", b"f"),
            ("666F", b"fo"),
            ("666F6F", b"foo"),
            ("666F6F62", b"foob"),
            ("666F6F6261", b"fooba"),
            ("666F6F626172", b"foobar"),
        ];
        for (hex_str, expected) in &vectors {
            assert_eq!(decode(hex_str.as_bytes()).unwrap(), *expected);
        }
    }

    #[test]
    fn rfc4648_test_vectors_lowercase() {
        let vectors = [
            ("66", b"f" as &[u8]),
            ("666f", b"fo" as &[u8]),
            ("666f6f", b"foo" as &[u8]),
            ("666f6f62", b"foob" as &[u8]),
            ("666f6f6261", b"fooba" as &[u8]),
            ("666f6f626172", b"foobar" as &[u8]),
        ];
        for (hex_str, expected) in &vectors {
            assert_eq!(decode(hex_str.as_bytes()).unwrap(), *expected);
        }
    }

    #[test]
    fn simd_boundary_nonuniform() {
        let sizes = [
            0, 1, 2, 3, 15, 16, 17, 31, 32, 33, 63, 64, 65, 95, 96, 127, 128, 129, 255, 256, 257,
        ];
        for &len in &sizes {
            let data: Vec<u8> = (0..len)
                .map(|i: usize| (i.wrapping_mul(17).wrapping_add(0xAB)) as u8)
                .collect();
            let hex = encode(&data);
            let decoded = decode(hex.as_bytes()).unwrap();
            assert_eq!(decoded, data, "non-uniform roundtrip failed for len={}", len);
        }
    }

    #[test]
    fn decode_into_too_small() {
        let mut out = [0u8; 1];
        assert_eq!(decode_into(&mut out, b"0000"), Err(DecodeError::InvalidOutputLength));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn encode_into_panics_on_too_small() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let mut out = [0u8; 1];
        let result = catch_unwind(AssertUnwindSafe(|| {
            encode_into(&mut out, b"hello", Alphabet::Lower).unwrap();
        }));
        assert!(result.is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        struct Data(#[serde(with = "crate::serde")] Vec<u8>);

        let data = Data(b"hello world".to_vec());
        let json = ::serde_json::to_string(&data).unwrap();
        assert_eq!(json, "\"68656c6c6f20776f726c64\"");
        let deserialized: Data = ::serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.0, b"hello world");
    }

    #[test]
    fn const_encode() {
        const DATA: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
        const HEX: [u8; 8] = encode_array::<8>(&DATA, Alphabet::Lower);
        const HEX_UPPER: [u8; 8] = encode_array::<8>(&DATA, Alphabet::Upper);
        assert_eq!(&HEX, b"deadbeef");
        assert_eq!(&HEX_UPPER, b"DEADBEEF");
    }

    #[test]
    fn const_encode_empty() {
        const HEX: [u8; 0] = encode_array::<0>(b"", Alphabet::Lower);
        assert_eq!(HEX.len(), 0);
    }

    #[test]
    fn const_decode() {
        const RESULT: Result<[u8; 4], DecodeError> = decode_array::<4>(b"deadbeef");
        assert_eq!(RESULT.unwrap(), [0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn const_decode_empty() {
        const RESULT: Result<[u8; 0], DecodeError> = decode_array::<0>(b"");
        assert_eq!(RESULT.unwrap().len(), 0);
    }

    #[test]
    fn const_decode_upper() {
        const RESULT: Result<[u8; 4], DecodeError> = decode_array::<4>(b"DEADBEEF");
        assert_eq!(RESULT.unwrap(), [0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn const_decode_invalid_character() {
        const ERR: Result<[u8; 1], DecodeError> = decode_array::<1>(b"0g");
        assert_eq!(ERR, Err(DecodeError::InvalidInput));
    }

    #[test]
    fn const_decode_odd_length() {
        const ERR: Result<[u8; 0], DecodeError> = decode_array::<0>(b"0");
        assert_eq!(ERR, Err(DecodeError::InvalidInputLength));
    }

    #[test]
    fn const_decode_wrong_output_size() {
        const ERR: Result<[u8; 2], DecodeError> = decode_array::<2>(b"00");
        assert_eq!(ERR, Err(DecodeError::InvalidOutputLength));
    }

    #[test]
    fn encode_into_string_empty() {
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, b"", Alphabet::Lower);
        assert_eq!(s, "");
    }

    #[test]
    fn encode_into_string_empty_data_nonempty_output() {
        let mut s = alloc::string::String::from("prefix");
        encode_into_string(&mut s, b"", Alphabet::Lower);
        assert_eq!(s, "prefix");
    }

    #[test]
    fn encode_into_string_single_byte() {
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, b"\x00", Alphabet::Lower);
        assert_eq!(s, "00");
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, b"\xFF", Alphabet::Upper);
        assert_eq!(s, "FF");
    }

    #[test]
    fn encode_into_string_multiple_bytes() {
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, b"hello", Alphabet::Lower);
        assert_eq!(s, "68656c6c6f");
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, b"hello", Alphabet::Upper);
        assert_eq!(s, "68656C6C6F");
    }

    #[test]
    fn encode_into_string_append() {
        let mut s = alloc::string::String::from("~~");
        encode_into_string(&mut s, b"\xDE\xAD", Alphabet::Lower);
        assert_eq!(s, "~~dead");
        encode_into_string(&mut s, b"\xBE\xEF", Alphabet::Lower);
        assert_eq!(s, "~~deadbeef");
    }

    #[test]
    fn encode_into_string_large() {
        let data: Vec<u8> = (0..255).cycle().take(4096).collect();
        let expected = encode_with_alphabet(&data, Alphabet::Lower);
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Lower);
        assert_eq!(s, expected);
    }

    #[test]
    fn encode_into_string_roundtrip() {
        let data: Vec<u8> = (0..=255).collect();
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Lower);
        let decoded = decode(s.as_bytes()).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn encode_into_string_small_boundary() {
        let mut s = alloc::string::String::new();
        encode_into_string(
            &mut s,
            b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F",
            Alphabet::Lower,
        );
        assert_eq!(s, "000102030405060708090a0b0c0d0e0f");
    }

    #[test]
    fn encode_into_string_all_alphabets() {
        let data = b"hello world";
        for alphabet in &[Alphabet::Lower, Alphabet::Upper] {
            let expected = encode_with_alphabet(data, *alphabet);
            let mut s = alloc::string::String::new();
            encode_into_string(&mut s, data, *alphabet);
            assert_eq!(s, expected, "mismatch for alphabet {alphabet:?}");
        }
    }

    #[test]
    fn encode_into_string_rfc4648_vectors() {
        let vectors = [
            (b"" as &[u8], "", ""),
            (b"f", "66", "66"),
            (b"fo", "666f", "666F"),
            (b"foo", "666f6f", "666F6F"),
            (b"foob", "666f6f62", "666F6F62"),
            (b"fooba", "666f6f6261", "666F6F6261"),
            (b"foobar", "666f6f626172", "666F6F626172"),
        ];
        for (input, expected_lower, expected_upper) in &vectors {
            let mut s = alloc::string::String::new();
            encode_into_string(&mut s, input, Alphabet::Lower);
            assert_eq!(s, *expected_lower);
            let mut s = alloc::string::String::new();
            encode_into_string(&mut s, input, Alphabet::Upper);
            assert_eq!(s, *expected_upper);
        }
    }

    #[test]
    fn encode_into_string_exact_stack_capacity() {
        let data: Vec<u8> = (0..128).collect();
        let expected = encode(&data);
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Lower);
        assert_eq!(s, expected);
    }

    #[test]
    fn encode_into_string_exceeds_stack_capacity() {
        let data: Vec<u8> = (0..129).collect();
        let expected = encode(&data);
        let mut s = alloc::string::String::new();
        encode_into_string(&mut s, &data, Alphabet::Lower);
        assert_eq!(s, expected);
    }
}
