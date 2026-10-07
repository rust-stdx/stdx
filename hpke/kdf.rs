//! Key derivation functions for HPKE (RFC 9180 Section 4 and Section 7.2).

use crypto::{
    Hash, MAX_HASH_OUTPUT_SIZE,
    hmac::Hmac,
    sha2::{Sha256, Sha512},
};

use super::HpkeError;

/// The HPKE version label used by the labeled derivations (RFC 9180 Section 4
/// and `draft-ietf-hpke-pq-05` Section 5).
const VERSION_LABEL: &[u8] = b"HPKE-v1";

/// Key derivation function (KDF) usable within an HPKE ciphersuite.
///
/// The default implementations follow the RFC 9180 constructions on top of
/// HKDF (RFC 5869) with the hash [`Kdf::Hash`]. `suite_id` is the caller's domain
/// separator: the 5-byte KEM `suite_id` (`"KEM" || I2OSP(kem_id, 2)`) inside a
/// KEM, or the 10-byte ciphersuite `suite_id` (`"HPKE" || I2OSP(kem_id, 2) ||
/// I2OSP(kdf_id, 2) || I2OSP(aead_id, 2)`) everywhere else. Passing the wrong
/// one silently breaks interoperability.
///
/// Implementors only need to provide [`Kdf::HPKE_KDF_ID`] and the hash [`Kdf::Hash`]; for
/// example, a KDF based on another hash is two lines.
///
/// # Single-stage KDFs
///
/// The RFC 9180 key schedule is *two-stage*: it is built from
/// [`Kdf::labeled_extract`] and [`Kdf::labeled_expand`]. KDFs based on an
/// extendable-output function, such as the SHAKE256 KDF of
/// `draft-ietf-hpke-pq-05`, are *single-stage*: they derive the whole key
/// schedule from a single [`Kdf::labeled_derive`] call. A single-stage KDF
/// sets [`Kdf::SINGLE_STAGE`] to `true` and overrides
/// [`Kdf::labeled_derive`] (its [`Kdf::labeled_extract`] and
/// [`Kdf::labeled_expand`] implementations are never used).
///
/// # Example
///
/// ```
/// use hpke::kdf::{HkdfSha256, Kdf};
///
/// let suite_id = b"HPKE\x00\x20\x00\x01\x00\x02";
/// let pseudorandom_key = HkdfSha256::labeled_extract(b"salt", suite_id, b"label", b"ikm");
/// let mut output_keying_material = [0u8; 32];
/// HkdfSha256::labeled_expand(&mut output_keying_material, &pseudorandom_key, suite_id, b"label", b"info").unwrap();
/// ```
pub trait Kdf {
    /// The HPKE KDF identifier (RFC 9180 Section 7.2, IANA "HPKE KDF
    /// Identifiers"), used to build the ciphersuite `suite_id`.
    const HPKE_KDF_ID: u16;

    /// The hash function underlying the default HKDF construction.
    type Hash: crypto::Hasher;

    /// Output size of the extract step (the hash output size).
    ///
    /// Must not exceed [`MAX_HASH_OUTPUT_SIZE`].
    const OUTPUT_SIZE: usize = <Self::Hash as crypto::Hasher>::OUTPUT_SIZE;

    /// Whether this is a single-stage KDF (`draft-ietf-hpke-pq-05` Section 5).
    ///
    /// Two-stage KDFs (the default) use the RFC 9180 key schedule built on
    /// [`Kdf::labeled_extract`] and [`Kdf::labeled_expand`]. Single-stage KDFs
    /// (e.g. [`Shake256`]) build the schedule from a single
    /// [`Kdf::labeled_derive`] call and MUST override it.
    const SINGLE_STAGE: bool = false;

    /// RFC 9180 Section 4 `LabeledExtract(salt, label, input_keying_material)`:
    /// `Extract(salt, "HPKE-v1" || suite_id || label || input_keying_material)`.
    ///
    /// `salt` may be empty, in which case the HKDF default salt (a string of
    /// [`Kdf::OUTPUT_SIZE`] zero bytes) is used. The prefix is fed to HMAC
    /// incrementally, so `input_keying_material` may be arbitrarily long.
    fn labeled_extract(salt: &[u8], suite_id: &[u8], label: &[u8], input_keying_material: &[u8]) -> Hash {
        const { assert!(Self::OUTPUT_SIZE <= MAX_HASH_OUTPUT_SIZE) };

        let mut mac = Hmac::<Self::Hash>::new(salt);
        mac.update(VERSION_LABEL);
        mac.update(suite_id);
        mac.update(label);
        mac.update(input_keying_material);
        return mac.finalize();
    }

    /// RFC 9180 Section 4 `LabeledExpand(pseudorandom_key, label, info, L)`:
    /// `Expand(pseudorandom_key, I2OSP(L, 2) || "HPKE-v1" || suite_id || label || info, L)`,
    /// where `L == out.len()`. The `info` string is absorbed incrementally, so
    /// it may be arbitrarily long.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::InvalidPseudorandomKey`] when `pseudorandom_key` is not exactly
    /// [`Kdf::OUTPUT_SIZE`] bytes long, and [`HpkeError::KdfOutputTooLong`]
    /// when `out.len() > 255 * OUTPUT_SIZE`.
    fn labeled_expand(
        out: &mut [u8],
        pseudorandom_key: &[u8],
        suite_id: &[u8],
        label: &[u8],
        info: &[u8],
    ) -> Result<(), HpkeError> {
        const { assert!(Self::OUTPUT_SIZE <= MAX_HASH_OUTPUT_SIZE) };

        if pseudorandom_key.len() != Self::OUTPUT_SIZE {
            return Err(HpkeError::InvalidPseudorandomKey);
        }

        let n = out.len();
        if n > 255 * Self::OUTPUT_SIZE {
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
            let mut mac = Hmac::<Self::Hash>::new(pseudorandom_key);
            mac.update(&t[..t_len]);
            mac.update(&(n as u16).to_be_bytes());
            mac.update(VERSION_LABEL);
            mac.update(suite_id);
            mac.update(label);
            mac.update(info);
            mac.update(&[counter]);

            let block = mac.finalize();
            let block_bytes = block.as_ref();
            let chunk_len = (n - offset).min(Self::OUTPUT_SIZE);
            out[offset..offset + chunk_len].copy_from_slice(&block_bytes[..chunk_len]);
            t[..Self::OUTPUT_SIZE].copy_from_slice(block_bytes);
            t_len = Self::OUTPUT_SIZE;
            offset += chunk_len;
            counter = counter.wrapping_add(1);
        }

        return Ok(());
    }

    /// `draft-ietf-hpke-pq-05` Section 5
    /// `LabeledDerive(input_keying_material, label, context, L)`:
    /// `Derive(input_keying_material || "HPKE-v1" || suite_id ||
    /// I2OSP(len(label), 2) || label || I2OSP(L, 2) || context, L)`, where
    /// `L == out.len()` and the elements of `input_keying_material` and
    /// `context` are absorbed in order.
    ///
    /// This is the single-stage counterpart of
    /// [`Kdf::labeled_extract`]/[`Kdf::labeled_expand`]. `suite_id` has the
    /// same meaning as for the two-stage functions: the 5-byte KEM `suite_id`
    /// inside a KEM, or the 10-byte ciphersuite `suite_id` everywhere else.
    ///
    /// # Errors
    ///
    /// Returns [`HpkeError::KdfOutputTooLong`] when `out` is longer than
    /// `2^16 - 1` bytes, and [`HpkeError::NotSupported`] by default (only
    /// single-stage KDFs implement this).
    fn labeled_derive(
        _out: &mut [u8],
        _input_keying_material: &[&[u8]],
        _suite_id: &[u8],
        _label: &[u8],
        _context: &[&[u8]],
    ) -> Result<(), HpkeError> {
        return Err(HpkeError::NotSupported);
    }
}

/// HKDF-SHA256 (KDF id `0x0001`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HkdfSha256;

impl Kdf for HkdfSha256 {
    const HPKE_KDF_ID: u16 = 0x0001;
    type Hash = Sha256;
}

/// HKDF-SHA512 (KDF id `0x0003`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HkdfSha512;

impl Kdf for HkdfSha512 {
    const HPKE_KDF_ID: u16 = 0x0003;
    type Hash = Sha512;
}

/// SHAKE256 single-stage KDF (KDF id `0x0011`), as specified in
/// [`draft-ietf-hpke-pq-05`](https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05)
/// Section 5.
///
/// SHAKE256 is a single-stage KDF with an output size of 64 bytes. It derives
/// the key schedule with a single [`Kdf::labeled_derive`] call instead of the
/// RFC 9180 `LabeledExtract`/`LabeledExpand` construction, and is a natural
/// pairing with the hybrid post-quantum KEMs defined in the same draft.
///
/// # Example
///
/// ```
/// use hpke::kdf::{Kdf, Shake256};
///
/// let suite_id = b"HPKE\x64\x7a\x00\x11\x00\x03";
/// let mut output_keying_material = [0u8; 32];
/// Shake256::labeled_derive(&mut output_keying_material, &[b"ikm"], suite_id, b"label", &[b"info"]).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shake256;

impl Kdf for Shake256 {
    const HPKE_KDF_ID: u16 = 0x0011;
    const SINGLE_STAGE: bool = true;
    type Hash = crypto::sha3::Shake256;

    fn labeled_derive(
        out: &mut [u8],
        input_keying_material: &[&[u8]],
        suite_id: &[u8],
        label: &[u8],
        context: &[&[u8]],
    ) -> Result<(), HpkeError> {
        if out.len() > u16::MAX as usize {
            return Err(HpkeError::KdfOutputTooLong);
        }
        if label.len() > u16::MAX as usize {
            return Err(HpkeError::ValidationError);
        }

        let label_len = (label.len() as u16).to_be_bytes();
        let out_len = (out.len() as u16).to_be_bytes();

        use crypto::Xof;
        let mut xof = crypto::sha3::Shake256::new();
        for part in input_keying_material {
            xof.absorb(part);
        }
        xof.absorb(VERSION_LABEL);
        xof.absorb(suite_id);
        xof.absorb(&label_len);
        xof.absorb(label);
        xof.absorb(&out_len);
        for part in context {
            xof.absorb(part);
        }
        xof.squeeze(out);
        return Ok(());
    }
}

/// BLAKE3 single-stage KDF (unofficial KDF id `0xFF01`).
///
/// BLAKE3 is an extendable-output function, so it is used exactly like
/// [`Shake256`]: the key schedule is built from a single [`Kdf::labeled_derive`]
/// call whose `Derive` primitive is the BLAKE3 XOF. The output size is 32
/// bytes.
///
/// # Interoperability
///
/// BLAKE3 is **not** registered in the IANA "HPKE KDF Identifiers" registry and
/// is not specified by any HPKE standard. The identifier `0xFF01` is an
/// stdx-private, unofficial value chosen to stay clear of the low IDs IANA is
/// likely to assign to future official KDFs; ciphersuites using this KDF only
/// interoperate with implementations that adopt the same convention.
///
/// # Example
///
/// ```
/// use hpke::kdf::{Blake3, Kdf};
///
/// let suite_id = b"HPKE\x64\x7a\xff\x01\x00\x03";
/// let mut output_keying_material = [0u8; 32];
/// Blake3::labeled_derive(&mut output_keying_material, &[b"ikm"], suite_id, b"label", &[b"info"]).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blake3;

impl Kdf for Blake3 {
    const HPKE_KDF_ID: u16 = 0xFF01;
    const SINGLE_STAGE: bool = true;
    type Hash = crypto::blake3::Blake3;

    fn labeled_derive(
        out: &mut [u8],
        input_keying_material: &[&[u8]],
        suite_id: &[u8],
        label: &[u8],
        context: &[&[u8]],
    ) -> Result<(), HpkeError> {
        if out.len() > u16::MAX as usize {
            return Err(HpkeError::KdfOutputTooLong);
        }
        if label.len() > u16::MAX as usize {
            return Err(HpkeError::ValidationError);
        }

        let label_len = (label.len() as u16).to_be_bytes();
        let out_len = (out.len() as u16).to_be_bytes();

        use crypto::{Hasher, Xof};
        let mut hasher = crypto::blake3::Blake3::new();
        for part in input_keying_material {
            hasher.update(part);
        }
        hasher.update(VERSION_LABEL);
        hasher.update(suite_id);
        hasher.update(&label_len);
        hasher.update(label);
        hasher.update(&out_len);
        for part in context {
            hasher.update(part);
        }
        hasher.finalize_xof().squeeze(out);
        return Ok(());
    }
}
