use core::fmt;

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
    #[cfg(feature = "std")]
    Io(std::io::Error),

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
            #[cfg(feature = "std")]
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::ConnectionClosed => write!(f, "connection closed"),
            Self::CryptoError(msg) => write!(f, "crypto error: {msg}"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            #[cfg(feature = "std")]
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
