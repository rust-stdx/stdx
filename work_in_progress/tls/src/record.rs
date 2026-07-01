use alloc::boxed::Box;

use bytes::{Bytes, BytesMut};

use crate::{Error, crypto::Aead, errors::HandshakeFailure, errors::DecodeFailure, errors::InternalFailure};

/// Maximum TLS fragment plaintext length (2^14 bytes). RFC 8446 §5.1.
pub const MAX_FRAGMENT_SIZE: usize = 16384;
/// Maximum TLS record payload (fragment + AEAD tag + inner content type byte).
/// 16384 + 1 (content type) + 256 (padding) + 16 (tag) = 16657.
pub const MAX_RECORD_PAYLOAD: usize = MAX_FRAGMENT_SIZE + 256 + 1 + 16;

/// Overwrite `iv` with zeros on drop (key-material cleanup).
#[cfg(feature = "zeroize")]
fn zeroize_iv(iv: &mut [u8; 12]) {
    use zeroize::Zeroize;
    iv.zeroize();
}

/// Without the `zeroize` feature, IVs are freed without explicit zeroization.
#[cfg(not(feature = "zeroize"))]
fn zeroize_iv(_iv: &mut [u8; 12]) {}

/// TLS record content types (RFC 8446 §5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ContentType {
    Invalid = 0,
    ChangeCipherSpec = 20,
    Alert = 21,
    Handshake = 22,
    ApplicationData = 23,
}

impl ContentType {
    pub fn from_u8(v: u8) -> Self {
        match v {
            20 => Self::ChangeCipherSpec,
            21 => Self::Alert,
            22 => Self::Handshake,
            23 => Self::ApplicationData,
            _ => Self::Invalid,
        }
    }
}

/// Per-direction state for the TLS record layer.
pub struct RecordState {
    seq_num: u64,
    key: Option<Box<dyn Aead>>,
    write_iv: [u8; 12],
    read_iv: [u8; 12],
    read_key: Option<Box<dyn Aead>>,
}

impl Drop for RecordState {
    fn drop(&mut self) {
        zeroize_iv(&mut self.write_iv);
        zeroize_iv(&mut self.read_iv);
    }
}

impl RecordState {
    pub fn new() -> Self {
        Self {
            seq_num: 0,
            key: None,
            write_iv: [0u8; 12],
            read_iv: [0u8; 12],
            read_key: None,
        }
    }

    /// Set the read keys (for decrypting records from the peer).
    /// Resets sequence number to zero per RFC 8446 §5.3.
    pub fn set_read_keys(&mut self, key: Box<dyn Aead>, iv: [u8; 12]) {
        self.read_key = Some(key);
        self.read_iv = iv;
        self.seq_num = 0;
    }

    /// Set the write keys (for encrypting records to the peer).
    /// Resets sequence number to zero per RFC 8446 §5.3.
    pub fn set_write_keys(&mut self, key: Box<dyn Aead>, iv: [u8; 12]) {
        self.key = Some(key);
        self.write_iv = iv;
        self.seq_num = 0;
    }

    /// Build a TLS record frame around `payload`.
    ///
    /// Returns the complete TLS record bytes ready to send.
    pub fn encrypt_record(&mut self, content_type: ContentType, payload: &[u8]) -> Result<Bytes, Error> {
        if payload.len() > MAX_FRAGMENT_SIZE {
            return Err(Error::RecordOverflow);
        }
        let key = self
            .key
            .as_ref()
            .ok_or_else(|| Error::InternalError(InternalFailure::WriteKeyNotSet))?;

        // Inner plaintext: payload || type (TLS 1.3 §5.2)
        let inner_len = 1 + payload.len();
        let tag_sz = key.tag_size();
        let total = 5 + inner_len + tag_sz;
        let mut buf = BytesMut::with_capacity(total);

        // Build the nonce: iv XOR sequence_number
        let mut nonce = self.write_iv;
        let seq_bytes = self.seq_num.to_be_bytes();
        for i in 0..8 {
            nonce[4 + i] ^= seq_bytes[i];
        }

        // Write 5-byte record header: type || legacy_version || length (placeholder)
        buf.extend_from_slice(&[ContentType::ApplicationData as u8, 0x03, 0x03, 0, 0]);
        // Write inner plaintext
        buf.extend_from_slice(payload);
        buf.extend_from_slice(&[content_type as u8]);
        // Fill in length
        let record_len = (inner_len + tag_sz) as u16;
        buf[3..5].copy_from_slice(&record_len.to_be_bytes());

        // AAD = the 5-byte record header (TLS 1.3 §5.2)
        let aad: [u8; 5] = <[u8; 5]>::try_from(&buf[..5]).unwrap();
        let tag = key.encrypt(&mut buf[5..], &nonce, &aad);
        buf.extend_from_slice(&tag);

        self.seq_num += 1;
        Ok(buf.freeze())
    }

    /// Decrypt a received TLS record, consuming its bytes from `buf`.
    ///
    /// Returns the content type and decrypted payload. On success the record
    /// bytes are removed from `buf` via an O(1) split.
    pub fn decrypt_record(&mut self, buf: &mut BytesMut) -> Result<Option<(ContentType, Bytes)>, Error> {
        if buf.len() < 5 {
            return Ok(None);
        }

        let content_type = ContentType::from_u8(buf[0]);
        let _legacy = u16::from_be_bytes([buf[1], buf[2]]);
        let length = u16::from_be_bytes([buf[3], buf[4]]) as usize;
        if length > MAX_RECORD_PAYLOAD {
            return Err(Error::RecordOverflow);
        }
        if buf.len() < 5 + length {
            return Ok(None);
        }

        // Move the entire TLS record out of `buf` (O(1) pointer advance).
        let mut record = buf.split_to(5 + length);

        match content_type {
            ContentType::ChangeCipherSpec => {
                // Ignore single CCS byte (TLS 1.3 middlebox compat)
                Ok(Some((ContentType::ChangeCipherSpec, record.split_off(5).freeze())))
            }
            ContentType::Alert => {
                let frag = &record[5..];
                if frag.len() >= 2 {
                    let level = frag[0];
                    let desc = frag[1];
                    if level == 1 && desc == 0 {
                        return Err(Error::ConnectionClosed);
                    }
                    Err(Error::HandshakeFailed(HandshakeFailure::PeerAlert {
                        level,
                        description: desc,
                    }))
                } else {
                    Err(Error::ConnectionClosed)
                }
            }
            ContentType::Handshake => {
                // Handshake in plaintext (before encryption is set up)
                Ok(Some((ContentType::Handshake, record.split_off(5).freeze())))
            }
            ContentType::ApplicationData => {
                let key = self
                    .read_key
                    .as_ref()
                    .ok_or_else(|| Error::InternalError(InternalFailure::ReadKeyNotSet))?;

                // Build nonce
                let mut nonce = self.read_iv;
                let seq_bytes = self.seq_num.to_be_bytes();
                for i in 0..8 {
                    nonce[4 + i] ^= seq_bytes[i];
                }
                self.seq_num += 1;

                // AAD = the 5-byte record header (TLS 1.3 §5.2)
                let aad: [u8; 5] = record[..5].try_into().unwrap();

                // Strip the 5-byte header; record now holds ciphertext + tag.
                let _header = record.split_to(5);

                // Decrypt in place on the already owned BytesMut.
                let plaintext_len = key.decrypt(&mut record[..], &nonce, &aad)?;
                if plaintext_len == 0 {
                    return Err(Error::DecryptFailed);
                }

                // TLS 1.3 inner plaintext: payload || type || zeros(padding)
                // Find the last non-zero byte in constant time to avoid a
                // timing side-channel on the padding length.
                let mut typ_idx = 0;
                for i in 0..plaintext_len {
                    let cond = (record[i] != 0) as usize;
                    typ_idx = (cond * i) | ((1usize.wrapping_sub(cond)) * typ_idx);
                }
                if record[typ_idx] == 0 {
                    return Err(Error::DecryptFailed);
                }
                let inner_type = ContentType::from_u8(record[typ_idx]);

                record.truncate(typ_idx);
                Ok(Some((inner_type, record.freeze())))
            }
            _ => Err(Error::DecodeError(DecodeFailure::UnknownContentType(buf[0]))),
        }
    }

    /// Encrypt an alert as a TLS record.
    pub fn encrypt_alert(&mut self, level: u8, description: u8) -> Result<Bytes, Error> {
        self.encrypt_record(ContentType::Alert, &[level, description])
    }
}
