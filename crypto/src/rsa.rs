//! RSA-PSS signature verification (RFC 8017 §9.1.2).
//!
//! Implements modular exponentiation and PSS padding verification for RSA.
//!
//! Only verification is supported; signing is not implemented.
//! Salt length equals hash length (Go's PSSSaltLengthEqualsHash convention).

use big_number::Uint;

use crate::{EllipticCurveError, Hasher};

/// Maximum RSA modulus size supported (4096 bits).
const RSA_MAX_BITS: usize = 4096;
const RSA_MAX_LIMBS: usize = RSA_MAX_BITS / 64;

/// An RSA public key parsed from a PKCS#1 `SubjectPublicKeyInfo` DER blob.
pub struct RsaPublicKey {
    /// Modulus `n` as a 4096-bit integer (zero-padded for smaller keys).
    n: Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>,
    /// Public exponent `e`.
    e: Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>,
    /// Byte length of the modulus (without padding).
    n_bytes: usize,
}

impl RsaPublicKey {
    /// Parse an RSA public key from the raw PKCS#1 bytes.
    ///
    /// The input is the content of the BIT STRING inside the SPKI —
    /// an ASN.1 `SEQUENCE { INTEGER n, INTEGER e }`.
    pub fn from_pkcs1_der(mut data: &[u8]) -> Result<Self, EllipticCurveError> {
        // Read SEQUENCE
        if data.is_empty() || data[0] != 0x30 {
            return Err(EllipticCurveError::Unspecified);
        }
        let (seq_len, len_size) = read_length(&data[1..])?;
        data = &data[1 + len_size..];
        if data.len() < seq_len {
            return Err(EllipticCurveError::Unspecified);
        }
        data = &data[..seq_len];

        // Read INTEGER: n
        let n_bytes = read_integer_bytes(data)?;
        let consumed = asn1_integer_size(data);
        data = &data[consumed..];

        // Read INTEGER: e
        let e_bytes = read_integer_bytes(data)?;

        // Parse n and e into Uint
        let n = uint_from_variable_be(&n_bytes);
        let e = uint_from_variable_be(&e_bytes);
        let n_bytes_len = n_bytes.len();

        Ok(RsaPublicKey {
            n,
            e,
            n_bytes: n_bytes_len,
        })
    }

    /// Verify an RSA-PSS signature (RFC 8017 §9.1.2).
    ///
    /// `message_digest` is the hash of the TLS signed_data.
    /// `hash_fn` produces a digest of `hash_len` bytes.
    /// `salt_len` equals the hash length (Go's PSSSaltLengthEqualsHash).
    pub fn verify_pss<F>(
        &self,
        signature: &[u8],
        message_digest: &[u8],
        hash_fn: F,
        hash_len: usize,
    ) -> Result<(), EllipticCurveError>
    where
        F: Fn(&[u8]) -> Vec<u8>,
    {
        if signature.len() != self.n_bytes {
            return Err(EllipticCurveError::Unspecified);
        }

        let s = uint_from_variable_be(signature);
        if s.ct_ge(&self.n) {
            return Err(EllipticCurveError::Unspecified);
        }

        let m = s.modpow(&self.e, &self.n);
        let em_len = self.n_bytes;

        let mut em = [0u8; RSA_MAX_BITS / 8];
        write_uint_be(&m, &mut em, em_len);

        // Check leftmost bits of EM are zero (RFC 8017 §9.1.2 step 6)
        let em_bits = self.n_bytes * 8 - 1;
        let leftmost_bits = 8 * em_len - em_bits;
        if leftmost_bits > 0 && leftmost_bits < 8 {
            if em[0] >> (8 - leftmost_bits) != 0 {
                return Err(EllipticCurveError::Unspecified);
            }
        }

        // EM = maskedDB || H || 0xBC
        let salt_len = hash_len;
        if em_len < hash_len + salt_len + 2 {
            return Err(EllipticCurveError::Unspecified);
        }
        if em[em_len - 1] != 0xBC {
            return Err(EllipticCurveError::Unspecified);
        }

        let masked_db_len = em_len - hash_len - 1;
        let masked_db = &em[..masked_db_len];
        let h = &em[masked_db_len..masked_db_len + hash_len];

        let db_mask = mgf1(h, &hash_fn, masked_db_len, hash_len);

        let mut db = [0u8; RSA_MAX_BITS / 8];
        for (i, (a, b)) in masked_db.iter().zip(db_mask.iter()).enumerate() {
            db[i] = a ^ b;
        }

        // Clear the leftmost bits of DB (must match EM's top bits, already zero)
        if leftmost_bits > 0 && leftmost_bits < 8 {
            db[0] &= 0xff >> leftmost_bits;
        }

        // DB = PS || 0x01 || salt
        let ps_len = em_len - hash_len - salt_len - 2;
        // PS must be all zeros
        if ps_len > 0 {
            for i in 0..ps_len {
                if db[i] != 0x00 {
                    return Err(EllipticCurveError::Unspecified);
                }
            }
        }
        if db[ps_len] != 0x01 {
            return Err(EllipticCurveError::Unspecified);
        }

        let salt = &db[ps_len + 1..ps_len + 1 + salt_len];

        // M' = 8 zero bytes || mHash || salt
        let mut mp = Vec::with_capacity(8 + hash_len + salt_len);
        mp.extend_from_slice(&[0u8; 8]);
        mp.extend_from_slice(message_digest);
        mp.extend_from_slice(salt);

        // H' = Hash(M')
        let hp = hash_fn(&mp);

        // Compare H == H' (constant-time)
        let mut ok = 0u8;
        for i in 0..hash_len {
            ok |= h[i] ^ hp[i];
        }
        if ok != 0 {
            return Err(EllipticCurveError::Unspecified);
        }

        Ok(())
    }
}

/// MGF1 (Mask Generation Function 1) per RFC 8017 Appendix B.2.1.
fn mgf1<F>(seed: &[u8], hash_fn: &F, out_len: usize, hash_len: usize) -> Vec<u8>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let mut out = Vec::with_capacity(out_len);
    let mut counter: u32 = 0;
    while out.len() < out_len {
        let mut input = Vec::with_capacity(seed.len() + 4);
        input.extend_from_slice(seed);
        input.extend_from_slice(&counter.to_be_bytes());
        let hash = hash_fn(&input);
        let take = (out_len - out.len()).min(hash_len);
        out.extend_from_slice(&hash[..take]);
        counter += 1;
    }
    out
}

/// Read an ASN.1 length field. Returns (value, bytes_consumed_for_length).
fn read_length(data: &[u8]) -> Result<(usize, usize), EllipticCurveError> {
    if data.is_empty() {
        return Err(EllipticCurveError::Unspecified);
    }
    if data[0] & 0x80 == 0 {
        Ok((data[0] as usize, 1))
    } else {
        let num_bytes = (data[0] & 0x7f) as usize;
        if num_bytes == 0 || num_bytes > 4 || data.len() < 1 + num_bytes {
            return Err(EllipticCurveError::Unspecified);
        }
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | data[1 + i] as usize;
        }
        Ok((len, 1 + num_bytes))
    }
}

/// Read the raw big-endian bytes of an ASN.1 INTEGER, skipping any leading
/// 0x00 sign byte.
fn read_integer_bytes(mut data: &[u8]) -> Result<Vec<u8>, EllipticCurveError> {
    if data.len() < 2 || data[0] != 0x02 {
        return Err(EllipticCurveError::Unspecified);
    }
    let (len, len_size) = read_length(&data[1..])?;
    data = &data[1 + len_size..];
    if data.len() < len {
        return Err(EllipticCurveError::Unspecified);
    }
    let bytes = &data[..len];
    if bytes.is_empty() {
        return Err(EllipticCurveError::Unspecified);
    }
    // Skip leading 0x00 (sign byte for positive numbers with MSB set)
    let start = if bytes[0] == 0x00 && bytes.len() > 1 { 1 } else { 0 };
    Ok(bytes[start..].to_vec())
}

/// Return the total byte size of an ASN.1 INTEGER (tag + length + value).
fn asn1_integer_size(data: &[u8]) -> usize {
    if data.len() < 2 || data[0] != 0x02 {
        return 0;
    }
    let (len, len_size) = read_length(&data[1..]).unwrap_or((0, 0));
    1 + len_size + len
}

/// Build a `Uint` from a variable-length big-endian byte slice
/// (left-padded with zeros to the full bit width).
fn uint_from_variable_be(bytes: &[u8]) -> Uint<RSA_MAX_BITS, RSA_MAX_LIMBS> {
    let max_bytes = RSA_MAX_BITS / 8;
    assert!(bytes.len() <= max_bytes);
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
    Uint::from_limbs(limbs)
}

/// Write a `Uint` as big-endian bytes into a buffer, right-aligned.
fn write_uint_be(value: &Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>, out: &mut [u8], byte_len: usize) {
    assert!(byte_len <= RSA_MAX_BITS / 8);
    let full = value.to_be_bytes_fixed::<{ RSA_MAX_BITS / 8 }>();
    let start = full.len() - byte_len;
    out[..byte_len].copy_from_slice(&full[start..]);
}

/// Convenience: verify RSA-PSS-SHA256.
pub fn verify_pss_sha256(pkcs1_der: &[u8], signature: &[u8], message_digest: &[u8]) -> Result<(), EllipticCurveError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss(
        signature,
        message_digest,
        |data| crate::sha2::Sha256::hash(data).as_ref().to_vec(),
        32,
    )
}

/// Convenience: verify RSA-PSS-SHA384.
pub fn verify_pss_sha384(pkcs1_der: &[u8], signature: &[u8], message_digest: &[u8]) -> Result<(), EllipticCurveError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss(
        signature,
        message_digest,
        |data| crate::sha2::Sha384::hash(data).as_ref().to_vec(),
        48,
    )
}

/// Convenience: verify RSA-PSS-SHA512.
pub fn verify_pss_sha512(pkcs1_der: &[u8], signature: &[u8], message_digest: &[u8]) -> Result<(), EllipticCurveError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss(
        signature,
        message_digest,
        |data| crate::sha2::Sha512::hash(data).as_ref().to_vec(),
        64,
    )
}

#[cfg(test)]
mod tests {
    use hex;

    use super::*;

    macro_rules! wycheproof_rsa_pss_test {
        ($path:expr, $hash_fn:expr, $hash_len:expr) => {{
            let data: serde_json::Value = serde_json::from_str(include_str!($path)).unwrap();
            let mut valid_tested = 0u64;
            let mut invalid_tested = 0u64;

            for group in data["testGroups"].as_array().unwrap() {
                let pkcs1_der = hex::decode(group["publicKeyAsn"].as_str().unwrap()).unwrap();
                let key = super::RsaPublicKey::from_pkcs1_der(&pkcs1_der).unwrap();

                for test in group["tests"].as_array().unwrap() {
                    let msg_hex = test["msg"].as_str().unwrap();
                    let sig_hex = test["sig"].as_str().unwrap();
                    let result = test["result"].as_str().unwrap();

                    let msg = hex::decode(msg_hex).unwrap();
                    let sig = hex::decode(sig_hex).unwrap();

                    let digest = ($hash_fn)(&msg);
                    let verify_result = key.verify_pss(&sig, &digest, $hash_fn, $hash_len);

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
        let base = uint_from_variable_be(&[3]);
        let exp = uint_from_variable_be(&[5]);
        let modulus = uint_from_variable_be(&[7]);
        let result = base.modpow(&exp, &modulus);
        let bytes = result.to_be_bytes_fixed::<512>();
        assert_eq!(bytes[511], 5, "3^5 mod 7 should be 5, got {}", bytes[511]);
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pss_2048_sha256() {
        wycheproof_rsa_pss_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_pss_2048_sha256_mgf1_32_test.json",
            |data| crate::sha2::Sha256::hash(data).as_ref().to_vec(),
            32
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pss_2048_sha384() {
        wycheproof_rsa_pss_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_pss_2048_sha384_mgf1_48_test.json",
            |data| crate::sha2::Sha384::hash(data).as_ref().to_vec(),
            48
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn wycheproof_rsa_pss_4096_sha512() {
        wycheproof_rsa_pss_test!(
            "../testdata/wycheproof/testvectors_v1/rsa_pss_4096_sha512_mgf1_64_test.json",
            |data| crate::sha2::Sha512::hash(data).as_ref().to_vec(),
            64
        );
    }
}
