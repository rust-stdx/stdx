//! HPKE key schedule, modes and encryption contexts
//! (`draft-ietf-hpke-hpke-05` Section 5).

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::marker::PhantomData;

use crypto::{Hash, MAX_HASH_OUTPUT_SIZE};

use super::{HpkeError, aead::Aead, kdf::Kdf, kem::Kem, wipe};

/// Maximum AEAD key size supported by the key schedule. The AEADs defined in
/// this document use at most 32-byte keys.
const MAX_AEAD_KEY_SIZE: usize = 64;

/// Maximum AEAD nonce size supported by the contexts. The AEADs defined in this
/// document use 12-byte nonces.
const MAX_AEAD_NONCE_SIZE: usize = 64;

/// HPKE mode values (`draft-ietf-hpke-hpke-05` Section 5.1).
///
/// The values `0x02` (`mode_auth`) and `0x03` (`mode_auth_psk`) were used by
/// RFC 9180 and are RESERVED in `draft-ietf-hpke-hpke-05`, which removed the
/// authenticated modes. They are deliberately not implemented here.
const MODE_BASE: u8 = 0x00;
const MODE_PRE_SHARED_KEY: u8 = 0x01;

/// The HPKE session mode, shared by the sender and recipient roles.
///
/// The mode selects whether the key schedule is bound to a pre-shared key
/// ([`Mode::PreSharedKey`]) or not ([`Mode::Base`]).
///
/// Pre-shared key bytes and their identifier are carried together, so the
/// inconsistent inputs rejected by the specification cannot be expressed.
///
/// `draft-ietf-hpke-hpke-05` removed the asymmetric authenticated modes of
/// RFC 9180 (`mode_auth` and `mode_auth_psk`). For sender *identity*
/// authentication, sign the `(encapped_key, ciphertext)` tuple (see the crate
/// documentation); a pre-shared key only proves possession of the key.
pub enum Mode<'a> {
    /// No sender authentication.
    Base,
    /// Pre-shared key mode: the shared key is mixed into the key schedule,
    /// proving possession of it. This does not authenticate a sender identity.
    PreSharedKey {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        pre_shared_key: &'a [u8],
        /// A public identifier for the pre-shared key.
        ///
        /// It's purpose is let a recipient pick which PSK to use out of several it holds, and
        /// to be bound into the key schedule so a wrong id yields a different key.
        ///
        /// As per the spec, `pre_shared_key_id`
        /// MUST not be empty in pre-shared key mode. We recommend the Nil UUID (`00000000-0000-0000-0000-000000000000`)
        /// if you don't have a relevant identifier for the pre-shared key.
        pre_shared_key_id: &'a [u8],
    },
}

impl<'a> Mode<'a> {
    /// Ensures that in [`Mode::PreSharedKey`] mode both `pre_shared_key` and `pre_shared_key_id`
    /// are not empty, as per the spec (`draft-ietf-hpke-hpke-05` Section 5.1 `VerifyPSKInputs`).
    ///
    /// Then decomposes the mode into the wire `mode` value, `pre_shared_key` and `pre_shared_key_id`.
    fn validate_and_get_parts(&self) -> Result<(u8, &'a [u8], &'a [u8]), HpkeError> {
        match self {
            Mode::Base => Ok((MODE_BASE, &[], &[])),
            Mode::PreSharedKey {
                pre_shared_key,
                pre_shared_key_id,
            } => {
                if pre_shared_key.is_empty() || pre_shared_key_id.is_empty() {
                    return Err(HpkeError::ValidationError);
                }
                Ok((MODE_PRE_SHARED_KEY, pre_shared_key, pre_shared_key_id))
            }
        }
    }
}

/// Sets up a sender context (`draft-ietf-hpke-hpke-05` Section 5.1).
///
/// The encapsulated key must be transmitted to the recipient alongside the
/// first message.
///
/// # Errors
///
/// - [`HpkeError::EncapError`] when the recipient's public key is invalid.
/// - [`HpkeError::ValidationError`] when the pre-shared key or its identifier
///   is empty in a pre-shared key mode.
/// - [`HpkeError::Random`] when the operating system's random number generator
///   is unavailable or fails.
#[cfg(feature = "random")]
pub fn new_sender<K: Kem, D: Kdf, A: Aead>(
    mode: &Mode<'_>,
    recipient_public_key: &K::PublicKey,
    info: &[u8],
) -> Result<(K::EncappedKey, SenderContext<A, D>), HpkeError> {
    const { assert!(K::SHARED_SECRET_SIZE <= MAX_HASH_OUTPUT_SIZE) };

    let (mode_id, pre_shared_key, pre_shared_key_id) = mode.validate_and_get_parts()?;

    let (mut shared_secret, encapped_key) = K::encap(recipient_public_key)?;

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

/// Sets up a recipient context (`draft-ietf-hpke-hpke-05` Section 5.1).
///
/// # Errors
///
/// - [`HpkeError::DecapError`] when the encapsulated key is invalid or the key
///   exchange fails.
/// - [`HpkeError::ValidationError`] when the pre-shared key or its identifier
///   is empty in a pre-shared key mode.
pub fn new_recipient<K: Kem, D: Kdf, A: Aead>(
    mode: &Mode<'_>,
    recipient_secret_key: &K::SecretKey,
    encapped_key: &K::EncappedKey,
    info: &[u8],
) -> Result<RecipientContext<A, D>, HpkeError> {
    const { assert!(K::SHARED_SECRET_SIZE <= MAX_HASH_OUTPUT_SIZE) };

    let (mode_id, pre_shared_key, pre_shared_key_id) = mode.validate_and_get_parts()?;

    let mut shared_secret = K::decap(encapped_key, recipient_secret_key)?;

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

/// Builds the HPKE `suite_id`: `"HPKE" || I2OSP(kem_id, 2) || I2OSP(kdf_id, 2)
/// || I2OSP(aead_id, 2)` (`draft-ietf-hpke-hpke-05` Section 5.1).
fn ciphersuite_id<K: Kem, D: Kdf, A: Aead>() -> [u8; 10] {
    let mut suite_id = [0u8; 10];
    suite_id[..4].copy_from_slice(b"HPKE");
    suite_id[4..6].copy_from_slice(&K::HPKE_KEM_ID.to_be_bytes());
    suite_id[6..8].copy_from_slice(&D::HPKE_KDF_ID.to_be_bytes());
    suite_id[8..10].copy_from_slice(&A::HPKE_AEAD_ID.to_be_bytes());
    return suite_id;
}

/// The result of the HPKE key schedule (`draft-ietf-hpke-hpke-05` Section 5.1).
struct KeySchedule<A: Aead> {
    aead: A,
    base_nonce: [u8; MAX_AEAD_NONCE_SIZE],
    exporter_secret: Hash,
}

/// `draft-ietf-hpke-hpke-05` Section 5.1 `CombineSecrets_TwoStage` (identical
/// to the RFC 9180 `KeySchedule`) for two-stage KDFs, or `CombineSecrets_OneStage`
/// for single-stage KDFs.
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
        // `0` is the Export-Only pseudo-AEAD, which has no nonce; every other
        // AEAD must have the nonce size the specification mandates.
        assert!(A::NONCE_SIZE == 0 || (A::NONCE_SIZE >= 8 && A::NONCE_SIZE <= MAX_AEAD_NONCE_SIZE));
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

/// `draft-ietf-hpke-hpke-05` Section 5.1 `CombineSecrets_OneStage` for
/// single-stage KDFs:
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
        // `0` is the Export-Only pseudo-AEAD, which has no nonce; every other
        // AEAD must have the nonce size the specification mandates.
        assert!(A::NONCE_SIZE == 0 || (A::NONCE_SIZE >= 8 && A::NONCE_SIZE <= MAX_AEAD_NONCE_SIZE));
        assert!(D::OUTPUT_SIZE <= MAX_HASH_OUTPUT_SIZE);
    }

    // `key_schedule` only dispatches here when `D::SINGLE_STAGE` is `true`, but
    // the branch is a runtime one, so this function is also monomorphized for
    // two-stage KDFs. Reject them at runtime instead of failing to compile.
    if !D::SINGLE_STAGE {
        return Err(HpkeError::NotSupported);
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
/// (`draft-ietf-hpke-hpke-05` Section 5.2).
fn compute_nonce<A: Aead>(base_nonce: &[u8; MAX_AEAD_NONCE_SIZE], sequence_number: u64) -> [u8; MAX_AEAD_NONCE_SIZE] {
    let mut nonce = *base_nonce;
    let sequence_number_bytes = sequence_number.to_be_bytes();
    let start = A::NONCE_SIZE - sequence_number_bytes.len();
    for (i, byte) in sequence_number_bytes.iter().enumerate() {
        nonce[start + i] ^= byte;
    }
    return nonce;
}

/// Shared state of an HPKE encryption context (`draft-ietf-hpke-hpke-05` Section 5.1).
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
        // The Export-Only pseudo-AEAD (`NONCE_SIZE`/`TAG_SIZE` of `0`) cannot
        // encrypt; report it as unsupported rather than failing to compile. The
        // `NONCE_SIZE < 8` bound also keeps `compute_nonce` from underflowing.
        if A::NONCE_SIZE < 8 || A::TAG_SIZE == 0 {
            return Err(HpkeError::NotSupported);
        }

        // `draft-ietf-hpke-hpke-05` Section 5.2: implementations MUST NOT
        // encrypt plaintexts larger than the AEAD's `P_MAX`.
        if in_out.len() as u64 > A::MAX_PLAINTEXT_SIZE {
            return Err(HpkeError::MessageLimitReached);
        }

        if self.sequence_number == u64::MAX {
            return Err(HpkeError::MessageLimitReached);
        }

        // Computing the nonce, sealing, and incrementing the sequence number is
        // a single `&mut self` operation, so it cannot be interleaved with
        // another `seal`/`open` on the same context. This satisfies the
        // atomicity requirement of `draft-ietf-hpke-hpke-05` Section 5.2.
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
        // See `seal_in_place`: the Export-Only pseudo-AEAD cannot open, and the
        // bound keeps `compute_nonce` from underflowing.
        if A::NONCE_SIZE < 8 || A::TAG_SIZE == 0 {
            return Err(HpkeError::NotSupported);
        }

        // `draft-ietf-hpke-hpke-05` Section 5.2: implementations MUST NOT open
        // ciphertexts larger than the AEAD's `C_MAX`. The ciphertext here is
        // the plaintext buffer plus the detached tag.
        if (in_out.len() as u64).saturating_add(tag.len() as u64) > A::MAX_CIPHERTEXT_SIZE {
            return Err(HpkeError::MessageLimitReached);
        }

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
    /// Returns [`HpkeError::MessageLimitReached`] when `in_out` is longer than
    /// the AEAD's maximum plaintext size (`P_MAX`) or the context's sequence
    /// number is exhausted, or [`HpkeError::NotSupported`] when the
    /// ciphersuite uses the [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    pub fn seal_in_place(&mut self, in_out: &mut [u8], associated_data: &[u8]) -> Result<Hash, HpkeError> {
        return self.0.seal_in_place(in_out, associated_data);
    }

    /// Encrypts `plaintext` and returns `ciphertext || tag`.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::MessageLimitReached`] when `plaintext` is longer
    /// than the AEAD's maximum plaintext size (`P_MAX`) or the context's
    /// sequence number is exhausted, or [`HpkeError::NotSupported`] when the
    /// ciphersuite uses the [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    #[cfg(feature = "alloc")]
    pub fn seal(&mut self, plaintext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, HpkeError> {
        return self.0.seal(plaintext, associated_data);
    }

    /// Derives `out.len()` bytes of exported key material, bound to
    /// `exporter_context` (`draft-ietf-hpke-hpke-05` Section 5.3).
    ///
    /// # Replay
    ///
    /// HPKE does not provide replay protection: replaying the encapsulated key
    /// `enc` produces an identical context and therefore identical exported
    /// secrets. Applications MUST NOT use an exported secret unless it is safe
    /// for the same value to be produced more than once, and MUST NOT derive an
    /// AEAD `(key, nonce)` pair from it (as the example in RFC 9180 Section 9.8
    /// did). When an exported secret feeds encryption, mix in fresh
    /// recipient-provided randomness, as in `draft-ietf-hpke-hpke-05`
    /// Section 9.8.
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
    /// [`HpkeError::MessageLimitReached`] when the ciphertext is longer than
    /// the AEAD's maximum (`C_MAX`) or the context's sequence number is
    /// exhausted, or [`HpkeError::NotSupported`] when the ciphersuite uses
    /// the [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    ///
    /// # Buffer contents on error
    ///
    /// When decryption succeeds but the sequence number is exhausted, the
    /// plaintext has already been written to `in_out` even though
    /// [`HpkeError::MessageLimitReached`] is returned (matching
    /// `draft-ietf-hpke-hpke-05` Section 5.2, which raises the error after
    /// `Open`). On [`HpkeError::OpenError`] the buffer is left untouched.
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
    /// ciphertext is longer than the AEAD's maximum (`C_MAX`) or the context's
    /// sequence number is exhausted, or [`HpkeError::NotSupported`] when the
    /// ciphersuite uses the
    /// [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    #[cfg(feature = "alloc")]
    pub fn open(&mut self, ciphertext: &[u8], associated_data: &[u8]) -> Result<Vec<u8>, HpkeError> {
        return self.0.open(ciphertext, associated_data);
    }

    /// Derives `out.len()` bytes of exported key material, bound to
    /// `exporter_context` (`draft-ietf-hpke-hpke-05` Section 5.3).
    ///
    /// # Replay
    ///
    /// HPKE does not provide replay protection: replaying the encapsulated key
    /// `enc` produces an identical context and therefore identical exported
    /// secrets. Applications MUST NOT use an exported secret unless it is safe
    /// for the same value to be produced more than once, and MUST NOT derive an
    /// AEAD `(key, nonce)` pair from it (as the example in RFC 9180 Section 9.8
    /// did). When an exported secret feeds encryption, mix in fresh
    /// recipient-provided randomness, as in `draft-ietf-hpke-hpke-05`
    /// Section 9.8.
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

/// Builds a sender context from an already-computed KEM `shared_secret`,
/// skipping encapsulation.
///
/// This exists so the test suite can forward-check the vector corpus: given the
/// `shared_secret` from a test vector, the resulting context must reproduce the
/// vector's ciphertexts and exported values. It is not part of the public API.
#[cfg(all(test, feature = "alloc", feature = "random"))]
pub(crate) fn new_sender_from_shared_secret<K: Kem, D: Kdf, A: Aead>(
    mode: &Mode<'_>,
    shared_secret: &Hash,
    info: &[u8],
) -> Result<SenderContext<A, D>, HpkeError> {
    const { assert!(K::SHARED_SECRET_SIZE <= MAX_HASH_OUTPUT_SIZE) };

    let (mode_id, pre_shared_key, pre_shared_key_id) = mode.validate_and_get_parts()?;

    let suite_id = ciphersuite_id::<K, D, A>();
    let scheduled = key_schedule::<D, A>(mode_id, suite_id, shared_secret, info, pre_shared_key, pre_shared_key_id)?;

    return Ok(SenderContext(Context {
        aead: scheduled.aead,
        suite_id,
        base_nonce: scheduled.base_nonce,
        sequence_number: 0,
        exporter_secret: scheduled.exporter_secret,
        _kdf: PhantomData,
    }));
}
