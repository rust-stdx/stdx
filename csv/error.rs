use core::fmt;

extern crate alloc;
#[cfg(feature = "std")]
use alloc::boxed::Box;
#[cfg(feature = "serde")]
use alloc::string::String;

/// Kinds of errors that can occur while reading CSV data.
#[derive(Clone, Debug, PartialEq)]
pub enum ReadErrorKind {
    /// A quoted field was opened but never closed before end of input.
    UnterminatedQuote,
    /// Characters appeared after a closing quote before a delimiter or newline.
    TrailingContent,
    /// A field contained invalid UTF-8 bytes.
    InvalidUtf8,
    /// The number of fields differs from previous rows (strict mode).
    InconsistentFieldCount { expected: usize, found: usize },
    /// A serde deserialization error occurred. Carries the error message.
    #[cfg(feature = "serde")]
    Deserialize(String),
    /// An I/O error occurred while reading the underlying source.
    Io,
}

/// An error returned when parsing a CSV row.
///
/// Includes the line number, the kind of error, and when applicable an
/// inner source error (e.g. the original `std::io::Error`).
#[derive(Debug)]
pub struct ReadError {
    pub kind: ReadErrorKind,
    pub line: usize,
    pub column: usize,
    #[cfg(feature = "std")]
    source: Option<Box<dyn core::error::Error + Send + Sync + 'static>>,
}

impl ReadError {
    pub fn new(kind: ReadErrorKind, line: usize, column: usize) -> Self {
        ReadError {
            kind,
            line,
            column,
            #[cfg(feature = "std")]
            source: None,
        }
    }

    pub fn kind(&self) -> &ReadErrorKind {
        &self.kind
    }

    /// The inner source error, if any (e.g. the original `std::io::Error` or serde error).
    #[cfg(feature = "std")]
    pub fn into_source(self) -> Option<Box<dyn core::error::Error + Send + Sync + 'static>> {
        self.source
    }
}

impl Clone for ReadError {
    fn clone(&self) -> Self {
        ReadError {
            kind: self.kind.clone(),
            line: self.line,
            column: self.column,
            #[cfg(feature = "std")]
            source: None,
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ReadErrorKind::UnterminatedQuote => {
                write!(f, "unterminated quote at line {}, column {}", self.line, self.column)
            }
            ReadErrorKind::TrailingContent => {
                write!(
                    f,
                    "trailing content after quoted field at line {}, column {}",
                    self.line, self.column
                )
            }
            ReadErrorKind::InvalidUtf8 => {
                write!(f, "invalid UTF-8 at line {}, column {}", self.line, self.column)
            }
            ReadErrorKind::InconsistentFieldCount {
                expected,
                found,
            } => {
                write!(
                    f,
                    "expected {expected} fields, found {found} at line {}, column {}",
                    self.line, self.column
                )
            }
            #[cfg(feature = "serde")]
            ReadErrorKind::Deserialize(msg) => {
                write!(f, "deserialization error at line {}, column {}: {msg}", self.line, self.column)
            }
            ReadErrorKind::Io => {
                write!(f, "I/O error at line {}, column {}", self.line, self.column)
            }
        }
    }
}

impl core::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        #[cfg(feature = "std")]
        {
            self.source
                .as_ref()
                .map(|b| b.as_ref() as &(dyn core::error::Error + 'static))
        }
        #[cfg(not(feature = "std"))]
        {
            None
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ReadError {
    fn from(e: std::io::Error) -> Self {
        ReadError {
            kind: ReadErrorKind::Io,
            line: 0,
            column: 0,
            source: Some(Box::new(e)),
        }
    }
}

/// An error returned when writing CSV data.
#[derive(Debug)]
pub enum WriteError {
    /// The number of fields in a row differs from previous rows.
    InconsistentFieldCount { expected: usize, found: usize, row: usize },
    /// A second attempt was made to write headers after they've already been written.
    HeadersAlreadyWritten,
    /// A serde serialization error occurred.
    #[cfg(feature = "serde")]
    Serialize(String),
    /// An I/O error occurred while writing.
    #[cfg(feature = "std")]
    Io(std::io::Error),
    /// An I/O error occurred while writing (no_std).
    #[cfg(not(feature = "std"))]
    Io,
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriteError::InconsistentFieldCount {
                expected,
                found,
                row,
            } => {
                write!(f, "expected {expected} fields, found {found} in row {row}")
            }
            WriteError::HeadersAlreadyWritten => write!(f, "headers have already been written"),
            #[cfg(feature = "serde")]
            WriteError::Serialize(msg) => write!(f, "serialization error: {msg}"),
            #[cfg(feature = "std")]
            WriteError::Io(e) => write!(f, "I/O error: {e}"),
            #[cfg(not(feature = "std"))]
            WriteError::Io => write!(f, "I/O error"),
        }
    }
}

impl core::error::Error for WriteError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            #[cfg(feature = "std")]
            WriteError::Io(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for WriteError {
    fn from(e: std::io::Error) -> Self {
        WriteError::Io(e)
    }
}
