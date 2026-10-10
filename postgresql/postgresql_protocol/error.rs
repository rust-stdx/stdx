//! Errors produced while encoding or decoding protocol messages.

use std::fmt;

/// A structured `ErrorResponse` / `NoticeResponse` from the server.
#[derive(Debug, Clone, Default)]
pub struct DbError {
    /// Localized severity (`ERROR`, `FATAL`, …).
    pub severity: String,
    /// SQLSTATE code.
    pub code: String,
    /// Primary human-readable message.
    pub message: String,
    /// Optional detail.
    pub detail: Option<String>,
    /// Optional hint.
    pub hint: Option<String>,
    /// Optional 1-based character position in the query.
    pub position: Option<i32>,
    /// Optional constraint name.
    pub constraint: Option<String>,
    /// Optional table name.
    pub table: Option<String>,
    /// Optional column name.
    pub column: Option<String>,
    /// Optional schema name.
    pub schema: Option<String>,
    /// Optional data type name.
    pub data_type: Option<String>,
}

impl DbError {
    /// Returns the SQLSTATE class (the first two characters), e.g. `23` for
    /// integrity constraint violations.
    ///
    /// Returns `""` when no code was reported. The code is server-supplied and
    /// may contain arbitrary bytes, so a short or non-ASCII code is truncated
    /// at a character boundary rather than panicking.
    pub fn class(&self) -> &str {
        self.code.get(..2).or_else(|| self.code.get(..1)).unwrap_or("")
    }

    /// `true` for SQLSTATE class `22` (data exception).
    pub fn is_data_exception(&self) -> bool {
        self.class() == "22"
    }

    /// `true` for SQLSTATE class `23` (integrity constraint violation).
    pub fn is_integrity_violation(&self) -> bool {
        self.class() == "23"
    }

    /// `true` for `23505` (unique violation).
    pub fn is_unique_violation(&self) -> bool {
        self.code == "23505"
    }

    /// `true` for `23503` (foreign key violation).
    pub fn is_foreign_key_violation(&self) -> bool {
        self.code == "23503"
    }

    /// `true` for `40P01` (deadlock detected) or `40001` (serialization failure),
    /// both of which are safe to retry.
    pub fn is_retryable(&self) -> bool {
        self.code == "40001" || self.code == "40P01"
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} (SQLSTATE {})", self.severity, self.message, self.code)?;
        if let Some(detail) = &self.detail {
            write!(f, "\nDETAIL: {detail}")?;
        }
        if let Some(hint) = &self.hint {
            write!(f, "\nHINT: {hint}")?;
        }
        Ok(())
    }
}

impl std::error::Error for DbError {}

/// Errors that can occur while speaking the PostgreSQL wire protocol.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The server sent a message we did not expect in this state.
    #[error("protocol error: {0}")]
    Protocol(String),
    /// A message could not be decoded.
    #[error("decode error: {0}")]
    Decode(String),
    /// A message could not be encoded.
    #[error("encode error: {0}")]
    Encode(String),
    /// The server reported an error.
    #[error("server error: {0}")]
    Server(#[from] DbError),
    /// Authentication failed.
    #[error("authentication error: {0}")]
    Auth(String),
}
