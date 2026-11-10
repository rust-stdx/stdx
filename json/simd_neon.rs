#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]
//! NEON string scanning. NEON is mandatory on AArch64.

use core::arch::aarch64::*;

pub(crate) unsafe fn find_string_special(input: &[u8]) -> usize {
    let quote = vdupq_n_u8(b'"');
    let backslash = vdupq_n_u8(b'\\');
    let control = vdupq_n_u8(0x1f);

    let mut i = 0;
    while i + 64 <= input.len() {
        let a = vld1q_u8(input.as_ptr().add(i));
        let b = vld1q_u8(input.as_ptr().add(i + 16));
        let c = vld1q_u8(input.as_ptr().add(i + 32));
        let d = vld1q_u8(input.as_ptr().add(i + 48));
        let q = vmaxq_u8(
            vmaxq_u8(vceqq_u8(a, quote), vceqq_u8(b, quote)),
            vmaxq_u8(vceqq_u8(c, quote), vceqq_u8(d, quote)),
        );
        let s = vmaxq_u8(
            vmaxq_u8(vceqq_u8(a, backslash), vceqq_u8(b, backslash)),
            vmaxq_u8(vceqq_u8(c, backslash), vceqq_u8(d, backslash)),
        );
        let x = vmaxq_u8(
            vmaxq_u8(vcleq_u8(a, control), vcleq_u8(b, control)),
            vmaxq_u8(vcleq_u8(c, control), vcleq_u8(d, control)),
        );
        let mask = vorrq_u8(vorrq_u8(q, s), x);
        if vmaxvq_u8(mask) != 0 {
            return i + super::scalar_find_string_special(&input[i..i + 64]);
        }
        i += 64;
    }

    while i + 16 <= input.len() {
        let bytes = vld1q_u8(input.as_ptr().add(i));
        let q = vceqq_u8(bytes, quote);
        let s = vceqq_u8(bytes, backslash);
        let x = vcleq_u8(bytes, control);
        let mask = vorrq_u8(vorrq_u8(q, s), x);
        if vmaxvq_u8(mask) != 0 {
            return i + super::scalar_find_string_special(&input[i..i + 16]);
        }
        i += 16;
    }
    i + super::scalar_find_string_special(&input[i..])
}
