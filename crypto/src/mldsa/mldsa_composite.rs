//! Composite ML-DSA signatures: ML-DSA combined with Ed25519.
//!
//! This module implements two composite algorithms built with the signature
//! combiner from [draft-ietf-lamps-pq-composite-sigs-19]:
//!
//! | Algorithm | Public key | Private key | Signature |
//! | --- | --- | --- | --- |
//! | `id-MLDSA65-Ed25519-SHA512` | 1984 B | 64 B | 3373 B |
//! | `MLDSA87-Ed25519-PARALLELHASH256` | 2624 B | 64 B | 4691 B |
//!
//! # Standard status
//!
//! `id-MLDSA65-Ed25519-SHA512` is fully specified by the draft. The
//! `MLDSA87-Ed25519-PARALLELHASH256` variant is a custom extension: draft-19
//! does not pair ML-DSA-87 with Ed25519 and does not define a ParallelHash256
//! pre-hash, and no OID has been assigned. It reuses the draft's construction
//! with ML-DSA-87, Ed25519 and ParallelHash256 (4096-byte block, 64-byte
//! output). It is interoperable only between peers that agree on those exact
//! parameters and label; do not use it where a standardized composite
//! algorithm is required.
//!
//! # Encodings
//!
//! A composite public key, private key and signature are the concatenation of
//! the component encodings:
//!
//! | Value | Encoding |
//! | --- | --- |
//! | Public key | ML-DSA public key ‖ Ed25519 public key |
//! | Private key | ML-DSA seed ‖ Ed25519 seed |
//! | Signature | ML-DSA signature ‖ Ed25519 signature |
//!
//! # Construction
//!
//! Both components sign the same message representative, which binds the
//! pre-hashed message to the application context and to this specific
//! algorithm label:
//!
//! ```text
//! M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ PH(M)
//! ```
//!
//! `Prefix` is the fixed ASCII string `CompositeAlgorithmSignatures2025`. The
//! label is `COMPSIG-MLDSA65-Ed25519-SHA512` or
//! `COMPSIG-MLDSA87-Ed25519-PARALLELHASH256`. `PH` is SHA-512 for the
//! ML-DSA-65 scheme, and ParallelHash256 with a 4096-byte block size, a
//! 64-byte output and the label as its customization string for the ML-DSA-87
//! scheme. The ML-DSA component signs `M'` with `Label` as its FIPS 204
//! context string, while the Ed25519 component signs `M'` in pure
//! (non-pre-hashed) mode.
//!
//! # Signing
//!
//! Build a private key from its interoperable 64-byte encoding with
//! [`MlDsa65Ed25519Sha512SecretKey::new`] or
//! [`MlDsa87Ed25519SecretKey::new`] (or use `generate` for a fresh random
//! key), then call `sign` (hedged/randomized) or `sign_derand` (deterministic
//! for a fixed nonce). Signing is stateless and never allocates.
//!
//! The expanded ML-DSA key is cached inside the private key, so repeated
//! signatures do not repeat key generation. The expanded secret key is about
//! 49 KiB for the ML-DSA-65 scheme and 82 KiB for the ML-DSA-87 scheme; keep
//! it in long-lived storage or box it rather than creating it in a deeply
//! nested call. ML-DSA-87 verification also needs roughly 106 KiB of transient
//! stack.
//!
//! ```
//! # use crypto::mldsa::{MlDsa65Ed25519Sha512SecretKey, MlDsa87Ed25519SecretKey};
//! # let seed = [7u8; 64];
//! {
//!     let key = MlDsa65Ed25519Sha512SecretKey::new(&seed);
//!     let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
//!     assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
//! }
//! {
//!     let key = MlDsa87Ed25519SecretKey::new(&seed);
//!     let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
//!     assert!(key.public_key().verify(b"message", &signature, b"").is_ok());
//! }
//! ```
//!
//! # Pre-hashing large messages
//!
//! `MlDsa87Ed25519SecretKey::sign_prehashed` and
//! [`MlDsa87Ed25519PublicKey::verify_prehashed`] accept an already-computed
//! 64-byte `PH(M)`, so a large message can be streamed through
//! [`ParallelHash256`](crate::sha3::ParallelHash256) instead of being held in
//! memory. [`MlDsa87Ed25519MessageRepresentative::hasher`] returns a hasher
//! already configured with the scheme's block size and customization string:
//!
//! ```
//! # use crypto::mldsa::{
//! #     MlDsa87Ed25519MessageRepresentative, MlDsa87Ed25519SecretKey,
//! # };
//! # let seed = [7u8; 64];
//! let key = MlDsa87Ed25519SecretKey::new(&seed);
//!
//! let mut hasher = MlDsa87Ed25519MessageRepresentative::hasher();
//! hasher.update(b"first chunk");
//! hasher.update(b"second chunk");
//! let mut prehash = [0u8; 64];
//! hasher.finalize_into(&mut prehash);
//!
//! let signature = key.sign_prehashed_derand(&prehash, b"", &[0u8; 32]).unwrap();
//! assert!(key.public_key().verify_prehashed(&prehash, &signature, b"").is_ok());
//! ```
//!
//! The pre-hash must use exactly the scheme's parameters. A digest computed
//! with a different block size, output length or customization string
//! produces a signature that will not verify.
//!
//! # External component signing
//!
//! Either component signature can be produced outside this crate, for example
//! by a hardware security module, an accelerator or a secure enclave. The crate
//! owns the domain-separated message representative `M'` and the final
//! encoding:
//!
//! 1. Build `M'` with [`MlDsa87Ed25519MessageRepresentative::from_message`]
//!    (or [`from_prehash`](MlDsa87Ed25519MessageRepresentative::from_prehash)
//!    for streamed messages).
//! 2. Have the ML-DSA-87 module sign `M'` as a message with
//!    [`MlDsa87Ed25519MessageRepresentative::LABEL`] as its FIPS 204 context
//!    string. Modules with an Apple CryptoKit-style API must use
//!    `signature(for:context:)`: the context-less overload signs with an empty
//!    context and does not produce a valid component.
//! 3. Have the Ed25519 module sign `M'` in pure (non-pre-hashed) mode.
//! 4. Combine the two results with
//!    [`MlDsa87Ed25519Signature::from_parts`].
//!
//! Either component can also stay local: use
//! [`MlDsa87Ed25519SecretKey::sign_mldsa_component`] or
//! [`MlDsa87Ed25519SecretKey::sign_ed25519_component`] for the half that is
//! not delegated.
//!
//! ```
//! # use crypto::curve25519::ed25519;
//! # use crypto::mldsa::{
//! #     ML_DSA_87_SEED_SIZE, MlDsa87Ed25519MessageRepresentative, MlDsa87Ed25519SecretKey,
//! #     MlDsa87Ed25519Signature, MlDsa87SecretKey,
//! # };
//! # let seed = [7u8; MlDsa87Ed25519SecretKey::SIZE];
//! let key = MlDsa87Ed25519SecretKey::new(&seed);
//! let public_key = key.public_key();
//! let message = b"message";
//! let ctx = b"context";
//!
//! // 1. The host builds the message representative once.
//! let m_prime = MlDsa87Ed25519MessageRepresentative::from_message(message, ctx).unwrap();
//!
//! // 2. The ML-DSA-87 module signs M' with the scheme label as context.
//! //    Simulated here with the component key and the standard API.
//! let mut mldsa_seed = [0u8; ML_DSA_87_SEED_SIZE];
//! mldsa_seed.copy_from_slice(&seed[..ML_DSA_87_SEED_SIZE]);
//! let mldsa_signature = MlDsa87SecretKey::new(&mldsa_seed)
//!     .sign_derand(
//!         m_prime.as_bytes(),
//!         MlDsa87Ed25519MessageRepresentative::LABEL,
//!         &[0u8; 32],
//!     )
//!     .unwrap();
//!
//! // 3. The Ed25519 module signs the same representative in pure mode.
//! let mut ed25519_seed = [0u8; ed25519::SECRET_KEY_SIZE];
//! ed25519_seed.copy_from_slice(&seed[ML_DSA_87_SEED_SIZE..]);
//! let ed25519_signature = ed25519::SecretKey::from_bytes(&ed25519_seed).sign(m_prime.as_bytes());
//!
//! // 4. The host combines the two component signatures.
//! let signature = MlDsa87Ed25519Signature::from_parts(&mldsa_signature, &ed25519_signature);
//! public_key.verify(message, &signature, ctx).unwrap();
//! ```
//!
//! # Verification
//!
//! [`MlDsa65Ed25519Sha512PublicKey::verify`] and
//! [`MlDsa87Ed25519PublicKey::verify`] return `Ok(())` only when both
//! component signatures verify. The Ed25519 public key is decoded strictly on
//! every verification, so non-canonical encodings and small-order points are
//! rejected with `InvalidPublicKey`.
//!
//! # Errors
//!
//! The context string may be at most 255 bytes; longer contexts return
//! `ContextTooLong` before any hashing or signing happens. Component failures
//! are reported through the flattened error enums.
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
//! `M'` is derived from a public message and is not secret, but a module that
//! signs it on demand is a signing oracle for its component key. When a
//! component is delegated, restrict the module to `M'` values produced for
//! this composite and this exact context: never expose raw component signing
//! to untrusted callers, and do not reuse the module (or its key) for any
//! other protocol.
//!
//! [draft-ietf-lamps-pq-composite-sigs-19]: https://datatracker.ietf.org/doc/draft-ietf-lamps-pq-composite-sigs/

use super::{
    MlDsa65PublicKey, MlDsa65SecretKey, MlDsa87PublicKey, MlDsa87SecretKey,
    mldsa::{MlDsaError, SEED_SIZE as ML_DSA_SEED_SIZE},
    mldsa65::{ML_DSA_65_PUBLIC_KEY_SIZE, ML_DSA_65_SIGNATURE_SIZE},
    mldsa87::{ML_DSA_87_PUBLIC_KEY_SIZE, ML_DSA_87_SIGNATURE_SIZE},
};
use crate::{Hasher, RandomError, curve25519::ed25519, sha2::Sha512, sha3::ParallelHash256};

/// Fixed prefix prepended to the message representative (section 2.2).
const PREFIX: &[u8] = b"CompositeAlgorithmSignatures2025";

/// Size in bytes of a SHA-512 digest, which is also the size of the
/// ParallelHash256 pre-hash.
const PH_SIZE: usize = 64;

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
    bytes: [u8; MlDsa65Ed25519Sha512PublicKey::SIZE],
}

impl MlDsa65Ed25519Sha512PublicKey {
    /// Size in bytes of an encoded public key.
    pub const SIZE: usize = ML_DSA_65_PUBLIC_KEY_SIZE + ed25519::PUBLIC_KEY_SIZE;

    /// Size in bytes of an encoded signature.
    pub const SIGNATURE_SIZE: usize = ML_DSA_65_SIGNATURE_SIZE + ed25519::SIGNATURE_SIZE;

    /// Maximum length in bytes of the application context string.
    pub const CONTEXT_MAX_LEN: usize = 255;

    /// Signature label for `id-MLDSA65-Ed25519-SHA512` (section 6).
    ///
    /// The ML-DSA component uses it as its FIPS 204 context string.
    pub const LABEL: &[u8] = b"COMPSIG-MLDSA65-Ed25519-SHA512";

    /// Maximum size in bytes of the message representative `M'`.
    ///
    /// `M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ SHA-512(M)`, with `len(ctx)` at
    /// most 255 bytes.
    pub(crate) const MESSAGE_REPRESENTATIVE_MAX_SIZE: usize =
        PREFIX.len() + Self::LABEL.len() + 1 + Self::CONTEXT_MAX_LEN + PH_SIZE;

    /// Creates a public key from its 1984-byte encoded form.
    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    /// Returns the 1984-byte encoded form of this key.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
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
        signature: &[u8; Self::SIGNATURE_SIZE],
        ctx: &[u8],
    ) -> Result<(), MlDsa65Ed25519Error> {
        let mut m_prime = [0u8; Self::MESSAGE_REPRESENTATIVE_MAX_SIZE];
        let m_prime_len = message_representative(message, ctx, &mut m_prime)?;
        let m_prime = &m_prime[..m_prime_len];

        let (mldsa_public_key, ed25519_public_key) = split_public_key(&self.bytes);
        let (mldsa_signature, ed25519_signature) = split_signature(signature);

        MlDsa65PublicKey::from_bytes(mldsa_public_key).verify(m_prime, mldsa_signature, Self::LABEL)?;

        let ed25519_public_key =
            ed25519::PublicKey::from_bytes(ed25519_public_key).map_err(|_| MlDsa65Ed25519Error::InvalidPublicKey)?;
        ed25519_public_key
            .verify(m_prime, ed25519_signature)
            .map_err(|_| MlDsa65Ed25519Error::InvalidSignature)?;

        Ok(())
    }
}

impl From<&[u8; MlDsa65Ed25519Sha512PublicKey::SIZE]> for MlDsa65Ed25519Sha512PublicKey {
    fn from(bytes: &[u8; MlDsa65Ed25519Sha512PublicKey::SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa65Ed25519Sha512PublicKey {
    type Error = MlDsa65Ed25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; MlDsa65Ed25519Sha512PublicKey::SIZE] =
            bytes.try_into().map_err(|_| MlDsa65Ed25519Error::InvalidPublicKey)?;
        Ok(Self::from_bytes(bytes))
    }
}

/// An expanded ML-DSA-65-Ed25519-SHA512 private key.
///
/// The value combines an expanded [`MlDsa65SecretKey`] (which caches the
/// NTT-domain matrix and secret vectors) with an expanded Ed25519 signing key.
/// It is a plain fixed-size value that never allocates, but it is large (about
/// [`MlDsa65Ed25519Sha512SecretKey::EXPANDED_SIZE`] bytes), so be careful if
/// allocating it on the stack.
///
/// The key is zeroized on drop when the `zeroize` feature is enabled.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct MlDsa65Ed25519Sha512SecretKey {
    mldsa: MlDsa65SecretKey,
    ed25519: ed25519::SecretKey,
}

impl MlDsa65Ed25519Sha512SecretKey {
    /// Size in bytes of the interoperable private-key encoding.
    ///
    /// This is a 32-byte ML-DSA seed followed by a 32-byte Ed25519 seed.
    pub const SIZE: usize = ML_DSA_SEED_SIZE + ed25519::SECRET_KEY_SIZE;

    /// Size in bytes of an initialized key.
    ///
    /// Useful for sizing caller-owned/arena storage on memory-constrained
    /// targets.
    pub const EXPANDED_SIZE: usize = core::mem::size_of::<Self>();

    /// Expands a private key from its interoperable 64-byte encoding.
    ///
    /// The encoding is a 32-byte ML-DSA seed followed by a 32-byte Ed25519
    /// seed. The same input always produces the same key.
    pub fn new(bytes: &[u8; Self::SIZE]) -> Self {
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
        let bytes: [u8; Self::SIZE] = crate::random::bytes()?;
        Ok(Self::new(&bytes))
    }

    /// Returns the 64-byte interoperable encoding of this key.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes[..ML_DSA_SEED_SIZE].copy_from_slice(self.mldsa.seed());
        bytes[ML_DSA_SEED_SIZE..].copy_from_slice(&self.ed25519.to_bytes());
        bytes
    }

    /// Returns the public key for this private key.
    pub fn public_key(&self) -> MlDsa65Ed25519Sha512PublicKey {
        let mut bytes = [0u8; MlDsa65Ed25519Sha512PublicKey::SIZE];
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
    ) -> Result<[u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE], MlDsa65Ed25519Error> {
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
    ) -> Result<[u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE], MlDsa65Ed25519Error> {
        let mut m_prime = [0u8; MlDsa65Ed25519Sha512PublicKey::MESSAGE_REPRESENTATIVE_MAX_SIZE];
        let m_prime_len = message_representative(message, ctx, &mut m_prime)?;
        let m_prime = &m_prime[..m_prime_len];

        let mldsa_signature = self
            .mldsa
            .sign_derand(m_prime, MlDsa65Ed25519Sha512PublicKey::LABEL, rnd)?;
        let ed25519_signature = self.ed25519.sign(m_prime);

        let mut signature = [0u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE];
        signature[..ML_DSA_65_SIGNATURE_SIZE].copy_from_slice(&mldsa_signature);
        signature[ML_DSA_65_SIGNATURE_SIZE..].copy_from_slice(&ed25519_signature);
        Ok(signature)
    }
}

impl From<&[u8; MlDsa65Ed25519Sha512SecretKey::SIZE]> for MlDsa65Ed25519Sha512SecretKey {
    fn from(bytes: &[u8; MlDsa65Ed25519Sha512SecretKey::SIZE]) -> Self {
        Self::new(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa65Ed25519Sha512SecretKey {
    type Error = MlDsa65Ed25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; MlDsa65Ed25519Sha512SecretKey::SIZE] =
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
    out: &mut [u8; MlDsa65Ed25519Sha512PublicKey::MESSAGE_REPRESENTATIVE_MAX_SIZE],
) -> Result<usize, MlDsa65Ed25519Error> {
    if ctx.len() > MlDsa65Ed25519Sha512PublicKey::CONTEXT_MAX_LEN {
        return Err(MlDsa65Ed25519Error::ContextTooLong);
    }

    let mut ph = [0u8; PH_SIZE];
    ph.copy_from_slice(Sha512::hash(message).as_ref());

    Ok(write_message_representative(
        MlDsa65Ed25519Sha512PublicKey::LABEL,
        &ph,
        ctx,
        out,
    ))
}

/// Writes `M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ prehash` into `out` and
/// returns its length.
///
/// The caller is responsible for checking the context length.
fn write_message_representative(label: &[u8], prehash: &[u8; PH_SIZE], ctx: &[u8], out: &mut [u8]) -> usize {
    let mut offset = 0;
    out[..PREFIX.len()].copy_from_slice(PREFIX);
    offset += PREFIX.len();
    out[offset..offset + label.len()].copy_from_slice(label);
    offset += label.len();
    out[offset] = ctx.len() as u8;
    offset += 1;
    out[offset..offset + ctx.len()].copy_from_slice(ctx);
    offset += ctx.len();
    out[offset..offset + PH_SIZE].copy_from_slice(prehash);
    offset += PH_SIZE;

    offset
}

fn split_public_key(
    bytes: &[u8; MlDsa65Ed25519Sha512PublicKey::SIZE],
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
    signature: &[u8; MlDsa65Ed25519Sha512PublicKey::SIGNATURE_SIZE],
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

//////////////////////////////////////////////////////////////////////////////////////////////////
// ML-DSA-87 + Ed25519 + ParallelHash256
//////////////////////////////////////////////////////////////////////////////////////////////////

/// Error type for ML-DSA-87-Ed25519-ParallelHash256 operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MlDsa87Ed25519Error {
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

impl core::fmt::Display for MlDsa87Ed25519Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MlDsa87Ed25519Error::ContextTooLong => write!(f, "context length exceeds 255 bytes"),
            MlDsa87Ed25519Error::InvalidPrivateKey => write!(f, "private key is not valid"),
            MlDsa87Ed25519Error::InvalidPublicKey => write!(f, "public key is not valid"),
            MlDsa87Ed25519Error::InvalidSignature => write!(f, "signature is not valid"),
            MlDsa87Ed25519Error::RetryLimitExceeded => write!(f, "signing retry limit exceeded"),
            MlDsa87Ed25519Error::Random(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for MlDsa87Ed25519Error {}

impl From<MlDsaError> for MlDsa87Ed25519Error {
    fn from(err: MlDsaError) -> Self {
        match err {
            MlDsaError::ContextTooLong => MlDsa87Ed25519Error::ContextTooLong,
            // Component values always have a fixed length here, so a length
            // error can only come from an internal inconsistency.
            MlDsaError::InvalidSignature | MlDsaError::InvalidSignatureLength => MlDsa87Ed25519Error::InvalidSignature,
            MlDsaError::InvalidPublicKey => MlDsa87Ed25519Error::InvalidPublicKey,
            MlDsaError::RetryLimitExceeded => MlDsa87Ed25519Error::RetryLimitExceeded,
            MlDsaError::Random(err) => MlDsa87Ed25519Error::Random(err),
        }
    }
}

impl From<RandomError> for MlDsa87Ed25519Error {
    fn from(err: RandomError) -> Self {
        MlDsa87Ed25519Error::Random(err)
    }
}

/// An ML-DSA-87-Ed25519-ParallelHash256 public key.
///
/// This is a local extension of
/// [draft-ietf-lamps-pq-composite-sigs-19]: draft-19 does not define the
/// ML-DSA-87 + Ed25519 pairing or a ParallelHash256 pre-hash. The encoding is
/// the 2592-byte ML-DSA-87 public key followed by the 32-byte Ed25519 public
/// key.
///
/// Verification is stateless. Build the key from its 2624-byte encoding with
/// [`from_bytes`](Self::from_bytes) and call [`verify`](Self::verify) with a
/// message, signature and optional context.
///
/// The Ed25519 component is validated lazily during verification:
/// [`from_bytes`](Self::from_bytes) does not parse the key material, matching
/// the draft's rule that deserialization does not check well-formedness.
///
/// ```
/// # use crypto::mldsa::MlDsa87Ed25519SecretKey;
/// # let seed = [7u8; 64];
/// let key = MlDsa87Ed25519SecretKey::new(&seed);
/// let signature = key.sign_derand(b"message", b"context", &[0u8; 32]).unwrap();
/// assert!(key.public_key().verify(b"message", &signature, b"context").is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MlDsa87Ed25519PublicKey {
    bytes: [u8; MlDsa87Ed25519PublicKey::SIZE],
}

impl MlDsa87Ed25519PublicKey {
    /// Size in bytes of an encoded public key.
    pub const SIZE: usize = ML_DSA_87_PUBLIC_KEY_SIZE + ed25519::PUBLIC_KEY_SIZE;

    /// Creates a public key from its 2624-byte encoded form.
    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    /// Returns the 2624-byte encoded form of this key.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.bytes
    }

    /// Verifies an ML-DSA-87-Ed25519-ParallelHash256 `signature` over
    /// `message` with the optional context `ctx`.
    ///
    /// Returns [`MlDsa87Ed25519Error::ContextTooLong`] if `ctx` exceeds 255
    /// bytes, [`MlDsa87Ed25519Error::InvalidPublicKey`] if the Ed25519 public
    /// key cannot be decoded, and [`MlDsa87Ed25519Error::InvalidSignature`] if
    /// either component signature does not verify.
    pub fn verify(
        &self,
        message: &[u8],
        signature: &MlDsa87Ed25519Signature,
        ctx: &[u8],
    ) -> Result<(), MlDsa87Ed25519Error> {
        let prehash = compute_prehash_87(message);
        self.verify_prehashed(&prehash, signature, ctx)
    }

    /// Verifies a signature over an already-computed `prehash` with the
    /// optional context `ctx`.
    ///
    /// `prehash` must be the 64-byte output of
    /// `PH(M) = ParallelHash256(M, B = 4096, L = 512 bits,
    /// S = MlDsa87Ed25519MessageRepresentative::LABEL)`. Use
    /// [`MlDsa87Ed25519MessageRepresentative::hasher`] to compute it from a
    /// streamed message.
    ///
    /// Returns the same errors as [`verify`](Self::verify).
    pub fn verify_prehashed(
        &self,
        prehash: &[u8; 64],
        signature: &MlDsa87Ed25519Signature,
        ctx: &[u8],
    ) -> Result<(), MlDsa87Ed25519Error> {
        let m_prime = MlDsa87Ed25519MessageRepresentative::from_prehash(prehash, ctx)?;
        let (mldsa_signature, ed25519_signature) = signature.to_parts();
        self.verify_components(m_prime.as_bytes(), mldsa_signature, ed25519_signature)
    }

    /// Verifies the two component signatures over an already-built `m_prime`.
    fn verify_components(
        &self,
        m_prime: &[u8],
        mldsa_signature: &[u8; ML_DSA_87_SIGNATURE_SIZE],
        ed25519_signature: &[u8; ed25519::SIGNATURE_SIZE],
    ) -> Result<(), MlDsa87Ed25519Error> {
        let (mldsa_public_key, ed25519_public_key) = split_public_key_87(&self.bytes);

        MlDsa87PublicKey::from_bytes(mldsa_public_key).verify(
            m_prime,
            mldsa_signature,
            MlDsa87Ed25519MessageRepresentative::LABEL,
        )?;

        let ed25519_public_key =
            ed25519::PublicKey::from_bytes(ed25519_public_key).map_err(|_| MlDsa87Ed25519Error::InvalidPublicKey)?;
        ed25519_public_key
            .verify(m_prime, ed25519_signature)
            .map_err(|_| MlDsa87Ed25519Error::InvalidSignature)
    }
}

impl From<&[u8; MlDsa87Ed25519PublicKey::SIZE]> for MlDsa87Ed25519PublicKey {
    fn from(bytes: &[u8; MlDsa87Ed25519PublicKey::SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa87Ed25519PublicKey {
    type Error = MlDsa87Ed25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; MlDsa87Ed25519PublicKey::SIZE] =
            bytes.try_into().map_err(|_| MlDsa87Ed25519Error::InvalidPublicKey)?;
        Ok(Self::from_bytes(bytes))
    }
}

/// The message representative `M'` signed by both components.
///
/// `M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ PH(M)` binds the pre-hashed message
/// to the application context and to this algorithm label. Build it once with
/// [`from_message`](Self::from_message) or
/// [`from_prehash`](Self::from_prehash), then hand `as_bytes()` to the
/// component signers.
///
/// ```
/// # use crypto::mldsa::MlDsa87Ed25519MessageRepresentative;
/// let m_prime =
///     MlDsa87Ed25519MessageRepresentative::from_message(b"message", b"context").unwrap();
/// assert_eq!(m_prime.as_bytes().len(), 143);
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct MlDsa87Ed25519MessageRepresentative {
    bytes: [u8; MlDsa87Ed25519MessageRepresentative::MAX_SIZE],
    len: usize,
}

impl MlDsa87Ed25519MessageRepresentative {
    /// Maximum size in bytes of the message representative `M'` (391).
    ///
    /// `M' = Prefix ‖ Label ‖ len(ctx) ‖ ctx ‖ PH(M)`, with `len(ctx)` at most
    /// [`Self::CONTEXT_MAX_LEN`] bytes and a 64-byte pre-hash.
    pub const MAX_SIZE: usize = PREFIX.len() + Self::LABEL.len() + 1 + Self::CONTEXT_MAX_LEN + PH_SIZE;

    /// Maximum length in bytes of the application context string.
    pub const CONTEXT_MAX_LEN: usize = 255;

    /// Signature label for `MLDSA87-Ed25519-PARALLELHASH256`.
    ///
    /// It is also used as the ParallelHash256 customization string and as the
    /// FIPS 204 context string of the ML-DSA-87 component.
    pub const LABEL: &[u8] = b"COMPSIG-MLDSA87-Ed25519-PARALLELHASH256";

    /// Returns a [`ParallelHash256`] hasher configured for `PH(M)`: a
    /// 4096-byte block size, a 64-byte output and [`Self::LABEL`] as the
    /// customization string.
    ///
    /// Feed the message with [`ParallelHash256::update`], finalize into a
    /// 64-byte buffer with [`ParallelHash256::finalize_into`], then pass the
    /// result to [`from_prehash`](Self::from_prehash),
    /// [`verify_prehashed`](MlDsa87Ed25519PublicKey::verify_prehashed) or
    /// [`sign_prehashed`](MlDsa87Ed25519SecretKey::sign_prehashed).
    pub fn hasher() -> ParallelHash256 {
        ParallelHash256::new(4096, Self::LABEL)
    }

    /// Computes `M'` from `message`, hashing it with the scheme's
    /// ParallelHash256 parameters.
    ///
    /// Returns [`MlDsa87Ed25519Error::ContextTooLong`] if `ctx` exceeds
    /// [`Self::CONTEXT_MAX_LEN`] bytes.
    pub fn from_message(message: &[u8], ctx: &[u8]) -> Result<Self, MlDsa87Ed25519Error> {
        let prehash = compute_prehash_87(message);
        Self::from_prehash(&prehash, ctx)
    }

    /// Computes `M'` from an already-computed 64-byte `prehash`.
    ///
    /// `prehash` must be `PH(M) = ParallelHash256(M, B = 4096, L = 512 bits,
    /// S = MlDsa87Ed25519MessageRepresentative::LABEL)`. Use
    /// [`hasher`](Self::hasher) to compute it from a streamed message.
    ///
    /// Returns [`MlDsa87Ed25519Error::ContextTooLong`] if `ctx` exceeds
    /// [`Self::CONTEXT_MAX_LEN`] bytes.
    pub fn from_prehash(prehash: &[u8; 64], ctx: &[u8]) -> Result<Self, MlDsa87Ed25519Error> {
        let mut bytes = [0u8; Self::MAX_SIZE];
        let len = message_representative_87_from_prehash(prehash, ctx, &mut bytes)?;
        Ok(Self {
            bytes,
            len,
        })
    }

    /// Returns the exact bytes both components sign.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl AsRef<[u8]> for MlDsa87Ed25519MessageRepresentative {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl core::fmt::Debug for MlDsa87Ed25519MessageRepresentative {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MlDsa87Ed25519MessageRepresentative")
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

/// An encoded ML-DSA-87-Ed25519-ParallelHash256 signature.
///
/// The value is the 4691-byte concatenation of the ML-DSA-87 signature and the
/// Ed25519 signature. Use [`from_parts`](Self::from_parts) to combine
/// component signatures produced separately, and
/// [`to_parts`](Self::to_parts) to split a signature for external
/// verification.
///
/// ```
/// # use crypto::curve25519::ed25519;
/// # use crypto::mldsa::{
/// #     ML_DSA_87_SIGNATURE_SIZE, MlDsa87Ed25519SecretKey, MlDsa87Ed25519Signature,
/// # };
/// # let key = MlDsa87Ed25519SecretKey::new(&[7u8; MlDsa87Ed25519SecretKey::SIZE]);
/// let signature = key.sign_derand(b"message", b"", &[0u8; 32]).unwrap();
/// let (mldsa_signature, ed25519_signature) = signature.to_parts();
/// assert_eq!(mldsa_signature.len(), ML_DSA_87_SIGNATURE_SIZE);
/// assert_eq!(ed25519_signature.len(), ed25519::SIGNATURE_SIZE);
/// assert_eq!(
///     MlDsa87Ed25519Signature::from_parts(mldsa_signature, ed25519_signature),
///     signature
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MlDsa87Ed25519Signature {
    bytes: [u8; MlDsa87Ed25519Signature::SIZE],
}

impl MlDsa87Ed25519Signature {
    /// Size in bytes of an encoded signature (4691).
    pub const SIZE: usize = ML_DSA_87_SIGNATURE_SIZE + ed25519::SIGNATURE_SIZE;

    /// Creates a signature from its 4691-byte encoded form.
    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    /// Returns the 4691-byte encoded form of this signature.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the 4691-byte encoded form of this signature by value.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.bytes
    }

    /// Combines an ML-DSA-87 signature and an Ed25519 signature.
    ///
    /// This does not verify either component; call
    /// [`MlDsa87Ed25519PublicKey::verify`] on the result to do that.
    pub fn from_parts(mldsa: &[u8; ML_DSA_87_SIGNATURE_SIZE], ed25519: &[u8; ed25519::SIGNATURE_SIZE]) -> Self {
        let mut bytes = [0u8; Self::SIZE];
        bytes[..ML_DSA_87_SIGNATURE_SIZE].copy_from_slice(mldsa);
        bytes[ML_DSA_87_SIGNATURE_SIZE..].copy_from_slice(ed25519);
        Self {
            bytes,
        }
    }

    /// Splits the signature into its ML-DSA-87 and Ed25519 components.
    pub fn to_parts(&self) -> (&[u8; ML_DSA_87_SIGNATURE_SIZE], &[u8; ed25519::SIGNATURE_SIZE]) {
        split_signature_87(&self.bytes)
    }
}

impl From<&[u8; MlDsa87Ed25519Signature::SIZE]> for MlDsa87Ed25519Signature {
    fn from(bytes: &[u8; MlDsa87Ed25519Signature::SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa87Ed25519Signature {
    type Error = MlDsa87Ed25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; MlDsa87Ed25519Signature::SIZE] =
            bytes.try_into().map_err(|_| MlDsa87Ed25519Error::InvalidSignature)?;
        Ok(Self::from_bytes(bytes))
    }
}

impl AsRef<[u8]> for MlDsa87Ed25519Signature {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// An expanded ML-DSA-87-Ed25519-ParallelHash256 private key.
///
/// The value combines an expanded [`MlDsa87SecretKey`] (which caches the
/// NTT-domain matrix and secret vectors) with an expanded Ed25519 signing key.
/// It is a plain fixed-size value that never allocates, but it is large (about
/// 82 KiB), so be careful if allocating it on the stack.
///
/// The key is zeroized on drop when the `zeroize` feature is enabled.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct MlDsa87Ed25519SecretKey {
    mldsa: MlDsa87SecretKey,
    ed25519: ed25519::SecretKey,
}

impl MlDsa87Ed25519SecretKey {
    /// Size in bytes of the interoperable private-key encoding.
    ///
    /// This is a 32-byte ML-DSA seed followed by a 32-byte Ed25519 seed.
    pub const SIZE: usize = ML_DSA_SEED_SIZE + ed25519::SECRET_KEY_SIZE;

    /// Expands a private key from its interoperable 64-byte encoding.
    ///
    /// The encoding is a 32-byte ML-DSA seed followed by a 32-byte Ed25519
    /// seed. The same input always produces the same key.
    pub fn new(bytes: &[u8; Self::SIZE]) -> Self {
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
            mldsa: MlDsa87SecretKey::new(mldsa_seed),
            ed25519: ed25519::SecretKey::from_bytes(ed25519_seed),
        }
    }

    /// Generates a fresh random private key.
    ///
    /// The 64-byte private key can be retrieved afterwards with
    /// [`to_bytes`](Self::to_bytes) so it can be persisted.
    ///
    /// Returns [`MlDsa87Ed25519Error::Random`] when the operating system's
    /// random number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<Self, MlDsa87Ed25519Error> {
        let bytes: [u8; Self::SIZE] = crate::random::bytes()?;
        Ok(Self::new(&bytes))
    }

    /// Returns the 64-byte interoperable encoding of this key.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes[..ML_DSA_SEED_SIZE].copy_from_slice(self.mldsa.seed());
        bytes[ML_DSA_SEED_SIZE..].copy_from_slice(&self.ed25519.to_bytes());
        bytes
    }

    /// Returns the public key for this private key.
    pub fn public_key(&self) -> MlDsa87Ed25519PublicKey {
        let mut bytes = [0u8; MlDsa87Ed25519PublicKey::SIZE];
        bytes[..ML_DSA_87_PUBLIC_KEY_SIZE].copy_from_slice(&self.mldsa.public_key().to_bytes());
        bytes[ML_DSA_87_PUBLIC_KEY_SIZE..].copy_from_slice(&self.ed25519.public_key().to_bytes());
        MlDsa87Ed25519PublicKey {
            bytes,
        }
    }

    /// Signs `message` with a fresh random nonce.
    ///
    /// `ctx` is the application context string and must be at most 255 bytes.
    ///
    /// Returns [`MlDsa87Ed25519Error::ContextTooLong`] if `ctx` is too long
    /// and [`MlDsa87Ed25519Error::Random`] when the operating system's random
    /// number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn sign(&self, message: &[u8], ctx: &[u8]) -> Result<MlDsa87Ed25519Signature, MlDsa87Ed25519Error> {
        let rnd: [u8; 32] = crate::random::bytes()?;
        self.sign_derand(message, ctx, &rnd)
    }

    /// Signs `message` deterministically for a fixed 32-byte `rnd`.
    ///
    /// Passing `rnd = [0u8; 32]` gives the deterministic ML-DSA signing
    /// variant; any other value gives the hedged/randomized variant. The
    /// Ed25519 component is always deterministic. `ctx` must be at most 255
    /// bytes, returning [`MlDsa87Ed25519Error::ContextTooLong`] otherwise.
    pub fn sign_derand(
        &self,
        message: &[u8],
        ctx: &[u8],
        rnd: &[u8; 32],
    ) -> Result<MlDsa87Ed25519Signature, MlDsa87Ed25519Error> {
        let prehash = compute_prehash_87(message);
        self.sign_prehashed_derand(&prehash, ctx, rnd)
    }

    /// Signs an already-computed `prehash` with a fresh random nonce.
    ///
    /// This lets callers stream a large message through
    /// [`MlDsa87Ed25519MessageRepresentative::hasher`] instead of holding it
    /// in memory.
    ///
    /// `prehash` must be the 64-byte output of
    /// `PH(M) = ParallelHash256(M, B = 4096, L = 512 bits,
    /// S = MlDsa87Ed25519MessageRepresentative::LABEL)`. `ctx` must be at most
    /// 255 bytes.
    ///
    /// Returns [`MlDsa87Ed25519Error::ContextTooLong`] if `ctx` is too long
    /// and [`MlDsa87Ed25519Error::Random`] when the operating system's random
    /// number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn sign_prehashed(
        &self,
        prehash: &[u8; 64],
        ctx: &[u8],
    ) -> Result<MlDsa87Ed25519Signature, MlDsa87Ed25519Error> {
        let rnd: [u8; 32] = crate::random::bytes()?;
        self.sign_prehashed_derand(prehash, ctx, &rnd)
    }

    /// Signs an already-computed `prehash` deterministically for a fixed
    /// 32-byte `rnd`.
    ///
    /// The pre-hash requirements are the same as for `sign_prehashed`. `ctx`
    /// must be at most 255 bytes, returning
    /// [`MlDsa87Ed25519Error::ContextTooLong`] otherwise.
    pub fn sign_prehashed_derand(
        &self,
        prehash: &[u8; 64],
        ctx: &[u8],
        rnd: &[u8; 32],
    ) -> Result<MlDsa87Ed25519Signature, MlDsa87Ed25519Error> {
        let m_prime = MlDsa87Ed25519MessageRepresentative::from_prehash(prehash, ctx)?;
        Ok(MlDsa87Ed25519Signature::from_parts(
            &self.sign_mldsa_component(&m_prime, rnd)?,
            &self.sign_ed25519_component(&m_prime),
        ))
    }

    /// Signs a message representative with only the ML-DSA-87 component.
    ///
    /// The scheme's [`MlDsa87Ed25519MessageRepresentative::LABEL`] is used as
    /// the FIPS 204 context string. Use this together with
    /// [`MlDsa87Ed25519Signature::from_parts`] when the Ed25519 component is
    /// produced externally.
    ///
    /// Returns [`MlDsa87Ed25519Error::RetryLimitExceeded`] in the
    /// (statistically impossible) event that the signing rejection loop
    /// exceeds the FIPS 204 limit.
    pub fn sign_mldsa_component(
        &self,
        message_representative: &MlDsa87Ed25519MessageRepresentative,
        rnd: &[u8; 32],
    ) -> Result<[u8; ML_DSA_87_SIGNATURE_SIZE], MlDsa87Ed25519Error> {
        Ok(self.mldsa.sign_derand(
            message_representative.as_bytes(),
            MlDsa87Ed25519MessageRepresentative::LABEL,
            rnd,
        )?)
    }

    /// Signs a message representative with only the Ed25519 component in pure
    /// (non-pre-hashed) mode.
    ///
    /// Use this together with [`MlDsa87Ed25519Signature::from_parts`] when the
    /// ML-DSA-87 component is produced externally.
    pub fn sign_ed25519_component(
        &self,
        message_representative: &MlDsa87Ed25519MessageRepresentative,
    ) -> [u8; ed25519::SIGNATURE_SIZE] {
        self.ed25519.sign(message_representative.as_bytes())
    }
}

impl From<&[u8; MlDsa87Ed25519SecretKey::SIZE]> for MlDsa87Ed25519SecretKey {
    fn from(bytes: &[u8; MlDsa87Ed25519SecretKey::SIZE]) -> Self {
        Self::new(bytes)
    }
}

impl TryFrom<&[u8]> for MlDsa87Ed25519SecretKey {
    type Error = MlDsa87Ed25519Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8; MlDsa87Ed25519SecretKey::SIZE] =
            bytes.try_into().map_err(|_| MlDsa87Ed25519Error::InvalidPrivateKey)?;
        Ok(Self::new(bytes))
    }
}

/// Computes `PH(M) = ParallelHash256(M, B = 4096, L = 512 bits, S = Label)`.
fn compute_prehash_87(message: &[u8]) -> [u8; 64] {
    let mut prehash = [0u8; 64];
    ParallelHash256::hash(message, 4096, MlDsa87Ed25519MessageRepresentative::LABEL, &mut prehash);
    prehash
}

/// Writes `M'` for the ML-DSA-87 scheme from an already-computed pre-hash.
fn message_representative_87_from_prehash(
    prehash: &[u8; 64],
    ctx: &[u8],
    out: &mut [u8; MlDsa87Ed25519MessageRepresentative::MAX_SIZE],
) -> Result<usize, MlDsa87Ed25519Error> {
    if ctx.len() > MlDsa87Ed25519MessageRepresentative::CONTEXT_MAX_LEN {
        return Err(MlDsa87Ed25519Error::ContextTooLong);
    }

    Ok(write_message_representative(
        MlDsa87Ed25519MessageRepresentative::LABEL,
        prehash,
        ctx,
        out,
    ))
}

fn split_public_key_87(
    bytes: &[u8; MlDsa87Ed25519PublicKey::SIZE],
) -> (&[u8; ML_DSA_87_PUBLIC_KEY_SIZE], &[u8; ed25519::PUBLIC_KEY_SIZE]) {
    let (mldsa, ed25519) = bytes.split_at(ML_DSA_87_PUBLIC_KEY_SIZE);
    (
        mldsa
            .try_into()
            .expect("ML-DSA public key length is checked by split_at"),
        ed25519
            .try_into()
            .expect("Ed25519 public key length is checked by split_at"),
    )
}

fn split_signature_87(
    signature: &[u8; MlDsa87Ed25519Signature::SIZE],
) -> (&[u8; ML_DSA_87_SIGNATURE_SIZE], &[u8; ed25519::SIGNATURE_SIZE]) {
    let (mldsa, ed25519) = signature.split_at(ML_DSA_87_SIGNATURE_SIZE);
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
