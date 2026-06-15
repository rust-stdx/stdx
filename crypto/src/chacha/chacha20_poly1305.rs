use super::{ChaCha, hchacha20};
use crate::{Aead, AeadError, Hash, StreamCipher, bytes::Bytes, poly1305::Poly1305};

/// ChaCha20-Poly1305 AEAD as specified in RFC 8439.
///
/// # Parameters
///
/// - Key: 256 bits (32 bytes)
/// - Nonce: 96 bits (12 bytes)
/// - Tag: 128 bits (16 bytes)
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct ChaCha20Poly1305 {
    key: [u8; 32],
}

impl ChaCha20Poly1305 {
    /// Creates a new AEAD instance from a 32-byte key.
    pub fn new(key: &[u8; 32]) -> ChaCha20Poly1305 {
        return ChaCha20Poly1305 {
            key: *key,
        };
    }

    /// Generates the one-time Poly1305 key using ChaCha20 with counter=0.
    fn poly1305_key_gen(&self, nonce: &[u8; 12]) -> [u8; 32] {
        let mut cipher = ChaCha::<20, true>::new(&self.key, nonce);
        cipher.set_counter(0);
        let mut block = [0u8; 64];
        cipher.xor_keystream(&mut block);
        let mut key = [0u8; 32];
        key.copy_from_slice(&block[..32]);
        return key;
    }
}

impl Aead for ChaCha20Poly1305 {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        let nonce: &[u8; 12] = nonce.try_into().expect("nonce must be 12 bytes");
        let otk = self.poly1305_key_gen(nonce);

        let mut cipher = ChaCha::<20, true>::new(&self.key, nonce);
        cipher.set_counter(1);
        cipher.xor_keystream(in_out);

        let mut mac = Poly1305::new(&otk);
        update_padded(&mut mac, aad);
        update_padded(&mut mac, in_out);
        mac.update(&(aad.len() as u64).to_le_bytes());
        mac.update(&(in_out.len() as u64).to_le_bytes());
        let tag_bytes = mac.finalize();

        let mut tag = Hash(Bytes::<64>::with_length(16));
        tag.as_mut().copy_from_slice(&tag_bytes);
        return tag;
    }

    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        if tag.len() != Self::TAG_SIZE {
            return Err(AeadError::InvalidCiphertext);
        }
        let nonce: &[u8; 12] = nonce.try_into().map_err(|_| AeadError::InvalidNonce)?;
        let otk = self.poly1305_key_gen(nonce);

        let mut mac = Poly1305::new(&otk);
        update_padded(&mut mac, aad);
        update_padded(&mut mac, in_out);
        mac.update(&(aad.len() as u64).to_le_bytes());
        mac.update(&(in_out.len() as u64).to_le_bytes());
        let computed = mac.finalize();

        if !constant_time_eq::constant_time_eq(&computed, tag) {
            return Err(AeadError::InvalidCiphertext);
        }

        let mut cipher = ChaCha::<20, true>::new(&self.key, nonce);
        cipher.set_counter(1);
        cipher.xor_keystream(in_out);

        return Ok(());
    }
}

/// XChaCha20-Poly1305 AEAD (draft-irtf-cfrg-xchacha-03).
///
/// Extends ChaCha20-Poly1305 with a 24-byte (192-bit) nonce.
/// Internally uses HChaCha20 to derive a subkey from the first 16 nonce bytes.
///
/// # Parameters
///
/// - Key: 256 bits (32 bytes)
/// - Nonce: 192 bits (24 bytes)
/// - Tag: 128 bits (16 bytes)
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct XChaCha20Poly1305 {
    key: [u8; 32],
}

impl XChaCha20Poly1305 {
    pub fn new(key: &[u8; 32]) -> XChaCha20Poly1305 {
        return XChaCha20Poly1305 {
            key: *key,
        };
    }

    fn derive_subkey(&self, nonce: &[u8; 24]) -> ([u8; 32], [u8; 12]) {
        let subkey = hchacha20(&self.key, nonce[..16].try_into().unwrap());
        let mut ietf_nonce = [0u8; 12];
        ietf_nonce[4..12].copy_from_slice(&nonce[16..24]);
        return (subkey, ietf_nonce);
    }
}

impl Aead for XChaCha20Poly1305 {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 24;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        let nonce: &[u8; 24] = nonce.try_into().expect("nonce must be 24 bytes");
        let (subkey, ietf_nonce) = self.derive_subkey(nonce);

        let mut keygen = ChaCha::<20, true>::new(&subkey, &ietf_nonce);
        keygen.set_counter(0);
        let mut block = [0u8; 64];
        keygen.xor_keystream(&mut block);
        let mut otk = [0u8; 32];
        otk.copy_from_slice(&block[..32]);

        let mut cipher = ChaCha::<20, true>::new(&subkey, &ietf_nonce);
        cipher.set_counter(1);
        cipher.xor_keystream(in_out);

        let mut mac = Poly1305::new(&otk);
        update_padded(&mut mac, aad);
        update_padded(&mut mac, in_out);
        mac.update(&(aad.len() as u64).to_le_bytes());
        mac.update(&(in_out.len() as u64).to_le_bytes());
        let tag_bytes = mac.finalize();

        let mut tag = Hash(Bytes::<64>::with_length(16));
        tag.as_mut().copy_from_slice(&tag_bytes);
        return tag;
    }

    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        if tag.len() != Self::TAG_SIZE {
            return Err(AeadError::InvalidCiphertext);
        }
        let nonce: &[u8; 24] = nonce.try_into().map_err(|_| AeadError::InvalidNonce)?;
        let (subkey, ietf_nonce) = self.derive_subkey(nonce);

        let mut keygen = ChaCha::<20, true>::new(&subkey, &ietf_nonce);
        keygen.set_counter(0);
        let mut block = [0u8; 64];
        keygen.xor_keystream(&mut block);
        let mut otk = [0u8; 32];
        otk.copy_from_slice(&block[..32]);

        let mut mac = Poly1305::new(&otk);
        update_padded(&mut mac, aad);
        update_padded(&mut mac, in_out);
        mac.update(&(aad.len() as u64).to_le_bytes());
        mac.update(&(in_out.len() as u64).to_le_bytes());
        let computed = mac.finalize();

        if !constant_time_eq::constant_time_eq(&computed, tag) {
            in_out.fill(0);
            return Err(AeadError::InvalidCiphertext);
        }

        let mut cipher = ChaCha::<20, true>::new(&subkey, &ietf_nonce);
        cipher.set_counter(1);
        cipher.xor_keystream(in_out);

        return Ok(());
    }
}

fn update_padded(mac: &mut Poly1305, data: &[u8]) {
    mac.update(data);
    let rem = data.len() % 16;
    if rem != 0 {
        let pad = [0u8; 15];
        mac.update(&pad[..16 - rem]);
    }
}

#[cfg(test)]
mod test {
    use super::ChaCha20Poly1305;
    use crate::Aead;

    /// RFC 8439 Appendix A.4: Poly1305 Key Generation Using ChaCha20.
    #[test]
    fn poly1305_key_gen_vectors() {
        struct KeyGenTest {
            key_hex: &'static str,
            nonce_hex: &'static str,
            expected_otk_hex: &'static str,
        }

        let tests = [
            KeyGenTest {
                key_hex: "0000000000000000000000000000000000000000000000000000000000000000",
                nonce_hex: "000000000000000000000000",
                expected_otk_hex: "76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7",
            },
            KeyGenTest {
                key_hex: "0000000000000000000000000000000000000000000000000000000000000001",
                nonce_hex: "000000000000000000000002",
                expected_otk_hex: "ecfa254f845f647473d3cb140da9e87606cb33066c447b87bc2666dde3fbb739",
            },
            KeyGenTest {
                key_hex: "1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0",
                nonce_hex: "000000000000000000000002",
                expected_otk_hex: "965e3bc6f9ec7ed9560808f4d229f94b137ff275ca9b3fcbdd59deaad23310ae",
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let key: [u8; 32] = hex::decode(test.key_hex).unwrap().try_into().unwrap();
            let nonce: [u8; 12] = hex::decode(test.nonce_hex).unwrap().try_into().unwrap();

            let ae = ChaCha20Poly1305::new(&key);
            let otk = ae.poly1305_key_gen(&nonce);
            let expected_otk = hex::decode(test.expected_otk_hex).unwrap();

            assert_eq!(otk.as_slice(), expected_otk.as_slice(), "key gen test [{i}] failed");
        }
    }

    /// RFC 8439 Appendix A.5: ChaCha20-Poly1305 AEAD Decryption.
    #[test]
    fn aead_decrypt_test() {
        let key: [u8; 32] = hex::decode("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0")
            .unwrap()
            .try_into()
            .unwrap();
        let nonce: [u8; 12] = hex::decode("000000000102030405060708").unwrap().try_into().unwrap();
        let aad = hex::decode("f33388860000000000004e91").unwrap();

        let ciphertext = hex::decode(concat!(
            "64a0861575861af460f062c79be643bd5e805cfd345cf389f108670ac76c8cb2",
            "4c6cfc18755d43eea09ee94e382d26b0bdb7b73c321b0100d4f03b7f355894cf",
            "332f830e710b97ce98c8a84abd0b948114ad176e008d33bd60f982b1ff37c855",
            "9797a06ef4f0ef61c186324e2b3506383606907b6a7c02b0f9f6157b53c867e4",
            "b9166c767b804d46a59b5216cde7a4e99040c5a40433225ee282a1b0a06c523e",
            "af4534d7f83fa1155b0047718cbc546a0d072b04b3564eea1b422273f548271a",
            "0bb2316053fa76991955ebd63159434ecebb4e466dae5a1073a6727627097a10",
            "49e617d91d361094fa68f0ff77987130305beaba2eda04df997b714d6c6f2c29",
            "a6ad5cb4022b02709b",
        ))
        .unwrap();

        let tag = hex::decode("eead9d67890cbb22392336fea1851f38").unwrap();

        let ae = ChaCha20Poly1305::new(&key);

        let mut decrypted = ciphertext.clone();
        ae.decrypt_in_place(&mut decrypted, &nonce, &aad, &tag).unwrap();

        let expected_plaintext = hex::decode(concat!(
            "496e7465726e65742d4472616674732061726520647261667420646f63756d65",
            "6e74732076616c696420666f722061206d6178696d756d206f6620736978206d",
            "6f6e74687320616e64206d617920626520757064617465642c207265706c6163",
            "65642c206f72206f62736f6c65746564206279206f7468657220646f63756d65",
            "6e747320617420616e792074696d652e20497420697320696e617070726f7072",
            "6961746520746f2075736520496e7465726e65742d4472616674732061732072",
            "65666572656e6365206d6174657269616c206f7220746f206369746520746865",
            "6d206f74686572207468616e206173202fe2809c776f726b20696e2070726f67",
            "726573732e2fe2809d",
        ))
        .unwrap();

        assert_eq!(decrypted, expected_plaintext);
    }

    /// Round-trip: encrypt then decrypt.
    #[test]
    fn aead_roundtrip() {
        let key: [u8; 32] = [0x55; 32];
        let nonce: [u8; 12] = [0xaa; 12];
        let aad = b"associated data";
        let plaintext = b"hello, world!";

        let ae = ChaCha20Poly1305::new(&key);

        let mut ciphertext = plaintext.to_vec();
        let tag = ae.encrypt_in_place(&mut ciphertext, &nonce, aad);

        let mut decrypted = ciphertext.clone();
        ae.decrypt_in_place(&mut decrypted, &nonce, aad, tag.as_ref()).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    /// Tampered tag should fail.
    #[test]
    fn aead_tampered_tag_fails() {
        let key: [u8; 32] = [0x55; 32];
        let nonce: [u8; 12] = [0xaa; 12];

        let ae = ChaCha20Poly1305::new(&key);

        let mut ciphertext = b"secret".to_vec();
        let mut tag = ae.encrypt_in_place(&mut ciphertext, &nonce, b"");
        tag.as_mut()[0] ^= 1;

        let mut decrypted = ciphertext.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &nonce, b"", tag.as_ref());
        assert!(result.is_err());
    }

    /// Tampered ciphertext should fail.
    #[test]
    fn aead_tampered_ciphertext_fails() {
        let key: [u8; 32] = [0x55; 32];
        let nonce: [u8; 12] = [0xaa; 12];

        let ae = ChaCha20Poly1305::new(&key);

        let mut ciphertext = b"secret".to_vec();
        let tag = ae.encrypt_in_place(&mut ciphertext, &nonce, b"");

        ciphertext[3] ^= 1;

        let mut decrypted = ciphertext.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &nonce, b"", tag.as_ref());
        assert!(result.is_err());
    }

    /// Wrong nonce should fail decryption.
    #[test]
    fn aead_wrong_nonce_fails() {
        let key: [u8; 32] = [0x55; 32];

        let ae = ChaCha20Poly1305::new(&key);

        let mut ciphertext = b"secret".to_vec();
        let tag = ae.encrypt_in_place(&mut ciphertext, &[0u8; 12], b"");

        let mut decrypted = ciphertext.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &[1u8; 12], b"", tag.as_ref());
        assert!(result.is_err());
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn wycheproof_chacha20_poly1305_vectors() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/chacha20_poly1305_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["keySize"].as_u64() != Some(256) {
                continue;
            }
            if group["ivSize"].as_u64() != Some(96) {
                continue;
            }
            if group["tagSize"].as_u64() != Some(128) {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let iv_hex = test["iv"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let aad_hex = test["aad"].as_str().unwrap();
                let ct_hex = test["ct"].as_str().unwrap();
                let tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key: [u8; 32] = hex::decode(key_hex).unwrap().try_into().unwrap();
                let nonce: [u8; 12] = hex::decode(iv_hex).unwrap().try_into().unwrap();
                let expected_ct = hex::decode(ct_hex).unwrap();
                let expected_tag: [u8; 16] = hex::decode(tag_hex).unwrap().try_into().unwrap();
                let pt = hex::decode(msg_hex).unwrap();
                let aad = hex::decode(aad_hex).unwrap();

                let cipher = ChaCha20Poly1305::new(&key);

                if result == "valid" {
                    let mut buf = pt.clone();
                    let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad);
                    assert_eq!(
                        buf, expected_ct,
                        "wycheproof ChaCha20-Poly1305 tcId={} ct mismatch",
                        test["tcId"]
                    );
                    assert_eq!(
                        tag.as_ref(),
                        &expected_tag[..],
                        "wycheproof ChaCha20-Poly1305 tcId={} tag mismatch",
                        test["tcId"]
                    );

                    let mut buf2 = expected_ct.clone();
                    cipher
                        .decrypt_in_place(&mut buf2, &nonce, &aad, &expected_tag)
                        .expect("wycheproof ChaCha20-Poly1305 decrypt failed");
                    assert_eq!(buf2, pt, "wycheproof ChaCha20-Poly1305 tcId={} pt mismatch", test["tcId"]);
                    valid_tested += 1;
                } else {
                    let mut buf = expected_ct.clone();
                    let res = cipher.decrypt_in_place(&mut buf, &nonce, &aad, &expected_tag);
                    assert!(
                        res.is_err(),
                        "wycheproof ChaCha20-Poly1305 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ChaCha20-Poly1305 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ChaCha20-Poly1305 wycheproof tests were run");
    }
}

#[cfg(test)]
mod xchacha20poly1305_tests {
    use super::XChaCha20Poly1305;
    use crate::{Aead, StreamCipher};

    /// draft-irtf-cfrg-xchacha-03, Appendix A.3.1: AEAD_XCHACHA20_POLY1305
    #[test]
    fn aead_xchacha20poly1305_test_vector() {
        let key: [u8; 32] = hex::decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f")
            .unwrap()
            .try_into()
            .unwrap();
        let nonce: [u8; 24] = hex::decode("404142434445464748494a4b4c4d4e4f5051525354555657")
            .unwrap()
            .try_into()
            .unwrap();
        let aad = hex::decode("50515253c0c1c2c3c4c5c6c7").unwrap();

        let plaintext = hex::decode(concat!(
            "4c616469657320616e642047656e746c",
            "656d656e206f662074686520636c6173",
            "73206f66202739393a20496620492063",
            "6f756c64206f6666657220796f75206f",
            "6e6c79206f6e652074697020666f7220",
            "746865206675747572652c2073756e73",
            "637265656e20776f756c642062652069",
            "742e",
        ))
        .unwrap();

        // Verify Poly1305 key derivation
        let ae = XChaCha20Poly1305::new(&key);
        let (subkey, _ietf_nonce) = ae.derive_subkey(&nonce);
        // The draft gives the Poly1305 key (first 32 bytes of ChaCha20 block 0)
        let mut keygen = crate::chacha::ChaCha::<20, true>::new(&subkey, &_ietf_nonce);
        keygen.set_counter(0);
        let mut block = [0u8; 64];
        keygen.xor_keystream(&mut block);
        let expected_otk = hex::decode("7b191f80f361f099094f6f4b8fb97df847cc6873a8f2b190dd73807183f907d5").unwrap();
        assert_eq!(&block[..32], expected_otk.as_slice(), "Poly1305 key derivation failed");

        // Encrypt
        let mut ciphertext = plaintext.clone();
        let tag = ae.encrypt_in_place(&mut ciphertext, &nonce, &aad);

        let expected_ciphertext = hex::decode(concat!(
            "bd6d179d3e83d43b9576579493c0e939572a1700252bfaccbed2902c21396cbb",
            "731c7f1b0b4aa6440bf3a82f4eda7e39ae64c6708c54c216cb96b72e1213b452",
            "2f8c9ba40db5d945b11b69b982c1bb9e3f3fac2bc369488f76b2383565d3fff9",
            "21f9664c97637da9768812f615c68b13b52e",
        ))
        .unwrap();
        assert_eq!(ciphertext, expected_ciphertext, "ciphertext mismatch");

        let expected_tag = hex::decode("c0875924c1c7987947deafd8780acf49").unwrap();
        assert_eq!(tag.as_ref(), expected_tag.as_slice(), "tag mismatch");

        // Decrypt
        let mut decrypted = ciphertext.clone();
        ae.decrypt_in_place(&mut decrypted, &nonce, &aad, tag.as_ref()).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn aead_xchacha20poly1305_roundtrip() {
        let key: [u8; 32] = [0x55; 32];
        let nonce: [u8; 24] = [0xaa; 24];
        let aad = b"associated data";
        let plaintext = b"hello, world!";

        let ae = XChaCha20Poly1305::new(&key);

        let mut ciphertext = plaintext.to_vec();
        let tag = ae.encrypt_in_place(&mut ciphertext, &nonce, aad);

        let mut decrypted = ciphertext.clone();
        ae.decrypt_in_place(&mut decrypted, &nonce, aad, tag.as_ref()).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn aead_xchacha20poly1305_tampered_tag_fails() {
        let key: [u8; 32] = [0x55; 32];
        let nonce: [u8; 24] = [0xaa; 24];

        let ae = XChaCha20Poly1305::new(&key);

        let mut ciphertext = b"secret".to_vec();
        let mut tag = ae.encrypt_in_place(&mut ciphertext, &nonce, b"");
        tag.as_mut()[0] ^= 1;

        let mut decrypted = ciphertext.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &nonce, b"", tag.as_ref());
        assert!(result.is_err());
        assert!(decrypted.iter().all(|b| *b == 0));
    }

    #[test]
    fn aead_xchacha20poly1305_wrong_nonce_fails() {
        let key: [u8; 32] = [0x55; 32];

        let ae = XChaCha20Poly1305::new(&key);

        let mut ciphertext = b"secret".to_vec();
        let tag = ae.encrypt_in_place(&mut ciphertext, &[0u8; 24], b"");

        let mut decrypted = ciphertext.clone();
        let result = ae.decrypt_in_place(&mut decrypted, &[1u8; 24], b"", tag.as_ref());
        assert!(result.is_err());
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn wycheproof_xchacha20_poly1305_vectors() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/xchacha20_poly1305_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["keySize"].as_u64() != Some(256) {
                continue;
            }
            if group["ivSize"].as_u64() != Some(192) {
                continue;
            }
            if group["tagSize"].as_u64() != Some(128) {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let key_hex = test["key"].as_str().unwrap();
                let iv_hex = test["iv"].as_str().unwrap();
                let msg_hex = test["msg"].as_str().unwrap();
                let aad_hex = test["aad"].as_str().unwrap();
                let ct_hex = test["ct"].as_str().unwrap();
                let tag_hex = test["tag"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let key: [u8; 32] = hex::decode(key_hex).unwrap().try_into().unwrap();
                let nonce: [u8; 24] = hex::decode(iv_hex).unwrap().try_into().unwrap();
                let expected_ct = hex::decode(ct_hex).unwrap();
                let expected_tag: [u8; 16] = hex::decode(tag_hex).unwrap().try_into().unwrap();
                let pt = hex::decode(msg_hex).unwrap();
                let aad = hex::decode(aad_hex).unwrap();

                let cipher = XChaCha20Poly1305::new(&key);

                if result == "valid" {
                    let mut buf = pt.clone();
                    let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad);
                    assert_eq!(
                        buf, expected_ct,
                        "wycheproof XChaCha20-Poly1305 tcId={} ct mismatch",
                        test["tcId"]
                    );
                    assert_eq!(
                        tag.as_ref(),
                        &expected_tag[..],
                        "wycheproof XChaCha20-Poly1305 tcId={} tag mismatch",
                        test["tcId"]
                    );

                    let mut buf2 = expected_ct.clone();
                    cipher
                        .decrypt_in_place(&mut buf2, &nonce, &aad, &expected_tag)
                        .expect("wycheproof XChaCha20-Poly1305 decrypt failed");
                    assert_eq!(buf2, pt, "wycheproof XChaCha20-Poly1305 tcId={} pt mismatch", test["tcId"]);
                    valid_tested += 1;
                } else {
                    let mut buf = expected_ct.clone();
                    let res = cipher.decrypt_in_place(&mut buf, &nonce, &aad, &expected_tag);
                    assert!(
                        res.is_err(),
                        "wycheproof XChaCha20-Poly1305 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid XChaCha20-Poly1305 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid XChaCha20-Poly1305 wycheproof tests were run");
    }
}
