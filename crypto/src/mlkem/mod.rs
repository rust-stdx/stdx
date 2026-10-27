//! ML-KEM post-quantum key encapsulation mechanism (KEM) standardized in FIPS 203.
//!
//! This module implements the `ML-KEM-768` and `ML-KEM-1024` parameter sets.
//! ML-KEM-512 is not implemented, mostly due to security concerns and recommendations against it
//! by the security community.
//!
//! It also implements the `MLKEM1024-X25519` composite KEM, which combines
//! ML-KEM-1024 with X25519 using the universal combiner from
//! draft-irtf-cfrg-hybrid-kems. See [`MlKem1024X25519SecretKey`] for details.
//!
//! Encapsulation keys are validated when parsed with
//! [`PublicKey768::from_bytes`] / [`PublicKey1024::from_bytes`] (FIPS 203
//! Section 7.2 modulus check). Decapsulation keys are checked for consistency
//! with `H(ek) == h` when they are used (FIPS 203 Section 7.3); parsing a
//! decapsulation key with `from_bytes` or `TryFrom` only checks its length.
//!
//! All operations use fixed-size stack buffers and never allocate on the heap.
//! Key generation and encapsulation materialize the `K x K` matrix, so they
//! need more stack than decapsulation; allow a few tens of KiB on small platforms.

mod mlkem;
mod mlkem1024;
mod mlkem768;
mod mlkem_composite;

pub use mlkem::{MlKemError, SHARED_SECRET_SIZE};
pub use mlkem_composite::{
    MLKEM_1024_X25519_CIPHERTEXT_SIZE, MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE, MLKEM_1024_X25519_LABEL,
    MLKEM_1024_X25519_PUBLIC_KEY_SIZE, MLKEM_1024_X25519_SECRET_KEY_SIZE, MLKEM_1024_X25519_SEED_SIZE,
    MLKEM_1024_X25519_SHARED_SECRET_SIZE, MlKem1024X25519Error, MlKem1024X25519PublicKey, MlKem1024X25519SecretKey,
};
pub use mlkem768::{CIPHERTEXT_SIZE_768, PUBLIC_KEY_SIZE_768, PublicKey768, SECRET_KEY_SIZE_768, SecretKey768};
pub use mlkem1024::{CIPHERTEXT_SIZE_1024, PUBLIC_KEY_SIZE_1024, PublicKey1024, SECRET_KEY_SIZE_1024, SecretKey1024};
