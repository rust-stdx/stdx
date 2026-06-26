//! A TLS 1.3 library with sans-IO design and pluggable crypto providers.
//!
//! # Architecture
//!
//! The library is split into two layers:
//!
//! 1. **Sans-IO core** — [`ClientConnection`] and [`ServerConnection`] are pure
//!    async state machines. They take bytes in, produce bytes out, with no I/O
//!    or runtime dependency. Configured via [`ClientConfig`] / [`ServerConfig`].
//!    The state machine is async because certificate provisioning, validation,
//!    and fingerprinting may require I/O (database fetch, OCSP check, etc.).
//!    For sync use cases a trivial executor like
//!    [`futures::executor::block_on`] suffices.
//!
//! 2. **IO adapter** — wraps a connection over
//!    [`AsyncRead`][futures_util::io::AsyncRead] +
//!    [`AsyncWrite`][futures_util::io::AsyncWrite] (via `futures-util`).
//!
//! # Quick start (default provider)
//!
//! ```ignore
//! use tls::{ClientConfig, ClientConnection, ServerConfig, ServerConnection};
//! use tls::config::{CertificateProvider, ProvidedCertificate, CertificateValidator};
//! use tls::crypto_default_provider::DefaultCryptoProvider;
//!
//! let provider = DefaultCryptoProvider::new();
//!
//! // Client
//! let mut client = ClientConnection::new(
//!     ClientConfig::new(provider, vec![], my_validator),
//!     Some("example.com".into()),
//! )?;
//!
//! // Server
//! let mut server = ServerConnection::new(
//!     ServerConfig::new(provider, vec![], my_cert_provider),
//! );
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
mod message;
mod record;

#[cfg(feature = "crypto-default-provider")]
pub mod crypto_default_provider;
#[cfg(feature = "webpki-validator")]
pub mod default_validator;
#[cfg(feature = "std")]
pub mod io;

pub mod config;
pub mod connection;
pub mod crypto;
pub mod key_schedule;

#[cfg(feature = "quic")]
pub mod quic;

pub use config::{
    CertificateProvider, CertificateValidator, ClientConfig, ClientHello, NoFingerprinter, ReceivedCertificate,
    ServerConfig, TlsFingerprinter,
};
pub use connection::{ClientConnection, ServerConnection};
pub use crypto::{CertType, CipherSuite, KeyExchangeGroup, SignatureScheme};
#[cfg(feature = "webpki-validator")]
pub use default_validator::WebPkiValidator;
pub use error::Error;
pub use key_schedule::{KeySchedule, TlsKeys};
