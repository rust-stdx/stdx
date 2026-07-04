use crate::{CryptoProvider, MAX_RECORD_SIZE, errors::Error};

/// TLS record content types (RFC 8446 §5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ContentType {
    ChangeCipherSpec = 20,
    Alert = 21,
    Handshake = 22,
    ApplicationData = 23,
}

impl ContentType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            20 => Some(Self::ChangeCipherSpec),
            21 => Some(Self::Alert),
            22 => Some(Self::Handshake),
            23 => Some(Self::ApplicationData),
            _ => None,
        }
    }
}

/// A 5-byte TLS record header.
pub struct RecordHeader {
    pub content_type: ContentType,
    pub version: [u8; 2],
    pub length: u16,
}

impl RecordHeader {
    pub const SIZE: usize = 5;

    /// Parse a record header from the given bytes.
    pub fn parse(data: &[u8]) -> Result<Self, Error> {
        let ct = ContentType::from_u8(data[0]).ok_or(Error::DecodeError)?;
        let version = [data[1], data[2]];
        let length = u16::from_be_bytes([data[3], data[4]]);
        Ok(Self {
            content_type: ct,
            version,
            length,
        })
    }

    /// Encode a record header (TLS 1.3 always uses legacy version 0x0303).
    pub fn encode(data: &mut [u8], content_type: ContentType, length: u16) {
        data[0] = content_type as u8;
        data[1] = 0x03;
        data[2] = 0x03;
        data[3..5].copy_from_slice(&length.to_be_bytes());
    }

    /// Build the AAD (additional authenticated data) for AEAD.
    /// In TLS 1.3, AAD = the 5-byte record header.
    pub fn aad(&self) -> [u8; 5] {
        let mut aad = [0u8; 5];
        aad[0] = self.content_type as u8;
        aad[1..3].copy_from_slice(&self.version);
        aad[3..5].copy_from_slice(&self.length.to_be_bytes());
        aad
    }
}

/// Build the nonce: IV XOR padded sequence number (RFC 8446 §5.3).
pub fn build_nonce(iv: &[u8; 12], seq: u64) -> [u8; 12] {
    let mut nonce = *iv;
    let seq_bytes = seq.to_be_bytes();
    for i in 0..8 {
        nonce[4 + i] ^= seq_bytes[i];
    }
    nonce
}

/// Try to read a complete record from `buf[..len]`.
/// Returns the record header and the body slice if a complete record is present.
pub fn try_read_record(buf: &[u8], len: usize) -> Result<Option<(RecordHeader, &[u8])>, Error> {
    if len < RecordHeader::SIZE {
        return Ok(None);
    }
    let header = RecordHeader::parse(buf)?;
    let total = RecordHeader::SIZE + header.length as usize;
    if total > MAX_RECORD_SIZE {
        return Err(Error::RecordOverflow);
    }
    if len < total {
        return Ok(None);
    }
    Ok(Some((header, &buf[RecordHeader::SIZE..total])))
}

/// Encrypt a plaintext payload into `write_buf` as a TLS 1.3 record.
///
/// `write_buf` must have enough space for the full record (5 + plaintext_len + 1 + tag_size).
/// Returns the total record length.
pub fn encrypt_record<P: CryptoProvider>(
    provider: &P,
    key: &P::AeadKey,
    iv: &[u8; 12],
    seq: u64,
    inner_content_type: ContentType,
    plaintext: &[u8],
    write_buf: &mut [u8],
) -> Result<usize, Error> {
    let tag_size = 16;
    let inner_len = plaintext.len() + 1;
    let total = 5 + inner_len + tag_size;

    if total > write_buf.len() {
        return Err(Error::InsufficientBuffer);
    }

    let nonce = build_nonce(iv, seq);
    let record_len = (inner_len + tag_size) as u16;

    RecordHeader::encode(&mut write_buf[..5], ContentType::ApplicationData, record_len);

    write_buf[5..5 + plaintext.len()].copy_from_slice(plaintext);
    write_buf[5 + plaintext.len()] = inner_content_type as u8;

    // Encrypt in place (split to avoid overlapping borrows)
    let (aad, data) = write_buf[..total].split_at_mut(5);
    let data_len = provider.aead_encrypt(key, &nonce, aad, data, inner_len)?;

    Ok(5 + data_len)
}

/// Decrypt a TLS 1.3 record body.
///
/// `body` is the record payload (ciphertext + tag).
/// Returns the inner content type and the decrypted payload slice.
pub fn decrypt_record<'a, P: CryptoProvider>(
    provider: &P,
    key: &P::AeadKey,
    iv: &[u8; 12],
    seq: u64,
    header: &RecordHeader,
    body: &'a mut [u8],
) -> Result<(ContentType, &'a mut [u8]), Error> {
    let nonce = build_nonce(iv, seq);
    let aad = header.aad();
    let plaintext_len = provider.aead_decrypt(key, &nonce, &aad, body)?;

    if plaintext_len == 0 {
        return Err(Error::DecodeError);
    }

    // TLS 1.3 inner plaintext: payload || type || zeros(padding)
    // Find the last non-zero byte (constant-time to avoid timing side-channel on padding length)
    let mut typ_idx = 0;
    for i in 0..plaintext_len {
        let cond = (body[i] != 0) as usize;
        typ_idx = (cond * i) | ((1usize.wrapping_sub(cond)) * typ_idx);
    }
    if body[typ_idx] == 0 {
        return Err(Error::DecodeError);
    }
    let inner_type = ContentType::from_u8(body[typ_idx]).ok_or(Error::DecodeError)?;

    let payload_len = typ_idx;
    let (payload, _) = body.split_at_mut(payload_len);

    Ok((inner_type, payload))
}
