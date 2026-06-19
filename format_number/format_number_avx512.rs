use core::{arch::x86_64::*, ptr};

use super::{DEC_DIGITS_LUT, MAX_LEN};

const TWOTO52: u64 = 0x10000000000000;

/// Heterogeneous AVX-512 IFMA entry point for n >= 100_000_000.
///
/// Dispatches into one of three paths based on digit count:
///   - 9..=16 digits: 16-digit SIMD kernel + masked store
///   - 17..=20 digits: scalar split + 16-digit SIMD kernel + 4-digit suffix
#[target_feature(enable = "avx512f,avx512vl,avx512ifma,avx512bw,avx512vbmi")]
pub(crate) unsafe fn format_u64_avx512(n: u64, buf: &mut [u8; MAX_LEN], pos: usize) -> usize {
    let ndigits = fast_digit_count(n);

    if ndigits <= 16 {
        // 9..=16 digits: 16-digit kernel + masked store.
        // Store the 16-byte SIMD result at pos-16 so the masked bytes
        // land at buf[pos-ndigits .. pos-1], never past buf[MAX_LEN-1].
        let digits = unsafe { to_string_16digits(n) };
        let simd_base = pos - 16;
        let mask = (0xFFFFu16 << (16 - ndigits)) as __mmask16;
        unsafe {
            _mm_mask_storeu_epi8(buf.as_mut_ptr().add(simd_base) as *mut i8, mask, digits);
        }
        pos - ndigits
    } else {
        // 17..=20 digits: split last 4 digits via /10000, format the
        // remaining high part with the 16-digit kernel, then write the
        // 4-digit suffix.
        // Store the SIMD result at pos-20 so the masked bytes end at
        // pos-5, leaving buf[pos-4 .. pos-1] for the suffix.
        let r = n % 10000;
        let q = n / 10000;
        let nq = ndigits - 4;

        let digits = unsafe { to_string_16digits(q) };
        let simd_base = pos - 20;
        let mask = (0xFFFFu16 << (16 - nq)) as __mmask16;
        unsafe {
            _mm_mask_storeu_epi8(buf.as_mut_ptr().add(simd_base) as *mut i8, mask, digits);
            write_four_digits(buf.as_mut_ptr().add(pos - 4), r);
        }
        pos - ndigits
    }
}

// Branchless digit count via CLZ + table lookup.
#[inline]
fn fast_digit_count(x: u64) -> usize {
    const DIGITS: [u8; 65] = [
        19, 19, 19, 19, 18, 18, 18, 17, 17, 17, 16, 16, 16, 16, 15, 15, 15, 14, 14, 14, 13, 13, 13, 13, 12, 12, 12, 11,
        11, 11, 10, 10, 10, 10, 9, 9, 9, 8, 8, 8, 7, 7, 7, 7, 6, 6, 6, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1,
        1, 1,
    ];
    const THRESHOLDS: [u64; 65] = [
        9999999999999999999,
        9999999999999999999,
        9999999999999999999,
        9999999999999999999,
        999999999999999999,
        999999999999999999,
        999999999999999999,
        99999999999999999,
        99999999999999999,
        99999999999999999,
        9999999999999999,
        9999999999999999,
        9999999999999999,
        9999999999999999,
        999999999999999,
        999999999999999,
        999999999999999,
        99999999999999,
        99999999999999,
        99999999999999,
        9999999999999,
        9999999999999,
        9999999999999,
        9999999999999,
        999999999999,
        999999999999,
        999999999999,
        99999999999,
        99999999999,
        99999999999,
        9999999999,
        9999999999,
        9999999999,
        9999999999,
        999999999,
        999999999,
        999999999,
        99999999,
        99999999,
        99999999,
        9999999,
        9999999,
        9999999,
        9999999,
        999999,
        999999,
        999999,
        99999,
        99999,
        99999,
        9999,
        9999,
        9999,
        9999,
        999,
        999,
        999,
        99,
        99,
        99,
        9,
        9,
        9,
        9,
        0,
    ];
    let lz = x.leading_zeros() as usize;
    (x > THRESHOLDS[lz]) as usize + DIGITS[lz] as usize
}

// Write exactly 4 digits from value in [0, 9999] using the existing LUT.
#[inline]
unsafe fn write_four_digits(buf: *mut u8, value: u64) {
    let v = value as usize;
    let d1 = (v / 100) * 2;
    let d2 = (v % 100) * 2;
    unsafe {
        ptr::copy_nonoverlapping(DEC_DIGITS_LUT.as_ptr().add(d1), buf, 2);
        ptr::copy_nonoverlapping(DEC_DIGITS_LUT.as_ptr().add(d2), buf.add(2), 2);
    }
}

// 16-digit kernel: extracts all 16 digits of n < 10^16.
// Returns a __m128i packed with 16 ASCII bytes in order.
#[inline]
#[target_feature(enable = "avx512f,avx512ifma,avx512vbmi")]
unsafe fn to_string_16digits(n: u64) -> __m128i {
    let n_hi = n / 100_000_000;
    let n_lo = n % 100_000_000;

    let bcstq_h = _mm512_set1_epi64(n_hi as i64);
    let bcstq_l = _mm512_set1_epi64(n_lo as i64);

    let c = _mm512_setr_epi64(
        (TWOTO52 / 100_000_000) as i64,
        (TWOTO52 / 10_000_000) as i64,
        (TWOTO52 / 1_000_000) as i64,
        (TWOTO52 / 100_000) as i64,
        (TWOTO52 / 10_000) as i64,
        (TWOTO52 / 1_000) as i64,
        (TWOTO52 / 100) as i64,
        (TWOTO52 / 10) as i64,
    );
    let vten = _mm512_set1_epi64(10);
    let vzero = _mm512_set1_epi64(b'0' as i64);

    let low_h = _mm512_madd52lo_epu64(c, bcstq_h, c);
    let low_l = _mm512_madd52lo_epu64(c, bcstq_l, c);
    let high_h = _mm512_madd52hi_epu64(vzero, vten, low_h);
    let high_l = _mm512_madd52hi_epu64(vzero, vten, low_l);

    // Permute: interleave hi 8 digits then lo 8 digits into 16 packed bytes.
    // The permutation mask uses the concatenation high_h || high_l as a 128-byte
    // table. Indices 0x00..0x38 select the hi bytes (one per lane), indices
    // 0x40..0x78 select the lo bytes.
    let perm_mask = _mm512_castsi128_si512(_mm_set_epi8(
        0x78, 0x70, 0x68, 0x60, 0x58, 0x50, 0x48, 0x40, 0x38, 0x30, 0x28, 0x20, 0x18, 0x10, 0x08, 0x00,
    ));
    let perm = _mm512_permutex2var_epi8(high_h, perm_mask, high_l);
    _mm512_castsi512_si128(perm)
}
