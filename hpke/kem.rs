//! Key encapsulation mechanisms (KEMs) for HPKE (RFC 9180 Section 4 and Section 7.1).

use crypto::{Hash, curve25519::x25519, p256, p521};

use super::{
    HpkeError,
    kdf::{HkdfSha256, HkdfSha512, Kdf},
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
/// Auth and AuthPSK modes additionally require `auth_encap`/`auth_decap`,
/// which are optional (RFC 9180 Section 7.1.5). KEMs without them (e.g.
/// ML-KEM-based or hybrid post-quantum KEMs such as [`XWing`]) support only
/// the Base and PSK modes; calling a setup function with an authenticated
/// mode returns [`HpkeError::NotSupported`].
///
/// # Example
///
/// ```
/// use hpke::kem::{Kem, X25519HkdfSha256};
///
/// let (sk, pk) = X25519HkdfSha256::generate_keypair().unwrap();
/// let (shared_secret, enc) = X25519HkdfSha256::encap(&pk).unwrap();
/// let decapsulated = X25519HkdfSha256::decap(&enc, &sk).unwrap();
/// assert_eq!(shared_secret.as_ref(), decapsulated.as_ref());
/// ```
///
/// # Custom KEMs
///
/// Implement this trait to use a custom KEM (e.g. ML-KEM for the
/// post-quantum HPKE draft) inside HPKE. The shared secret returned by
/// `encap`/`decap` must be exactly [`Kem::NSECRET`] bytes long. Use
/// [`Kdf::labeled_extract`] and
/// [`Kdf::labeled_expand`] to derive keys exactly
/// like the RFC 9180 KEMs do.
pub trait Kem: Sized {
    /// The HPKE KEM identifier (RFC 9180 Section 7.1, IANA "HPKE KEM
    /// Identifiers"), used to build the ciphersuite `suite_id`.
    const ID: u16;

    /// The size `Nsecret` of the shared secret produced by this KEM.
    ///
    /// Must not exceed 64 bytes, the maximum capacity of [`struct@Hash`].
    const NSECRET: usize;

    /// The size `Nenc` of an encapsulated key.
    const NENC: usize;

    /// The size `Npk` of a serialized public key.
    const NPK: usize;

    /// The size `Nsk` of a serialized secret key.
    const NSK: usize;

    /// The public (encapsulation) key type.
    type PublicKey: Clone;

    /// The secret (decapsulation) key type.
    type SecretKey: Clone;

    /// The encapsulated key type (the KEM ciphertext sent alongside the
    /// first encrypted message).
    type EncappedKey: Clone;

    /// Deterministically derives a key pair from input keying material.
    ///
    /// `ikm` SHOULD be at least `Nsk` bytes long and contain at least `Nsk`
    /// bytes of entropy (RFC 9180 Section 7.1.3). It MUST NOT be reused for
    /// any other purpose, in particular not with another KEM.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeriveKeyPairError`] when no valid key pair could
    /// be derived (rejection sampling exhausted).
    fn derive_keypair(ikm: &[u8]) -> Result<(Self::SecretKey, Self::PublicKey), HpkeError>;

    /// Generates a fresh random key pair using the operating system's random
    /// number generator.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::Random`] when the operating system's random
    /// number generator is unavailable or fails.
    #[cfg(feature = "random")]
    fn generate_keypair() -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
        // Maximum `Nsk` across the algorithms shipped by this crate (P-521:
        // 66 bytes). Custom KEMs with a larger `NSK` must override this method.
        const MAX_NSK: usize = 66;
        const { assert!(Self::NSK <= MAX_NSK, "NSK is too large; override generate_keypair") };

        let mut ikm = [0u8; MAX_NSK];
        crypto::random::fill(&mut ikm[..Self::NSK])?;
        return Self::derive_keypair(&ikm[..Self::NSK]);
    }

    /// Derives the public key of a secret key.
    fn sk_to_pk(sk: &Self::SecretKey) -> Self::PublicKey;

    /// Serializes a public key into `out`, which must be exactly
    /// [`Kem::NPK`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `out` has the wrong
    /// length.
    fn public_key_to_bytes(out: &mut [u8], pk: &Self::PublicKey) -> Result<(), HpkeError>;

    /// Deserializes a public key from `bytes`, which must be exactly
    /// [`Kem::NPK`] bytes long and a valid, fully validated public key.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `bytes` has the wrong
    /// length or does not encode a valid public key.
    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError>;

    /// Serializes a secret key into `out`, which must be exactly
    /// [`Kem::NSK`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `out` has the wrong
    /// length.
    fn secret_key_to_bytes(out: &mut [u8], sk: &Self::SecretKey) -> Result<(), HpkeError>;

    /// Deserializes a secret key from `bytes`, which must be exactly
    /// [`Kem::NSK`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `bytes` has the wrong
    /// length or does not encode a valid secret key.
    fn secret_key_from_bytes(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError>;

    /// Serializes an encapsulated key into `out`, which must be exactly
    /// [`Kem::NENC`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `out` has the wrong
    /// length.
    fn encapped_key_to_bytes(out: &mut [u8], enc: &Self::EncappedKey) -> Result<(), HpkeError>;

    /// Deserializes an encapsulated key from `bytes`, which must be exactly
    /// [`Kem::NENC`] bytes long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `bytes` has the wrong
    /// length or does not encode a valid encapsulated key.
    fn encapped_key_from_bytes(bytes: &[u8]) -> Result<Self::EncappedKey, HpkeError>;

    /// Encapsulates a shared secret against `pk_r`, using the given ephemeral
    /// secret key `sk_e`.
    ///
    /// Returns the shared secret and the encapsulated key that the owner of
    /// `pk_r` can use to recover it. This is the deterministic variant of
    /// [`Kem::encap`]; it exists for protocols that need reproducible
    /// encapsulation (e.g. test vectors) and MUST NOT be called with a
    /// non-fresh `sk_e`.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::EncapError`] when `pk_r` is invalid.
    fn encap_with_ephemeral(
        sk_e: &Self::SecretKey,
        pk_r: &Self::PublicKey,
    ) -> Result<(Hash, Self::EncappedKey), HpkeError>;

    /// Encapsulates a fresh shared secret against `pk_r` using a freshly
    /// generated ephemeral key pair.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::EncapError`] when `pk_r` is invalid, or
    /// [`HpkeError::Random`] when the operating system's random number
    /// generator is unavailable or fails.
    #[cfg(feature = "random")]
    fn encap(pk_r: &Self::PublicKey) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        let (sk_e, _pk_e) = Self::generate_keypair()?;
        return Self::encap_with_ephemeral(&sk_e, pk_r);
    }

    /// Recovers the shared secret encapsulated in `enc` with the secret key
    /// `sk_r`.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DecapError`] when `enc` is invalid or when the
    /// key exchange fails.
    fn decap(enc: &Self::EncappedKey, sk_r: &Self::SecretKey) -> Result<Hash, HpkeError>;

    /// Authenticated encapsulation against `pk_r` proving possession of the
    /// secret key `sk_s` (RFC 9180 Section 4.1 `AuthEncap`), using the given
    /// ephemeral secret key `sk_e`.
    ///
    /// The default implementation always fails: authenticated encapsulation
    /// is optional for a KEM (RFC 9180 Section 7.1.5).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::NotSupported`] by default, and
    /// [`HpkeError::EncapError`] when `pk_r` is invalid.
    fn auth_encap_with_ephemeral(
        _sk_e: &Self::SecretKey,
        _pk_r: &Self::PublicKey,
        _sk_s: &Self::SecretKey,
    ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        return Err(HpkeError::NotSupported);
    }

    /// Authenticated encapsulation with a freshly generated ephemeral key
    /// pair. See [`Kem::auth_encap_with_ephemeral`].
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::NotSupported`] by default, [`HpkeError::EncapError`]
    /// when `pk_r` is invalid, or [`HpkeError::Random`] when the operating
    /// system's random number generator is unavailable or fails.
    #[cfg(feature = "random")]
    fn auth_encap(pk_r: &Self::PublicKey, sk_s: &Self::SecretKey) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        let (sk_e, _pk_e) = Self::generate_keypair()?;
        return Self::auth_encap_with_ephemeral(&sk_e, pk_r, sk_s);
    }

    /// Authenticated decapsulation of `enc` with `sk_r`, verifying that the
    /// encapsulation was produced with the secret key matching `pk_s`
    /// (RFC 9180 Section 4.1 `AuthDecap`).
    ///
    /// The default implementation always fails: authenticated decapsulation
    /// is optional for a KEM (RFC 9180 Section 7.1.5).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::NotSupported`] by default, and
    /// [`HpkeError::DecapError`] when `enc` is invalid.
    fn auth_decap(
        _enc: &Self::EncappedKey,
        _sk_r: &Self::SecretKey,
        _pk_s: &Self::PublicKey,
    ) -> Result<Hash, HpkeError> {
        return Err(HpkeError::NotSupported);
    }
}

/// X-Wing hybrid post-quantum KEM (ML-KEM-768 with X25519), as specified in
/// `draft-connolly-cfrg-xwing-kem`.
///
/// X-Wing is usable with the Base and PSK modes of HPKE; it does not support
/// the authenticated modes (Auth and AuthPSK), which return
/// [`HpkeError::NotSupported`].
///
/// The KEM identifier is `0x647a`, as registered by the draft in the IANA
/// "HPKE KEM Identifiers" registry.
///
/// # Example
///
/// ```
/// use hpke::kem::{Kem, XWing};
///
/// let (sk, pk) = XWing::generate_keypair().unwrap();
/// let (shared_secret, enc) = XWing::encap(&pk).unwrap();
/// let decapsulated = XWing::decap(&enc, &sk).unwrap();
/// assert_eq!(shared_secret.as_ref(), decapsulated.as_ref());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XWing;

impl Kem for XWing {
    const ID: u16 = 0x647a;
    const NSECRET: usize = crypto::xwing::SHARED_SECRET_SIZE;
    const NENC: usize = crypto::xwing::CIPHERTEXT_SIZE;
    const NPK: usize = crypto::xwing::PUBLIC_KEY_SIZE;
    const NSK: usize = crypto::xwing::SECRET_KEY_SIZE;

    type PublicKey = crypto::xwing::PublicKey;
    type SecretKey = crypto::xwing::SecretKey;
    type EncappedKey = [u8; crypto::xwing::CIPHERTEXT_SIZE];

    fn derive_keypair(ikm: &[u8]) -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
        // Draft-connolly-cfrg-xwing-kem, Section 5.6:
        //   `sk = SHAKE256(ikm, 32 * 8); return GenerateKeyPairDerand(sk)`.
        let mut seed = [0u8; crypto::xwing::SECRET_KEY_SIZE];
        crypto::sha3::Shake256::hash(ikm, &mut seed);

        let keypair = crypto::xwing::generate_keypair_derand(&seed);
        wipe(&mut seed);
        return Ok(keypair);
    }

    #[cfg(feature = "random")]
    fn generate_keypair() -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
        return crypto::xwing::generate_keypair().map_err(HpkeError::from);
    }

    fn sk_to_pk(sk: &Self::SecretKey) -> Self::PublicKey {
        return sk.public_key();
    }

    fn public_key_to_bytes(out: &mut [u8], pk: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::NPK {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&pk.to_bytes());
        return Ok(());
    }

    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        let bytes: &[u8; Self::NPK] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return crypto::xwing::PublicKey::from_bytes(bytes).map_err(HpkeError::from);
    }

    fn secret_key_to_bytes(out: &mut [u8], sk: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::NSK {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&sk.to_bytes());
        return Ok(());
    }

    fn secret_key_from_bytes(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::NSK] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return Ok(crypto::xwing::generate_keypair_derand(bytes).0);
    }

    fn encapped_key_to_bytes(out: &mut [u8], enc: &Self::EncappedKey) -> Result<(), HpkeError> {
        if out.len() != Self::NENC {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(enc);
        return Ok(());
    }

    fn encapped_key_from_bytes(bytes: &[u8]) -> Result<Self::EncappedKey, HpkeError> {
        let bytes: &[u8; Self::NENC] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return Ok(*bytes);
    }

    fn encap_with_ephemeral(
        sk_e: &Self::SecretKey,
        pk_r: &Self::PublicKey,
    ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
        // The trait models the ephemeral secret as a KEM secret key; X-Wing
        // encapsulation consumes a 64-byte seed. Expand the 32-byte ephemeral
        // seed deterministically with SHAKE256.
        let mut eseed = [0u8; 64];
        crypto::sha3::Shake256::hash(&sk_e.to_bytes(), &mut eseed);

        let (shared_secret, enc) = pk_r.encapsulate_derand(&eseed);
        wipe(&mut eseed);
        return Ok((Hash::from(shared_secret), enc));
    }

    fn decap(enc: &Self::EncappedKey, sk_r: &Self::SecretKey) -> Result<Hash, HpkeError> {
        let shared_secret = sk_r.decapsulate(enc).map_err(|_| HpkeError::DecapError)?;
        return Ok(Hash::from(shared_secret));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// DHKEM (RFC 9180 Section 4.1)

/// Maximum serialized public key / encapsulated key size across the supported
/// groups (P-521: 133 bytes).
const MAX_NPK: usize = 133;
/// Maximum secret key and Diffie-Hellman output size across the supported
/// groups (P-521: 66 bytes).
const MAX_NDH: usize = 66;
/// Maximum size of a KEM context: `enc || pkRm [|| pkSm]`.
const MAX_KEM_CONTEXT: usize = 3 * MAX_NPK;

/// Builds the DHKEM `suite_id`: `"KEM" || I2OSP(kem_id, 2)` (RFC 9180
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

/// A Diffie-Hellman group usable by the DHKEM construction.
///
/// This trait is private: the crate provides the groups required by RFC 9180.
/// Custom KEMs are expected to implement [`Kem`] directly (the DHKEM
/// construction can be reproduced with [`Kdf::labeled_extract`] and
/// [`Kdf::labeled_expand`]).
trait DhGroup: Copy + Clone {
    /// Size of the raw Diffie-Hellman output.
    const NDH: usize;
    /// Size of a serialized public key.
    const NPK: usize;
    /// Size of a serialized secret key.
    const NSK: usize;
    /// Mask applied to the first byte of a NIST candidate scalar.
    const BITMASK: u8;
    /// Whether `DeriveKeyPair` uses NIST rejection sampling (true) or the
    /// single-shot X25519 derivation (false).
    const REJECTION_SAMPLING: bool;

    type SecretKey: Clone;
    type PublicKey: Clone;

    /// Computes the raw Diffie-Hellman shared secret, written into `out`
    /// (`out.len() == NDH`).
    ///
    /// Returns `Err(())` on failure (e.g. an all-zero X25519 output).
    fn dh(sk: &Self::SecretKey, pk: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()>;

    /// Derives the public key of a secret key.
    fn sk_to_pk(sk: &Self::SecretKey) -> Self::PublicKey;

    /// Serializes a public key into `out` (`out.len() == NPK`).
    fn serialize_public_key(out: &mut [u8], pk: &Self::PublicKey) -> Result<(), HpkeError>;

    /// Deserializes and validates a public key.
    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError>;

    /// Serializes a secret key into `out` (`out.len() == NSK`).
    fn serialize_secret_key(out: &mut [u8], sk: &Self::SecretKey) -> Result<(), HpkeError>;

    /// Deserializes a secret key.
    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError>;

    /// Interprets a candidate scalar for `DeriveKeyPair`, returning `Err(())`
    /// when it is out of range (rejection sampling).
    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct X25519Group;

impl DhGroup for X25519Group {
    const NDH: usize = x25519::SHARED_SECRET_SIZE;
    const NPK: usize = x25519::KEY_SIZE;
    const NSK: usize = x25519::KEY_SIZE;
    const BITMASK: u8 = 0xff;
    const REJECTION_SAMPLING: bool = false;

    type SecretKey = x25519::SecretKey;
    type PublicKey = x25519::PublicKey;

    fn dh(sk: &Self::SecretKey, pk: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()> {
        // `ecdh` rejects all-zero shared secrets, as RFC 9180 Section 7.1.4
        // requires for X25519.
        let shared = sk.ecdh(pk).map_err(|_| ())?;
        out.copy_from_slice(&shared);
        return Ok(());
    }

    fn sk_to_pk(sk: &Self::SecretKey) -> Self::PublicKey {
        return sk.public_key();
    }

    fn serialize_public_key(out: &mut [u8], pk: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::NPK {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&pk.to_bytes());
        return Ok(());
    }

    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        let bytes: &[u8; Self::NPK] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return Ok(x25519::PublicKey::from_bytes(bytes));
    }

    fn serialize_secret_key(out: &mut [u8], sk: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::NSK {
            return Err(HpkeError::DeserializeError);
        }
        // RFC 9180 Section 7.1.2 requires `SerializePrivateKey` to clamp.
        out.copy_from_slice(&clamp_scalar25519(sk.to_bytes()));
        return Ok(());
    }

    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::NSK] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        // RFC 9180 Section 7.1.2 requires `DeserializePrivateKey` to clamp.
        return Ok(x25519::SecretKey::from_bytes(&clamp_scalar25519(*bytes)));
    }

    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()> {
        let bytes: &[u8; Self::NSK] = bytes.try_into().map_err(|_| ())?;
        return Ok(x25519::SecretKey::from_bytes(&clamp_scalar25519(*bytes)));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P256Group;

impl DhGroup for P256Group {
    const NDH: usize = p256::ECDH_SHARED_SECRET_SIZE;
    const NPK: usize = p256::PUBLIC_KEY_UNCOMPRESSED_SIZE;
    const NSK: usize = p256::SECRET_KEY_SIZE;
    const BITMASK: u8 = 0xff;
    const REJECTION_SAMPLING: bool = true;

    type SecretKey = p256::SecretKey;
    type PublicKey = p256::PublicKey;

    fn dh(sk: &Self::SecretKey, pk: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()> {
        let shared = sk.ecdh(pk).map_err(|_| ())?;
        out.copy_from_slice(&shared);
        return Ok(());
    }

    fn sk_to_pk(sk: &Self::SecretKey) -> Self::PublicKey {
        return sk.public_key();
    }

    fn serialize_public_key(out: &mut [u8], pk: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::NPK {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&pk.to_bytes());
        return Ok(());
    }

    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        // RFC 9180 Section 7.1.1 requires the uncompressed SEC1 encoding.
        if bytes.len() != Self::NPK || bytes[0] != 0x04 {
            return Err(HpkeError::DeserializeError);
        }
        return p256::PublicKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn serialize_secret_key(out: &mut [u8], sk: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::NSK {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&sk.to_bytes());
        return Ok(());
    }

    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::NSK] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return p256::SecretKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()> {
        let bytes: &[u8; Self::NSK] = bytes.try_into().map_err(|_| ())?;
        return p256::SecretKey::from_bytes(bytes).map_err(|_| ());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P521Group;

impl DhGroup for P521Group {
    const NDH: usize = p521::ECDH_SHARED_SECRET_SIZE;
    const NPK: usize = p521::PUBLIC_KEY_UNCOMPRESSED_SIZE;
    const NSK: usize = p521::SECRET_KEY_SIZE;
    const BITMASK: u8 = 0x01;
    const REJECTION_SAMPLING: bool = true;

    type SecretKey = p521::SecretKey;
    type PublicKey = p521::PublicKey;

    fn dh(sk: &Self::SecretKey, pk: &Self::PublicKey, out: &mut [u8]) -> Result<(), ()> {
        let shared = sk.ecdh(pk).map_err(|_| ())?;
        out.copy_from_slice(&shared);
        return Ok(());
    }

    fn sk_to_pk(sk: &Self::SecretKey) -> Self::PublicKey {
        return sk.public_key();
    }

    fn serialize_public_key(out: &mut [u8], pk: &Self::PublicKey) -> Result<(), HpkeError> {
        if out.len() != Self::NPK {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&pk.to_bytes());
        return Ok(());
    }

    fn deserialize_public_key(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
        if bytes.len() != Self::NPK || bytes[0] != 0x04 {
            return Err(HpkeError::DeserializeError);
        }
        return p521::PublicKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn serialize_secret_key(out: &mut [u8], sk: &Self::SecretKey) -> Result<(), HpkeError> {
        if out.len() != Self::NSK {
            return Err(HpkeError::DeserializeError);
        }
        out.copy_from_slice(&sk.to_bytes());
        return Ok(());
    }

    fn deserialize_secret_key(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
        let bytes: &[u8; Self::NSK] = bytes.try_into().map_err(|_| HpkeError::DeserializeError)?;
        return p521::SecretKey::from_bytes(bytes).map_err(|_| HpkeError::DeserializeError);
    }

    fn scalar_from_candidate(bytes: &[u8]) -> Result<Self::SecretKey, ()> {
        let bytes: &[u8; Self::NSK] = bytes.try_into().map_err(|_| ())?;
        return p521::SecretKey::from_bytes(bytes).map_err(|_| ());
    }
}

/// RFC 9180 Section 4.1 `ExtractAndExpand`.
fn extract_and_expand<K: Kdf>(kem_id: u16, dh: &[u8], kem_context: &[u8]) -> Result<Hash, HpkeError> {
    const { assert!(K::NH <= crypto::MAX_HASH_OUTPUT_SIZE) };

    let suite_id = kem_suite_id(kem_id);
    let mut eae_prk = K::labeled_extract(b"", &suite_id, b"eae_prk", dh);

    let mut shared_secret_bytes = [0u8; crypto::MAX_HASH_OUTPUT_SIZE];
    K::labeled_expand(
        &mut shared_secret_bytes[..K::NH],
        &eae_prk,
        &suite_id,
        b"shared_secret",
        kem_context,
    )?;
    let shared_secret = Hash::try_from(&shared_secret_bytes[..K::NH]).expect("NSECRET fits in a Hash");

    wipe(&mut shared_secret_bytes);
    wipe(eae_prk.as_mut());
    return Ok(shared_secret);
}

/// RFC 9180 Section 7.1.3 `DeriveKeyPair` for a DHKEM.
fn dhkem_derive_keypair<G: DhGroup, K: Kdf>(
    kem_id: u16,
    ikm: &[u8],
) -> Result<(G::SecretKey, G::PublicKey), HpkeError> {
    const { assert!(G::NSK <= MAX_NDH && G::NPK <= MAX_NPK) };

    let suite_id = kem_suite_id(kem_id);
    let mut dkp_prk = K::labeled_extract(b"", &suite_id, b"dkp_prk", ikm);

    if G::REJECTION_SAMPLING {
        let mut candidate = [0u8; MAX_NDH];
        for counter in 0..=255u8 {
            K::labeled_expand(&mut candidate[..G::NSK], &dkp_prk, &suite_id, b"candidate", &[counter])?;
            candidate[0] &= G::BITMASK;

            if let Ok(sk) = G::scalar_from_candidate(&candidate[..G::NSK]) {
                let pk = G::sk_to_pk(&sk);
                wipe(&mut candidate);
                wipe(dkp_prk.as_mut());
                return Ok((sk, pk));
            }
        }
        wipe(&mut candidate);
        wipe(dkp_prk.as_mut());
        return Err(HpkeError::DeriveKeyPairError);
    }

    let mut sk_bytes = [0u8; MAX_NDH];
    K::labeled_expand(&mut sk_bytes[..G::NSK], &dkp_prk, &suite_id, b"sk", b"")?;
    let sk = G::scalar_from_candidate(&sk_bytes[..G::NSK]).map_err(|_| HpkeError::DeriveKeyPairError)?;
    let pk = G::sk_to_pk(&sk);
    wipe(&mut sk_bytes);
    wipe(dkp_prk.as_mut());
    return Ok((sk, pk));
}

/// RFC 9180 Section 4.1 `Encap` with a caller-provided ephemeral key.
fn dhkem_encap_with_ephemeral<G: DhGroup, K: Kdf>(
    kem_id: u16,
    sk_e: &G::SecretKey,
    pk_r: &G::PublicKey,
) -> Result<(Hash, G::PublicKey), HpkeError> {
    const { assert!(G::NSK <= MAX_NDH && G::NPK <= MAX_NPK) };

    let mut dh = [0u8; MAX_NDH];
    G::dh(sk_e, pk_r, &mut dh[..G::NDH]).map_err(|_| HpkeError::EncapError)?;

    let pk_e = G::sk_to_pk(sk_e);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::NPK], &pk_e)?;
    G::serialize_public_key(&mut kem_context[G::NPK..2 * G::NPK], pk_r)?;

    let shared_secret = extract_and_expand::<K>(kem_id, &dh[..G::NDH], &kem_context[..2 * G::NPK])?;
    wipe(&mut dh);
    return Ok((shared_secret, pk_e));
}

/// RFC 9180 Section 4.1 `Decap`.
fn dhkem_decap<G: DhGroup, K: Kdf>(kem_id: u16, enc: &G::PublicKey, sk_r: &G::SecretKey) -> Result<Hash, HpkeError> {
    const { assert!(G::NSK <= MAX_NDH && G::NPK <= MAX_NPK) };

    let mut dh = [0u8; MAX_NDH];
    G::dh(sk_r, enc, &mut dh[..G::NDH]).map_err(|_| HpkeError::DecapError)?;

    let pk_r = G::sk_to_pk(sk_r);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::NPK], enc)?;
    G::serialize_public_key(&mut kem_context[G::NPK..2 * G::NPK], &pk_r)?;

    let shared_secret = extract_and_expand::<K>(kem_id, &dh[..G::NDH], &kem_context[..2 * G::NPK])?;
    wipe(&mut dh);
    return Ok(shared_secret);
}

/// RFC 9180 Section 4.1 `AuthEncap` with a caller-provided ephemeral key.
fn dhkem_auth_encap_with_ephemeral<G: DhGroup, K: Kdf>(
    kem_id: u16,
    sk_e: &G::SecretKey,
    pk_r: &G::PublicKey,
    sk_s: &G::SecretKey,
) -> Result<(Hash, G::PublicKey), HpkeError> {
    const { assert!(G::NSK <= MAX_NDH && G::NPK <= MAX_NPK) };

    // dh = DH(skE, pkR) || DH(skS, pkR)
    let mut dh = [0u8; 2 * MAX_NDH];
    G::dh(sk_e, pk_r, &mut dh[..G::NDH]).map_err(|_| HpkeError::EncapError)?;
    G::dh(sk_s, pk_r, &mut dh[G::NDH..2 * G::NDH]).map_err(|_| HpkeError::EncapError)?;

    // kem_context = enc || pkRm || pkSm
    let pk_e = G::sk_to_pk(sk_e);
    let pk_s = G::sk_to_pk(sk_s);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::NPK], &pk_e)?;
    G::serialize_public_key(&mut kem_context[G::NPK..2 * G::NPK], pk_r)?;
    G::serialize_public_key(&mut kem_context[2 * G::NPK..3 * G::NPK], &pk_s)?;

    let shared_secret = extract_and_expand::<K>(kem_id, &dh[..2 * G::NDH], &kem_context[..3 * G::NPK])?;
    wipe(&mut dh);
    return Ok((shared_secret, pk_e));
}

/// RFC 9180 Section 4.1 `AuthDecap`.
fn dhkem_auth_decap<G: DhGroup, K: Kdf>(
    kem_id: u16,
    enc: &G::PublicKey,
    sk_r: &G::SecretKey,
    pk_s: &G::PublicKey,
) -> Result<Hash, HpkeError> {
    const { assert!(G::NSK <= MAX_NDH && G::NPK <= MAX_NPK) };

    // dh = DH(skR, pkE) || DH(skR, pkS)
    let mut dh = [0u8; 2 * MAX_NDH];
    G::dh(sk_r, enc, &mut dh[..G::NDH]).map_err(|_| HpkeError::DecapError)?;
    G::dh(sk_r, pk_s, &mut dh[G::NDH..2 * G::NDH]).map_err(|_| HpkeError::DecapError)?;

    // kem_context = enc || pkRm || pkSm
    let pk_r = G::sk_to_pk(sk_r);
    let mut kem_context = [0u8; MAX_KEM_CONTEXT];
    G::serialize_public_key(&mut kem_context[..G::NPK], enc)?;
    G::serialize_public_key(&mut kem_context[G::NPK..2 * G::NPK], &pk_r)?;
    G::serialize_public_key(&mut kem_context[2 * G::NPK..3 * G::NPK], pk_s)?;

    let shared_secret = extract_and_expand::<K>(kem_id, &dh[..2 * G::NDH], &kem_context[..3 * G::NPK])?;
    wipe(&mut dh);
    return Ok(shared_secret);
}

/// Defines a concrete DHKEM type implementing [`Kem`] on top of a private
/// [`DhGroup`] and a KDF.
macro_rules! dhkem {
    (
        $(#[$meta:meta])*
        $name:ident, $group:ty, $kdf:ty, $id:literal, $secret_key:ty, $public_key:ty
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name;

        impl Kem for $name {
            const ID: u16 = $id;
            const NSECRET: usize = <$kdf as Kdf>::NH;
            const NENC: usize = <$group as DhGroup>::NPK;
            const NPK: usize = <$group as DhGroup>::NPK;
            const NSK: usize = <$group as DhGroup>::NSK;

            type PublicKey = $public_key;
            type SecretKey = $secret_key;
            type EncappedKey = $public_key;

            fn derive_keypair(ikm: &[u8]) -> Result<(Self::SecretKey, Self::PublicKey), HpkeError> {
                return dhkem_derive_keypair::<$group, $kdf>($id, ikm);
            }

            fn sk_to_pk(sk: &Self::SecretKey) -> Self::PublicKey {
                return <$group as DhGroup>::sk_to_pk(sk);
            }

            fn public_key_to_bytes(out: &mut [u8], pk: &Self::PublicKey) -> Result<(), HpkeError> {
                return <$group as DhGroup>::serialize_public_key(out, pk);
            }

            fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, HpkeError> {
                return <$group as DhGroup>::deserialize_public_key(bytes);
            }

            fn secret_key_to_bytes(out: &mut [u8], sk: &Self::SecretKey) -> Result<(), HpkeError> {
                return <$group as DhGroup>::serialize_secret_key(out, sk);
            }

            fn secret_key_from_bytes(bytes: &[u8]) -> Result<Self::SecretKey, HpkeError> {
                return <$group as DhGroup>::deserialize_secret_key(bytes);
            }

            fn encapped_key_to_bytes(out: &mut [u8], enc: &Self::EncappedKey) -> Result<(), HpkeError> {
                return <$group as DhGroup>::serialize_public_key(out, enc);
            }

            fn encapped_key_from_bytes(bytes: &[u8]) -> Result<Self::EncappedKey, HpkeError> {
                return <$group as DhGroup>::deserialize_public_key(bytes);
            }

            fn encap_with_ephemeral(
                sk_e: &Self::SecretKey,
                pk_r: &Self::PublicKey,
            ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
                return dhkem_encap_with_ephemeral::<$group, $kdf>($id, sk_e, pk_r);
            }

            fn decap(enc: &Self::EncappedKey, sk_r: &Self::SecretKey) -> Result<Hash, HpkeError> {
                return dhkem_decap::<$group, $kdf>($id, enc, sk_r);
            }

            fn auth_encap_with_ephemeral(
                sk_e: &Self::SecretKey,
                pk_r: &Self::PublicKey,
                sk_s: &Self::SecretKey,
            ) -> Result<(Hash, Self::EncappedKey), HpkeError> {
                return dhkem_auth_encap_with_ephemeral::<$group, $kdf>($id, sk_e, pk_r, sk_s);
            }

            fn auth_decap(
                enc: &Self::EncappedKey,
                sk_r: &Self::SecretKey,
                pk_s: &Self::PublicKey,
            ) -> Result<Hash, HpkeError> {
                return dhkem_auth_decap::<$group, $kdf>($id, enc, sk_r, pk_s);
            }
        }
    };
}

dhkem!(
    /// DHKEM(X25519, HKDF-SHA256) (KEM id `0x0020`).
    X25519HkdfSha256,
    X25519Group,
    HkdfSha256,
    0x0020,
    x25519::SecretKey,
    x25519::PublicKey
);

dhkem!(
    /// DHKEM(P-256, HKDF-SHA256) (KEM id `0x0010`).
    P256HkdfSha256,
    P256Group,
    HkdfSha256,
    0x0010,
    p256::SecretKey,
    p256::PublicKey
);

dhkem!(
    /// DHKEM(P-521, HKDF-SHA512) (KEM id `0x0012`).
    P521HkdfSha512,
    P521Group,
    HkdfSha512,
    0x0012,
    p521::SecretKey,
    p521::PublicKey
);
