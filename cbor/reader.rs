//! Framing a single CBOR item from a buffered reader.

use std::io::{self, BufRead};

/// Read exactly one CBOR item from `reader`, returning its encoded bytes.
///
/// Bytes after the item remain buffered in `reader`. The function enforces a
/// nesting limit to avoid unbounded recursion on hostile input.
pub fn read_item<R: BufRead>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    copy_item(reader, &mut out, 0)?;
    Ok(out)
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn read_byte<R: BufRead>(reader: &mut R, out: &mut Vec<u8>) -> io::Result<u8> {
    let buffer = reader.fill_buf()?;
    let Some(&byte) = buffer.first() else {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "truncated CBOR item"));
    };
    reader.consume(1);
    out.push(byte);
    Ok(byte)
}

fn to_usize(len: u64) -> io::Result<usize> {
    usize::try_from(len).map_err(|_| invalid("CBOR length does not fit usize"))
}

fn read_exact<R: BufRead>(reader: &mut R, out: &mut Vec<u8>, len: usize) -> io::Result<()> {
    let mut chunk = [0u8; 4096];
    let mut remaining = len;
    while remaining > 0 {
        let take = remaining.min(chunk.len());
        reader.read_exact(&mut chunk[..take])?;
        out.extend_from_slice(&chunk[..take]);
        remaining -= take;
    }
    Ok(())
}

fn read_arg<R: BufRead>(reader: &mut R, out: &mut Vec<u8>, ai: u8) -> io::Result<Option<u64>> {
    Ok(match ai {
        0..=23 => Some(u64::from(ai)),
        24 => {
            let mut bytes = [0u8; 1];
            reader.read_exact(&mut bytes)?;
            out.extend_from_slice(&bytes);
            Some(u64::from(bytes[0]))
        }
        25 => {
            let mut bytes = [0u8; 2];
            reader.read_exact(&mut bytes)?;
            out.extend_from_slice(&bytes);
            Some(u64::from(u16::from_be_bytes(bytes)))
        }
        26 => {
            let mut bytes = [0u8; 4];
            reader.read_exact(&mut bytes)?;
            out.extend_from_slice(&bytes);
            Some(u64::from(u32::from_be_bytes(bytes)))
        }
        27 => {
            let mut bytes = [0u8; 8];
            reader.read_exact(&mut bytes)?;
            out.extend_from_slice(&bytes);
            Some(u64::from_be_bytes(bytes))
        }
        28..=30 => return Err(invalid("reserved CBOR additional information")),
        31 => None,
        _ => unreachable!(),
    })
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
                read_exact(reader, out, to_usize(len)?)?;
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
            0..=27 => {}
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
        read_exact(reader, out, to_usize(len)?)?;
    }
}

fn peek_break<R: BufRead>(reader: &mut R) -> io::Result<bool> {
    let buffer = reader.fill_buf()?;
    Ok(buffer.first() == Some(&0xff))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huge_lengths_do_not_allocate_or_truncate() {
        // Byte string with a 64-bit length of u64::MAX, then no content.
        let bytes = vec![0x5b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let mut reader = std::io::BufReader::new(std::io::Cursor::new(bytes));
        assert!(read_item(&mut reader).is_err());
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn lengths_that_do_not_fit_usize_are_rejected() {
        assert!(to_usize(u64::from(u32::MAX) + 1).is_err());
    }
}
