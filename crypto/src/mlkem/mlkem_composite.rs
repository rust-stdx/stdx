//! Composite ML-KEM-1024 + X25519 hybrid KEM (`MLKEM1024-X25519`).
//!
//! This module implements the UG framework from
//! [draft-irtf-cfrg-hybrid-kems-12]: ML-KEM-1024 is composed with the
//! Curve25519 nominal group from
//! [draft-irtf-cfrg-concrete-hybrid-kems-04] using the `UniversalCombiner`.
//! The construction is inspired by X-Wing, but includes both PQ and
//! traditional ciphertexts and encapsulation keys in the combiner.
//!
//! # Construction
//!
//! | Value | Encoding | Size |
//! | --- | --- | --- |
//! | Decapsulation key | 32-byte seed | 32 B |
//! | Encapsulation key | ML-KEM-1024 `ek` ‖ X25519 public key | 1600 B |
//! | Ciphertext | ML-KEM-1024 `ct` ‖ X25519 ephemeral public key | 1600 B |
//! | Shared secret | `SHA3-256` output | 32 B |
//!
//! Key generation expands the seed with SHAKE256 into 96 bytes. The first
//! 64 bytes are the ML-KEM-1024 key-generation seed (`d ‖ z`) and the last
//! 32 bytes are the X25519 scalar seed:
//!
//! ```text
//! seed_full = SHAKE256(seed, 96)
//! seed_PQ   = seed_full[0..64]
//! seed_T    = seed_full[64..96]
//!
//! (dk_PQ, ek_PQ) = ML-KEM-1024.KeyGen_internal(seed_PQ[0..32], seed_PQ[32..64])
//! dk_T = seed_T
//! ek_T = X25519(dk_T, 9)
//!
//! dk_H = seed
//! ek_H = ek_PQ ‖ ek_T
//! ```
//!
//! Encapsulation and decapsulation use the universal combiner, with
//! `SHA3-256` as the KDF and the 16-byte ASCII label `MLKEM1024-X25519`:
//!
//! ```text
//! ss_H = SHA3-256(ss_PQ ‖ ss_T ‖ ct_PQ ‖ ct_T ‖ ek_PQ ‖ ek_T ‖ LABEL)
//! ```
//!
//! Deterministic encapsulation takes 64 bytes of randomness: the first 32
//! bytes are the ML-KEM-1024 message `m`, and the last 32 bytes seed the
//! ephemeral X25519 scalar.
//!
//! # Rejection behaviour
//!
//! ML-KEM-1024 is implicitly rejecting: an invalid ciphertext produces a
//! deterministic pseudorandom shared secret. The X25519 component is
//! explicitly rejecting: a low-order encapsulation key is rejected by
//! [`MlKem1024X25519PublicKey::from_bytes`], and a low-order ciphertext
//! component makes [`MlKem1024X25519SecretKey::decapsulate`] return
//! [`MlKem1024X25519Error::InvalidCiphertext`]. This follows the explicit
//! rejection path permitted by the framework and by RFC 7748, which allows
//! aborting when the X25519 shared secret is all zeros. Rejection depends only
//! on public data (the key or ciphertext), so it does not leak secrets.
//!
//! Non-canonical X25519 public keys are accepted and reduced modulo the field
//! prime, as required by RFC 7748. The combiner binds the canonical encoding
//! of `ek_T`, but the exact bytes of `ct_T` as received.
//!
//! # Security
//!
//! The hybrid shared secret remains secure if either ML-KEM-1024 or X25519 is
//! secure. A key pair generated here **must not** be reused with ML-KEM-1024,
//! X25519, or another hybrid KEM, and component keys must not be used
//! standalone.
//!
//! [draft-irtf-cfrg-hybrid-kems-12]: https://datatracker.ietf.org/doc/draft-irtf-cfrg-hybrid-kems/
//! [draft-irtf-cfrg-concrete-hybrid-kems-04]: https://datatracker.ietf.org/doc/draft-irtf-cfrg-concrete-hybrid-kems/

use constant_time_eq::constant_time_eq;
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::{
    mlkem::SHARED_SECRET_SIZE,
    mlkem1024::{CIPHERTEXT_SIZE_1024, PUBLIC_KEY_SIZE_1024, PublicKey1024, SecretKey1024},
};
use crate::{
    Hasher, RandomError,
    curve25519::x25519,
    sha3::{Sha3_256, Shake256},
};

/// Size in bytes of a `MLKEM1024-X25519` seed and decapsulation key.
pub const MLKEM_1024_X25519_SEED_SIZE: usize = 32;

/// Size in bytes of an encoded `MLKEM1024-X25519` decapsulation key.
pub const MLKEM_1024_X25519_SECRET_KEY_SIZE: usize = MLKEM_1024_X25519_SEED_SIZE;

/// Size in bytes of an encoded `MLKEM1024-X25519` encapsulation key.
pub const MLKEM_1024_X25519_PUBLIC_KEY_SIZE: usize = PUBLIC_KEY_SIZE_1024 + x25519::KEY_SIZE;

/// Size in bytes of a `MLKEM1024-X25519` ciphertext.
pub const MLKEM_1024_X25519_CIPHERTEXT_SIZE: usize = CIPHERTEXT_SIZE_1024 + x25519::KEY_SIZE;

/// Size in bytes of a `MLKEM1024-X25519` shared secret.
pub const MLKEM_1024_X25519_SHARED_SECRET_SIZE: usize = SHARED_SECRET_SIZE;

/// Size in bytes of the randomness consumed by deterministic encapsulation.
///
/// The first 32 bytes are the ML-KEM-1024 message; the last 32 bytes seed the
/// ephemeral X25519 scalar.
pub const MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE: usize = 32 + x25519::KEY_SIZE;

/// Size in bytes of the SHAKE256 output used to expand a seed.
const EXPANDED_SEED_SIZE: usize = 64 + x25519::KEY_SIZE;

/// ASCII label bound into the universal combiner.
pub const MLKEM_1024_X25519_LABEL: &[u8] = b"MLKEM1024-X25519";

/// Error type for `MLKEM1024-X25519` operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MlKem1024X25519Error {
    /// The decapsulation key encoding has the wrong length, or the internal
    /// ML-KEM-1024 decapsulation-key check failed.
    InvalidSecretKey,
    /// The encapsulation key is malformed: the ML-KEM-1024 modulus check
    /// failed, or the X25519 component is a low-order point.
    InvalidPublicKey,
    /// The X25519 ciphertext component is a low-order point, so the shared
    /// secret would be all zeros.
    InvalidCiphertext,
    /// No random number generator is available or it failed.
    Random(RandomError),
}

impl core::fmt::Display for MlKem1024X25519Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MlKem1024X25519Error::InvalidSecretKey => write!(f, "secret key is not valid"),
            MlKem1024X25519Error::InvalidPublicKey => write!(f, "public key is not valid"),
            MlKem1024X25519Error::InvalidCiphertext => write!(f, "ciphertext is not valid"),
            MlKem1024X25519Error::Random(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for MlKem1024X25519Error {}

impl From<RandomError> for MlKem1024X25519Error {
    fn from(err: RandomError) -> Self {
        MlKem1024X25519Error::Random(err)
    }
}

/// `MLKEM1024-X25519` decapsulation (secret) key.
///
/// The interoperable encoding is a 32-byte seed. The ML-KEM-1024 and X25519
/// component keys are derived from it with SHAKE256, and the expanded
/// components are cached in this value.
///
/// # Example
///
/// ```
/// use crypto::mlkem::MlKem1024X25519SecretKey;
///
/// let (secret_key, public_key) = MlKem1024X25519SecretKey::generate().unwrap();
/// let (shared_secret, ciphertext) = public_key.encapsulate().unwrap();
/// let decapsulated = secret_key.decapsulate(&ciphertext).unwrap();
/// assert_eq!(shared_secret, decapsulated);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct MlKem1024X25519SecretKey {
    seed: [u8; MLKEM_1024_X25519_SEED_SIZE],
    mlkem_secret_key: SecretKey1024,
    x25519_secret_key: x25519::SecretKey,
    x25519_public_key_bytes: [u8; x25519::KEY_SIZE],
}

impl PartialEq for MlKem1024X25519SecretKey {
    fn eq(&self, other: &Self) -> bool {
        // The seed deterministically expands to every other field.
        constant_time_eq(&self.seed, &other.seed)
    }
}

impl Eq for MlKem1024X25519SecretKey {}

impl MlKem1024X25519SecretKey {
    /// Creates a decapsulation key from its 32-byte seed.
    ///
    /// The seed is expanded deterministically; every 32-byte value is a valid
    /// encoding. Use [`MlKem1024X25519SecretKey::generate`] for a fresh random
    /// key.
    pub fn from_bytes(bytes: &[u8; MLKEM_1024_X25519_SECRET_KEY_SIZE]) -> Self {
        Self::expand(bytes).0
    }

    /// Returns the 32-byte seed that encodes this decapsulation key.
    pub fn to_bytes(&self) -> [u8; MLKEM_1024_X25519_SECRET_KEY_SIZE] {
        self.seed
    }

    /// Generates a fresh random keypair.
    ///
    /// Returns [`MlKem1024X25519Error::Random`] when the operating system's
    /// random number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<(Self, MlKem1024X25519PublicKey), MlKem1024X25519Error> {
        let seed: [u8; MLKEM_1024_X25519_SEED_SIZE] = crate::random::bytes()?;
        Ok(Self::generate_derand(&seed))
    }

    /// Generates a keypair deterministically from a 32-byte seed.
    ///
    /// This is intended for test vectors and protocols that need reproducible
    /// key generation. The same seed always produces the same keypair; it must
    /// not be reused for a real key.
    ///
    /// The derived X25519 public key is a low-order point only with
    /// cryptographically negligible probability; if that happened, key
    /// generation still succeeds but encapsulation rejects the key.
    pub fn generate_derand(seed: &[u8; MLKEM_1024_X25519_SEED_SIZE]) -> (Self, MlKem1024X25519PublicKey) {
        Self::expand(seed)
    }

    /// Returns the encapsulation key corresponding to this decapsulation key.
    pub fn public_key(&self) -> MlKem1024X25519PublicKey {
        MlKem1024X25519PublicKey {
            mlkem_public_key: self.mlkem_secret_key.public_key(),
            x25519_public_key: x25519::PublicKey::from_bytes(&self.x25519_public_key_bytes),
        }
    }

    /// Decapsulates `ciphertext`, recovering the shared secret encapsulated in
    /// it.
    ///
    /// # Errors
    ///
    /// Returns [`MlKem1024X25519Error::InvalidCiphertext`] when the X25519
    /// ciphertext component is a low-order point (its shared secret would be
    /// all zeros). Returns [`MlKem1024X25519Error::InvalidSecretKey`] only if
    /// the internally expanded ML-KEM-1024 key fails its consistency check;
    /// this cannot happen for a key produced by this module.
    pub fn decapsulate(
        &self,
        ciphertext: &[u8; MLKEM_1024_X25519_CIPHERTEXT_SIZE],
    ) -> Result<[u8; MLKEM_1024_X25519_SHARED_SECRET_SIZE], MlKem1024X25519Error> {
        let (ct_pq, ct_t) = ciphertext.split_at(CIPHERTEXT_SIZE_1024);
        let ct_pq: &[u8; CIPHERTEXT_SIZE_1024] = ct_pq.try_into().expect("split length is fixed");
        let ct_t: &[u8; x25519::KEY_SIZE] = ct_t.try_into().expect("split length is fixed");

        let ss_pq = self
            .mlkem_secret_key
            .decapsulate(ct_pq)
            .map_err(|_| MlKem1024X25519Error::InvalidSecretKey)?;

        let ct_t_key = x25519::PublicKey::from_bytes(ct_t);
        let ss_t = self
            .x25519_secret_key
            .ecdh(&ct_t_key)
            .map_err(|_| MlKem1024X25519Error::InvalidCiphertext)?;

        let ek_pq = self.mlkem_secret_key.public_key().to_bytes();
        Ok(combiner(&ss_pq, &ss_t, ct_pq, ct_t, &ek_pq, &self.x25519_public_key_bytes))
    }

    fn expand(seed: &[u8; MLKEM_1024_X25519_SEED_SIZE]) -> (Self, MlKem1024X25519PublicKey) {
        let mut expanded = [0u8; EXPANDED_SEED_SIZE];
        Shake256::hash(seed, &mut expanded);

        let mlkem_coins: &[u8; 64] = expanded[..64].try_into().expect("split length is fixed");
        let (mlkem_secret_key, mlkem_public_key) = SecretKey1024::generate_derand(mlkem_coins);

        let x25519_secret_key = x25519::SecretKey::from_bytes(
            expanded[64..EXPANDED_SEED_SIZE]
                .try_into()
                .expect("split length is fixed"),
        );
        let x25519_public_key_bytes = x25519_secret_key.public_key().to_bytes();

        wipe(&mut expanded);

        let secret_key = MlKem1024X25519SecretKey {
            seed: *seed,
            mlkem_secret_key,
            x25519_secret_key,
            x25519_public_key_bytes,
        };
        let public_key = MlKem1024X25519PublicKey {
            mlkem_public_key,
            x25519_public_key: x25519::PublicKey::from_bytes(&x25519_public_key_bytes),
        };
        (secret_key, public_key)
    }
}

impl From<&[u8; MLKEM_1024_X25519_SECRET_KEY_SIZE]> for MlKem1024X25519SecretKey {
    fn from(bytes: &[u8; MLKEM_1024_X25519_SECRET_KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlKem1024X25519SecretKey {
    type Error = MlKem1024X25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(
            bytes.try_into().map_err(|_| MlKem1024X25519Error::InvalidSecretKey)?,
        ))
    }
}

/// `MLKEM1024-X25519` encapsulation (public) key.
///
/// See [`MlKem1024X25519SecretKey`] for a usage example.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MlKem1024X25519PublicKey {
    mlkem_public_key: PublicKey1024,
    x25519_public_key: x25519::PublicKey,
}

impl MlKem1024X25519PublicKey {
    /// Parses an encapsulation key from its 1600-byte encoding
    /// (ML-KEM-1024 encapsulation key followed by the X25519 public key).
    ///
    /// Non-canonical X25519 encodings are reduced modulo the field prime, as
    /// required by RFC 7748. Low-order X25519 points are rejected because they
    /// would make the X25519 shared secret all zeros.
    ///
    /// # Errors
    ///
    /// Returns [`MlKem1024X25519Error::InvalidPublicKey`] when the ML-KEM-1024
    /// part fails the FIPS 203 modulus check or the X25519 part is a low-order
    /// point.
    pub fn from_bytes(bytes: &[u8; MLKEM_1024_X25519_PUBLIC_KEY_SIZE]) -> Result<Self, MlKem1024X25519Error> {
        let (mlkem_bytes, x25519_bytes) = bytes.split_at(PUBLIC_KEY_SIZE_1024);
        let mlkem_public_key = PublicKey1024::from_bytes(mlkem_bytes.try_into().expect("split length is fixed"))
            .map_err(|_| MlKem1024X25519Error::InvalidPublicKey)?;

        let x25519_public_key = x25519::PublicKey::from_bytes(x25519_bytes.try_into().expect("split length is fixed"));
        if x25519_public_key.is_low_order() {
            return Err(MlKem1024X25519Error::InvalidPublicKey);
        }

        Ok(Self {
            mlkem_public_key,
            x25519_public_key,
        })
    }

    /// Returns the 1600-byte encoding of this encapsulation key.
    pub fn to_bytes(&self) -> [u8; MLKEM_1024_X25519_PUBLIC_KEY_SIZE] {
        let mut bytes = [0u8; MLKEM_1024_X25519_PUBLIC_KEY_SIZE];
        bytes[..PUBLIC_KEY_SIZE_1024].copy_from_slice(&self.mlkem_public_key.to_bytes());
        bytes[PUBLIC_KEY_SIZE_1024..].copy_from_slice(&self.x25519_public_key.to_bytes());
        bytes
    }

    /// Encapsulates a fresh random shared secret against this public key.
    ///
    /// Returns [`MlKem1024X25519Error::Random`] when the operating system's
    /// random number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn encapsulate(
        &self,
    ) -> Result<
        (
            [u8; MLKEM_1024_X25519_SHARED_SECRET_SIZE],
            [u8; MLKEM_1024_X25519_CIPHERTEXT_SIZE],
        ),
        MlKem1024X25519Error,
    > {
        let randomness: [u8; MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE] = crate::random::bytes()?;
        self.encapsulate_derand(&randomness)
    }

    /// Encapsulates a shared secret using the given 64-byte randomness.
    ///
    /// The first 32 bytes are the ML-KEM-1024 message and the last 32 bytes
    /// seed the ephemeral X25519 scalar. The same randomness must not be
    /// reused across encapsulations.
    ///
    /// # Errors
    ///
    /// Returns [`MlKem1024X25519Error::InvalidPublicKey`] if the (already
    /// validated) X25519 key produces an all-zero shared secret. This requires
    /// either a malformed key or an ephemeral scalar congruent to zero, and is
    /// cryptographically negligible.
    pub fn encapsulate_derand(
        &self,
        randomness: &[u8; MLKEM_1024_X25519_ENCAPS_RANDOMNESS_SIZE],
    ) -> Result<
        (
            [u8; MLKEM_1024_X25519_SHARED_SECRET_SIZE],
            [u8; MLKEM_1024_X25519_CIPHERTEXT_SIZE],
        ),
        MlKem1024X25519Error,
    > {
        let (mlkem_coins, x25519_seed) = randomness.split_at(32);
        let (ss_pq, ct_pq) = self
            .mlkem_public_key
            .encapsulate_derand(mlkem_coins.try_into().expect("split length is fixed"));

        let ephemeral = x25519::SecretKey::from_bytes(x25519_seed.try_into().expect("split length is fixed"));
        let ct_t = ephemeral.public_key().to_bytes();
        let ss_t = ephemeral
            .ecdh(&self.x25519_public_key)
            .map_err(|_| MlKem1024X25519Error::InvalidPublicKey)?;

        let ss = combiner(
            &ss_pq,
            &ss_t,
            &ct_pq,
            &ct_t,
            &self.mlkem_public_key.to_bytes(),
            &self.x25519_public_key.to_bytes(),
        );

        let mut ciphertext = [0u8; MLKEM_1024_X25519_CIPHERTEXT_SIZE];
        ciphertext[..CIPHERTEXT_SIZE_1024].copy_from_slice(&ct_pq);
        ciphertext[CIPHERTEXT_SIZE_1024..].copy_from_slice(&ct_t);
        Ok((ss, ciphertext))
    }
}

impl TryFrom<&[u8]> for MlKem1024X25519PublicKey {
    type Error = MlKem1024X25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes.try_into().map_err(|_| MlKem1024X25519Error::InvalidPublicKey)?)
    }
}

/// The UG `UniversalCombiner` with SHA3-256 as the KDF.
#[inline]
fn combiner(
    ss_pq: &[u8; SHARED_SECRET_SIZE],
    ss_t: &[u8; x25519::SHARED_SECRET_SIZE],
    ct_pq: &[u8; CIPHERTEXT_SIZE_1024],
    ct_t: &[u8; x25519::KEY_SIZE],
    ek_pq: &[u8; PUBLIC_KEY_SIZE_1024],
    ek_t: &[u8; x25519::KEY_SIZE],
) -> [u8; SHARED_SECRET_SIZE] {
    let mut hasher = Sha3_256::new();
    hasher.update(ss_pq);
    hasher.update(ss_t);
    hasher.update(ct_pq);
    hasher.update(ct_t);
    hasher.update(ek_pq);
    hasher.update(ek_t);
    hasher.update(MLKEM_1024_X25519_LABEL);
    hasher.sum().as_ref().try_into().expect("SHA3-256 output is 32 bytes")
}

/// Best-effort zeroization of intermediate secret material.
#[inline]
fn wipe(bytes: &mut [u8]) {
    #[cfg(feature = "zeroize")]
    {
        bytes.zeroize();
    }
    #[cfg(not(feature = "zeroize"))]
    {
        let _ = bytes;
    }
}

#[cfg(test)]
#[path = "mlkem_composite_tests.rs"]
mod tests;
