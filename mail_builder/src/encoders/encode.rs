use std::io;

use super::swar::{swar_del_or_above, swar_lane, swar_splat, swar_zero_lanes};
use crate::{
    encoders::{
        base64::{self, MIME},
        qp::{self, QuotedPrintable},
    },
    writer::Writer,
};

pub(crate) enum EncodingType {
    Base64,
    QuotedPrintable(bool),
    None,
}

pub(crate) enum BodyEncoding {
    SevenBit { bare_line_feed: bool },
    QuotedPrintable { engine: QuotedPrintable, len: usize },
    Base64,
}

const BLOCK_WORDS: usize = 128;
const MAX_LINE_LEN: usize = 77;
const DEL: u8 = 0x7F;
const MIME_LINE_INPUT: usize = 57;
const MIME_LINE_OUTPUT: usize = MIME.encoded_len(MIME_LINE_INPUT);

struct Scan {
    needs_encoding: bool,
    bare_line_feed: bool,
    line_start: usize,
}

impl Scan {
    #[inline(always)]
    fn newline<const IS_BODY: bool>(&mut self, input: &[u8], at: usize) {
        if at + 1 - self.line_start > MAX_LINE_LEN {
            self.needs_encoding = true;
        }
        self.line_start = at + 1;

        let after_cr = input.get(at.wrapping_sub(1)) == Some(&b'\r');
        self.bare_line_feed |= !after_cr;
        if IS_BODY {
            let white = if after_cr {
                at.wrapping_sub(2)
            } else {
                at.wrapping_sub(1)
            };
            if matches!(input.get(white), Some(b' ' | b'\t')) {
                self.needs_encoding = true;
            }
        } else if !after_cr {
            self.needs_encoding = true;
        }
    }

    fn run<const IS_BODY: bool>(input: &[u8]) -> Self {
        let mut scan = Scan {
            needs_encoding: false,
            bare_line_feed: false,
            line_start: 0,
        };

        let (chunks, tail) = input.as_chunks::<8>();
        for (block_index, block) in chunks.chunks(BLOCK_WORDS).enumerate() {
            let mut high_lanes = 0;
            for (word_index, chunk) in block.iter().enumerate() {
                let word = u64::from_le_bytes(*chunk);
                high_lanes |= swar_del_or_above(word);

                let newlines = swar_zero_lanes(word ^ swar_splat(b'\n'));
                if newlines != 0 {
                    let base = (block_index * BLOCK_WORDS + word_index) * 8;
                    let mut bits = newlines;
                    while bits != 0 {
                        scan.newline::<IS_BODY>(input, base + swar_lane(bits));
                        bits &= bits - 1;
                    }
                }
            }
            scan.needs_encoding |= high_lanes != 0;
            if scan.needs_encoding {
                return scan;
            }
        }

        let tail_start = chunks.len() * 8;
        for (offset, &ch) in tail.iter().enumerate() {
            if ch >= DEL {
                scan.needs_encoding = true;
            } else if ch == b'\n' {
                scan.newline::<IS_BODY>(input, tail_start + offset);
            }
        }

        if matches!(input.last(), Some(b' ' | b'\t')) || input.len() - scan.line_start > MAX_LINE_LEN {
            scan.needs_encoding = true;
        }
        scan
    }
}

impl EncodingType {
    /// Chooses how header text is written: as it is, or as "Q" or "B"
    /// encoded words, whichever is shorter.
    pub(crate) fn for_header(text: &[u8]) -> Self {
        let has_high = Self::has_high(text);
        let needs_encoding = has_high
            || Self::has_control(text)
            || Self::has_encoded_word(text)
            || text.len() > MAX_LINE_LEN
            || matches!(text.last(), Some(b' ' | b'\t'));
        if !needs_encoding {
            return EncodingType::None;
        }

        let base64_len = base64::STANDARD.encoded_len(text.len());
        match qp::Q_TEXT.encoded_len_within(text, base64_len.saturating_sub(1)) {
            Some(_) => EncodingType::QuotedPrintable(!has_high),
            None => EncodingType::Base64,
        }
    }

    /// True when `text` contains a control character other than the horizontal
    /// and vertical whitespace that header folding already handles. Such bytes
    /// cannot appear in an unstructured field or a phrase, so they force the
    /// value to be written as encoded words.
    fn has_control(text: &[u8]) -> bool {
        text.iter()
            .any(|&byte| (byte < 0x20 && !matches!(byte, b'\t' | b'\n' | b'\r')) || byte == 0x7F)
    }

    /// True when `text` contains something that already looks like an RFC 2047
    /// encoded word (`=?…?=`). RFC 2047 section 7 requires such a run to be a
    /// valid encoded word, so the value is itself encoded to keep a decoder
    /// from interpreting caller text as an encoding directive.
    fn has_encoded_word(text: &[u8]) -> bool {
        let mut at = 0;
        while let Some(open) = text
            .get(at..)
            .and_then(|rest| rest.windows(2).position(|window| window == b"=?"))
        {
            let start = at + open + 2;
            if text
                .get(start..)
                .is_some_and(|rest| rest.windows(2).any(|window| window == b"?="))
            {
                return true;
            }
            at = start;
        }
        false
    }

    fn has_high(text: &[u8]) -> bool {
        let (chunks, tail) = text.as_chunks::<8>();
        let high_lanes = chunks
            .iter()
            .fold(0, |found, chunk| found | swar_del_or_above(u64::from_le_bytes(*chunk)));
        high_lanes != 0 || tail.iter().any(|&ch| ch >= DEL)
    }
}

impl BodyEncoding {
    /// Chooses the cheapest valid transfer encoding for a text part: `7bit`
    /// when the text needs none, otherwise quoted-printable or base64,
    /// whichever is shorter.
    pub(crate) fn for_text(input: &[u8], is_body: bool) -> Self {
        let scan = if is_body {
            Scan::run::<true>(input)
        } else {
            Scan::run::<false>(input)
        };
        if !scan.needs_encoding {
            return BodyEncoding::SevenBit {
                bare_line_feed: scan.bare_line_feed || has_bare_cr(input),
            };
        }

        let engine = if is_body { qp::BODY } else { qp::BINARY };
        let base64_len = MIME.encoded_len(input.len());
        match engine.encoded_len_within(input, base64_len.saturating_sub(1)) {
            Some(len) => BodyEncoding::QuotedPrintable {
                engine,
                len,
            },
            None => BodyEncoding::Base64,
        }
    }

    /// Writes the `Content-Transfer-Encoding` header, the blank line and the
    /// encoded body for `input`.
    pub(crate) fn write(self, input: &[u8], output: &mut impl Writer) {
        match self {
            BodyEncoding::Base64 => {
                output.write(b"Content-Transfer-Encoding: base64\r\n\r\n");
                Self::write_base64(input, output);
            }
            BodyEncoding::QuotedPrintable {
                engine,
                len,
            } => {
                output.write(b"Content-Transfer-Encoding: quoted-printable\r\n\r\n");
                if len <= output.append_limit() {
                    output.append_with(len, |buffer| engine.encode_append(input, buffer));
                } else {
                    engine.encode_to_writer(input, &mut Stream(output)).unwrap_or_default();
                }
            }
            BodyEncoding::SevenBit {
                bare_line_feed,
            } => {
                output.write(b"Content-Transfer-Encoding: 7bit\r\n\r\n");
                if bare_line_feed {
                    write_crlf_normalized(input, output);
                } else {
                    output.write(input);
                }
            }
        }
    }

    fn write_base64(input: &[u8], output: &mut impl Writer) {
        let lines = (output.append_limit() / MIME_LINE_OUTPUT).max(1);
        for chunk in input.chunks(lines.saturating_mul(MIME_LINE_INPUT)) {
            output.append_with(MIME.encoded_len(chunk.len()), |buffer| MIME.encode_append(chunk, buffer));
        }
    }
}

struct Stream<'x, W: Writer>(&'x mut W);

impl<W: Writer> io::Write for Stream<'_, W> {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        Writer::write(self.0, bytes);
        Ok(bytes.len())
    }

    #[inline]
    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        Writer::write(self.0, bytes);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// True when `input` contains a carriage return that is not part of a CRLF
/// pair. Such bytes are not valid in a body (RFC 5322 section 2.3) and must be
/// normalized to CRLF.
fn has_bare_cr(input: &[u8]) -> bool {
    let mut at = 0;
    while at < input.len() {
        if input.get(at) == Some(&b'\r') && input.get(at + 1) != Some(&b'\n') {
            return true;
        }
        at += 1;
    }
    false
}

/// Writes `input`, turning every bare CR and every bare LF into a CRLF pair.
fn write_crlf_normalized(input: &[u8], output: &mut impl Writer) {
    let mut start = 0;
    let mut at = 0;
    while at < input.len() {
        match input.get(at) {
            Some(b'\r') => {
                if at > start {
                    output.write(input.get(start..at).unwrap_or_default());
                }
                output.write(b"\r\n");
                at += if input.get(at + 1) == Some(&b'\n') { 2 } else { 1 };
                start = at;
            }
            Some(b'\n') => {
                if at > start {
                    output.write(input.get(start..at).unwrap_or_default());
                }
                output.write(b"\r\n");
                at += 1;
                start = at;
            }
            Some(_) => at += 1,
            None => break,
        }
    }
    if start < input.len() {
        output.write(input.get(start..).unwrap_or_default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer::IoWriter;

    fn write_encoded_body(input: &[u8], output: &mut Vec<u8>, is_body: bool) {
        BodyEncoding::for_text(input, is_body).write(input, output);
    }

    #[test]
    fn test_write_encoded_body() {
        let mut output = Vec::new();
        write_encoded_body(b"a b c\r\n", &mut output, false);
        assert_eq!(output, b"Content-Transfer-Encoding: 7bit\r\n\r\na b c\r\n");

        let mut output = Vec::new();
        write_encoded_body(
            b"a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\r\n",
            &mut output,
            false,
        );
        assert_eq!(output, b"Content-Transfer-Encoding: 7bit\r\n\r\na a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\r\n");

        let long_line = "a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a";
        let mut output = Vec::new();
        write_encoded_body(format!("{long_line}\r\n").as_bytes(), &mut output, false);
        let expected = format!(
            "Content-Transfer-Encoding: quoted-printable\r\n\r\n{}=\r\n{}=0D=0A",
            &long_line[..75],
            &long_line[75..]
        );
        assert_eq!(output, expected.as_bytes());

        let mut output = Vec::new();
        let long_line = "a".repeat(100);
        write_encoded_body(long_line.as_bytes(), &mut output, false);
        let expected = format!(
            "Content-Transfer-Encoding: quoted-printable\r\n\r\n{}",
            "a".repeat(75) + "=\r\n" + &"a".repeat(25)
        );
        assert_eq!(output, expected.as_bytes());

        let mut output = Vec::new();
        write_encoded_body(b"one\ntwo\r\nthree\n", &mut output, true);
        assert_eq!(output, b"Content-Transfer-Encoding: 7bit\r\n\r\none\r\ntwo\r\nthree\r\n");
    }

    struct Plain(Vec<u8>);

    impl Writer for Plain {
        fn write(&mut self, bytes: &[u8]) {
            self.0.extend_from_slice(bytes);
        }
    }

    fn bodies() -> Vec<Vec<u8>> {
        let latin = "Grüße aus Köln, schöne Grüße! = \t\r\n".repeat(4_000);
        let cjk = "안녕하세요 세계 ".repeat(20_000);
        let binary = (0..300_000u32)
            .map(|value| (value * 31 % 251) as u8)
            .collect::<Vec<_>>();
        let long_ascii = "x=y ".repeat(50_000);
        vec![
            Vec::new(),
            b"a".to_vec(),
            "é".as_bytes().to_vec(),
            latin.into_bytes(),
            cjk.into_bytes(),
            binary,
            long_ascii.into_bytes(),
        ]
    }

    #[test]
    fn every_sink_receives_the_same_body() {
        for body in bodies() {
            for is_body in [true, false] {
                let mut expected = Vec::new();
                BodyEncoding::for_text(&body, is_body).write(&body, &mut expected);

                for capacity in [8, 100, 4096, 65536] {
                    let mut writer = IoWriter::with_capacity(capacity, Vec::new());
                    BodyEncoding::for_text(&body, is_body).write(&body, &mut writer);
                    let streamed = writer.finish().expect("writing to a Vec never fails");
                    assert!(streamed == expected, "capacity {capacity} is_body {is_body}");
                }

                let mut plain = Plain(Vec::new());
                BodyEncoding::for_text(&body, is_body).write(&body, &mut plain);
                assert!(plain.0 == expected, "plain writer is_body {is_body}");
            }

            let mut expected = Vec::new();
            BodyEncoding::Base64.write(&body, &mut expected);
            assert_eq!(
                expected.len(),
                "Content-Transfer-Encoding: base64\r\n\r\n".len() + MIME.encoded_len(body.len())
            );
            for capacity in [8, 100, 4096] {
                let mut writer = IoWriter::with_capacity(capacity, Vec::new());
                BodyEncoding::Base64.write(&body, &mut writer);
                let streamed = writer.finish().expect("writing to a Vec never fails");
                assert!(streamed == expected, "base64 capacity {capacity}");
            }
        }
    }

    #[test]
    fn quoted_printable_length_is_exact() {
        for body in bodies() {
            for is_body in [true, false] {
                if let BodyEncoding::QuotedPrintable {
                    engine,
                    len,
                } = BodyEncoding::for_text(&body, is_body)
                {
                    assert_eq!(engine.encode(&body).len(), len);
                }
            }
        }
    }
}
