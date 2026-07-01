use alloc::{boxed::Box, sync::Arc};

use heapless::Vec;

use crate::{
    Error, MAX_HASH_SIZE,
    crypto::{Aead, CipherSuite, CryptoProvider},
    message::Extension,
};

pub fn transport_parameters_extension(params: &[u8]) -> Extension {
    crate::message::ext_quic_transport_parameters(params)
}

// ── QUIC traffic secrets (output of the TLS handshake) ───────────────────

#[derive(Clone)]
pub struct QuicSecrets {
    pub client_early_traffic_secret: Vec<u8, MAX_HASH_SIZE>,
    pub client_handshake_traffic_secret: Vec<u8, MAX_HASH_SIZE>,
    pub server_handshake_traffic_secret: Vec<u8, MAX_HASH_SIZE>,
    pub client_application_traffic_secret: Vec<u8, MAX_HASH_SIZE>,
    pub server_application_traffic_secret: Vec<u8, MAX_HASH_SIZE>,
    pub exporter_master_secret: Vec<u8, MAX_HASH_SIZE>,
}

pub fn extract_quic_secrets(
    key_schedule: &crate::key_schedule::KeySchedule,
    server_hello_transcript: &[u8],
    server_finished_transcript: &[u8],
) -> QuicSecrets {
    let c_hs = key_schedule.client_handshake_traffic_secret(server_hello_transcript);
    let s_hs = key_schedule.server_handshake_traffic_secret(server_hello_transcript);
    let c_ap = key_schedule.client_application_traffic_secret(server_finished_transcript);
    let s_ap = key_schedule.server_application_traffic_secret(server_finished_transcript);
    let exp = key_schedule.exporter_master_secret(server_finished_transcript);

    let mut client_early = Vec::new();
    client_early.extend_from_slice(key_schedule.early_secret()).unwrap();

    QuicSecrets {
        client_early_traffic_secret: client_early,
        client_handshake_traffic_secret: c_hs,
        server_handshake_traffic_secret: s_hs,
        client_application_traffic_secret: c_ap,
        server_application_traffic_secret: s_ap,
        exporter_master_secret: exp,
    }
}

// ── QUIC Initial salt constants (RFC 9001 §5.2) ──────────────────────────

/// Fixed initial salt from RFC 9001 §5.2 (QUIC v1).
const INITIAL_SALT_V1: [u8; 20] = [
    0x38, 0x76, 0x2c, 0xf7, 0xf5, 0x59, 0x34, 0xb3, 0x4d, 0x17, 0x9a, 0xe6, 0xa4, 0xc8, 0x0c, 0xad, 0xcc, 0xbb, 0x7f,
    0x0a,
];

/// Initial salt for QUIC v2 (RFC 9368 §3).
const INITIAL_SALT_V2: [u8; 20] = [
    0xa7, 0x07, 0xc2, 0x03, 0xa5, 0x9b, 0x47, 0x18, 0x4a, 0x1d, 0x62, 0xca, 0x57, 0x04, 0x06, 0xea, 0x7a, 0xe3, 0xe5,
    0xd3,
];

// ── QuicPacketProtection ──────────────────────────────────────────────────

/// Per-packet AEAD protection + header protection for one direction at one
/// encryption level.
///
/// Created via [`derive_initial_keys`], [`derive_level_keys`], or
/// [`derive_next_keys`].
pub struct QuicPacketProtection {
    aead: Box<dyn Aead>,
    key: Vec<u8, MAX_HASH_SIZE>,
    hp_key: Vec<u8, MAX_HASH_SIZE>,
    suite: CipherSuite,
    iv: [u8; 12],
    provider: Arc<dyn CryptoProvider>,
}

impl QuicPacketProtection {
    /// Build protection from pre-computed key material and an AEAD.
    ///
    /// This is a low-level constructor; prefer [`derive_level_keys`].
    pub fn from_parts(
        aead: Box<dyn Aead>,
        key: Vec<u8, MAX_HASH_SIZE>,
        hp_key: Vec<u8, MAX_HASH_SIZE>,
        suite: CipherSuite,
        iv: [u8; 12],
        provider: Arc<dyn CryptoProvider>,
    ) -> Self {
        Self {
            aead,
            key,
            hp_key,
            suite,
            iv,
            provider,
        }
    }

    /// The cipher suite used for this protection.
    pub fn cipher_suite(&self) -> CipherSuite {
        self.suite
    }

    /// The raw AEAD key bytes.
    pub fn key_bytes(&self) -> &[u8] {
        &self.key
    }

    /// The raw header protection key bytes.
    pub fn hp_key_bytes(&self) -> &[u8] {
        &self.hp_key
    }

    /// The 12-byte IV.
    pub fn iv_bytes(&self) -> &[u8; 12] {
        &self.iv
    }

    /// Compute the QUIC nonce for the given packet number (RFC 9001 §5.3).
    pub fn nonce(&self, packet_number: u64) -> [u8; 12] {
        let mut nonce = self.iv;
        let pn_bytes = packet_number.to_be_bytes();
        for i in 0..8 {
            nonce[4 + i] ^= pn_bytes[i];
        }
        nonce
    }

    /// Encrypt a QUIC packet payload in place and append the 16-byte AEAD tag.
    ///
    /// `payload` contains the plaintext frames; on return it holds
    /// ciphertext + 16-byte tag.
    pub fn encrypt(&self, packet_number: u64, aad: &[u8], payload: &mut alloc::vec::Vec<u8>) -> Result<(), Error> {
        let nonce = self.nonce(packet_number);
        let tag = self.aead.encrypt(payload, &nonce, aad);
        payload.extend_from_slice(&tag);
        Ok(())
    }

    /// Decrypt a QUIC packet payload in place.
    ///
    /// `payload` contains ciphertext + 16-byte tag; on return it holds
    /// the plaintext (the tag is stripped).
    pub fn decrypt(&self, packet_number: u64, aad: &[u8], payload: &mut alloc::vec::Vec<u8>) -> Result<(), Error> {
        if payload.len() < 16 {
            return Err(Error::DecryptFailed);
        }
        let nonce = self.nonce(packet_number);
        let plaintext_len = self
            .aead
            .decrypt(payload, &nonce, aad)
            .map_err(|_| Error::DecryptFailed)?;
        payload.truncate(plaintext_len);
        Ok(())
    }

    /// Compute the 16-byte header protection mask for the given sample.
    pub fn header_protection_mask(&self, sample: &[u8; 16]) -> Result<[u8; 16], Error> {
        self.provider.header_protection_mask(self.suite, &self.hp_key, sample)
    }

    /// Apply header protection to a long header packet.
    ///
    /// Mask bits 0-3 of `first_byte`, all bits of `pn_bytes`.
    pub fn apply_header_protection_long(
        &self,
        first_byte: &mut u8,
        pn_bytes: &mut [u8],
        sample: &[u8; 16],
    ) -> Result<(), Error> {
        let mask = self.header_protection_mask(sample)?;
        *first_byte ^= mask[0] & 0x0f;
        for (i, b) in pn_bytes.iter_mut().enumerate() {
            *b ^= mask[1 + i];
        }
        Ok(())
    }

    /// Apply header protection to a short header packet.
    ///
    /// Mask bits 0-4 of `first_byte`, all bits of `pn_bytes`.
    pub fn apply_header_protection_short(
        &self,
        first_byte: &mut u8,
        pn_bytes: &mut [u8],
        sample: &[u8; 16],
    ) -> Result<(), Error> {
        let mask = self.header_protection_mask(sample)?;
        *first_byte ^= mask[0] & 0x1f;
        for (i, b) in pn_bytes.iter_mut().enumerate() {
            *b ^= mask[1 + i];
        }
        Ok(())
    }

    /// Remove header protection from a long header packet (same XOR, its
    /// own inverse).
    pub fn remove_header_protection_long(
        &self,
        first_byte: &mut u8,
        pn_bytes: &mut [u8],
        sample: &[u8; 16],
    ) -> Result<(), Error> {
        self.apply_header_protection_long(first_byte, pn_bytes, sample)
    }

    /// Remove header protection from a short header packet (same XOR, its
    /// own inverse).
    pub fn remove_header_protection_short(
        &self,
        first_byte: &mut u8,
        pn_bytes: &mut [u8],
        sample: &[u8; 16],
    ) -> Result<(), Error> {
        self.apply_header_protection_short(first_byte, pn_bytes, sample)
    }
}

// ── Key derivation ───────────────────────────────────────────────────────

/// Derive the QUIC Initial packet protection keys from the Destination
/// Connection ID and QUIC version.
///
/// Returns `(client_keys, server_keys)` where the client uses client_keys
/// to send and server_keys to receive Initial packets (RFC 9001 §5.2).
pub fn derive_initial_keys(
    provider: Arc<dyn CryptoProvider>,
    dcid: &[u8],
    version: u32,
) -> Result<(QuicPacketProtection, QuicPacketProtection), Error> {
    let salt: &[u8; 20] = match version {
        0x00000001 => &INITIAL_SALT_V1,
        0x6b3343cf => &INITIAL_SALT_V2,
        _ => &INITIAL_SALT_V1,
    };

    let suite = CipherSuite::TlsAes128GcmSha256;
    let initial_secret = provider.hkdf_extract(suite, salt, dcid);

    let client_secret = provider.hkdf_expand_label(suite, &initial_secret, b"tls13 client in", b"", 32);
    let server_secret = provider.hkdf_expand_label(suite, &initial_secret, b"tls13 server in", b"", 32);

    let client = derive_level_keys_inner(Arc::clone(&provider), suite, &client_secret)?;
    let server = derive_level_keys_inner(Arc::clone(&provider), suite, &server_secret)?;

    Ok((client, server))
}

/// Derive QUIC packet protection for an encryption level (Handshake or
/// 1-RTT) from the TLS traffic secret for that level.
///
/// Uses HKDF-Expand-Label with the quic-specific labels defined in
/// RFC 9001 §5.3.
pub fn derive_level_keys(
    provider: Arc<dyn CryptoProvider>,
    suite: CipherSuite,
    traffic_secret: &[u8],
) -> Result<QuicPacketProtection, Error> {
    derive_level_keys_inner(provider, suite, traffic_secret)
}

/// Derive the next set of 1-RTT keys for a QUIC key update (RFC 9001 §6).
///
/// Returns `(new_traffic_secret, new_protection)`.
pub fn derive_next_keys(
    provider: Arc<dyn CryptoProvider>,
    suite: CipherSuite,
    current_traffic_secret: &[u8],
) -> Result<(Vec<u8, MAX_HASH_SIZE>, QuicPacketProtection), Error> {
    let hash_len = suite.hash_size();
    let new_secret = provider.hkdf_expand_label(suite, current_traffic_secret, b"tls13 quic ku", b"", hash_len);
    let protection = derive_level_keys_inner(Arc::clone(&provider), suite, &new_secret)?;
    Ok((new_secret, protection))
}

// ── Internal helpers ─────────────────────────────────────────────────────

fn derive_level_keys_inner(
    provider: Arc<dyn CryptoProvider>,
    suite: CipherSuite,
    traffic_secret: &[u8],
) -> Result<QuicPacketProtection, Error> {
    let key_size = suite.key_size();
    let key = provider.hkdf_expand_label(suite, traffic_secret, b"tls13 quic key", b"", key_size);
    let iv_bytes = provider.hkdf_expand_label(suite, traffic_secret, b"tls13 quic iv", b"", 12);
    let hp_key = provider.hkdf_expand_label(suite, traffic_secret, b"tls13 quic hp", b"", key_size);

    let aead = provider.create_aead(suite, &key)?;

    let mut iv = [0u8; 12];
    if iv_bytes.len() >= 12 {
        iv.copy_from_slice(&iv_bytes[..12]);
    } else {
        iv[..iv_bytes.len()].copy_from_slice(&iv_bytes);
    }

    Ok(QuicPacketProtection {
        aead,
        key,
        hp_key,
        suite,
        iv,
        provider,
    })
}

/// Packet number encoding length in bytes for a given packet number
/// and the largest acknowledged.
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

/// Encode a packet number in truncated form into `buf`.
pub fn encode_pn(pn: u64, len: usize, buf: &mut alloc::vec::Vec<u8>) {
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

/// Decode a truncated packet number back to the full number.
pub fn decode_pn(truncated: &[u8], largest_received: u64) -> u64 {
    let truncated_val = match truncated.len() {
        1 => truncated[0] as u64,
        2 => u16::from_be_bytes([truncated[0], truncated[1]]) as u64,
        3 => u32::from_be_bytes([0, truncated[0], truncated[1], truncated[2]]) as u64,
        4 => u32::from_be_bytes([truncated[0], truncated[1], truncated[2], truncated[3]]) as u64,
        _ => panic!("invalid truncated PN length"),
    };
    if largest_received == 0 {
        return truncated_val;
    }
    let bits = truncated.len() * 8;
    let window = 1u64 << bits;
    let half_window = window >> 1;
    let pn_mask = window - 1;
    let expected_pn = largest_received + 1;
    let mut decoded = (expected_pn & !pn_mask) | truncated_val;
    if expected_pn >= half_window && decoded <= expected_pn - half_window {
        decoded = decoded.wrapping_add(window);
    } else if decoded > expected_pn + half_window && decoded > window {
        decoded = decoded.wrapping_sub(window);
    }
    decoded
}
