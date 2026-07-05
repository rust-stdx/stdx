mod aes;
mod aes_128_gcm;
mod aes_256_gcm;
mod aes_ctr;
mod ghash;

#[cfg(target_arch = "x86_64")]
mod aes_amd64;
#[cfg(target_arch = "aarch64")]
mod aes_arm64;

#[cfg(target_arch = "x86_64")]
mod aes_gcm_amd64;
#[cfg(target_arch = "aarch64")]
mod aes_gcm_arm64;

#[cfg(target_arch = "x86_64")]
mod aes_ctr_amd64;
#[cfg(target_arch = "aarch64")]
mod aes_ctr_arm64;

#[cfg(target_arch = "x86_64")]
mod ghash_amd64;
#[cfg(target_arch = "aarch64")]
mod ghash_arm64;

pub use aes::{RoundKeys, decrypt_block, encrypt_block, expand_key};
pub use aes_128_gcm::Aes128Gcm;
pub use aes_256_gcm::Aes256Gcm;
pub use aes_ctr::Aes256Ctr;
