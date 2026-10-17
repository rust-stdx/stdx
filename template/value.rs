//! Dynamic value type used during template rendering, with serde serialization support.

#[cfg(any(feature = "std", test))]
use alloc::string::ToString;
use alloc::{collections::BTreeMap, rc::Rc, string::String, vec::Vec};
use core::fmt;

#[cfg(feature = "std")]
use serde::ser::{self, Serialize, Serializer};

#[cfg(feature = "std")]
use crate::SerdeError;

/// A dynamic value used during template rendering.
///
/// Supports strings, numbers, booleans, null, arrays, maps, and
/// `Safe` strings that bypass auto-escaping.
///
/// This type is not meant to be used directly. Use the [`context!`] macro
/// or the [`IntoContext`](crate::context::IntoContext) trait instead.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    /// A string value. Will be auto-escaped in [`EscapeMode::Html`](crate::EscapeMode).
    Str(Rc<str>),
    /// A string that is already safe for output. Bypasses auto-escaping.
    /// Returned by `{{ value | safe }}`, the `escape` filter, and `{{ super() }}`.
    Safe(Rc<str>),
    Array(Rc<Vec<Value>>),
    Map(Rc<BTreeMap<String, Value>>),
}

impl PartialEq for Value {
    /// Compares values structurally.
    ///
    /// [`Value::Safe`] and [`Value::Str`] compare equal when their contents are
    /// equal, so `{{ value | safe }} == value` holds for the same string.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::I64(a), Value::I64(b)) => a == b,
            (Value::F64(a), Value::F64(b)) => a == b,
            (Value::Str(a) | Value::Safe(a), Value::Str(b) | Value::Safe(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            _ => false,
        }
    }
}

impl Value {
    #[cfg(feature = "std")]
    fn from_serialize_impl<S: Serialize>(value: &S) -> Result<Self, SerdeError> {
        let serializer = ValueSerializer;
        value.serialize(serializer)
    }

    /// Look up a key in a map value. Returns `None` if not a map or key missing.
    pub fn get(&self, key: &str) -> Option<Value> {
        match self {
            Value::Map(m) => m.get(key).cloned(),
            _ => None,
        }
    }

    /// Look up an index in an array value. Returns `None` if not an array or index out of bounds.
    pub fn get_index(&self, index: usize) -> Option<Value> {
        match self {
            Value::Array(a) => a.get(index).cloned(),
            _ => None,
        }
    }

    /// Return the string content if this is a `Str` or `Safe` value.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            Value::Safe(s) => Some(s),
            _ => None,
        }
    }

    /// Format the value into the given buffer.
    ///
    /// Arrays and maps render as `[Array]` / `[Object]` placeholders.
    pub fn fmt_to(&self, buf: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Value::Null => Ok(()),
            Value::Bool(b) => write!(buf, "{b}"),
            Value::I64(n) => write!(buf, "{n}"),
            Value::F64(n) => write!(buf, "{n}"),
            Value::Str(s) | Value::Safe(s) => buf.write_str(s),
            Value::Array(_) => write!(buf, "[Array]"),
            Value::Map(_) => write!(buf, "[Object]"),
        }
    }
}

#[cfg(feature = "std")]
struct ValueSerializer;

#[cfg(feature = "std")]
impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = SerdeError;

    type SerializeSeq = ValueSeqSerializer;
    type SerializeTuple = ValueSeqSerializer;
    type SerializeTupleStruct = ValueSeqSerializer;
    type SerializeTupleVariant = ValueSeqSerializer;
    type SerializeMap = ValueMapSerializer;
    type SerializeStruct = ValueMapSerializer;
    type SerializeStructVariant = ValueMapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Value, SerdeError> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Value, SerdeError> {
        Ok(Value::I64(v as i64))
    }

    fn serialize_i16(self, v: i16) -> Result<Value, SerdeError> {
        Ok(Value::I64(v as i64))
    }

    fn serialize_i32(self, v: i32) -> Result<Value, SerdeError> {
        Ok(Value::I64(v as i64))
    }

    fn serialize_i64(self, v: i64) -> Result<Value, SerdeError> {
        Ok(Value::I64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Value, SerdeError> {
        Ok(Value::I64(v as i64))
    }

    fn serialize_u16(self, v: u16) -> Result<Value, SerdeError> {
        Ok(Value::I64(v as i64))
    }

    fn serialize_u32(self, v: u32) -> Result<Value, SerdeError> {
        Ok(Value::I64(v as i64))
    }

    fn serialize_u64(self, v: u64) -> Result<Value, SerdeError> {
        if v <= i64::MAX as u64 {
            Ok(Value::I64(v as i64))
        } else {
            Ok(Value::F64(v as f64))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Value, SerdeError> {
        Ok(Value::F64(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Value, SerdeError> {
        Ok(Value::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Value, SerdeError> {
        let mut s = String::with_capacity(1);
        s.push(v);
        Ok(Value::Str(s.into()))
    }

    fn serialize_str(self, v: &str) -> Result<Value, SerdeError> {
        Ok(Value::Str(v.into()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Value, SerdeError> {
        Err(SerdeError("bytes not supported".into()))
    }

    fn serialize_none(self) -> Result<Value, SerdeError> {
        Ok(Value::Null)
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Value, SerdeError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value, SerdeError> {
        Ok(Value::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value, SerdeError> {
        Ok(Value::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value, SerdeError> {
        Ok(Value::Str(variant.into()))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value, SerdeError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Value, SerdeError> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<ValueSeqSerializer, SerdeError> {
        Ok(ValueSeqSerializer(Vec::new()))
    }

    fn serialize_tuple(self, len: usize) -> Result<ValueSeqSerializer, SerdeError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<ValueSeqSerializer, SerdeError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<ValueSeqSerializer, SerdeError> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<ValueMapSerializer, SerdeError> {
        Ok(ValueMapSerializer(BTreeMap::new(), None))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<ValueMapSerializer, SerdeError> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<ValueMapSerializer, SerdeError> {
        self.serialize_map(Some(len))
    }
}

#[cfg(feature = "std")]
struct ValueSeqSerializer(Vec<Value>);

#[cfg(feature = "std")]
impl ser::SerializeSeq for ValueSeqSerializer {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerdeError> {
        self.0.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, SerdeError> {
        Ok(Value::Array(Rc::new(self.0)))
    }
}

#[cfg(feature = "std")]
impl ser::SerializeTuple for ValueSeqSerializer {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerdeError> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value, SerdeError> {
        ser::SerializeSeq::end(self)
    }
}

#[cfg(feature = "std")]
impl ser::SerializeTupleStruct for ValueSeqSerializer {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerdeError> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value, SerdeError> {
        ser::SerializeSeq::end(self)
    }
}

#[cfg(feature = "std")]
impl ser::SerializeTupleVariant for ValueSeqSerializer {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerdeError> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value, SerdeError> {
        ser::SerializeSeq::end(self)
    }
}

#[cfg(feature = "std")]
struct ValueMapSerializer(BTreeMap<String, Value>, Option<String>);

#[cfg(feature = "std")]
impl ser::SerializeMap for ValueMapSerializer {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), SerdeError> {
        let k = key.serialize(MapKeySerializer)?;
        self.1 = Some(k);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), SerdeError> {
        let key = self
            .1
            .take()
            .ok_or_else(|| SerdeError("serialize_value called without serialize_key".into()))?;
        let v = value.serialize(ValueSerializer)?;
        self.0.insert(key, v);
        Ok(())
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), SerdeError> {
        let k = key.serialize(MapKeySerializer)?;
        let v = value.serialize(ValueSerializer)?;
        self.0.insert(k, v);
        Ok(())
    }

    fn end(self) -> Result<Value, SerdeError> {
        Ok(Value::Map(Rc::new(self.0)))
    }
}

#[cfg(feature = "std")]
impl ser::SerializeStruct for ValueMapSerializer {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), SerdeError> {
        let v = value.serialize(ValueSerializer)?;
        self.0.insert(key.to_string(), v);
        Ok(())
    }

    fn end(self) -> Result<Value, SerdeError> {
        Ok(Value::Map(Rc::new(self.0)))
    }
}

#[cfg(feature = "std")]
impl ser::SerializeStructVariant for ValueMapSerializer {
    type Ok = Value;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), SerdeError> {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Value, SerdeError> {
        ser::SerializeStruct::end(self)
    }
}

#[cfg(feature = "std")]
struct MapKeySerializer;

#[cfg(feature = "std")]
impl Serializer for MapKeySerializer {
    type Ok = String;
    type Error = SerdeError;

    type SerializeSeq = ser::Impossible<String, SerdeError>;
    type SerializeTuple = ser::Impossible<String, SerdeError>;
    type SerializeTupleStruct = ser::Impossible<String, SerdeError>;
    type SerializeTupleVariant = ser::Impossible<String, SerdeError>;
    type SerializeMap = ser::Impossible<String, SerdeError>;
    type SerializeStruct = ser::Impossible<String, SerdeError>;
    type SerializeStructVariant = ser::Impossible<String, SerdeError>;

    fn serialize_bool(self, v: bool) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_i8(self, v: i8) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_i16(self, v: i16) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_i32(self, v: i32) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_i64(self, v: i64) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_u8(self, v: u8) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_u16(self, v: u16) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_u32(self, v: u32) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_u64(self, v: u64) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_f32(self, v: f32) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_f64(self, v: f64) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_char(self, v: char) -> Result<String, SerdeError> {
        let mut s = String::with_capacity(1);
        s.push(v);
        Ok(s)
    }
    fn serialize_str(self, v: &str) -> Result<String, SerdeError> {
        Ok(v.to_string())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<String, SerdeError> {
        Err(SerdeError("bytes not supported as map key".into()))
    }
    fn serialize_none(self) -> Result<String, SerdeError> {
        Err(SerdeError("none not supported as map key".into()))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<String, SerdeError> {
        Err(SerdeError("some not supported as map key".into()))
    }
    fn serialize_unit(self) -> Result<String, SerdeError> {
        Ok(String::new())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, SerdeError> {
        Ok(String::new())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String, SerdeError> {
        Ok(variant.to_string())
    }
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<String, SerdeError> {
        Err(SerdeError("newtype struct not supported as map key".into()))
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String, SerdeError> {
        Err(SerdeError("newtype variant not supported as map key".into()))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, SerdeError> {
        Err(SerdeError("sequence not supported as map key".into()))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, SerdeError> {
        Err(SerdeError("tuple not supported as map key".into()))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, SerdeError> {
        Err(SerdeError("tuple struct not supported as map key".into()))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, SerdeError> {
        Err(SerdeError("tuple variant not supported as map key".into()))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, SerdeError> {
        Err(SerdeError("map not supported as map key".into()))
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, SerdeError> {
        Err(SerdeError("struct not supported as map key".into()))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, SerdeError> {
        Err(SerdeError("struct variant not supported as map key".into()))
    }
}

macro_rules! from_value {
    ($($ty:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$ty> for Value {
                fn from(v: $ty) -> Self {
                    Value::$variant(v)
                }
            }
        )*
    };
}

from_value! {
    bool => Bool,
    i64 => I64,
    f64 => F64,
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.into())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s.into())
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::I64(v as i64)
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        if v <= i64::MAX as usize {
            Value::I64(v as i64)
        } else {
            Value::F64(v as f64)
        }
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Value::Array(Rc::new(v.into_iter().map(Into::into).collect()))
    }
}

impl<T: Into<Value>> From<BTreeMap<String, T>> for Value {
    fn from(v: BTreeMap<String, T>) -> Self {
        Value::Map(Rc::new(v.into_iter().map(|(k, v)| (k, v.into())).collect()))
    }
}

impl From<Value> for String {
    fn from(v: Value) -> Self {
        let mut buf = String::new();
        v.fmt_to(&mut buf).unwrap();
        buf
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_to(f)
    }
}

#[cfg(feature = "std")]
pub(crate) fn to_value<S: Serialize>(value: S) -> Result<Value, SerdeError> {
    Value::from_serialize_impl(&value)
}

#[cfg(test)]
mod tests {
    use alloc::{collections::BTreeMap, vec};

    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_basic_types() {
        assert_eq!(to_value(42i64).unwrap(), Value::I64(42));
        assert_eq!(to_value(true).unwrap(), Value::Bool(true));
        assert_eq!(to_value("hello").unwrap(), Value::Str("hello".into()));
        assert_eq!(to_value(false).unwrap(), Value::Bool(false));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_option() {
        let some_val: Option<i64> = Some(42);
        assert_eq!(to_value(some_val).unwrap(), Value::I64(42));
        let none_val: Option<i64> = None;
        assert_eq!(to_value(none_val).unwrap(), Value::Null);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3];
        let val = to_value(v).unwrap();
        match val {
            Value::Array(ref arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::I64(1));
            }
            _ => panic!("expected array"),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_map() {
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), "Alice".to_string());
        let val = to_value(map).unwrap();
        match val {
            Value::Map(ref m) => {
                assert_eq!(m.get("name"), Some(&Value::Str("Alice".into())));
            }
            _ => panic!("expected map"),
        }
    }

    #[test]
    fn test_struct() {
        let user = BTreeMap::from([
            ("name".to_string(), Value::Str("Bob".into())),
            ("age".to_string(), Value::I64(30)),
        ]);
        let val = Value::Map(Rc::new(user));
        match val {
            Value::Map(ref m) => {
                assert_eq!(m.get("name"), Some(&Value::Str("Bob".into())));
                assert_eq!(m.get("age"), Some(&Value::I64(30)));
            }
            _ => panic!("expected map"),
        }
    }

    #[test]
    fn test_get() {
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), Value::I64(42));
        let val = Value::Map(Rc::new(map));
        assert_eq!(val.get("key"), Some(Value::I64(42)));
        assert_eq!(val.get("nonexistent"), None);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_u64_overflow() {
        // u64::MAX overflows i64, should serialize to f64
        let val = to_value(u64::MAX).unwrap();
        assert!(matches!(val, Value::F64(_)));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_large_u64_as_i64() {
        // i64::MAX as u64 should still be I64
        let val = to_value(i64::MAX as u64).unwrap();
        assert_eq!(val, Value::I64(i64::MAX));
    }

    #[test]
    fn test_safe_variant_as_str() {
        let val = Value::Safe("hello".into());
        assert_eq!(val.as_str(), Some("hello"));
    }

    #[test]
    fn test_safe_variant_fmt() {
        let val = Value::Safe("hello".into());
        let mut buf = String::new();
        val.fmt_to(&mut buf).unwrap();
        assert_eq!(buf, "hello");
    }

    #[test]
    fn test_safe_equals_str() {
        // `Safe` and `Str` with identical contents must compare equal.
        assert_eq!(Value::Safe("x".into()), Value::Str("x".into()));
        assert_eq!(Value::Str("x".into()), Value::Safe("x".into()));
        assert_ne!(Value::Safe("x".into()), Value::Str("y".into()));
        assert_ne!(Value::Safe("x".into()), Value::Safe("y".into()));
    }

    #[test]
    fn test_safe_nested_equality() {
        let a = Value::Array(Rc::new(vec![Value::Safe("x".into())]));
        let b = Value::Array(Rc::new(vec![Value::Str("x".into())]));
        assert_eq!(a, b);
    }

    #[test]
    fn test_get_on_non_map() {
        let val = Value::I64(42);
        assert_eq!(val.get("key"), None);
    }

    #[test]
    fn test_get_index_on_non_array() {
        let val = Value::I64(42);
        assert_eq!(val.get_index(0), None);
    }

    #[test]
    fn test_get_index_out_of_bounds() {
        let val = Value::Array(Rc::new(vec![Value::I64(1)]));
        assert_eq!(val.get_index(5), None);
    }

    #[test]
    fn test_as_str_on_non_string() {
        let val = Value::I64(42);
        assert_eq!(val.as_str(), None);
    }

    #[test]
    fn test_fmt_to_null() {
        let val = Value::Null;
        let mut buf = String::new();
        val.fmt_to(&mut buf).unwrap();
        assert_eq!(buf, "");
    }

    #[test]
    fn test_fmt_to_array() {
        let val = Value::Array(Rc::new(vec![Value::I64(1)]));
        let mut buf = String::new();
        val.fmt_to(&mut buf).unwrap();
        assert_eq!(buf, "[Array]");
    }

    #[test]
    fn test_fmt_to_map() {
        let mut m = BTreeMap::new();
        m.insert("k".to_string(), Value::I64(1));
        let val = Value::Map(Rc::new(m));
        let mut buf = String::new();
        val.fmt_to(&mut buf).unwrap();
        assert_eq!(buf, "[Object]");
    }
}
