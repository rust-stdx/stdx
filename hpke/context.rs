//! HPKE key schedule, modes and encryption contexts (RFC 9180 Section 5).

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::marker::PhantomData;

use crypto::{Hash, MAX_HASH_OUTPUT_SIZE};

use super::{HpkeError, aead::Aead, kdf::Kdf, kem::Kem, wipe};

/// Maximum AEAD key size supported by the key schedule. The RFC 9180 AEADs use
/// at most 32-byte keys.
const MAX_AEAD_KEY_SIZE: usize = 64;

/// Maximum AEAD nonce size supported by the contexts. The RFC 9180 AEADs use
/// 12-byte nonces.
const MAX_AEAD_NONCE_SIZE: usize = 64;

/// HPKE mode values (RFC 9180 Section 5.1).
const MODE_BASE: u8 = 0x00;
const MODE_PRE_SHARED_KEY: u8 = 0x01;
const MODE_AUTHENTICATED: u8 = 0x02;
const MODE_AUTHENTICATED_PRE_SHARED_KEY: u8 = 0x03;

/// The sender's view of an HPKE session mode.
///
/// The mode determines how the sender authenticates itself: not at all
/// ([`SenderMode::Base`]), with a pre-shared key
/// ([`SenderMode::PreSharedKey`]), with a KEM secret key
/// ([`SenderMode::Authenticated`]), or both
/// ([`SenderMode::AuthenticatedPreSharedKey`]).
///
/// Pre-shared key bytes and their identifier are carried together, so the
/// inconsistent inputs rejected by RFC 9180 cannot be expressed.
pub enum SenderMode<'a, K: Kem> {
    /// No sender authentication.
    Base,
    /// Sender authentication through a pre-shared key.
    PreSharedKey {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        pre_shared_key: &'a [u8],
        /// A public identifier for the pre-shared key.
        pre_shared_key_id: &'a [u8],
    },
    /// Sender authentication through a KEM secret key.
    Authenticated {
        /// The sender's static secret key.
        sender_secret_key: &'a K::SecretKey,
    },
    /// Sender authentication through both a pre-shared key and a KEM secret
    /// key.
    AuthenticatedPreSharedKey {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        pre_shared_key: &'a [u8],
        /// A public identifier for the pre-shared key.
        pre_shared_key_id: &'a [u8],
        /// The sender's static secret key.
        sender_secret_key: &'a K::SecretKey,
    },
}

/// The recipient's view of an HPKE session mode.
///
/// See [`SenderMode`] for the semantics of each mode.
pub enum RecipientMode<'a, K: Kem> {
    /// No sender authentication.
    Base,
    /// Sender authentication through a pre-shared key.
    PreSharedKey {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        pre_shared_key: &'a [u8],
        /// A public identifier for the pre-shared key.
        pre_shared_key_id: &'a [u8],
    },
    /// Sender authentication through the sender's KEM public key.
    Authenticated {
        /// The sender's static public key.
        sender_public_key: &'a K::PublicKey,
    },
    /// Sender authentication through both a pre-shared key and the sender's
    /// KEM public key.
    AuthenticatedPreSharedKey {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        pre_shared_key: &'a [u8],
        /// A public identifier for the pre-shared key.
        pre_shared_key_id: &'a [u8],
        /// The sender's static public key.
        sender_public_key: &'a K::PublicKey,
    },
}

/// The mode identifier and pre-shared key parameters shared by [`SenderMode`]
/// and [`RecipientMode`].
trait PreSharedKeyMode<'a> {
    /// Returns `(mode, pre_shared_key, pre_shared_key_id)` (RFC 9180 Section 5.1).
    fn pre_shared_key_parameters(&self) -> (u8, &'a [u8], &'a [u8]);
}

impl<'a, K: Kem> PreSharedKeyMode<'a> for SenderMode<'a, K> {
    fn pre_shared_key_parameters(&self) -> (u8, &'a [u8], &'a [u8]) {
        match self {
            SenderMode::Base => (MODE_BASE, &[], &[]),
            SenderMode::Authenticated {
                ..
            } => (MODE_AUTHENTICATED, &[], &[]),
            SenderMode::PreSharedKey {
                pre_shared_key,
                pre_shared_key_id,
            } => (MODE_PRE_SHARED_KEY, pre_shared_key, pre_shared_key_id),
            SenderMode::AuthenticatedPreSharedKey {
                pre_shared_key,
                pre_shared_key_id,
                ..
            } => (MODE_AUTHENTICATED_PRE_SHARED_KEY, pre_shared_key, pre_shared_key_id),
        }
    }
}

impl<'a, K: Kem> PreSharedKeyMode<'a> for RecipientMode<'a, K> {
    fn pre_shared_key_parameters(&self) -> (u8, &'a [u8], &'a [u8]) {
        match self {
            RecipientMode::Base => (MODE_BASE, &[], &[]),
            RecipientMode::Authenticated {
                ..
            } => (MODE_AUTHENTICATED, &[], &[]),
            RecipientMode::PreSharedKey {
                pre_shared_key,
                pre_shared_key_id,
            } => (MODE_PRE_SHARED_KEY, pre_shared_key, pre_shared_key_id),
            RecipientMode::AuthenticatedPreSharedKey {
                pre_shared_key,
                pre_shared_key_id,
                ..
            } => (MODE_AUTHENTICATED_PRE_SHARED_KEY, pre_shared_key, pre_shared_key_id),
        }
    }
}

/// RFC 9180 Section 5.1 `VerifyPSKInputs`, restricted to the checks that the
/// mode types cannot enforce statically: the "pre-shared key inputs provided
/// together" and "no pre-shared key in Base/Authenticated modes" rules are
/// encoded by [`SenderMode`] and [`RecipientMode`].
fn verify_pre_shared_key_inputs(mode: u8, pre_shared_key: &[u8], pre_shared_key_id: &[u8]) -> Result<(), HpkeError> {
    if matches!(mode, MODE_PRE_SHARED_KEY | MODE_AUTHENTICATED_PRE_SHARED_KEY)
        && (pre_shared_key.is_empty() || pre_shared_key_id.is_empty())
    {
        return Err(HpkeError::ValidationError);
    }
    return Ok(());
}

/// Builds the HPKE `suite_id`: `"HPKE" || I2OSP(kem_id, 2) || I2OSP(kdf_id, 2)
/// || I2OSP(aead_id, 2)` (RFC 9180 Section 5.1).
fn ciphersuite_id<K: Kem, D: Kdf, A: Aead>() -> [u8; 10] {
    let mut suite_id = [0u8; 10];
    suite_id[..4].copy_from_slice(b"HPKE");
    suite_id[4..6].copy_from_slice(&K::HPKE_KEM_ID.to_be_bytes());
    suite_id[6..8].copy_from_slice(&D::HPKE_KDF_ID.to_be_bytes());
    suite_id[8..10].copy_from_slice(&A::HPKE_AEAD_ID.to_be_bytes());
    return suite_id;
}

/// The result of the HPKE key schedule (RFC 9180 Section 5.1).
struct KeySchedule<A: Aead> {
    aead: A,
    base_nonce: [u8; MAX_AEAD_NONCE_SIZE],
    exporter_secret: Hash,
}

/// RFC 9180 Section 5.1 `KeySchedule` for two-stage KDFs, or
/// `draft-ietf-hpke-pq-05` Section 5.1 `KeySchedule` for single-stage KDFs.
fn key_schedule<D: Kdf, A: Aead>(
    mode: u8,
    suite_id: [u8; 10],
    shared_secret: &Hash,
    info: &[u8],
    pre_shared_key: &[u8],
    pre_shared_key_id: &[u8],
) -> Result<KeySchedule<A>, HpkeError> {
    if D::SINGLE_STAGE {
        return single_stage_key_schedule::<D, A>(
            mode,
            suite_id,
            shared_secret,
            info,
            pre_shared_key,
            pre_shared_key_id,
        );
    }

    const {
        assert!(A::KEY_SIZE <= MAX_AEAD_KEY_SIZE);
        assert!(A::NONCE_SIZE >= 8 && A::NONCE_SIZE <= MAX_AEAD_NONCE_SIZE);
        assert!(D::OUTPUT_SIZE <= MAX_HASH_OUTPUT_SIZE);
    }

    let pre_shared_key_id_hash = D::labeled_extract(b"", &suite_id, b"psk_id_hash", pre_shared_key_id);
    let info_hash = D::labeled_extract(b"", &suite_id, b"info_hash", info);

    let mut key_schedule_context = [0u8; 1 + 2 * MAX_HASH_OUTPUT_SIZE];
    key_schedule_context[0] = mode;
    key_schedule_context[1..1 + D::OUTPUT_SIZE].copy_from_slice(&pre_shared_key_id_hash);
    key_schedule_context[1 + D::OUTPUT_SIZE..1 + 2 * D::OUTPUT_SIZE].copy_from_slice(&info_hash);
    let key_schedule_context = &key_schedule_context[..1 + 2 * D::OUTPUT_SIZE];

    let mut secret = D::labeled_extract(shared_secret, &suite_id, b"secret", pre_shared_key);

    let mut key = [0u8; MAX_AEAD_KEY_SIZE];
    D::labeled_expand(&mut key[..A::KEY_SIZE], &secret, &suite_id, b"key", key_schedule_context)?;
    let aead = A::new(&key[..A::KEY_SIZE])?;
    wipe(&mut key);

    let mut base_nonce = [0u8; MAX_AEAD_NONCE_SIZE];
    D::labeled_expand(
        &mut base_nonce[..A::NONCE_SIZE],
        &secret,
        &suite_id,
        b"base_nonce",
        key_schedule_context,
    )?;

    let mut exporter_secret_bytes = [0u8; MAX_HASH_OUTPUT_SIZE];
    D::labeled_expand(
        &mut exporter_secret_bytes[..D::OUTPUT_SIZE],
        &secret,
        &suite_id,
        b"exp",
        key_schedule_context,
    )?;
    let exporter_secret = Hash::try_from(&exporter_secret_bytes[..D::OUTPUT_SIZE]).expect("OUTPUT_SIZE fits in a Hash");
    wipe(&mut exporter_secret_bytes);
    wipe(secret.as_mut());

    return Ok(KeySchedule {
        aead,
        base_nonce,
        exporter_secret,
    });
}

/// `I2OSP(value, 2)`: the big-endian two-byte encoding of `len`.
fn length_prefix(len: usize) -> [u8; 2] {
    return (len as u16).to_be_bytes();
}

/// `draft-ietf-hpke-pq-05` Section 5.1 `KeySchedule` for single-stage KDFs:
/// `secret = LabeledDerive(concat(lengthPrefixed(pre_shared_key),
/// lengthPrefixed(shared_secret)), "secret", concat(mode,
/// lengthPrefixed(pre_shared_key_id), lengthPrefixed(info)), KEY_SIZE + NONCE_SIZE + OUTPUT_SIZE)`,
/// split into the AEAD key, the base nonce and the exporter secret.
fn single_stage_key_schedule<D: Kdf, A: Aead>(
    mode: u8,
    suite_id: [u8; 10],
    shared_secret: &Hash,
    info: &[u8],
    pre_shared_key: &[u8],
    pre_shared_key_id: &[u8],
) -> Result<KeySchedule<A>, HpkeError> {
    const {
        assert!(A::KEY_SIZE <= MAX_AEAD_KEY_SIZE);
        assert!(A::NONCE_SIZE >= 8 && A::NONCE_SIZE <= MAX_AEAD_NONCE_SIZE);
        assert!(D::OUTPUT_SIZE <= MAX_HASH_OUTPUT_SIZE);
    }

    // The single-stage `LabeledDerive` encodes each length in two bytes.
    if pre_shared_key.len() > u16::MAX as usize
        || shared_secret.len() > u16::MAX as usize
        || pre_shared_key_id.len() > u16::MAX as usize
        || info.len() > u16::MAX as usize
    {
        return Err(HpkeError::ValidationError);
    }

    let mode = [mode];
    let pre_shared_key_len = length_prefix(pre_shared_key.len());
    let shared_secret_len = length_prefix(shared_secret.len());
    let pre_shared_key_id_len = length_prefix(pre_shared_key_id.len());
    let info_len = length_prefix(info.len());

    let secret_len = A::KEY_SIZE + A::NONCE_SIZE + D::OUTPUT_SIZE;
    let mut secret_bytes = [0u8; MAX_AEAD_KEY_SIZE + MAX_AEAD_NONCE_SIZE + MAX_HASH_OUTPUT_SIZE];
    D::labeled_derive(
        &mut secret_bytes[..secret_len],
        &[
            &pre_shared_key_len,
            pre_shared_key,
            &shared_secret_len,
            shared_secret.as_ref(),
        ],
        &suite_id,
        b"secret",
        &[&mode, &pre_shared_key_id_len, pre_shared_key_id, &info_len, info],
    )?;

    let aead = A::new(&secret_bytes[..A::KEY_SIZE])?;

    let mut base_nonce = [0u8; MAX_AEAD_NONCE_SIZE];
    base_nonce[..A::NONCE_SIZE].copy_from_slice(&secret_bytes[A::KEY_SIZE..A::KEY_SIZE + A::NONCE_SIZE]);

    let exporter_secret =
        Hash::try_from(&secret_bytes[A::KEY_SIZE + A::NONCE_SIZE..secret_len]).expect("OUTPUT_SIZE fits in a Hash");
    wipe(&mut secret_bytes);

    return Ok(KeySchedule {
        aead,
        base_nonce,
        exporter_secret,
    });
}

/// Computes the per-message nonce `base_nonce XOR I2OSP(sequence_number, NONCE_SIZE)`
/// (RFC 9180 Section 5.2).
fn compute_nonce<A: Aead>(base_nonce: &[u8; MAX_AEAD_NONCE_SIZE], sequence_number: u64) -> [u8; MAX_AEAD_NONCE_SIZE] {
    let mut nonce = *base_nonce;
    let sequence_number_bytes = sequence_number.to_be_bytes();
    let start = A::NONCE_SIZE - sequence_number_bytes.len();
    for (i, byte) in sequence_number_bytes.iter().enumerate() {
        nonce[start + i] ^= byte;
    }
    return nonce;
}

/// Shared state of an HPKE encryption context (RFC 9180 Section 5.1).
///
/// This is the common implementation behind [`SenderContext`] and
/// [`RecipientContext`]; the two wrappers restrict the exposed operations to
/// their role.
#[cfg_attr(feature = "zeroize", derive(zeroize::ZeroizeOnDrop))]
struct Context<A: Aead, D: Kdf> {
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    aead: A,
    suite_id: [u8; 10],
    base_nonce: [u8; MAX_AEAD_NONCE_SIZE],
    sequence_number: u64,
    exporter_secret: Hash,
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    _kdf: PhantomData<D>,
}

impl<A: Aead, D: Kdf> Context<A, D> {
    fn seal_in_place(&mut self, in_out: &mut [u8], associated_data: &[u8]) -> Result<Hash, HpkeError> {
        if self.sequence_number == u64::MAX {
            return Err(HpkeError::MessageLimitReached);
        }

        let nonce = compute_nonce::<A>(&self.base_nonce, self.sequence_number);
        let tag = self.aead.seal(in_out, &nonce[..A::NONCE_SIZE], associated_data)?;
        self.sequence_number += 1;
        return Ok(tag);
    }

    #[cfg(feature = "alloc")]
    fn seal(&mut self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, HpkeError> {
        let mut ciphertext = Vec::with_capacity(plaintext.len() + A::TAG_SIZE);
        ciphertext.extend_from_slice(plaintext);

        let tag = self.seal_in_place(&mut ciphertext, associated_data)?;
        ciphertext.extend_from_slice(tag.as_ref());
        return Ok(ciphertext);
    }

    fn open_in_place(&mut self, in_out: &mut [u8], associated_data: &[u8], tag: &[u8]) -> Result<(), HpkeError> {
        let nonce = compute_nonce::<A>(&self.base_nonce, self.sequence_number);
        self.aead.open(in_out, &nonce[..A::NONCE_SIZE], associated_data, tag)?;

        if self.sequence_number == u64::MAX {
            return Err(HpkeError::MessageLimitReached);
        }
        self.sequence_number += 1;
        return Ok(());
    }

    #[cfg(feature = "alloc")]
    fn open(&mut self, ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, HpkeError> {
        if ciphertext.len() < A::TAG_SIZE {
            return Err(HpkeError::OpenError);
        }

        let plaintext_len = ciphertext.len() - A::TAG_SIZE;
        let mut plaintext = Vec::with_capacity(plaintext_len);
        plaintext.extend_from_slice(&ciphertext[..plaintext_len]);

        self.open_in_place(&mut plaintext, associated_data, &ciphertext[plaintext_len..])?;
        return Ok(plaintext);
    }

    fn export(&self, out: &mut [u8], exporter_context: &[u8]) -> Result<(), HpkeError> {
        if D::SINGLE_STAGE {
            return D::labeled_derive(
                out,
                &[self.exporter_secret.as_ref()],
                &self.suite_id,
                b"sec",
                &[exporter_context],
            );
        }
        return D::labeled_expand(out, &self.exporter_secret, &self.suite_id, b"sec", exporter_context);
    }
}

/// The sender's encryption context, returned by [`new_sender`].
///
/// A context tracks a monotonically increasing sequence number; messages MUST
/// be sealed in order, and the matching [`RecipientContext`] MUST open them in
/// the same order. Sealing more than `2^64 - 1` messages with a single context
/// is not possible.
pub struct SenderContext<A: Aead, D: Kdf>(Context<A, D>);

impl<A: Aead, D: Kdf> SenderContext<A, D> {
    /// Encrypts `in_out` in place and returns the detached authentication tag.
    ///
    /// The ciphertext is `in_out` itself; combine it with the returned tag as
    /// `ciphertext || tag` for transmission.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::MessageLimitReached`] when the context's sequence
    /// number is exhausted, or [`HpkeError::NotSupported`] when the
    /// ciphersuite uses the [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    pub fn seal_in_place(&mut self, in_out: &mut [u8], associated_data: &[u8]) -> Result<Hash, HpkeError> {
        return self.0.seal_in_place(in_out, associated_data);
    }

    /// Encrypts `plaintext` and returns `ciphertext || tag`.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::MessageLimitReached`] when the context's sequence
    /// number is exhausted, or [`HpkeError::NotSupported`] when the
    /// ciphersuite uses the [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    #[cfg(feature = "alloc")]
    pub fn seal(&mut self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, HpkeError> {
        return self.0.seal(plaintext, associated_data);
    }

    /// Derives `out.len()` bytes of exported key material, bound to
    /// `exporter_context` (RFC 9180 Section 5.3).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::KdfOutputTooLong`] when `out` is longer than the
    /// KDF can produce (`255 * OUTPUT_SIZE` for two-stage KDFs, `2^16 - 1` bytes for
    /// single-stage KDFs).
    pub fn export(&self, out: &mut [u8], exporter_context: &[u8]) -> Result<(), HpkeError> {
        return self.0.export(out, exporter_context);
    }
}

/// The recipient's decryption context, returned by [`new_recipient`].
///
/// See [`SenderContext`] for the ordering and message-limit requirements.
pub struct RecipientContext<A: Aead, D: Kdf>(Context<A, D>);

impl<A: Aead, D: Kdf> RecipientContext<A, D> {
    /// Decrypts `in_out` in place using the detached authentication `tag`.
    ///
    /// The sequence number is only advanced when decryption succeeds.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::OpenError`] when the ciphertext is invalid (wrong
    /// key, tampered data or wrong associated data),
    /// [`HpkeError::MessageLimitReached`] when the context's sequence number
    /// is exhausted, or [`HpkeError::NotSupported`] when the ciphersuite uses
    /// the [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    ///
    /// # Buffer contents on error
    ///
    /// When decryption succeeds but the sequence number is exhausted, the
    /// plaintext has already been written to `in_out` even though
    /// [`HpkeError::MessageLimitReached`] is returned (matching RFC 9180
    /// Section 5.2, which raises the error after `Open`). On
    /// [`HpkeError::OpenError`] the buffer is left untouched.
    pub fn open_in_place(&mut self, in_out: &mut [u8], associated_data: &[u8], tag: &[u8]) -> Result<(), HpkeError> {
        return self.0.open_in_place(in_out, associated_data, tag);
    }

    /// Decrypts `ciphertext` (which must end with the authentication tag) and
    /// returns the plaintext.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::OpenError`] when the ciphertext is invalid or too
    /// short to contain a tag, [`HpkeError::MessageLimitReached`] when the
    /// context's sequence number is exhausted, or [`HpkeError::NotSupported`]
    /// when the ciphersuite uses the
    /// [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    #[cfg(feature = "alloc")]
    pub fn open(&mut self, ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, HpkeError> {
        return self.0.open(ciphertext, associated_data);
    }

    /// Derives `out.len()` bytes of exported key material, bound to
    /// `exporter_context` (RFC 9180 Section 5.3).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::KdfOutputTooLong`] when `out` is longer than the
    /// KDF can produce (`255 * OUTPUT_SIZE` for two-stage KDFs, `2^16 - 1` bytes for
    /// single-stage KDFs).
    pub fn export(&self, out: &mut [u8], exporter_context: &[u8]) -> Result<(), HpkeError> {
        return self.0.export(out, exporter_context);
    }
}

/// Sets up a sender context and, for authenticated modes, proves possession of
/// the sender's static key (RFC 9180 Section 5.1).
///
/// The encapsulated key must be transmitted to the recipient alongside the
/// first message.
///
/// # Errors
///
/// - [`HpkeError::EncapError`] when the recipient's public key is invalid.
/// - [`HpkeError::NotSupported`] when an authenticated mode is used with a KEM
///   that does not support it (e.g. [`MLKEM768X25519`](super::kem::MLKEM768X25519)).
/// - [`HpkeError::ValidationError`] when the pre-shared key or its identifier
///   is empty in a pre-shared key mode.
/// - [`HpkeError::Random`] when the operating system's random number generator
///   is unavailable or fails.
#[cfg(feature = "random")]
pub fn new_sender<K: Kem, D: Kdf, A: Aead>(
    mode: &SenderMode<'_, K>,
    recipient_public_key: &K::PublicKey,
    info: &[u8],
) -> Result<(K::EncappedKey, SenderContext<A, D>), HpkeError> {
    const { assert!(K::SHARED_SECRET_SIZE <= MAX_HASH_OUTPUT_SIZE) };

    let (mode_id, pre_shared_key, pre_shared_key_id) = mode.pre_shared_key_parameters();
    verify_pre_shared_key_inputs(mode_id, pre_shared_key, pre_shared_key_id)?;

    let (mut shared_secret, encapped_key) = match mode {
        SenderMode::Base
        | SenderMode::PreSharedKey {
            ..
        } => K::encap(recipient_public_key)?,
        SenderMode::Authenticated {
            sender_secret_key,
        }
        | SenderMode::AuthenticatedPreSharedKey {
            sender_secret_key, ..
        } => K::authenticated_encap(recipient_public_key, sender_secret_key)?,
    };

    let suite_id = ciphersuite_id::<K, D, A>();
    let scheduled = key_schedule::<D, A>(mode_id, suite_id, &shared_secret, info, pre_shared_key, pre_shared_key_id)?;
    wipe(shared_secret.as_mut());

    let context = SenderContext(Context {
        aead: scheduled.aead,
        suite_id,
        base_nonce: scheduled.base_nonce,
        sequence_number: 0,
        exporter_secret: scheduled.exporter_secret,
        _kdf: PhantomData,
    });
    return Ok((encapped_key, context));
}

/// Sets up a recipient context (RFC 9180 Section 5.1).
///
/// # Errors
///
/// - [`HpkeError::DecapError`] when the encapsulated key is invalid or the key
///   exchange fails. Note that in authenticated modes a wrong sender public
///   key does **not** fail here; it produces a different shared secret and the
///   error surfaces as [`HpkeError::OpenError`] on the first message.
/// - [`HpkeError::NotSupported`] when an authenticated mode is used with a KEM
///   that does not support it (e.g. [`MLKEM768X25519`](super::kem::MLKEM768X25519)).
/// - [`HpkeError::ValidationError`] when the pre-shared key or its identifier
///   is empty in a pre-shared key mode.
pub fn new_recipient<K: Kem, D: Kdf, A: Aead>(
    mode: &RecipientMode<'_, K>,
    recipient_secret_key: &K::SecretKey,
    encapped_key: &K::EncappedKey,
    info: &[u8],
) -> Result<RecipientContext<A, D>, HpkeError> {
    const { assert!(K::SHARED_SECRET_SIZE <= MAX_HASH_OUTPUT_SIZE) };

    let (mode_id, pre_shared_key, pre_shared_key_id) = mode.pre_shared_key_parameters();
    verify_pre_shared_key_inputs(mode_id, pre_shared_key, pre_shared_key_id)?;

    let mut shared_secret = match mode {
        RecipientMode::Base
        | RecipientMode::PreSharedKey {
            ..
        } => K::decap(encapped_key, recipient_secret_key)?,
        RecipientMode::Authenticated {
            sender_public_key,
        }
        | RecipientMode::AuthenticatedPreSharedKey {
            sender_public_key, ..
        } => K::authenticated_decap(encapped_key, recipient_secret_key, sender_public_key)?,
    };

    let suite_id = ciphersuite_id::<K, D, A>();
    let scheduled = key_schedule::<D, A>(mode_id, suite_id, &shared_secret, info, pre_shared_key, pre_shared_key_id)?;
    wipe(shared_secret.as_mut());

    let context = RecipientContext(Context {
        aead: scheduled.aead,
        suite_id,
        base_nonce: scheduled.base_nonce,
        sequence_number: 0,
        exporter_secret: scheduled.exporter_secret,
        _kdf: PhantomData,
    });
    return Ok(context);
}
