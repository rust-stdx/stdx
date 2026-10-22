//! The SHA-3 family of hashing functions (FIPS 202, NIST SP 800-185 and RFC
//! 9861), including SHA-3, SHAKE, cSHAKE, KMAC, TurboSHAKE256 and
//! KangarooTwelve-256 (KT256).

mod kangaroo_twelve;
pub mod keccak;
pub(crate) mod keccak_batch;
mod kmac;
mod parallel_hash;
mod sha3_256;
mod sha3_512;
mod shake128;
mod shake256;
mod turboshake;

// The module only exists when a caller can reach it: with `std` the CPU is
// probed at run time, without it FEAT_SHA3 must be enabled at compile time.
#[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
mod keccak_arm64;

pub use kangaroo_twelve::{KT256_CHUNK_SIZE, Kt256, Kt256Reader, kt256_backend_name};
pub use kmac::Kmac256;
pub use parallel_hash::{ParallelHash256, ParallelHashReader, backend_name as parallel_hash_backend_name};
pub use sha3_256::Sha3_256;
pub use sha3_512::Sha3_512;
pub use shake128::Shake128;
pub use shake256::{CShake256, Shake256};
pub use turboshake::{TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR, TurboShake256};
