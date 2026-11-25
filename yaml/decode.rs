//! Types that can be decoded from a YAML [`Value`].

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};

use crate::{Error, Number, Value};

/// A type that can be decoded from a [`Value`].
pub trait Decode: Sized {
    /// Decodes a value.
    fn decode(value: &Value) -> crate::Result<Self>;
}

impl Decode for Value {
    fn decode(value: &Value) -> crate::Result<Self> {
        Ok(value.clone())
    }
}

impl Decode for () {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Null => Ok(()),
            other => Err(Error::invalid_type("null", other)),
        }
    }
}

impl Decode for bool {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Bool(value) => Ok(*value),
            other => Err(Error::invalid_type("boolean", other)),
        }
    }
}

macro_rules! impl_decode_integer {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Decode for $ty {
                fn decode(value: &Value) -> crate::Result<Self> {
                    match value.untag() {
                        Value::Number(number) => {
                            let number = number.as_i64().ok_or_else(|| Error::invalid_type(stringify!($ty), value))?;
                            <$ty>::try_from(number).map_err(|_| Error::invalid_type(stringify!($ty), value))
                        }
                        other => Err(Error::invalid_type(stringify!($ty), other)),
                    }
                }
            }
        )*
    };
}

impl_decode_integer!(i8, i16, i32, i64, isize);

macro_rules! impl_decode_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Decode for $ty {
                fn decode(value: &Value) -> crate::Result<Self> {
                    match value.untag() {
                        Value::Number(number) => {
                            let number = number.as_u64().ok_or_else(|| Error::invalid_type(stringify!($ty), value))?;
                            <$ty>::try_from(number).map_err(|_| Error::invalid_type(stringify!($ty), value))
                        }
                        other => Err(Error::invalid_type(stringify!($ty), other)),
                    }
                }
            }
        )*
    };
}

impl_decode_unsigned!(u8, u16, u32, u64, usize);

impl Decode for f32 {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Number(number) => number
                .as_f64()
                .map(|value| value as f32)
                .ok_or_else(|| Error::invalid_type("f32", value)),
            other => Err(Error::invalid_type("f32", other)),
        }
    }
}

impl Decode for f64 {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Number(number) => number.as_f64().ok_or_else(|| Error::invalid_type("f64", value)),
            other => Err(Error::invalid_type("f64", other)),
        }
    }
}

impl Decode for String {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::String(value) => Ok(value.clone()),
            other => Err(Error::invalid_type("string", other)),
        }
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Null => Ok(None),
            _ => T::decode(value).map(Some),
        }
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Sequence(values) => values.iter().map(T::decode).collect(),
            other => Err(Error::invalid_type("sequence", other)),
        }
    }
}

impl<T: Decode> Decode for BTreeSet<T>
where
    T: Ord,
{
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Sequence(values) => values.iter().map(T::decode).collect(),
            other => Err(Error::invalid_type("sequence", other)),
        }
    }
}

impl<T: Decode> Decode for BTreeMap<String, T> {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Mapping(mapping) => {
                let mut out = BTreeMap::new();
                for (key, value) in mapping.iter() {
                    let Value::String(key) = key.untag() else {
                        return Err(Error::new("mapping key must be a string"));
                    };
                    out.insert(key.clone(), T::decode(value)?);
                }
                Ok(out)
            }
            other => Err(Error::invalid_type("mapping", other)),
        }
    }
}

#[cfg(feature = "std")]
impl<T: Decode, S: core::hash::BuildHasher + Default> Decode for std::collections::HashMap<String, T, S> {
    fn decode(value: &Value) -> crate::Result<Self> {
        match value.untag() {
            Value::Mapping(mapping) => {
                let mut out = std::collections::HashMap::with_hasher(S::default());
                for (key, value) in mapping.iter() {
                    let Value::String(key) = key.untag() else {
                        return Err(Error::new("mapping key must be a string"));
                    };
                    out.insert(key.clone(), T::decode(value)?);
                }
                Ok(out)
            }
            other => Err(Error::invalid_type("mapping", other)),
        }
    }
}

/// Decodes a value from a [`Value`].
pub fn from_value<T: Decode>(value: &Value) -> crate::Result<T> {
    T::decode(value)
}

#[allow(dead_code)]
fn _number_is_used(_: Number) {}
