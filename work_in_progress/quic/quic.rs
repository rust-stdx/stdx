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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod ack;
pub mod cid;
pub mod cmd_queue;
pub mod config;
pub mod congestion;
pub mod connection;
pub mod crypto_keys;
pub mod error;
pub mod frame;
pub mod instant;
pub mod loss;
pub mod packet;
pub mod server;
pub mod stream;
pub mod transport;
pub mod transport_params;
pub mod varint;

pub mod tls_adapter;

pub use config::Config;
pub use connection::Connection;
pub use error::{Error, IoError};
pub use instant::Instant;
pub use stream::{ReceiveStream, SendStream};
pub use transport::Transport;
