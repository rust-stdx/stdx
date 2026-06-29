use super::mlkem::{
    ML_KEM_768, MlKemError, SHARED_SECRET_SIZE, crypto_kem_dec, crypto_kem_enc_derand, crypto_kem_keypair_derand,
    indcpa_secret_key_bytes,
};

pub const PUBLIC_KEY_SIZE_768: usize = 1184;
pub const SECRET_KEY_SIZE_768: usize = 2400;
pub const CIPHERTEXT_SIZE_768: usize = 1088;

/// ML-KEM-768 decapsulation key (secret key) as defined in FIPS 203.
///
/// # Example
///
/// ```ignore
/// use crypto::mlkem::{SecretKey768, generate_keypair_768};
///
/// let (secret_key, public_key) = generate_keypair_768();
/// let (ciphertext, shared_secret) = public_key.encapsulate();
/// let decapsulated = secret_key.decapsulate(&ciphertext).unwrap();
/// assert_eq!(shared_secret, decapsulated);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct SecretKey768 {
    bytes: [u8; SECRET_KEY_SIZE_768],
}

/// ML-KEM-768 encapsulation key (public key) as defined in FIPS 203.
///
/// See [`SecretKey768`] for a full usage example.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey768 {
    bytes: [u8; PUBLIC_KEY_SIZE_768],
}

/// Generate an ML-KEM-768 keypair.
///
/// This is a convenience wrapper around [`SecretKey768::generate`].
///
/// See [`SecretKey768`] for a usage example.
#[inline]
#[cfg(feature = "random")]
pub fn generate_keypair_768() -> (SecretKey768, PublicKey768) {
    SecretKey768::generate()
}

#[inline]
pub(crate) fn generate_keypair_768_derand(coins: &[u8; 64]) -> (SecretKey768, PublicKey768) {
    SecretKey768::generate_derand(coins)
}

impl SecretKey768 {
    pub fn from_bytes(bytes: &[u8; SECRET_KEY_SIZE_768]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE_768] {
        self.bytes
    }

    #[cfg(feature = "random")]
    pub fn generate() -> (Self, PublicKey768) {
        let coins: [u8; 64] = crate::random::random_bytes();
        Self::generate_derand(&coins)
    }

    pub(crate) fn generate_derand(coins: &[u8; 64]) -> (Self, PublicKey768) {
        let (sk_bytes, pk_bytes) =
            crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, coins);
        (
            Self {
                bytes: sk_bytes,
            },
            PublicKey768 {
                bytes: pk_bytes,
            },
        )
    }

    pub fn decapsulate(&self, ciphertext: &[u8; CIPHERTEXT_SIZE_768]) -> Result<[u8; SHARED_SECRET_SIZE], MlKemError> {
        crypto_kem_dec::<3, SECRET_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &self.bytes, ciphertext)
    }

    pub fn public_key(&self) -> PublicKey768 {
        let offset = indcpa_secret_key_bytes::<3>();
        let mut pk_bytes = [0u8; PUBLIC_KEY_SIZE_768];
        pk_bytes.copy_from_slice(&self.bytes[offset..offset + PUBLIC_KEY_SIZE_768]);
        PublicKey768 {
            bytes: pk_bytes,
        }
    }
}

impl From<&[u8; SECRET_KEY_SIZE_768]> for SecretKey768 {
    fn from(bytes: &[u8; SECRET_KEY_SIZE_768]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for SecretKey768 {
    type Error = MlKemError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(bytes.try_into().map_err(|_| MlKemError::InvalidKey)?))
    }
}

impl PublicKey768 {
    pub fn from_bytes(bytes: &[u8; PUBLIC_KEY_SIZE_768]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE_768] {
        self.bytes
    }

    #[cfg(feature = "random")]
    pub fn encapsulate(&self) -> ([u8; CIPHERTEXT_SIZE_768], [u8; SHARED_SECRET_SIZE]) {
        let coins: [u8; 32] = crate::random::random_bytes();
        self.encapsulate_derand(&coins)
    }

    pub(crate) fn encapsulate_derand(&self, coins: &[u8; 32]) -> ([u8; CIPHERTEXT_SIZE_768], [u8; SHARED_SECRET_SIZE]) {
        crypto_kem_enc_derand::<3, PUBLIC_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &self.bytes, coins)
    }
}

impl From<&[u8; PUBLIC_KEY_SIZE_768]> for PublicKey768 {
    fn from(bytes: &[u8; PUBLIC_KEY_SIZE_768]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for PublicKey768 {
    type Error = MlKemError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(bytes.try_into().map_err(|_| MlKemError::InvalidKey)?))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::mlkem::{
            ML_KEM_768, crypto_kem_dec, crypto_kem_enc_derand, crypto_kem_keypair_derand, decode_hex_array,
            sha3_256_hex,
        },
        *,
    };

    #[test]
    fn ml_kem_768_round_trip() {
        let (private_key, public_key) = generate_keypair_768();
        let (ciphertext, encapsulated_secret) = public_key.encapsulate();
        let decapsulated_secret = private_key.decapsulate(&ciphertext).unwrap();

        assert_eq!(encapsulated_secret, decapsulated_secret);
    }

    #[test]
    fn ml_kem_768_decapsulation_rejects_tampered_ciphertext() {
        let (private_key, public_key) = generate_keypair_768();
        let (mut ciphertext, encapsulated_secret) = public_key.encapsulate();

        ciphertext[0] ^= 0x80;

        let decapsulated_secret = private_key.decapsulate(&ciphertext).unwrap();

        assert_ne!(encapsulated_secret, decapsulated_secret);
    }

    #[test]
    fn ml_kem_768_deterministic_derand_vectors_are_stable() {
        let key_coins = [7u8; 64];
        let enc_coins = [9u8; 32];
        let (secret_key, public_key) =
            crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &key_coins);
        let (ciphertext, shared_secret) =
            crypto_kem_enc_derand::<3, PUBLIC_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &public_key, &enc_coins);
        let decapsulated =
            crypto_kem_dec::<3, SECRET_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &secret_key, &ciphertext)
                .unwrap();

        assert_eq!(shared_secret, decapsulated);
        assert_eq!(
            hex::encode(&public_key[..32]),
            "925a2700ad064ff778b4da4cf51457a48224a52751250a8ee10b251c818bafca"
        );
        assert_eq!(
            hex::encode(&ciphertext[..32]),
            "766c326c3483444c5b6d917cdddc3c07fbf935295c8f17c92a187a80dc4d15f2"
        );
        assert_eq!(
            hex::encode(shared_secret),
            "afcf18dfd6b710a09b5cf591d0eb8229d83aa10904934a3ca60a52da5ff36b96"
        );
    }

    #[test]
    fn ml_kem_768_cctv_accumulated_10k() {
        use crate::{Xof, sha3::Shake128};

        let mut rng = Shake128::new();
        rng.absorb(&[]);

        let mut acc = Shake128::new();

        for _ in 0..10_000u32 {
            let mut d = [0u8; 32];
            let mut z = [0u8; 32];
            let mut m = [0u8; 32];
            let mut ct_random = [0u8; CIPHERTEXT_SIZE_768];

            rng.squeeze(&mut d);
            rng.squeeze(&mut z);
            rng.squeeze(&mut m);
            rng.squeeze(&mut ct_random);

            let mut coins = [0u8; 64];
            coins[..32].copy_from_slice(&d);
            coins[32..].copy_from_slice(&z);

            let (dk, ek) =
                crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &coins);
            let (ct, k_encaps) =
                crypto_kem_enc_derand::<3, PUBLIC_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &ek, &m);

            let k_decaps =
                crypto_kem_dec::<3, SECRET_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &dk, &ct).unwrap();
            assert_eq!(k_encaps, k_decaps);

            let k_decaps_random =
                crypto_kem_dec::<3, SECRET_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &dk, &ct_random).unwrap();

            acc.absorb(&ek);
            acc.absorb(&dk);
            acc.absorb(&ct);
            acc.absorb(&k_encaps);
            acc.absorb(&k_decaps_random);
        }

        let mut hash = [0u8; 32];
        acc.squeeze(&mut hash);
        assert_eq!(
            hex::encode(hash),
            "f959d18d3d1180121433bf0e05f11e7908cf9d03edc150b2b07cb90bef5bc1c1",
            "ML-KEM-768 CCTV accumulated hash mismatch"
        );
    }

    #[test]
    fn ml_kem_768_cctv_intermediate_vector() {
        let d: [u8; 32] = decode_hex_array("f688563f7c66a5da2d8bdb5a5f3e07bd8dce6f7efcec7f41298d79863459f7cd");
        let z: [u8; 32] = decode_hex_array("d1d49a515250dbceb9f6e3fcc1c7d5306918964b21ddb22207e03e57f0600da8");
        let m: [u8; 32] = decode_hex_array("3dc27ca0a6594b0e56320457c45a0f76bb8a213ea4a76d442186a0aefadbcdb9");

        let mut coins = [0u8; 64];
        coins[..32].copy_from_slice(&d);
        coins[32..].copy_from_slice(&z);

        let (dk, ek) = crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &coins);
        let (ct, k) = crypto_kem_enc_derand::<3, PUBLIC_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &ek, &m);

        assert_eq!(
            sha3_256_hex(&ek),
            "42d930a50dfd1f0541ca45c4598daebb4f51cd10d711a001bd9bb87d5c87a4bf"
        );
        assert_eq!(
            sha3_256_hex(&dk),
            "db563aebd9fdc875e88563693edad1e5e359cc37b0f685d2d0a3723b37253192"
        );
        assert_eq!(
            sha3_256_hex(&ct),
            "9d6e358208c4d583050becb319050b7f916de47caad1d589a1d01fea43fe1750"
        );
        assert_eq!(
            hex::encode(k),
            "ae726da2df66601c6648a7565c02b203a089276ac30f6cc226d048f93fafd78c"
        );
    }

    #[test]
    fn ml_kem_768_decapsulation_with_wrong_key_rejects() {
        let (_, alice_pk) = generate_keypair_768();
        let (bob_sk, _bob_pk) = generate_keypair_768();
        let (ct, _alice_ss) = alice_pk.encapsulate();

        let wrong_ss = bob_sk.decapsulate(&ct).unwrap();
        assert_ne!(_alice_ss, wrong_ss);
    }

    #[test]
    fn ml_kem_768_round_trip_many() {
        for _ in 0..100 {
            let (sk, pk) = generate_keypair_768();
            let (ct, ss_enc) = pk.encapsulate();
            let ss_dec = sk.decapsulate(&ct).unwrap();
            assert_eq!(ss_enc, ss_dec);
        }
    }

    #[test]
    fn ml_kem_768_all_zero_ciphertext_does_not_panic() {
        let (sk, _pk) = generate_keypair_768();
        let ct = [0u8; CIPHERTEXT_SIZE_768];
        let _result = sk.decapsulate(&ct);
    }

    #[test]
    fn ml_kem_768_all_ones_ciphertext_does_not_panic() {
        let (sk, _pk) = generate_keypair_768();
        let ct = [0xffu8; CIPHERTEXT_SIZE_768];
        let _result = sk.decapsulate(&ct);
    }

    #[test]
    fn ml_kem_768_derand_keygen_is_deterministic() {
        let coins = [7u8; 64];
        let (sk1, pk1) = crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &coins);
        let (sk2, pk2) = crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &coins);
        assert_eq!(sk1, sk2);
        assert_eq!(pk1, pk2);
    }

    #[test]
    fn ml_kem_768_key_sizes_are_correct() {
        let (sk, pk) = generate_keypair_768();
        let sk_bytes = sk.to_bytes();
        let pk_bytes = pk.to_bytes();
        assert_eq!(sk_bytes.len(), SECRET_KEY_SIZE_768);
        assert_eq!(pk_bytes.len(), PUBLIC_KEY_SIZE_768);
        let (ct, _) = pk.encapsulate();
        assert_eq!(ct.len(), CIPHERTEXT_SIZE_768);
    }

    #[test]
    fn ml_kem_768_encaps_is_deterministic_with_same_coins() {
        let enc_coins = [9u8; 32];
        let key_coins = [7u8; 64];
        let (_sk, pk) =
            crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &key_coins);
        let (ct1, ss1) =
            crypto_kem_enc_derand::<3, PUBLIC_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &pk, &enc_coins);
        let (ct2, ss2) =
            crypto_kem_enc_derand::<3, PUBLIC_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &pk, &enc_coins);
        assert_eq!(ct1, ct2);
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn ml_kem_768_decapsulation_with_wrong_key_is_deterministic() {
        let (_, pk_a) = generate_keypair_768();
        let (sk_b, _pk_b) = generate_keypair_768();
        let (ct, _) = pk_a.encapsulate();

        let ss1 = sk_b.decapsulate(&ct).unwrap();
        let ss2 = sk_b.decapsulate(&ct).unwrap();
        assert_eq!(ss1, ss2, "implicit rejection must be deterministic");
    }

    #[test]
    fn ml_kem_768_wycheproof_keygen() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/mlkem_768_keygen_seed_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-768") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let seed_hex = test["seed"].as_str().unwrap();
                let expected_ek_hex = test["ek"].as_str().unwrap();
                let expected_dk_hex = test["dk"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let seed = hex::decode_array::<64>(seed_hex.as_bytes()).unwrap();

                let (dk, ek) =
                    crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &seed);

                let ek_hex = hex::encode(ek);
                let dk_hex = hex::encode(dk);

                if result == "valid" {
                    assert_eq!(
                        ek_hex, expected_ek_hex,
                        "wycheproof keygen KAT tcId={} ek mismatch",
                        test["tcId"]
                    );
                    assert_eq!(
                        dk_hex, expected_dk_hex,
                        "wycheproof keygen KAT tcId={} dk mismatch",
                        test["tcId"]
                    );
                }
                tested += 1;
            }
        }
        assert!(tested > 0, "no ML-KEM-768 keygen tests were run");
    }

    fn wycheproof_kem_skip_invalid_lengths(seed_hex: &str, c_hex: &str, ct_size: usize) -> bool {
        seed_hex.len() != 128 || c_hex.len() != ct_size * 2
    }

    #[test]
    fn ml_kem_768_wycheproof_kem() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../testdata/wycheproof/testvectors_v1/mlkem_768_test.json")).unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-768") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let seed_hex = test["seed"].as_str().unwrap();
                let c_hex = test["c"].as_str().unwrap();
                let expected_k_hex = test["K"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                if wycheproof_kem_skip_invalid_lengths(seed_hex, c_hex, CIPHERTEXT_SIZE_768) {
                    tested += 1;
                    continue;
                }

                let seed = hex::decode_array::<64>(seed_hex.as_bytes()).unwrap();

                let (dk, ek) =
                    crypto_kem_keypair_derand::<3, SECRET_KEY_SIZE_768, PUBLIC_KEY_SIZE_768>(&ML_KEM_768, &seed);

                if let Some(expected_ek_hex) = test.get("ek").and_then(|v| v.as_str()) {
                    let ek_hex = hex::encode(ek);
                    assert_eq!(ek_hex, expected_ek_hex, "wycheproof KEM KAT tcId={} ek mismatch", test["tcId"]);
                }

                let c = decode_hex_array::<CIPHERTEXT_SIZE_768>(c_hex);
                let shared_secret = crypto_kem_dec::<3, SECRET_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &dk, &c);

                if result == "valid" {
                    let k = shared_secret.unwrap();
                    let k_hex = hex::encode(k);
                    assert_eq!(k_hex, expected_k_hex, "wycheproof KEM KAT tcId={} K mismatch", test["tcId"]);
                } else {
                    assert!(
                        shared_secret.is_ok(),
                        "wycheproof KEM KAT tcId={} unexpected error",
                        test["tcId"]
                    );
                }
                tested += 1;
            }
        }
        assert!(tested > 0, "no ML-KEM-768 KEM tests were run");
    }

    #[test]
    fn ml_kem_768_wycheproof_encaps() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/mlkem_768_encaps_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-768") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let ek_hex = test["ek"].as_str().unwrap();
                let m_hex = test["m"].as_str().unwrap();
                let expected_c_hex = test["c"].as_str().unwrap();
                let expected_k_hex = test["K"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                if ek_hex.len() != PUBLIC_KEY_SIZE_768 * 2 {
                    tested += 1;
                    continue;
                }

                let ek = decode_hex_array::<PUBLIC_KEY_SIZE_768>(ek_hex);

                if result == "valid" {
                    let m = decode_hex_array::<32>(m_hex);
                    let (c, k) =
                        crypto_kem_enc_derand::<3, PUBLIC_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &ek, &m);
                    let c_hex_out = hex::encode(c);
                    let k_hex_out = hex::encode(k);
                    assert_eq!(
                        c_hex_out, expected_c_hex,
                        "wycheproof encaps KAT tcId={} c mismatch",
                        test["tcId"]
                    );
                    assert_eq!(
                        k_hex_out, expected_k_hex,
                        "wycheproof encaps KAT tcId={} K mismatch",
                        test["tcId"]
                    );
                }
                tested += 1;
            }
        }
        assert!(tested > 0, "no ML-KEM-768 encaps tests were run");
    }

    #[test]
    fn ml_kem_768_wycheproof_decaps_validation() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/mlkem_768_semi_expanded_decaps_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-768") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let flags: Vec<&str> = test["flags"]
                    .as_array()
                    .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                    .unwrap_or_default();
                let dk_hex = test["dk"].as_str().unwrap();
                let c_hex = test["c"].as_str().unwrap();

                if flags.contains(&"IncorrectDecapsulationKeyLength") || flags.contains(&"IncorrectCiphertextLength") {
                    tested += 1;
                    continue;
                }

                let dk = decode_hex_array::<SECRET_KEY_SIZE_768>(dk_hex);
                let c = decode_hex_array::<CIPHERTEXT_SIZE_768>(c_hex);

                let result = crypto_kem_dec::<3, SECRET_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &dk, &c);

                assert!(result.is_ok(), "wycheproof decaps tcId={} panicked", test["tcId"]);
                tested += 1;
            }
        }
        assert!(tested > 0, "no ML-KEM-768 decaps validation tests were run");
    }

    #[test]
    fn ml_kem_768_cross_implementation_pqcrypto() {
        // Cross-implementation validation: decapsulate ciphertexts generated by
        // the pqcrypto (liboqs) ML-KEM-768 implementation.
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../testdata/mlkem/pqcrypto_768_vectors.json")).unwrap();
        let vectors = data.as_array().unwrap();
        assert!(vectors.len() >= 5, "not enough cross-impl vectors");

        for (i, vector) in vectors.iter().enumerate() {
            let sk_hex = vector["sk"].as_str().unwrap();
            let ct_hex = vector["ct"].as_str().unwrap();
            let expected_ss_hex = vector["ss"].as_str().unwrap();

            let sk = decode_hex_array::<SECRET_KEY_SIZE_768>(sk_hex);
            let ct = decode_hex_array::<CIPHERTEXT_SIZE_768>(ct_hex);

            let ss = crypto_kem_dec::<3, SECRET_KEY_SIZE_768, CIPHERTEXT_SIZE_768>(&ML_KEM_768, &sk, &ct).unwrap();
            assert_eq!(
                hex::encode(ss),
                expected_ss_hex,
                "cross-impl pqcrypto vector {i} decapsulation mismatch"
            );
        }
    }
}
