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
//!
//! 2. **IO adapters** — [`io_std`] and [`io_tokio`] provide blocking and async
//!    I/O wrappers respectively, each gated behind their own feature flag.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
mod message;
mod record;

#[cfg(feature = "crypto-default-provider")]
pub mod crypto_default_provider;
#[cfg(feature = "webpki-validator")]
pub mod default_validator;

pub mod config;
pub mod connection;
pub mod crypto;
pub mod key_schedule;

#[cfg(feature = "std")]
pub mod io_std;
#[cfg(feature = "tokio")]
pub mod io_tokio;

pub mod quic;

#[cfg(feature = "std")]
pub use config::InMemorySessionTicketStore;
pub use config::{
    CertificateProvider, CertificateValidator, ClientCertificate, ClientConfig, ClientHello, ClientSessionCache, Clock,
    NoFingerprinter, ReceivedCertificate, ServerConfig, SessionTicketStore, TlsFingerprinter,
};
pub use connection::{ClientConnection, QuicHandshake, QuicHandshakeEvent, ServerConnection};
pub use crypto::{CertType, CipherSuite, KeyExchangeGroup, SignatureScheme};
#[cfg(feature = "webpki-validator")]
pub use default_validator::WebPkiValidator;
pub use error::{
    CertificateValidationFailure, CryptoFailure, Error, HandshakeFailure, InvalidKeyFailure, IoError, IoErrorKind,
};
pub use key_schedule::{KeySchedule, TlsKeys};
