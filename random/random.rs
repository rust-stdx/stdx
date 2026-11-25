//! Fast and secure random number generation.
//!
//! This crate provides four pieces:
//!
//! - [`ChaChaRng`]: a deterministic, seedable generator based on the 12-round
//!   ChaCha stream cipher.
//! - [`SystemRng`]: to get random data directly from operating system's entropy source.
//! - [`thread_rng`]: a fast thread-local ChaCha12 generator that is periodically re-seeded
//!   from the operating system.
//! - [`RngExt`]: convenience methods such as [`random`](RngExt::random),
//!   [`random_range`](RngExt::random_range) and [`fill`](RngExt::fill).
//!
//! # Quick start
//!
//! ```
//! # #[cfg(feature = "std")]
//! # {
//! use random::RngExt;
//!
//! let mut rng = random::thread_rng();
//! let value: u64 = rng.random();
//! # let _ = value;
//! # }
//! ```
//!
//! For a deterministic generator, use [`ChaChaRng`]:
//!
//! ```
//! use random::{ChaChaRng, Rng, RngExt, SeedableRng};
//!
//! let mut rng = ChaChaRng::seed_from_u64(42);
//!
//! let value: u64 = rng.random();
//! let percentage: u8 = rng.random_range(0u8..100u8);
//! let mut key = [0u8; 32];
//! rng.fill_bytes(&mut key);
//!
//! assert!(percentage < 100);
//! ```
//!
//! # Seeds
//!
//! [`ChaChaRng`] accepts a 40-byte seed: the first 32 bytes are the ChaCha key
//! and the last 8 bytes are the nonce. Seeds created from a
//! `u64` are expanded to 40 bytes with SplitMix64. Deterministic seeds use
//! whatever nonce is supplied; the thread-local generator draws a fresh key and
//! nonce from the operating system on every re-seed.
//!
//! # Security
//!
//! [`ChaChaRng`] and [`thread_rng`] use the ChaCha stream cipher with 12 rounds.
//! [`thread_rng`] preiodically re-seeds from the operating system, so its
//! output provides forward secrecy and is suitable for generating UUIDs, nonce
//! seeds and other cryptographic material. [`ChaChaRng`] is deterministic and
//! never re-seeds; use it for tests, simulations and reproducible workloads,
//! not as a long-running cryptographic generator.
//!
//! [`SystemRng`] is a direct, fallible view of the operating system entropy
//! source. Prefer [`thread_rng`] when generating more than a few dozen bytes,
//! because it amortizes the cost of system calls.
//!
//! # `no_std`
//!
//! With `default-features = false`, only [`ChaChaRng`] and the traits are
//! available. `ChaChaRng` can be seeded from a hardware generator provided by a HAL or SDK:
//!
//! ```
//! use random::{ChaChaRng, SeedableRng, TryRng};
//!
//! struct HardwareRng;
//!
//! impl TryRng for HardwareRng {
//!     type Error = core::convert::Infallible;
//!
//!     fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
//!         // Read from the hardware peripheral.
//!         Ok(0)
//!     }
//! }
//!
//! let mut hardware = HardwareRng;
//! let rng = ChaChaRng::try_from_rng(&mut hardware).unwrap();
//! ```
//!
//! Alternatively, pass a 40-byte seed directly with
//! [`SeedableRng::from_seed`].
//!
//! # Features
//!
//! | Feature | Enables |
//! | --- | --- |
//! | `std` | Thread-local storage and the global helpers (implies `system`) |
//! | `system` | [`SystemRng`], backed by `getrandom` |
//!
//! On `wasm32-unknown-unknown`, the `getrandom` JavaScript backend is selected
//! automatically by target configuration.
//!
//! # Panics
//!
//! [`RngExt::random_range`] panics when given an empty range. [`thread_rng`]
//! panics on first use if the operating system cannot provide entropy.

#![no_std]

#[cfg(any(feature = "std", test))]
extern crate std;

mod rng;
pub mod rngs;

pub use rng::{Random, Rng, RngExt, SampleRange, SeedableRng, TryRng};
pub use rngs::ChaChaRng;
#[cfg(feature = "system")]
pub use rngs::SystemRng;

#[cfg(feature = "std")]
mod thread;

#[cfg(feature = "std")]
pub use thread::thread_rng;

/// Produces a random value from the thread-local generator.
///
/// This is a convenience wrapper around
/// [`thread_rng`] and [`RngExt::random`]. Use [`thread_rng`] directly when
/// producing several values in a row.
///
/// # Panics
///
/// Panics if the operating system cannot provide entropy on the first use in
/// the current thread.
#[cfg(feature = "std")]
#[inline]
pub fn random<T: Random>() -> T {
    thread_rng().random()
}

/// Produces a random value uniformly distributed in `range` from the
/// thread-local generator.
///
/// # Panics
///
/// Panics if `range` is empty, or if the operating system cannot provide
/// entropy on the first use in the current thread.
#[cfg(feature = "std")]
#[inline]
pub fn random_range<T, R: SampleRange<T>>(range: R) -> T {
    thread_rng().random_range(range)
}
