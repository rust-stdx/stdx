use alloc::format;
use core::fmt;

/// I/O error kind — mirrors common `std::io::ErrorKind` values without std.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoErrorKind {
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    TimedOut,
    Interrupted,
    UnexpectedEof,
    WriteZero,
    Other,
}

/// A no_std-friendly I/O error.
#[derive(Debug, Clone)]
pub struct IoError {
    pub kind: IoErrorKind,
    pub description: &'static str,
}

impl IoError {
    pub const fn new(kind: IoErrorKind, description: &'static str) -> Self {
        Self {
            kind,
            description,
        }
    }

    pub fn kind(&self) -> IoErrorKind {
        self.kind
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.description,
            match self.kind {
                IoErrorKind::ConnectionReset => "connection reset",
                IoErrorKind::ConnectionAborted => "connection aborted",
                IoErrorKind::NotConnected => "not connected",
                IoErrorKind::TimedOut => "timed out",
                IoErrorKind::Interrupted => "interrupted",
                IoErrorKind::UnexpectedEof => "unexpected eof",
                IoErrorKind::WriteZero => "write zero",
                IoErrorKind::Other => "other error",
            }
        )
    }
}

impl From<IoError> for alloc::string::String {
    fn from(e: IoError) -> Self {
        format!("{e}")
    }
}

/// Errors returned by the TLS library.
#[derive(Debug)]
pub enum Error {
    /// The peer sent an unexpected handshake message.
    UnexpectedMessage { expected: &'static str, got: &'static str },

    /// The handshake failed for the given reason.
    HandshakeFailed(alloc::string::String),

    /// Decryption of a record failed (wrong key or tampered data).
    DecryptFailed,

    /// A key is invalid (wrong length or format).
    InvalidKey(alloc::string::String),

    /// Certificate validation failed.
    CertificateValidationFailed(alloc::string::String),

    /// No cipher suite overlap between client and server.
    NoCipherSuitesInCommon,

    /// No key exchange group in common.
    NoKeyExchangeGroupInCommon,

    /// The peer sent a message that could not be parsed.
    DecodeError(alloc::string::String),

    /// An internal error in the TLS state machine.
    InternalError(alloc::string::String),

    /// An I/O error.
    Io(IoError),

    /// The connection was closed by the peer.
    ConnectionClosed,

    /// Crypto provider could not perform the requested operation.
    CryptoError(alloc::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedMessage {
                expected,
                got,
            } => {
                write!(f, "unexpected message: expected {expected}, got {got}")
            }
            Self::HandshakeFailed(msg) => write!(f, "handshake failed: {msg}"),
            Self::DecryptFailed => write!(f, "decryption failed"),
            Self::InvalidKey(msg) => write!(f, "invalid key: {msg}"),
            Self::CertificateValidationFailed(msg) => write!(f, "certificate validation failed: {msg}"),
            Self::NoCipherSuitesInCommon => write!(f, "no cipher suites in common"),
            Self::NoKeyExchangeGroupInCommon => write!(f, "no key exchange group in common"),
            Self::DecodeError(msg) => write!(f, "decode error: {msg}"),
            Self::InternalError(msg) => write!(f, "internal error: {msg}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::ConnectionClosed => write!(f, "connection closed"),
            Self::CryptoError(msg) => write!(f, "crypto error: {msg}"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl core::error::Error for IoError {}
