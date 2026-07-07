//! The SHA-2 family of hashing functions.
//!
//! # ⚠️ Warning
//!
//! SHA-2 function are vulnerable to length-extension attacks. Unless mandated, you may prefer `BLAKE3`
//! or the (slower) SHA-3 functions.

mod sha256;
mod sha384;
mod sha512;

#[cfg(target_arch = "x86_64")]
mod sha256_amd64;
#[cfg(target_arch = "aarch64")]
mod sha256_arm64;
#[cfg(target_arch = "x86_64")]
mod sha512_amd64;
#[cfg(target_arch = "aarch64")]
mod sha512_arm64;

pub use sha256::Sha256;
pub use sha384::Sha384;
pub use sha512::Sha512;
