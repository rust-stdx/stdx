//! Argon2id (RFC 9106) password hashing function.
//!
//! Argon2id is a memory-hard password hashing function that provides resistance
//! against both side-channel attacks and GPU/ASIC brute-force attacks.
//!
//! The memory-filling phase automatically selects the fastest compression
//! kernel available on the running CPU: NEON (optionally with the SHA-3 `xar`
//! extension) on AArch64, AVX2 on x86-64, and a portable scalar implementation
//! everywhere else. When the `std` feature is enabled, the lanes are also
//! filled in parallel using `p` threads.

mod argon2;

pub use argon2::{Argon2Error, Params, decode_phc, derive_key, encode_phc, hash_password, verify_password};
