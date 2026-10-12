//! RFC 2047 encoded-word payloads.
//!
//! A [`WordEncoder`] produces the payload of a `=?charset?B?…?=` or
//! `=?charset?Q?…?=` encoded word and never splits a character. The header
//! folder decides where the words are placed and how many fit on a line; this
//! module only answers "how long is this text as a word" and "encode as much
//! as fits a budget, without cutting a character".

use super::{base64, qp};

/// An RFC 2047 word payload encoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordEncoder {
    base64: bool,
    phrase: bool,
}

/// `B` encoding, used when it is shorter than `Q`.
pub const B: WordEncoder = WordEncoder {
    base64: true,
    phrase: false,
};

/// `Q` encoding for unstructured text.
pub const Q_TEXT: WordEncoder = WordEncoder {
    base64: false,
    phrase: false,
};

/// `Q` encoding for a phrase (a display name); a stricter character set.
pub const Q_PHRASE: WordEncoder = WordEncoder {
    base64: false,
    phrase: true,
};

impl WordEncoder {
    /// The length of `text` as a word payload.
    pub fn payload_len(&self, text: &str) -> usize {
        if self.base64 {
            base64::STANDARD.encoded_len(text.len())
        } else {
            qp::q_encoded_len(text.as_bytes(), self.phrase)
        }
    }

    /// Encodes the longest prefix of `text` whose payload is at most `budget`
    /// bytes, never splitting a character, and returns the number of bytes of
    /// `text` that were consumed. At least one character is always consumed so
    /// that callers can make progress.
    pub fn encode_payload(&self, text: &str, budget: usize, output: &mut Vec<u8>) -> usize {
        let bytes = text.as_bytes();
        let mut consumed = 0;
        let mut length = 0;

        for character in text.chars() {
            let extra = if self.base64 {
                base64::STANDARD.encoded_len(consumed + character.len_utf8()) - base64::STANDARD.encoded_len(consumed)
            } else {
                let start = consumed;
                bytes[start..start + character.len_utf8()]
                    .iter()
                    .map(|&byte| qp::q_byte_len(byte, self.phrase))
                    .sum()
            };

            if length + extra > budget && consumed > 0 {
                break;
            }
            length += extra;
            consumed += character.len_utf8();
        }

        let prefix = bytes.get(..consumed).unwrap_or_default();
        if self.base64 {
            base64::STANDARD.encode_append(prefix, output);
        } else if self.phrase {
            qp::Q_PHRASE.encode_append(prefix, output);
        } else {
            qp::Q_TEXT.encode_append(prefix, output);
        }
        consumed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_len_matches_encode_payload() {
        for text in ["hello", "héllo", "안녕하세요", "a b=c?d_e", ""] {
            for encoder in [B, Q_TEXT, Q_PHRASE] {
                let len = encoder.payload_len(text);
                let mut output = Vec::new();
                let consumed = encoder.encode_payload(text, usize::MAX, &mut output);
                assert_eq!(consumed, text.len());
                assert_eq!(output.len(), len, "{encoder:?} {text:?}");
            }
        }
    }

    #[test]
    fn encode_payload_never_splits_a_character() {
        let text = "안녕하세요 세계";
        for budget in 1..40 {
            for encoder in [B, Q_TEXT, Q_PHRASE] {
                let mut output = Vec::new();
                let consumed = encoder.encode_payload(text, budget, &mut output);
                assert!(text.is_char_boundary(consumed), "split character");
                assert!(consumed > 0, "no progress");
            }
        }
    }
}
