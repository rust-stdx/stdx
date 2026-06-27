use crate::error::Error;

/// QUIC variable-length integer (RFC 9000 §16).
///
/// The two most significant bits of the first byte encode the length:
///
/// | Bits | Length | Value range |
/// |------|--------|-------------|
/// | 00   | 1 byte | 0 .. 2⁶-1 |
/// | 01   | 2 bytes | 0 .. 2¹⁴-1 |
/// | 10   | 4 bytes | 0 .. 2³⁰-1 |
/// | 11   | 8 bytes | 0 .. 2⁶²-1 |

/// Return the encoded size (1, 2, 4, or 8) for the given first byte.
pub fn varint_len(first: u8) -> usize {
    match first >> 6 {
        0 => 1,
        1 => 2,
        2 => 4,
        3 => 8,
        _ => unreachable!(),
    }
}

/// Decode a single varint from `buf`.  Returns `(value, bytes_consumed)`.
pub fn decode(buf: &[u8]) -> Result<(u64, usize), Error> {
    let first = *buf.first().ok_or(Error::VarintDecode)?;
    let len = varint_len(first);
    if buf.len() < len {
        return Err(Error::VarintDecode);
    }
    let v: u64 = match len {
        1 => (first & 0x3f) as u64,
        2 => u16::from_be_bytes([buf[0], buf[1]]) as u64 & 0x3fff,
        4 => u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64 & 0x3fffffff,
        8 => u64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]) & 0x3fffffffffffffff,
        _ => unreachable!(),
    };
    Ok((v, len))
}

/// Encode `value` as a varint in minimal form.  Panics if the value is
/// too large (≥ 2⁶²).
pub fn encode(value: u64, buf: &mut Vec<u8>) {
    if value < 0x40 {
        buf.push(value as u8);
    } else if value < 0x4000 {
        let v = 0x4000u16 | value as u16;
        buf.extend_from_slice(&v.to_be_bytes());
    } else if value < 0x40000000 {
        let v = 0x80000000u32 | value as u32;
        buf.extend_from_slice(&v.to_be_bytes());
    } else if value < 0x4000000000000000 {
        let v = 0xc000000000000000u64 | value;
        buf.extend_from_slice(&v.to_be_bytes());
    } else {
        panic!("varint value too large: {value}");
    }
}

/// Length of the encoded varint without writing it.
pub fn encoded_len(value: u64) -> usize {
    if value < 0x40 {
        1
    } else if value < 0x4000 {
        2
    } else if value < 0x40000000 {
        4
    } else {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        for v in [0u64, 1, 63, 64, 16383, 16384, 1073741823, 1073741824, (1 << 62) - 1] {
            let mut buf = Vec::new();
            encode(v, &mut buf);
            let (decoded, _) = decode(&buf).unwrap();
            assert_eq!(decoded, v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn encode_decode_examples() {
        let cases = [
            (0u64, &[0x00u8][..]),
            (1, &[0x01]),
            (63, &[0x3f]),
            (15293, &[0x7b, 0xbd]),
            (494878333, &[0x9d, 0x7f, 0x3e, 0x7d][..]),
        ];
        for (val, expected) in &cases {
            let mut buf = Vec::new();
            encode(*val, &mut buf);
            assert_eq!(&buf, expected, "encode({val})");
            let (decoded, _) = decode(expected).unwrap();
            assert_eq!(decoded, *val, "decode");
        }
    }

    #[test]
    fn buffer_short_returns_error() {
        assert!(decode(&[]).is_err());
        assert!(decode(&[0x40]).is_err());
        assert!(decode(&[0x80, 0x00]).is_err());
    }
}
