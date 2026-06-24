#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{Aead, AeadError, Hash, Hasher, StreamCipher, Xof, blake3::Blake3, chacha::ChaCha20Djb};

/// ChaCha20-BLAKE3 AEAD (encrypt-then-MAC).
///
/// The master key and nonce are fed through a BLAKE3-based KDF to derive
/// a ChaCha20 encryption key, a ChaCha20 nonce, and an authentication key.
/// The plaintext is encrypted with ChaCha20 and then MACed with
/// BLAKE3(keyed, aad || len(aad) || ciphertext || len(ciphertext)).
///
/// # Parameters
///
/// - Key: 256 bits (32 bytes)
/// - Nonce: 256 bits (32 bytes)
/// - Tag: 256 bits (32 bytes)
///
/// # Panics
///
/// [`encrypt_in_place`](Aead::encrypt_in_place) and
/// [`decrypt_in_place`](Aead::decrypt_in_place) **panic** if the nonce is
/// not exactly 32 bytes.
///
/// # Example
///
/// ```
/// use crypto::{Aead, chacha::ChaCha20Blake3};
///
/// let key = [0xab; 32];
/// let nonce = [0xcd; 32];
/// let aad = b"associated data";
/// let plaintext = b"hello world";
///
/// let cipher = ChaCha20Blake3::new(&key);
///
/// let mut buf = plaintext.to_vec();
/// let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad);
///
/// // buf now holds the ciphertext; tag is the 32-byte authentication tag.
///
/// cipher.decrypt_in_place(&mut buf, &nonce, aad, tag.as_ref())
///     .expect("decryption failed");
/// assert_eq!(&buf, plaintext);
/// ```
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ChaCha20Blake3 {
    key: [u8; 32],
}

impl ChaCha20Blake3 {
    pub fn new(key: &[u8; 32]) -> Self {
        return ChaCha20Blake3 {
            key: *key,
        };
    }
}

impl Aead for ChaCha20Blake3 {
    const TAG_SIZE: usize = 32;
    const NONCE_SIZE: usize = 32;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        assert!(nonce.len() == 32, "nonce must be 32 bytes");

        // kdf_out = BLAKE3.keyed(key, nonce)
        let mut kdf_out = [0u8; 72];
        let mut blake3_kdf = Blake3::new_keyed(&self.key);
        blake3_kdf.update(nonce);
        blake3_kdf.finalize_xof().squeeze(&mut kdf_out);

        // chacha20_key = kdf_out[0..32]
        // authentication_key = kdf_out[32..64]
        // chacha20_nonce = kdf_out[64..72]
        let chacha20_key: &[u8; 32] = &kdf_out[..32].try_into().unwrap();
        let authentication_key: &[u8; 32] = &kdf_out[32..64].try_into().unwrap();
        let chacha20_nonce: &[u8; 8] = &kdf_out[64..].try_into().unwrap();

        ChaCha20Djb::new(chacha20_key, chacha20_nonce).xor_keystream(in_out);

        // mac = BLAKE3.keyed(authentication_key, aad || aad.len_uint64_little_endian() || ciphertext || ciphertext.len_uint64_little_endian())
        let mut mac_hasher = Blake3::new_keyed(authentication_key);
        mac_hasher.update(aad);
        mac_hasher.update(&(aad.len() as u64).to_le_bytes());
        mac_hasher.update(&in_out);
        mac_hasher.update(&(in_out.len() as u64).to_le_bytes());
        let tag = mac_hasher.sum();

        #[cfg(feature = "zeroize")]
        kdf_out.zeroize();

        return tag;
    }

    fn decrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), AeadError> {
        assert!(nonce.len() == 32, "nonce must be 32 bytes");

        // kdf_out = BLAKE3.keyed(key, nonce)
        let mut kdf_out = [0u8; 72];
        let mut blake3_kdf = Blake3::new_keyed(&self.key);
        blake3_kdf.update(nonce);
        blake3_kdf.finalize_xof().squeeze(&mut kdf_out);

        // chacha20_key = kdf_out[0..32]
        // authentication_key = kdf_out[32..64]
        // chacha20_nonce = kdf_out[64..72]
        let chacha20_key: &[u8; 32] = kdf_out[..32].try_into().unwrap();
        let authentication_key: &[u8; 32] = kdf_out[32..64].try_into().unwrap();
        let chacha20_nonce: &[u8; 8] = kdf_out[64..].try_into().unwrap();

        let mut mac_hasher = Blake3::new_keyed(&authentication_key);
        mac_hasher.update(aad);
        mac_hasher.update(&(aad.len() as u64).to_le_bytes());
        mac_hasher.update(in_out);
        mac_hasher.update(&(in_out.len() as u64).to_le_bytes());
        let mac = mac_hasher.sum();

        if !constant_time_eq::constant_time_eq(mac.as_ref(), tag) {
            return Err(AeadError::InvalidCiphertext);
        }

        ChaCha20Djb::new(&chacha20_key, &chacha20_nonce).xor_keystream(in_out);

        #[cfg(feature = "zeroize")]
        kdf_out.zeroize();

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::ChaCha20Blake3;
    use crate::Aead;

    struct Test {
        plaintext: &'static str,
        key: &'static str,
        nonce: &'static str,
        aad: &'static str,
        ct: &'static str,
    }

    #[test]
    fn aead_chacha20blake3_test_vectors() {
        let tests = [
            Test {
                plaintext: "",
                key: "0000000000000000000000000000000000000000000000000000000000000000",
                nonce: "0000000000000000000000000000000000000000000000000000000000000000",
                aad: "",
                ct: "e074bcc1f324f0139dea37f8465aa7edf565f968aeae9bfa348c9a9c1c702ad2",
            },
            Test {
                plaintext: "4368614368613230",
                key: "0100000000000000000000000000000000000000000000000000000000000010",
                nonce: "1000000000000000000000000000000000000000000000000000000000000001",
                aad: "424c414b4533",
                ct: "af4d5f3ac75f3753a764e5af1d3396f9f6f5b5ea94889665372f39a9ae7aa55aa3d77b69680bfe45",
            },
            Test {
                plaintext: "b8f60975cd7057a003ac84df00d514624fe40cb7855c50dd6594f59b3a2580e5",
                key: "3eb02a239a2a66de159b9bb5486ccc10a6f63ddf5862ef076650513372353622",
                nonce: "719d34360dcf03dc7af6a4d1d9fd311b035cbc148241f1419f166537a5552aec",
                aad: "c8d69ca92da6c5fd22f1805179fcd36cb7a9d45848fa346ba7118c2f34d23a48",
                ct: "42c948ee385574606ce91ed09f4ef744c69b7101ef682aee8acdd14fd827499eefe6f43193680e3685f3fe0a702a0c19ceb8d7b539a02edec99783fdb9816eb5",
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let key: [u8; 32] = hex::decode(test.key).unwrap().try_into().unwrap();
            let nonce: [u8; 32] = hex::decode(test.nonce).unwrap().try_into().unwrap();
            let pt = hex::decode(test.plaintext).unwrap();
            let aad = hex::decode(test.aad).unwrap();
            let ct_tag = hex::decode(test.ct).unwrap();

            let expected_ct = &ct_tag[..ct_tag.len() - 32];
            let expected_tag = &ct_tag[ct_tag.len() - 32..];

            let cipher = ChaCha20Blake3::new(&key);

            let mut buf = pt.clone();
            let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad);
            assert_eq!(buf, expected_ct, "test {i}: ciphertext mismatch");
            assert_eq!(tag.as_ref(), expected_tag, "test {i}: tag mismatch");

            let mut buf2 = expected_ct.to_vec();
            cipher.decrypt_in_place(&mut buf2, &nonce, &aad, expected_tag).unwrap();
            assert_eq!(buf2, pt, "test {i}: plaintext mismatch");
        }
    }
}
