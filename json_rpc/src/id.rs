use std::fmt;

use json::{Decode, Decoder, Encode, Encoder, ErrorKind, Result, Write};

/// A JSON-RPC request or response identifier.
///
/// Per the spec, an `Id` MUST be a `String`, `Number`, or `Null`.
/// Fractional numbers SHOULD NOT be used.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Id {
    /// An integer identifier.
    Number(i64),
    /// A string identifier.
    String(String),
    /// A null identifier (discouraged per spec).
    Null,
}

impl Id {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Null => write!(f, "null"),
        }
    }
}

impl From<i64> for Id {
    fn from(n: i64) -> Self {
        Self::Number(n)
    }
}

impl From<i32> for Id {
    fn from(n: i32) -> Self {
        Self::Number(n as i64)
    }
}

impl From<u64> for Id {
    fn from(n: u64) -> Self {
        Self::Number(n as i64)
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl Encode for Id {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        match self {
            Self::Number(n) => encoder.i64(*n),
            Self::String(s) => encoder.string(s),
            Self::Null => encoder.null(),
        }
    }
}

impl<'a> Decode<'a> for Id {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'"') => Ok(Self::String(decoder.decode_string()?)),
            Some(b'n') => {
                decoder.decode_null()?;
                Ok(Self::Null)
            }
            Some(b'-' | b'0'..=b'9') => {
                let number = decoder.decode_number()?;
                match number {
                    json::Number::PosInt(v) if v <= i64::MAX as u64 => Ok(Self::Number(v as i64)),
                    json::Number::NegInt(v) => Ok(Self::Number(v)),
                    json::Number::Float(v) if v.fract() == 0.0 && v >= i64::MIN as f64 && v <= i64::MAX as f64 => {
                        Ok(Self::Number(v as i64))
                    }
                    _ => Err(decoder.error(ErrorKind::Message("JSON-RPC id must be an integer or string"))),
                }
            }
            None => Err(decoder.error(ErrorKind::UnexpectedEof)),
            Some(_) => Err(decoder.error(ErrorKind::ExpectedValue)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_json_number() {
        let json = "42";
        let id: Id = json::from_str(json).unwrap();
        assert_eq!(id, Id::Number(42));
        assert_eq!(json::to_string(&id), json);
    }

    #[test]
    fn test_id_json_string() {
        let json = r#""abc""#;
        let id: Id = json::from_str(json).unwrap();
        assert_eq!(id, Id::String("abc".into()));
        assert_eq!(json::to_string(&id), json);
    }

    #[test]
    fn test_id_json_null() {
        let json = "null";
        let id: Id = json::from_str(json).unwrap();
        assert_eq!(id, Id::Null);
        assert_eq!(json::to_string(&id), json);
    }

    #[test]
    fn test_id_deserialize_float_rejected() {
        assert!(json::from_str::<Id>("1.5").is_err());
    }

    #[test]
    fn test_id_deserialize_whole_float_accepted() {
        let id: Id = json::from_str("1.0").unwrap();
        assert_eq!(id, Id::Number(1));
    }

    #[test]
    fn test_id_from_i64() {
        let id: Id = 42i64.into();
        assert_eq!(id, Id::Number(42));
    }

    #[test]
    fn test_id_from_str() {
        let id: Id = "foo".into();
        assert_eq!(id, Id::String("foo".into()));
    }

    #[test]
    fn test_id_display_number() {
        assert_eq!(Id::Number(42).to_string(), "42");
    }

    #[test]
    fn test_id_display_string() {
        assert_eq!(Id::String("abc".into()).to_string(), "abc");
    }

    #[test]
    fn test_id_display_null() {
        assert_eq!(Id::Null.to_string(), "null");
    }
}
