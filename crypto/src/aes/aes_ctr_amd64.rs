#![allow(unsafe_op_in_unsafe_fn)]
/// x86-64 AES-CTR using AES-NI, SSSE3 and SSE2.
use core::arch::x86_64::*;

use super::aes_amd64::aes_encrypt_block;

/// Byte-reversal shuffle mask: maps BE byte order to LE within each 32-bit lane
/// (bytes 0↔3, 1↔2, 4↔7, 5↔6, 8↔11, 9↔10, 12↔15, 13↔14).
pub(crate) const SWAP_BYTES: [i8; 16] = [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12];

/// Full 16-byte reversal: maps the big-endian counter block to little-endian.
const REVERSE_BYTES: [i8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

/// Increment the 16-byte big-endian counter block by one.
///
/// This is the NIST SP 800-38A §5.1 standard incrementing function applied to
/// the full 128-bit block: the byte-reversed counter is split into two 64-bit
/// halves and the carry is propagated from the low half into the high half, so
/// the low 32 bits never wrap on their own. No memory round-trip.
#[target_feature(enable = "ssse3,sse2")]
#[inline]
pub(crate) unsafe fn increment_counter(ctr: __m128i) -> __m128i {
    let reverse = _mm_loadu_si128(REVERSE_BYTES.as_ptr().cast());
    let le = _mm_shuffle_epi8(ctr, reverse);
    let lo = _mm_cvtsi128_si64(le) as u64;
    let hi = _mm_cvtsi128_si64(_mm_srli_si128(le, 8)) as u64;
    let (lo, carry) = lo.overflowing_add(1);
    let hi = hi.wrapping_add(carry as u64);
    let inc = _mm_set_epi64x(hi as i64, lo as i64);
    _mm_shuffle_epi8(inc, reverse)
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
