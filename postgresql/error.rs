//! Error types for the `postgresql` crate.

use postgresql_protocol::{DbError, error::Error as ProtocolError};

/// The error type returned by every fallible operation in this crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A server-reported error (`ErrorResponse`).
    #[error("{0}")]
    Server(#[from] DbError),

    /// A wire-protocol, decode, encode or authentication error.
    #[error(transparent)]
    Protocol(ProtocolError),

    /// An I/O error on the connection.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A TLS error.
    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),

    /// The connection string is invalid or unsupported.
    #[error("configuration error: {0}")]
    Config(String),

    /// The connection or pool has been closed.
    #[error("connection closed")]
    Closed,

    /// The connection failed while a result was still streaming.
    ///
    /// The message describes the underlying cause (protocol error, I/O
    /// failure, ...).
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// Acquiring a pooled connection timed out.
    #[error("connection pool timed out")]
    PoolTimedOut,

    /// The connection pool has been closed.
    #[error("connection pool closed")]
    PoolClosed,

    /// A query expected exactly one row but found none.
    #[error("row not found")]
    RowNotFound,

    /// A column name was not present in the result set.
    #[error("column not found: {0}")]
    ColumnNotFound(String),

    /// A `NULL` was found where a non-nullable value was expected.
    #[error("unexpected NULL in column {0}")]
    UnexpectedNull(String),

    /// A value could not be decoded.
    #[error("decode error: {0}")]
    Decode(String),

    /// A value could not be encoded.
    #[error("encode error: {0}")]
    Encode(String),

    /// A value was not a valid `time` date/time.
    #[error("time error: {0}")]
    Time(#[from] time::Error),

    /// A value was not a valid UUID.
    #[error("uuid error: {0}")]
    Uuid(#[from] uuid::Error),

    /// A value was not valid JSON.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// A value was not a valid IP network.
    #[error("ipnetwork error: {0}")]
    IpNetwork(#[from] ipnetwork::IpNetworkError),

    /// A string was not valid UTF-8.
    #[error("invalid UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

impl From<ProtocolError> for Error {
    fn from(err: ProtocolError) -> Self {
        match err {
            ProtocolError::Server(db) => Error::Server(db),
            other => Error::Protocol(other),
        }
    }
}
