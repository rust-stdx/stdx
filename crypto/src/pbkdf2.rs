//! PBKDF2 (Password-Based Key Derivation Function 2) as defined in RFC 2898.
use crate::{Hasher, MAX_HASH_BLOCK_SIZE, hmac::Hmac};

/// Derives a key using PBKDF2-HMAC with the given hash function.
///
/// PBKDF2 applies the HMAC-based PRF repeatedly (`iterations` times) to
/// produce a derived key of `N` bytes.
///
/// ⚠️ **PBKDF2 is not memory-hard**, making it vulnerable to GPU/ASIC-based
/// brute-force attacks. For password hashing, prefer [`crate::argon2::Argon2id`]
/// unless PBKDF2 is required for legacy compatibility or specific protocol
/// standards.
///
/// # Example
///
/// ```ignore
/// use crypto::pbkdf2;
/// use crypto::sha2::Sha256;
///
/// let key: [u8; 32] = pbkdf2::derive::<Sha256, 32>(
///     b"password",
///     b"salt",
///     4096,
/// );
/// ```
///
/// # Panics
///
/// `derive` panics if `iterations == 0` (iterations must be >= 1)
/// or `N > (2^32 - 1) * H::OUTPUT_SIZE` (output length exceeds RFC 2898 limit).
pub fn derive<H: Hasher, const N: usize>(password: &[u8], salt: &[u8], iterations: u32) -> [u8; N] {
    assert!(iterations != 0, "PBKDF2 iterations must be >= 1");
    const {
        let max_len: usize = if cfg!(target_pointer_width = "64") {
            (u32::MAX as usize) * H::OUTPUT_SIZE
        } else {
            usize::MAX
        };
        assert!(N <= max_len, "PBKDF2 output length exceeds RFC 2898 limit",);
    }

    let hlen = H::OUTPUT_SIZE;
    let block_count = (N + hlen - 1) / hlen;

    let mut okm = [0u8; N];
    let mut t = [0u8; MAX_HASH_BLOCK_SIZE];
    let mut u = [0u8; MAX_HASH_BLOCK_SIZE];

    for block in 1..=block_count {
        let mut mac = Hmac::<H>::new(password);
        mac.update(salt);
        mac.update(&(block as u32).to_be_bytes());
        let hash = mac.finalize();
        let hash_bytes = hash.as_ref();
        t[..hlen].copy_from_slice(hash_bytes);
        u[..hlen].copy_from_slice(hash_bytes);

        for _ in 2..=iterations {
            let mut mac = Hmac::<H>::new(password);
            mac.update(&u[..hlen]);
            let hash = mac.finalize();
            let hash_bytes = hash.as_ref();
            u[..hlen].copy_from_slice(hash_bytes);
            for i in 0..hlen {
                t[i] ^= u[i];
            }
        }

        let start = (block - 1) * hlen;
        let end = usize::min(start + hlen, N);
        okm[start..end].copy_from_slice(&t[..end - start]);
    }

    return okm;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sha2::{Sha256, Sha512};

    #[test]
    #[should_panic(expected = "PBKDF2 iterations must be >= 1")]
    fn pbkdf2_zero_iterations() {
        derive::<Sha256, 32>(b"password", b"salt", 0);
    }

    #[test]
    fn pbkdf2_zero_length_output() {
        assert_eq!(derive::<Sha256, 0>(b"password", b"salt", 1), [] as [u8; 0]);
    }

    #[test]
    fn pbkdf2_sha256_wycheproof() {
        const MAX_OUTPUT: usize = 128;

        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../testdata/wycheproof/testvectors_v1/pbkdf2_hmacsha256_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let password_hex = test["password"].as_str().unwrap();
                let salt_hex = test["salt"].as_str().unwrap();
                let iteration_count = test["iterationCount"].as_u64().unwrap() as u32;
                let dk_len = test["dkLen"].as_u64().unwrap() as usize;
                let expected_dk_hex = test["dk"].as_str().unwrap();

                let password = hex::decode(password_hex).unwrap();
                let salt = hex::decode(salt_hex).unwrap();

                let dk = derive::<Sha256, MAX_OUTPUT>(&password, &salt, iteration_count);
                let dk_hex = hex::encode(&dk[..dk_len]);
                assert_eq!(
                    dk_hex, expected_dk_hex,
                    "wycheproof PBKDF2-SHA-256 tcId={} dkLen={}",
                    test["tcId"], dk_len
                );
                tested += 1;
            }
        }
        assert!(tested > 0, "no PBKDF2-SHA-256 wycheproof tests were run");
    }

    #[test]
    fn pbkdf2_sha512_wycheproof() {
        const MAX_OUTPUT: usize = 128;

        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../testdata/wycheproof/testvectors_v1/pbkdf2_hmacsha512_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let password_hex = test["password"].as_str().unwrap();
                let salt_hex = test["salt"].as_str().unwrap();
                let iteration_count = test["iterationCount"].as_u64().unwrap() as u32;
                let dk_len = test["dkLen"].as_u64().unwrap() as usize;
                let expected_dk_hex = test["dk"].as_str().unwrap();

                let password = hex::decode(password_hex).unwrap();
                let salt = hex::decode(salt_hex).unwrap();

                let dk = derive::<Sha512, MAX_OUTPUT>(&password, &salt, iteration_count);
                let dk_hex = hex::encode(&dk[..dk_len]);
                assert_eq!(
                    dk_hex, expected_dk_hex,
                    "wycheproof PBKDF2-SHA-512 tcId={} dkLen={}",
                    test["tcId"], dk_len
                );
                tested += 1;
            }
        }
        assert!(tested > 0, "no PBKDF2-SHA-512 wycheproof tests were run");
    }
}
