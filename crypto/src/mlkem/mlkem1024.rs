use super::mlkem::{
    ML_KEM_1024, MlKemError, SHARED_SECRET_SIZE, crypto_kem_dec, crypto_kem_enc_derand, crypto_kem_keypair_derand,
    indcpa_secret_key_bytes,
};

pub const PUBLIC_KEY_SIZE_1024: usize = 1568;
pub const SECRET_KEY_SIZE_1024: usize = 3168;
pub const CIPHERTEXT_SIZE_1024: usize = 1568;

/// ML-KEM-1024 decapsulation key (secret key) as defined in FIPS 203.
///
/// # Example
///
/// ```ignore
/// use crypto::mlkem::{SecretKey1024, generate_keypair_1024};
///
/// let (secret_key, public_key) = generate_keypair_1024();
/// let (ciphertext, shared_secret) = public_key.encapsulate();
/// let decapsulated = secret_key.decapsulate(&ciphertext).unwrap();
/// assert_eq!(shared_secret, decapsulated);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct SecretKey1024 {
    bytes: [u8; SECRET_KEY_SIZE_1024],
}

/// ML-KEM-1024 encapsulation key (public key) as defined in FIPS 203.
///
/// See [`SecretKey1024`] for a full usage example.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey1024 {
    bytes: [u8; PUBLIC_KEY_SIZE_1024],
}

/// Generate an ML-KEM-1024 keypair.
///
/// This is a convenience wrapper around [`SecretKey1024::generate`].
///
/// See [`SecretKey1024`] for a usage example.
#[inline]
#[cfg(feature = "random")]
pub fn generate_keypair_1024() -> (SecretKey1024, PublicKey1024) {
    SecretKey1024::generate()
}

impl SecretKey1024 {
    pub fn from_bytes(bytes: &[u8; SECRET_KEY_SIZE_1024]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE_1024] {
        self.bytes
    }

    #[cfg(feature = "random")]
    pub fn generate() -> (Self, PublicKey1024) {
        let coins: [u8; 64] = crate::random::random_bytes();
        Self::generate_derand(&coins)
    }

    fn generate_derand(coins: &[u8; 64]) -> (Self, PublicKey1024) {
        let (sk_bytes, pk_bytes) =
            crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, coins);
        (
            Self {
                bytes: sk_bytes,
            },
            PublicKey1024 {
                bytes: pk_bytes,
            },
        )
    }

    pub fn decapsulate(&self, ciphertext: &[u8; CIPHERTEXT_SIZE_1024]) -> Result<[u8; SHARED_SECRET_SIZE], MlKemError> {
        crypto_kem_dec::<4, SECRET_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &self.bytes, ciphertext)
    }

    pub fn public_key(&self) -> PublicKey1024 {
        let offset = indcpa_secret_key_bytes::<4>();
        let mut pk_bytes = [0u8; PUBLIC_KEY_SIZE_1024];
        pk_bytes.copy_from_slice(&self.bytes[offset..offset + PUBLIC_KEY_SIZE_1024]);
        PublicKey1024 {
            bytes: pk_bytes,
        }
    }
}

impl From<&[u8; SECRET_KEY_SIZE_1024]> for SecretKey1024 {
    fn from(bytes: &[u8; SECRET_KEY_SIZE_1024]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for SecretKey1024 {
    type Error = MlKemError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(bytes.try_into().map_err(|_| MlKemError::InvalidKey)?))
    }
}

impl PublicKey1024 {
    pub fn from_bytes(bytes: &[u8; PUBLIC_KEY_SIZE_1024]) -> Self {
        Self {
            bytes: *bytes,
        }
    }

    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE_1024] {
        self.bytes
    }

    #[cfg(feature = "random")]
    pub fn encapsulate(&self) -> ([u8; CIPHERTEXT_SIZE_1024], [u8; SHARED_SECRET_SIZE]) {
        let coins: [u8; 32] = crate::random::random_bytes();
        self.encapsulate_derand(&coins)
    }

    fn encapsulate_derand(&self, coins: &[u8; 32]) -> ([u8; CIPHERTEXT_SIZE_1024], [u8; SHARED_SECRET_SIZE]) {
        crypto_kem_enc_derand::<4, PUBLIC_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &self.bytes, coins)
    }
}

impl From<&[u8; PUBLIC_KEY_SIZE_1024]> for PublicKey1024 {
    fn from(bytes: &[u8; PUBLIC_KEY_SIZE_1024]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for PublicKey1024 {
    type Error = MlKemError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(bytes.try_into().map_err(|_| MlKemError::InvalidKey)?))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::mlkem::{
            ML_KEM_1024, crypto_kem_dec, crypto_kem_enc_derand, crypto_kem_keypair_derand, decode_hex_array,
            sha3_256_hex,
        },
        *,
    };

    #[test]
    fn ml_kem_1024_round_trip() {
        let (private_key, public_key) = generate_keypair_1024();
        let (ciphertext, encapsulated_secret) = public_key.encapsulate();
        let decapsulated_secret = private_key.decapsulate(&ciphertext).unwrap();

        assert_eq!(encapsulated_secret, decapsulated_secret);
    }

    #[test]
    fn ml_kem_1024_deterministic_derand_vectors_are_stable() {
        let key_coins = [3u8; 64];
        let enc_coins = [5u8; 32];
        let (secret_key, public_key) =
            crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &key_coins);
        let (ciphertext, shared_secret) = crypto_kem_enc_derand::<4, PUBLIC_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(
            &ML_KEM_1024,
            &public_key,
            &enc_coins,
        );
        let decapsulated =
            crypto_kem_dec::<4, SECRET_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &secret_key, &ciphertext)
                .unwrap();

        assert_eq!(shared_secret, decapsulated);
        assert_eq!(
            hex::encode(&public_key[..32]),
            "2dd29da8b193397a4336c02382aab3bcfbac25f0cd71c888af379e1e75149a79"
        );
        assert_eq!(
            hex::encode(&ciphertext[..32]),
            "5f12f173ef59a45f910d3a225913f3297b2277636a72401a273648015cccf079"
        );
        assert_eq!(
            hex::encode(shared_secret),
            "8bf157178aa556b55f95686ba9b5afe13a6b75c848f1ddd9a334d50287bec24e"
        );
    }

    #[test]
    fn ml_kem_1024_cctv_accumulated_10k() {
        use crate::{Xof, sha3::Shake128};

        let mut rng = Shake128::new();
        rng.absorb(&[]);

        let mut acc = Shake128::new();

        for _ in 0..10_000u32 {
            let mut d = [0u8; 32];
            let mut z = [0u8; 32];
            let mut m = [0u8; 32];
            let mut ct_random = [0u8; CIPHERTEXT_SIZE_1024];

            rng.squeeze(&mut d);
            rng.squeeze(&mut z);
            rng.squeeze(&mut m);
            rng.squeeze(&mut ct_random);

            let mut coins = [0u8; 64];
            coins[..32].copy_from_slice(&d);
            coins[32..].copy_from_slice(&z);

            let (dk, ek) =
                crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &coins);
            let (ct, k_encaps) =
                crypto_kem_enc_derand::<4, PUBLIC_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &ek, &m);

            let k_decaps =
                crypto_kem_dec::<4, SECRET_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &dk, &ct).unwrap();
            assert_eq!(k_encaps, k_decaps);

            let k_decaps_random =
                crypto_kem_dec::<4, SECRET_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &dk, &ct_random).unwrap();

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
            "e3bf82b013307b2e9d47dde791ff6dfc82e694e6382404abdb948b908b75bad5",
            "ML-KEM-1024 CCTV accumulated hash mismatch"
        );
    }

    #[test]
    fn ml_kem_1024_cctv_intermediate_vector() {
        let d: [u8; 32] = decode_hex_array("2a62c39ef4fc499f2d132716f480bb7521a49558ae84ee80d9352e66daf1e3a8");
        let z: [u8; 32] = decode_hex_array("5f574ef7f013d4336801fed022178c3ed91d0b6d51325315fc1dcabf4770a2ea");
        let m: [u8; 32] = decode_hex_array("e07d685ed308e609c9c7842026e35732f6ffc6e2fee10f0afd348f2b42a8acb4");

        let mut coins = [0u8; 64];
        coins[..32].copy_from_slice(&d);
        coins[32..].copy_from_slice(&z);

        let (dk, ek) = crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &coins);
        let (ct, k) = crypto_kem_enc_derand::<4, PUBLIC_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &ek, &m);

        assert_eq!(
            sha3_256_hex(&ek),
            "3b308d1344ed70366b84d790acb705b86cd3dfd471fff171969aaa338f26dca5"
        );
        assert_eq!(
            sha3_256_hex(&dk),
            "aa63a9e0c035ada6635e7938b71856b24917ff9b3ebca1a4d205a83b502a415a"
        );
        assert_eq!(
            sha3_256_hex(&ct),
            "8caba02733421f12a7ba9a2bcbe4de7c9853156a0637df5a7a0f9127c81da943"
        );
        assert_eq!(
            hex::encode(k),
            "d53825c3ff666bb2881215dbec04a8bdce9099b2a3680938c2f199b54d505953"
        );
    }

    #[test]
    fn ml_kem_1024_decapsulation_rejects_tampered_ciphertext() {
        let (private_key, public_key) = generate_keypair_1024();
        let (mut ciphertext, encapsulated_secret) = public_key.encapsulate();

        ciphertext[0] ^= 0x80;

        let decapsulated_secret = private_key.decapsulate(&ciphertext).unwrap();

        assert_ne!(encapsulated_secret, decapsulated_secret);
    }

    #[test]
    fn ml_kem_1024_decapsulation_with_wrong_key_rejects() {
        let (_, alice_pk) = generate_keypair_1024();
        let (bob_sk, _bob_pk) = generate_keypair_1024();
        let (ct, _alice_ss) = alice_pk.encapsulate();

        let wrong_ss = bob_sk.decapsulate(&ct).unwrap();
        assert_ne!(_alice_ss, wrong_ss);
    }

    #[test]
    fn ml_kem_1024_round_trip_many() {
        for _ in 0..100 {
            let (sk, pk) = generate_keypair_1024();
            let (ct, ss_enc) = pk.encapsulate();
            let ss_dec = sk.decapsulate(&ct).unwrap();
            assert_eq!(ss_enc, ss_dec);
        }
    }

    #[test]
    fn ml_kem_1024_all_zero_ciphertext_does_not_panic() {
        let (sk, _pk) = generate_keypair_1024();
        let ct = [0u8; CIPHERTEXT_SIZE_1024];
        let _result = sk.decapsulate(&ct);
    }

    #[test]
    fn ml_kem_1024_all_ones_ciphertext_does_not_panic() {
        let (sk, _pk) = generate_keypair_1024();
        let ct = [0xffu8; CIPHERTEXT_SIZE_1024];
        let _result = sk.decapsulate(&ct);
    }

    #[test]
    fn ml_kem_1024_derand_keygen_is_deterministic() {
        let coins = [3u8; 64];
        let (sk1, pk1) =
            crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &coins);
        let (sk2, pk2) =
            crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &coins);
        assert_eq!(sk1, sk2);
        assert_eq!(pk1, pk2);
    }

    #[test]
    fn ml_kem_1024_key_sizes_are_correct() {
        let (sk, pk) = generate_keypair_1024();
        let sk_bytes = sk.to_bytes();
        let pk_bytes = pk.to_bytes();
        assert_eq!(sk_bytes.len(), SECRET_KEY_SIZE_1024);
        assert_eq!(pk_bytes.len(), PUBLIC_KEY_SIZE_1024);
        let (ct, _) = pk.encapsulate();
        assert_eq!(ct.len(), CIPHERTEXT_SIZE_1024);
    }

    #[test]
    fn ml_kem_1024_encaps_is_deterministic_with_same_coins() {
        let enc_coins = [5u8; 32];
        let key_coins = [3u8; 64];
        let (_sk, pk) =
            crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &key_coins);
        let (ct1, ss1) =
            crypto_kem_enc_derand::<4, PUBLIC_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &pk, &enc_coins);
        let (ct2, ss2) =
            crypto_kem_enc_derand::<4, PUBLIC_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &pk, &enc_coins);
        assert_eq!(ct1, ct2);
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn ml_kem_1024_decapsulation_with_wrong_key_is_deterministic() {
        let (_, pk_a) = generate_keypair_1024();
        let (sk_b, _pk_b) = generate_keypair_1024();
        let (ct, _) = pk_a.encapsulate();

        let ss1 = sk_b.decapsulate(&ct).unwrap();
        let ss2 = sk_b.decapsulate(&ct).unwrap();
        assert_eq!(ss1, ss2, "implicit rejection must be deterministic");
    }

    #[test]
    fn ml_kem_1024_wycheproof_keygen() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/mlkem_1024_keygen_seed_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-1024") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let seed_hex = test["seed"].as_str().unwrap();
                let expected_ek_hex = test["ek"].as_str().unwrap();
                let expected_dk_hex = test["dk"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let seed = hex::decode_array::<64>(seed_hex.as_bytes()).unwrap();

                let (dk, ek) =
                    crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &seed);

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
        assert!(tested > 0, "no ML-KEM-1024 keygen tests were run");
    }

    fn wycheproof_kem_skip_invalid_lengths(seed_hex: &str, c_hex: &str, ct_size: usize) -> bool {
        seed_hex.len() != 128 || c_hex.len() != ct_size * 2
    }

    #[test]
    fn ml_kem_1024_wycheproof_kem() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../testdata/wycheproof/testvectors_v1/mlkem_1024_test.json"))
                .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-1024") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let seed_hex = test["seed"].as_str().unwrap();
                let c_hex = test["c"].as_str().unwrap();
                let expected_k_hex = test["K"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                if wycheproof_kem_skip_invalid_lengths(seed_hex, c_hex, CIPHERTEXT_SIZE_1024) {
                    tested += 1;
                    continue;
                }

                let seed = hex::decode_array::<64>(seed_hex.as_bytes()).unwrap();

                let (dk, ek) =
                    crypto_kem_keypair_derand::<4, SECRET_KEY_SIZE_1024, PUBLIC_KEY_SIZE_1024>(&ML_KEM_1024, &seed);

                if let Some(expected_ek_hex) = test.get("ek").and_then(|v| v.as_str()) {
                    let ek_hex = hex::encode(ek);
                    assert_eq!(ek_hex, expected_ek_hex, "wycheproof KEM KAT tcId={} ek mismatch", test["tcId"]);
                }

                let c = decode_hex_array::<CIPHERTEXT_SIZE_1024>(c_hex);
                let shared_secret =
                    crypto_kem_dec::<4, SECRET_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &dk, &c);

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
        assert!(tested > 0, "no ML-KEM-1024 KEM tests were run");
    }

    #[test]
    fn ml_kem_1024_wycheproof_encaps() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/mlkem_1024_encaps_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-1024") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let ek_hex = test["ek"].as_str().unwrap();
                let m_hex = test["m"].as_str().unwrap();
                let expected_c_hex = test["c"].as_str().unwrap();
                let expected_k_hex = test["K"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                if ek_hex.len() != PUBLIC_KEY_SIZE_1024 * 2 {
                    tested += 1;
                    continue;
                }

                let ek = decode_hex_array::<PUBLIC_KEY_SIZE_1024>(ek_hex);

                if result == "valid" {
                    let m = decode_hex_array::<32>(m_hex);
                    let (c, k) =
                        crypto_kem_enc_derand::<4, PUBLIC_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &ek, &m);
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
        assert!(tested > 0, "no ML-KEM-1024 encaps tests were run");
    }

    #[test]
    fn ml_kem_1024_wycheproof_decaps_validation() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/mlkem_1024_semi_expanded_decaps_test.json"
        ))
        .unwrap();
        let mut tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["parameterSet"].as_str() != Some("ML-KEM-1024") {
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

                let dk = decode_hex_array::<SECRET_KEY_SIZE_1024>(dk_hex);
                let c = decode_hex_array::<CIPHERTEXT_SIZE_1024>(c_hex);

                let result = crypto_kem_dec::<4, SECRET_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &dk, &c);

                assert!(result.is_ok(), "wycheproof decaps tcId={} panicked", test["tcId"]);
                tested += 1;
            }
        }
        assert!(tested > 0, "no ML-KEM-1024 decaps validation tests were run");
    }

    #[test]
    fn ml_kem_1024_cross_implementation_pqcrypto() {
        // Cross-implementation validation: decapsulate ciphertexts generated by
        // the pqcrypto (liboqs) ML-KEM-1024 implementation.
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../testdata/mlkem/pqcrypto_1024_vectors.json")).unwrap();
        let vectors = data.as_array().unwrap();
        assert!(vectors.len() >= 5, "not enough cross-impl vectors");

        for (i, vector) in vectors.iter().enumerate() {
            let sk_hex = vector["sk"].as_str().unwrap();
            let ct_hex = vector["ct"].as_str().unwrap();
            let expected_ss_hex = vector["ss"].as_str().unwrap();

            let sk = decode_hex_array::<SECRET_KEY_SIZE_1024>(sk_hex);
            let ct = decode_hex_array::<CIPHERTEXT_SIZE_1024>(ct_hex);

            let ss = crypto_kem_dec::<4, SECRET_KEY_SIZE_1024, CIPHERTEXT_SIZE_1024>(&ML_KEM_1024, &sk, &ct).unwrap();
            assert_eq!(
                hex::encode(ss),
                expected_ss_hex,
                "cross-impl pqcrypto vector {i} decapsulation mismatch"
            );
        }
    }
}
