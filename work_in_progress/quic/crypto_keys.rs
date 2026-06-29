//! QUIC-specific cryptographic operations (RFC 9001).
//!
//! This module delegates all key derivation and packet protection to the
//! `tls::quic` module, which uses the pluggable [`tls::CryptoProvider`]
//! trait. The `DirectionKeys` type is retained as a lightweight handle that
//! wraps a [`tls::quic::QuicPacketProtection`].
//!
//! # Legacy re-exports
//!
//! The following symbols are re-exported from `tls::quic` so existing QUIC
//! code can migrate incrementally:
//!
//! * [`pn_encoding_len`]
//! * [`encode_pn`]
//! * [`decode_pn`]
//! * [`derive_initial_keys`]
//! * [`derive_level_keys`]
//! * [`derive_next_keys`]

use std::sync::Arc;

use tls::{CipherSuite, quic::QuicPacketProtection};

use crate::error::Error;

// ── DirectionKeys ────────────────────────────────────────────────────────

/// Keys for one encryption direction (client or server) at one level.
///
/// Wraps [`QuicPacketProtection`] so that callers that only need the
/// cipher suite or raw key material can still access it, while the AEAD
/// and header protection operations go through the TLS crypto provider.
pub struct DirectionKeys {
    /// Raw AEAD key bytes.
    pub key: Vec<u8>,
    /// 12-byte IV for packet protection.
    pub iv: [u8; 12],
    /// Header protection key bytes.
    pub hp_key: Vec<u8>,
    /// Cipher suite in use.
    pub cipher_suite: CipherSuite,
    /// The full packet protection object (owns the AEAD, provides
    /// encrypt/decrypt/header-protection methods).
    pub protection: QuicPacketProtection,
}

impl DirectionKeys {
    /// Build `DirectionKeys` from a [`QuicPacketProtection`] plus the raw key
    /// bytes (needed for callers that inspect the raw key).
    pub fn from_parts(
        key: Vec<u8>,
        iv: [u8; 12],
        hp_key: Vec<u8>,
        cipher_suite: CipherSuite,
        protection: QuicPacketProtection,
    ) -> Self {
        Self {
            key,
            iv,
            hp_key,
            cipher_suite,
            protection,
        }
    }
}

/// Packet protection keys for both directions at one encryption level.
pub struct LevelKeys {
    pub local: DirectionKeys,
    pub remote: DirectionKeys,
}

// ── Key derivation (delegates to tls::quic) ───────────────────────────────

/// Derive the Initial encryption keys from the Destination Connection ID (v1).
pub fn derive_initial_keys(dcid: &[u8]) -> (DirectionKeys, DirectionKeys) {
    derive_initial_keys_for_version(dcid, 0x00000001)
}

/// Derive the Initial encryption keys for the given QUIC version.
pub fn derive_initial_keys_for_version(dcid: &[u8], version: u32) -> (DirectionKeys, DirectionKeys) {
    let provider = Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new());
    let (ck, sk) = tls::quic::derive_initial_keys(provider, dcid, version).expect("Initial key derivation failed");
    (dir_keys_from_prot(ck), dir_keys_from_prot(sk))
}

/// Derive Handshake or 1-RTT keys from a TLS traffic secret.
pub fn derive_level_keys(traffic_secret: &[u8], cipher_suite: CipherSuite) -> DirectionKeys {
    let provider = Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new());
    let prot =
        tls::quic::derive_level_keys(provider, cipher_suite, traffic_secret).expect("level key derivation failed");
    dir_keys_from_prot(prot)
}

/// Derive the next set of 1-RTT keys for a key update (RFC 9001 §6).
///
/// Returns `(new_traffic_secret, new_protection_keys)`.
/// The caller must replace the current traffic secret with the new one
/// so that subsequent key updates chain correctly.
pub fn derive_next_keys(traffic_secret: &[u8], cipher_suite: CipherSuite) -> Result<(Vec<u8>, DirectionKeys), Error> {
    let provider = Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new());
    let (new_secret, prot) = tls::quic::derive_next_keys(provider, cipher_suite, traffic_secret)
        .map_err(|_| Error::Crypto(crypto::AeadError::InvalidCiphertext))?;
    let new_secret = new_secret.into_iter().collect::<Vec<u8>>();
    Ok((new_secret, dir_keys_from_prot(prot)))
}

/// Derive 0-RTT keys from a 0-RTT traffic secret (RFC 9001 §6.1).
pub fn derive_0rtt_keys(traffic_secret: &[u8], cipher_suite: CipherSuite) -> DirectionKeys {
    // 0-RTT keys are derived the same way as level keys from the 0-RTT secret.
    derive_level_keys(traffic_secret, cipher_suite)
}

// ── Header protection (delegates to QuicPacketProtection) ─────────────────

pub fn apply_header_protection(
    keys: &DirectionKeys,
    long_header: bool,
    first_byte: &mut u8,
    pn_bytes: &mut [u8],
    sample: &[u8],
) {
    let mut s = [0u8; 16];
    s.copy_from_slice(sample);
    if long_header {
        keys.protection
            .apply_header_protection_long(first_byte, pn_bytes, &s)
            .expect("header protection failed");
    } else {
        keys.protection
            .apply_header_protection_short(first_byte, pn_bytes, &s)
            .expect("header protection failed");
    }
}

pub fn remove_header_protection(
    keys: &DirectionKeys,
    long_header: bool,
    first_byte: &mut u8,
    pn_bytes: &mut [u8],
    sample: &[u8],
) {
    apply_header_protection(keys, long_header, first_byte, pn_bytes, sample);
}

// ── Packet protection (delegates to QuicPacketProtection) ────────────────

/// Encrypt a QUIC packet payload.
pub fn encrypt_payload(
    keys: &DirectionKeys,
    packet_number: u64,
    header: &[u8],
    payload: &mut Vec<u8>,
) -> Result<(), Error> {
    keys.protection
        .encrypt(packet_number, header, payload)
        .map_err(|e| Error::Crypto(crypto::AeadError::InvalidCiphertext))
}

/// Decrypt a QUIC packet payload.
pub fn decrypt_payload(
    keys: &DirectionKeys,
    packet_number: u64,
    header: &[u8],
    payload: &mut Vec<u8>,
) -> Result<(), Error> {
    keys.protection
        .decrypt(packet_number, header, payload)
        .map_err(|_| Error::Crypto(crypto::AeadError::InvalidCiphertext))
}

// ── Packet number encoding / decoding (re-export from tls::quic) ──────────

pub fn pn_encoding_len(pn: u64, largest_acked: u64) -> usize {
    tls::quic::pn_encoding_len(pn, largest_acked)
}

pub fn encode_pn(pn: u64, len: usize, buf: &mut Vec<u8>) {
    tls::quic::encode_pn(pn, len, buf)
}

pub fn decode_pn(truncated: &[u8], largest_received: u64) -> u64 {
    tls::quic::decode_pn(truncated, largest_received)
}

// ── Internal helpers ─────────────────────────────────────────────────────

fn dir_keys_from_prot(prot: QuicPacketProtection) -> DirectionKeys {
    let suite = prot.cipher_suite();
    let key = prot.key_bytes().to_vec();
    let iv = *prot.iv_bytes();
    let hp_key = prot.hp_key_bytes().to_vec();
    DirectionKeys {
        key,
        iv,
        hp_key,
        cipher_suite: suite,
        protection: prot,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────
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

        // Verify header protection mask.
        let sample = hex::decode("d1b1c98dd7689fb8ec11d242b123dc9b").unwrap();
        let mut s = [0u8; 16];
        s.copy_from_slice(&sample);
        let mask = client.protection.header_protection_mask(&s).unwrap();
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
