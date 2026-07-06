mod aes;
mod aes_ctr;
mod aes_gcm;
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

pub(crate) use aes::RoundKeys;
pub use aes::{decrypt_block, encrypt_block, expand_key};
pub use aes_ctr::{Aes128Ctr, Aes256Ctr};
pub use aes_gcm::{Aes128Gcm, Aes256Gcm};
