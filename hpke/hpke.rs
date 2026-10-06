//! HPKE: Hybrid Public Key Encryption, as specified in RFC 9180.
//!
//! HPKE provides public-key encryption of arbitrary-sized plaintexts for a
//! recipient public key. It also includes three authenticated variants, one
//! which authenticates possession of a pre-shared key (PSK), and two
//! additional ones which authenticate possession of a KEM private key.
//!
//! A ciphersuite is a triple `(KEM, KDF, AEAD)` selected at compile time via
//! type parameters. The crate ships the RFC 9180 registered algorithms plus
//! the X-Wing hybrid post-quantum KEM:
//!
//! - KEMs: [`kem::X25519HkdfSha256`], [`kem::P256HkdfSha256`],
//!   [`kem::P521HkdfSha512`], [`kem::XWing`]
//! - KDFs: [`kdf::HkdfSha256`], [`kdf::HkdfSha512`]
//! - AEADs: [`crypto::aes::Aes256Gcm`], [`crypto::chacha::ChaCha20Poly1305`],
//!   [`aead::ExportOnly`]
//!
//! Any of the three roles can be replaced by a custom implementation: define a
//! type and implement [`kem::Kem`], [`kdf::Kdf`] or [`aead::Aead`] for it.
//!
//! # Example
//!
//! One-shot encryption with the X25519 + HKDF-SHA256 + AES-256-GCM suite:
//!
//! ```
//! use crypto::aes::Aes256Gcm;
//! use hpke::{
//!     self, ModeReceiver, ModeSender, kdf::HkdfSha256, kem::Kem, kem::X25519HkdfSha256,
//! };
//!
//! let (bob_sk, bob_pk) = X25519HkdfSha256::generate_keypair().unwrap();
//!
//! let info = b"Alice and Bob's weekly chat";
//! let (enc, mut sender) = hpke::new_sender::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
//!     &ModeSender::Base, &bob_pk, info,
//! ).unwrap();
//! let ciphertext = sender.seal(b"fronthand or backhand?", b"a gentleman's game").unwrap();
//!
//! let mut receiver = hpke::new_receiver::<X25519HkdfSha256, HkdfSha256, Aes256Gcm>(
//!     &ModeReceiver::Base, &bob_sk, &enc, info,
//! ).unwrap();
//! let plaintext = receiver.open(&ciphertext, b"a gentleman's game").unwrap();
//! assert_eq!(plaintext, b"fronthand or backhand?");
//! ```
//!
//! # Security
//!
//! - Messages **must** be sealed and opened in the same order. There is no
//!   tolerance for reordering or loss; per-message context (e.g. a counter)
//!   can be authenticated by passing it as `aad`.
//! - Never call `seal` twice with the same nonce: this is catastrophic for the
//!   confidentiality of all messages sent under a context. HPKE prevents this
//!   internally by deriving the per-message nonce from a sequence number.
//! - The Auth and AuthPSK modes of DHKEM-based suites are vulnerable to key
//!   compromise impersonation (KCI): if the recipient's secret key leaks, an
//!   attacker can forge messages appearing to come from any sender. Sign the
//!   `(enc, ciphertext)` tuple if sender authenticity must hold after a
//!   recipient key compromise.
//! - PSKs must contain at least 32 bytes of entropy. Low-entropy PSKs are
//!   vulnerable to dictionary attacks.
//! - There is no forward secrecy against recipient static key compromise:
//!   anyone with `skR` can decrypt all past captures.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod aead;
mod context;
pub mod kdf;
pub mod kem;

#[cfg(all(test, feature = "alloc", feature = "random"))]
mod tests;

#[cfg(feature = "random")]
pub use context::new_sender;
pub use context::{ModeReceiver, ModeSender, ReceiverContext, SenderContext, new_receiver};

/// Best-effort wipe of transient secret buffers.
///
/// This is a no-op unless the `zeroize` feature is enabled.
#[inline(always)]
pub(crate) fn wipe(_bytes: &mut [u8]) {
    #[cfg(feature = "zeroize")]
    {
        use zeroize::Zeroize;
        _bytes.zeroize();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Errors

/// Errors that can occur during HPKE setup, key encapsulation, key derivation
/// or message encryption/decryption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HpkeError {
    /// A KEM input or output failed validation (e.g. an invalid or low-order
    /// public key was provided), or a PSK-mode input is missing (an empty PSK
    /// or PSK identifier).
    ValidationError,
    /// A key, encapsulated key or ciphertext could not be deserialized
    /// (e.g. wrong length or malformed encoding).
    DeserializeError,
    /// The encapsulation step failed (e.g. the recipient's public key is
    /// invalid).
    EncapError,
    /// The decapsulation step failed (e.g. the encapsulated key is invalid,
    /// or the DH output was rejected).
    DecapError,
    /// The AEAD failed to decrypt the ciphertext (wrong key, tampered
    /// ciphertext or wrong associated data).
    OpenError,
    /// Deriving a key pair from input keying material failed after
    /// exhausting all rejection-sampling candidates.
    DeriveKeyPairError,
    /// The per-context message limit was reached; no more messages can be
    /// sealed or opened with this context.
    MessageLimitReached,
    /// The operation is not supported by this KEM, KDF or AEAD (e.g. an
    /// authenticated mode with a KEM that does not support it, or sealing a
    /// message with the Export-Only pseudo-AEAD).
    NotSupported,
    /// The pseudorandom key given to the KDF expand step does not have the
    /// expected length.
    InvalidPrk,
    /// The KDF was asked for more output than it can produce (`> 255 * Nh`).
    KdfOutputTooLong,
    /// An underlying AEAD error.
    Aead(crypto::AeadError),
    /// An underlying elliptic-curve error.
    EllipticCurve(crypto::EllipticCurveError),
    /// An underlying ML-KEM error.
    MlKem(crypto::mlkem::MlKemError),
    /// The operating system's random number generator failed.
    Random(crypto::RandomError),
    /// Something went wrong, without further details.
    Unspecified,
}

impl core::fmt::Display for HpkeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HpkeError::ValidationError => write!(f, "key validation failed"),
            HpkeError::DeserializeError => write!(f, "key or ciphertext deserialization failed"),
            HpkeError::EncapError => write!(f, "encapsulation failed"),
            HpkeError::DecapError => write!(f, "decapsulation failed"),
            HpkeError::OpenError => write!(f, "decryption failed"),
            HpkeError::DeriveKeyPairError => write!(f, "key pair derivation failed"),
            HpkeError::MessageLimitReached => write!(f, "message limit reached"),
            HpkeError::NotSupported => write!(f, "operation is not supported"),
            HpkeError::InvalidPrk => write!(f, "PRK has an invalid length"),
            HpkeError::KdfOutputTooLong => write!(f, "KDF output length exceeds limit (255 * Nh)"),
            HpkeError::Aead(err) => write!(f, "{err}"),
            HpkeError::EllipticCurve(err) => write!(f, "{err}"),
            HpkeError::MlKem(err) => write!(f, "{err}"),
            HpkeError::Random(err) => write!(f, "{err}"),
            HpkeError::Unspecified => write!(f, "unknown error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HpkeError {}

impl From<crypto::AeadError> for HpkeError {
    fn from(err: crypto::AeadError) -> Self {
        HpkeError::Aead(err)
    }
}

impl From<crypto::HkdfError> for HpkeError {
    fn from(err: crypto::HkdfError) -> Self {
        match err {
            crypto::HkdfError::PrkIsTooShort(_) => HpkeError::InvalidPrk,
            crypto::HkdfError::OutputIsTooLong => HpkeError::KdfOutputTooLong,
        }
    }
}

impl From<crypto::EllipticCurveError> for HpkeError {
    fn from(err: crypto::EllipticCurveError) -> Self {
        HpkeError::EllipticCurve(err)
    }
}

impl From<crypto::mlkem::MlKemError> for HpkeError {
    fn from(err: crypto::mlkem::MlKemError) -> Self {
        HpkeError::MlKem(err)
    }
}

impl From<crypto::RandomError> for HpkeError {
    fn from(err: crypto::RandomError) -> Self {
        HpkeError::Random(err)
    }
}

impl From<crypto::xwing::XWingError> for HpkeError {
    fn from(err: crypto::xwing::XWingError) -> Self {
        match err {
            crypto::xwing::XWingError::MlKem(err) => HpkeError::MlKem(err),
            crypto::xwing::XWingError::Random(err) => HpkeError::Random(err),
        }
    }
}
