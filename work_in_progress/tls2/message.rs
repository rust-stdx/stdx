use crate::{
    CertType, CipherSuite, KeyExchangeGroup, KeyExchangePublicKey, ParsedCertificate, ReceivedCertificate,
    SignatureScheme, errors::Error,
};

// ── Wire format helpers ──────────────────────────────────────────────────

#[inline]
pub fn put_u16(buf: &mut [u8], v: u16) -> usize {
    buf[..2].copy_from_slice(&v.to_be_bytes());
    2
}

#[inline]
pub fn put_u24(buf: &mut [u8], v: u32) -> usize {
    let bytes = v.to_be_bytes();
    buf[..3].copy_from_slice(&bytes[1..]);
    3
}

#[inline]
pub fn put_slice_u8(buf: &mut [u8], data: &[u8]) -> usize {
    let n = data.len();
    buf[0] = n as u8;
    buf[1..1 + n].copy_from_slice(data);
    1 + n
}

#[inline]
pub fn put_slice_u16(buf: &mut [u8], data: &[u8]) -> usize {
    let n = data.len();
    buf[..2].copy_from_slice(&(n as u16).to_be_bytes());
    buf[2..2 + n].copy_from_slice(data);
    2 + n
}

#[inline]
pub fn read_u8(data: &[u8], offset: &mut usize) -> u8 {
    let v = data[*offset];
    *offset += 1;
    v
}

#[inline]
pub fn read_u16(data: &[u8], offset: &mut usize) -> u16 {
    let v = u16::from_be_bytes([data[*offset], data[*offset + 1]]);
    *offset += 2;
    v
}

#[inline]
pub fn read_u16_raw(data: &[u8]) -> u16 {
    u16::from_be_bytes([data[0], data[1]])
}

#[inline]
pub fn read_u24(data: &[u8], offset: &mut usize) -> u32 {
    let v = u32::from_be_bytes([0, data[*offset], data[*offset + 1], data[*offset + 2]]);
    *offset += 3;
    v
}

#[inline]
pub fn read_slice_u8<'a>(data: &'a [u8], offset: &mut usize) -> Result<&'a [u8], Error> {
    let len = read_u8(data, offset) as usize;
    if *offset + len > data.len() {
        return Err(Error::DecodeError);
    }
    let slice = &data[*offset..*offset + len];
    *offset += len;
    Ok(slice)
}

#[inline]
pub fn read_slice_u16<'a>(data: &'a [u8], offset: &mut usize) -> Result<&'a [u8], Error> {
    let len = read_u16(data, offset) as usize;
    if *offset + len > data.len() {
        return Err(Error::DecodeError);
    }
    let slice = &data[*offset..*offset + len];
    *offset += len;
    Ok(slice)
}

// ── Handshake types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HandshakeType {
    ClientHello = 1,
    ServerHello = 2,
    NewSessionTicket = 4,
    EncryptedExtensions = 8,
    Certificate = 11,
    CertificateRequest = 13,
    CertificateVerify = 15,
    Finished = 20,
    KeyUpdate = 24,
}

impl HandshakeType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::ClientHello),
            2 => Some(Self::ServerHello),
            4 => Some(Self::NewSessionTicket),
            8 => Some(Self::EncryptedExtensions),
            11 => Some(Self::Certificate),
            13 => Some(Self::CertificateRequest),
            15 => Some(Self::CertificateVerify),
            20 => Some(Self::Finished),
            24 => Some(Self::KeyUpdate),
            _ => None,
        }
    }
}

/// Encode a handshake header: type(1) + length(3). Returns bytes written.
pub fn encode_handshake_frame(buf: &mut [u8], msg_type: HandshakeType, body_len: usize) -> usize {
    buf[0] = msg_type as u8;
    1 + put_u24(&mut buf[1..], body_len as u32)
}

/// Decode a handshake header, returning the type and body slice.
pub fn decode_handshake_frame<'a>(data: &'a [u8], offset: &mut usize) -> Result<(HandshakeType, &'a [u8]), Error> {
    if *offset + 4 > data.len() {
        return Err(Error::DecodeError);
    }
    let msg_type = HandshakeType::from_u8(data[*offset]).ok_or(Error::DecodeError)?;
    *offset += 1;
    let len = read_u24(data, offset) as usize;
    if *offset + len > data.len() {
        return Err(Error::DecodeError);
    }
    let body = &data[*offset..*offset + len];
    *offset += len;
    Ok((msg_type, body))
}

// ── Extension types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ExtensionType {
    ServerName = 0,
    SupportedGroups = 10,
    SignatureAlgorithms = 13,
    ApplicationLayerProtocolNegotiation = 16,
    SupportedVersions = 43,
    KeyShare = 51,
    ServerCertificateType = 20,
}

impl ExtensionType {
    pub fn from_u16(v: u16) -> Option<Self> {
        match v {
            0 => Some(Self::ServerName),
            10 => Some(Self::SupportedGroups),
            13 => Some(Self::SignatureAlgorithms),
            16 => Some(Self::ApplicationLayerProtocolNegotiation),
            43 => Some(Self::SupportedVersions),
            51 => Some(Self::KeyShare),
            20 => Some(Self::ServerCertificateType),
            _ => None,
        }
    }
}

// ── ClientHello encoding ─────────────────────────────────────────────────

/// A single TLS extension to include in a ClientHello.
///
/// Each variant carries the data needed to encode its payload, all through
/// borrowed references — zero allocations.
#[derive(Clone, Copy, Debug)]
pub enum Extension<'a> {
    ServerName {
        host_name: &'a str,
    },
    SupportedGroups {
        key_share_entries: &'a [KeyExchangePublicKey],
    },
    SignatureAlgorithms {
        schemes: &'a [SignatureScheme],
    },
    ApplicationLayerProtocolNegotiation {
        protocols: &'a [&'a [u8]],
    },
    ServerCertificateType {
        types: &'a [CertType],
    },
    SupportedVersions,
    KeyShare {
        entries: &'a [KeyExchangePublicKey],
    },
}

/// Write type(2) + length(2) around a payload, then back-patch the data
/// length. Returns total extension bytes written (4 + payload).
fn encode_extension_with_type(
    buf: &mut [u8],
    ext_type: ExtensionType,
    payload: impl FnOnce(&mut [u8]) -> usize,
) -> usize {
    let data_len = payload(&mut buf[4..]);
    buf[..2].copy_from_slice(&(ext_type as u16).to_be_bytes());
    buf[2..4].copy_from_slice(&(data_len as u16).to_be_bytes());
    4 + data_len
}

/// Encode a single extension into `buf`. Returns bytes written.
fn encode_extension(buf: &mut [u8], ext: &Extension<'_>) -> usize {
    match ext {
        Extension::ServerName {
            host_name,
        } => {
            encode_extension_with_type(buf, ExtensionType::ServerName, |b| {
                let name_bytes = host_name.as_bytes();
                let mut p = 0;
                p += put_u16(&mut b[p..], 3 + name_bytes.len() as u16);
                b[p] = 0; // host_name type
                p += 1;
                p += put_slice_u16(&mut b[p..], name_bytes);
                p
            })
        }
        Extension::SupportedGroups {
            key_share_entries,
        } => encode_extension_with_type(buf, ExtensionType::SupportedGroups, |b| {
            let mut p = 2;
            for ks in *key_share_entries {
                let wire = ks.group().to_wire();
                b[p..p + 2].copy_from_slice(&wire);
                p += 2;
            }
            let list_len = (p - 2) as u16;
            b[..2].copy_from_slice(&list_len.to_be_bytes());
            p
        }),
        Extension::SignatureAlgorithms {
            schemes,
        } => encode_extension_with_type(buf, ExtensionType::SignatureAlgorithms, |b| {
            let mut p = 2;
            for s in *schemes {
                let wire = s.to_wire();
                b[p..p + 2].copy_from_slice(&wire);
                p += 2;
            }
            let list_len = (p - 2) as u16;
            b[..2].copy_from_slice(&list_len.to_be_bytes());
            p
        }),
        Extension::ApplicationLayerProtocolNegotiation {
            protocols,
        } => encode_extension_with_type(buf, ExtensionType::ApplicationLayerProtocolNegotiation, |b| {
            let mut p = 2;
            for proto in *protocols {
                p += put_slice_u8(&mut b[p..], proto);
            }
            let list_len = (p - 2) as u16;
            b[..2].copy_from_slice(&list_len.to_be_bytes());
            p
        }),
        Extension::ServerCertificateType {
            types,
        } => encode_extension_with_type(buf, ExtensionType::ServerCertificateType, |b| {
            b[0] = types.len() as u8;
            let mut p = 1;
            for ct in *types {
                b[p] = *ct as u8;
                p += 1;
            }
            p
        }),
        Extension::SupportedVersions => encode_extension_with_type(buf, ExtensionType::SupportedVersions, |b| {
            b[0] = 2;
            b[1..3].copy_from_slice(&[0x03, 0x04]);
            3
        }),
        Extension::KeyShare {
            entries,
        } => encode_extension_with_type(buf, ExtensionType::KeyShare, |b| {
            let mut p = 2;
            for ks in *entries {
                let wire = ks.group().to_wire();
                b[p..p + 2].copy_from_slice(&wire);
                p += 2;
                p += put_slice_u16(&mut b[p..], ks.bytes());
            }
            let list_len = (p - 2) as u16;
            b[..2].copy_from_slice(&list_len.to_be_bytes());
            p
        }),
    }
}

/// Encode a ClientHello message into `buf`.
pub fn encode_client_hello(
    buf: &mut [u8],
    random: &[u8; 32],
    session_id: &[u8],
    cipher_suites: &[CipherSuite],
    extensions: &[Extension<'_>],
) -> Result<usize, Error> {
    let mut pos = 0;

    // Handshake header placeholder (4 bytes)
    buf[pos] = HandshakeType::ClientHello as u8;
    pos += 1;
    let len_pos = pos;
    pos += 3;

    // legacy_version = 0x0303
    pos += put_u16(&mut buf[pos..], 0x0303);

    // random
    buf[pos..pos + 32].copy_from_slice(random);
    pos += 32;

    // legacy_session_id
    pos += put_slice_u8(&mut buf[pos..], session_id);

    // cipher_suites (2-byte length + 2 bytes per suite)
    let cs_start = pos;
    pos += 2;
    for cs in cipher_suites {
        buf[pos..pos + 2].copy_from_slice(&cs.to_wire());
        pos += 2;
    }
    let cs_len = (pos - cs_start - 2) as u16;
    buf[cs_start..cs_start + 2].copy_from_slice(&cs_len.to_be_bytes());

    // legacy_compression_methods (null only: length 1, method 0)
    buf[pos] = 1;
    pos += 1;
    buf[pos] = 0;
    pos += 1;

    // ── Extensions ──
    let ext_start = pos;
    pos += 2; // placeholder for total extensions length
    for ext in extensions {
        pos += encode_extension(&mut buf[pos..], ext);
    }
    let ext_total = (pos - ext_start - 2) as u16;
    buf[ext_start..ext_start + 2].copy_from_slice(&ext_total.to_be_bytes());

    // Fill handshake body length
    let body_len = (pos - len_pos - 3) as u32;
    buf[len_pos..len_pos + 3].copy_from_slice(&body_len.to_be_bytes()[1..]);

    Ok(pos)
}

// ── ServerHello decoding ─────────────────────────────────────────────────

pub struct ServerHelloData<'a> {
    pub random: [u8; 32],
    pub session_id: &'a [u8],
    pub cipher_suite: CipherSuite,
    pub key_share_group: KeyExchangeGroup,
    pub key_share_public: &'a [u8],
}

/// Decode a ServerHello handshake message.
pub fn decode_server_hello<'a>(body: &'a [u8]) -> Result<ServerHelloData<'a>, Error> {
    if body.len() < 38 {
        return Err(Error::DecodeError);
    }
    let mut off = 0;

    let _version = read_u16(body, &mut off);

    let mut random = [0u8; 32];
    random.copy_from_slice(&body[off..off + 32]);
    off += 32;

    let session_id = read_slice_u8(body, &mut off)?;

    let cs = CipherSuite::from_wire([body[off], body[off + 1]]).ok_or(Error::UnsupportedCipherSuite)?;
    off += 2;

    off += 1;

    if off + 2 > body.len() {
        return Err(Error::DecodeError);
    }
    let ext_total = read_u16(body, &mut off) as usize;
    if off + ext_total > body.len() {
        return Err(Error::DecodeError);
    }
    let ext_data = &body[off..off + ext_total];
    let mut ext_off = 0;

    let mut key_share_group = None;
    let mut key_share_public = &[][..];

    while ext_off + 4 <= ext_data.len() {
        let ext_type = read_u16(ext_data, &mut ext_off);
        let ext_len = read_u16(ext_data, &mut ext_off) as usize;
        if ext_off + ext_len > ext_data.len() {
            return Err(Error::DecodeError);
        }

        if let Some(ExtensionType::KeyShare) = ExtensionType::from_u16(ext_type) {
            if ext_len >= 4 {
                let group_bytes = [ext_data[ext_off], ext_data[ext_off + 1]];
                key_share_group =
                    Some(KeyExchangeGroup::from_wire(group_bytes).ok_or(Error::UnsupportedKeyExchangeGroup)?);
                key_share_public = &ext_data[ext_off + 4..ext_off + ext_len];
            }
        }
        ext_off += ext_len;
    }

    let kx_group = key_share_group.ok_or(Error::DecodeError)?;
    Ok(ServerHelloData {
        random,
        session_id,
        cipher_suite: cs,
        key_share_group: kx_group,
        key_share_public,
    })
}

// ── EncryptedExtensions decoding ─────────────────────────────────────────

/// Decode EncryptedExtensions, returning `(alpn_protocol, server_cert_type)`.
pub fn decode_encrypted_extensions<'a>(body: &'a [u8]) -> Result<(Option<&'a [u8]>, Option<CertType>), Error> {
    let mut off = 0;
    let ext_data = read_slice_u16(body, &mut off)?;
    let mut ext_off = 0;

    let mut alpn = None;
    let mut cert_type = None;

    while ext_off + 4 <= ext_data.len() {
        let ext_type = read_u16(ext_data, &mut ext_off);
        let ext_len = read_u16(ext_data, &mut ext_off) as usize;
        if ext_off + ext_len > ext_data.len() {
            return Err(Error::DecodeError);
        }

        match ExtensionType::from_u16(ext_type) {
            Some(ExtensionType::ApplicationLayerProtocolNegotiation) => {
                let alpn_body = &ext_data[ext_off..ext_off + ext_len];
                if alpn_body.len() < 3 {
                    return Err(Error::DecodeError);
                }
                let list_len = read_u16_raw(alpn_body) as usize;
                if list_len > alpn_body.len() - 2 {
                    return Err(Error::DecodeError);
                }
                let name_len = alpn_body[2] as usize;
                if 3 + name_len > alpn_body.len() {
                    return Err(Error::DecodeError);
                }
                alpn = Some(&alpn_body[3..3 + name_len]);
            }
            Some(ExtensionType::ServerCertificateType) => {
                if ext_len >= 1 {
                    cert_type = CertType::from_u8(ext_data[ext_off]);
                }
            }
            _ => {}
        }
        ext_off += ext_len;
    }
    Ok((alpn, cert_type))
}

// ── Certificate decoding ─────────────────────────────────────────────────

/// Decode extensions list: 2-byte total length + N × (type(2) + length(2) + data)
pub fn decode_extensions<'a>(data: &'a [u8], offset: &mut usize) -> Result<(u16, &'a [u8]), Error> {
    if *offset + 2 > data.len() {
        return Err(Error::DecodeError);
    }
    let total = read_u16(data, offset) as usize;
    if *offset + total > data.len() {
        return Err(Error::DecodeError);
    }
    let ext_data = &data[*offset..*offset + total];
    *offset += total;
    Ok((total as u16, ext_data))
}

/// Skip through extensions, looking for specific types.
/// Returns the selected ALPN protocol if the ALPN extension was found.
pub fn decode_server_hello_extensions<'a>(ext_data: &'a [u8]) -> Result<(KeyExchangeGroup, &'a [u8]), Error> {
    let mut ext_off = 0;
    let mut kx_group = None;
    let mut kx_public = &[][..];

    while ext_off + 4 <= ext_data.len() {
        let ext_type = read_u16(ext_data, &mut ext_off);
        let ext_len = read_u16(ext_data, &mut ext_off) as usize;
        if ext_off + ext_len > ext_data.len() {
            return Err(Error::DecodeError);
        }
        if let Some(ExtensionType::KeyShare) = ExtensionType::from_u16(ext_type) {
            if ext_len >= 4 {
                let group = KeyExchangeGroup::from_wire([ext_data[ext_off], ext_data[ext_off + 1]])
                    .ok_or(Error::UnsupportedKeyExchangeGroup)?;
                let ks_len = u16::from_be_bytes([ext_data[ext_off + 2], ext_data[ext_off + 3]]) as usize;
                if ext_off + 4 + ks_len > ext_data.len() {
                    return Err(Error::DecodeError);
                }
                kx_group = Some(group);
                kx_public = &ext_data[ext_off + 4..ext_off + 4 + ks_len];
            }
        }
        ext_off += ext_len;
    }
    Ok((kx_group.ok_or(Error::DecodeError)?, kx_public))
}

/// Decode Certificate message body.
///
/// For `X509` returns all certificate DERs in a chain. For `RawPublicKey`
/// extracts the SPKI from the first (and only) entry and returns the raw
/// key bytes.
pub fn decode_certificate<'a>(body: &'a [u8], cert_type: CertType) -> Result<ReceivedCertificate<'a>, Error> {
    let mut off = 0;

    let ctx_len = read_u8(body, &mut off) as usize;
    off += ctx_len;

    if off + 3 > body.len() {
        return Err(Error::DecodeError);
    }
    let list_len = read_u24(body, &mut off) as usize;
    if off + list_len > body.len() {
        return Err(Error::DecodeError);
    }

    let list_end = off + list_len;

    match cert_type {
        CertType::X509 => {
            let mut chain = heapless::Vec::new();
            while off < list_end {
                let cert_len = read_u24(body, &mut off) as usize;
                if off + cert_len > list_end {
                    return Err(Error::DecodeError);
                }
                let cert_der = &body[off..off + cert_len];
                off += cert_len;

                let ext_len = read_u16(body, &mut off) as usize;
                off += ext_len;

                let parsed = ParsedCertificate::from_der(cert_der)?;
                chain.push(parsed).map_err(|_| Error::DecodeError)?;
            }
            if chain.is_empty() {
                return Err(Error::DecodeError);
            }
            Ok(ReceivedCertificate::X509 {
                chain,
            })
        }
        CertType::RawPublicKey => {
            // RFC 7250 §3: the certificate_list contains a single entry
            // whose cert_data is the SubjectPublicKeyInfo DER.
            if off >= list_end {
                return Err(Error::CertificateEmptyRawPublicKey);
            }
            let pk_len = read_u24(body, &mut off) as usize;
            if off + pk_len > list_end {
                return Err(Error::DecodeError);
            }
            let spki_der = &body[off..off + pk_len];
            off += pk_len;

            // Skip extensions.
            let ext_len = read_u16(body, &mut off) as usize;
            off += ext_len;

            let public_key = x509::extract_key_from_spki(spki_der).map_err(|_| Error::DecodeError)?;
            // Scheme will be determined by the caller via detect_key_scheme.
            // Use Ed25519 as a placeholder (overwritten by probe).
            Ok(ReceivedCertificate::RawPublicKey {
                public_key,
                scheme: SignatureScheme::Ed25519,
            })
        }
    }
}

// ── CertificateVerify decoding ───────────────────────────────────────────

pub struct CertificateVerifyData<'a> {
    pub scheme: SignatureScheme,
    pub signature: &'a [u8],
}

/// Decode CertificateVerify and extract the signature scheme + signature bytes.
pub fn decode_certificate_verify<'a>(body: &'a [u8]) -> Result<CertificateVerifyData<'a>, Error> {
    let mut off = 0;
    let scheme = SignatureScheme::from_wire([body[off], body[off + 1]]).ok_or(Error::DecodeError)?;
    off += 2;
    let signature = read_slice_u16(body, &mut off)?;
    Ok(CertificateVerifyData {
        scheme,
        signature,
    })
}

// ── Finished decoding ────────────────────────────────────────────────────

/// Decode Finished and return the verify_data.
pub fn decode_finished<'a>(body: &'a [u8]) -> Result<&'a [u8], Error> {
    // Finished body is just the verify_data (hash_size bytes, no length prefix)
    Ok(body)
}

// ── NewSessionTicket decoding ────────────────────────────────────────────

pub struct NewSessionTicketData<'a> {
    pub lifetime_s: u32,
    pub age_add: u32,
    pub nonce: &'a [u8],
    pub ticket: &'a [u8],
}

/// Decode a NewSessionTicket message body.
pub fn decode_new_session_ticket<'a>(body: &'a [u8]) -> Result<NewSessionTicketData<'a>, Error> {
    let mut off = 0;

    if off + 4 > body.len() {
        return Err(Error::DecodeError);
    }
    let lifetime_s = u32::from_be_bytes([body[off], body[off + 1], body[off + 2], body[off + 3]]);
    off += 4;

    if off + 4 > body.len() {
        return Err(Error::DecodeError);
    }
    let age_add = u32::from_be_bytes([body[off], body[off + 1], body[off + 2], body[off + 3]]);
    off += 4;

    let nonce = read_slice_u8(body, &mut off)?;
    let ticket = read_slice_u16(body, &mut off)?;

    Ok(NewSessionTicketData {
        lifetime_s,
        age_add,
        nonce,
        ticket,
    })
}

// ── KeyUpdate decoding ───────────────────────────────────────────────────

/// Decode the KeyUpdate request_update byte.
pub fn decode_key_update(body: &[u8]) -> Result<u8, Error> {
    if body.is_empty() {
        return Err(Error::DecodeError);
    }
    Ok(body[0])
}
