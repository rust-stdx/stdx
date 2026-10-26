//! Composite ML-DSA: ML-DSA-65 combined with Ed25519
//! (`id-MLDSA65-Ed25519-SHA512`), as specified in
//! [draft-ietf-lamps-pq-composite-sigs-19].
//!
//! A composite signature is a single algorithm whose public key, private key
//! and signature are the concatenation of the component encodings:
//!
//! | Value      | Encoding                                  | Size      |
//! | ---        | ---                                       | ---       |
//! | Public key | ML-DSA-65 public key ‖ Ed25519 public key  | 1984 B    |
//! | Private key| ML-DSA seed ‖ Ed25519 seed                 | 64 B      |
//! | Signature  | ML-DSA-65 signature ‖ Ed25519 signature    | 3373 B    |
//!
//! # Construction
//!
//! Both components sign the same message representative, which binds the
//! pre-hashed message to the application context and to this specific
//! algorithm label:
//!
//! ```text
//! M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ SHA-512(M)
//! ```
//!
//! `Prefix` is the fixed ASCII string `CompositeAlgorithmSignatures2025` and
//! `Label` is `COMPSIG-MLDSA65-Ed25519-SHA512`. The ML-DSA component signs
//! `M'` with `Label` as its FIPS 204 context string, while the Ed25519
//! component signs `M'` in pure (non-pre-hashed) mode.
//!
//! # Signing
//!
//! Build a private key from its interoperable 64-byte encoding with
//! [`MlDsa65Ed25519Sha512SecretKey::new`] (or with
//! [`MlDsa65Ed25519Sha512SecretKey::generate`] for a fresh random key), then
//! call [`MlDsa65Ed25519Sha512SecretKey::sign`] (hedged/randomized) or
//! [`MlDsa65Ed25519Sha512SecretKey::sign_derand`] (deterministic for a fixed
//! nonce). Signing is stateless and never allocates.
//!
//! The expanded ML-DSA key is cached inside the private key, so repeated
//! signatures do not repeat key generation. Like the standalone ML-DSA-65
//! key, the expanded secret key is roughly 49 KiB; keep it in long-lived
//! storage or box it rather than creating it in a deeply nested call.
//!
//! ```
//! # use crypto::mldsa::MlDsa65Ed25519Sha512SecretKey;
//! # let seed = [7u8; 64];
//! let key = MlDsa65Ed25519Sha512SecretKey::new(&seed);
//! let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
//! assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
//! ```
//!
//! # Verification
//!
//! [`MlDsa65Ed25519Sha512PublicKey::verify`] returns `Ok(())` only when both
//! component signatures verify. The Ed25519 public key is decoded strictly
//! on every verification, so non-canonical encodings and small-order points
//! are rejected with [`MlDsa65Ed25519Error::InvalidPublicKey`].
//!
//! # Errors
//!
//! The context string may be at most 255 bytes; longer contexts return
//! [`MlDsa65Ed25519Error::ContextTooLong`] before any hashing or signing
//! happens. Component failures are reported through the flattened
//! [`MlDsa65Ed25519Error`] enum.
//!
//! # Security
//!
//! Composite ML-DSA is EUF-CMA secure if at least one component is, but it is
//! **not** SUF-CMA secure against a quantum adversary. Do not use it where
//! strong unforgeability is required. The component keys must be freshly
//! generated for the composite: never reuse an ML-DSA or Ed25519 key that is
//! also used standalone or in another composite, and do not treat the
//! component signatures as independent signatures.
//!
//! [draft-ietf-lamps-pq-composite-sigs-19]: https://datatracker.ietf.org/doc/draft-ietf-lamps-pq-composite-sigs/

use super::{
    MlDsa65PublicKey, MlDsa65SecretKey,
    mldsa::{MlDsaError, SEED_SIZE as ML_DSA_SEED_SIZE},
    mldsa65::{ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SIGNATURE_SIZE},
};
use crate::{Hasher, RandomError, curve25519::ed25519, sha2::Sha512};

/// Size in bytes of an encoded ML-DSA-65-Ed25519-SHA512 public key.
pub const ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE: usize = ML_DSA_65_PUBLIC_KEY_SIZE + ed25519::PUBLIC_KEY_SIZE;

/// Size in bytes of an encoded ML-DSA-65-Ed25519-SHA512 private key.
///
/// This is the interoperable encoding: a 32-byte ML-DSA seed followed by a
/// 32-byte Ed25519 seed.
pub const ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE: usize = ML_DSA_SEED_SIZE + ed25519::SECRET_KEY_SIZE;

/// Size in bytes of an encoded ML-DSA-65-Ed25519-SHA512 signature.
pub const ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE: usize = ML_DSA_65_SIGNATURE_SIZE + ed25519::SIGNATURE_SIZE;

/// Maximum length in bytes of the application context string.
pub const ML_DSA_65_ED25519_SHA512_CONTEXT_MAX_LEN: usize = 255;

/// Size in bytes of an initialized [`MlDsa65Ed25519Sha512SecretKey`].
///
/// Useful for sizing caller-owned/arena storage on memory-constrained targets.
pub const ML_DSA_65_ED25519_SHA512_EXPANDED_SECRET_KEY_SIZE: usize =
    core::mem::size_of::<MlDsa65Ed25519Sha512SecretKey>();

/// Fixed prefix prepended to the message representative (section 2.2).
const PREFIX: &[u8] = b"CompositeAlgorithmSignatures2025";

/// Signature label for `id-MLDSA65-Ed25519-SHA512` (section 6).
const LABEL: &[u8] = b"COMPSIG-MLDSA65-Ed25519-SHA512";

/// Size in bytes of a SHA-512 digest.
const PH_SIZE: usize = 64;

/// Maximum size in bytes of the message representative `M'`.
///
/// `M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ SHA-512(M)`, with `len(ctx)` at
/// most 255 bytes.
pub(crate) const MESSAGE_REPRESENTATIVE_MAX_SIZE: usize =
    PREFIX.len() + LABEL.len() + 1 + ML_DSA_65_ED25519_SHA512_CONTEXT_MAX_LEN + PH_SIZE;

/// Error type for ML-DSA-65-Ed25519-SHA512 operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MlDsa65Ed25519Error {
    /// The context string is longer than 255 bytes.
    ContextTooLong,
    /// The private key encoding has the wrong length.
    InvalidPrivateKey,
    /// A public key is malformed (wrong length, non-canonical encoding or a
    /// small-order Ed25519 point).
    InvalidPublicKey,
    /// A signature is invalid.
    InvalidSignature,
    /// The ML-DSA signing rejection loop exceeded the FIPS 204 limit of 814
    /// iterations. This is statistically impossible in practice and indicates
    /// a fault or a broken random number generator.
    RetryLimitExceeded,
    /// No random number generator is available or it failed.
    Random(RandomError),
}

impl core::fmt::Display for MlDsa65Ed25519Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MlDsa65Ed25519Error::ContextTooLong => write!(f, "context length exceeds 255 bytes"),
            MlDsa65Ed25519Error::InvalidPrivateKey => write!(f, "private key is not valid"),
            MlDsa65Ed25519Error::InvalidPublicKey => write!(f, "public key is not valid"),
            MlDsa65Ed25519Error::InvalidSignature => write!(f, "signature is not valid"),
            MlDsa65Ed25519Error::RetryLimitExceeded => write!(f, "signing retry limit exceeded"),
            MlDsa65Ed25519Error::Random(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for MlDsa65Ed25519Error {}

impl From<MlDsaError> for MlDsa65Ed25519Error {
    fn from(err: MlDsaError) -> Self {
        match err {
            MlDsaError::ContextTooLong => MlDsa65Ed25519Error::ContextTooLong,
            // Component values always have a fixed length here, so a length
            // error can only come from an internal inconsistency.
            MlDsaError::InvalidSignature | MlDsaError::InvalidSignatureLength => MlDsa65Ed25519Error::InvalidSignature,
            MlDsaError::InvalidPublicKey => MlDsa65Ed25519Error::InvalidPublicKey,
            MlDsaError::RetryLimitExceeded => MlDsa65Ed25519Error::RetryLimitExceeded,
            MlDsaError::Random(err) => MlDsa65Ed25519Error::Random(err),
        }
    }
}

impl From<RandomError> for MlDsa65Ed25519Error {
    fn from(err: RandomError) -> Self {
        MlDsa65Ed25519Error::Random(err)
    }
}

/// An ML-DSA-65-Ed25519-SHA512 public key.
///
/// Verification is stateless. Build it from its 1984-byte encoding with
/// [`from_bytes`](Self::from_bytes) and call
/// [`verify`](Self::verify) with a message, signature and optional context.
///
/// The Ed25519 component is validated lazily during verification:
/// [`from_bytes`](Self::from_bytes) does not parse the key material, matching
/// the draft's rule that deserialization does not check well-formedness.
///
/// ```
/// # use crypto::mldsa::{MlDsa65Ed25519Sha512PublicKey, MlDsa65Ed25519Sha512SecretKey};
/// # let seed = [7u8; 64];
/// let key = MlDsa65Ed25519Sha512SecretKey::new(&seed);
/// let signature = key.sign_derand(b"message", b"context", &[0u8; 32]).unwrap();
/// let public_key = MlDsa65Ed25519Sha512PublicKey::from_bytes(&key.public_key().to_bytes());
/// assert!(public_key.verify(b"message", &signature, b"context").is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MlDsa65Ed25519Sha512PublicKey {
    bytes: [u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE],
}

impl MlDsa65Ed25519Sha512PublicKey {
    /// Creates a public key from its 1984-byte encoded form.
    pub fn from_bytes(bytes: &[u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    /// Returns the 1984-byte encoded form of this key.
    pub fn to_bytes(&self) -> [u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE] {
        self.bytes
    }

    /// Verifies an ML-DSA-65-Ed25519-SHA512 `signature` over `message` with
    /// the optional context `ctx`.
    ///
    /// Returns [`MlDsa65Ed25519Error::ContextTooLong`] if `ctx` exceeds 255
    /// bytes, [`MlDsa65Ed25519Error::InvalidPublicKey`] if the Ed25519 public
    /// key cannot be decoded, and [`MlDsa65Ed25519Error::InvalidSignature`] if
    /// either component signature does not verify.
    pub fn verify(
        &self,
        message: &[u8],
        signature: &[u8; ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE],
        ctx: &[u8],
    ) -> Result<(), MlDsa65Ed25519Error> {
        let mut m_prime = [0u8; MESSAGE_REPRESENTATIVE_MAX_SIZE];
        let m_prime_len = message_representative(message, ctx, &mut m_prime)?;
        let m_prime = &m_prime[..m_prime_len];

        let (mldsa_public_key, ed25519_public_key) = split_public_key(&self.bytes);
        let (mldsa_signature, ed25519_signature) = split_signature(signature);

        MlDsa65PublicKey::from_bytes(mldsa_public_key).verify(m_prime, mldsa_signature, LABEL)?;

        let ed25519_public_key =
            ed25519::PublicKey::from_bytes(ed25519_public_key).map_err(|_| MlDsa65Ed25519Error::InvalidPublicKey)?;
        ed25519_public_key
            .verify(m_prime, ed25519_signature)
            .map_err(|_| MlDsa65Ed25519Error::InvalidSignature)?;

        Ok(())
    }
}

impl From<&[u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE]> for MlDsa65Ed25519Sha512PublicKey {
    fn from(bytes: &[u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa65Ed25519Sha512PublicKey {
    type Error = MlDsa65Ed25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE] =
            bytes.try_into().map_err(|_| MlDsa65Ed25519Error::InvalidPublicKey)?;
        Ok(Self::from_bytes(bytes))
    }
}

/// An expanded ML-DSA-65-Ed25519-SHA512 private key.
///
/// The value combines an expanded [`MlDsa65SecretKey`] (which caches the
/// NTT-domain matrix and secret vectors) with an expanded Ed25519 signing key.
/// It is a plain fixed-size value that never allocates, but it is large (about
/// [`ML_DSA_65_ED25519_SHA512_EXPANDED_SECRET_KEY_SIZE`] bytes). so be careful if allocating it on
/// the stack.
///
/// The key is zeroized on drop when the `zeroize` feature is enabled.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct MlDsa65Ed25519Sha512SecretKey {
    mldsa: MlDsa65SecretKey,
    ed25519: ed25519::SecretKey,
}

impl MlDsa65Ed25519Sha512SecretKey {
    /// Expands a private key from its interoperable 64-byte encoding.
    ///
    /// The encoding is a 32-byte ML-DSA seed followed by a 32-byte Ed25519
    /// seed. The same input always produces the same key.
    pub fn new(bytes: &[u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]) -> Self {
        let (mldsa_seed, ed25519_seed) = bytes.split_at(ML_DSA_SEED_SIZE);
        Self::from_seeds(
            mldsa_seed
                .try_into()
                .expect("ML-DSA seed length is checked by split_at"),
            ed25519_seed
                .try_into()
                .expect("Ed25519 seed length is checked by split_at"),
        )
    }

    /// Expands a private key from its two component seeds.
    ///
    /// The resulting key serializes to `mldsa_seed ‖ ed25519_seed`.
    pub fn from_seeds(mldsa_seed: &[u8; ML_DSA_SEED_SIZE], ed25519_seed: &[u8; ed25519::SECRET_KEY_SIZE]) -> Self {
        Self {
            mldsa: MlDsa65SecretKey::new(mldsa_seed),
            ed25519: ed25519::SecretKey::from_bytes(ed25519_seed),
        }
    }

    /// Generates a fresh random private key.
    ///
    /// The 64-byte private key can be retrieved afterwards with
    /// [`to_bytes`](Self::to_bytes) so it can be persisted.
    ///
    /// Returns [`MlDsa65Ed25519Error::Random`] when the operating system's
    /// random number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<Self, MlDsa65Ed25519Error> {
        let bytes: [u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE] = crate::random::bytes()?;
        Ok(Self::new(&bytes))
    }

    /// Returns the 64-byte interoperable encoding of this key.
    pub fn to_bytes(&self) -> [u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE] {
        let mut bytes = [0u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE];
        bytes[..ML_DSA_SEED_SIZE].copy_from_slice(self.mldsa.seed());
        bytes[ML_DSA_SEED_SIZE..].copy_from_slice(&self.ed25519.to_bytes());
        bytes
    }

    /// Returns the public key for this private key.
    pub fn public_key(&self) -> MlDsa65Ed25519Sha512PublicKey {
        let mut bytes = [0u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE];
        bytes[..ML_DSA_65_PUBLIC_KEY_SIZE].copy_from_slice(&self.mldsa.public_key().to_bytes());
        bytes[ML_DSA_65_PUBLIC_KEY_SIZE..].copy_from_slice(&self.ed25519.public_key().to_bytes());
        MlDsa65Ed25519Sha512PublicKey {
            bytes,
        }
    }

    /// Signs `message` with a fresh random nonce.
    ///
    /// `ctx` is the application context string and must be at most 255 bytes.
    ///
    /// Returns [`MlDsa65Ed25519Error::ContextTooLong`] if `ctx` is too long
    /// and [`MlDsa65Ed25519Error::Random`] when the operating system's random
    /// number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn sign(
        &self,
        message: &[u8],
        ctx: &[u8],
    ) -> Result<[u8; ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE], MlDsa65Ed25519Error> {
        let rnd: [u8; 32] = crate::random::bytes()?;
        self.sign_derand(message, ctx, &rnd)
    }

    /// Signs `message` deterministically for a fixed 32-byte `rnd`.
    ///
    /// Passing `rnd = [0u8; 32]` gives the deterministic ML-DSA signing
    /// variant; any other value gives the hedged/randomized variant. The
    /// Ed25519 component is always deterministic. `ctx` must be at most 255
    /// bytes, returning [`MlDsa65Ed25519Error::ContextTooLong`] otherwise.
    pub fn sign_derand(
        &self,
        message: &[u8],
        ctx: &[u8],
        rnd: &[u8; 32],
    ) -> Result<[u8; ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE], MlDsa65Ed25519Error> {
        let mut m_prime = [0u8; MESSAGE_REPRESENTATIVE_MAX_SIZE];
        let m_prime_len = message_representative(message, ctx, &mut m_prime)?;
        let m_prime = &m_prime[..m_prime_len];

        let mldsa_signature = self.mldsa.sign_derand(m_prime, LABEL, rnd)?;
        let ed25519_signature = self.ed25519.sign(m_prime);

        let mut signature = [0u8; ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE];
        signature[..ML_DSA_65_SIGNATURE_SIZE].copy_from_slice(&mldsa_signature);
        signature[ML_DSA_65_SIGNATURE_SIZE..].copy_from_slice(&ed25519_signature);
        Ok(signature)
    }
}

impl From<&[u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]> for MlDsa65Ed25519Sha512SecretKey {
    fn from(bytes: &[u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE]) -> Self {
        Self::new(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa65Ed25519Sha512SecretKey {
    type Error = MlDsa65Ed25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; ML_DSA_65_ED25519_SHA512_PRIVATE_KEY_SIZE] =
            bytes.try_into().map_err(|_| MlDsa65Ed25519Error::InvalidPrivateKey)?;
        Ok(Self::new(bytes))
    }
}

/// Computes `M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ SHA-512(message)` into
/// `out` and returns its length.
///
/// This is exposed to the crate so the test suite can compare the message
/// representative against fixed vectors.
pub(crate) fn message_representative(
    message: &[u8],
    ctx: &[u8],
    out: &mut [u8; MESSAGE_REPRESENTATIVE_MAX_SIZE],
) -> Result<usize, MlDsa65Ed25519Error> {
    if ctx.len() > ML_DSA_65_ED25519_SHA512_CONTEXT_MAX_LEN {
        return Err(MlDsa65Ed25519Error::ContextTooLong);
    }

    let ph = Sha512::hash(message);

    let mut offset = 0;
    out[..PREFIX.len()].copy_from_slice(PREFIX);
    offset += PREFIX.len();
    out[offset..offset + LABEL.len()].copy_from_slice(LABEL);
    offset += LABEL.len();
    out[offset] = ctx.len() as u8;
    offset += 1;
    out[offset..offset + ctx.len()].copy_from_slice(ctx);
    offset += ctx.len();
    out[offset..offset + PH_SIZE].copy_from_slice(ph.as_ref());
    offset += PH_SIZE;

    Ok(offset)
}

fn split_public_key(
    bytes: &[u8; ML_DSA_65_ED25519_SHA512_PUBLIC_KEY_SIZE],
) -> (&[u8; ML_DSA_65_PUBLIC_KEY_SIZE], &[u8; ed25519::PUBLIC_KEY_SIZE]) {
    let (mldsa, ed25519) = bytes.split_at(ML_DSA_65_PUBLIC_KEY_SIZE);
    (
        mldsa
            .try_into()
            .expect("ML-DSA public key length is checked by split_at"),
        ed25519
            .try_into()
            .expect("Ed25519 public key length is checked by split_at"),
    )
}

fn split_signature(
    signature: &[u8; ML_DSA_65_ED25519_SHA512_SIGNATURE_SIZE],
) -> (&[u8; ML_DSA_65_SIGNATURE_SIZE], &[u8; ed25519::SIGNATURE_SIZE]) {
    let (mldsa, ed25519) = signature.split_at(ML_DSA_65_SIGNATURE_SIZE);
    (
        mldsa
            .try_into()
            .expect("ML-DSA signature length is checked by split_at"),
        ed25519
            .try_into()
            .expect("Ed25519 signature length is checked by split_at"),
    )
}

#[cfg(test)]
#[path = "ml_dsa_composite_tests.rs"]
mod tests;
