use crate::{CipherSuite, CryptoProvider, KeyExchangeGroup, SignatureScheme, errors::Error};

// ── Wire format helpers ──────────────────────────────────────────────────

#[inline]
pub fn put_u16(buf: &mut [u8], offset: &mut usize, v: u16) {
    buf[*offset..*offset + 2].copy_from_slice(&v.to_be_bytes());
    *offset += 2;
}

#[inline]
pub fn put_u24(buf: &mut [u8], offset: &mut usize, v: u32) {
    let bytes = v.to_be_bytes();
    buf[*offset..*offset + 3].copy_from_slice(&bytes[1..]);
    *offset += 3;
}

#[inline]
pub fn put_slice_u8(buf: &mut [u8], offset: &mut usize, data: &[u8]) {
    buf[*offset] = data.len() as u8;
    *offset += 1;
    buf[*offset..*offset + data.len()].copy_from_slice(data);
    *offset += data.len();
}

#[inline]
pub fn put_slice_u16(buf: &mut [u8], offset: &mut usize, data: &[u8]) {
    put_u16(buf, offset, data.len() as u16);
    buf[*offset..*offset + data.len()].copy_from_slice(data);
    *offset += data.len();
}

#[inline]
pub fn put_slice_u24(buf: &mut [u8], offset: &mut usize, data: &[u8]) {
    put_u24(buf, offset, data.len() as u32);
    buf[*offset..*offset + data.len()].copy_from_slice(data);
    *offset += data.len();
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

#[inline]
pub fn read_slice_u24<'a>(data: &'a [u8], offset: &mut usize) -> Result<&'a [u8], Error> {
    let len = read_u24(data, offset) as usize;
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

/// Encode a handshake header: type(1) + length(3).
pub fn encode_handshake_frame<'a>(buf: &'a mut [u8], offset: &mut usize, msg_type: HandshakeType, body_len: usize) {
    buf[*offset] = msg_type as u8;
    *offset += 1;
    put_u24(buf, offset, body_len as u32);
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

/// Encode a ClientHello message into `buf` starting at `offset`.
/// Returns the number of bytes written.
pub fn encode_client_hello(
    buf: &mut [u8],
    offset: &mut usize,
    random: &[u8; 32],
    session_id: &[u8],
    cipher_suites: &[CipherSuite],
    key_share_group: KeyExchangeGroup,
    key_share_public: &[u8],
    server_name: Option<&str>,
    alpn_protocols: &[&[u8]],
    supported_groups: &[KeyExchangeGroup],
    signature_schemes: &[SignatureScheme],
) -> Result<usize, Error> {
    let body_start = *offset;

    // Handshake header placeholder (4 bytes)
    buf[*offset] = HandshakeType::ClientHello as u8;
    *offset += 1;
    let len_pos = *offset; // save position for body length
    *offset += 3;

    // legacy_version = 0x0303
    put_u16(buf, offset, 0x0303);

    // random
    buf[*offset..*offset + 32].copy_from_slice(random);
    *offset += 32;

    // legacy_session_id
    put_slice_u8(buf, offset, session_id);

    // cipher_suites (2-byte length + 2 bytes per suite)
    let cs_start = *offset;
    *offset += 2;
    for cs in cipher_suites {
        buf[*offset..*offset + 2].copy_from_slice(&cs.to_wire());
        *offset += 2;
    }
    let cs_len = (*offset - cs_start - 2) as u16;
    buf[cs_start..cs_start + 2].copy_from_slice(&cs_len.to_be_bytes());

    // legacy_compression_methods (null only: length 1, method 0)
    buf[*offset] = 1;
    *offset += 1;
    buf[*offset] = 0;
    *offset += 1;

    // ── Extensions ──
    let ext_total_start = *offset;
    *offset += 2; // placeholder for total extensions length

    // 1. KeyShare
    let ks_start = *offset;
    *offset += 4; // placeholder for ext_type + data length
    let list_len_pos = *offset;
    *offset += 2; // placeholder for client_shares length
    buf[*offset..*offset + 2].copy_from_slice(&key_share_group.to_wire());
    *offset += 2;
    put_slice_u16(buf, offset, key_share_public);
    let list_len = (*offset - list_len_pos - 2) as u16;
    buf[list_len_pos..list_len_pos + 2].copy_from_slice(&list_len.to_be_bytes());
    let ks_len = (*offset - ks_start - 4) as u16;
    buf[ks_start..ks_start + 2].copy_from_slice(&(ExtensionType::KeyShare as u16).to_be_bytes());
    buf[ks_start + 2..ks_start + 4].copy_from_slice(&ks_len.to_be_bytes());

    // 2. SupportedVersions (0x002B)
    let sv_start = *offset;
    *offset += 4;
    buf[*offset] = 2; // supported_versions length in bytes
    *offset += 1;
    buf[*offset..*offset + 2].copy_from_slice(&[0x03, 0x04]); // TLS 1.3 = 0x0304
    *offset += 2;
    let sv_len = (*offset - sv_start - 4) as u16;
    buf[sv_start..sv_start + 2].copy_from_slice(&(ExtensionType::SupportedVersions as u16).to_be_bytes());
    buf[sv_start + 2..sv_start + 4].copy_from_slice(&sv_len.to_be_bytes());

    // 3. SupportedGroups (NamedGroups)
    if !supported_groups.is_empty() {
        let sg_start = *offset;
        *offset += 4;
        let list_len_pos = *offset;
        *offset += 2;
        for g in supported_groups {
            buf[*offset..*offset + 2].copy_from_slice(&g.to_wire());
            *offset += 2;
        }
        let list_len = (*offset - list_len_pos - 2) as u16;
        buf[list_len_pos..list_len_pos + 2].copy_from_slice(&list_len.to_be_bytes());
        let sg_len = (*offset - sg_start - 4) as u16;
        buf[sg_start..sg_start + 2].copy_from_slice(&(ExtensionType::SupportedGroups as u16).to_be_bytes());
        buf[sg_start + 2..sg_start + 4].copy_from_slice(&sg_len.to_be_bytes());
    }

    // 4. SignatureAlgorithms
    if !signature_schemes.is_empty() {
        let sa_start = *offset;
        *offset += 4;
        let list_len_pos = *offset;
        *offset += 2;
        for s in signature_schemes {
            buf[*offset..*offset + 2].copy_from_slice(&s.to_wire());
            *offset += 2;
        }
        let list_len = (*offset - list_len_pos - 2) as u16;
        buf[list_len_pos..list_len_pos + 2].copy_from_slice(&list_len.to_be_bytes());
        let ext_len = (*offset - sa_start - 4) as u16;
        buf[sa_start..sa_start + 2].copy_from_slice(&(ExtensionType::SignatureAlgorithms as u16).to_be_bytes());
        buf[sa_start + 2..sa_start + 4].copy_from_slice(&ext_len.to_be_bytes());
    }

    // Actually the sig_algs building is getting complex with forward references.
    // Let me use a simpler approach: skip it for now, the handshake will work
    // without signature_algorithms extension for basic connections.

    // 5. ServerName (SNI)
    if let Some(name) = server_name {
        let sn_start = *offset;
        *offset += 4; // placeholder
        let name_bytes = name.as_bytes();
        // ServerNameList: length(2) + ServerName: type(1) + length(2) + name
        let host_len = name_bytes.len() as u16;
        let list_len = 3 + host_len; // type(1) + length(2) + name
        put_u16(buf, offset, list_len);
        buf[*offset] = 0; // host_name type
        *offset += 1;
        put_slice_u16(buf, offset, name_bytes);
        let sn_data_len = (*offset - sn_start - 4) as u16;
        buf[sn_start..sn_start + 2].copy_from_slice(&(ExtensionType::ServerName as u16).to_be_bytes());
        buf[sn_start + 2..sn_start + 4].copy_from_slice(&sn_data_len.to_be_bytes());
    }

    // 6. ALPN
    if !alpn_protocols.is_empty() {
        let alpn_start = *offset;
        *offset += 4;
        // ALPN: protocol_name_list length(2) + for each: name length(1) + name
        let alpn_body_start = *offset;
        *offset += 2;
        for p in alpn_protocols {
            put_slice_u8(buf, offset, p);
        }
        let alpn_list_len = (*offset - alpn_body_start - 2) as u16;
        buf[alpn_body_start..alpn_body_start + 2].copy_from_slice(&alpn_list_len.to_be_bytes());
        let alpn_data_len = (*offset - alpn_start - 4) as u16;
        buf[alpn_start..alpn_start + 2]
            .copy_from_slice(&(ExtensionType::ApplicationLayerProtocolNegotiation as u16).to_be_bytes());
        buf[alpn_start + 2..alpn_start + 4].copy_from_slice(&alpn_data_len.to_be_bytes());
    }

    // Fill total extensions length
    let ext_total = (*offset - ext_total_start - 2) as u16;
    buf[ext_total_start..ext_total_start + 2].copy_from_slice(&ext_total.to_be_bytes());

    // Fill handshake body length
    let body_len = (*offset - body_start - 4) as u32;
    let len_bytes = body_len.to_be_bytes();
    buf[len_pos..len_pos + 3].copy_from_slice(&len_bytes[1..]);

    Ok(*offset - body_start)
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

        if let Some(et) = ExtensionType::from_u16(ext_type) {
            if let ExtensionType::KeyShare = et {
                if ext_len < 4 {
                    return Err(Error::DecodeError);
                }
                let group_bytes = [ext_data[ext_off], ext_data[ext_off + 1]];
                key_share_group = Some(KeyExchangeGroup::from_wire(group_bytes).ok_or(Error::UnsupportedKeyExchangeGroup)?);
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

/// Decode EncryptedExtensions to extract the selected ALPN protocol.
pub fn decode_encrypted_extensions<'a>(body: &'a [u8]) -> Result<Option<&'a [u8]>, Error> {
    let mut off = 0;
    let ext_data = read_slice_u16(body, &mut off)?;
    let mut ext_off = 0;

    while ext_off + 4 <= ext_data.len() {
        let ext_type = read_u16(ext_data, &mut ext_off);
        let ext_len = read_u16(ext_data, &mut ext_off) as usize;
        if ext_off + ext_len > ext_data.len() {
            return Err(Error::DecodeError);
        }

        match ExtensionType::from_u16(ext_type) {
            Some(ExtensionType::ApplicationLayerProtocolNegotiation) => {
                // ALPN: protocol_name_list length(2) + for each: length(1) + name
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
                return Ok(Some(&alpn_body[3..3 + name_len]));
            }
            _ => {}
        }
        ext_off += ext_len;
    }
    Ok(None)
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
/// Decode Certificate message body, collecting cert DER slices.
/// Returns the first (end-entity) certificate's DER data.
const MAX_CERTS: usize = 6;
pub fn decode_certificate<'a>(body: &'a [u8]) -> Result<&'a [u8], Error> {
    let mut off = 0;

    // request_context (TLS 1.3: 1-byte length + context, usually empty)
    let ctx_len = read_u8(body, &mut off) as usize;
    off += ctx_len;

    // certificate_list: 3-byte length + entries
    if off + 3 > body.len() {
        return Err(Error::DecodeError);
    }
    let list_len = read_u24(body, &mut off) as usize;
    if off + list_len > body.len() {
        return Err(Error::DecodeError);
    }

    let list_end = off + list_len;
    let mut cert_count = 0;
    let mut end_entity_der = None;

    while off < list_end {
        if cert_count >= MAX_CERTS {
            break;
        }
        // cert_data: 3-byte length + DER
        let cert_len = read_u24(body, &mut off) as usize;
        if off + cert_len > list_end {
            return Err(Error::DecodeError);
        }
        let cert_der = &body[off..off + cert_len];
        off += cert_len;

        // extensions: 2-byte length (skip for now)
        let ext_len = read_u16(body, &mut off) as usize;
        off += ext_len;

        if end_entity_der.is_none() {
            end_entity_der = Some(cert_der);
        }
        cert_count += 1;
    }

    end_entity_der.ok_or(Error::DecodeError)
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

// ── Finished computation helpers ─────────────────────────────────────────

/// Compute the TLS 1.3 transcript hash up to current state.
/// Hash(prev_hash || handshake_msg) is computed by hashing the concatenation.
pub fn append_to_transcript(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    transcript: &mut [u8; 48],
    transcript_init: &mut bool,
    handshake_msg: &[u8],
) -> Result<(), Error> {
    let hash_size = suite.hash_size();

    if !*transcript_init {
        // Hash("") initial transcript
        provider.hash(suite, &[], &mut transcript[..hash_size])?;
        *transcript_init = true;
    }

    // Hash(transcript || message)
    // We need to concatenate transcript with the new message and hash.
    let mut combined = [0u8; 48 + 16384];
    combined[..hash_size].copy_from_slice(&transcript[..hash_size]);
    combined[hash_size..hash_size + handshake_msg.len()].copy_from_slice(handshake_msg);
    provider.hash(
        suite,
        &combined[..hash_size + handshake_msg.len()],
        &mut transcript[..hash_size],
    )?;

    Ok(())
}
