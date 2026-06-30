use alloc::{format, string::String};
use core::fmt;

/// I/O error type returned by the [`Transport`](crate::Transport) trait.
///
/// Distinguishes non-fatal (`WouldBlock`, `TimedOut`) from fatal errors
/// so the QUIC state machine can drive non-blocking I/O correctly.
#[derive(Debug)]
pub enum IoError {
    /// Operation would block; try again.
    WouldBlock,
    /// Deadline exceeded.
    TimedOut,
    /// Connection reset by peer.
    ConnectionReset,
    /// Other I/O error.
    Other(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::WouldBlock => write!(f, "operation would block"),
            IoError::TimedOut => write!(f, "timed out"),
            IoError::ConnectionReset => write!(f, "connection reset"),
            IoError::Other(msg) => write!(f, "I/O error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IoError {}

#[cfg(feature = "std")]
impl From<std::io::Error> for IoError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::WouldBlock => IoError::WouldBlock,
            std::io::ErrorKind::TimedOut => IoError::TimedOut,
            std::io::ErrorKind::ConnectionReset => IoError::ConnectionReset,
            _ => IoError::Other(e.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Tls(tls::Error),
    Crypto(crypto::AeadError),
    TransportParam(String),
    ProtocolViolation(String),
    ConnectionRejected(String),
    ConnectionTimedOut,
    ConnectionClosed(u64, String),
    StreamNotFound(u64),
    VarintDecode,
    PacketDecode(String),
    FrameDecode(String),
    InvalidState(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Tls(e) => write!(f, "TLS error: {e}"),
            Error::Crypto(e) => write!(f, "Crypto error: {e}"),
            Error::TransportParam(s) => write!(f, "Transport parameter error: {s}"),
            Error::ProtocolViolation(s) => write!(f, "Protocol violation: {s}"),
            Error::ConnectionRejected(s) => write!(f, "Connection rejected: {s}"),
            Error::ConnectionTimedOut => write!(f, "Connection timed out"),
            Error::ConnectionClosed(code, reason) => {
                write!(f, "Connection closed by peer: code={code} reason={reason}")
            }
            Error::StreamNotFound(id) => write!(f, "Stream not found: {id}"),
            Error::VarintDecode => write!(f, "Varint decode error"),
            Error::PacketDecode(s) => write!(f, "Packet decode error: {s}"),
            Error::FrameDecode(s) => write!(f, "Frame decode error: {s}"),
            Error::InvalidState(s) => write!(f, "Invalid state: {s}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Tls(e) => Some(e),
            Error::Crypto(e) => Some(e),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

/// Maps a TLS error to a QUIC error.
impl From<tls::Error> for Error {
    fn from(e: tls::Error) -> Self {
        match &e {
            tls::Error::ConnectionClosed => Error::ConnectionClosed(0, "TLS connection closed".into()),
            tls::Error::HandshakeFailed(reason) => Error::ConnectionRejected(format!("TLS handshake failed: {reason}")),
            tls::Error::CertificateValidationFailed(reason) => {
                Error::ConnectionRejected(format!("Certificate validation failed: {reason}"))
            }
            _ => Error::Tls(e),
        }
    }
}

impl From<crypto::AeadError> for Error {
    fn from(e: crypto::AeadError) -> Self {
        Error::Crypto(e)
    }
}
