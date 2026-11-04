//! CBOR data types, tokens and tag constants.

use core::fmt;

use crate::error::{Error, ErrorKind};

/// The type of a CBOR data item, as reported by [`Decoder::datatype`](crate::Decoder::datatype).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Type {
    /// `false` or `true`.
    Bool,
    /// Simple value 22.
    Null,
    /// Simple value 23.
    Undefined,
    /// Unsigned integer encoded with the shortest head.
    U8,
    /// Unsigned integer encoded with a 16-bit head.
    U16,
    /// Unsigned integer encoded with a 32-bit head.
    U32,
    /// Unsigned integer encoded with a 64-bit head.
    U64,
    /// Negative integer encoded with the shortest head.
    I8,
    /// Negative integer encoded with a 16-bit head.
    I16,
    /// Negative integer encoded with a 32-bit head.
    I32,
    /// Negative integer encoded with a 64-bit head.
    I64,
    /// Half-precision float.
    F16,
    /// Single-precision float.
    F32,
    /// Double-precision float.
    F64,
    /// A simple value other than `false`, `true`, `null` or `undefined`.
    Simple,
    /// Definite-length byte string.
    Bytes,
    /// Indefinite-length byte string.
    BytesIndef,
    /// Definite-length text string.
    Text,
    /// Indefinite-length text string.
    TextIndef,
    /// Definite-length array.
    Array,
    /// Indefinite-length array.
    ArrayIndef,
    /// Definite-length map.
    Map,
    /// Indefinite-length map.
    MapIndef,
    /// Tagged data item.
    Tag,
    /// The "break" stop code.
    Break,
    /// A reserved or unknown initial byte.
    Unknown(u8),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => f.write_str("bool"),
            Type::Null => f.write_str("null"),
            Type::Undefined => f.write_str("undefined"),
            Type::U8 => f.write_str("u8"),
            Type::U16 => f.write_str("u16"),
            Type::U32 => f.write_str("u32"),
            Type::U64 => f.write_str("u64"),
            Type::I8 => f.write_str("i8"),
            Type::I16 => f.write_str("i16"),
            Type::I32 => f.write_str("i32"),
            Type::I64 => f.write_str("i64"),
            Type::F16 => f.write_str("f16"),
            Type::F32 => f.write_str("f32"),
            Type::F64 => f.write_str("f64"),
            Type::Simple => f.write_str("simple"),
            Type::Bytes => f.write_str("bytes"),
            Type::BytesIndef => f.write_str("indefinite bytes"),
            Type::Text => f.write_str("text"),
            Type::TextIndef => f.write_str("indefinite text"),
            Type::Array => f.write_str("array"),
            Type::ArrayIndef => f.write_str("indefinite array"),
            Type::Map => f.write_str("map"),
            Type::MapIndef => f.write_str("indefinite map"),
            Type::Tag => f.write_str("tag"),
            Type::Break => f.write_str("break"),
            Type::Unknown(b) => write!(f, "unknown({b:#04x})"),
        }
    }
}

/// A CBOR integer.
///
/// CBOR encodes negative integers as `-1 - n`; this enum keeps the exact
/// magnitude so the full `-2^64 ..= 2^64 - 1` range is representable without
/// overflow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Integer {
    /// Non-negative integer.
    Unsigned(u64),
    /// Negative integer with magnitude bits `n`; the value is `-1 - n`.
    Negative(u64),
}

impl Integer {
    /// Create an integer from an `i64`.
    #[inline]
    pub const fn from_i64(value: i64) -> Self {
        if value >= 0 {
            Integer::Unsigned(value as u64)
        } else {
            Integer::Negative((-1 - value) as u64)
        }
    }

    /// Create an integer from a `u64`.
    #[inline]
    pub const fn from_u64(value: u64) -> Self {
        Integer::Unsigned(value)
    }

    /// Return `true` if the value is negative.
    #[inline]
    pub const fn is_negative(self) -> bool {
        matches!(self, Integer::Negative(_))
    }

    /// Return the non-negative value, if any.
    #[inline]
    pub const fn as_u64(self) -> Option<u64> {
        match self {
            Integer::Unsigned(v) => Some(v),
            Integer::Negative(_) => None,
        }
    }

    /// Return the value as an `i64`, if it fits.
    #[inline]
    pub const fn as_i64(self) -> Option<i64> {
        match self {
            Integer::Unsigned(v) if v <= i64::MAX as u64 => Some(v as i64),
            Integer::Negative(n) if n <= i64::MAX as u64 => Some(-1 - n as i64),
            _ => None,
        }
    }

    /// Return the value as an `i128`. Every CBOR basic integer fits.
    #[inline]
    pub const fn to_i128(self) -> i128 {
        match self {
            Integer::Unsigned(v) => v as i128,
            Integer::Negative(n) => -1 - n as i128,
        }
    }

    /// Return the value as a `u128`, if it is non-negative.
    #[inline]
    pub const fn to_u128(self) -> Option<u128> {
        match self {
            Integer::Unsigned(v) => Some(v as u128),
            Integer::Negative(_) => None,
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Integer::Unsigned(v) => v.fmt(f),
            Integer::Negative(n) => match self.as_i64() {
                Some(v) => v.fmt(f),
                None => write!(f, "-{}", (*n as u128) + 1),
            },
        }
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Integer::Unsigned(value)
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer::from_i64(value)
    }
}

/// A CBOR tag number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(u64);

impl Tag {
    /// Create a tag from its numeric value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Tag(value)
    }

    /// Return the numeric tag value.
    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for Tag {
    fn from(value: u64) -> Self {
        Tag(value)
    }
}

impl From<Tag> for u64 {
    fn from(value: Tag) -> Self {
        value.0
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A CBOR simple value.
///
/// Only the values that are not already represented by [`bool`], `null` or
/// `undefined` are valid here: `0..=19` and `32..=255`. Values `20..=23` are
/// the well-known `false`, `true`, `null` and `undefined`; `24..=31` are
/// reserved by RFC 8949 and cannot be encoded in a well-formed item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Simple(u8);

impl Simple {
    /// Create a simple value.
    ///
    /// Returns an error for `20..=31`, which are either the well-known
    /// boolean/null values or reserved by the specification.
    #[inline]
    pub const fn new(value: u8) -> Result<Self, Error> {
        if value < 20 || (value >= 32) {
            Ok(Simple(value))
        } else {
            Err(Error::new(ErrorKind::InvalidSimple(value), 0))
        }
    }

    /// Return the numeric simple value.
    #[inline]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl fmt::Display for Simple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "simple({})", self.0)
    }
}

/// Well-known IANA CBOR tag numbers.
///
/// See <https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml>.
pub mod tags {
    /// Standard date/time string.
    pub const DATE_TIME: u64 = 0;
    /// Epoch-based date/time.
    pub const EPOCH_TIME: u64 = 1;
    /// Unsigned bignum.
    pub const BIGPOS: u64 = 2;
    /// Negative bignum.
    pub const BIGNEG: u64 = 3;
    /// Decimal fraction.
    pub const DECIMAL_FRACTION: u64 = 4;
    /// Bigfloat.
    pub const BIGFLOAT: u64 = 5;
    /// Expected conversion to base64url.
    pub const BASE64URL: u64 = 21;
    /// Expected conversion to base64.
    pub const BASE64: u64 = 22;
    /// Expected conversion to base16.
    pub const BASE16: u64 = 23;
    /// Encoded CBOR data item.
    pub const ENCODED_CBOR: u64 = 24;
    /// URI.
    pub const URI: u64 = 32;
    /// base64url text.
    pub const BASE64URL_TEXT: u64 = 33;
    /// base64 text.
    pub const BASE64_TEXT: u64 = 34;
    /// Regular expression.
    pub const REGEX: u64 = 35;
    /// MIME message.
    pub const MIME: u64 = 36;
    /// Binary UUID.
    pub const UUID: u64 = 37;
    /// Set of unique items.
    pub const SET: u64 = 258;
    /// Self-described CBOR.
    pub const SELF_DESCRIBED: u64 = 55799;
}

#[cfg(feature = "serde")]
impl serde::Serialize for Simple {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct(crate::serde::SIMPLE_MARKER, &self.get())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Simple {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = Simple;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a CBOR simple value")
            }
            fn visit_u8<E: serde::de::Error>(self, value: u8) -> Result<Self::Value, E> {
                Simple::new(value).map_err(E::custom)
            }
        }
        deserializer.deserialize_newtype_struct(crate::serde::SIMPLE_MARKER, Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_roundtrips() {
        for v in [0i64, 1, 23, 24, 255, 256, i64::MAX, -1, -24, -25, -256, -257, i64::MIN] {
            assert_eq!(Integer::from_i64(v).as_i64(), Some(v));
        }
        assert_eq!(Integer::from_u64(u64::MAX).as_u64(), Some(u64::MAX));
        assert_eq!(Integer::from_u64(u64::MAX).as_i64(), None);
        assert_eq!(Integer::from_i64(i64::MIN).to_i128(), i64::MIN as i128);
    }

    #[test]
    fn simple_values_are_validated() {
        assert!(Simple::new(0).is_ok());
        assert!(Simple::new(19).is_ok());
        assert!(Simple::new(20).is_err());
        assert!(Simple::new(23).is_err());
        assert!(Simple::new(24).is_err());
        assert!(Simple::new(31).is_err());
        assert!(Simple::new(32).is_ok());
        assert!(Simple::new(255).is_ok());
    }
}
