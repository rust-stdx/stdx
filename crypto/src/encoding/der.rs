//! Minimal DER (ASN.1 Distinguished Encoding Rules) reader.
//!
//! This implements just enough DER to parse the key material handled by
//! [`pkcs8`](super::pkcs8) and the DER-encoded ECDSA signatures handled by
//! [`ecdsa`](super::ecdsa): definite lengths, single-byte tags, and the
//! primitive encodings those structures require.
//!
//! It intentionally rejects BER indefinite-length encoding, constructed
//! string forms, and high-tag-number forms. Inputs using them return
//! [`DerError::Unsupported`] or [`DerError::UnexpectedTag`].

/// Errors returned while reading DER-encoded data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerError {
    /// The input ended before a complete value could be read.
    Truncated,
    /// A length used an unsupported or non-minimal encoding.
    InvalidLength,
    /// A value had an unexpected tag or empty content.
    UnexpectedTag,
    /// The input had trailing bytes after the expected value(s).
    TrailingData,
    /// A value used a DER feature this reader does not support.
    Unsupported,
}

impl core::fmt::Display for DerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DerError::Truncated => write!(f, "truncated DER input"),
            DerError::InvalidLength => write!(f, "invalid DER length"),
            DerError::UnexpectedTag => write!(f, "unexpected DER tag"),
            DerError::TrailingData => write!(f, "trailing DER data"),
            DerError::Unsupported => write!(f, "unsupported DER encoding"),
        }
    }
}

impl core::error::Error for DerError {}

const TAG_INTEGER: u8 = 0x02;
const TAG_BIT_STRING: u8 = 0x03;
const TAG_OCTET_STRING: u8 = 0x04;
const TAG_OID: u8 = 0x06;
const TAG_SEQUENCE: u8 = 0x30;

/// A cursor over a DER input that yields borrowed value contents.
///
/// The reader is checked: every method validates the tag, the length, and the
/// available input before returning, so it never panics on malformed data.
#[derive(Debug, Clone)]
pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Creates a reader over `data`.
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
        }
    }

    /// Returns `true` when all input has been consumed.
    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    /// Returns the number of unread bytes.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    /// Returns the tag of the next value without consuming it.
    ///
    /// Returns [`DerError::Truncated`] when no value remains.
    pub fn peek_tag(&self) -> Result<u8, DerError> {
        self.data.get(self.pos).copied().ok_or(DerError::Truncated)
    }

    /// Reads one tag-length-value, returning its tag and content.
    ///
    /// Returns [`DerError`] when the value is truncated, uses an indefinite or
    /// non-minimal length, or has a high-tag-number form.
    pub fn read_tlv(&mut self) -> Result<(u8, &'a [u8]), DerError> {
        let tag = *self.data.get(self.pos).ok_or(DerError::Truncated)?;
        if tag & 0x1f == 0x1f {
            return Err(DerError::Unsupported);
        }
        self.pos += 1;

        let first = *self.data.get(self.pos).ok_or(DerError::Truncated)?;
        self.pos += 1;

        let len = match first {
            0x00..=0x7f => first as usize,
            // Indefinite length is BER, not DER.
            0x80 => return Err(DerError::InvalidLength),
            0x81..=0x84 => {
                let n = (first & 0x7f) as usize;
                let mut len: usize = 0;
                for _ in 0..n {
                    let b = *self.data.get(self.pos).ok_or(DerError::Truncated)?;
                    self.pos += 1;
                    len = (len << 8) | b as usize;
                }
                // DER requires the minimal number of length bytes.
                if len < 0x80 {
                    return Err(DerError::InvalidLength);
                }
                len
            }
            _ => return Err(DerError::InvalidLength),
        };

        let end = self.pos.checked_add(len).ok_or(DerError::InvalidLength)?;
        if end > self.data.len() {
            return Err(DerError::Truncated);
        }
        let content = &self.data[self.pos..end];
        self.pos = end;
        Ok((tag, content))
    }

    /// Reads one value and requires it to have `tag`, returning its content.
    pub fn read_expected(&mut self, tag: u8) -> Result<&'a [u8], DerError> {
        let (actual, content) = self.read_tlv()?;
        if actual != tag {
            return Err(DerError::UnexpectedTag);
        }
        Ok(content)
    }

    /// Reads a `SEQUENCE` and returns a reader over its content.
    pub fn read_sequence(&mut self) -> Result<Reader<'a>, DerError> {
        Ok(Reader::new(self.read_expected(TAG_SEQUENCE)?))
    }

    /// Reads an `INTEGER`, returning its canonical content bytes.
    ///
    /// The content may start with a single `0x00` when required to keep the
    /// sign bit clear. Empty or non-minimal encodings are rejected.
    pub fn read_integer(&mut self) -> Result<&'a [u8], DerError> {
        let content = self.read_expected(TAG_INTEGER)?;
        if content.is_empty() {
            return Err(DerError::UnexpectedTag);
        }
        // Reject a redundant leading zero (canonical DER).
        if content.len() > 1 && content[0] == 0 && content[1] & 0x80 == 0 {
            return Err(DerError::InvalidLength);
        }
        Ok(content)
    }

    /// Reads an `OCTET STRING`, returning its content bytes.
    pub fn read_octet_string(&mut self) -> Result<&'a [u8], DerError> {
        self.read_expected(TAG_OCTET_STRING)
    }

    /// Reads a `BIT STRING`, returning its bytes with the unused-bits count
    /// byte removed.
    ///
    /// Returns [`DerError::Unsupported`] when the value declares unused bits
    /// (only byte-aligned bit strings are supported).
    pub fn read_bit_string(&mut self) -> Result<&'a [u8], DerError> {
        let content = self.read_expected(TAG_BIT_STRING)?;
        let (unused, rest) = content.split_first().ok_or(DerError::UnexpectedTag)?;
        if *unused != 0 {
            return Err(DerError::Unsupported);
        }
        Ok(rest)
    }

    /// Reads an `OBJECT IDENTIFIER`, returning its content bytes (without the
    /// leading tag/length). Compare against known OID byte strings.
    pub fn read_oid(&mut self) -> Result<&'a [u8], DerError> {
        let content = self.read_expected(TAG_OID)?;
        if content.is_empty() {
            return Err(DerError::UnexpectedTag);
        }
        Ok(content)
    }

    /// Reads a context-specific `[tag]` explicit wrapper and returns a reader
    /// over the wrapped value. `tag` is the full identifier byte, e.g. `0xa0`.
    pub fn read_explicit(&mut self, tag: u8) -> Result<Reader<'a>, DerError> {
        Ok(Reader::new(self.read_expected(tag)?))
    }

    /// Asserts that all input was consumed.
    ///
    /// Returns [`DerError::TrailingData`] otherwise.
    pub fn finish(self) -> Result<(), DerError> {
        if self.pos == self.data.len() {
            Ok(())
        } else {
            Err(DerError::TrailingData)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_short_form_sequence() {
        // SEQUENCE { INTEGER 1 }
        let der = [0x30, 0x03, 0x02, 0x01, 0x01];
        let mut r = Reader::new(&der);
        let mut seq = r.read_sequence().unwrap();
        r.finish().unwrap();
        assert_eq!(seq.read_integer().unwrap(), &[0x01]);
        seq.finish().unwrap();
    }

    #[test]
    fn reads_long_form_length() {
        // OCTET STRING of 200 zero bytes: 04 81 c8 ...
        let mut der = vec![0x04, 0x81, 0xc8];
        der.extend_from_slice(&[0u8; 200]);
        let mut r = Reader::new(&der);
        assert_eq!(r.read_octet_string().unwrap().len(), 200);
        r.finish().unwrap();
    }

    #[test]
    fn rejects_indefinite_length() {
        let der = [0x30, 0x80, 0x00, 0x00];
        assert_eq!(Reader::new(&der).read_sequence().unwrap_err(), DerError::InvalidLength);
    }

    #[test]
    fn rejects_non_minimal_length() {
        // Length 0x01 encoded in long form (0x81 0x01) is not DER.
        let der = [0x04, 0x81, 0x01, 0xff];
        assert_eq!(Reader::new(&der).read_octet_string().unwrap_err(), DerError::InvalidLength);
    }

    #[test]
    fn rejects_truncated_content() {
        let der = [0x04, 0x05, 0x00, 0x00];
        assert_eq!(Reader::new(&der).read_octet_string().unwrap_err(), DerError::Truncated);
    }

    #[test]
    fn rejects_wrong_tag() {
        let der = [0x02, 0x01, 0x01];
        assert_eq!(Reader::new(&der).read_octet_string().unwrap_err(), DerError::UnexpectedTag);
    }

    #[test]
    fn rejects_trailing_data() {
        let der = [0x02, 0x01, 0x01, 0xff];
        let mut r = Reader::new(&der);
        r.read_integer().unwrap();
        assert_eq!(r.finish().unwrap_err(), DerError::TrailingData);
    }

    #[test]
    fn rejects_redundant_integer_zero() {
        // INTEGER 0x0001 with a redundant leading zero.
        let der = [0x02, 0x02, 0x00, 0x01];
        assert_eq!(Reader::new(&der).read_integer().unwrap_err(), DerError::InvalidLength);
    }

    #[test]
    fn reads_bit_string_ignoring_unused_bits_byte() {
        // BIT STRING, unused=0, 0x04 0x01
        let der = [0x03, 0x03, 0x00, 0x04, 0x01];
        assert_eq!(Reader::new(&der).read_bit_string().unwrap(), &[0x04, 0x01]);
    }

    #[test]
    fn rejects_non_zero_unused_bits() {
        let der = [0x03, 0x03, 0x07, 0x04, 0x01];
        assert_eq!(Reader::new(&der).read_bit_string().unwrap_err(), DerError::Unsupported);
    }

    #[test]
    fn reads_explicit_context_tag() {
        // [0] { INTEGER 2 }
        let der = [0xa0, 0x03, 0x02, 0x01, 0x02];
        let mut r = Reader::new(&der);
        let mut inner = r.read_explicit(0xa0).unwrap();
        assert_eq!(inner.read_integer().unwrap(), &[0x02]);
    }
}
