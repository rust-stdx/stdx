//! BLAKE3: a fast cryptographic hash function with support for keyed hashing
//! (MAC), key derivation and extendable output (XOF).
//!
//! Pure-Rust implementation without heap allocation: the incremental [`Blake3`] state
//! lives entirely on the stack (about 1.9 KiB, plus a few KiB of short-lived
//! scratch while `update` compresses a wide subtree).
//!
//! # Backends
//!
//! The tree structure of BLAKE3 lets several chunks be hashed in parallel:
//!
//! | Target        | Backend                          | Chunks in parallel |
//! |---------------|----------------------------------|--------------------|
//! | `x86_64`      | AVX-512 (runtime-detected with `std`) | 16           |
//! | `x86_64`      | AVX2 (runtime-detected with `std`) | 8                |
//! | `aarch64`     | NEON (always available)          | 4                  |
//! | `wasm32`      | `simd128` (compile-time)         | 4                  |
//! | anything else | portable scalar                  | 1                  |
//!
//! The extendable-output (XOF) reader also compresses several 64-byte output
//! blocks in parallel on those backends, and buffers the unread remainder of
//! the current block, so reading a stream in small pieces does not repeat
//! compression work.
//!
//! On `x86_64`, runtime detection requires the `std` feature; without it the
//! backend is chosen at compile time from `target_feature`. The AVX-512 kernel
//! uses only the Foundation subset (never VL/BW/DQ), so it is selected by the
//! `avx512f` feature alone and runs on every CPU that advertises it. See the
//! `avx512` module for why VL/BW/DQ are not required for these kernels and the
//! one path (single-block compression) where VL would help. On `wasm32` the
//! `simd128` kernel is compiled only when `-C target-feature=+simd128` is set.
//! The portable fallback is written with 32-bit words so that it maps well onto
//! 32-bit platforms.
//!
//! The `std` runtime check also verifies that the operating system saves the
//! AVX-512 vector and mask-register state, which is what makes the wide kernel
//! safe across context switches and signal delivery.
//!
//! # One-shot API
//!
//! ```ignore
//! use crypto::{Hasher, blake3::Blake3};
//!
//! let hash = Blake3::hash(b"hello world");
//! ```
//!
//! # Incremental API
//!
//! ```ignore
//! use crypto::{Hasher, blake3::Blake3};
//!
//! let mut hasher = Blake3::new();
//! hasher.update(b"hello ");
//! hasher.update(b"world");
//! let hash = hasher.sum();
//! ```
//!
//! # XOF (Extendable-Output Function) API
//!
//! ```ignore
//! use crypto::{Hasher, blake3::Blake3};
//!
//! let mut out = [0u8; 64];
//! let mut hasher = Blake3::new();
//! hasher.update(b"hello world");
//! hasher.finalize_xof().squeeze(&mut out);
//! ```
//!
//! # Keyed hashing
//!
//! ```ignore
//! use crypto::{Hasher, blake3::Blake3};
//!
//! let key = b"whats the Elvish word for friend";
//! let mut hasher = Blake3::new_keyed(key);
//! hasher.update(b"hello world");
//! let hash = hasher.sum();
//! ```
//!
//! The [`keyed_hash`](Blake3::keyed_hash) one-shot is also available:
//!
//! ```ignore
//! use crypto::blake3::Blake3;
//!
//! let key = b"whats the Elvish word for friend";
//! let hash = Blake3::keyed_hash(key, b"hello world");
//! ```
//!
//! # Derive-key mode
//!
//! ```ignore
//! use crypto::blake3::Blake3;
//!
//! let key = Blake3::derive_key("my app context", b"key material");
//! let mut hasher = Blake3::new_keyed(&key);
//! hasher.update(b"hello world");
//! let hash = hasher.sum();
//! ```
//!
//! # Security notes
//!
//! * Outputs are truncated to [`Hasher::OUTPUT_SIZE`] bytes; shorter outputs
//!   provide proportionally less security. Outputs longer than 32 bytes (via
//!   the XOF API) do not add any security margin.
//! * The XOF seek position is not secret. An attacker who knows the message
//!   (and key, if any) can recover the offset of an extended output block, so
//!   do not treat the position as a secret.
//! * Inputs are limited to less than 2^64 bytes. All counter arithmetic wraps
//!   rather than panicking, so a longer input (only reachable with more than
//!   2^64 bytes of input) yields an unspecified digest instead of a crash.
//! * [`Blake3::derive_key`] is **not** a password hash; use Argon2 or similar
//!   for passwords.

mod platform;

mod scalar;

// aarch64 assumes that NEON instructions are always present
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod neon;

// import if the target runtime supports the feature (wasm has no runtime detection)
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod wasm32_simd128;

// import if runtime CPU detection is enabled or if the target CPU supports the feature
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
mod avx512;

// import if runtime CPU detection is enabled or if the target CPU supports the feature
#[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
mod avx2;

mod hasher;

#[cfg(test)]
mod tests;

use crate::{Bytes, Hash, Hasher, Xof};

/// BLAKE3 hash function.
///
/// Implements the [`Hasher`] trait. See the [module documentation](self) for
/// examples and backend details.
///
/// When the `zeroize` feature is enabled, the internal state (key words, chunk
/// buffer and chaining-value stack) is overwritten with zeros when the hasher is
/// dropped, so key material does not linger in memory.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Blake3 {
    hasher: hasher::Hasher,
}

impl Blake3 {
    /// Create a keyed hasher (MAC) from a 32-byte key.
    #[inline]
    pub fn new_keyed(key: &[u8; 32]) -> Self {
        Self {
            hasher: hasher::Hasher::new_keyed(key),
        }
    }

    /// Create a key-derivation hasher from a context string.
    ///
    /// The context string should be hardcoded, globally unique and
    /// application-specific; see [`derive_key`](Blake3::derive_key).
    #[inline]
    pub fn new_derive_key(context: &str) -> Self {
        Self {
            hasher: hasher::Hasher::new_derive_key(context),
        }
    }

    /// Derive a 32-byte subkey from `key_material` and a context string.
    ///
    /// The context string should be hardcoded, globally unique and
    /// application-specific, e.g. `"example.com 2019-12-25 16:18:03 session
    /// tokens v1"`. This function is not a password hash.
    #[inline]
    pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; 32] {
        hasher::derive_key(context, key_material)
    }

    /// One-shot keyed hash (MAC).
    #[inline]
    pub fn keyed_hash(key: &[u8; 32], input: &[u8]) -> Hash {
        Hash(Bytes::<64>::from(hasher::keyed_hash(key, input)))
    }

    /// Finalize the hasher and return an extendable-output reader.
    #[inline]
    pub fn finalize_xof(self) -> Blake3XofReader {
        Blake3XofReader {
            reader: self.hasher.finalize_xof(),
        }
    }
}

impl Hasher for Blake3 {
    const BLOCK_SIZE: usize = 64;
    const OUTPUT_SIZE: usize = 32;

    #[inline]
    fn new() -> Self {
        Self {
            hasher: hasher::Hasher::new(),
        }
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    #[inline]
    fn sum(self) -> Hash {
        Hash(Bytes::<64>::from(self.hasher.finalize()))
    }

    #[inline]
    fn hash(data: &[u8]) -> Hash {
        Hash(Bytes::<64>::from(hasher::hash(data)))
    }
}

/// BLAKE3 extensible-output function (XOF) reader.
///
/// Produced by [`Blake3::finalize_xof`]. Implements the [`Xof`] trait.
///
/// **Note:** a BLAKE3 XOF is an *output* stream, so [`Xof::absorb`] is a no-op
/// and any bytes passed to it are silently ignored. Add input through
/// [`Blake3::update`](Hasher::update) before finalizing instead.
///
/// ```ignore
/// use crypto::{blake3::Blake3, Xof};
///
/// let mut hasher = Blake3::new();
/// hasher.update(b"hello world");
/// let mut xof = hasher.finalize_xof();
/// let mut out = [0u8; 64];
/// xof.squeeze(&mut out);
/// ```
///
/// When the `zeroize` feature is enabled, the retained root/node state — which
/// for a keyed hash includes key material — is overwritten with zeros when the
/// reader is dropped.
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Blake3XofReader {
    reader: hasher::OutputReader,
}

impl Xof for Blake3XofReader {
    /// No-op: a XOF reader cannot absorb data. Any bytes passed here are
    /// ignored; pass them to [`Blake3::update`](Hasher::update) before
    /// finalizing instead.
    fn absorb(&mut self, _data: &[u8]) {}

    /// Fill `out` with bytes.
    ///
    /// The unread remainder of the current 64-byte output block is buffered, so
    /// squeezing in small pieces does not repeat compression work. Supplying
    /// several whole blocks at once is still the fastest option: the SIMD
    /// backends then compress multiple output blocks per pass.
    fn squeeze(&mut self, out: &mut [u8]) {
        self.reader.fill(out);
    }
}
