use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidConfiguration,

    // ── Record layer ──
    InsufficientBuffer,
    RecordOverflow,
    UnexpectedAlert { level: u8, description: u8 },
    ConnectionClosed,

    // ── Decode ──
    DecodeError,
    UnsupportedCipherSuite,
    UnsupportedKeyExchangeGroup,

    // ── Handshake ──
    UnexpectedMessage,
    HandshakeFailure,
    HandshakeAborted { level: u8, description: u8 },
    InvalidCertificate,
    InvalidSignature,
    TranscriptMismatch,

    // ── Crypto ──
    CryptoError,
    AeadError,

    // ── General ──
    NotEstablished,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
