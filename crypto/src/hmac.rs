//! HKDF (HMAC-based Extract-and-Expand Key Derivation Function) key derivation function.

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

use crate::{Hash, Hasher, MAX_HASH_BLOCK_SIZE};

/// HMAC (Hash-based Message Authentication Code) implementation.
///
/// Uses a generic hash function implementing the [`Hasher`] trait (e.g.
/// [`Sha256`](crate::sha2::Sha256) or [`Sha512`](crate::sha2::Sha512)).
///
/// # One-shot API
///
/// ```ignore
/// use crypto::hmac::Hmac;
/// use crypto::sha2::Sha256;
///
/// let tag = Hmac::<Sha256>::mac(b"key", b"message");
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::hmac::Hmac;
/// use crypto::sha2::Sha256;
///
/// let mut mac = Hmac::<Sha256>::new(b"key");
/// mac.update(b"hello ");
/// mac.update(b"world");
/// let tag = mac.finalize();
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(Zeroize))]
pub struct Hmac<H: Hasher> {
    hash: H,
    opad: [u8; MAX_HASH_BLOCK_SIZE],
}

impl<H: Hasher> Hmac<H> {
    /// One-shot HMAC: computes `HMAC(key, data)` in a single call.
    ///
    /// This is a convenience wrapper around [`new`](Self::new) +
    /// [`update`](Self::update) + [`finalize`](Self::finalize).
    #[inline]
    pub fn mac(key: &[u8], data: &[u8]) -> Hash {
        let mut mac = Self::new(key);
        mac.update(data);
        return mac.finalize();
    }

    pub fn new(key: &[u8]) -> Self {
        let mut key_block = [0u8; MAX_HASH_BLOCK_SIZE];

        // normalize key to block size
        if key.len() > H::BLOCK_SIZE {
            let mut h = H::new();
            h.update(key);
            let hashed = h.sum();
            let hashed_bytes = hashed.as_ref();
            key_block[..hashed_bytes.len()].copy_from_slice(hashed_bytes);
        } else {
            key_block[..key.len()].copy_from_slice(key);
        }

        // inner pad = key ^ 0x36
        let mut inner_key = [0u8; MAX_HASH_BLOCK_SIZE];
        for i in 0..H::BLOCK_SIZE {
            inner_key[i] = key_block[i] ^ 0x36;
        }

        // outer pad = key ^ 0x5c
        let mut opad = [0u8; MAX_HASH_BLOCK_SIZE];
        for i in 0..H::BLOCK_SIZE {
            opad[i] = key_block[i] ^ 0x5c;
        }

        // initialize inner hash: create a fresh instance and feed inner pad
        let mut hash = H::new();
        hash.update(&inner_key[..H::BLOCK_SIZE]);

        Hmac {
            hash,
            opad,
        }
    }

    /// Feed message data to HMAC (can be called multiple times)
    pub fn update(&mut self, data: &[u8]) {
        self.hash.update(data);
    }

    /// Finalize and return HMAC tag. This consumes the Hmac state.
    pub fn finalize(self) -> Hash {
        let inner_sum = self.hash.sum();

        // compute outer hash using a fresh instance
        let mut outer = H::new();
        outer.update(&self.opad[..H::BLOCK_SIZE]);
        outer.update(inner_sum.as_ref());
        outer.sum()
    }
}

#[cfg(test)]
mod hmac_tests {
    use crate::{
        hmac::Hmac,
        sha2::{Sha256, Sha384, Sha512},
        sha3::{Sha3_256, Sha3_512},
    };

    #[derive(Clone, Copy)]
    enum TestInput {
        Bytes(&'static [u8]),
        Repeated { byte: u8, len: usize },
        RangeInclusive { start: u8, end: u8 },
    }

    #[derive(Clone, Copy)]
    struct HmacTestVector {
        source: &'static str,
        key: TestInput,
        data: TestInput,
        expected_sha256: &'static str,
        expected_sha512: &'static str,
    }

    const HMAC_TEST_VECTORS: [HmacTestVector; 6] = [
        // RFC 4231 TC1
        HmacTestVector {
            source: "RFC 4231 TC1",
            key: TestInput::Repeated {
                byte: 0x0b,
                len: 20,
            },
            data: TestInput::Bytes(b"Hi There"),
            expected_sha256: "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7",
            expected_sha512: "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854",
        },
        // RFC 4231 TC2
        HmacTestVector {
            source: "RFC 4231 TC2",
            key: TestInput::Bytes(b"Jefe"),
            data: TestInput::Bytes(b"what do ya want for nothing?"),
            expected_sha256: "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843",
            expected_sha512: "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737",
        },
        // RFC 4231 TC3
        HmacTestVector {
            source: "RFC 4231 TC3",
            key: TestInput::Repeated {
                byte: 0xaa,
                len: 20,
            },
            data: TestInput::Repeated {
                byte: 0xdd,
                len: 50,
            },
            expected_sha256: "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe",
            expected_sha512: "fa73b0089d56a284efb0f0756c890be9b1b5dbdd8ee81a3655f83e33b2279d39bf3e848279a722c806b485a47e67c807b946a337bee8942674278859e13292fb",
        },
        // RFC 4231 TC4
        HmacTestVector {
            source: "RFC 4231 TC4",
            key: TestInput::RangeInclusive {
                start: 0x01,
                end: 0x19,
            },
            data: TestInput::Repeated {
                byte: 0xcd,
                len: 50,
            },
            expected_sha256: "82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b",
            expected_sha512: "b0ba465637458c6990e5a8c5f61d4af7e576d97ff94b872de76f8050361ee3dba91ca5c11aa25eb4d679275cc5788063a5f19741120c4f2de2adebeb10a298dd",
        },
        // RFC 4231 TC6 (TC5 is truncated-output only)
        HmacTestVector {
            source: "RFC 4231 TC6",
            key: TestInput::Repeated {
                byte: 0xaa,
                len: 131,
            },
            data: TestInput::Bytes(b"Test Using Larger Than Block-Size Key - Hash Key First"),
            expected_sha256: "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54",
            expected_sha512: "80b24263c7c1a3ebb71493c1dd7be8b49b46d1f41b4aeec1121b013783f8f3526b56d037e05f2598bd0fd2215d6a1e5295e64f73f63f0aec8b915a985d786598",
        },
        // RFC 4231 TC7
        HmacTestVector {
            source: "RFC 4231 TC7",
            key: TestInput::Repeated {
                byte: 0xaa,
                len: 131,
            },
            data: TestInput::Bytes(
                b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.",
            ),
            expected_sha256: "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2",
            expected_sha512: "e37b6a775dc87dbaa4dfa9f96e5e3ffddebd71f8867289865df5a32d20cdc944b6022cac3c4982b10d5eeb55c3e4de15134676fb6de0446065c97440fa8c6a58",
        },
    ];

    fn materialize(input: TestInput) -> Vec<u8> {
        match input {
            TestInput::Bytes(bytes) => bytes.to_vec(),
            TestInput::Repeated {
                byte,
                len,
            } => vec![byte; len],
            TestInput::RangeInclusive {
                start,
                end,
            } => (start..=end).collect(),
        }
    }

    fn hmac256(key: &[u8], data: &[u8]) -> String {
        let mut mac = Hmac::<Sha256>::new(key);
        mac.update(data);
        hex::encode(mac.finalize().as_ref())
    }

    fn hmac512(key: &[u8], data: &[u8]) -> String {
        let mut mac = Hmac::<Sha512>::new(key);
        mac.update(data);
        hex::encode(mac.finalize().as_ref())
    }

    #[test]
    fn hmac_vectors() {
        for vector in HMAC_TEST_VECTORS {
            let key = materialize(vector.key);
            let data = materialize(vector.data);

            let single256 = hmac256(&key, &data);
            let single512 = hmac512(&key, &data);

            assert_eq!(single256, vector.expected_sha256, "{}", vector.source);
            assert_eq!(single512, vector.expected_sha512, "{}", vector.source);

            let mut mac256 = Hmac::<Sha256>::new(&key);
            for chunk in data.chunks(7) {
                mac256.update(chunk);
            }
            let incremental256 = hex::encode(mac256.finalize().as_ref());
            assert_eq!(incremental256, single256, "{} incremental sha256", vector.source);

            let mut mac512 = Hmac::<Sha512>::new(&key);
            for chunk in data.chunks(13) {
                mac512.update(chunk);
            }
            let incremental512 = hex::encode(mac512.finalize().as_ref());
            assert_eq!(incremental512, single512, "{} incremental sha512", vector.source);
        }
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn hmac_sha256_wycheproof() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hmac_sha256_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let tag_size_bits = group["tagSize"].as_u64().unwrap();
            let tag_size_bytes = (tag_size_bits / 8) as usize;
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let expected_tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key = hex::decode(key_hex).unwrap();
                let msg = hex::decode(msg_hex).unwrap();

                let computed = Hmac::<Sha256>::mac(&key, &msg);
                let computed_tag = hex::encode(&computed.as_ref()[..tag_size_bytes]);

                if result == "valid" {
                    assert_eq!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA-256 tcId={} tagSize={}",
                        test["tcId"], tag_size_bits
                    );
                    valid_tested += 1;
                } else {
                    assert_ne!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA-256 tcId={} ModifiedTag not detected",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HMAC-SHA-256 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HMAC-SHA-256 wycheproof tests were run");
    }

    #[test]
    fn hmac_sha512_wycheproof() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hmac_sha512_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let tag_size_bits = group["tagSize"].as_u64().unwrap();
            let tag_size_bytes = (tag_size_bits / 8) as usize;
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let expected_tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key = hex::decode(key_hex).unwrap();
                let msg = hex::decode(msg_hex).unwrap();

                let computed = Hmac::<Sha512>::mac(&key, &msg);
                let computed_tag = hex::encode(&computed.as_ref()[..tag_size_bytes]);

                if result == "valid" {
                    assert_eq!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA-512 tcId={} tagSize={}",
                        test["tcId"], tag_size_bits
                    );
                    valid_tested += 1;
                } else {
                    assert_ne!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA-512 tcId={} ModifiedTag not detected",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HMAC-SHA-512 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HMAC-SHA-512 wycheproof tests were run");
    }

    #[test]
    fn hmac_sha384_wycheproof() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hmac_sha384_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let tag_size_bits = group["tagSize"].as_u64().unwrap();
            let tag_size_bytes = (tag_size_bits / 8) as usize;
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let expected_tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key = hex::decode(key_hex).unwrap();
                let msg = hex::decode(msg_hex).unwrap();

                let computed = Hmac::<Sha384>::mac(&key, &msg);
                let computed_tag = hex::encode(&computed.as_ref()[..tag_size_bytes]);

                if result == "valid" {
                    assert_eq!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA-384 tcId={} tagSize={}",
                        test["tcId"], tag_size_bits
                    );
                    valid_tested += 1;
                } else {
                    assert_ne!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA-384 tcId={} ModifiedTag not detected",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HMAC-SHA-384 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HMAC-SHA-384 wycheproof tests were run");
    }

    #[test]
    fn hmac_sha3_256_wycheproof() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hmac_sha3_256_test.json"))
                .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let tag_size_bits = group["tagSize"].as_u64().unwrap();
            let tag_size_bytes = (tag_size_bits / 8) as usize;
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let expected_tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key = hex::decode(key_hex).unwrap();
                let msg = hex::decode(msg_hex).unwrap();

                let computed = Hmac::<Sha3_256>::mac(&key, &msg);
                let computed_tag = hex::encode(&computed.as_ref()[..tag_size_bytes]);

                if result == "valid" {
                    assert_eq!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA3-256 tcId={} tagSize={}",
                        test["tcId"], tag_size_bits
                    );
                    valid_tested += 1;
                } else {
                    assert_ne!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA3-256 tcId={} ModifiedTag not detected",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HMAC-SHA3-256 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HMAC-SHA3-256 wycheproof tests were run");
    }

    #[test]
    fn hmac_sha3_512_wycheproof() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/hmac_sha3_512_test.json"))
                .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let tag_size_bits = group["tagSize"].as_u64().unwrap();
            let tag_size_bytes = (tag_size_bits / 8) as usize;
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let expected_tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key = hex::decode(key_hex).unwrap();
                let msg = hex::decode(msg_hex).unwrap();

                let computed = Hmac::<Sha3_512>::mac(&key, &msg);
                let computed_tag = hex::encode(&computed.as_ref()[..tag_size_bytes]);

                if result == "valid" {
                    assert_eq!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA3-512 tcId={} tagSize={}",
                        test["tcId"], tag_size_bits
                    );
                    valid_tested += 1;
                } else {
                    assert_ne!(
                        computed_tag, expected_tag_hex,
                        "wycheproof HMAC-SHA3-512 tcId={} ModifiedTag not detected",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid HMAC-SHA3-512 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid HMAC-SHA3-512 wycheproof tests were run");
    }
}
