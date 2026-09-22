#![allow(unsafe_op_in_unsafe_fn)]
/// aarch64 AES-CTR using ARMv8 Crypto and NEON.
use core::arch::aarch64::*;

use super::aes_arm64::aes_encrypt_block;

/// Byte-reversal shuffle mask: maps byte i ↔ byte 15-i (full 16-byte reversal).
pub(crate) const SWAP_MASK: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

/// Increment the 16-byte big-endian counter block by one.
///
/// This is the NIST SP 800-38A §5.1 standard incrementing function applied to
/// the full 128-bit block: the byte-reversed counter is split into two 64-bit
/// lanes and the carry is propagated from the low lane into the high lane.
#[inline]
pub(crate) fn increment_counter(counter: uint8x16_t) -> uint8x16_t {
    unsafe {
        let swap = vld1q_u8(SWAP_MASK.as_ptr());
        let le = vqtbl1q_u8(counter, swap);
        let lanes = vreinterpretq_u64_u8(le);
        let lo = vgetq_lane_u64::<0>(lanes);
        let hi = vgetq_lane_u64::<1>(lanes);
        let (lo, carry) = lo.overflowing_add(1);
        let hi = hi.wrapping_add(carry as u64);
        let inc = vcombine_u64(vcreate_u64(lo), vcreate_u64(hi));
        vqtbl1q_u8(vreinterpretq_u8_u64(inc), swap)
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
