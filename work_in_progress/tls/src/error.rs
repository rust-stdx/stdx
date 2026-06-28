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

// ── Typed error sub-kinds ──────────────────────────────────────────────────

/// Specific reasons a handshake may fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandshakeFailure {
    /// The peer's Finished verify_data did not match.
    FinishedVerificationFailed,
    /// An unrecognized or unsupported KX group.
    UnsupportedKeyExchangeGroup(alloc::string::String),
    /// TLS 1.3 was not offered by the peer.
    Tls13NotOffered,
    /// No `key_share` extension in the hello message.
    NoKeyShare,
    /// CertificateProvider scheme selection conflict.
    SchemeNotOffered(alloc::string::String),
    /// Peer alert received.
    PeerAlert { level: u8, description: u8 },
    /// Other handshake failure.
    Other(alloc::string::String),
}

/// Specific reasons certificate validation may fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateValidationFailure {
    /// Empty certificate chain.
    EmptyChain,
    /// SNI is required for X.509 validation but was not provided.
    ServerNameRequired,
    /// Failed to parse the end-entity certificate.
    ParseError(alloc::string::String),
    /// Invalid server name (e.g. for SAN matching).
    InvalidServerName(alloc::string::String),
    /// Subject name / SAN did not match.
    SubjectNameMismatch(alloc::string::String),
    /// Chain validation against trust anchors failed.
    ChainValidation(alloc::string::String),
    /// Raw public key validation requires a custom validator.
    RawPublicKeyRequiresCustomValidator,
    /// Signature verification failed.
    SignatureVerificationFailed,
    /// Other validation failure.
    Other(alloc::string::String),
}

/// Specific crypto-related errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoFailure {
    /// Unsupported cipher suite.
    UnsupportedCipherSuite(alloc::string::String),
    /// Unsupported signature scheme.
    UnsupportedSignatureScheme(alloc::string::String),
    /// RSA verification failed.
    RsaVerification(alloc::string::String),
    /// Signing operation failed.
    SigningFailed,
    /// Other crypto failure.
    Other(alloc::string::String),
}

/// Specific key-related errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidKeyFailure {
    /// Wrong key length.
    WrongLength { algorithm: &'static str, expected: usize },
    /// Key material could not be parsed.
    ParseError(alloc::string::String),
    /// DER encoding error.
    DerError(alloc::string::String),
    /// Other key error.
    Other(alloc::string::String),
}

impl fmt::Display for HandshakeFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FinishedVerificationFailed => write!(f, "finished verification failed"),
            Self::UnsupportedKeyExchangeGroup(g) => write!(f, "unsupported key exchange group: {g}"),
            Self::Tls13NotOffered => write!(f, "TLS 1.3 not offered"),
            Self::NoKeyShare => write!(f, "no key_share extension"),
            Self::SchemeNotOffered(s) => write!(f, "scheme not offered: {s}"),
            Self::PeerAlert {
                level,
                description,
            } => write!(f, "peer alert: level={level} desc={description}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl fmt::Display for CertificateValidationFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyChain => write!(f, "empty certificate chain"),
            Self::ServerNameRequired => write!(f, "server name (SNI) required for X.509 validation"),
            Self::ParseError(e) => write!(f, "parse error: {e}"),
            Self::InvalidServerName(e) => write!(f, "invalid server name: {e}"),
            Self::SubjectNameMismatch(e) => write!(f, "subject name mismatch: {e}"),
            Self::ChainValidation(e) => write!(f, "chain validation failed: {e}"),
            Self::RawPublicKeyRequiresCustomValidator => {
                write!(f, "raw public key validation requires custom validator")
            }
            Self::SignatureVerificationFailed => write!(f, "signature verification failed"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl fmt::Display for CryptoFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCipherSuite(s) => write!(f, "unsupported cipher suite: {s}"),
            Self::UnsupportedSignatureScheme(s) => write!(f, "unsupported signature scheme: {s}"),
            Self::RsaVerification(e) => write!(f, "RSA verification failed: {e}"),
            Self::SigningFailed => write!(f, "signing failed"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl fmt::Display for InvalidKeyFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongLength {
                algorithm,
                expected,
            } => write!(f, "{algorithm} expects a {expected}-byte key"),
            Self::ParseError(e) => write!(f, "{e}"),
            Self::DerError(e) => write!(f, "DER error: {e}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

/// Errors returned by the TLS library.
#[derive(Debug)]
pub enum Error {
    /// The peer sent an unexpected handshake message.
    UnexpectedMessage { expected: &'static str, got: &'static str },

    /// The handshake failed for the given reason.
    HandshakeFailed(HandshakeFailure),

    /// Decryption of a record failed (wrong key or tampered data).
    DecryptFailed,

    /// A key is invalid (wrong length or format).
    InvalidKey(InvalidKeyFailure),

    /// Certificate validation failed.
    CertificateValidationFailed(CertificateValidationFailure),

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
    CryptoError(CryptoFailure),

    /// A TLS record exceeded the maximum allowed size.
    RecordOverflow,
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
            Self::RecordOverflow => write!(f, "record overflow"),
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
