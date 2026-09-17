//! ML-DSA post-quantum signatures standardized in FIPS 204.
//!
//! This module implements the ML-DSA-44, ML-DSA-65 and ML-DSA-87 parameter
//! sets through distinct key types:
//!
//! | Parameter set | Secret key | Public key | Public key size | Signature size |
//! | --- | --- | --- | --- | --- |
//! | ML-DSA-44 | [`MlDsa44SecretKey`] | [`MlDsa44PublicKey`] | 1312 B | 2420 B |
//! | ML-DSA-65 | [`MlDsa65SecretKey`] | [`MlDsa65PublicKey`] | 1952 B | 3309 B |
//! | ML-DSA-87 | [`MlDsa87SecretKey`] | [`MlDsa87PublicKey`] | 2592 B | 4627 B |
//!
//! # Signing
//!
//! Signing is stateful: build a secret key from a 32-byte seed with `new` (or
//! with `generate` for a fresh random key), then call `sign` (randomized) or
//! `sign_derand` (deterministic for a fixed nonce).
//!
//! The expanded key caches the NTT-domain matrix and secret vectors, so
//! repeated signatures skip key generation. It is a fixed-size value with no
//! heap allocation:
//!
//! ```
//! # use crypto::mldsa::MlDsa65SecretKey;
//! # let seed = [0u8; 32];
//! let key = MlDsa65SecretKey::new(&seed);
//! let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
//! assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
//! ```
//!
//! # Verification
//!
//! Verification is stateless: the public key returned by the secret key's
//! `public_key` method (or built from raw bytes with `from_bytes`)
//! exposes `verify` for a message and optional context, and
//! `verify_external_mu` for a precomputed 64-byte message representative
//! (FIPS 204 "external μ"). Both return [`MlDsaError`] on failure.

mod mldsa;
mod mldsa44;
mod mldsa65;
mod mldsa87;

pub use mldsa::MlDsaError;
pub use mldsa44::{
    ML_DSA_44_CONTEXT_MAX_LEN, ML_DSA_44_PUBLIC_KEY_SIZE, ML_DSA_44_SECRET_KEY_SIZE, ML_DSA_44_SEED_SIZE,
    ML_DSA_44_SIGNATURE_SIZE, MlDsa44PublicKey, MlDsa44SecretKey,
};
pub use mldsa65::{
    ML_DSA_65_CONTEXT_MAX_LEN, ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SECRET_KEY_SIZE, ML_DSA_65_SEED_SIZE,
    ML_DSA_65_SIGNATURE_SIZE, MlDsa65PublicKey, MlDsa65SecretKey,
};
pub use mldsa87::{
    ML_DSA_87_CONTEXT_MAX_LEN, ML_DSA_87_PUBLIC_KEY_SIZE, ML_DSA_87_SECRET_KEY_SIZE, ML_DSA_87_SEED_SIZE,
    ML_DSA_87_SIGNATURE_SIZE, MlDsa87PublicKey, MlDsa87SecretKey,
};

#[cfg(test)]
mod tests;
