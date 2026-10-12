//! Quoted-printable engines (RFC 2045 section 6.7, RFC 2047 section 4.2).
//!
//! Four configurations are provided: [`BODY`] and [`BINARY`] encode message
//! bodies and non-text parts respectively, while [`Q_TEXT`] and [`Q_PHRASE`]
//! produce the payload of an RFC 2047 `Q` encoded word. Everything here is
//! safe Rust and allocation free apart from the caller's output buffer.

use std::io::{self, Write};

const HEX: &[u8; 16] = b"0123456789ABCDEF";
/// RFC 2045 section 6.7 limits encoded lines to 76 characters.
const MAX_LINE: usize = 76;

/// A quoted-printable configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuotedPrintable {
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Text body: CRLF is kept, bare CR/LF are normalised, trailing
    /// whitespace is encoded.
    Body,
    /// Non-text part: CR and LF are encoded, no hard line breaks occur.
    Binary,
    /// RFC 2047 `Q` payload for unstructured text.
    QText,
    /// RFC 2047 `Q` payload for a phrase (display name); the printable set is
    /// restricted to the characters RFC 2047 section 5 allows in a phrase.
    QPhrase,
}

/// Quoted-printable for a `text/*` body.
pub const BODY: QuotedPrintable = QuotedPrintable {
    mode: Mode::Body,
};
/// Quoted-printable for a non-text part.
pub const BINARY: QuotedPrintable = QuotedPrintable {
    mode: Mode::Binary,
};
/// RFC 2047 `Q` payload for unstructured text.
pub const Q_TEXT: QuotedPrintable = QuotedPrintable {
    mode: Mode::QText,
};
/// RFC 2047 `Q` payload for a phrase.
pub const Q_PHRASE: QuotedPrintable = QuotedPrintable {
    mode: Mode::QPhrase,
};

/// A byte sink that may fail, so that writing to an [`io::Write`] propagates
/// errors while writing to a buffer does not.
trait Sink {
    fn put(&mut self, bytes: &[u8]) -> io::Result<()>;
    fn byte(&mut self, byte: u8) -> io::Result<()>;
}

#[derive(Default)]
struct Count(usize);

impl Sink for Count {
    fn put(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.0 += bytes.len();
        Ok(())
    }
    fn byte(&mut self, _byte: u8) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }
}

struct VecSink<'a>(&'a mut Vec<u8>);

impl Sink for VecSink<'_> {
    fn put(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.0.extend_from_slice(bytes);
        Ok(())
    }
    fn byte(&mut self, byte: u8) -> io::Result<()> {
        self.0.push(byte);
        Ok(())
    }
}

struct IoSink<W: Write>(W);

impl<W: Write> Sink for IoSink<W> {
    fn put(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.0.write_all(bytes)
    }
    fn byte(&mut self, byte: u8) -> io::Result<()> {
        self.0.write_all(&[byte])
    }
}

#[inline]
fn escape(byte: u8) -> [u8; 3] {
    [b'=', HEX[(byte >> 4) as usize], HEX[(byte & 0x0F) as usize]]
}

/// True when RFC 2047 section 5 allows `byte` to appear literally in a phrase.
#[inline]
const fn phrase_allowed(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'!' | b'*' | b'+' | b'-' | b'/')
}

/// Encodes a body or non-text part. When `BODY` is true the byte is part of a
/// `text/*` entity.
fn encode_part<const BODY: bool, S: Sink>(input: &[u8], sink: &mut S) -> io::Result<()> {
    let mut column = 0_usize;
    let mut at = 0_usize;

    while at < input.len() {
        let byte = input[at];
        let next = input.get(at + 1).copied();
        let at_line_end = at + 1 == input.len() || (BODY && matches!(next, Some(b'\r' | b'\n')));

        let must_escape = byte == b'='
            || byte >= 0x7F
            || (!BODY && (byte == b'\r' || byte == b'\n'))
            || (BODY && byte == b'\r' && next != Some(b'\n'))
            || ((byte == b' ' || byte == b'\t') && at_line_end);

        if must_escape {
            if column + 3 > MAX_LINE - 1 {
                sink.put(b"=\r\n")?;
                column = 0;
            }
            sink.put(&escape(byte))?;
            column += 3;
            at += 1;
        } else if BODY && byte == b'\r' && next == Some(b'\n') {
            sink.put(b"\r\n")?;
            column = 0;
            at += 2;
        } else if BODY && byte == b'\n' {
            sink.put(b"\r\n")?;
            column = 0;
            at += 1;
        } else {
            if column + 1 > MAX_LINE - 1 {
                sink.put(b"=\r\n")?;
                column = 0;
            }
            sink.byte(byte)?;
            column += 1;
            at += 1;
        }
    }

    Ok(())
}

/// The length of `byte` when written as part of a `Q` payload.
#[inline]
#[allow(clippy::manual_range_contains)] // `Range::contains` is not const yet
pub(crate) const fn q_byte_len(byte: u8, phrase: bool) -> usize {
    if byte == b' ' || (byte == b'_') {
        // `_` must be escaped, space is written as `_`.
        if byte == b'_' { 3 } else { 1 }
    } else if byte == b'='
        || byte == b'?'
        || byte == b'\t'
        || byte == b'\r'
        || byte == b'\n'
        || byte < 0x20
        || byte >= 0x7F
        || (phrase && !phrase_allowed(byte))
    {
        3
    } else {
        1
    }
}

fn encode_q_bytes<const PHRASE: bool, S: Sink>(input: &[u8], sink: &mut S) -> io::Result<()> {
    for &byte in input {
        if byte == b' ' {
            sink.byte(b'_')?;
        } else if byte == b'_' {
            sink.put(b"=5F")?;
        } else if byte == b'='
            || byte == b'?'
            || byte == b'\t'
            || byte == b'\r'
            || byte == b'\n'
            || !(0x20..0x7F).contains(&byte)
            || (PHRASE && !phrase_allowed(byte))
        {
            sink.put(&escape(byte))?;
        } else {
            sink.byte(byte)?;
        }
    }
    Ok(())
}

/// The number of bytes `input` occupies as an RFC 2047 `Q` payload.
pub(crate) fn q_encoded_len(input: &[u8], phrase: bool) -> usize {
    input.iter().map(|&byte| q_byte_len(byte, phrase)).sum()
}

impl QuotedPrintable {
    /// The exact number of bytes the encoding of `input` occupies.
    pub fn encoded_len<T: AsRef<[u8]>>(&self, input: T) -> usize {
        let input = input.as_ref();
        match self.mode {
            Mode::QText => q_encoded_len(input, false),
            Mode::QPhrase => q_encoded_len(input, true),
            Mode::Body => count_part::<true>(input),
            Mode::Binary => count_part::<false>(input),
        }
    }

    /// Returns the encoded length of `input` when it does not exceed `limit`,
    /// and `None` otherwise. Used to pick between quoted-printable and Base64.
    pub fn encoded_len_within<T: AsRef<[u8]>>(&self, input: T, limit: usize) -> Option<usize> {
        let len = self.encoded_len(input);
        (len <= limit).then_some(len)
    }

    /// Appends the encoding of `input` to `output`, returning the number of
    /// bytes appended.
    pub fn encode_append<T: AsRef<[u8]>>(&self, input: T, output: &mut Vec<u8>) -> usize {
        let input = input.as_ref();
        let before = output.len();
        {
            let mut sink = VecSink(&mut *output);
            let result = self.write_to(input, &mut sink);
            debug_assert!(result.is_ok());
        }
        output.len() - before
    }

    /// Encodes `input` into any [`io::Write`].
    pub fn encode_to_writer(&self, input: &[u8], writer: impl Write) -> io::Result<()> {
        let mut sink = IoSink(writer);
        self.write_to(input, &mut sink)
    }

    /// Encodes `input` into a new vector.
    #[allow(dead_code)]
    pub fn encode<T: AsRef<[u8]>>(&self, input: T) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.encoded_len(input.as_ref()));
        self.encode_append(input, &mut output);
        output
    }

    fn write_to<S: Sink>(&self, input: &[u8], sink: &mut S) -> io::Result<()> {
        match self.mode {
            Mode::Body => encode_part::<true, _>(input, sink),
            Mode::Binary => encode_part::<false, _>(input, sink),
            Mode::QText => encode_q_bytes::<false, _>(input, sink),
            Mode::QPhrase => encode_q_bytes::<true, _>(input, sink),
        }
    }
}

fn count_part<const BODY: bool>(input: &[u8]) -> usize {
    let mut sink = Count::default();
    let result = encode_part::<BODY, _>(input, &mut sink);
    debug_assert!(result.is_ok());
    sink.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_length(engine: QuotedPrintable, input: &[u8]) {
        let mut output = Vec::new();
        engine.encode_append(input, &mut output);
        assert_eq!(output.len(), engine.encoded_len(input), "{input:?}");
    }

    #[test]
    fn encoded_len_matches_output() {
        let inputs: &[&[u8]] = &[
            b"",
            b"hello world",
            b"hello = world =",
            b"hello   \nworld   \r\n   ",
            "Grüße aus Köln — schöne Grüße".as_bytes(),
            "안녕하세요 세계".as_bytes(),
            b"a\rb\nc\r\nd",
            &b"x".repeat(500),
            &[0u8; 32],
        ];
        for &input in inputs {
            check_length(BODY, input);
            check_length(BINARY, input);
            check_length(Q_TEXT, input);
            check_length(Q_PHRASE, input);
        }
    }

    #[test]
    fn lines_are_at_most_76_columns() {
        let mut output = Vec::new();
        BODY.encode_append(b"= ".repeat(200), &mut output);
        for line in output.split(|&b| b == b'\n') {
            let line = line.strip_suffix(b"\r").unwrap_or(line);
            assert!(line.len() <= 76, "{} columns", line.len());
        }
    }

    #[test]
    fn q_text_escapes_underscore_and_literal_specials() {
        assert_eq!(Q_TEXT.encode("snake_case"), b"snake=5Fcase");
        assert_eq!(Q_TEXT.encode("a b"), b"a_b");
        assert_eq!(Q_TEXT.encode("a?b=c"), b"a=3Fb=3Dc");
    }

    #[test]
    fn binary_encodes_cr_and_lf() {
        assert_eq!(BINARY.encode(b"a\r\nb"), b"a=0D=0Ab");
    }

    #[test]
    fn body_keeps_crlf_and_encodes_lone_cr() {
        assert_eq!(BODY.encode(b"a\r\nb"), b"a\r\nb");
        assert_eq!(BODY.encode(b"a\rb"), b"a=0Db");
        assert_eq!(BODY.encode(b"a\nb"), b"a\r\nb");
    }

    #[test]
    fn trailing_whitespace_is_encoded() {
        assert_eq!(BODY.encode(b"a "), b"a=20");
        assert_eq!(BODY.encode(b"a \r\nb"), b"a=20\r\nb");
    }
}
