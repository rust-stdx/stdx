//! The dynamic YAML value model.

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use core::{cmp::Ordering, fmt};

/// A YAML number, whether integer or floating point.
///
/// Equality follows YAML's canonical form: all `NaN` values compare equal to
/// each other and `-0.0` compares equal to `0.0`.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    /// A non-negative integer.
    PosInt(u64),
    /// A negative integer.
    NegInt(i64),
    /// A floating-point number, which may be infinite or NaN.
    Float(f64),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::PosInt(left), Number::PosInt(right)) => left == right,
            (Number::NegInt(left), Number::NegInt(right)) => left == right,
            (Number::Float(left), Number::Float(right)) => (left.is_nan() && right.is_nan()) || left == right,
            _ => false,
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn rank(value: &Number) -> u8 {
            match value {
                Number::PosInt(_) => 0,
                Number::NegInt(_) => 1,
                Number::Float(_) => 2,
            }
        }
        match (self, other) {
            (Number::PosInt(left), Number::PosInt(right)) => Some(left.cmp(right)),
            (Number::NegInt(left), Number::NegInt(right)) => Some(left.cmp(right)),
            (Number::Float(left), Number::Float(right)) if left.is_nan() && right.is_nan() => Some(Ordering::Equal),
            (Number::Float(left), Number::Float(right)) => left.partial_cmp(right),
            _ => Some(rank(self).cmp(&rank(other))),
        }
    }
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
            Number::Float(value) if value.is_nan() => formatter.write_str(".nan"),
            Number::Float(value) if value.is_infinite() && value.is_sign_positive() => formatter.write_str(".inf"),
            Number::Float(value) if value.is_infinite() => formatter.write_str("-.inf"),
            // `Debug` keeps a decimal point or an exponent, so the emitted
            // scalar is always resolved as a float instead of an integer.
            Number::Float(value) => write!(formatter, "{value:?}"),
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
/// Keys and values are arbitrary [`Value`]s, as required by YAML. Equality is
/// unordered, because a YAML mapping is a set of key/value pairs regardless of
/// the order they were written in.
#[derive(Clone, Default)]
pub struct Mapping {
    entries: Vec<(Value, Value)>,
    /// Hash bucket to entry indices. This keeps construction and lookup
    /// logarithmic instead of quadratic for large mappings.
    index: BTreeMap<u64, Vec<usize>>,
}

impl fmt::Debug for Mapping {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Mapping")
            .field("entries", &self.entries)
            .finish()
    }
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.entries.len() == other.entries.len()
            && self.entries.iter().all(|(key, value)| other.get(key) == Some(value))
    }
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
            index: BTreeMap::new(),
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
        let hash = value_hash(&key);
        if let Some(indices) = self.index.get(&hash) {
            for &index in indices {
                if self.entries[index].0 == key {
                    return Some(core::mem::replace(&mut self.entries[index].1, value));
                }
            }
        }
        let index = self.entries.len();
        self.entries.push((key, value));
        self.index.entry(hash).or_default().push(index);
        None
    }

    /// Returns the value for `key`.
    pub fn get(&self, key: &Value) -> Option<&Value> {
        let hash = value_hash(key);
        self.index.get(&hash)?.iter().find_map(|&index| {
            let (existing, value) = &self.entries[index];
            (existing == key).then_some(value)
        })
    }

    /// Returns the value for a string key.
    pub fn get_str(&self, key: &str) -> Option<&Value> {
        let hash = string_hash(key);
        self.index.get(&hash)?.iter().find_map(|&index| {
            let (existing, value) = &self.entries[index];
            (existing.as_str() == Some(key)).then_some(value)
        })
    }

    /// Returns the mutable value for a string key.
    pub fn get_str_mut(&mut self, key: &str) -> Option<&mut Value> {
        let hash = string_hash(key);
        let index = self
            .index
            .get(&hash)?
            .iter()
            .copied()
            .find(|&index| self.entries[index].0.as_str() == Some(key))?;
        Some(&mut self.entries[index].1)
    }

    /// Removes and returns the value for `key`.
    pub fn remove(&mut self, key: &Value) -> Option<Value> {
        let hash = value_hash(key);
        let index = self
            .index
            .get(&hash)?
            .iter()
            .copied()
            .find(|&index| self.entries[index].0 == *key)?;
        let (_, value) = self.entries.remove(index);
        self.rebuild_index();
        Some(value)
    }

    /// Iterates over the entries in insertion order.
    pub fn iter(&self) -> core::slice::Iter<'_, (Value, Value)> {
        self.entries.iter()
    }

    fn rebuild_index(&mut self) {
        self.index.clear();
        for (index, (key, _)) in self.entries.iter().enumerate() {
            self.index.entry(value_hash(key)).or_default().push(index);
        }
    }
}

/// A tiny FNV-style hash used only for the mapping index.
fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn hash_mix(hash: u64, value: u64) -> u64 {
    hash_bytes(hash, &value.to_le_bytes())
}

fn value_hash(value: &Value) -> u64 {
    let mut hash = 0xcbf29ce484222325;
    hash_value(value, &mut hash);
    hash
}

fn string_hash(value: &str) -> u64 {
    hash_bytes(hash_mix(0xcbf29ce484222325, 3), value.as_bytes())
}

fn hash_value(value: &Value, hash: &mut u64) {
    match value {
        Value::Null => *hash = hash_mix(*hash, 0),
        Value::Bool(value) => {
            *hash = hash_mix(*hash, 1);
            *hash = hash_mix(*hash, u64::from(u8::from(*value)));
        }
        Value::Number(value) => {
            *hash = hash_mix(*hash, 2);
            match value {
                Number::PosInt(value) => {
                    *hash = hash_mix(*hash, 0);
                    *hash = hash_mix(*hash, *value);
                }
                Number::NegInt(value) => {
                    *hash = hash_mix(*hash, 1);
                    *hash = hash_bytes(*hash, &value.to_le_bytes());
                }
                Number::Float(value) => {
                    *hash = hash_mix(*hash, 2);
                    let bits = if value.is_nan() {
                        f64::NAN.to_bits()
                    } else if *value == 0.0 {
                        0.0f64.to_bits()
                    } else {
                        value.to_bits()
                    };
                    *hash = hash_mix(*hash, bits);
                }
            }
        }
        Value::String(value) => {
            *hash = hash_mix(*hash, 3);
            *hash = hash_bytes(*hash, value.as_bytes());
        }
        Value::Sequence(values) => {
            *hash = hash_mix(*hash, 4);
            *hash = hash_mix(*hash, values.len() as u64);
            for value in values {
                hash_value(value, hash);
            }
        }
        Value::Mapping(mapping) => {
            *hash = hash_mix(*hash, 5);
            *hash = hash_mix(*hash, mapping.len() as u64);
            // Mapping equality is unordered, so fold the entry hashes with an
            // order-independent operation.
            let mut combined = 0u64;
            for (key, value) in mapping.iter() {
                let mut entry = 0xcbf29ce484222325;
                hash_value(key, &mut entry);
                hash_value(value, &mut entry);
                combined = combined.wrapping_add(entry);
            }
            *hash = hash_mix(*hash, combined);
        }
        Value::Tagged(tagged) => {
            *hash = hash_mix(*hash, 6);
            *hash = hash_bytes(*hash, tagged.tag.as_bytes());
            hash_value(&tagged.value, hash);
        }
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
        let mut mapping = Self::new();
        for (key, value) in iter {
            mapping.insert(key, value);
        }
        mapping
    }
}

/// A value annotated with a YAML tag.
#[derive(Clone, Debug, PartialEq)]
pub struct TaggedValue {
    /// The tag as written, including a leading `!` for local tags (for
    /// example `!point`) and the full `tag:yaml.org,2002:...` URI for the
    /// standard tags.
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
