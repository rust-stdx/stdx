const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// The percent-encoding engine used for extended MIME parameters.
#[derive(Debug, Clone, Copy)]
pub struct Percent;

/// Percent-encoding, as used by `filename*=UTF-8''…` parameter values.
pub const PERCENT: Percent = Percent;

#[inline]
const fn attr_safe(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#' | b'$' | b'&' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~'
        )
}

impl Percent {
    /// The number of bytes the encoding of `input` occupies.
    pub fn encoded_len<T: AsRef<[u8]>>(&self, input: T) -> usize {
        input
            .as_ref()
            .iter()
            .map(|&byte| if attr_safe(byte) { 1 } else { 3 })
            .sum()
    }

    /// Appends the percent-encoding of `input` to `output`, returning the
    /// number of bytes appended.
    pub fn encode_append<T: AsRef<[u8]>>(&self, input: T, output: &mut Vec<u8>) -> usize {
        let before = output.len();
        for &byte in input.as_ref() {
            if attr_safe(byte) {
                output.push(byte);
            } else {
                output.push(b'%');
                output.push(HEX[(byte >> 4) as usize]);
                output.push(HEX[(byte & 0x0F) as usize]);
            }
        }
        output.len() - before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encodes_rfc_2231_examples() {
        assert_eq!(
            String::from_utf8(PERCENT.encode_append_string("Jahresabschluß, 2024.pdf")).unwrap(),
            "Jahresabschlu%C3%9F%2C%202024.pdf"
        );
    }

    #[test]
    fn encoded_len_matches_output() {
        for input in ["plain.txt", "naïve.txt", "a\r\nb", "", "!*'()#"] {
            let mut output = Vec::new();
            let written = PERCENT.encode_append(input.as_bytes(), &mut output);
            assert_eq!(written, output.len());
            assert_eq!(output.len(), PERCENT.encoded_len(input.as_bytes()));
        }
    }

    impl Percent {
        fn encode_append_string(&self, input: &str) -> Vec<u8> {
            let mut output = Vec::new();
            self.encode_append(input.as_bytes(), &mut output);
            output
        }
    }
}
