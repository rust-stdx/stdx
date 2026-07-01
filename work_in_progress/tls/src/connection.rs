use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::VecDeque,
    format,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use bytes::{BufMut, Bytes, BytesMut};
use heapless;

use crate::{
    ALPN_PROTOCOL_MAX_SIZE, Error, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE, MAX_ALPN_PROTOCOLS, MAX_HASH_SIZE,
    MAX_KEY_EXCHANGE_PAIRS, MAX_SERVER_NAME_LENGTH, PSK_MAX_SIZE, SHARED_SECRET_MAX_SIZE,
    config::{ClientConfig, ClientHello, ReceivedCertificate, ServerConfig},
    crypto::{CertType, CipherSuite, KeyExchangeGroup, SignatureScheme},
    error::{CertificateValidationFailure, HandshakeFailure},
    key_schedule::{KeySchedule, TlsKeys},
    message::*,
    record::{ContentType, RecordState},
};

/// Extract raw public key bytes from a SubjectPublicKeyInfo DER blob.
///
/// The SPKI DER contains a BIT STRING that holds the raw key bytes
/// (uncompressed point for P-256, 32-byte key for Ed25519, etc.).
/// This walks the DER structure to locate the BIT STRING and returns
/// its content (minus the leading unused-bits byte).
fn extract_key_from_spki<'a>(spki_der: &'a [u8]) -> Result<&'a [u8], Error> {
    let oid_pos = spki_der
        .iter()
        .position(|&b| b == 0x06)
        .ok_or_else(|| Error::DecodeError("no OID in SPKI".into()))?;
    let oid_len = spki_der[oid_pos + 1] as usize;
    let bitstring_start = oid_pos + 2 + oid_len;
    if bitstring_start >= spki_der.len() || spki_der[bitstring_start] != 0x03 {
        return Err(Error::DecodeError("no BIT STRING after AlgorithmIdentifier in SPKI".into()));
    }
    let pos = bitstring_start;
    let bitstring_len = spki_der[pos + 1] as usize;
    if bitstring_len < 2 || pos + 2 + bitstring_len > spki_der.len() {
        return Err(Error::DecodeError("BIT STRING length invalid".into()));
    }
    let key_start = pos + 3;
    let key_len = bitstring_len - 1;
    Ok(&spki_der[key_start..key_start + key_len])
}

#[derive(Clone)]
pub struct AlpnProtocol(heapless::Vec<u8, ALPN_PROTOCOL_MAX_SIZE>);

impl AlpnProtocol {
    #[inline]
    pub fn from_slice(protocol: &[u8]) -> Result<Self, Error> {
        let ret = heapless::Vec::from_slice(protocol)
            .map_err(|_| Error::InternalError("ALPN protocol is too long. max: 32 bytes".into()))?;
        Ok(AlpnProtocol(ret))
    }

    #[inline]
    pub fn from_static(protocol: &'static [u8]) -> Self {
        AlpnProtocol(heapless::Vec::from_slice(protocol).expect("ALPN protocol exceeds maximum length of 32 bytes"))
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl AsRef<[u8]> for AlpnProtocol {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

/// Common handshake state shared by client and server.
struct HandshakeState {
    cipher_suite: Option<CipherSuite>,
    key_exchange_group: KeyExchangeGroup,
    kx_pairs: heapless::Vec<Box<dyn crate::crypto::KeyExchangeKeyPair>, MAX_KEY_EXCHANGE_PAIRS>,
    peer_public_key: Option<Bytes>,
    shared_secret: Option<heapless::Vec<u8, SHARED_SECRET_MAX_SIZE>>,
    key_schedule: Option<KeySchedule>,
    keys: Option<TlsKeys>,
    transcript: Vec<u8>,
    write_record: RecordState,
    read_record: RecordState,
    read_buf: BytesMut,
    alpn_selected: Option<AlpnProtocol>,
    cert_chain: Option<Vec<Bytes>>,
    negotiated_cert_type: CertType,
    handshake_payload: BytesMut,
    server_hello_hash: heapless::Vec<u8, MAX_HASH_SIZE>,
    server_finished_hash: heapless::Vec<u8, MAX_HASH_SIZE>,
    write_queue: VecDeque<Bytes>,
    app_data_queue: VecDeque<Bytes>,
    handshake_done: bool,
    negotiated_version: u16,
    close_received: bool,
    signature_scheme: Option<SignatureScheme>,
    quic_write_queue: VecDeque<Bytes>,
    is_quic: bool,
    quic_transport_params: Option<Bytes>,

    // Current application traffic secrets (for KeyUpdate).
    client_app_traffic_secret: heapless::Vec<u8, MAX_HASH_SIZE>,
    server_app_traffic_secret: heapless::Vec<u8, MAX_HASH_SIZE>,
    pending_key_update_response: bool,
    psk: Option<heapless::Vec<u8, MAX_HASH_SIZE>>,
    certificate_request_received: bool,
}

#[cfg(feature = "zeroize")]
fn zeroize_heapless_vec_conn<const N: usize>(v: &mut heapless::Vec<u8, N>) {
    use zeroize::Zeroize;
    v.as_mut_slice().zeroize();
}

#[cfg(feature = "zeroize")]
impl Drop for HandshakeState {
    fn drop(&mut self) {
        if let Some(ref mut ss) = self.shared_secret {
            zeroize_heapless_vec_conn(ss);
        }
        zeroize_heapless_vec_conn(&mut self.server_hello_hash);
    }
}

impl HandshakeState {
    fn new() -> Self {
        Self::with_transcript_capacity(8192)
    }

    fn with_transcript_capacity(cap: usize) -> Self {
        Self {
            cipher_suite: None,
            key_exchange_group: KeyExchangeGroup::X25519,
            kx_pairs: heapless::Vec::new(),
            peer_public_key: None,
            shared_secret: None,
            key_schedule: None,
            keys: None,
            transcript: Vec::with_capacity(cap),
            write_record: RecordState::new(),
            read_record: RecordState::new(),
            read_buf: BytesMut::new(),
            alpn_selected: None,
            cert_chain: None,
            negotiated_cert_type: CertType::X509,
            handshake_payload: BytesMut::new(),
            server_hello_hash: heapless::Vec::new(),
            server_finished_hash: heapless::Vec::new(),
            write_queue: VecDeque::new(),
            app_data_queue: VecDeque::new(),
            handshake_done: false,
            negotiated_version: 0,
            close_received: false,
            signature_scheme: None,
            quic_write_queue: VecDeque::new(),
            is_quic: false,
            quic_transport_params: None,
            client_app_traffic_secret: heapless::Vec::new(),
            server_app_traffic_secret: heapless::Vec::new(),
            pending_key_update_response: false,
            psk: None,
            certificate_request_received: false,
        }
    }
}

// ── Client Connection ─────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum ClientState {
    SentClientHello,
    WaitEncryptedExtensions,
    WaitCertificate,
    WaitCertificateVerify,
    WaitFinished,
    Done,
    Failed,
}

/// A TLS 1.3 client connection (sans-IO).
pub struct ClientConnection {
    config: ClientConfig,
    state: ClientState,
    handshake_state: HandshakeState,
    server_name: Option<String>,
}

impl ClientConnection {
    /// Return the selected ALPN protocol, if any.
    pub fn alpn_protocol(&self) -> Option<&[u8]> {
        self.handshake_state.alpn_selected.as_ref().map(|alpn| alpn.as_ref())
    }

    /// Return the negotiated cipher suite, if the handshake has progressed far enough.
    pub fn cipher_suite(&self) -> Option<CipherSuite> {
        self.handshake_state.cipher_suite
    }

    /// Return the key exchange group in use.
    pub fn key_exchange_group(&self) -> KeyExchangeGroup {
        self.handshake_state.key_exchange_group
    }

    /// Return the server name (SNI) used for this connection.
    pub fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    /// Encrypt application data for sending.
    pub fn send(&mut self, data: &[u8]) -> Result<Bytes, Error> {
        if !matches!(self.state, ClientState::Done) {
            return Err(Error::InternalError("handshake not complete".into()));
        }
        self.handshake_state
            .write_record
            .encrypt_record(ContentType::ApplicationData, data)
    }

    /// Initiate a clean close.
    pub fn close(&mut self) -> Result<Bytes, Error> {
        self.handshake_state.write_record.encrypt_alert(1, 0)
    }

    /// Initiate a post-handshake key update (RFC 8446 §4.6.3).
    pub fn initiate_key_update(&mut self, request_update: bool) -> Result<Bytes, Error> {
        if !matches!(self.state, ClientState::Done) {
            return Err(Error::InternalError("handshake not complete".into()));
        }
        let ks = self.handshake_state.key_schedule.as_ref().unwrap();
        let (new_secret, new_key, new_iv) = ks.key_update_traffic(&self.handshake_state.client_app_traffic_secret);
        self.handshake_state.client_app_traffic_secret = new_secret;
        let aead = self
            .config
            .crypto
            .create_aead(self.handshake_state.cipher_suite.unwrap(), &new_key)?;
        self.handshake_state.write_record.set_write_keys(aead, new_iv);
        let ku = encode_key_update(request_update as u8);
        self.handshake_state
            .write_record
            .encrypt_record(ContentType::Handshake, &ku)
    }

    /// Feed received bytes into the internal buffer.
    pub fn inject(&mut self, input: &[u8]) {
        self.handshake_state.read_buf.extend_from_slice(input);
    }

    /// Take the next chunk of decrypted application data.
    pub fn read_app_data(&mut self) -> Option<Bytes> {
        self.handshake_state.app_data_queue.pop_front()
    }

    /// Take the next chunk of TLS bytes to send to the peer.
    pub fn write_tls(&mut self) -> Option<Bytes> {
        self.handshake_state.write_queue.pop_front()
    }

    /// Is the handshake complete?
    pub fn handshake_done(&self) -> bool {
        self.handshake_state.handshake_done
    }

    /// Has the peer sent a close_notify alert?
    pub fn close_notified(&self) -> bool {
        self.handshake_state.close_received
    }

    /// The negotiated TLS protocol version (e.g. `0x0304` for TLS 1.3).
    pub fn negotiated_version(&self) -> u16 {
        self.handshake_state.negotiated_version
    }

    /// The signature scheme used by the server's CertificateVerify message,
    /// if the handshake has progressed far enough.
    pub fn signature_scheme(&self) -> Option<SignatureScheme> {
        self.handshake_state.signature_scheme
    }

    /// Create a new client connection.
    ///
    /// `server_name` is the SNI hostname to send; `None` disables SNI.
    ///
    /// The initial ClientHello bytes are queued internally and can be drained
    /// via [`write_tls`].
    pub async fn new(config: ClientConfig, server_name: Option<String>) -> Result<Self, Error> {
        let transcript_cap = if config.cert_types == [CertType::RawPublicKey] {
            1600
        } else {
            8192
        };
        let mut hs = HandshakeState::with_transcript_capacity(transcript_cap);
        let crypto_provider = &config.crypto;

        let supported_groups = crypto_provider.supported_key_exchange_groups();
        let key_exchange_group = *supported_groups.first().ok_or(Error::NoKeyExchangeGroupInCommon)?;

        let mut kx_pairs: heapless::Vec<Box<dyn crate::crypto::KeyExchangeKeyPair>, MAX_KEY_EXCHANGE_PAIRS> =
            heapless::Vec::new();
        let mut key_share_entries: heapless::Vec<
            (KeyExchangeGroup, heapless::Vec<u8, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE>),
            MAX_KEY_EXCHANGE_PAIRS,
        > = heapless::Vec::new();

        // Generate key pairs for all groups; send all in key_share.
        for &group in supported_groups {
            let kp = crypto_provider.create_kx_pair(group)?;
            let pk = kp.public_key_bytes();
            key_share_entries
                .push((group, pk))
                .map_err(|_| Error::InternalError("Too many key exchange groups. Limit: 6".into()))?;
            kx_pairs
                .push(kp)
                .map_err(|_| Error::InternalError("Too many key exchange groups. Limit: 6".into()))?;
        }

        hs.key_exchange_group = key_exchange_group;
        hs.kx_pairs = kx_pairs;

        let mut key_share_refs: heapless::Vec<(KeyExchangeGroup, &[u8]), MAX_KEY_EXCHANGE_PAIRS> = heapless::Vec::new();
        for (g, pk) in &key_share_entries {
            key_share_refs
                .push((*g, pk.as_slice()))
                .map_err(|_| Error::InternalError("Too many key exchange groups. Limit: 6".into()))?;
        }

        let mut exts: heapless::Vec<Extension, 12> = [
            ext_supported_versions(),
            ext_psk_key_exchange_modes(),
            ext_supported_groups(&supported_groups),
            ext_key_share_client(key_share_refs.as_slice()),
            ext_signature_algorithms(crypto_provider.supported_signature_schemes()),
        ]
        .into();
        if let Some(ref name) = server_name {
            let _ = exts.push(ext_server_name(name));
        }
        if !config.alpn_protocols.is_empty() {
            let alpn_refs: heapless::Vec<&[u8], MAX_ALPN_PROTOCOLS> =
                config.alpn_protocols.iter().map(|p| p.as_ref()).collect();
            let _ = exts.push(ext_alpn(&alpn_refs[..]));
        }
        if config.cert_types != [CertType::X509] || config.cert_types.len() != 1 {
            let _ = exts.push(ext_server_cert_type_client(&config.cert_types));
        }
        let cipher_suites = crypto_provider.supported_cipher_suites();

        // PSK resumption
        let mut psk_for_key_schedule: Option<heapless::Vec<u8, MAX_HASH_SIZE>> = None;
        let mut random = [0u8; 32];
        if let Some(ref cache) = config.session_cache {
            if let Some(ref name) = server_name {
                if let Some((ticket, psk)) = cache.get(name).await {
                    let suite = cipher_suites[0];
                    let mut zeros: heapless::Vec<u8, MAX_HASH_SIZE> = heapless::Vec::new();
                    zeros.resize(suite.hash_size(), 0).unwrap();
                    let early_secret = crypto_provider.hkdf_extract(suite, &zeros, &psk);
                    let binder_key = crypto_provider.hkdf_expand_label(
                        suite,
                        &early_secret,
                        b"tls13 res binder",
                        &[],
                        suite.hash_size(),
                    );
                    let zero_binder = [0u8; 48];
                    let zero_binder_slice = &zero_binder[..suite.hash_size()];
                    let partial_psk_ext = ext_pre_shared_key(&[(ticket.as_slice(), 0)], &[zero_binder_slice]);
                    let mut partial_exts: heapless::Vec<Extension, 12> = heapless::Vec::new();
                    for ext in exts.iter() {
                        let _ = partial_exts.push(ext.clone());
                    }
                    let _ = partial_exts.push(partial_psk_ext);
                    crypto_provider.secure_random(&mut random);
                    let partial_ch = encode_client_hello(&random, &[], &cipher_suites, &partial_exts);
                    let ch_hash = crypto_provider.hash(suite, &partial_ch);
                    let binder = crypto_provider.hmac(suite, &binder_key, &ch_hash);
                    let final_psk_ext = ext_pre_shared_key(&[(ticket.as_slice(), 0)], &[binder.as_slice()]);
                    let _ = exts.push(final_psk_ext);
                    let mut psk_vec = heapless::Vec::new();
                    let _ = psk_vec.extend_from_slice(&psk);
                    psk_for_key_schedule = Some(psk_vec);
                }
            }
        }
        if psk_for_key_schedule.is_none() {
            crypto_provider.secure_random(&mut random);
        }

        let client_hello = encode_client_hello(&random, &[], &cipher_suites, &exts);
        hs.transcript.extend_from_slice(&client_hello);

        let mut record = BytesMut::with_capacity(5 + client_hello.len());
        record.put_u8(ContentType::Handshake as u8);
        record.put_u16(0x0301);
        record.put_u16(client_hello.len() as u16);
        record.extend_from_slice(&client_hello);

        hs.write_queue.push_back(record.freeze());
        hs.psk = psk_for_key_schedule;

        Ok(Self {
            config,
            state: ClientState::SentClientHello,
            handshake_state: hs,
            server_name,
        })
    }

    /// Create a new QUIC-mode client connection.
    ///
    /// The initial ClientHello includes the QUIC transport parameters extension.
    /// Raw handshake messages are returned via [`write_handshake`] (no TLS
    /// record wrapping). Input is fed via [`inject_handshake`].
    pub fn new_quic_with_preferred_group(
        config: ClientConfig,
        server_name: Option<String>,
        transport_params: &[u8],
        alpn: &[&[u8]],
        preferred_group: Option<KeyExchangeGroup>,
    ) -> Result<Self, Error> {
        let mut handshake_state = HandshakeState::new();
        let crypto_provider = &config.crypto;

        let supported_groups = crypto_provider.supported_key_exchange_groups();
        let key_exchange_group = if let Some(pref) = preferred_group {
            if supported_groups.contains(&pref) {
                pref
            } else {
                *supported_groups.first().ok_or(Error::NoKeyExchangeGroupInCommon)?
            }
        } else {
            *supported_groups.first().ok_or(Error::NoKeyExchangeGroupInCommon)?
        };

        let mut kx_pairs: heapless::Vec<Box<dyn crate::crypto::KeyExchangeKeyPair>, MAX_KEY_EXCHANGE_PAIRS> =
            heapless::Vec::new();
        let mut key_share_entries: heapless::Vec<
            (KeyExchangeGroup, heapless::Vec<u8, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE>),
            MAX_KEY_EXCHANGE_PAIRS,
        > = heapless::Vec::new();

        for &group in supported_groups {
            let kp = crypto_provider.create_kx_pair(group)?;
            let pk = kp.public_key_bytes();
            if group == key_exchange_group {
                key_share_entries
                    .push((group, pk))
                    .map_err(|_| Error::InternalError("Too many key exchange groups. Limit: 6".into()))?;
            }
            kx_pairs
                .push(kp)
                .map_err(|_| Error::InternalError("Too many key exchange groups. Limit: 6".into()))?;
        }

        handshake_state.key_exchange_group = key_exchange_group;
        handshake_state.kx_pairs = kx_pairs;

        let mut key_share_refs: heapless::Vec<(KeyExchangeGroup, &[u8]), MAX_KEY_EXCHANGE_PAIRS> = heapless::Vec::new();
        for (g, pk) in &key_share_entries {
            key_share_refs
                .push((*g, pk.as_slice()))
                .map_err(|_| Error::InternalError("Too many key exchange groups. Limit: 6".into()))?;
        }

        let mut exts: heapless::Vec<Extension, 12> = [
            ext_supported_versions(),
            ext_psk_key_exchange_modes(),
            ext_supported_groups(supported_groups),
            ext_key_share_client(key_share_refs.as_slice()),
            ext_signature_algorithms(crypto_provider.supported_signature_schemes()),
        ]
        .into();
        if let Some(ref name) = server_name {
            exts.push(ext_server_name(name))
                .map_err(|_| Error::InternalError("Too many extensions".into()))?;
        }
        exts.push(ext_alpn(alpn))
            .map_err(|_| Error::InternalError("Too many extensions".into()))?;
        exts.push(ext_quic_transport_parameters(transport_params))
            .map_err(|_| Error::InternalError("Too many extensions".into()))?;
        handshake_state.is_quic = true;
        handshake_state.quic_transport_params = Some(Bytes::copy_from_slice(transport_params));
        if config.cert_types != [CertType::X509] || config.cert_types.len() != 1 {
            let _ = exts.push(ext_server_cert_type_client(&config.cert_types));
        }
        let cipher_suites = crypto_provider.supported_cipher_suites();

        let mut random = [0u8; 32];
        crypto_provider.secure_random(&mut random);

        let client_hello = encode_client_hello(&random, &[], &cipher_suites, &exts);
        handshake_state.transcript.extend_from_slice(&client_hello);

        handshake_state.quic_write_queue.push_back(Bytes::from(client_hello));

        Ok(Self {
            config,
            state: ClientState::SentClientHello,
            handshake_state,
            server_name,
        })
    }

    /// Take the next raw handshake message to send (QUIC mode only).
    pub fn write_handshake(&mut self) -> Option<Bytes> {
        self.handshake_state.quic_write_queue.pop_front()
    }

    /// Inject raw handshake bytes (QUIC mode only).
    pub fn inject_handshake(&mut self, data: &[u8]) {
        self.handshake_state.handshake_payload.extend_from_slice(data);
    }

    /// Return the QUIC traffic secrets after the handshake completes.
    pub fn quic_secrets(&self) -> Option<crate::quic::QuicSecrets> {
        let ks = self.handshake_state.key_schedule.as_ref()?;
        let suite = self.handshake_state.cipher_suite?;
        let sh_hash = self.handshake_state.server_hello_hash.as_slice();
        let sfin_hash_bytes = self.config.crypto.hash(suite, &self.handshake_state.transcript);
        let sfin_hash = sfin_hash_bytes.as_slice();
        Some(crate::quic::extract_quic_secrets(ks, sh_hash, sfin_hash))
    }

    /// Advance the state machine. Call after [`inject`]ing data.
    pub async fn process(&mut self) -> Result<(), Error> {
        loop {
            let made_progress = match self.state {
                ClientState::SentClientHello => self.process_hello().await?,
                ClientState::WaitEncryptedExtensions => self.process_encrypted_extensions().await?,
                ClientState::WaitCertificate => self.process_certificate().await?,
                ClientState::WaitCertificateVerify => self.process_certificate_verify().await?,
                ClientState::WaitFinished => self.process_finished().await?,
                ClientState::Done => self.process_application_data().await?,
                ClientState::Failed => {
                    return Err(Error::HandshakeFailed(HandshakeFailure::Other(
                        "connection in failed state".into(),
                    )));
                }
            };
            if !made_progress || self.state == ClientState::Done {
                break;
            }
        }
        Ok(())
    }

    /// Process application data records synchronously (post-handshake only).
    ///
    /// Call after [`inject`] to decrypt pending encrypted records.
    /// Returns `true` if any application data was decrypted.
    pub async fn process_app_data(&mut self) -> Result<bool, Error> {
        if !matches!(self.state, ClientState::Done) {
            return Err(Error::InternalError("handshake not complete".into()));
        }
        self.process_application_data().await
    }

    fn try_read_record(&mut self) -> Result<Option<(ContentType, Bytes)>, Error> {
        if self.handshake_state.handshake_payload.len() >= 4 {
            let msg_len = u32::from_be_bytes([
                0,
                self.handshake_state.handshake_payload[1],
                self.handshake_state.handshake_payload[2],
                self.handshake_state.handshake_payload[3],
            ]) as usize;
            if self.handshake_state.handshake_payload.len() >= 4 + msg_len {
                let msg = self.handshake_state.handshake_payload.split_to(4 + msg_len);
                return Ok(Some((ContentType::Handshake, msg.freeze())));
            }
        }

        if self.handshake_state.read_buf.is_empty() {
            return Ok(None);
        }
        match self
            .handshake_state
            .read_record
            .decrypt_record(&mut self.handshake_state.read_buf)?
        {
            Some((ct, payload)) => {
                if ct == ContentType::Handshake {
                    self.handshake_state.handshake_payload.extend_from_slice(&payload);
                    return self.try_read_record();
                }
                Ok(Some((ct, payload)))
            }
            None => Ok(None),
        }
    }

    async fn process_hello(&mut self) -> Result<bool, Error> {
        let payload = loop {
            match self.try_read_record()? {
                Some((ContentType::ChangeCipherSpec, _)) => continue,
                Some((ContentType::Handshake, payload)) => break payload,
                None => return Ok(false),
                _ => {
                    return Err(Error::UnexpectedMessage {
                        expected: "ServerHello",
                        got: "non-handshake",
                    });
                }
            }
        };

        let sh = ServerHello::decode(payload)?;
        self.handshake_state.transcript.extend_from_slice(&sh.raw);
        self.handshake_state.cipher_suite = Some(sh.cipher_suite);

        // Extract negotiated version from supported_versions extension
        self.handshake_state.negotiated_version =
            parse_supported_versions(find_extension(&sh.extensions, ExtensionType::SupportedVersions)).unwrap_or(0);

        if !self
            .config
            .crypto
            .supported_cipher_suites()
            .iter()
            .any(|s| *s == sh.cipher_suite)
        {
            return Err(Error::NoCipherSuitesInCommon);
        }

        let ks_ext = find_extension(&sh.extensions, ExtensionType::KeyShare)
            .ok_or_else(|| Error::HandshakeFailed(HandshakeFailure::Other("no key_share in ServerHello".into())))?;

        if self.handshake_state.is_quic && ks_ext.data.len() == 2 {
            return self.handle_quic_hrr(&sh, ks_ext);
        }

        let (group, peer_pk) = parse_key_share_server(ks_ext)?;
        self.handshake_state.peer_public_key = Some(peer_pk.clone());

        let kx = self
            .handshake_state
            .kx_pairs
            .iter()
            .find(|kp| kp.group() == group)
            .ok_or_else(|| Error::InternalError("no kx_pair for negotiated group".into()))?;
        self.handshake_state.shared_secret = Some(kx.shared_secret(&peer_pk)?);

        let suite = sh.cipher_suite;
        let mut ks = KeySchedule::new(suite, Arc::clone(&self.config.crypto), self.handshake_state.psk.as_deref());
        ks.add_shared_secret(self.handshake_state.shared_secret.as_ref().unwrap());
        self.handshake_state.key_schedule = Some(ks);

        // In QUIC mode, the Handshake-level keys must be derived from the
        // transcript hash *immediately* after ServerHello processing, because
        // the QUIC layer calls quic_secrets() before EncryptedExtensions
        // arrives (it arrives in a separate Handshake CRYPTO frame).
        if self.handshake_state.is_quic {
            let transcript_hash = self
                .config
                .crypto
                .hash(sh.cipher_suite, &self.handshake_state.transcript);
            self.handshake_state.server_hello_hash = transcript_hash;
        }

        self.state = ClientState::WaitEncryptedExtensions;
        Ok(true)
    }

    fn handle_quic_hrr(&mut self, _sh: &ServerHello, ks_ext: &Extension) -> Result<bool, Error> {
        let hrr_group = KeyExchangeGroup::from_wire([ks_ext.data[0], ks_ext.data[1]])
            .ok_or_else(|| Error::DecodeError("unknown KX group in HRR".into()))?;

        let crypto_provider = &self.config.crypto;
        let supported_groups = crypto_provider.supported_key_exchange_groups();

        if !supported_groups.contains(&hrr_group) {
            return Err(Error::HandshakeFailed(HandshakeFailure::Other(
                format!("HRR requested group {hrr_group:?} which is not supported").into(),
            )));
        }

        self.handshake_state.key_exchange_group = hrr_group;
        self.handshake_state.kx_pairs.clear();

        let mut key_share_entries: heapless::Vec<
            (KeyExchangeGroup, heapless::Vec<u8, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE>),
            4,
        > = heapless::Vec::new();
        for &group in supported_groups {
            let kp = crypto_provider.create_kx_pair(group)?;
            let pk = kp.public_key_bytes();
            if group == hrr_group {
                let _ = key_share_entries.push((group, pk));
            }
            let _ = self.handshake_state.kx_pairs.push(kp);
        }

        let mut key_share_refs: heapless::Vec<(KeyExchangeGroup, &[u8]), 4> = heapless::Vec::new();
        for (g, pk) in &key_share_entries {
            let _ = key_share_refs.push((*g, pk.as_slice()));
        }

        let mut exts: heapless::Vec<Extension, 12> = [
            ext_supported_versions(),
            ext_psk_key_exchange_modes(),
            ext_supported_groups(supported_groups),
            ext_key_share_client(key_share_refs.as_slice()),
            ext_signature_algorithms(crypto_provider.supported_signature_schemes()),
        ]
        .into();
        if let Some(ref name) = self.server_name {
            let _ = exts.push(ext_server_name(name));
        }
        let alpn_refs: heapless::Vec<&[u8], 8> = self.config.alpn_protocols.iter().map(|p| p.as_ref()).collect();
        let _ = exts.push(ext_alpn(&alpn_refs[..]));
        if let Some(ref tp) = self.handshake_state.quic_transport_params {
            let _ = exts.push(ext_quic_transport_parameters(tp));
        }
        let cipher_suites = crypto_provider.supported_cipher_suites();

        let mut random = [0u8; 32];
        crypto_provider.secure_random(&mut random);

        let ch = encode_client_hello(&random, &[], &cipher_suites, &exts);
        self.handshake_state.transcript.extend_from_slice(&ch);

        self.handshake_state.quic_write_queue.push_back(ch);

        Ok(true)
    }

    fn setup_handshake_read_keys(&mut self) -> Result<(), Error> {
        let suite = self.handshake_state.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto;
        let ks = self.handshake_state.key_schedule.as_ref().unwrap();

        let transcript_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);
        self.handshake_state.server_hello_hash = transcript_hash.clone();

        let s_hs_traffic = ks.server_handshake_traffic_secret(&transcript_hash);
        let s_hs_key = crypto_provider.hkdf_expand_label(suite, &s_hs_traffic, b"tls13 key", &[], suite.key_size());
        let s_hs_iv: [u8; 12] = crypto_provider
            .hkdf_expand_label(suite, &s_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();

        self.handshake_state
            .read_record
            .set_read_keys(crypto_provider.create_aead(suite, &s_hs_key)?, s_hs_iv);
        Ok(())
    }

    async fn process_encrypted_extensions(&mut self) -> Result<bool, Error> {
        self.setup_handshake_read_keys()?;

        let payload = loop {
            match self.try_read_record()? {
                Some((ContentType::ChangeCipherSpec, _)) => continue,
                Some((ContentType::Handshake, payload)) => break payload,
                None => return Ok(false),
                _ => {
                    return Err(Error::UnexpectedMessage {
                        expected: "EncryptedExtensions",
                        got: "other",
                    });
                }
            }
        };
        let ee = EncryptedExtensions::decode(payload)?;
        self.handshake_state.transcript.extend_from_slice(&ee.raw);

        if let Some(ext) = find_extension(&ee.extensions, ExtensionType::ApplicationLayerProtocolNegotiation) {
            let alpn = parse_alpn(ext)?;
            self.handshake_state.alpn_selected = alpn.into_iter().next().and_then(|p| AlpnProtocol::from_slice(p).ok());
        }

        if let Some(ext) = find_extension(&ee.extensions, ExtensionType::ServerCertificateType) {
            self.handshake_state.negotiated_cert_type = parse_server_cert_type_ee(ext)?;
        }

        self.state = ClientState::WaitCertificate;
        Ok(true)
    }

    async fn process_certificate(&mut self) -> Result<bool, Error> {
        let payload = loop {
            match self.try_read_record()? {
                Some((ContentType::ChangeCipherSpec, _)) => continue,
                Some((ContentType::Handshake, payload)) => {
                    if payload.len() >= 4 && payload[0] == HandshakeType::CertificateRequest as u8 {
                        self.handshake_state.transcript.extend_from_slice(&payload);
                        self.handshake_state.certificate_request_received = true;
                        continue;
                    }
                    break payload;
                }
                None => return Ok(false),
                _ => {
                    return Err(Error::UnexpectedMessage {
                        expected: "Certificate",
                        got: "other",
                    });
                }
            }
        };
        let cert = Certificate::decode(payload)?;
        self.handshake_state.transcript.extend_from_slice(&cert.raw);
        self.handshake_state.cert_chain = {
            let mut chain = Vec::with_capacity(cert.entries.len());
            for entry in cert.entries {
                chain.push(entry.cert_data);
            }
            Some(chain)
        };

        self.state = ClientState::WaitCertificateVerify;
        Ok(true)
    }

    async fn process_certificate_verify(&mut self) -> Result<bool, Error> {
        let payload = loop {
            match self.try_read_record()? {
                Some((ContentType::ChangeCipherSpec, _)) => continue,
                Some((ContentType::Handshake, payload)) => break payload,
                None => return Ok(false),
                _ => {
                    return Err(Error::UnexpectedMessage {
                        expected: "CertificateVerify",
                        got: "non-handshake",
                    });
                }
            }
        };
        let cv = CertificateVerify::decode(payload)?;
        self.handshake_state.signature_scheme = Some(cv.scheme);
        let transcript_len_before = self.handshake_state.transcript.len();
        self.handshake_state.transcript.extend_from_slice(&cv.raw);

        let chain = self
            .handshake_state
            .cert_chain
            .take()
            .ok_or_else(|| Error::InternalError("no cert chain".into()))?;
        if chain.is_empty() {
            return Err(Error::DecodeError("empty certificate chain".into()));
        }

        let received = match self.handshake_state.negotiated_cert_type {
            CertType::X509 => ReceivedCertificate::X509 {
                chain,
                verify_scheme: cv.scheme,
            },
            CertType::RawPublicKey => {
                let pk = match chain.first() {
                    Some(spki_der) => {
                        let pk_bytes = extract_key_from_spki(spki_der)?;
                        let mut pk = heapless::Vec::new();
                        pk.extend_from_slice(&pk_bytes)
                            .map_err(|_| Error::InternalError("public key too large".into()))?;
                        pk
                    }
                    None => return Err(Error::DecodeError("empty raw public key".into())),
                };
                ReceivedCertificate::RawPublicKey {
                    public_key: pk,
                    scheme: cv.scheme,
                }
            }
        };

        self.config
            .cert_validator
            .validate(&received, self.server_name.as_deref())
            .await
            .map_err(|e| {
                Error::CertificateValidationFailed(CertificateValidationFailure::Other(e.to_string().into()))
            })?;

        let pk: Cow<'_, [u8]> = match &received {
            ReceivedCertificate::X509 {
                chain, ..
            } => {
                #[cfg(feature = "webpki-validator")]
                {
                    let spki_der = x509::extract_spki_from_cert(&chain[0])
                        .map_err(|e| Error::DecodeError(format!("X.509 parse: {e}").into()))?;
                    let key_bytes = x509::extract_key_from_spki(spki_der)
                        .map_err(|e| Error::DecodeError(format!("X.509 key: {e}").into()))?;
                    Cow::Borrowed(key_bytes)
                }
                #[cfg(not(feature = "webpki-validator"))]
                return Err(Error::InternalError(
                    "X.509 certificate support requires the 'webpki-validator' feature".into(),
                ));
            }
            ReceivedCertificate::RawPublicKey {
                public_key, ..
            } => Cow::Borrowed(public_key),
        };

        let suite = self.handshake_state.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto;
        let transcript_hash = crypto_provider.hash(suite, &self.handshake_state.transcript[..transcript_len_before]);

        let mut signed_data: heapless::Vec<u8, 200> = heapless::Vec::new();
        let _ = signed_data.extend_from_slice(&[0x20; 64]);
        let _ = signed_data.extend_from_slice(b"TLS 1.3, server CertificateVerify");
        let _ = signed_data.push(0);
        let _ = signed_data.extend_from_slice(&transcript_hash);

        crypto_provider.verify_signature(cv.scheme, &pk[..], &signed_data, &cv.signature)?;

        self.state = ClientState::WaitFinished;
        Ok(true)
    }

    async fn process_finished(&mut self) -> Result<bool, Error> {
        let payload = loop {
            match self.try_read_record()? {
                Some((ContentType::ChangeCipherSpec, _)) => continue,
                Some((ContentType::Handshake, payload)) => break payload,
                None => return Ok(false),
                _ => {
                    return Err(Error::UnexpectedMessage {
                        expected: "Finished",
                        got: "other",
                    });
                }
            }
        };
        let fin = Finished::decode(payload)?;

        let suite = self.handshake_state.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto;
        let ks = self.handshake_state.key_schedule.as_ref().unwrap();

        let transcript_hash_before_fin = crypto_provider.hash(suite, &self.handshake_state.transcript);

        self.handshake_state.transcript.extend_from_slice(&fin.raw);

        let transcript_hash_after_sfin = crypto_provider.hash(suite, &self.handshake_state.transcript);

        let keys = ks.derive_traffic_keys(&self.handshake_state.server_hello_hash, &transcript_hash_after_sfin);

        let sfk = &keys.server_finished_key;
        let expected = crypto_provider.hmac(suite, sfk, &transcript_hash_before_fin);
        if !constant_time_eq::constant_time_eq(&expected, &fin.verify_data) {
            return Err(crate::Error::HandshakeFailed(HandshakeFailure::Other(
                "finished verification failed".into(),
            )));
        }

        let our_fin_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);
        let our_verify_data_expected = crypto_provider.hmac(suite, &keys.client_finished_key, &our_fin_hash);
        let fin_msg = encode_finished(&our_verify_data_expected);

        if self.handshake_state.is_quic {
            self.handshake_state.quic_write_queue.push_back(Bytes::from(fin_msg));
        } else {
            let c_hs_traffic = ks.client_handshake_traffic_secret(&self.handshake_state.server_hello_hash);
            let c_hs_key = crypto_provider.hkdf_expand_label(suite, &c_hs_traffic, b"tls13 key", &[], suite.key_size());
            let c_hs_iv: [u8; 12] = crypto_provider
                .hkdf_expand_label(suite, &c_hs_traffic, b"tls13 iv", &[], 12)
                .as_slice()
                .try_into()
                .unwrap();
            self.handshake_state
                .write_record
                .set_write_keys(crypto_provider.create_aead(suite, &c_hs_key)?, c_hs_iv);

            let encrypted_fin = self
                .handshake_state
                .write_record
                .encrypt_record(ContentType::Handshake, &fin_msg)?;
            self.handshake_state.write_queue.push_back(encrypted_fin);
        }

        self.handshake_state.read_record.set_read_keys(
            crypto_provider.create_aead(suite, &keys.server_application_key)?,
            keys.server_application_iv,
        );
        self.handshake_state.write_record.set_write_keys(
            crypto_provider.create_aead(suite, &keys.client_application_key)?,
            keys.client_application_iv,
        );

        self.handshake_state.client_app_traffic_secret = keys.client_application_traffic_secret.clone();
        self.handshake_state.server_app_traffic_secret = keys.server_application_traffic_secret.clone();

        self.handshake_state.keys = Some(keys);
        self.handshake_state.handshake_done = true;
        self.state = ClientState::Done;
        Ok(true)
    }

    async fn process_application_data(&mut self) -> Result<bool, Error> {
        let mut processed_any = false;
        loop {
            match self.try_read_record()? {
                Some((ContentType::ApplicationData, payload)) => {
                    self.handshake_state.app_data_queue.push_back(payload);
                    processed_any = true;
                }
                Some((ContentType::Alert, payload)) if payload.len() >= 2 && payload[0] == 1 && payload[1] == 0 => {
                    self.handshake_state.close_received = true;
                    return Err(Error::ConnectionClosed);
                }
                Some((ContentType::Alert, _)) => return Err(Error::ConnectionClosed),
                Some((ContentType::Handshake, payload)) => {
                    let (ht, _body) = decode_handshake_header(&payload)?;
                    if ht == HandshakeType::KeyUpdate {
                        self.handle_key_update_client(&payload)?;
                        processed_any = true;
                    } else if ht == HandshakeType::NewSessionTicket {
                        self.handle_new_session_ticket(payload).await?;
                        processed_any = true;
                    }
                    continue;
                }
                Some(_) => continue,
                None => break,
            }
        }
        Ok(processed_any)
    }

    fn handle_key_update_client(&mut self, payload: &[u8]) -> Result<(), Error> {
        let request_update = decode_key_update(payload)?;
        let suite = self.handshake_state.cipher_suite.unwrap();
        let ks = self.handshake_state.key_schedule.as_ref().unwrap();
        let crypto_provider = &self.config.crypto;
        let (new_secret, new_key, new_iv) = ks.key_update_traffic(&self.handshake_state.server_app_traffic_secret);
        self.handshake_state.server_app_traffic_secret = new_secret;
        let aead = crypto_provider.create_aead(suite, &new_key)?;
        self.handshake_state.read_record.set_read_keys(aead, new_iv);
        if request_update == 1 && !self.handshake_state.pending_key_update_response {
            self.handshake_state.pending_key_update_response = true;
            let (new_ws, new_wk, new_wiv) = ks.key_update_traffic(&self.handshake_state.client_app_traffic_secret);
            self.handshake_state.client_app_traffic_secret = new_ws;
            let waead = crypto_provider.create_aead(suite, &new_wk)?;
            self.handshake_state.write_record.set_write_keys(waead, new_wiv);
            let ku = encode_key_update(0);
            let encrypted = self
                .handshake_state
                .write_record
                .encrypt_record(ContentType::Handshake, &ku)?;
            self.handshake_state.write_queue.push_back(encrypted);
        }
        Ok(())
    }

    async fn handle_new_session_ticket(&mut self, payload: Bytes) -> Result<(), Error> {
        let nst = NewSessionTicket::decode(payload)?;
        let suite = self.handshake_state.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto;
        let res_master = self
            .handshake_state
            .keys
            .as_ref()
            .map(|k| k.resumption_master_secret.clone())
            .ok_or_else(|| Error::InternalError("no keys for NST".into()))?;
        if res_master.is_empty() {
            return Ok(());
        }
        let psk = crypto_provider.hkdf_expand_label(
            suite,
            &res_master,
            b"tls13 resumption",
            &nst.ticket_nonce,
            suite.hash_size(),
        );
        if let Some(ref cache) = self.config.session_cache {
            cache
                .put(self.server_name.as_deref().unwrap_or(""), &nst.ticket, &psk)
                .await;
        }
        Ok(())
    }
}

// ── Server Connection ─────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum ServerState {
    WaitClientHello,
    WaitClientCertificate,
    WaitClientCertificateVerify,
    WaitClientFinished,
    Done,
    Failed,
}

/// A TLS 1.3 server connection (sans-IO).
pub struct ServerConnection {
    config: ServerConfig,
    state: ServerState,
    handshake_state: HandshakeState,
    /// Fingerprint result from [`TlsFingerprinter`], if configured.
    pub fingerprint: Option<[u8; 64]>,
}

impl ServerConnection {
    /// Encrypt application data for sending.
    pub fn send(&mut self, data: &[u8]) -> Result<Bytes, Error> {
        if !matches!(self.state, ServerState::Done) {
            return Err(Error::InternalError("handshake not complete".into()));
        }
        self.handshake_state
            .write_record
            .encrypt_record(ContentType::ApplicationData, data)
    }

    /// Initiate a clean close.
    pub fn close(&mut self) -> Result<Bytes, Error> {
        self.handshake_state.write_record.encrypt_alert(1, 0)
    }

    /// Return the selected ALPN protocol, if any.
    pub fn alpn_protocol(&self) -> Option<&[u8]> {
        self.handshake_state.alpn_selected.as_ref().map(|alpn| alpn.as_ref())
    }

    /// Initiate a post-handshake key update (RFC 8446 §4.6.3).
    pub fn initiate_key_update(&mut self, request_update: bool) -> Result<Bytes, Error> {
        if !matches!(self.state, ServerState::Done) {
            return Err(Error::InternalError("handshake not complete".into()));
        }
        let ks = self.handshake_state.key_schedule.as_ref().unwrap();
        let (new_secret, new_key, new_iv) = ks.key_update_traffic(&self.handshake_state.server_app_traffic_secret);
        self.handshake_state.server_app_traffic_secret = new_secret;
        let aead = self
            .config
            .crypto_provider
            .create_aead(self.handshake_state.cipher_suite.unwrap(), &new_key)?;
        self.handshake_state.write_record.set_write_keys(aead, new_iv);
        let ku = encode_key_update(request_update as u8);
        self.handshake_state
            .write_record
            .encrypt_record(ContentType::Handshake, &ku)
    }

    /// Feed received bytes into the internal buffer.
    pub fn inject(&mut self, input: &[u8]) {
        self.handshake_state.read_buf.extend_from_slice(input);
    }

    /// Take the next chunk of decrypted application data.
    pub fn read_app_data(&mut self) -> Option<Bytes> {
        self.handshake_state.app_data_queue.pop_front()
    }

    /// Take the next chunk of TLS bytes to send to the peer.
    pub fn write_tls(&mut self) -> Option<Bytes> {
        self.handshake_state.write_queue.pop_front()
    }

    /// Is the handshake complete?
    pub fn handshake_done(&self) -> bool {
        self.handshake_state.handshake_done
    }

    /// Has the peer sent a close_notify alert?
    pub fn close_notified(&self) -> bool {
        self.handshake_state.close_received
    }

    /// The negotiated TLS protocol version (e.g. `0x0304` for TLS 1.3).
    pub fn negotiated_version(&self) -> u16 {
        self.handshake_state.negotiated_version
    }

    /// Create a new server connection, ready to receive a ClientHello.
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            state: ServerState::WaitClientHello,
            handshake_state: HandshakeState::new(),
            fingerprint: None,
        }
    }

    /// Create a new QUIC-mode server connection.
    ///
    /// In QUIC mode, handshake messages use raw CRYPTO frames instead of TLS
    /// record framing. Call [`write_handshake`] to get outgoing bytes and
    /// [`inject_handshake`] to feed incoming bytes.
    pub fn new_quic(config: ServerConfig) -> Self {
        let mut handshake_state = HandshakeState::new();
        handshake_state.is_quic = true;
        Self {
            config,
            state: ServerState::WaitClientHello,
            handshake_state,
            fingerprint: None,
        }
    }

    /// Take the next raw handshake message to send (QUIC mode only).
    pub fn write_handshake(&mut self) -> Option<Bytes> {
        self.handshake_state.quic_write_queue.pop_front()
    }

    /// Inject raw handshake bytes (QUIC mode only).
    pub fn inject_handshake(&mut self, data: &[u8]) {
        self.handshake_state.handshake_payload.extend_from_slice(data);
    }

    /// Set the QUIC transport parameters to include in EncryptedExtensions.
    ///
    /// Must be called before [`process`] is invoked, or the parameters will
    /// not be sent.
    pub fn set_quic_transport_params(&mut self, params: &[u8]) {
        self.handshake_state.quic_transport_params = Some(Bytes::copy_from_slice(params));
    }

    /// Return the QUIC traffic secrets after the handshake completes.
    pub fn quic_secrets(&self) -> Option<crate::quic::QuicSecrets> {
        let ks = self.handshake_state.key_schedule.as_ref()?;
        // let suite = self.hs.cipher_suite?;
        // let provider = &self.config.provider;
        let sh_hash = self.handshake_state.server_hello_hash.as_slice();
        let sfin_hash = self.handshake_state.server_finished_hash.as_slice();
        Some(crate::quic::extract_quic_secrets(ks, sh_hash, sfin_hash))
    }

    /// Advance the state machine. Call after [`inject`]ing data.
    pub async fn process(&mut self) -> Result<(), Error> {
        loop {
            let made_progress = match self.state {
                ServerState::WaitClientHello => self.process_client_hello().await?,
                ServerState::WaitClientCertificate => self.process_client_certificate()?,
                ServerState::WaitClientCertificateVerify => self.process_client_certificate_verify()?,
                ServerState::WaitClientFinished => self.process_client_finished().await?,
                ServerState::Done => self.process_application_data().await?,
                ServerState::Failed => {
                    return Err(Error::HandshakeFailed(HandshakeFailure::Other(
                        "connection in failed state".into(),
                    )));
                }
            };
            if !made_progress || self.state == ServerState::Done {
                break;
            }
        }
        Ok(())
    }

    /// Process application data records synchronously (post-handshake only).
    ///
    /// Call after [`inject`] to decrypt pending encrypted records.
    /// Returns `true` if any application data was decrypted.
    pub async fn process_app_data(&mut self) -> Result<bool, Error> {
        if !matches!(self.state, ServerState::Done) {
            return Err(Error::InternalError("handshake not complete".into()));
        }
        self.process_application_data().await
    }

    fn try_read_record(&mut self) -> Result<Option<(ContentType, Bytes)>, Error> {
        if self.handshake_state.handshake_payload.len() >= 4 {
            let msg_len = u32::from_be_bytes([
                0,
                self.handshake_state.handshake_payload[1],
                self.handshake_state.handshake_payload[2],
                self.handshake_state.handshake_payload[3],
            ]) as usize;
            if self.handshake_state.handshake_payload.len() >= 4 + msg_len {
                let msg = self.handshake_state.handshake_payload.split_to(4 + msg_len);
                return Ok(Some((ContentType::Handshake, msg.freeze())));
            }
        }

        if self.handshake_state.read_buf.is_empty() {
            return Ok(None);
        }
        match self
            .handshake_state
            .read_record
            .decrypt_record(&mut self.handshake_state.read_buf)?
        {
            Some((ct, payload)) => Ok(Some((ct, payload))),
            None => Ok(None),
        }
    }
    async fn try_resolve_psk(&self, ch: &ClientHelloMsg) -> Option<heapless::Vec<u8, PSK_MAX_SIZE>> {
        let psk_ext = find_extension(&ch.extensions, ExtensionType::PreSharedKey)?;
        let data = psk_ext.data.as_ref();
        if data.len() < 2 {
            return None;
        }
        let identity_list_len = u16::from_be_bytes([data[0], data[1]]) as usize;
        if data.len() < 2 + identity_list_len {
            return None;
        }
        let identities = &data[2..2 + identity_list_len];
        let mut offset = 0;
        if identities.len() < 2 {
            return None;
        }
        let id_len = u16::from_be_bytes([identities[0], identities[1]]) as usize;
        offset += 2;
        if identities.len() < offset + id_len + 4 {
            return None;
        }
        let ticket = &identities[offset..offset + id_len];
        let store = self.config.session_tickets.as_ref()?;
        store.get_psk(ticket).await
    }

    async fn process_client_hello(&mut self) -> Result<bool, Error> {
        let payload = match self.try_read_record()? {
            Some((ContentType::Handshake, payload)) => payload,
            None => return Ok(false),
            _ => {
                return Err(Error::UnexpectedMessage {
                    expected: "ClientHello",
                    got: "non-handshake",
                });
            }
        };

        let ch = ClientHelloMsg::decode(payload)?;
        self.handshake_state.transcript.extend_from_slice(&ch.raw);

        let sv_ext = find_extension(&ch.extensions, ExtensionType::SupportedVersions);
        if !check_supported_versions(sv_ext) {
            return Err(Error::HandshakeFailed(HandshakeFailure::Other("TLS 1.3 not offered".into())));
        }
        self.handshake_state.negotiated_version = parse_supported_versions(sv_ext).unwrap_or(0x0304);

        let crypto_provider = &self.config.crypto_provider;

        let suite = crypto_provider
            .supported_cipher_suites()
            .iter()
            .copied()
            .find(|s| ch.cipher_suites.contains(s))
            .ok_or(Error::NoCipherSuitesInCommon)?;
        self.handshake_state.cipher_suite = Some(suite);

        let ks_ext = find_extension(&ch.extensions, ExtensionType::KeyShare)
            .ok_or_else(|| Error::HandshakeFailed(HandshakeFailure::Other("no key_share in ClientHello".into())))?;
        let (group, peer_pk) = parse_key_share(ks_ext)?;
        self.handshake_state.peer_public_key = Some(peer_pk.clone());

        let mut kx_pair = crypto_provider.create_kx_pair(group)?;
        kx_pair.set_peer_public_key(&peer_pk)?;
        let kx_pub = kx_pair.public_key_bytes();
        let shared = kx_pair.shared_secret(self.handshake_state.peer_public_key.as_ref().unwrap())?;
        self.handshake_state.shared_secret = Some(shared);
        let _ = self.handshake_state.kx_pairs.push(kx_pair);

        let psk_resolved = self.try_resolve_psk(&ch).await;
        let mut ks = KeySchedule::new(suite, Arc::clone(crypto_provider), psk_resolved.as_deref());
        ks.add_shared_secret(self.handshake_state.shared_secret.as_ref().unwrap());
        self.handshake_state.key_schedule = Some(ks);

        let alpn_protos = find_extension(&ch.extensions, ExtensionType::ApplicationLayerProtocolNegotiation)
            .and_then(|e| parse_alpn(e).ok())
            .unwrap_or_default();
        let server_name_str = find_extension(&ch.extensions, ExtensionType::ServerName).and_then(|e| {
            let d = &e.data[..];
            if d.len() < 5 {
                return None;
            }
            let name_type = d[2];
            if name_type != 0 {
                return None;
            }
            let name_len = u16::from_be_bytes([d[3], d[4]]) as usize;
            if d.len() < 5 + name_len {
                return None;
            }
            let mut name = heapless::String::<MAX_SERVER_NAME_LENGTH>::new();
            core::str::from_utf8(&d[5..5 + name_len])
                .ok()
                .and_then(|s| name.push_str(s).ok().map(|_| name))
        });
        let sig_schemes = find_extension(&ch.extensions, ExtensionType::SignatureAlgorithms)
            .map(|e| parse_signature_algorithms(e))
            .transpose()?
            .unwrap_or_default();

        let client_cert_types = find_extension(&ch.extensions, ExtensionType::ServerCertificateType)
            .map(|e| parse_server_cert_type_ch(e))
            .transpose()?
            .unwrap_or_else(|| [CertType::X509].into());

        if let Some(ref fp) = self.config.fingerprinter {
            self.fingerprint = Some(fp.fingerprint(&ch.raw).await?);
        }

        let client_hello = ClientHello {
            server_name: server_name_str.as_deref(),
            alpn_protocols: &alpn_protos,
            cipher_suites: &ch.cipher_suites,
            key_exchange_group: group,
            sig_schemes: &sig_schemes,
            raw: &ch.raw,
        };

        let cert = self.config.cert_provider.provide(&client_hello).await?;

        if !sig_schemes.contains(&cert.scheme) {
            return Err(Error::HandshakeFailed(HandshakeFailure::Other(
                format!(
                    "CertificateProvider selected scheme {:?} which was not offered by client",
                    cert.scheme
                )
                .into(),
            )));
        }

        let selected_cert_type = if client_cert_types.contains(&CertType::RawPublicKey) {
            CertType::RawPublicKey
        } else {
            CertType::X509
        };

        let mut random = [0u8; 32];
        crypto_provider.secure_random(&mut random);

        let mut server_hello_exts: heapless::Vec<Extension, 6> =
            [ext_supported_versions_server(), ext_key_share_server(&kx_pub, group)].into();
        if psk_resolved.is_some() {
            let _ = server_hello_exts.push(ext_pre_shared_key_server(0));
        }
        let selected_alpn = client_hello
            .alpn_protocols
            .iter()
            .find(|p| self.config.alpn_protocols.iter().any(|a| a.as_ref() == **p))
            .copied();
        if let Some(protocol) = selected_alpn {
            let protocol = AlpnProtocol::from_slice(protocol)?;
            let _ = server_hello_exts.push(ext_alpn(&[protocol.as_slice()]));
            self.handshake_state.alpn_selected = Some(protocol);
        }

        let sh = encode_server_hello(&random, &ch.session_id, suite, &server_hello_exts);
        self.handshake_state.transcript.extend_from_slice(&sh);

        let transcript_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);
        self.handshake_state.server_hello_hash = transcript_hash.clone();

        let ks_ref = self.handshake_state.key_schedule.as_ref().unwrap();
        let c_hs_traffic = ks_ref.client_handshake_traffic_secret(&transcript_hash);
        let s_hs_traffic = ks_ref.server_handshake_traffic_secret(&transcript_hash);
        let s_hs_key = crypto_provider.hkdf_expand_label(suite, &s_hs_traffic, b"tls13 key", &[], suite.key_size());
        let s_hs_iv: [u8; 12] = crypto_provider
            .hkdf_expand_label(suite, &s_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();
        let c_hs_key = crypto_provider.hkdf_expand_label(suite, &c_hs_traffic, b"tls13 key", &[], suite.key_size());
        let c_hs_iv: [u8; 12] = crypto_provider
            .hkdf_expand_label(suite, &c_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();

        self.handshake_state
            .write_record
            .set_write_keys(crypto_provider.create_aead(suite, &s_hs_key)?, s_hs_iv);

        // 1) ServerHello (plaintext handshake record / QUIC CRYPTO frame)
        if self.handshake_state.is_quic {
            self.handshake_state.quic_write_queue.push_back(sh.clone());
        } else {
            let mut sh_record = BytesMut::with_capacity(5 + sh.len());
            sh_record.put_u8(ContentType::Handshake as u8);
            sh_record.put_u16(0x0303);
            sh_record.put_u16(sh.len() as u16);
            sh_record.extend_from_slice(&sh);
            self.handshake_state.write_queue.push_back(sh_record.freeze());
        }

        // 2) EncryptedExtensions
        let mut ee_exts: heapless::Vec<Extension, 2> = heapless::Vec::new();
        if client_cert_types.contains(&CertType::RawPublicKey) || client_cert_types.contains(&CertType::X509) {
            let _ = ee_exts.push(ext_server_cert_type_server(selected_cert_type));
        }
        let ee = encode_encrypted_extensions(&ee_exts);
        self.handshake_state.transcript.extend_from_slice(&ee);
        if self.handshake_state.is_quic {
            self.handshake_state
                .quic_write_queue
                .push_back(Bytes::copy_from_slice(&ee));
        } else {
            self.handshake_state.write_queue.push_back(
                self.handshake_state
                    .write_record
                    .encrypt_record(ContentType::Handshake, &ee)?,
            );
        }

        // 3) CertificateRequest (optional — mutual TLS)
        if self.config.require_client_auth {
            let cert_req = encode_certificate_request(&[], &sig_schemes);
            self.handshake_state.transcript.extend_from_slice(&cert_req);
            if self.handshake_state.is_quic {
                self.handshake_state
                    .quic_write_queue
                    .push_back(Bytes::copy_from_slice(&cert_req));
            } else {
                self.handshake_state.write_queue.push_back(
                    self.handshake_state
                        .write_record
                        .encrypt_record(ContentType::Handshake, &cert_req)?,
                );
            }
        }

        // 4) Certificate
        let (public_key, signer) = (&cert.payload.public_key, &cert.payload.signer);
        let cert_msg = encode_certificate_raw_public_key(&[], public_key, &[]);
        self.handshake_state.transcript.extend_from_slice(&cert_msg);
        if self.handshake_state.is_quic {
            self.handshake_state
                .quic_write_queue
                .push_back(Bytes::copy_from_slice(&cert_msg));
        } else {
            self.handshake_state.write_queue.push_back(
                self.handshake_state
                    .write_record
                    .encrypt_record(ContentType::Handshake, &cert_msg)?,
            );
        }

        // 5) CertificateVerify
        let cv_transcript_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);
        let mut signed_data: heapless::Vec<u8, 200> = heapless::Vec::new();
        let _ = signed_data.extend_from_slice(&[0x20; 64]);
        let _ = signed_data.extend_from_slice(b"TLS 1.3, server CertificateVerify");
        let _ = signed_data.push(0);
        let _ = signed_data.extend_from_slice(&cv_transcript_hash);
        let signature = signer.sign(&signed_data)?;

        let cv_msg = encode_certificate_verify(cert.scheme, &signature);
        self.handshake_state.transcript.extend_from_slice(&cv_msg);
        if self.handshake_state.is_quic {
            self.handshake_state
                .quic_write_queue
                .push_back(Bytes::copy_from_slice(&cv_msg));
        } else {
            self.handshake_state.write_queue.push_back(
                self.handshake_state
                    .write_record
                    .encrypt_record(ContentType::Handshake, &cv_msg)?,
            );
        }

        // 6) Server Finished
        let s_hs_traffic_for_fin = ks_ref.server_handshake_traffic_secret(&self.handshake_state.server_hello_hash);
        let s_fin_key =
            crypto_provider.hkdf_expand_label(suite, &s_hs_traffic_for_fin, b"tls13 finished", &[], suite.hash_size());

        let fin_transcript_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);

        let verify_data = crypto_provider.hmac(suite, &s_fin_key, &fin_transcript_hash);
        let fin_msg = encode_finished(&verify_data);
        self.handshake_state.transcript.extend_from_slice(&fin_msg);
        if self.handshake_state.is_quic {
            self.handshake_state
                .quic_write_queue
                .push_back(Bytes::copy_from_slice(&fin_msg));
        } else {
            self.handshake_state.write_queue.push_back(
                self.handshake_state
                    .write_record
                    .encrypt_record(ContentType::Handshake, &fin_msg)?,
            );
        }

        let post_sfin_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);

        let keys = ks_ref.derive_traffic_keys(&self.handshake_state.server_hello_hash, &post_sfin_hash);

        self.handshake_state
            .read_record
            .set_read_keys(crypto_provider.create_aead(suite, &c_hs_key)?, c_hs_iv);

        self.handshake_state.client_app_traffic_secret = keys.client_application_traffic_secret.clone();
        self.handshake_state.server_app_traffic_secret = keys.server_application_traffic_secret.clone();

        self.handshake_state.keys = Some(keys);
        self.state = if self.config.require_client_auth {
            ServerState::WaitClientCertificate
        } else {
            ServerState::WaitClientFinished
        };
        Ok(true)
    }

    fn process_client_certificate(&mut self) -> Result<bool, Error> {
        let payload = match self.try_read_record()? {
            Some((ContentType::Handshake, payload)) => payload,
            None => return Ok(false),
            _ => {
                return Err(Error::UnexpectedMessage {
                    expected: "Certificate",
                    got: "other",
                });
            }
        };
        let cert = Certificate::decode(payload)?;
        self.handshake_state.transcript.extend_from_slice(&cert.raw);
        self.handshake_state.cert_chain = {
            let mut chain = Vec::with_capacity(cert.entries.len());
            for e in cert.entries {
                chain.push(e.cert_data);
            }
            Some(chain)
        };
        self.state = ServerState::WaitClientCertificateVerify;
        Ok(true)
    }

    fn process_client_certificate_verify(&mut self) -> Result<bool, Error> {
        let payload = match self.try_read_record()? {
            Some((ContentType::Handshake, payload)) => payload,
            None => return Ok(false),
            _ => {
                return Err(Error::UnexpectedMessage {
                    expected: "CertificateVerify",
                    got: "other",
                });
            }
        };
        let cv = CertificateVerify::decode(payload)?;
        let chain = self
            .handshake_state
            .cert_chain
            .take()
            .ok_or_else(|| Error::InternalError("no client cert chain".into()))?;
        if chain.is_empty() {
            return Err(Error::DecodeError("empty client certificate chain".into()));
        }
        let spki_der = x509::extract_spki_from_cert(&chain[0])
            .map_err(|e| Error::DecodeError(format!("X.509 parse: {e}").into()))?;
        let pk =
            x509::extract_key_from_spki(spki_der).map_err(|e| Error::DecodeError(format!("X.509 key: {e}").into()))?;
        let transcript_len_before = self.handshake_state.transcript.len();
        self.handshake_state.transcript.extend_from_slice(&cv.raw);
        let suite = self.handshake_state.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto_provider;
        let transcript_hash = crypto_provider.hash(suite, &self.handshake_state.transcript[..transcript_len_before]);
        let mut signed_data: heapless::Vec<u8, 200> = heapless::Vec::new();
        let _ = signed_data.extend_from_slice(&[0x20; 64]);
        let _ = signed_data.extend_from_slice(b"TLS 1.3, client CertificateVerify");
        let _ = signed_data.push(0);
        let _ = signed_data.extend_from_slice(&transcript_hash);
        crypto_provider.verify_signature(cv.scheme, pk, &signed_data, &cv.signature)?;
        self.state = ServerState::WaitClientFinished;
        Ok(true)
    }

    async fn process_client_finished(&mut self) -> Result<bool, Error> {
        let payload = match self.try_read_record()? {
            Some((_ct @ ContentType::Handshake, payload)) => payload,
            None => return Ok(false),
            _ => {
                return Err(Error::UnexpectedMessage {
                    expected: "Finished",
                    got: "other",
                });
            }
        };

        let fin = Finished::decode(payload)?;

        // Capture the server_finished_transcript hash before extending with
        // the Client Finished — quic_secrets() needs this for deriving
        // application traffic secrets (RFC 9001 §4.1).
        let suite = self.handshake_state.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto_provider;
        let sfin_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);
        self.handshake_state.server_finished_hash.clear();
        self.handshake_state
            .server_finished_hash
            .extend_from_slice(&sfin_hash)
            .unwrap();

        self.handshake_state.transcript.extend_from_slice(&fin.raw);

        let suite = self.handshake_state.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto_provider;
        let ks = self.handshake_state.key_schedule.as_ref().unwrap();
        let keys = self
            .handshake_state
            .keys
            .as_ref()
            .ok_or_else(|| Error::InternalError("no keys".into()))?;

        let transcript_hash = crypto_provider.hash(
            suite,
            &self.handshake_state.transcript[..self.handshake_state.transcript.len() - fin.raw.len()],
        );

        ks.verify_finished(&keys.client_finished_key, &transcript_hash, &fin.verify_data)?;

        self.handshake_state.read_record.set_read_keys(
            crypto_provider.create_aead(suite, &keys.client_application_key)?,
            keys.client_application_iv,
        );
        self.handshake_state.write_record.set_write_keys(
            crypto_provider.create_aead(suite, &keys.server_application_key)?,
            keys.server_application_iv,
        );

        // Derive the resumption_master_secret now that the client's Finished is in the transcript.
        let client_fin_hash = crypto_provider.hash(suite, &self.handshake_state.transcript);
        let res_master = ks.derive_resumption_secret(&client_fin_hash);
        let res_master_clone = res_master.clone();
        self.handshake_state.keys.as_mut().unwrap().resumption_master_secret = res_master;

        // Send a NewSessionTicket if a ticket store is configured.
        if let Some(ref store) = self.config.session_tickets {
            let ticket_lifetime = 86400u32;
            let mut ticket_age_add = [0u8; 4];
            crypto_provider.secure_random(&mut ticket_age_add);
            let mut ticket_nonce = [0u8; 8];
            crypto_provider.secure_random(&mut ticket_nonce);
            let mut ticket = [0u8; 32];
            crypto_provider.secure_random(&mut ticket);
            let psk = crypto_provider.hkdf_expand_label(
                suite,
                &res_master_clone,
                b"tls13 resumption",
                &ticket_nonce,
                suite.hash_size(),
            );
            store.put_ticket("", &ticket, psk, ticket_lifetime).await;
            let nst = encode_new_session_ticket(
                ticket_lifetime,
                u32::from_be_bytes(ticket_age_add),
                &ticket_nonce,
                &ticket,
                &[],
            );
            if self.handshake_state.is_quic {
                self.handshake_state
                    .quic_write_queue
                    .push_back(Bytes::copy_from_slice(&nst));
            } else {
                self.handshake_state.write_queue.push_back(
                    self.handshake_state
                        .write_record
                        .encrypt_record(ContentType::Handshake, &nst)?,
                );
            }
        }

        self.handshake_state.handshake_done = true;
        self.state = ServerState::Done;
        Ok(true)
    }

    async fn process_application_data(&mut self) -> Result<bool, Error> {
        let mut processed_any = false;
        loop {
            match self.try_read_record()? {
                Some((ContentType::ApplicationData, payload)) => {
                    self.handshake_state.app_data_queue.push_back(payload);
                    processed_any = true;
                }
                Some((ContentType::Alert, payload)) if payload.len() >= 2 && payload[0] == 1 && payload[1] == 0 => {
                    self.handshake_state.close_received = true;
                    return Err(Error::ConnectionClosed);
                }
                Some((ContentType::Alert, _)) => return Err(Error::ConnectionClosed),
                Some((ContentType::Handshake, payload)) => {
                    let (ht, _body) = decode_handshake_header(&payload)?;
                    if ht == HandshakeType::KeyUpdate {
                        self.handle_key_update_server(&payload)?;
                        processed_any = true;
                    }
                    continue;
                }
                Some(_) => continue,
                None => break,
            }
        }
        Ok(processed_any)
    }

    fn handle_key_update_server(&mut self, payload: &[u8]) -> Result<(), Error> {
        let request_update = decode_key_update(payload)?;
        let suite = self.handshake_state.cipher_suite.unwrap();
        let ks = self.handshake_state.key_schedule.as_ref().unwrap();
        let crypto_provider = &self.config.crypto_provider;
        let (new_secret, new_key, new_iv) = ks.key_update_traffic(&self.handshake_state.client_app_traffic_secret);
        self.handshake_state.client_app_traffic_secret = new_secret;
        let aead = crypto_provider.create_aead(suite, &new_key)?;
        self.handshake_state.read_record.set_read_keys(aead, new_iv);
        if request_update == 1 && !self.handshake_state.pending_key_update_response {
            self.handshake_state.pending_key_update_response = true;
            let (new_ws, new_wk, new_wiv) = ks.key_update_traffic(&self.handshake_state.server_app_traffic_secret);
            self.handshake_state.server_app_traffic_secret = new_ws;
            let waead = crypto_provider.create_aead(suite, &new_wk)?;
            self.handshake_state.write_record.set_write_keys(waead, new_wiv);
            let ku = encode_key_update(0);
            let encrypted = self
                .handshake_state
                .write_record
                .encrypt_record(ContentType::Handshake, &ku)?;
            self.handshake_state.write_queue.push_back(encrypted);
        }
        Ok(())
    }
}

// ── QuicHandshake trait (async-friendly sans-IO QUIC handshake) ───────────

use async_trait::async_trait;

/// Events emitted by the QUIC handshake state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuicHandshakeEvent {
    /// More data is needed from the peer to make progress.
    NeedMoreData,
    /// The TLS handshake has completed and QUIC traffic secrets are
    /// available via [`QuicHandshake::quic_secrets`].
    HandshakeComplete,
}

/// A sans-IO TLS handshake that speaks QUIC CRYPTO frames (no TLS record
/// framing).
///
/// Implemented by both [`ClientConnection`] and [`ServerConnection`] so
/// that a QUIC transport layer can drive the handshake without knowing
/// which side it is.
#[async_trait]
pub trait QuicHandshake: Send {
    /// Take the next chunk of raw handshake bytes to send in a CRYPTO frame.
    fn write_handshake(&mut self) -> Option<Bytes>;

    /// Feed raw handshake bytes received from a CRYPTO frame.
    fn inject_handshake(&mut self, data: &[u8]);

    /// Advance the TLS state machine. Call after [`inject_handshake`].
    async fn process(&mut self) -> Result<QuicHandshakeEvent, Error>;

    /// Extract the QUIC traffic secrets after the handshake completes.
    fn quic_secrets(&self) -> Option<crate::quic::QuicSecrets>;

    /// The negotiated cipher suite, if the handshake has progressed far
    /// enough.
    fn cipher_suite(&self) -> Option<CipherSuite>;

    /// Whether the TLS handshake is complete.
    fn is_handshake_done(&self) -> bool;

    /// The key exchange group in use.
    fn key_exchange_group(&self) -> KeyExchangeGroup;
}

// ── QuicHandshake impl for ClientConnection ──────────────────────────────

#[async_trait]
impl QuicHandshake for ClientConnection {
    fn write_handshake(&mut self) -> Option<Bytes> {
        ClientConnection::write_handshake(self)
    }

    fn inject_handshake(&mut self, data: &[u8]) {
        ClientConnection::inject_handshake(self, data);
    }

    async fn process(&mut self) -> Result<QuicHandshakeEvent, Error> {
        let was_done_before = self.handshake_state.handshake_done;
        ClientConnection::process(self).await?;
        if self.handshake_state.handshake_done && !was_done_before {
            Ok(QuicHandshakeEvent::HandshakeComplete)
        } else {
            Ok(QuicHandshakeEvent::NeedMoreData)
        }
    }

    fn quic_secrets(&self) -> Option<crate::quic::QuicSecrets> {
        ClientConnection::quic_secrets(self)
    }

    fn cipher_suite(&self) -> Option<CipherSuite> {
        ClientConnection::cipher_suite(self)
    }

    fn is_handshake_done(&self) -> bool {
        ClientConnection::handshake_done(self)
    }

    fn key_exchange_group(&self) -> KeyExchangeGroup {
        ClientConnection::key_exchange_group(self)
    }
}

// ── QuicHandshake impl for ServerConnection ──────────────────────────────

#[async_trait]
impl QuicHandshake for ServerConnection {
    fn write_handshake(&mut self) -> Option<Bytes> {
        ServerConnection::write_handshake(self)
    }

    fn inject_handshake(&mut self, data: &[u8]) {
        ServerConnection::inject_handshake(self, data);
    }

    async fn process(&mut self) -> Result<QuicHandshakeEvent, Error> {
        let was_done_before = self.handshake_state.handshake_done;
        ServerConnection::process(self).await?;
        if self.handshake_state.handshake_done && !was_done_before {
            Ok(QuicHandshakeEvent::HandshakeComplete)
        } else {
            Ok(QuicHandshakeEvent::NeedMoreData)
        }
    }

    fn quic_secrets(&self) -> Option<crate::quic::QuicSecrets> {
        ServerConnection::quic_secrets(self)
    }

    fn cipher_suite(&self) -> Option<CipherSuite> {
        self.handshake_state.cipher_suite
    }

    fn is_handshake_done(&self) -> bool {
        self.handshake_state.handshake_done
    }

    fn key_exchange_group(&self) -> KeyExchangeGroup {
        self.handshake_state.key_exchange_group
    }
}

// ── TlsState (unified dispatch for post-handshake I/O) ──────────────────

/// Internal dispatch between client and server connections for I/O wrappers.
pub(crate) enum TlsState {
    Client(ClientConnection),
    Server(ServerConnection),
}

impl TlsState {
    pub(crate) fn inject(&mut self, data: &[u8]) {
        match self {
            TlsState::Client(c) => c.inject(data),
            TlsState::Server(c) => c.inject(data),
        }
    }

    pub(crate) async fn process_app_data(&mut self) -> Result<bool, Error> {
        match self {
            TlsState::Client(c) => c.process_app_data().await,
            TlsState::Server(c) => c.process_app_data().await,
        }
    }

    pub(crate) fn read_app_data(&mut self) -> Option<Bytes> {
        match self {
            TlsState::Client(c) => c.read_app_data(),
            TlsState::Server(c) => c.read_app_data(),
        }
    }

    pub(crate) fn send(&mut self, data: &[u8]) -> Result<Bytes, Error> {
        match self {
            TlsState::Client(c) => c.send(data),
            TlsState::Server(c) => c.send(data),
        }
    }

    pub(crate) fn close(&mut self) -> Result<Bytes, Error> {
        match self {
            TlsState::Client(c) => c.close(),
            TlsState::Server(c) => c.close(),
        }
    }

    pub(crate) fn close_notified(&self) -> bool {
        match self {
            TlsState::Client(c) => c.close_notified(),
            TlsState::Server(c) => c.close_notified(),
        }
    }

    pub(crate) fn cipher_suite(&self) -> Option<CipherSuite> {
        match self {
            TlsState::Client(c) => c.cipher_suite(),
            _ => None,
        }
    }

    pub(crate) fn key_exchange_group(&self) -> Option<KeyExchangeGroup> {
        match self {
            TlsState::Client(c) => Some(c.key_exchange_group()),
            _ => None,
        }
    }

    pub(crate) fn alpn_protocol(&self) -> Option<&[u8]> {
        match self {
            TlsState::Client(c) => c.alpn_protocol(),
            TlsState::Server(c) => c.alpn_protocol(),
        }
    }

    pub(crate) fn server_name(&self) -> Option<&str> {
        match self {
            TlsState::Client(c) => c.server_name(),
            _ => None,
        }
    }

    pub(crate) fn negotiated_version(&self) -> u16 {
        match self {
            TlsState::Client(c) => c.negotiated_version(),
            TlsState::Server(c) => c.negotiated_version(),
        }
    }

    pub(crate) fn signature_scheme(&self) -> Option<SignatureScheme> {
        match self {
            TlsState::Client(c) => c.signature_scheme(),
            _ => None,
        }
    }
}
