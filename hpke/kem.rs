//! Key encapsulation mechanisms (KEMs) for HPKE (RFC 9180 Section 4 and Section 7.1).

use crypto::{Hash, curve25519::x25519, p256, p521};

use super::{
    HpkeError,
    kdf::{HkdfSha256, HkdfSha512, Kdf, Shake256 as Shake256Kdf},
    wipe,
};

/// Key encapsulation mechanism (KEM) usable within an HPKE ciphersuite.
///
/// A KEM encapsulates a shared secret against a public key: the sender runs
/// [`Kem::encap`] to obtain a `(shared secret, encapsulated key)` pair, and
/// the recipient runs [`Kem::decap`] on the encapsulated key to recover the
/// same shared secret.
///
/// The four modes of HPKE only require `encap`/`decap`; the authenticated
/// modes additionally require `authenticated_encap`/`authenticated_decap`,
/// which are optional (RFC 9180 Section 7.1.5). KEMs without them (e.g.
/// ML-KEM-based or hybrid post-quantum KEMs such as [`MLKEM768X25519`]) support
/// only the Base and pre-shared key modes; calling a setup function with an
/// authenticated mode returns [`HpkeError::NotSupported`].
///
/// # Example
///
/// ```
/// use hpke::kem::{Kem, X25519HkdfSha256};
///
/// let (secret_key, public_key) = X25519HkdfSha256::generate_keypair().unwrap();
/// let (shared_secret, encapped_key) = X25519HkdfSha256::encap(&public_key).unwrap();
/// let decapsulated = X25519HkdfSha256::decap(&encapped_key, &secret_key).unwrap();
/// assert_eq!(shared_secret.as_ref(), decapsulated.as_ref());
/// ```
///
/// # Custom KEMs
///
/// Implement this trait to use a custom KEM (e.g. ML-KEM for the
/// post-quantum HPKE draft) inside HPKE. The shared secret returned by
/// `encap`/`decap` must be exactly [`Kem::SHARED_SECRET_SIZE`] bytes long. Use
/// [`Kdf::labeled_extract`] and
/// [`Kdf::labeled_expand`] to derive keys exactly
/// like the RFC 9180 KEMs do.
pub trait Kem: Sized {
    /// The HPKE KEM identifier (RFC 9180 Section 7.1, IANA "HPKE KEM
    /// Identifiers"), used to build the ciphersuite `suite_id`.
    const HPKE_KEM_ID: u16;

    /// The size of the shared secret produced by this KEM.
    ///
    /// Must not exceed 64 bytes, the maximum capacity of [`struct@Hash`].
    const SHARED_SECRET_SIZE: usize;

    /// The size of an encapsulated key.
    const ENCAPPED_KEY_SIZE: usize;

    /// The size of a serialized public key.
    const PUBLIC_KEY_SIZE: usize;

    /// The size of a serialized secret key.
    const SECRET_KEY_SIZE: usize;

    /// The size of the randomness consumed by [`Kem::encap_deterministic`].
    ///
    /// Defaults to [`Kem::SECRET_KEY_SIZE`], the size of an ephemeral secret
    /// key / the `DeriveKeyPair` input for the Diffie-Hellman KEMs. KEMs whose
    /// deterministic encapsulation takes a different amount of randomness (e.g.
    /// [`MLKEM768X25519`]) override this.
    const RANDOMNESS_SIZE: usize = Self::SECRET_KEY_SIZE;

    /// The public (encapsulation) key type.
    type PublicKey: Clone;

    /// The secret (decapsulation) key type.
    type SecretKey: Clone;

    /// The encapsulated key type (the KEM ciphertext sent alongside the
    /// first encrypted message).
    type EncappedKey: Clone;

    /// Deterministically derives a key pair from input keying material.
    ///
    /// `input_keying_material` SHOULD be at least
    /// [`Kem::SECRET_KEY_SIZE`] bytes long and contain at least that many
    /// bytes of entropy (RFC 9180 Section 7.1.3). It MUST NOT be reused for
    /// any other purpose, in particular not with another KEM.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeriveKeyPairError`] when no valid key pair could
    /// be derived (rejection sampling exhausted).
    fn derive_keypair(input_keying_material: &[u8]) -> Result<(Self::SecretKey, Self::PublicKey), HpkeError>;

    /// Generates a fresh random key pair using the operating system's random
    /// number generator.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::Random`] when the operating system's random
    /// number generator is unavailable or fails.
    #[cfg(feature = "random")]
    fn generate_keypair() -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
        // Maximum secret key size across the algorithms shipped by this crate
        // (P-521: 66 bytes). Custom KEMs with a larger `SECRET_KEY_SIZE` must
        // override this method.
        const MAX_SECRET_KEY_SIZE: usize = 66;
        const {
            assert!(
                Self::SECRET_KEY_SIZE <= MAX_SECRET_KEY_SIZE,
                "SECRET_KEY_SIZE is too large; override generate_keypair"
            )
        };

        let mut input_keying_material = [0u8; MAX_SECRET_KEY_SIZE];
        crypto::random::fill(&mut input_keying_material[..Self::SECRET_KEY_SIZE])?;
        return Self::derive_keypair(&input_keying_material[..Self::SECRET_KEY_SIZE]);
    }

    /// Derives the public key of a secret key.
    fn derive_public_key(secret_key: &Self::SecretKey) -> Self::PublicKey;

    /// Serializes a public key into `out`, which must be exactly
    /// [`Kem::PUBLIC_KEY_SIZE`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `out` has the wrong
    /// length.
    fn public_key_to_bytes(out: &mut [u8], public_key: &Self::PublicKey) -> Result<(), HpkeError>;

    /// Deserializes a public key from `bytes`, which must be exactly
    /// [`Kem::PUBLIC_KEY_SIZE`] bytes long and a valid, fully validated public key.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `bytes` has the wrong
    /// length or does not encode a valid public key.
    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError>;

    /// Serializes a secret key into `out`, which must be exactly
    /// [`Kem::SECRET_KEY_SIZE`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `out` has the wrong
    /// length.
    fn secret_key_to_bytes(out: &mut [u8], secret_key: &Self::SecretKey) -> Result<(), HpkeError>;

    /// Deserializes a secret key from `bytes`, which must be exactly
    /// [`Kem::SECRET_KEY_SIZE`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `bytes` has the wrong
    /// length or does not encode a valid secret key.
    fn secret_key_from_bytes(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError>;

    /// Serializes an encapsulated key into `out`, which must be exactly
    /// [`Kem::ENCAPPED_KEY_SIZE`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `out` has the wrong
    /// length.
    fn encapped_key_to_bytes(out: &mut [u8], encapped_key: &Self::EncappedKey) -> Result<(), HpkeError>;

    /// Deserializes an encapsulated key from `bytes`, which must be exactly
    /// [`Kem::ENCAPPED_KEY_SIZE`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `bytes` has the wrong
    /// length or does not encode a valid encapsulated key.
    fn encapped_key_from_bytes(bytes: &[u8]) -> Result<Self::EncappedKey, HpkeError>;

    /// Encapsulates a shared secret against `recipient_public_key`, using the given ephemeral
    /// secret key `ephemeral_secret_key`.
    ///
    /// Returns the shared secret and the encapsulated key that the owner of
    /// `recipient_public_key` can use to recover it. This is the deterministic variant of
    /// [`Kem::encap`]; it exists for protocols that need reproducible
    /// encapsulation (e.g. test vectors) and MUST NOT be called with a
    /// non-fresh `ephemeral_secret_key`.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::EncapError`] when `recipient_public_key` is invalid.
    fn encap_with_ephemeral(
        ephemeral_secret_key: &Self::SecretKey,
        recipient_public_key: &Self::PublicKey,
    ) -> Result<(Hash, Self::EncappedKey), HpkeError>;

    /// Encapsulates a fresh shared secret against `recipient_public_key` using a freshly
    /// generated ephemeral key pair.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::EncapError`] when `recipient_public_key` is invalid, or
    /// [`HpkeError::Random`] when the operating system's random number
    /// generator is unavailable or fails.
    #[cfg(feature = "random")]
    fn encap(recipient_public_key: &Self::PublicKey) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        let (ephemeral_secret_key, _ephemeral_public_key) = Self::generate_keypair()?;
        return Self::encap_with_ephemeral(&ephemeral_secret_key, recipient_public_key);
    }

    /// Encapsulates a shared secret against `recipient_public_key` from caller-provided
    /// `randomness` (`EncapDerand`).
    ///
    /// This is the deterministic equivalent of [`Kem::encap`], intended for
    /// test vectors and protocols that need reproducible encapsulation.
    /// `randomness` MUST be exactly [`Kem::RANDOMNESS_SIZE`] bytes long, uniformly
    /// random, and MUST NOT be reused across encapsulations.
    ///
    /// The default implementation derives an ephemeral key pair from
    /// `randomness` and forwards to [`Kem::encap_with_ephemeral`], which is
    /// correct for the Diffie-Hellman KEMs. KEMs with a different
    /// deterministic-encapsulation interface (e.g. [`MLKEM768X25519`], whose
    /// randomness is the 64-byte encapsulation randomness rather than a secret
    /// key) override this.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `randomness` does not have
    /// exactly [`Kem::RANDOMNESS_SIZE`] bytes, or [`HpkeError::EncapError`] when
    /// `recipient_public_key` is invalid.
    fn encap_deterministic(
        recipient_public_key: &Self::PublicKey,
        randomness: &[u8],
    ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        if randomness.len() != Self::RANDOMNESS_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        let (ephemeral_secret_key, _ephemeral_public_key) = Self::derive_keypair(randomness)?;
        return Self::encap_with_ephemeral(&ephemeral_secret_key, recipient_public_key);
    }

    /// Recovers the shared secret encapsulated in `encapped_key` with the secret key
    /// `recipient_secret_key`.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DecapError`] when `encapped_key` is invalid or when the
    /// key exchange fails.
    fn decap(encapped_key: &Self::EncappedKey, recipient_secret_key: &Self::SecretKey) -> Result<Hash, HpkeError>;

    /// Authenticated encapsulation against `recipient_public_key` proving possession of the
    /// secret key `sender_secret_key` (RFC 9180 Section 4.1 `AuthEncap`), using the given
    /// ephemeral secret key `ephemeral_secret_key`.
    ///
    /// The default implementation always fails: authenticated encapsulation
    /// is optional for a KEM (RFC 9180 Section 7.1.5).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::NotSupported`] by default, and
    /// [`HpkeError::EncapError`] when `recipient_public_key` is invalid.
    fn authenticated_encap_with_ephemeral(
        _ephemeral_secret_key: &Self::SecretKey,
        _recipient_public_key: &Self::PublicKey,
        _sender_secret_key: &Self::SecretKey,
    ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        return Err(HpkeError::NotSupported);
    }

    /// Authenticated encapsulation with a freshly generated ephemeral key
    /// pair. See [`Kem::authenticated_encap_with_ephemeral`].
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::NotSupported`] by default, [`HpkeError::EncapError`]
    /// when `recipient_public_key` is invalid, or [`HpkeError::Random`] when the operating
    /// system's random number generator is unavailable or fails.
    #[cfg(feature = "random")]
    fn authenticated_encap(
        recipient_public_key: &Self::PublicKey,
        sender_secret_key: &Self::SecretKey,
    ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        let (ephemeral_secret_key, _ephemeral_public_key) = Self::generate_keypair()?;
        return Self::authenticated_encap_with_ephemeral(
            &ephemeral_secret_key,
            recipient_public_key,
            sender_secret_key,
        );
    }

    /// Authenticated decapsulation of `encapped_key` with `recipient_secret_key`, verifying that the
    /// encapsulation was produced with the secret key matching `sender_public_key`
    /// (RFC 9180 Section 4.1 `AuthDecap`).
    ///
    /// The default implementation always fails: authenticated decapsulation
    /// is optional for a KEM (RFC 9180 Section 7.1.5).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::NotSupported`] by default, and
    /// [`HpkeError::DecapError`] when `encapped_key` is invalid.
    fn authenticated_decap(
        _encapped_key: &Self::EncappedKey,
        _recipient_secret_key: &Self::SecretKey,
        _sender_public_key: &Self::PublicKey,
    ) -> Result<Hash, HpkeError> {
        return Err(HpkeError::NotSupported);
    }
}

/// MLKEM768-X25519 hybrid post-quantum KEM (ML-KEM-768 with X25519), as
/// specified in `draft-ietf-hpke-pq-05`.
///
/// This is the same construction as X-Wing (`draft-connolly-cfrg-xwing-kem`),
/// as noted in `draft-irtf-cfrg-concrete-hybrid-kems-03` Section 4.2.
///
/// MLKEM768-X25519 is usable with the Base and pre-shared key modes of HPKE;
/// it does not support the authenticated modes
/// ([`SenderMode::Authenticated`](super::SenderMode::Authenticated) and
/// [`SenderMode::AuthenticatedPreSharedKey`](super::SenderMode::AuthenticatedPreSharedKey)),
/// which return [`HpkeError::NotSupported`].
///
/// The KEM identifier is `0x647a`, as registered by the draft in the IANA
/// "HPKE KEM Identifiers" registry.
///
/// # Example
///
/// ```
/// use hpke::kem::{Kem, MLKEM768X25519};
///
/// let (secret_key, public_key) = MLKEM768X25519::generate_keypair().unwrap();
/// let (shared_secret, encapped_key) = MLKEM768X25519::encap(&public_key).unwrap();
/// let decapsulated = MLKEM768X25519::decap(&encapped_key, &secret_key).unwrap();
/// assert_eq!(shared_secret.as_ref(), decapsulated.as_ref());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MLKEM768X25519;

impl Kem for MLKEM768X25519 {
    const HPKE_KEM_ID: u16 = 0x647a;
    const SHARED_SECRET_SIZE: usize = crypto::xwing::SHARED_SECRET_SIZE;
    const ENCAPPED_KEY_SIZE: usize = crypto::xwing::CIPHERTEXT_SIZE;
    const PUBLIC_KEY_SIZE: usize = crypto::xwing::PUBLIC_KEY_SIZE;
    const SECRET_KEY_SIZE: usize = crypto::xwing::SECRET_KEY_SIZE;
    const RANDOMNESS_SIZE: usize = 64;

    type PublicKey = crypto::xwing::PublicKey;
    type SecretKey = crypto::xwing::SecretKey;
    type EncappedKey = [u8; crypto::xwing::CIPHERTEXT_SIZE];

    fn derive_keypair(input_keying_material: &[u8]) -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
        // draft-ietf-hpke-pq-05 Section 4 (MLKEM768-X25519):
        //   seed = SHAKE256.LabeledDerive(input_keying_material, "DeriveKeyPair", "", 32)
        //   return KEM.DeriveKeyPair(seed)
        let suite_id = kem_suite_id(Self::HPKE_KEM_ID);
        let mut seed = [0u8; crypto::xwing::SECRET_KEY_SIZE];
        <Shake256Kdf as Kdf>::labeled_derive(&mut seed, &[input_keying_material], &suite_id, b"DeriveKeyPair", &[])?;

        let keypair = crypto::xwing::generate_keypair_derand(&seed);
        wipe(&mut seed);
        return Ok(keypair);
    }

    #[cfg(feature = "random")]
    fn generate_keypair() -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
        return crypto::xwing::generate_keypair().map_err(HpkeError::from);
    }

    fn derive_public_key(secret_key: &Self::SecretKey) -> Self::PublicKey {
        return secret_key.public_key();
    }

    fn public_key_to_bytes(out: &mut [u8], public_key: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::PUBLIC_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&public_key.to_bytes());
        return Ok(());
    }

    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        let bytes: &[u8; Self::PUBLIC_KEY_SIZE] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return crypto::xwing::PublicKey::from_bytes(bytes).map_err(HpkeError::from);
    }

    fn secret_key_to_bytes(out: &mut [u8], secret_key: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::SECRET_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&secret_key.to_bytes());
        return Ok(());
    }

    fn secret_key_from_bytes(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::SECRET_KEY_SIZE] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return Ok(crypto::xwing::generate_keypair_derand(bytes).0);
    }

    fn encapped_key_to_bytes(out: &mut [u8], encapped_key: &Self::EncappedKey) -> Result<(), HpkeError> {
        if out.len() != Self::ENCAPPED_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(encapped_key);
        return Ok(());
    }

    fn encapped_key_from_bytes(bytes: &[u8]) -> Result<Self::EncappedKey, HpkeError> {
        let bytes: &[u8; Self::ENCAPPED_KEY_SIZE] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return Ok(*bytes);
    }

    #[cfg(feature = "random")]
    fn encap(recipient_public_key: &Self::PublicKey) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        let mut randomness = [0u8; Self::RANDOMNESS_SIZE];
        crypto::random::fill(&mut randomness)?;
        let result = Self::encap_deterministic(recipient_public_key, &randomness);
        wipe(&mut randomness);
        return result;
    }

    fn encap_deterministic(
        recipient_public_key: &Self::PublicKey,
        randomness: &[u8],
    ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        // draft-irtf-cfrg-hybrid-kems-12 Appendix A: the hybrid KEM randomness
        // is the concatenation of the ML-KEM-768 and X25519 randomness, i.e.
        // exactly X-Wing's 64-byte `encapsulation_seed`.
        let randomness: &[u8; 64] = randomness.try_into().map_err(|_| HpkeError::DeserializeError)?;
        let (shared_secret, encapped_key) = recipient_public_key.encapsulate_derand(randomness);
        return Ok((Hash::from(shared_secret), encapped_key));
    }

    fn encap_with_ephemeral(
        ephemeral_secret_key: &Self::SecretKey,
        recipient_public_key: &Self::PublicKey,
    ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        // The trait models the ephemeral secret as a KEM secret key, whereas
        // MLKEM768-X25519 encapsulation consumes a 64-byte seed. Expand the
        // 32-byte ephemeral seed deterministically with SHAKE256.
        let mut encapsulation_seed = [0u8; 64];
        crypto::sha3::Shake256::hash(&ephemeral_secret_key.to_bytes(), &mut encapsulation_seed);

        let (shared_secret, encapped_key) = recipient_public_key.encapsulate_derand(&encapsulation_seed);
        wipe(&mut encapsulation_seed);
        return Ok((Hash::from(shared_secret), encapped_key));
    }

    fn decap(encapped_key: &Self::EncappedKey, recipient_secret_key: &Self::SecretKey) -> Result<Hash, HpkeError> {
        let shared_secret = recipient_secret_key
            .decapsulate(encapped_key)
            .map_err(|_| HpkeError::DecapError)?;
        return Ok(Hash::from(shared_secret));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Diffie-Hellman KEM (RFC 9180 Section 4.1)

/// Maximum serialized public key / encapsulated key size across the supported
/// groups (P-521: 133 bytes).
const MAX_PUBLIC_KEY_SIZE: usize = 133;
/// Maximum secret key and Diffie-Hellman output size across the supported
/// groups (P-521: 66 bytes).
const MAX_DIFFIE_HELLMAN_OUTPUT_SIZE: usize = 66;
/// Maximum size of a KEM context:
/// `encapped_key || recipient_public_key [|| sender_public_key]`.
const MAX_KEM_CONTEXT: usize = 3 * MAX_PUBLIC_KEY_SIZE;

/// Builds the Diffie-Hellman KEM `suite_id`: `"KEM" || I2OSP(kem_id, 2)` (RFC 9180
/// Section 4).
fn kem_suite_id(kem_id: u16) -> [u8; 5] {
    let mut suite_id = [0u8; 5];
    suite_id[..3].copy_from_slice(b"KEM");
    suite_id[3] = (kem_id >> 8) as u8;
    suite_id[4] = (kem_id & 0xff) as u8;
    return suite_id;
}

/// Applies the X25519 scalar clamping defined by RFC 7748 Section 5.
fn clamp_scalar25519(mut scalar: [u8; 32]) -> [u8; 32] {
    scalar[0] &= 248;
    scalar[31] &= 127;
    scalar[31] |= 64;
    return scalar;
}

/// A Diffie-Hellman group usable by the Diffie-Hellman KEM construction.
///
/// This trait is private: the crate provides the groups required by RFC 9180.
/// Custom KEMs are expected to implement [`Kem`] directly (the Diffie-Hellman KEM
/// construction can be reproduced with [`Kdf::labeled_extract`] and
/// [`Kdf::labeled_expand`]).
trait DiffieHellmanGroup: Copy + Clone {
    /// Size of the raw Diffie-Hellman output.
    const DIFFIE_HELLMAN_OUTPUT_SIZE: usize;
    /// Size of a serialized public key.
    const PUBLIC_KEY_SIZE: usize;
    /// Size of a serialized secret key.
    const SECRET_KEY_SIZE: usize;
    /// Mask applied to the first byte of a NIST candidate scalar.
    const BITMASK: u8;
    /// Whether `DeriveKeyPair` uses NIST rejection sampling (true) or the
    /// single-shot X25519 derivation (false).
    const REJECTION_SAMPLING: bool;

    type SecretKey: Clone;
    type PublicKey: Clone;

    /// Computes the raw Diffie-Hellman shared secret, written into `out`
    /// (`out.len() == DIFFIE_HELLMAN_OUTPUT_SIZE`).
    ///
    /// Returns `Err(())` on failure (e.g. an all-zero X25519 output).
    fn diffie_hellman(secret_key: &Self::SecretKey, public_key: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()>;

    /// Derives the public key of a secret key.
    fn derive_public_key(secret_key: &Self::SecretKey) -> Self::PublicKey;

    /// Serializes a public key into `out` (`out.len() == PUBLIC_KEY_SIZE`).
    fn serialize_public_key(out: &mut [u8], public_key: &Self::PublicKey) -> Result<(), HpkeError>;

    /// Deserializes and validates a public key.
    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError>;

    /// Serializes a secret key into `out` (`out.len() == SECRET_KEY_SIZE`).
    fn serialize_secret_key(out: &mut [u8], secret_key: &Self::SecretKey) -> Result<(), HpkeError>;

    /// Deserializes a secret key.
    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError>;

    /// Interprets a candidate scalar for `DeriveKeyPair`, returning `Err(())`
    /// when it is out of range (rejection sampling).
    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct X25519Group;

impl DiffieHellmanGroup for X25519Group {
    const DIFFIE_HELLMAN_OUTPUT_SIZE: usize = x25519::SHARED_SECRET_SIZE;
    const PUBLIC_KEY_SIZE: usize = x25519::KEY_SIZE;
    const SECRET_KEY_SIZE: usize = x25519::KEY_SIZE;
    const BITMASK: u8 = 0xff;
    const REJECTION_SAMPLING: bool = false;

    type SecretKey = x25519::SecretKey;
    type PublicKey = x25519::PublicKey;

    fn diffie_hellman(secret_key: &Self::SecretKey, public_key: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()> {
        // `ecdh` rejects all-zero shared secrets, as RFC 9180 Section 7.1.4
        // requires for X25519.
        let shared = secret_key.ecdh(public_key).map_err(|_| ())?;
        out.copy_from_slice(&shared);
        return Ok(());
    }

    fn derive_public_key(secret_key: &Self::SecretKey) -> Self::PublicKey {
        return secret_key.public_key();
    }

    fn serialize_public_key(out: &mut [u8], public_key: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::PUBLIC_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&public_key.to_bytes());
        return Ok(());
    }

    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        let bytes: &[u8; Self::PUBLIC_KEY_SIZE] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return Ok(x25519::PublicKey::from_bytes(bytes));
    }

    fn serialize_secret_key(out: &mut [u8], secret_key: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::SECRET_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        // RFC 9180 Section 7.1.2 requires `SerializePrivateKey` to clamp.
        out.copy_from_slice(&clamp_scalar25519(secret_key.to_bytes()));
        return Ok(());
    }

    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::SECRET_KEY_SIZE] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        // RFC 9180 Section 7.1.2 requires `DeserializePrivateKey` to clamp.
        return Ok(x25519::SecretKey::from_bytes(&clamp_scalar25519(*bytes)));
    }

    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()> {
        let bytes: &[u8; Self::SECRET_KEY_SIZE] = bytes.try_into().map_err(|_| ())?;
        return Ok(x25519::SecretKey::from_bytes(&clamp_scalar25519(*bytes)));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P256Group;

impl DiffieHellmanGroup for P256Group {
    const DIFFIE_HELLMAN_OUTPUT_SIZE: usize = p256::ECDH_SHARED_SECRET_SIZE;
    const PUBLIC_KEY_SIZE: usize = p256::PUBLIC_KEY_UNCOMPRESSED_SIZE;
    const SECRET_KEY_SIZE: usize = p256::SECRET_KEY_SIZE;
    const BITMASK: u8 = 0xff;
    const REJECTION_SAMPLING: bool = true;

    type SecretKey = p256::SecretKey;
    type PublicKey = p256::PublicKey;

    fn diffie_hellman(secret_key: &Self::SecretKey, public_key: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()> {
        let shared = secret_key.ecdh(public_key).map_err(|_| ())?;
        out.copy_from_slice(&shared);
        return Ok(());
    }

    fn derive_public_key(secret_key: &Self::SecretKey) -> Self::PublicKey {
        return secret_key.public_key();
    }

    fn serialize_public_key(out: &mut [u8], public_key: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::PUBLIC_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&public_key.to_bytes());
        return Ok(());
    }

    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        // RFC 9180 Section 7.1.1 requires the uncompressed SEC1 encoding.
        if bytes.len() != Self::PUBLIC_KEY_SIZE || bytes[0] != 0x04 {
            return Err(HpkeError::DeserializeError);
        }
        return p256::PublicKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn serialize_secret_key(out: &mut [u8], secret_key: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::SECRET_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&secret_key.to_bytes());
        return Ok(());
    }

    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::SECRET_KEY_SIZE] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return p256::SecretKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()> {
        let bytes: &[u8; Self::SECRET_KEY_SIZE] = bytes.try_into().map_err(|_| ())?;
        return p256::SecretKey::from_bytes(bytes).map_err(|_| ());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P521Group;

impl DiffieHellmanGroup for P521Group {
    const DIFFIE_HELLMAN_OUTPUT_SIZE: usize = p521::ECDH_SHARED_SECRET_SIZE;
    const PUBLIC_KEY_SIZE: usize = p521::PUBLIC_KEY_UNCOMPRESSED_SIZE;
    const SECRET_KEY_SIZE: usize = p521::SECRET_KEY_SIZE;
    const BITMASK: u8 = 0x01;
    const REJECTION_SAMPLING: bool = true;

    type SecretKey = p521::SecretKey;
    type PublicKey = p521::PublicKey;

    fn diffie_hellman(secret_key: &Self::SecretKey, public_key: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()> {
        let shared = secret_key.ecdh(public_key).map_err(|_| ())?;
        out.copy_from_slice(&shared);
        return Ok(());
    }

    fn derive_public_key(secret_key: &Self::SecretKey) -> Self::PublicKey {
        return secret_key.public_key();
    }

    fn serialize_public_key(out: &mut [u8], public_key: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::PUBLIC_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&public_key.to_bytes());
        return Ok(());
    }

    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        if bytes.len() != Self::PUBLIC_KEY_SIZE || bytes[0] != 0x04 {
            return Err(HpkeError::DeserializeError);
        }
        return p521::PublicKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn serialize_secret_key(out: &mut [u8], secret_key: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::SECRET_KEY_SIZE {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&secret_key.to_bytes());
        return Ok(());
    }

    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::SECRET_KEY_SIZE] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return p521::SecretKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()> {
        let bytes: &[u8; Self::SECRET_KEY_SIZE] = bytes.try_into().map_err(|_| ())?;
        return p521::SecretKey::from_bytes(bytes).map_err(|_| ());
    }
}

/// RFC 9180 Section 4.1 `ExtractAndExpand`.
fn extract_and_expand<K: Kdf>(kem_id: u16, diffie_hellman: &[u8], kem_context: &[u8]) -> Result<Hash, HpkeError> {
    const { assert!(K::OUTPUT_SIZE <= crypto::MAX_HASH_OUTPUT_SIZE) };

    let suite_id = kem_suite_id(kem_id);
    let mut pseudorandom_key = K::labeled_extract(b"", &suite_id, b"eae_prk", diffie_hellman);

    let mut shared_secret_bytes = [0u8; crypto::MAX_HASH_OUTPUT_SIZE];
    K::labeled_expand(
        &mut shared_secret_bytes[..K::OUTPUT_SIZE],
        &pseudorandom_key,
        &suite_id,
        b"shared_secret",
        kem_context,
    )?;
    let shared_secret =
        Hash::try_from(&shared_secret_bytes[..K::OUTPUT_SIZE]).expect("SHARED_SECRET_SIZE fits in a Hash");

    wipe(&mut shared_secret_bytes);
    wipe(pseudorandom_key.as_mut());
    return Ok(shared_secret);
}

/// RFC 9180 Section 7.1.3 `DeriveKeyPair` for a Diffie-Hellman KEM.
fn diffie_hellman_kem_derive_keypair<G: DiffieHellmanGroup, K: Kdf>(
    kem_id: u16,
    input_keying_material: &[u8],
) -> Result<(G::SecretKey, G::PublicKey), HpkeError> {
    const { assert!(G::SECRET_KEY_SIZE <= MAX_DIFFIE_HELLMAN_OUTPUT_SIZE && G::PUBLIC_KEY_SIZE <= MAX_PUBLIC_KEY_SIZE) };

    let suite_id = kem_suite_id(kem_id);
    let mut derive_key_pair_pseudorandom_key = K::labeled_extract(b"", &suite_id, b"dkp_prk", input_keying_material);

    if G::REJECTION_SAMPLING {
        let mut candidate = [0u8; MAX_DIFFIE_HELLMAN_OUTPUT_SIZE];
        for counter in 0..=255u8 {
            K::labeled_expand(
                &mut candidate[..G::SECRET_KEY_SIZE],
                &derive_key_pair_pseudorandom_key,
                &suite_id,
                b"candidate",
                &[counter],
            )?;
            candidate[0] &= G::BITMASK;

            if let Ok(secret_key) = G::scalar_from_candidate(&candidate[..G::SECRET_KEY_SIZE]) {
                let public_key = G::derive_public_key(&secret_key);
                wipe(&mut candidate);
                wipe(derive_key_pair_pseudorandom_key.as_mut());
                return Ok((secret_key, public_key));
            }
        }
        wipe(&mut candidate);
        wipe(derive_key_pair_pseudorandom_key.as_mut());
        return Err(HpkeError::DeriveKeyPairError);
    }

    let mut secret_key_bytes = [0u8; MAX_DIFFIE_HELLMAN_OUTPUT_SIZE];
    K::labeled_expand(
        &mut secret_key_bytes[..G::SECRET_KEY_SIZE],
        &derive_key_pair_pseudorandom_key,
        &suite_id,
        b"sk",
        b"",
    )?;
    let secret_key =
        G::scalar_from_candidate(&secret_key_bytes[..G::SECRET_KEY_SIZE]).map_err(|_| HpkeError::DeriveKeyPairError)?;
    let public_key = G::derive_public_key(&secret_key);
    wipe(&mut secret_key_bytes);
    wipe(derive_key_pair_pseudorandom_key.as_mut());
    return Ok((secret_key, public_key));
}

/// RFC 9180 Section 4.1 `Encap` with a caller-provided ephemeral key.
fn diffie_hellman_kem_encap_with_ephemeral<G: DiffieHellmanGroup, K: Kdf>(
    kem_id: u16,
    ephemeral_secret_key: &G::SecretKey,
    recipient_public_key: &G::PublicKey,
) -> Result<(Hash, G::PublicKey), HpkeError> {
    const { assert!(G::SECRET_KEY_SIZE <= MAX_DIFFIE_HELLMAN_OUTPUT_SIZE && G::PUBLIC_KEY_SIZE <= MAX_PUBLIC_KEY_SIZE) };

    let mut diffie_hellman = [0u8; MAX_DIFFIE_HELLMAN_OUTPUT_SIZE];
    G::diffie_hellman(
        ephemeral_secret_key,
        recipient_public_key,
        &mut diffie_hellman[..G::DIFFIE_HELLMAN_OUTPUT_SIZE],
    )
    .map_err(|_| HpkeError::EncapError)?;

    let ephemeral_public_key = G::derive_public_key(ephemeral_secret_key);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::PUBLIC_KEY_SIZE], &ephemeral_public_key)?;
    G::serialize_public_key(
        &mut kem_context[G::PUBLIC_KEY_SIZE..2 * G::PUBLIC_KEY_SIZE],
        recipient_public_key,
    )?;

    let shared_secret = extract_and_expand::<K>(
        kem_id,
        &diffie_hellman[..G::DIFFIE_HELLMAN_OUTPUT_SIZE],
        &kem_context[..2 * G::PUBLIC_KEY_SIZE],
    )?;
    wipe(&mut diffie_hellman);
    return Ok((shared_secret, ephemeral_public_key));
}

/// RFC 9180 Section 4.1 `Decap`.
fn diffie_hellman_kem_decap<G: DiffieHellmanGroup, K: Kdf>(
    kem_id: u16,
    encapped_key: &G::PublicKey,
    recipient_secret_key: &G::SecretKey,
) -> Result<Hash, HpkeError> {
    const { assert!(G::SECRET_KEY_SIZE <= MAX_DIFFIE_HELLMAN_OUTPUT_SIZE && G::PUBLIC_KEY_SIZE <= MAX_PUBLIC_KEY_SIZE) };

    let mut diffie_hellman = [0u8; MAX_DIFFIE_HELLMAN_OUTPUT_SIZE];
    G::diffie_hellman(
        recipient_secret_key,
        encapped_key,
        &mut diffie_hellman[..G::DIFFIE_HELLMAN_OUTPUT_SIZE],
    )
    .map_err(|_| HpkeError::DecapError)?;

    let recipient_public_key = G::derive_public_key(recipient_secret_key);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::PUBLIC_KEY_SIZE], encapped_key)?;
    G::serialize_public_key(
        &mut kem_context[G::PUBLIC_KEY_SIZE..2 * G::PUBLIC_KEY_SIZE],
        &recipient_public_key,
    )?;

    let shared_secret = extract_and_expand::<K>(
        kem_id,
        &diffie_hellman[..G::DIFFIE_HELLMAN_OUTPUT_SIZE],
        &kem_context[..2 * G::PUBLIC_KEY_SIZE],
    )?;
    wipe(&mut diffie_hellman);
    return Ok(shared_secret);
}

/// RFC 9180 Section 4.1 `AuthEncap` with a caller-provided ephemeral key.
fn diffie_hellman_kem_authenticated_encap_with_ephemeral<G: DiffieHellmanGroup, K: Kdf>(
    kem_id: u16,
    ephemeral_secret_key: &G::SecretKey,
    recipient_public_key: &G::PublicKey,
    sender_secret_key: &G::SecretKey,
) -> Result<(Hash, G::PublicKey), HpkeError> {
    const { assert!(G::SECRET_KEY_SIZE <= MAX_DIFFIE_HELLMAN_OUTPUT_SIZE && G::PUBLIC_KEY_SIZE <= MAX_PUBLIC_KEY_SIZE) };

    // diffie_hellman = DH(ephemeral_secret_key, recipient_public_key)
    //               || DH(sender_secret_key, recipient_public_key)
    let mut diffie_hellman = [0u8; 2 * MAX_DIFFIE_HELLMAN_OUTPUT_SIZE];
    G::diffie_hellman(
        ephemeral_secret_key,
        recipient_public_key,
        &mut diffie_hellman[..G::DIFFIE_HELLMAN_OUTPUT_SIZE],
    )
    .map_err(|_| HpkeError::EncapError)?;
    G::diffie_hellman(
        sender_secret_key,
        recipient_public_key,
        &mut diffie_hellman[G::DIFFIE_HELLMAN_OUTPUT_SIZE..2 * G::DIFFIE_HELLMAN_OUTPUT_SIZE],
    )
    .map_err(|_| HpkeError::EncapError)?;

    // kem_context = encapped_key || recipient_public_key || sender_public_key
    let ephemeral_public_key = G::derive_public_key(ephemeral_secret_key);
    let sender_public_key = G::derive_public_key(sender_secret_key);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::PUBLIC_KEY_SIZE], &ephemeral_public_key)?;
    G::serialize_public_key(
        &mut kem_context[G::PUBLIC_KEY_SIZE..2 * G::PUBLIC_KEY_SIZE],
        recipient_public_key,
    )?;
    G::serialize_public_key(
        &mut kem_context[2 * G::PUBLIC_KEY_SIZE..3 * G::PUBLIC_KEY_SIZE],
        &sender_public_key,
    )?;

    let shared_secret = extract_and_expand::<K>(
        kem_id,
        &diffie_hellman[..2 * G::DIFFIE_HELLMAN_OUTPUT_SIZE],
        &kem_context[..3 * G::PUBLIC_KEY_SIZE],
    )?;
    wipe(&mut diffie_hellman);
    return Ok((shared_secret, ephemeral_public_key));
}

/// RFC 9180 Section 4.1 `AuthDecap`.
fn diffie_hellman_kem_authenticated_decap<G: DiffieHellmanGroup, K: Kdf>(
    kem_id: u16,
    encapped_key: &G::PublicKey,
    recipient_secret_key: &G::SecretKey,
    sender_public_key: &G::PublicKey,
) -> Result<Hash, HpkeError> {
    const { assert!(G::SECRET_KEY_SIZE <= MAX_DIFFIE_HELLMAN_OUTPUT_SIZE && G::PUBLIC_KEY_SIZE <= MAX_PUBLIC_KEY_SIZE) };

    // diffie_hellman = DH(recipient_secret_key, ephemeral_public_key)
    //               || DH(recipient_secret_key, sender_public_key)
    let mut diffie_hellman = [0u8; 2 * MAX_DIFFIE_HELLMAN_OUTPUT_SIZE];
    G::diffie_hellman(
        recipient_secret_key,
        encapped_key,
        &mut diffie_hellman[..G::DIFFIE_HELLMAN_OUTPUT_SIZE],
    )
    .map_err(|_| HpkeError::DecapError)?;
    G::diffie_hellman(
        recipient_secret_key,
        sender_public_key,
        &mut diffie_hellman[G::DIFFIE_HELLMAN_OUTPUT_SIZE..2 * G::DIFFIE_HELLMAN_OUTPUT_SIZE],
    )
    .map_err(|_| HpkeError::DecapError)?;

    // kem_context = encapped_key || recipient_public_key || sender_public_key
    let recipient_public_key = G::derive_public_key(recipient_secret_key);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::PUBLIC_KEY_SIZE], encapped_key)?;
    G::serialize_public_key(
        &mut kem_context[G::PUBLIC_KEY_SIZE..2 * G::PUBLIC_KEY_SIZE],
        &recipient_public_key,
    )?;
    G::serialize_public_key(
        &mut kem_context[2 * G::PUBLIC_KEY_SIZE..3 * G::PUBLIC_KEY_SIZE],
        sender_public_key,
    )?;

    let shared_secret = extract_and_expand::<K>(
        kem_id,
        &diffie_hellman[..2 * G::DIFFIE_HELLMAN_OUTPUT_SIZE],
        &kem_context[..3 * G::PUBLIC_KEY_SIZE],
    )?;
    wipe(&mut diffie_hellman);
    return Ok(shared_secret);
}

/// Defines a concrete Diffie-Hellman KEM type implementing [`Kem`] on top of a private
/// [`DiffieHellmanGroup`] and a KDF.
macro_rules! diffie_hellman_kem {
    (
        $(#[$meta:meta])*
        $name:ident, $group:ty, $kdf:ty, $id:literal, $secret_key:ty, $public_key:ty
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name;

        impl Kem for $name {
            const HPKE_KEM_ID: u16 = $id;
            const SHARED_SECRET_SIZE: usize = <$kdf as Kdf>::OUTPUT_SIZE;
            const ENCAPPED_KEY_SIZE: usize = <$group as DiffieHellmanGroup>::PUBLIC_KEY_SIZE;
            const PUBLIC_KEY_SIZE: usize = <$group as DiffieHellmanGroup>::PUBLIC_KEY_SIZE;
            const SECRET_KEY_SIZE: usize = <$group as DiffieHellmanGroup>::SECRET_KEY_SIZE;

            type PublicKey = $public_key;
            type SecretKey = $secret_key;
            type EncappedKey = $public_key;

            fn derive_keypair(input_keying_material: &[u8]) -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
                return diffie_hellman_kem_derive_keypair::<$group, $kdf>($id, input_keying_material);
            }

            fn derive_public_key(secret_key: &Self::SecretKey) -> Self::PublicKey {
                return <$group as DiffieHellmanGroup>::derive_public_key(secret_key);
            }

            fn public_key_to_bytes(out: &mut [u8], public_key: &Self::PublicKey) -> Result<(), HpkeError> {
                return <$group as DiffieHellmanGroup>::serialize_public_key(out, public_key);
            }

            fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
                return <$group as DiffieHellmanGroup>::deserialize_public_key(bytes);
            }

            fn secret_key_to_bytes(out: &mut [u8], secret_key: &Self::SecretKey) -> Result<(), HpkeError> {
                return <$group as DiffieHellmanGroup>::serialize_secret_key(out, secret_key);
            }

            fn secret_key_from_bytes(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
                return <$group as DiffieHellmanGroup>::deserialize_secret_key(bytes);
            }

            fn encapped_key_to_bytes(out: &mut [u8], encapped_key: &Self::EncappedKey) -> Result<(), HpkeError> {
                return <$group as DiffieHellmanGroup>::serialize_public_key(out, encapped_key);
            }

            fn encapped_key_from_bytes(bytes: &[u8]) -> Result<Self::EncappedKey, HpkeError> {
                return <$group as DiffieHellmanGroup>::deserialize_public_key(bytes);
            }

            fn encap_with_ephemeral(
                ephemeral_secret_key: &Self::SecretKey,
                recipient_public_key: &Self::PublicKey,
            ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
                return diffie_hellman_kem_encap_with_ephemeral::<$group, $kdf>($id, ephemeral_secret_key, recipient_public_key);
            }

            fn decap(encapped_key: &Self::EncappedKey, recipient_secret_key: &Self::SecretKey) -> Result<Hash, HpkeError> {
                return diffie_hellman_kem_decap::<$group, $kdf>($id, encapped_key, recipient_secret_key);
            }

            fn authenticated_encap_with_ephemeral(
                ephemeral_secret_key: &Self::SecretKey,
                recipient_public_key: &Self::PublicKey,
                sender_secret_key: &Self::SecretKey,
            ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
                return diffie_hellman_kem_authenticated_encap_with_ephemeral::<$group, $kdf>($id, ephemeral_secret_key, recipient_public_key, sender_secret_key);
            }

            fn authenticated_decap(
                encapped_key: &Self::EncappedKey,
                recipient_secret_key: &Self::SecretKey,
                sender_public_key: &Self::PublicKey,
            ) -> Result<Hash, HpkeError> {
                return diffie_hellman_kem_authenticated_decap::<$group, $kdf>($id, encapped_key, recipient_secret_key, sender_public_key);
            }
        }
    };
}

diffie_hellman_kem!(
    /// Diffie-Hellman KEM(X25519, HKDF-SHA256) (KEM id `0x0020`).
    ///
    /// Provided for interoperability with existing RFC 9180 deployments. It is
    /// not post-quantum; new deployments SHOULD prefer the hybrid post-quantum
    /// [`MLKEM768X25519`] KEM.
    X25519HkdfSha256,
    X25519Group,
    HkdfSha256,
    0x0020,
    x25519::SecretKey,
    x25519::PublicKey
);

diffie_hellman_kem!(
    /// Diffie-Hellman KEM(P-256, HKDF-SHA256) (KEM id `0x0010`).
    ///
    /// Provided for interoperability with existing RFC 9180 deployments. It is
    /// not post-quantum; new deployments SHOULD prefer the hybrid post-quantum
    /// [`MLKEM768X25519`] KEM.
    P256HkdfSha256,
    P256Group,
    HkdfSha256,
    0x0010,
    p256::SecretKey,
    p256::PublicKey
);

diffie_hellman_kem!(
    /// Diffie-Hellman KEM(P-521, HKDF-SHA512) (KEM id `0x0012`).
    ///
    /// Provided for interoperability with existing RFC 9180 deployments. It is
    /// not post-quantum; new deployments SHOULD prefer the hybrid post-quantum
    /// [`MLKEM768X25519`] KEM.
    P521HkdfSha512,
    P521Group,
    HkdfSha512,
    0x0012,
    p521::SecretKey,
    p521::PublicKey
);
