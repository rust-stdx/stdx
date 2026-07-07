#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::time::Duration;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

mod jwt_crypto;
pub use jwt_crypto::*;

#[cfg(feature = "std")]
pub static SYSTEM_CLOCK: SystemClock = SystemClock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    /// The only valid value is "JWT"
    /// https://tools.ietf.org/html/rfc7519#section-5.1
    pub typ: TokenType,

    /// ttps://tools.ietf.org/html/rfc7515#section-4.1.1
    pub alg: Algorithm,

    /// Content type
    /// https://tools.ietf.org/html/rfc7519#section-5.2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cty: Option<String>,

    /// JSON Key URL
    /// https://tools.ietf.org/html/rfc7515#section-4.1.2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jku: Option<String>,

    /// JSON Web Key
    /// https://tools.ietf.org/html/rfc7515#section-4.1.3
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub jwk: Option<Jwk>,

    /// Key ID
    /// https://tools.ietf.org/html/rfc7515#section-4.1.4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,

    /// X.509 URL
    /// https://tools.ietf.org/html/rfc7515#section-4.1.5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5u: Option<String>,

    /// X.509 certificate chain.
    /// https://tools.ietf.org/html/rfc7515#section-4.1.6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5c: Option<Vec<String>>,

    /// X.509 SHA1 certificate Thumbprint
    /// https://tools.ietf.org/html/rfc7515#section-4.1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,

    /// X.509 SHA256 certificate Thumbprint
    /// https://tools.ietf.org/html/rfc7515#section-4.1.8
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "x5t#S256")]
    pub x5t_s256: Option<String>,
}

/// Registered claim names from https://www.rfc-editor.org/rfc/rfc7519#section-4.1
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct RegisteredClaims {
    /// Issuer
    /// https://www.rfc-editor.org/rfc/rfc7519#section-4.1.1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,

    /// Subject
    /// https://www.rfc-editor.org/rfc/rfc7519#section-4.1.2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,

    /// Audience
    /// https://www.rfc-editor.org/rfc/rfc7519#section-4.1.3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,

    /// Expiration Time
    /// https://www.rfc-editor.org/rfc/rfc7519#section-4.1.4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,

    /// Not Before
    /// https://www.rfc-editor.org/rfc/rfc7519#section-4.1.5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,

    /// Issued At
    /// https://www.rfc-editor.org/rfc/rfc7519#section-4.1.6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,

    /// JWT ID
    /// https://www.rfc-editor.org/rfc/rfc7519#section-4.1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TokenType {
    #[default]
    JWT,
}

/// The algorithms supported for signing / verifying JWTs
#[derive(Copy, Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    /// HMAC using SHA-256
    HS256,

    /// HMAC using SHA-384
    HS384,

    /// HMAC using SHA-512
    HS512,

    /// BLAKE3 in keyed mode
    BLAKE3,

    /// Edwards-curve Digital Signature Algorithm (EdDSA)
    EdDSA,

    /// ECDSA using P-256 and SHA-256
    ES256,

    /// ECDSA using P-384 and SHA-384
    ES384,

    /// ECDSA using P-521 and SHA-512
    ES512,

    /// ML-DSA-44
    MlDsa44,

    /// ML-DSA-65
    MlDsa65,

    /// ML-DSA-87
    MlDsa87,

    /// RSASSA-PKCS1-v1.5 with SHA-256
    RS256,

    /// RSASSA-PKCS1-v1.5 with SHA-384
    RS384,

    /// RSASSA-PKCS1-v1.5 with SHA-512
    RS512,

    /// RSASSA-PSS with SHA-256
    PS256,

    /// RSASSA-PSS with SHA-384
    PS384,

    /// RSASSA-PSS with SHA-512
    PS512,
}

impl Algorithm {
    #[inline]
    pub fn signature_size(&self) -> usize {
        match self {
            Algorithm::BLAKE3 => 32,
            Algorithm::HS256 => 32,
            Algorithm::HS512 => 64,
            Algorithm::EdDSA => 64,
            Algorithm::ES256 => 64,
            Algorithm::ES512 => 132,
            Algorithm::HS384 => todo!(),
            Algorithm::ES384 => todo!(),
            Algorithm::MlDsa44 => todo!(),
            Algorithm::MlDsa65 => todo!(),
            Algorithm::MlDsa87 => todo!(),
            Algorithm::RS256 => todo!(),
            Algorithm::RS384 => todo!(),
            Algorithm::RS512 => todo!(),
            Algorithm::PS256 => todo!(),
            Algorithm::PS384 => todo!(),
            Algorithm::PS512 => todo!(),
        }
    }
}

impl core::str::FromStr for Algorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BLAKE3" => Ok(Algorithm::BLAKE3),
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::HS384),
            "HS512" => Ok(Algorithm::HS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "ES512" => Ok(Algorithm::ES512),
            "EdDSA" => Ok(Algorithm::EdDSA),
            "ML-DSA-44" => Ok(Algorithm::MlDsa44),
            "ML-DSA-65" => Ok(Algorithm::MlDsa65),
            "ML-DSA-87" => Ok(Algorithm::MlDsa87),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" => Ok(Algorithm::RS512),
            "PS256" => Ok(Algorithm::PS256),
            "PS384" => Ok(Algorithm::PS384),
            "PS512" => Ok(Algorithm::PS512),
            _ => Err(Error::UnknownAlgorithm(s.to_string())),
        }
    }
}

impl core::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
pub enum Error {
    UnknownAlgorithm(String),
    InvalidCurve,
    InvalidTokenType(String),
    Json(serde_json::Error),
    InvalidToken,
    InvalidSignature,
    InvalidKey,
    InvalidEllipticCurve(String),
    InvalidJwk { kid: String, err: String },
    Unspecified(String),
    ClockIsMissing,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::UnknownAlgorithm(algorithm) => write!(f, "unknown algorithm: {algorithm}"),
            Error::InvalidCurve => f.write_str("invalid curve"),
            Error::InvalidTokenType(token_type) => write!(f, "invalid token type: {token_type}"),
            Error::Json(err) => write!(f, "error serializing JWT to JSON: {err}"),
            Error::InvalidToken => f.write_str("JWT is not valid"),
            Error::InvalidSignature => f.write_str("signature is not valid"),
            Error::InvalidKey => f.write_str("key is not valid"),
            Error::InvalidEllipticCurve(curve) => write!(f, "invalid elliptic curve: {curve}"),
            Error::InvalidJwk {
                kid,
                err,
            } => write!(f, "{kid} is not a valid JWK: {err}"),
            Error::Unspecified(err) => f.write_str(&err),
            Error::ClockIsMissing => f.write_str("a clock is needed for exp or nbf verification"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// A wall clock used to check tokens expiration.
///
/// Returns the current Unix timestamp (seconds since epoch). This trait
/// exists so that no_std environments can inject their own time source
/// (hardware RTC, NTP, etc.) instead of relying on `std::time::SystemTime`.
pub trait Clock: Send + Sync {
    fn now(&self) -> u64;
}

/// A [`Clock`] backed by `std::time::SystemTime`.
///
/// Available only when the `std` feature is enabled.
#[cfg(feature = "std")]
pub struct SystemClock;

#[cfg(feature = "std")]
impl Clock for SystemClock {
    #[inline]
    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

pub struct VerifyOptions<'a> {
    /// Allowed time drift for `nbf` and `exp` verification in order to account for devices with
    /// inacurate clocks.
    pub allowed_time_drift: Duration,
    pub nbf: bool,
    pub exp: bool,
    pub aud: Option<&'a [&'a str]>,
    pub iss: Option<&'a [&'a str]>,
    pub clock: Option<&'a dyn Clock>,
}

pub fn sign<C: Serialize>(key: &dyn Signer, header: &Header, claims: &C) -> Result<String, Error> {
    let header_base64 = base64::encode(serde_json::to_string(header)?.as_bytes(), base64::Alphabet::UrlNoPadding);
    let claims_base64 = base64::encode(serde_json::to_string(claims)?.as_bytes(), base64::Alphabet::UrlNoPadding);

    let mut jwt = String::with_capacity(
        header_base64.len()
            + claims_base64.len()
            + base64::encoded_length(key.algorithm().signature_size(), false)
                .expect("error getting base64 encoding length")
            + 2,
    );
    jwt.push_str(&header_base64);
    jwt.push('.');
    jwt.push_str(&claims_base64);

    let signature = key.sign(jwt.as_bytes())?;
    jwt.push('.');
    jwt.push_str(&base64::encode(signature.as_ref(), base64::Alphabet::UrlNoPadding));

    return Ok(jwt);
}

pub fn parse_header(token: &str) -> Result<Header, Error> {
    let mut parts = token.split('.');
    let header_base64 = parts.next().ok_or(Error::InvalidToken)?;
    if parts.count() != 2 {
        return Err(Error::InvalidToken);
    }

    let header_json = base64::decode(header_base64, base64::Alphabet::UrlNoPadding).map_err(|_| Error::InvalidToken)?;
    let header: Header = serde_json::from_slice(&header_json).map_err(|_| Error::InvalidToken)?;

    return Ok(header);
}

pub fn parse_and_verify<C: DeserializeOwned>(
    key: &dyn Verifier,
    header: &Header,
    token: &str,
    verify_options: &VerifyOptions,
) -> Result<C, Error> {
    if (verify_options.exp || verify_options.nbf) && verify_options.clock.is_none() {
        return Err(Error::ClockIsMissing);
    }

    let mut signature_buffer = [0u8; SIGNATURE_MAX_SIZE];
    let mut parts = token.split('.');

    let header_base64 = parts.next().ok_or(Error::InvalidToken)?;
    let claims_base64 = parts.next().ok_or(Error::InvalidToken)?;
    let signature_base64 = parts.next().ok_or(Error::InvalidToken)?;
    if parts.next().is_some() {
        return Err(Error::InvalidToken);
    }

    let signature_size = base64::decode_into(
        &mut signature_buffer,
        signature_base64.as_bytes(),
        base64::Alphabet::UrlNoPadding,
    )
    .map_err(|_| Error::InvalidSignature)?;
    let signature = Signature::try_from(&signature_buffer[..signature_size])?;

    let signed_message = &token[..header_base64.len() + 1 + claims_base64.len()].as_bytes();
    key.verify(signed_message, &signature)
        .map_err(|_| Error::InvalidSignature)?;

    if header.alg != key.algorithm() {
        return Err(Error::InvalidToken);
    }

    let claims_json =
        base64::decode(&claims_base64, base64::Alphabet::UrlNoPadding).map_err(|_| Error::InvalidToken)?;

    let claims =
        if verify_options.exp || verify_options.nbf || verify_options.aud.is_some() || verify_options.iss.is_some() {
            let claims_json_value: serde_json::Value =
                serde_json::from_slice(&claims_json).map_err(|_| Error::InvalidToken)?;

            match &claims_json_value {
                serde_json::Value::Object(claims_object) => {
                    if verify_options.exp {
                        match claims_object.get("exp") {
                            None => return Err(Error::InvalidToken),
                            Some(exp_value) => {
                                if let Some(exp) = exp_value.as_i64() {
                                    let now = verify_options.clock.unwrap().now();
                                    if (exp as u64) < (now - verify_options.allowed_time_drift.as_secs()) {
                                        return Err(Error::InvalidToken);
                                    }
                                } else {
                                    return Err(Error::InvalidToken);
                                }
                            }
                        }
                    }

                    if verify_options.nbf {
                        match claims_object.get("nbf") {
                            None => return Err(Error::InvalidToken),
                            Some(nbf_value) => {
                                if let Some(nbf) = nbf_value.as_i64() {
                                    let now = verify_options.clock.unwrap().now();
                                    if (nbf as u64) > (now + verify_options.allowed_time_drift.as_secs()) {
                                        return Err(Error::InvalidToken);
                                    }
                                } else {
                                    return Err(Error::InvalidToken);
                                }
                            }
                        }
                    }

                    if let Some(expected_aud) = verify_options.aud {
                        match claims_object.get("aud") {
                            None => return Err(Error::InvalidToken),
                            Some(aud_value) => {
                                if let Some(aud) = aud_value.as_str() {
                                    if !expected_aud.contains(&aud) {
                                        return Err(Error::InvalidToken);
                                    }
                                } else {
                                    return Err(Error::InvalidToken);
                                }
                            }
                        }
                    }

                    if let Some(expected_iss) = verify_options.iss {
                        match claims_object.get("iss") {
                            None => return Err(Error::InvalidToken),
                            Some(iss_value) => {
                                if let Some(iss) = iss_value.as_str() {
                                    if !expected_iss.contains(&iss) {
                                        return Err(Error::InvalidToken);
                                    }
                                } else {
                                    return Err(Error::InvalidToken);
                                }
                            }
                        }
                    }
                }
                _ => return Err(Error::InvalidToken),
            };

            serde_json::from_value(claims_json_value).map_err(|_| Error::InvalidToken)?
        } else {
            serde_json::from_slice(&claims_json).map_err(|_| Error::InvalidToken)?
        };

    return Ok(claims);
}
