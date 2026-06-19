#![no_std]
#![allow(unexpected_cfgs)]

//! xxHash — extremely fast non-cryptographic hash algorithm.
//!
//! Provides four variants:
//!
//! | Struct | Bits | Description |
//! |--------|------|-------------|
//! | [`Xxh3_64`] | 64 | Recommended — XXH3 algorithm, faster on modern CPUs |
//! | [`Xxh3_128`] | 128 | Recommended — XXH3 algorithm, 128-bit output |
//! | [`Xxh32`] | 32 | Classic xxHash (XXH32) |
//! | [`Xxh64`] | 64 | Classic xxHash (XXH64) |
//!
//! **Prefer [`Xxh3_64`] or [`Xxh3_128`] for new code.** XXH3 is the modern
//! variant: ~2x faster on large inputs and >3x faster on small inputs
//! compared to the classic XXH64, with better hash quality.
//!
//! All types implement the [`Checksum`] trait.
//!
//! # Const one-shot hashing
//!
//! Use the free functions [`xxh32`], [`xxh64`], [`xxh3_64`], and
//! [`xxh3_128`] for `const`-compatible one-shot hashing:
//!
//! ```rust
//! use xxhash::xxh3_64;
//!
//! const HASH: u64 = xxh3_64(b"hello");
//! assert_eq!(HASH, 0x9555E8555C62DCFD);
//! ```
//!
//! # Examples
//!
//! ## One-shot hashing (via trait)
//!
//! ```rust
//! use xxhash::{Xxh3_64, Checksum};
//!
//! let hash: u64 = Xxh3_64::checksum(b"hello world");
//! ```
//!
//! ## Incremental hashing
//!
//! ```rust
//! use xxhash::{Xxh3_128, Checksum};
//!
//! let mut hasher = Xxh3_128::new();
//! hasher.update(b"hello ");
//! hasher.update(b"world");
//! let hash: u128 = hasher.sum();
//! ```
//!
//! ## Seeded hashing
//!
//! ```rust
//! use xxhash::{Xxh3_64, Checksum};
//!
//! let mut hasher = Xxh3_64::with_seed(42);
//! hasher.update(b"data");
//! let hash: u64 = hasher.sum();
//! ```
//!
//! ## XXH3 with a custom secret
//!
//! ```rust
//! use xxhash::{Xxh3_64, Checksum};
//!
//! let secret = [0xAB; 192];
//! let mut hasher = Xxh3_64::with_secret(secret);
//! hasher.update(b"data");
//! let hash: u64 = hasher.sum();
//! ```

use core::fmt;

#[cfg(target_arch = "aarch64")]
#[path = "xxh3_neon.rs"]
mod xxh3_neon;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "xxh3_avx2.rs"]
mod xxh3_avx2;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[path = "xxh3_avx512.rs"]
mod xxh3_avx512;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[path = "xxh3_wasm_simd128.rs"]
mod xxh3_wasm_simd128;

mod xxh3;
mod xxh32;
mod xxh64;

mod sealed {
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
}

/// Trait for types that can be used as xxHash outputs.
///
/// This trait is sealed and only implemented for `u32`, `u64`, and `u128`.
pub trait ChecksumOutput: sealed::Sealed + Copy + Clone + fmt::Debug + PartialEq + 'static {}

impl ChecksumOutput for u32 {}
impl ChecksumOutput for u64 {}
impl ChecksumOutput for u128 {}

/// A trait for computing checksums.
///
/// Types implementing this trait can compute a hash incrementally via
/// [`update`](Checksum::update) and [`sum`](Checksum::sum), or in a single
/// call via [`checksum`](Checksum::checksum).
pub trait Checksum {
    /// The type of the resulting checksum value.
    type Output: ChecksumOutput;

    /// Create a new checksum instance with default settings (seed = 0,
    /// default secret for XXH3 variants).
    fn new() -> Self;

    /// Compute the checksum of `data` in a single call.
    fn checksum(data: &[u8]) -> Self::Output
    where
        Self: Sized,
    {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.sum()
    }

    /// Feed additional data into the checksum.
    fn update(&mut self, data: &[u8]);

    /// Finalize and return the computed checksum value.
    fn sum(self) -> Self::Output;
}

pub use xxh3::{Xxh3_64, Xxh3_128, xxh3_64, xxh3_128};
pub use xxh32::{Xxh32, xxh32};
pub use xxh64::{Xxh64, xxh64};

#[cfg(test)]
mod test_helpers {
    extern crate alloc;

    /// Fills a buffer with pseudorandom data, exactly matching the C reference's
    /// `XSUM_fillTestBuffer` from `xsum_sanity_check.c`.
    ///
    /// The PRNG uses:
    /// - `byteGen` initialized to `PRIME32` (0x9E3779B1)
    /// - Each iteration: `buf[i] = byteGen >> 56; byteGen *= PRIME64` where
    ///   `PRIME64` is the C test file's constant 0x9E3779B185EBCA8D (not the
    ///   hash PRIME64_1 constant).
    pub(crate) fn fill_test_buffer(len: usize) -> alloc::vec::Vec<u8> {
        const C_PRIME32: u64 = 0x9E3779B1;
        const C_PRIME64: u64 = 0x9E3779B185EBCA8D;

        let mut buf = alloc::vec::Vec::with_capacity(len);
        let mut byte_gen = C_PRIME32;
        for _ in 0..len {
            buf.push((byte_gen >> 56) as u8);
            byte_gen = byte_gen.wrapping_mul(C_PRIME64);
        }
        buf
    }
}
