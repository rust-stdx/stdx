//! QUIC (RFC 9001) packet protection for this provider.
//!
//! Each TLS 1.3 cipher suite exposes a [`KeyBuilder`] implementing
//! [`rustls::quic::Algorithm`], which rustls uses to derive QUIC packet
//! protection keys from the connection traffic secrets. The packet keys
//! perform AEAD protection of packet payloads; the header protection keys
//! apply the AES-ECB or ChaCha20 mask defined in RFC 9001 §5.4.

use crypto::{
    Aead, StreamCipher,
    aes::{Aes128Block, Aes256Block},
    chacha::ChaCha20Ietf,
};
use rustls::{
    Error,
    crypto::cipher::{AeadKey, Iv, Nonce},
    quic::{Algorithm, HeaderProtectionKey, PacketKey, Tag},
};

/// QUIC HP sample length for all supported cipher suites (RFC 9001 §5.4.2).
const SAMPLE_LEN: usize = 16;

/// AEAD construction used for packet protection.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PacketAlgorithm {
    Aes128Gcm,
    Aes256Gcm,
    ChaCha20Poly1305,
}

/// Header protection construction (RFC 9001 §5.4.3 and §5.4.4).
#[derive(Debug, Clone, Copy)]
pub(crate) enum HeaderAlgorithm {
    Aes128,
    Aes256,
    ChaCha20,
}

/// The QUIC key derivation and protection algorithms for one cipher suite.
#[derive(Debug)]
pub(crate) struct KeyBuilder {
    pub(crate) packet: PacketAlgorithm,
    pub(crate) header: HeaderAlgorithm,
    pub(crate) confidentiality_limit: u64,
    pub(crate) integrity_limit: u64,
}

impl Algorithm for KeyBuilder {
    fn packet_key(&self, key: AeadKey, iv: Iv) -> Box<dyn PacketKey> {
        Box::new(QuicPacketKey {
            cipher: self.packet.build(key.as_ref()),
            iv,
            confidentiality_limit: self.confidentiality_limit,
            integrity_limit: self.integrity_limit,
        })
    }

    fn header_protection_key(&self, key: AeadKey) -> Box<dyn HeaderProtectionKey> {
        // The AES key schedule is derived once here, not per packet: header
        // protection is on the per-packet hot path.
        let cipher = match self.header {
            HeaderAlgorithm::Aes128 => {
                let key: [u8; 16] = key.as_ref().try_into().expect("rustls supplies 16-byte AES-128 keys");
                HeaderCipher::Aes128(Aes128Block::new(&key))
            }
            HeaderAlgorithm::Aes256 => {
                let key: [u8; 32] = key.as_ref().try_into().expect("rustls supplies 32-byte AES-256 keys");
                HeaderCipher::Aes256(Aes256Block::new(&key))
            }
            HeaderAlgorithm::ChaCha20 => {
                let mut key_bytes = [0u8; 32];
                key_bytes.copy_from_slice(key.as_ref());
                HeaderCipher::ChaCha20(key_bytes)
            }
        };
        Box::new(QuicHeaderProtectionKey {
            cipher,
        })
    }

    fn aead_key_len(&self) -> usize {
        match self.packet {
            PacketAlgorithm::Aes128Gcm => 16,
            PacketAlgorithm::Aes256Gcm | PacketAlgorithm::ChaCha20Poly1305 => 32,
        }
    }
}

/// TLS 1.3 `TLS_AES_128_GCM_SHA256` QUIC parameters.
pub(crate) static AES_128_GCM: KeyBuilder = KeyBuilder {
    packet: PacketAlgorithm::Aes128Gcm,
    header: HeaderAlgorithm::Aes128,
    // ref: RFC 9001, appendix B.1.1 and B.1.2.
    confidentiality_limit: 1 << 23,
    integrity_limit: 1 << 52,
};

/// TLS 1.3 `TLS_AES_256_GCM_SHA384` QUIC parameters.
pub(crate) static AES_256_GCM: KeyBuilder = KeyBuilder {
    packet: PacketAlgorithm::Aes256Gcm,
    header: HeaderAlgorithm::Aes256,
    // ref: RFC 9001, appendix B.1.1 and B.1.2.
    confidentiality_limit: 1 << 23,
    integrity_limit: 1 << 52,
};

/// TLS 1.3 `TLS_CHACHA20_POLY1305_SHA256` QUIC parameters.
pub(crate) static CHACHA20_POLY1305: KeyBuilder = KeyBuilder {
    packet: PacketAlgorithm::ChaCha20Poly1305,
    header: HeaderAlgorithm::ChaCha20,
    // ref: RFC 9001, section 6.6.
    confidentiality_limit: u64::MAX,
    integrity_limit: 1 << 36,
};

enum PacketCipher {
    Aes128(crypto::aes::Aes128Gcm),
    Aes256(crypto::aes::Aes256Gcm),
    ChaCha20Poly1305(crypto::chacha::ChaCha20Poly1305),
}

impl PacketAlgorithm {
    fn build(&self, key: &[u8]) -> PacketCipher {
        match self {
            PacketAlgorithm::Aes128Gcm => {
                let key: [u8; 16] = key.try_into().expect("rustls supplies 16-byte AES-128 keys");
                PacketCipher::Aes128(crypto::aes::Aes128Gcm::new(&key))
            }
            PacketAlgorithm::Aes256Gcm => {
                let key: [u8; 32] = key.try_into().expect("rustls supplies 32-byte AES-256 keys");
                PacketCipher::Aes256(crypto::aes::Aes256Gcm::new(&key))
            }
            PacketAlgorithm::ChaCha20Poly1305 => {
                let key: [u8; 32] = key.try_into().expect("rustls supplies 32-byte ChaCha20 keys");
                PacketCipher::ChaCha20Poly1305(crypto::chacha::ChaCha20Poly1305::new(&key))
            }
        }
    }
}

impl PacketCipher {
    fn encrypt(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8]) -> [u8; 16] {
        let tag = match self {
            PacketCipher::Aes128(cipher) => cipher.encrypt_in_place(in_out, nonce, aad),
            PacketCipher::Aes256(cipher) => cipher.encrypt_in_place(in_out, nonce, aad),
            PacketCipher::ChaCha20Poly1305(cipher) => cipher.encrypt_in_place(in_out, nonce, aad),
        };
        tag.as_ref().try_into().expect("AEAD tag is 16 bytes")
    }

    fn decrypt(&self, in_out: &mut [u8], nonce: &[u8], aad: &[u8], tag: &[u8]) -> Result<(), Error> {
        let result = match self {
            PacketCipher::Aes128(cipher) => cipher.decrypt_in_place(in_out, nonce, aad, tag),
            PacketCipher::Aes256(cipher) => cipher.decrypt_in_place(in_out, nonce, aad, tag),
            PacketCipher::ChaCha20Poly1305(cipher) => cipher.decrypt_in_place(in_out, nonce, aad, tag),
        };
        result.map_err(|_| Error::DecryptError)
    }
}

struct QuicPacketKey {
    cipher: PacketCipher,
    iv: Iv,
    confidentiality_limit: u64,
    integrity_limit: u64,
}

impl PacketKey for QuicPacketKey {
    fn encrypt_in_place(&self, packet_number: u64, header: &[u8], payload: &mut [u8]) -> Result<Tag, Error> {
        let nonce = Nonce::new(&self.iv, packet_number);
        let tag = self.cipher.encrypt(payload, &nonce.0, header);
        Ok(Tag::from(&tag[..]))
    }

    fn encrypt_in_place_for_path(
        &self,
        path_id: u32,
        packet_number: u64,
        header: &[u8],
        payload: &mut [u8],
    ) -> Result<Tag, Error> {
        let nonce = Nonce::for_path(path_id, &self.iv, packet_number);
        let tag = self.cipher.encrypt(payload, &nonce.0, header);
        Ok(Tag::from(&tag[..]))
    }

    fn decrypt_in_place<'a>(
        &self,
        packet_number: u64,
        header: &[u8],
        payload: &'a mut [u8],
    ) -> Result<&'a [u8], Error> {
        if payload.len() < 16 {
            return Err(Error::DecryptError);
        }

        let nonce = Nonce::new(&self.iv, packet_number);
        let split = payload.len() - 16;
        let (data, tag) = payload.split_at_mut(split);
        self.cipher.decrypt(data, &nonce.0, header, tag)?;
        Ok(&payload[..split])
    }

    fn decrypt_in_place_for_path<'a>(
        &self,
        path_id: u32,
        packet_number: u64,
        header: &[u8],
        payload: &'a mut [u8],
    ) -> Result<&'a [u8], Error> {
        if payload.len() < 16 {
            return Err(Error::DecryptError);
        }

        let nonce = Nonce::for_path(path_id, &self.iv, packet_number);
        let split = payload.len() - 16;
        let (data, tag) = payload.split_at_mut(split);
        self.cipher.decrypt(data, &nonce.0, header, tag)?;
        Ok(&payload[..split])
    }

    fn tag_len(&self) -> usize {
        16
    }

    fn confidentiality_limit(&self) -> u64 {
        self.confidentiality_limit
    }

    fn integrity_limit(&self) -> u64 {
        self.integrity_limit
    }
}

/// The keyed header-protection primitive for one algorithm.
enum HeaderCipher {
    Aes128(Aes128Block),
    Aes256(Aes256Block),
    ChaCha20([u8; 32]),
}

struct QuicHeaderProtectionKey {
    cipher: HeaderCipher,
}

impl QuicHeaderProtectionKey {
    fn mask(&self, sample: &[u8]) -> Result<[u8; 5], Error> {
        let sample: [u8; SAMPLE_LEN] = sample
            .try_into()
            .map_err(|_| Error::General("invalid QUIC header protection sample length".into()))?;

        let mut mask = [0u8; 5];
        match &self.cipher {
            HeaderCipher::Aes128(cipher) => {
                mask.copy_from_slice(&cipher.encrypt_block(&sample)[..5]);
            }
            HeaderCipher::Aes256(cipher) => {
                mask.copy_from_slice(&cipher.encrypt_block(&sample)[..5]);
            }
            HeaderCipher::ChaCha20(key) => {
                // RFC 9001 §5.4.4: the first four bytes of the sample are the
                // block counter (little-endian), the rest is the nonce.
                let counter = u32::from_le_bytes(sample[..4].try_into().expect("four bytes"));
                let nonce: &[u8; 12] = sample[4..].try_into().expect("twelve bytes");
                let mut cipher = ChaCha20Ietf::new(key, nonce);
                cipher.set_counter(counter);
                cipher.xor_keystream(&mut mask);
            }
        }
        Ok(mask)
    }

    /// Applies or removes the header protection mask (RFC 9001 §5.4.1).
    fn xor_in_place(&self, sample: &[u8], first: &mut u8, packet_number: &mut [u8], masked: bool) -> Result<(), Error> {
        let mask = self.mask(sample)?;
        let (first_mask, pn_mask) = mask.split_first().expect("mask is non-empty");

        if packet_number.len() > pn_mask.len() {
            return Err(Error::General("QUIC packet number is too long".into()));
        }

        const LONG_HEADER_FORM: u8 = 0x80;
        let bits = if *first & LONG_HEADER_FORM == LONG_HEADER_FORM {
            0x0f
        } else {
            0x1f
        };

        let first_plain = if masked { *first ^ (first_mask & bits) } else { *first };
        let pn_len = (first_plain & 0x03) as usize + 1;

        *first ^= first_mask & bits;
        for (dst, m) in packet_number.iter_mut().zip(pn_mask).take(pn_len) {
            *dst ^= m;
        }

        Ok(())
    }
}

impl HeaderProtectionKey for QuicHeaderProtectionKey {
    fn encrypt_in_place(&self, sample: &[u8], first: &mut u8, packet_number: &mut [u8]) -> Result<(), Error> {
        self.xor_in_place(sample, first, packet_number, false)
    }

    fn decrypt_in_place(&self, sample: &[u8], first: &mut u8, packet_number: &mut [u8]) -> Result<(), Error> {
        self.xor_in_place(sample, first, packet_number, true)
    }

    fn sample_len(&self) -> usize {
        SAMPLE_LEN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex_to<const N: usize>(s: &str) -> [u8; N] {
        let bytes = hex::decode(s).unwrap();
        bytes.try_into().unwrap()
    }

    #[test]
    fn aes128_header_protection_rfc9001_a3() {
        // RFC 9001, appendix A.3.
        let key: [u8; 16] = hex_to("9f50449e04a0e810283a1e9933adedd2");
        let sample: [u8; 16] = hex_to("d1b1c98dd7689fb8ec11d242b123dc9b");
        let key = QuicHeaderProtectionKey {
            cipher: HeaderCipher::Aes128(Aes128Block::new(&key)),
        };
        assert_eq!(hex::encode(key.mask(&sample).unwrap()), "437b9aec36");
    }

    #[test]
    fn chacha20_header_protection_rfc9001_a5() {
        // RFC 9001, appendix A.5.
        let key: [u8; 32] = hex_to("25a282b9e82f06f21f488917a4fc8f1b73573685608597d0efcb076b0ab7a7a4");
        let sample: [u8; 16] = hex_to("5e5cd55c41f69080575d7999c25a5bfb");
        let key = QuicHeaderProtectionKey {
            cipher: HeaderCipher::ChaCha20(key),
        };
        assert_eq!(hex::encode(key.mask(&sample).unwrap()), "aefefe7d03");
    }

    #[test]
    fn chacha20_packet_key_rfc9001_a5() {
        // RFC 9001, appendix A.5: a minimal short-header packet.
        let key: [u8; 32] = hex_to("c6d98ff3441c3fe1b2182094f69caa2ed4b716b65488960a7a984979fb23e1c8");
        let iv = Iv::from(hex_to::<12>("e0459b3474bdd0e44a41c144"));
        let packet = QuicPacketKey {
            cipher: PacketCipher::ChaCha20Poly1305(crypto::chacha::ChaCha20Poly1305::new(&key)),
            iv,
            confidentiality_limit: u64::MAX,
            integrity_limit: 1 << 36,
        };

        let header = hex_to::<4>("4200bff4");
        let mut payload = [0x01u8];
        let tag = packet.encrypt_in_place(654_360_564, &header, &mut payload).unwrap();

        let mut expected = hex_to::<17>("655e5cd55c41f69080575d7999c25a5bfb");
        assert_eq!(payload[0], expected[0]);
        assert_eq!(tag.as_ref(), &expected[1..]);
        expected[1..].copy_from_slice(tag.as_ref());

        // Decryption returns the original plaintext.
        let decrypted = packet.decrypt_in_place(654_360_564, &header, &mut expected).unwrap();
        assert_eq!(decrypted, [0x01]);
    }

    #[test]
    fn header_protection_round_trip() {
        for algorithm in [
            HeaderAlgorithm::Aes128,
            HeaderAlgorithm::Aes256,
            HeaderAlgorithm::ChaCha20,
        ] {
            let cipher = match algorithm {
                HeaderAlgorithm::Aes128 => HeaderCipher::Aes128(Aes128Block::new(&[0x42; 16])),
                HeaderAlgorithm::Aes256 => HeaderCipher::Aes256(Aes256Block::new(&[0x42; 32])),
                HeaderAlgorithm::ChaCha20 => HeaderCipher::ChaCha20([0x42; 32]),
            };
            let key = QuicHeaderProtectionKey {
                cipher,
            };
            let sample = [0x24u8; 16];

            let mut first = 0xc3;
            let mut packet_number = [0x01, 0x02, 0x03, 0x04];
            let protected_first = first;
            let protected_pn = packet_number;

            key.encrypt_in_place(&sample, &mut first, &mut packet_number).unwrap();
            key.decrypt_in_place(&sample, &mut first, &mut packet_number).unwrap();

            assert_eq!(first, protected_first, "{algorithm:?}");
            assert_eq!(packet_number, protected_pn, "{algorithm:?}");
        }
    }

    #[test]
    fn packet_key_round_trip() {
        for packet in [
            PacketAlgorithm::Aes128Gcm,
            PacketAlgorithm::Aes256Gcm,
            PacketAlgorithm::ChaCha20Poly1305,
        ] {
            let key_len = match packet {
                PacketAlgorithm::Aes128Gcm => 16,
                PacketAlgorithm::Aes256Gcm | PacketAlgorithm::ChaCha20Poly1305 => 32,
            };
            let key = vec![0x11u8; key_len];
            let cipher = packet.build(&key);
            let packet_key = QuicPacketKey {
                cipher,
                iv: Iv::from([0x22u8; 12]),
                confidentiality_limit: u64::MAX,
                integrity_limit: u64::MAX,
            };

            let header = [0x40, 0x00, 0x11, 0x22];
            let mut payload = *b"quic payload";
            let tag = packet_key.encrypt_in_place(9, &header, &mut payload).unwrap();

            let mut packet = payload.to_vec();
            packet.extend_from_slice(tag.as_ref());
            let decrypted = packet_key.decrypt_in_place(9, &header, &mut packet).unwrap();
            assert_eq!(decrypted, b"quic payload");
        }
    }

    #[test]
    fn packet_key_multipath_round_trip() {
        for packet in [
            PacketAlgorithm::Aes128Gcm,
            PacketAlgorithm::Aes256Gcm,
            PacketAlgorithm::ChaCha20Poly1305,
        ] {
            let key_len = match packet {
                PacketAlgorithm::Aes128Gcm => 16,
                PacketAlgorithm::Aes256Gcm | PacketAlgorithm::ChaCha20Poly1305 => 32,
            };
            let packet_key = QuicPacketKey {
                cipher: packet.build(&vec![0x11u8; key_len]),
                iv: Iv::from([0x22u8; 12]),
                confidentiality_limit: u64::MAX,
                integrity_limit: u64::MAX,
            };

            let header = [0x40, 0x00, 0x11, 0x22];
            let plaintext = *b"quic multipath payload";

            for path_id in [0u32, 1, 2, 0xdead_beef] {
                let mut payload = plaintext;
                let tag = packet_key
                    .encrypt_in_place_for_path(path_id, 9, &header, &mut payload)
                    .unwrap();

                let mut packet = payload.to_vec();
                packet.extend_from_slice(tag.as_ref());
                let decrypted = packet_key
                    .decrypt_in_place_for_path(path_id, 9, &header, &mut packet)
                    .unwrap();
                assert_eq!(decrypted, &plaintext, "path {path_id}");
            }

            // A non-zero path id changes the nonce, so the ciphertext changes.
            let mut path0 = plaintext;
            let mut path1 = plaintext;
            packet_key.encrypt_in_place_for_path(0, 9, &header, &mut path0).unwrap();
            packet_key.encrypt_in_place_for_path(1, 9, &header, &mut path1).unwrap();
            assert_ne!(path0, path1, "path id must change the nonce");
        }
    }

    #[test]
    fn sample_len_is_16() {
        let key = QuicHeaderProtectionKey {
            cipher: HeaderCipher::Aes128(Aes128Block::new(&[0; 16])),
        };
        assert_eq!(key.sample_len(), 16);
    }
}
