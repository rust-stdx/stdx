//! ML-DSA-44 post-quantum signatures (FIPS 204, security category 2).
//!
//! See [`MlDsa44SecretKey`] and [`MlDsa44PublicKey`] for the signing and
//! verification APIs.

use super::mldsa::{self, MlDsaError, MlDsaKeyMaterial, PARAMS_44};

/// Size in bytes of an encoded ML-DSA-44 public key.
pub const ML_DSA_44_PUBLIC_KEY_SIZE: usize = 1312;
/// Size in bytes of an encoded ML-DSA-44 signature.
pub const ML_DSA_44_SIGNATURE_SIZE: usize = 2420;
/// Size in bytes of an ML-DSA-44 seed (private key).
pub const ML_DSA_44_SEED_SIZE: usize = mldsa::SEED_SIZE;
/// Maximum length in bytes of an ML-DSA-44 context string.
pub const ML_DSA_44_CONTEXT_MAX_LEN: usize = mldsa::CONTEXT_MAX_LEN;

const K: usize = 4;
const L: usize = 4;

/// An ML-DSA-44 public key.
///
/// Verification is stateless. Use [`MlDsa44PublicKey::verify`] for a message
/// and optional context, or [`MlDsa44PublicKey::verify_external_mu`] for a
/// precomputed 64-byte message representative.
///
/// ```
/// # use crypto::mldsa::MlDsa44SecretKey;
/// # let seed = [0u8; 32];
/// let key = MlDsa44SecretKey::new(&seed);
/// let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
/// assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MlDsa44PublicKey {
    bytes: [u8; ML_DSA_44_PUBLIC_KEY_SIZE],
}

impl MlDsa44PublicKey {
    /// Creates a public key from its 1312-byte encoded form.
    pub fn from_bytes(bytes: &[u8; ML_DSA_44_PUBLIC_KEY_SIZE]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    /// Returns the 1312-byte encoded form of this key.
    pub fn to_bytes(&self) -> [u8; ML_DSA_44_PUBLIC_KEY_SIZE] {
        self.bytes
    }

    /// Verifies an ML-DSA-44 `signature` over `message` with the optional
    /// context `ctx`.
    ///
    /// Returns [`MlDsaError::InvalidSignature`] if the signature is invalid and
    /// [`MlDsaError::ContextTooLong`] if `ctx` exceeds 255 bytes.
    pub fn verify(
        &self,
        message: &[u8],
        signature: &[u8; ML_DSA_44_SIGNATURE_SIZE],
        ctx: &[u8],
    ) -> Result<(), MlDsaError> {
        mldsa::verify_message::<K, L, ML_DSA_44_PUBLIC_KEY_SIZE, ML_DSA_44_SIGNATURE_SIZE>(
            &PARAMS_44,
            &self.bytes,
            message,
            signature,
            ctx,
        )
    }

    /// Verifies an ML-DSA-44 `signature` over a precomputed 64-byte message
    /// representative `mu` (FIPS 204 "external mu" verification).
    ///
    /// `mu` must be the output of the FIPS 204 message-representative
    /// computation; this function performs no domain separation or hashing.
    ///
    /// Returns [`MlDsaError::InvalidSignature`] if the signature is invalid.
    pub fn verify_external_mu(
        &self,
        mu: &[u8; 64],
        signature: &[u8; ML_DSA_44_SIGNATURE_SIZE],
    ) -> Result<(), MlDsaError> {
        mldsa::verify_external_mu::<K, L, ML_DSA_44_PUBLIC_KEY_SIZE, ML_DSA_44_SIGNATURE_SIZE>(
            &PARAMS_44,
            &self.bytes,
            mu,
            signature,
        )
    }
}

impl From<&[u8; ML_DSA_44_PUBLIC_KEY_SIZE]> for MlDsa44PublicKey {
    fn from(bytes: &[u8; ML_DSA_44_PUBLIC_KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa44PublicKey {
    type Error = MlDsaError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; ML_DSA_44_PUBLIC_KEY_SIZE] = bytes.try_into().map_err(|_| MlDsaError::InvalidPublicKey)?;
        Ok(Self::from_bytes(bytes))
    }
}

/// Size in bytes of an initialized [`MlDsa44SecretKey`].
///
/// Useful for sizing caller-owned/arena storage on memory-constrained targets.
pub const ML_DSA_44_SECRET_KEY_SIZE: usize = core::mem::size_of::<MlDsa44SecretKey>();

/// An expanded ML-DSA-44 secret key.
///
/// This is the only way to sign. Key generation runs once, in
/// [`MlDsa44SecretKey::new`] or [`MlDsa44SecretKey::generate`], and the
/// resulting matrix `A` and secret vectors in the NTT domain are cached, so
/// signing does not repeat the expensive key generation.
///
/// The key is a plain fixed-size value (about [`ML_DSA_44_SECRET_KEY_SIZE`]
/// bytes) that never allocates, which makes it usable on `no_std` and embedded
/// targets.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct MlDsa44SecretKey {
    inner: MlDsaKeyMaterial<K, L, ML_DSA_44_PUBLIC_KEY_SIZE>,
}

impl MlDsa44SecretKey {
    /// Expands `seed` into a secret key, running the full FIPS 204 key
    /// generation and caching the NTT-domain matrix and secret vectors.
    pub fn new(seed: &[u8; ML_DSA_44_SEED_SIZE]) -> Self {
        Self {
            inner: MlDsaKeyMaterial::from_seed(&PARAMS_44, seed),
        }
    }

    /// Generates a random secret key.
    ///
    /// The seed can be retrieved afterwards with [`MlDsa44SecretKey::seed`]
    /// so it can be persisted.
    ///
    /// Returns [`MlDsaError::Random`] when the operating system's random
    /// number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<Self, MlDsaError> {
        Ok(Self {
            inner: MlDsaKeyMaterial::from_random(&PARAMS_44)?,
        })
    }

    /// Returns the public key for this secret key.
    pub fn public_key(&self) -> MlDsa44PublicKey {
        MlDsa44PublicKey::from_bytes(self.inner.public_key())
    }

    /// Returns the 32-byte seed this key was initialized from.
    pub fn seed(&self) -> &[u8; ML_DSA_44_SEED_SIZE] {
        self.inner.seed()
    }

    /// Signs `message` with a fresh random nonce.
    ///
    /// `ctx` is the optional FIPS 204 context string and must be at most 255
    /// bytes; it returns [`MlDsaError::ContextTooLong`] otherwise. It returns
    /// [`MlDsaError::Random`] when the operating system's random number
    /// generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn sign(&self, message: &[u8], ctx: &[u8]) -> Result<[u8; ML_DSA_44_SIGNATURE_SIZE], MlDsaError> {
        let rnd: [u8; 32] = crate::random::bytes()?;
        self.sign_derand(message, ctx, &rnd)
    }

    /// Signs `message` deterministically for a fixed 32-byte `rnd`.
    ///
    /// Passing `rnd = [0u8; 32]` gives the deterministic FIPS 204 variant;
    /// any other value gives the hedged/randomized variant. `ctx` must be at
    /// most 255 bytes, returning [`MlDsaError::ContextTooLong`] otherwise.
    pub fn sign_derand(
        &self,
        message: &[u8],
        ctx: &[u8],
        rnd: &[u8; 32],
    ) -> Result<[u8; ML_DSA_44_SIGNATURE_SIZE], MlDsaError> {
        let mut sig = [0u8; ML_DSA_44_SIGNATURE_SIZE];
        self.inner.sign_derand_into(&PARAMS_44, message, ctx, rnd, &mut sig)?;
        Ok(sig)
    }

    /// Signs a precomputed 64-byte message representative `mu` (FIPS 204
    /// "external mu" signing) with a fresh random nonce.
    ///
    /// `mu` must be the output of the FIPS 204 message-representative
    /// computation; this function performs no domain separation or hashing.
    ///
    /// Returns [`MlDsaError::Random`] when the operating system's random
    /// number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn sign_external_mu(&self, mu: &[u8; 64]) -> Result<[u8; ML_DSA_44_SIGNATURE_SIZE], MlDsaError> {
        let rnd: [u8; 32] = crate::random::bytes()?;
        Ok(self.sign_external_mu_derand(mu, &rnd))
    }

    /// Signs a precomputed 64-byte message representative `mu` (FIPS 204
    /// "external mu" signing) deterministically for a fixed `rnd`.
    pub fn sign_external_mu_derand(&self, mu: &[u8; 64], rnd: &[u8; 32]) -> [u8; ML_DSA_44_SIGNATURE_SIZE] {
        let mut sig = [0u8; ML_DSA_44_SIGNATURE_SIZE];
        self.inner.sign_external_mu_derand_into(&PARAMS_44, mu, rnd, &mut sig);
        sig
    }
}
