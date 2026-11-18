//! Small portable bit-packing helpers.
//!
//! Values are packed least-significant bit first, which makes the layout
//! independent of host endianness. Width `0` is valid and means “every value is
//! the base value”; it produces no bytes.

use alloc::vec::Vec;

use crate::{Error, Result};

/// Number of bits needed to represent `value`.
#[inline]
pub fn bits_needed(value: u128) -> u8 {
    if value == 0 {
        0
    } else {
        (128 - value.leading_zeros()) as u8
    }
}

/// Writer for least-significant-bit-first packed values.
///
/// The writer appends to an existing buffer, so encoders can write payloads
/// directly into the output envelope without a temporary vector.
pub(crate) struct BitWriter<'a> {
    buf: &'a mut Vec<u8>,
    acc: u128,
    bits: usize,
}

impl<'a> BitWriter<'a> {
    pub(crate) fn new(buf: &'a mut Vec<u8>) -> Self {
        BitWriter {
            buf,
            acc: 0,
            bits: 0,
        }
    }

    #[inline]
    pub(crate) fn write(&mut self, value: u64, width: u8) {
        if width == 0 {
            return;
        }
        debug_assert!(width <= 64);
        if width == 64 && self.bits == 0 {
            self.buf.extend_from_slice(&value.to_le_bytes());
            return;
        }
        let masked = if width == 64 {
            u128::from(value)
        } else {
            u128::from(value & ((1u64 << width) - 1))
        };
        self.acc |= masked << self.bits;
        self.bits += usize::from(width);
        while self.bits >= 64 {
            self.buf.extend_from_slice(&(self.acc as u64).to_le_bytes());
            self.acc >>= 64;
            self.bits -= 64;
        }
    }

    pub(crate) fn finish(mut self) {
        if self.bits > 0 {
            let bytes = self.bits.div_ceil(8);
            let acc = self.acc.to_le_bytes();
            self.buf.extend_from_slice(&acc[..bytes]);
            self.acc = 0;
            self.bits = 0;
        }
    }
}

pub(crate) struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    acc: u128,
    bits: u32,
}

impl<'a> BitReader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            byte_pos: 0,
            acc: 0,
            bits: 0,
        }
    }

    #[inline]
    pub(crate) fn read(&mut self, width: u8) -> Result<u64> {
        if width == 0 {
            return Ok(0);
        }
        if width > 64 {
            return Err(Error::InvalidBitpackedValue);
        }
        if width == 64 && self.bits == 0 {
            let end = self.byte_pos.checked_add(8).ok_or(Error::LengthOverflow)?;
            let slice = self.data.get(self.byte_pos..end).ok_or(Error::InvalidBitpackedValue)?;
            self.byte_pos = end;
            return Ok(u64::from_le_bytes(slice.try_into().unwrap()));
        }
        while self.bits < u32::from(width) {
            let byte = *self.data.get(self.byte_pos).ok_or(Error::InvalidBitpackedValue)?;
            self.acc |= u128::from(byte) << self.bits;
            self.byte_pos += 1;
            self.bits += 8;
        }
        let mask = if width == 64 {
            u64::MAX as u128
        } else {
            (1u128 << width) - 1
        };
        let value = (self.acc & mask) as u64;
        self.acc >>= u32::from(width);
        self.bits -= u32::from(width);
        Ok(value)
    }
}

/// Decode `out.len()` packed values starting at `*bit_pos`.
///
/// The caller must have validated the payload length with [`packed_len`]; reads
/// therefore stay inside `data`. Widths above 64 are rejected.
#[inline]
pub(crate) fn unpack_into(data: &[u8], bit_pos: &mut usize, width: u8, out: &mut [u64]) -> Result<()> {
    if width == 0 {
        out.fill(0);
        return Ok(());
    }
    if width > 64 {
        return Err(Error::InvalidBitpackedValue);
    }
    let mask = if width == 64 { u64::MAX } else { (1u64 << width) - 1 };
    let width = usize::from(width);
    for slot in out.iter_mut() {
        let byte = *bit_pos >> 3;
        let shift = (*bit_pos & 7) as u32;
        *bit_pos += width;
        let word = match data.get(byte..) {
            Some(rest) if rest.len() >= 16 => {
                let lo = u64::from_le_bytes(rest[..8].try_into().unwrap());
                let hi = u64::from_le_bytes(rest[8..16].try_into().unwrap());
                u128::from(lo) | (u128::from(hi) << 64)
            }
            Some(rest) => {
                let mut word = 0u128;
                for (index, byte) in rest.iter().take(16).enumerate() {
                    word |= u128::from(*byte) << (8 * index);
                }
                word
            }
            None => return Err(Error::InvalidBitpackedValue),
        };
        *slot = ((word >> shift) as u64) & mask;
    }
    Ok(())
}

pub(crate) fn packed_len(count: usize, width: u8) -> Result<usize> {
    if width == 0 {
        return Ok(0);
    }
    count
        .checked_mul(usize::from(width))
        .and_then(|bits| bits.checked_add(7))
        .map(|bits| bits / 8)
        .ok_or(Error::LengthOverflow)
}

pub(crate) fn pack_u64<I>(values: I, width: u8) -> Vec<u8>
where
    I: Iterator<Item = u64>,
{
    let (lower, _) = values.size_hint();
    let capacity = packed_len(lower, width).unwrap_or(0);
    let mut buf = Vec::with_capacity(capacity);
    {
        let mut writer = BitWriter::new(&mut buf);
        for value in values {
            writer.write(value, width);
        }
        writer.finish();
    }
    buf
}

pub fn pack_u32<I>(values: I, width: u8) -> Vec<u8>
where
    I: Iterator<Item = u32>,
{
    pack_u64(values.map(u64::from), width)
}

pub fn unpack_u32(data: &[u8], count: usize, width: u8) -> Result<Vec<u32>> {
    if width > 32 {
        return Err(Error::InvalidBitpackedValue);
    }
    let expected = packed_len(count, width)?;
    if data.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: data.len(),
        });
    }
    let mut out = Vec::with_capacity(count);
    let mut buffer = [0u64; 64];
    let mut bit_pos = 0;
    let mut remaining = count;
    while remaining > 0 {
        let take = remaining.min(buffer.len());
        unpack_into(data, &mut bit_pos, width, &mut buffer[..take])?;
        for &value in &buffer[..take] {
            out.push(u32::try_from(value).map_err(|_| Error::InvalidBitpackedValue)?);
        }
        remaining -= take;
    }
    Ok(out)
}

/// Count the set bits in the first `count` bits of a bitmap.
pub(crate) fn count_valid(bitmap: &[u8], count: usize) -> usize {
    let full_bytes = count / 8;
    let full = &bitmap[..full_bytes.min(bitmap.len())];
    let mut set = 0usize;
    let (words, tail) = full.as_chunks::<8>();
    for word in words {
        set += u64::from_le_bytes(*word).count_ones() as usize;
    }
    if !tail.is_empty() {
        let mut word = 0u64;
        for (index, byte) in tail.iter().enumerate() {
            word |= u64::from(*byte) << (8 * index);
        }
        set += word.count_ones() as usize;
    }
    let rem = count % 8;
    if rem > 0
        && let Some(byte) = bitmap.get(full_bytes)
    {
        let mask = (1u8 << rem) - 1;
        set += (byte & mask).count_ones() as usize;
    }
    set
}
