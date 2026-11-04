//! Errors returned by COSE operations.

use core::fmt;

use crate::{Algorithm, Curve, KeyOp, KeyType};

/// The error type used throughout this crate.
///
/// The type is available without an allocator: it never stores a `String` or a
/// `Vec`.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// The input is not well-formed CBOR.
    Decode(cbor::Error),
    /// A value could not be encoded into the supplied buffer or output type.
    Encode,
    /// The top-level COSE structure has the wrong shape (wrong array length,
    /// wrong item type, or a missing required field).
    InvalidStructure,
    /// Bytes follow the complete CBOR item.
    TrailingBytes,
    /// A CBOR tag is present but is not the tag expected for this structure.
    WrongTag {
        /// The expected tag.
        expected: u64,
        /// The tag that was found.
        actual: u64,
    },
    /// A COSE map uses an indefinite length.
    IndefiniteLength,
    /// The same label appears more than once in a header or key map.
    DuplicateLabel,
    /// A header parameter has a value of the wrong type, or a header map is
    /// malformed.
    InvalidHeader,
    /// The protected header lists a critical label that the application did not
    /// declare as understood.
    UnknownCriticalHeader,
    /// The `crit` header parameter is present in the unprotected bucket.
    CriticalHeaderNotProtected,
    /// The message does not carry an `alg` header parameter.
    MissingAlgorithm,
    /// The message algorithm and the key algorithm are not compatible.
    AlgorithmMismatch {
        /// The algorithm from the message header.
        header: Algorithm,
        /// The algorithm reported by the key.
        key: Algorithm,
    },
    /// The algorithm is not implemented by this crate.
    UnsupportedAlgorithm(Algorithm),
    /// The algorithm is not in the caller-supplied allow-list.
    AlgorithmNotAllowed(Algorithm),
    /// The caller requires `alg` in the protected bucket, but it is absent.
    AlgorithmNotProtected,
    /// The COSE key type is not implemented.
    UnsupportedKeyType(KeyType),
    /// The COSE key curve is not implemented.
    UnsupportedCurve(Curve),
    /// The requested operation is not permitted by the key or its `key_ops`.
    UnsupportedKeyOperation(KeyOp),
    /// The key material is missing, malformed, or inconsistent with its type or
    /// algorithm.
    InvalidKey,
    /// A signature did not verify.
    InvalidSignature,
    /// A MAC tag did not verify.
    InvalidTag,
    /// Signing or MACing was requested without a payload.
    MissingPayload,
    /// The message carries a detached payload, but no payload was supplied.
    DetachedPayloadMissing,
    /// A detached-payload verification method was called for a message that
    /// carries an attached payload.
    PayloadNotDetached,
    /// A header, key, or signature collection exceeded the supported maximum.
    TooManyEntries,
    /// Verification stopped because the configured attempt budget was exhausted.
    ///
    /// This protects applications from messages that force an expensive scan
    /// over a large key set or over many signatures. The limit can be raised
    /// with [`VerifyOptions`](crate::VerifyOptions).
    TooManyAttempts {
        /// The maximum number of cryptographic verification attempts.
        limit: usize,
    },
    /// The supplied output or scratch buffer is too small.
    BufferTooSmall {
        /// The number of bytes required.
        required: usize,
        /// The number of bytes available.
        available: usize,
    },
    /// An elliptic-curve operation failed.
    EllipticCurve(crypto::EllipticCurveError),
    /// An ML-DSA operation failed.
    MlDsa(crypto::mldsa::MlDsaError),
    /// An RSA operation failed.
    Rsa(crypto::RsaError),
    /// The operating system's random number generator failed.
    Random(crypto::RandomError),
    /// Something went wrong without further detail.
    Unspecified(&'static str),
}

/// A `Result` with this crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Decode(err) => write!(f, "invalid CBOR: {err}"),
            Error::Encode => f.write_str("CBOR encoding failed"),
            Error::InvalidStructure => f.write_str("invalid COSE structure"),
            Error::TrailingBytes => f.write_str("trailing bytes after COSE object"),
            Error::WrongTag {
                expected,
                actual,
            } => write!(f, "wrong CBOR tag: expected {expected}, found {actual}"),
            Error::IndefiniteLength => f.write_str("indefinite-length CBOR is not accepted here"),
            Error::DuplicateLabel => f.write_str("duplicate label in COSE map"),
            Error::InvalidHeader => f.write_str("invalid COSE header"),
            Error::UnknownCriticalHeader => f.write_str("unknown critical header parameter"),
            Error::CriticalHeaderNotProtected => f.write_str("crit must be in the protected header"),
            Error::MissingAlgorithm => f.write_str("the COSE message has no alg header parameter"),
            Error::AlgorithmMismatch {
                header,
                key,
            } => write!(f, "message algorithm {header} does not match key algorithm {key}"),
            Error::UnsupportedAlgorithm(alg) => write!(f, "unsupported algorithm: {alg}"),
            Error::AlgorithmNotAllowed(alg) => write!(f, "algorithm not allowed: {alg}"),
            Error::AlgorithmNotProtected => f.write_str("alg must be in the protected header"),
            Error::UnsupportedKeyType(kty) => write!(f, "unsupported key type: {kty}"),
            Error::UnsupportedCurve(curve) => write!(f, "unsupported curve: {curve}"),
            Error::UnsupportedKeyOperation(op) => write!(f, "key operation not allowed: {op}"),
            Error::InvalidKey => f.write_str("key is not valid"),
            Error::InvalidSignature => f.write_str("signature is not valid"),
            Error::InvalidTag => f.write_str("MAC tag is not valid"),
            Error::MissingPayload => f.write_str("payload is required"),
            Error::DetachedPayloadMissing => f.write_str("detached payload is required"),
            Error::PayloadNotDetached => f.write_str("message has an attached payload"),
            Error::TooManyEntries => f.write_str("too many entries"),
            Error::TooManyAttempts {
                limit,
            } => write!(f, "verification attempt limit reached ({limit})"),
            Error::BufferTooSmall {
                required,
                available,
            } => write!(f, "buffer is too small: need {required} bytes, have {available}"),
            Error::EllipticCurve(err) => write!(f, "{err}"),
            Error::MlDsa(err) => write!(f, "{err}"),
            Error::Rsa(err) => write!(f, "{err}"),
            Error::Random(err) => write!(f, "{err}"),
            Error::Unspecified(err) => f.write_str(err),
        }
    }
}

impl core::error::Error for Error {}

impl From<cbor::Error> for Error {
    fn from(err: cbor::Error) -> Self {
        Error::Decode(err)
    }
}

impl From<crypto::EllipticCurveError> for Error {
    fn from(err: crypto::EllipticCurveError) -> Self {
        Error::EllipticCurve(err)
    }
}

impl From<crypto::mldsa::MlDsaError> for Error {
    fn from(err: crypto::mldsa::MlDsaError) -> Self {
        Error::MlDsa(err)
    }
}

impl From<crypto::RsaError> for Error {
    fn from(err: crypto::RsaError) -> Self {
        Error::Rsa(err)
    }
}

impl From<crypto::RandomError> for Error {
    fn from(err: crypto::RandomError) -> Self {
        Error::Random(err)
    }
}
