#![allow(unsafe_op_in_unsafe_fn)]
/// x86-64 AES-CTR counter helpers using SSSE3.
use core::arch::x86_64::*;

/// Byte-reversal shuffle mask: maps BE byte order to LE within each 32-bit lane
/// (bytes 0↔3, 1↔2, 4↔7, 5↔6, 8↔11, 9↔10, 12↔15, 13↔14).
pub(crate) const SWAP_BYTES: [i8; 16] = [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12];

/// Increment the big-endian 32-bit counter stored in bytes 12..15.
///
/// Uses `pshufb` to byte-swap the counter to little-endian, adds 1 to the
/// low 32-bit lane, then swaps back. No memory round-trip.
#[target_feature(enable = "ssse3,sse2")]
#[inline]
pub(crate) unsafe fn increment_counter(ctr: __m128i) -> __m128i {
    let swap = _mm_loadu_si128(SWAP_BYTES.as_ptr().cast());
    let le = _mm_shuffle_epi8(ctr, swap);
    let inc = _mm_add_epi32(le, _mm_set_epi32(1, 0, 0, 0));
    _mm_shuffle_epi8(inc, swap)
}
