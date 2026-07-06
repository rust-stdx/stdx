#![allow(unsafe_op_in_unsafe_fn)]
/// aarch64 AES-CTR counter helpers using NEON.
use core::arch::aarch64::*;

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
