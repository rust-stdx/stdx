use super::{
    aes::{RCON, SBOX, TE0, TE1, TE2, TE3},
    ghash::{compute_tag, precompute_ghash_table},
};
use crate::{Aead, AeadError, Hash};

/// AES-128-GCM authenticated cipher.
pub struct Aes128Gcm {
    key: [u8; 16],
    round_keys: [[u8; 16]; 11],
}

/// AES-128 key expansion. Returns 11 round keys.
pub fn key_expand_128(key: &[u8; 16]) -> [[u8; 16]; 11] {
    let mut w = [[0u8; 4]; 44];
    for i in 0..4 {
        w[i] = [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];
    }
    for i in 4..44 {
        let mut temp = w[i - 1];
        if i % 4 == 0 {
            temp = [
                SBOX[temp[1] as usize],
                SBOX[temp[2] as usize],
                SBOX[temp[3] as usize],
                SBOX[temp[0] as usize],
            ];
            temp[0] ^= RCON[i / 4];
        }
        w[i] = [
            w[i - 4][0] ^ temp[0],
            w[i - 4][1] ^ temp[1],
            w[i - 4][2] ^ temp[2],
            w[i - 4][3] ^ temp[3],
        ];
    }
    let mut rk = [[0u8; 16]; 11];
    for i in 0..11 {
        for j in 0..4 {
            rk[i][4 * j..4 * j + 4].copy_from_slice(&w[4 * i + j]);
        }
    }
    rk
}

/// Encrypt one 16-byte block using AES-128 (T-table accelerated).
pub fn encrypt_block_aes128(rk: &[[u8; 16]; 11], block: &[u8; 16]) -> [u8; 16] {
    let mut s = *block;
    for i in 0..16 {
        s[i] ^= rk[0][i];
    }
    for r in 1..10 {
        let t0 = TE0[s[0] as usize]
            ^ TE1[s[5] as usize]
            ^ TE2[s[10] as usize]
            ^ TE3[s[15] as usize]
            ^ u32::from_ne_bytes(rk[r][0..4].try_into().unwrap());
        let t1 = TE0[s[4] as usize]
            ^ TE1[s[9] as usize]
            ^ TE2[s[14] as usize]
            ^ TE3[s[3] as usize]
            ^ u32::from_ne_bytes(rk[r][4..8].try_into().unwrap());
        let t2 = TE0[s[8] as usize]
            ^ TE1[s[13] as usize]
            ^ TE2[s[2] as usize]
            ^ TE3[s[7] as usize]
            ^ u32::from_ne_bytes(rk[r][8..12].try_into().unwrap());
        let t3 = TE0[s[12] as usize]
            ^ TE1[s[1] as usize]
            ^ TE2[s[6] as usize]
            ^ TE3[s[11] as usize]
            ^ u32::from_ne_bytes(rk[r][12..16].try_into().unwrap());
        s[0..4].copy_from_slice(&t0.to_ne_bytes());
        s[4..8].copy_from_slice(&t1.to_ne_bytes());
        s[8..12].copy_from_slice(&t2.to_ne_bytes());
        s[12..16].copy_from_slice(&t3.to_ne_bytes());
    }
    s = [
        SBOX[s[0] as usize] ^ rk[10][0],
        SBOX[s[5] as usize] ^ rk[10][1],
        SBOX[s[10] as usize] ^ rk[10][2],
        SBOX[s[15] as usize] ^ rk[10][3],
        SBOX[s[4] as usize] ^ rk[10][4],
        SBOX[s[9] as usize] ^ rk[10][5],
        SBOX[s[14] as usize] ^ rk[10][6],
        SBOX[s[3] as usize] ^ rk[10][7],
        SBOX[s[8] as usize] ^ rk[10][8],
        SBOX[s[13] as usize] ^ rk[10][9],
        SBOX[s[2] as usize] ^ rk[10][10],
        SBOX[s[7] as usize] ^ rk[10][11],
        SBOX[s[12] as usize] ^ rk[10][12],
        SBOX[s[1] as usize] ^ rk[10][13],
        SBOX[s[6] as usize] ^ rk[10][14],
        SBOX[s[11] as usize] ^ rk[10][15],
    ];
    s
}

impl Aes128Gcm {
    pub const KEY_SIZE: usize = 16;

    pub fn new(key: &[u8; 16]) -> Self {
        Aes128Gcm {
            key: *key,
            round_keys: key_expand_128(key),
        }
    }

    pub(crate) fn encrypt_in_place_soft(&self, in_out: &mut [u8], nonce: &[u8; 12], aad: &[u8]) -> Hash {
        let h = encrypt_block_aes128(&self.round_keys, &[0u8; 16]);
        let ghash_table = precompute_ghash_table(&h);

        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block_aes128(&self.round_keys, &j0);

        j0[15] = 2;
        aes128_ctr_xor(&self.key, &self.round_keys, &j0, in_out);

        compute_tag(&ghash_table, aad, in_out, &ej0)
    }

    pub(crate) fn decrypt_in_place_soft(
        &self,
        in_out: &mut [u8],
        tag: &[u8; 16],
        nonce: &[u8; 12],
        aad: &[u8],
    ) -> Result<(), AeadError> {
        let h = encrypt_block_aes128(&self.round_keys, &[0u8; 16]);
        let ghash_table = precompute_ghash_table(&h);

        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block_aes128(&self.round_keys, &j0);

        let expected_tag = compute_tag(&ghash_table, aad, in_out, &ej0);

        let mut diff = 0u8;
        for i in 0..16 {
            diff |= expected_tag.as_ref()[i] ^ tag[i];
        }
        if diff != 0 {
            return Err(AeadError::InvalidCiphertext);
        }

        j0[15] = 2;
        aes128_ctr_xor(&self.key, &self.round_keys, &j0, in_out);

        Ok(())
    }
}

fn aes128_ctr_xor(key: &[u8; 16], rk: &[[u8; 16]; 11], counter: &[u8; 16], in_out: &mut [u8]) {
    let mut ctr = *counter;
    let n = in_out.len();
    let mut i = 0;
    while i + 16 <= n {
        let ks = encrypt_block_aes128(rk, &ctr);
        for j in 0..16 {
            in_out[i + j] ^= ks[j];
        }
        for j in (0..16).rev() {
            ctr[j] = ctr[j].wrapping_add(1);
            if ctr[j] != 0 {
                break;
            }
        }
        i += 16;
    }
    if i < n {
        let ks = encrypt_block_aes128(rk, &ctr);
        for j in 0..(n - i) {
            in_out[i + j] ^= ks[j];
        }
    }
}

impl Aead for Aes128Gcm {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        assert_eq!(nonce.len(), 12, "AES-128-GCM nonce must be 12 bytes");
        let nonce_arr: &[u8; 12] = nonce.try_into().unwrap();
        self.encrypt_in_place_soft(in_out, nonce_arr, aad)
    }

    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        assert_eq!(nonce.len(), 12, "AES-128-GCM nonce must be 12 bytes");
        let nonce_arr: &[u8; 12] = nonce.try_into().unwrap();
        let tag_arr: &[u8; 16] = tag.try_into().expect("AES-128-GCM tag must be 16 bytes");
        self.decrypt_in_place_soft(in_out, tag_arr, nonce_arr, aad)
    }
}

#[cfg(test)]
mod tests {
    use hex;

    use super::*;

    include!("aes_128_gcm_vectors.rs");

    #[test]
    fn aes128_encrypt_block_vector() {
        // NIST FIPS 197 Appendix B: AES-128 K=0, P=0 -> C=66e94bd4ef8a2c3b884cfa59ca342b2e
        let key = [0u8; 16];
        let pt = [0u8; 16];
        let rk = key_expand_128(&key);
        let ct = encrypt_block_aes128(&rk, &pt);
        let expected = hex::decode_array::<16>(b"66e94bd4ef8a2c3b884cfa59ca342b2e").unwrap();
        assert_eq!(ct, expected, "AES-128 encrypt K=0, P=0 mismatch");
    }

    #[test]
    fn aes128_fips197_vector() {
        let key = hex::decode_array::<16>(b"2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        let pt = hex::decode_array::<16>(b"3243f6a8885a308d313198a2e0370734").unwrap();
        let rk = key_expand_128(&key);

        // Verify round keys as hex strings for readability
        let rk_hex: Vec<String> = rk.iter().map(|rk_i| hex::encode(rk_i)).collect();
        assert_eq!(rk_hex[0], "2b7e151628aed2a6abf7158809cf4f3c", "rk0");
        assert_eq!(rk_hex[1], "a0fafe1788542cb123a339392a6c7605", "rk1");
        assert_eq!(rk_hex[2], "f2c295f27a96b9435935807a7359f67f", "rk2");

        let ct = encrypt_block_aes128(&rk, &pt);
        let ct_hex = hex::encode(&ct);
        assert_eq!(
            ct_hex, "3925841d02dc09fbdc118597196a0b32",
            "AES-128 FIPS 197 encrypt failed, rk3={}",
            rk_hex[3]
        );
    }

    fn run_gcm_vector(v: &Gcm128Vector) {
        let key: [u8; 16] = hex::decode_array::<16>(v.key.as_bytes()).unwrap();
        let nonce: [u8; 12] = hex::decode_array::<12>(v.nonce.as_bytes()).unwrap();
        let pt = hex::decode(v.pt).unwrap();
        let aad = hex::decode(v.aad).unwrap();
        let expected_ct = hex::decode(v.ct).unwrap();
        let expected_tag: [u8; 16] = hex::decode_array::<16>(v.tag.as_bytes()).unwrap();

        let cipher = Aes128Gcm::new(&key);

        let mut buf = pt.clone();
        let tag = cipher.encrypt_in_place_soft(&mut buf, &nonce, &aad);
        assert_eq!(buf, expected_ct, "ciphertext mismatch for key={}", v.key);
        assert_eq!(tag.as_ref(), &expected_tag[..], "tag mismatch for key={}", v.key);

        let mut buf2 = expected_ct.clone();
        cipher
            .decrypt_in_place_soft(&mut buf2, &expected_tag, &nonce, &aad)
            .expect("decrypt failed");
        assert_eq!(buf2, pt, "plaintext mismatch after decrypt for key={}", v.key);
    }

    #[test]
    fn aes128_gcm_roundtrip() {
        let key = [0xabu8; 16];
        let nonce = [0x01u8; 16 - 4]; // 12 bytes
        let aad = b"additional data";
        let plaintext: Vec<u8> = (0u8..=255u8).cycle().take(1024).collect();

        let cipher = Aes128Gcm::new(&key);
        let mut buf = plaintext.clone();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad);
        let tag_bytes: [u8; 16] = tag.as_ref().try_into().unwrap();
        cipher
            .decrypt_in_place(&mut buf, &nonce, aad, &tag_bytes)
            .expect("decrypt failed");
        assert_eq!(buf, plaintext);
    }

    #[test]
    fn aes128_gcm_empty_plaintext() {
        let key = [0x01u8; 16];
        let nonce = [0x02u8; 12];
        let aad = b"test";
        let cipher = Aes128Gcm::new(&key);
        let mut buf: Vec<u8> = vec![];
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad);
        let tag_bytes: [u8; 16] = tag.as_ref().try_into().unwrap();
        cipher
            .decrypt_in_place(&mut buf, &nonce, aad, &tag_bytes)
            .expect("decrypt failed");
    }

    #[test]
    fn aes128_gcm_tag_mismatch_returns_error() {
        let key = [0u8; 16];
        let nonce = [0u8; 12];
        let cipher = Aes128Gcm::new(&key);
        let mut buf = b"hello world".to_vec();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, &[]);
        let mut bad_tag: [u8; 16] = tag.as_ref().try_into().unwrap();
        bad_tag[0] ^= 0xff;
        let mut buf2 = buf.clone();
        assert!(cipher.decrypt_in_place(&mut buf2, &nonce, &[], &bad_tag).is_err());
    }

    #[test]
    fn aes128_gcm_nist_vectors() {
        for v in NIST_GCM_128_VECTORS.iter() {
            run_gcm_vector(v);
        }
    }

    #[test]
    fn aes128_block_encrypt_k0_j0() {
        let key = [0u8; 16];
        let j0: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let rk = key_expand_128(&key);
        let ct = encrypt_block_aes128(&rk, &j0);
        let expected = hex::decode_array::<16>(b"58e2fccefa7e3061367f1d57a4e7455a").unwrap();
        assert_eq!(ct, expected, "AES(K=0, J0=0...01) mismatch");
    }

    #[test]
    fn aes128_debug_round_state() {
        let key = [0u8; 16];
        let pt = [0u8; 16];
        let rk = key_expand_128(&key);

        // Compute first T-table round manually
        let mut s = pt;
        for i in 0..16 {
            s[i] ^= rk[0][i];
        }
        assert_eq!(s, [0; 16], "after AddRoundKey with K=0, state should be all zeros");

        // Round 1
        let r = 1usize;
        let t0 = TE0[s[0] as usize]
            ^ TE1[s[5] as usize]
            ^ TE2[s[10] as usize]
            ^ TE3[s[15] as usize]
            ^ u32::from_ne_bytes(rk[r][0..4].try_into().unwrap());
        let t1 = TE0[s[4] as usize]
            ^ TE1[s[9] as usize]
            ^ TE2[s[14] as usize]
            ^ TE3[s[3] as usize]
            ^ u32::from_ne_bytes(rk[r][4..8].try_into().unwrap());
        let t2 = TE0[s[8] as usize]
            ^ TE1[s[13] as usize]
            ^ TE2[s[2] as usize]
            ^ TE3[s[7] as usize]
            ^ u32::from_ne_bytes(rk[r][8..12].try_into().unwrap());
        let t3 = TE0[s[12] as usize]
            ^ TE1[s[1] as usize]
            ^ TE2[s[6] as usize]
            ^ TE3[s[11] as usize]
            ^ u32::from_ne_bytes(rk[r][12..16].try_into().unwrap());

        // Expected rk[1] = 62636363 repeated 4x = [0x62,0x63,0x63,0x63] repeated
        // TE0[0] = 0xa56363c6, TE1[0]=0x6363c6a5, TE2[0]=0x63c6a563, TE3[0]=0xc6a56363
        // t0 = 0xa56363c6 ^ 0x6363c6a5 ^ 0x63c6a563 ^ 0xc6a56363 ^ u32_from_ne([0x62,0x63,0x63,0x63])
        let rk1word0 = u32::from_ne_bytes([0x62, 0x63, 0x63, 0x63]);
        assert_eq!(rk1word0, 0x63636362, "rk[1] word 0 on LE should be 0x63636362");
        let te0_0 = TE0[0];
        let te1_0 = TE1[0];
        let te2_0 = TE2[0];
        let te3_0 = TE3[0];
        assert_eq!(te0_0, 0xa56363c6, "TE0[0]");
        assert_eq!(te1_0, 0x6363c6a5, "TE1[0]");
        assert_eq!(te2_0, 0x63c6a563, "TE2[0]");
        assert_eq!(te3_0, 0xc6a56363, "TE3[0]");

        let expected_t0 = 0xa56363c6u32 ^ 0x6363c6a5u32 ^ 0x63c6a563u32 ^ 0xc6a56363u32;
        // All XOR to 0 since they're cyclic permutations
        assert_eq!(expected_t0, 0x63636363u32, "TE0^TE1^TE2^TE3 should be 0x63636363");
        assert_eq!(t0, 0x63636363u32 ^ 0x63636362u32, "t0 after XOR with rk");
        assert_eq!(t0, 0x00000001u32, "t0 should be 1");

        let ct = encrypt_block_aes128(&rk, &pt);
        let ct_hex = hex::encode(&ct);
        assert_eq!(ct_hex, "66e94bd4ef8a2c3b884cfa59ca342b2e", "AES-128 K=0 P=0 full encrypt fails");
    }

    #[test]
    fn wycheproof_gcm_vectors() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../testdata/wycheproof/testvectors_v1/aes_gcm_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["keySize"].as_u64() != Some(128) {
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

                let key = hex::decode_array::<16>(key_hex.as_bytes()).unwrap();
                let nonce = hex::decode_array::<12>(iv_hex.as_bytes()).unwrap();
                let expected_ct = hex::decode(ct_hex).unwrap();
                let expected_tag = hex::decode_array::<16>(tag_hex.as_bytes()).unwrap();
                let pt = hex::decode(msg_hex).unwrap();
                let aad = hex::decode(aad_hex).unwrap();

                let cipher = Aes128Gcm::new(&key);

                if result == "valid" {
                    let mut buf = pt.clone();
                    let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad);
                    assert_eq!(buf, expected_ct, "wycheproof tcId={} ct mismatch", test["tcId"]);
                    assert_eq!(tag.as_ref(), &expected_tag[..], "wycheproof tcId={} tag mismatch", test["tcId"]);

                    let mut buf2 = expected_ct.clone();
                    cipher
                        .decrypt_in_place(&mut buf2, &nonce, &aad, &expected_tag[..])
                        .expect("wycheproof decrypt failed");
                    assert_eq!(buf2, pt, "wycheproof tcId={} pt mismatch", test["tcId"]);
                    valid_tested += 1;
                } else {
                    let mut buf = expected_ct.clone();
                    let result = cipher.decrypt_in_place(&mut buf, &nonce, &aad, &expected_tag[..]);
                    assert!(result.is_err(), "wycheproof tcId={} expected invalid but passed", test["tcId"]);
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid AES-128-GCM wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid AES-128-GCM wycheproof tests were run");
    }
}
