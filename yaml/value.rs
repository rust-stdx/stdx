//! The dynamic YAML value model.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt;

/// A YAML number, whether integer or floating point.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Number {
    /// A non-negative integer.
    PosInt(u64),
    /// A negative integer.
    NegInt(i64),
    /// A floating-point number, which may be infinite or NaN.
    Float(f64),
}

impl Number {
    /// Returns true for an integer that fits in `i64`.
    pub fn is_i64(&self) -> bool {
        match self {
            Number::PosInt(value) => *value <= i64::MAX as u64,
            Number::NegInt(_) => true,
            Number::Float(_) => false,
        }
    }

    /// Returns true for a non-negative integer that fits in `u64`.
    pub fn is_u64(&self) -> bool {
        matches!(self, Number::PosInt(_))
    }

    /// Returns true for a floating-point number.
    pub fn is_f64(&self) -> bool {
        matches!(self, Number::Float(_))
    }

    /// Returns the value as an `i64` when it is an integer in range.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::PosInt(value) => i64::try_from(*value).ok(),
            Number::NegInt(value) => Some(*value),
            Number::Float(_) => None,
        }
    }

    /// Returns the value as a `u64` when it is a non-negative integer in range.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Number::PosInt(value) => Some(*value),
            Number::NegInt(value) => u64::try_from(*value).ok(),
            Number::Float(_) => None,
        }
    }

    /// Returns the value as an `f64`.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Number::PosInt(value) => Some(*value as f64),
            Number::NegInt(value) => Some(*value as f64),
            Number::Float(value) => Some(*value),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::PosInt(value) => write!(formatter, "{value}"),
            Number::NegInt(value) => write!(formatter, "{value}"),
            Number::Float(value) => write!(formatter, "{value}"),
        }
    }
}

macro_rules! impl_from_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl From<$ty> for Number {
                fn from(value: $ty) -> Self {
                    Number::PosInt(value as u64)
                }
            }
        )*
    };
}

macro_rules! impl_from_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl From<$ty> for Number {
                fn from(value: $ty) -> Self {
                    if value < 0 {
                        Number::NegInt(value as i64)
                    } else {
                        Number::PosInt(value as u64)
                    }
                }
            }
        )*
    };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(value as f64)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

/// A YAML mapping that preserves insertion order.
///
/// Keys and values are arbitrary [`Value`]s, as required by YAML.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mapping {
    entries: Vec<(Value, Value)>,
}

impl Mapping {
    /// Creates an empty mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty mapping with the given initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the mapping is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inserts a key-value pair, replacing an existing value for an equal key.
    pub fn insert(&mut self, key: Value, value: Value) -> Option<Value> {
        if let Some((_, existing)) = self.entries.iter_mut().find(|(existing, _)| *existing == key) {
            return Some(core::mem::replace(existing, value));
        }
        self.entries.push((key, value));
        None
    }

    /// Returns the value for `key`.
    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.entries
            .iter()
            .find(|(existing, _)| existing == key)
            .map(|(_, value)| value)
    }

    /// Returns the value for a string key.
    pub fn get_str(&self, key: &str) -> Option<&Value> {
        self.entries
            .iter()
            .find(|(existing, _)| existing.as_str() == Some(key))
            .map(|(_, value)| value)
    }

    /// Returns the mutable value for a string key.
    pub fn get_str_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.entries
            .iter_mut()
            .find(|(existing, _)| existing.as_str() == Some(key))
            .map(|(_, value)| value)
    }

    /// Removes and returns the value for `key`.
    pub fn remove(&mut self, key: &Value) -> Option<Value> {
        let index = self.entries.iter().position(|(existing, _)| existing == key)?;
        Some(self.entries.remove(index).1)
    }

    /// Iterates over the entries in insertion order.
    pub fn iter(&self) -> core::slice::Iter<'_, (Value, Value)> {
        self.entries.iter()
    }
}

impl core::ops::Index<&str> for Mapping {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        self.get_str(key)
            .unwrap_or_else(|| panic!("no entry found for key `{key}`"))
    }
}

impl core::ops::Index<&str> for Value {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("no entry found for key `{key}`"))
    }
}

impl core::ops::Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(values) = self.as_sequence()
            && let Some(value) = values.get(index)
        {
            return value;
        }
        if let Some(mapping) = self.as_mapping()
            && let Some(value) = mapping.get(&Value::Number(Number::PosInt(index as u64)))
        {
            return value;
        }
        panic!("no entry found for index {index}")
    }
}

impl FromIterator<(Value, Value)> for Mapping {
    fn from_iter<T: IntoIterator<Item = (Value, Value)>>(iter: T) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}

/// A value annotated with a YAML tag.
#[derive(Clone, Debug, PartialEq)]
pub struct TaggedValue {
    /// The tag, without the leading `!`.
    pub tag: String,
    /// The tagged value.
    pub value: Value,
}

/// Any valid YAML value.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    /// `null`.
    #[default]
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// An integer or floating-point number.
    Number(Number),
    /// A string.
    String(String),
    /// A sequence.
    Sequence(Vec<Value>),
    /// A mapping.
    Mapping(Mapping),
    /// A value carrying an explicit YAML tag.
    Tagged(Box<TaggedValue>),
}

impl Value {
    /// Returns the YAML type name used in error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Sequence(_) => "sequence",
            Value::Mapping(_) => "mapping",
            Value::Tagged(_) => "tagged value",
        }
    }

    /// Returns true for `null`.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns the boolean value, if any.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the number, if any.
    pub fn as_number(&self) -> Option<Number> {
        match self {
            Value::Number(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the value as an `i64`, if it is an integer in range.
    pub fn as_i64(&self) -> Option<i64> {
        self.as_number()?.as_i64()
    }

    /// Returns the value as a `u64`, if it is a non-negative integer in range.
    pub fn as_u64(&self) -> Option<u64> {
        self.as_number()?.as_u64()
    }

    /// Returns the value as an `f64`, if it is a number.
    pub fn as_f64(&self) -> Option<f64> {
        self.as_number()?.as_f64()
    }

    /// Returns the string, if any.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the sequence, if any.
    pub fn as_sequence(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Sequence(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the mapping, if any.
    pub fn as_mapping(&self) -> Option<&Mapping> {
        match self {
            Value::Mapping(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the mapping mutably, if any.
    pub fn as_mapping_mut(&mut self) -> Option<&mut Mapping> {
        match self {
            Value::Mapping(value) => Some(value),
            _ => None,
        }
    }

    /// Returns true for a sequence.
    pub fn is_sequence(&self) -> bool {
        matches!(self, Value::Sequence(_))
    }

    /// Returns true for a mapping.
    pub fn is_mapping(&self) -> bool {
        matches!(self, Value::Mapping(_))
    }

    /// Returns true for a string.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns true for a number.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Returns true for a boolean.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns the mapping value for a string key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_mapping()?.get_str(key)
    }

    /// Returns the mutable mapping value for a string key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.as_mapping_mut()?.get_str_mut(key)
    }

    /// Removes a wrapper tag and returns the inner value.
    pub fn untag(&self) -> &Value {
        match self {
            Value::Tagged(tagged) => tagged.value.untag(),
            value => value,
        }
    }

    /// Converts the value into its untagged inner value.
    pub fn into_untagged(self) -> Value {
        match self {
            Value::Tagged(tagged) => tagged.value.into_untagged(),
            value => value,
        }
    }
}

macro_rules! impl_value_from {
    ($($ty:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$ty> for Value {
                fn from(value: $ty) -> Self {
                    Value::$variant(value.into())
                }
            }
        )*
    };
}

impl_value_from!(
    bool => Bool,
    u8 => Number,
    u16 => Number,
    u32 => Number,
    u64 => Number,
    usize => Number,
    i8 => Number,
    i16 => Number,
    i32 => Number,
    i64 => Number,
    isize => Number,
    f32 => Number,
    f64 => Number,
);

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(String::from(value))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Sequence(value)
    }
}

impl From<Mapping> for Value {
    fn from(value: Mapping) -> Self {
        Value::Mapping(value)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        value.map_or(Value::Null, Into::into)
    }
}
