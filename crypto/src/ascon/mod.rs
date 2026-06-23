//! # Ascon lightweight cryptography (NIST SP 800-232)
//!
//! Ascon is a family of authenticated encryption and hashing algorithms selected by NIST
//! for constrained environments. This module provides the four NIST-standardized,
//! little-endian variants:
//!
//! - [`AsconAead128`] — authenticated encryption with associated data (AEAD)
//! - [`AsconHash256`] — 256-bit cryptographic hash function
//! - [`AsconXof128`] — extensible-output function (XOF)
//! - [`AsconCxof128`] — customizable XOF (accepts a customization string)
//!
//! With optimized versions for both 32-bit and 64-bit CPUs.
//!
//! # Usage limits
//!
//! Per NIST SP 800-232 §4.3:
//! - Max data per key: 2^54 bytes
//! - Nonces must be distinct per key (up to 2^8 repetitions tolerated)
//! - Tag lengths below 64 bits are discouraged; below 32 bits are not allowed

mod ascon_aead128;
mod ascon_core;
mod ascon_cxof128;
mod ascon_hash256;
mod ascon_xof128;

pub use ascon_aead128::AsconAead128;
pub(crate) use ascon_core::{State, p8, p12};
pub use ascon_cxof128::AsconCxof128;
pub use ascon_hash256::AsconHash256;
pub use ascon_xof128::AsconXof128;
