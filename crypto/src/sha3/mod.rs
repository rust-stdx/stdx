//! The SHA-3 family of hashing functions (FIPS 202, NIST SP 800-185 and RFC
//! 9861).

pub mod keccak;
mod kmac;
mod parallel_hash;
mod sha3_256;
mod sha3_512;
mod shake128;
mod shake256;
mod turboshake;

#[cfg(target_arch = "aarch64")]
mod keccak_arm64;

pub use kmac::Kmac256;
pub use parallel_hash::{ParallelHash256, ParallelHashReader};
pub use sha3_256::Sha3_256;
pub use sha3_512::Sha3_512;
pub use shake128::Shake128;
pub use shake256::{CShake256, Shake256};
pub use turboshake::{TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR, TurboShake256};
