//! serde integration.
//!
//! The serializer is streaming and writes directly through [`Encoder`]. The
//! deserializer borrows definite-length text and byte strings from the input.
//!
//! Structs are encoded as text-key maps (serde's default), enums use serde's
//! externally tagged representation, and `serde_bytes` works as expected.
//! `Vec<u8>` is a CBOR array in serde's data model; use `serde_bytes` for byte
//! strings.
//!
//! `RawValue` uses an internal marker so it is spliced into the output and
//! captured from the input without decoding. `TaggedValue`/`Tagged` and
//! `Simple` use similar markers. serde's data model has no native tag,
//! simple-value or undefined representation, so use those wrappers (or the
//! typed API) when those CBOR features must be preserved. In particular, a
//! serde roundtrip of [`Value`] normalizes `Simple` to an integer, turns
//! `Undefined` into `null`, and does not preserve tags on the decoding side.
//!
//! With the default [`DuplicateKeys::Error`](crate::DuplicateKeys) policy, the
//! deserializer rejects duplicate map keys whose wire encodings are identical.
//! It cannot detect semantically equal keys encoded differently (for example
//! `0x01` and `0x1801`); use [`validate_canonical`](crate::validate_canonical)
//! or the native typed API when that distinction matters. The `First` and
//! `Last` policies are not applied by serde because serde's `MapAccess` cannot
//! suppress an already-decoded entry.

use alloc::{
    borrow::Cow,
    collections::BTreeSet,
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::io::BufRead;

#[cfg(feature = "std")]
use serde::de::DeserializeOwned;
use serde::{
    Deserializer as _,
    de::{self, Visitor},
    ser::{
        self, Impossible, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
        SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    },
};

use crate::{
    data::{Simple, Type},
    decode::{Decoder, DuplicateKeys},
    encode::{Count, Encoder, Write},
    error::Error as CoreError,
    value::Value,
};

pub(crate) const RAW_MARKER: &str = "@@cbor_raw@@";
pub(crate) const TAG_MARKER: &str = "@@cbor_tag@@";
pub(crate) const SIMPLE_MARKER: &str = "@@cbor_simple@@";

/// A serialization error.
#[derive(Debug)]
pub enum Error<E> {
    /// The underlying writer failed.
    Write(E),
    /// A static message.
    Message(&'static str),
    /// A dynamic message, usually from a custom `Serialize` implementation.
    Custom(String),
}

impl<E: core::fmt::Display> core::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Write(error) => write!(f, "write error: {error}"),
            Error::Message(message) => f.write_str(message),
            Error::Custom(message) => f.write_str(message),
        }
    }
}

impl<E: core::fmt::Debug + core::fmt::Display> core::error::Error for Error<E> {}

impl<E> ser::Error for Error<E>
where
    E: core::fmt::Debug + core::fmt::Display,
{
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

/// A deserialization error.
#[derive(Debug)]
pub struct DeError {
    message: Cow<'static, str>,
    source: Option<CoreError>,
}

impl DeError {
    fn message(message: impl Into<Cow<'static, str>>) -> Self {
        DeError {
            message: message.into(),
            source: None,
        }
    }

    fn from_core(error: CoreError) -> Self {
        DeError {
            message: error.to_string().into(),
            source: Some(error),
        }
    }

    /// Return the byte offset of the underlying CBOR error, if any.
    pub fn offset(&self) -> Option<usize> {
        self.source.map(|error| error.offset())
    }
}

impl core::fmt::Display for DeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl core::error::Error for DeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|error| error as &(dyn core::error::Error + 'static))
    }
}

impl de::Error for DeError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        DeError::message(msg.to_string())
    }
}

impl From<CoreError> for DeError {
    fn from(error: CoreError) -> Self {
        DeError::from_core(error)
    }
}

/// Serialize a value into a new `Vec<u8>`.
pub fn to_vec<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, Error<core::convert::Infallible>> {
    let mut buffer = Vec::with_capacity(128);
    {
        let mut serializer = Serializer {
            encoder: Encoder::new(&mut buffer),
        };
        value.serialize(&mut serializer)?;
    }
    Ok(buffer)
}

/// Serialize a value into a caller-provided buffer.
pub fn to_slice<'a, T: Serialize + ?Sized>(value: &T, buffer: &'a mut [u8]) -> Result<&'a mut [u8], Error<CoreError>> {
    let total = buffer.len();
    let used = {
        let mut writer = &mut *buffer;
        let mut serializer = Serializer {
            encoder: Encoder::new(&mut writer),
        };
        value.serialize(&mut serializer)?;
        total - writer.len()
    };
    Ok(&mut buffer[..used])
}

/// Return the exact encoded size of a serializable value.
pub fn serialized_size<T: Serialize + ?Sized>(value: &T) -> Result<usize, Error<core::convert::Infallible>> {
    let mut count = Count::new();
    {
        let mut serializer = Serializer {
            encoder: Encoder::new(&mut count),
        };
        value.serialize(&mut serializer)?;
    }
    Ok(count.get())
}

/// Serialize a value into a `std::io::Write` sink.
#[cfg(feature = "std")]
pub fn to_writer<T: Serialize + ?Sized, W: std::io::Write>(value: &T, writer: W) -> Result<(), Error<std::io::Error>> {
    let mut writer = StdWriter(writer);
    let mut serializer = Serializer {
        encoder: Encoder::new(&mut writer),
    };
    value.serialize(&mut serializer)
}

#[cfg(feature = "std")]
struct StdWriter<W>(W);

#[cfg(feature = "std")]
impl<W: std::io::Write> Write for StdWriter<W> {
    type Error = std::io::Error;

    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(bytes)
    }
}

/// Serialize a value in deterministic form.
///
/// This serializes normally, decodes the result into [`Value`], canonicalizes
/// it and re-encodes it. It is intended for signing and hashing, not for the
/// hot path.
pub fn to_canonical_vec<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, Error<core::convert::Infallible>> {
    let bytes = to_vec(value)?;
    let mut value: Value = crate::decode::from_slice(&bytes).map_err(|_| Error::Message("canonicalization failed"))?;
    value.canonicalize();
    Ok(crate::encode::to_canonical_vec(&value))
}

/// Deserialize a value from a byte slice.
pub fn from_slice<'de, T: serde::Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, DeError> {
    let mut deserializer = Deserializer::new(Decoder::new(bytes));
    let value = T::deserialize(&mut deserializer)?;
    if !deserializer.decoder.is_empty() {
        return Err(DeError::from_core(
            CoreError::trailing_data().at(deserializer.decoder.position()),
        ));
    }
    Ok(value)
}

/// Deserialize a value from a byte slice, returning the unconsumed remainder.
pub fn from_slice_prefix<'de, T: serde::Deserialize<'de>>(bytes: &'de [u8]) -> Result<(T, &'de [u8]), DeError> {
    let mut deserializer = Deserializer::new(Decoder::new(bytes));
    let value = T::deserialize(&mut deserializer)?;
    let position = deserializer.decoder.position();
    Ok((value, &bytes[position..]))
}

fn serialize_tagged<T: Serialize + ?Sized, W: Write>(
    value: &T,
    serializer: &mut Serializer<W>,
) -> Result<(), Error<W::Error>>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    value.serialize(TagSerializer {
        serializer,
    })
}

/// Serializer used only for the tagged-value marker. It captures the tag
/// number from the synthetic one-entry map and forwards the payload directly
/// to the underlying serializer, without buffering the payload.
struct TagSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
}

impl<'a, W: Write> ser::Serializer for TagSerializer<'a, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    type SerializeSeq = Impossible<(), Error<W::Error>>;
    type SerializeTuple = Impossible<(), Error<W::Error>>;
    type SerializeTupleStruct = Impossible<(), Error<W::Error>>;
    type SerializeTupleVariant = Impossible<(), Error<W::Error>>;
    type SerializeMap = TagMapSerializer<'a, W>;
    type SerializeStruct = Impossible<(), Error<W::Error>>;
    type SerializeStructVariant = Impossible<(), Error<W::Error>>;

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if len != Some(1) {
            return Err(Error::Message("tag payload must be a one-entry map"));
        }
        Ok(TagMapSerializer {
            serializer: self.serializer,
            tag: None,
            value_written: false,
        })
    }

    fn serialize_bool(self, _: bool) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_i8(self, _: i8) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_i16(self, _: i16) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_i32(self, _: i32) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_i64(self, _: i64) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_i128(self, _: i128) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_u8(self, _: u8) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_u16(self, _: u16) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_u32(self, _: u32) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_u64(self, _: u64) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_u128(self, _: u128) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_f32(self, _: f32) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_f64(self, _: f64) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_char(self, _: char) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_str(self, _: &str) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Message("expected a tagged CBOR value"))
    }
}

struct TagMapSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    tag: Option<u64>,
    value_written: bool,
}

impl<W: Write> ser::SerializeMap for TagMapSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        if self.value_written || self.tag.is_some() {
            return Err(Error::Message("tag payload must be a one-entry map"));
        }
        key.serialize(TagNumberSerializer {
            out: &mut self.tag,
            marker: core::marker::PhantomData,
        })
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        if self.value_written {
            return Err(Error::Message("tag payload must be a one-entry map"));
        }
        let tag = self
            .tag
            .take()
            .ok_or(Error::Message("tag payload is missing the tag number"))?;
        self.serializer.encoder.tag(tag).map_err(Error::Write)?;
        value.serialize(&mut *self.serializer)?;
        self.value_written = true;
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        if !self.value_written {
            return Err(Error::Message("tag payload is missing the value"));
        }
        Ok(())
    }
}

struct TagNumberSerializer<'a, E> {
    out: &'a mut Option<u64>,
    marker: core::marker::PhantomData<E>,
}

impl<E: core::fmt::Debug + core::fmt::Display> ser::Serializer for TagNumberSerializer<'_, E> {
    type Ok = ();
    type Error = Error<E>;
    type SerializeSeq = Impossible<(), Error<E>>;
    type SerializeTuple = Impossible<(), Error<E>>;
    type SerializeTupleStruct = Impossible<(), Error<E>>;
    type SerializeTupleVariant = Impossible<(), Error<E>>;
    type SerializeMap = Impossible<(), Error<E>>;
    type SerializeStruct = Impossible<(), Error<E>>;
    type SerializeStructVariant = Impossible<(), Error<E>>;

    fn serialize_u8(self, value: u8) -> Result<(), Self::Error> {
        *self.out = Some(u64::from(value));
        Ok(())
    }
    fn serialize_u16(self, value: u16) -> Result<(), Self::Error> {
        *self.out = Some(u64::from(value));
        Ok(())
    }
    fn serialize_u32(self, value: u32) -> Result<(), Self::Error> {
        *self.out = Some(u64::from(value));
        Ok(())
    }
    fn serialize_u64(self, value: u64) -> Result<(), Self::Error> {
        *self.out = Some(value);
        Ok(())
    }
    fn serialize_u128(self, value: u128) -> Result<(), Self::Error> {
        let value = u64::try_from(value).map_err(|_| Error::Message("tag number does not fit u64"))?;
        *self.out = Some(value);
        Ok(())
    }
    fn serialize_bool(self, _: bool) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_i8(self, _: i8) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_i16(self, _: i16) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_i32(self, _: i32) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_i64(self, _: i64) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_i128(self, _: i128) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_f32(self, _: f32) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_f64(self, _: f64) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_char(self, _: char) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_str(self, _: &str) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Message("tag number must be an unsigned integer"))
    }
}

/// The serializer.
pub struct Serializer<W> {
    encoder: Encoder<W>,
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;

    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = SeqSerializer<'a, W>;
    type SerializeTupleStruct = SeqSerializer<'a, W>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, W>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = StructVariantSerializer<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.encoder.bool(v).map_err(Error::Write)
    }
    fn serialize_i8(self, v: i8) -> Result<(), Self::Error> {
        self.encoder.i8(v).map_err(Error::Write)
    }
    fn serialize_i16(self, v: i16) -> Result<(), Self::Error> {
        self.encoder.i16(v).map_err(Error::Write)
    }
    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.encoder.i32(v).map_err(Error::Write)
    }
    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
        self.encoder.i64(v).map_err(Error::Write)
    }
    fn serialize_i128(self, v: i128) -> Result<(), Self::Error> {
        self.encoder.i128(v).map_err(Error::Write)
    }
    fn serialize_u8(self, v: u8) -> Result<(), Self::Error> {
        self.encoder.u8(v).map_err(Error::Write)
    }
    fn serialize_u16(self, v: u16) -> Result<(), Self::Error> {
        self.encoder.u16(v).map_err(Error::Write)
    }
    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
        self.encoder.u32(v).map_err(Error::Write)
    }
    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
        self.encoder.u64(v).map_err(Error::Write)
    }
    fn serialize_u128(self, v: u128) -> Result<(), Self::Error> {
        self.encoder.u128(v).map_err(Error::Write)
    }
    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
        self.encoder.f32(v).map_err(Error::Write)
    }
    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
        self.encoder.f64(v).map_err(Error::Write)
    }
    fn serialize_char(self, v: char) -> Result<(), Self::Error> {
        self.encoder.char(v).map_err(Error::Write)
    }
    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        self.encoder.text(v).map_err(Error::Write)
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        self.encoder.bytes(v).map_err(Error::Write)
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        self.encoder.null().map_err(Error::Write)
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), Self::Error> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        self.encoder.null().map_err(Error::Write)
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        self.encoder.null().map_err(Error::Write)
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.encoder.text(variant).map_err(Error::Write)
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, name: &'static str, value: &T) -> Result<(), Self::Error> {
        if name == RAW_MARKER {
            return value.serialize(RawSerializer {
                encoder: &mut self.encoder,
            });
        }
        if name == TAG_MARKER {
            return serialize_tagged(value, self);
        }
        if name == SIMPLE_MARKER {
            return value.serialize(SimpleSerializer {
                encoder: &mut self.encoder,
            });
        }
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.encoder.map(1).map_err(Error::Write)?;
        self.encoder.text(variant).map_err(Error::Write)?;
        value.serialize(self)
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let indefinite = len.is_none();
        match len {
            Some(len) => self.encoder.array(len).map_err(Error::Write)?,
            None => self.encoder.begin_array().map_err(Error::Write)?,
        }
        Ok(SeqSerializer {
            serializer: self,
            indefinite,
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.encoder.array(len).map_err(Error::Write)?;
        Ok(SeqSerializer {
            serializer: self,
            indefinite: false,
        })
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.encoder.array(len).map_err(Error::Write)?;
        Ok(SeqSerializer {
            serializer: self,
            indefinite: false,
        })
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.encoder.map(1).map_err(Error::Write)?;
        self.encoder.text(variant).map_err(Error::Write)?;
        self.encoder.array(len).map_err(Error::Write)?;
        Ok(TupleVariantSerializer {
            serializer: self,
        })
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let indefinite = len.is_none();
        match len {
            Some(len) => self.encoder.map(len).map_err(Error::Write)?,
            None => self.encoder.begin_map().map_err(Error::Write)?,
        }
        Ok(MapSerializer {
            serializer: self,
            indefinite,
        })
    }
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.encoder.map(len).map_err(Error::Write)?;
        Ok(MapSerializer {
            serializer: self,
            indefinite: false,
        })
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.encoder.map(1).map_err(Error::Write)?;
        self.encoder.text(variant).map_err(Error::Write)?;
        self.encoder.map(len).map_err(Error::Write)?;
        Ok(StructVariantSerializer {
            serializer: self,
        })
    }
}

/// Serializer for CBOR arrays and tuples.
pub struct SeqSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    indefinite: bool,
}

impl<W: Write> SerializeSeq for SeqSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<(), Self::Error> {
        if self.indefinite {
            self.serializer.encoder.end().map_err(Error::Write)?;
        }
        Ok(())
    }
}

impl<W: Write> SerializeTuple for SeqSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<W: Write> SerializeTupleStruct for SeqSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Serializer for a tuple enum variant.
pub struct TupleVariantSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
}

impl<W: Write> SerializeTupleVariant for TupleVariantSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Serializer for CBOR maps and structs.
pub struct MapSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
    indefinite: bool,
}

impl<W: Write> SerializeMap for MapSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        key.serialize(&mut *self.serializer)
    }
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<(), Self::Error> {
        if self.indefinite {
            self.serializer.encoder.end().map_err(Error::Write)?;
        }
        Ok(())
    }
}

impl<W: Write> SerializeStruct for MapSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        self.serializer.encoder.text(key).map_err(Error::Write)?;
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Serializer for a struct enum variant.
pub struct StructVariantSerializer<'a, W> {
    serializer: &'a mut Serializer<W>,
}

impl<W: Write> SerializeStructVariant for StructVariantSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        self.serializer.encoder.text(key).map_err(Error::Write)?;
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct SimpleSerializer<'a, W> {
    encoder: &'a mut Encoder<W>,
}

impl<W: Write> ser::Serializer for SimpleSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    type SerializeSeq = Impossible<(), Error<W::Error>>;
    type SerializeTuple = Impossible<(), Error<W::Error>>;
    type SerializeTupleStruct = Impossible<(), Error<W::Error>>;
    type SerializeTupleVariant = Impossible<(), Error<W::Error>>;
    type SerializeMap = Impossible<(), Error<W::Error>>;
    type SerializeStruct = Impossible<(), Error<W::Error>>;
    type SerializeStructVariant = Impossible<(), Error<W::Error>>;

    fn serialize_u8(self, value: u8) -> Result<(), Self::Error> {
        let simple = Simple::new(value).map_err(|_| Error::Message("invalid simple value"))?;
        self.encoder.simple(simple).map_err(Error::Write)
    }
    fn serialize_u64(self, value: u64) -> Result<(), Self::Error> {
        if value > u64::from(u8::MAX) {
            return Err(Error::Message("simple value does not fit u8"));
        }
        self.serialize_u8(value as u8)
    }
    fn serialize_bool(self, _: bool) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_i8(self, _: i8) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_i16(self, _: i16) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_i32(self, _: i32) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_i64(self, _: i64) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_i128(self, _: i128) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_u16(self, _: u16) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_u32(self, _: u32) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_u128(self, _: u128) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_f32(self, _: f32) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_f64(self, _: f64) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_char(self, _: char) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_str(self, _: &str) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Message("expected a simple value"))
    }
}

/// Serializer used only for the `RawValue` marker: it accepts a byte string
/// and splices its contents into the output unchanged.
struct RawSerializer<'a, W> {
    encoder: &'a mut Encoder<W>,
}

impl<W: Write> ser::Serializer for RawSerializer<'_, W>
where
    W::Error: core::fmt::Debug + core::fmt::Display,
{
    type Ok = ();
    type Error = Error<W::Error>;
    type SerializeSeq = Impossible<(), Error<W::Error>>;
    type SerializeTuple = Impossible<(), Error<W::Error>>;
    type SerializeTupleStruct = Impossible<(), Error<W::Error>>;
    type SerializeTupleVariant = Impossible<(), Error<W::Error>>;
    type SerializeMap = Impossible<(), Error<W::Error>>;
    type SerializeStruct = Impossible<(), Error<W::Error>>;
    type SerializeStructVariant = Impossible<(), Error<W::Error>>;

    fn serialize_bytes(self, value: &[u8]) -> Result<(), Self::Error> {
        self.encoder.raw(value).map_err(Error::Write)
    }
    fn serialize_bool(self, _: bool) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_i8(self, _: i8) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_i16(self, _: i16) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_i32(self, _: i32) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_i64(self, _: i64) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_i128(self, _: i128) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_u8(self, _: u8) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_u16(self, _: u16) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_u32(self, _: u32) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_u64(self, _: u64) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_u128(self, _: u128) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_f32(self, _: f32) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_f64(self, _: f64) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_char(self, _: char) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_str(self, _: &str) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, _: &T) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Message("expected raw CBOR bytes"))
    }
}

/// The deserializer.
pub struct Deserializer<'de> {
    decoder: Decoder<'de>,
    depth: usize,
}

impl<'de> Deserializer<'de> {
    /// Create a deserializer over a decoder.
    pub fn new(decoder: Decoder<'de>) -> Self {
        Deserializer {
            decoder,
            depth: 0,
        }
    }

    fn enter(&mut self) -> Result<(), DeError> {
        if self.depth >= self.decoder.options().max_depth {
            return Err(DeError::message("CBOR recursion limit exceeded"));
        }
        self.depth += 1;
        Ok(())
    }

    fn leave(&mut self) {
        debug_assert!(self.depth > 0);
        self.depth -= 1;
    }

    fn skip_tags(&mut self) -> Result<(), DeError> {
        while self.decoder.datatype()? == Type::Tag {
            let _ = self.decoder.tag()?;
        }
        Ok(())
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = DeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        // Tags are transparent to serde's data model. Consume them
        // iteratively so a long tag chain cannot exhaust the stack.
        self.skip_tags()?;
        match self.decoder.datatype()? {
            Type::Null => {
                self.decoder.null()?;
                visitor.visit_unit()
            }
            Type::Undefined => {
                self.decoder.undefined()?;
                visitor.visit_unit()
            }
            Type::Bool => visitor.visit_bool(self.decoder.bool()?),
            Type::U8 | Type::U16 | Type::U32 | Type::U64 => visitor.visit_u64(self.decoder.u64()?),
            Type::I8 | Type::I16 | Type::I32 | Type::I64 => visitor.visit_i64(self.decoder.i64()?),
            Type::F16 | Type::F32 | Type::F64 => visitor.visit_f64(self.decoder.f64()?),
            Type::Simple => visitor.visit_u64(u64::from(self.decoder.simple()?.get())),
            Type::Bytes => visitor.visit_borrowed_bytes(self.decoder.bytes()?),
            Type::BytesIndef => visitor.visit_byte_buf(self.decoder.bytes_owned()?),
            Type::Text => visitor.visit_borrowed_str(self.decoder.text()?),
            Type::TextIndef => visitor.visit_string(self.decoder.text_owned()?),
            Type::Array | Type::ArrayIndef => {
                let remaining = self.decoder.array()?;
                self.enter()?;
                let result = visitor.visit_seq(SeqAccess {
                    deserializer: self,
                    remaining,
                });
                self.leave();
                result
            }
            Type::Map | Type::MapIndef => {
                let remaining = self.decoder.map()?;
                self.enter()?;
                let result = visitor.visit_map(MapAccess::new(self, remaining));
                self.leave();
                result
            }
            Type::Tag => Err(DeError::message("unexpected tag")),
            Type::Break => Err(DeError::message("unexpected break")),
            Type::Unknown(_) => Err(DeError::message("reserved CBOR item")),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_bool(self.decoder.bool()?)
    }
    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i8(self.decoder.i8()?)
    }
    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i16(self.decoder.i16()?)
    }
    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i32(self.decoder.i32()?)
    }
    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i64(self.decoder.i64()?)
    }
    fn deserialize_i128<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i128(self.decoder.i128()?)
    }
    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u8(self.decoder.u8()?)
    }
    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u16(self.decoder.u16()?)
    }
    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u32(self.decoder.u32()?)
    }
    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u64(self.decoder.u64()?)
    }
    fn deserialize_u128<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u128(self.decoder.u128()?)
    }
    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_f32(self.decoder.f32()?)
    }
    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_f64(self.decoder.f64()?)
    }
    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_char(self.decoder.decode::<char>()?)
    }
    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.decoder.peek_byte()? & 0x1f == 31 {
            visitor.visit_string(self.decoder.text_owned()?)
        } else {
            visitor.visit_borrowed_str(self.decoder.text()?)
        }
    }
    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_string(self.decoder.text_owned()?)
    }
    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.decoder.peek_byte()? & 0x1f == 31 {
            visitor.visit_byte_buf(self.decoder.bytes_owned()?)
        } else {
            visitor.visit_borrowed_bytes(self.decoder.bytes()?)
        }
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_byte_buf(self.decoder.bytes_owned()?)
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.decoder.datatype()? {
            Type::Null | Type::Undefined => {
                self.decoder.skip()?;
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }
    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.decoder.null()?;
        visitor.visit_unit()
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_unit(visitor)
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if name == RAW_MARKER {
            let start = self.decoder.position();
            self.decoder.skip()?;
            let end = self.decoder.position();
            return visitor.visit_borrowed_bytes(&self.decoder.input()[start..end]);
        }
        if name == TAG_MARKER {
            let tag = self.decoder.tag()?;
            return visitor.visit_map(TagMapAccess {
                deserializer: self,
                tag: Some(tag.get()),
            });
        }
        if name == SIMPLE_MARKER {
            return visitor.visit_u8(self.decoder.simple()?.get());
        }
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.skip_tags()?;
        let remaining = self.decoder.array()?;
        self.enter()?;
        let result = visitor.visit_seq(SeqAccess {
            deserializer: self,
            remaining,
        });
        self.leave();
        result
    }
    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.skip_tags()?;
        let remaining = self.decoder.map()?;
        self.enter()?;
        let result = visitor.visit_map(MapAccess::new(self, remaining));
        self.leave();
        result
    }
    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.skip_tags()?;
        match self.decoder.datatype()? {
            Type::Map | Type::MapIndef => self.deserialize_map(visitor),
            Type::Array | Type::ArrayIndef => self.deserialize_seq(visitor),
            _ => Err(DeError::message("expected a CBOR map or array for a struct")),
        }
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        while self.decoder.datatype()? == Type::Tag {
            let _ = self.decoder.tag()?;
        }
        self.enter()?;
        let result = (|| match self.decoder.datatype()? {
            Type::Text => {
                let variant = self.decoder.text()?;
                visitor.visit_enum(UnitEnumAccess {
                    variant,
                })
            }
            Type::Map | Type::MapIndef => {
                let pair = self.decoder.map()?;
                match pair {
                    Some(1) => {}
                    Some(_) => return Err(DeError::message("expected a one-entry enum map")),
                    None if self.decoder.is_break() => {
                        return Err(DeError::message("expected a one-entry enum map"));
                    }
                    None => {}
                }
                let key = self.decoder.datatype()?;
                let variant = match key {
                    Type::Text => VariantId::Text(self.decoder.text()?),
                    Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::I8 | Type::I16 | Type::I32 | Type::I64 => {
                        VariantId::Int(self.decoder.i64()?)
                    }
                    _ => return Err(DeError::message("expected a variant identifier")),
                };
                let result = visitor.visit_enum(EnumAccess {
                    deserializer: self,
                    variant,
                    array: None,
                });
                if pair.is_none() {
                    if !self.decoder.is_break() {
                        return Err(DeError::message("expected one enum map entry"));
                    }
                    self.decoder.read_break()?;
                }
                result
            }
            Type::Array | Type::ArrayIndef => {
                let len = self.decoder.array()?;
                if len == Some(0) {
                    return Err(DeError::message("enum array is missing the variant index"));
                }
                if len.is_none() && self.decoder.is_break() {
                    return Err(DeError::message("enum array is missing the variant index"));
                }
                let index = self.decoder.u64()?;
                visitor.visit_enum(EnumAccess {
                    deserializer: self,
                    variant: VariantId::Index(index),
                    array: Some(len.map(|n| n.saturating_sub(1))),
                })
            }
            _ => Err(DeError::message("expected a CBOR text, map or array for an enum")),
        })();
        self.leave();
        result
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.decoder.datatype()? {
            Type::Text => visitor.visit_borrowed_str(self.decoder.text()?),
            Type::U8 | Type::U16 | Type::U32 | Type::U64 => visitor.visit_u64(self.decoder.u64()?),
            Type::I8 | Type::I16 | Type::I32 | Type::I64 => visitor.visit_i64(self.decoder.i64()?),
            _ => Err(DeError::message("expected a text or integer identifier")),
        }
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.decoder.skip()?;
        visitor.visit_unit()
    }
}

struct SeqAccess<'a, 'de> {
    deserializer: &'a mut Deserializer<'de>,
    remaining: Option<usize>,
}

impl<'de> de::SeqAccess<'de> for SeqAccess<'_, 'de> {
    type Error = DeError;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
        if let Some(remaining) = &mut self.remaining {
            if *remaining == 0 {
                return Ok(None);
            }
            *remaining -= 1;
        } else if self.deserializer.decoder.is_break() {
            self.deserializer.decoder.read_break()?;
            return Ok(None);
        }
        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

const INLINE_KEYS: usize = 8;

enum SeenKeys<'de> {
    Inline {
        keys: [Option<&'de [u8]>; INLINE_KEYS],
        len: usize,
    },
    Tree(BTreeSet<&'de [u8]>),
}

impl<'de> SeenKeys<'de> {
    fn new() -> Self {
        SeenKeys::Inline {
            keys: [None; INLINE_KEYS],
            len: 0,
        }
    }

    fn insert(&mut self, key: &'de [u8]) -> bool {
        match self {
            SeenKeys::Inline {
                keys,
                len,
            } => {
                if keys[..*len].iter().flatten().any(|seen| *seen == key) {
                    return false;
                }
                if *len < INLINE_KEYS {
                    keys[*len] = Some(key);
                    *len += 1;
                    return true;
                }
                let mut tree = BTreeSet::new();
                for seen in keys.iter().flatten() {
                    tree.insert(*seen);
                }
                let inserted = tree.insert(key);
                *self = SeenKeys::Tree(tree);
                inserted
            }
            SeenKeys::Tree(tree) => tree.insert(key),
        }
    }
}

struct MapAccess<'a, 'de> {
    deserializer: &'a mut Deserializer<'de>,
    remaining: Option<usize>,
    seen: Option<SeenKeys<'de>>,
}

impl<'a, 'de> MapAccess<'a, 'de> {
    fn new(deserializer: &'a mut Deserializer<'de>, remaining: Option<usize>) -> Self {
        let seen = if deserializer.decoder.options().duplicate_keys == DuplicateKeys::Error {
            Some(SeenKeys::new())
        } else {
            None
        };
        MapAccess {
            deserializer,
            remaining,
            seen,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'_, 'de> {
    type Error = DeError;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        if let Some(remaining) = &mut self.remaining {
            if *remaining == 0 {
                return Ok(None);
            }
            *remaining -= 1;
        } else if self.deserializer.decoder.is_break() {
            self.deserializer.decoder.read_break()?;
            return Ok(None);
        }
        let start = self.deserializer.decoder.position();
        let value = seed.deserialize(MapKeyDeserializer {
            deserializer: self.deserializer,
        })?;
        let end = self.deserializer.decoder.position();
        if let Some(seen) = &mut self.seen {
            // This catches duplicate keys with identical wire encodings. Keys
            // that are semantically equal but encoded differently are caught
            // only by the native typed API or a canonical validation pass.
            let key = &self.deserializer.decoder.input()[start..end];
            if !seen.insert(key) {
                return Err(DeError::from_core(CoreError::duplicate_key().at(start)));
            }
        }
        Ok(Some(value))
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        seed.deserialize(&mut *self.deserializer)
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

struct MapKeyDeserializer<'a, 'de> {
    deserializer: &'a mut Deserializer<'de>,
}

impl<'de> de::Deserializer<'de> for MapKeyDeserializer<'_, 'de> {
    type Error = DeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserializer.deserialize_any(visitor)
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserializer.deserialize_identifier(visitor)
    }
    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserializer.deserialize_str(visitor)
    }
    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserializer.deserialize_string(visitor)
    }
    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserializer.deserialize_u64(visitor)
    }
    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserializer.deserialize_i64(visitor)
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserializer.deserialize_ignored_any(visitor)
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i128 u8 u16 u32 u128 f32 f64 char bytes byte_buf option unit
        unit_struct newtype_struct seq tuple tuple_struct map struct enum
    }
}

enum VariantId<'de> {
    Text(&'de str),
    Int(i64),
    Index(u64),
}

struct VariantDeserializer<'de> {
    variant: VariantId<'de>,
}

impl<'de> de::Deserializer<'de> for VariantDeserializer<'de> {
    type Error = DeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.variant {
            VariantId::Text(text) => visitor.visit_borrowed_str(text),
            VariantId::Int(value) => visitor.visit_i64(value),
            VariantId::Index(value) => visitor.visit_u64(value),
        }
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum
        ignored_any
    }
}

struct TagMapAccess<'a, 'de> {
    deserializer: &'a mut Deserializer<'de>,
    tag: Option<u64>,
}

impl<'de> de::MapAccess<'de> for TagMapAccess<'_, 'de> {
    type Error = DeError;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        match self.tag.take() {
            Some(tag) => seed
                .deserialize(TagNumberDeserializer {
                    tag,
                })
                .map(Some),
            None => Ok(None),
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        seed.deserialize(&mut *self.deserializer)
    }
}

struct TagNumberDeserializer {
    tag: u64,
}

impl<'de> de::Deserializer<'de> for TagNumberDeserializer {
    type Error = DeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u64(self.tag)
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u64(self.tag)
    }
    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u64(self.tag)
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u128 f32 f64 char str string bytes byte_buf
        option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum ignored_any
    }
}

struct UnitEnumAccess<'de> {
    variant: &'de str,
}

impl<'de> de::EnumAccess<'de> for UnitEnumAccess<'de> {
    type Error = DeError;
    type Variant = UnitVariant;

    fn variant_seed<V: de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
        let value = seed.deserialize(VariantDeserializer {
            variant: VariantId::Text(self.variant),
        })?;
        Ok((value, UnitVariant))
    }
}

struct UnitVariant;

impl<'de> de::VariantAccess<'de> for UnitVariant {
    type Error = DeError;
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, _seed: T) -> Result<T::Value, Self::Error> {
        Err(DeError::message("expected a unit variant"))
    }
    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(DeError::message("expected a unit variant"))
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(DeError::message("expected a unit variant"))
    }
}

struct EnumAccess<'a, 'de> {
    deserializer: &'a mut Deserializer<'de>,
    variant: VariantId<'de>,
    array: Option<Option<usize>>,
}

impl<'a, 'de> de::EnumAccess<'de> for EnumAccess<'a, 'de> {
    type Error = DeError;
    type Variant = EnumVariant<'a, 'de>;

    fn variant_seed<V: de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
        let value = seed.deserialize(VariantDeserializer {
            variant: self.variant,
        })?;
        Ok((
            value,
            EnumVariant {
                deserializer: self.deserializer,
                array: self.array,
            },
        ))
    }
}

struct EnumVariant<'a, 'de> {
    deserializer: &'a mut Deserializer<'de>,
    array: Option<Option<usize>>,
}

impl<'a, 'de> de::VariantAccess<'de> for EnumVariant<'a, 'de> {
    type Error = DeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.array {
            Some(Some(remaining)) => {
                for _ in 0..remaining {
                    self.deserializer.decoder.skip()?;
                }
            }
            Some(None) => {
                while !self.deserializer.decoder.is_break() {
                    self.deserializer.decoder.skip()?;
                }
                self.deserializer.decoder.read_break()?;
            }
            None => {
                self.deserializer.decoder.skip()?;
            }
        }
        Ok(())
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
        seed.deserialize(&mut *self.deserializer)
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(remaining) = self.array {
            visitor.visit_seq(SeqAccess {
                deserializer: self.deserializer,
                remaining,
            })
        } else {
            self.deserializer.deserialize_seq(visitor)
        }
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if let Some(remaining) = self.array {
            let _ = fields;
            visitor.visit_seq(SeqAccess {
                deserializer: self.deserializer,
                remaining,
            })
        } else {
            self.deserializer.deserialize_map(visitor)
        }
    }
}

/// Deserialize a value from a buffered reader.
#[cfg(feature = "std")]
pub fn from_reader<R: BufRead, T: DeserializeOwned>(mut reader: R) -> Result<T, DeError> {
    let bytes = crate::reader::read_item(&mut reader).map_err(|error| DeError::message(error.to_string()))?;
    from_slice(&bytes)
}
