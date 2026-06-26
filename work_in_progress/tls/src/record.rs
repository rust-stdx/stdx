use alloc::{boxed::Box, format, vec::Vec};

use bytes::Bytes;

use crate::{Error, crypto::Aead};

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
        let key = self
            .key
            .as_ref()
            .ok_or_else(|| Error::InternalError("write key not set".into()))?;

        // Inner plaintext: payload || type (TLS 1.3 §5.2)
        let inner_len = 1 + payload.len();
        let tag_sz = key.tag_size();
        let total = 5 + inner_len + tag_sz;
        let mut buf = Vec::with_capacity(total);

        // Build the nonce: iv XOR sequence_number
        let mut nonce = [0u8; 12];
        nonce[..key.nonce_size()].copy_from_slice(&self.write_iv[..key.nonce_size()]);
        let seq_bytes = self.seq_num.to_be_bytes();
        for i in 0..8 {
            nonce[4 + i] ^= seq_bytes[i];
        }

        // Write 5-byte record header: type || legacy_version || length (placeholder)
        buf.extend_from_slice(&[ContentType::ApplicationData as u8, 0x03, 0x03, 0, 0]);
        // Write inner plaintext
        buf.extend_from_slice(payload);
        buf.push(content_type as u8);
        // Fill in length
        let record_len = (inner_len + tag_sz) as u16;
        buf[3..5].copy_from_slice(&record_len.to_be_bytes());

        // AAD = the 5-byte record header (TLS 1.3 §5.2)
        let aad: [u8; 5] = <[u8; 5]>::try_from(&buf[..5]).unwrap();
        let tag = key.encrypt(&mut buf[5..], &nonce, &aad);
        buf.extend_from_slice(&tag);

        self.seq_num += 1;
        Ok(Bytes::from(buf))
    }

    /// Decrypt a received TLS record.
    ///
    /// Returns the content type and decrypted payload.
    pub fn decrypt_record(&mut self, data: &[u8]) -> Result<Option<(ContentType, Bytes)>, Error> {
        if data.len() < 5 {
            return Ok(None); // need more data
        }

        let content_type = ContentType::from_u8(data[0]);
        let _legacy = u16::from_be_bytes([data[1], data[2]]);
        let length = u16::from_be_bytes([data[3], data[4]]) as usize;
        if data.len() < 5 + length {
            return Ok(None); // need more data
        }

        let fragment = &data[5..5 + length];

        match content_type {
            ContentType::ChangeCipherSpec => {
                // Ignore single CCS byte (TLS 1.3 middlebox compat)
                Ok(Some((ContentType::ChangeCipherSpec, Bytes::copy_from_slice(fragment))))
            }
            ContentType::Alert => {
                if fragment.len() >= 2 {
                    let level = fragment[0];
                    let desc = fragment[1];
                    if level == 1 && desc == 0 {
                        return Err(Error::ConnectionClosed);
                    }
                    Err(Error::HandshakeFailed(format!("alert: level={level} desc={desc}")))
                } else {
                    Err(Error::ConnectionClosed)
                }
            }
            ContentType::Handshake => {
                // Handshake in plaintext (before encryption is set up)
                Ok(Some((ContentType::Handshake, Bytes::copy_from_slice(fragment))))
            }
            ContentType::ApplicationData => {
                let key = self
                    .read_key
                    .as_ref()
                    .ok_or_else(|| Error::InternalError("read key not set".into()))?;

                // Build nonce
                let mut nonce = [0u8; 12];
                nonce[..key.nonce_size()].copy_from_slice(&self.read_iv[..key.nonce_size()]);
                let seq_bytes = self.seq_num.to_be_bytes();
                for i in 0..8 {
                    nonce[4 + i] ^= seq_bytes[i];
                }
                self.seq_num += 1;

                // AAD = the 5-byte record header (TLS 1.3 §5.2)
                let aad = &data[..5];

                let mut ciphertext = fragment.to_vec();
                let plaintext_len = key.decrypt(&mut ciphertext, &nonce, aad)?;

                if plaintext_len == 0 {
                    return Err(Error::DecryptFailed);
                }

                // TLS 1.3 inner plaintext: payload || type || zeros(padding)
                // Find the last non-zero byte in constant time to avoid a
                // timing side-channel on the padding length.
                let mut typ_idx = 0;
                for i in 0..plaintext_len {
                    let cond = (ciphertext[i] != 0) as usize;
                    typ_idx = (cond * i) | ((1usize.wrapping_sub(cond)) * typ_idx);
                }
                if ciphertext[typ_idx] == 0 {
                    return Err(Error::DecryptFailed);
                }
                let inner_type = ContentType::from_u8(ciphertext[typ_idx]);
                let payload = Bytes::copy_from_slice(&ciphertext[..typ_idx]);
                Ok(Some((inner_type, payload)))
            }
            _ => Err(Error::DecodeError(format!("unknown content type {}", data[0]))),
        }
    }

    /// Encrypt an alert as a TLS record.
    pub fn encrypt_alert(&mut self, level: u8, description: u8) -> Result<Bytes, Error> {
        self.encrypt_record(ContentType::Alert, &[level, description])
    }
}
