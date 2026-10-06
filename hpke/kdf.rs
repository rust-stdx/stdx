//! Key derivation functions for HPKE (RFC 9180 Section 4 and Section 7.2).

use crypto::{
    Hash, Hasher, MAX_HASH_OUTPUT_SIZE,
    hmac::Hmac,
    sha2::{Sha256, Sha512},
};

use super::HpkeError;

/// The HPKE version label, prepended by the default labeled derivations
/// (RFC 9180 Section 4).
const VERSION_LABEL: &[u8] = b"HPKE-v1";

/// Key derivation function (KDF) usable within an HPKE ciphersuite.
///
/// The default implementations follow the RFC 9180 constructions on top of
/// HKDF (RFC 5869) with the hash [`Kdf::H`]. `suite_id` is the caller's domain
/// separator: the 5-byte KEM `suite_id` (`"KEM" || I2OSP(kem_id, 2)`) inside a
/// KEM, or the 10-byte ciphersuite `suite_id` (`"HPKE" || I2OSP(kem_id, 2) ||
/// I2OSP(kdf_id, 2) || I2OSP(aead_id, 2)`) everywhere else. Passing the wrong
/// one silently breaks interoperability.
///
/// Implementors only need to provide [`Kdf::ID`] and the hash [`Kdf::H`]; for
/// example, a KDF based on another hash is two lines. A non-HKDF construction
/// (e.g. the SHAKE-based KDF of the post-quantum HPKE draft) overrides
/// [`Kdf::labeled_extract`] and [`Kdf::labeled_expand`] directly.
///
/// # Example
///
/// ```
/// use hpke::kdf::{HkdfSha256, Kdf};
///
/// let suite_id = b"HPKE\x00\x20\x00\x01\x00\x02";
/// let prk = HkdfSha256::labeled_extract(b"salt", suite_id, b"label", b"ikm");
/// let mut okm = [0u8; 32];
/// HkdfSha256::labeled_expand(&mut okm, &prk, suite_id, b"label", b"info").unwrap();
/// ```
pub trait Kdf {
    /// The HPKE KDF identifier (RFC 9180 Section 7.2, IANA "HPKE KDF
    /// Identifiers"), used to build the ciphersuite `suite_id`.
    const ID: u16;

    /// The hash function underlying the default HKDF construction.
    type Hash: Hasher;

    /// Output size `Nh` of the extract step (the hash output size).
    ///
    /// Must not exceed [`MAX_HASH_OUTPUT_SIZE`].
    const NH: usize = <Self::Hash as Hasher>::OUTPUT_SIZE;

    /// RFC 9180 Section 4 `LabeledExtract(salt, label, ikm)`:
    /// `Extract(salt, "HPKE-v1" || suite_id || label || ikm)`.
    ///
    /// `salt` may be empty, in which case the HKDF default salt (a string of
    /// `NH` zero bytes) is used. The prefix is fed to HMAC incrementally, so
    /// `ikm` may be arbitrarily long.
    fn labeled_extract(salt: &[u8], suite_id: &[u8], label: &[u8], ikm: &[u8]) -> Hash {
        const { assert!(Self::Hash::OUTPUT_SIZE <= MAX_HASH_OUTPUT_SIZE) };

        let mut mac = Hmac::<Self::Hash>::new(salt);
        mac.update(VERSION_LABEL);
        mac.update(suite_id);
        mac.update(label);
        mac.update(ikm);
        return mac.finalize();
    }

    /// RFC 9180 Section 4 `LabeledExpand(prk, label, info, L)`:
    /// `Expand(prk, I2OSP(L, 2) || "HPKE-v1" || suite_id || label || info, L)`,
    /// where `L == out.len()`. The `info` string is absorbed incrementally, so
    /// it may be arbitrarily long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::InvalidPrk`] when `prk` is not exactly
    /// [`Kdf::NH`] bytes long, and [`HpkeError::KdfOutputTooLong`] when
    /// `out.len() > 255 * NH`.
    fn labeled_expand(out: &mut [u8], prk: &[u8], suite_id: &[u8], label: &[u8], info: &[u8]) -> Result<(), HpkeError> {
        const { assert!(Self::Hash::OUTPUT_SIZE <= MAX_HASH_OUTPUT_SIZE) };

        if prk.len() != Self::Hash::OUTPUT_SIZE {
            return Err(HpkeError::InvalidPrk);
        }

        let n = out.len();
        if n > 255 * Self::Hash::OUTPUT_SIZE {
            return Err(HpkeError::KdfOutputTooLong);
        }

        if n == 0 {
            return Ok(());
        }

        let mut t = [0u8; MAX_HASH_OUTPUT_SIZE];
        let mut t_len = 0usize;
        let mut offset = 0usize;
        let mut counter = 1u8;

        while offset < n {
            let mut mac = Hmac::<Self::Hash>::new(prk);
            mac.update(&t[..t_len]);
            mac.update(&(n as u16).to_be_bytes());
            mac.update(VERSION_LABEL);
            mac.update(suite_id);
            mac.update(label);
            mac.update(info);
            mac.update(&[counter]);

            let block = mac.finalize();
            let block_bytes = block.as_ref();
            let chunk_len = (n - offset).min(Self::Hash::OUTPUT_SIZE);
            out[offset..offset + chunk_len].copy_from_slice(&block_bytes[..chunk_len]);
            t[..Self::Hash::OUTPUT_SIZE].copy_from_slice(block_bytes);
            t_len = Self::Hash::OUTPUT_SIZE;
            offset += chunk_len;
            counter = counter.wrapping_add(1);
        }

        return Ok(());
    }
}

/// HKDF-SHA256 (KDF id `0x0001`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HkdfSha256;

impl Kdf for HkdfSha256 {
    const ID: u16 = 0x0001;
    type Hash = Sha256;
}

/// HKDF-SHA512 (KDF id `0x0003`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HkdfSha512;

impl Kdf for HkdfSha512 {
    const ID: u16 = 0x0003;
    type Hash = Sha512;
}
