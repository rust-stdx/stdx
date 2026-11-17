//! Implementations of the column encodings for Rust types.
//!
//! Every type that can appear as a struct field implements [`Field`]. The
//! derive macro uses these implementations to turn `&[Struct]` into columns and
//! back.

use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::marker::PhantomData;

use cbor::{Decoder, Encoder};

use crate::{
    DecodeOptions, Encoding, Error, Result,
    bitpack::{BitReader, BitWriter, bits_needed, count_valid, pack_u64, packed_len},
    format::*,
};

/// A type that can be encoded as a column.
///
/// Decoding is provided separately by [`FieldDecode`] so that borrowed types
/// can tie their lifetime to the input buffer. Most users do not implement
/// either trait by hand; `#[derive(Columnar)]` generates them for structs and
/// enums.
pub trait Field: Sized {
    /// Encode a sequence of values as one column.
    ///
    /// `hint` is [`Encoding::Auto`] unless the field has a
    /// `#[columnar(encoding = "...")]` attribute.
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a;
}

/// A field that can be decoded from a column, borrowing from `d` when needed.
///
/// Owned types implement this for every `'de`. Borrowed types such as
/// `Cow<'de, [u8]>` and `Cow<'de, str>` implement it for their matching
/// lifetime.
pub trait FieldDecode<'de>: Sized {
    /// Decode one column with exactly `rows` logical values.
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>>;

    /// Decode exactly `rows` values in order, passing each one to `sink`.
    ///
    /// The default implementation decodes a vector and forwards its values.
    /// Types that can decode incrementally override this method so callers can
    /// avoid an intermediate vector, for example when decoding list elements.
    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        for value in Self::decode_column(d, rows, options)? {
            sink(value)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Shared payload helpers
// ---------------------------------------------------------------------------

pub(crate) fn packed_bytes<'a>(d: &mut Decoder<'a>, rows: usize, width: usize) -> Result<&'a [u8]> {
    let data = d.bytes()?;
    let expected = rows.checked_mul(width).ok_or(Error::LengthOverflow)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    Ok(data)
}

fn encode_packed_u64<I>(values: I, width: usize, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u64>,
{
    let (lower, _) = values.size_hint();
    let mut buf = Vec::with_capacity(lower.saturating_mul(width));
    for value in values {
        buf.extend_from_slice(&value.to_le_bytes()[..width]);
    }
    write_column(out, ENC_PACKED)?;
    out.bytes(&buf)?;
    Ok(())
}

fn encode_packed_u128<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u128>,
{
    let (lower, _) = values.size_hint();
    let mut buf = Vec::with_capacity(lower.saturating_mul(16));
    for value in values {
        buf.extend_from_slice(&value.to_le_bytes());
    }
    write_column(out, ENC_PACKED)?;
    out.bytes(&buf)?;
    Ok(())
}

fn encode_for_u64<I>(values: I, base: u64, width: u8, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u64>,
{
    write_column(out, ENC_FOR)?;
    out.array(3)?;
    out.u64(base)?;
    out.u64(u64::from(width))?;
    let packed = pack_u64(values.map(|value| value.wrapping_sub(base)), width);
    out.bytes(&packed)?;
    Ok(())
}

fn encode_for_i64<I>(values: I, base: i64, width: u8, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i64>,
{
    write_column(out, ENC_FOR)?;
    out.array(3)?;
    out.i64(base)?;
    out.u64(u64::from(width))?;
    let packed = pack_u64(values.map(|value| (value as i128 - base as i128) as u128 as u64), width);
    out.bytes(&packed)?;
    Ok(())
}

fn decode_for_unsigned_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<u128>,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 3)?;
    let base = d.u64()?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width > 64 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let data = d.bytes()?;
    let expected = packed_len(rows, width as u8)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    let mut reader = BitReader::new(data);
    for _ in 0..rows {
        let offset = u128::from(reader.read(width as u8)?);
        let value = u128::from(base).wrapping_add(offset);
        sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
    }
    Ok(())
}

fn decode_for_signed_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 3)?;
    let base = d.i64()?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width > 64 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let data = d.bytes()?;
    let expected = packed_len(rows, width as u8)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    let mut reader = BitReader::new(data);
    for _ in 0..rows {
        let offset = reader.read(width as u8)?;
        let value = base as i128 + offset as i128;
        sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
    }
    Ok(())
}

fn encode_varint_unsigned<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = u128>,
{
    let (lower, _) = values.size_hint();
    let mut buf = Vec::with_capacity(lower);
    for value in values {
        write_varint(&mut buf, value);
    }
    write_column(out, ENC_VARINT)?;
    out.bytes(&buf)?;
    Ok(())
}

fn encode_varint_signed<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i128>,
{
    let (lower, _) = values.size_hint();
    let mut buf = Vec::with_capacity(lower);
    for value in values {
        write_varint(&mut buf, zigzag(value));
    }
    write_column(out, ENC_VARINT)?;
    out.bytes(&buf)?;
    Ok(())
}

fn decode_varint_unsigned_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<u128>,
    F: FnMut(T) -> Result<()>,
{
    let data = d.bytes()?;
    let mut pos = 0;
    for _ in 0..rows {
        let value = read_varint(data, &mut pos)?;
        sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
    }
    if pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn decode_varint_signed_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, mut sink: F) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    let data = d.bytes()?;
    let mut pos = 0;
    for _ in 0..rows {
        let value = unzigzag(read_varint(data, &mut pos)?);
        sink(T::try_from(value).map_err(|_| Error::LengthOverflow)?)?;
    }
    if pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: data.len(),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// RLE and dictionary, shared by leaf types
// ---------------------------------------------------------------------------

fn encode_rle<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>, equal: impl Fn(&T, &T) -> bool) -> Result<()>
where
    T: Field + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut run_values: Vec<&'a T> = Vec::with_capacity(lower.min(4096));
    let mut lengths: Vec<u32> = Vec::with_capacity(lower.min(4096));
    for value in values {
        if let Some(last) = run_values.last()
            && equal(last, value)
        {
            let last_len = lengths.last_mut().expect("run length exists");
            *last_len = last_len.checked_add(1).ok_or(Error::LengthOverflow)?;
            continue;
        }
        run_values.push(value);
        lengths.push(1);
    }
    let mut length_buf = Vec::with_capacity(lengths.len());
    for length in lengths {
        write_varint(&mut length_buf, u128::from(length));
    }
    write_column(out, ENC_RLE)?;
    out.array(2)?;
    out.bytes(&length_buf)?;
    T::encode_column(run_values.iter().copied(), out, Encoding::Auto)?;
    Ok(())
}

fn decode_rle_into<'de, T, F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
where
    T: FieldDecode<'de> + Clone,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut pos = 0;
    let mut total = 0usize;
    let mut run_count = 0usize;
    while pos < length_data.len() {
        if run_count >= options.max_list_len {
            return Err(Error::LimitExceeded("rle runs"));
        }
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if length == 0 {
            return Err(Error::ZeroRun);
        }
        total = total.checked_add(length as usize).ok_or(Error::LengthOverflow)?;
        if total > rows {
            return Err(Error::LengthMismatch {
                expected: rows,
                actual: total,
            });
        }
        run_count += 1;
    }
    if total != rows {
        return Err(Error::LengthMismatch {
            expected: rows,
            actual: total,
        });
    }
    let mut pos = 0;
    for value in T::decode_column(d, run_count, options)? {
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        for _ in 0..length {
            sink(value.clone())?;
        }
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Delta RLE and delta-of-delta, shared by integer types
// ---------------------------------------------------------------------------

fn encode_delta_rle<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i128>,
{
    let (lower, _) = values.size_hint();
    let mut lengths: Vec<u32> = Vec::with_capacity(lower.min(4096));
    let mut delta_buf = Vec::with_capacity(lower);
    let mut previous = 0i128;
    let mut last_delta = None;
    for value in values {
        let delta = value.checked_sub(previous).ok_or(Error::DeltaOverflow)?;
        previous = value;
        if last_delta == Some(delta) {
            let last_length = lengths.last_mut().expect("delta run length exists");
            *last_length = last_length.checked_add(1).ok_or(Error::LengthOverflow)?;
            continue;
        }
        last_delta = Some(delta);
        lengths.push(1);
        write_varint(&mut delta_buf, zigzag(delta));
    }
    let mut length_buf = Vec::with_capacity(lengths.len());
    for length in lengths {
        write_varint(&mut length_buf, u128::from(length));
    }
    write_column(out, ENC_DELTA_RLE)?;
    out.array(2)?;
    out.bytes(&length_buf)?;
    write_column(out, ENC_VARINT)?;
    out.bytes(&delta_buf)?;
    Ok(())
}

fn decode_delta_rle_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut pos = 0;
    let mut total = 0usize;
    let mut run_count = 0usize;
    while pos < length_data.len() {
        if run_count >= options.max_list_len {
            return Err(Error::LimitExceeded("delta rle runs"));
        }
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if length == 0 {
            return Err(Error::ZeroRun);
        }
        total = total.checked_add(length as usize).ok_or(Error::LengthOverflow)?;
        if total > rows {
            return Err(Error::LengthMismatch {
                expected: rows,
                actual: total,
            });
        }
        run_count += 1;
    }
    if total != rows {
        return Err(Error::LengthMismatch {
            expected: rows,
            actual: total,
        });
    }
    let encoding = read_column(d)?;
    if encoding != ENC_VARINT {
        return Err(Error::InvalidEncoding(encoding));
    }
    let delta_data = d.bytes()?;
    let mut pos = 0;
    let mut delta_pos = 0;
    let mut absolute = 0i128;
    for _ in 0..run_count {
        let length = u32::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        let delta = unzigzag(read_varint(delta_data, &mut delta_pos)?);
        for _ in 0..length {
            absolute = absolute.checked_add(delta).ok_or(Error::DeltaOverflow)?;
            sink(T::try_from(absolute).map_err(|_| Error::DeltaOverflow)?)?;
        }
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if delta_pos != delta_data.len() {
        return Err(Error::LengthMismatch {
            expected: delta_pos,
            actual: delta_data.len(),
        });
    }
    Ok(())
}

fn encode_delta_of_delta<I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = i128>,
{
    let (lower, _) = values.size_hint();
    let mut buf = Vec::with_capacity(lower);
    let mut previous_value = None;
    let mut previous_delta = 0i128;
    for value in values {
        let encoded = if let Some(previous) = previous_value {
            let delta = value.checked_sub(previous).ok_or(Error::DeltaOverflow)?;
            let delta_of_delta = delta.checked_sub(previous_delta).ok_or(Error::DeltaOverflow)?;
            previous_delta = delta;
            delta_of_delta
        } else {
            value
        };
        previous_value = Some(value);
        write_varint(&mut buf, zigzag(encoded));
    }
    write_column(out, ENC_DELTA_OF_DELTA)?;
    write_column(out, ENC_VARINT)?;
    out.bytes(&buf)?;
    Ok(())
}

fn decode_delta_of_delta_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    _options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: TryFrom<i128>,
    F: FnMut(T) -> Result<()>,
{
    let encoding = read_column(d)?;
    if encoding != ENC_VARINT {
        return Err(Error::InvalidEncoding(encoding));
    }
    let data = d.bytes()?;
    let mut pos = 0;
    let mut previous_value = 0i128;
    let mut previous_delta = 0i128;
    for index in 0..rows {
        let encoded = unzigzag(read_varint(data, &mut pos)?);
        let value = if index == 0 {
            previous_value = encoded;
            encoded
        } else {
            previous_delta = previous_delta.checked_add(encoded).ok_or(Error::DeltaOverflow)?;
            previous_value = previous_value.checked_add(previous_delta).ok_or(Error::DeltaOverflow)?;
            previous_value
        };
        sink(T::try_from(value).map_err(|_| Error::DeltaOverflow)?)?;
    }
    if pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn encode_dictionary<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    T: Field + Ord + 'a,
    I: Iterator<Item = &'a T> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut dict: Vec<&T> = Vec::with_capacity(lower);
    dict.extend(values.clone());
    dict.sort_unstable();
    dict.dedup();
    let width = bits_needed(u128::from(dict.len().saturating_sub(1) as u64));
    let indices = {
        let mut writer = BitWriter::with_capacity(packed_len(lower, width).unwrap_or(0));
        for value in values {
            let index = dict.binary_search(&value).map_err(|_| Error::InvalidBitpackedValue)?;
            writer.write(index as u64, width);
        }
        writer.finish()
    };
    write_column(out, ENC_DICTIONARY)?;
    out.array(3)?;
    out.u64(dict.len() as u64)?;
    T::encode_column(dict.iter().copied(), out, Encoding::Auto)?;
    out.bytes(&indices)?;
    Ok(())
}

fn decode_dictionary_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: FieldDecode<'de> + Clone,
    F: FnMut(T) -> Result<()>,
{
    read_array_len(d, 3)?;
    let dict_len = d.u64()?;
    let dict_len = usize::try_from(dict_len).map_err(|_| Error::LengthOverflow)?;
    if dict_len > options.max_list_len {
        return Err(Error::LimitExceeded("dictionary length"));
    }
    if dict_len == 0 && rows != 0 {
        return Err(Error::DictionaryIndex {
            index: 0,
            len: 0,
        });
    }
    let dict = T::decode_column(d, dict_len, options)?;
    let indices = d.bytes()?;
    let width = bits_needed(u128::from(dict_len.saturating_sub(1) as u64));
    let expected = packed_len(rows, width)?;
    if indices.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: indices.len(),
        });
    }
    let mut reader = BitReader::new(indices);
    for _ in 0..rows {
        let index = u32::try_from(reader.read(width)?).map_err(|_| Error::InvalidBitpackedValue)?;
        let value = dict.get(index as usize).ok_or(Error::DictionaryIndex {
            index,
            len: dict.len(),
        })?;
        sink(value.clone())?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Integer macros
// ---------------------------------------------------------------------------

macro_rules! impl_unsigned_int {
    ($ty:ty, $bytes:expr, $bits:expr) => {
        impl Field for $ty {
            fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
            where
                I: Iterator<Item = &'a Self> + Clone,
                Self: 'a,
            {
                match hint {
                    Encoding::Auto | Encoding::ForBitpack => {
                        let mut min = <$ty>::MAX;
                        let mut max = <$ty>::MIN;
                        let mut any = false;
                        for value in values.clone() {
                            any = true;
                            min = min.min(*value);
                            max = max.max(*value);
                        }
                        let (min, width) = if any {
                            (min, bits_needed(u128::from(max.wrapping_sub(min))))
                        } else {
                            (0, 0)
                        };
                        if hint == Encoding::Auto && width == $bits {
                            encode_packed_u64(values.map(|value| u64::from(*value)), $bytes, out)
                        } else if width == 0 {
                            encode_for_u64(values.map(|value| u64::from(*value)), u64::from(min), 0, out)
                        } else {
                            encode_for_u64(values.map(|value| u64::from(*value)), u64::from(min), width, out)
                        }
                    }
                    Encoding::Packed => encode_packed_u64(values.map(|value| u64::from(*value)), $bytes, out),
                    Encoding::Varint => encode_varint_unsigned(values.map(|value| u128::from(*value)), out),
                    Encoding::Rle => encode_rle(values, out, |a, b| a == b),
                    Encoding::DeltaRle => encode_delta_rle(values.map(|value| i128::from(*value)), out),
                    Encoding::DeltaOfDelta => encode_delta_of_delta(values.map(|value| i128::from(*value)), out),
                    Encoding::Dictionary => encode_dictionary(values, out),
                    other => Err(Error::UnsupportedEncoding(other)),
                }
            }
        }

        impl<'de> FieldDecode<'de> for $ty {
            fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
                let mut out = Vec::with_capacity(rows);
                Self::decode_column_into(d, rows, options, |value| {
                    out.push(value);
                    Ok(())
                })?;
                Ok(out)
            }

            fn decode_column_into<F>(
                d: &mut Decoder<'de>,
                rows: usize,
                options: &DecodeOptions,
                mut sink: F,
            ) -> Result<()>
            where
                F: FnMut(Self) -> Result<()>,
            {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => {
                        let data = packed_bytes(d, rows, $bytes)?;
                        for chunk in data.chunks_exact($bytes) {
                            let mut bytes = [0u8; 8];
                            bytes[..$bytes].copy_from_slice(chunk);
                            let value = u64::from_le_bytes(bytes);
                            sink(value as $ty)?;
                        }
                        Ok(())
                    }
                    ENC_FOR => decode_for_unsigned_into::<$ty, _>(d, rows, sink),
                    ENC_VARINT => decode_varint_unsigned_into::<$ty, _>(d, rows, sink),
                    ENC_RLE => decode_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_RLE => decode_delta_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_OF_DELTA => decode_delta_of_delta_into::<$ty, _>(d, rows, options, sink),
                    ENC_DICTIONARY => decode_dictionary_into::<$ty, _>(d, rows, options, sink),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_unsigned_int!(u8, 1, 8);
impl_unsigned_int!(u16, 2, 16);
impl_unsigned_int!(u32, 4, 32);
impl_unsigned_int!(u64, 8, 64);
macro_rules! impl_signed_int {
    ($ty:ty, $bytes:expr, $bits:expr) => {
        impl Field for $ty {
            fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
            where
                I: Iterator<Item = &'a Self> + Clone,
                Self: 'a,
            {
                match hint {
                    Encoding::Auto | Encoding::ForBitpack => {
                        let mut min = <$ty>::MAX;
                        let mut max = <$ty>::MIN;
                        let mut any = false;
                        for value in values.clone() {
                            any = true;
                            min = min.min(*value);
                            max = max.max(*value);
                        }
                        let (min, width) = if any {
                            (min, bits_needed((max as i128 - min as i128) as u128))
                        } else {
                            (0, 0)
                        };
                        if hint == Encoding::Auto && width == $bits {
                            encode_packed_u64(values.map(|value| *value as u64), $bytes, out)
                        } else {
                            encode_for_i64(values.map(|value| i64::from(*value)), i64::from(min), width, out)
                        }
                    }
                    Encoding::Packed => encode_packed_u64(values.map(|value| *value as u64), $bytes, out),
                    Encoding::Varint => encode_varint_signed(values.map(|value| i128::from(*value)), out),
                    Encoding::Rle => encode_rle(values, out, |a, b| a == b),
                    Encoding::DeltaRle => encode_delta_rle(values.map(|value| i128::from(*value)), out),
                    Encoding::DeltaOfDelta => encode_delta_of_delta(values.map(|value| i128::from(*value)), out),
                    Encoding::Dictionary => encode_dictionary(values, out),
                    other => Err(Error::UnsupportedEncoding(other)),
                }
            }
        }

        impl<'de> FieldDecode<'de> for $ty {
            fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
                let mut out = Vec::with_capacity(rows);
                Self::decode_column_into(d, rows, options, |value| {
                    out.push(value);
                    Ok(())
                })?;
                Ok(out)
            }

            fn decode_column_into<F>(
                d: &mut Decoder<'de>,
                rows: usize,
                options: &DecodeOptions,
                mut sink: F,
            ) -> Result<()>
            where
                F: FnMut(Self) -> Result<()>,
            {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => {
                        let data = packed_bytes(d, rows, $bytes)?;
                        for chunk in data.chunks_exact($bytes) {
                            let mut bytes = [0u8; 8];
                            bytes[..$bytes].copy_from_slice(chunk);
                            let value = u64::from_le_bytes(bytes);
                            sink(value as $ty)?;
                        }
                        Ok(())
                    }
                    ENC_FOR => decode_for_signed_into::<$ty, _>(d, rows, sink),
                    ENC_VARINT => decode_varint_signed_into::<$ty, _>(d, rows, sink),
                    ENC_RLE => decode_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_RLE => decode_delta_rle_into::<$ty, _>(d, rows, options, sink),
                    ENC_DELTA_OF_DELTA => decode_delta_of_delta_into::<$ty, _>(d, rows, options, sink),
                    ENC_DICTIONARY => decode_dictionary_into::<$ty, _>(d, rows, options, sink),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_signed_int!(i8, 1, 8);
impl_signed_int!(i16, 2, 16);
impl_signed_int!(i32, 4, 32);
impl_signed_int!(i64, 8, 64);

// usize/isize are encoded as u64/i64 so the format is portable.
impl Field for usize {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        let converted: Vec<u64> = values.map(|value| *value as u64).collect();
        u64::encode_column(converted.iter(), out, hint)
    }
}

impl<'de> FieldDecode<'de> for usize {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        u64::decode_column_into(d, rows, options, |value| {
            sink(usize::try_from(value).map_err(|_| Error::LengthOverflow)?)
        })
    }
}

impl Field for isize {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        let converted: Vec<i64> = values.map(|value| *value as i64).collect();
        i64::encode_column(converted.iter(), out, hint)
    }
}

impl<'de> FieldDecode<'de> for isize {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        i64::decode_column_into(d, rows, options, |value| {
            sink(isize::try_from(value).map_err(|_| Error::LengthOverflow)?)
        })
    }
}

impl Field for u128 {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::Packed => encode_packed_u128(values.copied(), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for u128 {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_PACKED => {
                let data = packed_bytes(d, rows, 16)?;
                for chunk in data.as_chunks::<16>().0 {
                    sink(u128::from_le_bytes(*chunk))?;
                }
                Ok(())
            }
            ENC_RLE => decode_rle_into::<u128, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<u128, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

impl Field for i128 {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::Packed => encode_packed_u128(values.map(|value| *value as u128), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for i128 {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_PACKED => {
                let data = packed_bytes(d, rows, 16)?;
                for chunk in data.as_chunks::<16>().0 {
                    sink(i128::from_le_bytes(*chunk))?;
                }
                Ok(())
            }
            ENC_RLE => decode_rle_into::<i128, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<i128, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// bool, floats, char
// ---------------------------------------------------------------------------

impl Field for bool {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::Bitpack => {
                write_column(out, ENC_BITPACK)?;
                let packed = pack_u64(values.map(|value| u64::from(*value)), 1);
                out.bytes(&packed)?;
                Ok(())
            }
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for bool {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_BITPACK => {
                let data = d.bytes()?;
                let expected = packed_len(rows, 1)?;
                if data.len() != expected {
                    return Err(Error::LengthMismatch {
                        expected,
                        actual: data.len(),
                    });
                }
                let mut reader = BitReader::new(data);
                for _ in 0..rows {
                    let value = reader.read(1)?;
                    if value > 1 {
                        return Err(Error::InvalidBitpackedValue);
                    }
                    sink(value == 1)?;
                }
                Ok(())
            }
            ENC_RLE => decode_rle_into::<bool, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

macro_rules! impl_float {
    ($ty:ty, $bits:ty, $width:expr) => {
        impl Field for $ty {
            fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
            where
                I: Iterator<Item = &'a Self> + Clone,
                Self: 'a,
            {
                match hint {
                    Encoding::Auto | Encoding::Packed => {
                        let (lower, _) = values.size_hint();
                        let mut buf = Vec::with_capacity(lower.saturating_mul($width));
                        for value in values {
                            buf.extend_from_slice(&value.to_bits().to_le_bytes());
                        }
                        write_column(out, ENC_PACKED)?;
                        out.bytes(&buf)?;
                        Ok(())
                    }
                    Encoding::Rle => encode_rle(values, out, |a, b| a.to_bits() == b.to_bits()),
                    other => Err(Error::UnsupportedEncoding(other)),
                }
            }
        }

        impl<'de> FieldDecode<'de> for $ty {
            fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
                let mut out = Vec::with_capacity(rows);
                Self::decode_column_into(d, rows, options, |value| {
                    out.push(value);
                    Ok(())
                })?;
                Ok(out)
            }

            fn decode_column_into<F>(
                d: &mut Decoder<'de>,
                rows: usize,
                options: &DecodeOptions,
                mut sink: F,
            ) -> Result<()>
            where
                F: FnMut(Self) -> Result<()>,
            {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => {
                        let data = packed_bytes(d, rows, $width)?;
                        for chunk in data.chunks_exact($width) {
                            let bits = <$bits>::from_le_bytes(chunk.try_into().unwrap());
                            sink(<$ty>::from_bits(bits))?;
                        }
                        Ok(())
                    }
                    ENC_RLE => decode_rle_into::<$ty, _>(d, rows, options, sink),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_float!(f32, u32, 4);
impl_float!(f64, u64, 8);

impl Field for char {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        let converted: Vec<u32> = values.map(|value| u32::from(*value)).collect();
        u32::encode_column(converted.iter(), out, hint)
    }
}

impl<'de> FieldDecode<'de> for char {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        u32::decode_column_into(d, rows, options, |value| {
            sink(char::from_u32(value).ok_or(Error::InvalidChar(value))?)
        })
    }
}

// ---------------------------------------------------------------------------
// Strings
// ---------------------------------------------------------------------------

fn encode_strings<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut total = 0usize;
    for value in values.clone() {
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
    }
    let mut lengths = Vec::with_capacity(lower);
    let mut data = Vec::with_capacity(total);
    for value in values {
        write_varint(&mut lengths, value.len() as u128);
        data.extend_from_slice(value.as_bytes());
    }
    write_column(out, ENC_STRING_LENGTHS)?;
    out.array(2)?;
    out.bytes(&lengths)?;
    out.bytes(&data)?;
    Ok(())
}

fn decode_strings_into<'de, F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
where
    F: FnMut(String) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    let mut pos = 0;
    let mut data_pos = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if len > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        let end = data_pos.checked_add(len).ok_or(Error::LengthOverflow)?;
        let slice = data.get(data_pos..end).ok_or(Error::LengthMismatch {
            expected: end,
            actual: data.len(),
        })?;
        sink(core::str::from_utf8(slice).map_err(|_| Error::InvalidUtf8)?.to_string())?;
        data_pos = end;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if data_pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: data_pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn encode_string_offsets<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a str>,
{
    let (lower, _) = values.size_hint();
    let mut strings = Vec::with_capacity(lower);
    let mut total = 0usize;
    for value in values {
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        strings.push(value);
    }
    let width = if total <= u32::MAX as usize { 4usize } else { 8usize };
    let mut offsets = Vec::with_capacity((strings.len() + 1) * width);
    let mut data = Vec::with_capacity(total);
    let mut offset = 0usize;
    if width == 4 {
        offsets.extend_from_slice(&0u32.to_le_bytes());
    } else {
        offsets.extend_from_slice(&0u64.to_le_bytes());
    }
    for value in &strings {
        data.extend_from_slice(value.as_bytes());
        offset = offset.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        if width == 4 {
            offsets.extend_from_slice(&(offset as u32).to_le_bytes());
        } else {
            offsets.extend_from_slice(&(offset as u64).to_le_bytes());
        }
    }
    write_column(out, ENC_STRING_OFFSETS)?;
    out.array(3)?;
    out.u64(width as u64)?;
    out.bytes(&offsets)?;
    out.bytes(&data)?;
    Ok(())
}

pub(crate) fn read_offsets(data: &[u8], width: usize, rows: usize) -> Result<Vec<u64>> {
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let expected = (rows + 1).checked_mul(width).ok_or(Error::LengthOverflow)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    let mut offsets = Vec::with_capacity(rows + 1);
    for chunk in data.chunks_exact(width) {
        let value = if width == 4 {
            u64::from(u32::from_le_bytes(chunk.try_into().unwrap()))
        } else {
            u64::from_le_bytes(chunk.try_into().unwrap())
        };
        offsets.push(value);
    }
    Ok(offsets)
}

pub(crate) fn offset_at(data: &[u8], width: usize, index: usize) -> Result<u64> {
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let start = index.checked_mul(width).ok_or(Error::LengthOverflow)?;
    let end = start.checked_add(width).ok_or(Error::LengthOverflow)?;
    let slice = data.get(start..end).ok_or(Error::LengthMismatch {
        expected: end,
        actual: data.len(),
    })?;
    Ok(if width == 4 {
        u64::from(u32::from_le_bytes(slice.try_into().unwrap()))
    } else {
        u64::from_le_bytes(slice.try_into().unwrap())
    })
}

fn decode_string_offsets_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(String) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    let offsets_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    let offsets = read_offsets(offsets_data, width, rows)?;
    if offsets.first().copied() != Some(0) {
        return Err(Error::InvalidOffset);
    }
    for pair in offsets.windows(2) {
        let start = pair[0];
        let end = pair[1];
        if end < start || end > data.len() as u64 {
            return Err(Error::InvalidOffset);
        }
        let start = start as usize;
        let end = end as usize;
        if end - start > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        sink(
            core::str::from_utf8(&data[start..end])
                .map_err(|_| Error::InvalidUtf8)?
                .to_string(),
        )?;
    }
    if offsets.last().copied() != Some(data.len() as u64) {
        return Err(Error::InvalidOffset);
    }
    Ok(())
}

impl Field for String {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::StringLengths => encode_strings(values.map(|value| value.as_str()), out),
            Encoding::StringOffsets => encode_string_offsets(values.map(|value| value.as_str()), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for String {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_STRING_LENGTHS => decode_strings_into(d, rows, options, sink),
            ENC_STRING_OFFSETS => decode_string_offsets_into(d, rows, options, sink),
            ENC_RLE => decode_rle_into::<String, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<String, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Cow<[u8]>
// ---------------------------------------------------------------------------

fn encode_bytes<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a [u8]> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut total = 0usize;
    for value in values.clone() {
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
    }
    let mut lengths = Vec::with_capacity(lower);
    let mut data = Vec::with_capacity(total);
    for value in values {
        write_varint(&mut lengths, value.len() as u128);
        data.extend_from_slice(value);
    }
    write_column(out, ENC_STRING_LENGTHS)?;
    out.array(2)?;
    out.bytes(&lengths)?;
    out.bytes(&data)?;
    Ok(())
}

fn decode_bytes_into<'de, F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
where
    F: FnMut(Cow<'de, [u8]>) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("byte payload"));
    }
    let mut pos = 0;
    let mut data_pos = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if len > options.max_string_len {
            return Err(Error::LimitExceeded("byte length"));
        }
        let end = data_pos.checked_add(len).ok_or(Error::LengthOverflow)?;
        let slice = data.get(data_pos..end).ok_or(Error::LengthMismatch {
            expected: end,
            actual: data.len(),
        })?;
        sink(Cow::Borrowed(slice))?;
        data_pos = end;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if data_pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: data_pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn encode_byte_offsets<'a, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    I: Iterator<Item = &'a [u8]>,
{
    let (lower, _) = values.size_hint();
    let mut slices = Vec::with_capacity(lower);
    let mut total = 0usize;
    for value in values {
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        slices.push(value);
    }
    let width = if total <= u32::MAX as usize { 4usize } else { 8usize };
    let mut offsets = Vec::with_capacity((slices.len() + 1) * width);
    let mut data = Vec::with_capacity(total);
    let mut offset = 0usize;
    if width == 4 {
        offsets.extend_from_slice(&0u32.to_le_bytes());
    } else {
        offsets.extend_from_slice(&0u64.to_le_bytes());
    }
    for value in &slices {
        data.extend_from_slice(value);
        offset = offset.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        if width == 4 {
            offsets.extend_from_slice(&(offset as u32).to_le_bytes());
        } else {
            offsets.extend_from_slice(&(offset as u64).to_le_bytes());
        }
    }
    write_column(out, ENC_STRING_OFFSETS)?;
    out.array(3)?;
    out.u64(width as u64)?;
    out.bytes(&offsets)?;
    out.bytes(&data)?;
    Ok(())
}

fn decode_byte_offsets_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(Cow<'de, [u8]>) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    let offsets_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("byte payload"));
    }
    let offsets = read_offsets(offsets_data, width, rows)?;
    if offsets.first().copied() != Some(0) {
        return Err(Error::InvalidOffset);
    }
    for pair in offsets.windows(2) {
        let start = pair[0];
        let end = pair[1];
        if end < start || end > data.len() as u64 {
            return Err(Error::InvalidOffset);
        }
        let start = start as usize;
        let end = end as usize;
        if end - start > options.max_string_len {
            return Err(Error::LimitExceeded("byte length"));
        }
        sink(Cow::Borrowed(&data[start..end]))?;
    }
    if offsets.last().copied() != Some(data.len() as u64) {
        return Err(Error::InvalidOffset);
    }
    Ok(())
}

impl<'a> Field for Cow<'a, [u8]> {
    fn encode_column<'b, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'b Self> + Clone,
        Self: 'b,
    {
        match hint {
            Encoding::Auto | Encoding::StringLengths => encode_bytes(values.map(|value| value.as_ref()), out),
            Encoding::StringOffsets => encode_byte_offsets(values.map(|value| value.as_ref()), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for Cow<'de, [u8]> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_STRING_LENGTHS => decode_bytes_into(d, rows, options, sink),
            ENC_STRING_OFFSETS => decode_byte_offsets_into(d, rows, options, sink),
            ENC_RLE => decode_rle_into::<Cow<'de, [u8]>, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<Cow<'de, [u8]>, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Cow<str>
// ---------------------------------------------------------------------------

fn decode_str_lengths_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(Cow<'de, str>) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    let mut pos = 0;
    let mut data_pos = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        if len > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        let end = data_pos.checked_add(len).ok_or(Error::LengthOverflow)?;
        let slice = data.get(data_pos..end).ok_or(Error::LengthMismatch {
            expected: end,
            actual: data.len(),
        })?;
        sink(Cow::Borrowed(core::str::from_utf8(slice).map_err(|_| Error::InvalidUtf8)?))?;
        data_pos = end;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if data_pos != data.len() {
        return Err(Error::LengthMismatch {
            expected: data_pos,
            actual: data.len(),
        });
    }
    Ok(())
}

fn decode_str_offsets_into<'de, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    F: FnMut(Cow<'de, str>) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    let offsets_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    let offsets = read_offsets(offsets_data, width, rows)?;
    if offsets.first().copied() != Some(0) {
        return Err(Error::InvalidOffset);
    }
    for pair in offsets.windows(2) {
        let start = pair[0];
        let end = pair[1];
        if end < start || end > data.len() as u64 {
            return Err(Error::InvalidOffset);
        }
        let start = start as usize;
        let end = end as usize;
        if end - start > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        sink(Cow::Borrowed(
            core::str::from_utf8(&data[start..end]).map_err(|_| Error::InvalidUtf8)?,
        ))?;
    }
    if offsets.last().copied() != Some(data.len() as u64) {
        return Err(Error::InvalidOffset);
    }
    Ok(())
}

impl<'a> Field for Cow<'a, str> {
    fn encode_column<'b, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'b Self> + Clone,
        Self: 'b,
    {
        match hint {
            Encoding::Auto | Encoding::StringLengths => encode_strings(values.map(|value| value.as_ref()), out),
            Encoding::StringOffsets => encode_string_offsets(values.map(|value| value.as_ref()), out),
            Encoding::Rle => encode_rle(values, out, |a, b| a == b),
            Encoding::Dictionary => encode_dictionary(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de> FieldDecode<'de> for Cow<'de, str> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_STRING_LENGTHS => decode_str_lengths_into(d, rows, options, sink),
            ENC_STRING_OFFSETS => decode_str_offsets_into(d, rows, options, sink),
            ENC_RLE => decode_rle_into::<Cow<'de, str>, _>(d, rows, options, sink),
            ENC_DICTIONARY => decode_dictionary_into::<Cow<'de, str>, _>(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Option
// ---------------------------------------------------------------------------

impl<T: Field> Field for Option<T> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        let inner_hint = match hint {
            Encoding::Auto | Encoding::Nullable => Encoding::Auto,
            other => other,
        };
        let mut present = 0usize;
        for value in values.clone() {
            if value.is_some() {
                present += 1;
            }
        }
        if present == 0 {
            write_column(out, ENC_ALL_NULL)?;
            out.array(0)?;
            return Ok(());
        }
        write_column(out, ENC_NULLABLE)?;
        out.array(2)?;
        let (lower, _) = values.size_hint();
        let mut writer = BitWriter::with_capacity(lower.div_ceil(8));
        for value in values.clone() {
            writer.write(u64::from(value.is_some()), 1);
        }
        out.bytes(&writer.finish())?;
        T::encode_column(values.filter_map(|value| value.as_ref()), out, inner_hint)
    }
}

impl<'de, T: FieldDecode<'de>> FieldDecode<'de> for Option<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_ALL_NULL => {
                read_array_len(d, 0)?;
                for _ in 0..rows {
                    sink(None)?;
                }
                Ok(())
            }
            ENC_NULLABLE => {
                read_array_len(d, 2)?;
                let validity = d.bytes()?;
                let expected = rows.div_ceil(8);
                if validity.len() != expected {
                    return Err(Error::LengthMismatch {
                        expected,
                        actual: validity.len(),
                    });
                }
                let valid = count_valid(validity, rows);
                let mut index = 0usize;
                let mut emitted = 0usize;
                T::decode_column_into(d, valid, options, |value| {
                    while index < rows && validity[index / 8] & (1 << (index % 8)) == 0 {
                        sink(None)?;
                        index += 1;
                    }
                    if index >= rows {
                        return Err(Error::LengthMismatch {
                            expected: valid,
                            actual: emitted,
                        });
                    }
                    sink(Some(value))?;
                    index += 1;
                    emitted += 1;
                    Ok(())
                })?;
                while index < rows {
                    if validity[index / 8] & (1 << (index % 8)) != 0 {
                        return Err(Error::LengthMismatch {
                            expected: valid,
                            actual: emitted,
                        });
                    }
                    sink(None)?;
                    index += 1;
                }
                if emitted != valid {
                    return Err(Error::LengthMismatch {
                        expected: valid,
                        actual: emitted,
                    });
                }
                Ok(())
            }
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Vec
// ---------------------------------------------------------------------------

fn encode_list_lengths<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    T: Field + 'a,
    I: Iterator<Item = &'a Vec<T>> + Clone,
{
    let (lower, _) = values.size_hint();
    let mut lengths = Vec::with_capacity(lower);
    let mut count = 0usize;
    for value in values.clone() {
        let len = u32::try_from(value.len()).map_err(|_| Error::LengthOverflow)?;
        write_varint(&mut lengths, u128::from(len));
        count = count.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
    }
    if count > u32::MAX as usize {
        return Err(Error::LengthOverflow);
    }
    write_column(out, ENC_LIST_LENGTHS)?;
    out.array(2)?;
    out.bytes(&lengths)?;
    T::encode_column(values.flat_map(|value| value.iter()), out, Encoding::Auto)
}

fn decode_list_lengths_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: FieldDecode<'de>,
    F: FnMut(Vec<T>) -> Result<()>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut pos = 0;
    let mut total = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    if total > options.max_list_len {
        return Err(Error::LimitExceeded("list length"));
    }

    let mut pos = 0;
    let mut row_index = 0usize;
    let mut remaining = 0usize;
    let mut row = Vec::new();
    if rows > 0 {
        let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
        remaining = len;
        row = Vec::with_capacity(len);
    }
    let mut produced = 0usize;
    T::decode_column_into(d, total, options, |value| {
        produced += 1;
        while remaining == 0 {
            sink(core::mem::take(&mut row))?;
            row_index += 1;
            if row_index >= rows {
                return Err(Error::LengthMismatch {
                    expected: total,
                    actual: produced,
                });
            }
            let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
            remaining = len;
            row = Vec::with_capacity(len);
        }
        row.push(value);
        remaining -= 1;
        Ok(())
    })?;
    if produced != total {
        return Err(Error::LengthMismatch {
            expected: total,
            actual: produced,
        });
    }
    while row_index < rows {
        if remaining != 0 {
            return Err(Error::LengthMismatch {
                expected: total,
                actual: produced,
            });
        }
        sink(core::mem::take(&mut row))?;
        row_index += 1;
        if row_index < rows {
            let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
            remaining = len;
            row = Vec::with_capacity(len);
        }
    }
    if pos != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: pos,
            actual: length_data.len(),
        });
    }
    Ok(())
}

fn encode_list_offsets<'a, T, I>(values: I, out: &mut Encoder<Vec<u8>>) -> Result<()>
where
    T: Field + 'a,
    I: Iterator<Item = &'a Vec<T>> + Clone,
{
    let mut total = 0usize;
    let mut count = 0usize;
    for value in values.clone() {
        total = total.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        count += 1;
    }
    let width = if total <= u32::MAX as usize { 4usize } else { 8usize };
    let mut offsets = Vec::with_capacity((count + 1) * width);
    if width == 4 {
        offsets.extend_from_slice(&0u32.to_le_bytes());
    } else {
        offsets.extend_from_slice(&0u64.to_le_bytes());
    }
    let mut offset = 0usize;
    for value in values.clone() {
        offset = offset.checked_add(value.len()).ok_or(Error::LengthOverflow)?;
        if width == 4 {
            offsets.extend_from_slice(&(offset as u32).to_le_bytes());
        } else {
            offsets.extend_from_slice(&(offset as u64).to_le_bytes());
        }
    }
    write_column(out, ENC_LIST_OFFSETS)?;
    out.array(3)?;
    out.u64(width as u64)?;
    out.bytes(&offsets)?;
    T::encode_column(values.flat_map(|value| value.iter()), out, Encoding::Auto)
}

fn decode_list_offsets_into<'de, T, F>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
    mut sink: F,
) -> Result<()>
where
    T: FieldDecode<'de>,
    F: FnMut(Vec<T>) -> Result<()>,
{
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let offsets_data = d.bytes()?;
    let expected = (rows + 1).checked_mul(width).ok_or(Error::LengthOverflow)?;
    if offsets_data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: offsets_data.len(),
        });
    }
    if offset_at(offsets_data, width, 0)? != 0 {
        return Err(Error::InvalidOffset);
    }
    let mut previous = 0u64;
    for index in 0..rows {
        let next = offset_at(offsets_data, width, index + 1)?;
        if next < previous {
            return Err(Error::InvalidOffset);
        }
        previous = next;
    }
    let total = usize::try_from(previous).map_err(|_| Error::LimitExceeded("list length"))?;
    if total > options.max_list_len {
        return Err(Error::LimitExceeded("list length"));
    }

    let mut row_index = 0usize;
    let mut start = 0usize;
    let mut end = if rows > 0 {
        usize::try_from(offset_at(offsets_data, width, 1)?).map_err(|_| Error::LimitExceeded("list length"))?
    } else {
        0
    };
    let mut remaining = end - start;
    let mut row = Vec::with_capacity(remaining);
    let mut produced = 0usize;
    T::decode_column_into(d, total, options, |value| {
        produced += 1;
        while remaining == 0 {
            sink(core::mem::take(&mut row))?;
            row_index += 1;
            if row_index >= rows {
                return Err(Error::LengthMismatch {
                    expected: total,
                    actual: produced,
                });
            }
            start = usize::try_from(offset_at(offsets_data, width, row_index)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            end = usize::try_from(offset_at(offsets_data, width, row_index + 1)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            if end < start {
                return Err(Error::InvalidOffset);
            }
            remaining = end - start;
            row = Vec::with_capacity(remaining);
        }
        row.push(value);
        remaining -= 1;
        Ok(())
    })?;
    if produced != total {
        return Err(Error::LengthMismatch {
            expected: total,
            actual: produced,
        });
    }
    while row_index < rows {
        if remaining != 0 {
            return Err(Error::LengthMismatch {
                expected: total,
                actual: produced,
            });
        }
        sink(core::mem::take(&mut row))?;
        row_index += 1;
        if row_index < rows {
            start = usize::try_from(offset_at(offsets_data, width, row_index)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            end = usize::try_from(offset_at(offsets_data, width, row_index + 1)?)
                .map_err(|_| Error::LimitExceeded("list length"))?;
            if end < start {
                return Err(Error::InvalidOffset);
            }
            remaining = end - start;
            row = Vec::with_capacity(remaining);
        }
    }
    Ok(())
}

impl<T: Field> Field for Vec<T> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::ListLengths | Encoding::StringLengths => encode_list_lengths(values, out),
            Encoding::ListOffsets | Encoding::StringOffsets => encode_list_offsets(values, out),
            other => Err(Error::UnsupportedEncoding(other)),
        }
    }
}

impl<'de, T: FieldDecode<'de>> FieldDecode<'de> for Vec<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        let encoding = read_column(d)?;
        match encoding {
            ENC_LIST_LENGTHS => decode_list_lengths_into(d, rows, options, sink),
            ENC_LIST_OFFSETS => decode_list_offsets_into(d, rows, options, sink),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// BTreeMap
// ---------------------------------------------------------------------------

impl<K: Field + Ord, V: Field> Field for BTreeMap<K, V> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        match hint {
            Encoding::Auto | Encoding::ListLengths | Encoding::StringLengths => {}
            other => return Err(Error::UnsupportedEncoding(other)),
        }
        let (lower, _) = values.size_hint();
        let mut lengths = Vec::with_capacity(lower);
        for map in values.clone() {
            let len = u32::try_from(map.len()).map_err(|_| Error::LengthOverflow)?;
            write_varint(&mut lengths, u128::from(len));
        }
        write_column(out, ENC_LIST_LENGTHS)?;
        out.array(2)?;
        out.bytes(&lengths)?;
        write_column(out, ENC_STRUCT)?;
        out.array(2)?;
        K::encode_column(values.clone().flat_map(|map| map.keys()), out, Encoding::Auto)?;
        V::encode_column(values.flat_map(|map| map.values()), out, Encoding::Auto)?;
        Ok(())
    }
}

impl<'de, K: FieldDecode<'de> + Ord, V: FieldDecode<'de>> FieldDecode<'de> for BTreeMap<K, V> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let encoding = read_column(d)?;
        if encoding != ENC_LIST_LENGTHS {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 2)?;
        let length_data = d.bytes()?;
        let mut pos = 0;
        let mut lengths = Vec::with_capacity(rows);
        let mut total = 0usize;
        for _ in 0..rows {
            let len = usize::try_from(read_varint(length_data, &mut pos)?).map_err(|_| Error::LengthOverflow)?;
            total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
            lengths.push(len);
        }
        if pos != length_data.len() {
            return Err(Error::LengthMismatch {
                expected: pos,
                actual: length_data.len(),
            });
        }
        if total > options.max_list_len {
            return Err(Error::LimitExceeded("map length"));
        }
        let child = read_column(d)?;
        if child != ENC_STRUCT {
            return Err(Error::InvalidEncoding(child));
        }
        read_array_len(d, 2)?;
        let keys = K::decode_column(d, total, options)?;
        let values = V::decode_column(d, total, options)?;
        let mut keys = keys.into_iter();
        let mut values = values.into_iter();
        let mut out = Vec::with_capacity(rows);
        for len in lengths {
            let mut map = BTreeMap::new();
            for _ in 0..len {
                let key = keys.next().ok_or(Error::LengthMismatch {
                    expected: total,
                    actual: 0,
                })?;
                let value = values.next().ok_or(Error::LengthMismatch {
                    expected: total,
                    actual: 0,
                })?;
                if map.insert(key, value).is_some() {
                    return Err(Error::DuplicateMapKey);
                }
            }
            out.push(map);
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Box and unit
// ---------------------------------------------------------------------------

impl<T: Field> Field for Box<T> {
    fn encode_column<'a, I>(values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        T::encode_column(values.map(|value| value.as_ref()), out, hint)
    }
}

impl<'de, T: FieldDecode<'de>> FieldDecode<'de> for Box<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Vec<Self>> {
        let mut out = Vec::with_capacity(rows);
        Self::decode_column_into(d, rows, options, |value| {
            out.push(value);
            Ok(())
        })?;
        Ok(out)
    }

    fn decode_column_into<F>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions, mut sink: F) -> Result<()>
    where
        F: FnMut(Self) -> Result<()>,
    {
        T::decode_column_into(d, rows, options, |value| sink(Box::new(value)))
    }
}

impl Field for () {
    fn encode_column<'a, I>(_values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        if !matches!(hint, Encoding::Auto | Encoding::Struct) {
            return Err(Error::UnsupportedEncoding(hint));
        }
        write_column(out, ENC_STRUCT)?;
        out.array(0)?;
        Ok(())
    }
}

impl<'de> FieldDecode<'de> for () {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, _options: &DecodeOptions) -> Result<Vec<Self>> {
        let encoding = read_column(d)?;
        if encoding != ENC_STRUCT {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 0)?;
        Ok(core::iter::repeat_with(|| ()).take(rows).collect())
    }
}

impl<T> Field for PhantomData<T> {
    fn encode_column<'a, I>(_values: I, out: &mut Encoder<Vec<u8>>, hint: Encoding) -> Result<()>
    where
        I: Iterator<Item = &'a Self> + Clone,
        Self: 'a,
    {
        if !matches!(hint, Encoding::Auto | Encoding::Struct) {
            return Err(Error::UnsupportedEncoding(hint));
        }
        write_column(out, ENC_STRUCT)?;
        out.array(0)?;
        Ok(())
    }
}

impl<'de, T> FieldDecode<'de> for PhantomData<T> {
    fn decode_column(d: &mut Decoder<'de>, rows: usize, _options: &DecodeOptions) -> Result<Vec<Self>> {
        let encoding = read_column(d)?;
        if encoding != ENC_STRUCT {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 0)?;
        Ok(core::iter::repeat_with(|| PhantomData).take(rows).collect())
    }
}
