use crate::common::hpack_qpack_shared::{
    DecodeError, STATIC_TABLE, decode_prefix, encode_prefix, encode_raw, huffman_decode, huffman_encode,
};

pub struct HpackEncoder {}

impl HpackEncoder {
    pub fn new() -> Self {
        HpackEncoder {}
    }

    pub fn set_max_table_size(&mut self, _size: u32) {}

    pub fn encode(&mut self, headers: &[(impl AsRef<str>, impl AsRef<str>)]) -> Vec<u8> {
        let mut buf = Vec::new();
        for (name, value) in headers {
            self.encode_field(name.as_ref(), value.as_ref(), &mut buf);
        }
        buf
    }

    fn encode_field(&mut self, name: &str, value: &str, buf: &mut Vec<u8>) {
        if let Some(idx) = self.find_full_match(name, value) {
            encode_prefix(idx as u64, 6, buf, |v| 0xC0 | v);
            return;
        }

        if let Some(idx) = self.find_name_match(name) {
            encode_prefix(idx as u64, 4, buf, |v| 0x40 | v);
            self.encode_value_str(value, buf);
            return;
        }

        let name_bytes = name.as_bytes();
        let huf_name = huffman_encode(name_bytes);
        let h = huf_name.len() < name_bytes.len();
        let name_data = if h { &huf_name } else { name_bytes };
        encode_prefix(name_data.len() as u64, 3, buf, |v| if h { 0x08 | v } else { v });
        encode_raw(name_data, buf);
        self.encode_value_str(value, buf);
    }

    fn encode_value_str(&self, value: &str, buf: &mut Vec<u8>) {
        let bytes = value.as_bytes();
        let huf = huffman_encode(bytes);
        if huf.len() < bytes.len() {
            encode_prefix(huf.len() as u64, 7, buf, |v| 0x80 | v);
            encode_raw(&huf, buf);
        } else {
            encode_prefix(bytes.len() as u64, 7, buf, |v| v & 0x7F);
            encode_raw(bytes, buf);
        }
    }

    fn find_full_match(&self, name: &str, value: &str) -> Option<usize> {
        STATIC_TABLE
            .iter()
            .position(|&(n, v)| n.eq_ignore_ascii_case(name) && v == value)
    }

    fn find_name_match(&self, name: &str) -> Option<usize> {
        STATIC_TABLE.iter().position(|&(n, _)| n.eq_ignore_ascii_case(name))
    }

    pub fn encode_size_update(&mut self, new_size: u32, buf: &mut Vec<u8>) {
        encode_prefix(new_size as u64, 5, buf, |v| 0x20 | v);
    }
}

pub struct HpackDecoder {}

impl HpackDecoder {
    pub fn new() -> Self {
        HpackDecoder {}
    }

    pub fn set_max_table_size(&mut self, _size: u32) {}

    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<(String, String)>, DecodeError> {
        let mut pos = 0;
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
        match first & 0xC0 {
            0xC0 => self.decode_indexed(data),
            0x40 => self.decode_literal_name_ref(data),
            _ => {
                if first & 0xE0 == 0x20 {
                    let (_new_size, consumed) = decode_prefix(data, 5)?;
                    let rest = &data[consumed..];
                    if rest.is_empty() {
                        return Err(DecodeError::Truncated);
                    }
                    let (field, fc) = self.decode_field(rest)?;
                    Ok((field, consumed + fc))
                } else {
                    self.decode_literal_new_name(data)
                }
            }
        }
    }

    fn decode_indexed(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let (idx, consumed) = decode_prefix(data, 6)?;
        let tbl_idx = idx as usize;
        if tbl_idx >= STATIC_TABLE.len() {
            return Ok(((format!("unknown-{tbl_idx}"), String::new()), consumed));
        }
        let (name, value) = STATIC_TABLE[tbl_idx];
        Ok(((name.to_string(), value.to_string()), consumed))
    }

    fn decode_literal_name_ref(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let (idx, consumed) = decode_prefix(data, 4)?;
        let tbl_idx = idx as usize;
        let name = if tbl_idx < STATIC_TABLE.len() {
            STATIC_TABLE[tbl_idx].0.to_string()
        } else {
            format!("unknown-{tbl_idx}")
        };
        let (value, vc) = self.decode_value_str(&data[consumed..])?;
        Ok(((name, value), consumed + vc))
    }

    fn decode_literal_new_name(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let first = *data.first().ok_or(DecodeError::Truncated)?;
        let name_huffman = (first & 0x08) != 0;
        let (name_len, consumed) = decode_prefix(data, 3)?;
        let name_start = consumed;
        let name_end = name_start + name_len as usize;
        if data.len() < name_end {
            return Err(DecodeError::Truncated);
        }
        let name_raw = &data[name_start..name_end];
        let name = if name_huffman {
            huffman_decode(name_raw).unwrap_or_else(|| String::from_utf8_lossy(name_raw).into_owned())
        } else {
            String::from_utf8_lossy(name_raw).into_owned()
        };
        let (value, vc) = self.decode_value_str(&data[name_end..])?;
        Ok(((name, value), name_end + vc))
    }

    fn decode_value_str(&self, data: &[u8]) -> Result<(String, usize), DecodeError> {
        let first = *data.first().ok_or(DecodeError::Truncated)?;
        let huffman = (first & 0x80) != 0;
        let (len, consumed) = decode_prefix(data, 7)?;
        let start = consumed;
        let end = start + len as usize;
        if data.len() < end {
            return Err(DecodeError::Truncated);
        }
        let raw = &data[start..end];
        let value = if huffman {
            huffman_decode(raw).unwrap_or_else(|| String::from_utf8_lossy(raw).into_owned())
        } else {
            String::from_utf8_lossy(raw).into_owned()
        };
        Ok((value, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(headers: &[(&str, &str)]) {
        let mut enc = HpackEncoder::new();
        let mut dec = HpackDecoder::new();
        let encoded = enc.encode(headers);
        let decoded = dec.decode(&encoded).unwrap();
        let expected: Vec<(String, String)> = headers.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_indexed_header() {
        roundtrip(&[(":method", "GET")]);
    }

    #[test]
    fn test_indexed_status() {
        roundtrip(&[(":status", "200")]);
    }

    #[test]
    fn test_roundtrip_simple() {
        roundtrip(&[(":method", "GET"), (":path", "/"), (":scheme", "https")]);
    }

    #[test]
    fn test_roundtrip_literal_name_ref() {
        roundtrip(&[(":authority", "example.com")]);
    }

    #[test]
    fn test_roundtrip_literal_new_name() {
        roundtrip(&[("x-custom", "custom-value")]);
    }

    #[test]
    fn test_roundtrip_empty_value() {
        roundtrip(&[("accept-language", "")]);
    }

    #[test]
    fn test_roundtrip_multibyte() {
        roundtrip(&[("x-unicode", "héllo wörld")]);
    }

    #[test]
    fn test_roundtrip_empty() {
        let mut enc = HpackEncoder::new();
        let mut dec = HpackDecoder::new();
        let input: &[(&str, &str)] = &[];
        let encoded = enc.encode(input);
        assert!(encoded.is_empty());
        let decoded = dec.decode(&encoded).unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_multiple_headers() {
        roundtrip(&[
            (":method", "GET"),
            (":scheme", "https"),
            (":authority", "example.com"),
            (":path", "/"),
            ("user-agent", "test"),
            ("accept", "*/*"),
        ]);
    }

    #[test]
    fn test_long_header_value() {
        let long_value = "a".repeat(500);
        roundtrip(&[("x-long", long_value.as_str())]);
    }

    #[test]
    fn test_decoder_indexed() {
        let mut dec = HpackDecoder::new();
        let encoded = vec![0xC0 | 17]; // index 17 = :method GET
        let decoded = dec.decode(&encoded).unwrap();
        assert_eq!(decoded, vec![(":method".to_string(), "GET".to_string())]);
    }

    #[test]
    fn test_decode_truncated() {
        let mut enc = HpackEncoder::new();
        let encoded = enc.encode(&[("x-test", "value")]);
        let truncated = &encoded[..encoded.len() - 1];
        let mut dec = HpackDecoder::new();
        assert!(dec.decode(truncated).is_err());
    }

    #[test]
    fn test_table_size_update() {
        let mut enc = HpackEncoder::new();
        let mut buf = Vec::new();
        enc.encode_size_update(2048, &mut buf);
        assert!(!buf.is_empty());
    }
}
