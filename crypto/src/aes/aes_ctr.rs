#![allow(unsafe_op_in_unsafe_fn)]

use super::aes::{encrypt_block, expand_key};
use crate::{StreamCipher, aes::RoundKeys};

/// AES-128 in CTR mode.
///
/// Create a new cipher with [`new`](Aes128Ctr::new).
/// [`xor_keystream`](StreamCipher::xor_keystream) to encrypt or decrypt
/// (CTR mode is symmetric).
/// You can move in the keystream with [`set_counter`](Aes128Ctr::set_counter).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes128Ctr(pub(crate) AesCtr<11>);

impl Aes128Ctr {
    /// Create a new AES-128-CTR stream cipher from a 16-byte key.
    ///
    /// The initial counter is zeroed.
    #[inline]
    pub fn new(key: &[u8; 16]) -> Self {
        Self(AesCtr::<11>::new(key))
    }

    /// Set the 16-byte counter block.
    ///
    /// For GCM this is `nonce || 0x00000002` (J₀ + 1).
    #[inline]
    pub fn set_counter(&mut self, counter: &[u8; 16]) {
        self.0.set_counter(counter)
    }
}

impl StreamCipher for Aes128Ctr {
    #[inline]
    fn xor_keystream(&mut self, in_out: &mut [u8]) {
        self.0.xor_keystream(in_out)
    }
}

/// AES-256 in CTR mode.
///
/// Create a new cipher with [`new`](Aes256Ctr::new).
/// [`xor_keystream`](StreamCipher::xor_keystream) to encrypt or decrypt
/// (CTR mode is symmetric).
/// You can move in the keystream with [`set_counter`](Aes256Ctr::set_counter).
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct Aes256Ctr(pub(crate) AesCtr<15>);

impl Aes256Ctr {
    /// Create a new AES-256-CTR stream cipher from a 32-byte key.
    ///
    /// The initial counter is zeroed.
    #[inline]
    pub fn new(key: &[u8; 32]) -> Self {
        Self(AesCtr::<15>::new(key))
    }

    /// Set the 16-byte counter block.
    ///
    /// For GCM this is `nonce || 0x00000002` (J₀ + 1).
    #[inline]
    pub fn set_counter(&mut self, counter: &[u8; 16]) {
        self.0.set_counter(counter)
    }
}

impl StreamCipher for Aes256Ctr {
    #[inline]
    fn xor_keystream(&mut self, in_out: &mut [u8]) {
        self.0.xor_keystream(in_out)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Generic implementation of AES in counter mode.
/// `N` is the number of rounds. 11 for AES-128 and 15 for AES-256
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct AesCtr<const N: usize> {
    round_keys: RoundKeys<N>,
    counter: [u8; 16],
}

impl AesCtr<11> {
    pub fn new(key: &[u8; 16]) -> Self {
        Self::new_inner(key)
    }
}

impl AesCtr<15> {
    pub fn new(key: &[u8; 32]) -> Self {
        Self::new_inner(key)
    }
}

impl<const N: usize> AesCtr<N> {
    /// Create a new cipher from a 16-byte key.
    ///
    /// The initial counter is zeroed.
    fn new_inner(key: &[u8]) -> Self {
        const { assert!(N == 11 || N == 15) };

        let round_keys_software = expand_key(key);

        #[cfg(target_arch = "aarch64")]
        {
            #[cfg(any(feature = "std", target_feature = "aes"))]
            use crate::aes::aes_arm64::expand_key_armv8;

            #[cfg(feature = "std")]
            if std::arch::is_aarch64_feature_detected!("aes") {
                return AesCtr {
                    round_keys: RoundKeys::Armv8(expand_key_armv8(round_keys_software)),
                    counter: [0u8; 16],
                };
            }

            #[cfg(all(not(feature = "std"), target_feature = "aes"))]
            return AesCtr {
                round_keys: RoundKeys::Armv8(expand_key_armv8(round_keys_software)),
                counter: [0u8; 16],
            };
        }

        #[cfg(target_arch = "x86_64")]
        {
            use crate::aes::aes_amd64::expand_key_x86_64;

            #[cfg(feature = "std")]
            if std::arch::is_x86_feature_detected!("aes") {
                return AesCtr {
                    round_keys: RoundKeys::X86_64(expand_key_x86_64(round_keys_software)),
                    counter: [0u8; 16],
                };
            }

            #[cfg(all(not(feature = "std"), target_feature = "aes"))]
            return AesCtr {
                round_keys: RoundKeys::X86_64(expand_key_x86_64(round_keys_software)),
                counter: [0u8; 16],
            };
        }

        AesCtr {
            round_keys: RoundKeys::Software(round_keys_software),
            counter: [0u8; 16],
        }
    }

    /// Create a new cipher from pre-computed round keys.
    /// It's useful to re-use AES-CTR in another cipher such AES-GCM
    #[inline]
    pub(crate) fn from_round_keys(round_keys: RoundKeys<N>) -> Self {
        const { assert!(N == 11 || N == 15) };

        Self {
            round_keys,
            counter: [0u8; 16],
        }
    }

    pub(crate) fn xor_keystream(&mut self, in_out: &mut [u8]) {
        match &self.round_keys {
            #[cfg(target_arch = "aarch64")]
            RoundKeys::Armv8(round_keys) => unsafe {
                use super::aes_ctr_arm64::xor_keystream_armv8;
                xor_keystream_armv8(round_keys, &mut self.counter, in_out);
            },
            #[cfg(target_arch = "x86_64")]
            RoundKeys::X86_64(round_keys) => unsafe {
                use super::aes_ctr_amd64::xor_keystream_aesni;
                xor_keystream_aesni(round_keys, &mut self.counter, in_out);
            },
            RoundKeys::Software(round_keys) => xor_keystream_soft(round_keys, &mut self.counter, in_out),
        }
    }

    /// Set the 16-byte counter block.
    ///
    /// For GCM this is `nonce || 0x00000002` (J₀ + 1).
    #[inline]
    pub fn set_counter(&mut self, counter: &[u8; 16]) {
        self.counter = *counter;
    }
}

fn xor_keystream_soft<const N: usize>(round_keys: &[[u8; 16]; N], counter: &mut [u8; 16], in_out: &mut [u8]) {
    let n = in_out.len();
    let mut i = 0;

    while i + 16 <= n {
        let ks = encrypt_block(&round_keys, counter);
        for k in 0..16 {
            in_out[i + k] ^= ks[k];
        }
        increment_counter(counter);
        i += 16;
    }

    if i < n {
        let ks = encrypt_block(&round_keys, counter);
        for k in 0..n - i {
            in_out[i + k] ^= ks[k];
        }
    }
}

#[inline]
fn increment_counter(counter: &mut [u8; 16]) {
    let counter_value = u32::from_be_bytes(counter[12..16].try_into().unwrap());
    counter[12..16].copy_from_slice(&counter_value.wrapping_add(1).to_be_bytes());
}

#[cfg(test)]
mod tests {
    use hex;

    use super::*;

    struct CtrVector {
        key: &'static str,
        counter: &'static str,
        plaintext: &'static str,
        ciphertext: &'static str,
    }

    // NIST SP 800-38A – Appendix F.5.1 CTR-AES128.Encrypt
    // Each vector shows a single block with its corresponding counter block value
    // (the counter is incremented in the last 4 bytes after each block).
    const NIST_CTR_128_VECTORS: &[CtrVector] = &[
        CtrVector {
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
            plaintext: "6bc1bee22e409f96e93d7e117393172a",
            ciphertext: "874d6191b620e3261bef6864990db6ce",
        },
        CtrVector {
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdff00",
            plaintext: "ae2d8a571e03ac9c9eb76fac45af8e51",
            ciphertext: "9806f66b7970fdff8617187bb9fffdff",
        },
        CtrVector {
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdff01",
            plaintext: "30c81c46a35ce411e5fbc1191a0a52ef",
            ciphertext: "5ae4df3edbd5d35e5b4f09020db03eab",
        },
        CtrVector {
            key: "2b7e151628aed2a6abf7158809cf4f3c",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdff02",
            plaintext: "f69f2445df4f9b17ad2b417be66c3710",
            ciphertext: "1e031dda2fbe03d1792170a0f3009cee",
        },
    ];

    // NIST SP 800-38A – Appendix F.5.3 CTR-AES256.Encrypt
    const NIST_CTR_256_VECTORS: &[CtrVector] = &[
        CtrVector {
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
            plaintext: "6bc1bee22e409f96e93d7e117393172a",
            ciphertext: "601ec313775789a5b7a7f504bbf3d228",
        },
        CtrVector {
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdff00",
            plaintext: "ae2d8a571e03ac9c9eb76fac45af8e51",
            ciphertext: "f443e3ca4d62b59aca84e990cacaf5c5",
        },
        CtrVector {
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdff01",
            plaintext: "30c81c46a35ce411e5fbc1191a0a52ef",
            ciphertext: "2b0930daa23de94ce87017ba2d84988d",
        },
        CtrVector {
            key: "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4",
            counter: "f0f1f2f3f4f5f6f7f8f9fafbfcfdff02",
            plaintext: "f69f2445df4f9b17ad2b417be66c3710",
            ciphertext: "dfc9c58db67aada613c2dd08457941a6",
        },
    ];

    use super::super::aes::expand_key;

    fn run_ctr_vector_128(v: &CtrVector) {
        let key: [u8; 16] = hex::decode_array::<16>(v.key.as_bytes()).unwrap();
        let counter: [u8; 16] = hex::decode_array::<16>(v.counter.as_bytes()).unwrap();
        let pt = hex::decode(v.plaintext).unwrap();
        let expected_ct = hex::decode(v.ciphertext).unwrap();

        let mut buf = pt.clone();
        let mut cipher = Aes128Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_ct, "AES-128-CTR mismatch for key={}", v.key);

        let mut buf_soft = pt.clone();
        let mut ctr_soft = counter;
        let rk: [[u8; 16]; 11] = expand_key::<11>(&key);
        xor_keystream_soft(&rk, &mut ctr_soft, &mut buf_soft);
        assert_eq!(buf_soft, expected_ct, "AES-128-CTR soft mismatch for key={}", v.key);

        assert_eq!(buf, buf_soft, "dispatch and soft xor_keystream differ for key={}", v.key);
    }

    fn run_ctr_vector_256(v: &CtrVector) {
        let key: [u8; 32] = hex::decode_array::<32>(v.key.as_bytes()).unwrap();
        let counter: [u8; 16] = hex::decode_array::<16>(v.counter.as_bytes()).unwrap();
        let pt = hex::decode(v.plaintext).unwrap();
        let expected_ct = hex::decode(v.ciphertext).unwrap();

        let mut buf = pt.clone();
        let mut cipher = Aes256Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_ct, "AES-256-CTR mismatch for key={}", v.key);

        let mut buf_soft = pt.clone();
        let mut ctr_soft = counter;
        let rk: [[u8; 16]; 15] = expand_key::<15>(&key);
        xor_keystream_soft(&rk, &mut ctr_soft, &mut buf_soft);
        assert_eq!(buf_soft, expected_ct, "AES-256-CTR soft mismatch for key={}", v.key);

        assert_eq!(buf, buf_soft, "dispatch and soft xor_keystream differ for key={}", v.key);
    }

    #[test]
    fn nist_aes128_ctr_vectors() {
        for v in NIST_CTR_128_VECTORS {
            run_ctr_vector_128(v);
        }
    }

    #[test]
    fn nist_aes128_ctr_combined_blocks() {
        // Encrypt all 4 blocks in one call to verify counter chaining
        let key: [u8; 16] = hex::decode_array::<16>(b"2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        let counter: [u8; 16] = hex::decode_array::<16>(b"f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap();
        let pt = hex::decode(b"6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710").unwrap();
        let expected_ct = hex::decode(b"874d6191b620e3261bef6864990db6ce9806f66b7970fdff8617187bb9fffdff5ae4df3edbd5d35e5b4f09020db03eab1e031dda2fbe03d1792170a0f3009cee").unwrap();

        let mut buf = pt.clone();
        let mut cipher = Aes128Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_ct);

        let mut buf_soft = pt.clone();
        let mut ctr_soft = counter;
        let rk: [[u8; 16]; 11] = expand_key::<11>(&key);
        xor_keystream_soft(&rk, &mut ctr_soft, &mut buf_soft);
        assert_eq!(buf_soft, expected_ct, "AES-128-CTR soft multi-block mismatch");
        assert_eq!(buf, buf_soft, "dispatch and soft xor_keystream differ on multi-block");
    }

    #[test]
    fn nist_aes256_ctr_vectors() {
        for v in NIST_CTR_256_VECTORS {
            run_ctr_vector_256(v);
        }
    }

    #[test]
    fn nist_aes256_ctr_combined_blocks() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4").unwrap();
        let counter: [u8; 16] = hex::decode_array::<16>(b"f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap();
        let pt = hex::decode(b"6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710").unwrap();
        let expected_ct = hex::decode(b"601ec313775789a5b7a7f504bbf3d228f443e3ca4d62b59aca84e990cacaf5c52b0930daa23de94ce87017ba2d84988ddfc9c58db67aada613c2dd08457941a6").unwrap();

        let mut buf = pt.clone();
        let mut cipher = Aes256Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_ct);

        let mut buf_soft = pt.clone();
        let mut ctr_soft = counter;
        let rk: [[u8; 16]; 15] = expand_key::<15>(&key);
        xor_keystream_soft(&rk, &mut ctr_soft, &mut buf_soft);
        assert_eq!(buf_soft, expected_ct, "AES-256-CTR soft multi-block mismatch");
        assert_eq!(buf, buf_soft, "dispatch and soft xor_keystream differ on multi-block");
    }

    #[test]
    fn aes128_ctr_empty_plaintext() {
        let key = [0xabu8; 16];
        let counter = [0x01u8; 16];
        let mut buf: Vec<u8> = vec![];
        let mut cipher = Aes128Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert!(buf.is_empty());
    }

    #[test]
    fn aes128_ctr_partial_block() {
        let key: [u8; 16] = hex::decode_array::<16>(b"2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        let counter: [u8; 16] = hex::decode_array::<16>(b"f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap();
        // First 5 bytes of the first NIST block
        let pt = hex::decode(b"6bc1bee22e").unwrap();
        let expected_ct = hex::decode(b"874d6191b6").unwrap();

        let mut buf = pt.clone();
        let mut cipher = Aes128Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_ct);
    }

    #[test]
    fn aes128_ctr_cross_block_boundary() {
        // 17 bytes – spans from block 1 into block 2
        let key: [u8; 16] = hex::decode_array::<16>(b"2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        let counter: [u8; 16] = hex::decode_array::<16>(b"f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap();
        // Take first block + 1 byte of second block
        let pt = hex::decode(b"6bc1bee22e409f96e93d7e117393172aae").unwrap();
        let expected_ct = hex::decode(b"874d6191b620e3261bef6864990db6ce98").unwrap();

        let mut buf = pt.clone();
        let mut cipher = Aes128Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_ct);
    }

    #[test]
    fn aes128_ctr_roundtrip() {
        let key = [0x42u8; 16];
        let counter = [0x07u8; 16];
        let pt: Vec<u8> = (0u8..=255u8).cycle().take(1024).collect();

        let mut buf = pt.clone();
        let mut cipher = Aes128Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        // XOR again with same keystream yields original plaintext
        let mut cipher2 = Aes128Ctr::new(&key);
        cipher2.set_counter(&counter);
        cipher2.xor_keystream(&mut buf);
        assert_eq!(buf, pt);
    }

    #[test]
    fn aes256_ctr_roundtrip() {
        let key = [0x42u8; 32];
        let counter = [0x07u8; 16];
        let pt: Vec<u8> = (0u8..=255u8).cycle().take(1024).collect();

        let mut buf = pt.clone();
        let mut cipher = Aes256Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        let mut cipher2 = Aes256Ctr::new(&key);
        cipher2.set_counter(&counter);
        cipher2.xor_keystream(&mut buf);
        assert_eq!(buf, pt);
    }

    #[test]
    fn aes128_ctr_zero_key_zero_counter() {
        let key = [0u8; 16];
        let counter = [0u8; 16];
        let pt = [0u8; 16];
        let expected_keystream: [u8; 16] = hex::decode_array::<16>(b"66e94bd4ef8a2c3b884cfa59ca342b2e").unwrap();

        let mut buf = pt.to_vec();
        let mut cipher = Aes128Ctr::new(&key);
        cipher.set_counter(&counter);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_keystream.to_vec());

        // decrypt (XOR again)
        let mut cipher2 = Aes128Ctr::new(&key);
        cipher2.set_counter(&counter);
        cipher2.xor_keystream(&mut buf);
        assert_eq!(buf, pt.to_vec());
    }

    #[test]
    fn aes128_ctr_increment_counter() {
        let mut ctr = [0u8; 16];
        increment_counter(&mut ctr);
        assert_eq!(ctr, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        let mut ctr2 = [0xffu8; 16];
        increment_counter(&mut ctr2);
        let expected: [u8; 16] = hex::decode_array::<16>(b"ffffffffffffffffffffffff00000000").unwrap();
        assert_eq!(ctr2, expected);
    }
}
