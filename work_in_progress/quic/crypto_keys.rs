//! QUIC-specific cryptographic operations (RFC 9001).
//!
//! Provides HKDF-Expand-Label, Initial/Handshake/1-RTT key derivation,
//! header protection, and AEAD-based packet protection.

use crypto::{
    Aead, AeadError, StreamCipher,
    aes::{Aes128Gcm, Aes256Gcm, RoundKeys, encrypt_block, encrypt_block_aes128, key_expand, key_expand_128},
    chacha::{ChaCha, ChaCha20Poly1305},
    hmac::Hmac,
    sha2::Sha256,
};
use tls::CipherSuite;

use crate::error::Error;

/// Fixed initial salt from RFC 9001 §5.2.
const INITIAL_SALT: [u8; 20] = [
    0x38, 0x76, 0x2c, 0xf7, 0xf5, 0x59, 0x34, 0xb3, 0x4d, 0x17, 0x9a, 0xe6, 0xa4, 0xc8, 0x0c, 0xad, 0xcc, 0xbb, 0x7f,
    0x0a,
];

/// Keys for one encryption direction (client or server) at one level.
#[derive(Clone)]
pub struct DirectionKeys {
    pub key: Vec<u8>,
    pub iv: [u8; 12],
    pub hp_key: Vec<u8>,
    pub cipher_suite: CipherSuite,
}

impl DirectionKeys {
    fn new(key: Vec<u8>, iv: [u8; 12], hp_key: Vec<u8>, cipher_suite: CipherSuite) -> Self {
        Self {
            key,
            iv,
            hp_key,
            cipher_suite,
        }
    }
}

/// Packet protection keys for both directions at one encryption level.
pub struct LevelKeys {
    pub local: DirectionKeys,
    pub remote: DirectionKeys,
}

// ── HKDF-Expand-Label (RFC 9001 §5.1) ──────────────────────────────────────

/// RFC 9001 §5.1: HKDF-Expand-Label(Secret, Label, Ctx, Length)
pub fn hkdf_expand_label(secret: &[u8], label: &[u8], ctx: &[u8], length: usize) -> Vec<u8> {
    let label_total = 6 + label.len(); // "tls13 " (6 bytes) + actual_label
    let mut hkdf_label = Vec::with_capacity(2 + 1 + label_total + 1 + ctx.len());
    hkdf_label.extend_from_slice(&(length as u16).to_be_bytes());
    hkdf_label.push(label_total as u8);
    hkdf_label.extend_from_slice(b"tls13 ");
    hkdf_label.extend_from_slice(label);
    hkdf_label.push(ctx.len() as u8);
    hkdf_label.extend_from_slice(ctx);

    let hash_len = 32; // SHA-256 output
    let n = (length + hash_len - 1) / hash_len;
    let mut output = Vec::with_capacity(n * hash_len);
    let mut prev: Vec<u8> = Vec::new();

    for i in 1..=n {
        let mut mac = Hmac::<Sha256>::new(secret);
        if !prev.is_empty() {
            mac.update(&prev);
        }
        let mut info = hkdf_label.clone();
        info.push(i as u8);
        mac.update(&info);
        let block = mac.finalize();
        output.extend_from_slice(block.as_ref());
        prev = block.as_ref().to_vec();
    }
    output.truncate(length);
    output
}

// ── Initial key derivation (RFC 9001 §5.2) ─────────────────────────────────

/// Derive the Initial encryption keys from the Destination Connection ID.
pub fn derive_initial_keys(dcid: &[u8]) -> (DirectionKeys, DirectionKeys) {
    let initial_secret = crypto::hkdf::extract::<Sha256>(Some(&INITIAL_SALT), dcid);
    let client_secret = hkdf_expand_label(initial_secret.as_ref(), b"client in", b"", 32);
    let server_secret = hkdf_expand_label(initial_secret.as_ref(), b"server in", b"", 32);
    let client_keys = derive_quic_keys_from_secret(&client_secret, 16, CipherSuite::TlsAes128GcmSha256);
    let server_keys = derive_quic_keys_from_secret(&server_secret, 16, CipherSuite::TlsAes128GcmSha256);
    (client_keys, server_keys)
}

fn derive_quic_keys_from_secret(secret: &[u8], key_len: usize, cipher_suite: CipherSuite) -> DirectionKeys {
    let key = hkdf_expand_label(secret, b"quic key", b"", key_len);
    let iv_bytes = hkdf_expand_label(secret, b"quic iv", b"", 12);
    let hp_key = hkdf_expand_label(secret, b"quic hp", b"", key_len);
    let mut iv = [0u8; 12];
    iv.copy_from_slice(&iv_bytes);
    DirectionKeys::new(key, iv, hp_key, cipher_suite)
}

/// Derive Handshake or 1-RTT keys from a TLS traffic secret.
pub fn derive_level_keys(traffic_secret: &[u8], cipher_suite: CipherSuite) -> DirectionKeys {
    derive_quic_keys_from_secret(traffic_secret, cipher_suite.key_size(), cipher_suite)
}

// ── Header Protection (RFC 9001 §5.4) ──────────────────────────────────────

/// Apply (or remove) header protection. XOR is its own inverse.
fn apply_header_mask(
    hp_key: &[u8],
    cipher_suite: CipherSuite,
    long_header: bool,
    first_byte: &mut u8,
    pn_bytes: &mut [u8],
    sample: &[u8],
) {
    let mask = compute_header_protection_mask(hp_key, cipher_suite, sample);
    if long_header {
        *first_byte ^= mask[0] & 0x0f;
    } else {
        *first_byte ^= mask[0] & 0x1f;
    }
    for (i, b) in pn_bytes.iter_mut().enumerate() {
        *b ^= mask[1 + i];
    }
}

pub fn apply_header_protection(
    keys: &DirectionKeys,
    long_header: bool,
    first_byte: &mut u8,
    pn_bytes: &mut [u8],
    sample: &[u8],
) {
    apply_header_mask(&keys.hp_key, keys.cipher_suite, long_header, first_byte, pn_bytes, sample)
}

pub fn remove_header_protection(
    keys: &DirectionKeys,
    long_header: bool,
    first_byte: &mut u8,
    pn_bytes: &mut [u8],
    sample: &[u8],
) {
    apply_header_mask(&keys.hp_key, keys.cipher_suite, long_header, first_byte, pn_bytes, sample)
}

fn compute_header_protection_mask(hp_key: &[u8], cipher_suite: CipherSuite, sample: &[u8]) -> [u8; 16] {
    match cipher_suite {
        CipherSuite::TlsAes128GcmSha256 => {
            let key: &[u8; 16] = hp_key.try_into().expect("16-byte HP key");
            let mut block = [0u8; 16];
            block.copy_from_slice(&sample[..16]);
            let rk = key_expand_128(key);
            encrypt_block_aes128(&rk, &block)
        }
        CipherSuite::TlsAes256GcmSha384 => {
            let key: &[u8; 32] = hp_key.try_into().expect("32-byte HP key");
            let mut block = [0u8; 16];
            block.copy_from_slice(&sample[..16]);
            let rk = key_expand(key);
            encrypt_block(&rk, &block)
        }
        CipherSuite::TlsChaCha20Poly1305Sha256 => {
            let key: &[u8; 32] = hp_key.try_into().expect("32-byte HP key");
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&sample[4..16]);
            let ctr = u32::from_le_bytes([sample[0], sample[1], sample[2], sample[3]]);
            let mut cipher = ChaCha::<20, true>::new(key, &nonce);
            cipher.set_counter(ctr as u32);
            let mut out = [0u8; 64];
            cipher.xor_keystream(&mut out);
            let mut mask = [0u8; 16];
            mask.copy_from_slice(&out[..16]);
            mask
        }
    }
}

// ── Packet Protection (RFC 9001 §5.3) ──────────────────────────────────────

/// Encrypt a QUIC packet payload.
/// `payload` contains the plaintext; on return it holds ciphertext + 16-byte tag.
pub fn encrypt_payload(
    keys: &DirectionKeys,
    packet_number: u64,
    header: &[u8],
    payload: &mut Vec<u8>,
) -> Result<(), Error> {
    let nonce = compute_nonce(&keys.iv, packet_number);
    match keys.cipher_suite {
        CipherSuite::TlsAes128GcmSha256 => {
            let k: &[u8; 16] = keys.key.as_slice().try_into().unwrap();
            let cipher = Aes128Gcm::new(k);
            let tag = cipher.encrypt_in_place(payload, &nonce, header);
            payload.extend_from_slice(tag.as_ref());
        }
        CipherSuite::TlsAes256GcmSha384 => {
            let k: &[u8; 32] = keys.key.as_slice().try_into().unwrap();
            let cipher = Aes256Gcm::new(k);
            let tag = cipher.encrypt_in_place(payload, &nonce, header);
            payload.extend_from_slice(tag.as_ref());
        }
        CipherSuite::TlsChaCha20Poly1305Sha256 => {
            let k: &[u8; 32] = keys.key.as_slice().try_into().unwrap();
            let cipher = ChaCha20Poly1305::new(k);
            let tag = cipher.encrypt_in_place(payload, &nonce, header);
            payload.extend_from_slice(tag.as_ref());
        }
    }
    Ok(())
}

/// Decrypt a QUIC packet payload.
/// `payload` contains ciphertext + 16-byte tag; on return it holds plaintext.
pub fn decrypt_payload(
    keys: &DirectionKeys,
    packet_number: u64,
    header: &[u8],
    payload: &mut Vec<u8>,
) -> Result<(), Error> {
    if payload.len() < 16 {
        return Err(Error::Crypto(AeadError::InvalidCiphertext));
    }
    let tag = payload.split_off(payload.len() - 16);
    let nonce = compute_nonce(&keys.iv, packet_number);
    match keys.cipher_suite {
        CipherSuite::TlsAes128GcmSha256 => {
            let k: &[u8; 16] = keys.key.as_slice().try_into().unwrap();
            let cipher = Aes128Gcm::new(k);
            cipher
                .decrypt_in_place(payload, &nonce, header, &tag)
                .map_err(|_| Error::Crypto(AeadError::InvalidCiphertext))
        }
        CipherSuite::TlsAes256GcmSha384 => {
            let k: &[u8; 32] = keys.key.as_slice().try_into().unwrap();
            let cipher = Aes256Gcm::new(k);
            cipher
                .decrypt_in_place(payload, &nonce, header, &tag)
                .map_err(|_| Error::Crypto(AeadError::InvalidCiphertext))
        }
        CipherSuite::TlsChaCha20Poly1305Sha256 => {
            let k: &[u8; 32] = keys.key.as_slice().try_into().unwrap();
            let cipher = ChaCha20Poly1305::new(k);
            cipher
                .decrypt_in_place(payload, &nonce, header, &tag)
                .map_err(|_| Error::Crypto(AeadError::InvalidCiphertext))
        }
    }
}

fn compute_nonce(iv: &[u8; 12], packet_number: u64) -> [u8; 12] {
    let mut nonce = *iv;
    let pn_bytes = packet_number.to_be_bytes();
    for i in 0..8 {
        nonce[4 + i] ^= pn_bytes[i];
    }
    nonce
}

// ── Packet number encoding / decoding ─────────────────────────────────────

pub fn pn_encoding_len(pn: u64, largest_acked: u64) -> usize {
    let diff = pn.wrapping_sub(largest_acked);
    if diff < 0x80 {
        1
    } else if diff < 0x8000 {
        2
    } else if diff < 0x800000 {
        3
    } else {
        4
    }
}

pub fn encode_pn(pn: u64, len: usize, buf: &mut Vec<u8>) {
    match len {
        1 => buf.push(pn as u8),
        2 => buf.extend_from_slice(&(pn as u16).to_be_bytes()),
        3 => {
            let b = pn.to_be_bytes();
            buf.extend_from_slice(&b[5..]);
        }
        4 => buf.extend_from_slice(&(pn as u32).to_be_bytes()),
        _ => panic!("invalid PN encoding length"),
    }
}

pub fn decode_pn(truncated: &[u8], largest_received: u64) -> u64 {
    let truncated_val = match truncated.len() {
        1 => truncated[0] as u64,
        2 => u16::from_be_bytes([truncated[0], truncated[1]]) as u64,
        3 => u32::from_be_bytes([0, truncated[0], truncated[1], truncated[2]]) as u64,
        4 => u32::from_be_bytes([truncated[0], truncated[1], truncated[2], truncated[3]]) as u64,
        _ => panic!("invalid truncated PN length"),
    };
    // First received packet: return truncated value directly
    if largest_received == 0 {
        return truncated_val;
    }
    let bits = truncated.len() * 8;
    let window = 1u64 << bits;
    let half_window = window >> 1;
    let pn_mask = window - 1;
    let expected_pn = largest_received + 1;
    let mut decoded = (expected_pn & !pn_mask) | truncated_val;
    // RFC 9000 Appendix A.3: avoid wrapping-underflow when expected_pn < half_window.
    if expected_pn >= half_window && decoded <= expected_pn - half_window {
        decoded = decoded.wrapping_add(window);
    } else if decoded > expected_pn + half_window && decoded > window {
        decoded = decoded.wrapping_sub(window);
    }
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_key_derivation() {
        let dcid = hex::decode("8394c8f03e515708").unwrap();
        let (client, server) = derive_initial_keys(&dcid);
        assert_eq!(client.key.len(), 16);
        assert_eq!(client.iv.len(), 12);
        assert_eq!(client.hp_key.len(), 16);
        assert_eq!(server.key.len(), 16);
    }

    #[test]
    fn test_rfc9001_appendix_a_initial_packet() {
        let dcid = hex::decode("8394c8f03e515708").unwrap();
        let (client, _server) = derive_initial_keys(&dcid);

        assert_eq!(hex::encode(&client.key), "1f369613dd76d5467730efcbe3b1a22d");
        assert_eq!(hex::encode(&client.iv), "fa044b2f42a3fd3b46fb255c");
        assert_eq!(hex::encode(&client.hp_key), "9f50449e04a0e810283a1e9933adedd2");

        // RFC A.2 sample ciphertext used for header protection
        let sample = hex::decode("d1b1c98dd7689fb8ec11d242b123dc9b").unwrap();
        let mut sample_arr = [0u8; 16];
        sample_arr.copy_from_slice(&sample);
        let mask = compute_header_protection_mask(&client.hp_key, client.cipher_suite, &sample_arr);
        // Expected mask from RFC: 437b9aec36 (first 5 bytes used)
        assert_eq!(mask[0], 0x43, "mask[0]");
        assert_eq!(mask[1], 0x7b, "mask[1]");
        assert_eq!(mask[2], 0x9a, "mask[2]");
        assert_eq!(mask[3], 0xec, "mask[3]");
        assert_eq!(mask[4], 0x36, "mask[4]");
    }

    #[test]
    fn test_pn_decode() {
        assert_eq!(decode_pn(&[0x00], 0), 0);
        assert_eq!(decode_pn(&[0x01], 0), 1);
        assert_eq!(decode_pn(&[0x00], 0x7fff), 0x8000);
        assert_eq!(decode_pn(&[0x00], 0xffff), 0x10000);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let dcid = hex::decode("8394c8f03e515708").unwrap();
        let (client, _server) = derive_initial_keys(&dcid);
        let mut payload = b"hello quic".to_vec();
        let header = b"\xc0\x00\x00\x00\x01\x08\x83\x94\xc8\xf0\x3e\x51\x57\x08\x00";
        let pn = 0u64;

        encrypt_payload(&client, pn, header, &mut payload).unwrap();
        assert!(payload.len() > 16);

        let result = decrypt_payload(&client, pn, header, &mut payload);
        assert!(result.is_ok());
        assert_eq!(&payload, b"hello quic");
    }
}
