//! Scalar and SIMD byte classification helpers.

use crate::error::{Error, Result};

/// Returns the index of the first byte in `input` that is a quote, a
/// backslash, or an unescaped control character, or `input.len()`.
#[inline]
#[allow(unsafe_code)]
#[allow(unreachable_code)]
pub(crate) fn find_string_special(input: &[u8]) -> usize {
    #[cfg(all(feature = "std", target_arch = "x86_64"))]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: the runtime check guarantees AVX2.
            return unsafe { simd_avx2::find_string_special(input) };
        }
    }

    #[cfg(all(not(feature = "std"), target_arch = "x86_64", target_feature = "avx2"))]
    {
        // SAFETY: the compile-time target feature guarantees AVX2.
        return unsafe { simd_avx2::find_string_special(input) };
    }

    #[cfg(target_arch = "aarch64")]
    {
        // SAFETY: NEON is mandatory on AArch64.
        return unsafe { simd_neon::find_string_special(input) };
    }

    scalar_find_string_special(input)
}

#[inline]
fn is_special(byte: u8) -> bool {
    byte == b'"' || byte == b'\\' || byte < 0x20
}

#[inline]
fn has_zero_byte(value: u64) -> bool {
    (value.wrapping_sub(0x0101_0101_0101_0101)) & !value & 0x8080_8080_8080_8080 != 0
}

#[inline]
pub(crate) fn scalar_find_string_special(input: &[u8]) -> usize {
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

#[cfg(all(target_arch = "x86_64", feature = "std"))]
#[path = "simd_avx2.rs"]
mod simd_avx2;

#[cfg(all(target_arch = "x86_64", not(feature = "std"), target_feature = "avx2"))]
#[path = "simd_avx2.rs"]
mod simd_avx2;

#[cfg(target_arch = "aarch64")]
#[path = "simd_neon.rs"]
mod simd_neon;

/// Validates that `bytes` is valid UTF-8 and returns it as a string.
#[inline]
#[allow(unsafe_code)]
pub(crate) fn str_from_utf8(bytes: &[u8], offset: usize) -> Result<&str> {
    if bytes.is_ascii() {
        // SAFETY: ASCII is valid UTF-8.
        return Ok(unsafe { core::str::from_utf8_unchecked(bytes) });
    }
    utf8::from_utf8(bytes).map_err(|_| Error::new(crate::error::ErrorKind::InvalidUtf8, offset))
}

/// Scans a JSON string body starting at `start` (the byte after the opening
/// quote) and returns the index of the closing quote and whether the body
/// contains an escape.
#[inline]
pub(crate) fn scan_body(input: &[u8], start: usize, max_len: Option<usize>) -> Result<(usize, bool)> {
    let mut i = start;
    let mut escaped = false;
    while i < input.len() {
        let found = find_string_special(&input[i..]);
        i += found;
        if i >= input.len() {
            return Err(Error::new(crate::error::ErrorKind::UnexpectedEof, i));
        }
        match input[i] {
            b'"' => {
                if let Some(max) = max_len
                    && i - start > max
                {
                    return Err(Error::new(crate::error::ErrorKind::StringLimitExceeded, start));
                }
                return Ok((i, escaped));
            }
            b'\\' => {
                escaped = true;
                i += 1;
                if i >= input.len() {
                    return Err(Error::new(crate::error::ErrorKind::UnexpectedEof, i));
                }
                i += 1;
            }
            byte if byte < 0x20 => {
                return Err(Error::new(crate::error::ErrorKind::ControlCharacter, i));
            }
            _ => {
                i += 1;
            }
        }
    }
    Err(Error::new(crate::error::ErrorKind::UnexpectedEof, i))
}

/// Decodes the body of a JSON string (the bytes between the quotes) into `out`.
///
/// The body must already have been scanned with [`scan_body`], so control
/// characters and the closing quote are not part of `body`. `body` is still
/// validated as UTF-8 by this function.
#[cfg(feature = "alloc")]
#[inline]
pub(crate) fn unescape_into(body: &[u8], out: &mut alloc::string::String, offset: usize) -> Result<()> {
    let text = str_from_utf8(body, offset)?;
    unescape_text_into(text, out, offset)
}

#[cfg(feature = "alloc")]
#[inline]
#[allow(unsafe_code)]
pub(crate) fn unescape_into_unchecked(body: &[u8], out: &mut alloc::string::String, offset: usize) -> Result<()> {
    // SAFETY: the caller guarantees that `body` is a valid UTF-8 substring.
    let text = unsafe { core::str::from_utf8_unchecked(body) };
    unescape_text_into(text, out, offset)
}

#[cfg(feature = "alloc")]
#[inline]
fn unescape_text_into(text: &str, out: &mut alloc::string::String, offset: usize) -> Result<()> {
    out.reserve(text.len());
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'\\' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\\' {
                i += 1;
            }
            // A backslash byte cannot occur inside a UTF-8 multi-byte
            // sequence, so this slice boundary is a character boundary.
            out.push_str(&text[start..i]);
            continue;
        }

        i += 1;
        let escape = *bytes
            .get(i)
            .ok_or_else(|| Error::new(crate::error::ErrorKind::InvalidEscape, offset + i))?;
        i += 1;
        match escape {
            b'"' => out.push('"'),
            b'\\' => out.push('\\'),
            b'/' => out.push('/'),
            b'b' => out.push('\u{0008}'),
            b'f' => out.push('\u{000C}'),
            b'n' => out.push('\n'),
            b'r' => out.push('\r'),
            b't' => out.push('\t'),
            b'u' => {
                let (ch, consumed) = parse_unicode_escape(bytes, i, offset)?;
                i += consumed;
                out.push(ch);
            }
            _ => return Err(Error::new(crate::error::ErrorKind::InvalidEscape, offset + i - 1)),
        }
    }
    Ok(())
}

/// Scans and validates a quoted JSON string without producing its contents.
///
/// Returns the index of the closing quote. Escape sequences are validated in
/// the same pass; UTF-8 is only checked when `input_is_utf8` is false.
pub(crate) fn skip(input: &[u8], start: usize, max_len: Option<usize>, input_is_utf8: bool) -> Result<usize> {
    let mut i = start;
    while i < input.len() {
        let run = find_string_special(&input[i..]);
        if run != 0 {
            if let Some(max) = max_len
                && i + run - start > max
            {
                return Err(Error::new(crate::error::ErrorKind::StringLimitExceeded, start));
            }
            if !input_is_utf8 {
                str_from_utf8(&input[i..i + run], i)?;
            }
            i += run;
            if i >= input.len() {
                return Err(Error::new(crate::error::ErrorKind::UnexpectedEof, i));
            }
        }
        match input[i] {
            b'"' => {
                if let Some(max) = max_len
                    && i - start > max
                {
                    return Err(Error::new(crate::error::ErrorKind::StringLimitExceeded, start));
                }
                return Ok(i);
            }
            b'\\' => {
                i += 1;
                let escape = *input
                    .get(i)
                    .ok_or_else(|| Error::new(crate::error::ErrorKind::UnexpectedEof, i))?;
                i += 1;
                match escape {
                    b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {}
                    b'u' => {
                        let (_, consumed) = parse_unicode_escape(input, i, start)?;
                        i += consumed;
                    }
                    _ => return Err(Error::new(crate::error::ErrorKind::InvalidEscape, i - 1)),
                }
            }
            byte if byte < 0x20 => {
                return Err(Error::new(crate::error::ErrorKind::ControlCharacter, i));
            }
            _ => i += 1,
        }
    }
    Err(Error::new(crate::error::ErrorKind::UnexpectedEof, i))
}

/// Decodes the body of a JSON string into a byte buffer. Returns the number of
/// UTF-8 bytes written.
#[inline]
pub(crate) fn unescape_slice(body: &[u8], out: &mut [u8], offset: usize) -> Result<usize> {
    let text = str_from_utf8(body, offset)?;
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut written = 0;
    while i < bytes.len() {
        if bytes[i] != b'\\' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\\' {
                i += 1;
            }
            let run = &bytes[start..i];
            let end = written + run.len();
            if end > out.len() {
                return Err(Error::buffer_too_small());
            }
            out[written..end].copy_from_slice(run);
            written = end;
            continue;
        }

        i += 1;
        let escape = *bytes
            .get(i)
            .ok_or_else(|| Error::new(crate::error::ErrorKind::InvalidEscape, offset + i))?;
        i += 1;
        let mut buf = [0u8; 4];
        let piece: &[u8] = match escape {
            b'"' => b"\"",
            b'\\' => b"\\",
            b'/' => b"/",
            b'b' => b"\x08",
            b'f' => b"\x0c",
            b'n' => b"\n",
            b'r' => b"\r",
            b't' => b"\t",
            b'u' => {
                let (ch, consumed) = parse_unicode_escape(bytes, i, offset)?;
                i += consumed;
                ch.encode_utf8(&mut buf).as_bytes()
            }
            _ => return Err(Error::new(crate::error::ErrorKind::InvalidEscape, offset + i - 1)),
        };
        let end = written + piece.len();
        if end > out.len() {
            return Err(Error::buffer_too_small());
        }
        out[written..end].copy_from_slice(piece);
        written = end;
    }
    Ok(written)
}

fn parse_unicode_escape(bytes: &[u8], start: usize, offset: usize) -> Result<(char, usize)> {
    let first = parse_hex4(bytes, start, offset)?;
    let mut consumed = 4;
    let code = if (0xD800..=0xDBFF).contains(&first) {
        // A high surrogate must be followed by a \u low surrogate.
        let next = start + 4;
        if bytes.get(next) != Some(&b'\\') || bytes.get(next + 1) != Some(&b'u') {
            return Err(Error::new(crate::error::ErrorKind::LoneSurrogate, offset + start));
        }
        let second = parse_hex4(bytes, next + 2, offset)?;
        if !(0xDC00..=0xDFFF).contains(&second) {
            return Err(Error::new(crate::error::ErrorKind::LoneSurrogate, offset + next + 2));
        }
        consumed += 6;
        0x1_0000 + (((first as u32) - 0xD800) << 10) + ((second as u32) - 0xDC00)
    } else if (0xDC00..=0xDFFF).contains(&first) {
        return Err(Error::new(crate::error::ErrorKind::LoneSurrogate, offset + start));
    } else {
        first as u32
    };
    let ch = char::from_u32(code).ok_or_else(|| Error::new(crate::error::ErrorKind::LoneSurrogate, offset + start))?;
    Ok((ch, consumed))
}

fn parse_hex4(bytes: &[u8], start: usize, offset: usize) -> Result<u16> {
    let mut value = 0u16;
    for i in 0..4 {
        let byte = *bytes
            .get(start + i)
            .ok_or_else(|| Error::new(crate::error::ErrorKind::InvalidUnicodeEscape, offset + start + i))?;
        let digit = match byte {
            b'0'..=b'9' => (byte - b'0') as u16,
            b'a'..=b'f' => (byte - b'a' + 10) as u16,
            b'A'..=b'F' => (byte - b'A' + 10) as u16,
            _ => return Err(Error::new(crate::error::ErrorKind::InvalidUnicodeEscape, offset + start + i)),
        };
        value = (value << 4) | digit;
    }
    Ok(value)
}

/// Escapes `text` into `out`. `out` is a byte writer provided by the encoder.
#[inline]
pub(crate) fn escape_into<W: crate::encode::Write>(text: &str, out: &mut W) -> core::result::Result<(), W::Error> {
    out.reserve(text.len() + 2);
    let bytes = text.as_bytes();
    let first = find_string_special(bytes);
    if first == bytes.len() {
        if bytes.len() <= 64 {
            let mut buffer = [0u8; 66];
            buffer[0] = b'"';
            buffer[1..1 + bytes.len()].copy_from_slice(bytes);
            buffer[1 + bytes.len()] = b'"';
            return out.write_all(&buffer[..bytes.len() + 2]);
        }
        out.write_all(b"\"")?;
        out.write_all(bytes)?;
        return out.write_all(b"\"");
    }
    out.write_all(b"\"")?;
    out.write_all(&bytes[..first])?;
    let mut i = first;
    while i < bytes.len() {
        let run = find_string_special(&bytes[i..]);
        if run != 0 {
            out.write_all(&bytes[i..i + run])?;
            i += run;
            if i == bytes.len() {
                break;
            }
        }
        let byte = bytes[i];
        match byte {
            b'"' => out.write_all(b"\\\"")?,
            b'\\' => out.write_all(b"\\\\")?,
            0x08 => out.write_all(b"\\b")?,
            0x0C => out.write_all(b"\\f")?,
            b'\n' => out.write_all(b"\\n")?,
            b'\r' => out.write_all(b"\\r")?,
            b'\t' => out.write_all(b"\\t")?,
            0x00..=0x1F => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                let escaped = [
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX[(byte >> 4) as usize],
                    HEX[(byte & 0x0F) as usize],
                ];
                out.write_all(&escaped)?;
            }
            _ => unreachable!("find_string_special only returns special bytes"),
        }
        i += 1;
    }
    out.write_all(b"\"")
}
