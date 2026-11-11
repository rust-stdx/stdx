//! AEAD algorithms for HPKE ciphersuites (`draft-ietf-hpke-hpke-05` Section 5.2
//! and Section 7.3).
//!
//! The AEAD implementations are owned by the `crypto` crate and exposed here,
//! so users of this crate never need to depend on `crypto` directly:
//!
//! - [`Aes256Gcm`] (AEAD id `0x0002`)
//! - [`ChaCha20Poly1305`] (AEAD id `0x0003`)
//! - [`Aegis256`] (unofficial AEAD id `0xff01`)
//! - [`Aegis256X2`] (unofficial AEAD id `0xff02`)
//! - [`Aegis256X4`] (unofficial AEAD id `0xff03`)
//! - [`ExportOnly`] (AEAD id `0xffff`), the local pseudo-AEAD
//!
//! The AEGIS variants are stdx-private extensions that are not registered in the
//! IANA "HPKE AEAD Identifiers" registry. They use 32-byte nonces (the registered
//! AEADs use 12-byte nonces) and 256-bit tags, so an HPKE ciphertext is 32 bytes
//! longer than its plaintext. The tag size is not encoded in the AEAD identifier:
//! both peers must agree on it.
//!
//! HPKE-specific metadata (the AEAD identifier, message length limits, and the
//! keyed constructor) is added through the [`Aead`] extension trait.

use crypto::{Hash, aegis};
pub use crypto::{aes::Aes256Gcm, chacha::ChaCha20Poly1305};

use super::HpkeError;

/// AEGIS-256 with a 256-bit tag (unofficial AEAD id `0xFF01`).
///
/// This is [`crypto::aegis::Aegis256`] fixed to `TAG = 32`. It uses a 256-bit
/// key, a 256-bit nonce and a 256-bit tag, so the key schedule derives a
/// 32-byte `base_nonce` and an HPKE ciphertext is 32 bytes longer than its
/// plaintext.
///
/// # Interoperability
///
/// AEGIS-256 is **not** registered in the IANA "HPKE AEAD Identifiers" registry
/// and is not specified by any HPKE standard. The identifier `0xFF01` is an
/// stdx-private, unofficial value chosen to stay clear of the low IDs IANA is
/// likely to assign to future official AEADs. Ciphersuites using it only
/// interoperate with implementations that adopt both this identifier and the
/// 256-bit tag size.
pub type Aegis256 = aegis::Aegis256<32>;

/// AEGIS-256X2 with a 256-bit tag (unofficial AEAD id `0xFF02`).
///
/// This is [`crypto::aegis::Aegis256X2`] fixed to `TAG = 32`. It has the same
/// key, nonce and tag sizes as [`Aegis256`]; see that type for the
/// interoperability caveat.
pub type Aegis256X2 = aegis::Aegis256X2<32>;

/// AEGIS-256X4 with a 256-bit tag (unofficial AEAD id `0xFF03`).
///
/// This is [`crypto::aegis::Aegis256X4`] fixed to `TAG = 32`. It has the same
/// key, nonce and tag sizes as [`Aegis256`]; see that type for the
/// interoperability caveat.
pub type Aegis256X4 = aegis::Aegis256X4<32>;

/// Maximum plaintext length for AES-GCM (`P_MAX`), 2^36 - 31 bytes ([RFC 5116]
/// Section 5.2.1.1).
const AES_GCM_MAX_PLAINTEXT_SIZE: u64 = (1u64 << 36) - 31;
/// Maximum ciphertext length for AES-GCM (`C_MAX`), 2^36 - 15 bytes ([RFC 5116]
/// Section 5.2.1.1).
const AES_GCM_MAX_CIPHERTEXT_SIZE: u64 = (1u64 << 36) - 15;
/// Maximum plaintext length for ChaCha20-Poly1305 (`P_MAX`), 2^38 - 64 bytes
/// ([RFC 8439] Section 2.8).
const CHACHA20_POLY1305_MAX_PLAINTEXT_SIZE: u64 = (1u64 << 38) - 64;
/// Maximum ciphertext length for ChaCha20-Poly1305 (`C_MAX`), 2^38 - 48 bytes
/// ([RFC 8439] Section 2.8).
const CHACHA20_POLY1305_MAX_CIPHERTEXT_SIZE: u64 = (1u64 << 38) - 48;
/// Maximum plaintext length for AEGIS (`P_MAX`), 2^61 - 1 bytes ([RFC 10032]
/// Section 4).
const AEGIS_MAX_PLAINTEXT_SIZE: u64 = (1u64 << 61) - 1;
/// Maximum ciphertext length for AEGIS (`C_MAX`) with a 256-bit tag,
/// `P_MAX + 32` bytes ([RFC 10032] Section 4).
const AEGIS_MAX_CIPHERTEXT_SIZE: u64 = AEGIS_MAX_PLAINTEXT_SIZE + 32;

/// HPKE ciphersuite binding for an AEAD algorithm.
///
/// An AEAD is the symmetric cipher used to seal and open messages. This trait
/// only adds what HPKE itself needs on top of [`crypto::Aead`] (from which the
/// nonce size, tag size, and the underlying cipher operations are inherited):
///
/// - [`Aead::HPKE_AEAD_ID`], the IANA "HPKE AEAD Identifiers" value used to build
///   the ciphersuite `suite_id`,
/// - [`Aead::MAX_PLAINTEXT_SIZE`] and [`Aead::MAX_CIPHERTEXT_SIZE`], the message
///   length limits `P_MAX` and `C_MAX` defined by [RFC 5116],
/// - [`Aead::new`], the construction of a ready-to-use cipher from the
///   raw key schedule output.
///
/// # Example
///
/// ```
/// use hpke::aead::{Aead, Aegis256, Aes256Gcm, ExportOnly};
///
/// let cipher = <Aes256Gcm as Aead>::new(&[0x42u8; 32]).unwrap();
/// assert_eq!(Aes256Gcm::HPKE_AEAD_ID, 0x0002);
/// assert_eq!(Aegis256::HPKE_AEAD_ID, 0xff01);
/// assert_eq!(ExportOnly::HPKE_AEAD_ID, 0xffff);
/// # let _ = cipher;
/// ```
///
/// # Custom AEADs
///
/// Implement [`crypto::Aead`] for the cipher's operations, then implement this
/// trait to give it an HPKE identifier, key size, and message length limits. Any
/// nonce and tag size is supported as long as `KEY_SIZE` is at most 64 bytes and
/// `NONCE_SIZE` is between 8 and 64 bytes; larger values fail to compile when
/// the cipher is used with [`new_sender`](crate::new_sender) or
/// [`new_recipient`](crate::new_recipient).
pub trait Aead: Sized + crypto::Aead {
    /// The HPKE AEAD identifier (`draft-ietf-hpke-hpke-05` Section 7.3, IANA
    /// "HPKE AEAD Identifiers"), used to build the ciphersuite `suite_id`.
    const HPKE_AEAD_ID: u16;

    /// Maximum length, in bytes, of a plaintext passed to [`Aead::seal`], i.e.
    /// `P_MAX` ([RFC 5116]).
    ///
    /// HPKE refuses to encrypt longer plaintexts with
    /// [`HpkeError::MessageLimitReached`]: exceeding `P_MAX` would void the
    /// confidentiality and integrity guarantees of the AEAD
    /// (`draft-ietf-hpke-hpke-05` Section 5.2). There is no default; every AEAD
    /// must state its limit.
    const MAX_PLAINTEXT_SIZE: u64;

    /// Maximum length, in bytes, of a ciphertext passed to
    /// [`Aead::open`], i.e. `C_MAX` ([RFC 5116]).
    ///
    /// HPKE refuses to open longer ciphertexts with
    /// [`HpkeError::MessageLimitReached`] (`draft-ietf-hpke-hpke-05`
    /// Section 5.2). There is no default; every AEAD must state its limit.
    const MAX_CIPHERTEXT_SIZE: u64;

    /// Instantiates the cipher from `KEY_SIZE` raw key bytes, as produced by
    /// the HPKE key schedule.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::InvalidKey`] when `key` does not have exactly
    /// [`crypto::Aead::KEY_SIZE`] bytes.
    fn new(key: &[u8]) -> Result<Self, HpkeError>;

    /// Encrypts `in_out` in place and returns the detached authentication tag,
    /// forwarding to [`crypto::Aead::encrypt_in_place`].
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::NotSupported`] when the cipher is the
    /// [`ExportOnly`] pseudo-AEAD.
    fn seal(&self, in_out: &mut [u8], nonce: &[u8], associated_data: &[u8]) -> Result<Hash, HpkeError> {
        return Ok(crypto::Aead::encrypt_in_place(self, in_out, nonce, associated_data));
    }

    /// Decrypts `in_out` in place using the detached authentication `tag`,
    /// forwarding to [`crypto::Aead::decrypt_in_place`].
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::OpenError`] when authentication fails, or
    /// [`HpkeError::NotSupported`] when the cipher is the [`ExportOnly`]
    /// pseudo-AEAD.
    fn open(&self, in_out: &mut [u8], nonce: &[u8], associated_data: &[u8], tag: &[u8]) -> Result<(), HpkeError> {
        return crypto::Aead::decrypt_in_place(self, in_out, nonce, associated_data, tag)
            .map_err(|_| HpkeError::OpenError);
    }
}

/// AES-256-GCM (AEAD id `0x0002`).
impl Aead for Aes256Gcm {
    const HPKE_AEAD_ID: u16 = 0x0002;
    const MAX_PLAINTEXT_SIZE: u64 = AES_GCM_MAX_PLAINTEXT_SIZE;
    const MAX_CIPHERTEXT_SIZE: u64 = AES_GCM_MAX_CIPHERTEXT_SIZE;

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        let key: &[u8; 32] = key.try_into().map_err(|_| HpkeError::InvalidKey)?;
        return Ok(Aes256Gcm::new(key));
    }
}

/// ChaCha20-Poly1305 (AEAD id `0x0003`).
impl Aead for ChaCha20Poly1305 {
    const HPKE_AEAD_ID: u16 = 0x0003;
    const MAX_PLAINTEXT_SIZE: u64 = CHACHA20_POLY1305_MAX_PLAINTEXT_SIZE;
    const MAX_CIPHERTEXT_SIZE: u64 = CHACHA20_POLY1305_MAX_CIPHERTEXT_SIZE;

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        let key: &[u8; 32] = key.try_into().map_err(|_| HpkeError::InvalidKey)?;
        return Ok(ChaCha20Poly1305::new(key));
    }
}

/// AEGIS-256 (unofficial AEAD id `0xFF01`).
impl Aead for Aegis256 {
    const HPKE_AEAD_ID: u16 = 0xFF01;
    const MAX_PLAINTEXT_SIZE: u64 = AEGIS_MAX_PLAINTEXT_SIZE;
    const MAX_CIPHERTEXT_SIZE: u64 = AEGIS_MAX_CIPHERTEXT_SIZE;

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        let key: &[u8; 32] = key.try_into().map_err(|_| HpkeError::InvalidKey)?;
        return Ok(Aegis256::new(key));
    }
}

/// AEGIS-256X2 (unofficial AEAD id `0xFF02`).
impl Aead for Aegis256X2 {
    const HPKE_AEAD_ID: u16 = 0xFF02;
    const MAX_PLAINTEXT_SIZE: u64 = AEGIS_MAX_PLAINTEXT_SIZE;
    const MAX_CIPHERTEXT_SIZE: u64 = AEGIS_MAX_CIPHERTEXT_SIZE;

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        let key: &[u8; 32] = key.try_into().map_err(|_| HpkeError::InvalidKey)?;
        return Ok(Aegis256X2::new(key));
    }
}

/// AEGIS-256X4 (unofficial AEAD id `0xFF03`).
impl Aead for Aegis256X4 {
    const HPKE_AEAD_ID: u16 = 0xFF03;
    const MAX_PLAINTEXT_SIZE: u64 = AEGIS_MAX_PLAINTEXT_SIZE;
    const MAX_CIPHERTEXT_SIZE: u64 = AEGIS_MAX_CIPHERTEXT_SIZE;

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        let key: &[u8; 32] = key.try_into().map_err(|_| HpkeError::InvalidKey)?;
        return Ok(Aegis256X4::new(key));
    }
}

/// Export-Only pseudo-AEAD (AEAD id `0xffff`).
///
/// Ciphersuites using this "AEAD" can only derive secrets through
/// [`SenderContext::export`](crate::SenderContext::export) and
/// [`RecipientContext::export`](crate::RecipientContext::export); [`Aead::seal`]
/// and [`Aead::open`] return [`HpkeError::NotSupported`].
///
/// The [`crypto::Aead`] implementation below only exists to satisfy the
/// [`Aead`] supertrait bound and must not be called directly: HPKE never
/// invokes it, and it does not perform any encryption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExportOnly;

impl crypto::Aead for ExportOnly {
    const KEY_SIZE: usize = 0;
    const TAG_SIZE: usize = 0;
    const NONCE_SIZE: usize = 0;

    fn encrypt_in_place(&self, _in_out: &mut [u8], _nonce: &[u8], _associated_data: &[u8]) -> Hash {
        return Hash::from([0u8; Self::TAG_SIZE]);
    }

    fn decrypt_in_place(
        &self,
        _in_out: &mut [u8],
        _nonce: &[u8],
        _associated_data: &[u8],
        _tag: &[u8],
    ) -> Result<(), crypto::AeadError> {
        return Err(crypto::AeadError::Unsupported);
    }
}

impl Aead for ExportOnly {
    const HPKE_AEAD_ID: u16 = 0xffff;
    // The Export-Only pseudo-AEAD never encrypts or decrypts, so it has no
    // message length limits.
    const MAX_PLAINTEXT_SIZE: u64 = 0;
    const MAX_CIPHERTEXT_SIZE: u64 = 0;

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        if key.is_empty() {
            return Ok(ExportOnly);
        }
        return Err(HpkeError::InvalidKey);
    }

    fn seal(&self, _in_out: &mut [u8], _nonce: &[u8], _associated_data: &[u8]) -> Result<Hash, HpkeError> {
        return Err(HpkeError::NotSupported);
    }

    fn open(&self, _in_out: &mut [u8], _nonce: &[u8], _associated_data: &[u8], _tag: &[u8]) -> Result<(), HpkeError> {
        return Err(HpkeError::NotSupported);
    }
}
