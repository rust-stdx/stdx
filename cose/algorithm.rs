//! COSE identifiers: algorithms, labels, key types, curves, and operations.

use core::fmt;

use crate::Error;

/// A COSE algorithm identifier (RFC 9053, RFC 9864, RFC 9964).
///
/// Only algorithms implemented by this crate have named variants. Any other
/// numeric identifier is preserved as [`Algorithm::Unknown`], which lets COSE
/// keys and private-use messages round-trip without understanding them.
///
/// This crate implements signature and MAC algorithms only. Encryption and key
/// distribution identifiers are preserved as `Unknown`.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Algorithm {
    /// ECDSA using P-256 and SHA-256 (fully specified, recommended).
    ESP256,
    /// ECDSA using P-384 and SHA-384 (fully specified, recommended).
    ESP384,
    /// ECDSA using P-521 and SHA-512 (fully specified, recommended).
    ESP512,
    /// ECDSA using P-256 and SHA-256 (deprecated polymorphic identifier).
    ES256,
    /// ECDSA using P-384 and SHA-384 (deprecated polymorphic identifier).
    ES384,
    /// ECDSA using P-521 and SHA-512 (deprecated polymorphic identifier).
    ES512,
    /// EdDSA using the Ed25519 parameter set (fully specified).
    Ed25519,
    /// EdDSA (deprecated polymorphic identifier).
    EdDSA,
    /// ML-DSA-44.
    MlDsa44,
    /// ML-DSA-65.
    MlDsa65,
    /// ML-DSA-87.
    MlDsa87,
    /// RSASSA-PKCS1-v1_5 with SHA-256.
    RS256,
    /// RSASSA-PKCS1-v1_5 with SHA-384.
    RS384,
    /// RSASSA-PKCS1-v1_5 with SHA-512.
    RS512,
    /// RSASSA-PSS with SHA-256.
    PS256,
    /// RSASSA-PSS with SHA-384.
    PS384,
    /// RSASSA-PSS with SHA-512.
    PS512,
    /// HMAC with SHA-256 truncated to 64 bits.
    Hmac256_64,
    /// HMAC with SHA-256.
    Hmac256_256,
    /// HMAC with SHA-384.
    Hmac384_384,
    /// HMAC with SHA-512.
    Hmac512_512,
    /// Direct key distribution. Only valid in a `COSE_Mac` recipient.
    Direct,
    /// An unregistered or unsupported numeric algorithm identifier.
    Unknown(i64),
}

impl Algorithm {
    /// Returns the IANA numeric value.
    #[inline]
    pub const fn as_i64(self) -> i64 {
        match self {
            Algorithm::ESP256 => -9,
            Algorithm::ESP384 => -51,
            Algorithm::ESP512 => -52,
            Algorithm::ES256 => -7,
            Algorithm::ES384 => -35,
            Algorithm::ES512 => -36,
            Algorithm::Ed25519 => -19,
            Algorithm::EdDSA => -8,
            Algorithm::MlDsa44 => -48,
            Algorithm::MlDsa65 => -49,
            Algorithm::MlDsa87 => -50,
            Algorithm::RS256 => -257,
            Algorithm::RS384 => -258,
            Algorithm::RS512 => -259,
            Algorithm::PS256 => -37,
            Algorithm::PS384 => -38,
            Algorithm::PS512 => -39,
            Algorithm::Hmac256_64 => 4,
            Algorithm::Hmac256_256 => 5,
            Algorithm::Hmac384_384 => 6,
            Algorithm::Hmac512_512 => 7,
            Algorithm::Direct => -6,
            Algorithm::Unknown(value) => value,
        }
    }

    /// Maps an IANA numeric value to a named variant when this crate knows it.
    #[inline]
    pub const fn from_i64(value: i64) -> Self {
        match value {
            -9 => Algorithm::ESP256,
            -51 => Algorithm::ESP384,
            -52 => Algorithm::ESP512,
            -7 => Algorithm::ES256,
            -35 => Algorithm::ES384,
            -36 => Algorithm::ES512,
            -19 => Algorithm::Ed25519,
            -8 => Algorithm::EdDSA,
            -48 => Algorithm::MlDsa44,
            -49 => Algorithm::MlDsa65,
            -50 => Algorithm::MlDsa87,
            -257 => Algorithm::RS256,
            -258 => Algorithm::RS384,
            -259 => Algorithm::RS512,
            -37 => Algorithm::PS256,
            -38 => Algorithm::PS384,
            -39 => Algorithm::PS512,
            4 => Algorithm::Hmac256_64,
            5 => Algorithm::Hmac256_256,
            6 => Algorithm::Hmac384_384,
            7 => Algorithm::Hmac512_512,
            -6 => Algorithm::Direct,
            other => Algorithm::Unknown(other),
        }
    }

    /// Returns the registered name, or `None` for an unknown numeric value.
    pub const fn name(self) -> Option<&'static str> {
        match self {
            Algorithm::ESP256 => Some("ESP256"),
            Algorithm::ESP384 => Some("ESP384"),
            Algorithm::ESP512 => Some("ESP512"),
            Algorithm::ES256 => Some("ES256"),
            Algorithm::ES384 => Some("ES384"),
            Algorithm::ES512 => Some("ES512"),
            Algorithm::Ed25519 => Some("Ed25519"),
            Algorithm::EdDSA => Some("EdDSA"),
            Algorithm::MlDsa44 => Some("ML-DSA-44"),
            Algorithm::MlDsa65 => Some("ML-DSA-65"),
            Algorithm::MlDsa87 => Some("ML-DSA-87"),
            Algorithm::RS256 => Some("RS256"),
            Algorithm::RS384 => Some("RS384"),
            Algorithm::RS512 => Some("RS512"),
            Algorithm::PS256 => Some("PS256"),
            Algorithm::PS384 => Some("PS384"),
            Algorithm::PS512 => Some("PS512"),
            Algorithm::Hmac256_64 => Some("HMAC 256/64"),
            Algorithm::Hmac256_256 => Some("HMAC 256/256"),
            Algorithm::Hmac384_384 => Some("HMAC 384/384"),
            Algorithm::Hmac512_512 => Some("HMAC 512/512"),
            Algorithm::Direct => Some("direct"),
            Algorithm::Unknown(_) => None,
        }
    }

    /// Returns true if this is a signature algorithm.
    #[inline]
    pub const fn is_signature(self) -> bool {
        matches!(
            self,
            Algorithm::ESP256
                | Algorithm::ESP384
                | Algorithm::ESP512
                | Algorithm::ES256
                | Algorithm::ES384
                | Algorithm::ES512
                | Algorithm::Ed25519
                | Algorithm::EdDSA
                | Algorithm::MlDsa44
                | Algorithm::MlDsa65
                | Algorithm::MlDsa87
                | Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512
        )
    }

    /// Returns true if this is a MAC algorithm.
    #[inline]
    pub const fn is_mac(self) -> bool {
        matches!(
            self,
            Algorithm::Hmac256_64 | Algorithm::Hmac256_256 | Algorithm::Hmac384_384 | Algorithm::Hmac512_512
        )
    }

    /// Returns true if this is a key distribution algorithm.
    #[inline]
    pub const fn is_key_distribution(self) -> bool {
        matches!(self, Algorithm::Direct)
    }

    /// Returns true if this is a registered algorithm known to this crate.
    ///
    /// This includes [`Algorithm::Direct`], which is a key-distribution
    /// identifier rather than a signing or MAC algorithm. Use
    /// [`Algorithm::is_signature`] or [`Algorithm::is_mac`] to decide whether
    /// the algorithm can be used for message verification.
    #[inline]
    pub const fn is_supported(self) -> bool {
        !matches!(self, Algorithm::Unknown(_))
    }

    /// Returns the fixed signature size, or `None` for RSA (whose signature size
    /// depends on the modulus) and for non-signature algorithms.
    pub const fn signature_len(self) -> Option<usize> {
        match self {
            Algorithm::ESP256 | Algorithm::ES256 => Some(64),
            Algorithm::ESP384 | Algorithm::ES384 => Some(96),
            Algorithm::ESP512 | Algorithm::ES512 => Some(132),
            Algorithm::Ed25519 | Algorithm::EdDSA => Some(64),
            Algorithm::MlDsa44 => Some(crypto::mldsa::ML_DSA_44_SIGNATURE_SIZE),
            Algorithm::MlDsa65 => Some(crypto::mldsa::ML_DSA_65_SIGNATURE_SIZE),
            Algorithm::MlDsa87 => Some(crypto::mldsa::ML_DSA_87_SIGNATURE_SIZE),
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => None,
            _ => None,
        }
    }

    /// Returns the MAC tag size, or `None` for non-MAC algorithms.
    pub const fn tag_len(self) -> Option<usize> {
        match self {
            Algorithm::Hmac256_64 => Some(8),
            Algorithm::Hmac256_256 => Some(32),
            Algorithm::Hmac384_384 => Some(48),
            Algorithm::Hmac512_512 => Some(64),
            _ => None,
        }
    }

    /// Returns true when the two algorithms denote the same cryptographic
    /// operation.
    ///
    /// The legacy polymorphic identifiers are compatible with their
    /// fully-specified replacements: `ES256`/`ESP256`, `ES384`/`ESP384`,
    /// `ES512`/`ESP512`, and `EdDSA`/`Ed25519`. All other algorithms require an
    /// exact match.
    pub fn is_compatible(self, other: Self) -> bool {
        if self == other {
            return true;
        }

        matches!(
            (self, other),
            (Algorithm::ES256, Algorithm::ESP256)
                | (Algorithm::ESP256, Algorithm::ES256)
                | (Algorithm::ES384, Algorithm::ESP384)
                | (Algorithm::ESP384, Algorithm::ES384)
                | (Algorithm::ES512, Algorithm::ESP512)
                | (Algorithm::ESP512, Algorithm::ES512)
                | (Algorithm::EdDSA, Algorithm::Ed25519)
                | (Algorithm::Ed25519, Algorithm::EdDSA)
        )
    }

    /// Returns true when a key of `kty` and (for EC2/OKP) `curve` can be used
    /// with this algorithm.
    pub fn is_compatible_key(self, kty: KeyType, curve: Option<Curve>) -> bool {
        match self {
            Algorithm::ESP256 | Algorithm::ES256 => kty == KeyType::Ec2 && curve == Some(Curve::P256),
            Algorithm::ESP384 | Algorithm::ES384 => kty == KeyType::Ec2 && curve == Some(Curve::P384),
            Algorithm::ESP512 | Algorithm::ES512 => kty == KeyType::Ec2 && curve == Some(Curve::P521),
            Algorithm::Ed25519 | Algorithm::EdDSA => kty == KeyType::Okp && curve == Some(Curve::Ed25519),
            Algorithm::MlDsa44 | Algorithm::MlDsa65 | Algorithm::MlDsa87 => kty == KeyType::Akp,
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => kty == KeyType::Rsa,
            Algorithm::Hmac256_64 | Algorithm::Hmac256_256 | Algorithm::Hmac384_384 | Algorithm::Hmac512_512 => {
                kty == KeyType::Symmetric
            }
            Algorithm::Direct | Algorithm::Unknown(_) => false,
        }
    }
}

impl From<i64> for Algorithm {
    #[inline]
    fn from(value: i64) -> Self {
        Algorithm::from_i64(value)
    }
}

impl From<Algorithm> for i64 {
    #[inline]
    fn from(value: Algorithm) -> Self {
        value.as_i64()
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "{}", self.as_i64()),
        }
    }
}

impl core::str::FromStr for Algorithm {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let algorithm = match value {
            "ESP256" => Algorithm::ESP256,
            "ESP384" => Algorithm::ESP384,
            "ESP512" => Algorithm::ESP512,
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            "ES512" => Algorithm::ES512,
            "Ed25519" => Algorithm::Ed25519,
            "EdDSA" => Algorithm::EdDSA,
            "ML-DSA-44" | "MLDSA44" | "ML-DSA44" => Algorithm::MlDsa44,
            "ML-DSA-65" | "MLDSA65" | "ML-DSA65" => Algorithm::MlDsa65,
            "ML-DSA-87" | "MLDSA87" | "ML-DSA87" => Algorithm::MlDsa87,
            "RS256" => Algorithm::RS256,
            "RS384" => Algorithm::RS384,
            "RS512" => Algorithm::RS512,
            "PS256" => Algorithm::PS256,
            "PS384" => Algorithm::PS384,
            "PS512" => Algorithm::PS512,
            "HMAC 256/64" | "HS256/64" | "HS256_64" | "HMAC256_64" => Algorithm::Hmac256_64,
            "HMAC 256/256" | "HS256" | "HMAC256_256" => Algorithm::Hmac256_256,
            "HMAC 384/384" | "HS384" | "HMAC384_384" => Algorithm::Hmac384_384,
            "HMAC 512/512" | "HS512" | "HMAC512_512" => Algorithm::Hmac512_512,
            "direct" | "Direct" => Algorithm::Direct,
            other => {
                let value = other
                    .parse::<i64>()
                    .map_err(|_| Error::Unspecified("unknown COSE algorithm name"))?;
                Algorithm::from_i64(value)
            }
        };
        Ok(algorithm)
    }
}

/// A COSE map label: a signed integer or a text string.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Label<'a> {
    /// An integer label.
    Int(i64),
    /// A text label.
    Text(&'a str),
}

impl<'a> From<i64> for Label<'a> {
    #[inline]
    fn from(value: i64) -> Self {
        Label::Int(value)
    }
}

impl<'a> From<&'a str> for Label<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Label::Text(value)
    }
}

impl fmt::Display for Label<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Label::Int(value) => write!(f, "{value}"),
            Label::Text(value) => f.write_str(value),
        }
    }
}

/// The value of a COSE `content type` header parameter.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContentType<'a> {
    /// A media type such as `application/json`.
    Text(&'a str),
    /// A CoAP Content-Format number.
    Uint(u64),
}

impl<'a> From<&'a str> for ContentType<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        ContentType::Text(value)
    }
}

impl From<u64> for ContentType<'_> {
    #[inline]
    fn from(value: u64) -> Self {
        ContentType::Uint(value)
    }
}

impl fmt::Display for ContentType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::Text(value) => f.write_str(value),
            ContentType::Uint(value) => write!(f, "{value}"),
        }
    }
}

/// A COSE key type (`kty`).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum KeyType {
    /// Octet Key Pair.
    Okp,
    /// Two-coordinate elliptic-curve key.
    Ec2,
    /// RSA key.
    Rsa,
    /// Symmetric key.
    Symmetric,
    /// Algorithm Key Pair (RFC 9964, used by ML-DSA).
    Akp,
    /// An unregistered key type.
    Unknown(i64),
}

impl KeyType {
    /// Returns the IANA numeric value.
    #[inline]
    pub const fn as_i64(self) -> i64 {
        match self {
            KeyType::Okp => 1,
            KeyType::Ec2 => 2,
            KeyType::Rsa => 3,
            KeyType::Symmetric => 4,
            KeyType::Akp => 7,
            KeyType::Unknown(value) => value,
        }
    }

    /// Maps an IANA numeric value to a variant.
    #[inline]
    pub const fn from_i64(value: i64) -> Self {
        match value {
            1 => KeyType::Okp,
            2 => KeyType::Ec2,
            3 => KeyType::Rsa,
            4 => KeyType::Symmetric,
            7 => KeyType::Akp,
            other => KeyType::Unknown(other),
        }
    }

    /// Returns the registered name, or `None`.
    pub const fn name(self) -> Option<&'static str> {
        match self {
            KeyType::Okp => Some("OKP"),
            KeyType::Ec2 => Some("EC2"),
            KeyType::Rsa => Some("RSA"),
            KeyType::Symmetric => Some("Symmetric"),
            KeyType::Akp => Some("AKP"),
            KeyType::Unknown(_) => None,
        }
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "{}", self.as_i64()),
        }
    }
}

/// A COSE elliptic curve identifier (`crv`).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Curve {
    /// NIST P-256.
    P256,
    /// NIST P-384.
    P384,
    /// NIST P-521.
    P521,
    /// Ed25519.
    Ed25519,
    /// An unregistered curve.
    Unknown(i64),
}

impl Curve {
    /// Returns the IANA numeric value.
    #[inline]
    pub const fn as_i64(self) -> i64 {
        match self {
            Curve::P256 => 1,
            Curve::P384 => 2,
            Curve::P521 => 3,
            Curve::Ed25519 => 6,
            Curve::Unknown(value) => value,
        }
    }

    /// Maps an IANA numeric value to a variant.
    #[inline]
    pub const fn from_i64(value: i64) -> Self {
        match value {
            1 => Curve::P256,
            2 => Curve::P384,
            3 => Curve::P521,
            6 => Curve::Ed25519,
            other => Curve::Unknown(other),
        }
    }

    /// Returns the registered name, or `None`.
    pub const fn name(self) -> Option<&'static str> {
        match self {
            Curve::P256 => Some("P-256"),
            Curve::P384 => Some("P-384"),
            Curve::P521 => Some("P-521"),
            Curve::Ed25519 => Some("Ed25519"),
            Curve::Unknown(_) => None,
        }
    }

    /// Returns the coordinate/scalar size in bytes for the curves implemented
    /// by this crate.
    pub const fn key_size(self) -> Option<usize> {
        match self {
            Curve::P256 => Some(32),
            Curve::P384 => Some(48),
            Curve::P521 => Some(66),
            Curve::Ed25519 => Some(32),
            Curve::Unknown(_) => None,
        }
    }
}

impl fmt::Display for Curve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "{}", self.as_i64()),
        }
    }
}

/// A COSE key operation (`key_ops` value).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum KeyOp {
    /// Create signatures.
    Sign,
    /// Verify signatures.
    Verify,
    /// Create MAC tags.
    MacCreate,
    /// Verify MAC tags.
    MacVerify,
    /// An operation this crate does not model.
    Unknown(i64),
}

impl KeyOp {
    /// Returns the IANA numeric value.
    #[inline]
    pub const fn as_i64(self) -> i64 {
        match self {
            KeyOp::Sign => 1,
            KeyOp::Verify => 2,
            KeyOp::MacCreate => 9,
            KeyOp::MacVerify => 10,
            KeyOp::Unknown(value) => value,
        }
    }

    /// Maps an IANA numeric value to a variant.
    #[inline]
    pub const fn from_i64(value: i64) -> Self {
        match value {
            1 => KeyOp::Sign,
            2 => KeyOp::Verify,
            9 => KeyOp::MacCreate,
            10 => KeyOp::MacVerify,
            other => KeyOp::Unknown(other),
        }
    }

    /// Returns the registered name, or `None`.
    pub const fn name(self) -> Option<&'static str> {
        match self {
            KeyOp::Sign => Some("sign"),
            KeyOp::Verify => Some("verify"),
            KeyOp::MacCreate => Some("MAC create"),
            KeyOp::MacVerify => Some("MAC verify"),
            KeyOp::Unknown(_) => None,
        }
    }
}

impl fmt::Display for KeyOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "{}", self.as_i64()),
        }
    }
}
