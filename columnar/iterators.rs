//! Concrete lazy column iterators.
//!
//! These types implement [`FieldIter`] without boxing. Each encoding is a
//! small state machine over the input slice, so decoding a column does not
//! allocate a `Vec` for all of its values. RLE and dictionary iterators are the
//! exception: they materialize their run values or dictionary once, because
//! those values are needed out of order.

use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::marker::PhantomData;

use cbor::Decoder;

use crate::{
    DecodeOptions, Error, FieldIter, Result,
    bitpack::{BitReader, bits_needed, count_valid, packed_len},
    field::{FieldDecode, offset_at, packed_bytes},
    format::{
        ENC_ALL_NULL, ENC_BITPACK, ENC_DELTA_OF_DELTA, ENC_DELTA_RLE, ENC_DICTIONARY, ENC_FOR, ENC_LIST_LENGTHS,
        ENC_LIST_OFFSETS, ENC_NULLABLE, ENC_PACKED, ENC_RLE, ENC_STRING_LENGTHS, ENC_STRING_OFFSETS, ENC_STRUCT,
        ENC_VARINT, read_array_len, read_column, read_varint, unzigzag,
    },
};

// ---------------------------------------------------------------------------
// Packed scalar helpers
// ---------------------------------------------------------------------------

pub trait PackedValue: Copy {
    const WIDTH: usize;
    fn from_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_packed_value {
    ($ty:ty, $width:expr) => {
        impl PackedValue for $ty {
            const WIDTH: usize = $width;

            fn from_bytes(bytes: &[u8]) -> Self {
                <$ty>::from_le_bytes(bytes.try_into().unwrap())
            }
        }
    };
}

impl_packed_value!(u8, 1);
impl_packed_value!(u16, 2);
impl_packed_value!(u32, 4);
impl_packed_value!(u64, 8);
impl_packed_value!(u128, 16);
impl_packed_value!(i8, 1);
impl_packed_value!(i16, 2);
impl_packed_value!(i32, 4);
impl_packed_value!(i64, 8);
impl_packed_value!(i128, 16);

pub trait FloatValue: Copy {
    const WIDTH: usize;
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FloatValue for f32 {
    const WIDTH: usize = 4;

    fn from_bytes(bytes: &[u8]) -> Self {
        f32::from_bits(u32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl FloatValue for f64 {
    const WIDTH: usize = 8;

    fn from_bytes(bytes: &[u8]) -> Self {
        f64::from_bits(u64::from_le_bytes(bytes.try_into().unwrap()))
    }
}

// ---------------------------------------------------------------------------
// Packed
// ---------------------------------------------------------------------------

pub struct PackedIter<'de, T> {
    data: &'de [u8],
    position: usize,
    remaining: usize,
    _ty: PhantomData<T>,
}

impl<'de, T: PackedValue> PackedIter<'de, T> {
    fn new(data: &'de [u8], remaining: usize) -> Self {
        PackedIter {
            data,
            position: 0,
            remaining,
            _ty: PhantomData,
        }
    }
}

impl<T: PackedValue> Iterator for PackedIter<'_, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let end = self.position + T::WIDTH;
        let bytes = match self.data.get(self.position..end) {
            Some(bytes) => bytes,
            None => {
                self.remaining = 0;
                return Some(Err(Error::LengthMismatch {
                    expected: end,
                    actual: self.data.len(),
                }));
            }
        };
        self.position = end;
        self.remaining -= 1;
        Some(Ok(T::from_bytes(bytes)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: PackedValue> ExactSizeIterator for PackedIter<'_, T> {}
impl<T: PackedValue> core::iter::FusedIterator for PackedIter<'_, T> {}

pub struct FloatPackedIter<'de, T> {
    data: &'de [u8],
    position: usize,
    remaining: usize,
    _ty: PhantomData<T>,
}

impl<'de, T: FloatValue> FloatPackedIter<'de, T> {
    fn new(data: &'de [u8], remaining: usize) -> Self {
        FloatPackedIter {
            data,
            position: 0,
            remaining,
            _ty: PhantomData,
        }
    }
}

impl<T: FloatValue> Iterator for FloatPackedIter<'_, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let end = self.position + T::WIDTH;
        let bytes = match self.data.get(self.position..end) {
            Some(bytes) => bytes,
            None => {
                self.remaining = 0;
                return Some(Err(Error::LengthMismatch {
                    expected: end,
                    actual: self.data.len(),
                }));
            }
        };
        self.position = end;
        self.remaining -= 1;
        Some(Ok(T::from_bytes(bytes)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: FloatValue> ExactSizeIterator for FloatPackedIter<'_, T> {}
impl<T: FloatValue> core::iter::FusedIterator for FloatPackedIter<'_, T> {}

// ---------------------------------------------------------------------------
// Frame of reference
// ---------------------------------------------------------------------------

pub struct ForIter<'de, T> {
    reader: BitReader<'de>,
    base: i128,
    unsigned: bool,
    width: u8,
    remaining: usize,
    _ty: PhantomData<T>,
}

impl<'de, T> ForIter<'de, T>
where
    T: TryFrom<u128> + TryFrom<i128>,
{
    fn next_value(&mut self) -> Result<T> {
        let offset = self.reader.read(self.width)?;
        if self.unsigned {
            let value = (self.base as u64).wrapping_add(offset);
            T::try_from(u128::from(value)).map_err(|_| Error::LengthOverflow)
        } else {
            let value = self.base + i128::from(offset);
            T::try_from(value).map_err(|_| Error::LengthOverflow)
        }
    }
}

impl<'de, T> Iterator for ForIter<'de, T>
where
    T: TryFrom<u128> + TryFrom<i128>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        Some(self.next_value())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'de, T> ExactSizeIterator for ForIter<'de, T> where T: TryFrom<u128> + TryFrom<i128> {}
impl<'de, T> core::iter::FusedIterator for ForIter<'de, T> where T: TryFrom<u128> + TryFrom<i128> {}

// ---------------------------------------------------------------------------
// Varint
// ---------------------------------------------------------------------------

pub struct VarintIter<'de, T> {
    data: &'de [u8],
    position: usize,
    remaining: usize,
    signed: bool,
    _ty: PhantomData<T>,
}

impl<'de, T> VarintIter<'de, T>
where
    T: TryFrom<u128> + TryFrom<i128>,
{
    fn next_value(&mut self) -> Result<T> {
        let raw = read_varint(self.data, &mut self.position)?;
        if self.signed {
            T::try_from(unzigzag(raw)).map_err(|_| Error::LengthOverflow)
        } else {
            T::try_from(raw).map_err(|_| Error::LengthOverflow)
        }
    }
}

impl<'de, T> Iterator for VarintIter<'de, T>
where
    T: TryFrom<u128> + TryFrom<i128>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        if self.position == self.data.len() {
            Some(Err(Error::LengthMismatch {
                expected: self.data.len(),
                actual: self.position,
            }))
        } else {
            Some(self.next_value())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'de, T> ExactSizeIterator for VarintIter<'de, T> where T: TryFrom<u128> + TryFrom<i128> {}
impl<'de, T> core::iter::FusedIterator for VarintIter<'de, T> where T: TryFrom<u128> + TryFrom<i128> {}

// ---------------------------------------------------------------------------
// Enum tags
// ---------------------------------------------------------------------------

/// Iterator over bit-packed enum tags.
#[doc(hidden)]
pub struct TagIter<'de> {
    reader: BitReader<'de>,
    width: u8,
    remaining: usize,
    variant_count: u32,
}

impl<'de> TagIter<'de> {
    /// Create a tag iterator after validating the tag payload.
    pub fn new(tags_data: &'de [u8], rows: usize, width: u8, variant_count: usize) -> Result<Self> {
        let expected = packed_len(rows, width)?;
        if tags_data.len() != expected {
            return Err(Error::LengthMismatch {
                expected,
                actual: tags_data.len(),
            });
        }
        Ok(TagIter {
            reader: BitReader::new(tags_data),
            width,
            remaining: rows,
            variant_count: u32::try_from(variant_count).map_err(|_| Error::LengthOverflow)?,
        })
    }
}

impl Iterator for TagIter<'_> {
    type Item = Result<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let tag = match self.reader.read(self.width) {
            Ok(tag) => tag,
            Err(error) => return Some(Err(error)),
        };
        let tag = match u32::try_from(tag) {
            Ok(tag) => tag,
            Err(_) => return Some(Err(Error::InvalidBitpackedValue)),
        };
        if tag >= self.variant_count {
            return Some(Err(Error::InvalidBitpackedValue));
        }
        Some(Ok(tag))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for TagIter<'_> {}
impl core::iter::FusedIterator for TagIter<'_> {}

/// Count rows per enum variant without allocating.
#[doc(hidden)]
pub fn count_enum_tags(tags_data: &[u8], rows: usize, width: u8, counts: &mut [usize]) -> Result<()> {
    let expected = packed_len(rows, width)?;
    if tags_data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: tags_data.len(),
        });
    }
    let mut reader = BitReader::new(tags_data);
    for _ in 0..rows {
        let tag = usize::try_from(reader.read(width)?).map_err(|_| Error::InvalidBitpackedValue)?;
        let count = counts.get_mut(tag).ok_or(Error::InvalidBitpackedValue)?;
        *count += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// RLE, dictionary, delta encodings
// ---------------------------------------------------------------------------

pub struct RleIter<'de, T> {
    length_data: &'de [u8],
    length_position: usize,
    values: alloc::vec::IntoIter<T>,
    current: Option<T>,
    run_remaining: u32,
    remaining: usize,
}

fn rle_iter<'de, T>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<RleIter<'de, T>>
where
    T: FieldDecode<'de> + Clone,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut position = 0;
    let mut total = 0usize;
    let mut run_count = 0usize;
    while position < length_data.len() {
        if run_count >= options.max_list_len {
            return Err(Error::LimitExceeded("rle runs"));
        }
        let length = u32::try_from(read_varint(length_data, &mut position)?).map_err(|_| Error::LengthOverflow)?;
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
    let values = T::decode_column(d, run_count, options)?;
    Ok(RleIter {
        length_data,
        length_position: 0,
        values: values.into_iter(),
        current: None,
        run_remaining: 0,
        remaining: rows,
    })
}

impl<T: Clone> Iterator for RleIter<'_, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        if self.run_remaining == 0 {
            let value = match self.values.next() {
                Some(value) => value,
                None => {
                    self.remaining = 0;
                    return Some(Err(Error::LengthMismatch {
                        expected: self.remaining,
                        actual: 0,
                    }));
                }
            };
            let length = match read_varint(self.length_data, &mut self.length_position) {
                Ok(length) => match u32::try_from(length) {
                    Ok(length) if length != 0 => length,
                    Ok(_) => {
                        self.remaining = 0;
                        return Some(Err(Error::ZeroRun));
                    }
                    Err(_) => {
                        self.remaining = 0;
                        return Some(Err(Error::LengthOverflow));
                    }
                },
                Err(error) => {
                    self.remaining = 0;
                    return Some(Err(error));
                }
            };
            self.current = Some(value);
            self.run_remaining = length;
        }
        self.run_remaining -= 1;
        self.remaining -= 1;
        Some(Ok(self.current.clone().unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: Clone> ExactSizeIterator for RleIter<'_, T> {}
impl<T: Clone> core::iter::FusedIterator for RleIter<'_, T> {}

pub struct DictionaryIter<'de, T> {
    dictionary: Vec<T>,
    indices: BitReader<'de>,
    width: u8,
    remaining: usize,
    _ty: PhantomData<T>,
}

fn dictionary_iter<'de, T>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<DictionaryIter<'de, T>>
where
    T: FieldDecode<'de> + Clone,
{
    read_array_len(d, 3)?;
    let dictionary_len = d.u64()?;
    let dictionary_len = usize::try_from(dictionary_len).map_err(|_| Error::LengthOverflow)?;
    if dictionary_len > options.max_list_len {
        return Err(Error::LimitExceeded("dictionary length"));
    }
    options.check_output_bytes::<T>(dictionary_len)?;
    if dictionary_len == 0 && rows != 0 {
        return Err(Error::DictionaryIndex {
            index: 0,
            len: 0,
        });
    }
    let dictionary = T::decode_column(d, dictionary_len, options)?;
    let index_data = d.bytes()?;
    let width = bits_needed(u128::from(dictionary_len.saturating_sub(1) as u64));
    let expected = packed_len(rows, width)?;
    if index_data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: index_data.len(),
        });
    }
    Ok(DictionaryIter {
        dictionary,
        indices: BitReader::new(index_data),
        width,
        remaining: rows,
        _ty: PhantomData,
    })
}

impl<T: Clone> Iterator for DictionaryIter<'_, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let index = match self.indices.read(self.width) {
            Ok(index) => index,
            Err(error) => return Some(Err(error)),
        };
        let index = match u32::try_from(index) {
            Ok(index) => index,
            Err(_) => return Some(Err(Error::InvalidBitpackedValue)),
        };
        match self.dictionary.get(index as usize) {
            Some(value) => Some(Ok(value.clone())),
            None => Some(Err(Error::DictionaryIndex {
                index,
                len: self.dictionary.len(),
            })),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: Clone> ExactSizeIterator for DictionaryIter<'_, T> {}
impl<T: Clone> core::iter::FusedIterator for DictionaryIter<'_, T> {}

pub struct DeltaRleIter<'de, T> {
    length_data: &'de [u8],
    length_position: usize,
    delta_data: &'de [u8],
    delta_position: usize,
    delta: i128,
    absolute: i128,
    run_remaining: u32,
    remaining: usize,
    _ty: PhantomData<T>,
}

fn delta_rle_iter<'de, T>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<DeltaRleIter<'de, T>>
where
    T: TryFrom<i128>,
{
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut position = 0;
    let mut total = 0usize;
    let mut run_count = 0usize;
    while position < length_data.len() {
        if run_count >= options.max_list_len {
            return Err(Error::LimitExceeded("delta rle runs"));
        }
        let length = u32::try_from(read_varint(length_data, &mut position)?).map_err(|_| Error::LengthOverflow)?;
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
    Ok(DeltaRleIter {
        length_data,
        length_position: 0,
        delta_data,
        delta_position: 0,
        delta: 0,
        absolute: 0,
        run_remaining: 0,
        remaining: rows,
        _ty: PhantomData,
    })
}

impl<'de, T> Iterator for DeltaRleIter<'de, T>
where
    T: TryFrom<i128>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        if self.run_remaining == 0 {
            let length = match read_varint(self.length_data, &mut self.length_position) {
                Ok(length) => match u32::try_from(length) {
                    Ok(length) if length != 0 => length,
                    Ok(_) => {
                        self.remaining = 0;
                        return Some(Err(Error::ZeroRun));
                    }
                    Err(_) => {
                        self.remaining = 0;
                        return Some(Err(Error::LengthOverflow));
                    }
                },
                Err(error) => {
                    self.remaining = 0;
                    return Some(Err(error));
                }
            };
            let delta = match read_varint(self.delta_data, &mut self.delta_position) {
                Ok(delta) => unzigzag(delta),
                Err(error) => {
                    self.remaining = 0;
                    return Some(Err(error));
                }
            };
            self.delta = delta;
            self.run_remaining = length;
        }
        self.run_remaining -= 1;
        self.remaining -= 1;
        self.absolute = match self.absolute.checked_add(self.delta) {
            Some(absolute) => absolute,
            None => return Some(Err(Error::DeltaOverflow)),
        };
        Some(T::try_from(self.absolute).map_err(|_| Error::DeltaOverflow))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'de, T> ExactSizeIterator for DeltaRleIter<'de, T> where T: TryFrom<i128> {}
impl<'de, T> core::iter::FusedIterator for DeltaRleIter<'de, T> where T: TryFrom<i128> {}

pub struct DeltaOfDeltaIter<'de, T> {
    data: &'de [u8],
    position: usize,
    remaining: usize,
    index: usize,
    previous_value: i128,
    previous_delta: i128,
    _ty: PhantomData<T>,
}

fn delta_of_delta_iter<'de, T>(d: &mut Decoder<'de>, rows: usize) -> Result<DeltaOfDeltaIter<'de, T>>
where
    T: TryFrom<i128>,
{
    let encoding = read_column(d)?;
    if encoding != ENC_VARINT {
        return Err(Error::InvalidEncoding(encoding));
    }
    let data = d.bytes()?;
    let mut position = 0;
    for _ in 0..rows {
        read_varint(data, &mut position)?;
    }
    if position != data.len() {
        return Err(Error::LengthMismatch {
            expected: position,
            actual: data.len(),
        });
    }
    Ok(DeltaOfDeltaIter {
        data,
        position: 0,
        remaining: rows,
        index: 0,
        previous_value: 0,
        previous_delta: 0,
        _ty: PhantomData,
    })
}

impl<'de, T> Iterator for DeltaOfDeltaIter<'de, T>
where
    T: TryFrom<i128>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let encoded = match read_varint(self.data, &mut self.position) {
            Ok(encoded) => unzigzag(encoded),
            Err(error) => return Some(Err(error)),
        };
        let value = if self.index == 0 {
            self.previous_value = encoded;
            encoded
        } else {
            self.previous_delta = match self.previous_delta.checked_add(encoded) {
                Some(delta) => delta,
                None => return Some(Err(Error::DeltaOverflow)),
            };
            self.previous_value = match self.previous_value.checked_add(self.previous_delta) {
                Some(value) => value,
                None => return Some(Err(Error::DeltaOverflow)),
            };
            self.previous_value
        };
        self.index += 1;
        Some(T::try_from(value).map_err(|_| Error::DeltaOverflow))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'de, T> ExactSizeIterator for DeltaOfDeltaIter<'de, T> where T: TryFrom<i128> {}
impl<'de, T> core::iter::FusedIterator for DeltaOfDeltaIter<'de, T> where T: TryFrom<i128> {}

// ---------------------------------------------------------------------------
// Integer column iterators
// ---------------------------------------------------------------------------

pub enum IntIter<'de, T> {
    Packed(PackedIter<'de, T>),
    For(ForIter<'de, T>),
    Varint(VarintIter<'de, T>),
    Rle(RleIter<'de, T>),
    DeltaRle(DeltaRleIter<'de, T>),
    DeltaOfDelta(DeltaOfDeltaIter<'de, T>),
    Dictionary(DictionaryIter<'de, T>),
}

impl<'de, T> Iterator for IntIter<'de, T>
where
    T: PackedValue + FieldDecode<'de> + Clone + TryFrom<u128> + TryFrom<i128>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntIter::Packed(iter) => iter.next(),
            IntIter::For(iter) => iter.next(),
            IntIter::Varint(iter) => iter.next(),
            IntIter::Rle(iter) => iter.next(),
            IntIter::DeltaRle(iter) => iter.next(),
            IntIter::DeltaOfDelta(iter) => iter.next(),
            IntIter::Dictionary(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IntIter::Packed(iter) => iter.size_hint(),
            IntIter::For(iter) => iter.size_hint(),
            IntIter::Varint(iter) => iter.size_hint(),
            IntIter::Rle(iter) => iter.size_hint(),
            IntIter::DeltaRle(iter) => iter.size_hint(),
            IntIter::DeltaOfDelta(iter) => iter.size_hint(),
            IntIter::Dictionary(iter) => iter.size_hint(),
        }
    }
}

impl<'de, T> ExactSizeIterator for IntIter<'de, T> where
    T: PackedValue + FieldDecode<'de> + Clone + TryFrom<u128> + TryFrom<i128>
{
}
impl<'de, T> core::iter::FusedIterator for IntIter<'de, T> where
    T: PackedValue + FieldDecode<'de> + Clone + TryFrom<u128> + TryFrom<i128>
{
}

fn read_packed_iter<'de, T: PackedValue>(d: &mut Decoder<'de>, rows: usize) -> Result<PackedIter<'de, T>> {
    let data = packed_bytes(d, rows, T::WIDTH)?;
    Ok(PackedIter::new(data, rows))
}

fn read_for_iter<'de, T>(d: &mut Decoder<'de>, rows: usize, unsigned: bool) -> Result<ForIter<'de, T>>
where
    T: TryFrom<u128> + TryFrom<i128>,
{
    read_array_len(d, 3)?;
    let base = if unsigned {
        i128::from(d.u64()?)
    } else {
        i128::from(d.i64()?)
    };
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
    Ok(ForIter {
        reader: BitReader::new(data),
        base,
        unsigned,
        width: width as u8,
        remaining: rows,
        _ty: PhantomData,
    })
}

fn read_varint_iter<'de, T>(d: &mut Decoder<'de>, rows: usize, signed: bool) -> Result<VarintIter<'de, T>> {
    let data = d.bytes()?;
    let mut position = 0;
    for _ in 0..rows {
        read_varint(data, &mut position)?;
    }
    if position != data.len() {
        return Err(Error::LengthMismatch {
            expected: position,
            actual: data.len(),
        });
    }
    Ok(VarintIter {
        data,
        position: 0,
        remaining: rows,
        signed,
        _ty: PhantomData,
    })
}

macro_rules! impl_int_iter {
    ($ty:ty, $unsigned:expr) => {
        impl<'de> FieldIter<'de> for $ty {
            type Iter = IntIter<'de, $ty>;

            fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => Ok(IntIter::Packed(read_packed_iter(d, rows)?)),
                    ENC_FOR => Ok(IntIter::For(read_for_iter(d, rows, $unsigned)?)),
                    ENC_VARINT => Ok(IntIter::Varint(read_varint_iter(d, rows, !$unsigned)?)),
                    ENC_RLE => Ok(IntIter::Rle(rle_iter(d, rows, options)?)),
                    ENC_DELTA_RLE => Ok(IntIter::DeltaRle(delta_rle_iter(d, rows, options)?)),
                    ENC_DELTA_OF_DELTA => Ok(IntIter::DeltaOfDelta(delta_of_delta_iter(d, rows)?)),
                    ENC_DICTIONARY => Ok(IntIter::Dictionary(dictionary_iter(d, rows, options)?)),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_int_iter!(u8, true);
impl_int_iter!(u16, true);
impl_int_iter!(u32, true);
impl_int_iter!(u64, true);
impl_int_iter!(i8, false);
impl_int_iter!(i16, false);
impl_int_iter!(i32, false);
impl_int_iter!(i64, false);

macro_rules! impl_wide_int_iter {
    ($ty:ty) => {
        impl<'de> FieldIter<'de> for $ty {
            type Iter = IntIter<'de, $ty>;

            fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => Ok(IntIter::Packed(read_packed_iter(d, rows)?)),
                    ENC_RLE => Ok(IntIter::Rle(rle_iter(d, rows, options)?)),
                    ENC_DICTIONARY => Ok(IntIter::Dictionary(dictionary_iter(d, rows, options)?)),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_wide_int_iter!(u128);
impl_wide_int_iter!(i128);

// ---------------------------------------------------------------------------
// Delegated scalar types
// ---------------------------------------------------------------------------

pub struct UsizeIter<'de> {
    inner: IntIter<'de, u64>,
}

impl Iterator for UsizeIter<'_> {
    type Item = Result<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Ok(value) => Some(usize::try_from(value).map_err(|_| Error::LengthOverflow)),
            Err(error) => Some(Err(error)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for UsizeIter<'_> {}
impl core::iter::FusedIterator for UsizeIter<'_> {}

pub struct IsizeIter<'de> {
    inner: IntIter<'de, i64>,
}

impl Iterator for IsizeIter<'_> {
    type Item = Result<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Ok(value) => Some(isize::try_from(value).map_err(|_| Error::LengthOverflow)),
            Err(error) => Some(Err(error)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for IsizeIter<'_> {}
impl core::iter::FusedIterator for IsizeIter<'_> {}

pub struct CharIter<'de> {
    inner: IntIter<'de, u32>,
}

impl Iterator for CharIter<'_> {
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Ok(value) => Some(char::from_u32(value).ok_or(Error::InvalidChar(value))),
            Err(error) => Some(Err(error)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for CharIter<'_> {}
impl core::iter::FusedIterator for CharIter<'_> {}

impl<'de> FieldIter<'de> for usize {
    type Iter = UsizeIter<'de>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        Ok(UsizeIter {
            inner: u64::decode_column_iter(d, rows, options)?,
        })
    }
}

impl<'de> FieldIter<'de> for isize {
    type Iter = IsizeIter<'de>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        Ok(IsizeIter {
            inner: i64::decode_column_iter(d, rows, options)?,
        })
    }
}

impl<'de> FieldIter<'de> for char {
    type Iter = CharIter<'de>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        Ok(CharIter {
            inner: u32::decode_column_iter(d, rows, options)?,
        })
    }
}

// ---------------------------------------------------------------------------
// bool and floats
// ---------------------------------------------------------------------------

pub struct BoolBitpackIter<'de> {
    reader: BitReader<'de>,
    remaining: usize,
}

pub enum BoolIter<'de> {
    Bitpack(BoolBitpackIter<'de>),
    Rle(RleIter<'de, bool>),
}

impl Iterator for BoolIter<'_> {
    type Item = Result<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BoolIter::Bitpack(iter) => iter.next(),
            BoolIter::Rle(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            BoolIter::Bitpack(iter) => iter.size_hint(),
            BoolIter::Rle(iter) => iter.size_hint(),
        }
    }
}

impl ExactSizeIterator for BoolIter<'_> {}
impl core::iter::FusedIterator for BoolIter<'_> {}

impl Iterator for BoolBitpackIter<'_> {
    type Item = Result<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let value = match self.reader.read(1) {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        if value > 1 {
            return Some(Err(Error::InvalidBitpackedValue));
        }
        Some(Ok(value == 1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for BoolBitpackIter<'_> {}
impl core::iter::FusedIterator for BoolBitpackIter<'_> {}

impl<'de> FieldIter<'de> for bool {
    type Iter = BoolIter<'de>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
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
                Ok(BoolIter::Bitpack(BoolBitpackIter {
                    reader: BitReader::new(data),
                    remaining: rows,
                }))
            }
            ENC_RLE => Ok(BoolIter::Rle(rle_iter(d, rows, options)?)),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

pub enum FloatIter<'de, T> {
    Packed(FloatPackedIter<'de, T>),
    Rle(RleIter<'de, T>),
}

impl<'de, T> Iterator for FloatIter<'de, T>
where
    T: FloatValue + FieldDecode<'de> + Clone,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FloatIter::Packed(iter) => iter.next(),
            FloatIter::Rle(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            FloatIter::Packed(iter) => iter.size_hint(),
            FloatIter::Rle(iter) => iter.size_hint(),
        }
    }
}

impl<'de, T> ExactSizeIterator for FloatIter<'de, T> where T: FloatValue + FieldDecode<'de> + Clone {}
impl<'de, T> core::iter::FusedIterator for FloatIter<'de, T> where T: FloatValue + FieldDecode<'de> + Clone {}

macro_rules! impl_float_iter {
    ($ty:ty) => {
        impl<'de> FieldIter<'de> for $ty {
            type Iter = FloatIter<'de, $ty>;

            fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
                let encoding = read_column(d)?;
                match encoding {
                    ENC_PACKED => {
                        let data = packed_bytes(d, rows, Self::WIDTH)?;
                        Ok(FloatIter::Packed(FloatPackedIter::new(data, rows)))
                    }
                    ENC_RLE => Ok(FloatIter::Rle(rle_iter(d, rows, options)?)),
                    other => Err(Error::InvalidEncoding(other)),
                }
            }
        }
    };
}

impl_float_iter!(f32);
impl_float_iter!(f64);

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

pub trait TextValue<'de>: Sized {
    fn from_str(value: &'de str) -> Result<Self>;
}

impl<'de> TextValue<'de> for String {
    fn from_str(value: &'de str) -> Result<Self> {
        Ok(value.to_string())
    }
}

impl<'de> TextValue<'de> for Cow<'de, str> {
    fn from_str(value: &'de str) -> Result<Self> {
        Ok(Cow::Borrowed(value))
    }
}

pub struct TextLengthsIter<'de, T> {
    length_data: &'de [u8],
    length_position: usize,
    data: &'de [u8],
    data_position: usize,
    remaining: usize,
    max_string_len: usize,
    _ty: PhantomData<T>,
}

impl<'de, T: TextValue<'de>> Iterator for TextLengthsIter<'de, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let len = match read_varint(self.length_data, &mut self.length_position) {
            Ok(len) => match usize::try_from(len) {
                Ok(len) => len,
                Err(_) => return Some(Err(Error::LengthOverflow)),
            },
            Err(error) => return Some(Err(error)),
        };
        if len > self.max_string_len {
            return Some(Err(Error::LimitExceeded("string length")));
        }
        let end = match self.data_position.checked_add(len) {
            Some(end) => end,
            None => return Some(Err(Error::LengthOverflow)),
        };
        let slice = match self.data.get(self.data_position..end) {
            Some(slice) => slice,
            None => {
                return Some(Err(Error::LengthMismatch {
                    expected: end,
                    actual: self.data.len(),
                }));
            }
        };
        self.data_position = end;
        let value = match core::str::from_utf8(slice) {
            Ok(value) => value,
            Err(_) => return Some(Err(Error::InvalidUtf8)),
        };
        Some(T::from_str(value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'de, T: TextValue<'de>> ExactSizeIterator for TextLengthsIter<'de, T> {}
impl<'de, T: TextValue<'de>> core::iter::FusedIterator for TextLengthsIter<'de, T> {}

pub struct TextOffsetsIter<'de, T> {
    offsets_data: &'de [u8],
    width: usize,
    index: usize,
    data: &'de [u8],
    rows: usize,
    max_string_len: usize,
    _ty: PhantomData<T>,
}

impl<'de, T: TextValue<'de>> Iterator for TextOffsetsIter<'de, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.rows {
            return None;
        }
        let start = match offset_at(self.offsets_data, self.width, self.index) {
            Ok(start) => start,
            Err(error) => return Some(Err(error)),
        };
        let end = match offset_at(self.offsets_data, self.width, self.index + 1) {
            Ok(end) => end,
            Err(error) => return Some(Err(error)),
        };
        self.index += 1;
        if end < start || end > self.data.len() as u64 {
            return Some(Err(Error::InvalidOffset));
        }
        let start = start as usize;
        let end = end as usize;
        if end - start > self.max_string_len {
            return Some(Err(Error::LimitExceeded("string length")));
        }
        let value = match core::str::from_utf8(&self.data[start..end]) {
            Ok(value) => value,
            Err(_) => return Some(Err(Error::InvalidUtf8)),
        };
        Some(T::from_str(value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rows - self.index, Some(self.rows - self.index))
    }
}

impl<'de, T: TextValue<'de>> ExactSizeIterator for TextOffsetsIter<'de, T> {}
impl<'de, T: TextValue<'de>> core::iter::FusedIterator for TextOffsetsIter<'de, T> {}

pub enum TextIter<'de, T> {
    Lengths(TextLengthsIter<'de, T>),
    Offsets(TextOffsetsIter<'de, T>),
    Rle(RleIter<'de, T>),
    Dictionary(DictionaryIter<'de, T>),
}

impl<'de, T> Iterator for TextIter<'de, T>
where
    T: TextValue<'de> + FieldDecode<'de> + Clone,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TextIter::Lengths(iter) => iter.next(),
            TextIter::Offsets(iter) => iter.next(),
            TextIter::Rle(iter) => iter.next(),
            TextIter::Dictionary(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            TextIter::Lengths(iter) => iter.size_hint(),
            TextIter::Offsets(iter) => iter.size_hint(),
            TextIter::Rle(iter) => iter.size_hint(),
            TextIter::Dictionary(iter) => iter.size_hint(),
        }
    }
}

impl<'de, T> ExactSizeIterator for TextIter<'de, T> where T: TextValue<'de> + FieldDecode<'de> + Clone {}
impl<'de, T> core::iter::FusedIterator for TextIter<'de, T> where T: TextValue<'de> + FieldDecode<'de> + Clone {}

fn text_lengths_iter<'de, T: TextValue<'de>>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
) -> Result<TextLengthsIter<'de, T>> {
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
    let mut position = 0;
    let mut total = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut position)?).map_err(|_| Error::LengthOverflow)?;
        if len > options.max_string_len {
            return Err(Error::LimitExceeded("string length"));
        }
        total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
    }
    if position != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: position,
            actual: length_data.len(),
        });
    }
    if total != data.len() {
        return Err(Error::LengthMismatch {
            expected: total,
            actual: data.len(),
        });
    }
    Ok(TextLengthsIter {
        length_data,
        length_position: 0,
        data,
        data_position: 0,
        remaining: rows,
        max_string_len: options.max_string_len,
        _ty: PhantomData,
    })
}

fn text_offsets_iter<'de, T: TextValue<'de>>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
) -> Result<TextOffsetsIter<'de, T>> {
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let offsets_data = d.bytes()?;
    let data = d.bytes()?;
    if data.len() > options.max_total_bytes {
        return Err(Error::LimitExceeded("string payload"));
    }
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
    if previous != data.len() as u64 {
        return Err(Error::InvalidOffset);
    }
    Ok(TextOffsetsIter {
        offsets_data,
        width,
        index: 0,
        data,
        rows,
        max_string_len: options.max_string_len,
        _ty: PhantomData,
    })
}

fn text_iter<'de, T>(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<TextIter<'de, T>>
where
    T: TextValue<'de> + FieldDecode<'de> + Clone,
{
    let encoding = read_column(d)?;
    match encoding {
        ENC_STRING_LENGTHS => Ok(TextIter::Lengths(text_lengths_iter(d, rows, options)?)),
        ENC_STRING_OFFSETS => Ok(TextIter::Offsets(text_offsets_iter(d, rows, options)?)),
        ENC_RLE => Ok(TextIter::Rle(rle_iter(d, rows, options)?)),
        ENC_DICTIONARY => Ok(TextIter::Dictionary(dictionary_iter(d, rows, options)?)),
        other => Err(Error::InvalidEncoding(other)),
    }
}

impl<'de> FieldIter<'de> for String {
    type Iter = TextIter<'de, String>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        text_iter(d, rows, options)
    }
}

impl<'de> FieldIter<'de> for Cow<'de, str> {
    type Iter = TextIter<'de, Cow<'de, str>>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        text_iter(d, rows, options)
    }
}

// ---------------------------------------------------------------------------
// Byte strings
// ---------------------------------------------------------------------------

pub struct BytesLengthsIter<'de> {
    length_data: &'de [u8],
    length_position: usize,
    data: &'de [u8],
    data_position: usize,
    remaining: usize,
    max_string_len: usize,
}

impl<'de> Iterator for BytesLengthsIter<'de> {
    type Item = Result<Cow<'de, [u8]>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let len = match read_varint(self.length_data, &mut self.length_position) {
            Ok(len) => match usize::try_from(len) {
                Ok(len) => len,
                Err(_) => return Some(Err(Error::LengthOverflow)),
            },
            Err(error) => return Some(Err(error)),
        };
        if len > self.max_string_len {
            return Some(Err(Error::LimitExceeded("byte length")));
        }
        let end = match self.data_position.checked_add(len) {
            Some(end) => end,
            None => return Some(Err(Error::LengthOverflow)),
        };
        let slice = match self.data.get(self.data_position..end) {
            Some(slice) => slice,
            None => {
                return Some(Err(Error::LengthMismatch {
                    expected: end,
                    actual: self.data.len(),
                }));
            }
        };
        self.data_position = end;
        Some(Ok(Cow::Borrowed(slice)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for BytesLengthsIter<'_> {}
impl core::iter::FusedIterator for BytesLengthsIter<'_> {}

pub struct BytesOffsetsIter<'de> {
    offsets_data: &'de [u8],
    width: usize,
    index: usize,
    data: &'de [u8],
    rows: usize,
    max_string_len: usize,
}

impl<'de> Iterator for BytesOffsetsIter<'de> {
    type Item = Result<Cow<'de, [u8]>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.rows {
            return None;
        }
        let start = match offset_at(self.offsets_data, self.width, self.index) {
            Ok(start) => start,
            Err(error) => return Some(Err(error)),
        };
        let end = match offset_at(self.offsets_data, self.width, self.index + 1) {
            Ok(end) => end,
            Err(error) => return Some(Err(error)),
        };
        self.index += 1;
        if end < start || end > self.data.len() as u64 {
            return Some(Err(Error::InvalidOffset));
        }
        let start = start as usize;
        let end = end as usize;
        if end - start > self.max_string_len {
            return Some(Err(Error::LimitExceeded("byte length")));
        }
        Some(Ok(Cow::Borrowed(&self.data[start..end])))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rows - self.index, Some(self.rows - self.index))
    }
}

impl ExactSizeIterator for BytesOffsetsIter<'_> {}
impl core::iter::FusedIterator for BytesOffsetsIter<'_> {}

pub enum BytesIter<'de> {
    Lengths(BytesLengthsIter<'de>),
    Offsets(BytesOffsetsIter<'de>),
    Rle(RleIter<'de, Cow<'de, [u8]>>),
    Dictionary(DictionaryIter<'de, Cow<'de, [u8]>>),
}

impl<'de> Iterator for BytesIter<'de> {
    type Item = Result<Cow<'de, [u8]>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BytesIter::Lengths(iter) => iter.next(),
            BytesIter::Offsets(iter) => iter.next(),
            BytesIter::Rle(iter) => iter.next(),
            BytesIter::Dictionary(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            BytesIter::Lengths(iter) => iter.size_hint(),
            BytesIter::Offsets(iter) => iter.size_hint(),
            BytesIter::Rle(iter) => iter.size_hint(),
            BytesIter::Dictionary(iter) => iter.size_hint(),
        }
    }
}

impl<'de> ExactSizeIterator for BytesIter<'de> {}
impl<'de> core::iter::FusedIterator for BytesIter<'de> {}

impl<'de> FieldIter<'de> for Cow<'de, [u8]> {
    type Iter = BytesIter<'de>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        let encoding = read_column(d)?;
        match encoding {
            ENC_STRING_LENGTHS => {
                read_array_len(d, 2)?;
                let length_data = d.bytes()?;
                let data = d.bytes()?;
                if data.len() > options.max_total_bytes {
                    return Err(Error::LimitExceeded("byte payload"));
                }
                let mut position = 0;
                let mut total = 0usize;
                for _ in 0..rows {
                    let len =
                        usize::try_from(read_varint(length_data, &mut position)?).map_err(|_| Error::LengthOverflow)?;
                    if len > options.max_string_len {
                        return Err(Error::LimitExceeded("byte length"));
                    }
                    total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
                }
                if position != length_data.len() {
                    return Err(Error::LengthMismatch {
                        expected: position,
                        actual: length_data.len(),
                    });
                }
                if total != data.len() {
                    return Err(Error::LengthMismatch {
                        expected: total,
                        actual: data.len(),
                    });
                }
                Ok(BytesIter::Lengths(BytesLengthsIter {
                    length_data,
                    length_position: 0,
                    data,
                    data_position: 0,
                    remaining: rows,
                    max_string_len: options.max_string_len,
                }))
            }
            ENC_STRING_OFFSETS => {
                read_array_len(d, 3)?;
                let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
                if width != 4 && width != 8 {
                    return Err(Error::InvalidEncoding(width as u64));
                }
                let offsets_data = d.bytes()?;
                let data = d.bytes()?;
                if data.len() > options.max_total_bytes {
                    return Err(Error::LimitExceeded("byte payload"));
                }
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
                if previous != data.len() as u64 {
                    return Err(Error::InvalidOffset);
                }
                Ok(BytesIter::Offsets(BytesOffsetsIter {
                    offsets_data,
                    width,
                    index: 0,
                    data,
                    rows,
                    max_string_len: options.max_string_len,
                }))
            }
            ENC_RLE => Ok(BytesIter::Rle(rle_iter(d, rows, options)?)),
            ENC_DICTIONARY => Ok(BytesIter::Dictionary(dictionary_iter(d, rows, options)?)),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Option
// ---------------------------------------------------------------------------

pub enum OptionIter<'de, I> {
    AllNull {
        remaining: usize,
    },
    Nullable {
        inner: I,
        validity: &'de [u8],
        index: usize,
        rows: usize,
        valid: usize,
    },
}

impl<'de, I, T> Iterator for OptionIter<'de, I>
where
    I: Iterator<Item = Result<T>>,
{
    type Item = Result<Option<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OptionIter::AllNull {
                remaining,
            } => {
                if *remaining == 0 {
                    return None;
                }
                *remaining -= 1;
                Some(Ok(None))
            }
            OptionIter::Nullable {
                inner,
                validity,
                index,
                rows,
                valid,
            } => {
                if *index >= *rows {
                    return None;
                }
                if validity[*index / 8] & (1 << (*index % 8)) == 0 {
                    *index += 1;
                    return Some(Ok(None));
                }
                let value = match inner.next() {
                    Some(Ok(value)) => value,
                    Some(Err(error)) => {
                        *index = *rows;
                        return Some(Err(error));
                    }
                    None => {
                        *index = *rows;
                        return Some(Err(Error::LengthMismatch {
                            expected: *valid,
                            actual: 0,
                        }));
                    }
                };
                *index += 1;
                Some(Ok(Some(value)))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            OptionIter::AllNull {
                remaining,
            } => (*remaining, Some(*remaining)),
            OptionIter::Nullable {
                index,
                rows,
                ..
            } => (*rows - *index, Some(*rows - *index)),
        }
    }
}

impl<'de, I, T> ExactSizeIterator for OptionIter<'de, I> where I: Iterator<Item = Result<T>> {}
impl<'de, I, T> core::iter::FusedIterator for OptionIter<'de, I> where I: Iterator<Item = Result<T>> {}

impl<'de, T> FieldIter<'de> for Option<T>
where
    T: FieldIter<'de>,
{
    type Iter = OptionIter<'de, T::Iter>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        let encoding = read_column(d)?;
        match encoding {
            ENC_ALL_NULL => {
                read_array_len(d, 0)?;
                Ok(OptionIter::AllNull {
                    remaining: rows,
                })
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
                let inner = T::decode_column_iter(d, valid, options)?;
                Ok(OptionIter::Nullable {
                    inner,
                    validity,
                    index: 0,
                    rows,
                    valid,
                })
            }
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Lists
// ---------------------------------------------------------------------------

pub enum ListIter<'de, T, I> {
    Lengths {
        length_data: &'de [u8],
        length_position: usize,
        inner: I,
        rows: usize,
        row_index: usize,
        remaining_in_row: usize,
        options: DecodeOptions,
        _ty: PhantomData<T>,
    },
    Offsets {
        offsets_data: &'de [u8],
        width: usize,
        inner: I,
        rows: usize,
        row_index: usize,
        remaining_in_row: usize,
        options: DecodeOptions,
        _ty: PhantomData<T>,
    },
}

impl<'de, T, I> Iterator for ListIter<'de, T, I>
where
    I: Iterator<Item = Result<T>>,
{
    type Item = Result<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ListIter::Lengths {
                length_data,
                length_position,
                inner,
                rows,
                row_index,
                remaining_in_row,
                options,
                ..
            } => {
                if *row_index >= *rows {
                    return None;
                }
                let capacity = match options.output_capacity::<T>(*remaining_in_row) {
                    Ok(capacity) => capacity,
                    Err(error) => {
                        *row_index = *rows;
                        return Some(Err(error));
                    }
                };
                let mut row = Vec::with_capacity(capacity);
                for _ in 0..*remaining_in_row {
                    match inner.next() {
                        Some(Ok(value)) => row.push(value),
                        Some(Err(error)) => {
                            *row_index = *rows;
                            return Some(Err(error));
                        }
                        None => {
                            *row_index = *rows;
                            return Some(Err(Error::LengthMismatch {
                                expected: *remaining_in_row,
                                actual: 0,
                            }));
                        }
                    }
                }
                *row_index += 1;
                if *row_index < *rows {
                    let len = match read_varint(length_data, length_position) {
                        Ok(len) => match usize::try_from(len) {
                            Ok(len) => len,
                            Err(_) => {
                                *row_index = *rows;
                                return Some(Err(Error::LengthOverflow));
                            }
                        },
                        Err(error) => {
                            *row_index = *rows;
                            return Some(Err(error));
                        }
                    };
                    *remaining_in_row = len;
                } else {
                    *remaining_in_row = 0;
                }
                Some(Ok(row))
            }
            ListIter::Offsets {
                offsets_data,
                width,
                inner,
                rows,
                row_index,
                remaining_in_row,
                options,
                ..
            } => {
                if *row_index >= *rows {
                    return None;
                }
                let capacity = match options.output_capacity::<T>(*remaining_in_row) {
                    Ok(capacity) => capacity,
                    Err(error) => {
                        *row_index = *rows;
                        return Some(Err(error));
                    }
                };
                let mut row = Vec::with_capacity(capacity);
                for _ in 0..*remaining_in_row {
                    match inner.next() {
                        Some(Ok(value)) => row.push(value),
                        Some(Err(error)) => {
                            *row_index = *rows;
                            return Some(Err(error));
                        }
                        None => {
                            *row_index = *rows;
                            return Some(Err(Error::LengthMismatch {
                                expected: *remaining_in_row,
                                actual: 0,
                            }));
                        }
                    }
                }
                *row_index += 1;
                if *row_index < *rows {
                    let start = match offset_at(offsets_data, *width, *row_index) {
                        Ok(start) => start,
                        Err(error) => {
                            *row_index = *rows;
                            return Some(Err(error));
                        }
                    };
                    let end = match offset_at(offsets_data, *width, *row_index + 1) {
                        Ok(end) => end,
                        Err(error) => {
                            *row_index = *rows;
                            return Some(Err(error));
                        }
                    };
                    if end < start {
                        *row_index = *rows;
                        return Some(Err(Error::InvalidOffset));
                    }
                    *remaining_in_row = (end - start) as usize;
                } else {
                    *remaining_in_row = 0;
                }
                Some(Ok(row))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            ListIter::Lengths {
                rows,
                row_index,
                ..
            }
            | ListIter::Offsets {
                rows,
                row_index,
                ..
            } => (*rows - *row_index, Some(*rows - *row_index)),
        }
    }
}

impl<'de, T, I> ExactSizeIterator for ListIter<'de, T, I> where I: Iterator<Item = Result<T>> {}
impl<'de, T, I> core::iter::FusedIterator for ListIter<'de, T, I> where I: Iterator<Item = Result<T>> {}

fn list_lengths_iter<'de, T: FieldIter<'de>>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
) -> Result<ListIter<'de, T, T::Iter>> {
    read_array_len(d, 2)?;
    let length_data = d.bytes()?;
    let mut position = 0;
    let mut total = 0usize;
    for _ in 0..rows {
        let len = usize::try_from(read_varint(length_data, &mut position)?).map_err(|_| Error::LengthOverflow)?;
        total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
    }
    if position != length_data.len() {
        return Err(Error::LengthMismatch {
            expected: position,
            actual: length_data.len(),
        });
    }
    if total > options.max_list_len {
        return Err(Error::LimitExceeded("list length"));
    }
    options.check_output_bytes::<T>(total)?;
    options.check_output_bytes::<Vec<T>>(rows)?;
    let inner = T::decode_column_iter(d, total, options)?;
    let mut first_position = 0;
    let remaining_in_row = if rows > 0 {
        usize::try_from(read_varint(length_data, &mut first_position)?).map_err(|_| Error::LengthOverflow)?
    } else {
        0
    };
    Ok(ListIter::Lengths {
        length_data,
        length_position: first_position,
        inner,
        rows,
        row_index: 0,
        remaining_in_row,
        options: *options,
        _ty: PhantomData,
    })
}

fn list_offsets_iter<'de, T: FieldIter<'de>>(
    d: &mut Decoder<'de>,
    rows: usize,
    options: &DecodeOptions,
) -> Result<ListIter<'de, T, T::Iter>> {
    read_array_len(d, 3)?;
    let width = usize::try_from(d.u64()?).map_err(|_| Error::LengthOverflow)?;
    if width != 4 && width != 8 {
        return Err(Error::InvalidEncoding(width as u64));
    }
    let offsets_data = d.bytes()?;
    let expected = rows
        .checked_add(1)
        .and_then(|count| count.checked_mul(width))
        .ok_or(Error::LengthOverflow)?;
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
    options.check_output_bytes::<T>(total)?;
    options.check_output_bytes::<Vec<T>>(rows)?;
    let inner = T::decode_column_iter(d, total, options)?;
    let remaining_in_row = if rows > 0 {
        usize::try_from(offset_at(offsets_data, width, 1)?).map_err(|_| Error::LimitExceeded("list length"))?
    } else {
        0
    };
    Ok(ListIter::Offsets {
        offsets_data,
        width,
        inner,
        rows,
        row_index: 0,
        remaining_in_row,
        options: *options,
        _ty: PhantomData,
    })
}

impl<'de, T: FieldIter<'de> + 'de> FieldIter<'de> for Vec<T> {
    type Iter = ListIter<'de, T, T::Iter>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        let encoding = read_column(d)?;
        match encoding {
            ENC_LIST_LENGTHS => list_lengths_iter(d, rows, options),
            ENC_LIST_OFFSETS => list_offsets_iter(d, rows, options),
            other => Err(Error::InvalidEncoding(other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Maps
// ---------------------------------------------------------------------------

pub struct MapIter<'de, K, V, KI, VI> {
    length_data: &'de [u8],
    length_position: usize,
    keys: KI,
    values: VI,
    rows: usize,
    row_index: usize,
    remaining_in_row: usize,
    _ty: PhantomData<(K, V)>,
}

impl<'de, K, V, KI, VI> Iterator for MapIter<'de, K, V, KI, VI>
where
    KI: Iterator<Item = Result<K>>,
    VI: Iterator<Item = Result<V>>,
    K: Ord,
{
    type Item = Result<BTreeMap<K, V>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index >= self.rows {
            return None;
        }
        let mut map = BTreeMap::new();
        for _ in 0..self.remaining_in_row {
            let key = match self.keys.next() {
                Some(Ok(key)) => key,
                Some(Err(error)) => {
                    self.row_index = self.rows;
                    return Some(Err(error));
                }
                None => {
                    self.row_index = self.rows;
                    return Some(Err(Error::LengthMismatch {
                        expected: self.remaining_in_row,
                        actual: 0,
                    }));
                }
            };
            let value = match self.values.next() {
                Some(Ok(value)) => value,
                Some(Err(error)) => {
                    self.row_index = self.rows;
                    return Some(Err(error));
                }
                None => {
                    self.row_index = self.rows;
                    return Some(Err(Error::LengthMismatch {
                        expected: self.remaining_in_row,
                        actual: 0,
                    }));
                }
            };
            if map.insert(key, value).is_some() {
                self.row_index = self.rows;
                return Some(Err(Error::DuplicateMapKey));
            }
        }
        self.row_index += 1;
        if self.row_index < self.rows {
            let len = match read_varint(self.length_data, &mut self.length_position) {
                Ok(len) => match usize::try_from(len) {
                    Ok(len) => len,
                    Err(_) => {
                        self.row_index = self.rows;
                        return Some(Err(Error::LengthOverflow));
                    }
                },
                Err(error) => {
                    self.row_index = self.rows;
                    return Some(Err(error));
                }
            };
            self.remaining_in_row = len;
        } else {
            self.remaining_in_row = 0;
        }
        Some(Ok(map))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rows - self.row_index, Some(self.rows - self.row_index))
    }
}

impl<'de, K, V, KI, VI> ExactSizeIterator for MapIter<'de, K, V, KI, VI>
where
    KI: Iterator<Item = Result<K>>,
    VI: Iterator<Item = Result<V>>,
    K: Ord,
{
}
impl<'de, K, V, KI, VI> core::iter::FusedIterator for MapIter<'de, K, V, KI, VI>
where
    KI: Iterator<Item = Result<K>>,
    VI: Iterator<Item = Result<V>>,
    K: Ord,
{
}

impl<'de, K, V> FieldIter<'de> for BTreeMap<K, V>
where
    K: FieldIter<'de> + Ord + 'de,
    V: FieldIter<'de> + 'de,
{
    type Iter = MapIter<'de, K, V, K::Iter, V::Iter>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        let encoding = read_column(d)?;
        if encoding != ENC_LIST_LENGTHS {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 2)?;
        let length_data = d.bytes()?;
        let mut position = 0;
        let mut total = 0usize;
        for _ in 0..rows {
            let len = usize::try_from(read_varint(length_data, &mut position)?).map_err(|_| Error::LengthOverflow)?;
            total = total.checked_add(len).ok_or(Error::LengthOverflow)?;
        }
        if position != length_data.len() {
            return Err(Error::LengthMismatch {
                expected: position,
                actual: length_data.len(),
            });
        }
        if total > options.max_list_len {
            return Err(Error::LimitExceeded("map length"));
        }
        options.check_output_bytes::<K>(total)?;
        options.check_output_bytes::<V>(total)?;
        let child = read_column(d)?;
        if child != ENC_STRUCT {
            return Err(Error::InvalidEncoding(child));
        }
        read_array_len(d, 2)?;
        let keys = K::decode_column_iter(d, total, options)?;
        let values = V::decode_column_iter(d, total, options)?;
        let mut first_position = 0;
        let remaining_in_row = if rows > 0 {
            usize::try_from(read_varint(length_data, &mut first_position)?).map_err(|_| Error::LengthOverflow)?
        } else {
            0
        };
        Ok(MapIter {
            length_data,
            length_position: first_position,
            keys,
            values,
            rows,
            row_index: 0,
            remaining_in_row,
            _ty: PhantomData,
        })
    }
}

// ---------------------------------------------------------------------------
// Box, unit and PhantomData
// ---------------------------------------------------------------------------

pub struct BoxIter<I> {
    inner: I,
}

impl<I, T> Iterator for BoxIter<I>
where
    I: Iterator<Item = Result<T>>,
{
    type Item = Result<Box<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Ok(value) => Some(Ok(Box::new(value))),
            Err(error) => Some(Err(error)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I, T> ExactSizeIterator for BoxIter<I> where I: Iterator<Item = Result<T>> {}
impl<I, T> core::iter::FusedIterator for BoxIter<I> where I: Iterator<Item = Result<T>> + core::iter::FusedIterator {}

impl<'de, T> FieldIter<'de> for Box<T>
where
    T: FieldIter<'de>,
{
    type Iter = BoxIter<T::Iter>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, options: &DecodeOptions) -> Result<Self::Iter> {
        Ok(BoxIter {
            inner: T::decode_column_iter(d, rows, options)?,
        })
    }
}

pub struct UnitIter {
    remaining: usize,
}

impl Iterator for UnitIter {
    type Item = Result<()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        Some(Ok(()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for UnitIter {}
impl core::iter::FusedIterator for UnitIter {}

impl<'de> FieldIter<'de> for () {
    type Iter = UnitIter;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, _options: &DecodeOptions) -> Result<Self::Iter> {
        let encoding = read_column(d)?;
        if encoding != ENC_STRUCT {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 0)?;
        Ok(UnitIter {
            remaining: rows,
        })
    }
}

pub struct PhantomIter<T> {
    remaining: usize,
    _ty: PhantomData<T>,
}

impl<T> Iterator for PhantomIter<T> {
    type Item = Result<PhantomData<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        Some(Ok(PhantomData))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> ExactSizeIterator for PhantomIter<T> {}
impl<T> core::iter::FusedIterator for PhantomIter<T> {}

impl<'de, T: 'de> FieldIter<'de> for PhantomData<T> {
    type Iter = PhantomIter<T>;

    fn decode_column_iter(d: &mut Decoder<'de>, rows: usize, _options: &DecodeOptions) -> Result<Self::Iter> {
        let encoding = read_column(d)?;
        if encoding != ENC_STRUCT {
            return Err(Error::InvalidEncoding(encoding));
        }
        read_array_len(d, 0)?;
        Ok(PhantomIter {
            remaining: rows,
            _ty: PhantomData,
        })
    }
}

// ---------------------------------------------------------------------------
// Optional-default field iterator
// ---------------------------------------------------------------------------

/// Iterator used for a field that may be absent from the envelope.
#[doc(hidden)]
pub enum MaybeDefault<I, T> {
    Values(I),
    Default { remaining: usize, create: fn() -> T },
}

impl<I, T> Iterator for MaybeDefault<I, T>
where
    I: Iterator<Item = Result<T>>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MaybeDefault::Values(iter) => iter.next(),
            MaybeDefault::Default {
                remaining,
                create,
            } => {
                if *remaining == 0 {
                    return None;
                }
                *remaining -= 1;
                Some(Ok((*create)()))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            MaybeDefault::Values(iter) => iter.size_hint(),
            MaybeDefault::Default {
                remaining, ..
            } => (*remaining, Some(*remaining)),
        }
    }
}

impl<I, T> ExactSizeIterator for MaybeDefault<I, T> where I: Iterator<Item = Result<T>> {}
impl<I, T> core::iter::FusedIterator for MaybeDefault<I, T> where
    I: Iterator<Item = Result<T>> + core::iter::FusedIterator
{
}

/// Materialized iterator for custom field types that do not implement a
/// streaming state machine.
///
/// This is a convenience for [`FieldIter`] implementations:
///
/// ```ignore
/// type Iter = columnar::__private::Materialized<Self>;
/// ```
#[doc(hidden)]
pub struct Materialized<T> {
    inner: alloc::vec::IntoIter<T>,
}

impl<T> Materialized<T> {
    /// Create an iterator over an already decoded vector.
    pub fn new(values: Vec<T>) -> Self {
        Materialized {
            inner: values.into_iter(),
        }
    }
}

impl<T> Iterator for Materialized<T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Ok)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> ExactSizeIterator for Materialized<T> {}
impl<T> core::iter::FusedIterator for Materialized<T> {}
