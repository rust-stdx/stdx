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
//! variant and is around twice as fast as the classic XXH64 on large inputs,
//! with better hash quality.
//!
//! All types implement the [`Checksum`] trait. Incremental hashing agrees with
//! one-shot hashing for every input length and every split of the input across
//! [`update`](Checksum::update) calls, and no input can make any of these
//! functions panic.
//!
//! Results are identical on every platform and match the
//! [reference implementation](https://github.com/Cyan4973/xxHash).
//!
//! # SIMD acceleration
//!
//! The long-input path of XXH3 is accelerated with NEON (aarch64), AVX2 /
//! AVX-512 (x86) or SIMD128 (wasm32) kernels:
//!
//! - With the (default) `std` feature enabled, the kernel is chosen at runtime
//!   from the CPU features actually available, so a binary compiled for a
//!   baseline target still uses the widest instructions the machine supports.
//! - With `default-features = false` (i.e. `no_std`), the kernel is chosen at
//!   compile time from the enabled `target-feature`s; pass e.g.
//!   `RUSTFLAGS="-C target-feature=+avx2"` to enable them.
//!
//! The [`const`](xxh3_64) one-shot functions are scalar-only, so prefer
//! [`Checksum::checksum`] or [`Checksum::update`] for long inputs when runtime
//! performance matters. Both produce the same value.
//!
//! # Security
//!
//! xxHash is **not** a cryptographic hash. It must not be used to authenticate
//! messages, derive keys, or store passwords, and a seed or a custom secret is
//! not a security boundary. Its collision resistance is not sufficient to
//! protect a hash table from attacker-chosen keys (hash flooding): XXH3 in
//! particular has known collision weaknesses for 17..=240 byte inputs. For
//! those uses, prefer a keyed cryptographic hash.
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

// Runtime CPU detection requires the standard library. This crate is `no_std`
// by default; enabling the (default) `std` feature makes the SIMD kernels
// reachable on any CPU that supports them, without requiring `RUSTFLAGS`.
#[cfg(feature = "std")]
extern crate std;

// SIMD kernels are compiled when the matching target feature is enabled at
// compile time, and — for x86, when the `std` feature is on — also when it is
// not, so that they can be selected at runtime by `x86_dispatch`.
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[path = "xxh3_neon.rs"]
mod xxh3_neon;

#[cfg(all(target_arch = "x86_64", any(target_feature = "avx2", feature = "std")))]
#[path = "xxh3_avx2.rs"]
mod xxh3_avx2;

#[cfg(all(target_arch = "x86_64", any(target_feature = "avx512f", feature = "std")))]
#[path = "xxh3_avx512.rs"]
mod xxh3_avx512;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[path = "xxh3_wasm_simd128.rs"]
mod xxh3_wasm_simd128;

/// Cached runtime CPU feature detection for x86 targets (requires `std`).
///
/// The detected level is computed once and cached in an atomic, so the cost per
/// lookup is a single relaxed load plus a branch.
#[cfg(all(target_arch = "x86_64", feature = "std"))]
mod x86_dispatch {
    use core::sync::atomic::{AtomicU8, Ordering};

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Level {
        Scalar,
        Avx2,
        Avx512,
    }

    const UNKNOWN: u8 = 0;
    const SCALAR: u8 = 1;
    const AVX2: u8 = 2;
    const AVX512: u8 = 3;

    static LEVEL: AtomicU8 = AtomicU8::new(UNKNOWN);

    #[inline]
    pub(crate) fn level() -> Level {
        let cached = LEVEL.load(Ordering::Relaxed);
        if cached != UNKNOWN {
            return decode(cached);
        }
        let detected = if std::arch::is_x86_feature_detected!("avx512f") {
            AVX512
        } else if std::arch::is_x86_feature_detected!("avx2") {
            AVX2
        } else {
            SCALAR
        };
        LEVEL.store(detected, Ordering::Relaxed);
        decode(detected)
    }

    #[inline]
    const fn decode(level: u8) -> Level {
        match level {
            AVX512 => Level::Avx512,
            AVX2 => Level::Avx2,
            _ => Level::Scalar,
        }
    }
}

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
#[path = "test_vectors.rs"]
mod test_vectors;

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
