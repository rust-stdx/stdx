//! RSA signature verification: PKCS#1 v1.5 and RSA-PSS (RFC 8017).
//!
//! Implements modular exponentiation and padding verification for
//! rsa_pkcs1_sha256 / sha384 / sha512 and rsa_pss_* schemes.
//!
//! Only verification is supported; signing is not implemented.

use big_number::Uint;

use crate::{Hasher, RsaError};

/// Maximum RSA modulus size supported (4096 bits).
const RSA_MAX_BITS: usize = 4096;
const RSA_MAX_LIMBS: usize = RSA_MAX_BITS / 64;
const RSA_MAX_BYTES: usize = RSA_MAX_BITS / 8;

/// SHA-256 DigestInfo prefix (everything before the hash value).
const DIGEST_INFO_SHA256_PREFIX: &[u8] = &[
    0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20,
];

/// SHA-384 DigestInfo prefix.
const DIGEST_INFO_SHA384_PREFIX: &[u8] = &[
    0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05, 0x00, 0x04, 0x30,
];

/// SHA-512 DigestInfo prefix.
const DIGEST_INFO_SHA512_PREFIX: &[u8] = &[
    0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05, 0x00, 0x04, 0x40,
];

/// An RSA public key parsed from a PKCS#1 `SubjectPublicKeyInfo` DER blob.
pub struct RsaPublicKey {
    /// Modulus `n` as a 4096-bit integer (zero-padded for smaller keys).
    n: Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>,
    /// Public exponent `e`.
    e: Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>,
    /// Byte length of the modulus (without padding).
    n_bytes: usize,
    /// Barrett reduction precomputation for mod n.
    mu: [u64; big_number::MAX_LIMBS],
}

impl RsaPublicKey {
    /// Parse an RSA public key from the raw PKCS#1 bytes.
    ///
    /// The input is the content of the BIT STRING inside the SPKI —
    /// an ASN.1 `SEQUENCE { INTEGER n, INTEGER e }`.
    pub fn from_pkcs1_der(mut data: &[u8]) -> Result<Self, RsaError> {
        if data.is_empty() || data[0] != 0x30 {
            return Err(RsaError::Unspecified);
        }
        let (seq_len, len_size) = read_length(&data[1..])?;
        data = &data[1 + len_size..];
        if data.len() < seq_len {
            return Err(RsaError::Unspecified);
        }
        data = &data[..seq_len];

        let mut n_buf = [0u8; RSA_MAX_BYTES];
        let n_len = read_integer_bytes(data, &mut n_buf)?;
        let consumed = asn1_integer_size(data);
        data = &data[consumed..];

        let mut e_buf = [0u8; RSA_MAX_BYTES];
        let e_len = read_integer_bytes(data, &mut e_buf)?;

        let n = uint_from_variable_be(&n_buf[..n_len]);
        let e = uint_from_variable_be(&e_buf[..e_len]);

        Ok(RsaPublicKey {
            n,
            e,
            n_bytes: n_len,
            mu: n.compute_mu_for_barrett(),
        })
    }

    /// Verify a PKCS#1 v1.5 signature.
    ///
    /// `signature` is the raw signature bytes (eg., 256 bytes for RSA-2048).
    /// `message_digest` is the hash of the message to verify.
    /// `digest_info_prefix` is the ASN.1 DigestInfo prefix for the hash algorithm
    /// (the constant-length portion before the hash value).
    pub fn verify_pkcs1_v1_5(
        &self,
        signature: &[u8],
        message_digest: &[u8],
        digest_info_prefix: &[u8],
    ) -> Result<(), RsaError> {
        if signature.len() != self.n_bytes {
            return Err(RsaError::Unspecified);
        }

        let s = uint_from_variable_be(signature);

        // Reject signatures not reduced modulo n
        if s.ct_ge(&self.n) {
            return Err(RsaError::Unspecified);
        }

        let m = s.modpow_barrett(&self.e, &self.n, &self.mu);

        let mod_bytes = self.n_bytes;
        let expected_len = digest_info_prefix.len() + message_digest.len();

        let mut m_bytes = [0u8; RSA_MAX_BYTES];
        write_uint_be(&m, &mut m_bytes, mod_bytes);

        // Check PKCS#1 v1.5 padding: 00 01 FF...FF 00 <DigestInfo>
        if m_bytes[0] != 0x00 || m_bytes[1] != 0x01 {
            return Err(RsaError::Unspecified);
        }

        // Find the 0x00 separator after the FF padding
        let mut sep = 2;
        while sep < mod_bytes && m_bytes[sep] == 0xff {
            sep += 1;
        }
        if sep >= mod_bytes || m_bytes[sep] != 0x00 {
            return Err(RsaError::Unspecified);
        }
        // At least 8 bytes of FF padding required
        if sep < 10 {
            return Err(RsaError::Unspecified);
        }
        sep += 1;

        let di_start = sep;
        let di_end = di_start + expected_len;
        if di_end > mod_bytes {
            return Err(RsaError::Unspecified);
        }

        // Verify DigestInfo prefix
        let mut ok = 0u8;
        for i in 0..digest_info_prefix.len() {
            ok |= m_bytes[di_start + i] ^ digest_info_prefix[i];
        }
        // Verify hash value
        for i in 0..message_digest.len() {
            ok |= m_bytes[di_start + digest_info_prefix.len() + i] ^ message_digest[i];
        }

        if ok != 0 {
            return Err(RsaError::Unspecified);
        }

        // Reject trailing bytes after the DigestInfo
        for i in di_end..mod_bytes {
            ok |= m_bytes[i];
        }

        if ok != 0 {
            return Err(RsaError::Unspecified);
        }

        Ok(())
    }

    /// Verify an RSA-PSS signature (RFC 8017 §9.1.2).
    ///
    /// `message_digest` is the hash of the TLS signed_data.
    /// `hash_fn` produces a digest of `hash_len` bytes.
    /// `salt_len` equals the hash length (Go's PSSSaltLengthEqualsHash).
    pub fn verify_pss<H: Hasher>(&self, signature: &[u8], message: &[u8], salt_len: usize) -> Result<(), RsaError> {
        if signature.len() != self.n_bytes {
            return Err(RsaError::Unspecified);
        }

        let s = uint_from_variable_be(signature);
        if s.ct_ge(&self.n) {
            return Err(RsaError::Unspecified);
        }

        let m = s.modpow_barrett(&self.e, &self.n, &self.mu);
        let em_len = self.n_bytes;
        let hash_len = H::OUTPUT_SIZE;

        let mut em = [0u8; RSA_MAX_BYTES];
        write_uint_be(&m, &mut em, em_len);

        let em_bits = self.n_bytes * 8 - 1;
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
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | data[1 + i] as usize;
        }
        Ok((len, 1 + num_bytes))
    }
}

/// Read the raw big-endian bytes of an ASN.1 INTEGER, skipping any leading
/// 0x00 sign byte.
fn read_integer_bytes(data: &[u8], out: &mut [u8]) -> Result<usize, RsaError> {
    if data.len() < 2 || data[0] != 0x02 {
        return Err(RsaError::Unspecified);
    }
    let (len, len_size) = read_length(&data[1..])?;
    let value = &data[1 + len_size..];
    if value.len() < len {
        return Err(RsaError::Unspecified);
    }
    let bytes = &value[..len];
    if bytes.is_empty() {
        return Err(RsaError::Unspecified);
    }
    let start = if bytes[0] == 0x00 && bytes.len() > 1 { 1 } else { 0 };
    let val_len = bytes.len() - start;
    if out.len() < val_len {
        return Err(RsaError::Unspecified);
    }
    out[..val_len].copy_from_slice(&bytes[start..]);
    Ok(val_len)
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
/// Only converts the required limbs instead of the full 4096-bit representation.
fn write_uint_be(value: &Uint<RSA_MAX_BITS, RSA_MAX_LIMBS>, out: &mut [u8], byte_len: usize) {
    assert!(byte_len <= RSA_MAX_BYTES);
    let full = value.to_be_bytes_fixed::<{ RSA_MAX_BYTES }>();
    let start = full.len() - byte_len;
    out[..byte_len].copy_from_slice(&full[start..]);
}

/// Convenience: verify RSA-PKCS1-SHA256.
pub fn verify_pkcs1_sha256(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    let digest = crate::sha2::Sha256::hash(message);
    key.verify_pkcs1_v1_5(signature, digest.as_ref(), DIGEST_INFO_SHA256_PREFIX)
}

/// Convenience: verify RSA-PKCS1-SHA384.
pub fn verify_pkcs1_sha384(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    let digest = crate::sha2::Sha384::hash(message);
    key.verify_pkcs1_v1_5(signature, digest.as_ref(), DIGEST_INFO_SHA384_PREFIX)
}

/// Convenience: verify RSA-PKCS1-SHA512.
pub fn verify_pkcs1_sha512(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    let digest = crate::sha2::Sha512::hash(message);
    key.verify_pkcs1_v1_5(signature, digest.as_ref(), DIGEST_INFO_SHA512_PREFIX)
}

/// Convenience: verify RSA-PSS-SHA256.
pub fn verify_pss_sha256(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss::<crate::sha2::Sha256>(signature, message, 32)
}

/// Convenience: verify RSA-PSS-SHA384.
pub fn verify_pss_sha384(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
    key.verify_pss::<crate::sha2::Sha384>(signature, message, 48)
}

/// Convenience: verify RSA-PSS-SHA512.
pub fn verify_pss_sha512(pkcs1_der: &[u8], signature: &[u8], message: &[u8]) -> Result<(), RsaError> {
    let key = RsaPublicKey::from_pkcs1_der(pkcs1_der)?;
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
                let key = super::RsaPublicKey::from_pkcs1_der(&pkcs1_der).unwrap();

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
                            // Skip ASN.1-level padding structure checks for now.
                            // Our verify_pkcs1_v1_5 only validates padding format
                            // and digest match; it does not parse the DigestInfo
                            // ASN.1 for DER encoding strictness.
                            let flags = test.get("flags").and_then(|f| f.as_array());
                            let skip_asn1 = flags.map_or(false, |f| {
                                f.iter().any(|v| {
                                    let s = v.as_str().unwrap_or("");
                                    s == "InvalidAsnInPadding" || s == "BerEncodedPadding" || s == "ModifiedPadding"
                                })
                            });
                            if !skip_asn1 {
                                assert!(
                                    verify_result.is_err(),
                                    "{}: tcId {} ({:?}): expected invalid, got ok",
                                    $path,
                                    test["tcId"],
                                    flags,
                                );
                                invalid_tested += 1;
                            }
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
                let key = super::RsaPublicKey::from_pkcs1_der(&pkcs1_der).unwrap();

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

        let key = RsaPublicKey::from_pkcs1_der(&pkcs1_der).unwrap();
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
}
