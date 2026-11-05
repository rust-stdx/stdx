//! Internal CBOR helpers shared by headers, keys, and messages.

use cbor::{Decoder, Encoder, Type, Write};
use heapless::Vec as HeaplessVec;

use crate::{Error, Label, Result, header::HeaderValue};

/// Maximum number of entries accepted in a single header or key map.
///
/// This bounds parsing cost for attacker-controlled maps. It is intentionally
/// small; COSE headers and keys rarely contain more than a handful of
/// parameters. Exceeding it returns [`Error::TooManyEntries`].
pub(crate) const MAX_MAP_ENTRIES: usize = 32;

/// Maximum CBOR nesting depth accepted in opaque COSE values.
///
/// This prevents stack exhaustion and super-linear parsing on hostile input.
/// Exceeding it returns a CBOR depth-limit error.
pub(crate) const MAX_DEPTH: usize = 32;

pub(crate) fn options() -> cbor::Options {
    cbor::Options {
        max_depth: MAX_DEPTH,
        duplicate_keys: cbor::DuplicateKeys::Allow,
        ..Default::default()
    }
}

pub(crate) fn decoder(input: &[u8]) -> Decoder<'_> {
    Decoder::with_options(input, options())
}

/// A bounded writer over a caller-provided slice.
///
/// The encoder writes into this type so encoder failures and cursor overflow
/// use the crate's own [`Error`] instead of a separate encoding error type.
pub(crate) struct Cursor<'a> {
    buffer: &'a mut [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub(crate) fn new(buffer: &'a mut [u8]) -> Self {
        Cursor {
            buffer,
            position: 0,
        }
    }

    #[inline]
    pub(crate) fn position(&self) -> usize {
        self.position
    }
}

impl Write for Cursor<'_> {
    type Error = Error;

    fn write_all(&mut self, bytes: &[u8]) -> core::result::Result<(), Self::Error> {
        let end = self.position.checked_add(bytes.len()).ok_or(Error::Encode)?;
        let target = self.buffer.get_mut(self.position..end).ok_or(Error::Encode)?;
        target.copy_from_slice(bytes);
        self.position = end;
        Ok(())
    }
}

#[inline]
pub(crate) fn cbor_len_uint(value: u64) -> usize {
    match value {
        0..=23 => 1,
        24..=255 => 2,
        256..=65535 => 3,
        65536..=4294967295 => 5,
        _ => 9,
    }
}

#[inline]
pub(crate) fn cbor_len_int(value: i64) -> usize {
    if value >= 0 {
        cbor_len_uint(value as u64)
    } else {
        cbor_len_uint((-1 - value) as u64)
    }
}

#[inline]
pub(crate) fn cbor_len_bytes(len: usize) -> usize {
    cbor_len_uint(len as u64) + len
}

#[inline]
pub(crate) fn cbor_len_str(len: usize) -> usize {
    cbor_len_bytes(len)
}

#[inline]
pub(crate) fn cbor_len_tag(tag: u64) -> usize {
    cbor_len_uint(tag)
}

#[inline]
pub(crate) fn cbor_len_array(len: u64) -> usize {
    cbor_len_uint(len)
}

#[inline]
pub(crate) fn label_len(label: Label<'_>) -> usize {
    match label {
        Label::Int(value) => cbor_len_int(value),
        Label::Text(value) => cbor_len_str(value.len()),
    }
}

#[inline]
pub(crate) fn value_len(value: HeaderValue<'_>) -> usize {
    match value {
        HeaderValue::Int(value) => cbor_len_int(value),
        HeaderValue::Uint(value) => cbor_len_uint(value),
        HeaderValue::Bytes(value) => cbor_len_bytes(value.len()),
        HeaderValue::Text(value) => cbor_len_str(value.len()),
        HeaderValue::Bool(_) => 1,
        HeaderValue::Null => 1,
        HeaderValue::Raw(value) => value.len(),
    }
}

pub(crate) fn write_label<W: Write>(e: &mut Encoder<W>, label: Label<'_>) -> Result<()> {
    match label {
        Label::Int(value) => {
            e.i64(value).map_err(|_| Error::Encode)?;
        }
        Label::Text(value) => {
            e.text(value).map_err(|_| Error::Encode)?;
        }
    }
    Ok(())
}

pub(crate) fn write_value<W: Write>(e: &mut Encoder<W>, value: HeaderValue<'_>) -> Result<()> {
    match value {
        HeaderValue::Int(value) => {
            e.i64(value).map_err(|_| Error::Encode)?;
        }
        HeaderValue::Uint(value) => {
            e.u64(value).map_err(|_| Error::Encode)?;
        }
        HeaderValue::Bytes(value) => {
            e.bytes(value).map_err(|_| Error::Encode)?;
        }
        HeaderValue::Text(value) => {
            e.text(value).map_err(|_| Error::Encode)?;
        }
        HeaderValue::Bool(value) => {
            e.bool(value).map_err(|_| Error::Encode)?;
        }
        HeaderValue::Null => {
            e.null().map_err(|_| Error::Encode)?;
        }
        HeaderValue::Raw(value) => {
            e.writer_mut().write_all(value).map_err(|_| Error::Encode)?;
        }
    }
    Ok(())
}

pub(crate) fn decode_label<'a>(d: &mut Decoder<'a>) -> Result<Label<'a>> {
    match d.datatype()? {
        Type::U8 | Type::U16 | Type::U32 | Type::U64 => {
            let value = d.u64()?;
            let value = i64::try_from(value).map_err(|_| Error::InvalidHeader)?;
            Ok(Label::Int(value))
        }
        Type::I8 | Type::I16 | Type::I32 | Type::I64 => Ok(Label::Int(d.i64()?)),
        Type::Text => Ok(Label::Text(d.text()?)),
        _ => Err(Error::InvalidHeader),
    }
}

#[inline]
pub(crate) fn labels_equal(a: Label<'_>, b: Label<'_>) -> bool {
    match (a, b) {
        (Label::Int(a), Label::Int(b)) => a == b,
        (Label::Text(a), Label::Text(b)) => a == b,
        _ => false,
    }
}

/// Validates that `bytes` is a definite-length map with no duplicate labels.
pub(crate) fn validate_map(bytes: &[u8]) -> Result<usize> {
    if bytes.is_empty() {
        return Ok(0);
    }

    let mut d = decoder(bytes);
    let len = d.map()?.ok_or(Error::IndefiniteLength)?;
    if len > MAX_MAP_ENTRIES {
        return Err(Error::TooManyEntries);
    }

    let mut labels: HeaplessVec<Label<'_>, MAX_MAP_ENTRIES> = HeaplessVec::new();
    for _ in 0..len {
        let label = decode_label(&mut d)?;
        if labels.iter().any(|seen| labels_equal(*seen, label)) {
            return Err(Error::DuplicateLabel);
        }
        labels.push(label).map_err(|_| Error::TooManyEntries)?;
        d.skip()?;
    }

    if d.position() != bytes.len() {
        return Err(Error::TrailingBytes);
    }
    Ok(len)
}

/// Validates that `bytes` is a complete definite-length array.
pub(crate) fn validate_array(bytes: &[u8], min: usize, max: usize) -> Result<usize> {
    let mut d = decoder(bytes);
    let len = d.array()?.ok_or(Error::IndefiniteLength)?;
    if len < min || len > max {
        return Err(Error::InvalidStructure);
    }
    for _ in 0..len {
        d.skip()?;
    }
    if d.position() != bytes.len() {
        return Err(Error::TrailingBytes);
    }
    Ok(len)
}

/// Finds a value in an encoded map and returns its raw CBOR encoding.
pub(crate) fn find_value<'a>(bytes: &'a [u8], wanted: Label<'_>) -> Result<Option<&'a [u8]>> {
    if bytes.is_empty() {
        return Ok(None);
    }

    let mut d = decoder(bytes);
    let len = d.map()?.ok_or(Error::IndefiniteLength)?;
    for _ in 0..len {
        let label = decode_label(&mut d)?;
        let start = d.position();
        d.skip()?;
        if labels_equal(label, wanted) {
            return Ok(Some(&bytes[start..d.position()]));
        }
    }
    Ok(None)
}

/// Decodes a single CBOR integer.
pub(crate) fn decode_i64(bytes: &[u8]) -> Result<i64> {
    let mut d = decoder(bytes);
    let value = d.i64()?;
    if d.position() != bytes.len() {
        return Err(Error::TrailingBytes);
    }
    Ok(value)
}

/// Decodes a single CBOR byte string.
pub(crate) fn decode_bytes(bytes: &[u8]) -> Result<&[u8]> {
    let mut d = decoder(bytes);
    let value = d.bytes()?;
    if d.position() != bytes.len() {
        return Err(Error::TrailingBytes);
    }
    Ok(value)
}

/// Returns the raw encoded value at the decoder's current position and advances
/// past it.
pub(crate) fn take_raw<'a>(input: &'a [u8], d: &mut Decoder<'a>) -> Result<&'a [u8]> {
    let start = d.position();
    d.skip()?;
    Ok(&input[start..d.position()])
}
