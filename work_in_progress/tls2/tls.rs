//! Zero-allocation, `no_std` compatible TLS

#![cfg(feature = "std")]
extern crate alloc;

mod client;
mod crypto;
mod errors;
pub mod key_schedule;
mod message;
mod record;

#[cfg(feature = "crypto-default-provider")]
pub mod crypto_default_provider;

#[cfg(feature = "tokio")]
pub mod tokio;

// Re-exports from the public API

pub use client::*;
pub use crypto::*;
pub use errors::*;

/// The maximum size, in bytes, of a hash
pub const MAX_HASH_SIZE: usize = 48;
pub const PSK_MAX_SIZE: usize = MAX_HASH_SIZE;
pub const AEAD_MAX_KEY_SIZE: usize = 32;
/// The fixed size of AEAD tags
pub const AEAD_TAG_SIZE: usize = 16;
/// 32-byte X25519 seed + 64-byte ML-KEM 768 seed
pub const KEY_EXCHANGE_SECRET_KEY_MAX_SIZE: usize = 32 + 64;
/// The maximum size, in bytes, of a key exchange public key
pub const KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE: usize = 1216;
pub const KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE: usize = 64;
/// The maximum number of supported key exchange groups in ClientHello
pub const KEY_EXCHANGE_MAX_GROUPS: usize = 3;
/// The maximum size, in bytes, of a SPKI-encoded signing public key
pub const SIGNING_PUBLIC_KEY_MAX_SIZE: usize = 294;
pub const SIGNATURE_MAX_SIZE: usize = 256;
pub const ALPN_PROTOCOL_MAX_SIZE: usize = 32;

/// Maximum size of a fully framed TLS record (5-byte header + payload + padding + tag).
/// 5 + 16384 + 1 + 256 + 16 = 16662.
pub const MAX_RECORD_SIZE: usize = 16662;

/// Maximum number of certificates allowed in a chain.
pub const MAX_CERTS: usize = 10;
