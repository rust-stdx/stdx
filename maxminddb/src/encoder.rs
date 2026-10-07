use std::{collections::BTreeMap, fmt};

use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeTupleStruct},
};

pub type EncodeResult<T> = Result<T, String>;

#[derive(Debug)]
pub struct SerError(pub String);

impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::error::Error for SerError {}

impl ser::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerError(msg.to_string())
    }
}

const FIRST_SIZE: usize = 29;
const SECOND_SIZE: usize = FIRST_SIZE + 256;
const THIRD_SIZE: usize = SECOND_SIZE + (1 << 16);

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

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MaxMind DB value")
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Value, E> {
                Ok(Value::Bool(v))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
                if let Ok(v) = i32::try_from(v) {
                    Ok(Value::Int32(v))
                } else {
                    Ok(Value::Uint64(v as u64))
                }
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
                Ok(Value::Uint64(v))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
                Ok(Value::Float64(v))
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
                Ok(Value::String(v.to_string()))
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Value, E> {
                Ok(Value::String(v))
            }

            fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Value, E> {
                Ok(Value::Bytes(v.to_vec()))
            }

            fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Value, E> {
                Ok(Value::Bytes(v))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
                let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(1));
                while let Some(elem) = seq.next_element::<Value>()? {
                    items.push(elem);
                }
                Ok(Value::Slice(items))
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
                let mut entries = BTreeMap::new();
                while let Some((key, val)) = map.next_entry::<String, Value>()? {
                    entries.insert(key, val);
                }
                Ok(Value::Map(entries))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

// ---------------------------------------------------------------------------
// ValueSerializer — converts Serialize into owned Value (used by insert_value)
// ---------------------------------------------------------------------------

pub struct ValueSerializer;

impl<'a> ser::Serializer for &'a ValueSerializer {
    type Ok = Value;
    type Error = SerError;

    type SerializeSeq = OwnedMapOrSlice;
    type SerializeTuple = OwnedMapOrSlice;
    type SerializeTupleStruct = OwnedMapOrSlice;
    type SerializeTupleVariant = ser::Impossible<Value, SerError>;
    type SerializeMap = OwnedMapOrSlice;
    type SerializeStruct = OwnedMapOrSlice;
    type SerializeStructVariant = ser::Impossible<Value, SerError>;

    fn serialize_bool(self, v: bool) -> Result<Value, SerError> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Value, SerError> {
        Ok(Value::Int32(v as i32))
    }

    fn serialize_i16(self, v: i16) -> Result<Value, SerError> {
        Ok(Value::Int32(v as i32))
    }

    fn serialize_i32(self, v: i32) -> Result<Value, SerError> {
        Ok(Value::Int32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Value, SerError> {
        i32::try_from(v)
            .map(Value::Int32)
            .or_else(|_| Ok(Value::Uint64(v as u64)))
            .map_err(SerError)
    }

    fn serialize_u8(self, v: u8) -> Result<Value, SerError> {
        Ok(Value::Uint16(v as u16))
    }

    fn serialize_u16(self, v: u16) -> Result<Value, SerError> {
        Ok(Value::Uint16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Value, SerError> {
        Ok(Value::Uint32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Value, SerError> {
        Ok(Value::Uint64(v))
    }

    fn serialize_u128(self, v: u128) -> Result<Value, SerError> {
        Ok(Value::Uint128(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Value, SerError> {
        Ok(Value::Float32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Value, SerError> {
        Ok(Value::Float64(v))
    }

    fn serialize_char(self, v: char) -> Result<Value, SerError> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Value, SerError> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Value, SerError> {
        Ok(Value::Bytes(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Value, SerError> {
        Err(SerError("MaxMind DB has no null type".into()))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Value, SerError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value, SerError> {
        Err(SerError("MaxMind DB has no null type".into()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value, SerError> {
        Err(SerError("MaxMind DB has no null type".into()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value, SerError> {
        Ok(Value::String(variant.to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value, SerError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Value, SerError> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<OwnedMapOrSlice, SerError> {
        Ok(OwnedMapOrSlice::slice())
    }

    fn serialize_tuple(self, len: usize) -> Result<OwnedMapOrSlice, SerError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<OwnedMapOrSlice, SerError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<OwnedMapOrSlice, SerError> {
        Ok(OwnedMapOrSlice::map())
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<OwnedMapOrSlice, SerError> {
        self.serialize_map(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, SerError> {
        Err(SerError("tuple variants not supported".into()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, SerError> {
        Err(SerError("struct variants not supported".into()))
    }
}

pub struct OwnedMapOrSlice {
    pending_key: Option<String>,
    inner: OwnedMapOrSliceInner,
}

enum OwnedMapOrSliceInner {
    Map(BTreeMap<String, Value>),
    Slice(Vec<Value>),
}

impl OwnedMapOrSlice {
    fn map() -> Self {
        OwnedMapOrSlice {
            pending_key: None,
            inner: OwnedMapOrSliceInner::Map(BTreeMap::new()),
        }
    }
    fn slice() -> Self {
        OwnedMapOrSlice {
            pending_key: None,
            inner: OwnedMapOrSliceInner::Slice(Vec::new()),
        }
    }
}

impl SerializeSeq for OwnedMapOrSlice {
    type Ok = Value;
    type Error = SerError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        match self.inner {
            OwnedMapOrSliceInner::Slice(ref mut v) => {
                v.push(value.serialize(&ValueSerializer)?);
                Ok(())
            }
            OwnedMapOrSliceInner::Map(_) => Err(SerError("expected map entry, got element".into())),
        }
    }

    fn end(self) -> Result<Value, SerError> {
        match self.inner {
            OwnedMapOrSliceInner::Slice(v) => Ok(Value::Slice(v)),
            OwnedMapOrSliceInner::Map(m) => Ok(Value::Map(m)),
        }
    }
}

impl ser::SerializeTuple for OwnedMapOrSlice {
    type Ok = Value;
    type Error = SerError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value, SerError> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for OwnedMapOrSlice {
    type Ok = Value;
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value, SerError> {
        SerializeSeq::end(self)
    }
}

impl SerializeMap for OwnedMapOrSlice {
    type Ok = Value;
    type Error = SerError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), SerError> {
        match self.inner {
            OwnedMapOrSliceInner::Slice(_) => Err(SerError("expected element, got map key".into())),
            OwnedMapOrSliceInner::Map(_) => {
                let k = key.serialize(&ValueSerializer)?;
                match k {
                    Value::String(s) => {
                        self.pending_key = Some(s);
                        Ok(())
                    }
                    _ => Err(SerError("map key must be a string".into())),
                }
            }
        }
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        match self.inner {
            OwnedMapOrSliceInner::Slice(_) => Err(SerError("expected element, got map value".into())),
            OwnedMapOrSliceInner::Map(ref mut m) => {
                let key = self
                    .pending_key
                    .take()
                    .ok_or_else(|| SerError("missing map key".into()))?;
                let v = value.serialize(&ValueSerializer)?;
                m.insert(key, v);
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Value, SerError> {
        match self.inner {
            OwnedMapOrSliceInner::Map(m) => Ok(Value::Map(m)),
            OwnedMapOrSliceInner::Slice(_) => Err(SerError("expected map, got slice".into())),
        }
    }
}

impl SerializeStruct for OwnedMapOrSlice {
    type Ok = Value;
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), SerError> {
        match self.inner {
            OwnedMapOrSliceInner::Map(ref mut m) => {
                let v = value.serialize(&ValueSerializer)?;
                m.insert(key.to_string(), v);
                Ok(())
            }
            OwnedMapOrSliceInner::Slice(_) => Err(SerError("expected map, got struct".into())),
        }
    }

    fn end(self) -> Result<Value, SerError> {
        match self.inner {
            OwnedMapOrSliceInner::Map(m) => Ok(Value::Map(m)),
            OwnedMapOrSliceInner::Slice(_) => Err(SerError("expected map, got slice".into())),
        }
    }
}

// ---------------------------------------------------------------------------
// ByteSerializer — writes MaxMind DB binary directly into a Vec<u8>,
//                  avoiding the intermediate Value allocation.
// ---------------------------------------------------------------------------

pub struct ByteSerializer {
    buf: Vec<u8>,
}

impl ByteSerializer {
    pub fn new() -> Self {
        ByteSerializer {
            buf: Vec::new(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

fn ctrl_and_size(w: &mut Vec<u8>, type_num: u8, payload_len: usize) {
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

struct ByteMapOrSlice<'a> {
    ser: &'a mut ByteSerializer,
    state: ByteMOS,
    pending_key: Option<String>,
}

enum ByteMOS {
    Map(BTreeMap<String, Vec<u8>>),
    Slice(Vec<Vec<u8>>),
}

impl ByteMapOrSlice<'_> {
    fn flush_map(self) -> Result<(), SerError> {
        match self.state {
            ByteMOS::Map(m) => {
                ctrl_and_size(&mut self.ser.buf, 7, m.len());
                for (k, v) in m {
                    write_str_raw(&mut self.ser.buf, &k);
                    self.ser.buf.extend_from_slice(&v);
                }
                Ok(())
            }
            ByteMOS::Slice(s) => {
                ctrl_and_size(&mut self.ser.buf, 11, s.len());
                for v in s {
                    self.ser.buf.extend_from_slice(&v);
                }
                Ok(())
            }
        }
    }
}

fn write_str_raw(w: &mut Vec<u8>, s: &str) {
    ctrl_and_size(w, 2, s.len());
    w.extend_from_slice(s.as_bytes());
}

fn write_bytes_raw(w: &mut Vec<u8>, b: &[u8]) {
    ctrl_and_size(w, 4, b.len());
    w.extend_from_slice(b);
}

impl SerializeSeq for ByteMapOrSlice<'_> {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        let mut sub = ByteSerializer::new();
        value.serialize(ByteSerializerRef(&mut sub))?;
        match self.state {
            ByteMOS::Slice(ref mut v) => v.push(sub.buf),
            ByteMOS::Map(_) => return Err(SerError("expected map entry, got element".into())),
        }
        Ok(())
    }

    fn end(self) -> Result<(), SerError> {
        self.flush_map()
    }
}

impl ser::SerializeTuple for ByteMapOrSlice<'_> {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), SerError> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for ByteMapOrSlice<'_> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), SerError> {
        SerializeSeq::end(self)
    }
}

impl SerializeMap for ByteMapOrSlice<'_> {
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), SerError> {
        match self.state {
            ByteMOS::Slice(_) => Err(SerError("expected element, got map key".into())),
            ByteMOS::Map(_) => {
                let mut sub = ByteSerializer::new();
                key.serialize(ByteSerializerRef(&mut sub))?;
                // let raw = sub.buf;
                // let s = String::new();
                // decode the key from the encoded bytes
                // simplest: serialize through ValueSerializer then extract
                let k = key.serialize(&ValueSerializer)?;
                match k {
                    Value::String(s_val) => {
                        self.pending_key = Some(s_val);
                        Ok(())
                    }
                    _ => Err(SerError("map key must be a string".into())),
                }
            }
        }
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerError> {
        match self.state {
            ByteMOS::Slice(_) => Err(SerError("expected element, got map value".into())),
            ByteMOS::Map(ref mut m) => {
                let key = self
                    .pending_key
                    .take()
                    .ok_or_else(|| SerError("missing map key".into()))?;
                let mut sub = ByteSerializer::new();
                value.serialize(ByteSerializerRef(&mut sub))?;
                m.insert(key, sub.buf);
                Ok(())
            }
        }
    }

    fn end(self) -> Result<(), SerError> {
        self.flush_map()
    }
}

impl SerializeStruct for ByteMapOrSlice<'_> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), SerError> {
        match self.state {
            ByteMOS::Map(ref mut m) => {
                let mut sub = ByteSerializer::new();
                value.serialize(ByteSerializerRef(&mut sub))?;
                m.insert(key.to_string(), sub.buf);
                Ok(())
            }
            ByteMOS::Slice(_) => Err(SerError("expected map, got struct".into())),
        }
    }

    fn end(self) -> Result<(), SerError> {
        self.flush_map()
    }
}

struct ByteSerializerRef<'a>(&'a mut ByteSerializer);

impl<'a> ser::Serializer for ByteSerializerRef<'a> {
    type Ok = ();
    type Error = SerError;

    type SerializeSeq = ByteMapOrSlice<'a>;
    type SerializeTuple = ByteMapOrSlice<'a>;
    type SerializeTupleStruct = ByteMapOrSlice<'a>;
    type SerializeTupleVariant = ser::Impossible<(), SerError>;
    type SerializeMap = ByteMapOrSlice<'a>;
    type SerializeStruct = ByteMapOrSlice<'a>;
    type SerializeStructVariant = ser::Impossible<(), SerError>;

    fn serialize_bool(self, v: bool) -> Result<(), SerError> {
        let payload_len = if v { 1 } else { 0 };
        ctrl_and_size(&mut self.0.buf, 14, payload_len);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), SerError> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i16(self, v: i16) -> Result<(), SerError> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i32(self, v: i32) -> Result<(), SerError> {
        let u = v as u32;
        let size = 4 - (u.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.0.buf, 8, size);
        for i in (0..size).rev() {
            self.0.buf.push((u >> (8 * i)) as u8);
        }
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), SerError> {
        if let Ok(v) = i32::try_from(v) {
            self.serialize_i32(v)
        } else {
            self.serialize_u64(v as u64)
        }
    }

    fn serialize_u8(self, v: u8) -> Result<(), SerError> {
        self.serialize_u16(v as u16)
    }

    fn serialize_u16(self, v: u16) -> Result<(), SerError> {
        let size = 2 - (v.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.0.buf, 5, size);
        for i in (0..size).rev() {
            self.0.buf.push((v >> (8 * i)) as u8);
        }
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), SerError> {
        let size = 4 - (v.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.0.buf, 6, size);
        for i in (0..size).rev() {
            self.0.buf.push((v >> (8 * i)) as u8);
        }
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), SerError> {
        let size = 8 - (v.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.0.buf, 9, size);
        for i in (0..size).rev() {
            self.0.buf.push((v >> (8 * i)) as u8);
        }
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<(), SerError> {
        let size = 16 - (v.leading_zeros() as usize / 8);
        ctrl_and_size(&mut self.0.buf, 10, size);
        for i in (0..size).rev() {
            self.0.buf.push((v >> (8 * i)) as u8);
        }
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), SerError> {
        ctrl_and_size(&mut self.0.buf, 15, 4);
        self.0.buf.extend_from_slice(&v.to_bits().to_be_bytes());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), SerError> {
        ctrl_and_size(&mut self.0.buf, 3, 8);
        self.0.buf.extend_from_slice(&v.to_bits().to_be_bytes());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), SerError> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<(), SerError> {
        write_str_raw(&mut self.0.buf, v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), SerError> {
        write_bytes_raw(&mut self.0.buf, v);
        Ok(())
    }

    fn serialize_none(self) -> Result<(), SerError> {
        Err(SerError("MaxMind DB has no null type".into()))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), SerError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), SerError> {
        Err(SerError("MaxMind DB has no null type".into()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), SerError> {
        Err(SerError("MaxMind DB has no null type".into()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), SerError> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<(), SerError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<(), SerError> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<ByteMapOrSlice<'a>, SerError> {
        Ok(ByteMapOrSlice {
            ser: self.0,
            state: ByteMOS::Slice(Vec::new()),
            pending_key: None,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<ByteMapOrSlice<'a>, SerError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<ByteMapOrSlice<'a>, SerError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<ByteMapOrSlice<'a>, SerError> {
        Ok(ByteMapOrSlice {
            ser: self.0,
            state: ByteMOS::Map(BTreeMap::new()),
            pending_key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<ByteMapOrSlice<'a>, SerError> {
        self.serialize_map(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, SerError> {
        Err(SerError("tuple variants not supported".into()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, SerError> {
        Err(SerError("struct variants not supported".into()))
    }
}

/// Serialize a `Serialize` value directly into MaxMind DB binary format.
/// Returns the encoded bytes. This avoids allocating an intermediate `Value`.
pub fn encode_serialize(v: &impl Serialize) -> Result<Vec<u8>, SerError> {
    let mut ser = ByteSerializer::new();
    v.serialize(ByteSerializerRef(&mut ser))?;
    Ok(ser.buf)
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

impl ByteSerializerRef<'_> {
    fn serialize_value(self, v: &Value) -> Result<(), SerError> {
        match v {
            Value::Pointer(p) => {
                encode_pointer_raw(&mut self.0.buf, *p);
                Ok(())
            }
            Value::String(s) => {
                write_str_raw(&mut self.0.buf, s);
                Ok(())
            }
            Value::Float64(f) => {
                ctrl_and_size(&mut self.0.buf, 3, 8);
                self.0.buf.extend_from_slice(&f.to_bits().to_be_bytes());
                Ok(())
            }
            Value::Bytes(b) => {
                write_bytes_raw(&mut self.0.buf, b);
                Ok(())
            }
            Value::Uint16(n) => {
                let size = 2 - (n.leading_zeros() as usize / 8);
                ctrl_and_size(&mut self.0.buf, 5, size);
                for i in (0..size).rev() {
                    self.0.buf.push((n >> (8 * i)) as u8);
                }
                Ok(())
            }
            Value::Uint32(n) => {
                let size = 4 - (n.leading_zeros() as usize / 8);
                ctrl_and_size(&mut self.0.buf, 6, size);
                for i in (0..size).rev() {
                    self.0.buf.push((n >> (8 * i)) as u8);
                }
                Ok(())
            }
            Value::Int32(n) => {
                let u = *n as u32;
                let size = 4 - (u.leading_zeros() as usize / 8);
                ctrl_and_size(&mut self.0.buf, 8, size);
                for i in (0..size).rev() {
                    self.0.buf.push((u >> (8 * i)) as u8);
                }
                Ok(())
            }
            Value::Uint64(n) => {
                let size = 8 - (n.leading_zeros() as usize / 8);
                ctrl_and_size(&mut self.0.buf, 9, size);
                for i in (0..size).rev() {
                    self.0.buf.push((n >> (8 * i)) as u8);
                }
                Ok(())
            }
            Value::Uint128(n) => {
                let size = 16 - (n.leading_zeros() as usize / 8);
                ctrl_and_size(&mut self.0.buf, 10, size);
                for i in (0..size).rev() {
                    self.0.buf.push((n >> (8 * i)) as u8);
                }
                Ok(())
            }
            Value::Map(m) => {
                ctrl_and_size(&mut self.0.buf, 7, m.len());
                for (k, val) in m.iter() {
                    write_str_raw(&mut self.0.buf, k);
                    ByteSerializerRef(&mut *self.0).serialize_value(val)?;
                }
                Ok(())
            }
            Value::Slice(s) => {
                ctrl_and_size(&mut self.0.buf, 11, s.len());
                for val in s {
                    ByteSerializerRef(&mut *self.0).serialize_value(val)?;
                }
                Ok(())
            }
            Value::Bool(b) => {
                let payload_len = if *b { 1 } else { 0 };
                ctrl_and_size(&mut self.0.buf, 14, payload_len);
                Ok(())
            }
            Value::Float32(f) => {
                ctrl_and_size(&mut self.0.buf, 15, 4);
                self.0.buf.extend_from_slice(&f.to_bits().to_be_bytes());
                Ok(())
            }
        }
    }
}

pub fn encode_value(v: &Value) -> EncodeResult<Vec<u8>> {
    let mut ser = ByteSerializer::new();
    ByteSerializerRef(&mut ser).serialize_value(v).map_err(|e| e.0)?;
    Ok(ser.into_bytes())
}

pub fn encoded_size(v: &Value) -> usize {
    encode_value(v).map(|b| b.len()).unwrap_or(0)
}
