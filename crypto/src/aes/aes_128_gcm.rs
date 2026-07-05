#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__m128i;

use super::{
    aes::{encrypt_block, expand_key},
    ghash::{compute_tag, precompute_ghash_powers_128, precompute_ghash_table},
};
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
use crate::aes::RoundKeys;
use crate::{
    Aead, AeadError, Hash,
    aes::{RoundKeys, aes::GCM_MAX_LEN},
};

/// AES-128-GCM authenticated cipher.
///
/// On x86-64 machines with AES-NI + PCLMULQDQ the methods automatically
/// dispatch to the hardware-accelerated path.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes128Gcm {
    pub(crate) key: [u8; 16],
    /// x86_64 AES-NI round keys (precomputed in `new()`).
    #[cfg(target_arch = "x86_64")]
    pub(crate) round_keys_ni: [__m128i; 11],
    /// Precomputed GHASH powers [H¹..H⁸] in bit-reversed-per-byte form.
    #[cfg(target_arch = "x86_64")]
    pub(crate) h_powers: [__m128i; 8],
    /// aarch64 ARMv8 round keys (precomputed in `new()`).
    #[cfg(target_arch = "aarch64")]
    pub(crate) round_keys_arm: [core::arch::aarch64::uint8x16_t; 11],
    /// Precomputed GHASH powers [H¹..H⁸] in bit-reversed-per-byte form.
    #[cfg(target_arch = "aarch64")]
    pub(crate) h_powers: [core::arch::aarch64::uint8x16_t; 8],
    /// Software round keys (targets without hardware acceleration).
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    pub(crate) round_keys: RoundKeys<11>,
}

impl Aes128Gcm {
    pub const KEY_SIZE: usize = 16;

    pub fn new(key: &[u8; 16]) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            use core::arch::x86_64::*;
            let rk_soft: [[u8; 16]; 11] = expand_key::<11>(key);
            let mut rk = unsafe { [_mm_setzero_si128(); 11] };
            for i in 0..11 {
                rk[i] = unsafe { _mm_loadu_si128(rk_soft[i].as_ptr().cast()) };
            }
            let (h_powers_bytes, _h) = precompute_ghash_powers_128(key);
            let mut h_powers = unsafe { [_mm_setzero_si128(); 8] };
            for i in 0..8 {
                h_powers[i] = unsafe { _mm_loadu_si128(h_powers_bytes[i].as_ptr().cast()) };
            }
            Aes128Gcm {
                key: *key,
                round_keys_ni: rk,
                h_powers,
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            use core::arch::aarch64::*;
            let rk_soft: [[u8; 16]; 11] = expand_key::<11>(key);
            let mut rk = [unsafe { vdupq_n_u8(0) }; 11];
            for i in 0..11 {
                rk[i] = unsafe { vld1q_u8(rk_soft[i].as_ptr()) };
            }
            let (h_powers_bytes, _h) = precompute_ghash_powers_128(key);
            let mut h_powers = [unsafe { vdupq_n_u8(0) }; 8];
            for i in 0..8 {
                h_powers[i] = unsafe { vld1q_u8(h_powers_bytes[i].as_ptr()) };
            }
            Aes128Gcm {
                key: *key,
                round_keys_arm: rk,
                h_powers,
            }
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Aes128Gcm {
                key: *key,
                round_keys: expand_key::<11>(key),
            }
        }
    }

    pub(crate) fn encrypt_in_place_soft(&self, in_out: &mut [u8], nonce: &[u8; 12], aad: &[u8]) -> Hash {
        let rk: [[u8; 16]; 11] = expand_key::<11>(&self.key);
        let h = encrypt_block::<11>(&rk, &[0u8; 16]);
        let ghash_table = precompute_ghash_table(&h);

        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block::<11>(&rk, &j0);

        j0[15] = 2;
        aes128_ctr_xor(&rk, &j0, in_out);

        compute_tag(&ghash_table, aad, in_out, &ej0)
    }

    pub(crate) fn decrypt_in_place_soft(
        &self,
        in_out: &mut [u8],
        tag: &[u8; 16],
        nonce: &[u8; 12],
        aad: &[u8],
    ) -> Result<(), AeadError> {
        let rk: [[u8; 16]; 11] = expand_key::<11>(&self.key);
        let h = encrypt_block::<11>(&rk, &[0u8; 16]);
        let ghash_table = precompute_ghash_table(&h);

        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block::<11>(&rk, &j0);

        let expected_tag = compute_tag(&ghash_table, aad, in_out, &ej0);

        let mut diff = 0u8;
        for i in 0..16 {
            diff |= expected_tag.as_ref()[i] ^ tag[i];
        }
        if diff != 0 {
            return Err(AeadError::InvalidCiphertext);
        }

        j0[15] = 2;
        aes128_ctr_xor(&rk, &j0, in_out);

        Ok(())
    }
}

fn aes128_ctr_xor(round_keys: &RoundKeys<11>, counter: &[u8; 16], in_out: &mut [u8]) {
    let mut ctr = *counter;
    let n = in_out.len();
    let mut i = 0;
    while i + 16 <= n {
        let ks = encrypt_block::<11>(round_keys, &ctr);
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
        let ks = encrypt_block::<11>(round_keys, &ctr);
        for j in 0..(n - i) {
            in_out[i + j] ^= ks[j];
        }
    }
}

impl Aead for Aes128Gcm {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    #[inline]
    #[allow(unreachable_code)]
    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        assert!(
            in_out.len() <= GCM_MAX_LEN,
            "GCM plaintext exceeds maximum allowed length (2^32 - 2 blocks)"
        );

        assert_eq!(nonce.len(), 12, "AES-128-GCM nonce must be 12 bytes");
        let nonce_arr: &[u8; 12] = nonce.try_into().unwrap();

        // dynamic dispatch if the "std" feature is enabled
        #[cfg(feature = "std")]
        {
            #[cfg(target_arch = "x86_64")]
            {
                use crate::aes::aes_gcm_amd64::gcm_encrypt_aesni;
                if std::arch::is_x86_feature_detected!("aes")
                    && std::arch::is_x86_feature_detected!("pclmulqdq")
                    && std::arch::is_x86_feature_detected!("ssse3")
                    && std::arch::is_x86_feature_detected!("sse4.1")
                {
                    return unsafe { gcm_encrypt_aesni(&self.round_keys_ni, &self.h_powers, in_out, nonce_arr, aad) };
                }
            }

            #[cfg(target_arch = "aarch64")]
            {
                use crate::aes::aes_gcm_arm64::gcm_encrypt_armv8;
                if std::arch::is_aarch64_feature_detected!("aes") {
                    return unsafe { gcm_encrypt_armv8(&self.round_keys_arm, &self.h_powers, in_out, nonce_arr, aad) };
                }
            }
        }

        // otherwise, static dispatch depending on the CPU instructions available on the compiling machine
        #[cfg(not(feature = "std"))]
        {
            #[cfg(all(
                target_arch = "x86_64",
                target_feature = "aes",
                target_feature = "pclmulqdq",
                target_feature = "ssse3",
                target_feature = "sse4.1"
            ))]
            {
                use crate::aes::aes_gcm_amd64::gcm_encrypt_aesni;
                return unsafe { gcm_encrypt_aesni(&self.round_keys_ni, &self.h_powers, in_out, nonce_arr, aad) };
            }

            #[cfg(all(target_arch = "aarch64", target_feature = "aes",))]
            {
                use crate::aes::aes_gcm_arm64::gcm_encrypt_armv8;
                return unsafe { gcm_encrypt_armv8(&self.round_keys_arm, &self.h_powers, in_out, nonce_arr, aad) };
            }
        }

        self.encrypt_in_place_soft(in_out, nonce_arr, aad)
    }

    #[inline]
    #[allow(unreachable_code)]
    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        assert_eq!(nonce.len(), 12, "AES-128-GCM nonce must be 12 bytes");
        let nonce_arr: &[u8; 12] = nonce.try_into().unwrap();
        let tag_arr: &[u8; 16] = tag.try_into().expect("AES-128-GCM tag must be 16 bytes");

        // dynamic dispatch if the "std" feature is enabled
        #[cfg(feature = "std")]
        {
            #[cfg(target_arch = "x86_64")]
            {
                use crate::aes::aes_gcm_amd64::gcm_decrypt_aesni;
                if std::arch::is_x86_feature_detected!("aes")
                    && std::arch::is_x86_feature_detected!("pclmulqdq")
                    && std::arch::is_x86_feature_detected!("ssse3")
                    && std::arch::is_x86_feature_detected!("sse4.1")
                {
                    unsafe {
                        return gcm_decrypt_aesni(&self.round_keys_ni, &self.h_powers, in_out, tag_arr, nonce_arr, aad);
                    }
                }
            }

            #[cfg(target_arch = "aarch64")]
            {
                use crate::aes::aes_gcm_arm64::gcm_decrypt_armv8;
                if std::arch::is_aarch64_feature_detected!("aes") {
                    unsafe {
                        return gcm_decrypt_armv8(
                            &self.round_keys_arm,
                            &self.h_powers,
                            in_out,
                            tag_arr,
                            nonce_arr,
                            aad,
                        );
                    }
                }
            }
        }

        // otherwise, static dispatch depending on the CPU instructions available on the compiling machine
        #[cfg(not(feature = "std"))]
        {
            #[cfg(all(
                target_feature = "aes",
                target_feature = "pclmulqdq",
                target_feature = "ssse3",
                target_feature = "sse4.1"
            ))]
            {
                use crate::aes::aes_gcm_amd64::gcm_decrypt_aesni;
                unsafe {
                    return gcm_decrypt_aesni(&self.round_keys_ni, &self.h_powers, in_out, tag_arr, nonce_arr, aad);
                }
            }

            #[cfg(all(target_arch = "aarch64", target_feature = "aes",))]
            {
                use crate::aes::aes_gcm_arm64::gcm_decrypt_armv8;
                unsafe {
                    return gcm_decrypt_armv8(&self.round_keys_arm, &self.h_powers, in_out, tag_arr, nonce_arr, aad);
                }
            }
        }

        self.decrypt_in_place_soft(in_out, tag_arr, nonce_arr, aad)
    }
}

#[cfg(test)]
mod tests {
    use hex;

    use super::*;
    use crate::aes::aes::{TE0, TE1, TE2, TE3, encrypt_block, expand_key};

    include!("aes_128_gcm_vectors.rs");

    #[test]
    fn aes128_encrypt_block_vector() {
        // NIST FIPS 197 Appendix B: AES-128 K=0, P=0 -> C=66e94bd4ef8a2c3b884cfa59ca342b2e
        let key = [0u8; 16];
        let pt = [0u8; 16];
        let rk: [[u8; 16]; 11] = expand_key::<11>(&key);
        let ct = encrypt_block::<11>(&rk, &pt);
        let expected = hex::decode_array::<16>(b"66e94bd4ef8a2c3b884cfa59ca342b2e").unwrap();
        assert_eq!(ct, expected, "AES-128 encrypt K=0, P=0 mismatch");
    }

    #[test]
    fn aes128_fips197_vector() {
        let key = hex::decode_array::<16>(b"2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        let pt = hex::decode_array::<16>(b"3243f6a8885a308d313198a2e0370734").unwrap();
        let rk: [[u8; 16]; 11] = expand_key::<11>(&key);

        let rk_hex: Vec<String> = rk.iter().map(|rk_i| hex::encode(rk_i)).collect();
        assert_eq!(rk_hex[0], "2b7e151628aed2a6abf7158809cf4f3c", "rk0");
        assert_eq!(rk_hex[1], "a0fafe1788542cb123a339392a6c7605", "rk1");
        assert_eq!(rk_hex[2], "f2c295f27a96b9435935807a7359f67f", "rk2");

        let ct = encrypt_block::<11>(&rk, &pt);
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
        let rk: [[u8; 16]; 11] = expand_key::<11>(&key);
        let ct = encrypt_block::<11>(&rk, &j0);
        let expected = hex::decode_array::<16>(b"58e2fccefa7e3061367f1d57a4e7455a").unwrap();
        assert_eq!(ct, expected, "AES(K=0, J0=0...01) mismatch");
    }

    #[test]
    fn aes128_debug_round_state() {
        let key = [0u8; 16];
        let pt = [0u8; 16];
        let rk: [[u8; 16]; 11] = expand_key::<11>(&key);

        let mut s = pt;
        for i in 0..16 {
            s[i] ^= rk[0][i];
        }
        assert_eq!(s, [0; 16], "after AddRoundKey with K=0, state should be all zeros");

        let r = 1usize;
        let t0 = TE0[s[0] as usize]
            ^ TE1[s[5] as usize]
            ^ TE2[s[10] as usize]
            ^ TE3[s[15] as usize]
            ^ u32::from_ne_bytes(rk[r][0..4].try_into().unwrap());

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
        assert_eq!(expected_t0, 0x63636363u32, "TE0^TE1^TE2^TE3 should be 0x63636363");
        assert_eq!(t0, 0x63636363u32 ^ 0x63636362u32, "t0 after XOR with rk");
        assert_eq!(t0, 0x00000001u32, "t0 should be 1");

        let ct = encrypt_block::<11>(&rk, &pt);
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
