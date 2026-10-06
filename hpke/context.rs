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
const MODE_PSK: u8 = 0x01;
const MODE_AUTH: u8 = 0x02;
const MODE_AUTH_PSK: u8 = 0x03;

/// The sender's view of an HPKE session mode.
///
/// The mode determines how the sender authenticates itself: not at all
/// ([`ModeSender::Base`]), with a pre-shared key ([`ModeSender::Psk`]), with a
/// KEM secret key ([`ModeSender::Auth`]), or both ([`ModeSender::AuthPsk`]).
///
/// PSK bytes and their identifier are carried together, so the inconsistent
/// PSK inputs rejected by RFC 9180 cannot be expressed.
pub enum ModeSender<'a, K: Kem> {
    /// No sender authentication.
    Base,
    /// Sender authentication through a pre-shared key.
    Psk {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        psk: &'a [u8],
        /// A public identifier for the pre-shared key.
        psk_id: &'a [u8],
    },
    /// Sender authentication through a KEM secret key.
    Auth {
        /// The sender's static secret key.
        sk_s: &'a K::SecretKey,
    },
    /// Sender authentication through both a pre-shared key and a KEM secret
    /// key.
    AuthPsk {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        psk: &'a [u8],
        /// A public identifier for the pre-shared key.
        psk_id: &'a [u8],
        /// The sender's static secret key.
        sk_s: &'a K::SecretKey,
    },
}

/// The receiver's view of an HPKE session mode.
///
/// See [`ModeSender`] for the semantics of each mode.
pub enum ModeReceiver<'a, K: Kem> {
    /// No sender authentication.
    Base,
    /// Sender authentication through a pre-shared key.
    Psk {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        psk: &'a [u8],
        /// A public identifier for the pre-shared key.
        psk_id: &'a [u8],
    },
    /// Sender authentication through the sender's KEM public key.
    Auth {
        /// The sender's static public key.
        pk_s: &'a K::PublicKey,
    },
    /// Sender authentication through both a pre-shared key and the sender's
    /// KEM public key.
    AuthPsk {
        /// The pre-shared key. MUST contain at least 32 bytes of entropy.
        psk: &'a [u8],
        /// A public identifier for the pre-shared key.
        psk_id: &'a [u8],
        /// The sender's static public key.
        pk_s: &'a K::PublicKey,
    },
}

/// The mode identifier and PSK parameters shared by [`ModeSender`] and
/// [`ModeReceiver`].
trait PskMode<'a> {
    /// Returns `(mode, psk, psk_id)` (RFC 9180 Section 5.1).
    fn psk_parameters(&self) -> (u8, &'a [u8], &'a [u8]);
}

impl<'a, K: Kem> PskMode<'a> for ModeSender<'a, K> {
    fn psk_parameters(&self) -> (u8, &'a [u8], &'a [u8]) {
        match self {
            ModeSender::Base => (MODE_BASE, &[], &[]),
            ModeSender::Auth {
                ..
            } => (MODE_AUTH, &[], &[]),
            ModeSender::Psk {
                psk,
                psk_id,
            } => (MODE_PSK, psk, psk_id),
            ModeSender::AuthPsk {
                psk,
                psk_id,
                ..
            } => (MODE_AUTH_PSK, psk, psk_id),
        }
    }
}

impl<'a, K: Kem> PskMode<'a> for ModeReceiver<'a, K> {
    fn psk_parameters(&self) -> (u8, &'a [u8], &'a [u8]) {
        match self {
            ModeReceiver::Base => (MODE_BASE, &[], &[]),
            ModeReceiver::Auth {
                ..
            } => (MODE_AUTH, &[], &[]),
            ModeReceiver::Psk {
                psk,
                psk_id,
            } => (MODE_PSK, psk, psk_id),
            ModeReceiver::AuthPsk {
                psk,
                psk_id,
                ..
            } => (MODE_AUTH_PSK, psk, psk_id),
        }
    }
}

/// RFC 9180 Section 5.1 `VerifyPSKInputs`, restricted to the checks that the
/// mode types cannot enforce statically: the "PSK inputs provided together"
/// and "no PSK in Base/Auth modes" rules are encoded by [`ModeSender`] and
/// [`ModeReceiver`].
fn verify_psk_inputs(mode: u8, psk: &[u8], psk_id: &[u8]) -> Result<(), HpkeError> {
    if matches!(mode, MODE_PSK | MODE_AUTH_PSK) && (psk.is_empty() || psk_id.is_empty()) {
        return Err(HpkeError::ValidationError);
    }
    return Ok(());
}

/// Builds the HPKE `suite_id`: `"HPKE" || I2OSP(kem_id, 2) || I2OSP(kdf_id, 2)
/// || I2OSP(aead_id, 2)` (RFC 9180 Section 5.1).
fn ciphersuite_id<K: Kem, D: Kdf, A: Aead>() -> [u8; 10] {
    let mut suite_id = [0u8; 10];
    suite_id[..4].copy_from_slice(b"HPKE");
    suite_id[4..6].copy_from_slice(&K::ID.to_be_bytes());
    suite_id[6..8].copy_from_slice(&D::ID.to_be_bytes());
    suite_id[8..10].copy_from_slice(&A::HPKE_AEAD_ID.to_be_bytes());
    return suite_id;
}

/// The result of the HPKE key schedule (RFC 9180 Section 5.1).
struct KeySchedule<A: Aead> {
    aead: A,
    base_nonce: [u8; MAX_AEAD_NONCE_SIZE],
    exporter_secret: Hash,
}

/// RFC 9180 Section 5.1 `KeySchedule`.
fn key_schedule<D: Kdf, A: Aead>(
    mode: u8,
    suite_id: [u8; 10],
    shared_secret: &Hash,
    info: &[u8],
    psk: &[u8],
    psk_id: &[u8],
) -> Result<KeySchedule<A>, HpkeError> {
    const {
        assert!(A::KEY_SIZE <= MAX_AEAD_KEY_SIZE);
        assert!(A::NONCE_SIZE >= 8 && A::NONCE_SIZE <= MAX_AEAD_NONCE_SIZE);
        assert!(D::NH <= MAX_HASH_OUTPUT_SIZE);
    }

    let psk_id_hash = D::labeled_extract(b"", &suite_id, b"psk_id_hash", psk_id);
    let info_hash = D::labeled_extract(b"", &suite_id, b"info_hash", info);

    let mut key_schedule_context = [0u8; 1 + 2 * MAX_HASH_OUTPUT_SIZE];
    key_schedule_context[0] = mode;
    key_schedule_context[1..1 + D::NH].copy_from_slice(&psk_id_hash);
    key_schedule_context[1 + D::NH..1 + 2 * D::NH].copy_from_slice(&info_hash);
    let key_schedule_context = &key_schedule_context[..1 + 2 * D::NH];

    let mut secret = D::labeled_extract(shared_secret, &suite_id, b"secret", psk);

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
        &mut exporter_secret_bytes[..D::NH],
        &secret,
        &suite_id,
        b"exp",
        key_schedule_context,
    )?;
    let exporter_secret = Hash::try_from(&exporter_secret_bytes[..D::NH]).expect("Nh fits in a Hash");
    wipe(&mut exporter_secret_bytes);
    wipe(secret.as_mut());

    return Ok(KeySchedule {
        aead,
        base_nonce,
        exporter_secret,
    });
}

/// Computes the per-message nonce `base_nonce XOR I2OSP(seq, Nn)` (RFC 9180
/// Section 5.2).
fn compute_nonce<A: Aead>(base_nonce: &[u8; MAX_AEAD_NONCE_SIZE], seq: u64) -> [u8; MAX_AEAD_NONCE_SIZE] {
    let mut nonce = *base_nonce;
    let seq_bytes = seq.to_be_bytes();
    let start = A::NONCE_SIZE - seq_bytes.len();
    for (i, byte) in seq_bytes.iter().enumerate() {
        nonce[start + i] ^= byte;
    }
    return nonce;
}

/// Shared state of an HPKE encryption context (RFC 9180 Section 5.1).
///
/// This is the common implementation behind [`SenderContext`] and
/// [`ReceiverContext`]; the two wrappers restrict the exposed operations to
/// their role.
#[cfg_attr(feature = "zeroize", derive(zeroize::ZeroizeOnDrop))]
struct Context<A: Aead, D: Kdf> {
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    aead: A,
    suite_id: [u8; 10],
    base_nonce: [u8; MAX_AEAD_NONCE_SIZE],
    seq: u64,
    exporter_secret: Hash,
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    _kdf: PhantomData<D>,
}

impl<A: Aead, D: Kdf> Context<A, D> {
    fn seal_in_place(&mut self, in_out: &mut [u8], aad: &[u8]) -> Result<Hash, HpkeError> {
        if self.seq == u64::MAX {
            return Err(HpkeError::MessageLimitReached);
        }

        let nonce = compute_nonce::<A>(&self.base_nonce, self.seq);
        let tag = self.aead.seal(in_out, &nonce[..A::NONCE_SIZE], aad)?;
        self.seq += 1;
        return Ok(tag);
    }

    #[cfg(feature = "alloc")]
    fn seal(&mut self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, HpkeError> {
        let mut ciphertext = Vec::with_capacity(plaintext.len() + A::TAG_SIZE);
        ciphertext.extend_from_slice(plaintext);

        let tag = self.seal_in_place(&mut ciphertext, aad)?;
        ciphertext.extend_from_slice(tag.as_ref());
        return Ok(ciphertext);
    }

    fn open_in_place(&mut self, in_out: &mut [u8], aad: &[u8], tag: &[u8]) -> Result<(), HpkeError> {
        let nonce = compute_nonce::<A>(&self.base_nonce, self.seq);
        self.aead.open(in_out, &nonce[..A::NONCE_SIZE], aad, tag)?;

        if self.seq == u64::MAX {
            return Err(HpkeError::MessageLimitReached);
        }
        self.seq += 1;
        return Ok(());
    }

    #[cfg(feature = "alloc")]
    fn open(&mut self, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>, HpkeError> {
        if ciphertext.len() < A::TAG_SIZE {
            return Err(HpkeError::OpenError);
        }

        let plaintext_len = ciphertext.len() - A::TAG_SIZE;
        let mut plaintext = Vec::with_capacity(plaintext_len);
        plaintext.extend_from_slice(&ciphertext[..plaintext_len]);

        self.open_in_place(&mut plaintext, aad, &ciphertext[plaintext_len..])?;
        return Ok(plaintext);
    }

    fn export(&self, out: &mut [u8], exporter_context: &[u8]) -> Result<(), HpkeError> {
        return D::labeled_expand(out, &self.exporter_secret, &self.suite_id, b"sec", exporter_context);
    }
}

/// The sender's encryption context, returned by [`new_sender`].
///
/// A context tracks a monotonically increasing sequence number; messages MUST
/// be sealed in order, and the matching [`ReceiverContext`] MUST open them in
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
    pub fn seal_in_place(&mut self, in_out: &mut [u8], aad: &[u8]) -> Result<Hash, HpkeError> {
        return self.0.seal_in_place(in_out, aad);
    }

    /// Encrypts `plaintext` and returns `ciphertext || tag`.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::MessageLimitReached`] when the context's sequence
    /// number is exhausted, or [`HpkeError::NotSupported`] when the
    /// ciphersuite uses the [`ExportOnly`](super::aead::ExportOnly) pseudo-AEAD.
    #[cfg(feature = "alloc")]
    pub fn seal(&mut self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, HpkeError> {
        return self.0.seal(plaintext, aad);
    }

    /// Derives `out.len()` bytes of exported key material, bound to
    /// `exporter_context` (RFC 9180 Section 5.3).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::KdfOutputTooLong`] when `out` is longer than
    /// `255 * Nh` bytes.
    pub fn export(&self, out: &mut [u8], exporter_context: &[u8]) -> Result<(), HpkeError> {
        return self.0.export(out, exporter_context);
    }
}

/// The receiver's decryption context, returned by [`new_receiver`].
///
/// See [`SenderContext`] for the ordering and message-limit requirements.
pub struct ReceiverContext<A: Aead, D: Kdf>(Context<A, D>);

impl<A: Aead, D: Kdf> ReceiverContext<A, D> {
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
    pub fn open_in_place(&mut self, in_out: &mut [u8], aad: &[u8], tag: &[u8]) -> Result<(), HpkeError> {
        return self.0.open_in_place(in_out, aad, tag);
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
    pub fn open(&mut self, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>, HpkeError> {
        return self.0.open(ciphertext, aad);
    }

    /// Derives `out.len()` bytes of exported key material, bound to
    /// `exporter_context` (RFC 9180 Section 5.3).
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::KdfOutputTooLong`] when `out` is longer than
    /// `255 * Nh` bytes.
    pub fn export(&self, out: &mut [u8], exporter_context: &[u8]) -> Result<(), HpkeError> {
        return self.0.export(out, exporter_context);
    }
}

/// Sets up a sender context and, for authenticated modes, proves possession of
/// the sender's static key (RFC 9180 Section 5.1).
///
/// The encapsulated key `enc` must be transmitted to the receiver alongside
/// the first message.
///
/// # Errors
///
/// - [`HpkeError::EncapError`] when the recipient's public key is invalid.
/// - [`HpkeError::NotSupported`] when an authenticated mode is used with a KEM
///   that does not support it (e.g. [`XWing`](super::kem::XWing)).
/// - [`HpkeError::ValidationError`] when the PSK or its identifier is empty in
///   a PSK mode.
/// - [`HpkeError::Random`] when the operating system's random number generator
///   is unavailable or fails.
#[cfg(feature = "random")]
pub fn new_sender<K: Kem, D: Kdf, A: Aead>(
    mode: &ModeSender<'_, K>,
    pk_recipient: &K::PublicKey,
    info: &[u8],
) -> Result<(K::EncappedKey, SenderContext<A, D>), HpkeError> {
    const { assert!(K::NSECRET <= MAX_HASH_OUTPUT_SIZE) };

    let (mode_id, psk, psk_id) = mode.psk_parameters();
    verify_psk_inputs(mode_id, psk, psk_id)?;

    let (mut shared_secret, enc) = match mode {
        ModeSender::Base
        | ModeSender::Psk {
            ..
        } => K::encap(pk_recipient)?,
        ModeSender::Auth {
            sk_s,
        }
        | ModeSender::AuthPsk {
            sk_s, ..
        } => K::auth_encap(pk_recipient, sk_s)?,
    };

    let suite_id = ciphersuite_id::<K, D, A>();
    let scheduled = key_schedule::<D, A>(mode_id, suite_id, &shared_secret, info, psk, psk_id)?;
    wipe(shared_secret.as_mut());

    let context = SenderContext(Context {
        aead: scheduled.aead,
        suite_id,
        base_nonce: scheduled.base_nonce,
        seq: 0,
        exporter_secret: scheduled.exporter_secret,
        _kdf: PhantomData,
    });
    return Ok((enc, context));
}

/// Sets up a receiver context (RFC 9180 Section 5.1).
///
/// # Errors
///
/// - [`HpkeError::DecapError`] when `enc` is invalid or the key exchange
///   fails. Note that in authenticated modes a wrong sender public key does
///   **not** fail here; it produces a different shared secret and the error
///   surfaces as [`HpkeError::OpenError`] on the first message.
/// - [`HpkeError::NotSupported`] when an authenticated mode is used with a KEM
///   that does not support it (e.g. [`XWing`](super::kem::XWing)).
/// - [`HpkeError::ValidationError`] when the PSK or its identifier is empty in
///   a PSK mode.
pub fn new_receiver<K: Kem, D: Kdf, A: Aead>(
    mode: &ModeReceiver<'_, K>,
    sk_r: &K::SecretKey,
    enc: &K::EncappedKey,
    info: &[u8],
) -> Result<ReceiverContext<A, D>, HpkeError> {
    const { assert!(K::NSECRET <= MAX_HASH_OUTPUT_SIZE) };

    let (mode_id, psk, psk_id) = mode.psk_parameters();
    verify_psk_inputs(mode_id, psk, psk_id)?;

    let mut shared_secret = match mode {
        ModeReceiver::Base
        | ModeReceiver::Psk {
            ..
        } => K::decap(enc, sk_r)?,
        ModeReceiver::Auth {
            pk_s,
        }
        | ModeReceiver::AuthPsk {
            pk_s, ..
        } => K::auth_decap(enc, sk_r, pk_s)?,
    };

    let suite_id = ciphersuite_id::<K, D, A>();
    let scheduled = key_schedule::<D, A>(mode_id, suite_id, &shared_secret, info, psk, psk_id)?;
    wipe(shared_secret.as_mut());

    let context = ReceiverContext(Context {
        aead: scheduled.aead,
        suite_id,
        base_nonce: scheduled.base_nonce,
        seq: 0,
        exporter_secret: scheduled.exporter_secret,
        _kdf: PhantomData,
    });
    return Ok(context);
}
