use crate::{
    Aead, AeadError, Bytes, Hash, StreamCipher,
    chacha::{ChaCha, chacha20_poly1305::update_poly1305_padded},
    poly1305::Poly1305,
};

/// The ChaCha8-Poly1305 AEAD, derived from ChaCha20-Poly1305 as standardized in RFC 8439
/// but with a reduced number of ChaCha rounds for embedded platforms.
///
/// # Parameters
///
/// - Key: 256 bits (32 bytes)
/// - Nonce: 96 bits (12 bytes)
/// - Tag: 128 bits (16 bytes)
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct ChaCha8Poly1305 {
    key: [u8; 32],
}

impl ChaCha8Poly1305 {
    /// Creates a new AEAD instance from a 32-byte key.
    pub fn new(key: &[u8; 32]) -> ChaCha8Poly1305 {
        return ChaCha8Poly1305 {
            key: *key,
        };
    }

    /// Generates the one-time Poly1305 key using ChaCha20 with counter=0.
    #[inline]
    fn poly1305_key_gen(&self, nonce: &[u8; 12]) -> ([u8; 32], ChaCha<8, true>) {
        let mut cipher = ChaCha::<8, true>::new(&self.key, nonce);
        cipher.set_counter(0);
        let mut block = [0u8; 64];
        cipher.xor_keystream(&mut block);
        let mut key = [0u8; 32];
        key.copy_from_slice(&block[..32]);
        return (key, cipher);
    }
}

impl Aead for ChaCha8Poly1305 {
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    fn encrypt_in_place(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> Hash {
        let nonce: &[u8; 12] = nonce.try_into().expect("nonce must be 12 bytes");
        let (poly1305key, mut cipher) = self.poly1305_key_gen(nonce);

        cipher.set_counter(1);
        cipher.xor_keystream(in_out);

        let mut mac = Poly1305::new(&poly1305key);
        update_poly1305_padded(&mut mac, aad);
        update_poly1305_padded(&mut mac, in_out);
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
        let (poly1305key, mut cipher) = self.poly1305_key_gen(nonce);

        let mut mac = Poly1305::new(&poly1305key);
        update_poly1305_padded(&mut mac, aad);
        update_poly1305_padded(&mut mac, in_out);
        mac.update(&(aad.len() as u64).to_le_bytes());
        mac.update(&(in_out.len() as u64).to_le_bytes());
        let computed = mac.finalize();

        if !constant_time_eq::constant_time_eq(&computed, tag) {
            return Err(AeadError::InvalidCiphertext);
        }

        cipher.set_counter(1);
        cipher.xor_keystream(in_out);

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aead_roundtrip() {
        let key = [0x55; 32];
        let nonce = [0xaa; 12];
        let aad = b"authenticated but not encrypted";
        let aead = ChaCha8Poly1305::new(&key);
        let plaintext = b"hello, world!";
        let mut buf = plaintext.to_vec();
        let tag = aead.encrypt_in_place(&mut buf, &nonce, aad);
        let result = aead.decrypt_in_place(&mut buf, &nonce, aad, tag.as_ref());
        assert!(result.is_ok());
        assert_eq!(&buf, plaintext);
    }
}
