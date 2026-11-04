//! Decoding: [`Decoder`], the [`Decode`] trait and the convenience
//! constructors.

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::{marker::PhantomData, str};

use crate::{
    data::{Integer, Simple, Tag, Type},
    error::{Error, ErrorKind, Result},
    float::f16_to_f64,
};

/// Policy for duplicate map keys.
///
/// Native typed maps honor every variant. The serde adapter honors `Error` for
/// keys with byte-identical wire encodings; it treats `First`, `Last` and
/// `Allow` as lenient because serde's `MapAccess` cannot suppress an entry
/// after decoding its key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DuplicateKeys {
    /// Reject a map containing the same key twice.
    Error,
    /// Keep the first value for a duplicated key.
    First,
    /// Keep the last value for a duplicated key.
    Last,
    /// Pass every pair to the caller (only useful for order-preserving maps).
    Allow,
}

/// Bound a decoder's initial allocation for a collection.
///
/// The length comes from untrusted input, so preallocating `len * size_of::<T>()`
/// can amplify a small payload into a huge allocation when `T` is large. This
/// caps the initial request to a small byte budget; the collection still grows
/// normally as items are successfully decoded.
#[cfg(feature = "alloc")]
#[inline]
pub(crate) fn collection_capacity<T>(len: usize, remaining: usize) -> usize {
    const MAX_PREALLOC_BYTES: usize = 64 * 1024;
    const INPUT_EXPANSION: usize = 8;
    let element_size = core::mem::size_of::<T>().max(1);
    let budget = remaining.saturating_mul(INPUT_EXPANSION).min(MAX_PREALLOC_BYTES);
    len.min(budget / element_size)
}

/// Decoder configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    /// Maximum nesting depth for recursive decoding and validation.
    pub max_depth: usize,
    /// Duplicate-key policy for typed maps.
    pub duplicate_keys: DuplicateKeys,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            max_depth: 32,
            duplicate_keys: DuplicateKeys::Error,
        }
    }
}

/// A non-allocating CBOR decoder over a byte slice.
#[derive(Clone, Debug)]
pub struct Decoder<'b> {
    input: &'b [u8],
    pos: usize,
    depth: usize,
    options: Options,
}

impl<'b> Decoder<'b> {
    /// Create a decoder over `input`.
    #[inline]
    pub fn new(input: &'b [u8]) -> Self {
        Decoder {
            input,
            pos: 0,
            depth: 0,
            options: Options::default(),
        }
    }

    /// Create a decoder with explicit options.
    #[inline]
    pub fn with_options(input: &'b [u8], options: Options) -> Self {
        Decoder {
            input,
            pos: 0,
            depth: 0,
            options,
        }
    }

    /// Return the decoder options.
    #[inline]
    pub const fn options(&self) -> Options {
        self.options
    }

    /// Replace the decoder options.
    #[inline]
    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    /// Return the current byte position.
    #[inline]
    pub const fn position(&self) -> usize {
        self.pos
    }

    /// Set the current byte position.
    ///
    /// The caller is responsible for leaving the decoder at an item boundary.
    #[inline]
    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos.min(self.input.len());
    }

    /// Return the full input slice.
    #[inline]
    pub const fn input(&self) -> &'b [u8] {
        self.input
    }

    /// Return the number of unread bytes.
    #[inline]
    pub const fn remaining(&self) -> usize {
        self.input.len() - self.pos
    }

    /// Return `true` if all input has been consumed.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Return the unread input.
    #[inline]
    pub fn remaining_slice(&self) -> &'b [u8] {
        &self.input[self.pos..]
    }

    #[inline(always)]
    fn read(&mut self) -> Result<u8> {
        match self.input.get(self.pos) {
            Some(&b) => {
                self.pos += 1;
                Ok(b)
            }
            None => Err(Error::eof().at(self.pos)),
        }
    }

    #[inline(always)]
    fn peek(&self) -> Result<u8> {
        self.input
            .get(self.pos)
            .copied()
            .ok_or_else(|| Error::eof().at(self.pos))
    }

    #[inline(always)]
    fn take(&mut self, len: usize) -> Result<&'b [u8]> {
        if len > self.remaining() {
            return Err(Error::eof().at(self.pos));
        }
        let start = self.pos;
        self.pos += len;
        Ok(&self.input[start..self.pos])
    }

    #[inline(always)]
    fn take_u64(&mut self, len: u64) -> Result<&'b [u8]> {
        let len = usize::try_from(len).map_err(|_| Error::new(ErrorKind::LengthOverflow, self.pos))?;
        self.take(len)
    }

    #[inline(always)]
    fn read_arg(&mut self, ai: u8) -> Result<u64> {
        match ai {
            0..=23 => Ok(u64::from(ai)),
            24 => Ok(u64::from(self.read()?)),
            25 => {
                let b = self.take(2)?;
                Ok(u64::from(u16::from_be_bytes([b[0], b[1]])))
            }
            26 => {
                let b = self.take(4)?;
                Ok(u64::from(u32::from_be_bytes([b[0], b[1], b[2], b[3]])))
            }
            27 => {
                let b = self.take(8)?;
                Ok(u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
            }
            28..=30 => Err(Error::message("reserved additional information").at(self.pos)),
            31 => Err(Error::message("unexpected indefinite length or break").at(self.pos)),
            _ => unreachable!(),
        }
    }

    /// Return the byte at the current position without consuming it.
    #[inline]
    pub fn peek_byte(&self) -> Result<u8> {
        self.peek()
    }

    /// Return `true` if the next item is the `break` stop code.
    #[inline]
    pub fn is_break(&self) -> bool {
        matches!(self.peek(), Ok(0xff))
    }

    /// Consume a `break` stop code.
    #[inline]
    pub fn read_break(&mut self) -> Result<()> {
        match self.peek()? {
            0xff => {
                self.pos += 1;
                Ok(())
            }
            _ => Err(Error::message("expected break").at(self.pos)),
        }
    }

    /// Return the type of the next item without consuming it.
    pub fn datatype(&self) -> Result<Type> {
        let b = self.peek()?;
        let major = b & 0xe0;
        let ai = b & 0x1f;
        Ok(match major {
            0x00 => match ai {
                0..=24 => Type::U8,
                25 => Type::U16,
                26 => Type::U32,
                27 => Type::U64,
                _ => Type::Unknown(b),
            },
            0x20 => match ai {
                0..=24 => Type::I8,
                25 => Type::I16,
                26 => Type::I32,
                27 => Type::I64,
                _ => Type::Unknown(b),
            },
            0x40 => {
                if ai == 31 {
                    Type::BytesIndef
                } else {
                    Type::Bytes
                }
            }
            0x60 => {
                if ai == 31 {
                    Type::TextIndef
                } else {
                    Type::Text
                }
            }
            0x80 => {
                if ai == 31 {
                    Type::ArrayIndef
                } else {
                    Type::Array
                }
            }
            0xa0 => {
                if ai == 31 {
                    Type::MapIndef
                } else {
                    Type::Map
                }
            }
            0xc0 => Type::Tag,
            _ => match ai {
                0..=19 => Type::Simple,
                20 | 21 => Type::Bool,
                22 => Type::Null,
                23 => Type::Undefined,
                24 => Type::Simple,
                25 => Type::F16,
                26 => Type::F32,
                27 => Type::F64,
                28..=30 => Type::Unknown(b),
                31 => Type::Break,
                _ => unreachable!(),
            },
        })
    }

    /// Increment the recursion depth, failing if the limit is reached.
    #[inline]
    pub fn enter_depth(&mut self) -> Result<()> {
        if self.depth >= self.options.max_depth {
            return Err(Error::depth_limit().at(self.pos));
        }
        self.depth += 1;
        Ok(())
    }

    /// Decrement the recursion depth.
    #[inline]
    pub fn leave_depth(&mut self) {
        debug_assert!(self.depth > 0);
        self.depth -= 1;
    }

    /// Return the current recursion depth.
    #[inline]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Decode an unsigned integer.
    pub fn u64(&mut self) -> Result<u64> {
        let start = self.pos;
        let b = self.read()?;
        match b {
            0x00..=0x17 => Ok(u64::from(b)),
            0x18 => Ok(u64::from(self.read()?)),
            0x19 => {
                let b = self.take(2)?;
                Ok(u64::from(u16::from_be_bytes([b[0], b[1]])))
            }
            0x1a => {
                let b = self.take(4)?;
                Ok(u64::from(u32::from_be_bytes([b[0], b[1], b[2], b[3]])))
            }
            0x1b => {
                let b = self.take(8)?;
                Ok(u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
            }
            _ => Err(Error::type_mismatch(Type::U64, type_of_byte(b)).at(start)),
        }
    }

    /// Decode a `u8`.
    pub fn u8(&mut self) -> Result<u8> {
        let v = self.u64()?;
        u8::try_from(v).map_err(|_| Error::message("integer does not fit u8").at(self.pos))
    }

    /// Decode a `u16`.
    pub fn u16(&mut self) -> Result<u16> {
        let v = self.u64()?;
        u16::try_from(v).map_err(|_| Error::message("integer does not fit u16").at(self.pos))
    }

    /// Decode a `u32`.
    pub fn u32(&mut self) -> Result<u32> {
        let v = self.u64()?;
        u32::try_from(v).map_err(|_| Error::message("integer does not fit u32").at(self.pos))
    }

    /// Decode a `usize`.
    pub fn usize(&mut self) -> Result<usize> {
        let v = self.u64()?;
        usize::try_from(v).map_err(|_| Error::new(ErrorKind::LengthOverflow, self.pos))
    }

    /// Decode a signed integer.
    pub fn i64(&mut self) -> Result<i64> {
        let start = self.pos;
        let b = self.read()?;
        match b {
            0x00..=0x17 => Ok(i64::from(b)),
            0x18 => Ok(i64::from(self.read()?)),
            0x19 => {
                let b = self.take(2)?;
                Ok(i64::from(u16::from_be_bytes([b[0], b[1]])))
            }
            0x1a => {
                let b = self.take(4)?;
                Ok(i64::from(u32::from_be_bytes([b[0], b[1], b[2], b[3]])))
            }
            0x1b => {
                let b = self.take(8)?;
                i64::try_from(u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
                    .map_err(|_| Error::message("integer does not fit i64").at(start))
            }
            0x20..=0x37 => Ok(-1 - i64::from(b - 0x20)),
            0x38 => Ok(-1 - i64::from(self.read()?)),
            0x39 => {
                let b = self.take(2)?;
                Ok(-1 - i64::from(u16::from_be_bytes([b[0], b[1]])))
            }
            0x3a => {
                let b = self.take(4)?;
                Ok(-1 - i64::from(u32::from_be_bytes([b[0], b[1], b[2], b[3]])))
            }
            0x3b => {
                let b = self.take(8)?;
                let n = u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                if n > i64::MAX as u64 {
                    return Err(Error::message("integer does not fit i64").at(start));
                }
                Ok(-1 - n as i64)
            }
            _ => Err(Error::type_mismatch(Type::I64, type_of_byte(b)).at(start)),
        }
    }

    /// Decode an `i8`.
    pub fn i8(&mut self) -> Result<i8> {
        let v = self.i64()?;
        i8::try_from(v).map_err(|_| Error::message("integer does not fit i8").at(self.pos))
    }

    /// Decode an `i16`.
    pub fn i16(&mut self) -> Result<i16> {
        let v = self.i64()?;
        i16::try_from(v).map_err(|_| Error::message("integer does not fit i16").at(self.pos))
    }

    /// Decode an `i32`.
    pub fn i32(&mut self) -> Result<i32> {
        let v = self.i64()?;
        i32::try_from(v).map_err(|_| Error::message("integer does not fit i32").at(self.pos))
    }

    /// Decode an `isize`.
    pub fn isize(&mut self) -> Result<isize> {
        let v = self.i64()?;
        isize::try_from(v).map_err(|_| Error::message("integer does not fit isize").at(self.pos))
    }

    /// Decode a CBOR basic integer.
    pub fn int(&mut self) -> Result<Integer> {
        let start = self.pos;
        let b = self.read()?;
        match b & 0xe0 {
            0x00 => Ok(Integer::Unsigned(self.read_arg(b & 0x1f)?)),
            0x20 => Ok(Integer::Negative(self.read_arg(b & 0x1f)?)),
            _ => Err(Error::type_mismatch(Type::U64, type_of_byte(b)).at(start)),
        }
    }

    /// Decode a `u128`, accepting tag 2 bignums that fit.
    ///
    /// All well-formed tag 2 encodings are accepted, including non-shortest
    /// tags, indefinite-length byte strings, and leading zero bytes.
    pub fn u128(&mut self) -> Result<u128> {
        if self.peek()? & 0xe0 == 0xc0 {
            let start = self.pos;
            let tag = self.tag()?;
            if tag.get() != crate::data::tags::BIGPOS {
                return Err(Error::type_mismatch(Type::U64, Type::Tag).at(start));
            }
            self.bignum_magnitude(start)
        } else {
            let v = self.u64()?;
            Ok(u128::from(v))
        }
    }

    /// Decode an `i128`, accepting tags 2 and 3.
    ///
    /// All well-formed bignum encodings are accepted, including non-shortest
    /// tags, indefinite-length byte strings, and leading zero bytes.
    pub fn i128(&mut self) -> Result<i128> {
        let start = self.pos;
        if self.peek()? & 0xe0 == 0xc0 {
            let tag = self.tag()?;
            let magnitude = self.bignum_magnitude(start)?;
            if magnitude > i128::MAX as u128 {
                return Err(Error::message("bignum does not fit i128").at(start));
            }
            return match tag.get() {
                crate::data::tags::BIGPOS => Ok(magnitude as i128),
                crate::data::tags::BIGNEG => Ok(-1 - magnitude as i128),
                _ => Err(Error::type_mismatch(Type::I64, Type::Tag).at(start)),
            };
        }
        let b = self.read()?;
        match b & 0xe0 {
            0x00 => Ok(i128::from(self.read_arg(b & 0x1f)?)),
            0x20 => {
                let n = self.read_arg(b & 0x1f)?;
                Ok(-1 - i128::from(n))
            }
            _ => Err(Error::type_mismatch(Type::I64, type_of_byte(b)).at(start)),
        }
    }

    fn bignum_magnitude(&mut self, start: usize) -> Result<u128> {
        let b = self.read()?;
        if b & 0xe0 != 0x40 {
            return Err(Error::type_mismatch(Type::Bytes, type_of_byte(b)).at(start));
        }
        let mut value = 0u128;
        if b & 0x1f == 31 {
            loop {
                let chunk_start = self.pos;
                let head = self.peek()?;
                if head == 0xff {
                    self.pos += 1;
                    return Ok(value);
                }
                if head & 0xe0 != 0x40 || head & 0x1f == 31 {
                    return Err(Error::message("invalid indefinite bytes chunk").at(chunk_start));
                }
                self.pos += 1;
                let len = self.read_arg(head & 0x1f)?;
                let bytes = self.take_u64(len)?;
                for &byte in bytes {
                    value = value
                        .checked_mul(256)
                        .and_then(|value| value.checked_add(u128::from(byte)))
                        .ok_or_else(|| Error::message("bignum does not fit integer type").at(start))?;
                }
            }
        }
        let len = self.read_arg(b & 0x1f)?;
        let bytes = self.take_u64(len)?;
        for &byte in bytes {
            value = value
                .checked_mul(256)
                .and_then(|value| value.checked_add(u128::from(byte)))
                .ok_or_else(|| Error::message("bignum does not fit integer type").at(start))?;
        }
        Ok(value)
    }

    /// Decode a float as `f64`, accepting f16, f32 and f64.
    pub fn f64(&mut self) -> Result<f64> {
        let start = self.pos;
        match self.read()? {
            0xf9 => {
                let b = self.take(2)?;
                Ok(f16_to_f64(u16::from_be_bytes([b[0], b[1]])))
            }
            0xfa => {
                let b = self.take(4)?;
                Ok(f64::from(f32::from_be_bytes([b[0], b[1], b[2], b[3]])))
            }
            0xfb => {
                let b = self.take(8)?;
                Ok(f64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
            }
            b => Err(Error::type_mismatch(Type::F64, type_of_byte(b)).at(start)),
        }
    }

    /// Decode a float as `f32`.
    ///
    /// Returns an error if the value cannot be represented as `f32` without
    /// loss. Use [`Decoder::f64`] for a lossless conversion.
    pub fn f32(&mut self) -> Result<f32> {
        let value = self.f64()?;
        if value.is_nan() {
            return Ok(f32::NAN);
        }
        let narrowed = value as f32;
        if f64::from(narrowed).to_bits() == value.to_bits() {
            Ok(narrowed)
        } else {
            Err(Error::message("float cannot be represented as f32 without loss").at(self.pos))
        }
    }

    /// Decode a boolean.
    pub fn bool(&mut self) -> Result<bool> {
        let start = self.pos;
        match self.read()? {
            0xf4 => Ok(false),
            0xf5 => Ok(true),
            b => Err(Error::type_mismatch(Type::Bool, type_of_byte(b)).at(start)),
        }
    }

    /// Decode `null`.
    pub fn null(&mut self) -> Result<()> {
        let start = self.pos;
        match self.read()? {
            0xf6 => Ok(()),
            b => Err(Error::type_mismatch(Type::Null, type_of_byte(b)).at(start)),
        }
    }

    /// Decode `undefined`.
    pub fn undefined(&mut self) -> Result<()> {
        let start = self.pos;
        match self.read()? {
            0xf7 => Ok(()),
            b => Err(Error::type_mismatch(Type::Undefined, type_of_byte(b)).at(start)),
        }
    }

    /// Decode a simple value.
    pub fn simple(&mut self) -> Result<Simple> {
        let start = self.pos;
        match self.read()? {
            b @ 0xe0..=0xf3 => Simple::new(b & 0x1f).map_err(|e| e.at(start)),
            0xf8 => {
                let value = self.read()?;
                if value < 32 {
                    Err(Error::new(ErrorKind::InvalidSimple(value), start))
                } else {
                    Simple::new(value).map_err(|e| e.at(start))
                }
            }
            b => Err(Error::type_mismatch(Type::Simple, type_of_byte(b)).at(start)),
        }
    }

    /// Decode a definite-length byte string.
    ///
    /// Indefinite-length strings must be read with [`Decoder::bytes_iter`] or
    /// [`Decoder::bytes_owned`].
    pub fn bytes(&mut self) -> Result<&'b [u8]> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0x40 || b & 0x1f == 31 {
            return Err(Error::type_mismatch(Type::Bytes, type_of_byte(b)).at(start));
        }
        let len = self.read_arg(b & 0x1f)?;
        self.take_u64(len)
    }

    /// Decode a definite-length text string.
    ///
    /// Indefinite-length strings must be read with [`Decoder::text_iter`] or
    /// [`Decoder::text_owned`].
    pub fn text(&mut self) -> Result<&'b str> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0x60 || b & 0x1f == 31 {
            return Err(Error::type_mismatch(Type::Text, type_of_byte(b)).at(start));
        }
        let len = self.read_arg(b & 0x1f)?;
        let bytes = self.take_u64(len)?;
        str::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::InvalidUtf8, start))
    }

    /// Return an iterator over the chunks of a byte string.
    pub fn bytes_iter(&mut self) -> Result<BytesIter<'_, 'b>> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0x40 {
            return Err(Error::type_mismatch(Type::Bytes, type_of_byte(b)).at(start));
        }
        if b & 0x1f == 31 {
            Ok(BytesIter {
                decoder: self,
                remaining: None,
                done: false,
            })
        } else {
            let len = self.read_arg(b & 0x1f)?;
            if len > self.remaining() as u64 {
                return Err(Error::eof().at(self.pos));
            }
            Ok(BytesIter {
                decoder: self,
                remaining: Some(len),
                done: false,
            })
        }
    }

    /// Return an iterator over the chunks of a text string.
    pub fn text_iter(&mut self) -> Result<TextIter<'_, 'b>> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0x60 {
            return Err(Error::type_mismatch(Type::Text, type_of_byte(b)).at(start));
        }
        if b & 0x1f == 31 {
            Ok(TextIter {
                decoder: self,
                remaining: None,
                done: false,
            })
        } else {
            let len = self.read_arg(b & 0x1f)?;
            if len > self.remaining() as u64 {
                return Err(Error::eof().at(self.pos));
            }
            Ok(TextIter {
                decoder: self,
                remaining: Some(len),
                done: false,
            })
        }
    }

    /// Read an array header.
    ///
    /// Returns `None` for an indefinite-length array.
    pub fn array(&mut self) -> Result<Option<usize>> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0x80 {
            return Err(Error::type_mismatch(Type::Array, type_of_byte(b)).at(start));
        }
        if b & 0x1f == 31 {
            return Ok(None);
        }
        let len = self.read_arg(b & 0x1f)?;
        if len > self.remaining() as u64 {
            return Err(Error::eof().at(self.pos));
        }
        Ok(Some(len as usize))
    }

    /// Read a map header.
    ///
    /// Returns `None` for an indefinite-length map.
    pub fn map(&mut self) -> Result<Option<usize>> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0xa0 {
            return Err(Error::type_mismatch(Type::Map, type_of_byte(b)).at(start));
        }
        if b & 0x1f == 31 {
            return Ok(None);
        }
        let len = self.read_arg(b & 0x1f)?;
        if len > (self.remaining() / 2) as u64 {
            return Err(Error::eof().at(self.pos));
        }
        Ok(Some(len as usize))
    }

    /// Read a tag number.
    pub fn tag(&mut self) -> Result<Tag> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0xc0 || b & 0x1f == 31 {
            return Err(Error::type_mismatch(Type::Tag, type_of_byte(b)).at(start));
        }
        Ok(Tag::new(self.read_arg(b & 0x1f)?))
    }

    /// Read a tag and require it to equal `expected`.
    pub fn tag_expected(&mut self, expected: u64) -> Result<()> {
        let start = self.pos;
        let tag = self.tag()?;
        if tag.get() == expected {
            Ok(())
        } else {
            Err(Error::message("unexpected tag").at(start))
        }
    }

    /// Skip one complete item.
    ///
    /// Text strings are checked for UTF-8 validity, matching [`validate`](crate::validate).
    /// The configured depth limit applies to nested containers and tags.
    pub fn skip(&mut self) -> Result<()> {
        self.skip_depth(0)
    }

    fn skip_depth(&mut self, depth: usize) -> Result<()> {
        if depth > self.options.max_depth {
            return Err(Error::depth_limit().at(self.pos));
        }
        let start = self.pos;
        let b = self.read()?;
        let major = b & 0xe0;
        let ai = b & 0x1f;
        match major {
            0x00 | 0x20 => {
                self.read_arg(ai)?;
            }
            0x40 | 0x60 => {
                if ai == 31 {
                    self.skip_indefinite_string(major, depth)?;
                } else {
                    let len = self.read_arg(ai)?;
                    let bytes = self.take_u64(len)?;
                    if major == 0x60 && str::from_utf8(bytes).is_err() {
                        return Err(Error::new(ErrorKind::InvalidUtf8, start));
                    }
                }
            }
            0x80 => {
                if ai == 31 {
                    loop {
                        if self.is_break() {
                            self.read_break()?;
                            break;
                        }
                        self.skip_depth(depth + 1)?;
                    }
                } else {
                    let len = self.read_arg(ai)?;
                    for _ in 0..len {
                        self.skip_depth(depth + 1)?;
                    }
                }
            }
            0xa0 => {
                if ai == 31 {
                    loop {
                        if self.is_break() {
                            self.read_break()?;
                            break;
                        }
                        self.skip_depth(depth + 1)?;
                        if self.is_break() {
                            return Err(Error::message("indefinite map has an odd number of items").at(self.pos));
                        }
                        self.skip_depth(depth + 1)?;
                    }
                } else {
                    let len = self.read_arg(ai)?;
                    for _ in 0..len {
                        self.skip_depth(depth + 1)?;
                        self.skip_depth(depth + 1)?;
                    }
                }
            }
            0xc0 => {
                self.read_arg(ai)?;
                self.skip_depth(depth + 1)?;
            }
            _ => match ai {
                0..=23 => {}
                24 => {
                    let value = self.read()?;
                    if value < 32 {
                        return Err(Error::new(ErrorKind::InvalidSimple(value), start));
                    }
                }
                25 => {
                    self.take(2)?;
                }
                26 => {
                    self.take(4)?;
                }
                27 => {
                    self.take(8)?;
                }
                28..=30 => return Err(Error::message("reserved additional information").at(start)),
                31 => return Err(Error::message("unexpected break").at(start)),
                _ => unreachable!(),
            },
        }
        Ok(())
    }

    fn skip_indefinite_string(&mut self, major: u8, depth: usize) -> Result<()> {
        loop {
            let start = self.pos;
            let b = self.peek()?;
            if b == 0xff {
                self.pos += 1;
                return Ok(());
            }
            if b & 0xe0 != major || b & 0x1f == 31 {
                return Err(Error::message("invalid indefinite string chunk").at(start));
            }
            self.pos += 1;
            let len = self.read_arg(b & 0x1f)?;
            let bytes = self.take_u64(len)?;
            if major == 0x60 && str::from_utf8(bytes).is_err() {
                return Err(Error::new(ErrorKind::InvalidUtf8, start));
            }
            let _ = depth;
        }
    }

    /// Decode a definite or indefinite text string into a `String`.
    #[cfg(feature = "alloc")]
    pub fn text_owned(&mut self) -> Result<String> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0x60 {
            return Err(Error::type_mismatch(Type::Text, type_of_byte(b)).at(start));
        }
        if b & 0x1f != 31 {
            let len = self.read_arg(b & 0x1f)?;
            let bytes = self.take_u64(len)?;
            return String::from_utf8(bytes.to_vec()).map_err(|_| Error::new(ErrorKind::InvalidUtf8, start));
        }
        let mut out = String::new();
        loop {
            let chunk_start = self.pos;
            let b = self.peek()?;
            if b == 0xff {
                self.pos += 1;
                return Ok(out);
            }
            if b & 0xe0 != 0x60 || b & 0x1f == 31 {
                return Err(Error::message("invalid indefinite text chunk").at(chunk_start));
            }
            self.pos += 1;
            let len = self.read_arg(b & 0x1f)?;
            let bytes = self.take_u64(len)?;
            let text = str::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::InvalidUtf8, chunk_start))?;
            out.push_str(text);
        }
    }

    /// Decode a definite or indefinite byte string into a `Vec<u8>`.
    #[cfg(feature = "alloc")]
    pub fn bytes_owned(&mut self) -> Result<Vec<u8>> {
        let start = self.pos;
        let b = self.read()?;
        if b & 0xe0 != 0x40 {
            return Err(Error::type_mismatch(Type::Bytes, type_of_byte(b)).at(start));
        }
        if b & 0x1f != 31 {
            let len = self.read_arg(b & 0x1f)?;
            return Ok(self.take_u64(len)?.to_vec());
        }
        let mut out = Vec::new();
        loop {
            let chunk_start = self.pos;
            let b = self.peek()?;
            if b == 0xff {
                self.pos += 1;
                return Ok(out);
            }
            if b & 0xe0 != 0x40 || b & 0x1f == 31 {
                return Err(Error::message("invalid indefinite bytes chunk").at(chunk_start));
            }
            self.pos += 1;
            let len = self.read_arg(b & 0x1f)?;
            let bytes = self.take_u64(len)?;
            out.extend_from_slice(bytes);
        }
    }

    /// Decode any type implementing [`Decode`].
    #[inline]
    pub fn decode<T: Decode<'b>>(&mut self) -> Result<T> {
        T::decode(self)
    }
}

/// An iterator over the chunks of a byte string.
pub struct BytesIter<'a, 'b> {
    decoder: &'a mut Decoder<'b>,
    remaining: Option<u64>,
    done: bool,
}

impl<'b> Iterator for BytesIter<'_, 'b> {
    type Item = Result<&'b [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if let Some(remaining) = self.remaining {
            self.done = true;
            if remaining == 0 {
                return None;
            }
            return Some(self.decoder.take_u64(remaining));
        }
        match self.decoder.peek() {
            Ok(0xff) => {
                self.decoder.pos += 1;
                self.done = true;
                None
            }
            Ok(b) if b & 0xe0 == 0x40 && b & 0x1f != 31 => {
                let start = self.decoder.pos;
                self.decoder.pos += 1;
                let len = match self.decoder.read_arg(b & 0x1f) {
                    Ok(len) => len,
                    Err(e) => {
                        self.done = true;
                        return Some(Err(e));
                    }
                };
                match self.decoder.take_u64(len) {
                    Ok(bytes) => Some(Ok(bytes)),
                    Err(error) => {
                        self.done = true;
                        Some(Err(error.at(start)))
                    }
                }
            }
            Ok(_) => {
                self.done = true;
                Some(Err(Error::message("invalid indefinite bytes chunk").at(self.decoder.pos)))
            }
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }
}

/// An iterator over the chunks of a text string.
pub struct TextIter<'a, 'b> {
    decoder: &'a mut Decoder<'b>,
    remaining: Option<u64>,
    done: bool,
}

impl<'b> Iterator for TextIter<'_, 'b> {
    type Item = Result<&'b str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if let Some(remaining) = self.remaining {
            self.done = true;
            if remaining == 0 {
                return None;
            }
            return Some(match self.decoder.take_u64(remaining) {
                Ok(bytes) => str::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::InvalidUtf8, self.decoder.pos)),
                Err(e) => Err(e),
            });
        }
        match self.decoder.peek() {
            Ok(0xff) => {
                self.decoder.pos += 1;
                self.done = true;
                None
            }
            Ok(b) if b & 0xe0 == 0x60 && b & 0x1f != 31 => {
                let start = self.decoder.pos;
                self.decoder.pos += 1;
                let len = match self.decoder.read_arg(b & 0x1f) {
                    Ok(len) => len,
                    Err(e) => {
                        self.done = true;
                        return Some(Err(e));
                    }
                };
                match self.decoder.take_u64(len) {
                    Ok(bytes) => Some(str::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::InvalidUtf8, start))),
                    Err(e) => {
                        self.done = true;
                        Some(Err(e))
                    }
                }
            }
            Ok(_) => {
                self.done = true;
                Some(Err(Error::message("invalid indefinite text chunk").at(self.decoder.pos)))
            }
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }
}

/// Decode a value from a CBOR item.
pub trait Decode<'b>: Sized {
    /// Decode this value.
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self>;
}

#[cfg(feature = "alloc")]
impl<'b, T: Decode<'b>> Decode<'b> for Box<T> {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        Ok(Box::new(T::decode(d)?))
    }
}

#[cfg(feature = "alloc")]
impl<'b, T: Decode<'b>> Decode<'b> for alloc::rc::Rc<T> {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        Ok(alloc::rc::Rc::new(T::decode(d)?))
    }
}

#[cfg(feature = "alloc")]
impl<'b, T: Decode<'b>> Decode<'b> for alloc::sync::Arc<T> {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        Ok(alloc::sync::Arc::new(T::decode(d)?))
    }
}

macro_rules! impl_decode_int {
    ($($ty:ty),*) => {
        $(
            impl<'b> Decode<'b> for $ty {
                #[inline]
                fn decode(d: &mut Decoder<'b>) -> Result<Self> {
                    <$ty>::try_from(d.u64()?).map_err(|_| {
                        Error::message("integer does not fit target type").at(d.position())
                    })
                }
            }
        )*
    };
}

impl_decode_int!(u8, u16, u32, u64, usize);

macro_rules! impl_decode_signed {
    ($($ty:ty),*) => {
        $(
            impl<'b> Decode<'b> for $ty {
                #[inline]
                fn decode(d: &mut Decoder<'b>) -> Result<Self> {
                    <$ty>::try_from(d.i64()?).map_err(|_| {
                        Error::message("integer does not fit target type").at(d.position())
                    })
                }
            }
        )*
    };
}

impl_decode_signed!(i8, i16, i32, i64, isize);

impl<'b> Decode<'b> for u128 {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.u128()
    }
}

impl<'b> Decode<'b> for i128 {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.i128()
    }
}

impl<'b> Decode<'b> for f32 {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.f32()
    }
}

impl<'b> Decode<'b> for f64 {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.f64()
    }
}

impl<'b> Decode<'b> for bool {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.bool()
    }
}

impl<'b> Decode<'b> for char {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let text = d.text()?;
        let mut chars = text.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(Error::message("expected a single-character text string").at(d.position())),
        }
    }
}

impl<'b, 'a> Decode<'b> for &'a str
where
    'b: 'a,
{
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.text()
    }
}

#[cfg(feature = "alloc")]
impl<'b> Decode<'b> for String {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.text_owned()
    }
}

impl<'b> Decode<'b> for Integer {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.int()
    }
}

impl<'b> Decode<'b> for Simple {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.simple()
    }
}

impl<'b> Decode<'b> for () {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.null()
    }
}

impl<'b, T: Decode<'b>> Decode<'b> for Option<T> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        match d.datatype()? {
            Type::Null | Type::Undefined => {
                d.skip()?;
                Ok(None)
            }
            _ => Ok(Some(T::decode(d)?)),
        }
    }
}

impl<'b, T: Decode<'b>> Decode<'b> for PhantomData<T> {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.null()?;
        Ok(PhantomData)
    }
}

#[cfg(feature = "alloc")]
impl<'b, T: Decode<'b>> Decode<'b> for Vec<T> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        match d.array()? {
            Some(len) => {
                let mut out = Vec::with_capacity(collection_capacity::<T>(len, d.remaining()));
                for _ in 0..len {
                    out.push(T::decode(d)?);
                }
                Ok(out)
            }
            None => {
                let mut out = Vec::new();
                while !d.is_break() {
                    out.push(T::decode(d)?);
                }
                d.read_break()?;
                Ok(out)
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl<'b, T: Decode<'b>> Decode<'b> for VecDeque<T> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        match d.array()? {
            Some(len) => {
                let mut out = VecDeque::with_capacity(collection_capacity::<T>(len, d.remaining()));
                for _ in 0..len {
                    out.push_back(T::decode(d)?);
                }
                Ok(out)
            }
            None => {
                let mut out = VecDeque::new();
                while !d.is_break() {
                    out.push_back(T::decode(d)?);
                }
                d.read_break()?;
                Ok(out)
            }
        }
    }
}

impl<'b, T: Decode<'b>, const N: usize> Decode<'b> for [T; N] {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        match d.array()? {
            Some(len) => {
                if len != N {
                    return Err(Error::message("array length does not match").at(d.position()));
                }
                let mut out: [Option<T>; N] = core::array::from_fn(|_| None);
                for slot in &mut out {
                    *slot = Some(T::decode(d)?);
                }
                Ok(out.map(|slot| match slot {
                    Some(value) => value,
                    None => unreachable!(),
                }))
            }
            None => {
                let mut out: [Option<T>; N] = core::array::from_fn(|_| None);
                for slot in &mut out {
                    *slot = Some(T::decode(d)?);
                }
                if !d.is_break() {
                    return Err(Error::message("array has too many elements").at(d.position()));
                }
                d.read_break()?;
                Ok(out.map(|slot| match slot {
                    Some(value) => value,
                    None => unreachable!(),
                }))
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl<'b, K: Decode<'b> + Ord, V: Decode<'b>> Decode<'b> for BTreeMap<K, V> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = BTreeMap::new();
        decode_map_into(d, &mut out)?;
        Ok(out)
    }
}

#[cfg(feature = "std")]
impl<'b, K: Decode<'b> + core::hash::Hash + Eq, V: Decode<'b>, S: core::hash::BuildHasher + Default> Decode<'b>
    for std::collections::HashMap<K, V, S>
{
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = std::collections::HashMap::with_hasher(S::default());
        decode_map_into(d, &mut out)?;
        Ok(out)
    }
}

#[cfg(feature = "alloc")]
impl<'b, T: Decode<'b> + Ord> Decode<'b> for BTreeSet<T> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = BTreeSet::new();
        match d.array()? {
            Some(len) => {
                for _ in 0..len {
                    out.insert(T::decode(d)?);
                }
            }
            None => {
                while !d.is_break() {
                    out.insert(T::decode(d)?);
                }
                d.read_break()?;
            }
        }
        Ok(out)
    }
}

#[cfg(feature = "std")]
impl<'b, T: Decode<'b> + core::hash::Hash + Eq, S: core::hash::BuildHasher + Default> Decode<'b>
    for std::collections::HashSet<T, S>
{
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = std::collections::HashSet::with_hasher(S::default());
        match d.array()? {
            Some(len) => {
                for _ in 0..len {
                    out.insert(T::decode(d)?);
                }
            }
            None => {
                while !d.is_break() {
                    out.insert(T::decode(d)?);
                }
                d.read_break()?;
            }
        }
        Ok(out)
    }
}

#[cfg(feature = "alloc")]
fn decode_map_into<'b, K, V, M>(d: &mut Decoder<'b>, out: &mut M) -> Result<()>
where
    K: Decode<'b>,
    V: Decode<'b>,
    M: MapInsert<K, V>,
{
    let duplicate = d.options().duplicate_keys;
    match d.map()? {
        Some(len) => {
            for _ in 0..len {
                let key = K::decode(d)?;
                let value = V::decode(d)?;
                insert_map(out, key, value, duplicate)?;
            }
        }
        None => loop {
            if d.is_break() {
                d.read_break()?;
                break;
            }
            let key = K::decode(d)?;
            if d.is_break() {
                return Err(Error::message("indefinite map has an odd number of items").at(d.position()));
            }
            let value = V::decode(d)?;
            insert_map(out, key, value, duplicate)?;
        },
    }
    Ok(())
}

#[cfg(feature = "alloc")]
trait MapInsert<K, V> {
    fn contains(&self, key: &K) -> bool;
    fn insert(&mut self, key: K, value: V);
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> MapInsert<K, V> for BTreeMap<K, V> {
    fn contains(&self, key: &K) -> bool {
        BTreeMap::contains_key(self, key)
    }
    fn insert(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
}

#[cfg(feature = "std")]
impl<K: core::hash::Hash + Eq, V, S: core::hash::BuildHasher> MapInsert<K, V> for std::collections::HashMap<K, V, S> {
    fn contains(&self, key: &K) -> bool {
        std::collections::HashMap::contains_key(self, key)
    }
    fn insert(&mut self, key: K, value: V) {
        std::collections::HashMap::insert(self, key, value);
    }
}

#[cfg(feature = "alloc")]
fn insert_map<M, K, V>(out: &mut M, key: K, value: V, policy: DuplicateKeys) -> Result<()>
where
    M: MapInsert<K, V>,
{
    if out.contains(&key) {
        match policy {
            DuplicateKeys::Error => return Err(Error::duplicate_key()),
            DuplicateKeys::First => return Ok(()),
            DuplicateKeys::Last | DuplicateKeys::Allow => {}
        }
    }
    out.insert(key, value);
    Ok(())
}

macro_rules! impl_tuple_decode {
    ($($name:ident),+) => {
        impl<'b, $($name: Decode<'b>),+> Decode<'b> for ($($name,)+) {
            #[allow(non_snake_case)]
            fn decode(d: &mut Decoder<'b>) -> Result<Self> {
                let len = d.array()?;
                let expected = 0 $(+ { let _ = stringify!($name); 1 })+;
                if let Some(len) = len {
                    if len != expected {
                        return Err(Error::message("array length does not match tuple").at(d.position()));
                    }
                }
                $(let $name = $name::decode(d)?;)+
                if len.is_none() {
                    if !d.is_break() {
                        return Err(Error::message("array has too many elements").at(d.position()));
                    }
                    d.read_break()?;
                }
                Ok(($($name,)+))
            }
        }
    };
}

impl_tuple_decode!(A);
impl_tuple_decode!(A, B);
impl_tuple_decode!(A, B, C);
impl_tuple_decode!(A, B, C, D);
impl_tuple_decode!(A, B, C, D, F);
impl_tuple_decode!(A, B, C, D, F, G);
impl_tuple_decode!(A, B, C, D, F, G, H);
impl_tuple_decode!(A, B, C, D, F, G, H, I);
impl_tuple_decode!(A, B, C, D, F, G, H, I, J);
impl_tuple_decode!(A, B, C, D, F, G, H, I, J, K);
impl_tuple_decode!(A, B, C, D, F, G, H, I, J, K, L);
impl_tuple_decode!(A, B, C, D, F, G, H, I, J, K, L, M);
impl_tuple_decode!(A, B, C, D, F, G, H, I, J, K, L, M, N);
impl_tuple_decode!(A, B, C, D, F, G, H, I, J, K, L, M, N, O);
impl_tuple_decode!(A, B, C, D, F, G, H, I, J, K, L, M, N, O, P);

/// Decode a value from a byte slice, requiring exactly one item.
#[inline]
pub fn from_slice<'b, T: Decode<'b>>(bytes: &'b [u8]) -> Result<T> {
    let mut decoder = Decoder::new(bytes);
    let value = T::decode(&mut decoder)?;
    if !decoder.is_empty() {
        return Err(Error::trailing_data().at(decoder.position()));
    }
    Ok(value)
}

/// Decode a value from a byte slice, returning the unconsumed remainder.
#[inline]
pub fn from_slice_prefix<'b, T: Decode<'b>>(bytes: &'b [u8]) -> Result<(T, &'b [u8])> {
    let mut decoder = Decoder::new(bytes);
    let value = T::decode(&mut decoder)?;
    let pos = decoder.position();
    Ok((value, &bytes[pos..]))
}

/// Decode a value from a byte slice with explicit options.
#[inline]
pub fn from_slice_with_options<'b, T: Decode<'b>>(bytes: &'b [u8], options: Options) -> Result<T> {
    let mut decoder = Decoder::with_options(bytes, options);
    let value = T::decode(&mut decoder)?;
    if !decoder.is_empty() {
        return Err(Error::trailing_data().at(decoder.position()));
    }
    Ok(value)
}

#[inline]
pub(crate) fn type_of_byte(b: u8) -> Type {
    let major = b & 0xe0;
    let ai = b & 0x1f;
    match major {
        0x00 => match ai {
            0..=24 => Type::U8,
            25 => Type::U16,
            26 => Type::U32,
            27 => Type::U64,
            _ => Type::Unknown(b),
        },
        0x20 => match ai {
            0..=24 => Type::I8,
            25 => Type::I16,
            26 => Type::I32,
            27 => Type::I64,
            _ => Type::Unknown(b),
        },
        0x40 => {
            if ai == 31 {
                Type::BytesIndef
            } else {
                Type::Bytes
            }
        }
        0x60 => {
            if ai == 31 {
                Type::TextIndef
            } else {
                Type::Text
            }
        }
        0x80 => {
            if ai == 31 {
                Type::ArrayIndef
            } else {
                Type::Array
            }
        }
        0xa0 => {
            if ai == 31 {
                Type::MapIndef
            } else {
                Type::Map
            }
        }
        0xc0 => Type::Tag,
        _ => match ai {
            0..=19 => Type::Simple,
            20 | 21 => Type::Bool,
            22 => Type::Null,
            23 => Type::Undefined,
            24 => Type::Simple,
            25 => Type::F16,
            26 => Type::F32,
            27 => Type::F64,
            28..=30 => Type::Unknown(b),
            31 => Type::Break,
            _ => unreachable!(),
        },
    }
}
