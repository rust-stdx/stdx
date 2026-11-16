use std::collections::BTreeMap;

use super::{
    DecodeLimits, MaxMindDBError,
    MaxMindDBError::{DecodingError, InvalidDatabaseError, ResourceLimit},
};

fn to_usize(base: u8, bytes: &[u8]) -> usize {
    bytes.iter().fold(base as usize, |acc, &b| (acc << 8) | b as usize)
}

/// Decodes a MaxMind DB data value.
///
/// Implement this trait manually or with `#[derive(maxminddb::Decode)]`.
/// Types with borrowed fields such as `&str` or `&[u8]` decode directly from
/// the database buffer without copying.
pub trait Decode<'de>: Sized {
    /// Decode the next value from `decoder`.
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self>;
}

/// Decodes MaxMind DB data-section values.
///
/// A decoder tracks the resource limits configured on the owning [`Reader`], so
/// decoding a hostile or corrupt database returns
/// [`MaxMindDBError::ResourceLimit`] instead of recursing without bound or
/// expanding a small entry into a huge structure.
///
/// [`Reader`]: crate::Reader
#[derive(Debug)]
pub struct Decoder<'de> {
    buf: &'de [u8],
    current_ptr: usize,
    limits: DecodeLimits,
    depth: usize,
    values_left: usize,
    payload_left: usize,
}

impl<'de> Decoder<'de> {
    /// Create a decoder starting at `start_ptr` with the default resource limits.
    pub fn new(buf: &'de [u8], start_ptr: usize) -> Decoder<'de> {
        Decoder::with_limits(buf, start_ptr, DecodeLimits::default())
    }

    /// Create a decoder starting at `start_ptr` with explicit resource limits.
    ///
    /// See [`DecodeLimits`] for the meaning of each limit and the recommended
    /// defaults.
    pub fn with_limits(buf: &'de [u8], start_ptr: usize, limits: DecodeLimits) -> Decoder<'de> {
        Decoder {
            buf,
            current_ptr: start_ptr,
            values_left: limits.max_values,
            payload_left: limits.max_payload_bytes,
            limits,
            depth: 0,
        }
    }

    fn ensure_available(&self, n: usize) -> DecodeResult<()> {
        match self.current_ptr.checked_add(n) {
            Some(end) if end <= self.buf.len() => Ok(()),
            _ => Err(InvalidDatabaseError(format!(
                "truncated data: expected {} bytes at offset {} but buffer length is {}",
                n,
                self.current_ptr,
                self.buf.len()
            ))),
        }
    }

    fn eat_byte(&mut self) -> DecodeResult<u8> {
        self.ensure_available(1)?;
        let b = self.buf[self.current_ptr];
        self.current_ptr += 1;
        Ok(b)
    }

    fn size_from_ctrl_byte(&mut self, ctrl_byte: u8) -> DecodeResult<usize> {
        let size = (ctrl_byte & 0x1f) as usize;

        let bytes_to_read = size.saturating_sub(28);

        self.ensure_available(bytes_to_read)?;
        let new_offset = self.current_ptr + bytes_to_read;
        let size_bytes = &self.buf[self.current_ptr..new_offset];
        self.current_ptr = new_offset;

        Ok(match size {
            s if s < 29 => s,
            29 => 29_usize + size_bytes[0] as usize,
            30 => 285_usize + to_usize(0, size_bytes),
            _ => 65_821_usize + to_usize(0, size_bytes),
        })
    }

    /// Returns the size and type number of the value at the cursor.
    ///
    /// The type is returned as a `u16` so that an extended type byte of
    /// `249..=255` (which would overflow a `u8` when `7` is added) is reported
    /// as an unknown type instead of wrapping to an unrelated type.
    fn size_and_type(&mut self) -> DecodeResult<(usize, u16)> {
        let ctrl_byte = self.eat_byte()?;
        let mut type_num = u16::from(ctrl_byte >> 5);
        if type_num == 0 {
            type_num = u16::from(self.eat_byte()?) + 7;
        }
        let size = self.size_from_ctrl_byte(ctrl_byte)?;
        Ok((size, type_num))
    }

    /// Charges one logical value against the value budget.
    fn charge_value(&mut self) -> DecodeResult<()> {
        if self.values_left == 0 {
            return Err(ResourceLimit(format!(
                "decoded value count exceeded the configured limit of {}",
                self.limits.max_values
            )));
        }
        self.values_left -= 1;
        Ok(())
    }

    /// Charges `n` bytes of string or byte payload against the payload budget.
    fn charge_payload(&mut self, n: usize) -> DecodeResult<()> {
        match self.payload_left.checked_sub(n) {
            Some(rest) => {
                self.payload_left = rest;
                Ok(())
            }
            None => Err(ResourceLimit(format!(
                "decoded string/bytes payload exceeded the configured limit of {} bytes",
                self.limits.max_payload_bytes
            ))),
        }
    }

    /// Increases the current nesting depth and rejects values that are nested
    /// more deeply than the configured limit.
    fn enter_depth(&mut self) -> DecodeResult<()> {
        self.depth = self.depth.saturating_add(1);
        if self.depth > self.limits.max_depth {
            return Err(ResourceLimit(format!(
                "data structure nesting exceeded the configured maximum depth of {}",
                self.limits.max_depth
            )));
        }
        Ok(())
    }

    /// Decode the next value header without invoking a callback.
    ///
    /// This is used by recursive decoders such as [`crate::Value`] where an
    /// additional stack frame per nesting level would be wasteful. The returned
    /// guard restores the cursor and depth when dropped.
    fn scoped_value(&mut self) -> DecodeResult<ScopedValue<'_, 'de>> {
        self.charge_value()?;
        let saved_depth = self.depth;
        self.enter_depth()?;

        let mut restore_ptr = None;
        loop {
            let (size, type_num) = match self.size_and_type() {
                Ok(value) => value,
                Err(error) => {
                    self.depth = saved_depth;
                    return Err(error);
                }
            };

            if type_num == 1 {
                let target = match self.decode_pointer(size) {
                    Ok(target) => target,
                    Err(error) => {
                        self.depth = saved_depth;
                        return Err(error);
                    }
                };
                let after_pointer = self.current_ptr;
                if target >= self.buf.len() {
                    self.depth = saved_depth;
                    return Err(InvalidDatabaseError(format!(
                        "pointer at offset {} points beyond end of buffer ({})",
                        after_pointer - 1,
                        target
                    )));
                }
                if let Err(error) = self.enter_depth() {
                    self.depth = saved_depth;
                    return Err(error);
                }
                if restore_ptr.is_none() {
                    restore_ptr = Some(after_pointer);
                }
                self.current_ptr = target;
                continue;
            }

            if type_num == 7 {
                let values = match size.checked_mul(2) {
                    Some(values) => values,
                    None => {
                        self.depth = saved_depth;
                        return Err(InvalidDatabaseError("map size overflow".to_owned()));
                    }
                };
                if values > self.values_left {
                    self.depth = saved_depth;
                    return Err(ResourceLimit(format!(
                        "map of {size} entries exceeds the remaining decoded value budget of {}",
                        self.values_left
                    )));
                }
            } else if type_num == 11 && size > self.values_left {
                self.depth = saved_depth;
                return Err(ResourceLimit(format!(
                    "array of {size} entries exceeds the remaining decoded value budget of {}",
                    self.values_left
                )));
            }

            return Ok(ScopedValue {
                de: self,
                size,
                type_num,
                restore_ptr,
                saved_depth,
            });
        }
    }

    /// Resolve pointers and invoke `f` with the final value header.
    ///
    /// The cursor and depth are restored after `f` returns, including when it
    /// consumes a container that was reached through a pointer.
    fn with_value<R>(&mut self, f: impl FnOnce(&mut Self, usize, u16) -> DecodeResult<R>) -> DecodeResult<R> {
        self.charge_value()?;
        let saved_depth = self.depth;
        self.enter_depth()?;

        let mut restore_ptr = None;
        let result = (|| {
            loop {
                let (size, type_num) = self.size_and_type()?;

                if type_num == 1 {
                    let target = self.decode_pointer(size)?;
                    let after_pointer = self.current_ptr;
                    if target >= self.buf.len() {
                        return Err(InvalidDatabaseError(format!(
                            "pointer at offset {} points beyond end of buffer ({})",
                            after_pointer - 1,
                            target
                        )));
                    }
                    // Following a pointer counts as one level of nesting. This
                    // bounds both unbounded pointer chains and pointer cycles.
                    self.enter_depth()?;
                    if restore_ptr.is_none() {
                        restore_ptr = Some(after_pointer);
                    }
                    self.current_ptr = target;
                    continue;
                }

                return f(self, size, type_num);
            }
        })();

        self.depth = saved_depth;
        if let Some(ptr) = restore_ptr {
            self.current_ptr = ptr;
        }
        result
    }

    fn unexpected(type_num: u16, expected: &str) -> MaxMindDBError {
        DecodingError(format!("expected {expected}, found data type {type_num}"))
    }

    /// Decode a `bool`.
    pub fn read_bool(&mut self) -> DecodeResult<bool> {
        self.with_value(|de, size, type_num| {
            if type_num != 14 {
                return Err(Self::unexpected(type_num, "a boolean"));
            }
            de.decode_bool(size)
        })
    }

    /// Decode an unsigned integer.
    ///
    /// Accepts any unsigned wire integer type as long as the value fits in
    /// `u128`. Negative integers are rejected.
    pub fn read_unsigned(&mut self) -> DecodeResult<u128> {
        self.with_value(|de, size, type_num| match type_num {
            5 => Ok(u128::from(de.decode_uint16(size)?)),
            6 => Ok(u128::from(de.decode_uint32(size)?)),
            9 => Ok(u128::from(de.decode_uint64(size)?)),
            10 => de.decode_uint128(size),
            8 => match de.decode_int(size)? {
                value if value >= 0 => Ok(value as u128),
                value => Err(DecodingError(format!("negative integer {value} does not fit an unsigned type"))),
            },
            _ => Err(Self::unexpected(type_num, "an unsigned integer")),
        })
    }

    /// Decode a signed integer.
    ///
    /// Accepts any integer wire type as long as the value fits in `i128`.
    pub fn read_signed(&mut self) -> DecodeResult<i128> {
        self.with_value(|de, size, type_num| match type_num {
            8 => Ok(i128::from(de.decode_int(size)?)),
            5 => Ok(i128::from(de.decode_uint16(size)?)),
            6 => Ok(i128::from(de.decode_uint32(size)?)),
            9 => Ok(i128::from(de.decode_uint64(size)?)),
            10 => i128::try_from(de.decode_uint128(size)?)
                .map_err(|_| DecodingError("integer does not fit in i128".to_owned())),
            _ => Err(Self::unexpected(type_num, "an integer")),
        })
    }

    /// Decode an `f32`.
    pub fn read_f32(&mut self) -> DecodeResult<f32> {
        self.with_value(|de, size, type_num| match type_num {
            15 => de.decode_float(size),
            3 => Ok(de.decode_double(size)? as f32),
            _ => Err(Self::unexpected(type_num, "a float")),
        })
    }

    /// Decode an `f64`.
    pub fn read_f64(&mut self) -> DecodeResult<f64> {
        self.with_value(|de, size, type_num| match type_num {
            3 => de.decode_double(size),
            15 => Ok(f64::from(de.decode_float(size)?)),
            _ => Err(Self::unexpected(type_num, "a float")),
        })
    }

    /// Decode a UTF-8 string borrowed from the database.
    pub fn read_str(&mut self) -> DecodeResult<&'de str> {
        self.with_value(|de, size, type_num| {
            if type_num != 2 {
                return Err(Self::unexpected(type_num, "a string"));
            }
            de.decode_string(size)
        })
    }

    /// Decode a byte string borrowed from the database.
    pub fn read_bytes(&mut self) -> DecodeResult<&'de [u8]> {
        self.with_value(|de, size, type_num| {
            if type_num != 4 {
                return Err(Self::unexpected(type_num, "a byte string"));
            }
            de.decode_bytes(size)
        })
    }

    /// Decode a map, invoking `f` once for its entries.
    pub fn with_map<R>(&mut self, f: impl FnOnce(&mut MapEntries<'_, 'de>) -> DecodeResult<R>) -> DecodeResult<R> {
        self.with_value(|de, size, type_num| {
            if type_num != 7 {
                return Err(Self::unexpected(type_num, "a map"));
            }
            de.with_map_entries(size, f)
        })
    }

    /// Decode an array, invoking `f` once for its elements.
    pub fn with_array<R>(&mut self, f: impl FnOnce(&mut ArrayEntries<'_, 'de>) -> DecodeResult<R>) -> DecodeResult<R> {
        self.with_value(|de, size, type_num| {
            if type_num != 11 {
                return Err(Self::unexpected(type_num, "an array"));
            }
            de.with_array_entries(size, f)
        })
    }

    fn with_map_entries<R>(
        &mut self,
        size: usize,
        f: impl FnOnce(&mut MapEntries<'_, 'de>) -> DecodeResult<R>,
    ) -> DecodeResult<R> {
        let values = size
            .checked_mul(2)
            .ok_or_else(|| InvalidDatabaseError("map size overflow".to_owned()))?;
        if values > self.values_left {
            return Err(ResourceLimit(format!(
                "map of {size} entries exceeds the remaining decoded value budget of {}",
                self.values_left
            )));
        }

        let mut entries = MapEntries {
            de: self,
            remaining: size,
            awaiting_value: false,
        };
        let result = f(&mut entries)?;
        if entries.remaining != 0 || entries.awaiting_value {
            return Err(DecodingError("map was not fully consumed".to_owned()));
        }
        Ok(result)
    }

    fn with_array_entries<R>(
        &mut self,
        size: usize,
        f: impl FnOnce(&mut ArrayEntries<'_, 'de>) -> DecodeResult<R>,
    ) -> DecodeResult<R> {
        if size > self.values_left {
            return Err(ResourceLimit(format!(
                "array of {size} entries exceeds the remaining decoded value budget of {}",
                self.values_left
            )));
        }

        let mut entries = ArrayEntries {
            de: self,
            remaining: size,
        };
        let result = f(&mut entries)?;
        if entries.remaining != 0 {
            return Err(DecodingError("array was not fully consumed".to_owned()));
        }
        Ok(result)
    }

    /// Skip the next value without materializing it.
    ///
    /// Pointers are skipped without following them; the pointed-to value is
    /// still available elsewhere in the data section.
    pub fn skip_value(&mut self) -> DecodeResult<()> {
        self.charge_value()?;
        let saved_depth = self.depth;
        self.enter_depth()?;
        let result = self.skip_value_inner();
        self.depth = saved_depth;
        result
    }

    fn skip_value_inner(&mut self) -> DecodeResult<()> {
        let (size, type_num) = self.size_and_type()?;
        match type_num {
            1 => {
                self.decode_pointer(size)?;
                Ok(())
            }
            2 => {
                self.charge_payload(size)?;
                self.skip_bytes(size)
            }
            3 => {
                self.decode_double(size)?;
                Ok(())
            }
            4 => {
                self.charge_payload(size)?;
                self.skip_bytes(size)
            }
            5 => {
                self.decode_uint16(size)?;
                Ok(())
            }
            6 => {
                self.decode_uint32(size)?;
                Ok(())
            }
            7 => {
                for _ in 0..size {
                    self.skip_value()?;
                    self.skip_value()?;
                }
                Ok(())
            }
            8 => {
                self.decode_int(size)?;
                Ok(())
            }
            9 => {
                self.decode_uint64(size)?;
                Ok(())
            }
            10 => {
                self.decode_uint128(size)?;
                Ok(())
            }
            11 => {
                for _ in 0..size {
                    self.skip_value()?;
                }
                Ok(())
            }
            14 => {
                self.decode_bool(size)?;
                Ok(())
            }
            15 => {
                self.decode_float(size)?;
                Ok(())
            }
            u => Err(InvalidDatabaseError(format!("Unknown data type: {u:?}"))),
        }
    }

    fn skip_bytes(&mut self, size: usize) -> DecodeResult<()> {
        let new_offset = self
            .current_ptr
            .checked_add(size)
            .ok_or_else(|| InvalidDatabaseError(format!("bytes length {size} overflows the buffer offset")))?;
        self.ensure_available(size)?;
        self.current_ptr = new_offset;
        Ok(())
    }

    fn decode_bool(&mut self, size: usize) -> DecodeResult<bool> {
        match size {
            0 | 1 => Ok(size != 0),
            s => Err(InvalidDatabaseError(format!("bool of size {s:?}"))),
        }
    }

    fn decode_bytes(&mut self, size: usize) -> DecodeResult<&'de [u8]> {
        self.charge_payload(size)?;
        let new_offset = self
            .current_ptr
            .checked_add(size)
            .ok_or_else(|| InvalidDatabaseError(format!("bytes length {size} overflows the buffer offset")))?;
        self.ensure_available(size)?;
        let u8_slice = &self.buf[self.current_ptr..new_offset];
        self.current_ptr = new_offset;

        Ok(u8_slice)
    }

    fn decode_float(&mut self, size: usize) -> DecodeResult<f32> {
        self.ensure_available(size)?;
        let new_offset = self.current_ptr + size;
        let value: [u8; 4] = self.buf[self.current_ptr..new_offset]
            .try_into()
            .map_err(|_| InvalidDatabaseError(format!("float of size {:?}", new_offset - self.current_ptr)))?;
        self.current_ptr = new_offset;
        let float_value = f32::from_be_bytes(value);
        Ok(float_value)
    }

    fn decode_double(&mut self, size: usize) -> DecodeResult<f64> {
        self.ensure_available(size)?;
        let new_offset = self.current_ptr + size;
        let value: [u8; 8] = self.buf[self.current_ptr..new_offset]
            .try_into()
            .map_err(|_| InvalidDatabaseError(format!("double of size {:?}", new_offset - self.current_ptr)))?;
        self.current_ptr = new_offset;
        let float_value = f64::from_be_bytes(value);
        Ok(float_value)
    }

    fn decode_uint64(&mut self, size: usize) -> DecodeResult<u64> {
        match size {
            s if s <= 8 => {
                self.ensure_available(size)?;
                let new_offset = self.current_ptr + size;

                let value = self.buf[self.current_ptr..new_offset]
                    .iter()
                    .fold(0_u64, |acc, &b| (acc << 8) | u64::from(b));
                self.current_ptr = new_offset;
                Ok(value)
            }
            s => Err(InvalidDatabaseError(format!("u64 of size {s:?}"))),
        }
    }

    fn decode_uint128(&mut self, size: usize) -> DecodeResult<u128> {
        match size {
            s if s <= 16 => {
                self.ensure_available(size)?;
                let new_offset = self.current_ptr + size;

                let value = self.buf[self.current_ptr..new_offset]
                    .iter()
                    .fold(0_u128, |acc, &b| (acc << 8) | u128::from(b));
                self.current_ptr = new_offset;
                Ok(value)
            }
            s => Err(InvalidDatabaseError(format!("u128 of size {s:?}"))),
        }
    }

    fn decode_uint32(&mut self, size: usize) -> DecodeResult<u32> {
        match size {
            s if s <= 4 => {
                self.ensure_available(size)?;
                let new_offset = self.current_ptr + size;

                let value = self.buf[self.current_ptr..new_offset]
                    .iter()
                    .fold(0_u32, |acc, &b| (acc << 8) | u32::from(b));
                self.current_ptr = new_offset;
                Ok(value)
            }
            s => Err(InvalidDatabaseError(format!("u32 of size {s:?}"))),
        }
    }

    fn decode_uint16(&mut self, size: usize) -> DecodeResult<u16> {
        match size {
            s if s <= 2 => {
                self.ensure_available(size)?;
                let new_offset = self.current_ptr + size;

                let value = self.buf[self.current_ptr..new_offset]
                    .iter()
                    .fold(0_u16, |acc, &b| (acc << 8) | u16::from(b));
                self.current_ptr = new_offset;
                Ok(value)
            }
            s => Err(InvalidDatabaseError(format!("u16 of size {s:?}"))),
        }
    }

    fn decode_int(&mut self, size: usize) -> DecodeResult<i32> {
        match size {
            s if s <= 4 => {
                self.ensure_available(size)?;
                let new_offset = self.current_ptr + size;

                let value = self.buf[self.current_ptr..new_offset]
                    .iter()
                    .fold(0_i32, |acc, &b| (acc << 8) | i32::from(b));
                self.current_ptr = new_offset;
                Ok(value)
            }
            s => Err(InvalidDatabaseError(format!("int32 of size {s:?}"))),
        }
    }

    fn decode_pointer(&mut self, size: usize) -> DecodeResult<usize> {
        let pointer_value_offset = [0, 0, 2048, 526_336, 0];
        let pointer_size = ((size >> 3) & 0x3) + 1;
        self.ensure_available(pointer_size)?;
        let new_offset = self.current_ptr + pointer_size;
        let pointer_bytes = &self.buf[self.current_ptr..new_offset];
        self.current_ptr = new_offset;

        let base = if pointer_size == 4 { 0 } else { (size & 0x7) as u8 };
        let unpacked = to_usize(base, pointer_bytes);

        Ok(unpacked + pointer_value_offset[pointer_size])
    }

    fn decode_string(&mut self, size: usize) -> DecodeResult<&'de str> {
        use std::str::from_utf8;

        self.charge_payload(size)?;
        let new_offset = self
            .current_ptr
            .checked_add(size)
            .ok_or_else(|| InvalidDatabaseError(format!("string length {size} overflows the buffer offset")))?;
        self.ensure_available(size)?;
        let bytes = &self.buf[self.current_ptr..new_offset];
        self.current_ptr = new_offset;
        match from_utf8(bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(InvalidDatabaseError("error decoding string".to_owned())),
        }
    }
}

pub type DecodeResult<T> = Result<T, MaxMindDBError>;

struct ScopedValue<'a, 'de: 'a> {
    de: &'a mut Decoder<'de>,
    size: usize,
    type_num: u16,
    restore_ptr: Option<usize>,
    saved_depth: usize,
}

impl Drop for ScopedValue<'_, '_> {
    fn drop(&mut self) {
        self.de.depth = self.saved_depth;
        if let Some(pointer) = self.restore_ptr {
            self.de.current_ptr = pointer;
        }
    }
}

/// Entries of a map being decoded.
///
/// A key must be followed by [`MapEntries::next_value`] or
/// [`MapEntries::skip_value`] before the next key can be read.
pub struct MapEntries<'a, 'de: 'a> {
    de: &'a mut Decoder<'de>,
    remaining: usize,
    awaiting_value: bool,
}

impl<'de, 'a> MapEntries<'a, 'de> {
    /// Decode the next map key, or return `None` at the end of the map.
    pub fn next_key(&mut self) -> DecodeResult<Option<&'de str>> {
        if self.awaiting_value {
            return Err(DecodingError("map value was not consumed".to_owned()));
        }
        if self.remaining == 0 {
            return Ok(None);
        }
        let key = self.de.read_str()?;
        self.awaiting_value = true;
        Ok(Some(key))
    }

    /// Decode the value belonging to the last key.
    pub fn next_value<T: Decode<'de>>(&mut self) -> DecodeResult<T> {
        if !self.awaiting_value {
            return Err(DecodingError("map value requested before a key".to_owned()));
        }
        self.awaiting_value = false;
        self.remaining -= 1;
        T::decode(self.de)
    }

    /// Skip the value belonging to the last key.
    pub fn skip_value(&mut self) -> DecodeResult<()> {
        if !self.awaiting_value {
            return Err(DecodingError("map value requested before a key".to_owned()));
        }
        self.awaiting_value = false;
        self.remaining -= 1;
        self.de.skip_value()
    }
}

/// Elements of an array being decoded.
pub struct ArrayEntries<'a, 'de: 'a> {
    de: &'a mut Decoder<'de>,
    remaining: usize,
}

impl<'de, 'a> ArrayEntries<'a, 'de> {
    /// Decode the next element, or return `None` at the end of the array.
    pub fn next_element<T: Decode<'de>>(&mut self) -> DecodeResult<Option<T>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        T::decode(self.de).map(Some)
    }
}

impl<'de> Decode<'de> for bool {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.read_bool()
    }
}

macro_rules! impl_decode_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<'de> Decode<'de> for $ty {
                fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
                    <$ty>::try_from(decoder.read_unsigned()?)
                        .map_err(|_| DecodingError("integer does not fit target type".to_owned()))
                }
            }
        )*
    };
}

impl_decode_unsigned!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_decode_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<'de> Decode<'de> for $ty {
                fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
                    <$ty>::try_from(decoder.read_signed()?)
                        .map_err(|_| DecodingError("integer does not fit target type".to_owned()))
                }
            }
        )*
    };
}

impl_decode_signed!(i8, i16, i32, i64, i128, isize);

impl<'de> Decode<'de> for f32 {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.read_f32()
    }
}

impl<'de> Decode<'de> for f64 {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.read_f64()
    }
}

impl<'de: 'a, 'a> Decode<'de> for &'a str {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.read_str()
    }
}

impl<'de> Decode<'de> for String {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        Ok(decoder.read_str()?.to_owned())
    }
}

impl<'de: 'a, 'a> Decode<'de> for &'a [u8] {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.read_bytes()
    }
}

/// MaxMind DB has no null value, so a decoded `Option` is always `Some`.
///
/// The derive uses [`Option`] to represent fields that may be absent from a
/// map; those fields are decoded as `None` when their key is missing.
impl<'de, T: Decode<'de>> Decode<'de> for Option<T> {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        T::decode(decoder).map(Some)
    }
}

impl<'de, T: Decode<'de>> Decode<'de> for Box<T> {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        T::decode(decoder).map(Box::new)
    }
}

impl<'de, T: Decode<'de>> Decode<'de> for Vec<T> {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.with_array(|array| {
            let mut values = Vec::new();
            while let Some(value) = array.next_element::<T>()? {
                values.push(value);
            }
            Ok(values)
        })
    }
}

impl<'de, V: Decode<'de>> Decode<'de> for BTreeMap<String, V> {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.with_map(|map| {
            let mut values = BTreeMap::new();
            while let Some(key) = map.next_key()? {
                values.insert(key.to_owned(), map.next_value::<V>()?);
            }
            Ok(values)
        })
    }
}

impl<'de: 'a, 'a, V: Decode<'de>> Decode<'de> for BTreeMap<&'a str, V> {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        decoder.with_map(|map| {
            let mut values = BTreeMap::new();
            while let Some(key) = map.next_key()? {
                values.insert(key, map.next_value::<V>()?);
            }
            Ok(values)
        })
    }
}

impl<'de> Decode<'de> for crate::encoder::Value {
    fn decode(decoder: &mut Decoder<'de>) -> DecodeResult<Self> {
        use crate::encoder::Value;

        // Use the scoped header directly rather than a callback so deeply
        // nested arrays and maps stay within a modest stack budget.
        let scoped = decoder.scoped_value()?;
        match scoped.type_num {
            2 => Ok(Value::String(scoped.de.decode_string(scoped.size)?.to_owned())),
            3 => Ok(Value::Float64(scoped.de.decode_double(scoped.size)?)),
            4 => Ok(Value::Bytes(scoped.de.decode_bytes(scoped.size)?.to_vec())),
            5 => Ok(Value::Uint64(u64::from(scoped.de.decode_uint16(scoped.size)?))),
            6 => Ok(Value::Uint64(u64::from(scoped.de.decode_uint32(scoped.size)?))),
            7 => {
                let mut map = BTreeMap::new();
                for _ in 0..scoped.size {
                    let key = scoped.de.read_str()?;
                    let value = Value::decode(scoped.de)?;
                    map.insert(key.to_owned(), value);
                }
                Ok(Value::Map(map))
            }
            8 => Ok(Value::Int32(scoped.de.decode_int(scoped.size)?)),
            9 => Ok(Value::Uint64(scoped.de.decode_uint64(scoped.size)?)),
            10 => Ok(Value::Uint128(scoped.de.decode_uint128(scoped.size)?)),
            11 => {
                let mut values = Vec::new();
                for _ in 0..scoped.size {
                    values.push(Value::decode(scoped.de)?);
                }
                Ok(Value::Slice(values))
            }
            14 => Ok(Value::Bool(scoped.de.decode_bool(scoped.size)?)),
            15 => Ok(Value::Float32(scoped.de.decode_float(scoped.size)?)),
            u => Err(InvalidDatabaseError(format!("Unknown data type: {u:?}"))),
        }
    }
}
