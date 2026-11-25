//! Types that can be represented as YAML values.

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};

use crate::{Mapping, Number, Value};

/// A type that can be converted into a [`Value`].
pub trait Encode {
    /// Converts this value into a YAML value.
    fn encode(&self) -> Value;
}

impl<T: Encode + ?Sized> Encode for &T {
    fn encode(&self) -> Value {
        (**self).encode()
    }
}

impl Encode for Value {
    fn encode(&self) -> Value {
        self.clone()
    }
}

impl Encode for () {
    fn encode(&self) -> Value {
        Value::Null
    }
}

impl Encode for bool {
    fn encode(&self) -> Value {
        Value::Bool(*self)
    }
}

macro_rules! impl_encode_number {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Encode for $ty {
                fn encode(&self) -> Value {
                    Value::Number(Number::from(*self))
                }
            }
        )*
    };
}

impl_encode_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

impl Encode for str {
    fn encode(&self) -> Value {
        Value::String(String::from(self))
    }
}

impl Encode for String {
    fn encode(&self) -> Value {
        Value::String(self.clone())
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self) -> Value {
        self.as_ref().map_or(Value::Null, Encode::encode)
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self) -> Value {
        Value::Sequence(self.iter().map(Encode::encode).collect())
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self) -> Value {
        Value::Sequence(self.iter().map(Encode::encode).collect())
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    fn encode(&self) -> Value {
        Value::Sequence(self.iter().map(Encode::encode).collect())
    }
}

impl<V: Encode> Encode for BTreeMap<String, V> {
    fn encode(&self) -> Value {
        let mut mapping = Mapping::with_capacity(self.len());
        for (key, value) in self {
            mapping.insert(Value::String(key.clone()), value.encode());
        }
        Value::Mapping(mapping)
    }
}

#[cfg(feature = "std")]
impl<V: Encode, S: core::hash::BuildHasher> Encode for std::collections::HashMap<String, V, S> {
    fn encode(&self) -> Value {
        let mut mapping = Mapping::with_capacity(self.len());
        for (key, value) in self {
            mapping.insert(Value::String(key.clone()), value.encode());
        }
        Value::Mapping(mapping)
    }
}

impl<T: Encode> Encode for BTreeSet<T> {
    fn encode(&self) -> Value {
        Value::Sequence(self.iter().map(Encode::encode).collect())
    }
}

/// Converts an encodable value into a [`Value`].
pub fn to_value<T: Encode + ?Sized>(value: &T) -> Value {
    value.encode()
}
