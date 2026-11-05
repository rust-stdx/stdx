//! CBOR decoding and encoding errors.
//!
//! Most variants are produced while decoding. [`ErrorKind::BufferTooSmall`] is
//! produced when encoding into a fixed-size buffer.

use core::fmt;

use crate::data::Type;

/// The kind of a decoding error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The input ended in the middle of an item.
    UnexpectedEof,
    /// A data item had an unexpected type.
    UnexpectedType {
        /// The type that was expected.
        expected: Type,
        /// The type that was found.
        actual: Type,
    },
    /// A text string was not valid UTF-8.
    InvalidUtf8,
    /// A simple value was reserved or not representable.
    InvalidSimple(u8),
    /// The configured recursion limit was exceeded.
    DepthLimitExceeded,
    /// The configured container entry limit was exceeded, or an
    /// indefinite-length container was found while a limit was configured.
    ContainerLimitExceeded,
    /// The decoder-wide duplicate-check budget was exceeded.
    DuplicateCheckLimitExceeded,
    /// A map contained the same key twice.
    DuplicateKey,
    /// An enum variant index or name was not recognized.
    UnknownVariant,
    /// A required struct field was missing.
    MissingField(&'static str),
    /// Bytes remained after the item was decoded.
    TrailingData,
    /// A length did not fit in the platform's `usize`.
    LengthOverflow,
    /// The output buffer was too small.
    BufferTooSmall,
    /// A generic error message.
    Message(&'static str),
}

/// A decoding error with the offset of the offending item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    offset: usize,
}

impl Error {
    /// Create an error at the given offset.
    #[inline]
    pub const fn new(kind: ErrorKind, offset: usize) -> Self {
        Error {
            kind,
            offset,
        }
    }

    /// Return the error kind.
    #[inline]
    pub const fn kind(self) -> ErrorKind {
        self.kind
    }

    /// Return the byte offset at which the error was detected.
    #[inline]
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Return a copy of this error with a new offset.
    #[inline]
    pub const fn at(self, offset: usize) -> Self {
        Error {
            kind: self.kind,
            offset,
        }
    }

    /// Return `true` if the error was caused by the end of the input.
    #[inline]
    pub const fn is_eof(self) -> bool {
        matches!(self.kind, ErrorKind::UnexpectedEof)
    }

    /// Create an unexpected-type error.
    #[inline]
    pub const fn type_mismatch(expected: Type, actual: Type) -> Self {
        Error::new(
            ErrorKind::UnexpectedType {
                expected,
                actual,
            },
            0,
        )
    }

    /// Create a generic message error.
    #[inline]
    pub const fn message(message: &'static str) -> Self {
        Error::new(ErrorKind::Message(message), 0)
    }

    /// Create an end-of-input error.
    #[inline]
    pub const fn eof() -> Self {
        Error::new(ErrorKind::UnexpectedEof, 0)
    }

    /// Create a duplicate-key error.
    #[inline]
    pub const fn duplicate_key() -> Self {
        Error::new(ErrorKind::DuplicateKey, 0)
    }

    /// Create a missing-field error.
    #[inline]
    pub const fn missing_field(name: &'static str) -> Self {
        Error::new(ErrorKind::MissingField(name), 0)
    }

    /// Create a trailing-data error.
    #[inline]
    pub const fn trailing_data() -> Self {
        Error::new(ErrorKind::TrailingData, 0)
    }

    /// Create a buffer-too-small error.
    #[inline]
    pub const fn buffer_too_small() -> Self {
        Error::new(ErrorKind::BufferTooSmall, 0)
    }

    /// Create a depth-limit error.
    #[inline]
    pub const fn depth_limit() -> Self {
        Error::new(ErrorKind::DepthLimitExceeded, 0)
    }

    /// Create a container entry-limit error.
    #[inline]
    pub const fn container_limit() -> Self {
        Error::new(ErrorKind::ContainerLimitExceeded, 0)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::UnexpectedEof => write!(f, "unexpected end of input at offset {}", self.offset),
            ErrorKind::UnexpectedType {
                expected,
                actual,
            } => {
                write!(f, "expected {expected}, found {actual} at offset {}", self.offset)
            }
            ErrorKind::InvalidUtf8 => write!(f, "invalid UTF-8 at offset {}", self.offset),
            ErrorKind::InvalidSimple(v) => write!(f, "invalid simple value {v} at offset {}", self.offset),
            ErrorKind::DepthLimitExceeded => {
                write!(f, "recursion limit exceeded at offset {}", self.offset)
            }
            ErrorKind::ContainerLimitExceeded => {
                write!(f, "container entry limit exceeded at offset {}", self.offset)
            }
            ErrorKind::DuplicateCheckLimitExceeded => {
                write!(f, "duplicate-check limit exceeded at offset {}", self.offset)
            }
            ErrorKind::DuplicateKey => write!(f, "duplicate map key at offset {}", self.offset),
            ErrorKind::UnknownVariant => write!(f, "unknown enum variant at offset {}", self.offset),
            ErrorKind::MissingField(name) => {
                write!(f, "missing field `{name}` at offset {}", self.offset)
            }
            ErrorKind::TrailingData => write!(f, "trailing data after item at offset {}", self.offset),
            ErrorKind::LengthOverflow => write!(f, "length does not fit in usize at offset {}", self.offset),
            ErrorKind::BufferTooSmall => write!(f, "output buffer too small at offset {}", self.offset),
            ErrorKind::Message(message) => write!(f, "{message} at offset {}", self.offset),
        }
    }
}

impl core::error::Error for Error {}

/// A decoding result.
pub type Result<T, E = Error> = core::result::Result<T, E>;
