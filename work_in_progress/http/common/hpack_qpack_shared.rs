pub fn encode_prefix(value: u64, prefix_bits: u32, out: &mut Vec<u8>, set_prefix: impl FnOnce(u8) -> u8) {
    let max_prefix = (1u64 << prefix_bits) - 1;
    if value < max_prefix {
        out.push(set_prefix(value as u8));
    } else {
        out.push(set_prefix(max_prefix as u8));
        let mut remaining = value - max_prefix;
        while remaining >= 128 {
            out.push((remaining & 0x7f) as u8 | 0x80);
            remaining >>= 7;
        }
        out.push(remaining as u8);
    }
}

pub fn decode_prefix(data: &[u8], prefix_bits: u32) -> Result<(u64, usize), DecodeError> {
    let first = *data.first().ok_or(DecodeError::Truncated)?;
    let max_prefix = (1u64 << prefix_bits) - 1;
    let mask = max_prefix as u8;
    let prefix_val = (first & mask) as u64;
    if prefix_val < max_prefix {
        return Ok((prefix_val, 1));
    }
    let mut value = max_prefix;
    let mut shift = 0u32;
    let mut pos = 1;
    loop {
        if pos >= data.len() {
            return Err(DecodeError::Truncated);
        }
        let byte = data[pos];
        pos += 1;
        value += ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok((value, pos))
}

pub fn encode_raw(data: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(data);
}

pub fn encode_value(value: &str, out: &mut Vec<u8>) {
    let bytes = value.as_bytes();
    let huffman_encoded = huffman_encode(bytes);
    if huffman_encoded.len() < bytes.len() {
        encode_prefix(huffman_encoded.len() as u64, 7, out, |v| 0x80 | v);
        encode_raw(&huffman_encoded, out);
    } else {
        encode_prefix(bytes.len() as u64, 7, out, |v| v & 0x7f);
        encode_raw(bytes, out);
    }
}

pub fn decode_value(data: &[u8]) -> Result<(String, usize), DecodeError> {
    let first = *data.first().ok_or(DecodeError::Truncated)?;
    let _huffman = (first & 0x80) != 0;
    let (len, consumed) = decode_prefix(data, 7)?;
    let start = consumed;
    let end = start + len as usize;
    if data.len() < end {
        return Err(DecodeError::Truncated);
    }
    let raw = &data[start..end];
    let value = if _huffman {
        huffman_decode(raw).unwrap_or_else(|| String::from_utf8_lossy(raw).into_owned())
    } else {
        String::from_utf8_lossy(raw).into_owned()
    };
    Ok((value, end))
}

pub fn huffman_encode(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut bits: u64 = 0;
    let mut bits_available: u32 = 0;
    for &byte in data {
        let (code, code_len) = HUFFMAN[byte as usize];
        bits = (bits << code_len) | (code as u64);
        bits_available += code_len as u32;
        while bits_available >= 8 {
            bits_available -= 8;
            result.push(((bits >> bits_available) & 0xFF) as u8);
        }
    }
    if bits_available > 0 {
        let (_eos_code, _eos_len) = HUFFMAN[256];
        let pad_needed = 8 - bits_available;
        bits = (bits << pad_needed) | ((_eos_code as u64) >> (_eos_len as u32 - pad_needed));
        result.push(bits as u8);
    }
    result
}

pub fn get_huffman_tree() -> &'static [(u16, u16); 512] {
    use std::sync::OnceLock;
    static TREE: OnceLock<Box<[(u16, u16); 512]>> = OnceLock::new();
    TREE.get_or_init(|| {
        let mut tree = [(0u16, 0u16); 512];
        let mut next_free: u16 = 1;

        for sym in 0u16..257u16 {
            let (code, code_len) = HUFFMAN[sym as usize];
            let mut node: u16 = 0;
            for i in (0..code_len).rev() {
                let bit = ((code >> i) & 1) as usize;
                let is_last = i == 0;
                let child_val = if bit == 0 {
                    tree[node as usize].0
                } else {
                    tree[node as usize].1
                };
                if is_last {
                    let leaf = sym + 512;
                    if bit == 0 {
                        tree[node as usize].0 = leaf;
                    } else {
                        tree[node as usize].1 = leaf;
                    }
                    break;
                } else {
                    let next = if child_val == 0 {
                        let new_node = next_free;
                        next_free += 1;
                        if bit == 0 {
                            tree[node as usize].0 = new_node;
                        } else {
                            tree[node as usize].1 = new_node;
                        }
                        new_node
                    } else {
                        child_val
                    };
                    node = next;
                }
            }
        }
        Box::new(tree)
    })
}

pub fn huffman_decode(data: &[u8]) -> Option<String> {
    let tree = get_huffman_tree();
    let mut result = Vec::new();
    let mut bits: u128 = 0;
    let mut bits_available: u32 = 0;

    for &byte in data {
        bits = (bits << 8) | (byte as u128);
        bits_available += 8;
        if bits_available > 64 {
            bits_available = 64;
        }

        loop {
            let mut node: u16 = 0;
            let mut matched: Option<(u16, u8)> = None;
            for shift in (0..bits_available.min(30)).rev() {
                let bit = ((bits >> shift) & 1) as usize;
                let child = if bit == 0 {
                    tree[node as usize].0
                } else {
                    tree[node as usize].1
                };
                if child >= 512 {
                    matched = Some((child - 512, (bits_available - shift as u32) as u8));
                    break;
                }
                if child == 0 {
                    break;
                }
                node = child;
            }
            match matched {
                Some((sym, code_len)) if sym == 256 => {
                    let remaining = bits_available - code_len as u32;
                    if remaining > 0 {
                        let mask = (1u128 << remaining) - 1;
                        if (bits & mask) != mask {
                            return None;
                        }
                    }
                    bits_available = 0;
                }
                Some((sym, code_len)) => {
                    result.push(sym as u8);
                    bits_available -= code_len as u32;
                }
                None => break,
            }
        }
    }
    if bits_available >= 8 {
        return None;
    }
    if bits_available > 0 {
        let mask = (1u128 << bits_available) - 1;
        if (bits & mask) != mask {
            return None;
        }
    }
    String::from_utf8(result).ok()
}

#[derive(Debug)]
pub enum DecodeError {
    Truncated,
    BadPrefix,
}

pub const STATIC_TABLE: &[(&str, &str)] = &[
    (":authority", ""),
    (":path", "/"),
    ("age", "0"),
    ("content-disposition", ""),
    ("content-length", "0"),
    ("cookie", ""),
    ("date", ""),
    ("etag", ""),
    ("if-modified-since", ""),
    ("if-none-match", ""),
    ("last-modified", ""),
    ("link", ""),
    ("location", ""),
    ("referer", ""),
    ("set-cookie", ""),
    (":method", "CONNECT"),
    (":method", "DELETE"),
    (":method", "GET"),
    (":method", "HEAD"),
    (":method", "OPTIONS"),
    (":method", "POST"),
    (":method", "PUT"),
    (":scheme", "http"),
    (":scheme", "https"),
    (":status", "103"),
    (":status", "200"),
    (":status", "304"),
    (":status", "404"),
    (":status", "503"),
    ("accept", "*/*"),
    ("accept", "application/dns-message"),
    ("accept-encoding", "gzip, deflate, br"),
    ("accept-ranges", "bytes"),
    ("access-control-allow-headers", "cache-control"),
    ("access-control-allow-headers", "content-type"),
    ("access-control-allow-origin", "*"),
    ("cache-control", "max-age=0"),
    ("cache-control", "max-age=2592000"),
    ("cache-control", "max-age=604800"),
    ("cache-control", "no-cache"),
    ("cache-control", "no-store"),
    ("cache-control", "public, max-age=31536000"),
    ("content-disposition", "attachment"),
    ("content-encoding", "br"),
    ("content-encoding", "gzip"),
    ("content-type", "application/dns-message"),
    ("content-type", "application/javascript"),
    ("content-type", "application/json"),
    ("content-type", "application/x-www-form-urlencoded"),
    ("content-type", "image/gif"),
    ("content-type", "image/jpeg"),
    ("content-type", "image/png"),
    ("content-type", "text/css"),
    ("content-type", "text/html; charset=utf-8"),
    ("content-type", "text/plain"),
    ("content-type", "text/plain;charset=utf-8"),
    ("range", "bytes=0-"),
    ("strict-transport-security", "max-age=31536000"),
    ("strict-transport-security", "max-age=31536000; includesubdomains"),
    ("strict-transport-security", "max-age=31536000; includesubdomains; preload"),
    ("vary", "accept-encoding"),
    ("vary", "origin"),
    ("x-content-type-options", "nosiff"),
    ("x-xss-protection", "1; mode=block"),
    (":status", "100"),
    (":status", "204"),
    (":status", "206"),
    (":status", "302"),
    (":status", "400"),
    (":status", "401"),
    (":status", "403"),
    (":status", "421"),
    (":status", "425"),
    (":status", "500"),
    ("accept-language", ""),
    ("access-control-allow-credentials", "FALSE"),
    ("access-control-allow-credentials", "TRUE"),
    ("access-control-allow-headers", "*"),
    ("access-control-allow-methods", "get"),
    ("access-control-allow-methods", "get, post, options"),
    ("access-control-allow-methods", "options"),
    ("access-control-expose-headers", "content-length"),
    ("access-control-request-headers", "content-type"),
    ("access-control-request-method", "get"),
    ("access-control-request-method", "post"),
    ("alt-svc", "clear"),
    ("authorization", ""),
    (
        "content-security-policy",
        "script-src 'none'; object-src 'none'; base-uri 'none'",
    ),
    ("early-data", "1"),
    ("expect-ct", ""),
    ("forwarded", ""),
    ("if-range", ""),
    ("origin", ""),
    ("purpose", "prefetch"),
    ("server", ""),
    ("timing-allow-origin", "*"),
    ("upgrade-insecure-requests", "1"),
    ("user-agent", ""),
    ("x-forwarded-for", ""),
    ("x-frame-options", "deny"),
    ("x-frame-options", "sameorigin"),
];

const HUFFMAN: [(u64, u8); 257] = [
    (0x1ff8, 13),
    (0x7fffd8, 23),
    (0xfffffe2, 28),
    (0xfffffe3, 28),
    (0xfffffe4, 28),
    (0xfffffe5, 28),
    (0xfffffe6, 28),
    (0xfffffe7, 28),
    (0xfffffe8, 28),
    (0xffffea, 24),
    (0x3ffffffc, 30),
    (0xfffffe9, 28),
    (0xfffffea, 28),
    (0x3ffffffd, 30),
    (0xfffffeb, 28),
    (0xfffffec, 28),
    (0xfffffed, 28),
    (0xfffffee, 28),
    (0xfffffef, 28),
    (0xffffff0, 28),
    (0xffffff1, 28),
    (0xffffff2, 28),
    (0x3ffffffe, 30),
    (0xffffff3, 28),
    (0xffffff4, 28),
    (0xffffff5, 28),
    (0xffffff6, 28),
    (0xffffff7, 28),
    (0xffffff8, 28),
    (0xffffff9, 28),
    (0xffffffa, 28),
    (0xffffffb, 28),
    (0x14, 6),
    (0x3f8, 10),
    (0x3f9, 10),
    (0xffa, 12),
    (0x1ff9, 13),
    (0x15, 6),
    (0xf8, 8),
    (0x7fa, 11),
    (0x3fa, 10),
    (0x3fb, 10),
    (0xf9, 8),
    (0x7fb, 11),
    (0xfa, 8),
    (0x16, 6),
    (0x17, 6),
    (0x18, 6),
    (0x0, 5),
    (0x1, 5),
    (0x2, 5),
    (0x19, 6),
    (0x1a, 6),
    (0x1b, 6),
    (0x1c, 6),
    (0x1d, 6),
    (0x1e, 6),
    (0x1f, 6),
    (0x5c, 7),
    (0xfb, 8),
    (0x7ffc, 15),
    (0x20, 6),
    (0xffb, 12),
    (0x3fc, 10),
    (0x1ffa, 13),
    (0x21, 6),
    (0x5d, 7),
    (0x5e, 7),
    (0x5f, 7),
    (0x60, 7),
    (0x61, 7),
    (0x62, 7),
    (0x63, 7),
    (0x64, 7),
    (0x65, 7),
    (0x66, 7),
    (0x67, 7),
    (0x68, 7),
    (0x69, 7),
    (0x6a, 7),
    (0x6b, 7),
    (0x6c, 7),
    (0x6d, 7),
    (0x6e, 7),
    (0x6f, 7),
    (0x70, 7),
    (0x71, 7),
    (0x72, 7),
    (0xfc, 8),
    (0x73, 7),
    (0xfd, 8),
    (0x1ffb, 13),
    (0x7fff0, 19),
    (0x1ffc, 13),
    (0x3ffc, 14),
    (0x22, 6),
    (0x7ffd, 15),
    (0x3, 5),
    (0x23, 6),
    (0x4, 5),
    (0x24, 6),
    (0x5, 5),
    (0x25, 6),
    (0x26, 6),
    (0x27, 6),
    (0x6, 5),
    (0x74, 7),
    (0x75, 7),
    (0x28, 6),
    (0x29, 6),
    (0x2a, 6),
    (0x7, 5),
    (0x2b, 6),
    (0x76, 7),
    (0x2c, 6),
    (0x8, 5),
    (0x9, 5),
    (0x2d, 6),
    (0x77, 7),
    (0x78, 7),
    (0x79, 7),
    (0x7a, 7),
    (0x7b, 7),
    (0x7ffe, 15),
    (0x7fc, 11),
    (0x3ffd, 14),
    (0x1ffd, 13),
    (0xffffffc, 28),
    (0xfffe6, 20),
    (0x3fffd2, 22),
    (0xfffe7, 20),
    (0xfffe8, 20),
    (0x3fffd3, 22),
    (0x3fffd4, 22),
    (0x3fffd5, 22),
    (0x7fffd9, 23),
    (0x3fffd6, 22),
    (0x7fffda, 23),
    (0x7fffdb, 23),
    (0x7fffdc, 23),
    (0x7fffdd, 23),
    (0x7fffde, 23),
    (0xffffeb, 24),
    (0x7fffdf, 23),
    (0xffffec, 24),
    (0xffffed, 24),
    (0x3fffd7, 22),
    (0x7fffe0, 23),
    (0xffffee, 24),
    (0x7fffe1, 23),
    (0x7fffe2, 23),
    (0x7fffe3, 23),
    (0x7fffe4, 23),
    (0x1fffdc, 21),
    (0x3fffd8, 22),
    (0x7fffe5, 23),
    (0x3fffd9, 22),
    (0x7fffe6, 23),
    (0x7fffe7, 23),
    (0xffffef, 24),
    (0x3fffda, 22),
    (0x1fffdd, 21),
    (0xfffe9, 20),
    (0x3fffdb, 22),
    (0x3fffdc, 22),
    (0x7fffe8, 23),
    (0x7fffe9, 23),
    (0x1fffde, 21),
    (0x7fffea, 23),
    (0x3fffdd, 22),
    (0x3fffde, 22),
    (0xfffff0, 24),
    (0x1fffdf, 21),
    (0x3fffdf, 22),
    (0x7fffeb, 23),
    (0x7fffec, 23),
    (0x1fffe0, 21),
    (0x1fffe1, 21),
    (0x3fffe0, 22),
    (0x1fffe2, 21),
    (0x7fffed, 23),
    (0x3fffe1, 22),
    (0x7fffee, 23),
    (0x7fffef, 23),
    (0xfffea, 20),
    (0x3fffe2, 22),
    (0x3fffe3, 22),
    (0x3fffe4, 22),
    (0x7ffff0, 23),
    (0x3fffe5, 22),
    (0x3fffe6, 22),
    (0x7ffff1, 23),
    (0x3ffffe0, 26),
    (0x3ffffe1, 26),
    (0xfffeb, 20),
    (0x7fff1, 19),
    (0x3fffe7, 22),
    (0x7ffff2, 23),
    (0x3fffe8, 22),
    (0x1ffffec, 25),
    (0x3ffffe2, 26),
    (0x3ffffe3, 26),
    (0x3ffffe4, 26),
    (0x7ffffde, 27),
    (0x7ffffdf, 27),
    (0x3ffffe5, 26),
    (0xfffff1, 24),
    (0x1ffffed, 25),
    (0x7fff2, 19),
    (0x1fffe3, 21),
    (0x3ffffe6, 26),
    (0x7ffffe0, 27),
    (0x7ffffe1, 27),
    (0x3ffffe7, 26),
    (0x7ffffe2, 27),
    (0xfffff2, 24),
    (0x1fffe4, 21),
    (0x1fffe5, 21),
    (0x3ffffe8, 26),
    (0x3ffffe9, 26),
    (0xffffffd, 28),
    (0x7ffffe3, 27),
    (0x7ffffe4, 27),
    (0x7ffffe5, 27),
    (0xfffec, 20),
    (0xfffff3, 24),
    (0xfffed, 20),
    (0x1fffe6, 21),
    (0x3fffe9, 22),
    (0x1fffe7, 21),
    (0x1fffe8, 21),
    (0x7ffff3, 23),
    (0x3fffea, 22),
    (0x3fffeb, 22),
    (0x1ffffee, 25),
    (0x1ffffef, 25),
    (0xfffff4, 24),
    (0xfffff5, 24),
    (0x3ffffea, 26),
    (0x7ffff4, 23),
    (0x3ffffeb, 26),
    (0x7ffffe6, 27),
    (0x3ffffec, 26),
    (0x3ffffed, 26),
    (0x7ffffe7, 27),
    (0x7ffffe8, 27),
    (0x7ffffe9, 27),
    (0x7ffffea, 27),
    (0x7ffffeb, 27),
    (0xffffffe, 28),
    (0x7ffffec, 27),
    (0x7ffffed, 27),
    (0x7ffffee, 27),
    (0x7ffffef, 27),
    (0x7fffff0, 27),
    (0x3ffffee, 26),
    (0x3fffffff, 30),
];
