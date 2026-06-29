use crate::common::hpack_qpack_shared::{
    DecodeError, STATIC_TABLE, decode_prefix, decode_value, encode_prefix, encode_raw, encode_value, huffman_decode,
};

// These are re-exported so existing tests and code that references them still work.

pub struct QpackEncoder;

impl QpackEncoder {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&mut self, fields: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = Vec::new();
        encode_prefix(0, 8, &mut buf, |v| v);
        encode_prefix(0, 7, &mut buf, |v| v);
        for &(name, value) in fields {
            self.encode_field(name, value, &mut buf);
        }
        buf
    }

    fn encode_field(&mut self, name: &str, value: &str, buf: &mut Vec<u8>) {
        if let Some(idx) = self.find_full_match(name, value) {
            encode_prefix(idx as u64, 6, buf, |v| 0xC0 | v);
            return;
        }

        if let Some(idx) = self.find_name_match(name) {
            encode_prefix(idx as u64, 4, buf, |v| 0x50 | v);
            encode_value(value, buf);
            return;
        }

        let name_bytes = name.as_bytes();
        encode_prefix(name_bytes.len() as u64, 3, buf, |v| 0x20 | v);
        encode_raw(name_bytes, buf);
        encode_value(value, buf);
    }

    fn find_full_match(&self, name: &str, value: &str) -> Option<usize> {
        STATIC_TABLE
            .iter()
            .position(|&(n, v)| n.eq_ignore_ascii_case(name) && v == value)
    }

    fn find_name_match(&self, name: &str) -> Option<usize> {
        STATIC_TABLE.iter().position(|&(n, _)| n.eq_ignore_ascii_case(name))
    }
}

pub struct QpackDecoder;

impl QpackDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<(String, String)>, DecodeError> {
        let mut pos = 0;
        let (_, ric_len) = decode_prefix(&data[pos..], 8)?;
        pos += ric_len;
        let (_, base_len) = decode_prefix(&data[pos..], 7)?;
        pos += base_len;
        let mut fields = Vec::new();
        while pos < data.len() {
            let (field, consumed) = self.decode_field(&data[pos..])?;
            fields.push(field);
            pos += consumed;
        }
        Ok(fields)
    }

    fn decode_field(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let first = *data.first().ok_or(DecodeError::Truncated)?;
        if first & 0x80 != 0 {
            self.decode_indexed(data)
        } else if first & 0x40 != 0 {
            self.decode_literal_name_ref(data)
        } else if first & 0x20 != 0 {
            self.decode_literal_literal(data)
        } else {
            Err(DecodeError::BadPrefix)
        }
    }

    fn decode_indexed(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let first = *data.first().ok_or(DecodeError::Truncated)?;
        let t_bit = (first & 0x40) != 0;
        let (idx, consumed) = decode_prefix(data, 6)?;
        if t_bit {
            let tbl_idx = idx as usize;
            if tbl_idx >= STATIC_TABLE.len() {
                return Ok(((format!("unknown-{tbl_idx}"), String::new()), consumed));
            }
            let (name, value) = STATIC_TABLE[tbl_idx];
            Ok(((name.to_string(), value.to_string()), consumed))
        } else {
            Ok(((format!("dynamic-{idx}"), String::new()), consumed))
        }
    }

    fn decode_literal_name_ref(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let first = *data.first().ok_or(DecodeError::Truncated)?;
        let t_bit = (first & 0x10) != 0;
        let (idx, consumed) = decode_prefix(data, 4)?;
        let name = if t_bit {
            let tbl_idx = idx as usize;
            if tbl_idx < STATIC_TABLE.len() {
                STATIC_TABLE[tbl_idx].0.to_string()
            } else {
                format!("unknown-{tbl_idx}")
            }
        } else {
            format!("dynamic-{idx}")
        };
        let (value, vc) = decode_value(&data[consumed..])?;
        Ok(((name, value), consumed + vc))
    }

    fn decode_literal_literal(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let first = *data.first().ok_or(DecodeError::Truncated)?;
        let _name_huffman = (first & 0x08) != 0;
        let (name_len, consumed) = decode_prefix(data, 3)?;
        let name_start = consumed;
        let name_end = name_start + name_len as usize;
        if data.len() < name_end {
            return Err(DecodeError::Truncated);
        }
        let name_raw = &data[name_start..name_end];
        let name = if _name_huffman {
            huffman_decode(name_raw).unwrap_or_else(|| String::from_utf8_lossy(name_raw).into_owned())
        } else {
            String::from_utf8_lossy(name_raw).into_owned()
        };
        let (value, vc) = decode_value(&data[name_end..])?;
        Ok(((name, value), name_end + vc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::hpack_qpack_shared::huffman_encode;

    #[test]
    fn roundtrip_simple() {
        let mut enc = QpackEncoder::new();
        let mut dec = QpackDecoder::new();
        let input = &[(":method", "GET"), (":scheme", "https"), (":path", "/")];
        let encoded = enc.encode(input);
        let decoded = dec.decode(&encoded).unwrap();
        let expected: Vec<(String, String)> = input.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn roundtrip_with_literals() {
        let mut enc = QpackEncoder::new();
        let mut dec = QpackDecoder::new();
        let input = &[(":authority", "example.com"), ("user-agent", "stdx-h3/0.1")];
        let encoded = enc.encode(input);
        let decoded = dec.decode(&encoded).unwrap();
        let expected: Vec<(String, String)> = input.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn roundtrip_full_request() {
        let mut enc = QpackEncoder::new();
        let mut dec = QpackDecoder::new();
        let input = &[
            (":method", "GET"),
            (":scheme", "https"),
            (":authority", "cloudflare-quic.com"),
            (":path", "/"),
            ("user-agent", "stdx-h3/0.1"),
        ];
        let encoded = enc.encode(input);
        let decoded = dec.decode(&encoded).unwrap();
        let expected: Vec<(String, String)> = input.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn roundtrip_empty_value() {
        let mut enc = QpackEncoder::new();
        let mut dec = QpackDecoder::new();
        let input = &[("accept-language", "")];
        let encoded = enc.encode(input);
        let decoded = dec.decode(&encoded).unwrap();
        assert_eq!(decoded, vec![("accept-language".to_string(), String::new())]);
    }

    #[test]
    fn roundtrip_empty_field_section() {
        let mut enc = QpackEncoder::new();
        let mut dec = QpackDecoder::new();
        let input: &[(&str, &str)] = &[];
        let encoded = enc.encode(input);
        let decoded = dec.decode(&encoded).unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn roundtrip_multibyte() {
        let mut enc = QpackEncoder::new();
        let mut dec = QpackDecoder::new();
        let input = &[("x-unicode", "héllo wörld ñ")];
        let encoded = enc.encode(input);
        let decoded = dec.decode(&encoded).unwrap();
        assert_eq!(decoded, vec![("x-unicode".to_string(), "héllo wörld ñ".to_string())]);
    }

    #[test]
    fn field_section_prefix_is_present() {
        let mut enc = QpackEncoder::new();
        let encoded = enc.encode(&[(":path", "/")]);
        assert_eq!(encoded[0], 0x00, "missing Required Insert Count");
        assert_eq!(encoded[1], 0x00, "missing Base");
    }

    #[test]
    fn huffman_decode_hello() {
        let input = b"hello";
        let encoded = huffman_encode(input);
        let decoded = huffman_decode(&encoded).unwrap();
        assert_eq!(decoded, "hello");
    }

    #[test]
    fn huffman_decode_all_ascii_printable() {
        let input: Vec<u8> = (32u8..=126u8).collect();
        let encoded = huffman_encode(&input);
        let decoded = huffman_decode(&encoded).unwrap();
        let expected = String::from_utf8(input).unwrap();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn prefix_roundtrip_8bit() {
        let mut buf = Vec::new();
        for val in [0u64, 1, 100, 254, 255, 256, 1000] {
            buf.clear();
            encode_prefix(val, 8, &mut buf, |v| v);
            let (decoded, consumed) = decode_prefix(&buf, 8).unwrap();
            assert_eq!(decoded, val);
            assert_eq!(consumed, buf.len());
        }
    }

    #[test]
    fn decode_truncated_field_section() {
        let mut dec = QpackDecoder::new();
        assert!(dec.decode(&[0x00]).is_err());
        assert!(dec.decode(&[0x00, 0x00]).unwrap().is_empty());
    }
}
