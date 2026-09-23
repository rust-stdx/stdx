//! RSA signature verification: PKCS#1 v1.5 and RSA-PSS (RFC 8017).
//!
//! Implements modular exponentiation and padding verification for
//! rsa_pkcs1_sha256 / sha384 / sha512 and rsa_pss_* schemes.
//!
//! Only verification is supported; signing is not implemented.

use big_number::Uint;

use crate::{Hasher, RsaError};

/// Minimum RSA modulus size accepted (2048 bits).
const RSA_MIN_BITS: usize = 2048;
/// Maximum RSA modulus size supported (8192 bits).
const RSA_MAX_BITS: usize = 8192;
const RSA_MAX_LIMBS: usize = RSA_MAX_BITS / 64;
const RSA_MAX_BYTES: usize = RSA_MAX_BITS / 8;

/// SHA-256 DigestInfo prefix for RSA PKCS#1 v1.5.
pub const DIGEST_INFO_SHA256_PREFIX: &[u8] = &[
    0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20,
];

/// SHA-384 DigestInfo prefix for RSA PKCS#1 v1.5.
pub const DIGEST_INFO_SHA384_PREFIX: &[u8] = &[
    0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05, 0x00, 0x04, 0x30,
];

/// SHA-512 DigestInfo prefix for RSA PKCS#1 v1.5.
pub const DIGEST_INFO_SHA512_PREFIX: &[u8] = &[
    0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05, 0x00, 0x04, 0x40,
];

/// An RSA public key parsed from a PKCS#1 `SubjectPublicKeyInfo` DER blob.
pub struct PublicKey {
    /// Modulus `n` as a 8192-bit integer (zero-padded for smaller keys).
    n: Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>,
    /// Public exponent `e`.
    e: Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>,
    /// Byte length of the modulus (without padding).
    n_len: usize,
    /// Byte length of the public exponent (without leading zeros).
    e_len: usize,
    /// Barrett reduction precomputation for mod n.
    mu: [u64; big_number::MAX_LIMBS],
}

impl PublicKey {
    /// Build an RSA public key from raw modulus `n` and public exponent `e`
    /// (both big-endian byte slices).
    ///
    /// This is useful when importing keys from formats like JWK where `n` and
    /// `e` are available directly as bytes rather than inside an ASN.1 DER
    /// wrapper.
    ///
    /// The parameters are validated before the key is accepted: `n` must be
    /// odd and between 2048 and 8192 bits, and `e` must be odd, at least 3,
    /// and strictly smaller than `n`.
    ///
    /// # Errors
    ///
    /// Returns [`RsaError::InvalidKey`] if `n` or `e` fail the checks above
    /// (including empty or even `n`, `e < 3`, even `e`, or `e >= n`), and
    /// [`RsaError::NotSupported`] if either value exceeds 8192 bits.
    #[inline]
    pub fn from_n_e(n_bytes: &[u8], e: &[u8]) -> Result<Self, RsaError> {
        let n = uint_from_variable_be(n_bytes)?;
        let e_uint = uint_from_variable_be(e)?;
        if n.is_zero() || !n.is_odd() || n.bit_len() < RSA_MIN_BITS {
            return Err(RsaError::InvalidKey);
        }
        if e_uint.bit_len() < 2 || !e_uint.is_odd() || e_uint.ct_ge(&n) {
            return Err(RsaError::InvalidKey);
        }
        let e_len = {
            let mut start: usize = 0;
            while start < e.len().saturating_sub(1) && e[start] == 0 {
                start += 1;
            }
            e.len() - start
        };
        Ok(PublicKey {
            n_len: n_bytes.len(),
            mu: n.compute_mu_for_barrett(),
            e: e_uint,
            e_len,
            n,
        })
    }

    /// Parse an RSA public key from the raw PKCS#1 bytes.
    ///
    /// The input is the content of the BIT STRING inside the SPKI —
    /// a strict DER `SEQUENCE { INTEGER n, INTEGER e }`. Non-minimal length
    /// encodings, negative or non-minimally-encoded INTEGERs, an oversized
    /// outer SEQUENCE, and any trailing bytes inside or after the SEQUENCE are
    /// rejected.
    ///
    /// The same parameter validation as [`PublicKey::from_n_e`] is applied.
    ///
    /// # Errors
    ///
    /// Returns [`RsaError::Unspecified`] if `data` is not the exact strict DER
    /// encoding of `SEQUENCE { INTEGER n, INTEGER e }`, and
    /// [`RsaError::InvalidKey`] or [`RsaError::NotSupported`] when the
    /// modulus/exponent fail the [`PublicKey::from_n_e`] checks.
    pub fn from_pkcs1_der(data: &[u8]) -> Result<Self, RsaError> {
        if data.len() < 2 || data[0] != 0x30 {
            return Err(RsaError::Unspecified);
        }
        let (seq_len, len_size) = read_length(&data[1..])?;
        let header = 1 + len_size;
        if data.len() != header + seq_len {
            return Err(RsaError::Unspecified);
        }
        let mut body = &data[header..];

        let mut n_buf = [0u8; RSA_MAX_BYTES];
        let (n_len, consumed) = read_integer(body, &mut n_buf)?;
        body = &body[consumed..];

        let mut e_buf = [0u8; RSA_MAX_BYTES];
        let (e_len, consumed) = read_integer(body, &mut e_buf)?;
        body = &body[consumed..];

        if !body.is_empty() {
            return Err(RsaError::Unspecified);
        }

        Self::from_n_e(&n_buf[..n_len], &e_buf[..e_len])
    }

    /// Verify a PKCS#1 v1.5 signature.
    ///
    /// `signature` is the raw signature bytes (eg., 256 bytes for RSA-2048).
    /// `message_digest` is the hash of the message to verify.
    /// `digest_info_prefix` is the ASN.1 DigestInfo prefix for the hash algorithm
    /// (the constant-length portion before the hash value).
    ///
    /// The padding and DigestInfo encoding must be strict DER and the DigestInfo
    /// must fill the modulus exactly (`EM = 00 01 PS 00 T`, RFC 8017 §9.2).
    /// Non-canonical encodings, including trailing zero bytes after the
    /// DigestInfo, are rejected.
    ///
    /// # Errors
    ///
    /// Returns [`RsaError::Unspecified`] if `signature` is not exactly `n_len`
    /// bytes long, if the signature is not reduced modulo `n`, or if the padded
    /// message does not match the canonical encoding for `digest_info_prefix`
    /// and `message_digest`.
    pub fn verify_pkcs1_v1_5(
        &self,
        signature: &[u8],
        message_digest: &[u8],
        digest_info_prefix: &[u8],
    ) -> Result<(), RsaError> {
        if signature.len() != self.n_len {
            return Err(RsaError::Unspecified);
        }

        let s = uint_from_variable_be(signature)?;

        // Reject signatures not reduced modulo n
        if s.ct_ge(&self.n) {
            return Err(RsaError::Unspecified);
        }

        let m = s.modpow_barrett(&self.e, &self.n, &self.mu);

        let mod_bytes = self.n_len;
        let mut m_bytes = [0u8; RSA_MAX_BYTES];
        write_uint_be(&m, &mut m_bytes, mod_bytes);

        if !pkcs1_v1_5_em_matches(&m_bytes[..mod_bytes], digest_info_prefix, message_digest) {
            return Err(RsaError::Unspecified);
        }

        Ok(())
    }

    /// Verify an RSA-PSS signature (RFC 8017 §9.1.2).
    ///
    /// `message` is hashed with `H`, whose digest length must equal `salt_len`
    /// (Go's `PSSSaltLengthEqualsHash`).
    ///
    /// Per RFC 8017, the encoded-message bit length is `emBits = bitlen(n) - 1`
    /// and `emLen = ceil(emBits / 8)`. This matters for moduli whose top bit is
    /// not set: `emLen` can be one byte shorter than the modulus and the number
    /// of leftmost bits cleared from `DB` is `8·emLen - emBits`, not always 1.
    pub fn verify_pss<H: Hasher>(&self, signature: &[u8], message: &[u8], salt_len: usize) -> Result<(), RsaError> {
        if signature.len() != self.n_len {
            return Err(RsaError::Unspecified);
        }

        let s = uint_from_variable_be(signature)?;
        if s.ct_ge(&self.n) {
            return Err(RsaError::Unspecified);
        }

        let m = s.modpow_barrett(&self.e, &self.n, &self.mu);
        let hash_len = H::OUTPUT_SIZE;

        // RFC 8017 §9.1.2: emBits = modBits - 1 and emLen = ceil(emBits / 8),
        // where modBits = bitlen(n). This can differ from n_len when the top
        // bit of the modulus is not set.
        let em_bits = self.n.bit_len() - 1;
        let em_len = em_bits.div_ceil(8);

        // I2OSP(m, emLen) requires m < 256^emLen. Only reachable when emBits is
        // a multiple of 8, in which case m may need emLen + 1 bytes.
        if m.bit_len() > 8 * em_len {
            return Err(RsaError::Unspecified);
        }

        let mut em = [0u8; RSA_MAX_BYTES];
        write_uint_be(&m, &mut em, em_len);

        let leftmost_bits = 8 * em_len - em_bits;
        if leftmost_bits > 0 && leftmost_bits < 8 {
            if em[0] >> (8 - leftmost_bits) != 0 {
                return Err(RsaError::Unspecified);
            }
        }

        if em_len < hash_len + salt_len + 2 {
            return Err(RsaError::Unspecified);
        }
        if em[em_len - 1] != 0xBC {
            return Err(RsaError::Unspecified);
        }

        let masked_db_len = em_len - hash_len - 1;
        let masked_db = &em[..masked_db_len];
        let h = &em[masked_db_len..masked_db_len + hash_len];

        let mut db_mask = [0u8; RSA_MAX_BYTES];
        mgf1::<H>(h, &mut db_mask[..masked_db_len]);

        let mut db = [0u8; RSA_MAX_BYTES];
        for (i, (a, b)) in masked_db.iter().zip(db_mask[..masked_db_len].iter()).enumerate() {
            db[i] = a ^ b;
        }

        if leftmost_bits > 0 && leftmost_bits < 8 {
            db[0] &= 0xff >> leftmost_bits;
        }

        let ps_len = em_len - hash_len - salt_len - 2;
        if ps_len > 0 {
            for i in 0..ps_len {
                if db[i] != 0x00 {
                    return Err(RsaError::Unspecified);
                }
            }
        }
        if db[ps_len] != 0x01 {
            return Err(RsaError::Unspecified);
        }

        let salt = &db[ps_len + 1..ps_len + 1 + salt_len];

        let m_hash = H::hash(message);
        let mut mp = [0u8; 8 + 64 + RSA_MAX_BYTES];
        let mp_len = 8 + hash_len + salt_len;
        mp[8..8 + hash_len].copy_from_slice(m_hash.as_ref());
        mp[8 + hash_len..mp_len].copy_from_slice(salt);

        let hp = H::hash(&mp[..mp_len]);

        let mut ok = 0u8;
        for i in 0..hash_len {
            ok |= h[i] ^ hp.as_ref()[i];
        }
        if ok != 0 {
            return Err(RsaError::Unspecified);
        }

        Ok(())
    }

    /// Returns the modulus `n` as big-endian bytes, trimmed to the actual key size.
    /// (e.g. 256 bytes for RSA-2048, 512 bytes for RSA-4096, 1024 bytes for RSA-8192).
    #[cfg(feature = "alloc")]
    pub fn n_bytes(&self) -> alloc::vec::Vec<u8> {
        let full = self.n.to_be_bytes_fixed::<{ RSA_MAX_BYTES }>();
        full[full.len() - self.n_len..].to_vec()
    }

    /// Returns the public exponent `e` as big-endian bytes, trimmed of leading zeros.
    /// (e.g. `[1, 0, 1]` for 65537).
    #[cfg(feature = "alloc")]
    pub fn e_bytes(&self) -> smallvec::SmallVec<u8, 4> {
        let full = self.e.to_be_bytes_fixed::<{ RSA_MAX_BYTES }>();
        full[full.len() - self.e_len..].into()
    }
}

/// Validate a PKCS#1 v1.5 encoded message `EM = 00 01 PS 00 T`.
///
/// `em` is the full `k`-byte encoded message. Returns `true` only when the
/// padding is canonical (`00 01`, at least 8 `0xFF` bytes, `0x00` separator),
/// when `digest_info_prefix` and `message_digest` match the trailing bytes, and
/// when that DigestInfo occupies the remainder of `em` exactly. The last rule
/// is what rejects non-conforming encodings that merely append zero bytes after
/// a valid DigestInfo (RFC 8017 §9.2 requires `T` to fill the modulus).
fn pkcs1_v1_5_em_matches(em: &[u8], digest_info_prefix: &[u8], message_digest: &[u8]) -> bool {
    let mod_bytes = em.len();
    if mod_bytes < 2 || em[0] != 0x00 || em[1] != 0x01 {
        return false;
    }

    // Find the 0x00 separator after the FF padding.
    let mut sep = 2;
    while sep < mod_bytes && em[sep] == 0xff {
        sep += 1;
    }
    // Require the separator and at least 8 bytes of FF padding.
    if sep >= mod_bytes || em[sep] != 0x00 || sep < 10 {
        return false;
    }

    let di_start = sep + 1;
    let expected_len = digest_info_prefix.len() + message_digest.len();
    // `di_start <= mod_bytes` because `sep < mod_bytes`; the DigestInfo must end
    // exactly at the modulus (no shortfall, no trailing bytes).
    if mod_bytes - di_start != expected_len {
        return false;
    }

    // Verify the canonical DigestInfo prefix and the hash value.
    let mut ok = 0u8;
    for i in 0..digest_info_prefix.len() {
        ok |= em[di_start + i] ^ digest_info_prefix[i];
    }
    for i in 0..message_digest.len() {
        ok |= em[di_start + digest_info_prefix.len() + i] ^ message_digest[i];
    }
    ok == 0
}

/// MGF1 (Mask Generation Function 1) per RFC 8017 Appendix B.2.1.
fn mgf1<H: Hasher>(seed: &[u8], out: &mut [u8]) {
    let out_len = out.len();
    let hash_len = H::OUTPUT_SIZE;
    let mut offset = 0;
    let mut counter: u32 = 0;
    let input_prefix = seed.len();
    let mut input_buf = [0u8; RSA_MAX_BYTES + 4];
    input_buf[..input_prefix].copy_from_slice(seed);
    while offset < out_len {
        input_buf[input_prefix..input_prefix + 4].copy_from_slice(&counter.to_be_bytes());
        let hash = H::hash(&input_buf[..input_prefix + 4]);
        let take = (out_len - offset).min(hash_len);
        out[offset..offset + take].copy_from_slice(&hash.as_ref()[..take]);
        offset += take;
        counter += 1;
    }
}

/// Read an ASN.1 length field. Returns (value, bytes_consumed_for_length).
///
/// The encoding must be the minimal DER form prescribed by X.690 §10.1:
/// long form is accepted only when the length is at least 128 and its first
/// content byte is non-zero. Indefinite lengths (`0x80`) and long forms wider
/// than four bytes are rejected.
fn read_length(data: &[u8]) -> Result<(usize, usize), RsaError> {
    if data.is_empty() {
        return Err(RsaError::Unspecified);
    }
    if data[0] & 0x80 == 0 {
        Ok((data[0] as usize, 1))
    } else {
        let num_bytes = (data[0] & 0x7f) as usize;
        if num_bytes == 0 || num_bytes > 4 || data.len() < 1 + num_bytes {
            return Err(RsaError::Unspecified);
        }
        if data[1] == 0x00 {
            return Err(RsaError::Unspecified);
        }
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | data[1 + i] as usize;
        }
        if len < 0x80 {
            return Err(RsaError::Unspecified);
        }
        Ok((len, 1 + num_bytes))
    }
}

/// Read an ASN.1 INTEGER, writing its magnitude into `out` and returning
/// `(value_len, total_consumed)`.
///
/// The encoding must be strict DER: the contents may not be empty, may not be
/// negative (top bit set), and any leading `0x00` is allowed only when needed to
/// keep the following byte's top bit from being read as a sign bit. This is the
/// canonical form required by RFC 8017 for RSA public key parameters.
fn read_integer(data: &[u8], out: &mut [u8]) -> Result<(usize, usize), RsaError> {
    if data.len() < 2 || data[0] != 0x02 {
        return Err(RsaError::Unspecified);
    }
    let (len, len_size) = read_length(&data[1..])?;
    let consumed = 1 + len_size + len;
    if len == 0 || data.len() < consumed {
        return Err(RsaError::Unspecified);
    }
    let bytes = &data[1 + len_size..consumed];
    if bytes[0] & 0x80 != 0 {
        return Err(RsaError::Unspecified);
    }
    let (value, val_len) = if bytes[0] == 0x00 {
        if bytes.len() < 2 || bytes[1] & 0x80 == 0 {
            return Err(RsaError::Unspecified);
        }
        (&bytes[1..], bytes.len() - 1)
    } else {
        (bytes, bytes.len())
    };
    if out.len() < val_len {
        return Err(RsaError::Unspecified);
    }
    out[..val_len].copy_from_slice(value);
    Ok((val_len, consumed))
}

/// Build a `Uint` from a variable-length big-endian byte slice
/// (left-padded with zeros to the full bit width).
fn uint_from_variable_be(bytes: &[u8]) -> Result<Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>, RsaError> {
    let max_bytes = RSA_MAX_BITS / 8;
    if bytes.len() > max_bytes {
        return Err(RsaError::NotSupported);
    }
    let mut limbs = [0u64; RSA_MAX_LIMBS];
    let byte_count = bytes.len();
    let mut i = 0;
    while i < RSA_MAX_LIMBS {
        let limb_start = byte_count.saturating_sub((i + 1) * 8);
        let limb_end = byte_count.saturating_sub(i * 8);
        let len = limb_end - limb_start;
        let mut buf = [0u8; 8];
        if len > 0 {
            buf[8 - len..].copy_from_slice(&bytes[limb_start..limb_end]);
        }
        limbs[i] = u64::from_be_bytes(buf);
        i += 1;
    }
    Ok(Uint::from_limbs(limbs))
}

/// Write a `Uint` as big-endian bytes into a buffer, right-aligned.
/// Only converts the required limbs instead of the full 8192-bit representation.
fn write_uint_be(value: &Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>, out: &mut [u8], byte_len: usize) {
    assert!(byte_len <= RSA_MAX_BYTES);
    let full = value.to_be_bytes_fixed::<{ RSA_MAX_BYTES }>();
    let start = full.len() - byte_len;
    out[..byte_len].copy_from_slice(&full[start..]);
}

/// Convenience: verify RSA-PKCS1-SHA256.
pub fn verify_pkcs1_sha256(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = PublicKey::from_pkcs1_der(pkcs1_der)?;
    let digest = crate::sha2::Sha256::hash(message);
    key.verify_pkcs1_v1_5(signature, digest.as_ref(), DIGEST_INFO_SHA256_PREFIX)
}

/// Convenience: verify RSA-PKCS1-SHA384.
pub fn verify_pkcs1_sha384(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = PublicKey::from_pkcs1_der(pkcs1_der)?;
    let digest = crate::sha2::Sha384::hash(message);
    key.verify_pkcs1_v1_5(signature, digest.as_ref(), DIGEST_INFO_SHA384_PREFIX)
}

/// Convenience: verify RSA-PKCS1-SHA512.
pub fn verify_pkcs1_sha512(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = PublicKey::from_pkcs1_der(pkcs1_der)?;
    let digest = crate::sha2::Sha512::hash(message);
    key.verify_pkcs1_v1_5(signature, digest.as_ref(), DIGEST_INFO_SHA512_PREFIX)
}

/// Convenience: verify RSA-PSS-SHA256.
pub fn verify_pss_sha256(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = PublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss::<crate::sha2::Sha256>(signature, message, 32)
}

/// Convenience: verify RSA-PSS-SHA384.
pub fn verify_pss_sha384(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = PublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss::<crate::sha2::Sha384>(signature, message, 48)
}

/// Convenience: verify RSA-PSS-SHA512.
pub fn verify_pss_sha512(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = PublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss::<crate::sha2::Sha512>(signature, message, 64)
}

#[cfg(test)]
mod tests {
    use hex;

    use super::*;

    macro_rules! wycheproof_rsa_test {
        ($path:expr, $hasher:ty, $di_prefix:expr) => {{
            let data: serde_json::Value = serde_json::from_str(include_str!($path)).unwrap();
            let mut valid_tested = 0u64;
            let mut invalid_tested = 0u64;

            for group in data["testGroups"].as_array().unwrap() {
                let pkcs1_der = hex::decode(group["publicKeyAsn"].as_str().unwrap()).unwrap();
                let key = super::PublicKey::from_pkcs1_der(&pkcs1_der).unwrap();

                for test in group["tests"].as_array().unwrap() {
                    let msg_hex = test["msg"].as_str().unwrap();
                    let sig_hex = test["sig"].as_str().unwrap();
                    let result = test["result"].as_str().unwrap();

                    let msg = hex::decode(msg_hex).unwrap();
                    let sig = hex::decode(sig_hex).unwrap();

                    let digest = <$hasher as crate::Hasher>::hash(&msg);
                    let verify_result = key.verify_pkcs1_v1_5(&sig, digest.as_ref(), $di_prefix);

                    match result {
                        "valid" => {
                            assert!(
                                verify_result.is_ok(),
                                "{}: tcId {}: expected valid, got error",
                                $path,
                                test["tcId"]
                            );
                            valid_tested += 1;
                        }
                        "invalid" => {
                            assert!(
                                verify_result.is_err(),
                                "{}: tcId {} ({:?}): expected invalid, got ok",
                                $path,
                                test["tcId"],
                                test.get("flags"),
                            );
                            invalid_tested += 1;
                        }
                        "acceptable" => {}
                        _ => panic!("unknown result: {result}"),
                    }
                }
            }

            assert!(valid_tested > 0, "no valid RSA tests were run");
            assert!(invalid_tested > 0, "no invalid RSA tests were run");
        }};
    }

    macro_rules! wycheproof_rsa_pss_test {
        ($path:expr, $hasher:ty, $hash_len:expr) => {{
            let data: serde_json::Value = serde_json::from_str(include_str!($path)).unwrap();
            let mut valid_tested = 0u64;
            let mut invalid_tested = 0u64;

            for group in data["testGroups"].as_array().unwrap() {
                let pkcs1_der = hex::decode(group["publicKeyAsn"].as_str().unwrap()).unwrap();
                let key = super::PublicKey::from_pkcs1_der(&pkcs1_der).unwrap();

                for test in group["tests"].as_array().unwrap() {
                    let msg_hex = test["msg"].as_str().unwrap();
                    let sig_hex = test["sig"].as_str().unwrap();
                    let result = test["result"].as_str().unwrap();

                    let msg = hex::decode(msg_hex).unwrap();
                    let sig = hex::decode(sig_hex).unwrap();

                    let verify_result = key.verify_pss::<$hasher>(&sig, &msg, $hash_len);

                    match result {
                        "valid" => {
                            assert!(
                                verify_result.is_ok(),
                                "{}: tcId {}: expected valid, got error: {:?}",
                                $path,
                                test["tcId"],
                                verify_result,
                            );
                            valid_tested += 1;
                        }
                        "invalid" => {
                            assert!(
                                verify_result.is_err(),
                                "{}: tcId {} ({:?}): expected invalid, got ok",
                                $path,
                                test["tcId"],
                                test.get("flags"),
                            );
                            invalid_tested += 1;
                        }
                        "acceptable" => {}
                        _ => panic!("unknown result: {result}"),
                    }
                }
            }

            assert!(valid_tested > 0, "no valid RSA-PSS tests were run for {}", $path);
            assert!(invalid_tested > 0, "no invalid RSA-PSS tests were run for {}", $path);
        }};
    }

    #[test]
    fn modpow_works() {
        let base = uint_from_variable_be(&[3]).unwrap();
        let exp = uint_from_variable_be(&[5]).unwrap();
        let modulus = uint_from_variable_be(&[7]).unwrap();
        let result = base.modpow(&exp, &modulus);
        let bytes = result.to_be_bytes_fixed::<{ RSA_MAX_BYTES }>();
        assert_eq!(
            bytes[RSA_MAX_BYTES - 1],
            5,
            "3^5 mod 7 should be 5, got {}",
            bytes[RSA_MAX_BYTES - 1]
        );
    }

    #[test]
    fn from_n_e_matches_der() {
        let sig = hex::decode(
            "9d00f18defaa95b474b06ac4674b1b9270e110c6f474ce29e3aa972eca09137c9a82267e634986ecd54734f2edb1b3d72b539b8608e23074898c56042f9f014bfff59abce81c57d606b60f80ae4e110fc6f9dea99ce2897ce1d90661ab3d3b3f1a5ddf258b920a51c8c8758ab2da3da20da99c84eb2f57859b36918447c4cdbfa16cc09523fd27d28d4e97fa9ff0ea4d633c937a904a196a64e934851ee02b7922a8f5a4534bb10b8e16b89c12ddc347d7b4317f8b9d3dfed07a442d47351b18db38f45cc92e5c577b866df21766094d1f737ea418852827be3aec10d3c5a65a40087d9647b91a4d9419ad784a31caf02254cfc01a682bb6f5a231307f0fc8d9",
        ).unwrap();
        let digest = hex::decode("41cb0773387b187c038b7015498534c11369f1cfd094a714f4f39cf63ebb42ba").unwrap();

        // n and e extracted from the DER above
        let n = hex::decode(
            "b1d59f746650c6a4360d26dc2e05581e1bd12cddcfc459a75dd2ef6d38cb6e977c72cad72f5e8ad4795484211e71e9a292d25a3901fca4cd242649f56cce50ad6ba148658d71f3a9c8b39e92a7a49543243df8ca2688292d47ff2a92a6ee0c9151162936791f522afccd6a7508251934b909d62fa805bae0d79f83f3c981b39c15ea79ce7b4ec2ff82240ce2a9fb93ae49d7697d1248f73d4ad23461055f469a3936ab959a0c6a067aa19521650f3649a028e2ebe355909aae7c95d3fc988684478b2bb11b307cb58c6c14727e1b62103d400ac8eed0e0d6d7f7d7cfc1f4ae4cbd9759372f8408c52174abb05f134ca6788fb60ba3f35c57c07cd44011bb113b",
        ).unwrap();
        let e = hex::decode("010001").unwrap();

        let key = PublicKey::from_n_e(&n, &e).unwrap();
        key.verify_pkcs1_v1_5(&sig, &digest, DIGEST_INFO_SHA256_PREFIX).unwrap();
    }

    #[test]
    fn pkcs1_v1_5_requires_digest_info_to_fill_modulus() {
        let hash = [0xa5u8; 32];
        let prefix = DIGEST_INFO_SHA256_PREFIX;
        let expected_len = prefix.len() + hash.len();
        let mod_bytes = 256usize;
        // Canonical separator: the DigestInfo ends exactly at the modulus.
        let sep = mod_bytes - 1 - expected_len;

        let mut em = [0u8; 256];
        em[0] = 0x00;
        em[1] = 0x01;
        for b in em[2..sep].iter_mut() {
            *b = 0xff;
        }
        em[sep] = 0x00;
        em[sep + 1..sep + 1 + prefix.len()].copy_from_slice(prefix);
        em[sep + 1 + prefix.len()..sep + 1 + expected_len].copy_from_slice(&hash);
        assert!(sep >= 10, "canonical padding must keep at least 8 FF bytes");
        assert!(pkcs1_v1_5_em_matches(&em, prefix, &hash));

        // Same modulus, but the DigestInfo is shifted one byte earlier and the
        // final byte is a trailing 0x00: EM = 00 01 FF..FF 00 T || 00. This is
        // accepted by a "trailing bytes must be zero" check but rejected because
        // the DigestInfo does not fill the modulus (M-09).
        let mut em_trailing = [0u8; 256];
        em_trailing[0] = 0x00;
        em_trailing[1] = 0x01;
        for b in em_trailing[2..sep - 1].iter_mut() {
            *b = 0xff;
        }
        em_trailing[sep - 1] = 0x00;
        let di = sep;
        em_trailing[di..di + prefix.len()].copy_from_slice(prefix);
        em_trailing[di + prefix.len()..di + expected_len].copy_from_slice(&hash);
        assert!(!pkcs1_v1_5_em_matches(&em_trailing, prefix, &hash));
    }

    #[test]
    fn from_n_e_validates_parameters() {
        // 2048-bit odd modulus.
        let n = hex::decode(
            "b1d59f746650c6a4360d26dc2e05581e1bd12cddcfc459a75dd2ef6d38cb6e977c72cad72f5e8ad4795484211e71e9a292d25a3901fca4cd242649f56cce50ad6ba148658d71f3a9c8b39e92a7a49543243df8ca2688292d47ff2a92a6ee0c9151162936791f522afccd6a7508251934b909d62fa805bae0d79f83f3c981b39c15ea79ce7b4ec2ff82240ce2a9fb93ae49d7697d1248f73d4ad23461055f469a3936ab959a0c6a067aa19521650f3649a028e2ebe355909aae7c95d3fc988684478b2bb11b307cb58c6c14727e1b62103d400ac8eed0e0d6d7f7d7cfc1f4ae4cbd9759372f8408c52174abb05f134ca6788fb60ba3f35c57c07cd44011bb113b",
        ).unwrap();
        let e = hex::decode("010001").unwrap();

        // Valid odd exponents >= 3.
        assert!(PublicKey::from_n_e(&n, &e).is_ok());
        assert!(PublicKey::from_n_e(&n, &[3]).is_ok());

        // Exponents zero, one, two, and even values are rejected.
        assert!(PublicKey::from_n_e(&n, &[0]).is_err());
        assert!(PublicKey::from_n_e(&n, &[1]).is_err());
        assert!(PublicKey::from_n_e(&n, &[2]).is_err());
        assert!(PublicKey::from_n_e(&n, &[4]).is_err());

        // Empty, zero, even, and below-minimum moduli are rejected.
        assert!(PublicKey::from_n_e(&[], &e).is_err());
        assert!(PublicKey::from_n_e(&[0], &e).is_err());
        assert!(PublicKey::from_n_e(&[2], &e).is_err());
        let mut small = n[..128].to_vec();
        small[127] |= 1;
        assert!(PublicKey::from_n_e(&small, &e).is_err());

        // The exponent must be strictly smaller than the modulus.
        assert!(PublicKey::from_n_e(&n, &n).is_err());
    }

    /// Encode a DER length using the minimal form (test helper).
    fn der_length(len: usize) -> Vec<u8> {
        if len < 0x80 {
            vec![len as u8]
        } else if len <= 0xff {
            vec![0x81, len as u8]
        } else {
            vec![0x82, (len >> 8) as u8, (len & 0xff) as u8]
        }
    }

    /// Encode a non-negative big-endian magnitude as a strict DER INTEGER
    /// (test helper).
    fn der_integer(value: &[u8]) -> Vec<u8> {
        let mut start = 0;
        while start + 1 < value.len() && value[start] == 0 {
            start += 1;
        }
        let value = &value[start..];
        let mut out = vec![0x02];
        if value[0] & 0x80 != 0 {
            out.extend(der_length(value.len() + 1));
            out.push(0x00);
        } else {
            out.extend(der_length(value.len()));
        }
        out.extend_from_slice(value);
        out
    }

    /// Wrap a body in a strict DER SEQUENCE (test helper).
    fn der_sequence(body: &[u8]) -> Vec<u8> {
        let mut out = vec![0x30];
        out.extend(der_length(body.len()));
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn read_length_requires_minimal_der() {
        assert_eq!(read_length(&[0x05]).unwrap(), (5, 1));
        assert_eq!(read_length(&[0x81, 0x80]).unwrap(), (128, 2));
        assert_eq!(read_length(&[0x82, 0x01, 0x00]).unwrap(), (256, 3));

        // Indefinite length is forbidden in DER.
        assert!(read_length(&[0x80, 0x00]).is_err());
        // Long form with a leading zero byte.
        assert!(read_length(&[0x82, 0x00, 0x80]).is_err());
        // Long form used for a value that fits in the short form.
        assert!(read_length(&[0x81, 0x05]).is_err());
        assert!(read_length(&[0x81, 0x7f]).is_err());
        // Truncated and empty inputs.
        assert!(read_length(&[0x82, 0x01]).is_err());
        assert!(read_length(&[]).is_err());
    }

    #[test]
    fn read_integer_requires_minimal_der() {
        let mut out = [0u8; 8];

        // Minimal positive INTEGER, e = 65537.
        assert_eq!(read_integer(&[0x02, 0x03, 0x01, 0x00, 0x01], &mut out).unwrap(), (3, 5));
        assert_eq!(&out[..3], &[0x01, 0x00, 0x01]);

        // A leading zero is required when the top bit of the value is set.
        assert_eq!(read_integer(&[0x02, 0x02, 0x00, 0x80], &mut out).unwrap(), (1, 4));
        assert_eq!(out[0], 0x80);

        // Negative INTEGER (top bit set without a sign byte).
        assert!(read_integer(&[0x02, 0x01, 0x80], &mut out).is_err());
        // Superfluous leading zero.
        assert!(read_integer(&[0x02, 0x02, 0x00, 0x7f], &mut out).is_err());
        // Non-minimal length.
        assert!(read_integer(&[0x02, 0x81, 0x01, 0x05], &mut out).is_err());
        // Empty and truncated values, and the wrong tag.
        assert!(read_integer(&[0x02, 0x00], &mut out).is_err());
        assert!(read_integer(&[0x02, 0x02, 0x01], &mut out).is_err());
        assert!(read_integer(&[0x04, 0x01, 0x01], &mut out).is_err());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_pkcs1_der_is_strict() {
        // 2048-bit odd modulus (top bit set, so DER carries a 0x00 sign byte)
        // with e = 65537.
        let mut n = [0x80u8; 256];
        n[255] |= 1;
        let e = [0x01u8, 0x00, 0x01];

        let mut body = der_integer(&n);
        body.extend(der_integer(&e));
        let der = der_sequence(&body);

        let key = PublicKey::from_pkcs1_der(&der).unwrap();
        assert_eq!(key.n_bytes(), n.to_vec());
        assert_eq!(key.e_bytes().as_slice(), &e);

        // Trailing byte inside the SEQUENCE after `e`.
        let mut body_trailing = body.clone();
        body_trailing.push(0x00);
        assert!(PublicKey::from_pkcs1_der(&der_sequence(&body_trailing)).is_err());

        // Trailing byte after the outer SEQUENCE.
        let mut der_trailing = der.clone();
        der_trailing.push(0x00);
        assert!(PublicKey::from_pkcs1_der(&der_trailing).is_err());

        // Non-minimal outer SEQUENCE length (`83 00 01 0A` instead of `82 01 0A`).
        let mut non_minimal = vec![0x30, 0x83, 0x00];
        non_minimal.extend_from_slice(&(body.len() as u16).to_be_bytes());
        non_minimal.extend_from_slice(&body);
        assert!(PublicKey::from_pkcs1_der(&non_minimal).is_err());

        // Non-minimal INTEGER length for `e`.
        let mut body_bad_e_len = der_integer(&n);
        body_bad_e_len.extend_from_slice(&[0x02, 0x81, 0x03, 0x01, 0x00, 0x01]);
        assert!(PublicKey::from_pkcs1_der(&der_sequence(&body_bad_e_len)).is_err());

        // Negative `e` (top bit set without a sign byte).
        let mut body_neg_e = der_integer(&n);
        body_neg_e.extend_from_slice(&[0x02, 0x01, 0x80]);
        assert!(PublicKey::from_pkcs1_der(&der_sequence(&body_neg_e)).is_err());
    }

    #[test]
    fn pss_non_full_length_modulus() {
        // Regression for L-03. This modulus is 2049 bits (top byte 0x01): it is
        // 257 bytes long, but RFC 8017 sets emBits = bitlen(n) - 1 = 2048 and
        // emLen = 256, with no leftmost bits cleared from DB. Deriving
        // emBits/emLen from the byte length instead (emBits = 2055,
        // emLen = 257) rejected this valid signature.
        let n = hex::decode(
            "0164172bd317d5b8c209ea97405faad6a4ddb71dcf1233600f52d902ce3194409fe87e3efb35ea5c3d3dad4b224112105fa999fa26a44b5ed8d1bd3a6db9f8661f5cdd519805cb188ae89883799a0f82ae0f024d6faacaae8eecce96a9963d776779dfce1f2b43a012be154f321edb881401e062b985f2c2f6c69a6d08492e648a4f8084ccac74d29088a0baad787dd14b307c02f545f50662e08de2f0a6a52b5d400bb0cb48b1dec3c33c84bfc40021ac760d9d3be5229796f2a676408774e7c4f27492bce1731d51d58f56fd86e4c0b90134f115c94a0ab42aef65fbce2e4ea92e0d11937f4f612142fc888d3b040c8052026293b89b2d718cf85cf881dfc603",
        ).unwrap();
        let e = hex::decode("010001").unwrap();
        let sig = hex::decode(
            "0100156ede19d636555868311341b128ce65ec53c27f3e7dbfd35319e4356d93e9b7b968dca073ecc6caa1c0519fa89ff0cab4e49a21b0a37311ddca42de06468827880a574b96d83a572a0867ca1bed49644275ae8c3a9860377135728a3d6ee2b07625be2f434ca63fa9341fefccfbe9d9a356da673e40dbb2e87efa2a8ef271bc1aaee7a7e4443e39e14ff3def38a7987d5fa2bf6453c6dedb3a376c58d259ffae0e337f6d6a447ada7db55dabb6208e38e0d86280eeb2802da551e15ad4b52cd18d4dbe146f87349ba52332a2e8baac6c3d1606b97b625d4fb2d958643c7bb57319869366b91900deb1e72df47e92a6e1bbd38c5dfe66d359d283dde248b27",
        ).unwrap();
        let msg = b"stdx RSA-PSS L-03 regression vector";

        let key = PublicKey::from_n_e(&n, &e).unwrap();
        assert_eq!(key.n.bit_len(), 2049);
        key.verify_pss::<crate::sha2::Sha256>(&sig, msg, 32).unwrap();

        // A modified message must not verify.
        let mut tampered = msg.to_vec();
        tampered[0] ^= 0x01;
        assert!(key.verify_pss::<crate::sha2::Sha256>(&sig, &tampered, 32).is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pss_2048_sha256() {
        wycheproof_rsa_pss_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_pss_2048_sha256_mgf1_32_test.json",
            crate::sha2::Sha256,
            32
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pss_2048_sha384() {
        wycheproof_rsa_pss_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_pss_2048_sha384_mgf1_48_test.json",
            crate::sha2::Sha384,
            48
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pss_3072_sha256() {
        wycheproof_rsa_pss_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_pss_3072_sha256_mgf1_32_test.json",
            crate::sha2::Sha256,
            32
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pss_4096_sha512() {
        wycheproof_rsa_pss_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_pss_4096_sha512_mgf1_64_test.json",
            crate::sha2::Sha512,
            64
        );
    }

    #[test]
    fn rsa_pkcs1_sha256_verify_works() {
        let pkcs1_der = hex::decode(
            "3082010a0282010100b1d59f746650c6a4360d26dc2e05581e1bd12cddcfc459a75dd2ef6d38cb6e977c72cad72f5e8ad4795484211e71e9a292d25a3901fca4cd242649f56cce50ad6ba148658d71f3a9c8b39e92a7a49543243df8ca2688292d47ff2a92a6ee0c9151162936791f522afccd6a7508251934b909d62fa805bae0d79f83f3c981b39c15ea79ce7b4ec2ff82240ce2a9fb93ae49d7697d1248f73d4ad23461055f469a3936ab959a0c6a067aa19521650f3649a028e2ebe355909aae7c95d3fc988684478b2bb11b307cb58c6c14727e1b62103d400ac8eed0e0d6d7f7d7cfc1f4ae4cbd9759372f8408c52174abb05f134ca6788fb60ba3f35c57c07cd44011bb113b0203010001",
        ).unwrap();
        let sig = hex::decode(
            "9d00f18defaa95b474b06ac4674b1b9270e110c6f474ce29e3aa972eca09137c9a82267e634986ecd54734f2edb1b3d72b539b8608e23074898c56042f9f014bfff59abce81c57d606b60f80ae4e110fc6f9dea99ce2897ce1d90661ab3d3b3f1a5ddf258b920a51c8c8758ab2da3da20da99c84eb2f57859b36918447c4cdbfa16cc09523fd27d28d4e97fa9ff0ea4d633c937a904a196a64e934851ee02b7922a8f5a4534bb10b8e16b89c12ddc347d7b4317f8b9d3dfed07a442d47351b18db38f45cc92e5c577b866df21766094d1f737ea418852827be3aec10d3c5a65a40087d9647b91a4d9419ad784a31caf02254cfc01a682bb6f5a231307f0fc8d9",
        ).unwrap();
        let digest = hex::decode("41cb0773387b187c038b7015498534c11369f1cfd094a714f4f39cf63ebb42ba").unwrap();

        let key = PublicKey::from_pkcs1_der(&pkcs1_der).unwrap();
        key.verify_pkcs1_v1_5(&sig, &digest, DIGEST_INFO_SHA256_PREFIX).unwrap();
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_2048_sha256() {
        use crate::sha2::Sha256;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_2048_sha256_test.json",
            Sha256,
            DIGEST_INFO_SHA256_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_2048_sha384() {
        use crate::sha2::Sha384;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_2048_sha384_test.json",
            Sha384,
            DIGEST_INFO_SHA384_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_2048_sha512() {
        use crate::sha2::Sha512;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_2048_sha512_test.json",
            Sha512,
            DIGEST_INFO_SHA512_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_3072_sha256() {
        use crate::sha2::Sha256;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_3072_sha256_test.json",
            Sha256,
            DIGEST_INFO_SHA256_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_3072_sha384() {
        use crate::sha2::Sha384;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_3072_sha384_test.json",
            Sha384,
            DIGEST_INFO_SHA384_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_3072_sha512() {
        use crate::sha2::Sha512;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_3072_sha512_test.json",
            Sha512,
            DIGEST_INFO_SHA512_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_4096_sha256() {
        use crate::sha2::Sha256;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_4096_sha256_test.json",
            Sha256,
            DIGEST_INFO_SHA256_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_4096_sha384() {
        use crate::sha2::Sha384;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_4096_sha384_test.json",
            Sha384,
            DIGEST_INFO_SHA384_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_4096_sha512() {
        use crate::sha2::Sha512;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_4096_sha512_test.json",
            Sha512,
            DIGEST_INFO_SHA512_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_8192_sha256() {
        use crate::sha2::Sha256;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_8192_sha256_test.json",
            Sha256,
            DIGEST_INFO_SHA256_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_8192_sha384() {
        use crate::sha2::Sha384;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_8192_sha384_test.json",
            Sha384,
            DIGEST_INFO_SHA384_PREFIX
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pkcs1_8192_sha512() {
        use crate::sha2::Sha512;
        wycheproof_rsa_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_signature_8192_sha512_test.json",
            Sha512,
            DIGEST_INFO_SHA512_PREFIX
        );
    }
}
