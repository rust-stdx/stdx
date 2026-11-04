//! Well-formedness and deterministic-encoding validation.

use core::cmp::Ordering;

use crate::{
    data::Type,
    decode::{Decoder, Options},
    error::{Error, ErrorKind, Result},
    float::shortest_f64,
};

/// The map-key ordering rule used by a deterministic encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeyOrder {
    /// RFC 8949 core deterministic encoding: bytewise lexicographic order of
    /// the encoded keys.
    Core,
    /// RFC 8949 length-first ordering (the older RFC 7049 "canonical" order,
    /// also used by CTAP2).
    LengthFirst,
}

/// Validate exactly one well-formed CBOR item, including text UTF-8 validity.
///
/// This does not check deterministic encoding and does not reject duplicate
/// map keys. Use [`validate_canonical`] for the deterministic form.
pub fn validate(bytes: &[u8]) -> Result<()> {
    let used = validate_prefix(bytes)?;
    if used != bytes.len() {
        return Err(Error::trailing_data().at(used));
    }
    Ok(())
}

/// Validate one well-formed CBOR item and return its encoded length.
pub fn validate_prefix(bytes: &[u8]) -> Result<usize> {
    validate_prefix_with(bytes, Options::default(), KeyOrder::Core, false)
}

/// Validate a CBOR sequence (zero or more concatenated items).
pub fn validate_sequence(bytes: &[u8]) -> Result<()> {
    let mut pos = 0;
    while pos < bytes.len() {
        let used = validate_prefix_with(&bytes[pos..], Options::default(), KeyOrder::Core, false)?;
        pos += used;
    }
    Ok(())
}

/// Validate exactly one deterministically encoded CBOR item.
///
/// This checks the RFC 8949 core deterministic form, including preferred
/// bignum serialization (tags 2 and 3). Tag validity beyond the bignum
/// preferred form is left to the application.
pub fn validate_canonical(bytes: &[u8]) -> Result<()> {
    validate_canonical_with(bytes, KeyOrder::Core)
}

/// Validate exactly one deterministically encoded CBOR item with an explicit
/// key ordering.
pub fn validate_canonical_with(bytes: &[u8], order: KeyOrder) -> Result<()> {
    let used = validate_prefix_with(bytes, Options::default(), order, true)?;
    if used != bytes.len() {
        return Err(Error::trailing_data().at(used));
    }
    Ok(())
}

/// Validate one item with explicit options.
pub fn validate_prefix_with(bytes: &[u8], options: Options, order: KeyOrder, canonical: bool) -> Result<usize> {
    let mut validator = Validator {
        decoder: Decoder::with_options(bytes, options),
        order,
        canonical,
    };
    validator.item(0)?;
    Ok(validator.decoder.position())
}

struct Validator<'a> {
    decoder: Decoder<'a>,
    order: KeyOrder,
    canonical: bool,
}

impl<'a> Validator<'a> {
    fn item(&mut self, depth: usize) -> Result<()> {
        if depth > self.decoder.options().max_depth {
            return Err(Error::depth_limit().at(self.decoder.position()));
        }
        let start = self.decoder.position();
        let b = self.decoder.peek_byte()?;
        let major = b & 0xe0;
        let ai = b & 0x1f;

        if self.canonical {
            if ai == 31 {
                return Err(Error::message("indefinite-length item in canonical encoding").at(start));
            }
            // Float arguments are bit patterns, not integers; their shortest
            // form is checked by `float`.
            if major != 0xe0 || ai == 24 {
                self.check_shortest(start, major, ai)?;
            }
        }

        match major {
            0x00 | 0x20 => {
                self.decoder.int()?;
            }
            0x40 => self.string(start, major, ai)?,
            0x60 => self.string(start, major, ai)?,
            0x80 => self.array(depth)?,
            0xa0 => self.map(depth)?,
            0xc0 => {
                let tag = self.decoder.tag()?;
                let content_start = self.decoder.position();
                self.item(depth + 1)?;
                if self.canonical && matches!(tag.get(), 2 | 3) {
                    self.bignum(content_start)?;
                }
            }
            _ => self.simple(start, ai)?,
        }
        Ok(())
    }

    fn string(&mut self, _start: usize, major: u8, ai: u8) -> Result<()> {
        if ai != 31 {
            if major == 0x40 {
                self.decoder.bytes()?;
            } else {
                self.decoder.text()?;
            }
            return Ok(());
        }
        // Indefinite-length string. Each chunk must be a definite-length
        // string of the same major type. Text chunks must each be valid
        // UTF-8; RFC 8949 forbids splitting a code point across chunks.
        if major == 0x40 {
            let iter = self.decoder.bytes_iter()?;
            for chunk in iter {
                chunk?;
            }
        } else {
            let iter = self.decoder.text_iter()?;
            for chunk in iter {
                chunk?;
            }
        }
        Ok(())
    }

    fn array(&mut self, depth: usize) -> Result<()> {
        match self.decoder.array()? {
            Some(len) => {
                for _ in 0..len {
                    self.item(depth + 1)?;
                }
            }
            None => {
                while !self.decoder.is_break() {
                    self.item(depth + 1)?;
                }
                self.decoder.read_break()?;
            }
        }
        Ok(())
    }

    fn map(&mut self, depth: usize) -> Result<()> {
        let mut previous: Option<(usize, usize)> = None;
        match self.decoder.map()? {
            Some(len) => {
                for _ in 0..len {
                    let key_start = self.decoder.position();
                    self.item(depth + 1)?;
                    let key_end = self.decoder.position();
                    if self.canonical {
                        self.check_key_order(previous, (key_start, key_end))?;
                    }
                    previous = Some((key_start, key_end));
                    self.item(depth + 1)?;
                }
            }
            None => loop {
                if self.decoder.is_break() {
                    self.decoder.read_break()?;
                    break;
                }
                let key_start = self.decoder.position();
                self.item(depth + 1)?;
                let key_end = self.decoder.position();
                if self.canonical {
                    self.check_key_order(previous, (key_start, key_end))?;
                }
                previous = Some((key_start, key_end));
                if self.decoder.is_break() {
                    return Err(Error::message("indefinite map has an odd number of items").at(self.decoder.position()));
                }
                self.item(depth + 1)?;
            },
        }
        Ok(())
    }

    fn check_key_order(&self, previous: Option<(usize, usize)>, current: (usize, usize)) -> Result<()> {
        let Some(previous) = previous else {
            return Ok(());
        };
        let a = &self.decoder.input()[previous.0..previous.1];
        let b = &self.decoder.input()[current.0..current.1];
        let ordering = match self.order {
            KeyOrder::Core => a.cmp(b),
            KeyOrder::LengthFirst => a.len().cmp(&b.len()).then_with(|| a.cmp(b)),
        };
        if ordering != Ordering::Less {
            let message = if a == b {
                "duplicate map key"
            } else {
                "map keys are not sorted"
            };
            return Err(Error::message(message).at(current.0));
        }
        Ok(())
    }

    fn simple(&mut self, start: usize, ai: u8) -> Result<()> {
        match ai {
            0..=19 => {
                self.decoder.skip()?;
            }
            20..=23 => {
                self.decoder.skip()?;
            }
            24 => {
                let value = *self
                    .decoder
                    .input()
                    .get(start + 1)
                    .ok_or_else(|| Error::eof().at(start))?;
                if value < 32 {
                    return Err(Error::new(ErrorKind::InvalidSimple(value), start));
                }
                self.decoder.skip()?;
            }
            25 => {
                self.float(start)?;
            }
            26 => {
                self.float(start)?;
            }
            27 => {
                self.float(start)?;
            }
            28..=30 => {
                return Err(Error::message("reserved additional information").at(start));
            }
            31 => {
                return Err(Error::message("unexpected break").at(start));
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn float(&mut self, start: usize) -> Result<()> {
        let value = self.decoder.f64()?;
        if self.canonical {
            let end = self.decoder.position();
            let (tag, bits) = shortest_f64(value);
            let mut buf = [0u8; 9];
            let len = match tag {
                0xf9 => {
                    buf[0] = 0xf9;
                    buf[1..3].copy_from_slice(&(bits as u16).to_be_bytes());
                    3
                }
                0xfa => {
                    buf[0] = 0xfa;
                    buf[1..5].copy_from_slice(&(bits as u32).to_be_bytes());
                    5
                }
                _ => {
                    buf[0] = 0xfb;
                    buf[1..9].copy_from_slice(&bits.to_be_bytes());
                    9
                }
            };
            if self.decoder.input().get(start..end) != Some(&buf[..len]) {
                return Err(Error::message("float is not in shortest form").at(start));
            }
        }
        Ok(())
    }

    fn bignum(&self, start: usize) -> Result<()> {
        let input = self.decoder.input();
        let end = self.decoder.position();
        let encoded = input.get(start..end).ok_or_else(|| Error::eof().at(start))?;
        let first = *encoded.first().ok_or_else(|| Error::eof().at(start))?;
        // Tag validity is a separate concern; only enforce preferred bignum
        // serialization when the content is a byte string.
        if first & 0xe0 != 0x40 || first & 0x1f == 31 {
            return Ok(());
        }
        let ai = first & 0x1f;
        let (header_len, len) = match ai {
            0..=23 => (1usize, u64::from(ai)),
            24 => (2usize, u64::from(*encoded.get(1).ok_or_else(|| Error::eof().at(start))?)),
            25 => {
                let b = encoded.get(1..3).ok_or_else(|| Error::eof().at(start))?;
                (3usize, u64::from(u16::from_be_bytes([b[0], b[1]])))
            }
            26 => {
                let b = encoded.get(1..5).ok_or_else(|| Error::eof().at(start))?;
                (5usize, u64::from(u32::from_be_bytes([b[0], b[1], b[2], b[3]])))
            }
            27 => {
                let b = encoded.get(1..9).ok_or_else(|| Error::eof().at(start))?;
                (9usize, u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
            }
            _ => return Ok(()),
        };
        let len = usize::try_from(len).map_err(|_| Error::new(ErrorKind::LengthOverflow, start))?;
        let body_end = header_len
            .checked_add(len)
            .ok_or_else(|| Error::new(ErrorKind::LengthOverflow, start))?;
        let body = encoded
            .get(header_len..body_end)
            .ok_or_else(|| Error::eof().at(start))?;
        if body.first() == Some(&0) {
            return Err(Error::message("bignum has leading zero bytes").at(start));
        }
        if body.len() <= 8 {
            return Err(Error::message("bignum value fits a basic integer").at(start));
        }
        Ok(())
    }

    fn check_shortest(&self, start: usize, major: u8, ai: u8) -> Result<()> {
        let input = self.decoder.input();
        let argument = match ai {
            24 => u64::from(*input.get(start + 1).ok_or_else(|| Error::eof().at(start))?),
            25 => {
                let b = input.get(start + 1..start + 3).ok_or_else(|| Error::eof().at(start))?;
                u64::from(u16::from_be_bytes([b[0], b[1]]))
            }
            26 => {
                let b = input.get(start + 1..start + 5).ok_or_else(|| Error::eof().at(start))?;
                u64::from(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
            }
            27 => {
                let b = input.get(start + 1..start + 9).ok_or_else(|| Error::eof().at(start))?;
                u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
            }
            28..=30 => {
                return Err(Error::message("reserved additional information").at(start));
            }
            31 => return Ok(()),
            _ => return Ok(()),
        };
        let minimum = match ai {
            24 => 24,
            25 => 0x100,
            26 => 0x1_0000,
            27 => 0x1_0000_0000,
            _ => return Ok(()),
        };
        if argument < minimum {
            // For major type 7, ai 24 is a simple value, but the rule is the
            // same: values below 32 cannot use the two-byte form.
            if major == 0xe0 && ai == 24 {
                if argument < 32 {
                    return Err(Error::new(ErrorKind::InvalidSimple(argument as u8), start));
                }
                return Ok(());
            }
            return Err(Error::message("argument is not in shortest form").at(start));
        }
        Ok(())
    }
}

impl Type {
    /// Return `true` if this is an indefinite-length type.
    #[inline]
    pub const fn is_indefinite(self) -> bool {
        matches!(self, Type::BytesIndef | Type::TextIndef | Type::ArrayIndef | Type::MapIndef)
    }
}
