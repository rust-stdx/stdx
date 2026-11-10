use std::fmt;

use crypto::{Hasher, p256::PublicKey, sha2::Sha256};
// use rustls_pki_types::CertificateDer;
use json::{DecodeOwned, Encode};
use reqwest::Response;
use thiserror::Error;
// use ureq::Response;

/// Error type for instant-acme
#[derive(Debug, Error)]
pub enum Error {
    /// An JSON problem as returned by the ACME server
    ///
    /// RFC 8555 uses problem documents as described in RFC 7807.
    #[error(transparent)]
    Api(#[from] Problem),
    /// Failed to base64-decode data
    #[error("base64 decoding failed: {0}")]
    Base64(#[from] base64::DecodeError),
    /// Failed from cryptographic operations
    #[error("cryptographic operation failed")]
    Crypto,
    /// Failed to instantiate a private key
    #[error("invalid key bytes")]
    CryptoKey,
    /// HTTP request failure
    #[error("HTTP request failure: {0}")]
    Http(#[from] reqwest::Error),
    /// HTTP IO failure
    #[error("HTTP IO failure: {0}")]
    HttpIo(#[from] std::io::Error),
    /// Failed to (de)serialize a JSON object
    #[error("failed to (de)serialize JSON: {0}")]
    Json(#[from] json::Error),
    /// Miscellaneous errors
    #[error("missing data: {0}")]
    Str(&'static str),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Str(s)
    }
}

/// ACME account credentials
///
/// This opaque type contains the account ID, the private key data and the
/// server URLs from the relevant ACME server. This can be used to serialize
/// the account credentials to a file or secret manager and restore the
/// account from persistent storage.
#[derive(json::Decode, json::Encode, Clone)]
pub struct AccountCredentials {
    pub(crate) id: String,
    /// Stored in DER, serialized as base64
    #[json(with = "pkcs8_serde")]
    pub(crate) key_pkcs8: Vec<u8>,
    pub(crate) directory: Option<String>,
    pub(crate) urls: Option<DirectoryUrls>,
}

mod pkcs8_serde {
    use json::{Decoder, Encoder, Error, Result, Write};

    pub(crate) fn serialize<W: Write>(
        key_pkcs8: &Vec<u8>,
        encoder: &mut Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        encoder.string(&base64::encode(key_pkcs8, base64::Alphabet::UrlNoPadding))
    }

    pub(crate) fn deserialize<'a>(decoder: &mut Decoder<'a>) -> Result<Vec<u8>> {
        let text = decoder.decode_str()?;
        base64::decode(text, base64::Alphabet::UrlNoPadding)
            .map_err(|_| Error::message("invalid base64 private key", decoder.position()))
    }
}

/// An RFC 7807 problem document as returned by the ACME server
#[derive(Clone, Debug, json::Decode)]
#[json(rename_all = "camelCase")]
pub struct Problem {
    /// One of an enumerated list of problem types
    ///
    /// See <https://datatracker.ietf.org/doc/html/rfc8555#section-6.7>
    pub r#type: Option<String>,
    /// A human-readable explanation of the problem
    pub detail: Option<String>,
    /// The HTTP status code returned for this response
    pub status: Option<u16>,
}

impl Problem {
    pub(crate) async fn check<T: DecodeOwned>(rsp: Response) -> Result<T, Error> {
        let body = rsp.bytes().await?;
        Ok(T::decode_owned(&body)?)
    }

    pub(crate) async fn from_response(rsp: Response) -> Result<Vec<u8>, Error> {
        let status = rsp.status();
        let body = rsp.bytes().await?;

        if (100..=399).contains(&status.as_u16()) {
            return Ok(body.to_vec());
        }

        Err(json::from_slice::<Problem>(&body)?.into())
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("API error")?;
        if let Some(detail) = &self.detail {
            write!(f, ": {detail}")?;
        }

        if let Some(r#type) = &self.r#type {
            write!(f, " ({})", r#type)?;
        }

        Ok(())
    }
}

impl core::error::Error for Problem {}

#[derive(Debug, json::Encode)]
pub(crate) struct FinalizeRequest {
    csr: String,
}

impl FinalizeRequest {
    pub(crate) fn new(csr_der: &[u8]) -> Self {
        Self {
            csr: base64::encode(csr_der, base64::Alphabet::UrlNoPadding),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Header<'a> {
    pub(crate) alg: SigningAlgorithm,
    pub(crate) key: KeyOrKeyId<'a>,
    pub(crate) nonce: Option<&'a str>,
    pub(crate) url: &'a str,
}

impl Encode for Header<'_> {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_object()?;
        encoder.comma(true)?;
        encoder.key("alg")?;
        self.alg.encode(encoder)?;
        match &self.key {
            KeyOrKeyId::Key(jwk) => {
                encoder.comma(false)?;
                encoder.key("jwk")?;
                jwk.encode(encoder)?;
            }
            KeyOrKeyId::KeyId(kid) => {
                encoder.comma(false)?;
                encoder.key("kid")?;
                encoder.string(kid)?;
            }
        }
        if let Some(nonce) = self.nonce {
            encoder.comma(false)?;
            encoder.key("nonce")?;
            encoder.string(nonce)?;
        }
        encoder.comma(false)?;
        encoder.key("url")?;
        encoder.string(self.url)?;
        encoder.end_object()
    }
}

#[derive(Debug)]
pub(crate) enum KeyOrKeyId<'a> {
    Key(Jwk),
    KeyId(&'a str),
}

impl<'a> KeyOrKeyId<'a> {
    pub(crate) fn from_key(key: &PublicKey) -> KeyOrKeyId<'static> {
        KeyOrKeyId::Key(Jwk::new(key))
    }
}

#[derive(Debug, json::Encode)]
pub(crate) struct Jwk {
    alg: SigningAlgorithm,
    crv: &'static str,
    kty: &'static str,
    r#use: &'static str,
    x: String,
    y: String,
}

impl Jwk {
    pub(crate) fn new(key: &PublicKey) -> Self {
        let bytes = key.to_bytes();
        let (x, y) = bytes[1..].split_at(32);
        Self {
            alg: SigningAlgorithm::Es256,
            crv: "P-256",
            kty: "EC",
            r#use: "sig",
            x: base64::encode(x, base64::Alphabet::UrlNoPadding),
            y: base64::encode(y, base64::Alphabet::UrlNoPadding),
        }
    }

    pub(crate) fn thumb_sha256(key: &PublicKey) -> Result<[u8; 32], json::Error> {
        let jwk = Self::new(key);
        let hash = Sha256::hash(&json::to_vec(&JwkThumb {
            crv: jwk.crv,
            kty: jwk.kty,
            x: &jwk.x,
            y: &jwk.y,
        }));
        let mut out = [0u8; 32];
        out.copy_from_slice(&hash.as_ref()[..32]);
        Ok(out)
    }
}

#[derive(Debug, json::Encode)]
struct JwkThumb<'a> {
    crv: &'a str,
    kty: &'a str,
    x: &'a str,
    y: &'a str,
}

/// An ACME challenge as described in RFC 8555 (section 7.1.5)
///
/// <https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.5>
#[derive(Debug, json::Decode)]
pub struct Challenge {
    /// Type of challenge
    pub r#type: ChallengeType,
    /// Challenge identifier
    pub url: String,
    /// Token for this challenge
    pub token: String,
    /// Current status
    pub status: ChallengeStatus,
    /// Potential error state
    pub error: Option<Problem>,
}

/// Contents of an ACME order as described in RFC 8555 (section 7.1.3)
///
/// The order identity will usually be represented by an [Order](crate::Order).
///
/// <https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.3>
#[derive(Debug, json::Decode)]
#[json(rename_all = "camelCase")]
pub struct OrderState {
    /// Current status
    pub status: OrderStatus,
    /// Authorization URLs for this order
    ///
    /// There should be one authorization per identifier in the order.
    pub authorizations: Vec<String>,
    /// Potential error state
    pub error: Option<Problem>,
    /// A finalization URL, to be used once status becomes `Ready`
    pub finalize: String,
    /// The certificate URL, which becomes available after finalization
    pub certificate: Option<String>,
}

/// Input data for [Order](crate::Order) creation
///
/// To be passed into [Account::new_order()](crate::Account::new_order()).
#[derive(Debug, json::Encode)]
#[json(rename_all = "camelCase")]
pub struct NewOrder<'a> {
    /// Identifiers to be included in the order
    pub identifiers: &'a [Identifier],
}

/// The reason for a certificate revocation
/// Defined in <https://datatracker.ietf.org/doc/html/rfc5280#section-5.3.1>
#[allow(missing_docs)]
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum RevocationReason {
    Unspecified = 0,
    KeyCompromise = 1,
    CaCompromise = 2,
    AffiliationChanged = 3,
    Superseded = 4,
    CessationOfOperation = 5,
    CertificateHold = 6,
    RemoveFromCrl = 8,
    PrivilegeWithdrawn = 9,
    AaCompromise = 10,
}

impl Encode for RevocationReason {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.u64(self.clone() as u64)
    }
}

pub(crate) struct NewAccountPayload<'a> {
    pub(crate) new_account: &'a NewAccount<'a>,
    pub(crate) external_account_binding: Option<JoseJson>,
}

impl Encode for NewAccountPayload<'_> {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_object()?;
        encoder.comma(true)?;
        encoder.key("contact")?;
        self.new_account.contact.encode(encoder)?;
        encoder.comma(false)?;
        encoder.key("termsOfServiceAgreed")?;
        self.new_account.terms_of_service_agreed.encode(encoder)?;
        encoder.comma(false)?;
        encoder.key("onlyReturnExisting")?;
        self.new_account.only_return_existing.encode(encoder)?;
        if let Some(external) = &self.external_account_binding {
            encoder.comma(false)?;
            encoder.key("externalAccountBinding")?;
            external.encode(encoder)?;
        }
        encoder.end_object()
    }
}

/// Input data for [Account](crate::Account) creation
///
/// To be passed into [Account::create()](crate::Account::create()).
#[derive(Debug, json::Encode)]
#[json(rename_all = "camelCase")]
pub struct NewAccount<'a> {
    /// A list of contact URIs (like `mailto:info@example.com`)
    pub contact: &'a [&'a str],
    /// Whether you agree to the terms of service
    pub terms_of_service_agreed: bool,
    /// Set to `true` in order to retrieve an existing account
    ///
    /// Setting this to `false` has not been tested.
    pub only_return_existing: bool,
}

#[derive(Debug, Clone, json::Decode, json::Encode)]
#[json(rename_all = "camelCase")]
pub(crate) struct DirectoryUrls {
    pub(crate) new_nonce: String,
    pub(crate) new_account: String,
    pub(crate) new_order: String,
    pub(crate) revoke_cert: String,
}

#[derive(json::Encode)]
pub(crate) struct JoseJson {
    pub(crate) protected: String,
    pub(crate) payload: String,
    pub(crate) signature: String,
}

impl JoseJson {
    pub(crate) fn new(
        payload: Option<&impl Encode>,
        protected: Header<'_>,
        signer: &impl Signer,
    ) -> Result<Self, Error> {
        let protected = base64(&protected)?;
        let payload = match payload {
            Some(data) => base64(&data)?,
            None => String::new(),
        };

        let combined = format!("{protected}.{payload}");
        let signature = signer.sign(combined.as_bytes())?;
        Ok(Self {
            protected,
            payload,
            signature: base64::encode(signature.as_ref(), base64::Alphabet::UrlNoPadding),
        })
    }
}

pub(crate) trait Signer {
    type Signature: AsRef<[u8]>;

    fn header<'n, 'u: 'n, 's: 'u>(&'s self, nonce: Option<&'n str>, url: &'u str) -> Header<'n>;

    fn sign(&self, payload: &[u8]) -> Result<Self::Signature, Error>;
}

fn base64(data: &impl Encode) -> Result<String, json::Error> {
    let json_data = json::to_vec(data);
    let encoded_json_data = base64::encode(&json_data, base64::Alphabet::UrlNoPadding);
    return Ok(encoded_json_data);
}

/// An ACME authorization as described in RFC 8555 (section 7.1.4)
#[derive(Debug, json::Decode)]
#[json(rename_all = "camelCase")]
pub struct Authorization {
    /// The identifier that the account is authorized to represent
    pub identifier: Identifier,
    /// Current state of the authorization
    pub status: AuthorizationStatus,
    /// Possible challenges for the authorization
    pub challenges: Vec<Challenge>,
}

/// Status for an [`Authorization`]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, json::Decode)]
#[json(rename_all = "camelCase")]
pub enum AuthorizationStatus {
    Pending,
    Valid,
    Invalid,
    Revoked,
    Expired,
}

/// Represent an identifier in an ACME [Order](crate::Order)
#[allow(missing_docs)]
#[derive(Clone, Debug, json::Encode, json::Decode)]
#[json(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Identifier {
    Dns(String),
}

/// The challenge type
#[derive(Clone, Copy, Debug, json::Decode, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ChallengeType {
    #[json(rename = "http-01")]
    Http01,
    #[json(rename = "dns-01")]
    Dns01,
    #[json(rename = "tls-alpn-01")]
    TlsAlpn01,
}

#[derive(Clone, Copy, Debug, json::Decode)]
#[json(rename_all = "camelCase")]
pub enum ChallengeStatus {
    Pending,
    Processing,
    Valid,
    Invalid,
}

/// Status of an [Order](crate::Order)
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, json::Decode, PartialEq, Eq)]
#[json(rename_all = "camelCase")]
pub enum OrderStatus {
    Pending,
    Ready,
    Processing,
    Valid,
    Invalid,
}

/// Helper type to reference Let's Encrypt server URLs
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum LetsEncrypt {
    Production,
    Staging,
}

impl LetsEncrypt {
    /// Get the directory URL for the given Let's Encrypt server
    pub const fn url(&self) -> &'static str {
        match self {
            Self::Production => "https://acme-v02.api.letsencrypt.org/directory",
            Self::Staging => "https://acme-staging-v02.api.letsencrypt.org/directory",
        }
    }
}

/// ZeroSSL ACME only supports production at the moment
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum ZeroSsl {
    Production,
}

impl ZeroSsl {
    /// Get the directory URL for the given ZeroSSL server
    pub const fn url(&self) -> &'static str {
        match self {
            Self::Production => "https://acme.zerossl.com/v2/DV90",
        }
    }
}

#[derive(Clone, Copy, Debug, json::Encode)]
#[json(rename_all = "UPPERCASE")]
pub(crate) enum SigningAlgorithm {
    /// ECDSA using P-256 and SHA-256
    Es256,
    /// HMAC with SHA-256,
    Hs256,
}

#[derive(Debug, json::Encode, json::Decode)]
pub(crate) struct Empty {}
