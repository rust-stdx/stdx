#![allow(unsafe_op_in_unsafe_fn)]
/// x86-64 AES-CTR using AES-NI, SSSE3 and SSE2.
use core::arch::x86_64::*;

use super::aes_amd64::{aes_encrypt_block, aes_encrypt_vaes256, aes_encrypt_vaes512};

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

/// Write the big-endian counter block `base + block` at `buf[offset..]`.
#[inline]
fn write_counter(buf: &mut [u8], offset: usize, base: u128, block: u128) {
    buf[offset..offset + 16].copy_from_slice(&base.wrapping_add(block).to_be_bytes());
}

/// XOR the AES-CTR keystream using VAES with 256-bit registers.
///
/// Processes eight blocks per iteration. The counter is a full 128-bit
/// big-endian integer and is advanced by one for every complete block only, like
/// the AES-NI and software implementations. `counter` is left pointing at the
/// partial block when `in_out.len()` is not a multiple of 16.
#[target_feature(enable = "vaes,avx2")]
pub(crate) unsafe fn xor_keystream_vaes256<const N: usize>(
    round_keys: &[__m128i; N],
    counter: &mut [u8; 16],
    in_out: &mut [u8],
) {
    const {
        assert!(N == 11 || N == 15);
    }

    let base = u128::from_be_bytes(*counter);
    let n = in_out.len();
    let full = n / 16;
    let mut blocks = 0usize;
    let mut i = 0usize;

    // Eight blocks (128 bytes): four YMM registers.
    while blocks + 8 <= full {
        let mut ctrs = [0u8; 128];
        for j in 0..8 {
            write_counter(&mut ctrs, j * 16, base, blocks as u128 + j as u128);
        }
        let b = [
            _mm256_loadu_si256(ctrs.as_ptr().cast()),
            _mm256_loadu_si256(ctrs.as_ptr().add(32).cast()),
            _mm256_loadu_si256(ctrs.as_ptr().add(64).cast()),
            _mm256_loadu_si256(ctrs.as_ptr().add(96).cast()),
        ];
        let k = aes_encrypt_vaes256::<N, 4>(round_keys, b);
        for j in 0..4 {
            let p = _mm256_loadu_si256(in_out.as_ptr().add(i + j * 32).cast());
            _mm256_storeu_si256(in_out.as_mut_ptr().add(i + j * 32).cast(), _mm256_xor_si256(p, k[j]));
        }
        i += 128;
        blocks += 8;
    }

    // Four blocks (64 bytes): two YMM registers.
    while blocks + 4 <= full {
        let mut ctrs = [0u8; 64];
        for j in 0..4 {
            write_counter(&mut ctrs, j * 16, base, blocks as u128 + j as u128);
        }
        let b = [
            _mm256_loadu_si256(ctrs.as_ptr().cast()),
            _mm256_loadu_si256(ctrs.as_ptr().add(32).cast()),
        ];
        let k = aes_encrypt_vaes256::<N, 2>(round_keys, b);
        for j in 0..2 {
            let p = _mm256_loadu_si256(in_out.as_ptr().add(i + j * 32).cast());
            _mm256_storeu_si256(in_out.as_mut_ptr().add(i + j * 32).cast(), _mm256_xor_si256(p, k[j]));
        }
        i += 64;
        blocks += 4;
    }

    // One block at a time with AES-NI.
    while blocks < full {
        let ctr_bytes = base.wrapping_add(blocks as u128).to_be_bytes();
        let ctr = _mm_loadu_si128(ctr_bytes.as_ptr().cast());
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let p = _mm_loadu_si128(in_out.as_ptr().add(i).cast());
        _mm_storeu_si128(in_out.as_mut_ptr().add(i).cast(), _mm_xor_si128(p, ks));
        i += 16;
        blocks += 1;
    }

    // Partial final block. It consumes the counter but does not advance it,
    // matching the existing AES-NI and software paths.
    let rem = n - i;
    if rem != 0 {
        let ctr_bytes = base.wrapping_add(full as u128).to_be_bytes();
        let ctr = _mm_loadu_si128(ctr_bytes.as_ptr().cast());
        let ks = aes_encrypt_block::<N>(round_keys, ctr);
        let mut ks_bytes = [0u8; 16];
        _mm_storeu_si128(ks_bytes.as_mut_ptr().cast(), ks);
        for k in 0..rem {
            in_out[i + k] ^= ks_bytes[k];
        }
    }

    *counter = base.wrapping_add(full as u128).to_be_bytes();
    _mm256_zeroupper();
}

/// XOR the AES-CTR keystream using VAES with 512-bit registers.
///
/// Processes sixteen blocks (256 bytes) per iteration. The remaining blocks and
/// the partial tail are delegated to the 256-bit kernel, which keeps the large
/// path small and shares the well-tested counter/tail logic. Both kernels use
/// the same full 128-bit big-endian increment.
#[target_feature(enable = "vaes,avx512f,avx512bw,avx2")]
pub(crate) unsafe fn xor_keystream_vaes512<const N: usize>(
    round_keys: &[__m128i; N],
    counter: &mut [u8; 16],
    in_out: &mut [u8],
) {
    const {
        assert!(N == 11 || N == 15);
    }

    let base = u128::from_be_bytes(*counter);
    let n = in_out.len();
    let full = n / 16;
    let mut blocks = 0usize;
    let mut i = 0usize;

    // Sixteen blocks (256 bytes): four ZMM registers.
    while blocks + 16 <= full {
        let mut ctrs = [0u8; 256];
        for j in 0..16 {
            write_counter(&mut ctrs, j * 16, base, blocks as u128 + j as u128);
        }
        let b = [
            _mm512_loadu_si512(ctrs.as_ptr().cast()),
            _mm512_loadu_si512(ctrs.as_ptr().add(64).cast()),
            _mm512_loadu_si512(ctrs.as_ptr().add(128).cast()),
            _mm512_loadu_si512(ctrs.as_ptr().add(192).cast()),
        ];
        let k = aes_encrypt_vaes512::<N, 4>(round_keys, b);
        for j in 0..4 {
            let p = _mm512_loadu_si512(in_out.as_ptr().add(i + j * 64).cast());
            _mm512_storeu_si512(in_out.as_mut_ptr().add(i + j * 64).cast(), _mm512_xor_si512(p, k[j]));
        }
        i += 256;
        blocks += 16;
    }

    if blocks < full || !n.is_multiple_of(16) {
        let mut tail = base.wrapping_add(blocks as u128).to_be_bytes();
        xor_keystream_vaes256::<N>(round_keys, &mut tail, &mut in_out[i..]);
        *counter = tail;
    } else {
        *counter = base.wrapping_add(full as u128).to_be_bytes();
    }

    _mm256_zeroupper();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aes::{
        aes::{RoundKeysSoftware, expand_key},
        aes_ct::xor_keystream_soft,
    };

    fn have_vaes256() -> bool {
        std::arch::is_x86_feature_detected!("vaes") && std::arch::is_x86_feature_detected!("avx2")
    }

    fn have_vaes512() -> bool {
        have_vaes256()
            && std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512bw")
    }

    const COUNTERS: [[u8; 16]; 6] = [
        [0u8; 16],
        [0xffu8; 16],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff],
        [0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        [0x5a; 16],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];

    fn pattern(len: usize) -> Vec<u8> {
        (0..len).map(|i| (i as u8).wrapping_mul(37).wrapping_add(11)).collect()
    }

    #[test]
    fn vaes256_ctr_matches_soft() {
        if !have_vaes256() {
            eprintln!("skipping vaes256_ctr_matches_soft: VAES/AVX2 not available");
            return;
        }
        let key = [0x3cu8; 16];
        let soft = expand_key::<11>(&key);
        check_256_128(&soft);
        let key = [0x9eu8; 32];
        let soft = expand_key::<15>(&key);
        check_256_256(&soft);
    }

    #[test]
    fn vaes512_ctr_matches_soft() {
        if !have_vaes512() {
            eprintln!("skipping vaes512_ctr_matches_soft: VAES/AVX-512 not available");
            return;
        }
        let key = [0x3cu8; 16];
        let soft = expand_key::<11>(&key);
        check_512_128(&soft);
        let key = [0x9eu8; 32];
        let soft = expand_key::<15>(&key);
        check_512_256(&soft);
    }

    fn check_256_128(soft: &RoundKeysSoftware<11>) {
        let rk = rk_from_soft(soft);
        check_kernel(soft, &rk, false);
    }
    fn check_256_256(soft: &RoundKeysSoftware<15>) {
        let rk = rk_from_soft(soft);
        check_kernel(soft, &rk, false);
    }
    fn check_512_128(soft: &RoundKeysSoftware<11>) {
        let rk = rk_from_soft(soft);
        check_kernel(soft, &rk, true);
    }
    fn check_512_256(soft: &RoundKeysSoftware<15>) {
        let rk = rk_from_soft(soft);
        check_kernel(soft, &rk, true);
    }

    fn check_kernel<const N: usize>(soft: &RoundKeysSoftware<N>, rk: &[__m128i; N], use512: bool) {
        for len in 0..=600usize {
            for base in COUNTERS {
                let input = pattern(len);
                let mut got = input.clone();
                let mut got_ctr = base;
                let mut want = input.clone();
                let mut want_ctr = base;

                unsafe {
                    if use512 {
                        xor_keystream_vaes512(rk, &mut got_ctr, &mut got);
                    } else {
                        xor_keystream_vaes256(rk, &mut got_ctr, &mut got);
                    }
                }
                xor_keystream_soft(soft, &mut want_ctr, &mut want);

                assert_eq!(got, want, "ciphertext mismatch len={len}");
                assert_eq!(got_ctr, want_ctr, "counter mismatch len={len}");
            }
        }
    }

    fn rk_from_soft<const N: usize>(soft: &RoundKeysSoftware<N>) -> [__m128i; N] {
        let mut rk = [unsafe { _mm_setzero_si128() }; N];
        for i in 0..N {
            rk[i] = unsafe { _mm_loadu_si128(soft[i].as_ptr().cast()) };
        }
        rk
    }

    #[test]
    fn vaes256_ctr_unaligned_and_canaries() {
        if !have_vaes256() {
            eprintln!("skipping vaes256_ctr_unaligned_and_canaries: VAES/AVX2 not available");
            return;
        }
        let key = [0x11u8; 16];
        let soft = expand_key::<11>(&key);
        let rk = rk_from_soft(&soft);

        for len in 0..=257usize {
            let mut got = vec![0xa5u8; len + 32];
            let mut want = vec![0xa5u8; len];
            let mut ctr = [0x42u8; 16];
            let mut want_ctr = ctr;
            unsafe {
                xor_keystream_vaes256(&rk, &mut ctr, &mut got[15..15 + len]);
            }
            xor_keystream_soft(&soft, &mut want_ctr, &mut want);
            assert_eq!(&got[15..15 + len], &want[..], "len={len}");
            assert_eq!(ctr, want_ctr);
            // Canaries around the processed slice.
            assert!(got[..15].iter().all(|&b| b == 0xa5));
            assert!(got[15 + len..].iter().all(|&b| b == 0xa5));
        }
    }
}
