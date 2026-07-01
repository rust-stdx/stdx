use core::fmt;

use crate::{CipherSuite, KeyExchangeGroup, SignatureScheme};

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
        Self { kind, description }
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

// ── Peer side (client / server) ──────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerSide {
    Client,
    Server,
}

impl fmt::Display for PeerSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Client => f.write_str("client"),
            Self::Server => f.write_str("server"),
        }
    }
}

// ── Decode failure (replaces Cow<'static, str>) ──────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeFailure {
    // SPKI extraction
    SpkiMissingOid,
    SpkiMissingBitString,
    SpkiInvalidBitStringLength,
    // Certificate
    EmptyCertificateChain,
    EmptyRawPublicKey,
    EmptyClientCertificateChain,
    CertificateParseError,
    CertificateKeyError,
    // TLS record
    UnknownContentType(u8),
    // Handshake message framing
    HandshakeMessageTooShort,
    UnknownHandshakeType(u8),
    HandshakeMessageTruncated,
    // Extensions
    ExtensionsTooShort,
    ExtensionsTruncated,
    ExtensionTruncated,
    // ClientHello
    ClientHelloTooShort,
    ClientHelloSessionIdTruncated,
    SessionIdTooLong,
    ClientHelloCipherSuitesMalformed,
    // ServerHello
    ServerHelloTooShort,
    ServerHelloSessionIdTruncated,
    UnknownCipherSuiteInServerHello,
    NonNullCompressionInServerHello,
    // Certificate msg
    CertificateEmpty,
    CertificateContextTruncated,
    CertificateListLengthTruncated,
    CertificateListTruncated,
    CertificateEntryDataLenTruncated,
    CertificateEntryDataTruncated,
    // CertificateVerify
    CertificateVerifyTooShort,
    UnknownSignatureSchemeInCertificateVerify,
    CertificateVerifySignatureTruncated,
    SignatureTooLong,
    // Finished
    VerifyDataTooLong,
    // KeyUpdate
    KeyUpdateInvalidRequestUpdate,
    // NewSessionTicket
    NewSessionTicketTooShort,
    TicketNonceTooLong,
    // KeyShare
    KeyShareTooShort,
    UnknownKeyExchangeGroupInKeyShare,
    KeyShareDataTooLarge,
    KeyShareServerHelloTooShort,
    UnknownKeyExchangeGroupInServerHelloKeyShare,
    // SignatureAlgorithms
    SignatureAlgorithmsTooShort,
    SignatureAlgorithmsMalformed,
    TooManySignatureAlgorithms,
    // ALPN
    AlpnTooShort,
    AlpnProtocolTooLong,
    AlpnTooManyProtocols,
    // ServerCertificateType
    EmptyServerCertificateType,
    UnknownCertType(u8),
    ServerCertificateTypeListTruncated,
    TooManyServerCertificateTypes,
    // HRR
    UnknownKeyExchangeGroupInHrr,
}

impl fmt::Display for DecodeFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpkiMissingOid => f.write_str("no OID in SPKI"),
            Self::SpkiMissingBitString => f.write_str("no BIT STRING after AlgorithmIdentifier in SPKI"),
            Self::SpkiInvalidBitStringLength => f.write_str("BIT STRING length invalid"),
            Self::EmptyCertificateChain => f.write_str("empty certificate chain"),
            Self::EmptyRawPublicKey => f.write_str("empty raw public key"),
            Self::EmptyClientCertificateChain => f.write_str("empty client certificate chain"),
            Self::CertificateParseError => f.write_str("X.509 certificate parse error"),
            Self::CertificateKeyError => f.write_str("X.509 key extraction error"),
            Self::UnknownContentType(ct) => write!(f, "unknown content type {ct}"),
            Self::HandshakeMessageTooShort => f.write_str("handshake message too short"),
            Self::UnknownHandshakeType(t) => write!(f, "unknown handshake type {t}"),
            Self::HandshakeMessageTruncated => f.write_str("handshake message truncated"),
            Self::ExtensionsTooShort => f.write_str("extensions too short"),
            Self::ExtensionsTruncated => f.write_str("extensions truncated"),
            Self::ExtensionTruncated => f.write_str("extension truncated"),
            Self::ClientHelloTooShort => f.write_str("ClientHello too short"),
            Self::ClientHelloSessionIdTruncated => f.write_str("ClientHello session_id truncated"),
            Self::SessionIdTooLong => f.write_str("session_id too long"),
            Self::ClientHelloCipherSuitesMalformed => f.write_str("ClientHello cipher_suites malformed"),
            Self::ServerHelloTooShort => f.write_str("ServerHello too short"),
            Self::ServerHelloSessionIdTruncated => f.write_str("ServerHello session_id truncated"),
            Self::UnknownCipherSuiteInServerHello => f.write_str("unknown cipher suite in ServerHello"),
            Self::NonNullCompressionInServerHello => f.write_str("non-null compression in ServerHello"),
            Self::CertificateEmpty => f.write_str("empty Certificate"),
            Self::CertificateContextTruncated => f.write_str("certificate context truncated"),
            Self::CertificateListLengthTruncated => f.write_str("certificate list length truncated"),
            Self::CertificateListTruncated => f.write_str("certificate list truncated"),
            Self::CertificateEntryDataLenTruncated => f.write_str("certificate entry datalen truncated"),
            Self::CertificateEntryDataTruncated => f.write_str("certificate entry data truncated"),
            Self::CertificateVerifyTooShort => f.write_str("CertificateVerify too short"),
            Self::UnknownSignatureSchemeInCertificateVerify => f.write_str("unknown signature scheme in CertificateVerify"),
            Self::CertificateVerifySignatureTruncated => f.write_str("CertificateVerify signature truncated"),
            Self::SignatureTooLong => f.write_str("signature too long"),
            Self::VerifyDataTooLong => f.write_str("verify_data too long"),
            Self::KeyUpdateInvalidRequestUpdate => f.write_str("KeyUpdate: invalid request_update"),
            Self::NewSessionTicketTooShort => f.write_str("NewSessionTicket too short"),
            Self::TicketNonceTooLong => f.write_str("ticket_nonce too long"),
            Self::KeyShareTooShort => f.write_str("key_share too short"),
            Self::UnknownKeyExchangeGroupInKeyShare => f.write_str("unknown kx group in key_share"),
            Self::KeyShareDataTooLarge => f.write_str("key share data too large"),
            Self::KeyShareServerHelloTooShort => f.write_str("key_share (ServerHello) too short"),
            Self::UnknownKeyExchangeGroupInServerHelloKeyShare => {
                f.write_str("unknown kx group in ServerHello key_share")
            }
            Self::SignatureAlgorithmsTooShort => f.write_str("signature_algorithms extension too short"),
            Self::SignatureAlgorithmsMalformed => f.write_str("signature_algorithms extension malformed"),
            Self::TooManySignatureAlgorithms => f.write_str("too many signature_algorithms. limit: 24"),
            Self::AlpnTooShort => f.write_str("ALPN extension too short"),
            Self::AlpnProtocolTooLong => f.write_str("ALPN protocol is too long (max: 32 bytes)"),
            Self::AlpnTooManyProtocols => f.write_str("ALPN: too many protocols (max 8)"),
            Self::EmptyServerCertificateType => f.write_str("empty server_certificate_type extension"),
            Self::UnknownCertType(t) => write!(f, "unknown cert type {t}"),
            Self::ServerCertificateTypeListTruncated => f.write_str("server_certificate_type list truncated"),
            Self::TooManyServerCertificateTypes => f.write_str("too many server_certificate_type. limit: 4"),
            Self::UnknownKeyExchangeGroupInHrr => f.write_str("unknown KX group in HRR"),
        }
    }
}

// ── Internal failure (replaces Cow<'static, str>) ────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalFailure {
    HandshakeNotComplete,
    TooManyKeyExchangeGroups,
    TooManyExtensions,
    ServerNameTooLong,
    AlpnProtocolTooLong,
    WriteKeyNotSet,
    ReadKeyNotSet,
    NoCertChain,
    PublicKeyTooLarge,
    NoKeys,
    ConnectionInFailedState,
    NoKxPairForNegotiatedGroup,
    NoClientCertChain,
    WebpkiValidatorRequired,
}

impl fmt::Display for InternalFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HandshakeNotComplete => f.write_str("handshake not complete"),
            Self::TooManyKeyExchangeGroups => f.write_str("too many key exchange groups. limit: 6"),
            Self::TooManyExtensions => f.write_str("too many extensions"),
            Self::ServerNameTooLong => f.write_str("server name too long"),
            Self::AlpnProtocolTooLong => f.write_str("ALPN protocol is too long. max: 32 bytes"),
            Self::WriteKeyNotSet => f.write_str("write key not set"),
            Self::ReadKeyNotSet => f.write_str("read key not set"),
            Self::NoCertChain => f.write_str("no cert chain"),
            Self::PublicKeyTooLarge => f.write_str("public key too large"),
            Self::NoKeys => f.write_str("no keys"),
            Self::ConnectionInFailedState => f.write_str("connection in failed state"),
            Self::NoKxPairForNegotiatedGroup => f.write_str("no kx_pair for negotiated group"),
            Self::NoClientCertChain => f.write_str("no client cert chain"),
            Self::WebpkiValidatorRequired => {
                f.write_str("X.509 certificate support requires the 'webpki-validator' feature")
            }
        }
    }
}

// ── Certificate parse failure ────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateParseFailure {
    IssuerSpkiParse { chain_index: usize },
    IssuerSubjectDnParse { chain_index: usize },
    CertIssuerDnParse { chain_index: usize },
    CertIssuerDn,
    Validity,
    SanParse,
    TbsParse,
    SignatureParse,
    IssuerPublicKeyParse,
    CertSpkiForKeyMatching,
    SignatureAlgorithm,
    SpkiAlgorithm,
    InvalidCertDer,
    InvalidTbs,
    InvalidSignatureAlgorithm,
    InvalidSignatureValue,
    EmptySignature,
}

impl fmt::Display for CertificateParseFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IssuerSpkiParse { chain_index } => {
                write!(f, "issuer SPKI parse error at chain_index={chain_index}")
            }
            Self::IssuerSubjectDnParse { chain_index } => {
                write!(f, "issuer subject DN parse error at chain_index={chain_index}")
            }
            Self::CertIssuerDnParse { chain_index } => {
                write!(f, "cert issuer DN parse error at chain_index={chain_index}")
            }
            Self::CertIssuerDn => f.write_str("cert issuer DN parse error"),
            Self::Validity => f.write_str("certificate validity error"),
            Self::SanParse => f.write_str("SAN extension parse error"),
            Self::TbsParse => f.write_str("TBS certificate parse error"),
            Self::SignatureParse => f.write_str("signature parse error"),
            Self::IssuerPublicKeyParse => f.write_str("issuer public key parse error"),
            Self::CertSpkiForKeyMatching => f.write_str("cert SPKI parse error for key matching"),
            Self::SignatureAlgorithm => f.write_str("signature algorithm parse error"),
            Self::SpkiAlgorithm => f.write_str("SPKI algorithm parse error"),
            Self::InvalidCertDer => f.write_str("invalid cert DER"),
            Self::InvalidTbs => f.write_str("invalid TBS"),
            Self::InvalidSignatureAlgorithm => f.write_str("invalid signatureAlgorithm"),
            Self::InvalidSignatureValue => f.write_str("invalid signatureValue"),
            Self::EmptySignature => f.write_str("empty signature"),
        }
    }
}

// ── Certificate chain failure ────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateChainFailure {
    IssuerSubjectDnMismatch,
    SignatureVerificationFailed { chain_index: usize },
    CertificateNotYetValid,
    CertificateExpired,
    IntermediateNotCa,
    EndEntityMustNotBeCa,
    EkuDoesNotIncludeServerAuth,
    UnsupportedSignatureAlgorithm,
    NoRootFoundBySpkiMatching,
    NoTrustedRootFound { searched_roots: usize },
}

impl fmt::Display for CertificateChainFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IssuerSubjectDnMismatch => f.write_str("issuer/subject DN mismatch"),
            Self::SignatureVerificationFailed { chain_index } => {
                write!(f, "signature verification failed at chain_index={chain_index}")
            }
            Self::CertificateNotYetValid => f.write_str("certificate not yet valid"),
            Self::CertificateExpired => f.write_str("certificate expired"),
            Self::IntermediateNotCa => f.write_str("intermediate certificate is not a CA"),
            Self::EndEntityMustNotBeCa => f.write_str("end-entity certificate must not be a CA"),
            Self::EkuDoesNotIncludeServerAuth => f.write_str("EKU does not include serverAuth"),
            Self::UnsupportedSignatureAlgorithm => f.write_str("unsupported signature algorithm"),
            Self::NoRootFoundBySpkiMatching => f.write_str("no root found by SPKI matching"),
            Self::NoTrustedRootFound { searched_roots } => {
                write!(f, "no trusted root found (searched {searched_roots} roots)")
            }
        }
    }
}

// ── Invalid key parse failure ────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidKeyParseFailure {
    InvalidP256PrivateKey,
    InvalidP384PrivateKey,
    InvalidEd25519PublicKey,
    InvalidP256PublicKey,
    InvalidP384PublicKey,
    X25519MlKem768CiphertextTooShort,
}

impl fmt::Display for InvalidKeyParseFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidP256PrivateKey => f.write_str("invalid P-256 private key"),
            Self::InvalidP384PrivateKey => f.write_str("invalid P-384 private key"),
            Self::InvalidEd25519PublicKey => f.write_str("invalid Ed25519 public key"),
            Self::InvalidP256PublicKey => f.write_str("invalid P-256 public key"),
            Self::InvalidP384PublicKey => f.write_str("invalid P-384 public key"),
            Self::X25519MlKem768CiphertextTooShort => {
                f.write_str("X25519MLKEM768 peer key: ML-KEM ciphertext too short")
            }
        }
    }
}

// ── ECDSA DER error ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcdsaDerError {
    TooShort,
    ExpectedSequence,
    Truncated,
    ExpectedIntegerR,
    RTruncated,
    SHeaderTruncated,
    ExpectedIntegerS,
    STruncated,
}

impl fmt::Display for EcdsaDerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort => f.write_str("too short"),
            Self::ExpectedSequence => f.write_str("expected SEQUENCE"),
            Self::Truncated => f.write_str("truncated"),
            Self::ExpectedIntegerR => f.write_str("expected INTEGER r"),
            Self::RTruncated => f.write_str("r truncated"),
            Self::SHeaderTruncated => f.write_str("s header truncated"),
            Self::ExpectedIntegerS => f.write_str("expected INTEGER s"),
            Self::STruncated => f.write_str("s truncated"),
        }
    }
}

// ── Typed error sub-kinds ────────────────────────────────────────────────

/// Specific reasons a handshake may fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeFailure {
    FinishedVerificationFailed,
    Tls13NotOffered,
    NoKeyShare,
    PeerAlert { level: u8, description: u8 },
    ConnectionInFailedState,
    HrrRequestedGroupNotSupported(KeyExchangeGroup),
    CertificateProviderSchemeNotOffered(SignatureScheme),
}

impl fmt::Display for HandshakeFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FinishedVerificationFailed => f.write_str("finished verification failed"),
            Self::Tls13NotOffered => f.write_str("TLS 1.3 not offered"),
            Self::NoKeyShare => f.write_str("no key_share extension"),
            Self::PeerAlert { level, description } => {
                write!(f, "peer alert: level={level} desc={description}")
            }
            Self::ConnectionInFailedState => f.write_str("connection in failed state"),
            Self::HrrRequestedGroupNotSupported(g) => {
                write!(f, "HRR requested group {g:?} which is not supported")
            }
            Self::CertificateProviderSchemeNotOffered(s) => {
                write!(f, "CertificateProvider selected scheme {s:?} which was not offered by client")
            }
        }
    }
}

/// Specific reasons certificate validation may fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateValidationFailure {
    EmptyChain,
    ServerNameRequired,
    ParseError(CertificateParseFailure),
    SubjectNameMismatch,
    ChainValidation(CertificateChainFailure),
    RawPublicKeyRequiresCustomValidator,
    SignatureVerificationFailed,
}

impl fmt::Display for CertificateValidationFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyChain => f.write_str("empty certificate chain"),
            Self::ServerNameRequired => f.write_str("server name (SNI) required for X.509 validation"),
            Self::ParseError(e) => write!(f, "parse error: {e}"),
            Self::SubjectNameMismatch => f.write_str("subject name mismatch"),
            Self::ChainValidation(e) => write!(f, "chain validation failed: {e}"),
            Self::RawPublicKeyRequiresCustomValidator => {
                f.write_str("raw public key validation requires custom validator")
            }
            Self::SignatureVerificationFailed => f.write_str("signature verification failed"),
        }
    }
}

/// Specific crypto-related errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoFailure {
    UnsupportedCipherSuite(CipherSuite),
    UnsupportedSignatureScheme(SignatureScheme),
    RsaVerificationFailed(SignatureScheme),
    SigningFailed,
}

impl fmt::Display for CryptoFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCipherSuite(c) => write!(f, "unsupported cipher suite: {c:?}"),
            Self::UnsupportedSignatureScheme(s) => write!(f, "unsupported signature scheme: {s:?}"),
            Self::RsaVerificationFailed(s) => write!(f, "RSA verification failed: {s:?}"),
            Self::SigningFailed => f.write_str("signing failed"),
        }
    }
}

/// Specific key-related errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidKeyFailure {
    WrongLength { algorithm: &'static str, expected: usize },
    ParseError(InvalidKeyParseFailure),
    DerError(EcdsaDerError),
    X25519MlKem768PeerKeyLengthMismatch { side: PeerSide, expected: usize, got: usize },
    X25519MlKem768NoSecretKey,
    MlKemDecapsulationFailed,
}

impl fmt::Display for InvalidKeyFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongLength { algorithm, expected } => write!(f, "{algorithm} expects a {expected}-byte key"),
            Self::ParseError(e) => write!(f, "{e}"),
            Self::DerError(e) => write!(f, "DER error: {e}"),
            Self::X25519MlKem768PeerKeyLengthMismatch { side, expected, got } => {
                write!(f, "X25519MLKEM768 peer ({side}) key must be {expected} bytes, got {got}")
            }
            Self::X25519MlKem768NoSecretKey => {
                f.write_str("X25519MLKEM768: no ML-KEM secret key available for decapsulation")
            }
            Self::MlKemDecapsulationFailed => f.write_str("ML-KEM decapsulation failed"),
        }
    }
}

/// Errors returned by the TLS library.
#[derive(Debug)]
pub enum Error {
    UnexpectedMessage { expected: &'static str, got: &'static str },
    HandshakeFailed(HandshakeFailure),
    DecryptFailed,
    InvalidKey(InvalidKeyFailure),
    CertificateValidationFailed(CertificateValidationFailure),
    NoCipherSuitesInCommon,
    NoKeyExchangeGroupInCommon,
    DecodeError(DecodeFailure),
    InternalError(InternalFailure),
    Io(IoError),
    ConnectionClosed,
    CryptoError(CryptoFailure),
    RecordOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedMessage { expected, got } => {
                write!(f, "unexpected message: expected {expected}, got {got}")
            }
            Self::HandshakeFailed(msg) => write!(f, "handshake failed: {msg}"),
            Self::DecryptFailed => f.write_str("decryption failed"),
            Self::InvalidKey(msg) => write!(f, "invalid key: {msg}"),
            Self::CertificateValidationFailed(msg) => write!(f, "certificate validation failed: {msg}"),
            Self::NoCipherSuitesInCommon => f.write_str("no cipher suites in common"),
            Self::NoKeyExchangeGroupInCommon => f.write_str("no key exchange group in common"),
            Self::DecodeError(msg) => write!(f, "decode error: {msg}"),
            Self::InternalError(msg) => write!(f, "internal error: {msg}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::ConnectionClosed => f.write_str("connection closed"),
            Self::CryptoError(msg) => write!(f, "crypto error: {msg}"),
            Self::RecordOverflow => f.write_str("record overflow"),
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
