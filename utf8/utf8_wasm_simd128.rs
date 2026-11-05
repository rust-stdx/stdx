//! WebAssembly SIMD128 UTF-8 validation.
//!
//! This kernel is deliberately limited to the ASCII fast path. If a block
//! contains a non-ASCII byte, the whole input is validated by the standard
//! library. This keeps the unsafe surface small while accelerating the common
//! CBOR case (ASCII text keys and messages).

#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::wasm32::*;

/// Validate `bytes` using SIMD128 for ASCII blocks.
///
/// # Safety
///
/// The caller must ensure that the `simd128` target feature is available.
#[target_feature(enable = "simd128")]
pub(crate) unsafe fn validate(bytes: &[u8]) -> bool {
    let mut index = 0;
    let len = bytes.len();
    let ptr = bytes.as_ptr();

    while index + 64 <= len {
        let a = v128_load(ptr.add(index).cast());
        let b = v128_load(ptr.add(index + 16).cast());
        let c = v128_load(ptr.add(index + 32).cast());
        let d = v128_load(ptr.add(index + 48).cast());
        let combined = v128_or(v128_or(a, b), v128_or(c, d));
        if v128_any_true(v128_and(combined, u8x16_splat(0x80))) {
            return core::str::from_utf8(bytes).is_ok();
        }
        index += 64;
    }

    while index + 16 <= len {
        let block = v128_load(ptr.add(index).cast());
        let high_bits = v128_and(block, u8x16_splat(0x80));
        if v128_any_true(high_bits) {
            return core::str::from_utf8(bytes).is_ok();
        }
        index += 16;
    }

    if index < len {
        return core::str::from_utf8(&bytes[index..]).is_ok();
    }
    true
}
