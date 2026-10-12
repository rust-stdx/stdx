//! Base64 engines used by the message builder.
//!
//! The actual encoding is delegated to the [`base64`] crate; this module only
//! adds the MIME line wrapping (76 columns, CRLF separators) that RFC 2045
//! section 6.8 requires and exposes the small API the serializers use.

use base64::{Alphabet, encode_into, encoded_length};

/// Number of input bytes that map to one 76 column Base64 line.
const MIME_LINE_INPUT: usize = 57;
/// Length of a full MIME line including its CRLF.
const MIME_LINE_OUTPUT: usize = 76 + 2;

/// A Base64 configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base64 {
    wrap: bool,
}

/// Standard Base64 without line wrapping.
pub const STANDARD: Base64 = Base64 {
    wrap: false,
};

/// Base64 wrapped at 76 columns with CRLF, as required for MIME bodies.
pub const MIME: Base64 = Base64 {
    wrap: true,
};

impl Base64 {
    /// The exact number of bytes [`Base64::encode_append`] will produce for an
    /// input of `input_len` bytes, including any line breaks.
    pub const fn encoded_len(&self, input_len: usize) -> usize {
        let raw = match encoded_length(input_len, true) {
            Some(len) => len,
            None => return 0,
        };
        if !self.wrap || input_len == 0 {
            return raw;
        }
        let full_lines = input_len / MIME_LINE_INPUT;
        let remainder = input_len % MIME_LINE_INPUT;
        let tail = if remainder == 0 {
            0
        } else {
            let tail_len = match encoded_length(remainder, true) {
                Some(len) => len,
                None => return 0,
            };
            tail_len + 2
        };
        full_lines * MIME_LINE_OUTPUT + tail
    }

    /// Appends the Base64 encoding of `input` to `output` and returns the
    /// number of bytes appended.
    pub fn encode_append<T: AsRef<[u8]>>(&self, input: T, output: &mut Vec<u8>) -> usize {
        let input = input.as_ref();
        let before = output.len();

        if self.wrap {
            for chunk in input.chunks(MIME_LINE_INPUT) {
                let encoded = match encoded_length(chunk.len(), true) {
                    Some(len) => len,
                    None => break,
                };
                let start = output.len();
                output.resize(start + encoded, 0);
                if encode_into(&mut output[start..], chunk, Alphabet::Standard).is_err() {
                    output.truncate(start);
                    break;
                }
                output.extend_from_slice(b"\r\n");
            }
        } else {
            let encoded = match encoded_length(input.len(), true) {
                Some(len) => len,
                None => return 0,
            };
            let start = output.len();
            output.resize(start + encoded, 0);
            if encode_into(&mut output[start..], input, Alphabet::Standard).is_err() {
                output.truncate(start);
                return 0;
            }
        }

        output.len() - before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoded_len_matches_encode_append() {
        for len in 0..200 {
            let input = vec![0xABu8; len];
            for engine in [STANDARD, MIME] {
                let mut output = Vec::new();
                let written = engine.encode_append(&input, &mut output);
                assert_eq!(written, output.len(), "return value");
                assert_eq!(output.len(), engine.encoded_len(len), "len {len}");
            }
        }
    }

    #[test]
    fn mime_lines_are_at_most_76_columns() {
        let input = vec![0x5Au8; 1_000];
        let mut output = Vec::new();
        MIME.encode_append(&input, &mut output);
        for line in output.split(|&b| b == b'\n') {
            let line = line.strip_suffix(b"\r").unwrap_or(line);
            assert!(line.len() <= 76, "line of {} columns", line.len());
        }
    }

    #[test]
    fn empty_input_produces_nothing() {
        let mut output = Vec::new();
        assert_eq!(MIME.encode_append([], &mut output), 0);
        assert!(output.is_empty());
    }
}
