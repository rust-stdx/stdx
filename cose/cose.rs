//! CBOR Object Signing and Encryption (COSE) structures and processing.
//!
//! This crate implements the signing and MAC half of COSE (RFC 9052/RFC 9053):
//!
//! - [`Sign1`] — COSE_Sign1
//! - [`Sign`] — COSE_Sign (multiple signers)
//! - [`Mac0`] — COSE_Mac0
//! - [`Mac`] — COSE_Mac (direct recipients only)
//! - [`CoseKey`] / [`CoseKeySet`] — COSE_Key / COSE_KeySet
//!
//! Encryption, AES, ECDH, and HKDF are intentionally out of scope. Numeric
//! identifiers for those algorithms are preserved as [`Algorithm::Unknown`] so
//! that keys and private-use messages round-trip unchanged.
//!
//! # Allocation
//!
//! The core API works without an allocator:
//!
//! - Parsing returns values that borrow the input bytes.
//! - Headers and keys are built into caller-provided buffers with
//!   `build_*_into` / `build_into`.
//! - Signing and MACing write into caller-provided buffers with `sign_into`,
//!   `sign_into_with_signature`, and `mac_into`.
//! - Verification uses caller-provided scratch space with
//!   `verify_with_scratch`.
//!
//! With the `alloc` feature, the crate additionally provides `Vec`-returning
//! convenience methods, owned header/key builders, and conversions from
//! `crypto` key types.
//!
//! # Limits and security policy
//!
//! Untrusted input is bounded:
//!
//! - header and COSE_Key maps may contain at most 32 entries;
//! - opaque CBOR values may nest at most 32 levels;
//! - COSE_Sign may contain at most 64 signatures;
//! - COSE_Mac may contain at most 64 recipients;
//! - COSE_KeySet may contain at most 1024 keys;
//! - key-set verification tries at most [`DEFAULT_MAX_KEY_CANDIDATES`] keys by
//!   default, and `Sign::verify_any` tries at most
//!   [`DEFAULT_MAX_SIGNATURE_ATTEMPTS`] signatures. These budgets can be
//!   changed through [`VerifyOptions`].
//!
//! [`VerifyOptions::require_protected_alg`] defaults to `false` for
//! interoperability. New applications should set it to `true` and should use
//! [`VerifyOptions::allowed_algorithms`] to pin the accepted algorithms.
//!
//! COSE_Mac direct-recipient verification is strict: exactly one recipient is
//! accepted, its `alg` must be `direct`, its protected header must be empty,
//! its ciphertext must be the empty byte string, and it must not contain
//! nested recipients.
//!
//! With the `zeroize` feature, owned header and key buffers are overwritten
//! before their allocation is released. Buffers supplied by the caller are
//! never overwritten by this crate.
//!
//! Only integer algorithm identifiers are supported. Text `alg` values,
//! countersignatures, x5chain, CWT, encryption, ECDH, and HKDF are outside the
//! scope of this crate; unknown algorithms and parameters are preserved so
//! that keys and messages still round-trip.
//!
//! # Features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `std` | yes | Standard library, randomness, and `alloc`. |
//! | `alloc` | via `std` | `Vec`-returning conveniences, owned builders, and key conversions. |
//! | `random` | via `std` | ML-DSA signing and key generation in `crypto`. |
//! | `zeroize` | yes | Zeroize owned key/header buffers on drop. |
//!
//! # Crypto key types
//!
//! Like the `jwt` crate, this crate implements [`Signer`], [`Verifier`], and
//! [`Macer`] directly for the stdx `crypto` key types. The `crypto` crate is
//! **not** re-exported; users import it themselves.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "alloc")] {
//! use cose::{Algorithm, Header, Sign1, Sign1Builder, VerifyOptions};
//! use crypto::curve25519::ed25519;
//!
//! let secret = ed25519::SecretKey::from_bytes(&[7u8; 32]);
//! let public = secret.public_key();
//!
//! let protected = Header::builder()
//!     .algorithm(Algorithm::Ed25519)
//!     .key_id(b"key-1")
//!     .build_protected()
//!     .unwrap();
//!
//! let token = Sign1Builder::new()
//!     .protected(protected)
//!     .payload(b"hello world")
//!     .sign(&secret)
//!     .unwrap();
//!
//! let message = Sign1::from_slice(&token).unwrap();
//! message
//!     .verify(&public, b"", &VerifyOptions::default())
//!     .unwrap();
//! # }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod algorithm;
mod bytes;
mod codec;
mod error;
mod header;
mod impls;
mod key;
mod mac;
mod sign;

// Re-export cbor so applications can decode private header/key parameters
// without risking a version mismatch. The cryptographic crate is deliberately
// not re-exported.
pub use ::cbor;
pub use algorithm::{Algorithm, ContentType, Curve, KeyOp, KeyType, Label};
pub use error::{Error, Result};
pub use header::{Crit, Header, HeaderBuilder, HeaderIter, HeaderRole, HeaderValue, Headers};
pub use impls::{RsaPublicKey, SymmetricKey};
#[cfg(feature = "alloc")]
pub use key::KeySetBuilder;
pub use key::{
    AkpKey, CoseKey, CoseKeySet, CoseKeySetIter, Ec2Key, KeyBuilder, KeyIter, KeyOps, OkpKey, RsaKey, YCoordinate,
};
pub use mac::{Mac, Mac0, Mac0Builder, MacBuilder, Macer, Recipient, Recipients};
#[cfg(feature = "alloc")]
pub use sign::SignBuilder;
pub use sign::{Sign, Sign1, Sign1Builder, SignatureSlot, SignatureView, Signatures, Signer, Verifier};

/// The default maximum number of key candidates tried by one verification.
///
/// Each candidate costs one cryptographic verification. A message with a
/// `kid` normally narrows the set to a single key; this budget only limits the
/// worst case when the message does not identify a key.
pub const DEFAULT_MAX_KEY_CANDIDATES: usize = 8;

/// The default maximum number of signatures tried by one verification.
pub const DEFAULT_MAX_SIGNATURE_ATTEMPTS: usize = 8;

/// Policy options for signature and MAC verification.
pub struct VerifyOptions<'a> {
    /// Additional critical header labels understood by the application.
    ///
    /// Labels 1 through 6 (`alg`, `crit`, `content type`, `kid`, `IV`,
    /// `Partial IV`) are understood by this crate and need not be listed.
    pub understood_header_labels: &'a [Label<'a>],

    /// If present, the message algorithm must be one of these exact values.
    pub allowed_algorithms: Option<&'a [Algorithm]>,

    /// Require the message algorithm to be in the protected header.
    ///
    /// The default is `false` for interoperability. New applications should
    /// normally set this to `true`.
    pub require_protected_alg: bool,

    /// Maximum number of key candidates tried by a key-set verification.
    ///
    /// Defaults to [`DEFAULT_MAX_KEY_CANDIDATES`]. Set to `None` to try every
    /// matching key. Only actual cryptographic attempts count against the
    /// budget; keys skipped because of a `kid` or algorithm mismatch do not.
    pub max_key_candidates: Option<usize>,

    /// Maximum number of signatures tried by
    /// [`Sign::verify_any`](crate::Sign::verify_any) and its scratch variant.
    ///
    /// Defaults to [`DEFAULT_MAX_SIGNATURE_ATTEMPTS`]. Set to `None` to try
    /// every signature.
    pub max_signature_attempts: Option<usize>,
}

impl Default for VerifyOptions<'_> {
    fn default() -> Self {
        Self {
            understood_header_labels: &[],
            allowed_algorithms: None,
            require_protected_alg: false,
            max_key_candidates: Some(DEFAULT_MAX_KEY_CANDIDATES),
            max_signature_attempts: Some(DEFAULT_MAX_SIGNATURE_ATTEMPTS),
        }
    }
}
