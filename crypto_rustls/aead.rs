//! TLS 1.3 AEAD implementations for rustls.
//!
//! Maps the crate's in-place AEAD constructions
//! ([`crypto::aes::Aes128Gcm`], [`crypto::aes::Aes256Gcm`] and
//! [`crypto::chacha::ChaCha20Poly1305`]) onto rustls'
//! [`Tls13AeadAlgorithm`] / [`MessageEncrypter`] / [`MessageDecrypter`]
//! interfaces.
//!
//! TLS 1.3 records are encrypted with a nonce derived from the write IV and
//! the record sequence number, an additional-data value constructed from the
//! record header, and the inner plaintext's content-type byte. The 16-byte
//! authentication tag is appended to the ciphertext on the wire.

use crypto::Aead;
use rustls::{
    ConnectionTrafficSecrets, ContentType, Error, ProtocolVersion,
    crypto::cipher::{
        AeadKey, InboundOpaqueMessage, InboundPlainMessage, Iv, MessageDecrypter, MessageEncrypter, Nonce,
        OutboundOpaqueMessage, OutboundPlainMessage, PrefixedPayload, Tls13AeadAlgorithm, UnsupportedOperationError,
        make_tls13_aad,
    },
};

/// The TLS 1.3 AEAD construction for `TLS_AES_128_GCM_SHA256`.
#[derive(Debug)]
pub(crate) struct Aes128Gcm;

impl Tls13AeadAlgorithm for Aes128Gcm {
    fn encrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageEncrypter> {
        let key: [u8; crypto::aes::Aes128Gcm::KEY_SIZE] = key
            .as_ref()
            .try_into()
            .expect("rustls supplies a key of the length reported by key_len()");
        Box::new(Tls13Encrypter {
            aead: crypto::aes::Aes128Gcm::new(&key),
            iv,
        })
    }

    fn decrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageDecrypter> {
        let key: [u8; crypto::aes::Aes128Gcm::KEY_SIZE] = key
            .as_ref()
            .try_into()
            .expect("rustls supplies a key of the length reported by key_len()");
        Box::new(Tls13Decrypter {
            aead: crypto::aes::Aes128Gcm::new(&key),
            iv,
        })
    }

    fn key_len(&self) -> usize {
        crypto::aes::Aes128Gcm::KEY_SIZE
    }

    fn extract_keys(&self, key: AeadKey, iv: Iv) -> Result<ConnectionTrafficSecrets, UnsupportedOperationError> {
        Ok(ConnectionTrafficSecrets::Aes128Gcm {
            key,
            iv,
        })
    }
}

/// The TLS 1.3 AEAD construction for `TLS_AES_256_GCM_SHA384`.
#[derive(Debug)]
pub(crate) struct Aes256Gcm;

impl Tls13AeadAlgorithm for Aes256Gcm {
    fn encrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageEncrypter> {
        let key: [u8; crypto::aes::Aes256Gcm::KEY_SIZE] = key
            .as_ref()
            .try_into()
            .expect("rustls supplies a key of the length reported by key_len()");
        Box::new(Tls13Encrypter {
            aead: crypto::aes::Aes256Gcm::new(&key),
            iv,
        })
    }

    fn decrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageDecrypter> {
        let key: [u8; crypto::aes::Aes256Gcm::KEY_SIZE] = key
            .as_ref()
            .try_into()
            .expect("rustls supplies a key of the length reported by key_len()");
        Box::new(Tls13Decrypter {
            aead: crypto::aes::Aes256Gcm::new(&key),
            iv,
        })
    }

    fn key_len(&self) -> usize {
        crypto::aes::Aes256Gcm::KEY_SIZE
    }

    fn extract_keys(&self, key: AeadKey, iv: Iv) -> Result<ConnectionTrafficSecrets, UnsupportedOperationError> {
        Ok(ConnectionTrafficSecrets::Aes256Gcm {
            key,
            iv,
        })
    }
}

/// The TLS 1.3 AEAD construction for `TLS_CHACHA20_POLY1305_SHA256`.
#[derive(Debug)]
pub(crate) struct ChaCha20Poly1305;

impl Tls13AeadAlgorithm for ChaCha20Poly1305 {
    fn encrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageEncrypter> {
        let key: [u8; 32] = key
            .as_ref()
            .try_into()
            .expect("rustls supplies a key of the length reported by key_len()");
        Box::new(Tls13Encrypter {
            aead: crypto::chacha::ChaCha20Poly1305::new(&key),
            iv,
        })
    }

    fn decrypter(&self, key: AeadKey, iv: Iv) -> Box<dyn MessageDecrypter> {
        let key: [u8; 32] = key
            .as_ref()
            .try_into()
            .expect("rustls supplies a key of the length reported by key_len()");
        Box::new(Tls13Decrypter {
            aead: crypto::chacha::ChaCha20Poly1305::new(&key),
            iv,
        })
    }

    fn key_len(&self) -> usize {
        32
    }

    fn extract_keys(&self, key: AeadKey, iv: Iv) -> Result<ConnectionTrafficSecrets, UnsupportedOperationError> {
        Ok(ConnectionTrafficSecrets::Chacha20Poly1305 {
            key,
            iv,
        })
    }
}

struct Tls13Encrypter<A> {
    aead: A,
    iv: Iv,
}

impl<A: Aead + Send + Sync> MessageEncrypter for Tls13Encrypter<A> {
    fn encrypt(&mut self, msg: OutboundPlainMessage<'_>, seq: u64) -> Result<OutboundOpaqueMessage, Error> {
        let total_len = self.encrypted_payload_len(msg.payload.len());

        // Reserve the record header and lay out the inner plaintext:
        // content || content_type.
        let mut payload = PrefixedPayload::with_capacity(total_len);
        payload.extend_from_chunks(&msg.payload);
        payload.extend_from_slice(&msg.typ.to_array());

        // The additional data covers the whole ciphertext, including the tag.
        let aad = make_tls13_aad(total_len);
        let nonce = Nonce::new(&self.iv, seq);
        let tag = self.aead.encrypt_in_place(payload.as_mut(), &nonce.0, &aad);
        payload.extend_from_slice(tag.as_ref());

        Ok(OutboundOpaqueMessage::new(
            ContentType::ApplicationData,
            ProtocolVersion::TLSv1_2,
            payload,
        ))
    }

    fn encrypted_payload_len(&self, payload_len: usize) -> usize {
        payload_len + 1 + A::TAG_SIZE
    }
}

struct Tls13Decrypter<A> {
    aead: A,
    iv: Iv,
}

impl<A: Aead + Send + Sync> MessageDecrypter for Tls13Decrypter<A> {
    fn decrypt<'a>(&mut self, mut msg: InboundOpaqueMessage<'a>, seq: u64) -> Result<InboundPlainMessage<'a>, Error> {
        if msg.payload.len() < A::TAG_SIZE + 1 {
            return Err(Error::DecryptError);
        }

        let aad = make_tls13_aad(msg.payload.len());
        let nonce = Nonce::new(&self.iv, seq);

        let cipher_len = msg.payload.len() - A::TAG_SIZE;
        let (ciphertext, tag) = msg.payload.split_at_mut(cipher_len);

        self.aead
            .decrypt_in_place(ciphertext, &nonce.0, &aad, tag)
            .map_err(|_| Error::DecryptError)?;

        msg.payload.truncate(cipher_len);
        msg.into_tls13_unpadded_message()
    }
}

#[cfg(test)]
mod tests {
    use rustls::crypto::cipher::OutboundChunks;

    use super::*;

    const IV_BYTES: [u8; 12] = [0x11; 12];

    fn round_trip<A: Aead + Send + Sync>(make: impl Fn() -> A) {
        let mut enc = Tls13Encrypter {
            aead: make(),
            iv: Iv::from(IV_BYTES),
        };
        let mut dec = Tls13Decrypter {
            aead: make(),
            iv: Iv::from(IV_BYTES),
        };

        let plaintext = b"hello tls 1.3 record";
        let chunks = [&plaintext[..]];
        let msg = OutboundPlainMessage {
            typ: ContentType::ApplicationData,
            version: ProtocolVersion::TLSv1_3,
            payload: OutboundChunks::new(&chunks),
        };
        let opaque = enc.encrypt(msg, 7).unwrap();
        let mut buf = opaque.payload.as_ref().to_vec();
        assert_eq!(buf.len(), plaintext.len() + 1 + A::TAG_SIZE);

        let inbound = InboundOpaqueMessage::new(ContentType::ApplicationData, ProtocolVersion::TLSv1_2, &mut buf);
        let decrypted = dec.decrypt(inbound, 7).unwrap();
        assert_eq!(decrypted.payload, plaintext);
        assert_eq!(decrypted.typ, ContentType::ApplicationData);
    }

    fn rejects_tampering<A: Aead + Send + Sync>(make: impl FnOnce() -> A) {
        let mut dec = Tls13Decrypter {
            aead: make(),
            iv: Iv::from(IV_BYTES),
        };
        let mut buf = vec![0u8; 32];
        buf[0] ^= 0xff;
        let inbound = InboundOpaqueMessage::new(ContentType::ApplicationData, ProtocolVersion::TLSv1_2, &mut buf);
        assert!(matches!(dec.decrypt(inbound, 1), Err(Error::DecryptError)));
    }

    #[test]
    fn aes128gcm_round_trip() {
        round_trip(|| crypto::aes::Aes128Gcm::new(&[0x24; 16]));
        rejects_tampering(|| crypto::aes::Aes128Gcm::new(&[0x24; 16]));
    }

    #[test]
    fn aes256gcm_round_trip() {
        round_trip(|| crypto::aes::Aes256Gcm::new(&[0x24; 32]));
        rejects_tampering(|| crypto::aes::Aes256Gcm::new(&[0x24; 32]));
    }

    #[test]
    fn chacha20poly1305_round_trip() {
        round_trip(|| crypto::chacha::ChaCha20Poly1305::new(&[0x24; 32]));
        rejects_tampering(|| crypto::chacha::ChaCha20Poly1305::new(&[0x24; 32]));
    }

    #[test]
    fn decrypter_rejects_short_record() {
        let mut dec = Tls13Decrypter {
            aead: crypto::aes::Aes128Gcm::new(&[0x24; 16]),
            iv: Iv::from(IV_BYTES),
        };
        let mut buf = [0u8; 8];
        let inbound = InboundOpaqueMessage::new(ContentType::ApplicationData, ProtocolVersion::TLSv1_2, &mut buf);
        assert!(matches!(dec.decrypt(inbound, 1), Err(Error::DecryptError)));
    }
}
