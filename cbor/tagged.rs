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
