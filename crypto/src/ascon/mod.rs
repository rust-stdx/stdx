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
//! With a state representation optimized for both 32-bit and 64-bit CPUs: the
//! 64-bit implementation is compiled on 64-bit targets and the 32-bit
//! bit-interleaved one everywhere else, behind the same API.
//!
//! # Usage limits
//!
//! Per NIST SP 800-232 §4.3:
//! - Max data per key: 2^54 bytes
//! - Nonces must be distinct per key (up to 2^8 repetitions tolerated)
//! - Tag lengths below 64 bits are discouraged; below 32 bits are not allowed

// Ascon permutation round constants (only the low byte of word 2 is touched).
// Shared by both the 32-bit and 64-bit implementations.
const RC4: u8 = 0xf0;
const RC5: u8 = 0xe1;
const RC6: u8 = 0xd2;
const RC7: u8 = 0xc3;
const RC8: u8 = 0xb4;
const RC9: u8 = 0xa5;
const RC10: u8 = 0x96;
const RC11: u8 = 0x87;
const RC12: u8 = 0x78;
const RC13: u8 = 0x69;
const RC14: u8 = 0x5a;
const RC15: u8 = 0x4b;

#[cfg(target_pointer_width = "32")]
mod ascon_32b;
#[cfg(not(target_pointer_width = "32"))]
mod ascon_64b;

mod ascon_aead128;
mod ascon_cxof128;
mod ascon_hash256;
mod ascon_xof128;

#[cfg(target_pointer_width = "32")]
pub(crate) use ascon_32b::{State, p8, p12};
#[cfg(not(target_pointer_width = "32"))]
pub(crate) use ascon_64b::{State, p8, p12};
pub use ascon_aead128::AsconAead128;
pub use ascon_cxof128::AsconCxof128;
pub use ascon_hash256::AsconHash256;
pub use ascon_xof128::AsconXof128;
