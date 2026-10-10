//! PostgreSQL wire protocol: message encoding/decoding, type OIDs, and the
//! authentication state machine.
//!
//! This crate is intentionally free of any async runtime. It operates on
//! [`bytes::BytesMut`] buffers and is shared between the `postgresql` client
//! and the compile-time query macros, so both understand the protocol from a
//! single implementation.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
// Length and count fields are untrusted on both directions; a truncating or
// sign-changing cast is a protocol-smuggling bug so the whole crate must be free of them.
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

pub mod backend;
pub mod error;
pub mod frontend;
pub mod oid;
pub mod scram;

pub use backend::{BackendMessage, DEFAULT_MAX_MESSAGE_LEN, FieldDescription, decode, decode_with_limit};
pub use error::{DbError, Error};
pub use oid::{Format, Oid, PgType};
pub use scram::ScramClient;
