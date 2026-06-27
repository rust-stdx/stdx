//! A pure-Rust QUIC (RFC 9000) library built on the stdx workspace.
//!
//! Provides:
//! - Transport trait for pluggable UDP (or any datagram) transport
//! - Connection management (handshake, streams, datagrams)
//! - TLS 1.3 integration via the stdx `tls` crate
//! - Congestion control via `CongestionController` trait
//!
//! # Crate structure
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`transport`] | Transport trait for pluggable datagram I/O |
//! | [`config`] | Connection configuration |
//! | [`error`] | Error types |
//! | [`varint`] | QUIC variable-length integer encoding |
//! | [`cid`] | Connection ID types |
//! | [`crypto`] | QUIC key derivation, header/packet protection |
//! | [`packet`] | Packet header parsing and building |
//! | [`frame`] | Frame encoding and decoding |
//! | [`tls`] | TLS 1.3 handshake adapter for QUIC |
//! | [`connection`] | Main `Connection` struct |

pub mod config;
pub mod connection;
pub mod crypto_keys;
pub mod error;
pub mod frame;
pub mod packet;
pub mod transport;
pub mod transport_params;
pub mod varint;

#[cfg(feature = "std")]
pub mod ack;
#[cfg(feature = "std")]
pub mod cid;
#[cfg(feature = "std")]
pub mod congestion;
#[cfg(feature = "std")]
pub mod loss;
#[cfg(feature = "std")]
pub mod stream;
#[cfg(feature = "std")]
pub mod tls_adapter;

pub use config::Config;
pub use connection::Connection;
pub use error::Error;
pub use transport::Transport;
