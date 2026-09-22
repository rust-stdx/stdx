//! NIST P-224, P-256, P-384 and P-521 prime-order curves.
//!
//! The curves share a single generic implementation in [`p_curves`];
//! each per-curve module supplies only its parameters and public API. The
//! public entry points are [`p224`], [`p256`], [`p384`] and [`p521`].

mod p_curves;

pub mod p224;
pub mod p256;
pub mod p384;
pub mod p521;
