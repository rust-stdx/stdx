use alloc::boxed::Box;

use heapless::Vec;

use crate::Error;

pub const MAX_HASH_OUTPUT: usize = 48;
pub const MAX_KEY_SIZE: usize = 32;
pub const MAX_AEAD_TAG_SIZE: usize = 32;
pub const MAX_KX_PUBLIC_KEY: usize = 1216;
pub const MAX_SHARED_SECRET: usize = 64;
pub const MAX_PUBLIC_KEY_BYTES: usize = 97;
pub const MAX_SIGNATURE_SIZE: usize = 256;
pub const MAX_SESSION_ID: usize = 32;
pub const MAX_CERT_TYPES: usize = 4;
pub const PSK_MAX_SIZE: usize = MAX_HASH_OUTPUT;

/// A cryptographic provider that supplies all primitives needed for a TLS 1.3
/// connection.
///
/// The provider *determines* which cipher suites, key-exchange groups, and
/// signature schemes are available. Implement this trait to use your own
/// crypto backend.
///
/// A default implementation backed by the `crypto` crate is available behind
/// the `crypto-default-provider` feature.
pub trait CryptoProvider: Send + Sync + 'static {
    /// The cipher suites supported by this provider.
    fn supported_cipher_suites(&self) -> &[CipherSuite];

    /// The key exchange groups supported by this provider.
    fn supported_key_exchange_groups(&self) -> &[KeyExchangeGroup];

    /// The signature schemes supported by this provider.
    fn supported_signature_schemes(&self) -> &[SignatureScheme];

    /// Create an AEAD instance for the given cipher suite and traffic key.
    fn create_aead(&self, suite: CipherSuite, key: &[u8]) -> Result<Box<dyn Aead>, Error>;

    /// Generate a fresh key-exchange key pair for `group`.
    fn create_kx_pair(&self, group: KeyExchangeGroup) -> Result<Box<dyn KeyExchangeKeyPair>, Error>;

    /// Create a signer from a raw Ed25519 secret key (32 bytes).
    fn create_signer(&self, scheme: SignatureScheme, secret_key: &[u8]) -> Result<Box<dyn Signer>, Error>;

    /// Verify a signature over `data` using the given scheme and public key.
    fn verify_signature(
        &self,
        scheme: SignatureScheme,
        public_key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<(), Error>;

    /// Hash the given data using the hash function associated with the given
    /// cipher suite (oneshot).
    fn hash(&self, suite: CipherSuite, data: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT>;

    /// Fill `buf` with cryptographically secure random bytes.
    fn secure_random(&self, buf: &mut [u8]);

    /// Compute HMAC-Hash for the given cipher suite.
    fn hmac(&self, suite: CipherSuite, key: &[u8], data: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT>;

    /// HKDF-Extract for the given cipher suite.
    fn hkdf_extract(&self, suite: CipherSuite, salt: &[u8], ikm: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT>;

    /// HKDF-Expand for the given cipher suite.
    fn hkdf_expand(&self, suite: CipherSuite, prk: &[u8], info: &[u8], length: usize) -> Vec<u8, MAX_HASH_OUTPUT>;

    /// HKDF-Expand-Label (RFC 8446 §7.1).
    ///
    /// `label` must be the full label bytes including the `"tls13 "` prefix.
    fn hkdf_expand_label(
        &self,
        suite: CipherSuite,
        secret: &[u8],
        label: &[u8],
        context: &[u8],
        length: usize,
    ) -> Vec<u8, MAX_HASH_OUTPUT>;

    /// Compute the QUIC header protection mask (RFC 9001 §5.4).
    ///
    /// Given the header protection key and a 16-byte `sample` from the
    /// encrypted packet payload, returns a 16-byte mask to XOR with the
    /// first byte and packet number.
    ///
    /// Panics if the cipher suite is not supported.
    fn header_protection_mask(&self, suite: CipherSuite, hp_key: &[u8], sample: &[u8; 16]) -> Result<[u8; 16], Error>;
}

impl<T: CryptoProvider> CryptoProvider for alloc::sync::Arc<T> {
    fn supported_cipher_suites(&self) -> &[CipherSuite] {
        (**self).supported_cipher_suites()
    }

    fn supported_key_exchange_groups(&self) -> &[KeyExchangeGroup] {
        (**self).supported_key_exchange_groups()
    }

    fn supported_signature_schemes(&self) -> &[SignatureScheme] {
        (**self).supported_signature_schemes()
    }

    fn create_aead(&self, suite: CipherSuite, key: &[u8]) -> Result<Box<dyn Aead>, Error> {
        (**self).create_aead(suite, key)
    }

    fn create_kx_pair(&self, group: KeyExchangeGroup) -> Result<Box<dyn KeyExchangeKeyPair>, Error> {
        (**self).create_kx_pair(group)
    }

    fn create_signer(&self, scheme: SignatureScheme, secret_key: &[u8]) -> Result<Box<dyn Signer>, Error> {
        (**self).create_signer(scheme, secret_key)
    }

    fn verify_signature(
        &self,
        scheme: SignatureScheme,
        public_key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<(), Error> {
        (**self).verify_signature(scheme, public_key, data, signature)
    }

    fn hash(&self, suite: CipherSuite, data: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        (**self).hash(suite, data)
    }

    fn secure_random(&self, buf: &mut [u8]) {
        (**self).secure_random(buf)
    }

    fn hmac(&self, suite: CipherSuite, key: &[u8], data: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        (**self).hmac(suite, key, data)
    }

    fn hkdf_extract(&self, suite: CipherSuite, salt: &[u8], ikm: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        (**self).hkdf_extract(suite, salt, ikm)
    }

    fn hkdf_expand(&self, suite: CipherSuite, prk: &[u8], info: &[u8], length: usize) -> Vec<u8, MAX_HASH_OUTPUT> {
        (**self).hkdf_expand(suite, prk, info, length)
    }

    fn hkdf_expand_label(
        &self,
        suite: CipherSuite,
        secret: &[u8],
        label: &[u8],
        context: &[u8],
        length: usize,
    ) -> Vec<u8, MAX_HASH_OUTPUT> {
        (**self).hkdf_expand_label(suite, secret, label, context, length)
    }

    fn header_protection_mask(&self, suite: CipherSuite, hp_key: &[u8], sample: &[u8; 16]) -> Result<[u8; 16], Error> {
        (**self).header_protection_mask(suite, hp_key, sample)
    }
}

/// An AEAD cipher for encrypting / decrypting TLS records.
///
/// Implementations must clear internal key material on drop.
pub trait Aead: Send + Sync {
    /// Encrypt `plaintext` in place, appending the authentication tag.
    /// The buffer must have space for `plaintext.len() + tag_size()` bytes past the plaintext.
    fn encrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> Vec<u8, MAX_AEAD_TAG_SIZE>;

    /// Decrypt `ciphertext` in place.
    ///
    /// `buf` must contain `ciphertext || tag`. On success returns the plaintext
    /// length (i.e. `buf.len() - tag_size()`). The plaintext replaces the
    /// ciphertext in `buf[..returned_length]`.
    fn decrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> Result<usize, Error>;

    /// Size of the AEAD key in bytes.
    fn key_size(&self) -> usize;

    /// Size of the IV / nonce in bytes.
    fn nonce_size(&self) -> usize;

    /// Size of the authentication tag in bytes.
    fn tag_size(&self) -> usize;
}

/// A key-exchange key pair (can compute a shared secret with a peer's public key).
pub trait KeyExchangeKeyPair: Send + Sync {
    /// The group this key pair belongs to.
    fn group(&self) -> KeyExchangeGroup;

    /// Our public key bytes.
    fn public_key_bytes(&self) -> Vec<u8, MAX_KX_PUBLIC_KEY>;

    /// Derive the shared secret from the peer's public key.
    fn shared_secret(&self, peer_public_key: &[u8]) -> Result<Vec<u8, MAX_SHARED_SECRET>, Error>;

    /// Inject the peer's public key before [`public_key_bytes`](Self::public_key_bytes)
    /// is called.
    ///
    /// This is needed for KEM-based key exchange groups (such as
    /// X25519MLKEM768) where our key share depends on the peer's public
    /// key (the ML-KEM ciphertext is produced by encapsulating to the
    /// peer's key). For pure DH groups the default no-op implementation is
    /// sufficient.
    fn set_peer_public_key(&mut self, _peer_public_key: &[u8]) -> Result<(), Error> {
        Ok(())
    }
}

/// A signer for raw public key authentication.
pub trait Signer: Send + Sync {
    /// The signature scheme.
    fn scheme(&self) -> SignatureScheme;

    /// Raw public key bytes that will be placed in the Certificate message.
    fn public_key_bytes(&self) -> Vec<u8, MAX_PUBLIC_KEY_BYTES>;

    /// Sign the given data.
    fn sign(&self, data: &[u8]) -> Result<Vec<u8, MAX_SIGNATURE_SIZE>, Error>;
}

/// Cipher suites supported by a [`CryptoProvider`].
///
/// In this library the cipher suite prescribes the AEAD cipher, the hash
/// function used throughout the TLS 1.3 key schedule, and the transcript hash.
///
/// TLS 1.3 cipher suites always pair one AEAD with one hash:
///
/// | Suite                          | AEAD             | Hash   |
/// |--------------------------------|------------------|--------|
/// | `TLS_AES_128_GCM_SHA256`       | AES-128-GCM      | SHA256 |
/// | `TLS_AES_256_GCM_SHA384`       | AES-256-GCM      | SHA384 |
/// | `TLS_CHACHA20_POLY1305_SHA256` | ChaCha20-Poly1305| SHA256 |
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

    /// Size in bytes of the AEAD nonce for this suite.
    pub const fn nonce_size(self) -> usize {
        12
    }

    /// Size in bytes of the AEAD authentication tag.
    pub const fn tag_size(self) -> usize {
        16
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
    pub fn to_wire(self) -> [u8; 2] {
        match self {
            Self::X25519 => [0x00, 0x1D],
            Self::X25519MlKem768 => [0x11, 0xEC],
        }
    }

    /// Parse a key exchange group from its wire identifier.
    pub fn from_wire(bytes: [u8; 2]) -> Option<Self> {
        match bytes {
            [0x00, 0x1D] => Some(Self::X25519),
            [0x11, 0xEC] => Some(Self::X25519MlKem768),
            _ => None,
        }
    }

    /// Size of the client-side KeyShare entry for this group, in bytes.
    pub fn public_key_size_client(self) -> usize {
        match self {
            Self::X25519 => 32,
            Self::X25519MlKem768 => 1216,
        }
    }

    /// Size of the server-side KeyShare entry for this group, in bytes.
    ///
    /// For KEM-based groups the server returns a ciphertext (which may
    /// differ from the client's public key size).
    pub fn public_key_size_server(self) -> usize {
        match self {
            Self::X25519 => 32,
            Self::X25519MlKem768 => 1120,
        }
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
