use std::{collections::BTreeMap, fmt};

pub type EncodeResult<T> = Result<T, String>;

/// Error returned while encoding a value for a MaxMind DB.
#[derive(Debug)]
pub struct SerError(pub String);

impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::error::Error for SerError {}

const FIRST_SIZE: usize = 29;
const SECOND_SIZE: usize = FIRST_SIZE + 256;
const THIRD_SIZE: usize = SECOND_SIZE + (1 << 16);

/// A MaxMind DB value.
///
/// This is a fully owned representation. When reading, prefer decoding into a
/// borrowed type (for example the structs in [`crate::geoip2`]) to avoid
/// copying strings and byte arrays out of the database.
///
/// [`Value::Pointer`] is only meaningful while encoding: the numeric value is a
/// raw data-section pointer. Use it only if you are building pointer-based
/// databases by hand.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Pointer(u32),
    String(String),
    Float64(f64),
    Bytes(Vec<u8>),
    Uint16(u16),
    Uint32(u32),
    Int32(i32),
    Map(BTreeMap<String, Value>),
    Uint64(u64),
    Uint128(u128),
    Slice(Vec<Value>),
    Bool(bool),
    Float32(f32),
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Value::Uint16(v)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Uint32(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int32(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::Uint64(v)
    }
}

impl From<u128> for Value {
    fn from(v: u128) -> Self {
        Value::Uint128(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float64(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float32(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

/// Build a [`Value::Map`] from `key => value` pairs.
///
/// Keys are converted with `to_string`; values with `Into<Value>`.
#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut __map = ::std::collections::BTreeMap::new();
        $(
            __map.insert(
                ($key).to_string(),
                ::std::convert::Into::<$crate::encoder::Value>::into($val),
            );
        )*
        $crate::encoder::Value::Map(__map)
    }};
}

/// Encode a value in MaxMind DB binary form.
///
/// Implement this trait manually or with `#[derive(maxminddb::Encode)]`.
pub trait Encode {
    /// Encode this value into `encoder`.
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError>;
}

/// Writes MaxMind DB binary values into an in-memory buffer.
#[derive(Debug, Default)]
pub struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    /// Create an empty encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consume the encoder and return the encoded bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    /// Return the encoded bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    /// Write a raw data-section pointer.
    pub fn pointer(&mut self, pointer: u32) {
        encode_pointer_raw(&mut self.buf, pointer);
    }

    /// Write a map header with `len` entries.
    pub fn map(&mut self, len: usize) -> Result<(), SerError> {
        check_payload_size(len)?;
        ctrl_and_size(&mut self.buf, 7, len);
        Ok(())
    }

    /// Write an array header with `len` elements.
    pub fn array(&mut self, len: usize) -> Result<(), SerError> {
        check_payload_size(len)?;
        ctrl_and_size(&mut self.buf, 11, len);
        Ok(())
    }

    /// Write a UTF-8 string.
    pub fn str(&mut self, value: &str) -> Result<(), SerError> {
        write_str_raw(&mut self.buf, value)
    }

    /// Write a byte string.
    pub fn bytes(&mut self, value: &[u8]) -> Result<(), SerError> {
        write_bytes_raw(&mut self.buf, value)
    }

    /// Write a boolean.
    pub fn bool(&mut self, value: bool) {
        let payload_len = usize::from(value);
        ctrl_and_size(&mut self.buf, 14, payload_len);
    }

    /// Write a `u16`.
    pub fn u16(&mut self, value: u16) {
        let size = 2 - (value.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.buf, 5, size);
        for i in (0..size).rev() {
            self.buf.push((value >> (8 * i)) as u8);
        }
    }

    /// Write a `u32`.
    pub fn u32(&mut self, value: u32) {
        let size = 4 - (value.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.buf, 6, size);
        for i in (0..size).rev() {
            self.buf.push((value >> (8 * i)) as u8);
        }
    }

    /// Write a `u64`.
    pub fn u64(&mut self, value: u64) {
        let size = 8 - (value.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.buf, 9, size);
        for i in (0..size).rev() {
            self.buf.push((value >> (8 * i)) as u8);
        }
    }

    /// Write a `u128`.
    pub fn u128(&mut self, value: u128) {
        let size = 16 - (value.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.buf, 10, size);
        for i in (0..size).rev() {
            self.buf.push((value >> (8 * i)) as u8);
        }
    }

    /// Write an `i32`.
    pub fn i32(&mut self, value: i32) {
        let unsigned = value as u32;
        let size = 4 - (unsigned.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.buf, 8, size);
        for i in (0..size).rev() {
            self.buf.push((unsigned >> (8 * i)) as u8);
        }
    }

    /// Write an `f32`.
    pub fn f32(&mut self, value: f32) {
        ctrl_and_size(&mut self.buf, 15, 4);
        self.buf.extend_from_slice(&value.to_bits().to_be_bytes());
    }

    /// Write an `f64`.
    pub fn f64(&mut self, value: f64) {
        ctrl_and_size(&mut self.buf, 3, 8);
        self.buf.extend_from_slice(&value.to_bits().to_be_bytes());
    }
}

impl<T: Encode + ?Sized> Encode for &T {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        (*self).encode(encoder)
    }
}

impl<T: Encode + ?Sized> Encode for Box<T> {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        (**self).encode(encoder)
    }
}

impl Encode for bool {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.bool(*self);
        Ok(())
    }
}

impl Encode for u8 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.u16(u16::from(*self));
        Ok(())
    }
}

impl Encode for u16 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.u16(*self);
        Ok(())
    }
}

impl Encode for u32 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.u32(*self);
        Ok(())
    }
}

impl Encode for u64 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.u64(*self);
        Ok(())
    }
}

impl Encode for u128 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.u128(*self);
        Ok(())
    }
}

impl Encode for usize {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.u64(*self as u64);
        Ok(())
    }
}

impl Encode for i8 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.i32(i32::from(*self));
        Ok(())
    }
}

impl Encode for i16 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.i32(i32::from(*self));
        Ok(())
    }
}

impl Encode for i32 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.i32(*self);
        Ok(())
    }
}

impl Encode for i64 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        if let Ok(value) = i32::try_from(*self) {
            encoder.i32(value);
        } else if *self >= 0 {
            encoder.u64(*self as u64);
        } else {
            return Err(SerError(format!(
                "negative integer {self} is out of range for a MaxMind DB value (minimum is {})",
                i32::MIN
            )));
        }
        Ok(())
    }
}

impl Encode for i128 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        if let Ok(value) = i32::try_from(*self) {
            encoder.i32(value);
        } else if let Ok(value) = u64::try_from(*self) {
            encoder.u64(value);
        } else if let Ok(value) = u128::try_from(*self) {
            encoder.u128(value);
        } else {
            return Err(SerError(format!(
                "negative integer {self} is out of range for a MaxMind DB value (minimum is {})",
                i32::MIN
            )));
        }
        Ok(())
    }
}

impl Encode for isize {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        (*self as i64).encode(encoder)
    }
}

impl Encode for f32 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.f32(*self);
        Ok(())
    }
}

impl Encode for f64 {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.f64(*self);
        Ok(())
    }
}

impl Encode for str {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.str(self)
    }
}

impl Encode for String {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.str(self)
    }
}

impl Encode for char {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.str(&self.to_string())
    }
}

impl Encode for [u8] {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.bytes(self)
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.array(self.len())?;
        for value in self {
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<K: AsRef<str>, V: Encode> Encode for BTreeMap<K, V> {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        encoder.map(self.len())?;
        for (key, value) in self {
            encoder.str(key.as_ref())?;
            value.encode(encoder)?;
        }
        Ok(())
    }
}

/// MaxMind DB has no null value. The derive omits `None` fields; encoding a
/// top-level `None` is an error.
impl<T: Encode> Encode for Option<T> {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        match self {
            Some(value) => value.encode(encoder),
            None => Err(SerError("MaxMind DB has no null type".to_owned())),
        }
    }
}

impl Encode for Value {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerError> {
        match self {
            Value::Pointer(pointer) => {
                encoder.pointer(*pointer);
                Ok(())
            }
            Value::String(value) => encoder.str(value),
            Value::Float64(value) => {
                encoder.f64(*value);
                Ok(())
            }
            Value::Bytes(value) => encoder.bytes(value),
            Value::Uint16(value) => {
                encoder.u16(*value);
                Ok(())
            }
            Value::Uint32(value) => {
                encoder.u32(*value);
                Ok(())
            }
            Value::Int32(value) => {
                encoder.i32(*value);
                Ok(())
            }
            Value::Map(map) => {
                encoder.map(map.len())?;
                for (key, value) in map {
                    encoder.str(key)?;
                    value.encode(encoder)?;
                }
                Ok(())
            }
            Value::Uint64(value) => {
                encoder.u64(*value);
                Ok(())
            }
            Value::Uint128(value) => {
                encoder.u128(*value);
                Ok(())
            }
            Value::Slice(values) => {
                encoder.array(values.len())?;
                for value in values {
                    value.encode(encoder)?;
                }
                Ok(())
            }
            Value::Bool(value) => {
                encoder.bool(*value);
                Ok(())
            }
            Value::Float32(value) => {
                encoder.f32(*value);
                Ok(())
            }
        }
    }
}

/// Largest payload size representable in a MaxMind DB header. The three-byte
/// size extension covers values below this bound.
const MAX_PAYLOAD_SIZE: usize = 65_821 + (1 << 24);

/// Reject payloads whose length cannot be encoded.
fn check_payload_size(len: usize) -> Result<(), SerError> {
    if len >= MAX_PAYLOAD_SIZE {
        Err(SerError(format!(
            "payload of {len} bytes is too large to encode (maximum is {})",
            MAX_PAYLOAD_SIZE - 1
        )))
    } else {
        Ok(())
    }
}

fn ctrl_and_size(w: &mut Vec<u8>, type_num: u8, payload_len: usize) {
    debug_assert!(payload_len < MAX_PAYLOAD_SIZE, "payload size {payload_len} is not encodable");
    let first_byte: u8;
    let second_byte: u8;

    if type_num < 8 {
        first_byte = type_num << 5;
        second_byte = 0;
    } else {
        first_byte = 0;
        second_byte = type_num - 7;
    }

    let size_val = match payload_len {
        s if s < FIRST_SIZE => s as u8,
        s if s < SECOND_SIZE => 29u8,
        s if s < THIRD_SIZE => 30u8,
        _ => 31u8,
    };
    w.push(first_byte | size_val);

    if second_byte != 0 {
        w.push(second_byte);
    }

    match payload_len {
        s if s < FIRST_SIZE => {}
        s if s < SECOND_SIZE => {
            w.push((s - FIRST_SIZE) as u8);
        }
        s if s < THIRD_SIZE => {
            let v = s - SECOND_SIZE;
            w.push((v >> 8) as u8);
            w.push((v & 0xFF) as u8);
        }
        s => {
            let v = s - THIRD_SIZE;
            w.push((v >> 16) as u8);
            w.push((v >> 8) as u8);
            w.push((v & 0xFF) as u8);
        }
    }
}

fn write_str_raw(w: &mut Vec<u8>, s: &str) -> Result<(), SerError> {
    check_payload_size(s.len())?;
    ctrl_and_size(w, 2, s.len());
    w.extend_from_slice(s.as_bytes());
    Ok(())
}

fn write_bytes_raw(w: &mut Vec<u8>, b: &[u8]) -> Result<(), SerError> {
    check_payload_size(b.len())?;
    ctrl_and_size(w, 4, b.len());
    w.extend_from_slice(b);
    Ok(())
}

fn encode_pointer_raw(w: &mut Vec<u8>, pointer: u32) {
    match pointer {
        p if p < 2048 => {
            w.push(0b00100000 | ((p >> 8) as u8 & 0x07));
            w.push((p & 0xFF) as u8);
        }
        p if p < 526336 => {
            let v = p - 2048;
            w.push(0b00101000 | ((v >> 16) as u8 & 0x07));
            w.push((v >> 8) as u8);
            w.push((v & 0xFF) as u8);
        }
        p if p < 134744064 => {
            let v = p - 526336;
            w.push(0b00110000 | ((v >> 24) as u8 & 0x07));
            w.push((v >> 16) as u8);
            w.push((v >> 8) as u8);
            w.push((v & 0xFF) as u8);
        }
        p => {
            w.push(0b00111000);
            w.push((p >> 24) as u8);
            w.push((p >> 16) as u8);
            w.push((p >> 8) as u8);
            w.push((p & 0xFF) as u8);
        }
    }
}

/// Encode `value` into MaxMind DB binary form.
///
/// # Errors
///
/// Returns an error if a payload is larger than the format can encode
/// (`16_843_036` bytes) or if the value cannot be represented.
pub fn to_vec<T: Encode + ?Sized>(value: &T) -> Result<Vec<u8>, SerError> {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.into_bytes())
}

/// Encode `v` into MaxMind DB binary form.
///
/// # Errors
///
/// Returns an error if a payload is larger than the format can encode
/// (`16_843_036` bytes).
pub fn encode_value(v: &Value) -> EncodeResult<Vec<u8>> {
    to_vec(v).map_err(|error| error.0)
}

/// Encode `v` and return the number of bytes it would occupy.
///
/// Returns `0` if `v` cannot be encoded; use [`encode_value`] if you need the
/// error.
pub fn encoded_size(v: &Value) -> usize {
    encode_value(v).map(|b| b.len()).unwrap_or(0)
}
