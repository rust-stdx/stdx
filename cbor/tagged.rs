//! Tagged values.

use core::ops::{Deref, DerefMut};

use crate::{
    data::Tag,
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::Result,
};

/// A value wrapped in a compile-time known CBOR tag.
///
/// Encoding writes the tag followed by the value; decoding requires the tag
/// to be present and equal to `TAG`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tagged<T, const TAG: u64>(pub T);

impl<T, const TAG: u64> Tagged<T, TAG> {
    /// Create a tagged value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Tagged(value)
    }

    /// Return the inner value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, const TAG: u64> Deref for Tagged<T, TAG> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const TAG: u64> DerefMut for Tagged<T, TAG> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const TAG: u64> From<T> for Tagged<T, TAG> {
    #[inline]
    fn from(value: T) -> Self {
        Tagged(value)
    }
}

impl<T: Encode, const TAG: u64> Encode for Tagged<T, TAG> {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.tag(TAG)?;
        self.0.encode(e)
    }
}

impl<'b, T: Decode<'b>, const TAG: u64> Decode<'b> for Tagged<T, TAG> {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        d.tag_expected(TAG)?;
        Ok(Tagged(T::decode(d)?))
    }
}

/// A value wrapped in a runtime tag number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaggedValue<T> {
    /// The tag number.
    pub tag: Tag,
    /// The tagged value.
    pub value: T,
}

impl<T> TaggedValue<T> {
    /// Create a tagged value.
    #[inline]
    pub const fn new(tag: Tag, value: T) -> Self {
        TaggedValue {
            tag,
            value,
        }
    }

    /// Return the inner value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: Encode> Encode for TaggedValue<T> {
    #[inline]
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.tag(self.tag)?;
        self.value.encode(e)
    }
}

impl<'b, T: Decode<'b>> Decode<'b> for TaggedValue<T> {
    #[inline]
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let tag = d.tag()?;
        Ok(TaggedValue {
            tag,
            value: T::decode(d)?,
        })
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for TaggedValue<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        struct Repr<'a, T> {
            tag: u64,
            value: &'a T,
        }
        impl<T: serde::Serialize> serde::Serialize for Repr<'_, T> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&self.tag, self.value)?;
                map.end()
            }
        }
        serializer.serialize_newtype_struct(
            crate::serde::TAG_MARKER,
            &Repr {
                tag: self.tag.get(),
                value: &self.value,
            },
        )
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for TaggedValue<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V<T>(core::marker::PhantomData<T>);
        impl<'de, T: serde::Deserialize<'de>> Visitor<'de> for V<T> {
            type Value = TaggedValue<T>;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a tagged CBOR value")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let tag = map
                    .next_key::<u64>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let value = map.next_value::<T>()?;
                Ok(TaggedValue {
                    tag: Tag::new(tag),
                    value,
                })
            }
        }
        deserializer.deserialize_newtype_struct(crate::serde::TAG_MARKER, V(core::marker::PhantomData))
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize, const TAG: u64> serde::Serialize for Tagged<T, TAG> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        TaggedValue {
            tag: Tag::new(TAG),
            value: &self.0,
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>, const TAG: u64> serde::Deserialize<'de> for Tagged<T, TAG> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tagged = TaggedValue::<T>::deserialize(deserializer)?;
        if tagged.tag.get() != TAG {
            return Err(serde::de::Error::custom("unexpected CBOR tag"));
        }
        Ok(Tagged(tagged.value))
    }
}
