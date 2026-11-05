//! AVX2 UTF-8 validation.
//!
//! This kernel is deliberately limited to the ASCII fast path. If a block
//! contains a non-ASCII byte, the whole input is validated by the standard
//! library. This keeps the unsafe surface small while accelerating the common
//! CBOR case (ASCII text keys and messages).

#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::x86_64::*;

/// Validate `bytes` using AVX2 for ASCII blocks.
///
/// # Safety
///
/// The caller must ensure that AVX2 is available on the current CPU.
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn validate(bytes: &[u8]) -> bool {
    let mut index = 0;
    let len = bytes.len();
    let ptr = bytes.as_ptr();

    while index + 128 <= len {
        let a = _mm256_loadu_si256(ptr.add(index).cast());
        let b = _mm256_loadu_si256(ptr.add(index + 32).cast());
        let c = _mm256_loadu_si256(ptr.add(index + 64).cast());
        let d = _mm256_loadu_si256(ptr.add(index + 96).cast());
        let combined = _mm256_or_si256(_mm256_or_si256(a, b), _mm256_or_si256(c, d));
        if _mm256_movemask_epi8(combined) != 0 {
            return core::str::from_utf8(bytes).is_ok();
        }
        index += 128;
    }

    while index + 32 <= len {
        let block = _mm256_loadu_si256(ptr.add(index).cast());
        if _mm256_movemask_epi8(block) != 0 {
            return core::str::from_utf8(bytes).is_ok();
        }
        index += 32;
    }

    if index < len {
        return core::str::from_utf8(&bytes[index..]).is_ok();
    }
    true
}
