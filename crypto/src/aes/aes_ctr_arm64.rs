#![allow(unsafe_op_in_unsafe_fn)]
/// aarch64 AES-CTR using ARMv8 Crypto and NEON.
use core::arch::aarch64::*;

use super::aes_arm64::aes_encrypt_block;

/// Byte-reversal shuffle mask: maps byte i ↔ byte 15-i (full 16-byte reversal).
pub(crate) const SWAP_MASK: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

/// Increment the big-endian 32-bit counter stored in bytes 12..15.
#[inline]
pub(crate) fn increment_counter(counter: uint8x16_t) -> uint8x16_t {
    unsafe {
        let swap = vld1q_u8(SWAP_MASK.as_ptr());
        let swapped = vqtbl1q_u8(counter, swap);
        let one = vsetq_lane_u32(1, vdupq_n_u32(0), 0);
        let incremented = vaddq_u32(vreinterpretq_u32_u8(swapped), one);
        vqtbl1q_u8(vreinterpretq_u8_u32(incremented), swap)
    }
}

/// XOR the keystream over `in_out` using ARMv8 Crypto extensions.
///
/// `N` is the number of round keys: 11 for AES-128, 15 for AES-256.
/// The counter is read from and written back to `counter` so the caller
/// can resume from the same state on subsequent calls.
pub(crate) unsafe fn xor_keystream_armv8<const N: usize>(
    round_keys: &[uint8x16_t; N],
    counter: &mut [u8; 16],
    in_out: &mut [u8],
) {
    const {
        assert!(N == 11 || N == 15);
    }

    let n = in_out.len();
    let mut i = 0;
    let mut ctr = vld1q_u8(counter.as_ptr());

    while i + 16 <= n {
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let p = vld1q_u8(in_out.as_ptr().add(i));
        vst1q_u8(in_out.as_mut_ptr().add(i), veorq_u8(p, ks));
        ctr = increment_counter(ctr);
        i += 16;
    }
    if i < n {
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let mut ks_bytes = [0u8; 16];
        vst1q_u8(ks_bytes.as_mut_ptr(), ks);
        for k in 0..n - i {
            in_out[i + k] ^= ks_bytes[k];
        }
    }

    vst1q_u8(counter.as_mut_ptr(), ctr);
}
