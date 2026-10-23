use constant_time_eq::constant_time_eq;

#[cfg(target_arch = "x86_64")]
use super::ghash::precompute_ghash_powers;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use super::ghash::precompute_ghash_powers_8;
use super::{
    aes::{GCM_MAX_LEN, encrypt_block, expand_key},
    ghash::{GHashPowers, compute_tag},
};
use crate::{
    Aead, AeadError, Hash,
    aes::{RoundKeys, aes::RoundKeysSoftware},
};

/// AES-128-GCM authenticated cipher.
///
/// Create a new cipher with [`new`](Aes128Gcm::new).
/// [`encrypt_in_place`](Aead::encrypt_in_place) and
/// [`decrypt_in_place`](Aead::decrypt_in_place) for authenticated encryption.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes128Gcm(pub(crate) AesGcm<11>);

impl Aes128Gcm {
    pub const KEY_SIZE: usize = 16;

    /// Create a new AES-128-GCM instance from a 16-byte key.
    ///
    /// Precomputes the target-specific round keys and GHASH powers (H, H², H³, H⁴)
    /// using software GF(2¹²⁸) multiplication, so `new()` is safe on any CPU
    /// and does not require hardware feature detection.
    #[inline]
    pub fn new(key: &[u8; Self::KEY_SIZE]) -> Self {
        Self(AesGcm::<11>::new(key))
    }
}

impl Aead for Aes128Gcm {
    const KEY_SIZE: usize = 16;
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    /// # Panics
    ///
    /// Panics if `nonce.len() != 12`.
    #[inline]
    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        let nonce_arr: &[u8; 12] = nonce.try_into().map_err(|_| AeadError::InvalidNonce).unwrap();

        self.0.encrypt_in_place(in_out, nonce_arr, aad)
    }

    #[inline]
    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        let nonce_arr: &[u8; 12] = nonce.try_into().map_err(|_| AeadError::InvalidNonce)?;
        let tag_arr: &[u8; 16] = tag.try_into().map_err(|_| AeadError::InvalidCiphertext)?;

        self.0.decrypt_in_place(in_out, nonce_arr, aad, tag_arr)
    }
}

/// AES-256-GCM authenticated cipher.
///
/// Create a new cipher with [`new`](Aes256Gcm::new).
/// [`encrypt_in_place`](Aead::encrypt_in_place) and
/// [`decrypt_in_place`](Aead::decrypt_in_place) for authenticated encryption.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes256Gcm(pub(crate) AesGcm<15>);

impl Aes256Gcm {
    pub const KEY_SIZE: usize = 32;

    /// Create a new AES-256-GCM instance from a 32-byte key.
    ///
    /// Precomputes the target-specific round keys and GHASH powers (H, H², H³, H⁴)
    /// using software GF(2¹²⁸) multiplication, so `new()` is safe on any CPU
    /// and does not require hardware feature detection.
    #[inline]
    pub fn new(key: &[u8; Self::KEY_SIZE]) -> Self {
        Self(AesGcm::<15>::new(key))
    }
}

impl Aead for Aes256Gcm {
    const KEY_SIZE: usize = 32;
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    /// # Panics
    ///
    /// Panics if `nonce.len() != 12`.
    #[inline]
    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        let nonce_arr: &[u8; 12] = nonce.try_into().map_err(|_| AeadError::InvalidNonce).unwrap();

        self.0.encrypt_in_place(in_out, nonce_arr, aad)
    }

    #[inline]
    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        let nonce_arr: &[u8; 12] = nonce.try_into().map_err(|_| AeadError::InvalidNonce)?;
        let tag_arr: &[u8; 16] = tag.try_into().map_err(|_| AeadError::InvalidCiphertext)?;

        self.0.decrypt_in_place(in_out, nonce_arr, aad, tag_arr)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/// AES-GCM authenticated cipher, generic over the number of round keys.
///
/// `N = 11` for AES-128-GCM, `N = 15` for AES-256-GCM.
///
/// On x86-64 machines the methods automatically dispatch to the best available
/// hardware path (see `aes_gcm_amd64`): VAES-512 + 512-bit VPCLMULQDQ GHASH,
/// VAES-256 + 256-bit VPCLMULQDQ GHASH, VAES-256 + 128-bit PCLMULQDQ GHASH, or
/// AES-NI + PCLMULQDQ.
///
/// The struct stores **only** the round keys native to the target architecture.
/// - x86_64: stores `round_keys_ni` (`[__m128i; N]`) + precomputed GHASH powers
/// - aarch64: stores `round_keys_arm` (`[uint8x16_t; N]`) + precomputed GHASH powers
/// - other: stores `round_keys` (`[[u8; 16]; N]`)
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct AesGcm<const N: usize> {
    pub(crate) round_keys: RoundKeys<N>,
    pub(crate) h_powers: GHashPowers,
    #[cfg(target_arch = "x86_64")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub(crate) backend: super::aes_amd64::X86Backend,
    /// Whether the 128-bit/256-bit VPCLMULQDQ GHASH path may be used.
    #[cfg(target_arch = "x86_64")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub(crate) vpclmul: bool,
}

impl AesGcm<11> {
    pub const KEY_SIZE: usize = 16;

    pub fn new(key: &[u8; Self::KEY_SIZE]) -> Self {
        Self::new_inner(key)
    }
}

impl AesGcm<15> {
    pub const KEY_SIZE: usize = 32;

    pub fn new(key: &[u8; Self::KEY_SIZE]) -> Self {
        Self::new_inner(key)
    }
}

/// Load H¹..H¹⁶ for the selected x86 backend, computing only the powers that
/// the backend can use.
#[cfg(target_arch = "x86_64")]
fn x86_h_powers<const N: usize>(
    key: &[u8],
    backend: super::aes_amd64::X86Backend,
) -> [core::arch::x86_64::__m128i; 16] {
    use core::arch::x86_64::*;

    let mut bytes = [[0u8; 16]; 16];
    if backend == super::aes_amd64::X86Backend::Vaes512 {
        bytes = precompute_ghash_powers::<N>(key).0;
    } else {
        let (powers8, _) = precompute_ghash_powers_8::<N>(key);
        bytes[..8].copy_from_slice(&powers8);
    }

    let mut h_powers = unsafe { [_mm_setzero_si128(); 16] };
    for (i, power) in bytes.iter().enumerate() {
        h_powers[i] = unsafe { _mm_loadu_si128(power.as_ptr().cast()) };
    }
    h_powers
}

impl<const N: usize> AesGcm<N> {
    fn new_inner(key: &[u8]) -> Self {
        const { assert!(N == 11 || N == 15) };

        let round_keys_software = expand_key(key);

        #[cfg(target_arch = "aarch64")]
        {
            use core::arch::aarch64::*;

            let (h_powers_bytes, _h) = precompute_ghash_powers_8::<N>(key);
            let mut h_powers = [unsafe { vdupq_n_u8(0) }; 8];
            for i in 0..8 {
                h_powers[i] = unsafe { vld1q_u8(h_powers_bytes[i].as_ptr()) };
            }
            let h_powers = GHashPowers::Armv8(h_powers);

            #[cfg(feature = "std")]
            if std::arch::is_aarch64_feature_detected!("aes") {
                return AesGcm {
                    round_keys: RoundKeys::Armv8(super::aes_arm64::expand_key_armv8(round_keys_software)),
                    h_powers,
                };
            }

            #[cfg(all(not(feature = "std"), target_feature = "aes"))]
            return AesGcm {
                round_keys: RoundKeys::Armv8(super::aes_arm64::expand_key_armv8(round_keys_software)),
                h_powers,
            };
        }

        #[cfg(target_arch = "x86_64")]
        {
            #[cfg(feature = "std")]
            if std::arch::is_x86_feature_detected!("aes")
                && std::arch::is_x86_feature_detected!("pclmulqdq")
                && std::arch::is_x86_feature_detected!("ssse3")
                && std::arch::is_x86_feature_detected!("sse4.1")
            {
                let vpclmul = std::arch::is_x86_feature_detected!("vpclmulqdq");
                let mut backend = super::aes_amd64::X86Backend::detect();
                // The 512-bit GCM path uses 512-bit VPCLMULQDQ for GHASH. If a
                // CPU ever exposes VAES-512 without VPCLMUL, use the 256-bit
                // VAES path with 128-bit PCLMUL GHASH instead.
                if backend == super::aes_amd64::X86Backend::Vaes512 && !vpclmul {
                    backend = super::aes_amd64::X86Backend::Vaes256;
                }
                return AesGcm {
                    round_keys: RoundKeys::X86_64(super::aes_amd64::expand_key_x86_64(round_keys_software)),
                    h_powers: GHashPowers::X86_64(x86_h_powers::<N>(key, backend)),
                    backend,
                    vpclmul,
                };
            }

            #[cfg(all(
                not(feature = "std"),
                target_feature = "aes",
                target_feature = "pclmulqdq",
                target_feature = "ssse3",
                target_feature = "sse4.1"
            ))]
            {
                let vpclmul = cfg!(target_feature = "vpclmulqdq");
                let mut backend = super::aes_amd64::X86Backend::detect();
                if backend == super::aes_amd64::X86Backend::Vaes512 && !vpclmul {
                    backend = super::aes_amd64::X86Backend::Vaes256;
                }
                return AesGcm {
                    round_keys: RoundKeys::X86_64(super::aes_amd64::expand_key_x86_64(round_keys_software)),
                    h_powers: GHashPowers::X86_64(x86_h_powers::<N>(key, backend)),
                    backend,
                    vpclmul,
                };
            }
        }

        AesGcm {
            round_keys: RoundKeys::Software(round_keys_software),
            h_powers: GHashPowers::Software(encrypt_block(&round_keys_software, &[0u8; 16])),
            #[cfg(target_arch = "x86_64")]
            backend: super::aes_amd64::X86Backend::AesNi,
            #[cfg(target_arch = "x86_64")]
            vpclmul: false,
        }
    }

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8; 12], aad: &[u8]) -> Hash {
        assert!(
            in_out.len() as u64 <= GCM_MAX_LEN,
            "GCM plaintext exceeds maximum allowed length (2^32 - 2 blocks)"
        );

        match (&self.round_keys, &self.h_powers) {
            #[cfg(target_arch = "aarch64")]
            (RoundKeys::Armv8(round_keys), GHashPowers::Armv8(h_powers)) => unsafe {
                use crate::aes::aes_gcm_arm64::gcm_encrypt_armv8;
                gcm_encrypt_armv8(round_keys, h_powers, in_out, nonce, aad)
            },
            #[cfg(target_arch = "x86_64")]
            (RoundKeys::X86_64(round_keys), GHashPowers::X86_64(h_powers)) => unsafe {
                use super::{
                    aes_amd64::X86Backend,
                    aes_gcm_amd64::{gcm_encrypt_aesni, gcm_encrypt_vaes256, gcm_encrypt_vaes512},
                };
                match self.backend {
                    X86Backend::AesNi => gcm_encrypt_aesni(round_keys, h_powers, in_out, nonce, aad),
                    X86Backend::Vaes256 => {
                        if self.vpclmul {
                            gcm_encrypt_vaes256::<N, true>(round_keys, h_powers, in_out, nonce, aad)
                        } else {
                            gcm_encrypt_vaes256::<N, false>(round_keys, h_powers, in_out, nonce, aad)
                        }
                    }
                    X86Backend::Vaes512 => gcm_encrypt_vaes512(round_keys, h_powers, in_out, nonce, aad),
                }
            },
            (RoundKeys::Software(round_keys), GHashPowers::Software(h)) => {
                self.encrypt_in_place_soft(in_out, round_keys, h, nonce, aad)
            }
            _ => unreachable!(),
        }
    }

    fn decrypt_in_place(
        &self,
        in_out: &mut [u8],
        nonce: &[u8; 12],
        aad: &[u8],
        tag: &[u8; 16],
    ) -> Result<(), AeadError> {
        if in_out.len() as u64 > GCM_MAX_LEN {
            return Err(AeadError::InvalidCiphertext);
        }

        match (&self.round_keys, &self.h_powers) {
            #[cfg(target_arch = "aarch64")]
            (RoundKeys::Armv8(round_keys), GHashPowers::Armv8(h_powers)) => unsafe {
                use crate::aes::aes_gcm_arm64::gcm_decrypt_armv8;
                gcm_decrypt_armv8(round_keys, h_powers, in_out, tag, nonce, aad)
            },
            #[cfg(target_arch = "x86_64")]
            (RoundKeys::X86_64(round_keys), GHashPowers::X86_64(h_powers)) => unsafe {
                use super::{
                    aes_amd64::X86Backend,
                    aes_gcm_amd64::{gcm_decrypt_aesni, gcm_decrypt_vaes256, gcm_decrypt_vaes512},
                };
                match self.backend {
                    X86Backend::AesNi => gcm_decrypt_aesni(round_keys, h_powers, in_out, tag, nonce, aad),
                    X86Backend::Vaes256 => {
                        if self.vpclmul {
                            gcm_decrypt_vaes256::<N, true>(round_keys, h_powers, in_out, tag, nonce, aad)
                        } else {
                            gcm_decrypt_vaes256::<N, false>(round_keys, h_powers, in_out, tag, nonce, aad)
                        }
                    }
                    X86Backend::Vaes512 => gcm_decrypt_vaes512(round_keys, h_powers, in_out, tag, nonce, aad),
                }
            },
            (RoundKeys::Software(round_keys), GHashPowers::Software(h)) => {
                self.decrypt_in_place_soft(in_out, round_keys, h, tag, nonce, aad)
            }
            _ => unreachable!(),
        }
    }

    /// Pure-Rust encrypt implementation.
    pub(crate) fn encrypt_in_place_soft(
        &self,
        in_out: &mut [u8],
        round_keys: &RoundKeysSoftware<N>,
        h: &[u8; 16],
        nonce: &[u8; 12],
        aad: &[u8],
    ) -> Hash {
        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block(round_keys, &j0);

        j0[15] = 2;
        super::aes_ct::xor_keystream_soft(round_keys, &mut j0, in_out);
        compute_tag(h, aad, in_out, &ej0)
    }

    /// Pure-Rust decrypt implementation.
    pub(crate) fn decrypt_in_place_soft(
        &self,
        in_out: &mut [u8],
        round_keys: &RoundKeysSoftware<N>,
        h: &[u8; 16],
        tag: &[u8; 16],
        nonce: &[u8; 12],
        aad: &[u8],
    ) -> Result<(), AeadError> {
        let mut j0 = [0u8; 16];
        j0[..12].copy_from_slice(nonce);
        j0[15] = 1;

        let ej0 = encrypt_block(round_keys, &j0);

        let expected_tag = compute_tag(h, aad, in_out, &ej0);

        if !constant_time_eq(tag, &expected_tag) {
            return Err(AeadError::InvalidCiphertext);
        }

        j0[15] = 2;
        super::aes_ct::xor_keystream_soft(round_keys, &mut j0, in_out);

        Ok(())
    }
}

#[cfg(test)]
mod tests_128 {
    use hex;

    use super::*;
    use crate::{
        Aead,
        aes::aes::{encrypt_block, expand_key},
    };

    include!("aes_gcm_128_vectors.rs");

    #[test]
    fn aes128_encrypt_block_vector() {
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

        let rk_hex: Vec<String> = rk.iter().map(hex::encode).collect();
        assert_eq!(rk_hex[0], "2b7e151628aed2a6abf7158809cf4f3c", "rk0");
        assert_eq!(rk_hex[1], "a0fafe1788542cb123a339392a6c7605", "rk1");
        assert_eq!(rk_hex[2], "f2c295f27a96b9435935807a7359f67f", "rk2");

        let ct = encrypt_block::<11>(&rk, &pt);
        let ct_hex = hex::encode(ct);
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
        let round_keys = expand_key::<11>(&key);
        let h = encrypt_block::<11>(&round_keys, &[0u8; 16]);

        let cipher = Aes128Gcm::new(&key);

        let mut buf = pt.clone();
        let tag = cipher.0.encrypt_in_place_soft(&mut buf, &round_keys, &h, &nonce, &aad);
        assert_eq!(buf, expected_ct, "ciphertext mismatch for key={}", v.key);
        assert_eq!(tag.as_ref(), &expected_tag[..], "tag mismatch for key={}", v.key);

        let mut buf2 = expected_ct.clone();
        cipher
            .0
            .decrypt_in_place_soft(&mut buf2, &round_keys, &h, &expected_tag, &nonce, &aad)
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
        assert!(cipher.0.decrypt_in_place(&mut buf2, &nonce, &[], &bad_tag).is_err());
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
    fn aes128_zero_key_zero_pt_vector() {
        let key = [0u8; 16];
        let pt = [0u8; 16];
        let rk: [[u8; 16]; 11] = expand_key::<11>(&key);
        let ct = encrypt_block::<11>(&rk, &pt);
        let ct_hex = hex::encode(ct);
        assert_eq!(ct_hex, "66e94bd4ef8a2c3b884cfa59ca342b2e", "AES-128 K=0 P=0 encrypt failed");
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

#[cfg(test)]
mod tests_256 {
    use hex;

    use super::*;
    use crate::Aead;

    include!("aes_gcm_256_vectors.rs");

    fn run_gcm_vector_soft(v: &GcmVector) {
        let key: [u8; 32] = hex::decode_array::<32>(v.key.as_bytes()).unwrap();
        let nonce: [u8; 12] = hex::decode_array::<12>(v.nonce.as_bytes()).unwrap();
        let pt = hex::decode(v.pt).unwrap();
        let aad = hex::decode(v.aad).unwrap();
        let expected_ct = hex::decode(v.ct).unwrap();
        let expected_tag: [u8; 16] = hex::decode_array::<16>(v.tag.as_bytes()).unwrap();
        let round_keys = expand_key(&key);
        let h = encrypt_block(&round_keys, &[0u8; 16]);

        let cipher = Aes256Gcm::new(&key);

        let mut buf = pt.clone();
        let tag = cipher.0.encrypt_in_place_soft(&mut buf, &round_keys, &h, &nonce, &aad);
        assert_eq!(buf, expected_ct, "ciphertext mismatch for key={}", v.key);
        assert_eq!(tag.as_ref(), &expected_tag[..], "tag mismatch for key={}", v.key);

        let mut buf2 = expected_ct.clone();
        cipher
            .0
            .decrypt_in_place_soft(&mut buf2, &round_keys, &h, &expected_tag, &nonce, &aad)
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
    fn x86_backend_selection_is_consistent() {
        #[cfg(target_arch = "x86_64")]
        {
            use crate::aes::aes_amd64::X86Backend;

            let cipher = Aes256Gcm::new(&[0u8; 32]);
            let vaes = std::arch::is_x86_feature_detected!("vaes") && std::arch::is_x86_feature_detected!("avx2");
            let avx512 = vaes
                && std::arch::is_x86_feature_detected!("avx512f")
                && std::arch::is_x86_feature_detected!("avx512bw");
            let vpclmul = std::arch::is_x86_feature_detected!("vpclmulqdq");

            let expected = if avx512 && vpclmul {
                X86Backend::Vaes512
            } else if vaes {
                X86Backend::Vaes256
            } else {
                X86Backend::AesNi
            };

            assert_eq!(cipher.0.backend, expected);
            assert_eq!(cipher.0.vpclmul, vpclmul);
            eprintln!(
                "AES-GCM x86 backend: {:?} (vaes={vaes} avx512={avx512} vpclmul={vpclmul})",
                cipher.0.backend
            );
        }
    }

    #[test]
    fn gcm_max_len_constant() {
        // NIST SP 800-38D: at most 2^32 - 2 blocks of 16 bytes, leaving one
        // block of margin before the 32-bit counter wraps.
        assert_eq!(GCM_MAX_LEN, (u32::MAX as u64 - 1) * 16);
        assert_eq!(GCM_MAX_LEN, 68_719_476_704);
    }

    /// The hardware dispatch and the portable software path must agree on
    /// every short plaintext/AAD length, including the partial-block tails.
    #[test]
    fn gcm_dispatch_matches_soft_for_all_short_lengths() {
        let key = [0x3cu8; 32];
        let nonce = [0x5au8; 12];
        let round_keys = expand_key::<15>(&key);
        let h = encrypt_block::<15>(&round_keys, &[0u8; 16]);
        let cipher = Aes256Gcm::new(&key);

        for pt_len in 0..=129usize {
            for aad_len in 0..=33usize {
                let pt: Vec<u8> = (0..pt_len)
                    .map(|i| (i as u8).wrapping_mul(37).wrapping_add(11))
                    .collect();
                let aad: Vec<u8> = (0..aad_len)
                    .map(|i| (i as u8).wrapping_mul(53).wrapping_add(7))
                    .collect();

                let mut soft = pt.clone();
                let soft_tag = cipher.0.encrypt_in_place_soft(&mut soft, &round_keys, &h, &nonce, &aad);
                let mut dispatch = pt.clone();
                let dispatch_tag = cipher.encrypt_in_place(&mut dispatch, &nonce, &aad);

                assert_eq!(soft, dispatch, "ciphertext mismatch for pt_len={pt_len} aad_len={aad_len}");
                assert_eq!(
                    soft_tag.as_ref(),
                    dispatch_tag.as_ref(),
                    "tag mismatch for pt_len={pt_len} aad_len={aad_len}"
                );

                let tag: [u8; 16] = soft_tag.as_ref().try_into().unwrap();
                let mut soft_plain = dispatch.clone();
                cipher
                    .0
                    .decrypt_in_place_soft(&mut soft_plain, &round_keys, &h, &tag, &nonce, &aad)
                    .expect("software decrypt failed");
                assert_eq!(soft_plain, pt);

                let mut dispatch_plain = dispatch.clone();
                cipher
                    .decrypt_in_place(&mut dispatch_plain, &nonce, &aad, dispatch_tag.as_ref())
                    .expect("dispatch decrypt failed");
                assert_eq!(dispatch_plain, pt);
            }
        }
    }

    #[test]
    fn gcm_tag_mismatch_returns_error_soft() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let cipher = Aes256Gcm::new(&key);
        let mut buf = b"hello world".to_vec();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, &[]);
        let mut bad_tag: [u8; 16] = tag.as_ref().try_into().unwrap();
        bad_tag[0] ^= 0xff;
        let mut buf2 = buf.clone();
        assert!(cipher.decrypt_in_place(&mut buf2, &bad_tag, &nonce, &[]).is_err());
    }

    #[test]
    fn gcm_encrypt_decrypt_large_soft() {
        let key = [0xabu8; 32];
        let nonce = [0x01u8; 12];
        let aad = b"additional data";
        let plaintext: Vec<u8> = (0u8..=255u8).cycle().take(1024).collect();
        let round_keys = expand_key(&key);
        let h = encrypt_block(&round_keys, &[0u8; 16]);

        let cipher = Aes256Gcm::new(&key);
        let mut buf = plaintext.clone();
        let tag = cipher.0.encrypt_in_place_soft(&mut buf, &round_keys, &h, &nonce, aad);
        let tag_bytes: [u8; 16] = tag.as_ref().try_into().unwrap();
        cipher
            .0
            .decrypt_in_place_soft(&mut buf, &round_keys, &h, &tag_bytes, &nonce, aad)
            .expect("decrypt failed");
        assert_eq!(buf, plaintext);
    }

    #[test]
    fn gcm_empty_plaintext_nonempty_aad_soft() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308").unwrap();
        let nonce: [u8; 12] = hex::decode_array::<12>(b"cafebabefacedbaddecaf888").unwrap();
        let aad = hex::decode("feedfacedeadbeeffeedfacedeadbeef").unwrap();
        let round_keys = expand_key(&key);
        let h = encrypt_block(&round_keys, &[0u8; 16]);
        let cipher = Aes256Gcm::new(&key);
        let mut buf: Vec<u8> = vec![];
        let tag = cipher.0.encrypt_in_place_soft(&mut buf, &round_keys, &h, &nonce, &aad);
        let tag_bytes: [u8; 16] = tag.as_ref().try_into().unwrap();
        cipher
            .0
            .decrypt_in_place_soft(&mut buf, &round_keys, &h, &tag_bytes, &nonce, &aad)
            .expect("decrypt failed");
    }

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
