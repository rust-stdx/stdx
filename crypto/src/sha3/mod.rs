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

// The modules only exist when a caller can reach them: with `std` the CPU is
// probed at run time, without it the feature must be enabled at compile time.
#[cfg(all(
    target_arch = "x86_64",
    any(
        feature = "std",
        target_feature = "bmi1",
        target_feature = "avx2",
        target_feature = "avx512f"
    )
))]
mod keccak_amd64;
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

/// Returns the name of the single-state Keccak-p[1600] backend in use.
///
/// This is the permutation behind [`Sha3_256`], [`Sha3_512`], [`Shake128`],
/// [`Shake256`], [`TurboShake256`] and the serial first and final node of
/// [`Kt256`]. The batched backends reported by [`kt256_backend_name`] and
/// [`parallel_hash_backend_name`] are detected independently.
///
/// ```
/// println!("Keccak backend: {}", crypto::sha3::keccak_backend_name());
/// ```
pub fn keccak_backend_name() -> &'static str {
    return keccak::PermuteBackend::detect().name();
}
