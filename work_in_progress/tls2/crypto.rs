use core::ops::{Deref, DerefMut};

use heapless::Vec;

use crate::{
    KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE, KEY_EXCHANGE_SECRET_KEY_MAX_SIZE, KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE, MAX_CERTS,
    MAX_HASH_SIZE, SIGNATURE_MAX_SIZE, errors::Error,
};

/// A fixed-capacity byte buffer for cryptographic hash outputs.
///
/// Stores up to 48 bytes (enough for SHA-384) and tracks how many are active
/// (`len`).  `Deref`/`DerefMut` yield a `&[u8]`/`&mut [u8]` of exactly `len`
/// bytes.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Hash {
    buf: [u8; Self::MAX_LEN],
    len: u8,
}

impl Hash {
    pub const MAX_LEN: usize = MAX_HASH_SIZE;

    pub fn new_zeroed(len: u8) -> Self {
        assert!((len as usize) <= Self::MAX_LEN);
        Self {
            buf: [0u8; Self::MAX_LEN],
            len,
        }
    }

    pub fn from_slice(data: &[u8]) -> Self {
        assert!(data.len() <= Self::MAX_LEN);
        let mut buf = [0u8; Self::MAX_LEN];
        buf[..data.len()].copy_from_slice(data);
        Self {
            buf,
            len: data.len() as u8,
        }
    }

    pub const fn zeroed() -> Self {
        Self {
            buf: [0u8; Self::MAX_LEN],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.buf = [0u8; Self::MAX_LEN];
        self.len = 0;
    }
}

impl Deref for Hash {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.buf[..self.len as usize]
    }
}

impl DerefMut for Hash {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..self.len as usize]
    }
}

/// A pre-parsed X.509 certificate with all commonly accessed fields
/// extracted in a single DER walk.
pub struct ParsedCertificate<'a> {
    /// Full DER encoding.
    pub der: &'a [u8],
    /// SubjectPublicKeyInfo DER.
    pub spki: &'a [u8],
    /// Raw public key bytes (BIT STRING content, minus unused-bits byte).
    pub public_key: &'a [u8],
    /// Issuer Distinguished Name (value of the Name SEQUENCE).
    pub issuer_dn: &'a [u8],
    /// Subject Distinguished Name (value of the Name SEQUENCE).
    pub subject_dn: &'a [u8],
    /// TBSCertificate raw bytes (tag + length + value) — the signed portion.
    pub tbs: &'a [u8],
    /// Signature value bytes (BIT STRING content).
    pub signature_value: &'a [u8],
    /// Signature algorithm OID.
    pub sig_alg_oid: &'a [u8],
    /// SPKI algorithm OID.
    pub spki_alg_oid: &'a [u8],
    /// Whether Basic Constraints cA is TRUE (`None` = extension absent).
    pub is_ca: Option<bool>,
    /// Whether EKU includes serverAuth (`None` = extension absent).
    pub has_server_auth_eku: Option<bool>,
    /// notBefore as Unix timestamp (seconds since epoch).
    pub not_before: u64,
    /// notAfter as Unix timestamp (seconds since epoch).
    pub not_after: u64,
}

impl<'a> ParsedCertificate<'a> {
    /// Parse a DER-encoded X.509 certificate, extracting all fields at once.
    pub fn from_der(der: &'a [u8]) -> Result<Self, Error> {
        let spki = x509::extract_spki_from_cert(der).map_err(|_| Error::CertificateParseFailed)?;
        let public_key = x509::extract_key_from_spki(spki).map_err(|_| Error::CertificateParseFailed)?;
        let issuer_dn = x509::extract_issuer_dn(der).map_err(|_| Error::CertificateParseFailed)?;
        let subject_dn = x509::extract_subject_dn(der).map_err(|_| Error::CertificateParseFailed)?;
        let tbs = x509::extract_tbs_cert(der).map_err(|_| Error::CertificateParseFailed)?;
        let signature_value = x509::extract_signature_value(der).map_err(|_| Error::CertificateParseFailed)?;
        let sig_alg_oid = x509::extract_signature_algorithm_oid(der).map_err(|_| Error::CertificateParseFailed)?;
        let spki_alg_oid = x509::extract_spki_algorithm_oid(spki).map_err(|_| Error::CertificateParseFailed)?;
        let is_ca = x509::is_ca(der);
        let has_server_auth_eku = x509::has_eku_server_auth(der);
        let (nb, na) = x509::parse_validity(der).map_err(|_| Error::CertificateParseFailed)?;

        Ok(Self {
            der,
            spki,
            public_key,
            issuer_dn,
            subject_dn,
            tbs,
            signature_value,
            sig_alg_oid,
            spki_alg_oid,
            is_ca,
            has_server_auth_eku,
            not_before: nb.to_unix_seconds(),
            not_after: na.to_unix_seconds(),
        })
    }
}

pub trait CryptoProvider: Clone {
    const CIPHER_SUITES: &[CipherSuite];
    const KEY_EXCHANGE_GROUPS: &[KeyExchangeGroup];
    const SIGNATURE_SCHEMES: &[SignatureScheme];

    /// Opaque incremental hash state, must be Clone for transcript checkpointing.
    type Hasher: Clone + Unpin;

    /// A distinct type is used to avoid computation for ench encrypt / decrypt operation for some
    /// ciphers (e.g. AES key expanding)
    type AeadKey: Unpin;

    fn secure_random(&self, buf: &mut [u8]);

    // Hash / HMAC / HKDF

    /// Create a new incremental hash state for the given suite.
    fn new_hash(&self, suite: CipherSuite) -> Self::Hasher;
    /// Absorb data into the hash state.
    fn hash_update(&self, state: &mut Self::Hasher, data: &[u8]);
    /// Finalize the hash and write the digest into `out`. Consumes the state.
    fn hash_finalize(&self, state: Self::Hasher) -> Result<Hash, Error>;

    /// One-shot hash (default implementation uses the incremental API).
    fn hash(&self, suite: CipherSuite, data: &[u8]) -> Result<Hash, Error> {
        let mut state = self.new_hash(suite);
        self.hash_update(&mut state, data);
        self.hash_finalize(state)
    }

    fn hmac(&self, suite: CipherSuite, key: &Hash, data: &[u8]) -> Result<Hash, Error>;
    fn hkdf_extract(&self, suite: CipherSuite, salt: &Hash, ikm: &[u8]) -> Result<Hash, Error>;
    fn hkdf_expand_label(
        &self,
        out: &mut [u8],
        suite: CipherSuite,
        secret: &Hash,
        label: &[u8],
        context: &[u8],
    ) -> Result<(), Error>;

    // AEAD
    fn new_aead_key(&self, suite: CipherSuite, key: &[u8]) -> Self::AeadKey;
    fn aead_encrypt(
        &self,
        key: &Self::AeadKey,
        nonce: &[u8],
        aad: &[u8],
        data: &mut [u8],
        plaintext_len: usize,
    ) -> Result<usize, Error>;
    fn aead_decrypt(&self, key: &Self::AeadKey, nonce: &[u8], aad: &[u8], data: &mut [u8]) -> Result<usize, Error>;

    // Key exchange
    fn key_exchange_generate_keypair(
        &self,
        group: KeyExchangeGroup,
    ) -> Result<(KeyExchangeSecretKey, KeyExchangePublicKey), Error>;
    fn key_exchange(
        &self,
        secret: &KeyExchangeSecretKey,
        peer_public: &[u8],
    ) -> Result<Vec<u8, KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE>, Error>;

    // Signatures
    fn sign(
        &self,
        scheme: SignatureScheme,
        secret_key: &[u8],
        data: &[u8],
    ) -> Result<Vec<u8, SIGNATURE_MAX_SIZE>, Error>;
    fn verify(&self, scheme: SignatureScheme, public_key: &[u8], data: &[u8], signature: &[u8]) -> Result<(), Error>;

    // Certificate validation
    fn verify_certificate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error>;
}

/// A trusted root certificate authority with pre-parsed fields.
///
/// All fields use fixed-capacity [`heapless::Vec`] for zero-allocation
/// storage. Use [`RootCa::from_der`] to parse a DER-encoded certificate.
#[derive(Clone)]
pub struct RootCa {
    pub subject_dn: heapless::Vec<u8, 256>,
    pub spki: heapless::Vec<u8, 512>,
    pub spki_alg_oid: heapless::Vec<u8, 16>,
}

impl RootCa {
    /// Parse a DER-encoded X.509 certificate into a root trust anchor.
    pub fn from_der(der: &[u8]) -> Result<Self, Error> {
        let parsed = ParsedCertificate::from_der(der)?;
        let mut subject_dn = heapless::Vec::new();
        subject_dn
            .extend_from_slice(parsed.subject_dn)
            .map_err(|_| Error::CertificateParseFailed)?;
        let mut spki = heapless::Vec::new();
        spki.extend_from_slice(parsed.spki)
            .map_err(|_| Error::CertificateParseFailed)?;
        let mut spki_alg_oid = heapless::Vec::new();
        spki_alg_oid
            .extend_from_slice(parsed.spki_alg_oid)
            .map_err(|_| Error::CertificateParseFailed)?;
        Ok(Self {
            subject_dn,
            spki,
            spki_alg_oid,
        })
    }
}

/// A certificate received from the peer during the TLS handshake.
pub enum ReceivedCertificate<'a> {
    /// X.509 certificate chain, end-entity first.
    X509 {
        chain: heapless::Vec<ParsedCertificate<'a>, MAX_CERTS>,
    },
    /// Raw public key (RFC 7250).
    RawPublicKey {
        public_key: &'a [u8],
        scheme: SignatureScheme,
    },
}

/// Cipher suites supported by a [`CryptoProvider`].
///
/// In this library the cipher suite prescribes the AEAD cipher, the hash
/// function used throughout the TLS 1.3 key schedule, and the transcript hash.
///
/// TLS 1.3 cipher suites always pair one AEAD with one hash.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CipherSuite {
    /// AES-128-GCM with SHA-256
    TlsAes128GcmSha256,
    /// AES-256-GCM with SHA-384
    TlsAes256GcmSha384,
    /// ChaCha20-Poly1305 with SHA-256
    TlsChaCha20Poly1305Sha256,
}

impl core::fmt::Debug for CipherSuite {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TlsAes128GcmSha256 => write!(f, "TLS13_AES_128_GCM_SHA256"),
            Self::TlsAes256GcmSha384 => write!(f, "TLS13_AES_256_GCM_SHA384"),
            Self::TlsChaCha20Poly1305Sha256 => write!(f, "TLS13_CHACHA20_POLY1305_SHA256"),
        }
    }
}

impl CipherSuite {
    /// TLS wire identifier (two bytes, big-endian).
    pub const fn to_wire(self) -> [u8; 2] {
        match self {
            Self::TlsAes128GcmSha256 => [0x13, 0x01],
            Self::TlsAes256GcmSha384 => [0x13, 0x02],
            Self::TlsChaCha20Poly1305Sha256 => [0x13, 0x03],
        }
    }

    /// Parse a cipher suite from its wire identifier.
    pub const fn from_wire(bytes: [u8; 2]) -> Option<Self> {
        match bytes {
            [0x13, 0x01] => Some(Self::TlsAes128GcmSha256),
            [0x13, 0x02] => Some(Self::TlsAes256GcmSha384),
            [0x13, 0x03] => Some(Self::TlsChaCha20Poly1305Sha256),
            _ => None,
        }
    }

    /// Size in bytes of the AEAD key for this suite.
    pub const fn key_size(self) -> usize {
        match self {
            Self::TlsAes128GcmSha256 => 16,
            Self::TlsAes256GcmSha384 => 32,
            Self::TlsChaCha20Poly1305Sha256 => 32,
        }
    }

    /// Size in bytes of the hash output.
    pub const fn hash_size(self) -> usize {
        match self {
            Self::TlsAes128GcmSha256 => 32,
            Self::TlsAes256GcmSha384 => 48,
            Self::TlsChaCha20Poly1305Sha256 => 32,
        }
    }
}

/// Key exchange group identifiers (RFC 8446 §4.2.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyExchangeGroup {
    /// X25519 (ECDHE with Curve25519)
    X25519,
    /// X25519MLKEM768 — post-quantum hybrid (X25519 ECDHE + ML-KEM-768 KEM).
    /// draft-ietf-tls-ecdhe-mlkem
    X25519MlKem768,
}

impl KeyExchangeGroup {
    /// TLS wire identifier (two bytes, big-endian).
    pub const fn to_wire(self) -> [u8; 2] {
        match self {
            Self::X25519 => [0x00, 0x1D],
            Self::X25519MlKem768 => [0x11, 0xEC],
        }
    }

    /// Parse a key exchange group from its wire identifier.
    pub const fn from_wire(bytes: [u8; 2]) -> Option<Self> {
        match bytes {
            [0x00, 0x1D] => Some(Self::X25519),
            [0x11, 0xEC] => Some(Self::X25519MlKem768),
            _ => None,
        }
    }

    /// Size of the client-side KeyShare entry for this group, in bytes.
    pub const fn public_key_size_client(self) -> usize {
        match self {
            Self::X25519 => 32,
            Self::X25519MlKem768 => 1216,
        }
    }

    /// Size of the server-side KeyShare entry for this group, in bytes.
    ///
    /// For KEM-based groups the server returns a ciphertext (which may
    /// differ from the client's public key size).
    pub const fn public_key_size_server(self) -> usize {
        match self {
            Self::X25519 => 32,
            Self::X25519MlKem768 => 1120,
        }
    }
}

pub struct KeyExchangeSecretKey {
    bytes: Vec<u8, KEY_EXCHANGE_SECRET_KEY_MAX_SIZE>,
    group: KeyExchangeGroup,
}

impl KeyExchangeSecretKey {
    #[inline]
    pub fn new(group: KeyExchangeGroup, bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.try_into().unwrap(),
            group,
        }
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    pub fn group(&self) -> KeyExchangeGroup {
        self.group
    }
}

pub struct KeyExchangePublicKey {
    bytes: Vec<u8, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE>,
    group: KeyExchangeGroup,
}

impl KeyExchangePublicKey {
    #[inline]
    pub fn new(group: KeyExchangeGroup, bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.try_into().unwrap(),
            group,
        }
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    pub fn group(&self) -> KeyExchangeGroup {
        self.group
    }
}

/// Signature schemes (RFC 8446 §4.2.3).
///
/// Only the schemes needed for raw public key authentication are listed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignatureScheme {
    /// Ed25519
    Ed25519,
    /// ECDSA with NIST P-256 and SHA-256
    EcdsaP256Sha256,
    /// ECDSA with NIST P-384 and SHA-384
    EcdsaP384Sha384,
    /// RSA PKCS#1 v1.5 with SHA-256
    RsaPkcs1Sha256,
    /// RSA PKCS#1 v1.5 with SHA-384
    RsaPkcs1Sha384,
    /// RSA PKCS#1 v1.5 with SHA-512
    RsaPkcs1Sha512,
    /// RSA-PSS with SHA-256 (RFC 8017)
    RsaPssRsaSha256,
    /// RSA-PSS with SHA-384 (RFC 8017)
    RsaPssRsaSha384,
    /// RSA-PSS with SHA-512 (RFC 8017)
    RsaPssRsaSha512,
}

impl SignatureScheme {
    /// TLS wire identifier (two bytes, big-endian).
    pub fn to_wire(self) -> [u8; 2] {
        match self {
            Self::Ed25519 => [0x08, 0x07],
            Self::EcdsaP256Sha256 => [0x04, 0x03],
            Self::EcdsaP384Sha384 => [0x05, 0x03],
            Self::RsaPkcs1Sha256 => [0x04, 0x01],
            Self::RsaPkcs1Sha384 => [0x05, 0x01],
            Self::RsaPkcs1Sha512 => [0x06, 0x01],
            Self::RsaPssRsaSha256 => [0x08, 0x04],
            Self::RsaPssRsaSha384 => [0x08, 0x05],
            Self::RsaPssRsaSha512 => [0x08, 0x06],
        }
    }

    /// Parse a signature scheme from its wire identifier.
    pub fn from_wire(bytes: [u8; 2]) -> Option<Self> {
        match bytes {
            [0x08, 0x07] => Some(Self::Ed25519),
            [0x04, 0x03] => Some(Self::EcdsaP256Sha256),
            [0x05, 0x03] => Some(Self::EcdsaP384Sha384),
            [0x04, 0x01] => Some(Self::RsaPkcs1Sha256),
            [0x05, 0x01] => Some(Self::RsaPkcs1Sha384),
            [0x06, 0x01] => Some(Self::RsaPkcs1Sha512),
            [0x08, 0x04] => Some(Self::RsaPssRsaSha256),
            [0x08, 0x05] => Some(Self::RsaPssRsaSha384),
            [0x08, 0x06] => Some(Self::RsaPssRsaSha512),
            _ => None,
        }
    }

    /// Expected size of the raw secret key in bytes.
    pub fn secret_key_size(self) -> usize {
        match self {
            Self::Ed25519 => 32,
            Self::EcdsaP256Sha256 => 32,
            Self::EcdsaP384Sha384 => 48,
            Self::RsaPkcs1Sha256 => 256, // typical RSA PKCS#8
            Self::RsaPkcs1Sha384 => 256,
            Self::RsaPkcs1Sha512 => 256,
            Self::RsaPssRsaSha256 => 256,
            Self::RsaPssRsaSha384 => 256,
            Self::RsaPssRsaSha512 => 256,
        }
    }

    /// Expected size of the raw public key in bytes.
    pub fn public_key_size(self) -> usize {
        match self {
            Self::Ed25519 => 32,
            Self::EcdsaP256Sha256 => 65,
            Self::EcdsaP384Sha384 => 97,
            Self::RsaPkcs1Sha256 => 294, // typical RSA 2048-bit
            Self::RsaPkcs1Sha384 => 294,
            Self::RsaPkcs1Sha512 => 294,
            Self::RsaPssRsaSha256 => 294,
            Self::RsaPssRsaSha384 => 294,
            Self::RsaPssRsaSha512 => 294,
        }
    }

    /// Signature size in bytes.
    pub fn signature_size(self) -> usize {
        match self {
            Self::Ed25519 => 64,
            Self::EcdsaP256Sha256 => 64,
            Self::EcdsaP384Sha384 => 96,
            Self::RsaPkcs1Sha256 => 256, // RSA 2048-bit
            Self::RsaPkcs1Sha384 => 256,
            Self::RsaPkcs1Sha512 => 256,
            Self::RsaPssRsaSha256 => 256,
            Self::RsaPssRsaSha384 => 256,
            Self::RsaPssRsaSha512 => 256,
        }
    }
}

/// Certificate types for `server_certificate_type` / `client_certificate_type`
/// extension negotiation (RFC 7250 / RFC 9633).
///
/// In TLS 1.3 the default is X.509. Raw public keys require explicit
/// negotiation via the `server_certificate_type` extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CertType {
    /// X.509 certificate (default).
    X509 = 0,
    /// Raw public key (RFC 7250).
    RawPublicKey = 1,
}

impl CertType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::X509),
            1 => Some(Self::RawPublicKey),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::X509 => "X.509",
            Self::RawPublicKey => "RawPublicKey",
        }
    }
}
