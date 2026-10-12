//! Encoding primitives used by the message builder.
//!
//! Base64 comes from the [`base64`] crate; quoted-printable, RFC 2047 word
//! payloads and RFC 2231 percent-encoding are implemented here in safe Rust.
//! No module in this crate uses `unsafe`.

pub(crate) mod base64;
pub(crate) mod encode;
pub(crate) mod hex;
pub(crate) mod qp;
pub(crate) mod rfc2047;
pub(crate) mod swar;
