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

/// A raw AES-128 block cipher.
///
/// Use this when the same key encrypts many single blocks, such as QUIC header protection
/// (RFC 9001 §5.4.3), where the per-packet cost would otherwise be dominated by
/// the schedule.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes128Block {
    schedule: aes_ct::CtSchedule,
}

impl Aes128Block {
    /// Precomputes the AES-128 key schedule for `key`.
    pub fn new(key: &[u8; 16]) -> Self {
        Self {
            schedule: aes_ct::keysched(&aes::expand_key::<11>(key)),
        }
    }

    /// Encrypts one 16-byte block using the precomputed schedule.
    #[inline(always)]
    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        aes_ct::encrypt_block_sched(&self.schedule, block)
    }
}

/// A raw AES-256 block cipher.
///
/// See [`Aes128Block`] to learn when to use this.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes256Block {
    schedule: aes_ct::CtSchedule,
}

impl Aes256Block {
    /// Precomputes the AES-256 key schedule for `key`.
    pub fn new(key: &[u8; 32]) -> Self {
        Self {
            schedule: aes_ct::keysched(&aes::expand_key::<15>(key)),
        }
    }

    /// Encrypts one 16-byte block using the precomputed schedule.
    #[inline(always)]
    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        aes_ct::encrypt_block_sched(&self.schedule, block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encrypt_block_128(key: &[u8; 16], block: &[u8; 16]) -> [u8; 16] {
        encrypt_block(&expand_key::<11>(key), block)
    }

    fn encrypt_block_256(key: &[u8; 32], block: &[u8; 16]) -> [u8; 16] {
        encrypt_block(&expand_key::<15>(key), block)
    }

    #[test]
    fn precomputed_block_ciphers_match_one_shot() {
        let key128 = [0x11u8; 16];
        let key256 = [0x22u8; 32];
        let block = [0x33u8; 16];

        let aes128 = Aes128Block::new(&key128);
        assert_eq!(aes128.encrypt_block(&block), encrypt_block_128(&key128, &block));

        let aes256 = Aes256Block::new(&key256);
        assert_eq!(aes256.encrypt_block(&block), encrypt_block_256(&key256, &block));
    }
}
