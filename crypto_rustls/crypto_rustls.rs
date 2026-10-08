//! A [`rustls`](https://github.com/rustls/rustls) [`CryptoProvider`] backed by
//! the stdx [`crypto`] crate.
//!
//! This crate lets rustls use the pure-Rust, `no_std`-friendly primitives from
//! [`crypto`] instead of rustls' built-in `aws-lc-rs` or `ring` backends.
//! Rustls is configured with `default-features = false`, so no C or
//! assembly-backed cryptography is compiled in.
//!
//! # Supported algorithms
//!
//! * TLS 1.3 cipher suites: `TLS_AES_128_GCM_SHA256`,
//!   `TLS_AES_256_GCM_SHA384`, `TLS_CHACHA20_POLY1305_SHA256`.
//! * Key exchange: `X25519MLKEM768` (post-quantum hybrid, preferred),
//!   `X25519`, `secp256r1` and `secp384r1`.
//! * Signature verification: ECDSA P-256/P-384, Ed25519, RSA PKCS#1 v1.5 and
//!   PSS, and ML-DSA-44/65/87 (FIPS 204).
//! * Signing (for serving TLS or client certificates): ECDSA P-256/P-384,
//!   Ed25519 and ML-DSA-44/65/87. RSA private keys are not supported.
//! * QUIC (RFC 9001) packet protection for all three cipher suites.
//!
//! ML-DSA is supported in the "pure" mode required by RFC 9881 and the TLS 1.3
//! ML-DSA profile (empty FIPS 204 context). Private keys must be in the RFC
//! 9881 seed form; keys stored only as an expanded key are rejected because the
//! seed cannot be recovered from them.
//!
//! TLS 1.2 is not supported.
//!
//! # Usage
//!
//! Install this provider as the process default before building any rustls
//! configuration:
//!
//! ```ignore
//! crypto_rustls::install_default_rustls_provider();
//! ```
//!
//! Or pass [`default_provider`] explicitly to
//! `ClientConfig::builder_with_provider()` /
//! `ServerConfig::builder_with_provider()`.

mod aead;
mod ciphersuites;
mod hash;
mod hmac;
mod hybrid_key_exchange;
mod key_exchange;
mod quic;
mod random;
mod sign;
mod verify;

use rustls::crypto::CryptoProvider;

/// Returns this crate's [`CryptoProvider`].
///
/// Install it with [`CryptoProvider::install_default`], or pass it to
/// `ClientConfig::builder_with_provider()` /
/// `ServerConfig::builder_with_provider()`.
///
/// The provider offers only TLS 1.3, prefers the `X25519MLKEM768`
/// post-quantum hybrid key exchange, and supports QUIC.
pub fn default_provider() -> CryptoProvider {
    CryptoProvider {
        cipher_suites: ciphersuites::ALL_CIPHER_SUITES.to_vec(),
        kx_groups: key_exchange::KX_GROUPS.to_vec(),
        signature_verification_algorithms: verify::ALGORITHMS,
        secure_random: &random::RngProvider,
        key_provider: &sign::Provider,
    }
}

/// Installs [`default_provider`] as the process-default rustls
/// [`CryptoProvider`].
///
/// This is the intended entry point when `rustls` (or `reqwest`) is compiled
/// without a built-in cryptography provider, such as when using reqwest's
/// `rustls-no-provider` feature. Call it once before building any rustls
/// configuration.
///
/// The call is safe to repeat and never panics: the provider is installed only
/// on the first call, and if a provider is already installed as the process
/// default, that provider is left untouched.
pub fn install_default_rustls_provider() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = default_provider().install_default();
    });
}
