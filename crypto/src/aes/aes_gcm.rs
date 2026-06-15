#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__m128i;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
use super::aes::RoundKeys;
use super::{
    aes::{encrypt_block, key_expand},
    aes_ctr::Aes256Ctr,
    ghash::{compute_tag, precompute_ghash_powers, precompute_ghash_table},
};
use crate::{Aead, AeadError, Hash, StreamCipher};

/// AES-256-GCM authenticated cipher.
///
/// On x86-64 machines with AES-NI + PCLMULQDQ the methods automatically
/// dispatch to the hardware-accelerated path (see `aes_gcm_amd64`).
///
/// The struct stores **only** the round keys native to the target architecture.
/// - x86_64: stores `round_keys_ni` (`[__m128i; 15]`) + precomputed GHASH powers
/// - aarch64: stores `round_keys_arm` (`[uint8x16_t; 15]`) + precomputed GHASH powers
/// - other: stores `round_keys` (`[[u8; 16]; 15]`)
///
/// The raw 32-byte key is retained so the software fallback can recompute
/// the expanded key on the rare occasion the hardware path is unavailable.
pub(crate) const MAX_GCM_LEN: usize = (u32::MAX as usize - 1) * 16;

pub struct Aes256Gcm {
    pub(crate) key: [u8; 32],
    /// x86_64 AES-NI round keys (precomputed in `new()`).
    #[cfg(target_arch = "x86_64")]
    pub(crate) round_keys_ni: [__m128i; 15],
    /// Precomputed GHASH powers [H, H², H³, H⁴] in bit-reversed-per-byte form.
    /// Used by 4-block aggregated GHASH to avoid recomputing H on every call.
    #[cfg(target_arch = "x86_64")]
    pub(crate) h_powers: [__m128i; 8],
    /// aarch64 ARMv8 round keys (precomputed in `new()`).
    #[cfg(target_arch = "aarch64")]
    pub(crate) round_keys_arm: [core::arch::aarch64::uint8x16_t; 15],
    /// Precomputed GHASH powers [H¹..H⁸] in bit-reversed-per-byte form.
    /// Used by 8-block aggregated GHASH to avoid recomputing H on every call.
    #[cfg(target_arch = "aarch64")]
    pub(crate) h_powers: [core::arch::aarch64::uint8x16_t; 8],
    /// Software round keys (targets without hardware acceleration).
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    pub(crate) round_keys: RoundKeys,
}

impl Aes256Gcm {
    pub const KEY_SIZE: usize = 32;
    /// Create a new `Aes256Gcm` instance from a 32-byte key.
    ///
    /// Precomputes the target-specific round keys and GHASH powers (H, H², H³, H⁴)
    /// using software GF(2¹²⁸) multiplication, so `new()` is safe on any CPU
    /// and does not require hardware feature detection.
    pub fn new(key: &[u8; 32]) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            use core::arch::x86_64::*;
            let rk_soft = key_expand(key);
            let mut rk = unsafe { [_mm_setzero_si128(); 15] };
            for i in 0..15 {
                rk[i] = unsafe { _mm_loadu_si128(rk_soft[i].as_ptr().cast()) };
            }
            let (h_powers_bytes, _h) = precompute_ghash_powers(key);
            let mut h_powers = unsafe { [_mm_setzero_si128(); 8] };
            for i in 0..8 {
                h_powers[i] = unsafe { _mm_loadu_si128(h_powers_bytes[i].as_ptr().cast()) };
            }
            Aes256Gcm {
                key: *key,
                round_keys_ni: rk,
                h_powers,
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            use core::arch::aarch64::*;
            let rk_soft = key_expand(key);
            let mut rk = [unsafe { vdupq_n_u8(0) }; 15];
            for i in 0..15 {
                rk[i] = unsafe { vld1q_u8(rk_soft[i].as_ptr()) };
            }
            let (h_powers_bytes, _h) = precompute_ghash_powers(key);
            let mut h_powers = [unsafe { vdupq_n_u8(0) }; 8];
            for i in 0..8 {
                h_powers[i] = unsafe { vld1q_u8(h_powers_bytes[i].as_ptr()) };
            }
            Aes256Gcm {
                key: *key,
                round_keys_arm: rk,
                h_powers,
            }
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Aes256Gcm {
                key: *key,
                round_keys: key_expand(key),
            }
        }
    }

    /// Pure-Rust encrypt implementation.
    ///
    /// The expanded key is recomputed here because on x86_64/aarch64 we store
    /// only the hardware-specific keys. This path is only reached when
    /// the hardware accelerator is unavailable, so the overhead is negligible.
    pub(crate) fn encrypt_in_place_soft(&self, in_out: &mut [u8], nonce: &[u8; 12], aad: &[u8]) -> Hash {
        assert!(
            in_out.len() <= MAX_GCM_LEN,
            "GCM plaintext exceeds maximum allowed length (2^32 - 2 blocks)"
        );
        let rk = key_expand(&self.key);
        let h = encrypt_block(&rk, &[0u8; 16]);
        let ghash_table = precompute_ghash_table(&h);

        // J0 = nonce || 0x00000001
        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block(&rk, &j0);

        // CTR starts at J0 + 1 (= nonce || 0x00000002)
        j0[15] = 2;

        let mut aes_ctr = Aes256Ctr::new(&self.key);
        aes_ctr.set_counter(&j0);
        aes_ctr.xor_keystream(in_out);
        compute_tag(&ghash_table, aad, in_out, &ej0)
    }

    /// Pure-Rust decrypt implementation.
    pub(crate) fn decrypt_in_place_soft(
        &self,
        in_out: &mut [u8],
        tag: &[u8; 16],
        nonce: &[u8; 12],
        aad: &[u8],
    ) -> Result<(), AeadError> {
        if in_out.len() > MAX_GCM_LEN {
            return Err(AeadError::InvalidCiphertext);
        }
        let rk = key_expand(&self.key);
        let h = encrypt_block(&rk, &[0u8; 16]);
        let ghash_table = precompute_ghash_table(&h);

        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block(&rk, &j0);

        // Verify tag before decrypting (authenticate-then-decrypt ordering)
        let expected_tag = compute_tag(&ghash_table, aad, in_out, &ej0);

        // Constant-time comparison to avoid timing oracle
        let mut diff = 0u8;
        for i in 0..16 {
            diff |= expected_tag.as_ref()[i] ^ tag[i];
        }
        if diff != 0 {
            return Err(AeadError::InvalidCiphertext);
        }

        // CTR starts at J0 + 1 (= nonce || 0x00000002)
        j0[15] = 2;
        let mut aes_ctr = Aes256Ctr::new(&self.key);
        aes_ctr.set_counter(&j0);
        aes_ctr.xor_keystream(in_out);

        Ok(())
    }
}

impl Aead for Aes256Gcm {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    #[inline]
    #[allow(unreachable_code)]
    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        assert_eq!(nonce.len(), 12, "AES-256-GCM nonce must be 12 bytes");
        let nonce_arr: &[u8; 12] = nonce.try_into().unwrap();

        #[cfg(target_arch = "aarch64")]
        {
            use crate::aes::aes_gcm_arm64::gcm_encrypt_armv8;
            return unsafe { gcm_encrypt_armv8(&self.round_keys_arm, &self.h_powers, in_out, nonce_arr, aad) };
        }

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
        }

        #[cfg(not(feature = "std"))]
        {
            #[cfg(all(
                target_feature = "aes",
                target_feature = "pclmulqdq",
                target_feature = "ssse3",
                target_feature = "sse4.1"
            ))]
            {
                use crate::aes::aes_gcm_amd64::gcm_encrypt_aesni;
                return unsafe { gcm_encrypt_aesni(&self.round_keys_ni, &self.h_powers, in_out, nonce_arr, aad) };
            }
        }

        self.encrypt_in_place_soft(in_out, nonce_arr, aad)
    }

    #[inline]
    #[allow(unreachable_code)]
    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        assert_eq!(nonce.len(), 12, "AES-256-GCM nonce must be 12 bytes");
        let nonce_arr: &[u8; 12] = nonce.try_into().unwrap();
        let tag_arr: &[u8; 16] = tag.try_into().expect("AES-256-GCM tag must be 16 bytes");

        #[cfg(target_arch = "aarch64")]
        {
            use crate::aes::aes_gcm_arm64::gcm_decrypt_armv8;
            unsafe { return gcm_decrypt_armv8(&self.round_keys_arm, &self.h_powers, in_out, tag_arr, nonce_arr, aad) }
        }

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
        }

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
        }

        self.decrypt_in_place_soft(in_out, tag_arr, nonce_arr, aad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── AES-256-GCM (NIST SP 800-38D Appendix B and additional vectors) ────────

    include!("aes_gcm_vectors.rs");

    fn run_gcm_vector_soft(v: &GcmVector) {
        let key: [u8; 32] = hex::decode_array::<32>(v.key.as_bytes()).unwrap();
        let nonce: [u8; 12] = hex::decode_array::<12>(v.nonce.as_bytes()).unwrap();
        let pt = hex::decode(v.pt).unwrap();
        let aad = hex::decode(v.aad).unwrap();
        let expected_ct = hex::decode(v.ct).unwrap();
        let expected_tag: [u8; 16] = hex::decode_array::<16>(v.tag.as_bytes()).unwrap();

        let cipher = Aes256Gcm::new(&key);

        // Encrypt
        let mut buf = pt.clone();
        let tag = cipher.encrypt_in_place_soft(&mut buf, &nonce, &aad);
        assert_eq!(buf, expected_ct, "ciphertext mismatch for key={}", v.key);
        assert_eq!(tag.as_ref(), &expected_tag[..], "tag mismatch for key={}", v.key);

        // Decrypt
        let mut buf2 = expected_ct.clone();
        cipher
            .decrypt_in_place_soft(&mut buf2, &expected_tag, &nonce, &aad)
            .expect("decrypt failed");
        assert_eq!(buf2, pt, "plaintext mismatch after decrypt for key={}", v.key);
    }

    #[test]
    fn nist_gcm_test_vectors_soft() {
        for v in NIST_GCM_VECTORS.iter().chain(EXTRA_GCM_VECTORS.iter()) {
            run_gcm_vector_soft(v);
        }
    }

    #[test]
    fn gcm_tag_mismatch_returns_error_soft() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let cipher = Aes256Gcm::new(&key);
        let mut buf = b"hello world".to_vec();
        let tag = cipher.encrypt_in_place_soft(&mut buf, &nonce, &[]);
        // Flip one tag byte
        let mut bad_tag: [u8; 16] = tag.as_ref().try_into().unwrap();
        bad_tag[0] ^= 0xff;
        let mut buf2 = buf.clone();
        assert!(cipher.decrypt_in_place_soft(&mut buf2, &bad_tag, &nonce, &[]).is_err());
    }

    #[test]
    fn gcm_encrypt_decrypt_large_soft() {
        let key = [0xabu8; 32];
        let nonce = [0x01u8; 12];
        let aad = b"additional data";
        let plaintext: Vec<u8> = (0u8..=255u8).cycle().take(1024).collect();

        let cipher = Aes256Gcm::new(&key);
        let mut buf = plaintext.clone();
        let tag = cipher.encrypt_in_place_soft(&mut buf, &nonce, aad);
        let tag_bytes: [u8; 16] = tag.as_ref().try_into().unwrap();
        cipher
            .decrypt_in_place_soft(&mut buf, &tag_bytes, &nonce, aad)
            .expect("decrypt failed");
        assert_eq!(buf, plaintext);
    }

    #[test]
    fn gcm_empty_plaintext_nonempty_aad_soft() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308").unwrap();
        let nonce: [u8; 12] = hex::decode_array::<12>(b"cafebabefacedbaddecaf888").unwrap();
        let aad = hex::decode("feedfacedeadbeeffeedfacedeadbeef").unwrap();
        let cipher = Aes256Gcm::new(&key);
        let mut buf: Vec<u8> = vec![];
        let tag = cipher.encrypt_in_place_soft(&mut buf, &nonce, &aad);
        let tag_bytes: [u8; 16] = tag.as_ref().try_into().unwrap();
        cipher
            .decrypt_in_place_soft(&mut buf, &tag_bytes, &nonce, &aad)
            .expect("decrypt failed");
    }

    // ── Dispatching wrappers (use hardware path when available) ───────────────

    #[test]
    fn nist_gcm_test_vectors_dispatch() {
        for v in NIST_GCM_VECTORS.iter().chain(EXTRA_GCM_VECTORS.iter()) {
            let key: [u8; 32] = hex::decode_array::<32>(v.key.as_bytes()).unwrap();
            let nonce: [u8; 12] = hex::decode_array::<12>(v.nonce.as_bytes()).unwrap();
            let pt = hex::decode(v.pt).unwrap();
            let aad = hex::decode(v.aad).unwrap();
            let expected_ct = hex::decode(v.ct).unwrap();
            let expected_tag: [u8; 16] = hex::decode_array::<16>(v.tag.as_bytes()).unwrap();

            let cipher = Aes256Gcm::new(&key);

            let mut buf = pt.clone();
            let tag = cipher.encrypt_in_place(&mut buf, &nonce[..], &aad);
            assert_eq!(&buf[..], &expected_ct[..], "dispatch ciphertext mismatch key={}", v.key);
            assert_eq!(tag.as_ref(), &expected_tag[..], "dispatch tag mismatch key={}", v.key);

            let mut buf2 = expected_ct.clone();
            cipher
                .decrypt_in_place(&mut buf2, &nonce[..], &aad, &expected_tag)
                .expect("dispatch decrypt failed");
            assert_eq!(buf2, pt);
        }
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn wycheproof_gcm_vectors() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../testdata/wycheproof/testvectors_v1/aes_gcm_test.json")).unwrap();
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

                let key = hex::decode_array::<32>(key_hex.as_bytes()).unwrap();
                let nonce = hex::decode_array::<12>(iv_hex.as_bytes()).unwrap();
                let expected_ct = hex::decode(ct_hex).unwrap();
                let expected_tag = hex::decode_array::<16>(tag_hex.as_bytes()).unwrap();
                let pt = hex::decode(msg_hex).unwrap();
                let aad = hex::decode(aad_hex).unwrap();

                let cipher = Aes256Gcm::new(&key);

                if result == "valid" {
                    let mut buf = pt.clone();
                    let tag = cipher.encrypt_in_place(&mut buf, &nonce[..], &aad);
                    assert_eq!(buf, expected_ct, "wycheproof GCM tcId={} ct mismatch", test["tcId"]);
                    assert_eq!(
                        tag.as_ref(),
                        &expected_tag[..],
                        "wycheproof GCM tcId={} tag mismatch",
                        test["tcId"]
                    );

                    let mut buf2 = expected_ct.clone();
                    cipher
                        .decrypt_in_place(&mut buf2, &nonce[..], &aad, &expected_tag[..])
                        .expect("wycheproof GCM decrypt failed");
                    assert_eq!(buf2, pt, "wycheproof GCM tcId={} pt mismatch", test["tcId"]);
                    valid_tested += 1;
                } else {
                    let mut buf = expected_ct.clone();
                    let result = cipher.decrypt_in_place(&mut buf, &nonce[..], &aad, &expected_tag[..]);
                    assert!(
                        result.is_err(),
                        "wycheproof GCM tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid AES-GCM wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid AES-GCM wycheproof tests were run");
    }
}
