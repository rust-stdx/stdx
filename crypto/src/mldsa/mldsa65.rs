//! ML-DSA-65 post-quantum signatures (FIPS 204, security category 3).
//!
//! See [`MlDsa65SecretKey`] and [`MlDsa65PublicKey`] for the signing and
//! verification APIs.

use super::mldsa::{self, MlDsaError, MlDsaKeyMaterial, PARAMS_65};

/// Size in bytes of an encoded ML-DSA-65 public key.
pub const ML_DSA_65_PUBLIC_KEY_SIZE: usize = 1952;
/// Size in bytes of an encoded ML-DSA-65 signature.
pub const ML_DSA_65_SIGNATURE_SIZE: usize = 3309;
/// Size in bytes of an ML-DSA-65 seed (private key).
pub const ML_DSA_65_SEED_SIZE: usize = mldsa::SEED_SIZE;
/// Maximum length in bytes of an ML-DSA-65 context string.
pub const ML_DSA_65_CONTEXT_MAX_LEN: usize = mldsa::CONTEXT_MAX_LEN;

const K: usize = 6;
const L: usize = 5;

/// An ML-DSA-65 public key.
///
/// Verification is stateless. Use [`MlDsa65PublicKey::verify`] for a message
/// and optional context, or [`MlDsa65PublicKey::verify_external_mu`] for a
/// precomputed 64-byte message representative.
///
/// ```
/// # use crypto::mldsa::MlDsa65SecretKey;
/// # let seed = [0u8; 32];
/// let key = MlDsa65SecretKey::new(&seed);
/// let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
/// assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MlDsa65PublicKey {
    bytes: [u8; ML_DSA_65_PUBLIC_KEY_SIZE],
}

impl MlDsa65PublicKey {
    /// Creates a public key from its 1952-byte encoded form.
    pub fn from_bytes(bytes: &[u8; ML_DSA_65_PUBLIC_KEY_SIZE]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    /// Returns the 1952-byte encoded form of this key.
    pub fn to_bytes(&self) -> [u8; ML_DSA_65_PUBLIC_KEY_SIZE] {
        self.bytes
    }

    /// Verifies an ML-DSA-65 `signature` over `message` with the optional
    /// context `ctx`.
    ///
    /// Returns [`MlDsaError::InvalidSignature`] if the signature is invalid and
    /// [`MlDsaError::ContextTooLong`] if `ctx` exceeds 255 bytes.
    pub fn verify(
        &self,
        message: &[u8],
        signature: &[u8; ML_DSA_65_SIGNATURE_SIZE],
        ctx: &[u8],
    ) -> Result<(), MlDsaError> {
        mldsa::verify_message::<K, L, ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SIGNATURE_SIZE>(
            &PARAMS_65,
            &self.bytes,
            message,
            signature,
            ctx,
        )
    }

    /// Verifies an ML-DSA-65 `signature` over a precomputed 64-byte message
    /// representative `mu` (FIPS 204 "external mu" verification).
    ///
    /// `mu` must be the output of the FIPS 204 message-representative
    /// computation; this function performs no domain separation or hashing.
    ///
    /// Returns [`MlDsaError::InvalidSignature`] if the signature is invalid.
    pub fn verify_external_mu(
        &self,
        mu: &[u8; 64],
        signature: &[u8; ML_DSA_65_SIGNATURE_SIZE],
    ) -> Result<(), MlDsaError> {
        mldsa::verify_external_mu::<K, L, ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SIGNATURE_SIZE>(
            &PARAMS_65,
            &self.bytes,
            mu,
            signature,
        )
    }
}

impl From<&[u8; ML_DSA_65_PUBLIC_KEY_SIZE]> for MlDsa65PublicKey {
    fn from(bytes: &[u8; ML_DSA_65_PUBLIC_KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa65PublicKey {
    type Error = MlDsaError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; ML_DSA_65_PUBLIC_KEY_SIZE] = bytes.try_into().map_err(|_| MlDsaError::InvalidPublicKey)?;
        Ok(Self::from_bytes(bytes))
    }
}

/// Size in bytes of an initialized [`MlDsa65SecretKey`].
///
/// Useful for sizing caller-owned/arena storage on memory-constrained targets.
pub const ML_DSA_65_SECRET_KEY_SIZE: usize = core::mem::size_of::<MlDsa65SecretKey>();

/// An expanded ML-DSA-65 secret key.
///
/// This is the only way to sign. Key generation runs once, in
/// [`MlDsa65SecretKey::new`] or [`MlDsa65SecretKey::generate`], and the
/// resulting matrix `A` and secret vectors in the NTT domain are cached, so
/// signing does not repeat the expensive key generation.
///
/// The key is a plain fixed-size value (about [`ML_DSA_65_SECRET_KEY_SIZE`]
/// bytes) that never allocates, which makes it usable on `no_std` and embedded
/// targets.
///
/// Secrets are zeroized on drop when the `zeroize` feature is enabled.
#[derive(Debug)]
pub struct MlDsa65SecretKey {
    inner: MlDsaKeyMaterial<K, L, ML_DSA_65_PUBLIC_KEY_SIZE>,
}

impl MlDsa65SecretKey {
    /// Expands `seed` into a secret key, running the full FIPS 204 key
    /// generation and caching the NTT-domain matrix and secret vectors.
    pub fn new(seed: &[u8; ML_DSA_65_SEED_SIZE]) -> Self {
        Self {
            inner: MlDsaKeyMaterial::from_seed(&PARAMS_65, seed),
        }
    }

    /// Generates a random secret key.
    ///
    /// The seed can be retrieved afterwards with [`MlDsa65SecretKey::seed`]
    /// so it can be persisted.
    #[cfg(feature = "random")]
    pub fn generate() -> Self {
        Self {
            inner: MlDsaKeyMaterial::from_random(&PARAMS_65),
        }
    }

    /// Returns the public key for this secret key.
    pub fn public_key(&self) -> MlDsa65PublicKey {
        MlDsa65PublicKey::from_bytes(self.inner.public_key())
    }

    /// Returns the 32-byte seed this key was initialized from.
    pub fn seed(&self) -> &[u8; ML_DSA_65_SEED_SIZE] {
        self.inner.seed()
    }

    /// Signs `message` with a fresh random nonce.
    ///
    /// `ctx` is the optional FIPS 204 context string and must be at most 255
    /// bytes; it returns [`MlDsaError::ContextTooLong`] otherwise.
    #[cfg(feature = "random")]
    pub fn sign(&self, message: &[u8], ctx: &[u8]) -> Result<[u8; ML_DSA_65_SIGNATURE_SIZE], MlDsaError> {
        let rnd: [u8; 32] = crate::random::random_bytes();
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
    ) -> Result<[u8; ML_DSA_65_SIGNATURE_SIZE], MlDsaError> {
        let mut sig = [0u8; ML_DSA_65_SIGNATURE_SIZE];
        self.inner.sign_derand_into(&PARAMS_65, message, ctx, rnd, &mut sig)?;
        Ok(sig)
    }

    /// Signs a precomputed 64-byte message representative `mu` (FIPS 204
    /// "external mu" signing) with a fresh random nonce.
    ///
    /// `mu` must be the output of the FIPS 204 message-representative
    /// computation; this function performs no domain separation or hashing.
    #[cfg(feature = "random")]
    pub fn sign_external_mu(&self, mu: &[u8; 64]) -> [u8; ML_DSA_65_SIGNATURE_SIZE] {
        let rnd: [u8; 32] = crate::random::random_bytes();
        self.sign_external_mu_derand(mu, &rnd)
    }

    /// Signs a precomputed 64-byte message representative `mu` (FIPS 204
    /// "external mu" signing) deterministically for a fixed `rnd`.
    pub fn sign_external_mu_derand(&self, mu: &[u8; 64], rnd: &[u8; 32]) -> [u8; ML_DSA_65_SIGNATURE_SIZE] {
        let mut sig = [0u8; ML_DSA_65_SIGNATURE_SIZE];
        self.inner.sign_external_mu_derand_into(&PARAMS_65, mu, rnd, &mut sig);
        sig
    }
}
