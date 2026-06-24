#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{Aead, AeadError, Hash, Hasher, StreamCipher, Xof, blake3::Blake3, chacha::ChaCha12Djb};

/// A Faster but slightly "less secure" version of [`super::ChaCha20Blake3`] using `ChaCha12`
/// instead of `ChaCha20` and truncating the `BLAKE3` authentication tah to 16 bytes. `ChaCha12` still
/// has higher security margins than `AES-256`.
///
/// You might want to use `ChaCha12Blake3` on embedded systems, microcontrollers and for radio protocols.
/// In all other situations, you might prefer the stronger [`super::ChaCha20Blake3`].
///
/// # Parameters
///
/// - Key: 256 bits (32 bytes)
/// - Nonce: 256 bits (32 bytes)
/// - Tag: 128 bits (16 bytes)
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
/// use crypto::{Aead, chacha::ChaCha12Blake3};
///
/// let key = [0xab; 32];
/// let nonce = [0xcd; 32];
/// let aad = b"associated data";
/// let plaintext = b"hello world";
///
/// let cipher = ChaCha12Blake3::new(&key);
///
/// let mut buf = plaintext.to_vec();
/// let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad);
///
/// // buf now holds the ciphertext; tag is the 16-byte authentication tag.
///
/// cipher.decrypt_in_place(&mut buf, &nonce, aad, tag.as_ref())
///     .expect("decryption failed");
/// assert_eq!(&buf, plaintext);
/// ```
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ChaCha12Blake3 {
    key: [u8; 32],
}

impl ChaCha12Blake3 {
    pub fn new(key: &[u8; 32]) -> Self {
        return ChaCha12Blake3 {
            key: *key,
        };
    }
}

impl Aead for ChaCha12Blake3 {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 32;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        assert!(nonce.len() == 32, "nonce must be 32 bytes");

        // kdf_out = BLAKE3.keyed(key, nonce)
        let mut kdf_out = [0u8; 72];
        let mut blake3_kdf = Blake3::new_keyed(&self.key);
        blake3_kdf.update(nonce);
        blake3_kdf.finalize_xof().squeeze(&mut kdf_out);

        // chacha12_key = kdf_out[0..32]
        // authentication_key = kdf_out[32..64]
        // chacha12_nonce = kdf_out[64..72]
        let chacha12_key: &[u8; 32] = &kdf_out[..32].try_into().unwrap();
        let authentication_key: &[u8; 32] = &kdf_out[32..64].try_into().unwrap();
        let chacha12_nonce: &[u8; 8] = &kdf_out[64..].try_into().unwrap();

        ChaCha12Djb::new(chacha12_key, chacha12_nonce).xor_keystream(in_out);

        // mac = BLAKE3.keyed(authentication_key, aad || aad.len_uint64_little_endian() || ciphertext || ciphertext.len_uint64_little_endian())
        let mut mac_hasher = Blake3::new_keyed(authentication_key);
        mac_hasher.update(aad);
        mac_hasher.update(&(aad.len() as u64).to_le_bytes());
        mac_hasher.update(&in_out);
        mac_hasher.update(&(in_out.len() as u64).to_le_bytes());
        let mut tag = mac_hasher.sum();
        tag.0.length = 16;

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

        // chacha12_key = kdf_out[0..32]
        // authentication_key = kdf_out[32..64]
        // chacha12_nonce = kdf_out[64..72]
        let chacha12_key: &[u8; 32] = kdf_out[..32].try_into().unwrap();
        let authentication_key: &[u8; 32] = kdf_out[32..64].try_into().unwrap();
        let chacha12_nonce: &[u8; 8] = kdf_out[64..].try_into().unwrap();

        let mut mac_hasher = Blake3::new_keyed(&authentication_key);
        mac_hasher.update(aad);
        mac_hasher.update(&(aad.len() as u64).to_le_bytes());
        mac_hasher.update(in_out);
        mac_hasher.update(&(in_out.len() as u64).to_le_bytes());
        let mut mac = mac_hasher.sum();
        mac.0.length = 16;

        if !constant_time_eq::constant_time_eq(mac.as_ref(), tag) {
            return Err(AeadError::InvalidCiphertext);
        }

        ChaCha12Djb::new(&chacha12_key, &chacha12_nonce).xor_keystream(in_out);

        #[cfg(feature = "zeroize")]
        kdf_out.zeroize();

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::ChaCha12Blake3;
    use crate::{Aead, Hasher, Xof, blake3::Blake3};

    #[test]
    fn tag_is_truncated_blake3_mac() {
        let key = [0xABu8; 32];
        let nonce = [0xCDu8; 32];
        let plaintext = b"hello ChaCha12-BLAKE3";
        let aad = b"some AAD";

        let cipher = ChaCha12Blake3::new(&key);

        let mut buf = plaintext.to_vec();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad);

        let mut kdf_out = [0u8; 72];
        let mut blake3_kdf = Blake3::new_keyed(&key);
        blake3_kdf.update(&nonce);
        blake3_kdf.finalize_xof().squeeze(&mut kdf_out);
        let authentication_key: &[u8; 32] = &kdf_out[32..64].try_into().unwrap();

        let mut mac_hasher = Blake3::new_keyed(authentication_key);
        mac_hasher.update(aad);
        mac_hasher.update(&(aad.len() as u64).to_le_bytes());
        mac_hasher.update(&buf);
        mac_hasher.update(&(buf.len() as u64).to_le_bytes());
        let expected_tag = mac_hasher.sum();

        assert_eq!(
            tag.as_ref(),
            &expected_tag.as_ref()[..16],
            "tag should be truncated BLAKE3 MAC of aad|len|ciphertext|len"
        );
    }

    #[test]
    fn roundtrip() {
        let key = [0xABu8; 32];
        let nonce = [0xCDu8; 32];
        let plaintext = b"hello ChaCha12-BLAKE3";
        let aad = b"some AAD";

        let cipher = ChaCha12Blake3::new(&key);

        let mut buf = plaintext.to_vec();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad);
        assert_ne!(buf, plaintext, "ciphertext should differ from plaintext");

        cipher.decrypt_in_place(&mut buf, &nonce, aad, tag.as_ref()).unwrap();
        assert_eq!(buf, plaintext, "roundtrip failed");
    }
}
