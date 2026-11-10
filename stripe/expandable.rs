//! Expandable fields.
//!
//! Stripe returns a related object either as an id string or, when the request
//! asked for it with `expand`, as the full object. [`Expandable`] models both
//! without ambiguity and without buffering the whole JSON document: a string is
//! parsed straight to [`Expandable::Id`], and an object is streamed into `T`.

use json::{Decode, Decoder, Encode, Encoder, ErrorKind, Result, Write};

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

impl<'a, T: Decode<'a>> Decode<'a> for Expandable<T> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'"') => Ok(Expandable::Id(decoder.decode_string()?)),
            Some(b'{') => Ok(Expandable::Object(Box::new(decoder.decode::<T>()?))),
            None => Err(decoder.error(ErrorKind::UnexpectedEof)),
            Some(_) => Err(decoder.error(ErrorKind::Message("expected an id or an expanded object"))),
        }
    }
}

impl<T: Encode> Encode for Expandable<T> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        match self {
            Expandable::Id(id) => encoder.string(id),
            Expandable::Object(object) => object.encode(encoder),
        }
    }
}
