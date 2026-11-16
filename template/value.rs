//! Dynamic value type used during template rendering and the [`ToValue`] trait.

#[cfg(feature = "std")]
use alloc::sync::Arc;
use alloc::{
    boxed::Box,
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

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

/// Converts a value into a template [`Value`].
///
/// This trait is implemented for scalar, string, option, tuple, collection and
/// string-keyed map types. Derive it with `#[derive(template::ToValue)]` for
/// structs and enums.
pub trait ToValue {
    /// Convert `self` into a template value.
    fn to_value(&self) -> Value;
}

impl<T: ToValue + ?Sized> ToValue for &T {
    fn to_value(&self) -> Value {
        T::to_value(*self)
    }
}

impl<T: ToValue + ?Sized> ToValue for Box<T> {
    fn to_value(&self) -> Value {
        T::to_value(&**self)
    }
}

impl<T: ToValue + ?Sized> ToValue for Rc<T> {
    fn to_value(&self) -> Value {
        T::to_value(&**self)
    }
}

#[cfg(feature = "std")]
impl<T: ToValue + ?Sized> ToValue for Arc<T> {
    fn to_value(&self) -> Value {
        T::to_value(&**self)
    }
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

impl ToValue for bool {
    fn to_value(&self) -> Value {
        Value::Bool(*self)
    }
}

macro_rules! impl_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value(&self) -> Value {
                    Value::I64(*self as i64)
                }
            }
        )*
    };
}

impl_signed!(i8, i16, i32, i64, isize);

impl ToValue for i128 {
    fn to_value(&self) -> Value {
        match i64::try_from(*self) {
            Ok(value) => Value::I64(value),
            Err(_) => Value::F64(*self as f64),
        }
    }
}

macro_rules! impl_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value(&self) -> Value {
                    let value = *self as u128;
                    if value <= i64::MAX as u128 {
                        Value::I64(value as i64)
                    } else {
                        Value::F64(value as f64)
                    }
                }
            }
        )*
    };
}

impl_unsigned!(u8, u16, u32, u64, usize, u128);

impl ToValue for f32 {
    fn to_value(&self) -> Value {
        Value::F64(f64::from(*self))
    }
}

impl ToValue for f64 {
    fn to_value(&self) -> Value {
        Value::F64(*self)
    }
}

impl ToValue for char {
    fn to_value(&self) -> Value {
        Value::Str(Rc::from(self.to_string()))
    }
}

impl ToValue for str {
    fn to_value(&self) -> Value {
        Value::Str(Rc::from(self))
    }
}

impl ToValue for String {
    fn to_value(&self) -> Value {
        Value::Str(Rc::from(self.as_str()))
    }
}

impl ToValue for () {
    fn to_value(&self) -> Value {
        Value::Null
    }
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        match self {
            Some(value) => value.to_value(),
            None => Value::Null,
        }
    }
}

impl<T: ToValue> ToValue for [T] {
    fn to_value(&self) -> Value {
        Value::Array(Rc::new(self.iter().map(ToValue::to_value).collect()))
    }
}

impl<T: ToValue, const N: usize> ToValue for [T; N] {
    fn to_value(&self) -> Value {
        self.as_slice().to_value()
    }
}

impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> Value {
        self.as_slice().to_value()
    }
}

impl<K: AsRef<str>, V: ToValue> ToValue for BTreeMap<K, V> {
    fn to_value(&self) -> Value {
        Value::Map(Rc::new(
            self.iter()
                .map(|(key, value)| (key.as_ref().to_string(), value.to_value()))
                .collect(),
        ))
    }
}

#[cfg(feature = "std")]
impl<K: AsRef<str> + Eq + core::hash::Hash, V: ToValue> ToValue for std::collections::HashMap<K, V> {
    fn to_value(&self) -> Value {
        Value::Map(Rc::new(
            self.iter()
                .map(|(key, value)| (key.as_ref().to_string(), value.to_value()))
                .collect(),
        ))
    }
}

macro_rules! impl_tuple {
    ($($name:ident),+ $(,)?) => {
        impl<$($name: ToValue),+> ToValue for ($($name,)+) {
            #[allow(non_snake_case)]
            fn to_value(&self) -> Value {
                let ($($name,)+) = self;
                Value::Array(Rc::new(Vec::from([$($name.to_value()),+])))
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);

#[cfg(test)]
mod tests {
    use alloc::{collections::BTreeMap, vec};

    use super::*;

    #[test]
    fn test_basic_types() {
        assert_eq!(42i64.to_value(), Value::I64(42));
        assert_eq!(true.to_value(), Value::Bool(true));
        assert_eq!("hello".to_value(), Value::Str("hello".into()));
        assert_eq!(false.to_value(), Value::Bool(false));
    }

    #[test]
    fn test_option() {
        let some_val: Option<i64> = Some(42);
        assert_eq!(some_val.to_value(), Value::I64(42));
        let none_val: Option<i64> = None;
        assert_eq!(none_val.to_value(), Value::Null);
    }

    #[test]
    fn test_vec() {
        let v = vec![1, 2, 3];
        let val = v.to_value();
        match val {
            Value::Array(ref arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::I64(1));
            }
            _ => panic!("expected array"),
        }
    }

    #[test]
    fn test_map() {
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), "Alice".to_string());
        let val = map.to_value();
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

    #[test]
    fn test_u64_overflow() {
        // u64::MAX overflows i64, should convert to f64
        let val = u64::MAX.to_value();
        assert!(matches!(val, Value::F64(_)));
    }

    #[test]
    fn test_large_u64_as_i64() {
        // i64::MAX as u64 should still be I64
        let val = (i64::MAX as u64).to_value();
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
