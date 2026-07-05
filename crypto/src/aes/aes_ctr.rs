#![allow(unsafe_op_in_unsafe_fn)]
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// AES-256-CTR stream cipher with hardware acceleration.
///
/// Wraps the AES-256 block cipher in CTR mode as a [`StreamCipher`].
/// On x86-64 with AES-NI + SSSE3, and on aarch64 with ARMv8 Crypto
/// extensions, the keystream generation is hardware-accelerated.
use super::aes::{RoundKeys, encrypt_block, expand_key};
#[cfg(target_arch = "x86_64")]
use super::aes_amd64::aes_encrypt_block;
#[cfg(target_arch = "aarch64")]
use super::aes_arm64::aes_encrypt_block;
#[cfg(target_arch = "x86_64")]
use super::aes_ctr_amd64::ctr_inc as ctr_inc_ni;
#[cfg(target_arch = "aarch64")]
use super::aes_ctr_arm64::ctr_inc as ctr_inc_arm;
use crate::StreamCipher;

/// AES-256 in CTR mode.
///
/// Create a new cipher with [`new`](Aes256Ctr::new).
/// [`xor_keystream`](StreamCipher::xor_keystream) to encrypt or decrypt
/// (CTR mode is symmetric).
/// You can move in the keystream with [`set_counter`](Aes256Ctr::set_counter).
pub struct Aes256Ctr {
    round_keys: RoundKeys<15>,
    #[cfg(target_arch = "x86_64")]
    round_keys_aesni: [__m128i; 15],
    #[cfg(target_arch = "aarch64")]
    round_keys_armv8: [uint8x16_t; 15],
    counter: [u8; 16],
}

impl Aes256Ctr {
    /// Create a new cipher from a 32-byte key.
    ///
    /// The initial counter is zeroed.
    pub fn new(key: &[u8; 32]) -> Self {
        let round_keys = expand_key::<15>(key);
        #[cfg(target_arch = "x86_64")]
        {
            let mut round_keys_aesni = unsafe { [_mm_setzero_si128(); 15] };
            for i in 0..15 {
                round_keys_aesni[i] = unsafe { _mm_loadu_si128(round_keys[i].as_ptr().cast()) };
            }
            return Self {
                round_keys,
                round_keys_aesni,
                counter: [0u8; 16],
            };
        }
        #[cfg(target_arch = "aarch64")]
        {
            let mut round_keys_armv8 = [unsafe { vdupq_n_u8(0) }; 15];
            for i in 0..15 {
                round_keys_armv8[i] = unsafe { vld1q_u8(round_keys[i].as_ptr()) };
            }
            return Self {
                round_keys,
                round_keys_armv8,
                counter: [0u8; 16],
            };
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Self {
                round_keys,
                counter: [0u8; 16],
            }
        }
    }

    fn xor_keystream_soft(&mut self, in_out: &mut [u8]) {
        let n = in_out.len();
        let mut i = 0;
        while i + 16 <= n {
            let ks = encrypt_block(&self.round_keys, &self.counter);
            for k in 0..16 {
                in_out[i + k] ^= ks[k];
            }
            self.increment_counter();
            i += 16;
        }
        if i < n {
            let ks = encrypt_block(&self.round_keys, &self.counter);
            for k in 0..n - i {
                in_out[i + k] ^= ks[k];
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "aes,ssse3,sse2")]
    unsafe fn xor_keystream_aesni(&mut self, in_out: &mut [u8]) {
        let n = in_out.len();
        let mut i = 0;
        let mut ctr = _mm_loadu_si128(self.counter.as_ptr().cast());

        while i + 16 <= n {
            let ks = aes_encrypt_block(&self.round_keys_aesni, ctr);
            let p = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
            _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), _mm_xor_si128(p, ks));
            ctr = ctr_inc_ni(ctr);
            i += 16;
        }
        if i < n {
            let ks = aes_encrypt_block(&self.round_keys_aesni, ctr);
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
        let n = in_out.len();
        let mut i = 0;
        let mut ctr = vld1q_u8(self.counter.as_ptr());

        while i + 16 <= n {
            let ks = aes_encrypt_block(&self.round_keys_armv8, ctr);
            let p = vld1q_u8(in_out.as_ptr().add(i));
            vst1q_u8(in_out.as_mut_ptr().add(i), veorq_u8(p, ks));
            ctr = ctr_inc_arm(ctr);
            i += 16;
        }
        if i < n {
            let ks = aes_encrypt_block(&self.round_keys_armv8, ctr);
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
    fn increment_counter(&mut self) {
        let counter_value = u32::from_be_bytes(self.counter[12..16].try_into().unwrap());
        self.counter[12..16].copy_from_slice(&counter_value.wrapping_add(1).to_be_bytes());
    }
}

impl StreamCipher for Aes256Ctr {
    #[allow(unreachable_code)]
    fn xor_keystream(&mut self, in_out: &mut [u8]) {
        #[cfg(target_arch = "aarch64")]
        {
            unsafe {
                self.xor_keystream_armv8(in_out);
            }
            return;
        }

        #[cfg(feature = "std")]
        {
            #[cfg(target_arch = "x86_64")]
            {
                if std::arch::is_x86_feature_detected!("aes") && std::arch::is_x86_feature_detected!("ssse3") {
                    unsafe {
                        self.xor_keystream_aesni(in_out);
                    }
                    return;
                }
            }
        }

        #[cfg(not(feature = "std"))]
        {
            #[cfg(all(target_arch = "x86_64", target_feature = "aes", target_feature = "ssse3"))]
            {
                unsafe {
                    self.xor_keystream_aesni(in_out);
                }
                return;
            }
        }

        self.xor_keystream_soft(in_out);
    }
}
