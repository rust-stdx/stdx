//! NEON UTF-8 validation.
//!
//! This kernel is deliberately limited to the ASCII fast path. If a block
//! contains a non-ASCII byte, the whole input is validated by the standard
//! library. This keeps the unsafe surface small while accelerating the common
//! CBOR case (ASCII text keys and messages).

#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::aarch64::*;

/// Validate `bytes` using NEON for ASCII blocks.
///
/// # Safety
///
/// NEON is mandatory on AArch64, but the caller must still satisfy the
/// `#[target_feature]` contract.
#[target_feature(enable = "neon")]
pub(crate) unsafe fn validate(bytes: &[u8]) -> bool {
    let mut index = 0;
    let len = bytes.len();
    let ptr = bytes.as_ptr();

    while index + 64 <= len {
        let a = vld1q_u8(ptr.add(index));
        let b = vld1q_u8(ptr.add(index + 16));
        let c = vld1q_u8(ptr.add(index + 32));
        let d = vld1q_u8(ptr.add(index + 48));
        let maximum = vmaxvq_u8(vmaxq_u8(vmaxq_u8(a, b), vmaxq_u8(c, d)));
        if maximum >= 0x80 {
            return core::str::from_utf8(bytes).is_ok();
        }
        index += 64;
    }

    while index + 16 <= len {
        let block = vld1q_u8(ptr.add(index));
        if vmaxvq_u8(block) >= 0x80 {
            return core::str::from_utf8(bytes).is_ok();
        }
        index += 16;
    }

    if index < len {
        return core::str::from_utf8(&bytes[index..]).is_ok();
    }
    true
}
