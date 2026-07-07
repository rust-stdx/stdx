//! HMAC (Hash-based Message Authentication Code)

use crate::{Hash, Hasher, HkdfError, hmac::Hmac};

const DEFAULT_SALT: [u8; 64] = [0u8; 64];

/// HKDF extract step: `PRK = HMAC-Hash(salt, IKM)`.
///
/// If `salt` is `None`, a string of `H::OUTPUT_SIZE` zero bytes is used.
///
/// # Example
///
/// ```ignore
/// use crypto::hkdf;
/// use crypto::sha2::Sha256;
///
/// let prk = hkdf::extract::<Sha256>(Some(b"salt"), b"input key material");
/// ```
pub fn extract<H: Hasher>(salt: Option<&[u8]>, ikm: &[u8]) -> Hash {
    let salt = salt.unwrap_or(&DEFAULT_SALT[..H::OUTPUT_SIZE]);
    let mut mac = Hmac::<H>::new(salt);
    mac.update(ikm);
    return mac.finalize();
}

/// HKDF expand step: `OKM = T(1) || T(2) || ...`, where
/// `T(i) = HMAC-Hash(PRK, T(i-1) || info || i)`.
///
/// The output is written into `okm`. The length of the output is determined by
/// `okm.len()`.
///
/// # Example
///
/// ```ignore
/// use crypto::hkdf;
/// use crypto::sha2::Sha256;
///
/// let prk = hkdf::extract::<Sha256>(Some(b"salt"), b"input key material");
/// let mut okm = [0u8; 32];
/// hkdf::expand::<Sha256>(&mut okm, &prk, b"context info").unwrap();
/// ```
///
/// # Error
///
/// Returns an error if `okm.len() > 255 * H::OUTPUT_SIZE` or if `prk.len() < H::OUTPUT_SIZE`.
pub fn expand<H: Hasher>(okm: &mut [u8], prk: &[u8], info: &[u8]) -> Result<(), HkdfError> {
    let n = okm.len();

    if prk.len() < H::OUTPUT_SIZE {
        return Err(HkdfError::PrkIsTooShort(H::OUTPUT_SIZE));
    }

    if n > 255 * H::OUTPUT_SIZE {
        return Err(HkdfError::OutputIsTooLong);
    }

    if n == 0 {
        return Ok(());
    }

    let mut t = [0u8; 64];
    let mut t_len = 0usize;
    let mut offset = 0usize;
    let mut counter = 1u8;

    while offset < n {
        let mut mac = Hmac::<H>::new(&prk[..H::OUTPUT_SIZE]);
        mac.update(&t[..t_len]);
        mac.update(info);
        mac.update(&[counter]);
        let block = mac.finalize();
        let block_bytes = block.as_ref();
        let chunk_len = (n - offset).min(H::OUTPUT_SIZE);
        okm[offset..offset + chunk_len].copy_from_slice(&block_bytes[..chunk_len]);
        t[..H::OUTPUT_SIZE].copy_from_slice(block_bytes);
        t_len = H::OUTPUT_SIZE;
        offset += chunk_len;
        counter = counter.wrapping_add(1);
    }

    Ok(())
}

/// One-shot HKDF: extract-then-expand in a single call.
///
/// # Example
///
/// ```ignore
/// use crypto::hkdf;
/// use crypto::sha2::Sha256;
///
/// let okm: [u8; 32] = hkdf::derive_key::<Sha256, 32>(
///     b"input key material",
///     b"context info",
///     Some(b"salt"),
/// ).unwrap();
/// ```
///
/// # Error
///
/// Returns an error if `N > 255 * H::OUTPUT_SIZE`.
pub fn derive_key<H: Hasher, const N: usize>(
    ikm: &[u8],
    info: &[u8],
    salt: Option<&[u8]>,
) -> Result<[u8; N], HkdfError> {
    let prk = extract::<H>(salt, ikm);
    let mut okm = [0u8; N];
    expand::<H>(&mut okm, prk.as_ref(), info)?;
    Ok(okm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sha2::{Sha256, Sha384, Sha512};

    struct TestVector {
        ikm: &'static str,
        salt: Option<&'static str>,
        info: &'static str,
        expected_prk: &'static str,
        expected_okm: &'static str,
    }

    fn decode_hex(input: &str) -> Vec<u8> {
        let input = input.replace(|c: char| c.is_whitespace(), "");
        (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
            .collect()
    }

    const SHA256_VECTORS: [TestVector; 4] = [
        TestVector {
            ikm: "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
            salt: Some("000102030405060708090a0b0c"),
            info: "f0f1f2f3f4f5f6f7f8f9",
            expected_prk: "077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5",
            expected_okm: "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865",
        },
        TestVector {
            ikm: "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\
                  202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f\
                  404142434445464748494a4b4c4d4e4f",
            salt: Some(
                "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f\
                 808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f\
                 a0a1a2a3a4a5a6a7a8a9aaabacadaeaf",
            ),
            info: "b0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecf\
                  d0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeef\
                  f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
            expected_prk: "06a6b88c5853361a06104c9ceb35b45cef760014904671014a193f40c15fc244",
            expected_okm: "b11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c59045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71cc30c58179ec3e87c14c01d5c1f3434f1d87",
        },
        TestVector {
            ikm: "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
            salt: Some(""),
            info: "",
            expected_prk: "19ef24a32c717b167f33a91d6f648bdf96596776afdb6377ac434c1c293ccb04",
            expected_okm: "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8",
        },
        TestVector {
            ikm: "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
            salt: None,
            info: "",
            expected_prk: "19ef24a32c717b167f33a91d6f648bdf96596776afdb6377ac434c1c293ccb04",
            expected_okm: "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8",
        },
    ];

    const SHA512_VECTORS: [TestVector; 4] = [
        TestVector {
            ikm: "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
            salt: Some("000102030405060708090a0b0c"),
            info: "f0f1f2f3f4f5f6f7f8f9",
            expected_prk: "665799823737ded04a88e47e54a5890bb2c3d247c7a4254a8e61350723590a26c36238127d8661b88cf80ef802d57e2f7cebcf1e00e083848be19929c61b4237",
            expected_okm: "832390086cda71fb47625bb5ceb168e4c8e26a1a16ed34d9fc7fe92c1481579338da362cb8d9f925d7cb",
        },
        TestVector {
            ikm: "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\
                  202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f\
                  404142434445464748494a4b4c4d4e4f",
            salt: Some(
                "606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f\
                 808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f\
                 a0a1a2a3a4a5a6a7a8a9aaabacadaeaf",
            ),
            info: "b0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecf\
                  d0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeef\
                  f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
            expected_prk: "35672542907d4e142c00e84499e74e1de08be86535f924e022804ad775dde27ec86cd1e5b7d178c74489bdbeb30712beb82d4f97416c5a94ea81ebdf3e629e4a",
            expected_okm: "ce6c97192805b346e6161e821ed165673b84f400a2b514b2fe23d84cd189ddf1b695b48cbd1c8388441137b3ce28f16aa64ba33ba466b24df6cfcb021ecff235f6a2056ce3af1de44d572097a8505d9e7a93",
        },
        TestVector {
            ikm: "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
            salt: Some(""),
            info: "",
            expected_prk: "fd200c4987ac491313bd4a2a13287121247239e11c9ef82802044b66ef357e5b194498d0682611382348572a7b1611de54764094286320578a863f36562b0df6",
            expected_okm: "f5fa02b18298a72a8c23898a8703472c6eb179dc204c03425c970e3b164bf90fff22d04836d0e2343bac",
        },
        TestVector {
            ikm: "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
            salt: None,
            info: "",
            expected_prk: "fd200c4987ac491313bd4a2a13287121247239e11c9ef82802044b66ef357e5b194498d0682611382348572a7b1611de54764094286320578a863f36562b0df6",
            expected_okm: "f5fa02b18298a72a8c23898a8703472c6eb179dc204c03425c970e3b164bf90fff22d04836d0e2343bac",
        },
    ];

    #[test]
    fn hkdf_sha256_vectors() {
        for (i, vector) in SHA256_VECTORS.iter().enumerate() {
            let ikm = decode_hex(vector.ikm);
            let salt = vector.salt.map(decode_hex);
            let info = decode_hex(vector.info);
            let expected_prk = decode_hex(vector.expected_prk);
            let expected_okm = decode_hex(vector.expected_okm);

            let prk = extract::<Sha256>(salt.as_deref(), &ikm);
            assert_eq!(prk.as_ref(), expected_prk.as_slice(), "vector {} PRK", i);

            let mut buf = [0u8; 82];
            match expected_okm.len() {
                42 => expand::<Sha256>(&mut buf[..42], prk.as_ref(), &info).unwrap(),
                82 => expand::<Sha256>(&mut buf[..82], prk.as_ref(), &info).unwrap(),
                _ => unreachable!(),
            };
            assert_eq!(&buf[..expected_okm.len()], expected_okm, "vector {} OKM", i);

            let derived = match expected_okm.len() {
                42 => derive_key::<Sha256, 42>(&ikm, &info, salt.as_deref()).unwrap().to_vec(),
                82 => derive_key::<Sha256, 82>(&ikm, &info, salt.as_deref()).unwrap().to_vec(),
                _ => unreachable!(),
            };
            assert_eq!(derived, expected_okm, "vector {} derive_key OKM", i);
        }
    }

    #[test]
    fn hkdf_sha512_vectors() {
        for (i, vector) in SHA512_VECTORS.iter().enumerate() {
            let ikm = decode_hex(vector.ikm);
            let salt = vector.salt.map(decode_hex);
            let info = decode_hex(vector.info);
            let expected_prk = decode_hex(vector.expected_prk);
            let expected_okm = decode_hex(vector.expected_okm);

            let prk = extract::<Sha512>(salt.as_deref(), &ikm);
            assert_eq!(prk.as_ref(), expected_prk.as_slice(), "vector {} PRK", i);

            let mut buf = [0u8; 82];
            match expected_okm.len() {
                42 => expand::<Sha512>(&mut buf[..42], prk.as_ref(), &info).unwrap(),
                82 => expand::<Sha512>(&mut buf[..82], prk.as_ref(), &info).unwrap(),
                _ => unreachable!(),
            };
            assert_eq!(&buf[..expected_okm.len()], expected_okm, "vector {} OKM", i);

            let derived = match expected_okm.len() {
                42 => derive_key::<Sha512, 42>(&ikm, &info, salt.as_deref()).unwrap().to_vec(),
                82 => derive_key::<Sha512, 82>(&ikm, &info, salt.as_deref()).unwrap().to_vec(),
                _ => unreachable!(),
            };
            assert_eq!(derived, expected_okm, "vector {} derive_key OKM", i);
        }
    }

    #[test]
    fn hkdf_zero_length_output() {
        let prk = [0u8; 32];
        let mut buf = [0u8; 0];
        assert_eq!(expand::<Sha256>(&mut buf, &prk, b""), Ok(()));
        assert_eq!(derive_key::<Sha256, 0>(b"ikm", b"info", None).unwrap(), [] as [u8; 0]);
    }

    #[test]
    fn hkdf_expand_panics_when_output_is_too_large() {
        let prk = [0u8; 32];
        let mut buf = vec![0u8; Sha256::BLOCK_SIZE * 300];
        assert_eq!(expand::<Sha256>(&mut buf, &prk, b""), Err(HkdfError::OutputIsTooLong));
    }

    #[test]
    fn hkdf_expand_panics_when_prk_is_too_short() {
        let mut buf = [0u8; 32];
        assert_eq!(
            expand::<Sha256>(&mut buf, &[0u8; 31], b""),
            Err(HkdfError::PrkIsTooShort(Sha256::OUTPUT_SIZE))
        );
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn hkdf_sha256_wycheproof() {
        // Maximum valid HKDF-SHA-256 output: 255 * 32 = 8160 bytes.
        const MAX_OKM: usize = 8160;
        const SIZE_TOO_LARGE: usize = 8161;

        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hkdf_sha256_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let ikm_hex = test["ikm"].as_str().unwrap();
                let salt_hex = test["salt"].as_str().unwrap();
                let info_hex = test["info"].as_str().unwrap();
                let size = test["size"].as_u64().unwrap() as usize;
                let expected_okm_hex = test["okm"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let ikm = hex::decode(ikm_hex).unwrap();
                let info = hex::decode(info_hex).unwrap();
                let salt: Option<Vec<u8>> = if salt_hex.is_empty() {
                    None
                } else {
                    Some(hex::decode(salt_hex).unwrap())
                };

                if result == "valid" {
                    let okm = derive_key::<Sha256, MAX_OKM>(&ikm, &info, salt.as_deref()).unwrap();
                    let okm_hex = hex::encode(&okm[..size]);
                    assert_eq!(
                        okm_hex, expected_okm_hex,
                        "wycheproof HKDF-SHA-256 tcId={} size={}",
                        test["tcId"], size
                    );
                    valid_tested += 1;
                } else {
                    assert_eq!(
                        derive_key::<Sha256, SIZE_TOO_LARGE>(&ikm, &info, salt.as_deref()),
                        Err(HkdfError::OutputIsTooLong),
                        "wycheproof HKDF-SHA-256 tcId={} size={} should reject",
                        test["tcId"],
                        size
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HKDF-SHA-256 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HKDF-SHA-256 wycheproof tests were run");
    }

    #[test]
    fn hkdf_sha512_wycheproof() {
        // Maximum valid HKDF-SHA-512 output: 255 * 64 = 16320 bytes.
        const MAX_OKM: usize = 16320;
        const SIZE_TOO_LARGE: usize = 16321;

        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hkdf_sha512_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let ikm_hex = test["ikm"].as_str().unwrap();
                let salt_hex = test["salt"].as_str().unwrap();
                let info_hex = test["info"].as_str().unwrap();
                let size = test["size"].as_u64().unwrap() as usize;
                let expected_okm_hex = test["okm"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let ikm = hex::decode(ikm_hex).unwrap();
                let info = hex::decode(info_hex).unwrap();
                let salt: Option<Vec<u8>> = if salt_hex.is_empty() {
                    None
                } else {
                    Some(hex::decode(salt_hex).unwrap())
                };

                if result == "valid" {
                    let okm = derive_key::<Sha512, MAX_OKM>(&ikm, &info, salt.as_deref()).unwrap();
                    let okm_hex = hex::encode(&okm[..size]);
                    assert_eq!(
                        okm_hex, expected_okm_hex,
                        "wycheproof HKDF-SHA-512 tcId={} size={}",
                        test["tcId"], size
                    );
                    valid_tested += 1;
                } else {
                    assert_eq!(
                        derive_key::<Sha512, SIZE_TOO_LARGE>(&ikm, &info, salt.as_deref()),
                        Err(HkdfError::OutputIsTooLong),
                        "wycheproof HKDF-SHA-512 tcId={} size={} should reject",
                        test["tcId"],
                        size
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HKDF-SHA-512 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HKDF-SHA-512 wycheproof tests were run");
    }

    #[test]
    fn hkdf_sha384_wycheproof() {
        // Maximum valid HKDF-SHA-384 output: 255 * 48 = 12240 bytes.
        const MAX_OKM: usize = 12240;
        const SIZE_TOO_LARGE: usize = 12241;

        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hkdf_sha384_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let ikm_hex = test["ikm"].as_str().unwrap();
                let salt_hex = test["salt"].as_str().unwrap();
                let info_hex = test["info"].as_str().unwrap();
                let size = test["size"].as_u64().unwrap() as usize;
                let expected_okm_hex = test["okm"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let ikm = hex::decode(ikm_hex).unwrap();
                let info = hex::decode(info_hex).unwrap();
                let salt: Option<Vec<u8>> = if salt_hex.is_empty() {
                    None
                } else {
                    Some(hex::decode(salt_hex).unwrap())
                };

                if result == "valid" {
                    let okm = derive_key::<Sha384, MAX_OKM>(&ikm, &info, salt.as_deref()).unwrap();
                    let okm_hex = hex::encode(&okm[..size]);
                    assert_eq!(
                        okm_hex, expected_okm_hex,
                        "wycheproof HKDF-SHA-384 tcId={} size={}",
                        test["tcId"], size
                    );
                    valid_tested += 1;
                } else {
                    assert_eq!(
                        derive_key::<Sha384, SIZE_TOO_LARGE>(&ikm, &info, salt.as_deref()),
                        Err(HkdfError::OutputIsTooLong),
                        "wycheproof HKDF-SHA-384 tcId={} size={} should reject",
                        test["tcId"],
                        size
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HKDF-SHA-384 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HKDF-SHA-384 wycheproof tests were run");
    }
}
