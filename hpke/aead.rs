//! AEAD algorithms for HPKE ciphersuites (RFC 9180 Section 5.2 and Section 7.3).

use crypto::{Hash, aes::Aes256Gcm, chacha::ChaCha20Poly1305};

use super::HpkeError;

/// HPKE ciphersuite binding for an AEAD algorithm.
///
/// An AEAD is the symmetric cipher used to seal and open messages. This trait
/// only adds what HPKE itself needs on top of [`crypto::Aead`] (from which the
/// nonce size, tag size, and the underlying cipher operations are inherited):
///
/// - [`Aead::HPKE_AEAD_ID`], the IANA "HPKE AEAD Identifiers" value used to build
///   the ciphersuite `suite_id`,
/// - [`Aead::new`], the construction of a ready-to-use cipher from the
///   raw key schedule output.
///
/// # Example
///
/// ```
/// use crypto::aes::Aes256Gcm;
/// use hpke::aead::{Aead, ExportOnly};
///
/// let cipher = <Aes256Gcm as Aead>::new(&[0x42u8; 32]).unwrap();
/// assert_eq!(Aes256Gcm::HPKE_AEAD_ID, 0x0002);
/// assert_eq!(ExportOnly::HPKE_AEAD_ID, 0xffff);
/// # let _ = cipher;
/// ```
///
/// # Custom AEADs
///
/// Implement [`crypto::Aead`] for the cipher's operations, then implement this
/// trait to give it an HPKE identifier and key size. Any nonce and tag size is
/// supported as long as `KEY_SIZE` is at most 64 bytes and `NONCE_SIZE` is
/// between 8 and 64 bytes; larger values fail to compile when the cipher is
/// used with [`new_sender`](crate::new_sender) or
/// [`new_recipient`](crate::new_recipient).
pub trait Aead: Sized + crypto::Aead {
    /// The HPKE AEAD identifier (RFC 9180 Section 7.3, IANA "HPKE AEAD
    /// Identifiers"), used to build the ciphersuite `suite_id`.
    const HPKE_AEAD_ID: u16;

    /// Instantiates the cipher from `KEY_SIZE` raw key bytes, as produced by
    /// the HPKE key schedule.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::DeserializeError`] when `key` does not have
    /// exactly [`crypto::Aead::KEY_SIZE`] bytes.
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

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        let key: &[u8; 32] = key.try_into().map_err(|_| HpkeError::InvalidKey)?;
        return Ok(Aes256Gcm::new(key));
    }
}

/// ChaCha20-Poly1305 (AEAD id `0x0003`).
impl Aead for ChaCha20Poly1305 {
    const HPKE_AEAD_ID: u16 = 0x0003;

    fn new(key: &[u8]) -> Result<Self, HpkeError> {
        let key: &[u8; 32] = key.try_into().map_err(|_| HpkeError::InvalidKey)?;
        return Ok(ChaCha20Poly1305::new(key));
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
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

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
