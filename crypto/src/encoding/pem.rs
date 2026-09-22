use alloc::{string::String, vec::Vec};
use core::fmt;

use memchr::{memchr, memchr2, memmem};

const BEGIN_MARKER: &[u8] = b"-----BEGIN ";
const END_MARKER: &[u8] = b"-----END ";
const MARKER_END: &[u8] = b"-----";
const LINE_WIDTH: usize = 64;

#[derive(Clone, PartialEq, Eq)]
pub struct Block<'a> {
    pub r#type: &'a str,
    pub headers: Headers,
    pub contents: Vec<u8>,
}

impl fmt::Debug for Block<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("type", &self.r#type)
            .field("headers", &self.headers)
            .field("contents.len", &self.contents.len())
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PemError<'a> {
    InvalidEncoding(&'static str),
    Base64(base64::DecodeError),
    LabelMismatch { expected: &'a str, actual: &'a str },
}

impl<'a> From<base64::DecodeError> for PemError<'a> {
    fn from(err: base64::DecodeError) -> Self {
        PemError::Base64(err)
    }
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for PemError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PemError::InvalidEncoding(str) => write!(f, "{str}"),
            PemError::Base64(err) => write!(f, "base64 decode error: {err}"),
            PemError::LabelMismatch {
                expected,
                actual,
            } => {
                write!(f, "label mismatch: expected '{expected}', got '{actual}'")
            }
        }
    }
}

pub fn encode(blocks: &[Block<'_>]) -> Vec<u8> {
    let mut capacity = 0usize;
    let mut base64_capacity = 0;

    for block in blocks {
        capacity += 11 + block.r#type.len() + 6;
        for (k, v) in block.headers.iter() {
            capacity += k.len() + 2 + v.len() + 1;
        }
        if !block.headers.is_empty() {
            capacity += 1;
        }
        let b64_len = base64::encoded_length(block.contents.len(), true).unwrap_or_default();
        base64_capacity += b64_len;
        capacity += b64_len + b64_len / 64 + 1;
        capacity += 9 + block.r#type.len() + 6;
    }

    let mut output = Vec::with_capacity(capacity);
    let mut b64_buf = String::with_capacity(base64_capacity);

    for block in blocks {
        output.extend_from_slice(b"-----BEGIN ");
        output.extend_from_slice(block.r#type.as_bytes());
        output.extend_from_slice(b"-----\n");

        for (key, value) in block.headers.iter() {
            output.extend_from_slice(key.as_bytes());
            output.extend_from_slice(b": ");
            output.extend_from_slice(value.as_bytes());
            output.push(b'\n');
        }

        if !block.headers.is_empty() {
            output.push(b'\n');
        }

        base64::encode_into_string(&mut b64_buf, &block.contents, base64::Alphabet::Standard);
        for chunk in b64_buf.as_bytes().chunks(LINE_WIDTH) {
            output.extend_from_slice(chunk);
            output.push(b'\n');
        }
        b64_buf.clear();

        output.extend_from_slice(b"-----END ");
        output.extend_from_slice(block.r#type.as_bytes());
        output.extend_from_slice(b"-----\n");
    }
    return output;
}

/// Iterates over the textual encoding blocks found in `pem`.
///
/// Each item is a decoded [`Block`] or a [`PemError`]. The body of every block
/// must be standard base64; spaces, tabs, carriage returns, line feeds, vertical
/// tabs and form feeds are ignored, while any other byte (including punctuation
/// and non-ASCII data) is rejected with [`PemError::InvalidEncoding`].
pub fn decode<'a>(pem: &'a [u8]) -> Blocks<'a> {
    Blocks {
        input: pem,
        pos: 0,
    }
}

pub struct Blocks<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for Blocks<'a> {
    type Item = Result<Block<'a>, PemError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        let result = parse_one_block(self.input, &mut self.pos);

        return match result {
            Ok(block) => Some(Ok(block)),
            Err(err) => {
                self.pos = self.input.len();
                Some(Err(err))
            }
        };
    }
}

#[derive(Clone, Copy)]
struct Header {
    key_start: usize,
    key_len: usize,
    value_start: usize,
    value_len: usize,
}

#[derive(Clone)]
pub struct Headers {
    buf: String,
    pairs: Vec<Header>,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            buf: String::new(),
            pairs: Vec::new(),
        }
    }

    pub fn with_capacity(buf: usize, headers: usize) -> Self {
        Headers {
            buf: String::with_capacity(buf),
            pairs: Vec::with_capacity(headers),
        }
    }

    pub fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        let buf_capacity = pairs.iter().fold(0, |acc, pair| acc + pair.0.len() + pair.1.len());
        let mut headers = Headers::with_capacity(buf_capacity, pairs.len());

        for (k, v) in pairs {
            headers.buf.reserve(k.len() + v.len());
            let key_start = headers.buf.len();
            headers.buf.push_str(k);
            let value_start = headers.buf.len();
            headers.buf.push_str(v);
            headers.pairs.push(Header {
                key_start,
                key_len: value_start - key_start,
                value_start,
                value_len: headers.buf.len() - value_start,
            });
        }

        headers
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.pairs.iter().map(move |span| {
            let key = &self.buf[span.key_start..span.key_start + span.key_len];
            let value = &self.buf[span.value_start..span.value_start + span.value_len];
            (key, value)
        })
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn push(&mut self, key: &str, value: &str) {
        let key_start = self.buf.len();
        self.buf.push_str(key);
        let value_start = self.buf.len();
        self.buf.push_str(value);
        self.pairs.push(Header {
            key_start,
            key_len: value_start - key_start,
            value_start,
            value_len: self.buf.len() - value_start,
        });
    }
}

impl PartialEq for Headers {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl Eq for Headers {}

impl fmt::Debug for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

fn parse_one_block<'a>(input: &'a [u8], pos: &mut usize) -> Result<Block<'a>, PemError<'a>> {
    let remaining = &input[*pos..];
    let begin_offset = find_pattern(remaining, BEGIN_MARKER).ok_or(PemError::InvalidEncoding("no BEGIN line found"))?;

    let label_start = *pos + begin_offset + BEGIN_MARKER.len();
    let (r#type, advance) = {
        let remaining = &input[label_start..];
        let line_end = find_line_end(remaining).ok_or(PemError::InvalidEncoding("unexpected end of PEM data"))?;
        let label_bytes = &remaining[..line_end];

        if label_bytes.len() < MARKER_END.len()
            || label_bytes[label_bytes.len() - MARKER_END.len()..] != *MARKER_END
            || (label_bytes.len() > MARKER_END.len() && label_bytes[label_bytes.len() - MARKER_END.len() - 1] == b'-')
        {
            return Err(PemError::InvalidEncoding("malformed BEGIN line"));
        }

        let label = &label_bytes[..label_bytes.len() - MARKER_END.len()];
        let r#type = core::str::from_utf8(label).map_err(|_| PemError::InvalidEncoding("non-UTF-8 label"))?;
        (r#type, line_advance(remaining))
    };

    let mut cursor = label_start + advance;

    let mut header_buf = String::new();
    let mut pairs: Vec<Header> = Vec::new();

    loop {
        if cursor >= input.len() {
            return Err(PemError::InvalidEncoding("unexpected end of PEM data"));
        }
        let remaining = &input[cursor..];
        let line_end = find_line_end(remaining).ok_or(PemError::InvalidEncoding("unexpected end of PEM data"))?;
        let line = &remaining[..line_end];

        if line.starts_with(END_MARKER) {
            let contents = Vec::new();
            *pos = cursor + line_advance(remaining);
            return Ok(Block {
                r#type,
                headers: Headers {
                    buf: header_buf,
                    pairs,
                },
                contents,
            });
        }

        if line.is_empty() || line.iter().all(|&b| b.is_ascii_whitespace()) {
            cursor += line_advance(remaining);
            break;
        }

        if let Some(colon_pos) = memchr(b':', line) {
            let key = core::str::from_utf8(&line[..colon_pos])
                .map_err(|_| PemError::InvalidEncoding("non-UTF-8 header key"))?;
            let value_start = colon_pos + 1;
            let value = if value_start < line.len() && line[value_start] == b' ' {
                &line[value_start + 1..]
            } else {
                &line[value_start..]
            };
            let value_str =
                core::str::from_utf8(value).map_err(|_| PemError::InvalidEncoding("non-UTF-8 header value"))?;

            cursor += line_advance(remaining);

            let key_start = header_buf.len();
            header_buf.push_str(key);
            let key_len = header_buf.len() - key_start;

            let has_continuation = cursor < input.len() && {
                let rest = &input[cursor..];
                let cont_line_end = find_line_end(rest).unwrap_or(rest.len());
                let cont_line = &rest[..cont_line_end];
                !cont_line.is_empty() && (cont_line[0] == b' ' || cont_line[0] == b'\t')
            };

            if has_continuation {
                let mut full_value = String::from(value_str);
                loop {
                    if cursor >= input.len() {
                        break;
                    }
                    let rest = &input[cursor..];
                    let cont_line_end =
                        find_line_end(rest).ok_or(PemError::InvalidEncoding("unexpected end of PEM data"))?;
                    let cont_line = &rest[..cont_line_end];
                    if cont_line.is_empty() {
                        break;
                    }
                    if cont_line[0] != b' ' && cont_line[0] != b'\t' {
                        break;
                    }
                    let cont_trimmed = cont_line.trim_ascii_start();
                    if !cont_trimmed.is_empty() {
                        if !full_value.is_empty() {
                            full_value.push(' ');
                        }
                        if let Ok(s) = core::str::from_utf8(cont_trimmed) {
                            full_value.push_str(s);
                        }
                    }
                    cursor += line_advance(rest);
                }
                let val_start = header_buf.len();
                header_buf.push_str(&full_value);
                let val_len = header_buf.len() - val_start;
                pairs.push(Header {
                    key_start,
                    key_len,
                    value_start: val_start,
                    value_len: val_len,
                });
            } else {
                let val_start = header_buf.len();
                header_buf.push_str(value_str);
                let val_len = header_buf.len() - val_start;
                pairs.push(Header {
                    key_start,
                    key_len,
                    value_start: val_start,
                    value_len: val_len,
                });
            }
        } else {
            break;
        }
    }

    let base64_start = cursor;

    let b64_data_end;
    let mut search_pos = base64_start;

    loop {
        if search_pos >= input.len() {
            return Err(PemError::InvalidEncoding("missing END line"));
        }
        let remaining = &input[search_pos..];
        let end_offset = find_pattern(remaining, END_MARKER);
        match end_offset {
            Some(eo) => {
                let candidate = search_pos + eo;
                let end_rest = &input[candidate..];
                let end_line_end =
                    find_line_end(end_rest).ok_or(PemError::InvalidEncoding("unexpected end of data"))?;
                let end_line = &end_rest[..end_line_end];

                let end_label_bytes = &end_line[END_MARKER.len()..];
                if end_label_bytes.len() >= MARKER_END.len()
                    && end_label_bytes[end_label_bytes.len() - MARKER_END.len()..] == *MARKER_END
                    && (end_label_bytes.len() == MARKER_END.len()
                        || end_label_bytes[end_label_bytes.len() - MARKER_END.len() - 1] != b'-')
                {
                    let end_label = &end_label_bytes[..end_label_bytes.len() - MARKER_END.len()];
                    if end_label == r#type.as_bytes() {
                        b64_data_end = candidate;
                        break;
                    }
                    let actual = core::str::from_utf8(end_label).unwrap_or("(invalid UTF-8)");
                    return Err(PemError::LabelMismatch {
                        expected: r#type,
                        actual,
                    });
                }
                search_pos = candidate + 1;
            }
            None => {
                return Err(PemError::InvalidEncoding("missing END line"));
            }
        }
    }

    let b64_text = &input[base64_start..b64_data_end];

    let end_remaining = &input[b64_data_end..];
    *pos = b64_data_end + line_advance(end_remaining);

    let mut b64_clean: Vec<u8> = Vec::with_capacity(b64_text.len());
    for &b in b64_text {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' | b'=' => b64_clean.push(b),
            b' ' | b'\t' | b'\r' | b'\n' | 0x0B | 0x0C => {}
            _ => return Err(PemError::InvalidEncoding("invalid character in base64 body")),
        }
    }

    let contents = base64::decode(&b64_clean, base64::Alphabet::Standard)?;

    return Ok(Block {
        r#type,
        headers: Headers {
            buf: header_buf,
            pairs,
        },
        contents,
    });
}

fn find_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() || data.len() < pattern.len() {
        return None;
    }
    memmem::find(data, pattern)
}

fn find_line_end(data: &[u8]) -> Option<usize> {
    if data.is_empty() {
        return None;
    }
    Some(memchr2(b'\n', b'\r', data).unwrap_or(data.len()))
}

fn line_advance(data: &[u8]) -> usize {
    if data.is_empty() {
        return 0;
    }
    if let Some(i) = memchr2(b'\n', b'\r', data) {
        if data[i] == b'\n' {
            return i + 1;
        }
        // data[i] == b'\r'
        if i + 1 < data.len() && data[i + 1] == b'\n' {
            return i + 2;
        }
        return i + 1;
    }
    data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(blocks: &[Block<'_>]) {
        let pem = encode(blocks);
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem).collect();
        for (i, result) in decoded.iter().enumerate() {
            let decoded_block = result.as_ref().unwrap();
            assert_eq!(decoded_block.r#type, blocks[i].r#type, "block {} type mismatch", i);
            assert_eq!(decoded_block.contents, blocks[i].contents, "block {} contents mismatch", i);
            assert_eq!(decoded_block.headers, blocks[i].headers, "block {} headers mismatch", i);
        }
        assert_eq!(decoded.len(), blocks.len(), "block count mismatch");
    }

    #[test]
    fn empty_content() {
        let blocks = [Block {
            r#type: "CERTIFICATE".into(),
            headers: Headers::new(),
            contents: Vec::new(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn simple_certificate() {
        let blocks = [Block {
            r#type: "CERTIFICATE".into(),
            headers: Headers::new(),
            contents: b"hello world".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn binary_content() {
        let contents: Vec<u8> = (0u8..255).collect();
        let blocks = [Block {
            r#type: "PRIVATE KEY".into(),
            headers: Headers::new(),
            contents,
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn exact_48_bytes() {
        let contents = b"1234567890abcdef1234567890abcdef1234567890abcdef"; // 48 bytes
        let blocks = [Block {
            r#type: "CERTIFICATE".into(),
            headers: Headers::new(),
            contents: contents.to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn multiple_blocks() {
        let blocks = [
            Block {
                r#type: "CERTIFICATE".into(),
                headers: Headers::new(),
                contents: b"first certificate data".to_vec(),
            },
            Block {
                r#type: "CERTIFICATE".into(),
                headers: Headers::new(),
                contents: b"second certificate data".to_vec(),
            },
        ];
        roundtrip(&blocks);
    }

    #[test]
    fn different_labels() {
        let blocks = [
            Block {
                r#type: "CERTIFICATE".into(),
                headers: Headers::new(),
                contents: b"cert data".to_vec(),
            },
            Block {
                r#type: "PRIVATE KEY".into(),
                headers: Headers::new(),
                contents: b"key data".to_vec(),
            },
            Block {
                r#type: "PUBLIC KEY".into(),
                headers: Headers::new(),
                contents: b"pubkey data".to_vec(),
            },
        ];
        roundtrip(&blocks);
    }

    #[test]
    fn with_rfc1421_headers() {
        let blocks = [Block {
            r#type: "PRIVACY-ENHANCED MESSAGE".into(),
            headers: Headers::from_pairs(&[
                ("Proc-Type", "4,ENCRYPTED"),
                ("Content-Domain", "RFC822"),
                ("DEK-Info", "DES-CBC,F8143EDE5960C597"),
            ]),
            contents: b"encrypted message data".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn header_with_folded_value() {
        let pem: &[u8] = b"-----BEGIN PRIVACY-ENHANCED MESSAGE-----\n\
Proc-Type: 4,ENCRYPTED\n\
Originator-Certificate:\n MIIBlTCCAScCAWUw\n\
\n\
SGVsbG8gV29ybGQ=\n\
-----END PRIVACY-ENHANCED MESSAGE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "PRIVACY-ENHANCED MESSAGE");
        assert!(block.headers.len() >= 1);

        let proc_type = block.headers.iter().find(|&(k, _)| k == "Proc-Type");
        assert!(proc_type.is_some(), "Proc-Type not found. Headers: {:?}", block.headers);
        assert_eq!(proc_type.unwrap().1, "4,ENCRYPTED");

        let originator = block.headers.iter().find(|&(k, _)| k == "Originator-Certificate");
        assert!(
            originator.is_some(),
            "Originator-Certificate not found. Headers: {:?}",
            block.headers
        );
        assert_eq!(
            originator.unwrap().1,
            "MIIBlTCCAScCAWUw",
            "Originator-Certificate value mismatch. Got: '{}'",
            originator.unwrap().1
        );

        assert_eq!(block.contents, b"Hello World");
    }

    #[test]
    fn decode_from_rfc7468_certificate() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     MIICLDCCAdKgAwIBAgIBADAKBggqhkjOPQQDAjB9MQswCQYDVQQGEwJCRTEPMA0G\n\
                     A1UEChMGR251VExTMSUwIwYDVQQLExxHbnVUTFMgY2VydGlmaWNhdGUgYXV0aG9y\n\
                     aXR5MQ8wDQYDVQQIEwZMZXV2ZW4xJTAjBgNVBAMTHEdudVRMUyBjZXJ0aWZpY2F0\n\
                     ZSBhdXRob3JpdHkwHhcNMTEwNTIzMjAzODIxWhcNMTIxMjIyMDc0MTUxWjB9MQsw\n\
                     CQYDVQQGEwJCRTEPMA0GA1UEChMGR251VExTMSUwIwYDVQQLExxHbnVUTFMgY2Vy\n\
                     dGlmaWNhdGUgYXV0aG9yaXR5MQ8wDQYDVQQIEwZMZXV2ZW4xJTAjBgNVBAMTHEdu\n\
                     dVRMUyBjZXJ0aWZpY2F0ZSBhdXRob3JpdHkwWTATBgcqhkjOPQIBBggqhkjOPQMB\n\
                     BwNCAARS2I0jiuNn14Y2sSALCX3IybqiIJUvxUpj+oNfzngvj/Niyv2394BWnW4X\n\
                     uQ4RTEiywK87WRcWMGgJB5kX/t2no0MwQTAPBgNVHRMBAf8EBTADAQH/MA8GA1Ud\n\
                     DwEB/wQFAwMHBgAwHQYDVR0OBBYEFPC0gf6YEr+1KLlkQAPLzB9mTigDMAoGCCqG\n\
                     SM49BAMCA0gAMEUCIDGuwD1KPyG+hRf88MeyMQcqOFZD0TbVleF+UsAGQ4enAiEA\n\
                     l4wOuDwKQa+upc8GftXE2C//4mKANBC6It01gUaTIpo=\n\
                     -----END CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "CERTIFICATE");
        assert!(block.headers.is_empty());
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_private_key() {
        let pem = b"-----BEGIN PRIVATE KEY-----\n\
                     MIGEAgEAMBAGByqGSM49AgEGBSuBBAAKBG0wawIBAQQgVcB/UNPxalR9zDYAjQIf\n\
                     jojUDiQuGnSJrFEEzZPT/92hRANCAASc7UJtgnF/abqWM60T3XNJEzBv5ez9TdwK\n\
                     H0M6xpM2q+53wmsN/eYLdgtjgBd3DBmHtPilCkiFICXyaA8z9LkJ\n\
                     -----END PRIVATE KEY-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "PRIVATE KEY");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_crlf_line_endings() {
        let pem = b"-----BEGIN CERTIFICATE-----\r\n\
                     SGVsbG8gV29ybGQ=\r\n\
                     -----END CERTIFICATE-----\r\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "CERTIFICATE");
        assert_eq!(block.contents, b"Hello World");
    }

    #[test]
    fn decode_mac_line_endings() {
        let pem = b"-----BEGIN CERTIFICATE-----\r\
                     SGVsbG8gV29ybGQ=\r\
                     -----END CERTIFICATE-----\r";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.contents, b"Hello World");
    }

    #[test]
    fn decode_multiple_blocks() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     Rmlyc3Q=\n\
                     -----END CERTIFICATE-----\n\
                     -----BEGIN CERTIFICATE-----\n\
                     U2Vjb25k\n\
                     -----END CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"First");
        assert_eq!(decoded[1].as_ref().unwrap().contents, b"Second");
    }

    #[test]
    fn decode_with_leading_text() {
        let pem = b"Some explanatory text\n\
                     Subject: CN=Test\n\
                     Issuer: CN=Test\n\
                     -----BEGIN CERTIFICATE-----\n\
                     SGVsbG8=\n\
                     -----END CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"Hello");
    }

    #[test]
    fn decode_empty_pem() {
        let pem = b"";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert!(decoded.is_empty());
    }

    #[test]
    fn decode_label_mismatch() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     SGVsbG8=\n\
                     -----END PRIVATE KEY-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err());
        match &decoded[0] {
            Err(PemError::LabelMismatch {
                expected,
                actual,
            }) => {
                assert_eq!(*expected, "CERTIFICATE");
                assert_eq!(*actual, "PRIVATE KEY");
            }
            other => panic!("expected LabelMismatch, got {:?}", other),
        }
    }

    #[test]
    fn decode_missing_end_line() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     SGVsbG8=\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err());
    }

    #[test]
    fn decode_invalid_base64() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     !!!invalid!!!\n\
                     -----END CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err());
    }

    #[test]
    fn decode_multiline_base64() {
        let contents = b"This is a test message that is long enough to span multiple lines when base64 encoded with 64 character line width";
        let blocks = [Block {
            r#type: "MESSAGE".into(),
            headers: Headers::new(),
            contents: contents.to_vec(),
        }];
        let pem = encode(&blocks);
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, contents);
    }

    #[test]
    fn encode_produces_valid_pem() {
        let block = Block {
            r#type: "TEST".into(),
            headers: Headers::new(),
            contents: b"data".to_vec(),
        };
        let pem = encode(&[block]);
        let pem_str = core::str::from_utf8(&pem).unwrap();
        assert!(pem_str.starts_with("-----BEGIN TEST-----\n"));
        assert!(pem_str.contains("\n-----END TEST-----\n"));
    }

    #[test]
    fn encode_line_wrapping() {
        let contents = vec![b'A'; 200];
        let block = Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents,
        };
        let pem = encode(&[block]);
        let pem_str = core::str::from_utf8(&pem).unwrap();
        let body = pem_str
            .strip_prefix("-----BEGIN DATA-----\n")
            .unwrap()
            .strip_suffix("\n-----END DATA-----\n")
            .unwrap();
        for line in body.lines() {
            if !line.is_empty() {
                assert!(line.len() <= 64, "line too long: {} > 64", line.len());
            }
        }
    }

    #[test]
    fn decode_no_trailing_newline() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     SGVsbG8=\n\
                     -----END CERTIFICATE-----";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"Hello");
    }

    #[test]
    fn decode_whitespace_in_base64() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     SGVs\tbG8g\n\
                     V29y bGQ=\n\
                     -----END CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"Hello World");
    }

    #[test]
    fn decode_interleaved_comment_lines() {
        let pem = b"-----BEGIN CERTIFICATE-----\n\
                     Proc-Type: 4,ENCRYPTED\n\
                     \n\
                     SGVsbG8=\n\
                     -----END CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.contents, b"Hello");
        assert_eq!(block.r#type, "CERTIFICATE");
    }

    #[test]
    fn decode_header_no_space_after_colon() {
        let pem = b"-----BEGIN PRIVACY-ENHANCED MESSAGE-----\n\
                     Proc-Type:4,ENCRYPTED\n\
                     \n\
                     SGVsbG8=\n\
                     -----END PRIVACY-ENHANCED MESSAGE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.contents, b"Hello");
        assert_eq!(block.headers.iter().next().unwrap().1, "4,ENCRYPTED");
    }

    #[test]
    fn openssl_test_vector() {
        let der: Vec<u8> = (0u8..48).collect();

        let b64 = base64::encode(&der, base64::Alphabet::Standard);
        let mut lines = Vec::new();
        for chunk in b64.as_bytes().chunks(64) {
            lines.push(core::str::from_utf8(chunk).unwrap().to_string());
        }

        let mut pem = String::from("-----BEGIN CERTIFICATE-----\n");
        for line in &lines {
            pem.push_str(line);
            pem.push('\n');
        }
        pem.push_str("-----END CERTIFICATE-----\n");

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem.as_bytes()).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, der);
    }

    #[test]
    fn openssl_ec_private_key() {
        let pem = b"-----BEGIN EC PRIVATE KEY-----\n\
                     MHQCAQEEIIm3VYFh8WkH4lA2KJ6tC3R0H3G7LgZc1Y0Z5Q7sZq6oBwYFK4EE\n\
                     AaahRANCAAQrR4q6kQ8V5lY6Lq3XZ0gG5f7J2sLvG8kH7X4KxVc5oBwYFK4E\n\
                     AaE=\n\
                     -----END EC PRIVATE KEY-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "EC PRIVATE KEY");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn python_generated_vector() {
        let data = b"Hello from Python!";
        let b64 = base64::encode(data, base64::Alphabet::Standard);
        let pem = alloc::format!("-----BEGIN PYTHON DATA-----\n{}\n-----END PYTHON DATA-----\n", b64);

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem.as_bytes()).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, data);
    }

    #[test]
    fn python_generated_with_headers() {
        let mut pem = String::from("-----BEGIN PYTHON DATA-----\n");
        pem.push_str("Content-Type: application/octet-stream\n");
        pem.push_str("Content-Transfer-Encoding: base64\n");
        pem.push('\n');
        let b64 = base64::encode(b"Python header data", base64::Alphabet::Standard);
        pem.push_str(&b64);
        pem.push('\n');
        pem.push_str("-----END PYTHON DATA-----\n");

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem.as_bytes()).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "PYTHON DATA");
        assert_eq!(block.contents, b"Python header data");
        let h_vec: Vec<(&str, &str)> = block.headers.iter().collect();
        assert_eq!(h_vec.len(), 2);
    }

    #[test]
    fn label_with_spaces() {
        let blocks = [Block {
            r#type: "CERTIFICATE REQUEST".into(),
            headers: Headers::new(),
            contents: b"csr data".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn label_empty() {
        let blocks = [Block {
            r#type: "",
            headers: Headers::new(),
            contents: b"data".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn large_content() {
        let contents = vec![0xABu8; 10000];
        let blocks = [Block {
            r#type: "LARGE DATA".into(),
            headers: Headers::new(),
            contents,
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn decode_block_without_base64() {
        let pem = b"-----BEGIN EMPTY-----\n\
                     -----END EMPTY-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "EMPTY");
        assert!(block.contents.is_empty());
    }

    #[test]
    fn roundtrip_rfc7468_certificate() {
        let der: Vec<u8> = (0u8..128).collect();
        let original = Block {
            r#type: "CERTIFICATE".into(),
            headers: Headers::new(),
            contents: der,
        };
        roundtrip(&[original]);
    }

    #[test]
    fn roundtrip_rfc1421_encrypted_message() {
        let original = Block {
            r#type: "PRIVACY-ENHANCED MESSAGE".into(),
            headers: Headers::from_pairs(&[
                ("Proc-Type", "4,ENCRYPTED"),
                ("Content-Domain", "RFC822"),
                ("DEK-Info", "DES-CBC,BFF968AA74691AC1"),
            ]),
            contents: b"encrypted content here".to_vec(),
        };
        roundtrip(&[original]);
    }

    #[test]
    fn encode_skip_headers_when_empty() {
        let block = Block {
            r#type: "TEST".into(),
            headers: Headers::new(),
            contents: b"data".to_vec(),
        };
        let pem = encode(&[block]);
        let pem_str = core::str::from_utf8(&pem).unwrap();
        let lines: Vec<&str> = pem_str.lines().collect();
        assert_eq!(lines[0], "-----BEGIN TEST-----");
        assert_eq!(lines[1], "ZGF0YQ==");
        assert_eq!(lines[2], "-----END TEST-----");
    }

    #[test]
    fn encode_include_headers() {
        let block = Block {
            r#type: "MESSAGE".into(),
            headers: Headers::from_pairs(&[("X-Custom", "value123")]),
            contents: b"data".to_vec(),
        };
        let pem = encode(&[block]);
        let pem_str = core::str::from_utf8(&pem).unwrap();
        assert!(pem_str.contains("X-Custom: value123\n"));
        assert!(pem_str.contains("\n\nZGF0YQ=="));
    }

    #[test]
    fn decode_no_newline_before_end_marker() {
        let pem = b"-----BEGIN DATA-----\n\
                     ZGF0YQ==\n\
                     -----END DATA-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    #[test]
    fn decode_header_values_with_colons() {
        let pem = b"-----BEGIN MESSAGE-----\n\
                     Proc-Type: 4,ENCRYPTED\n\
                     Key-Info: RSA,abc123\n\
                     \n\
                     ZGF0YQ==\n\
                     -----END MESSAGE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        let h_vec: Vec<(&str, &str)> = block.headers.iter().collect();
        assert_eq!(h_vec.len(), 2);
        assert_eq!(h_vec[1].0, "Key-Info");
        assert!(h_vec[1].1.contains("RSA,abc123"));
    }

    #[test]
    fn decode_only_first_block_on_error() {
        let pem = b"-----BEGIN GOOD-----\n\
                     R29vZEF0YQ==\n\
                     -----END GOOD-----\n\
                     -----BEGIN BAD-----\n\
                     !!!invalid!!!\n\
                     -----END BAD-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 2);
        assert!(decoded[0].is_ok());
        assert!(decoded[1].is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn match_openssl_output() {
        use std::process::Command;

        let result = Command::new("sh").arg("-c").arg("which openssl").output();

        if result.map(|r| r.status.success()).unwrap_or(false) {
            let output = Command::new("openssl")
                .args([
                    "req",
                    "-x509",
                    "-newkey",
                    "rsa:2048",
                    "-keyout",
                    "/dev/null",
                    "-out",
                    "/dev/stdout",
                    "-days",
                    "365",
                    "-nodes",
                    "-subj",
                    "/CN=TestCert",
                ])
                .output()
                .expect("openssl failed");

            assert!(output.status.success());
            let pem_bytes = output.stdout;

            let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem_bytes).collect();
            assert_eq!(decoded.len(), 1);
            let block = decoded[0].as_ref().unwrap();
            assert_eq!(block.r#type, "CERTIFICATE");
            assert!(!block.contents.is_empty());

            let reencoded = encode(&[block.clone()]);
            let re_decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&reencoded).collect();
            assert_eq!(re_decoded.len(), 1);
            assert_eq!(re_decoded[0].as_ref().unwrap().contents, block.contents);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn match_python_output() {
        use std::process::Command;

        let result = Command::new("sh").arg("-c").arg("which python3").output();

        if result.map(|r| r.status.success()).unwrap_or(false) {
            let py_script = "\
import base64
data = b'Python PEM test data'
b64 = base64.b64encode(data).decode('ascii')
pem = f\"-----BEGIN PYTHON DATA-----\\n{b64}\\n-----END PYTHON DATA-----\\n\"
print(pem, end='')
";
            let output = Command::new("python3")
                .arg("-c")
                .arg(py_script)
                .output()
                .expect("python3 failed");

            assert!(output.status.success());
            let pem_bytes = output.stdout;

            let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem_bytes).collect();
            assert_eq!(decoded.len(), 1);
            let block = decoded[0].as_ref().unwrap();
            assert_eq!(block.r#type, "PYTHON DATA");
            assert_eq!(block.contents, b"Python PEM test data");
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn match_openssl_ec_key_roundtrip() {
        use std::process::Command;

        let result = Command::new("sh").arg("-c").arg("which openssl").output();

        if result.map(|r| r.status.success()).unwrap_or(false) {
            let output = Command::new("openssl")
                .args(["ecparam", "-genkey", "-name", "prime256v1", "-outform", "PEM"])
                .output()
                .expect("openssl ecparam failed");

            assert!(output.status.success());
            let pem_bytes = output.stdout;

            let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem_bytes).collect();
            assert!(decoded.len() >= 1);
            let first_block = decoded[0].as_ref().unwrap();
            assert!(!first_block.contents.is_empty());

            let reencoded = encode(&[first_block.clone()]);
            let re_decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&reencoded).collect();
            assert_eq!(re_decoded.len(), 1);
            assert_eq!(re_decoded[0].as_ref().unwrap().contents, first_block.contents);
        }
    }

    #[test]
    fn encode_with_special_chars_in_label() {
        let label = "X.509 CERTIFICATE";
        let block = Block {
            r#type: label,
            headers: Headers::new(),
            contents: b"data".to_vec(),
        };
        let pem = encode(&[block.clone()]);
        assert!(pem.starts_with(b"-----BEGIN X.509 CERTIFICATE-----\n"));
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().r#type, label);
    }

    #[test]
    fn decode_non_ascii_rejected() {
        let mut pem = b"-----BEGIN CERTIFICATE-----\n".to_vec();
        pem.push(0x80);
        pem.extend_from_slice(b"\n");
        pem.extend_from_slice(b"SGVsbG8=\n");
        pem.extend_from_slice(b"-----END CERTIFICATE-----\n");

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err());
    }

    #[test]
    fn decode_punctuation_in_body_rejected() {
        let pem = b"-----BEGIN DATA-----\n\
                     ZGF0!YQ==\n\
                     -----END DATA-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err(), "punctuation in body must be rejected");
    }

    #[test]
    fn decode_binary_junk_mid_body_rejected() {
        let mut pem = b"-----BEGIN DATA-----\nZGF0YQ==\n".to_vec();
        pem.extend_from_slice(&[0x00, 0x01, 0x02]);
        pem.extend_from_slice(b"\n-----END DATA-----\n");

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err(), "binary junk mid-body must be rejected");
    }

    #[test]
    fn decode_lax_vertical_tab_and_form_feed() {
        let pem = b"-----BEGIN DATA-----\n\
                     ZGF0\x0bYQ==\x0c\n\
                     -----END DATA-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    // ─── RFC 7468 Section 5-13: Label-specific examples ───

    #[test]
    fn decode_rfc7468_section6_x509_crl() {
        let pem = b"-----BEGIN X509 CRL-----\n\
                     MIIB9DCCAV8CAQEwCwYJKoZIhvcNAQEFMIIBCDEXMBUGA1UEChMOVmVyaVNpZ24s\n\
                     IEluYy4xHzAdBgNVBAsTFlZlcmlTaWduIFRydXN0IE5ldHdvcmsxRjBEBgNVBAsT\n\
                     PXd3dy52ZXJpc2lnbi5jb20vcmVwb3NpdG9yeS9SUEEgSW5jb3JwLiBieSBSZWYu\n\
                     LExJQUIuTFREKGMpOTgxHjAcBgNVBAsTFVBlcnNvbmEgTm90IFZhbGlkYXRlZDEm\n\
                     MCQGA1UECxMdRGlnaXRhbCBJRCBDbGFzcyAxIC0gTmV0c2NhcGUxGDAWBgNVBAMU\n\
                     D1NpbW9uIEpvc2Vmc3NvbjEiMCAGCSqGSIb3DQEJARYTc2ltb25Aam9zZWZzc29u\n\
                     Lm9yZxcNMDYxMjI3MDgwMjM0WhcNMDcwMjA3MDgwMjM1WjAjMCECEC4QNwPfRoWd\n\
                     elUNpllhhTgXDTA2MTIyNzA4MDIzNFowCwYJKoZIhvcNAQEFA4GBAD0zX+J2hkcc\n\
                     Nbrq1Dn5IKL8nXLgPGcHv1I/le1MNo9t1ohGQxB5HnFUkRPAY82fR6Epor4aHgVy\n\
                     b+5y+neKN9Kn2mPF4iiun+a4o26CjJ0pArojCL1p8T0yyi9Xxvyc/ezaZ98HiIyP\n\
                     c3DGMNR+oUmSjKZ0jIhAYmeLxaPHfQwR\n\
                     -----END X509 CRL-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "X509 CRL");
        assert!(block.headers.is_empty());
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_section7_certificate_request() {
        let pem = b"-----BEGIN CERTIFICATE REQUEST-----\n\
                     MIIBWDCCAQcCAQAwTjELMAkGA1UEBhMCU0UxJzAlBgNVBAoTHlNpbW9uIEpvc2Vm\n\
                     c3NvbiBEYXRha29uc3VsdCBBQjEWMBQGA1UEAxMNam9zZWZzc29uLm9yZzBOMBAG\n\
                     ByqGSM49AgEGBSuBBAAhAzoABLLPSkuXY0l66MbxVJ3Mot5FCFuqQfn6dTs+9/CM\n\
                     EOlSwVej77tj56kj9R/j9Q+LfysX8FO9I5p3oGIwYAYJKoZIhvcNAQkOMVMwUTAY\n\
                     BgNVHREEETAPgg1qb3NlZnNzb24ub3JnMAwGA1UdEwEB/wQCMAAwDwYDVR0PAQH/\n\
                     BAUDAwegADAWBgNVHSUBAf8EDDAKBggrBgEFBQcDATAKBggqhkjOPQQDAgM/ADA8\n\
                     AhxBvfhxPFfbBbsE1NoFmCUczOFApEuQVUw3ZP69AhwWXk3dgSUsKnuwL5g/ftAY\n\
                     dEQc8B8jAcnuOrfU\n\
                     -----END CERTIFICATE REQUEST-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "CERTIFICATE REQUEST");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_section8_pkcs7() {
        let pem = b"-----BEGIN PKCS7-----\n\
                     MIHjBgsqhkiG9w0BCRABF6CB0zCB0AIBADFho18CAQCgGwYJKoZIhvcNAQUMMA4E\n\
                     CLfrI6dr0gUWAgITiDAjBgsqhkiG9w0BCRADCTAUBggqhkiG9w0DBwQIZpECRWtz\n\
                     u5kEGDCjerXY8odQ7EEEromZJvAurk/j81IrozBSBgkqhkiG9w0BBwEwMwYLKoZI\n\
                     hvcNAQkQAw8wJDAUBggqhkiG9w0DBwQI0tCBcU09nxEwDAYIKwYBBQUIAQIFAIAQ\n\
                     OsYGYUFdAH0RNc1p4VbKEAQUM2Xo8PMHBoYdqEcsbTodlCFAZH4=\n\
                     -----END PKCS7-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "PKCS7");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_section9_cms() {
        let pem = b"-----BEGIN CMS-----\n\
                     MIGDBgsqhkiG9w0BCRABCaB0MHICAQAwDQYLKoZIhvcNAQkQAwgwXgYJKoZIhvcN\n\
                     AQcBoFEET3icc87PK0nNK9ENqSxItVIoSa0o0S/ISczMs1ZIzkgsKk4tsQ0N1nUM\n\
                     dvb05OXi5XLPLEtViMwvLVLwSE0sKlFIVHAqSk3MBkkBAJv0Fx0=\n\
                     -----END CMS-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "CMS");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_section11_encrypted_private_key() {
        let pem = b"-----BEGIN ENCRYPTED PRIVATE KEY-----\n\
                     MIHNMEAGCSqGSIb3DQEFDTAzMBsGCSqGSIb3DQEFDDAOBAghhICA6T/51QICCAAw\n\
                     FAYIKoZIhvcNAwcECBCxDgvI59i9BIGIY3CAqlMNBgaSI5QiiWVNJ3IpfLnEiEsW\n\
                     Z0JIoHyRmKK/+cr9QPLnzxImm0TR9s4JrG3CilzTWvb0jIvbG3hu0zyFPraoMkap\n\
                     8eRzWsIvC5SVel+CSjoS2mVS87cyjlD+txrmrXOVYDE+eTgMLbrLmsWh3QkCTRtF\n\
                     QC7k0NNzUHTV9yGDwfqMbw==\n\
                     -----END ENCRYPTED PRIVATE KEY-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "ENCRYPTED PRIVATE KEY");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_section12_attribute_certificate() {
        let pem = b"-----BEGIN ATTRIBUTE CERTIFICATE-----\n\
                     MIICKzCCAZQCAQEwgZeggZQwgYmkgYYwgYMxCzAJBgNVBAYTAlVTMREwDwYDVQQI\n\
                     DAhOZXcgWW9yazEUMBIGA1UEBwwLU3RvbnkgQnJvb2sxDzANBgNVBAoMBkNTRTU5\n\
                     MjE6MDgGA1UEAwwxU2NvdHQgU3RhbGxlci9lbWFpbEFkZHJlc3M9c3N0YWxsZXJA\n\
                     aWMuc3VueXNiLmVkdQIGARWrgUUSoIGMMIGJpIGGMIGDMQswCQYDVQQGEwJVUzER\n\
                     MA8GA1UECAwITmV3IFlvcmsxFDASBgNVBAcMC1N0b255IEJyb29rMQ8wDQYDVQQK\n\
                     DAZDU0U1OTIxOjA4BgNVBAMMMVNjb3R0IFN0YWxsZXIvZW1haWxBZGRyZXNzPXNz\n\
                     dGFsbGVyQGljLnN1bnlzYi5lZHUwDQYJKoZIhvcNAQEFBQACBgEVq4FFSjAiGA8z\n\
                     OTA3MDIwMTA1MDAwMFoYDzM5MTEwMTMxMDUwMDAwWjArMCkGA1UYSDEiMCCGHmh0\n\
                     dHA6Ly9pZGVyYXNobi5vcmcvaW5kZXguaHRtbDANBgkqhkiG9w0BAQUFAAOBgQAV\n\
                     M9axFPXXozEFcer06bj9MCBBCQLtAM7ZXcZjcxyva7xCBDmtZXPYUluHf5OcWPJz\n\
                     5XPus/xS9wBgtlM3fldIKNyNO8RsMp6Ocx+PGlICc7zpZiGmCYLl64lAEGPO/bsw\n\
                     Smluak1aZIttePeTAHeJJs8izNJ5aR3Wcd3A5gLztQ==\n\
                     -----END ATTRIBUTE CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "ATTRIBUTE CERTIFICATE");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_section13_public_key() {
        let pem = b"-----BEGIN PUBLIC KEY-----\n\
                     MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEn1LlwLN/KBYQRVH6HfIMTzfEqJOVztLe\n\
                     kLchp2hi78cCaMY81FBlYs8J9l7krc+M4aBeCGYFjba+hiXttJWPL7ydlE+5UG4U\n\
                     Nkn3Eos8EiZByi9DVsyfy9eejh+8AXgp\n\
                     -----END PUBLIC KEY-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "PUBLIC KEY");
        assert!(!block.contents.is_empty());
    }

    // ─── RFC 7468 Appendix A: Non-standard label variants ───

    #[test]
    fn decode_rfc7468_appendix_x509_certificate() {
        let pem = b"-----BEGIN X509 CERTIFICATE-----\n\
                     MIIBHDCBxaADAgECAgIcxzAJBgcqhkjOPQQBMBAxDjAMBgNVBAMUBVBLSVghMB4X\n\
                     DTE0MDkxNDA2MTU1MFoXDTI0MDkxNDA2MTU1MFowEDEOMAwGA1UEAxQFUEtJWCEw\n\
                     WTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATwoQSr863QrR0PoRIYQ96H7WykDePH\n\
                     Wa0eVAE24bth43wCNc+U5aZ761dhGhSSJkVWRgVH5+prLIr+nzfIq+X4oxAwDjAM\n\
                     BgNVHRMBAf8EAjAAMAkGByqGSM49BAEDRwAwRAIfMdKS5F63lMnWVhi7uaKJzKCs\n\
                     NnY/OKgBex6MIEAv2AIhAI2GdvfL+mGvhyPZE+JxRxWChmggb5/9eHdUcmW/jkOH\n\
                     -----END X509 CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "X509 CERTIFICATE");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_appendix_dot_x509_certificate() {
        let pem = b"-----BEGIN X.509 CERTIFICATE-----\n\
                     MIIBHDCBxaADAgECAgIcxzAJBgcqhkjOPQQBMBAxDjAMBgNVBAMUBVBLSVghMB4X\n\
                     DTE0MDkxNDA2MTU1MFoXDTI0MDkxNDA2MTU1MFowEDEOMAwGA1UEAxQFUEtJWCEw\n\
                     WTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATwoQSr863QrR0PoRIYQ96H7WykDePH\n\
                     Wa0eVAE24bth43wCNc+U5aZ761dhGhSSJkVWRgVH5+prLIr+nzfIq+X4oxAwDjAM\n\
                     BgNVHRMBAf8EAjAAMAkGByqGSM49BAEDRwAwRAIfMdKS5F63lMnWVhi7uaKJzKCs\n\
                     NnY/OKgBex6MIEAv2AIhAI2GdvfL+mGvhyPZE+JxRxWChmggb5/9eHdUcmW/jkOH\n\
                     -----END X.509 CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "X.509 CERTIFICATE");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_appendix_new_certificate_request() {
        let pem = b"-----BEGIN NEW CERTIFICATE REQUEST-----\n\
                     MIIBWDCCAQcCAQAwTjELMAkGA1UEBhMCU0UxJzAlBgNVBAoTHlNpbW9uIEpvc2Vm\n\
                     c3NvbiBEYXRha29uc3VsdCBBQjEWMBQGA1UEAxMNam9zZWZzc29uLm9yZzBOMBAG\n\
                     ByqGSM49AgEGBSuBBAAhAzoABLLPSkuXY0l66MbxVJ3Mot5FCFuqQfn6dTs+9/CM\n\
                     EOlSwVej77tj56kj9R/j9Q+LfysX8FO9I5p3oGIwYAYJKoZIhvcNAQkOMVMwUTAY\n\
                     BgNVHREEETAPgg1qb3NlZnNzb24ub3JnMAwGA1UdEwEB/wQCMAAwDwYDVR0PAQH/\n\
                     BAUDAwegADAWBgNVHSUBAf8EDDAKBggrBgEFBQcDATAKBggqhkjOPQQDAgM/ADA8\n\
                     AhxBvfhxPFfbBbsE1NoFmCUczOFApEuQVUw3ZP69AhwWXk3dgSUsKnuwL5g/ftAY\n\
                     dEQc8B8jAcnuOrfU\n\
                     -----END NEW CERTIFICATE REQUEST-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "NEW CERTIFICATE REQUEST");
        assert!(!block.contents.is_empty());
    }

    #[test]
    fn decode_rfc7468_appendix_certificate_chain() {
        let pem = b"-----BEGIN CERTIFICATE CHAIN-----\n\
                     MIHjBgsqhkiG9w0BCRABF6CB0zCB0AIBADFho18CAQCgGwYJKoZIhvcNAQUMMA4E\n\
                     CLfrI6dr0gUWAgITiDAjBgsqhkiG9w0BCRADCTAUBggqhkiG9w0DBwQIZpECRWtz\n\
                     u5kEGDCjerXY8odQ7EEEromZJvAurk/j81IrozBSBgkqhkiG9w0BBwEwMwYLKoZI\n\
                     hvcNAQkQAw8wJDAUBggqhkiG9w0DBwQI0tCBcU09nxEwDAYIKwYBBQUIAQIFAIAQ\n\
                     OsYGYUFdAH0RNc1p4VbKEAQUM2Xo8PMHBoYdqEcsbTodlCFAZH4=\n\
                     -----END CERTIFICATE CHAIN-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "CERTIFICATE CHAIN");
        assert!(!block.contents.is_empty());
    }

    // ─── Bad PEM rejection (matching Go's badPEMTests) ───

    #[test]
    fn reject_too_few_trailing_dashes_begin() {
        let pem = b"-----BEGIN FOO----\ndGVzdA==\n-----END FOO-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert!(
            decoded.is_empty() || decoded[0].is_err(),
            "expected error for BEGIN with 4 trailing dashes"
        );
    }

    #[test]
    fn reject_too_many_trailing_dashes_begin() {
        let pem = b"-----BEGIN FOO-------\ndGVzdA==\n-----END FOO-------\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert!(
            decoded.is_empty() || decoded[0].is_err(),
            "expected error for BEGIN with 7 trailing dashes"
        );
    }

    #[test]
    fn reject_too_few_trailing_dashes_end() {
        let pem = b"-----BEGIN FOO-----\ndGVzdA==\n-----END FOO----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert!(
            decoded.is_empty() || decoded[0].is_err(),
            "expected error for END with 4 trailing dashes"
        );
    }

    #[test]
    fn reject_too_many_trailing_dashes_end() {
        let pem = b"-----BEGIN FOO-----\ndGVzdA==\n-----END FOO------\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert!(
            decoded.is_empty() || decoded[0].is_err(),
            "expected error for END with 6 trailing dashes"
        );
    }

    #[test]
    fn reject_missing_ending_space() {
        let pem = b"-----BEGIN FOO-----\ndGVzdA==\n-----ENDBAR-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert!(
            decoded.is_empty() || decoded[0].is_err(),
            "expected error for missing space between END and label"
        );
    }

    #[test]
    fn reject_trailing_non_whitespace_on_end() {
        let pem = b"-----BEGIN FOO-----\ndGVzdA==\n-----END FOO----- .\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err(), "expected error for trailing '. ' on END line");
    }

    #[test]
    fn reject_repeating_begin_no_end() {
        let input = b"-----BEGIN \n".repeat(100);
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&input).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err(), "expected error for 100 repeated BEGIN lines with no END");
    }

    #[test]
    fn reject_only_end_marker() {
        let pem = b"-----END PUBLIC KEY-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert!(
            decoded.is_empty() || decoded[0].is_err(),
            "expected no valid block from input containing only END marker"
        );
    }

    // ─── Go-equivalence: Strange cases ───

    #[test]
    fn decode_empty_lines_before_end() {
        let pem = b"-----BEGIN DATA-----\n\
                     ZGF0YQ==\n\
                     \n\
                     \n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    #[test]
    fn decode_blank_lines_between_begin_and_end() {
        let pem = b"-----BEGIN EMPTY-----\n\
                     \n\
                     \n\
                     \n\
                     -----END EMPTY-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].as_ref().unwrap().contents.is_empty());
    }

    #[test]
    fn decode_header_key_only_no_value() {
        let pem = b"-----BEGIN DATA-----\n\
                     Key-Only:\n\
                     \n\
                     ZGF0YQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.contents, b"data");
        let header = block.headers.iter().find(|&(k, _)| k == "Key-Only");
        assert!(header.is_some());
        assert_eq!(header.unwrap().1, "");
    }

    #[test]
    fn decode_multiple_blank_lines_before_body() {
        let pem = b"-----BEGIN DATA-----\n\
                     \n\
                     \n\
                     \n\
                     \n\
                     ZGF0YQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    #[test]
    fn decode_header_value_with_spaces_only() {
        let pem = b"-----BEGIN DATA-----\n\
                     Key:   \n\
                     \n\
                     ZGF0YQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.contents, b"data");
    }

    #[test]
    fn decode_header_continuation_empty_line() {
        let pem = b"-----BEGIN DATA-----\n\
                     Key: start\n\
                     \t\n\
                     \n\
                     ZGF0YQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.contents, b"data");
    }

    #[test]
    fn decode_header_with_no_blank_line_before_body() {
        let pem = b"-----BEGIN FOO-----\n\
                     Header: 1\n\
                     -----END FOO-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "FOO");
        assert_eq!(block.headers.len(), 1);
        assert_eq!(block.headers.iter().next().unwrap().0, "Header");
        assert_eq!(block.headers.iter().next().unwrap().1, "1");
        assert!(block.contents.is_empty());
    }

    // ─── Roundtrip: various content sizes ───

    #[test]
    fn roundtrip_0_bytes() {
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents: Vec::new(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_1_byte() {
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents: vec![0x00],
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_2_bytes() {
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents: vec![0x00, 0x01],
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_3_bytes() {
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents: vec![0x00, 0x01, 0x02],
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_64_bytes() {
        let contents: Vec<u8> = (0u8..64).collect();
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents,
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_65_bytes() {
        let contents: Vec<u8> = (0u8..65).collect();
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents,
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_128_bytes() {
        let contents: Vec<u8> = (0u8..128).collect();
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents,
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_255_bytes() {
        let contents: Vec<u8> = (0u8..255).collect();
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents,
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_256_bytes() {
        let contents: Vec<u8> = (0u8..=255).collect();
        let blocks = [Block {
            r#type: "DATA".into(),
            headers: Headers::new(),
            contents,
        }];
        roundtrip(&blocks);
    }

    // ─── Encode line wrapping edge cases ───

    #[test]
    fn encode_line_wrapping_exact_multiple_of_64() {
        let contents = vec![b'x'; 48];
        let blocks = [Block {
            r#type: "TEST".into(),
            headers: Headers::new(),
            contents,
        }];
        let pem = encode(&blocks);
        let pem_str = core::str::from_utf8(&pem).unwrap();
        let body = pem_str
            .strip_prefix("-----BEGIN TEST-----\n")
            .unwrap()
            .strip_suffix("\n-----END TEST-----\n")
            .unwrap();
        assert_eq!(body.lines().count(), 1);
        assert!(body.len() <= 64, "single line should be <= 64 chars");
    }

    #[test]
    fn encode_line_wrapping_exactly_64_per_line() {
        let contents = vec![0u8; 64];
        let blocks = [Block {
            r#type: "TEST".into(),
            headers: Headers::new(),
            contents,
        }];
        let pem = encode(&blocks);
        let pem_str = core::str::from_utf8(&pem).unwrap();
        let body = pem_str
            .strip_prefix("-----BEGIN TEST-----\n")
            .unwrap()
            .strip_suffix("\n-----END TEST-----\n")
            .unwrap();
        assert!(!body.is_empty());
    }

    // ─── Label edge cases ───

    #[test]
    fn roundtrip_label_with_hyphen() {
        let blocks = [Block {
            r#type: "TEST-DATA".into(),
            headers: Headers::new(),
            contents: b"hyphen test".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_label_with_numbers() {
        let blocks = [Block {
            r#type: "AES-256-CBC".into(),
            headers: Headers::new(),
            contents: b"number test".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_label_with_underscores() {
        let blocks = [Block {
            r#type: "MY_CUSTOM_LABEL".into(),
            headers: Headers::new(),
            contents: b"underscore test".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_label_with_at_sign() {
        let blocks = [Block {
            r#type: "KEY@DOMAIN.COM".into(),
            headers: Headers::new(),
            contents: b"at sign test".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_single_char_label() {
        let blocks = [Block {
            r#type: "X".into(),
            headers: Headers::new(),
            contents: b"single char".to_vec(),
        }];
        roundtrip(&blocks);
    }

    #[test]
    fn roundtrip_long_label() {
        let label = "A".repeat(100);
        let blocks = [Block {
            r#type: &label,
            headers: Headers::new(),
            contents: b"long label test".to_vec(),
        }];
        roundtrip(&blocks);
    }

    // ─── Decoding: whitespace robustness ───

    #[test]
    fn decode_leading_whitespace_before_begin() {
        let pem = b"\n\n\n-----BEGIN DATA-----\n\
                     ZGF0YQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    #[test]
    fn decode_tabs_in_body_only() {
        let pem = b"-----BEGIN DATA-----\n\
                     ZGF0\tYQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    #[test]
    fn decode_line_breaks_within_base64_line() {
        let pem = b"-----BEGIN DATA-----\n\
                     ZGF0\n\
                     YQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    // ─── Encode: header format precision ───

    #[test]
    fn encode_multiple_header_pairs() {
        let block = Block {
            r#type: "DATA".into(),
            headers: Headers::from_pairs(&[("Key1", "val1"), ("Key2", "val2"), ("Key3", "val3")]),
            contents: b"multi header".to_vec(),
        };
        let pem = encode(&[block.clone()]);
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().headers, block.headers);
        assert_eq!(decoded[0].as_ref().unwrap().contents, block.contents);
    }

    #[test]
    fn encode_header_with_empty_value() {
        let block = Block {
            r#type: "DATA".into(),
            headers: Headers::from_pairs(&[("Empty-Key", "")]),
            contents: b"empty value".to_vec(),
        };
        let pem = encode(&[block.clone()]);
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(&pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().headers.iter().next().unwrap().1, "");
    }

    // ─── Multiple blocks roundtrip ───

    #[test]
    fn roundtrip_five_blocks() {
        let type_strs: Vec<String> = (0..5).map(|i| alloc::format!("BLOCK{}", i)).collect();
        let mut blocks = Vec::new();
        for i in 0..5 {
            blocks.push(Block {
                r#type: &type_strs[i],
                headers: Headers::new(),
                contents: alloc::format!("content {}", i).into_bytes(),
            });
        }
        roundtrip(&blocks);
    }

    #[test]
    fn decode_block_with_explanatory_text() {
        let pem = b"Subject: CN=Atlantis\n\
                     Issuer: CN=Atlantis\n\
                     Validity: from 7/9/2012 3:10:38 AM UTC to 7/9/2013 3:10:37 AM UTC\n\
                     -----BEGIN CERTIFICATE-----\n\
                     SGVsbG8=\n\
                     -----END CERTIFICATE-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"Hello");
    }

    // ─── Fuzz-style: random roundtrip ───

    #[test]
    fn fuzz_random_roundtrip() {
        let mut state: u64 = 42;
        for i in 0..20 {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let len = (state % 512) as usize;
            let contents: Vec<u8> = (0..len)
                .map(|j| {
                    ((state
                        .wrapping_add(j as u64)
                        .wrapping_mul(1103515245)
                        .wrapping_add(12345))
                        >> 16) as u8
                })
                .collect();

            let type_str = alloc::format!("TEST{}", i);
            let mut headers = Headers::new();
            for hi in 0..(state % 3) as usize {
                headers.push(&alloc::format!("H{}{}", i, hi), &alloc::format!("V{}{}", i, hi));
            }
            let blocks = [Block {
                r#type: &type_str,
                headers,
                contents,
            }];
            roundtrip(&blocks);
        }
    }

    // ─── RFC 1421: Encrypted message format with header continuation ───

    #[test]
    fn decode_rfc1421_full_encrypted_message() {
        let pem: &[u8] = b"-----BEGIN PRIVACY-ENHANCED MESSAGE-----\n\
                     Proc-Type: 4,ENCRYPTED\n\
                     Content-Domain: RFC822\n\
                     DEK-Info: DES-CBC,F8143EDE5960C597\n\
                     Originator-ID-Symmetric: linn@zendia.enet.dec.com,,\n\
                     Recipient-ID-Symmetric: linn@zendia.enet.dec.com,ptf-kmc,3\n\
                     Key-Info: DES-ECB,RSA-MD2,9FD3AAD2F2691B9A,\n\x20B70665BB9BF7CBCDA60195DB94F727D3\n\
                     \n\
                     SGVsbG8=\n\
                     -----END PRIVACY-ENHANCED MESSAGE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "PRIVACY-ENHANCED MESSAGE");
        assert_eq!(block.contents, b"Hello");
        assert!(block.headers.len() >= 4);
    }

    // ─── Edge: base64 body with only padding ───

    #[test]
    fn decode_body_only_padding() {
        let pem = b"-----BEGIN DATA-----\n=\n-----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err(), "a single = is invalid base64");
    }

    #[test]
    fn decode_body_only_padding_pair() {
        let pem = b"-----BEGIN DATA-----\n==\n-----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert!(decoded[0].is_err(), "just == is invalid base64");
    }

    // ─── Edge: END line with no newline and matching label ───

    #[test]
    fn decode_end_no_newline_matching() {
        let pem = b"-----BEGIN FOO-----\nZGF0YQ==\n-----END FOO-----";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    // ─── End-to-end: Read PEM from RFC 7468 Section 5 explanatory text example ───

    #[test]
    fn decode_rfc7468_section5_explanatory_text() {
        let pem = b"Subject: CN=Atlantis\n\
                     Issuer: CN=Atlantis\n\
                     Validity: from 7/9/2012 3:10:38 AM UTC to 7/9/2013 3:10:37 AM UTC\n\
                     -----BEGIN CERTIFICATE-----\n\
                     MIIBmTCCAUegAwIBAgIBKjAJBgUrDgMCHQUAMBMxETAPBgNVBAMTCEF0bGFudGlz\n\
                     MB4XDTEyMDcwOTAzMTAzOFoXDTEzMDcwOTAzMTAzN1owEzERMA8GA1UEAxMIQXRs\n\
                     YW50aXMwXDANBgkqhkiG9w0BAQEFAANLADBIAkEAu+BXo+miabDIHHx+yquqzqNh\n\
                     Ryn/XtkJIIHVcYtHvIX+S1x5ErgMoHehycpoxbErZmVR4GCq1S2diNmRFZCRtQID\n\
                     AQABo4GJMIGGMAwGA1UdEwEB/wQCMAAwIAYDVR0EAQH/BBYwFDAOMAwGCisGAQQB\n\
                     gjcCARUDAgeAMB0GA1UdJQQWMBQGCCsGAQUFBwMCBggrBgEFBQcDAzA1BgNVHQEE\n\
                     LjAsgBA0jOnSSuIHYmnVryHAdywMoRUwEzERMA8GA1UEAxMIQXRsYW50aXOCASow\n\
                     CQYFKw4DAh0FAANBAKi6HRBaNEL5R0n56nvfclQNaXiDT174uf+lojzA4lhVInc0\n\
                     ILwpnZ1izL4MlI9eCSHhVQBHEp2uQdXJB+d5Byg=\n\
                     -----END CERTIFICATE-----\n";

        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        let block = decoded[0].as_ref().unwrap();
        assert_eq!(block.r#type, "CERTIFICATE");
        assert!(!block.contents.is_empty());
    }

    // ─── decode_base64: trailing whitespace before END marker ───

    #[test]
    fn decode_base64_with_trailing_whitespace_in_line() {
        let pem = b"-----BEGIN DATA-----\n\
                     ZGF0YQ==   \n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }

    // ─── decode_base64: leading whitespace in body line ───

    #[test]
    fn decode_base64_with_leading_whitespace() {
        let pem = b"-----BEGIN DATA-----\n\
                        ZGF0YQ==\n\
                     -----END DATA-----\n";
        let decoded: Vec<Result<Block<'_>, PemError<'_>>> = decode(pem).collect();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].as_ref().unwrap().contents, b"data");
    }
}
