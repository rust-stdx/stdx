use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidConfiguration,

    // ── Record layer ──
    InsufficientBuffer,
    RecordOverflow,
    UnexpectedAlert {
        level: u8,
        description: u8,
    },
    ConnectionClosed,

    // ── Decode ──
    DecodeError,
    UnsupportedCipherSuite,
    UnsupportedKeyExchangeGroup,

    // ── Handshake ──
    UnexpectedMessage,
    HandshakeFailure,
    HandshakeAborted {
        level: u8,
        description: u8,
    },
    InvalidSignature,
    TranscriptMismatch,

    // ── Certificate verification ──
    /// Chain is empty
    CertificateEmptyChain,
    /// Server name is required for X.509 but was not provided
    CertificateServerNameRequired,
    /// No SAN DNS name matched the requested server name
    CertificateSubjectNameMismatch,
    /// End-entity certificate has cA=true
    CertificateEndEntityMustNotBeCa,
    /// EKU does not include serverAuth
    CertificateEkuDoesNotIncludeServerAuth,
    /// An intermediate CA does not have cA=true
    CertificateIntermediateNotCa,
    /// Issuer DN of a cert does not match subject DN of the issuer
    CertificateIssuerSubjectDnMismatch,
    /// Certificate signature verification failed
    CertificateSignatureVerificationFailed,
    /// Certificate is not yet valid
    CertificateNotYetValid,
    /// Certificate has expired
    CertificateExpired,
    /// No trusted root was found for the chain
    CertificateNoTrustedRootFound {
        searched_roots: usize,
    },
    /// No root matched by SPKI (cross-signing)
    CertificateNoRootFoundBySpkiMatching,
    /// Certificate DER parse failure
    CertificateParseFailed,
    /// Certificate list was empty for a RawPublicKey certificate type
    CertificateEmptyRawPublicKey,
    /// Unsupported signature algorithm in certificate
    CertificateUnsupportedSignatureAlgorithm,
    /// RawPublicKey requires a custom verifier
    CertificateRawPublicKeyRequiresCustomVerifier,

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
