#![no_std]

//! Padmé length-hiding padding.
//!
//! This implements the padding function from Nikitin et al., *Reducing
//! Metadata Leakage from Encrypted Files and Communication with PURBs*
//! (PoPETs 2019, <https://arxiv.org/abs/1806.03160>).
//!
//! The padded length is the next length whose binary floating-point
//! representation has a mantissa no longer than its exponent. This limits
//! length leakage to a small number of size classes while keeping the
//! overhead below roughly 12%, decreasing with the payload size.

/// Returns the padded length for `len` bytes.
///
/// `padded_len(0)` is `0`. For every other input the result is at least
/// `len`. If `len + padding` would overflow `usize`, the result is
/// `usize::MAX`; callers always bound payloads to a much smaller maximum.
#[must_use]
pub fn padded_len(len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    // floor(log2(len)) and its bit length (the floating-point exponent and
    // the number of significant mantissa bits). Both are `u32`; using
    // `u32::BITS` here is required because `usize::BITS` is the width of the
    // *value*, not of the exponent.
    let exponent = usize::BITS - 1 - len.leading_zeros();
    let significant = u32::BITS - exponent.leading_zeros();
    let shift = exponent - significant;

    let mask = (1usize << shift) - 1;
    match len.checked_add(mask) {
        Some(rounded) => rounded & !mask,
        None => usize::MAX,
    }
}

/// Returns the number of padding bytes added for `len` bytes.
#[must_use]
pub fn padding_len(len: usize) -> usize {
    padded_len(len) - len
}
