//! Wire-format constants and low-level helpers.

use alloc::{collections::BTreeSet, vec::Vec};

use cbor::{Decoder, Encoder};

use crate::{DecodeOptions, Error, Result};

/// The only envelope version understood by this build.
pub(crate) const VERSION: u64 = 1;

/// Duplicate-key tracker that avoids allocating for keys below 64.
#[doc(hidden)]
pub struct SeenKeys {
    low: u64,
    high: BTreeSet<u64>,
}

impl SeenKeys {
    /// Create an empty tracker.
    pub fn new() -> Self {
        SeenKeys {
            low: 0,
            high: BTreeSet::new(),
        }
    }

    /// Insert a key and return `true` if it was not present.
    pub fn insert(&mut self, key: u64) -> bool {
        if key < 64 {
            let bit = 1u64 << key;
            if self.low & bit != 0 {
                false
            } else {
                self.low |= bit;
                true
            }
        } else {
            self.high.insert(key)
        }
    }
}

impl Default for SeenKeys {
    fn default() -> Self {
        Self::new()
    }
}

/// Encoding identifier for a struct column.
pub const ENC_STRUCT: u64 = 1;
/// Encoding identifier for an enum column.
pub const ENC_ENUM: u64 = 2;
pub(crate) const ENC_PACKED: u64 = 3;
pub(crate) const ENC_FOR: u64 = 4;
pub(crate) const ENC_BITPACK: u64 = 5;
pub(crate) const ENC_VARINT: u64 = 6;
pub(crate) const ENC_RLE: u64 = 7;
pub(crate) const ENC_DICTIONARY: u64 = 8;
pub(crate) const ENC_STRING_LENGTHS: u64 = 9;
pub(crate) const ENC_STRING_OFFSETS: u64 = 10;
pub(crate) const ENC_NULLABLE: u64 = 11;
pub(crate) const ENC_LIST_LENGTHS: u64 = 12;
pub(crate) const ENC_LIST_OFFSETS: u64 = 13;
pub(crate) const ENC_ALL_NULL: u64 = 14;
pub(crate) const ENC_DELTA_RLE: u64 = 15;
pub(crate) const ENC_DELTA_OF_DELTA: u64 = 16;

/// A requested or discovered column encoding.
///
/// [`Encoding::Auto`] is a hint, not a wire value. The other variants are
/// mostly useful in `#[columnar(encoding = "...")]` attributes to override the
/// default chosen for a Rust type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Encoding {
    /// Let the type choose its encoding.
    ///
    /// Integer, float, boolean, string and byte columns are scanned for their
    /// range and runs. On large columns, first and second differences are
    /// estimated from a bounded sample to keep encoding fast. [`Encoding::Auto`]
    /// keeps packed/FOR lengths or bit-packing unless RLE or a delta encoding
    /// is estimated to be at least 5% smaller. Columns with fewer than 32
    /// values always use the simple type default.
    Auto,
    /// Fixed-width little-endian values.
    Packed,
    /// Frame-of-reference plus bit-packing.
    ForBitpack,
    /// Plain bit-packing with a type-defined width.
    Bitpack,
    /// Unsigned LEB128 (signed values are zig-zag encoded).
    Varint,
    /// Run-length encoding.
    Rle,
    /// Delta plus run-length encoding.
    ///
    /// Each value is stored as its difference from the previous value (the
    /// first difference is relative to zero) and equal consecutive differences
    /// share one run. This is useful for identifiers, counters and timestamps
    /// that grow or shrink by the same step. Only integer columns up to 64 bits
    /// support it.
    DeltaRle,
    /// Delta-of-delta encoding.
    ///
    /// The first value is stored absolutely. The second value stores its first
    /// difference relative to a zero previous delta, and every following value
    /// stores a second difference. This is useful when the first differences
    /// change often but the second differences stay small, such as jittered
    /// timestamps. Only integer columns up to 64 bits support it.
    DeltaOfDelta,
    /// Dictionary encoding.
    Dictionary,
    /// Varint lengths followed by UTF-8/byte data.
    StringLengths,
    /// Monotonic offsets followed by UTF-8/byte data.
    StringOffsets,
    /// Validity bitmap followed by an inner column.
    Nullable,
    /// Varint element lengths followed by a child column.
    ListLengths,
    /// Monotonic element offsets followed by a child column.
    ListOffsets,
    /// Child columns for a struct.
    Struct,
    /// Tag column followed by dense variant columns.
    Enum,
    /// Every value is null.
    AllNull,
}

pub fn encoder() -> Encoder<Vec<u8>> {
    Encoder::new(Vec::new())
}

pub fn encoder_with_capacity(capacity: usize) -> Encoder<Vec<u8>> {
    Encoder::new(Vec::with_capacity(capacity))
}

pub fn finish(encoder: Encoder<Vec<u8>>) -> Vec<u8> {
    encoder.into_inner()
}

pub fn decoder(bytes: &[u8], options: DecodeOptions) -> Decoder<'_> {
    Decoder::with_options(bytes, options.cbor_options())
}

pub fn write_header(out: &mut Encoder<Vec<u8>>, rows: usize) -> Result<()> {
    out.array(2)?;
    out.u64(VERSION)?;
    out.u64(rows as u64)?;
    Ok(())
}

pub fn read_header(d: &mut Decoder<'_>, options: DecodeOptions) -> Result<usize> {
    let len = d.array()?.ok_or(Error::InvalidHeader)?;
    if len != 2 {
        return Err(Error::InvalidHeader);
    }
    let version = d.u64()?;
    if version != VERSION {
        return Err(Error::UnsupportedVersion(version));
    }
    let rows = d.u64()?;
    let rows = usize::try_from(rows).map_err(|_| Error::LimitExceeded("rows"))?;
    if rows > options.max_rows {
        return Err(Error::LimitExceeded("rows"));
    }
    Ok(rows)
}

pub fn write_column(out: &mut Encoder<Vec<u8>>, encoding: u64) -> Result<()> {
    out.array(2)?;
    out.u64(encoding)?;
    Ok(())
}

pub fn read_column(d: &mut Decoder<'_>) -> Result<u64> {
    let len = d.array()?.ok_or(Error::InvalidEncoding(0))?;
    if len != 2 {
        return Err(Error::InvalidEncoding(len as u64));
    }
    Ok(d.u64()?)
}

pub fn read_array_len(d: &mut Decoder<'_>, expected: usize) -> Result<()> {
    let len = d.array()?.ok_or(Error::InvalidHeader)?;
    if len != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: len,
        });
    }
    Ok(())
}

pub fn read_map_len(d: &mut Decoder<'_>) -> Result<usize> {
    d.map()?.ok_or(Error::InvalidHeader)
}

pub(crate) fn write_varint(out: &mut Vec<u8>, mut value: u128) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

/// Number of bytes `value` occupies as an unsigned LEB128 varint.
#[inline]
pub(crate) fn varint_len(value: u128) -> usize {
    let bits = if value == 0 { 0 } else { 128 - value.leading_zeros() };
    (bits.max(1) as usize).div_ceil(7)
}

pub(crate) fn read_varint(data: &[u8], pos: &mut usize) -> Result<u128> {
    let mut value = 0u128;
    let mut shift = 0u32;
    loop {
        let byte = *data.get(*pos).ok_or(Error::InvalidVarint)?;
        *pos += 1;
        let payload = u128::from(byte & 0x7f);
        if shift == 126 {
            // Only two payload bits fit in a u128.
            if payload > 0x03 || byte & 0x80 != 0 || payload == 0 {
                return Err(Error::InvalidVarint);
            }
            return Ok(value | (payload << 126));
        }
        if shift > 126 {
            return Err(Error::InvalidVarint);
        }
        value |= payload << shift;
        if byte & 0x80 == 0 {
            // A multi-byte varint whose final payload is zero is overlong: the
            // same value has a shorter encoding.
            if shift != 0 && payload == 0 {
                return Err(Error::InvalidVarint);
            }
            return Ok(value);
        }
        shift += 7;
    }
}

pub(crate) fn zigzag(value: i128) -> u128 {
    ((value << 1) ^ (value >> 127)) as u128
}

pub(crate) fn unzigzag(value: u128) -> i128 {
    ((value >> 1) as i128) ^ -((value & 1) as i128)
}
