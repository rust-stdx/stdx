//! Errors returned by the `yaml` crate.

use alloc::string::String;
use core::fmt;

/// Result type used throughout the crate.
pub type Result<T> = core::result::Result<T, Error>;

/// A YAML parsing, encoding, or decoding error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
    line: Option<u64>,
    column: Option<u64>,
}

impl Error {
    /// Creates an error with a message and an optional source location.
    ///
    /// Lines and columns are one-based.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    /// Creates an error with a message and a one-based line and column.
    pub fn at(message: impl Into<String>, line: u64, column: u64) -> Self {
        Self {
            message: message.into(),
            line: Some(line),
            column: Some(column),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the one-based line where the error occurred, if known.
    pub fn line(&self) -> Option<u64> {
        self.line
    }

    /// Returns the one-based column where the error occurred, if known.
    pub fn column(&self) -> Option<u64> {
        self.column
    }

    pub fn missing_field(name: &str) -> Self {
        Self::new(alloc::format!("missing field `{name}`"))
    }

    pub fn invalid_type(expected: &str, value: &crate::Value) -> Self {
        Self::new(alloc::format!("expected {expected}, found {}", value.type_name()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)?;
        match (self.line, self.column) {
            (Some(line), Some(column)) => write!(formatter, " at line {line} column {column}"),
            (Some(line), None) => write!(formatter, " at line {line}"),
            _ => Ok(()),
        }
    }
}

impl core::error::Error for Error {}
