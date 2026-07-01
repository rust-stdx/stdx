#![allow(dead_code)]
use alloc::{format, vec, vec::Vec};

use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    Error,
    crypto::{
        CertType, CipherSuite, KeyExchangeGroup, MAX_HASH_OUTPUT, MAX_KX_PUBLIC_KEY, MAX_SESSION_ID,
        MAX_SIGNATURE_SIZE, SignatureScheme,
    },
};

// ── Handshake types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HandshakeType {
    ClientHello = 1,
    ServerHello = 2,
    NewSessionTicket = 4,
    EndOfEarlyData = 5,
    EncryptedExtensions = 8,
    Certificate = 11,
    CertificateRequest = 13,
    CertificateVerify = 15,
    Finished = 20,
    KeyUpdate = 24,
    MessageHash = 254,
}

impl HandshakeType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::ClientHello),
            2 => Some(Self::ServerHello),
            4 => Some(Self::NewSessionTicket),
            5 => Some(Self::EndOfEarlyData),
            8 => Some(Self::EncryptedExtensions),
            11 => Some(Self::Certificate),
            13 => Some(Self::CertificateRequest),
            15 => Some(Self::CertificateVerify),
            20 => Some(Self::Finished),
            24 => Some(Self::KeyUpdate),
            254 => Some(Self::MessageHash),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ClientHello => "ClientHello",
            Self::ServerHello => "ServerHello",
            Self::NewSessionTicket => "NewSessionTicket",
            Self::EndOfEarlyData => "EndOfEarlyData",
            Self::EncryptedExtensions => "EncryptedExtensions",
            Self::Certificate => "Certificate",
            Self::CertificateRequest => "CertificateRequest",
            Self::CertificateVerify => "CertificateVerify",
            Self::Finished => "Finished",
            Self::KeyUpdate => "KeyUpdate",
            Self::MessageHash => "MessageHash",
        }
    }
}

// ── Wire format helpers ───────────────────────────────────────────────────

fn put_u16(buf: &mut BytesMut, v: u16) {
    buf.extend_from_slice(&v.to_be_bytes());
}

fn put_u24(buf: &mut BytesMut, v: u32) {
    buf.extend_from_slice(&v.to_be_bytes()[1..]);
}

fn put_u8_slice(buf: &mut BytesMut, data: &[u8]) {
    buf.put_u8(data.len() as u8);
    buf.extend_from_slice(data);
}

fn put_u16_slice(buf: &mut BytesMut, data: &[u8]) {
    put_u16(buf, data.len() as u16);
    buf.extend_from_slice(data);
}

fn put_u24_slice(buf: &mut BytesMut, data: &[u8]) {
    put_u24(buf, data.len() as u32);
    buf.extend_from_slice(data);
}

/// Handshake message header.
///
/// ```ignore
/// struct {
///     HandshakeType msg_type;
///     uint24 length;
///     select (Handshake.msg_type) { ... } body;
/// } Handshake;
/// ```
pub fn encode_handshake(msg_type: HandshakeType, body: &[u8]) -> Bytes {
    let mut buf = BytesMut::with_capacity(4 + body.len());
    buf.put_u8(msg_type as u8);
    put_u24(&mut buf, body.len() as u32);
    buf.extend_from_slice(body);
    buf.freeze()
}

/// Parse a handshake message header, returning the type and body.
pub fn decode_handshake_header(data: &[u8]) -> Result<(HandshakeType, &[u8]), Error> {
    if data.len() < 4 {
        return Err(Error::DecodeError("handshake message too short".into()));
    }
    let msg_type = HandshakeType::from_u8(data[0])
        .ok_or_else(|| Error::DecodeError(format!("unknown handshake type {}", data[0]).into()))?;
    let length = u32::from_be_bytes([0, data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + length {
        return Err(Error::DecodeError("handshake message truncated".into()));
    }
    Ok((msg_type, &data[4..4 + length]))
}

// ── Extensions ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ExtensionType {
    ServerName = 0,
    SupportedGroups = 10,
    SignatureAlgorithms = 13,
    ApplicationLayerProtocolNegotiation = 16,
    EarlyData = 42,
    SupportedVersions = 43,
    PreSharedKey = 41,
    PskKeyExchangeModes = 45,
    KeyShare = 51,
    ServerCertificateType = 50, // RFC 7250 / RFC 9633
    QuicTransportParameters = 0x0039,
}

impl ExtensionType {
    pub fn from_u16(v: u16) -> Option<Self> {
        match v {
            0 => Some(Self::ServerName),
            10 => Some(Self::SupportedGroups),
            13 => Some(Self::SignatureAlgorithms),
            16 => Some(Self::ApplicationLayerProtocolNegotiation),
            41 => Some(Self::PreSharedKey),
            42 => Some(Self::EarlyData),
            43 => Some(Self::SupportedVersions),
            45 => Some(Self::PskKeyExchangeModes),
            50 => Some(Self::ServerCertificateType),
            51 => Some(Self::KeyShare),
            0x0039 => Some(Self::QuicTransportParameters),
            _ => None,
        }
    }
}

/// A single extension: type + opaque data.
#[derive(Debug, Clone)]
pub struct Extension {
    pub ext_type: ExtensionType,
    pub data: Bytes,
}

impl Extension {
    pub fn new(ext_type: ExtensionType, data: Bytes) -> Self {
        Self {
            ext_type,
            data,
        }
    }
}

pub fn encode_extensions(exts: &[Extension]) -> Bytes {
    let total_size: usize = exts.iter().map(|e| 4 + e.data.len()).sum();
    let mut body = BytesMut::with_capacity(total_size);
    for ext in exts {
        put_u16(&mut body, ext.ext_type as u16);
        put_u16_slice(&mut body, &ext.data);
    }
    let mut buf = BytesMut::with_capacity(2 + body.len());
    put_u16(&mut buf, body.len() as u16);
    buf.extend_from_slice(&body);
    buf.freeze()
}

pub fn decode_extensions(mut data: &[u8]) -> Result<Vec<Extension>, Error> {
    if data.len() < 2 {
        return Err(Error::DecodeError("extensions too short".into()));
    }
    let total = u16::from_be_bytes([data[0], data[1]]) as usize;
    data = &data[2..];
    if data.len() < total {
        return Err(Error::DecodeError("extensions truncated".into()));
    }
    data = &data[..total];

    let mut exts = Vec::with_capacity(total / 4);
    while data.len() >= 4 {
        let ext_type = ExtensionType::from_u16(u16::from_be_bytes([data[0], data[1]]));
        let len = u16::from_be_bytes([data[2], data[3]]) as usize;
        if data.len() < 4 + len {
            return Err(Error::DecodeError("extension truncated".into()));
        }
        if let Some(ext_type) = ext_type {
            exts.push(Extension {
                ext_type,
                data: Bytes::copy_from_slice(&data[4..4 + len]),
            });
        }
        data = &data[4 + len..];
    }
    Ok(exts)
}

// ── ClientHello ───────────────────────────────────────────────────────────

/// ```ignore
/// struct {
///     ProtocolVersion legacy_version = 0x0303;
///     Random random;
///     opaque legacy_session_id<0..32>;
///     CipherSuite cipher_suites<2..2^16-2>;
///     opaque legacy_compression_methods<1..2^8-1>;
///     Extension extensions<8..2^16-1>;
/// } ClientHello;
/// ```
pub fn encode_client_hello(
    random: &[u8; 32],
    session_id: &[u8],
    cipher_suites: &[CipherSuite],
    extensions: &[Extension],
) -> Bytes {
    let cs_size: usize = cipher_suites.len() * 2;
    let ext_size: usize = extensions.iter().map(|e| 4 + e.data.len()).sum();
    let mut body = BytesMut::with_capacity(2 + 32 + 1 + session_id.len() + 2 + cs_size + 2 + ext_size);

    // legacy_version = 0x0303
    put_u16(&mut body, 0x0303);
    // random
    body.extend_from_slice(random);
    // session_id
    put_u8_slice(&mut body, session_id);
    // cipher_suites
    let mut cs_buf = BytesMut::new();
    for cs in cipher_suites {
        cs_buf.extend_from_slice(&cs.to_wire());
    }
    put_u16_slice(&mut body, &cs_buf);
    // legacy_compression_methods (null only)
    body.extend_from_slice(&[1, 0]);
    // extensions
    let ext_bytes = encode_extensions(extensions);
    body.extend_from_slice(&ext_bytes);

    encode_handshake(HandshakeType::ClientHello, &body)
}

#[derive(Debug, Clone)]
pub struct ClientHelloMsg {
    pub random: [u8; 32],
    pub session_id: heapless::Vec<u8, MAX_SESSION_ID>,
    pub cipher_suites: Vec<CipherSuite>,
    pub extensions: Vec<Extension>,
    pub raw: Bytes,
}

impl ClientHelloMsg {
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (msg_type, body) = decode_handshake_header(data)?;
        if msg_type != HandshakeType::ClientHello {
            return Err(Error::UnexpectedMessage {
                expected: "ClientHello",
                got: msg_type.name(),
            });
        }
        if body.len() < 38 {
            return Err(Error::DecodeError("ClientHello too short".into()));
        }

        let legacy_version = u16::from_be_bytes([body[0], body[1]]);
        let _ = legacy_version; // 0x0303
        let mut random = [0u8; 32];
        random.copy_from_slice(&body[2..34]);
        let session_id_len = body[34] as usize;
        if body.len() < 35 + session_id_len {
            return Err(Error::DecodeError("ClientHello session_id truncated".into()));
        }
        let mut session_id = heapless::Vec::<u8, MAX_SESSION_ID>::new();
        session_id
            .extend_from_slice(&body[35..35 + session_id_len])
            .map_err(|_| Error::DecodeError("session_id too long".into()))?;
        let mut off = 35 + session_id_len;

        let cs_len = u16::from_be_bytes([body[off], body[off + 1]]) as usize;
        off += 2;
        if body.len() < off + cs_len || cs_len % 2 != 0 {
            return Err(Error::DecodeError("ClientHello cipher_suites malformed".into()));
        }
        let mut cipher_suites = Vec::with_capacity(cs_len / 2);
        for i in (0..cs_len).step_by(2) {
            let b: [u8; 2] = [body[off + i], body[off + i + 1]];
            if let Some(cs) = CipherSuite::from_wire(b) {
                cipher_suites.push(cs);
            }
        }
        off += cs_len;

        // compression methods
        let comp_len = body[off] as usize;
        off += 1 + comp_len;

        let extensions = decode_extensions(&body[off..])?;

        Ok(ClientHelloMsg {
            random,
            session_id,
            cipher_suites,
            extensions,
            raw: Bytes::copy_from_slice(data),
        })
    }
}

// ── ServerHello ───────────────────────────────────────────────────────────

/// ```ignore
/// struct {
///     ProtocolVersion legacy_version = 0x0303;
///     Random random;
///     opaque legacy_session_id_echo<0..32>;
///     CipherSuite cipher_suite;
///     uint8 legacy_compression_method = 0;
///     Extension extensions<6..2^16-1>;
/// } ServerHello;
/// ```
pub fn encode_server_hello(
    random: &[u8; 32],
    session_id: &[u8],
    cipher_suite: CipherSuite,
    extensions: &[Extension],
) -> Bytes {
    let ext_size: usize = extensions.iter().map(|e| 4 + e.data.len()).sum();
    let mut body = BytesMut::with_capacity(2 + 32 + 1 + session_id.len() + 2 + 1 + ext_size);
    put_u16(&mut body, 0x0303);
    body.extend_from_slice(random);
    put_u8_slice(&mut body, session_id);
    body.extend_from_slice(&cipher_suite.to_wire());
    body.put_u8(0); // compression
    let ext_bytes = encode_extensions(extensions);
    body.extend_from_slice(&ext_bytes);

    encode_handshake(HandshakeType::ServerHello, &body)
}

#[derive(Debug, Clone)]
pub struct ServerHello {
    pub random: [u8; 32],
    pub session_id: heapless::Vec<u8, MAX_SESSION_ID>,
    pub cipher_suite: CipherSuite,
    pub extensions: Vec<Extension>,
    pub raw: Bytes,
}

impl ServerHello {
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (msg_type, body) = decode_handshake_header(data)?;
        if msg_type != HandshakeType::ServerHello {
            return Err(Error::UnexpectedMessage {
                expected: "ServerHello",
                got: msg_type.name(),
            });
        }
        if body.len() < 40 {
            return Err(Error::DecodeError("ServerHello too short".into()));
        }

        let _legacy = u16::from_be_bytes([body[0], body[1]]);
        let mut random = [0u8; 32];
        random.copy_from_slice(&body[2..34]);
        let sid_len = body[34] as usize;
        if body.len() < 35 + sid_len {
            return Err(Error::DecodeError("ServerHello session_id truncated".into()));
        }
        let mut session_id = heapless::Vec::<u8, MAX_SESSION_ID>::new();
        session_id
            .extend_from_slice(&body[35..35 + sid_len])
            .map_err(|_| Error::DecodeError("session_id too long".into()))?;
        let off = 35 + sid_len;

        let cs = CipherSuite::from_wire([body[off], body[off + 1]])
            .ok_or_else(|| Error::DecodeError("unknown cipher suite in ServerHello".into()))?;
        let comp = body[off + 2];
        if comp != 0 {
            return Err(Error::DecodeError("non-null compression in ServerHello".into()));
        }

        let extensions = decode_extensions(&body[off + 3..])?;

        Ok(ServerHello {
            random,
            session_id,
            cipher_suite: cs,
            extensions,
            raw: Bytes::copy_from_slice(data),
        })
    }
}

// ── EncryptedExtensions ───────────────────────────────────────────────────

pub fn encode_encrypted_extensions(extensions: &[Extension]) -> Bytes {
    let ext_bytes = encode_extensions(extensions);
    encode_handshake(HandshakeType::EncryptedExtensions, &ext_bytes)
}

#[derive(Debug, Clone)]
pub struct EncryptedExtensions {
    pub extensions: Vec<Extension>,
    pub raw: Bytes,
}

impl EncryptedExtensions {
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (msg_type, body) = decode_handshake_header(data)?;
        if msg_type != HandshakeType::EncryptedExtensions {
            return Err(Error::UnexpectedMessage {
                expected: "EncryptedExtensions",
                got: msg_type.name(),
            });
        }
        let extensions = decode_extensions(body)?;
        Ok(EncryptedExtensions {
            extensions,
            raw: Bytes::copy_from_slice(data),
        })
    }
}

// ── Certificate (X.509 chain or raw public key) ──────────────────────────

/// One entry in the `certificate_list` of a TLS 1.3 Certificate message.
///
/// In X.509 mode the `cert_data` is a DER-encoded X.509 certificate.
/// In raw public key mode (RFC 7250) it is a DER-encoded SubjectPublicKeyInfo.
#[derive(Debug, Clone)]
pub struct CertificateEntry {
    pub cert_data: Vec<u8>,
    pub extensions: Vec<Extension>,
}

/// RFC 7250 / RFC 8446 Certificate message.
///
/// ```ignore
/// struct {
///     opaque certificate_request_context<0..2^8-1>;
///     CertificateEntry certificate_list<0..2^24-1>;
/// } Certificate;
///
/// struct {
///     opaque cert_data<1..2^24-1>;
///     Extension extensions<0..2^16-1>;
/// } CertificateEntry;
/// ```
pub fn encode_certificate_raw_public_key(context: &[u8], public_key: &[u8], extensions: &[Extension]) -> Bytes {
    let mut entry = BytesMut::new();
    put_u24_slice(&mut entry, public_key);
    let entry_exts = encode_extensions(extensions);
    entry.extend_from_slice(&entry_exts);

    let mut body = BytesMut::new();
    put_u8_slice(&mut body, context);
    put_u24_slice(&mut body, &entry);

    encode_handshake(HandshakeType::Certificate, &body)
}

/// Encode a Certificate message with an X.509 certificate chain.
///
/// `chain` is ordered end-entity first, followed by intermediates.
pub fn encode_certificate_chain(context: &[u8], chain: &[Vec<u8>], extensions_per_entry: &[Vec<Extension>]) -> Bytes {
    let mut entries = BytesMut::new();
    for (i, cert_der) in chain.iter().enumerate() {
        put_u24_slice(&mut entries, cert_der);
        let exts = extensions_per_entry.get(i).map(|v| v.as_slice()).unwrap_or(&[]);
        let ext_bytes = encode_extensions(exts);
        entries.extend_from_slice(&ext_bytes);
    }

    let mut body = BytesMut::new();
    put_u8_slice(&mut body, context);
    put_u24_slice(&mut body, &entries);

    encode_handshake(HandshakeType::Certificate, &body)
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub context: heapless::Vec<u8, 255>,
    pub entries: Vec<CertificateEntry>,
    pub raw: Bytes,
}

impl Certificate {
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (msg_type, body) = decode_handshake_header(data)?;
        if msg_type != HandshakeType::Certificate {
            return Err(Error::UnexpectedMessage {
                expected: "Certificate",
                got: msg_type.name(),
            });
        }
        if body.is_empty() {
            return Err(Error::DecodeError("empty Certificate".into()));
        }
        let ctx_len = body[0] as usize;
        if 1 + ctx_len > body.len() {
            return Err(Error::DecodeError("certificate context truncated".into()));
        }
        let mut context = heapless::Vec::<u8, 255>::new();
        context.extend_from_slice(&body[1..1 + ctx_len]).ok();
        let mut off = 1 + ctx_len;
        if body.len() < off + 3 {
            return Err(Error::DecodeError("certificate list length truncated".into()));
        }
        let list_len = u32::from_be_bytes([0, body[off], body[off + 1], body[off + 2]]) as usize;
        off += 3;
        let list_end = off + list_len;
        if body.len() < list_end {
            return Err(Error::DecodeError("certificate list truncated".into()));
        }

        let mut entries = Vec::with_capacity(list_len / 6);
        while off < list_end {
            if body.len() < off + 3 {
                return Err(Error::DecodeError("certificate entry datalen truncated".into()));
            }
            let cert_data_len = u32::from_be_bytes([0, body[off], body[off + 1], body[off + 2]]) as usize;
            off += 3;
            if off + cert_data_len > list_end {
                return Err(Error::DecodeError("certificate entry data truncated".into()));
            }
            let cert_data = body[off..off + cert_data_len].to_vec();
            off += cert_data_len;

            let remaining = list_end.saturating_sub(off);
            let exts = if remaining >= 2 {
                let exts_len = u16::from_be_bytes([body[off], body[off + 1]]) as usize;
                // Limit to remaining bytes
                let el = exts_len.min(remaining - 2);
                let ext_bytes = &body[off..off + 2 + el];
                off += 2 + el;
                decode_extensions(ext_bytes).unwrap_or_default()
            } else {
                Vec::new()
            };

            entries.push(CertificateEntry {
                cert_data,
                extensions: exts,
            });
        }

        Ok(Certificate {
            context,
            entries,
            raw: Bytes::copy_from_slice(data),
        })
    }
}

// ── CertificateRequest ─────────────────────────────────────────────────────

/// Encode a `CertificateRequest` message (RFC 8446 §4.3.2).
///
/// The `context` should be empty for server-initiated requests, and
/// `sig_schemes` is the list of acceptable signature schemes (the
/// `signature_algorithms` extension is required).
pub fn encode_certificate_request(context: &[u8], sig_schemes: &[SignatureScheme]) -> Bytes {
    let exts = vec![ext_signature_algorithms(sig_schemes)];
    let ext_bytes = encode_extensions(&exts);
    let mut body = BytesMut::new();
    put_u8_slice(&mut body, context);
    body.extend_from_slice(&ext_bytes);
    encode_handshake(HandshakeType::CertificateRequest, &body)
}

// ── CertificateVerify ─────────────────────────────────────────────────────

pub fn encode_certificate_verify(scheme: SignatureScheme, signature: &[u8]) -> Bytes {
    let mut body = BytesMut::new();
    body.extend_from_slice(&scheme.to_wire());
    put_u16_slice(&mut body, signature);
    encode_handshake(HandshakeType::CertificateVerify, &body)
}

#[derive(Debug, Clone)]
pub struct CertificateVerify {
    pub scheme: SignatureScheme,
    pub signature: heapless::Vec<u8, MAX_SIGNATURE_SIZE>,
    pub raw: Bytes,
}

impl CertificateVerify {
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (msg_type, body) = decode_handshake_header(data)?;
        if msg_type != HandshakeType::CertificateVerify {
            return Err(Error::UnexpectedMessage {
                expected: "CertificateVerify",
                got: msg_type.name(),
            });
        }
        if body.len() < 2 {
            return Err(Error::DecodeError("CertificateVerify too short".into()));
        }
        let scheme = SignatureScheme::from_wire([body[0], body[1]])
            .ok_or_else(|| Error::DecodeError("unknown signature scheme in CertificateVerify".into()))?;
        let sig_len = u16::from_be_bytes([body[2], body[3]]) as usize;
        if body.len() < 4 + sig_len {
            return Err(Error::DecodeError("CertificateVerify signature truncated".into()));
        }
        let mut signature = heapless::Vec::<u8, MAX_SIGNATURE_SIZE>::new();
        signature
            .extend_from_slice(&body[4..4 + sig_len])
            .map_err(|_| Error::DecodeError("signature too long".into()))?;
        Ok(CertificateVerify {
            scheme,
            signature,
            raw: Bytes::copy_from_slice(data),
        })
    }
}

// ── Finished ──────────────────────────────────────────────────────────────

pub fn encode_finished(verify_data: &[u8]) -> Bytes {
    encode_handshake(HandshakeType::Finished, verify_data)
}

#[derive(Debug, Clone)]
pub struct Finished {
    pub verify_data: heapless::Vec<u8, MAX_HASH_OUTPUT>,
    pub raw: Bytes,
}

impl Finished {
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (msg_type, body) = decode_handshake_header(data)?;
        if msg_type != HandshakeType::Finished {
            return Err(Error::UnexpectedMessage {
                expected: "Finished",
                got: msg_type.name(),
            });
        }
        let mut verify_data = heapless::Vec::<u8, MAX_HASH_OUTPUT>::new();
        verify_data
            .extend_from_slice(body)
            .map_err(|_| Error::DecodeError("verify_data too long".into()))?;
        Ok(Finished {
            verify_data,
            raw: Bytes::copy_from_slice(data),
        })
    }
}

// ── KeyUpdate ──────────────────────────────────────────────────────────────

/// Encode a `KeyUpdate` post-handshake message (RFC 8446 §4.6.3).
///
/// `request_update` is `0` for `update_not_requested` and `1` for
/// `update_requested`.
pub fn encode_key_update(request_update: u8) -> Bytes {
    encode_handshake(HandshakeType::KeyUpdate, &[request_update])
}

/// Decode a `KeyUpdate` post-handshake message.
///
/// Returns `request_update`: `0` = update_not_requested, `1` = update_requested.
pub fn decode_key_update(data: &[u8]) -> Result<u8, Error> {
    let (msg_type, body) = decode_handshake_header(data)?;
    if msg_type != HandshakeType::KeyUpdate {
        return Err(Error::UnexpectedMessage {
            expected: "KeyUpdate",
            got: msg_type.name(),
        });
    }
    if body.len() != 1 || body[0] > 1 {
        return Err(Error::DecodeError("KeyUpdate: invalid request_update".into()));
    }
    Ok(body[0])
}

// ── NewSessionTicket ──────────────────────────────────────────────────────

pub fn encode_new_session_ticket(
    ticket_lifetime: u32,
    ticket_age_add: u32,
    ticket_nonce: &[u8],
    ticket: &[u8],
    extensions: &[Extension],
) -> Bytes {
    let mut body = BytesMut::new();
    body.extend_from_slice(&ticket_lifetime.to_be_bytes());
    body.extend_from_slice(&ticket_age_add.to_be_bytes());
    put_u8_slice(&mut body, ticket_nonce);
    put_u16_slice(&mut body, ticket);
    let ext_bytes = encode_extensions(extensions);
    body.extend_from_slice(&ext_bytes);
    encode_handshake(HandshakeType::NewSessionTicket, &body)
}

#[derive(Debug, Clone)]
pub struct NewSessionTicket {
    pub ticket_lifetime: u32,
    pub ticket_age_add: u32,
    pub ticket_nonce: Vec<u8>,
    pub ticket: Vec<u8>,
    pub extensions: Vec<Extension>,
    pub raw: Bytes,
}

impl NewSessionTicket {
    #[allow(dead_code)]
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let (msg_type, body) = decode_handshake_header(data)?;
        if msg_type != HandshakeType::NewSessionTicket {
            return Err(Error::UnexpectedMessage {
                expected: "NewSessionTicket",
                got: msg_type.name(),
            });
        }
        if body.len() < 10 {
            return Err(Error::DecodeError("NewSessionTicket too short".into()));
        }
        let ticket_lifetime = u32::from_be_bytes([body[0], body[1], body[2], body[3]]);
        let ticket_age_add = u32::from_be_bytes([body[4], body[5], body[6], body[7]]);
        let nonce_len = body[8] as usize;
        let ticket_nonce = body[9..9 + nonce_len].to_vec();
        let mut off = 9 + nonce_len;
        let ticket_len = u16::from_be_bytes([body[off], body[off + 1]]) as usize;
        off += 2;
        let ticket = body[off..off + ticket_len].to_vec();
        off += ticket_len;
        let extensions = decode_extensions(&body[off..]).unwrap_or_default();
        Ok(NewSessionTicket {
            ticket_lifetime,
            ticket_age_add,
            ticket_nonce,
            ticket,
            extensions,
            raw: Bytes::copy_from_slice(data),
        })
    }
}

// ── Extension builders ────────────────────────────────────────────────────

/// Build a `supported_versions` extension (TLS 1.3 only).
pub fn ext_supported_versions() -> Extension {
    let data = Bytes::from_static(&[2, 3, 4]); // length + 0x0304
    Extension {
        ext_type: ExtensionType::SupportedVersions,
        data,
    }
}

/// Build a `supported_versions` extension for ServerHello (TLS 1.3 only).
pub fn ext_supported_versions_server() -> Extension {
    let data = Bytes::from_static(&[2, 3, 4]); // 0x0304 only
    Extension {
        ext_type: ExtensionType::SupportedVersions,
        data,
    }
}

/// Build a `supported_groups` extension.
pub fn ext_supported_groups(groups: &[KeyExchangeGroup]) -> Extension {
    let mut body = BytesMut::with_capacity(groups.len() * 2);
    for g in groups {
        body.extend_from_slice(&g.to_wire());
    }
    let mut data = BytesMut::with_capacity(2 + body.len());
    put_u16(&mut data, body.len() as u16);
    data.extend_from_slice(&body);
    Extension {
        ext_type: ExtensionType::SupportedGroups,
        data: data.freeze(),
    }
}

/// Build a `key_share` extension for the client hello (offer).
///
/// Each entry is a `(KeyExchangeGroup, public_key_bytes)` pair. The extension data
/// contains the list length prefix followed by all entries.
pub fn ext_key_share_client(entries: &[(KeyExchangeGroup, &[u8])]) -> Extension {
    let entries_size: usize = entries.iter().map(|(_, pk)| 2 + 2 + pk.len()).sum();
    let mut all_entries = BytesMut::with_capacity(entries_size);
    for (group, public_key) in entries {
        all_entries.extend_from_slice(&group.to_wire());
        put_u16(&mut all_entries, public_key.len() as u16);
        all_entries.extend_from_slice(public_key);
    }
    let mut data = BytesMut::new();
    put_u16(&mut data, all_entries.len() as u16);
    data.extend_from_slice(&all_entries);
    Extension {
        ext_type: ExtensionType::KeyShare,
        data: data.freeze(),
    }
}

/// Build a `key_share` extension for the server hello (response).
///
/// ServerHello key_share has no total length prefix — just the single entry.
pub fn ext_key_share_server(public_key: &[u8], group: KeyExchangeGroup) -> Extension {
    let mut entry = BytesMut::with_capacity(2 + 2 + public_key.len());
    entry.extend_from_slice(&group.to_wire());
    put_u16(&mut entry, public_key.len() as u16);
    entry.extend_from_slice(public_key);

    Extension {
        ext_type: ExtensionType::KeyShare,
        data: entry.freeze(),
    }
}

/// Build a `key_share` extension for a HelloRetryRequest.
///
/// In HRR the `key_share` extension carries only a 2-byte `NamedGroup`
/// (the group the server wants the client to retry with), with no public key
/// (RFC 8446 §4.1.4).
pub fn ext_key_share_hrr(group: KeyExchangeGroup) -> Extension {
    Extension {
        ext_type: ExtensionType::KeyShare,
        data: Bytes::copy_from_slice(&group.to_wire()),
    }
}

/// Parse a key share extension from a ClientHello.
pub fn parse_key_share(ext: &Extension) -> Result<(KeyExchangeGroup, heapless::Vec<u8, MAX_KX_PUBLIC_KEY>), Error> {
    let data = &ext.data[..];
    if data.len() < 6 {
        return Err(Error::DecodeError("key_share too short".into()));
    }
    let _total = u16::from_be_bytes([data[0], data[1]]) as usize;
    let group = KeyExchangeGroup::from_wire([data[2], data[3]])
        .ok_or_else(|| Error::DecodeError("unknown kx group in key_share".into()))?;
    let pk_len = u16::from_be_bytes([data[4], data[5]]) as usize;
    let mut pk = heapless::Vec::<u8, MAX_KX_PUBLIC_KEY>::new();
    pk.extend_from_slice(&data[6..6 + pk_len])
        .map_err(|_| Error::DecodeError("key share data too large".into()))?;
    Ok((group, pk))
}

/// Parse a key share extension from a ServerHello (no total length prefix).
pub fn parse_key_share_server(
    ext: &Extension,
) -> Result<(KeyExchangeGroup, heapless::Vec<u8, MAX_KX_PUBLIC_KEY>), Error> {
    let data = &ext.data[..];
    if data.len() < 4 {
        return Err(Error::DecodeError("key_share (ServerHello) too short".into()));
    }
    let group = KeyExchangeGroup::from_wire([data[0], data[1]])
        .ok_or_else(|| Error::DecodeError("unknown kx group in ServerHello key_share".into()))?;
    let pk_len = u16::from_be_bytes([data[2], data[3]]) as usize;
    let mut pk = heapless::Vec::<u8, MAX_KX_PUBLIC_KEY>::new();
    pk.extend_from_slice(&data[4..4 + pk_len])
        .map_err(|_| Error::DecodeError("key share data too large".into()))?;
    Ok((group, pk))
}

/// Build a `signature_algorithms` extension.
pub fn ext_signature_algorithms(schemes: &[SignatureScheme]) -> Extension {
    let mut body = BytesMut::with_capacity(schemes.len() * 2);
    for s in schemes {
        body.extend_from_slice(&s.to_wire());
    }
    let mut data = BytesMut::with_capacity(2 + body.len());
    put_u16(&mut data, body.len() as u16);
    data.extend_from_slice(&body);
    Extension {
        ext_type: ExtensionType::SignatureAlgorithms,
        data: data.freeze(),
    }
}

/// Parse a `signature_algorithms` extension, returning the list of schemes.
pub fn parse_signature_algorithms(ext: &Extension) -> Result<Vec<SignatureScheme>, Error> {
    let data = &ext.data[..];
    if data.len() < 2 {
        return Err(Error::DecodeError("signature_algorithms extension too short".into()));
    }
    let total = u16::from_be_bytes([data[0], data[1]]) as usize;
    if data.len() < 2 + total || total % 2 != 0 {
        return Err(Error::DecodeError("signature_algorithms extension malformed".into()));
    }
    let mut schemes = Vec::with_capacity(total / 2);
    for i in (0..total).step_by(2) {
        let b: [u8; 2] = [data[2 + i], data[2 + i + 1]];
        if let Some(s) = SignatureScheme::from_wire(b) {
            schemes.push(s);
        }
    }
    Ok(schemes)
}

/// Build a `server_name` (SNI) extension.
pub fn ext_server_name(hostname: &str) -> Extension {
    let entry_len = 1 + 2 + hostname.len();
    let mut entry = BytesMut::with_capacity(entry_len);
    entry.put_u8(0); // host_name type
    put_u16(&mut entry, hostname.len() as u16);
    entry.extend_from_slice(hostname.as_bytes());

    let mut data = BytesMut::with_capacity(2 + entry.len());
    put_u16(&mut data, entry.len() as u16);
    data.extend_from_slice(&entry);
    Extension {
        ext_type: ExtensionType::ServerName,
        data: data.freeze(),
    }
}

/// Build an ALPN extension.
pub fn ext_alpn(protocols: &[Bytes]) -> Extension {
    let body_size: usize = protocols.iter().map(|p| 1 + p.len()).sum();
    let mut body = BytesMut::with_capacity(body_size);
    for p in protocols {
        put_u8_slice(&mut body, p);
    }
    let mut data = BytesMut::with_capacity(2 + body.len());
    put_u16(&mut data, body.len() as u16);
    data.extend_from_slice(&body);
    Extension {
        ext_type: ExtensionType::ApplicationLayerProtocolNegotiation,
        data: data.freeze(),
    }
}

/// Parse ALPN extension, returning the list of protocols (max 8).
pub fn parse_alpn(ext: &Extension) -> Result<heapless::Vec<Bytes, 8>, Error> {
    let data = &ext.data[..];
    if data.len() < 2 {
        return Err(Error::DecodeError("ALPN extension too short".into()));
    }
    let total = u16::from_be_bytes([data[0], data[1]]) as usize;
    let data = &data[2..];
    let mut items: heapless::Vec<Bytes, 8> = heapless::Vec::new();
    let mut off = 0;
    while off + 1 <= total && off < data.len() {
        let len = data[off] as usize;
        off += 1;
        if off + len > data.len() {
            break;
        }
        items
            .push(Bytes::copy_from_slice(&data[off..off + len]))
            .map_err(|_| Error::DecodeError("ALPN: too many protocols (max 8)".into()))?;
        off += len;
    }
    Ok(items)
}

/// Build a `server_certificate_type` extension for the ClientHello.
///
/// `types` is the list of certificate types the client supports,
/// in preference order (each is one byte: 0 for X.509, 1 for RawPublicKey).
pub fn ext_server_cert_type_client(types: &[CertType]) -> Extension {
    let mut data = BytesMut::with_capacity(1 + types.len());
    data.put_u8(types.len() as u8);
    for t in types {
        data.put_u8(*t as u8);
    }
    Extension {
        ext_type: ExtensionType::ServerCertificateType,
        data: data.freeze(),
    }
}

/// Build a `server_certificate_type` extension for the EncryptedExtensions.
///
/// `cert_type` is the single certificate type selected by the server.
pub fn ext_server_cert_type_server(cert_type: CertType) -> Extension {
    Extension {
        ext_type: ExtensionType::ServerCertificateType,
        data: Bytes::from(vec![cert_type as u8]),
    }
}

/// Parse a `server_certificate_type` extension from EncryptedExtensions.
///
/// Returns the certificate type selected by the server (1 byte).
pub fn parse_server_cert_type_ee(ext: &Extension) -> Result<CertType, Error> {
    let data = &ext.data[..];
    if data.is_empty() {
        return Err(Error::DecodeError("empty server_certificate_type extension".into()));
    }
    CertType::from_u8(data[0]).ok_or_else(|| Error::DecodeError(format!("unknown cert type {}", data[0]).into()))
}

/// Parse a `server_certificate_type` extension from a ClientHello.
///
/// Returns the list of certificate types offered by the client.
pub fn parse_server_cert_type_ch(ext: &Extension) -> Result<Vec<CertType>, Error> {
    let data = &ext.data[..];
    if data.is_empty() {
        return Err(Error::DecodeError("empty server_certificate_type extension".into()));
    }
    let len = data[0] as usize;
    if data.len() < 1 + len {
        return Err(Error::DecodeError("server_certificate_type list truncated".into()));
    }
    let mut types = Vec::with_capacity(len);
    for i in 0..len {
        if let Some(ct) = CertType::from_u8(data[1 + i]) {
            types.push(ct);
        }
    }
    Ok(types)
}

/// Build the `pre_shared_key` extension (ClientHello).
///
/// `identities` is one tuple (identity, obfuscated_ticket_age) per PSK.
/// `binders` are the computed binder MAC values.
pub fn ext_pre_shared_key(identities: &[(Vec<u8>, u32)], binders: &[Vec<u8>]) -> Extension {
    let id_total: usize = identities.iter().map(|(id, _)| 2 + id.len() + 4).sum();
    let b_total: usize = binders.iter().map(|b| 1 + b.len()).sum();
    let mut body = BytesMut::with_capacity(2 + id_total + 2 + b_total);

    // identities
    let mut id_body = BytesMut::with_capacity(id_total);
    for (id, age) in identities {
        put_u16_slice(&mut id_body, id);
        id_body.extend_from_slice(&age.to_be_bytes());
    }
    put_u16(&mut body, id_body.len() as u16);
    body.extend_from_slice(&id_body);

    // binders
    let mut b_body = BytesMut::with_capacity(b_total);
    for b in binders {
        put_u8_slice(&mut b_body, b);
    }
    put_u16(&mut body, b_body.len() as u16);
    body.extend_from_slice(&b_body);

    Extension {
        ext_type: ExtensionType::PreSharedKey,
        data: body.freeze(),
    }
}

/// Build the `psk_key_exchange_modes` extension.
pub fn ext_psk_key_exchange_modes() -> Extension {
    Extension {
        ext_type: ExtensionType::PskKeyExchangeModes,
        data: Bytes::from_static(&[1, 1]), // psk_dhe_ke
    }
}

/// Build an `early_data` extension for ClientHello (RFC 8446 §4.2.10).
///
/// Indicates the client wishes to send 0-RTT data. The extension carries
/// no data in ClientHello.
pub fn ext_early_data_client() -> Extension {
    Extension {
        ext_type: ExtensionType::EarlyData,
        data: Bytes::new(),
    }
}

/// Build an `early_data` extension for EncryptedExtensions (server acceptance).
pub fn ext_early_data_encrypted_extensions() -> Extension {
    Extension {
        ext_type: ExtensionType::EarlyData,
        data: Bytes::new(),
    }
}

/// Build the `pre_shared_key` extension for a ServerHello.
///
/// `selected_identity` is the 0-based index of the PSK identity that
/// the server selected from the client's offer (RFC 8446 §4.2.11).
pub fn ext_pre_shared_key_server(selected_identity: u16) -> Extension {
    Extension {
        ext_type: ExtensionType::PreSharedKey,
        data: Bytes::copy_from_slice(&selected_identity.to_be_bytes()),
    }
}

/// Build a QUIC transport parameters extension.
pub fn ext_quic_transport_parameters(params: &[u8]) -> Extension {
    Extension {
        ext_type: ExtensionType::QuicTransportParameters,
        data: Bytes::copy_from_slice(params),
    }
}

/// Find an extension by type in a list.
pub fn find_extension<'a>(exts: &'a [Extension], ext_type: ExtensionType) -> Option<&'a Extension> {
    exts.iter().find(|e| e.ext_type == ext_type)
}

/// Read the supported_versions extension — must include TLS 1.3 (0x0304).
pub fn check_supported_versions(ext: Option<&Extension>) -> bool {
    let data = match ext {
        Some(e) => &e.data[..],
        None => return false,
    };
    if data.len() < 3 {
        return false;
    }
    let len = data[0] as usize;
    if data.len() < 1 + len {
        return false;
    }
    for i in (1..1 + len).step_by(2) {
        if i + 1 < 1 + len {
            let v = u16::from_be_bytes([data[i], data[i + 1]]);
            if v == 0x0304 {
                return true;
            }
        }
    }
    false
}

/// Extract the negotiated TLS version from a `supported_versions` extension.
/// Handles both ServerHello (version only) and ClientHello (length + list) formats.
pub fn parse_supported_versions(ext: Option<&Extension>) -> Option<u16> {
    let data = ext?.data.as_ref();
    if data.len() < 2 {
        return None;
    }
    // The version is always the last 2 bytes in both formats
    Some(u16::from_be_bytes([data[data.len() - 2], data[data.len() - 1]]))
}

/// Magic `random` value that distinguishes a HelloRetryRequest from a normal
/// ServerHello (RFC 8446 §4.1.4).
pub const HRR_RANDOM: [u8; 32] = [
    0xCF, 0x21, 0xAD, 0x74, 0xE5, 0x9A, 0x61, 0x11, 0xBE, 0x1D, 0x8C, 0x02, 0x1E, 0x65, 0xB8, 0x91, 0xC2, 0xA2, 0x11,
    0x16, 0x7A, 0xBB, 0x8C, 0x5E, 0x07, 0x9E, 0x09, 0xE2, 0xC8, 0xA8, 0x33, 0x9C,
];

/// Encode a `MessageHash` handshake message (RFC 8446 §4.4.1).
///
/// Used during HelloRetryRequest to replace the original ClientHello in the
/// transcript with a hash of it.
pub fn encode_message_hash(hash: &[u8]) -> Bytes {
    encode_handshake(HandshakeType::MessageHash, hash)
}

// ── GREASE (RFC 8701) ─────────────────────────────────────────────────────

/// Generate a GREASE value for cipher suites (2 bytes) using a seed byte.
///
/// GREASE values are of the form `0x?A?A` where `?` is a random nibble in
/// 0..16. The seed should come from a CSPRNG.
pub fn grease_cipher_suite(seed_byte: u8) -> [u8; 2] {
    let nibble = (seed_byte & 0x0f) as u16;
    let v = nibble << 12 | 0x0A << 8 | nibble << 4 | 0x0A;
    v.to_be_bytes()
}

/// Generate a GREASE extension type (2 bytes) using a seed byte.
pub fn grease_extension_type(seed_byte: u8) -> u16 {
    let nibble = (seed_byte & 0x0f) as u16;
    (nibble << 12 | 0x0A << 8 | nibble << 4 | 0x0A) | 0x8000 // highest bit set for GREASE extension
}

/// Generate a GREASE supported group (2 bytes) using a seed byte.
pub fn grease_supported_group(seed_byte: u8) -> [u8; 2] {
    let nibble = (seed_byte & 0x0f) as u16;
    let v = nibble << 12 | 0x0A << 8 | nibble << 4 | 0x0A;
    v.to_be_bytes()
}
