//! Error types for columnar encoding and decoding.

use core::{convert::Infallible, fmt};

use crate::Encoding;

/// Result type used by the columnar codec.
pub type Result<T> = core::result::Result<T, Error>;

/// An error returned while encoding or decoding a columnar value.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The encoder could not write to its output buffer.
    ///
    /// Encoding into a `Vec<u8>` never produces this error in practice; it is
    /// kept so the encoder API does not panic on impossible I/O failures.
    Encode,
    /// The CBOR envelope or a nested payload was malformed.
    Cbor(cbor::Error),
    /// The envelope version is not supported by this build.
    UnsupportedVersion(u64),
    /// The envelope header (`key = 0`) is missing or invalid.
    InvalidHeader,
    /// A column appeared before the envelope header.
    MissingHeader,
    /// A map key appeared more than once.
    DuplicateKey(u64),
    /// Key `0` is reserved for the envelope header.
    ReservedKey(u64),
    /// An unknown or unsupported column encoding identifier was found.
    InvalidEncoding(u64),
    /// The requested encoding is not valid for the Rust type.
    UnsupportedEncoding(Encoding),
    /// A column's logical length is inconsistent with the envelope.
    LengthMismatch {
        /// The expected number of values.
        expected: usize,
        /// The number of values that were actually present.
        actual: usize,
    },
    /// A string column contains invalid UTF-8.
    InvalidUtf8,
    /// A `char` column contains an invalid Unicode scalar value.
    InvalidChar(u32),
    /// A bit-packed payload contains a value that does not fit its width.
    InvalidBitpackedValue,
    /// A variable-length integer is overlong or does not fit in 128 bits.
    InvalidVarint,
    /// A run-length encoded payload contains a zero-length run.
    ZeroRun,
    /// A map column contains the same key twice in one row.
    DuplicateMapKey,
    /// A dictionary index is outside the dictionary.
    DictionaryIndex {
        /// The invalid index.
        index: u32,
        /// The dictionary length.
        len: usize,
    },
    /// A list offset is not monotonically increasing or is out of bounds.
    InvalidOffset,
    /// A declared or computed length overflowed `usize`.
    LengthOverflow,
    /// A delta or delta-of-delta value does not fit the accumulator or the
    /// column type.
    DeltaOverflow,
    /// A configured decoding limit was exceeded.
    LimitExceeded(&'static str),
    /// Data remains after the envelope.
    TrailingData,
    /// A required field is missing and has no default.
    MissingField(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Encode => f.write_str("columnar encode error"),
            Error::Cbor(error) => write!(f, "CBOR error: {error}"),
            Error::UnsupportedVersion(version) => write!(f, "unsupported columnar version {version}"),
            Error::InvalidHeader => f.write_str("invalid columnar header"),
            Error::MissingHeader => f.write_str("columnar header must be the first map entry"),
            Error::DuplicateKey(key) => write!(f, "duplicate columnar key {key}"),
            Error::ReservedKey(key) => write!(f, "columnar key {key} is reserved"),
            Error::InvalidEncoding(encoding) => write!(f, "invalid columnar encoding {encoding}"),
            Error::UnsupportedEncoding(encoding) => {
                write!(f, "encoding {encoding:?} is not supported for this type")
            }
            Error::LengthMismatch {
                expected,
                actual,
            } => {
                write!(f, "columnar length mismatch: expected {expected}, got {actual}")
            }
            Error::InvalidUtf8 => f.write_str("invalid UTF-8 in string column"),
            Error::InvalidChar(value) => write!(f, "invalid Unicode scalar value {value:#x}"),
            Error::InvalidBitpackedValue => f.write_str("bit-packed value does not fit its width"),
            Error::InvalidVarint => f.write_str("invalid or overlong variable-length integer"),
            Error::ZeroRun => f.write_str("run-length encoding contains a zero-length run"),
            Error::DuplicateMapKey => f.write_str("duplicate key in map column"),
            Error::DictionaryIndex {
                index,
                len,
            } => {
                write!(f, "dictionary index {index} is outside dictionary of length {len}")
            }
            Error::InvalidOffset => f.write_str("invalid list or string offset"),
            Error::LengthOverflow => f.write_str("columnar length overflow"),
            Error::DeltaOverflow => f.write_str("columnar delta value overflows the column type"),
            Error::LimitExceeded(what) => write!(f, "columnar decoding limit exceeded: {what}"),
            Error::TrailingData => f.write_str("trailing data after columnar envelope"),
            Error::MissingField(name) => write!(f, "missing columnar field {name}"),
        }
    }
}

impl From<cbor::Error> for Error {
    fn from(error: cbor::Error) -> Self {
        Error::Cbor(error)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Encode
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Cbor(error) => Some(error),
            _ => None,
        }
    }
}
