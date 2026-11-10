//! JSON parsing and encoding errors.
//!
//! Every parsing error carries the byte offset at which it was detected and,
//! when the input is available, the corresponding one-based line and column.

use core::fmt;

/// The kind of a JSON error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The input ended in the middle of a value.
    UnexpectedEof,
    /// A value was expected but another byte was found.
    ExpectedValue,
    /// A `:` was expected after an object key.
    ExpectedColon,
    /// A `,` or the end of a container was expected.
    ExpectedComma,
    /// An object key (a string) was expected.
    ExpectedKey,
    /// A closing bracket or brace was expected.
    ExpectedEnd,
    /// Bytes remained after the decoded value.
    TrailingData,
    /// A number did not conform to the RFC 8259 grammar.
    InvalidNumber,
    /// A number was outside the requested range or precision.
    NumberOutOfRange,
    /// An invalid escape sequence was found in a string.
    InvalidEscape,
    /// A `\u` escape did not contain four hexadecimal digits.
    InvalidUnicodeEscape,
    /// A lone UTF-16 surrogate was found in a `\u` escape.
    LoneSurrogate,
    /// An unescaped control character (U+0000 to U+001F) was found.
    ControlCharacter,
    /// A string was not valid UTF-8.
    InvalidUtf8,
    /// The configured nesting depth was exceeded.
    DepthLimitExceeded,
    /// The configured container entry limit was exceeded.
    ContainerLimitExceeded,
    /// The configured string length limit was exceeded.
    StringLimitExceeded,
    /// The same object key occurred more than once and duplicates are rejected.
    DuplicateKey,
    /// An unknown object key was found while unknown fields are denied.
    UnknownField,
    /// An enum variant name was not recognised.
    UnknownVariant,
    /// A required struct field was missing.
    MissingField(&'static str),
    /// A value had an unexpected JSON type.
    UnexpectedType {
        /// The type that was expected.
        expected: &'static str,
        /// The type that was found.
        actual: &'static str,
    },
    /// The output buffer was too small.
    BufferTooSmall,
    /// A non-finite floating-point value cannot be represented in JSON.
    NonFiniteNumber,
    /// A generic error with a static message.
    Message(&'static str),
}

/// A JSON error with its byte offset and, when known, its line and column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    offset: usize,
    line: u32,
    column: u32,
}

impl Error {
    /// Creates an error at `offset` without line/column information.
    #[inline]
    pub const fn new(kind: ErrorKind, offset: usize) -> Self {
        Self {
            kind,
            offset,
            line: 0,
            column: 0,
        }
    }

    /// Creates an error at `offset`, deriving line and column from `input`.
    pub fn at(kind: ErrorKind, input: &[u8], offset: usize) -> Self {
        let offset = offset.min(input.len());
        let (line, column) = line_column(input, offset);
        Self {
            kind,
            offset,
            line,
            column,
        }
    }

    /// Returns the error kind.
    #[inline]
    pub const fn kind(self) -> ErrorKind {
        self.kind
    }

    /// Returns the byte offset at which the error was detected.
    #[inline]
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Returns the one-based line, or `0` when it is unknown.
    #[inline]
    pub const fn line(self) -> u32 {
        self.line
    }

    /// Returns the one-based column, or `0` when it is unknown.
    #[inline]
    pub const fn column(self) -> u32 {
        self.column
    }

    /// Returns a copy of this error at a different offset.
    ///
    /// Line and column information is cleared; it can be recomputed with
    /// [`Error::at`] when the input is available.
    #[inline]
    pub const fn at_offset(self, offset: usize) -> Self {
        Self {
            kind: self.kind,
            offset,
            line: 0,
            column: 0,
        }
    }

    /// Returns `true` if the error was caused by the end of the input.
    #[inline]
    pub const fn is_eof(self) -> bool {
        matches!(self.kind, ErrorKind::UnexpectedEof)
    }

    /// Creates an unexpected-type error.
    #[inline]
    pub const fn type_mismatch(expected: &'static str, actual: &'static str, offset: usize) -> Self {
        Self::new(
            ErrorKind::UnexpectedType {
                expected,
                actual,
            },
            offset,
        )
    }

    /// Creates a missing-field error.
    #[inline]
    pub const fn missing_field(name: &'static str, offset: usize) -> Self {
        Self::new(ErrorKind::MissingField(name), offset)
    }

    /// Creates a buffer-too-small error.
    #[inline]
    pub const fn buffer_too_small() -> Self {
        Self::new(ErrorKind::BufferTooSmall, 0)
    }

    /// Creates a depth-limit error.
    #[inline]
    pub const fn depth_limit(offset: usize) -> Self {
        Self::new(ErrorKind::DepthLimitExceeded, offset)
    }

    /// Creates a generic message error.
    #[inline]
    pub const fn message(message: &'static str, offset: usize) -> Self {
        Self::new(ErrorKind::Message(message), offset)
    }
}

fn line_column(input: &[u8], offset: usize) -> (u32, u32) {
    let mut line = 1u32;
    let mut column = 1u32;
    for &byte in &input[..offset] {
        if byte == b'\n' {
            line = line.saturating_add(1);
            column = 1;
        } else {
            column = column.saturating_add(1);
        }
    }
    (line, column)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::UnexpectedEof => write!(f, "unexpected end of input"),
            ErrorKind::ExpectedValue => write!(f, "expected a JSON value"),
            ErrorKind::ExpectedColon => write!(f, "expected `:` after object key"),
            ErrorKind::ExpectedComma => write!(f, "expected `,` or the end of the container"),
            ErrorKind::ExpectedKey => write!(f, "expected an object key"),
            ErrorKind::ExpectedEnd => write!(f, "expected the end of the container"),
            ErrorKind::TrailingData => write!(f, "trailing data after the JSON value"),
            ErrorKind::InvalidNumber => write!(f, "invalid JSON number"),
            ErrorKind::NumberOutOfRange => write!(f, "number out of range"),
            ErrorKind::InvalidEscape => write!(f, "invalid escape sequence"),
            ErrorKind::InvalidUnicodeEscape => write!(f, "invalid `\\u` escape"),
            ErrorKind::LoneSurrogate => write!(f, "lone UTF-16 surrogate"),
            ErrorKind::ControlCharacter => write!(f, "unescaped control character"),
            ErrorKind::InvalidUtf8 => write!(f, "invalid UTF-8"),
            ErrorKind::DepthLimitExceeded => write!(f, "nesting depth limit exceeded"),
            ErrorKind::ContainerLimitExceeded => write!(f, "container entry limit exceeded"),
            ErrorKind::StringLimitExceeded => write!(f, "string length limit exceeded"),
            ErrorKind::DuplicateKey => write!(f, "duplicate object key"),
            ErrorKind::UnknownField => write!(f, "unknown object field"),
            ErrorKind::UnknownVariant => write!(f, "unknown enum variant"),
            ErrorKind::MissingField(name) => write!(f, "missing field `{name}`"),
            ErrorKind::UnexpectedType {
                expected,
                actual,
            } => write!(f, "expected {expected}, found {actual}"),
            ErrorKind::BufferTooSmall => write!(f, "output buffer too small"),
            ErrorKind::NonFiniteNumber => write!(f, "non-finite number cannot be encoded as JSON"),
            ErrorKind::Message(message) => f.write_str(message),
        }?;

        if self.line != 0 {
            write!(f, " at line {} column {}", self.line, self.column)?;
        } else if self.offset != 0 {
            write!(f, " at offset {}", self.offset)?;
        }
        Ok(())
    }
}

impl core::error::Error for Error {}

/// A JSON result.
pub type Result<T, E = Error> = core::result::Result<T, E>;
