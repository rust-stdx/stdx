#![allow(unsafe_op_in_unsafe_fn)]
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::aes::{encrypt_block, expand_key};
#[cfg(target_arch = "x86_64")]
use super::aes_amd64::aes_encrypt_block;
#[cfg(target_arch = "aarch64")]
use super::aes_arm64::aes_encrypt_block;
use crate::{StreamCipher, aes::RoundKeys};

/// AES-128 in CTR mode.
///
/// Create a new cipher with [`new`](Aes128Ctr::new).
/// [`xor_keystream`](StreamCipher::xor_keystream) to encrypt or decrypt
/// (CTR mode is symmetric).
/// You can move in the keystream with [`set_counter`](Aes128Ctr::set_counter).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes128Ctr(pub(crate) AesCtr<11>);

impl Aes128Ctr {
    /// Create a new AES-128-CTR stream cipher from a 16-byte key.
    ///
    /// The initial counter is zeroed.
    #[inline]
    pub fn new(key: &[u8; 16]) -> Self {
        Self(AesCtr::<11>::new(key))
    }

    /// Set the 16-byte counter block.
    ///
    /// For GCM this is `nonce || 0x00000002` (J₀ + 1).
    #[inline]
    pub fn set_counter(&mut self, counter: &[u8; 16]) {
        self.0.set_counter(counter)
    }
}

impl StreamCipher for Aes128Ctr {
    #[inline]
    fn xor_keystream(&mut self, in_out: &mut [u8]) {
        self.0.xor_keystream(in_out)
    }
}

/// AES-256 in CTR mode.
///
/// Create a new cipher with [`new`](Aes256Ctr::new).
/// [`xor_keystream`](StreamCipher::xor_keystream) to encrypt or decrypt
/// (CTR mode is symmetric).
/// You can move in the keystream with [`set_counter`](Aes256Ctr::set_counter).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes256Ctr(pub(crate) AesCtr<15>);

impl Aes256Ctr {
    /// Create a new AES-256-CTR stream cipher from a 32-byte key.
    ///
    /// The initial counter is zeroed.
    #[inline]
    pub fn new(key: &[u8; 32]) -> Self {
        Self(AesCtr::<15>::new(key))
    }

    /// Set the 16-byte counter block.
    ///
    /// For GCM this is `nonce || 0x00000002` (J₀ + 1).
    #[inline]
    pub fn set_counter(&mut self, counter: &[u8; 16]) {
        self.0.set_counter(counter)
    }
}

impl StreamCipher for Aes256Ctr {
    #[inline]
    fn xor_keystream(&mut self, in_out: &mut [u8]) {
        self.0.xor_keystream(in_out)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Generic implementation of AES in counter mode.
/// `N` is the number of rounds. 11 for AES-128 and 15 for AES-256
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct AesCtr<const N: usize> {
    round_keys: RoundKeys<N>,
    counter: [u8; 16],
}

impl AesCtr<11> {
    pub fn new(key: &[u8; 16]) -> Self {
        Self::new_inner(key)
    }
}

impl AesCtr<15> {
    pub fn new(key: &[u8; 32]) -> Self {
        Self::new_inner(key)
    }
}

impl<const N: usize> AesCtr<N> {
    /// Create a new cipher from a 16-byte key.
    ///
    /// The initial counter is zeroed.
    fn new_inner(key: &[u8]) -> Self {
        const { assert!(N == 11 || N == 15) };

        let round_keys_software = expand_key(key);

        #[cfg(target_arch = "aarch64")]
        {
            #[cfg(any(feature = "std", target_feature = "aes"))]
            use crate::aes::aes_arm64::expand_key_armv8;

            #[cfg(feature = "std")]
            if std::arch::is_aarch64_feature_detected!("aes") {
                return AesCtr {
                    round_keys: RoundKeys::Armv8(expand_key_armv8(round_keys_software)),
                    counter: [0u8; 16],
                };
            }

            #[cfg(all(not(feature = "std"), target_feature = "aes"))]
            return AesCtr {
                round_keys: RoundKeys::Armv8(expand_key_armv8(round_keys_software)),
                counter: [0u8; 16],
            };
        }

        #[cfg(target_arch = "x86_64")]
        {
            use crate::aes::aes_amd64::expand_key_x86_64;

            #[cfg(feature = "std")]
            if std::arch::is_x86_feature_detected!("aes") {
                return AesCtr {
                    round_keys: RoundKeys::X86_64(expand_key_x86_64(round_keys_software)),
                    counter: [0u8; 16],
                };
            }

            #[cfg(all(not(feature = "std"), target_feature = "aes"))]
            return AesCtr {
                round_keys: RoundKeys::X86_64(expand_key_x86_64(round_keys_software)),
                counter: [0u8; 16],
            };
        }

        AesCtr {
            round_keys: RoundKeys::Software(round_keys_software),
            counter: [0u8; 16],
        }
    }

    /// Create a new cipher from pre-computed round keys.
    /// It's useful to re-use AES-CTR in another cipher such AES-GCM
    #[inline]
    pub(crate) fn from_round_keys(round_keys: RoundKeys<N>) -> Self {
        const { assert!(N == 11 || N == 15) };

        Self {
            round_keys,
            counter: [0u8; 16],
        }
    }

    fn xor_keystream_soft(&mut self, in_out: &mut [u8]) {
        let RoundKeys::Software(round_keys) = &self.round_keys else {
            unreachable!()
        };

        let n = in_out.len();
        let mut i = 0;
        let mut counter = self.counter;

        while i + 16 <= n {
            let ks = encrypt_block(&round_keys, &self.counter);
            for k in 0..16 {
                in_out[i + k] ^= ks[k];
            }
            Self::increment_counter(&mut counter);
            i += 16;
        }
        self.counter = counter;

        if i < n {
            let ks = encrypt_block(&round_keys, &self.counter);
            for k in 0..n - i {
                in_out[i + k] ^= ks[k];
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "aes,ssse3,sse2")]
    unsafe fn xor_keystream_aesni(&mut self, in_out: &mut [u8]) {
        use super::aes_ctr_amd64::increment_counter;

        let RoundKeys::X86_64(round_keys) = &self.round_keys else {
            unreachable!()
        };

        let n = in_out.len();
        let mut i = 0;
        let mut ctr = _mm_loadu_si128(self.counter.as_ptr().cast());

        while i + 16 <= n {
            let ks = aes_encrypt_block(&round_keys, ctr);
            let p = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
            _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), _mm_xor_si128(p, ks));
            ctr = increment_counter(ctr);
            i += 16;
        }

        if i < n {
            let ks = aes_encrypt_block(&round_keys, ctr);
            let mut ks_bytes = [0u8; 16];
            _mm_storeu_si128(ks_bytes.as_mut_ptr().cast(), ks);
            for k in 0..n - i {
                in_out[i + k] ^= ks_bytes[k];
            }
        }

        _mm_storeu_si128(self.counter.as_mut_ptr().cast(), ctr);
    }

    #[cfg(target_arch = "aarch64")]
    unsafe fn xor_keystream_armv8(&mut self, in_out: &mut [u8]) {
        use super::aes_ctr_arm64::increment_counter;

        let RoundKeys::Armv8(round_keys) = &self.round_keys else {
            unreachable!()
        };
        let n = in_out.len();
        let mut i = 0;
        let mut ctr = vld1q_u8(self.counter.as_ptr());

        while i + 16 <= n {
            let ks = aes_encrypt_block(round_keys, ctr);
            let p = vld1q_u8(in_out.as_ptr().add(i));
            vst1q_u8(in_out.as_mut_ptr().add(i), veorq_u8(p, ks));
            ctr = increment_counter(ctr);
            i += 16;
        }
        if i < n {
            let ks = aes_encrypt_block(round_keys, ctr);
            let mut ks_bytes = [0u8; 16];
            vst1q_u8(ks_bytes.as_mut_ptr(), ks);
            for k in 0..n - i {
                in_out[i + k] ^= ks_bytes[k];
            }
        }

        vst1q_u8(self.counter.as_mut_ptr(), ctr);
    }

    /// Set the 16-byte counter block.
    ///
    /// For GCM this is `nonce || 0x00000002` (J₀ + 1).
    #[inline]
    pub fn set_counter(&mut self, counter: &[u8; 16]) {
        self.counter = *counter;
    }

    #[inline]
    fn increment_counter(counter: &mut [u8; 16]) {
        let counter_value = u32::from_be_bytes(counter[12..16].try_into().unwrap());
        counter[12..16].copy_from_slice(&counter_value.wrapping_add(1).to_be_bytes());
    }
}

impl<const N: usize> StreamCipher for AesCtr<N> {
    #[inline]
    fn xor_keystream(&mut self, in_out: &mut [u8]) {
        match &self.round_keys {
            #[cfg(target_arch = "aarch64")]
            RoundKeys::Armv8(_) => unsafe {
                self.xor_keystream_armv8(in_out);
            },
            #[cfg(target_arch = "x86_64")]
            RoundKeys::X86_64(_) => unsafe {
                self.xor_keystream_aesni(in_out);
            },
            RoundKeys::Software(_) => self.xor_keystream_soft(in_out),
        }
    }
}
