use heapless::Vec;

use crate::{
    KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE, KEY_EXCHANGE_SECRET_KEY_MAX_SIZE, KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE,
    SIGNATURE_MAX_SIZE, errors::Error,
};

pub trait CryptoProvider: Clone {
    const CIPHER_SUITES: &[CipherSuite];
    const KEY_EXCHANGE_GROUPS: &[KeyExchangeGroup];
    const SIGNATURE_SCHEMES: &[SignatureScheme];

    fn secure_random(&self, buf: &mut [u8]);

    // Hash / HMAC / HKDF (write into caller-provided output buffer)
    fn hash(&self, suite: CipherSuite, data: &[u8], out: &mut [u8]) -> Result<(), Error>;
    fn hmac(&self, suite: CipherSuite, key: &[u8], data: &[u8], out: &mut [u8]) -> Result<(), Error>;
    fn hkdf_extract(&self, suite: CipherSuite, salt: &[u8], ikm: &[u8], out: &mut [u8]) -> Result<(), Error>;
    fn hkdf_expand_label(
        &self,
        suite: CipherSuite,
        secret: &[u8],
        label: &[u8],
        context: &[u8],
        out: &mut [u8],
    ) -> Result<(), Error>;

    // AEAD (stateless, key expanded internally per call)
    fn aead_encrypt(
        &self,
        suite: CipherSuite,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        data: &mut [u8],
        plaintext_len: usize,
    ) -> Result<usize, Error>;
    fn aead_decrypt(
        &self,
        suite: CipherSuite,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        data: &mut [u8],
    ) -> Result<usize, Error>;

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
        sig_out: &mut [u8],
    ) -> Result<Vec<u8, SIGNATURE_MAX_SIZE>, Error>;
    fn verify(&self, scheme: SignatureScheme, public_key: &[u8], data: &[u8], signature: &[u8]) -> Result<(), Error>;

    // Certificate validation
    fn validate_cert_chain(
        &self,
        chain: &[&[u8]],
        server_name: Option<&str>,
        public_key_out: &mut [u8],
    ) -> Result<(SignatureScheme, usize), Error>;
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
    pub fn to_wire(self) -> [u8; 2] {
        match self {
            Self::TlsAes128GcmSha256 => [0x13, 0x01],
            Self::TlsAes256GcmSha384 => [0x13, 0x02],
            Self::TlsChaCha20Poly1305Sha256 => [0x13, 0x03],
        }
    }

    /// Parse a cipher suite from its wire identifier.
    pub fn from_wire(bytes: [u8; 2]) -> Option<Self> {
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
    /// draft-ietf-tls-hybrid-design
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
