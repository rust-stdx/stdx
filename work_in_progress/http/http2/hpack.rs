use crate::common::hpack_qpack_shared::{
    DecodeError, decode_prefix, encode_prefix, encode_raw, huffman_decode, huffman_encode,
};

// ── RFC 7541 static table (61 entries, 1-based on the wire) ─────────────

/// Position 0 = HPACK index 1, position 1 = HPACK index 2, etc.
const HPACK_STATIC: &[(&str, &str)] = &[
    (":authority", ""),
    (":method", "GET"),
    (":method", "POST"),
    (":path", "/"),
    (":path", "/index.html"),
    (":scheme", "http"),
    (":scheme", "https"),
    (":status", "200"),
    (":status", "204"),
    (":status", "206"),
    (":status", "304"),
    (":status", "400"),
    (":status", "404"),
    (":status", "500"),
    ("accept-charset", ""),
    ("accept-encoding", "gzip, deflate"),
    ("accept-language", ""),
    ("accept-ranges", ""),
    ("accept", ""),
    ("access-control-allow-origin", ""),
    ("age", ""),
    ("allow", ""),
    ("authorization", ""),
    ("cache-control", ""),
    ("content-disposition", ""),
    ("content-encoding", ""),
    ("content-language", ""),
    ("content-length", ""),
    ("content-location", ""),
    ("content-range", ""),
    ("content-type", ""),
    ("cookie", ""),
    ("date", ""),
    ("etag", ""),
    ("expect", ""),
    ("expires", ""),
    ("from", ""),
    ("host", ""),
    ("if-match", ""),
    ("if-modified-since", ""),
    ("if-none-match", ""),
    ("if-range", ""),
    ("if-unmodified-since", ""),
    ("last-modified", ""),
    ("link", ""),
    ("location", ""),
    ("max-forwards", ""),
    ("proxy-authenticate", ""),
    ("proxy-authorization", ""),
    ("range", ""),
    ("referer", ""),
    ("refresh", ""),
    ("retry-after", ""),
    ("server", ""),
    ("set-cookie", ""),
    ("strict-transport-security", ""),
    ("transfer-encoding", ""),
    ("user-agent", ""),
    ("vary", ""),
    ("via", ""),
    ("www-authenticate", ""),
];

// ── Encoder ──────────────────────────────────────────────────────────────

pub struct HpackEncoder {
    max_table_size: u32,
}

impl HpackEncoder {
    pub fn new() -> Self {
        HpackEncoder {
            max_table_size: 4096,
        }
    }

    /// Set the maximum dynamic table size. The encoder may emit a
    /// table-size-update signal when encoding the next header block.
    pub fn set_max_table_size(&mut self, size: u32) {
        self.max_table_size = size;
    }

    /// Encode a set of header fields into an HPACK header block.
    ///
    /// Uses the RFC 7541 static table for full and name-only matches.
    /// Dynamic table entries are not created (simplified encoder).
    pub fn encode(&mut self, headers: &[(impl AsRef<str>, impl AsRef<str>)]) -> Vec<u8> {
        let mut buf = Vec::new();
        for (name, value) in headers {
            self.encode_field(name.as_ref(), value.as_ref(), &mut buf);
        }
        buf
    }

    fn encode_field(&mut self, name: &str, value: &str, out: &mut Vec<u8>) {
        // 1) Indexed Header Field (Section 6.1): full match in static table
        if let Some(pos) = HPACK_STATIC
            .iter()
            .position(|&(n, v)| n.eq_ignore_ascii_case(name) && v == value)
        {
            encode_prefix((pos + 1) as u64, 7, out, |v| 0x80 | v);
            return;
        }

        // 2) Literal with Incremental Indexing, indexed name (Section 6.2.1)
        if let Some(pos) = HPACK_STATIC.iter().position(|&(n, _)| n.eq_ignore_ascii_case(name)) {
            encode_prefix((pos + 1) as u64, 6, out, |v| 0x40 | v);
            encode_str(value, out);
            return;
        }

        // 3) Literal without Indexing, new name (Section 6.2.2, index = 0)
        out.push(0x00);
        encode_str(name, out);
        encode_str(value, out);
    }

    /// Encode a dynamic-table-size-update into `out` (Section 6.3).
    pub fn encode_size_update(&mut self, new_size: u32, out: &mut Vec<u8>) {
        encode_prefix(new_size as u64, 5, out, |v| 0x20 | v);
    }
}

/// Encode a string literal with a 7-bit prefix and optional Huffman coding.
fn encode_str(s: &str, out: &mut Vec<u8>) {
    let raw = s.as_bytes();
    let huf = huffman_encode(raw);
    if huf.len() < raw.len() {
        encode_prefix(huf.len() as u64, 7, out, |v| 0x80 | v);
        encode_raw(&huf, out);
    } else {
        encode_prefix(raw.len() as u64, 7, out, |v| v & 0x7F);
        encode_raw(raw, out);
    }
}

/// Decode a string literal (7-bit prefix, optional Huffman).
fn decode_str(data: &[u8]) -> Result<(String, usize), DecodeError> {
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

// ── Decoder ──────────────────────────────────────────────────────────────

pub struct HpackDecoder {
    max_table_size: u32,
}

impl HpackDecoder {
    pub fn new() -> Self {
        HpackDecoder {
            max_table_size: 4096,
        }
    }

    /// Set the maximum dynamic table size the decoder will accept.
    pub fn set_max_table_size(&mut self, size: u32) {
        self.max_table_size = size;
    }

    /// Decode a complete HPACK header block. Returns the decoded
    /// `(name, value)` pairs in order.
    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<(String, String)>, DecodeError> {
        let mut headers = Vec::new();
        let mut pos = 0;
        while pos < data.len() {
            let (field, consumed) = self.decode_field(&data[pos..])?;
            headers.push(field);
            pos += consumed;
        }
        Ok(headers)
    }

    fn decode_field(&mut self, data: &[u8]) -> Result<((String, String), usize), DecodeError> {
        let first = *data.first().ok_or(DecodeError::Truncated)?;

        if first & 0x80 != 0 {
            // Indexed Header Field (Section 6.1): 7-bit prefix, 1-based index
            let (idx, consumed) = decode_prefix(data, 7)?;
            if idx == 0 {
                return Err(DecodeError::BadPrefix);
            }
            let pos0 = (idx - 1) as usize;
            if pos0 >= HPACK_STATIC.len() {
                return Err(DecodeError::BadPrefix);
            }
            let (name, value) = HPACK_STATIC[pos0];
            Ok(((name.to_string(), value.to_string()), consumed))
        } else if first & 0x40 != 0 {
            // Literal with Incremental Indexing (Section 6.2.1): 6-bit prefix
            let (idx, consumed) = decode_prefix(data, 6)?;
            let (name, tail) = if idx == 0 {
                // new name
                let (n, nc) = decode_str(&data[consumed..])?;
                (n, consumed + nc)
            } else {
                let pos0 = (idx - 1) as usize;
                if pos0 >= HPACK_STATIC.len() {
                    return Err(DecodeError::BadPrefix);
                }
                (HPACK_STATIC[pos0].0.to_string(), consumed)
            };
            let (value, vc) = decode_str(&data[tail..])?;
            Ok(((name, value), tail + vc))
        } else if first & 0x20 != 0 {
            // Dynamic Table Size Update (Section 6.3): 5-bit prefix
            let (size, consumed) = decode_prefix(data, 5)?;
            self.max_table_size = size as u32;
            // The update consumes bytes but produces no header. Advance and
            // re-parse what follows: RFC allows zero or more updates followed
            // by the next field in the same byte stream.
            if consumed >= data.len() {
                return Err(DecodeError::Truncated);
            }
            let (field, fc) = self.decode_field(&data[consumed..])?;
            Ok((field, consumed + fc))
        } else {
            // "Without Indexing" (Section 6.2.2) or "Never Indexed" (6.2.3):
            // both have a 4-bit prefix index.
            let (idx, consumed) = decode_prefix(data, 4)?;
            let (name, tail) = if idx == 0 {
                // new name
                let (n, nc) = decode_str(&data[consumed..])?;
                (n, consumed + nc)
            } else {
                let pos0 = (idx - 1) as usize;
                if pos0 >= HPACK_STATIC.len() {
                    return Err(DecodeError::BadPrefix);
                }
                (HPACK_STATIC[pos0].0.to_string(), consumed)
            };
            let (value, vc) = decode_str(&data[tail..])?;
            Ok(((name, value), tail + vc))
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(headers: &[(&str, &str)]) {
        let mut enc = HpackEncoder::new();
        let mut dec = HpackDecoder::new();
        let encoded = enc.encode(headers);
        let decoded = dec.decode(&encoded).unwrap();
        let expected: Vec<(String, String)> = headers.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        assert_eq!(decoded, expected, "roundtrip failed: {headers:?}");
    }

    // ── encode-then-decode roundtrips ───────────────────────────────

    #[test]
    fn rt_indexed_method() {
        roundtrip(&[(":method", "GET")]);
    }

    #[test]
    fn rt_indexed_status() {
        roundtrip(&[(":status", "200")]);
    }

    #[test]
    fn rt_indexed_scheme() {
        roundtrip(&[(":scheme", "https")]);
    }

    #[test]
    fn rt_indexed_path() {
        roundtrip(&[(":path", "/")]);
    }

    #[test]
    fn rt_name_ref_authority() {
        roundtrip(&[(":authority", "example.com")]);
    }

    #[test]
    fn rt_literal_new_name() {
        roundtrip(&[("x-custom", "custom-value")]);
    }

    #[test]
    fn rt_regular_headers() {
        roundtrip(&[
            ("user-agent", "curl/8.0"),
            ("accept", "*/*"),
            ("content-type", "text/html"),
        ]);
    }

    #[test]
    fn rt_empty_value() {
        roundtrip(&[("accept-language", "")]);
    }

    #[test]
    fn rt_multibyte() {
        roundtrip(&[("x-unicode", "héllo wörld ñ")]);
    }

    #[test]
    fn rt_empty_header_list() {
        let mut enc = HpackEncoder::new();
        let mut dec = HpackDecoder::new();
        let input: &[(&str, &str)] = &[];
        let encoded = enc.encode(input);
        assert!(encoded.is_empty());
        let decoded = dec.decode(&encoded).unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn rt_full_request() {
        roundtrip(&[
            (":method", "GET"),
            (":scheme", "https"),
            (":authority", "example.com"),
            (":path", "/"),
            ("user-agent", "stdx-h2/0.1"),
            ("accept", "*/*"),
        ]);
    }

    #[test]
    fn rt_long_value() {
        let v = "a".repeat(500);
        roundtrip(&[("x-long", v.as_str())]);
    }

    // ── fixed-wire tests (RFC 7541 examples) ────────────────────────

    #[test]
    fn wire_indexed_method_get() {
        // RFC 7541: :method GET is static index 2, encoded as 0x82
        let mut dec = HpackDecoder::new();
        let headers = dec.decode(&[0x82]).unwrap();
        assert_eq!(headers, vec![(":method".to_string(), "GET".to_string())]);
    }

    #[test]
    fn wire_indexed_path_root() {
        // :path / is static index 4, encoded as 0x84
        let mut dec = HpackDecoder::new();
        let headers = dec.decode(&[0x84]).unwrap();
        assert_eq!(headers, vec![(":path".to_string(), "/".to_string())]);
    }

    #[test]
    fn wire_indexed_scheme_https() {
        // :scheme https is static index 7, encoded as 0x87
        let mut dec = HpackDecoder::new();
        let headers = dec.decode(&[0x87]).unwrap();
        assert_eq!(headers, vec![(":scheme".to_string(), "https".to_string())]);
    }

    #[test]
    fn wire_indexed_status_200() {
        // :status 200 is static index 8, encoded as 0x88
        let mut dec = HpackDecoder::new();
        let headers = dec.decode(&[0x88]).unwrap();
        assert_eq!(headers, vec![(":status".to_string(), "200".to_string())]);
    }

    #[test]
    fn wire_indexed_status_404() {
        // :status 404 is static index 13, encoded as 0x8d
        let mut dec = HpackDecoder::new();
        let headers = dec.decode(&[0x8d]).unwrap();
        assert_eq!(headers, vec![(":status".to_string(), "404".to_string())]);
    }

    #[test]
    fn wire_literal_with_name_ref() {
        // RFC 7541 Section 6.2.1 example: authority index 1, value "www.example.com"
        // 0x40 | 1 = 0x41, then value "www.example.com" (15 bytes, plain)
        let mut wire = vec![0x41, 0x0f];
        wire.extend_from_slice(b"www.example.com");
        let mut dec = HpackDecoder::new();
        let headers = dec.decode(&wire).unwrap();
        assert_eq!(headers, vec![(":authority".to_string(), "www.example.com".to_string())]);
    }

    #[test]
    fn wire_size_update_followed_by_indexed() {
        // Dynamic table size update to 512 (5-bit prefix):
        //   512 >= 31, first byte = 0x20 | 31 = 0x3F
        //   512-31=481 = 3*128+97, so 0xE1 (97|0x80), then 0x03
        //   Then indexed :status 200 at 0x88
        let wire = vec![0x3f, 0xE1, 0x03, 0x88];
        let mut dec = HpackDecoder::new();
        let headers = dec.decode(&wire).unwrap();
        assert_eq!(headers, vec![(":status".to_string(), "200".to_string())]);
    }

    #[test]
    fn wire_decode_truncated() {
        let mut dec = HpackDecoder::new();
        assert!(dec.decode(&[0x80]).is_err()); // truncated 7-bit varint
    }

    // ── prefix tests ────────────────────────────────────────────────

    #[test]
    fn prefix_roundtrip_7bit() {
        let mut buf = Vec::new();
        for val in [0u64, 1, 100, 126, 127, 128, 500, 1000, u32::MAX as u64] {
            buf.clear();
            encode_prefix(val, 7, &mut buf, |v| 0x80 | v);
            let (decoded, consumed) = decode_prefix(&buf, 7).unwrap();
            assert_eq!(decoded, val, "7-bit roundtrip failed for {val}");
            assert_eq!(consumed, buf.len());
        }
    }

    #[test]
    fn prefix_roundtrip_6bit() {
        let mut buf = Vec::new();
        for val in [0u64, 1, 62, 63, 64, 500] {
            buf.clear();
            encode_prefix(val, 6, &mut buf, |v| 0x40 | v);
            let (decoded, consumed) = decode_prefix(&buf, 6).unwrap();
            assert_eq!(decoded, val, "6-bit roundtrip failed for {val}");
            assert_eq!(consumed, buf.len());
        }
    }

    #[test]
    fn prefix_roundtrip_5bit() {
        let mut buf = Vec::new();
        for val in [0u64, 1, 30, 31, 32, 500] {
            buf.clear();
            encode_prefix(val, 5, &mut buf, |v| 0x20 | v);
            let (decoded, consumed) = decode_prefix(&buf, 5).unwrap();
            assert_eq!(decoded, val, "5-bit roundtrip failed for {val}");
            assert_eq!(consumed, buf.len());
        }
    }

    #[test]
    fn prefix_roundtrip_4bit() {
        let mut buf = Vec::new();
        for val in [0u64, 1, 14, 15, 16, 100] {
            buf.clear();
            encode_prefix(val, 4, &mut buf, |v| 0x10 | v);
            let (decoded, consumed) = decode_prefix(&buf, 4).unwrap();
            assert_eq!(decoded, val, "4-bit roundtrip failed for {val}");
            assert_eq!(consumed, buf.len());
        }
    }

    // ── size update ─────────────────────────────────────────────────

    #[test]
    fn encode_size_update_small() {
        let mut enc = HpackEncoder::new();
        let mut buf = Vec::new();
        enc.encode_size_update(100, &mut buf);
        assert!(!buf.is_empty());
        // 100 in 5-bit prefix: 100 < 31, so single byte: 0x20 | 100 = 0x20 | 0x64 = 0x84... wait
        // Actually 100 = 0x64, 5-bit mask = 31 = 0x1F, 0x64 & 0x1F = 0x04, 0x20 | 0x04 = 0x24
        // Wait no: 100 in binary is 01100100. Lower 5 bits: 00100 = 4. 0x20 | 4 = 0x24.
        // But 100 >= 31, so 100 - 31 = 69. First byte: 0x20 | 31 = 0x3F.
        // 69 = 0x45, 69 < 128, so second byte: 0x45.
        assert_eq!(buf, vec![0x3F, 0x45]);
    }

    #[test]
    fn encode_size_update_zero() {
        let mut enc = HpackEncoder::new();
        let mut buf = Vec::new();
        enc.encode_size_update(0, &mut buf);
        assert_eq!(buf, vec![0x00 | 0x20]);
    }
}
