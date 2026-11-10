//! ML-KEM post-quantum key encapsulation mechanism (KEM) standardized in FIPS 203.
//!
//! This module implements the `ML-KEM-768` and `ML-KEM-1024` parameter sets.
//! ML-KEM-512 is not implemented, mostly due to security concerns and recommendations against it
//! by the security community.
//!
//! It also implements X-Wing, which combines ML-KEM-768 with X25519 as
//! specified in draft-connolly-cfrg-xwing-kem. See [`XWingSecretKey`] for
//! details.
//!
//! Finally, it implements the `MLKEM1024-X25519` composite KEM, which combines
//! ML-KEM-1024 with X25519 using the universal combiner from
//! draft-irtf-cfrg-hybrid-kems. See [`MlKem1024X25519SecretKey`] for details.
//!
//! | Parameter set | Secret key | Public key | Public key size | Ciphertext size |
//! | --- | --- | --- | --- | --- |
//! | ML-KEM-768 | [`SecretKey768`] | [`PublicKey768`] | 1184 B | 1088 B |
//! | ML-KEM-1024 | [`SecretKey1024`] | [`PublicKey1024`] | 1568 B | 1568 B |
//! | X-Wing (MLKEM768-X25519) | [`XWingSecretKey`] | [`XWingPublicKey`] | 1216 B | 1120 B |
//! | MLKEM1024-X25519 | [`MlKem1024X25519SecretKey`] | [`MlKem1024X25519PublicKey`] | 1600 B | 1600 B |
//!
//! Shared secrets are always [`SHARED_SECRET_SIZE`] bytes (32 B). The composite
//! and X-Wing secret keys are 32-byte seeds.
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
mod xwing;

pub use mlkem::{MlKemError, SHARED_SECRET_SIZE};
pub use mlkem_composite::{
    MLKEM_1024_X25519_CIPHERTEXT_SIZE, MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE, MLKEM_1024_X25519_LABEL,
    MLKEM_1024_X25519_PUBLIC_KEY_SIZE, MLKEM_1024_X25519_SECRET_KEY_SIZE, MLKEM_1024_X25519_SEED_SIZE,
    MLKEM_1024_X25519_SHARED_SECRET_SIZE, MlKem1024X25519Error, MlKem1024X25519PublicKey, MlKem1024X25519SecretKey,
};
pub use mlkem768::{CIPHERTEXT_SIZE_768, PUBLIC_KEY_SIZE_768, PublicKey768, SECRET_KEY_SIZE_768, SecretKey768};
pub use mlkem1024::{CIPHERTEXT_SIZE_1024, PUBLIC_KEY_SIZE_1024, PublicKey1024, SECRET_KEY_SIZE_1024, SecretKey1024};
#[cfg(feature = "random")]
pub use xwing::generate_xwing_keypair;
pub use xwing::{
    XWING_CIPHERTEXT_SIZE, XWING_PUBLIC_KEY_SIZE, XWING_SECRET_KEY_SIZE, XWING_SHARED_SECRET_SIZE, XWingError,
    XWingPublicKey, XWingSecretKey, generate_xwing_keypair_derand,
};
