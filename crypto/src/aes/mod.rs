mod aes;
mod aes_ct;
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

/// Encrypts one 16-byte block with the raw AES-128 block cipher.
///
/// This is the bare block transform (equivalent to a single-block ECB
/// operation), not an authenticated or length-preserving mode. It exists for
/// protocol constructions that need the raw permutation, such as QUIC header
/// protection (RFC 9001 §5.4.3).
#[inline(always)]
pub fn encrypt_block_128(key: &[u8; 16], block: &[u8; 16]) -> [u8; 16] {
    encrypt_block(&expand_key::<11>(key), block)
}

/// Encrypts one 16-byte block with the raw AES-256 block cipher.
///
/// See [`encrypt_block_128`] for when this is appropriate.
#[inline(always)]
pub fn encrypt_block_256(key: &[u8; 32], block: &[u8; 16]) -> [u8; 16] {
    encrypt_block(&expand_key::<15>(key), block)
}
