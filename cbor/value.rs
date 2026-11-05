//! Dynamic CBOR values.

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt;

use crate::{
    data::{Integer, Simple, Type},
    decode::{Decode, Decoder, collection_capacity},
    encode::{Encode, Encoder, Write},
    error::{Error, Result},
};

/// An arbitrary-precision CBOR bignum (tags 2 and 3).
///
/// `bytes` holds the magnitude in big-endian order with no leading zeroes.
/// The represented value is `bytes` when `negative` is false, and
/// `-1 - bytes` when it is true.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bignum {
    /// Whether the value is negative.
    pub negative: bool,
    /// Magnitude bytes, big-endian, without leading zeroes.
    pub bytes: Vec<u8>,
}

impl Bignum {
    /// Create a bignum, normalizing leading zero bytes.
    pub fn new(negative: bool, mut bytes: Vec<u8>) -> Self {
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
        if first == bytes.len() {
            bytes = Vec::new();
        } else if first > 0 {
            bytes.drain(..first);
        }
        Bignum {
            negative,
            bytes,
        }
    }

    /// Return the magnitude as a `u128`, if it fits.
    pub fn to_u128(&self) -> Option<u128> {
        if self.bytes.len() > 16 {
            return None;
        }
        let mut value = 0u128;
        for &b in &self.bytes {
            value = (value << 8) | u128::from(b);
        }
        Some(value)
    }
}

/// An order-preserving CBOR map.
///
/// Unlike `BTreeMap` or `HashMap`, `Map` preserves the exact wire order and
/// keeps duplicate keys if they were present in the input. This is the map
/// type used by [`Value`].
///
/// Duplicate detection during decoding compares each key with the entries
/// already seen. With the default [`DuplicateKeys::Error`](crate::DuplicateKeys)
/// policy this is quadratic in the number of entries. The decoder's
/// [`Options::max_duplicate_checks`](crate::Options::max_duplicate_checks)
/// budget bounds the total work. For very large maps from untrusted input,
/// use [`DuplicateKeys::Allow`](crate::DuplicateKeys) and validate separately,
/// or decode into [`Value`].
///
/// Comparison uses [`PartialEq`]. This means `-0.0` and `0.0` keys are
/// considered duplicates, while `NaN` is not equal to itself: byte-identical
/// `NaN` keys are not detected, and `First`/`Last` cannot replace a `NaN` key
/// by value. Different `NaN` payloads are distinct at the generic data-model
/// level, but deterministic encoding collapses them; use
/// [`try_to_canonical_vec`](crate::try_to_canonical_vec) when that matters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Map<K = Value, V = Value>(Vec<(K, V)>);

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Map(Vec::new())
    }
}

impl<K, V> Map<K, V> {
    /// Create an empty map.
    #[inline]
    pub const fn new() -> Self {
        Map(Vec::new())
    }

    /// Create an empty map with the given capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map(Vec::with_capacity(capacity))
    }

    /// Return the number of pairs.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return `true` if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterate over the pairs in wire order.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, (K, V)> {
        self.0.iter()
    }

    /// Mutably iterate over the pairs in wire order.
    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, (K, V)> {
        self.0.iter_mut()
    }

    /// Consume the map and return its pairs.
    #[inline]
    pub fn into_pairs(self) -> Vec<(K, V)> {
        self.0
    }

    /// Append a pair without checking for duplicates.
    #[inline]
    pub fn push(&mut self, key: K, value: V) {
        self.0.push((key, value));
    }

    /// Remove all pairs.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Return a reference to the first value whose key equals `key`.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Return a mutable reference to the first value whose key equals `key`.
    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    where
        K: PartialEq,
    {
        self.0.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Insert a pair, replacing the first existing key.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: PartialEq,
    {
        if let Some((_, slot)) = self.0.iter_mut().find(|(k, _)| *k == key) {
            Some(core::mem::replace(slot, value))
        } else {
            self.0.push((key, value));
            None
        }
    }
}

impl<K, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Map(iter.into_iter().collect())
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = alloc::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
    type Item = &'a (K, V);
    type IntoIter = core::slice::Iter<'a, (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Map<Value, Value> {
    /// Sort the map by the RFC 8949 core-deterministic encoding of its keys.
    ///
    /// Nested maps are canonicalized before their keys are compared. This does
    /// not reject duplicate or collapsing keys; the sorted map retains them.
    /// Use [`try_to_canonical_vec`](crate::try_to_canonical_vec) to validate an
    /// encoding.
    pub fn sort_canonical(&mut self) {
        for (key, value) in &mut self.0 {
            key.canonicalize();
            value.canonicalize();
        }
        let mut keyed: Vec<(Vec<u8>, usize)> = self
            .0
            .iter()
            .enumerate()
            .map(|(i, (key, _))| (crate::encode::to_canonical_vec(key), i))
            .collect();
        keyed.sort_by(|a, b| a.0.cmp(&b.0));
        let mut pairs: Vec<Option<(Value, Value)>> = self.0.drain(..).map(Some).collect();
        for (_, index) in keyed {
            if let Some(pair) = pairs.get_mut(index).and_then(Option::take) {
                self.0.push(pair);
            }
        }
    }
}

/// A dynamic CBOR value.
///
/// Decoding into `Value` preserves map wire order and keeps duplicate keys;
/// [`Options::duplicate_keys`](crate::Options::duplicate_keys) does not apply.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Value {
    /// `null`.
    Null,
    /// `undefined`.
    Undefined,
    /// A boolean.
    Bool(bool),
    /// A basic integer.
    Integer(Integer),
    /// A tag 2 or tag 3 bignum.
    Bignum(Bignum),
    /// A floating-point value.
    Float(f64),
    /// A byte string.
    Bytes(Vec<u8>),
    /// A text string.
    Text(String),
    /// An array.
    Array(Vec<Value>),
    /// A map, preserving wire order.
    Map(Map<Value, Value>),
    /// A tagged value.
    Tag(u64, Box<Value>),
    /// A simple value.
    Simple(Simple),
}

impl Value {
    /// Recursively normalize the value for deterministic encoding.
    ///
    /// This normalizes bignums, canonicalizes nested map ordering and leaves
    /// floats to the encoder's shortest-form/NaN handling.
    ///
    /// This does not reject duplicate or collapsing map keys. Use
    /// [`try_to_canonical_vec`](crate::try_to_canonical_vec) to validate the
    /// final encoding.
    pub fn canonicalize(&mut self) {
        match self {
            Value::Bignum(b) => {
                *b = Bignum::new(b.negative, core::mem::take(&mut b.bytes));
                if let Some(magnitude) = b.to_u128() {
                    if !b.negative && magnitude <= u64::MAX as u128 {
                        *self = Value::Integer(Integer::Unsigned(magnitude as u64));
                    } else if b.negative && magnitude <= u64::MAX as u128 {
                        *self = Value::Integer(Integer::Negative(magnitude as u64));
                    }
                }
            }
            Value::Array(values) => {
                for value in values {
                    value.canonicalize();
                }
            }
            Value::Map(map) => map.sort_canonical(),
            Value::Tag(_, value) => value.canonicalize(),
            _ => {}
        }
    }

    /// Return the value as a `bool`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Return the value as a `u64`.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Integer(i) => i.as_u64(),
            _ => None,
        }
    }

    /// Return the value as an `i64`.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => i.as_i64(),
            _ => None,
        }
    }

    /// Return the value as a `f64`.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(*v),
            _ => None,
        }
    }

    /// Return the value as a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(v) => Some(v),
            _ => None,
        }
    }

    /// Return the value as bytes.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(v) => Some(v),
            _ => None,
        }
    }

    /// Return the value as an array.
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Return the value as a map.
    pub fn as_map(&self) -> Option<&Map<Value, Value>> {
        match self {
            Value::Map(v) => Some(v),
            _ => None,
        }
    }

    /// Look up a map key.
    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.as_map().and_then(|map| map.get(key))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        crate::diag::write_value(self, f)
    }
}

impl Encode for Value {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        match self {
            Value::Null => e.null(),
            Value::Undefined => e.undefined(),
            Value::Bool(v) => e.bool(*v),
            Value::Integer(v) => e.int(*v),
            Value::Bignum(b) => {
                let first = b.bytes.iter().position(|&x| x != 0).unwrap_or(b.bytes.len());
                let magnitude = &b.bytes[first..];
                if e.is_canonical() && magnitude.len() <= 8 {
                    let mut value = 0u64;
                    for &byte in magnitude {
                        value = (value << 8) | u64::from(byte);
                    }
                    return if b.negative {
                        e.int(Integer::Negative(value))
                    } else {
                        e.int(Integer::Unsigned(value))
                    };
                }
                let tag = if b.negative {
                    crate::data::tags::BIGNEG
                } else {
                    crate::data::tags::BIGPOS
                };
                e.tag(tag)?;
                e.bytes(magnitude)
            }
            Value::Float(v) => e.f64(*v),
            Value::Bytes(v) => e.bytes(v),
            Value::Text(v) => e.text(v),
            Value::Array(values) => {
                e.array(values.len())?;
                for value in values {
                    value.encode(e)?;
                }
                Ok(())
            }
            Value::Map(map) => {
                if e.is_canonical() {
                    let mut entries: Vec<(Vec<u8>, &Value)> = map
                        .iter()
                        .map(|(key, value)| {
                            let mut key = key.clone();
                            key.canonicalize();
                            (crate::encode::to_canonical_vec(&key), value)
                        })
                        .collect();
                    entries.sort_by(|a, b| a.0.cmp(&b.0));
                    e.map(entries.len())?;
                    for (key, value) in entries {
                        e.raw(&key)?;
                        value.encode(e)?;
                    }
                    Ok(())
                } else {
                    e.map(map.len())?;
                    for (key, value) in map.iter() {
                        key.encode(e)?;
                        value.encode(e)?;
                    }
                    Ok(())
                }
            }
            Value::Tag(tag, value) => {
                e.tag(*tag)?;
                value.encode(e)
            }
            Value::Simple(v) => e.simple(*v),
        }
    }
}

impl<'b> Decode<'b> for Value {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.enter_depth()?;
        let result = (|| {
            let ty = d.datatype()?;
            match ty {
                Type::Null => {
                    d.null()?;
                    Ok(Value::Null)
                }
                Type::Undefined => {
                    d.undefined()?;
                    Ok(Value::Undefined)
                }
                Type::Bool => Ok(Value::Bool(d.bool()?)),
                Type::U8 | Type::U16 | Type::U32 | Type::U64 => Ok(Value::Integer(Integer::Unsigned(d.u64()?))),
                Type::I8 | Type::I16 | Type::I32 | Type::I64 => Ok(Value::Integer(d.int()?)),
                Type::F16 | Type::F32 | Type::F64 => Ok(Value::Float(d.f64()?)),
                Type::Simple => Ok(Value::Simple(d.simple()?)),
                Type::Bytes | Type::BytesIndef => Ok(Value::Bytes(d.bytes_owned()?)),
                Type::Text | Type::TextIndef => Ok(Value::Text(d.text_owned()?)),
                Type::Array | Type::ArrayIndef => match d.array()? {
                    Some(len) => {
                        let mut values = Vec::with_capacity(collection_capacity::<Value>(len, d.remaining()));
                        for _ in 0..len {
                            values.push(Value::decode(d)?);
                        }
                        Ok(Value::Array(values))
                    }
                    None => {
                        let mut values = Vec::new();
                        while !d.is_break() {
                            values.push(Value::decode(d)?);
                        }
                        d.read_break()?;
                        Ok(Value::Array(values))
                    }
                },
                Type::Map | Type::MapIndef => {
                    let mut map = Map::new();
                    match d.map()? {
                        Some(len) => {
                            map = Map::with_capacity(collection_capacity::<(Value, Value)>(len, d.remaining()));
                            for _ in 0..len {
                                let key = Value::decode(d)?;
                                let value = Value::decode(d)?;
                                map.push(key, value);
                            }
                        }
                        None => loop {
                            if d.is_break() {
                                d.read_break()?;
                                break;
                            }
                            let key = Value::decode(d)?;
                            if d.is_break() {
                                return Err(
                                    Error::message("indefinite map has an odd number of items").at(d.position())
                                );
                            }
                            let value = Value::decode(d)?;
                            map.push(key, value);
                        },
                    }
                    Ok(Value::Map(map))
                }
                Type::Tag => {
                    let tag = d.tag()?;
                    if tag.get() == crate::data::tags::BIGPOS || tag.get() == crate::data::tags::BIGNEG {
                        let bytes = d.bytes_owned()?;
                        Ok(Value::Bignum(Bignum::new(tag.get() == crate::data::tags::BIGNEG, bytes)))
                    } else {
                        Ok(Value::Tag(tag.get(), Box::new(Value::decode(d)?)))
                    }
                }
                Type::Break => Err(Error::message("unexpected break").at(d.position())),
                Type::Unknown(_) => Err(Error::message("reserved or unknown item").at(d.position())),
            }
        })();
        d.leave_depth();
        result
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}
impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}
macro_rules! impl_from_unsigned {
    ($($ty:ty),*) => {
        $(impl From<$ty> for Value {
            fn from(v: $ty) -> Self {
                Value::Integer(Integer::Unsigned(v as u64))
            }
        })*
    };
}
impl_from_unsigned!(u8, u16, u32, u64, usize);
impl From<u128> for Value {
    fn from(v: u128) -> Self {
        if v <= u64::MAX as u128 {
            Value::Integer(Integer::Unsigned(v as u64))
        } else {
            Value::Bignum(Bignum::new(false, v.to_be_bytes().to_vec()))
        }
    }
}
macro_rules! impl_from_signed {
    ($($ty:ty),*) => {
        $(impl From<$ty> for Value {
            fn from(v: $ty) -> Self {
                Value::Integer(Integer::from_i64(v as i64))
            }
        })*
    };
}
impl_from_signed!(i8, i16, i32, i64, isize);
impl From<i128> for Value {
    fn from(v: i128) -> Self {
        if v >= i64::MIN as i128 && v <= i64::MAX as i128 {
            Value::Integer(Integer::from_i64(v as i64))
        } else if v < 0 {
            Value::Bignum(Bignum::new(true, (-(v + 1)).to_be_bytes().to_vec()))
        } else {
            Value::Bignum(Bignum::new(false, v.to_be_bytes().to_vec()))
        }
    }
}
impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(f64::from(v))
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}
impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(String::from(v))
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}
impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}
impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Array(v)
    }
}
impl From<Map<Value, Value>> for Value {
    fn from(v: Map<Value, Value>) -> Self {
        Value::Map(v)
    }
}
impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

impl<K: Encode, V: Encode> Encode for Map<K, V> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        if e.is_canonical() {
            let mut entries: Vec<(Vec<u8>, &V)> = self
                .0
                .iter()
                .map(|(key, value)| (crate::encode::to_canonical_vec(key), value))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            e.map(entries.len())?;
            for (key, value) in entries {
                e.raw(&key)?;
                value.encode(e)?;
            }
            return Ok(());
        }
        e.map(self.len())?;
        for (key, value) in &self.0 {
            key.encode(e)?;
            value.encode(e)?;
        }
        Ok(())
    }
}

impl<'b, K: Decode<'b> + PartialEq, V: Decode<'b>> Decode<'b> for Map<K, V> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut map = Map::new();
        let policy = d.options().duplicate_keys;
        let checks_duplicates = policy != crate::decode::DuplicateKeys::Allow;
        match d.map()? {
            Some(len) => {
                if checks_duplicates {
                    // Charging the whole map up front bounds the O(n^2) work
                    // before any key or value is decoded.
                    let cost = len.saturating_mul(len.saturating_sub(1)) / 2;
                    d.charge_duplicate_checks(cost)?;
                }
                map = Map::with_capacity(collection_capacity::<(K, V)>(len, d.remaining()));
                for _ in 0..len {
                    let key_start = d.position();
                    let key = K::decode(d)?;
                    let value = V::decode(d)?;
                    insert_pair(&mut map, key, value, policy, key_start)?;
                }
            }
            None => loop {
                if d.is_break() {
                    d.read_break()?;
                    break;
                }
                if checks_duplicates {
                    // `insert_pair` scans the entries already present, so the
                    // cost of this insertion is `map.len()`.
                    d.charge_duplicate_checks(map.len())?;
                }
                let key_start = d.position();
                let key = K::decode(d)?;
                if d.is_break() {
                    return Err(Error::message("indefinite map has an odd number of items").at(d.position()));
                }
                let value = V::decode(d)?;
                insert_pair(&mut map, key, value, policy, key_start)?;
            },
        }
        Ok(map)
    }
}

fn insert_pair<K: PartialEq, V>(
    map: &mut Map<K, V>,
    key: K,
    value: V,
    policy: crate::decode::DuplicateKeys,
    offset: usize,
) -> Result<()> {
    if policy == crate::decode::DuplicateKeys::Allow {
        map.push(key, value);
        return Ok(());
    }
    let position = map.0.iter().position(|(existing, _)| *existing == key);
    match position {
        None => map.push(key, value),
        Some(index) => match policy {
            crate::decode::DuplicateKeys::Error => return Err(Error::duplicate_key().at(offset)),
            crate::decode::DuplicateKeys::First => {}
            crate::decode::DuplicateKeys::Last => map.0[index].1 = value,
            crate::decode::DuplicateKeys::Allow => {}
        },
    }
    Ok(())
}

#[cfg(feature = "serde")]
impl serde::Serialize for Value {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::{SerializeMap, SerializeSeq};
        struct SerBytes<'a>(&'a [u8]);
        impl serde::Serialize for SerBytes<'_> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_bytes(self.0)
            }
        }
        match self {
            Value::Null | Value::Undefined => serializer.serialize_unit(),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Integer(v) => match v {
                Integer::Unsigned(v) => serializer.serialize_u64(*v),
                Integer::Negative(n) => match Integer::Negative(*n).as_i64() {
                    Some(v) => serializer.serialize_i64(v),
                    None => serializer.serialize_i128(Integer::Negative(*n).to_i128()),
                },
            },
            Value::Bignum(b) => {
                let tag = if b.negative { 3 } else { 2 };
                crate::tagged::TaggedValue {
                    tag: crate::Tag::new(tag),
                    value: SerBytes(&b.bytes),
                }
                .serialize(serializer)
            }
            Value::Float(v) => serializer.serialize_f64(*v),
            Value::Bytes(v) => serializer.serialize_bytes(v),
            Value::Text(v) => serializer.serialize_str(v),
            Value::Array(values) => {
                let mut seq = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            Value::Map(map) => {
                let mut out = serializer.serialize_map(Some(map.len()))?;
                for (key, value) in map.iter() {
                    out.serialize_entry(key, value)?;
                }
                out.end()
            }
            Value::Tag(tag, value) => crate::tagged::TaggedValue {
                tag: crate::Tag::new(*tag),
                value: value.as_ref(),
            }
            .serialize(serializer),
            Value::Simple(simple) => simple.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, SeqAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Value;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a CBOR value")
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Value::Null)
            }
            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Value::Null)
            }
            fn visit_some<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
                serde::Deserialize::deserialize(deserializer)
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Value::Bool(v))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Value::Integer(Integer::from_i64(v)))
            }
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> {
                Ok(Value::from(v))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Value::Integer(Integer::Unsigned(v)))
            }
            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> {
                Ok(Value::from(v))
            }
            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> {
                Ok(Value::Float(f64::from(v)))
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(Value::Float(v))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Value::Text(String::from(v)))
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(Value::Text(v))
            }
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(Value::Bytes(Vec::from(v)))
            }
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(Value::Bytes(v))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }
                Ok(Value::Array(values))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = Map::new();
                while let Some((key, value)) = map.next_entry()? {
                    out.push(key, value);
                }
                Ok(Value::Map(out))
            }
        }
        deserializer.deserialize_any(V)
    }
}
