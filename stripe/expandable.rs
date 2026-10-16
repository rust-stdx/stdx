//! Expandable fields.
//!
//! Stripe returns a related object either as an id string or, when the request
//! asked for it with `expand`, as the full object. [`Expandable`] models both
//! without ambiguity and without buffering the whole JSON document: a string is
//! parsed straight to [`Expandable::Id`], and an object is streamed into `T`.

use core::{fmt, marker::PhantomData};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor, value::MapAccessDeserializer},
};

/// A field that may be either an id or the expanded object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expandable<T> {
    /// The un-expanded id, for example `"cus_123"`.
    Id(String),
    /// The expanded object.
    Object(Box<T>),
}

impl<T> Expandable<T> {
    /// Returns the id, if this is not expanded.
    pub fn as_id(&self) -> Option<&str> {
        match self {
            Expandable::Id(id) => Some(id),
            Expandable::Object(_) => None,
        }
    }

    /// Returns the expanded object, if present.
    pub fn as_object(&self) -> Option<&T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(object) => Some(object),
        }
    }

    /// Consumes self and returns the id, if this is not expanded.
    pub fn into_id(self) -> Option<String> {
        match self {
            Expandable::Id(id) => Some(id),
            Expandable::Object(_) => None,
        }
    }

    /// Consumes self and returns the object, if expanded.
    pub fn into_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(object) => Some(*object),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Expandable<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExpandableVisitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for ExpandableVisitor<T> {
            type Value = Expandable<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a string id or an expanded object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Expandable::Id(value.to_string()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Expandable::Id(value))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let object = T::deserialize(MapAccessDeserializer::new(map))?;
                Ok(Expandable::Object(Box::new(object)))
            }
        }

        deserializer.deserialize_any(ExpandableVisitor(PhantomData))
    }
}

impl<T: Serialize> Serialize for Expandable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Expandable::Id(id) => serializer.serialize_str(id),
            Expandable::Object(object) => object.serialize(serializer),
        }
    }
}
