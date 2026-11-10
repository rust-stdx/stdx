//! The dynamic JSON [`Value`] type and ordered object [`Map`].

use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};
use core::{
    fmt,
    ops::{Deref, Index, IndexMut},
};

use crate::error::{ErrorKind, Result};
pub use crate::number::Number;

/// An ordered JSON object.
///
/// Keys and values preserve the order in which they were inserted. Lookups are
/// linear, which is fast for the small objects common in JSON. For large,
/// lookup-heavy objects decode into a `HashMap` or `BTreeMap` instead.
#[derive(Clone, Debug, Default)]
pub struct Map<'a>(Vec<(Cow<'a, str>, Value<'a>)>);

impl<'a> Map<'a> {
    /// Creates an empty map.
    #[inline]
    pub const fn new() -> Self {
        Map(Vec::new())
    }

    /// Creates an empty map with capacity for `capacity` entries.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map(Vec::with_capacity(capacity))
    }

    /// Returns the number of entries.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the entries.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, (Cow<'a, str>, Value<'a>)> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the entries.
    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, (Cow<'a, str>, Value<'a>)> {
        self.0.iter_mut()
    }

    /// Returns an iterator over the keys.
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|(key, _)| key.as_ref())
    }

    /// Returns an iterator over the values.
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Value<'a>> {
        self.0.iter().map(|(_, value)| value)
    }

    /// Returns the value for `key`.
    pub fn get(&self, key: &str) -> Option<&Value<'a>> {
        self.0
            .iter()
            .rev()
            .find(|(candidate, _)| candidate.as_ref() == key)
            .map(|(_, value)| value)
    }

    /// Returns a mutable value for `key`.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value<'a>> {
        self.0
            .iter_mut()
            .rev()
            .find(|(candidate, _)| candidate.as_ref() == key)
            .map(|(_, value)| value)
    }

    /// Inserts an entry, replacing the value of an existing key while keeping
    /// its original position. Returns the replaced value.
    pub fn insert(&mut self, key: impl Into<Cow<'a, str>>, value: Value<'a>) -> Option<Value<'a>> {
        let key = key.into();
        if let Some((_, existing)) = self
            .0
            .iter_mut()
            .find(|(candidate, _)| candidate.as_ref() == key.as_ref())
        {
            return Some(core::mem::replace(existing, value));
        }
        self.0.push((key, value));
        None
    }

    /// Appends an entry without checking for a duplicate key.
    #[inline]
    pub fn push(&mut self, key: Cow<'a, str>, value: Value<'a>) {
        self.0.push((key, value));
    }

    /// Removes an entry.
    pub fn remove(&mut self, key: &str) -> Option<Value<'a>> {
        let index = self.0.iter().position(|(candidate, _)| candidate.as_ref() == key)?;
        Some(self.0.remove(index).1)
    }

    /// Returns `true` if the map contains `key`.
    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Removes all entries.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Converts the map into its entries.
    #[inline]
    pub fn into_pairs(self) -> Vec<(Cow<'a, str>, Value<'a>)> {
        self.0
    }

    /// Returns the entries as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[(Cow<'a, str>, Value<'a>)] {
        &self.0
    }

    /// Applies the duplicate-key policy to a map built by appending entries.
    ///
    /// Small objects use a linear scan. Larger objects use a temporary
    /// open-addressing index so parsing stays linear instead of quadratic.
    pub(crate) fn finish_duplicates(
        &mut self,
        policy: crate::decode::DuplicateKeys,
    ) -> core::result::Result<(), crate::ErrorKind> {
        use crate::decode::DuplicateKeys;

        let len = self.0.len();
        if len <= 1 {
            return Ok(());
        }

        if len <= 16 {
            let mut keep = [true; 16];
            let mut any_duplicate = false;
            for i in 0..len {
                for j in 0..i {
                    if keep[j] && self.0[i].0.as_ref() == self.0[j].0.as_ref() {
                        match policy {
                            DuplicateKeys::LastWins => {
                                let value = core::mem::replace(&mut self.0[i].1, Value::Null);
                                self.0[j].1 = value;
                            }
                            DuplicateKeys::FirstWins => {}
                            DuplicateKeys::Reject => return Err(crate::ErrorKind::DuplicateKey),
                        }
                        keep[i] = false;
                        any_duplicate = true;
                        break;
                    }
                }
            }
            if any_duplicate && matches!(policy, DuplicateKeys::LastWins | DuplicateKeys::FirstWins) {
                let mut unique = Vec::with_capacity(len);
                for (i, pair) in core::mem::take(&mut self.0).into_iter().enumerate() {
                    if keep[i] {
                        unique.push(pair);
                    }
                }
                self.0 = unique;
            }
            return Ok(());
        }

        let capacity = len.saturating_mul(2).checked_next_power_of_two().unwrap_or(usize::MAX);
        let mut table = alloc::vec![u32::MAX; capacity];
        let mut keep = alloc::vec![true; len];
        let mut any_duplicate = false;
        let mask = capacity - 1;

        for i in 0..len {
            let mut slot = (hash_key(self.0[i].0.as_ref()) as usize) & mask;
            loop {
                let entry = table[slot];
                if entry == u32::MAX {
                    table[slot] = i as u32 + 1;
                    break;
                }
                let j = (entry - 1) as usize;
                if self.0[i].0.as_ref() == self.0[j].0.as_ref() {
                    match policy {
                        DuplicateKeys::LastWins => {
                            let value = core::mem::replace(&mut self.0[i].1, Value::Null);
                            self.0[j].1 = value;
                        }
                        DuplicateKeys::FirstWins => {}
                        DuplicateKeys::Reject => return Err(crate::ErrorKind::DuplicateKey),
                    }
                    keep[i] = false;
                    any_duplicate = true;
                    break;
                }
                slot = (slot + 1) & mask;
            }
        }

        if any_duplicate && matches!(policy, DuplicateKeys::LastWins | DuplicateKeys::FirstWins) {
            let mut unique = Vec::with_capacity(len);
            for (i, pair) in core::mem::take(&mut self.0).into_iter().enumerate() {
                if keep[i] {
                    unique.push(pair);
                }
            }
            self.0 = unique;
        }
        Ok(())
    }
}

fn hash_key(key: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in key.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

impl PartialEq for Map<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().all(|(key, value)| other.get(key.as_ref()) == Some(value))
    }
}

impl<'a> Deref for Map<'a> {
    type Target = [(Cow<'a, str>, Value<'a>)];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromIterator<(Cow<'a, str>, Value<'a>)> for Map<'a> {
    fn from_iter<T: IntoIterator<Item = (Cow<'a, str>, Value<'a>)>>(iter: T) -> Self {
        Map(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for Map<'a> {
    type Item = (Cow<'a, str>, Value<'a>);
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'b Map<'a> {
    type Item = &'b (Cow<'a, str>, Value<'a>);
    type IntoIter = core::slice::Iter<'b, (Cow<'a, str>, Value<'a>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Any JSON value.
///
/// Strings borrowed from the input are stored as [`Cow::Borrowed`]; strings
/// containing escapes are owned. See [`Value::into_owned`] to detach a value
/// from its input buffer.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value<'a> {
    /// `null`.
    #[default]
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// A number.
    Number(Number),
    /// A string.
    String(Cow<'a, str>),
    /// An array.
    Array(Vec<Value<'a>>),
    /// An object.
    Object(Map<'a>),
}

impl<'a> Value<'a> {
    /// Creates a `null` value.
    #[inline]
    pub const fn null() -> Self {
        Value::Null
    }

    /// Returns `true` if this is `null`.
    #[inline]
    pub const fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns the boolean value, if any.
    #[inline]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the number, if any.
    #[inline]
    pub const fn as_number(&self) -> Option<Number> {
        match self {
            Value::Number(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the value as a `u64`.
    #[inline]
    pub const fn as_u64(&self) -> Option<u64> {
        match self.as_number() {
            Some(number) => number.as_u64(),
            None => None,
        }
    }

    /// Returns the value as an `i64`.
    #[inline]
    pub const fn as_i64(&self) -> Option<i64> {
        match self.as_number() {
            Some(number) => number.as_i64(),
            None => None,
        }
    }

    /// Returns the value as an `f64`.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        self.as_number().map(Number::as_f64)
    }

    /// Returns the string, if any.
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value.as_ref()),
            _ => None,
        }
    }

    /// Returns the array, if any.
    #[inline]
    pub fn as_array(&self) -> Option<&Vec<Value<'a>>> {
        match self {
            Value::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the array mutably, if any.
    #[inline]
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value<'a>>> {
        match self {
            Value::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the object, if any.
    #[inline]
    pub fn as_object(&self) -> Option<&Map<'a>> {
        match self {
            Value::Object(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the object mutably, if any.
    #[inline]
    pub fn as_object_mut(&mut self) -> Option<&mut Map<'a>> {
        match self {
            Value::Object(value) => Some(value),
            _ => None,
        }
    }

    /// Looks up an object key.
    #[inline]
    pub fn get(&self, key: &str) -> Option<&Value<'a>> {
        self.as_object().and_then(|map| map.get(key))
    }

    /// Looks up an object key mutably.
    #[inline]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value<'a>> {
        self.as_object_mut().and_then(|map| map.get_mut(key))
    }

    /// Looks up an array index.
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&Value<'a>> {
        self.as_array().and_then(|array| array.get(index))
    }

    /// Looks up an array index mutably.
    #[inline]
    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut Value<'a>> {
        self.as_array_mut().and_then(|array| array.get_mut(index))
    }

    /// Recursively replaces borrowed strings with owned strings.
    pub fn into_owned(self) -> Value<'static> {
        match self {
            Value::Null => Value::Null,
            Value::Bool(value) => Value::Bool(value),
            Value::Number(value) => Value::Number(value),
            Value::String(value) => Value::String(Cow::Owned(value.into_owned())),
            Value::Array(values) => Value::Array(values.into_iter().map(Value::into_owned).collect()),
            Value::Object(map) => Value::Object(Map(map
                .into_pairs()
                .into_iter()
                .map(|(key, value)| (Cow::Owned(key.into_owned()), value.into_owned()))
                .collect())),
        }
    }

    /// Returns `true` if this is a string.
    #[inline]
    pub const fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns `true` if this is a number.
    #[inline]
    pub const fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Returns `true` if this is a boolean.
    #[inline]
    pub const fn is_boolean(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns `true` if this is an array.
    #[inline]
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// Returns `true` if this is an object.
    #[inline]
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    /// Resolves an RFC 6901 JSON Pointer against this value.
    pub fn pointer(&self, pointer: &str) -> Option<&Value<'a>> {
        if pointer.is_empty() {
            return Some(self);
        }
        let rest = pointer.strip_prefix('/')?;
        let mut current = self;
        for token in rest.split('/') {
            match current {
                Value::Array(values) => {
                    if token.is_empty() || token.starts_with('+') || token.starts_with('-') {
                        return None;
                    }
                    if token.len() > 1 && token.starts_with('0') {
                        return None;
                    }
                    let index = token.parse::<usize>().ok()?;
                    current = values.get(index)?;
                }
                Value::Object(map) => {
                    let mut key = alloc::string::String::new();
                    let mut chars = token.chars();
                    while let Some(ch) = chars.next() {
                        if ch == '~' {
                            match chars.next() {
                                Some('0') => key.push('~'),
                                Some('1') => key.push('/'),
                                _ => return None,
                            }
                        } else {
                            key.push(ch);
                        }
                    }
                    current = map.get(&key)?;
                }
                _ => return None,
            }
        }
        Some(current)
    }

    /// Returns a short name for the value's type.
    pub const fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }
}

/// An owned JSON value that can be decoded from any input lifetime.
///
/// This is the type to use for `serde_json::Value`-style fields in generated
/// or schema-driven code: unlike [`Value<'static>`], it implements
/// [`crate::Decode`] for every input lifetime and therefore works with the
/// normal derive.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct OwnedValue(pub Value<'static>);

impl core::ops::Deref for OwnedValue {
    type Target = Value<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for OwnedValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Value<'a>> for OwnedValue {
    fn from(value: Value<'a>) -> Self {
        OwnedValue(value.into_owned())
    }
}

impl From<&OwnedValue> for OwnedValue {
    fn from(value: &OwnedValue) -> Self {
        value.clone()
    }
}

impl<'a> From<&Value<'a>> for OwnedValue {
    fn from(value: &Value<'a>) -> Self {
        OwnedValue(value.clone().into_owned())
    }
}

impl From<OwnedValue> for Value<'static> {
    fn from(value: OwnedValue) -> Self {
        value.0
    }
}

impl fmt::Display for OwnedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl crate::encode::Encode for OwnedValue {
    fn encode<W: crate::encode::Write>(
        &self,
        encoder: &mut crate::encode::Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        self.0.encode(encoder)
    }
}

impl<'a> crate::decode::Decode<'a> for OwnedValue {
    fn decode(decoder: &mut crate::decode::Decoder<'a>) -> Result<Self> {
        let value: Value<'a> = decoder.decode_inline()?;
        Ok(OwnedValue(value.into_owned()))
    }
}

static NULL: Value<'static> = Value::Null;

impl<'a> Index<&str> for Value<'a> {
    type Output = Value<'a>;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).unwrap_or(&NULL)
    }
}

impl<'a> IndexMut<&str> for Value<'a> {
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        if self.as_object().is_none() {
            *self = Value::Object(Map::new());
        }
        let map = self.as_object_mut().unwrap();
        if map.get(key).is_none() {
            map.push(Cow::Owned(key.to_owned()), Value::Null);
        }
        map.get_mut(key).unwrap()
    }
}

impl<'a> Index<usize> for Value<'a> {
    type Output = Value<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index).unwrap_or(&NULL)
    }
}

impl<'a> IndexMut<usize> for Value<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.as_array().is_none() {
            *self = Value::Array(Vec::new());
        }
        let array = self.as_array_mut().unwrap();
        while array.len() <= index {
            array.push(Value::Null);
        }
        &mut array[index]
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::encode::display(self, f)
    }
}

impl<'a> From<()> for Value<'a> {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl<'a> From<Number> for Value<'a> {
    fn from(value: Number) -> Self {
        Value::Number(value)
    }
}

macro_rules! impl_from_number {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<'a> From<$ty> for Value<'a> {
                fn from(value: $ty) -> Self {
                    Value::Number(Number::from(value))
                }
            }
        )*
    };
}

impl_from_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Value::String(Cow::Borrowed(value))
    }
}

impl<'a> From<String> for Value<'a> {
    fn from(value: String) -> Self {
        Value::String(Cow::Owned(value))
    }
}

impl<'a> From<Cow<'a, str>> for Value<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Value::String(value)
    }
}

impl<'a, T: Into<Value<'a>>> From<Vec<T>> for Value<'a> {
    fn from(value: Vec<T>) -> Self {
        Value::Array(value.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Into<Value<'a>>, const N: usize> From<[T; N]> for Value<'a> {
    fn from(value: [T; N]) -> Self {
        Value::Array(value.into_iter().map(Into::into).collect())
    }
}

impl<'a, 'b, T: Clone + Into<Value<'a>>> From<&'b [T]> for Value<'a> {
    fn from(value: &'b [T]) -> Self {
        Value::Array(value.iter().cloned().map(Into::into).collect())
    }
}

impl<'a> From<Map<'a>> for Value<'a> {
    fn from(value: Map<'a>) -> Self {
        Value::Object(value)
    }
}

impl<'a, T: Into<Value<'a>>> From<Option<T>> for Value<'a> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl<'a> crate::decode::Decode<'a> for Value<'a> {
    fn decode(decoder: &mut crate::decode::Decoder<'a>) -> Result<Self> {
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            None => Err(decoder.error(ErrorKind::UnexpectedEof)),
            Some(b'n') => {
                decoder.decode_null()?;
                Ok(Value::Null)
            }
            Some(b't') => {
                decoder.decode_true()?;
                Ok(Value::Bool(true))
            }
            Some(b'f') => {
                decoder.decode_false()?;
                Ok(Value::Bool(false))
            }
            Some(b'"') => Ok(Value::String(decoder.decode_cow_str()?)),
            Some(b'[') => {
                decoder.begin_array()?;
                let mut values = Vec::with_capacity(decoder.array_hint()?);
                let mut first = true;
                while decoder.array_next(first)? {
                    first = false;
                    values.push(decoder.decode_inline::<Value<'a>>()?);
                }
                Ok(Value::Array(values))
            }
            Some(b'{') => {
                decoder.begin_object()?;
                let mut map = Map::with_capacity(decoder.object_hint()?);
                let mut first = true;
                while decoder.object_next(first)? {
                    first = false;
                    let key = decoder.key()?;
                    let value = decoder.decode_inline::<Value<'a>>()?;
                    map.push(key.into_cow(), value);
                }
                map.finish_duplicates(decoder.options().duplicate_keys)
                    .map_err(|kind| decoder.error(kind))?;
                Ok(Value::Object(map))
            }
            Some(b'-' | b'0'..=b'9') => Ok(Value::Number(decoder.decode_number()?)),
            Some(_) => Err(decoder.error(ErrorKind::ExpectedValue)),
        }
    }
}

impl crate::encode::Encode for Value<'_> {
    #[inline]
    fn encode<W: crate::encode::Write>(
        &self,
        encoder: &mut crate::encode::Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        match self {
            Value::Null => encoder.null(),
            Value::Bool(value) => encoder.boolean(*value),
            Value::Number(value) => encoder.number(*value),
            Value::String(value) => encoder.string(value),
            Value::Array(values) => {
                encoder.begin_array()?;
                for (index, value) in values.iter().enumerate() {
                    encoder.comma(index == 0)?;
                    value.encode(encoder)?;
                }
                encoder.end_array()
            }
            Value::Object(map) => {
                encoder.begin_object()?;
                for (index, (key, value)) in map.iter().enumerate() {
                    encoder.comma(index == 0)?;
                    encoder.key(key)?;
                    value.encode(encoder)?;
                }
                encoder.end_object()
            }
        }
    }
}

impl<'a> Value<'a> {
    /// Builds a value from a raw JSON slice.
    pub fn from_slice(input: &'a [u8]) -> Result<Self> {
        crate::from_slice(input)
    }
}

/// Constructs a [`Value`] from a JSON-like literal.
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// let value = json::json!({
///     "name": "alice",
///     "age": 42,
///     "tags": [true, null],
/// });
/// assert_eq!(value["name"].as_str(), Some("alice"));
/// # }
/// ```
#[macro_export]
macro_rules! json {
    (null) => { $crate::Value::Null };
    (true) => { $crate::Value::Bool(true) };
    (false) => { $crate::Value::Bool(false) };
    ([ $($tt:tt)* ]) => { $crate::Value::Array($crate::json_internal_array!($($tt)*)) };
    ({ $($tt:tt)* }) => { $crate::Value::Object($crate::json_internal_object!($($tt)*)) };
    ($other:expr) => { $crate::Value::from($other) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! json_internal_array {
    () => { $crate::__private::Vec::new() };
    ($($value:tt),+ $(,)?) => {{
        let mut values = $crate::__private::Vec::new();
        $( values.push($crate::json!($value)); )+
        values
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! json_internal_object {
    () => { $crate::Map::new() };
    ($($key:literal : $value:tt),+ $(,)?) => {{
        let mut map = $crate::Map::new();
        $( map.push($crate::__private::Cow::Borrowed($key), $crate::json!($value)); )+
        map
    }};
}
