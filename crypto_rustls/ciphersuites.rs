//! TLS 1.3 cipher suites for this provider.
//!
//! The three mandatory TLS 1.3 suites are offered, in priority order:
//! AES-256-GCM, ChaCha20-Poly1305 and AES-128-GCM.

use rustls::{
    CipherSuite, SupportedCipherSuite, Tls13CipherSuite,
    crypto::{CipherSuiteCommon, tls13::HkdfUsingHmac},
};

use crate::{aead, hash, hmac, quic};

/// All cipher suites supported by this provider, in preference order.
pub(crate) static ALL_CIPHER_SUITES: &[SupportedCipherSuite] = &[
    TLS13_AES_256_GCM_SHA384,
    TLS13_CHACHA20_POLY1305_SHA256,
    TLS13_AES_128_GCM_SHA256,
];

/// `TLS_AES_128_GCM_SHA256`.
pub(crate) static TLS13_AES_128_GCM_SHA256: SupportedCipherSuite = SupportedCipherSuite::Tls13(&Tls13CipherSuite {
    common: CipherSuiteCommon {
        suite: CipherSuite::TLS13_AES_128_GCM_SHA256,
        hash_provider: &hash::Sha256,
        // ref: draft-irtf-cfrg-aead-limits-08, table 1.
        confidentiality_limit: 1 << 24,
    },
    hkdf_provider: &HkdfUsingHmac(&hmac::Sha256Hmac),
    aead_alg: &aead::Aes128Gcm,
    quic: Some(&quic::AES_128_GCM),
});

/// `TLS_AES_256_GCM_SHA384`.
pub(crate) static TLS13_AES_256_GCM_SHA384: SupportedCipherSuite = SupportedCipherSuite::Tls13(&Tls13CipherSuite {
    common: CipherSuiteCommon {
        suite: CipherSuite::TLS13_AES_256_GCM_SHA384,
        hash_provider: &hash::Sha384,
        confidentiality_limit: 1 << 24,
    },
    hkdf_provider: &HkdfUsingHmac(&hmac::Sha384Hmac),
    aead_alg: &aead::Aes256Gcm,
    quic: Some(&quic::AES_256_GCM),
});

/// `TLS_CHACHA20_POLY1305_SHA256`.
pub(crate) static TLS13_CHACHA20_POLY1305_SHA256: SupportedCipherSuite =
    SupportedCipherSuite::Tls13(&Tls13CipherSuite {
        common: CipherSuiteCommon {
            suite: CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
            hash_provider: &hash::Sha256,
            // ref: draft-irtf-cfrg-aead-limits-08, section 5.2.1.
            confidentiality_limit: u64::MAX,
        },
        hkdf_provider: &HkdfUsingHmac(&hmac::Sha256Hmac),
        aead_alg: &aead::ChaCha20Poly1305,
        quic: Some(&quic::CHACHA20_POLY1305),
    });
