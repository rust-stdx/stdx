#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]
//! AVX2 string scanning.

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn find_string_special(input: &[u8]) -> usize {
    use core::arch::x86_64::*;

    let quote = _mm256_set1_epi8(b'"' as i8);
    let backslash = _mm256_set1_epi8(b'\\' as i8);
    let control = _mm256_set1_epi8(0x1f);

    let mut i = 0;
    while i + 64 <= input.len() {
        let a = _mm256_loadu_si256(input.as_ptr().add(i).cast());
        let b = _mm256_loadu_si256(input.as_ptr().add(i + 32).cast());
        let q = _mm256_or_si256(_mm256_cmpeq_epi8(a, quote), _mm256_cmpeq_epi8(b, quote));
        let s = _mm256_or_si256(_mm256_cmpeq_epi8(a, backslash), _mm256_cmpeq_epi8(b, backslash));
        let x = _mm256_or_si256(
            _mm256_cmpeq_epi8(_mm256_min_epu8(a, control), a),
            _mm256_cmpeq_epi8(_mm256_min_epu8(b, control), b),
        );
        let mask = _mm256_or_si256(_mm256_or_si256(q, s), x);
        if _mm256_testz_si256(mask, mask) == 0 {
            // Find the first byte in the 64-byte window.
            return i + super::scalar_find_string_special(&input[i..i + 64]);
        }
        i += 64;
    }

    while i + 32 <= input.len() {
        let bytes = _mm256_loadu_si256(input.as_ptr().add(i).cast());
        let q = _mm256_cmpeq_epi8(bytes, quote);
        let s = _mm256_cmpeq_epi8(bytes, backslash);
        let x = _mm256_cmpeq_epi8(_mm256_min_epu8(bytes, control), bytes);
        let mask = _mm256_or_si256(_mm256_or_si256(q, s), x);
        if _mm256_testz_si256(mask, mask) == 0 {
            let bits = _mm256_movemask_epi8(mask) as u32;
            return i + bits.trailing_zeros() as usize;
        }
        i += 32;
    }
    i + super::scalar_find_string_special(&input[i..])
}
