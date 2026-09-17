//! ML-DSA-87 post-quantum signatures (FIPS 204, security category 5).
//!
//! See [`MlDsa87SigningKey`] and [`MlDsa87VerifyingKey`] for the signing and
//! verification APIs.

use super::mldsa::{self, MlDsaError, MlDsaKeyMaterial, PARAMS_87};

/// Size in bytes of an encoded ML-DSA-87 public key.
pub const ML_DSA_87_PUBLIC_KEY_SIZE: usize = 2592;
/// Size in bytes of an encoded ML-DSA-87 signature.
pub const ML_DSA_87_SIGNATURE_SIZE: usize = 4627;
/// Size in bytes of an ML-DSA-87 seed (private key).
pub const ML_DSA_87_SEED_SIZE: usize = mldsa::SEED_SIZE;
/// Maximum length in bytes of an ML-DSA-87 context string.
pub const ML_DSA_87_CONTEXT_MAX_LEN: usize = mldsa::CONTEXT_MAX_LEN;

const K: usize = 8;
const L: usize = 7;

/// An ML-DSA-87 verifying (public) key.
///
/// Verification is stateless. Use [`MlDsa87VerifyingKey::verify`] for a message
/// and optional context, or [`MlDsa87VerifyingKey::verify_external_mu`] for a
/// precomputed 64-byte message representative.
///
/// ```
/// # use crypto::mldsa::MlDsa87SigningKey;
/// # let seed = [0u8; 32];
/// let mut key = MlDsa87SigningKey::new();
/// key.init(&seed);
/// let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
/// assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MlDsa87VerifyingKey {
    bytes: [u8; ML_DSA_87_PUBLIC_KEY_SIZE],
}

impl MlDsa87VerifyingKey {
    /// Creates a verifying key from its 2592-byte encoded form.
    pub fn from_bytes(bytes: &[u8; ML_DSA_87_PUBLIC_KEY_SIZE]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    /// Returns the 2592-byte encoded form of this key.
    pub fn to_bytes(&self) -> [u8; ML_DSA_87_PUBLIC_KEY_SIZE] {
        self.bytes
    }

    /// Verifies an ML-DSA-87 `signature` over `message` with the optional
    /// context `ctx`.
    ///
    /// Returns [`MlDsaError::InvalidSignature`] if the signature is invalid and
    /// [`MlDsaError::ContextTooLong`] if `ctx` exceeds 255 bytes.
    pub fn verify(
        &self,
        message: &[u8],
        signature: &[u8; ML_DSA_87_SIGNATURE_SIZE],
        ctx: &[u8],
    ) -> Result<(), MlDsaError> {
        mldsa::verify_message::<K, L, ML_DSA_87_PUBLIC_KEY_SIZE, ML_DSA_87_SIGNATURE_SIZE>(
            &PARAMS_87,
            &self.bytes,
            message,
            signature,
            ctx,
        )
    }

    /// Verifies an ML-DSA-87 `signature` over a precomputed 64-byte message
    /// representative `mu` (FIPS 204 "external mu" verification).
    ///
    /// `mu` must be the output of the FIPS 204 message-representative
    /// computation; this function performs no domain separation or hashing.
    ///
    /// Returns [`MlDsaError::InvalidSignature`] if the signature is invalid.
    pub fn verify_external_mu(
        &self,
        mu: &[u8; 64],
        signature: &[u8; ML_DSA_87_SIGNATURE_SIZE],
    ) -> Result<(), MlDsaError> {
        mldsa::verify_external_mu::<K, L, ML_DSA_87_PUBLIC_KEY_SIZE, ML_DSA_87_SIGNATURE_SIZE>(
            &PARAMS_87,
            &self.bytes,
            mu,
            signature,
        )
    }
}

impl From<&[u8; ML_DSA_87_PUBLIC_KEY_SIZE]> for MlDsa87VerifyingKey {
    fn from(bytes: &[u8; ML_DSA_87_PUBLIC_KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa87VerifyingKey {
    type Error = MlDsaError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; ML_DSA_87_PUBLIC_KEY_SIZE] = bytes.try_into().map_err(|_| MlDsaError::InvalidPublicKey)?;
        Ok(Self::from_bytes(bytes))
    }
}

/// Size in bytes of an initialized [`MlDsa87SigningKey`].
///
/// Useful for sizing `static`/arena storage on memory-constrained targets.
pub const ML_DSA_87_SIGNING_KEY_SIZE: usize = core::mem::size_of::<MlDsa87SigningKey>();

/// An expanded ML-DSA-87 signing key.
///
/// This is the only way to sign: key generation runs once when the key is
/// initialized, and the resulting matrix `A` and secret vectors in the NTT
/// domain are cached, so signing does not repeat the expensive key generation.
///
/// The key is a plain fixed-size value (about [`ML_DSA_87_SIGNING_KEY_SIZE`]
/// bytes) and never allocates, which makes it usable on `no_std` and embedded
/// targets. On constrained devices place it in a `static` with
/// [`MlDsa87SigningKey::new`] and initialize it once with
/// [`MlDsa87SigningKey::init`] or [`MlDsa87SigningKey::generate`]; signing then
/// only needs a shared reference.
///
/// Secrets are zeroized on drop when the `zeroize` feature is enabled.
#[derive(Debug)]
pub struct MlDsa87SigningKey {
    inner: MlDsaKeyMaterial<K, L, ML_DSA_87_PUBLIC_KEY_SIZE>,
}

impl MlDsa87SigningKey {
    /// Creates a zeroed, uninitialized signing key.
    ///
    /// This is a `const fn` so the (large) key can be placed in a `static` or
    /// another caller-owned location. The key must be initialized with
    /// [`MlDsa87SigningKey::init`] or [`MlDsa87SigningKey::generate`] before
    /// signing; signing an uninitialized key yields a signature that does not
    /// verify.
    pub const fn new() -> Self {
        Self {
            inner: MlDsaKeyMaterial::new(),
        }
    }

    /// Expands `seed` into a signing key, overwriting any previous state.
    ///
    /// This runs the full FIPS 204 key generation and caches the NTT-domain
    /// matrix and secret vectors. Call it once per key; re-initializing the
    /// same value is allowed and simply replaces the previous key.
    pub fn init(&mut self, seed: &[u8; ML_DSA_87_SEED_SIZE]) {
        self.inner.init(&PARAMS_87, seed);
    }

    /// Generates a random seed, initializes the key from it, and returns the
    /// seed so it can be persisted.
    #[cfg(feature = "random")]
    pub fn generate(&mut self) -> [u8; ML_DSA_87_SEED_SIZE] {
        self.inner.generate(&PARAMS_87)
    }

    /// Returns the verifying (public) key for this signing key.
    pub fn public_key(&self) -> MlDsa87VerifyingKey {
        MlDsa87VerifyingKey::from_bytes(self.inner.public_key())
    }

    /// Returns the 32-byte seed this key was initialized from.
    pub fn seed(&self) -> &[u8; ML_DSA_87_SEED_SIZE] {
        self.inner.seed()
    }

    /// Signs `message` with a fresh random nonce.
    ///
    /// `ctx` is the optional FIPS 204 context string and must be at most 255
    /// bytes; it returns [`MlDsaError::ContextTooLong`] otherwise.
    #[cfg(feature = "random")]
    pub fn sign(&self, message: &[u8], ctx: &[u8]) -> Result<[u8; ML_DSA_87_SIGNATURE_SIZE], MlDsaError> {
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
    ) -> Result<[u8; ML_DSA_87_SIGNATURE_SIZE], MlDsaError> {
        let mut sig = [0u8; ML_DSA_87_SIGNATURE_SIZE];
        self.inner.sign_derand_into(&PARAMS_87, message, ctx, rnd, &mut sig)?;
        Ok(sig)
    }

    /// Signs a precomputed 64-byte message representative `mu` (FIPS 204
    /// "external mu" signing) with a fresh random nonce.
    ///
    /// `mu` must be the output of the FIPS 204 message-representative
    /// computation; this function performs no domain separation or hashing.
    #[cfg(feature = "random")]
    pub fn sign_external_mu(&self, mu: &[u8; 64]) -> [u8; ML_DSA_87_SIGNATURE_SIZE] {
        let rnd: [u8; 32] = crate::random::random_bytes();
        self.sign_external_mu_derand(mu, &rnd)
    }

    /// Signs a precomputed 64-byte message representative `mu` (FIPS 204
    /// "external mu" signing) deterministically for a fixed `rnd`.
    pub fn sign_external_mu_derand(&self, mu: &[u8; 64], rnd: &[u8; 32]) -> [u8; ML_DSA_87_SIGNATURE_SIZE] {
        let mut sig = [0u8; ML_DSA_87_SIGNATURE_SIZE];
        self.inner.sign_external_mu_derand_into(&PARAMS_87, mu, rnd, &mut sig);
        sig
    }
}
