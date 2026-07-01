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

/// The maximum size, in bytes, of a hash
pub const MAX_HASH_SIZE: usize = 48;
pub const AEAD_MAX_KEY_SIZE: usize = 32;
pub const AEAD_MAX_TAG_SIZE: usize = 16;
/// The maximum size, in bytes, of a key exchange public key
pub const KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE: usize = 1216;
pub const SHARED_SECRET_MAX_SIZE: usize = 64;
/// The maximum size, in bytes, of a SPKI-encoded signing public key
pub const SIGNING_PUBLIC_KEY_MAX_SIZE: usize = 97;
pub const MAX_SIGNATURE_SIZE: usize = 256;
pub const MAX_SESSION_ID: usize = 32;
pub const MAX_CERT_TYPES: usize = 4;
pub const MAX_KEY_EXCHANGE_PAIRS: usize = 6;
pub const PSK_MAX_SIZE: usize = MAX_HASH_SIZE;
/// Maximum size of a NewSessionTicket nonce (RFC 8446: opaque ticket_nonce<0..255>)
pub const TICKET_NONCE_MAX_SIZE: usize = 255;
/// Maximum size of a ticket. Larger tickets are ignored.
pub const TICKET_MAX_SIZE: usize = 64;

/// The maxmimum number of ALPN protocols support by clients / servers
pub const MAX_ALPN_PROTOCOLS: usize = 8;
pub const ALPN_PROTOCOL_MAX_SIZE: usize = 32;
/// The maximum length of a valid server name, in bytes
pub const MAX_SERVER_NAME_LENGTH: usize = 256;

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
pub use connection::{AlpnProtocol, ClientConnection, QuicHandshake, QuicHandshakeEvent, ServerConnection};
pub use crypto::{CertType, CipherSuite, KeyExchangeGroup, SignatureScheme};
#[cfg(feature = "webpki-validator")]
pub use default_validator::WebPkiValidator;
pub use error::{
    CertificateValidationFailure, CryptoFailure, Error, HandshakeFailure, InvalidKeyFailure, IoError, IoErrorKind,
};
pub use key_schedule::{KeySchedule, TlsKeys};
