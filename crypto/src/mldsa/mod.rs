//! ML-DSA post-quantum signatures standardized in FIPS 204.
//!
//! This module implements the ML-DSA-44, ML-DSA-65 and ML-DSA-87 parameter
//! sets through distinct key types:
//!
//! | Parameter set | Signing key | Verifying key | Public key | Signature |
//! | --- | --- | --- | --- | --- |
//! | ML-DSA-44 | [`MlDsa44SigningKey`] | [`MlDsa44VerifyingKey`] | 1312 B | 2420 B |
//! | ML-DSA-65 | [`MlDsa65SigningKey`] | [`MlDsa65VerifyingKey`] | 1952 B | 3309 B |
//! | ML-DSA-87 | [`MlDsa87SigningKey`] | [`MlDsa87VerifyingKey`] | 2592 B | 4627 B |
//!
//! # Signing
//!
//! Signing is stateful: create a signing key, initialize it once from a 32-byte
//! seed with `init` (or `generate` for a fresh random key), then call `sign`
//! (randomized) or `sign_derand` (deterministic for a fixed nonce).
//!
//! The expanded key caches the NTT-domain matrix and secret vectors, so
//! repeated signatures skip key generation. It is a fixed-size value with no
//! heap allocation, so it can be placed in a `static` on embedded targets:
//!
//! ```
//! # use crypto::mldsa::MlDsa65SigningKey;
//! # let seed = [0u8; 32];
//! let mut key = MlDsa65SigningKey::new();
//! key.init(&seed);
//! let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
//! assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
//! ```
//!
//! # Verification
//!
//! Verification is stateless: the verifying key returned by
//! `SigningKey::public_key` (or built from raw bytes with `from_bytes`)
//! exposes `verify` for a message and optional context, and
//! `verify_external_mu` for a precomputed 64-byte message representative
//! (FIPS 204 "external μ"). Both return [`MlDsaError`] on failure.

mod mldsa;
mod mldsa44;
mod mldsa65;
mod mldsa87;

pub use mldsa::MlDsaError;
pub use mldsa44::{
    ML_DSA_44_CONTEXT_MAX_LEN, ML_DSA_44_PUBLIC_KEY_SIZE, ML_DSA_44_SEED_SIZE, ML_DSA_44_SIGNATURE_SIZE,
    ML_DSA_44_SIGNING_KEY_SIZE, MlDsa44SigningKey, MlDsa44VerifyingKey,
};
pub use mldsa65::{
    ML_DSA_65_CONTEXT_MAX_LEN, ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SEED_SIZE, ML_DSA_65_SIGNATURE_SIZE,
    ML_DSA_65_SIGNING_KEY_SIZE, MlDsa65SigningKey, MlDsa65VerifyingKey,
};
pub use mldsa87::{
    ML_DSA_87_CONTEXT_MAX_LEN, ML_DSA_87_PUBLIC_KEY_SIZE, ML_DSA_87_SEED_SIZE, ML_DSA_87_SIGNATURE_SIZE,
    ML_DSA_87_SIGNING_KEY_SIZE, MlDsa87SigningKey, MlDsa87VerifyingKey,
};

#[cfg(test)]
mod tests;
