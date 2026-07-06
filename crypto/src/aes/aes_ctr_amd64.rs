#![allow(unsafe_op_in_unsafe_fn)]
/// x86-64 AES-CTR using AES-NI, SSSE3 and SSE2.
use core::arch::x86_64::*;

use super::aes_amd64::aes_encrypt_block;

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

/// XOR the keystream over `in_out` using AES-NI.
///
/// `N` is the number of round keys: 11 for AES-128, 15 for AES-256.
/// The counter is read from and written back to `counter` so the caller
/// can resume from the same state on subsequent calls.
#[target_feature(enable = "aes,ssse3,sse2")]
pub(crate) unsafe fn xor_keystream_aesni<const N: usize>(
    round_keys: &[__m128i; N],
    counter: &mut [u8; 16],
    in_out: &mut [u8],
) {
    const {
        assert!(N == 11 || N == 15);
    }

    let n = in_out.len();
    let mut i = 0;
    let mut ctr = _mm_loadu_si128(counter.as_ptr().cast());

    while i + 16 <= n {
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let p = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), _mm_xor_si128(p, ks));
        ctr = increment_counter(ctr);
        i += 16;
    }

    if i < n {
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let mut ks_bytes = [0u8; 16];
        _mm_storeu_si128(ks_bytes.as_mut_ptr().cast(), ks);
        for k in 0..n - i {
            in_out[i + k] ^= ks_bytes[k];
        }
    }

    _mm_storeu_si128(counter.as_mut_ptr().cast(), ctr);
}
