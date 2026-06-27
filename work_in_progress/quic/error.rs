use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TLS error: {0}")]
    Tls(#[source] tls::Error),

    #[error("Crypto error: {0}")]
    Crypto(#[source] crypto::AeadError),

    #[error("Transport parameter error: {0}")]
    TransportParam(String),

    #[error("Protocol violation: {0}")]
    ProtocolViolation(String),

    #[error("Connection rejected: {0}")]
    ConnectionRejected(String),

    #[error("Connection timed out")]
    ConnectionTimedOut,

    #[error("Connection closed by peer: code={0} reason={1}")]
    ConnectionClosed(u64, String),

    #[error("Stream not found: {0}")]
    StreamNotFound(u64),

    #[error("Varint decode error")]
    VarintDecode,

    #[error("Packet decode error: {0}")]
    PacketDecode(String),

    #[error("Frame decode error: {0}")]
    FrameDecode(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
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
