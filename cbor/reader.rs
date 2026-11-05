//! Framing a single CBOR item from a buffered reader.

use std::io::{self, BufRead};

/// Read exactly one CBOR item from `reader`, returning its encoded bytes.
///
/// Bytes after the item remain buffered in `reader`. The function enforces a
/// nesting limit to avoid unbounded recursion on hostile input.
pub fn read_item<R: BufRead>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    read_item_into(reader, &mut out)?;
    Ok(out)
}

/// Read exactly one CBOR item from `reader` into `out`, replacing its contents.
///
/// Returns the number of bytes written. Bytes after the item remain buffered in
/// `reader`. Reusing `out` across calls avoids one allocation per item.
///
/// The contents of `out` are unspecified if an error is returned.
pub fn read_item_into<R: BufRead>(reader: &mut R, out: &mut Vec<u8>) -> io::Result<usize> {
    out.clear();
    // Use whatever is already buffered as a cheap capacity hint, but never
    // trust it for the full item size.
    let hint = reader.fill_buf()?.len().min(1024);
    out.reserve(hint);
    copy_item(reader, out, 0)?;
    Ok(out.len())
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn unexpected_eof() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "truncated CBOR item")
}

fn read_byte<R: BufRead>(reader: &mut R, out: &mut Vec<u8>) -> io::Result<u8> {
    let buffer = reader.fill_buf()?;
    let Some(&byte) = buffer.first() else {
        return Err(unexpected_eof());
    };
    reader.consume(1);
    out.push(byte);
    Ok(byte)
}

fn to_usize(len: u64) -> io::Result<usize> {
    usize::try_from(len).map_err(|_| invalid("CBOR length does not fit usize"))
}

fn read_exact_into<R: BufRead>(reader: &mut R, out: &mut Vec<u8>, len: usize) -> io::Result<()> {
    out.reserve(len.min(64 * 1024));
    let mut remaining = len;
    while remaining > 0 {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            return Err(unexpected_eof());
        }
        let take = remaining.min(buffer.len());
        out.extend_from_slice(&buffer[..take]);
        reader.consume(take);
        remaining -= take;
    }
    Ok(())
}

fn read_arg<R: BufRead>(reader: &mut R, out: &mut Vec<u8>, ai: u8) -> io::Result<Option<u64>> {
    let len = match ai {
        0..=23 => return Ok(Some(u64::from(ai))),
        24 => 1,
        25 => 2,
        26 => 4,
        27 => 8,
        28..=30 => return Err(invalid("reserved CBOR additional information")),
        31 => return Ok(None),
        _ => unreachable!(),
    };

    let mut bytes = [0u8; 8];
    reader.read_exact(&mut bytes[..len])?;
    out.extend_from_slice(&bytes[..len]);
    let value = match len {
        1 => u64::from(bytes[0]),
        2 => u64::from(u16::from_be_bytes([bytes[0], bytes[1]])),
        4 => u64::from(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
        8 => u64::from_be_bytes(bytes),
        _ => unreachable!(),
    };
    Ok(Some(value))
}

fn copy_item<R: BufRead>(reader: &mut R, out: &mut Vec<u8>, depth: usize) -> io::Result<()> {
    if depth > 64 {
        return Err(invalid("CBOR nesting limit exceeded"));
    }
    let head = read_byte(reader, out)?;
    let major = head & 0xe0;
    let ai = head & 0x1f;
    let argument = read_arg(reader, out, ai)?;

    match major {
        0x00 | 0x20 => {
            if argument.is_none() {
                return Err(invalid("indefinite integer"));
            }
        }
        0x40 | 0x60 => {
            if let Some(len) = argument {
                read_exact_into(reader, out, to_usize(len)?)?;
            } else {
                copy_indefinite_string(reader, out, major)?;
            }
        }
        0x80 => {
            if let Some(len) = argument {
                for _ in 0..len {
                    copy_item(reader, out, depth + 1)?;
                }
            } else {
                loop {
                    if peek_break(reader)? {
                        read_byte(reader, out)?;
                        break;
                    }
                    copy_item(reader, out, depth + 1)?;
                }
            }
        }
        0xa0 => {
            if let Some(len) = argument {
                for _ in 0..len {
                    copy_item(reader, out, depth + 1)?;
                    copy_item(reader, out, depth + 1)?;
                }
            } else {
                loop {
                    if peek_break(reader)? {
                        read_byte(reader, out)?;
                        break;
                    }
                    copy_item(reader, out, depth + 1)?;
                    if peek_break(reader)? {
                        return Err(invalid("indefinite map has an odd number of items"));
                    }
                    copy_item(reader, out, depth + 1)?;
                }
            }
        }
        0xc0 => {
            if argument.is_none() {
                return Err(invalid("tag with indefinite length"));
            }
            copy_item(reader, out, depth + 1)?;
        }
        _ => match ai {
            0..=23 => {}
            24 => {
                // RFC 8949 section 3.3: a two-byte simple value must be at
                // least 32.
                if argument.unwrap_or(u64::MAX) < 32 {
                    return Err(invalid("reserved CBOR simple value"));
                }
            }
            25..=27 => {}
            28..=30 => return Err(invalid("reserved CBOR additional information")),
            31 => return Err(invalid("unexpected CBOR break")),
            _ => unreachable!(),
        },
    }
    Ok(())
}

fn copy_indefinite_string<R: BufRead>(reader: &mut R, out: &mut Vec<u8>, major: u8) -> io::Result<()> {
    loop {
        if peek_break(reader)? {
            read_byte(reader, out)?;
            return Ok(());
        }
        let head = read_byte(reader, out)?;
        if head & 0xe0 != major || head & 0x1f == 31 {
            return Err(invalid("invalid indefinite string chunk"));
        }
        let len = read_arg(reader, out, head & 0x1f)?.ok_or_else(|| invalid("invalid indefinite string chunk"))?;
        read_exact_into(reader, out, to_usize(len)?)?;
    }
}

fn peek_break<R: BufRead>(reader: &mut R) -> io::Result<bool> {
    let buffer = reader.fill_buf()?;
    Ok(buffer.first() == Some(&0xff))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn huge_lengths_do_not_allocate_or_truncate() {
        // Byte string with a 64-bit length of u64::MAX, then no content.
        let bytes = vec![0x5b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let mut reader = std::io::BufReader::new(Cursor::new(bytes));
        assert!(read_item(&mut reader).is_err());
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn lengths_that_do_not_fit_usize_are_rejected() {
        assert!(to_usize(u64::from(u32::MAX) + 1).is_err());
    }

    #[test]
    fn read_item_into_reuses_the_buffer() {
        let item = crate::to_vec(&("hello", 42u64));
        let mut stream = item.clone();
        stream.extend_from_slice(&item);

        let mut reader = std::io::BufReader::new(Cursor::new(stream));
        let mut out = Vec::new();

        let first = read_item_into(&mut reader, &mut out).unwrap();
        assert_eq!(&out[..first], &item[..]);

        let second = read_item_into(&mut reader, &mut out).unwrap();
        assert_eq!(&out[..second], &item[..]);
        assert_eq!(reader.fill_buf().unwrap(), &[]);
    }

    #[test]
    fn small_buffers_produce_the_same_bytes() {
        let item = crate::to_vec(&("a moderately long text", vec![1u8, 2, 3, 4], 7u64));
        for capacity in [1usize, 2, 3, 7, 16, 64] {
            let mut reader = std::io::BufReader::with_capacity(capacity, Cursor::new(item.clone()));
            assert_eq!(read_item(&mut reader).unwrap(), item, "capacity {capacity}");
        }
    }

    #[test]
    fn reserved_two_byte_simple_values_are_rejected() {
        for bytes in [
            &[0xf8, 0x00][..],
            &[0xf8, 0x01][..],
            &[0xf8, 0x18][..],
            &[0xf8, 0x1f][..],
        ] {
            let mut reader = std::io::BufReader::new(Cursor::new(bytes));
            assert!(read_item(&mut reader).is_err(), "accepted {bytes:02x?}");
        }
        // The smallest valid two-byte simple value.
        let mut reader = std::io::BufReader::new(Cursor::new([0xf8, 0x20]));
        assert_eq!(read_item(&mut reader).unwrap(), vec![0xf8, 0x20]);
    }

    #[test]
    fn truncated_item_is_an_error() {
        for cut in 0..5 {
            let item = crate::to_vec(&("hello", 42u64));
            let mut reader = std::io::BufReader::new(Cursor::new(&item[..cut]));
            assert!(read_item(&mut reader).is_err(), "cut {cut}");
        }
    }
}
