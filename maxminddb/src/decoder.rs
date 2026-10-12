use core::convert::TryInto;

use serde::{
    de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    forward_to_deserialize_any,
};

use super::{
    DecodeLimits, MaxMindDBError,
    MaxMindDBError::{DecodingError, InvalidDatabaseError, ResourceLimit},
};

fn to_usize(base: u8, bytes: &[u8]) -> usize {
    bytes.iter().fold(base as usize, |acc, &b| (acc << 8) | b as usize)
}

enum Value<'a, 'de> {
    Bytes(&'de [u8]),
    String(&'de str),
    Bool(bool),
    I32(i32),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F64(f64),
    F32(f32),
    Map(MapAccessor<'a, 'de>),
    Array(ArrayAccess<'a, 'de>),
}

/// Decodes MaxMind DB data-section values into Serde types.
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

    /// Decodes the next value and hands it to `visitor`.
    ///
    /// The nesting depth is restored on every exit path, so sibling values do
    /// not accumulate depth.
    fn decode_any<V: Visitor<'de>>(&mut self, visitor: V) -> DecodeResult<V::Value> {
        self.charge_value()?;
        let saved_depth = self.depth;
        self.enter_depth()?;
        let result = self.decode_any_inner(visitor);
        self.depth = saved_depth;
        result
    }

    fn decode_any_inner<V: Visitor<'de>>(&mut self, visitor: V) -> DecodeResult<V::Value> {
        // A data value may be a chain of pointers. Follow it iteratively (so a
        // chain cannot exhaust the call stack) and remember where to restore
        // the cursor once the final value has been decoded.
        let mut restore_ptr = None;

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

            let value = self.decode_value(size, type_num)?;
            let result = match value {
                Value::Bool(x) => visitor.visit_bool(x),
                Value::Bytes(x) => visitor.visit_borrowed_bytes(x),
                Value::String(x) => visitor.visit_borrowed_str(x),
                Value::I32(x) => visitor.visit_i32(x),
                Value::U16(x) => visitor.visit_u16(x),
                Value::U32(x) => visitor.visit_u32(x),
                Value::U64(x) => visitor.visit_u64(x),
                Value::U128(x) => visitor.visit_u128(x),
                Value::F64(x) => visitor.visit_f64(x),
                Value::F32(x) => visitor.visit_f32(x),
                Value::Map(x) => visitor.visit_map(x),
                Value::Array(x) => visitor.visit_seq(x),
            };

            if let Some(ptr) = restore_ptr {
                self.current_ptr = ptr;
            }
            return result;
        }
    }

    fn decode_value(&mut self, size: usize, type_num: u16) -> DecodeResult<Value<'_, 'de>> {
        Ok(match type_num {
            2 => Value::String(self.decode_string(size)?),
            3 => Value::F64(self.decode_double(size)?),
            4 => Value::Bytes(self.decode_bytes(size)?),
            5 => Value::U16(self.decode_uint16(size)?),
            6 => Value::U32(self.decode_uint32(size)?),
            7 => self.decode_map(size)?,
            8 => Value::I32(self.decode_int(size)?),
            9 => Value::U64(self.decode_uint64(size)?),
            10 => Value::U128(self.decode_uint128(size)?),
            11 => self.decode_array(size)?,
            14 => Value::Bool(self.decode_bool(size)?),
            15 => Value::F32(self.decode_float(size)?),
            u => return Err(InvalidDatabaseError(format!("Unknown data type: {u:?}"))),
        })
    }

    fn decode_array(&mut self, size: usize) -> DecodeResult<Value<'_, 'de>> {
        // Refuse a declared length that cannot fit the remaining budget before
        // any storage is reserved for it.
        if size > self.values_left {
            return Err(ResourceLimit(format!(
                "array of {size} entries exceeds the remaining decoded value budget of {}",
                self.values_left
            )));
        }
        Ok(Value::Array(ArrayAccess {
            de: self,
            count: size,
        }))
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

    fn decode_map(&mut self, size: usize) -> DecodeResult<Value<'_, 'de>> {
        let entries = size
            .checked_mul(2)
            .ok_or_else(|| InvalidDatabaseError("map size overflow".to_owned()))?;
        // A map entry decodes a key and a value, so a declared size larger than
        // the remaining budget can never succeed. Reject it before reserving
        // anything for it.
        if entries > self.values_left {
            return Err(ResourceLimit(format!(
                "map of {size} entries exceeds the remaining decoded value budget of {}",
                self.values_left
            )));
        }
        Ok(Value::Map(MapAccessor {
            de: self,
            count: entries,
        }))
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

impl<'de: 'a, 'a> de::Deserializer<'de> for &'a mut Decoder<'de> {
    type Error = MaxMindDBError;

    fn deserialize_any<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.decode_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct ArrayAccess<'a, 'de: 'a> {
    de: &'a mut Decoder<'de>,
    count: usize,
}

impl<'de, 'a> SeqAccess<'de> for ArrayAccess<'a, 'de> {
    type Error = MaxMindDBError;

    fn next_element_seed<T>(&mut self, seed: T) -> DecodeResult<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.count == 0 {
            return Ok(None);
        }
        self.count -= 1;

        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct MapAccessor<'a, 'de: 'a> {
    de: &'a mut Decoder<'de>,
    count: usize,
}

impl<'de, 'a> MapAccess<'de> for MapAccessor<'a, 'de> {
    type Error = MaxMindDBError;

    fn next_key_seed<K>(&mut self, seed: K) -> DecodeResult<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.count == 0 {
            return Ok(None);
        }
        self.count -= 1;

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> DecodeResult<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.count == 0 {
            return Err(DecodingError("no more entries".to_owned()));
        }
        self.count -= 1;

        seed.deserialize(&mut *self.de)
    }
}
