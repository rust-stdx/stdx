use core::ops::{Deref, DerefMut};

use heapless::Vec;

use crate::{
    KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE, KEY_EXCHANGE_SECRET_KEY_MAX_SIZE, KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE,
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

pub trait CryptoProvider: Clone + Send + Sync {
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
    /// NIST P-256 (secp256r1) ECDHE
    Secp256r1,
    /// NIST P-384 (secp384r1) ECDHE
    Secp384r1,
    /// NIST P-521 (secp521r1) ECDHE
    Secp521r1,
    /// X448 (ECDHE with Curve448)
    X448,
    /// SecP256r1MLKEM768 — post-quantum hybrid (secp256r1 ECDHE + ML-KEM-768 KEM).
    /// draft-ietf-tls-ecdhe-mlkem
    Secp256r1MlKem768,
    /// SecP384r1MLKEM1024 — post-quantum hybrid (secp384r1 ECDHE + ML-KEM-1024 KEM).
    /// draft-ietf-tls-ecdhe-mlkem
    Secp384r1MlKem1024,
}

impl KeyExchangeGroup {
    /// TLS wire identifier (two bytes, big-endian).
    pub const fn to_wire(self) -> [u8; 2] {
        match self {
            Self::X25519 => [0x00, 0x1D],
            Self::X25519MlKem768 => [0x11, 0xEC],
            Self::Secp256r1 => [0x00, 0x17],
            Self::Secp384r1 => [0x00, 0x18],
            Self::Secp521r1 => [0x00, 0x19],
            Self::X448 => [0x00, 0x1E],
            Self::Secp256r1MlKem768 => [0x11, 0xEB],
            Self::Secp384r1MlKem1024 => [0x11, 0xED],
        }
    }

    /// Parse a key exchange group from its wire identifier.
    pub const fn from_wire(bytes: [u8; 2]) -> Option<Self> {
        match bytes {
            [0x00, 0x1D] => Some(Self::X25519),
            [0x11, 0xEC] => Some(Self::X25519MlKem768),
            [0x00, 0x17] => Some(Self::Secp256r1),
            [0x00, 0x18] => Some(Self::Secp384r1),
            [0x00, 0x19] => Some(Self::Secp521r1),
            [0x00, 0x1E] => Some(Self::X448),
            [0x11, 0xEB] => Some(Self::Secp256r1MlKem768),
            [0x11, 0xED] => Some(Self::Secp384r1MlKem1024),
            _ => None,
        }
    }

    /// Size of the client-side KeyShare entry for this group, in bytes.
    pub const fn public_key_size_client(self) -> usize {
        match self {
            Self::X25519 => 32,
            Self::X25519MlKem768 => 1216,
            Self::Secp256r1 => 65,
            Self::Secp384r1 => 97,
            Self::Secp521r1 => 133,
            Self::X448 => 56,
            Self::Secp256r1MlKem768 => 1249,
            Self::Secp384r1MlKem1024 => 1665,
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
            Self::Secp256r1 => 65,
            Self::Secp384r1 => 97,
            Self::Secp521r1 => 133,
            Self::X448 => 56,
            Self::Secp256r1MlKem768 => 1153,
            Self::Secp384r1MlKem1024 => 1665,
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

#[derive(Debug)]
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
