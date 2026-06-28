//! QUIC packet header format (RFC 9000 §17).
//!
//! Two header forms:
//! - **Long header** (Initial, 0-RTT, Handshake, Retry)
//! - **Short header** (1-RTT)

use crate::{cid::ConnectionId, error::Error, varint};

/// QUIC version 1.
pub const QUIC_VERSION_V1: u32 = 0x00000001;

/// Packet types for long headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LongPacketType {
    Initial = 0x00,
    ZeroRtt = 0x10,
    Handshake = 0x20,
    Retry = 0x30,
}

impl LongPacketType {
    fn from_bits(bits: u8) -> Option<Self> {
        Some(match bits & 0x30 {
            0x00 => Self::Initial,
            0x10 => Self::ZeroRtt,
            0x20 => Self::Handshake,
            0x30 => Self::Retry,
            _ => return None,
        })
    }
}

/// Parsed long header fields.
pub struct LongHeader {
    pub ty: LongPacketType,
    pub version: u32,
    pub dcid: ConnectionId,
    pub scid: ConnectionId,
    pub token: Option<Vec<u8>>, // Initial only
    pub payload_length: u64,
    pub packet_number: u64,
    /// Offset where the packet number field starts.
    pub pn_offset: usize,
    /// Raw bytes of the packet number field (up to 4, since length is protected).
    pub pn_raw: Vec<u8>,
}

/// Parsed short header fields.
pub struct ShortHeader<'a> {
    pub dcid: &'a [u8],
    pub packet_number: u64,
    pub key_phase: bool,
    pub pn_len: usize,
    /// Offset in the packet buffer where the encrypted payload begins.
    pub payload_offset: usize,
}

/// Parse a long header packet. Returns the parsed header.
/// The packet number length is not yet known because it is protected by
/// header protection; the caller must remove header protection and then
/// derive the actual length from the unprotected first byte.
pub fn parse_long_header(buf: &[u8]) -> Result<LongHeader, Error> {
    if buf.is_empty() || buf[0] >> 7 != 1 {
        return Err(Error::PacketDecode("not a long header".into()));
    }
    let first = buf[0];
    let ty = LongPacketType::from_bits(first).ok_or_else(|| Error::PacketDecode("invalid long packet type".into()))?;

    let version = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);

    let mut off = 5usize;
    let dcid_len = buf[off] as usize;
    off += 1;
    if dcid_len > 20 || off + dcid_len > buf.len() {
        return Err(Error::PacketDecode("invalid DCID length".into()));
    }
    let dcid = ConnectionId::new(&buf[off..off + dcid_len]);
    off += dcid_len;

    let scid_len = buf[off] as usize;
    off += 1;
    if scid_len > 20 || off + scid_len > buf.len() {
        return Err(Error::PacketDecode("invalid SCID length".into()));
    }
    let scid = ConnectionId::new(&buf[off..off + scid_len]);
    off += scid_len;

    // Token (Initial only)
    let token = if ty == LongPacketType::Initial {
        if off >= buf.len() {
            return Err(Error::PacketDecode("truncated Initial token".into()));
        }
        let (token_len, consumed) =
            varint::decode(&buf[off..]).map_err(|_| Error::PacketDecode("bad token len".into()))?;
        off += consumed;
        if off + token_len as usize > buf.len() {
            return Err(Error::PacketDecode("token extends past packet end".into()));
        }
        let token_data = buf[off..off + token_len as usize].to_vec();
        off += token_len as usize;
        Some(token_data)
    } else {
        None
    };

    // Retry packets have no length field; the rest is Retry Token + 16-byte integrity tag.
    let (payload_len, pn_offset, pn_raw) = if ty == LongPacketType::Retry {
        let remaining = buf.len().saturating_sub(off);
        // For Retry, pn_offset marks the start of the Retry Token.
        // There is no packet number; pn_raw is empty.
        (remaining as u64, off, Vec::new())
    } else {
        // Payload length varint
        let (payload_len, consumed) =
            varint::decode(&buf[off..]).map_err(|_| Error::PacketDecode("bad payload len".into()))?;
        off += consumed;
        // Packet number field starts here (length is protected, assume 4 bytes for sampling)
        let pn_offset = off;
        if buf.len() < pn_offset + 4 {
            return Err(Error::PacketDecode("packet too short for protected packet number".into()));
        }
        let pn_raw = buf[pn_offset..pn_offset + 4].to_vec();
        (payload_len, pn_offset, pn_raw)
    };

    Ok(LongHeader {
        ty,
        version,
        dcid,
        scid,
        token,
        payload_length: payload_len,
        packet_number: 0,
        pn_offset,
        pn_raw,
    })
}

/// Build a long header packet.
pub fn build_long_header(
    ty: LongPacketType,
    version: u32,
    dcid: &ConnectionId,
    scid: &ConnectionId,
    token: Option<&[u8]>,
    payload: &[u8],
    packet_number: u64,
    pn_len: usize,
) -> Vec<u8> {
    let mut buf = Vec::with_capacity(50 + payload.len());
    // First byte: long header flag + type bits + reserved + pn length bits
    let first_byte = 0x80 | (ty as u8) | ((pn_len - 1) as u8);
    buf.push(first_byte);
    buf.extend_from_slice(&version.to_be_bytes());
    buf.push(dcid.len() as u8);
    buf.extend_from_slice(dcid.as_bytes());
    buf.push(scid.len() as u8);
    buf.extend_from_slice(scid.as_bytes());

    if ty == LongPacketType::Initial {
        if let Some(tok) = token {
            varint::encode(tok.len() as u64, &mut buf);
            buf.extend_from_slice(tok);
        } else {
            varint::encode(0, &mut buf);
        }
    }

    // Payload length (everything after the length field)
    let payload_len = pn_len + payload.len() + 16; // +16 for AEAD tag
    varint::encode(payload_len as u64, &mut buf);

    // Packet number
    crate::crypto_keys::encode_pn(packet_number, pn_len, &mut buf);

    // Payload
    buf.extend_from_slice(payload);

    buf
}

/// Parse a short header packet.
pub fn parse_short_header<'a>(buf: &'a [u8], dcid_len: usize) -> Result<(ShortHeader<'a>, &'a [u8]), Error> {
    if buf.is_empty() || buf[0] >> 7 != 0 {
        return Err(Error::PacketDecode("not a short header".into()));
    }
    let first = buf[0];
    let key_phase = (first & 0x04) != 0;
    let pn_len = (first as usize & 0x03) + 1;

    let mut off = 1usize;
    let dcid = &buf[off..off + dcid_len];
    off += dcid_len;

    if off + pn_len > buf.len() {
        return Err(Error::PacketDecode("truncated short header".into()));
    }
    let pn_bytes = &buf[off..off + pn_len];
    let packet_number = crate::crypto_keys::decode_pn(pn_bytes, 0); // placeholder
    let payload_offset = off + pn_len;

    Ok((
        ShortHeader {
            dcid,
            packet_number,
            key_phase,
            pn_len,
            payload_offset,
        },
        pn_bytes,
    ))
}

/// Build a short header packet.
pub fn build_short_header(
    dcid: &ConnectionId,
    key_phase: bool,
    payload: &[u8],
    packet_number: u64,
    pn_len: usize,
) -> Vec<u8> {
    let mut buf = Vec::with_capacity(50 + payload.len());
    let mut first_byte = 0x00u8; // short header
    if key_phase {
        first_byte |= 0x04;
    }
    first_byte |= (pn_len - 1) as u8;
    buf.push(first_byte);
    buf.extend_from_slice(dcid.as_bytes());
    crate::crypto_keys::encode_pn(packet_number, pn_len, &mut buf);
    buf.extend_from_slice(payload);
    buf
}
