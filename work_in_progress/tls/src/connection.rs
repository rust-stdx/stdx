use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::VecDeque,
    format,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};

use bytes::{Bytes, BytesMut};

use crate::{
    Error,
    config::{ClientConfig, ClientHello, ReceivedCertificate, ServerConfig},
    crypto::{CertType, CipherSuite, KeyExchangeGroup, MAX_HASH_OUTPUT, MAX_KX_PUBLIC_KEY, MAX_SHARED_SECRET},
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

/// Common handshake state shared by client and server.
struct HandshakeState {
    cipher_suite: Option<CipherSuite>,
    kx_group: KeyExchangeGroup,
    kx_pairs: Vec<Box<dyn crate::crypto::KeyExchangeKeyPair>>,
    peer_public_key: Option<heapless::Vec<u8, MAX_KX_PUBLIC_KEY>>,
    shared_secret: Option<heapless::Vec<u8, MAX_SHARED_SECRET>>,
    key_schedule: Option<KeySchedule>,
    keys: Option<TlsKeys>,
    transcript: Vec<u8>,
    write_record: RecordState,
    read_record: RecordState,
    read_buf: BytesMut,
    alpn_selected: Option<Bytes>,
    cert_chain: Option<Vec<Vec<u8>>>,
    negotiated_cert_type: CertType,
    handshake_payload: BytesMut,
    server_hello_hash: heapless::Vec<u8, MAX_HASH_OUTPUT>,
    write_queue: VecDeque<Bytes>,
    app_data_queue: VecDeque<Bytes>,
    handshake_done: bool,
}

impl HandshakeState {
    fn new() -> Self {
        Self {
            cipher_suite: None,
            kx_group: KeyExchangeGroup::X25519,
            kx_pairs: Vec::new(),
            peer_public_key: None,
            shared_secret: None,
            key_schedule: None,
            keys: None,
            transcript: Vec::new(),
            write_record: RecordState::new(),
            read_record: RecordState::new(),
            read_buf: BytesMut::new(),
            alpn_selected: None,
            cert_chain: None,
            negotiated_cert_type: CertType::X509,
            handshake_payload: BytesMut::new(),
            server_hello_hash: heapless::Vec::new(),
            write_queue: VecDeque::new(),
            app_data_queue: VecDeque::new(),
            handshake_done: false,
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
    hs: HandshakeState,
    server_name: Option<String>,
}

impl ClientConnection {
    /// Return the selected ALPN protocol, if any.
    pub fn alpn_protocol(&self) -> Option<&Bytes> {
        self.hs.alpn_selected.as_ref()
    }

    /// Return the negotiated cipher suite, if the handshake has progressed far enough.
    pub fn cipher_suite(&self) -> Option<CipherSuite> {
        self.hs.cipher_suite
    }

    /// Return the key exchange group in use.
    pub fn kx_group(&self) -> KeyExchangeGroup {
        self.hs.kx_group
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
        self.hs.write_record.encrypt_record(ContentType::ApplicationData, data)
    }

    /// Initiate a clean close.
    pub fn close(&mut self) -> Result<Bytes, Error> {
        self.hs.write_record.encrypt_alert(1, 0)
    }

    /// Feed received bytes into the internal buffer.
    pub fn inject(&mut self, input: &[u8]) {
        self.hs.read_buf.extend_from_slice(input);
    }

    /// Take the next chunk of decrypted application data.
    pub fn read_app_data(&mut self) -> Option<Bytes> {
        self.hs.app_data_queue.pop_front()
    }

    /// Take the next chunk of TLS bytes to send to the peer.
    pub fn write_tls(&mut self) -> Option<Bytes> {
        self.hs.write_queue.pop_front()
    }

    /// Is the handshake complete?
    pub fn handshake_done(&self) -> bool {
        self.hs.handshake_done
    }

    /// Create a new client connection.
    ///
    /// `server_name` is the SNI hostname to send; `None` disables SNI.
    ///
    /// The initial ClientHello bytes are queued internally and can be drained
    /// via [`write_tls`].
    pub fn new(config: ClientConfig, server_name: Option<String>) -> Result<Self, Error> {
        let mut hs = HandshakeState::new();
        let crypto_provider = &config.crypto;

        let supported_groups = crypto_provider.supported_key_exchange_groups();
        let kx_group = *supported_groups.first().ok_or(Error::NoKeyExchangeGroupInCommon)?;

        let mut kx_pairs: Vec<Box<dyn crate::crypto::KeyExchangeKeyPair>> = Vec::new();
        let mut key_share_entries: Vec<(KeyExchangeGroup, heapless::Vec<u8, MAX_KX_PUBLIC_KEY>)> = Vec::new();

        // Generate key pairs for all groups; send only the preferred
        // (first) group in key_share.
        for &group in supported_groups {
            let kp = crypto_provider.create_kx_pair(group)?;
            let pk = kp.public_key_bytes();
            if group == kx_group {
                key_share_entries.push((group, pk));
            }
            kx_pairs.push(kp);
        }

        hs.kx_group = kx_group;
        hs.kx_pairs = kx_pairs;

        let key_share_refs: Vec<(KeyExchangeGroup, &[u8])> =
            key_share_entries.iter().map(|(g, pk)| (*g, pk.as_slice())).collect();

        let mut exts = vec![
            ext_supported_versions(),
            ext_supported_groups(&supported_groups),
            ext_key_share_client(&key_share_refs),
            ext_signature_algorithms(crypto_provider.supported_signature_schemes()),
        ];
        if let Some(ref name) = server_name {
            exts.push(ext_server_name(name));
        }
        if !config.alpn_protocols.is_empty() {
            exts.push(ext_alpn(&config.alpn_protocols));
        }
        if config.cert_types != [CertType::X509] || config.cert_types.len() != 1 {
            exts.push(ext_server_cert_type_client(&config.cert_types));
        }
        let cipher_suites: Vec<_> = crypto_provider.supported_cipher_suites().to_vec();

        let mut random = [0u8; 32];
        crypto_provider.secure_random(&mut random);

        let ch = encode_client_hello(&random, &[], &cipher_suites, &exts);
        hs.transcript.extend_from_slice(&ch);

        let mut record = Vec::with_capacity(5 + ch.len());
        record.push(ContentType::Handshake as u8);
        record.extend_from_slice(&0x0301u16.to_be_bytes());
        record.extend_from_slice(&(ch.len() as u16).to_be_bytes());
        record.extend_from_slice(&ch);

        hs.write_queue.push_back(Bytes::from(record));

        Ok(Self {
            config,
            state: ClientState::SentClientHello,
            hs,
            server_name,
        })
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
                ClientState::Done => self.process_application_data()?,
                ClientState::Failed => {
                    return Err(Error::HandshakeFailed("connection in failed state".into()));
                }
            };
            if !made_progress || self.state == ClientState::Done {
                break;
            }
        }
        Ok(())
    }

    fn try_read_record(&mut self) -> Result<Option<(ContentType, Bytes)>, Error> {
        if self.hs.handshake_payload.len() >= 4 {
            let msg_len = u32::from_be_bytes([
                0,
                self.hs.handshake_payload[1],
                self.hs.handshake_payload[2],
                self.hs.handshake_payload[3],
            ]) as usize;
            if self.hs.handshake_payload.len() >= 4 + msg_len {
                let msg = self.hs.handshake_payload.split_to(4 + msg_len);
                return Ok(Some((ContentType::Handshake, msg.freeze())));
            }
        }

        if self.hs.read_buf.is_empty() {
            return Ok(None);
        }
        let consumed = {
            let data = &self.hs.read_buf[..];
            self.hs.read_record.decrypt_record(data)
        }?;
        match consumed {
            Some((ct, payload)) => {
                let record_len = if self.hs.read_buf.len() >= 5 {
                    5 + u16::from_be_bytes([self.hs.read_buf[3], self.hs.read_buf[4]]) as usize
                } else {
                    return Ok(None);
                };
                let _ = self.hs.read_buf.split_to(record_len);
                if ct == ContentType::Handshake {
                    self.hs.handshake_payload.extend_from_slice(&payload);
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

        let sh = ServerHello::decode(&payload)?;
        self.hs.transcript.extend_from_slice(&sh.raw);
        self.hs.cipher_suite = Some(sh.cipher_suite);

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
            .ok_or_else(|| Error::HandshakeFailed("no key_share in ServerHello".into()))?;
        let (group, peer_pk) = parse_key_share_server(ks_ext)?;
        self.hs.peer_public_key = Some(peer_pk.clone());

        let kx = self
            .hs
            .kx_pairs
            .iter()
            .find(|kp| kp.group() == group)
            .ok_or_else(|| Error::InternalError("no kx_pair for negotiated group".into()))?;
        self.hs.shared_secret = Some(kx.shared_secret(&peer_pk)?);

        let suite = sh.cipher_suite;
        let mut ks = KeySchedule::new(suite, Arc::clone(&self.config.crypto), None);
        ks.add_shared_secret(self.hs.shared_secret.as_ref().unwrap());
        self.hs.key_schedule = Some(ks);

        self.state = ClientState::WaitEncryptedExtensions;
        Ok(true)
    }

    fn setup_handshake_read_keys(&mut self) -> Result<(), Error> {
        let suite = self.hs.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto;
        let ks = self.hs.key_schedule.as_ref().unwrap();

        let transcript_hash = crypto_provider.hash(suite, &self.hs.transcript);
        self.hs.server_hello_hash = transcript_hash.clone();

        let s_hs_traffic = ks.server_handshake_traffic_secret(&transcript_hash);
        let s_hs_key = crypto_provider.hkdf_expand_label(suite, &s_hs_traffic, b"tls13 key", &[], suite.key_size());
        let s_hs_iv: [u8; 12] = crypto_provider
            .hkdf_expand_label(suite, &s_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();

        self.hs
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
        let ee = EncryptedExtensions::decode(&payload)?;
        self.hs.transcript.extend_from_slice(&ee.raw);

        if let Some(ext) = find_extension(&ee.extensions, ExtensionType::ApplicationLayerProtocolNegotiation) {
            let alpn = parse_alpn(ext)?;
            self.hs.alpn_selected = alpn.into_iter().next();
        }

        if let Some(ext) = find_extension(&ee.extensions, ExtensionType::ServerCertificateType) {
            self.hs.negotiated_cert_type = parse_server_cert_type_ee(ext)?;
        }

        self.state = ClientState::WaitCertificate;
        Ok(true)
    }

    async fn process_certificate(&mut self) -> Result<bool, Error> {
        let payload = loop {
            match self.try_read_record()? {
                Some((ContentType::ChangeCipherSpec, _)) => continue,
                Some((ContentType::Handshake, payload)) => break payload,
                None => return Ok(false),
                _ => {
                    return Err(Error::UnexpectedMessage {
                        expected: "Certificate",
                        got: "other",
                    });
                }
            }
        };
        let cert = Certificate::decode(&payload)?;
        self.hs.transcript.extend_from_slice(&cert.raw);
        self.hs.cert_chain = Some(cert.entries.into_iter().map(|e| e.cert_data).collect());

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
                        got: "other",
                    });
                }
            }
        };
        let cv = CertificateVerify::decode(&payload)?;
        self.hs.transcript.extend_from_slice(&cv.raw);

        let chain = self
            .hs
            .cert_chain
            .take()
            .ok_or_else(|| Error::InternalError("no cert chain".into()))?;
        if chain.is_empty() {
            return Err(Error::DecodeError("empty certificate chain".into()));
        }

        let received = match self.hs.negotiated_cert_type {
            CertType::X509 => ReceivedCertificate::X509 {
                chain,
                verify_scheme: cv.scheme,
            },
            CertType::RawPublicKey => {
                let pk = match chain.first() {
                    Some(spki_der) => extract_key_from_spki(spki_der)?.to_vec(),
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
            .map_err(|e| Error::CertificateValidationFailed(e.to_string()))?;

        let pk: Cow<'_, [u8]> = match &received {
            ReceivedCertificate::X509 {
                chain, ..
            } => {
                #[cfg(feature = "webpki-validator")]
                {
                    use x509_cert::{Certificate as X509Cert, der::Decode};
                    let cert =
                        X509Cert::from_der(&chain[0]).map_err(|e| Error::DecodeError(format!("X.509 parse: {e}")))?;
                    let spki = cert
                        .tbs_certificate
                        .subject_public_key_info
                        .subject_public_key
                        .as_bytes()
                        .ok_or_else(|| Error::DecodeError("empty SPKI in X.509 cert".into()))?;
                    Cow::Owned(spki.to_vec())
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

        let suite = self.hs.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto;
        let transcript_hash =
            crypto_provider.hash(suite, &self.hs.transcript[..self.hs.transcript.len() - cv.raw.len()]);

        let mut signed_data = Vec::with_capacity(64 + 34 + 1 + 48);
        signed_data.extend_from_slice(&[0x20; 64]);
        signed_data.extend_from_slice(b"TLS 1.3, server CertificateVerify");
        signed_data.push(0);
        signed_data.extend_from_slice(&transcript_hash);

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
        let fin = Finished::decode(&payload)?;

        let suite = self.hs.cipher_suite.unwrap();
        let crypto_provider = &self.config.crypto;
        let ks = self.hs.key_schedule.as_ref().unwrap();

        let transcript_hash_before_fin = crypto_provider.hash(suite, &self.hs.transcript);

        self.hs.transcript.extend_from_slice(&fin.raw);

        let transcript_hash_after_sfin = crypto_provider.hash(suite, &self.hs.transcript);

        let keys = ks.derive_keys(
            &self.hs.server_hello_hash,
            &transcript_hash_after_sfin,
            &transcript_hash_after_sfin,
        );

        let sfk = &keys.server_finished_key;
        let expected = crypto_provider.hmac(suite, sfk, &transcript_hash_before_fin);
        if !constant_time_eq::constant_time_eq(&expected, &fin.verify_data) {
            return Err(crate::Error::HandshakeFailed("finished verification failed".into()));
        }

        let our_fin_hash = crypto_provider.hash(suite, &self.hs.transcript);
        let our_verify_data_expected = crypto_provider.hmac(suite, &keys.client_finished_key, &our_fin_hash);
        let fin_msg = encode_finished(&our_verify_data_expected);

        let c_hs_traffic = ks.client_handshake_traffic_secret(&self.hs.server_hello_hash);
        let c_hs_key = crypto_provider.hkdf_expand_label(suite, &c_hs_traffic, b"tls13 key", &[], suite.key_size());
        let c_hs_iv: [u8; 12] = crypto_provider
            .hkdf_expand_label(suite, &c_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();
        self.hs
            .write_record
            .set_write_keys(crypto_provider.create_aead(suite, &c_hs_key)?, c_hs_iv);

        let encrypted_fin = self.hs.write_record.encrypt_record(ContentType::Handshake, &fin_msg)?;
        self.hs.write_queue.push_back(encrypted_fin);

        self.hs.read_record.set_read_keys(
            crypto_provider.create_aead(suite, &keys.server_application_key)?,
            keys.server_application_iv,
        );

        self.hs.write_record.set_write_keys(
            crypto_provider.create_aead(suite, &keys.client_application_key)?,
            keys.client_application_iv,
        );

        self.hs.keys = Some(keys);
        self.hs.handshake_done = true;
        self.state = ClientState::Done;
        Ok(true)
    }

    fn process_application_data(&mut self) -> Result<bool, Error> {
        let mut processed_any = false;
        loop {
            match self.try_read_record()? {
                Some((ContentType::ApplicationData, payload)) => {
                    self.hs.app_data_queue.push_back(payload);
                    processed_any = true;
                }
                Some((ContentType::Alert, _)) => return Err(Error::ConnectionClosed),
                Some(_) => continue,
                None => break,
            }
        }
        Ok(processed_any)
    }
}

// ── Server Connection ─────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum ServerState {
    WaitClientHello,
    WaitClientFinished,
    Done,
    Failed,
}

/// A TLS 1.3 server connection (sans-IO).
pub struct ServerConnection {
    config: ServerConfig,
    state: ServerState,
    hs: HandshakeState,
    /// Fingerprint result from [`TlsFingerprinter`], if configured.
    pub fingerprint: Option<[u8; 64]>,
}

impl ServerConnection {
    /// Encrypt application data for sending.
    pub fn send(&mut self, data: &[u8]) -> Result<Bytes, Error> {
        if !matches!(self.state, ServerState::Done) {
            return Err(Error::InternalError("handshake not complete".into()));
        }
        self.hs.write_record.encrypt_record(ContentType::ApplicationData, data)
    }

    /// Initiate a clean close.
    pub fn close(&mut self) -> Result<Bytes, Error> {
        self.hs.write_record.encrypt_alert(1, 0)
    }

    /// Return the selected ALPN protocol, if any.
    pub fn alpn_protocol(&self) -> Option<&Bytes> {
        self.hs.alpn_selected.as_ref()
    }

    /// Feed received bytes into the internal buffer.
    pub fn inject(&mut self, input: &[u8]) {
        self.hs.read_buf.extend_from_slice(input);
    }

    /// Take the next chunk of decrypted application data.
    pub fn read_app_data(&mut self) -> Option<Bytes> {
        self.hs.app_data_queue.pop_front()
    }

    /// Take the next chunk of TLS bytes to send to the peer.
    pub fn write_tls(&mut self) -> Option<Bytes> {
        self.hs.write_queue.pop_front()
    }

    /// Is the handshake complete?
    pub fn handshake_done(&self) -> bool {
        self.hs.handshake_done
    }

    /// Create a new server connection, ready to receive a ClientHello.
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            state: ServerState::WaitClientHello,
            hs: HandshakeState::new(),
            fingerprint: None,
        }
    }

    /// Advance the state machine. Call after [`inject`]ing data.
    pub async fn process(&mut self) -> Result<(), Error> {
        loop {
            let made_progress = match self.state {
                ServerState::WaitClientHello => self.process_client_hello().await?,
                ServerState::WaitClientFinished => self.process_client_finished()?,
                ServerState::Done => self.process_application_data()?,
                ServerState::Failed => {
                    return Err(Error::HandshakeFailed("connection in failed state".into()));
                }
            };
            if !made_progress || self.state == ServerState::Done {
                break;
            }
        }
        Ok(())
    }

    fn try_read_record(&mut self) -> Result<Option<(ContentType, Bytes)>, Error> {
        if self.hs.read_buf.is_empty() {
            return Ok(None);
        }
        let consumed = {
            let data = &self.hs.read_buf[..];
            self.hs.read_record.decrypt_record(data)
        }?;
        match consumed {
            Some((ct, payload)) => {
                let record_len = if self.hs.read_buf.len() >= 5 {
                    5 + u16::from_be_bytes([self.hs.read_buf[3], self.hs.read_buf[4]]) as usize
                } else {
                    return Ok(None);
                };
                let _ = self.hs.read_buf.split_to(record_len);
                Ok(Some((ct, payload)))
            }
            None => Ok(None),
        }
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

        let ch = ClientHelloMsg::decode(&payload)?;
        self.hs.transcript.extend_from_slice(&ch.raw);

        let sv_ext = find_extension(&ch.extensions, ExtensionType::SupportedVersions);
        if !check_supported_versions(sv_ext) {
            return Err(Error::HandshakeFailed("TLS 1.3 not offered".into()));
        }

        let provider = &self.config.provider;

        let offer: Vec<_> = ch.cipher_suites.iter().copied().collect();
        let suite = provider
            .supported_cipher_suites()
            .iter()
            .map(|s| *s)
            .find(|s| offer.contains(s))
            .ok_or(Error::NoCipherSuitesInCommon)?;
        self.hs.cipher_suite = Some(suite);

        let ks_ext = find_extension(&ch.extensions, ExtensionType::KeyShare)
            .ok_or_else(|| Error::HandshakeFailed("no key_share in ClientHello".into()))?;
        let (group, peer_pk) = parse_key_share(ks_ext)?;
        self.hs.peer_public_key = Some(peer_pk.clone());

        let mut kx_pair = provider.create_kx_pair(group)?;
        kx_pair.set_peer_public_key(&peer_pk)?;
        let kx_pub = kx_pair.public_key_bytes();
        let shared = kx_pair.shared_secret(self.hs.peer_public_key.as_ref().unwrap())?;
        self.hs.shared_secret = Some(shared);
        self.hs.kx_pairs.push(kx_pair);

        let mut ks = KeySchedule::new(suite, Arc::clone(provider), None);
        ks.add_shared_secret(self.hs.shared_secret.as_ref().unwrap());
        self.hs.key_schedule = Some(ks);

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
            String::from_utf8(d[5..5 + name_len].to_vec()).ok()
        });
        let sig_schemes = find_extension(&ch.extensions, ExtensionType::SignatureAlgorithms)
            .map(|e| parse_signature_algorithms(e))
            .transpose()?
            .unwrap_or_default();

        let client_cert_types = find_extension(&ch.extensions, ExtensionType::ServerCertificateType)
            .map(|e| parse_server_cert_type_ch(e))
            .transpose()?
            .unwrap_or_else(|| vec![CertType::X509]);

        if let Some(ref fp) = self.config.fingerprinter {
            self.fingerprint = Some(fp.fingerprint(&ch.raw).await?);
        }

        let client_hello = ClientHello {
            server_name: server_name_str.as_deref(),
            alpn_protocols: &alpn_protos,
            cipher_suites: &ch.cipher_suites,
            kx_group: group,
            sig_schemes: &sig_schemes,
            raw: &ch.raw,
        };

        let cert = self.config.cert_provider.provide(&client_hello).await?;

        if !sig_schemes.contains(&cert.scheme) {
            return Err(Error::HandshakeFailed(format!(
                "CertificateProvider selected scheme {:?} which was not offered by client",
                cert.scheme
            )));
        }

        let selected_cert_type = if client_cert_types.contains(&CertType::RawPublicKey) {
            CertType::RawPublicKey
        } else {
            CertType::X509
        };

        let mut random = [0u8; 32];
        provider.secure_random(&mut random);

        let mut sh_exts = vec![ext_supported_versions_server(), ext_key_share_server(&kx_pub, group)];
        let alpn_sel = client_hello
            .alpn_protocols
            .iter()
            .find(|p| self.config.alpn_protocols.contains(p))
            .cloned();
        if let Some(ref proto) = alpn_sel {
            sh_exts.push(ext_alpn(&[proto.clone()]));
            self.hs.alpn_selected = Some(proto.clone());
        }

        let sh = encode_server_hello(&random, &ch.session_id, suite, &sh_exts);
        self.hs.transcript.extend_from_slice(&sh);

        let transcript_hash = provider.hash(suite, &self.hs.transcript);
        self.hs.server_hello_hash = transcript_hash.clone();

        let ks_ref = self.hs.key_schedule.as_ref().unwrap();
        let c_hs_traffic = ks_ref.client_handshake_traffic_secret(&transcript_hash);
        let s_hs_traffic = ks_ref.server_handshake_traffic_secret(&transcript_hash);
        let s_hs_key = provider.hkdf_expand_label(suite, &s_hs_traffic, b"tls13 key", &[], suite.key_size());
        let s_hs_iv: [u8; 12] = provider
            .hkdf_expand_label(suite, &s_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();
        let c_hs_key = provider.hkdf_expand_label(suite, &c_hs_traffic, b"tls13 key", &[], suite.key_size());
        let c_hs_iv: [u8; 12] = provider
            .hkdf_expand_label(suite, &c_hs_traffic, b"tls13 iv", &[], 12)
            .as_slice()
            .try_into()
            .unwrap();

        self.hs
            .write_record
            .set_write_keys(provider.create_aead(suite, &s_hs_key)?, s_hs_iv);

        // 1) ServerHello (plaintext handshake record)
        let mut sh_record = Vec::new();
        sh_record.push(ContentType::Handshake as u8);
        sh_record.extend_from_slice(&0x0303u16.to_be_bytes());
        sh_record.extend_from_slice(&(sh.len() as u16).to_be_bytes());
        sh_record.extend_from_slice(&sh);
        self.hs.write_queue.push_back(Bytes::from(sh_record));

        // 2) EncryptedExtensions
        let ee_exts =
            if client_cert_types.contains(&CertType::RawPublicKey) || client_cert_types.contains(&CertType::X509) {
                vec![ext_server_cert_type_server(selected_cert_type)]
            } else {
                vec![]
            };
        let ee = encode_encrypted_extensions(&ee_exts);
        self.hs.transcript.extend_from_slice(&ee);
        self.hs
            .write_queue
            .push_back(self.hs.write_record.encrypt_record(ContentType::Handshake, &ee)?);

        // 3) Certificate
        let (public_key, signer) = (&cert.payload.public_key, &cert.payload.signer);
        let cert_msg = encode_certificate_raw_public_key(&[], public_key, &[]);
        self.hs.transcript.extend_from_slice(&cert_msg);
        self.hs
            .write_queue
            .push_back(self.hs.write_record.encrypt_record(ContentType::Handshake, &cert_msg)?);

        // 4) CertificateVerify
        let cv_transcript_hash = provider.hash(suite, &self.hs.transcript);
        let mut signed_data = Vec::with_capacity(64 + 34 + 1 + 48);
        signed_data.extend_from_slice(&[0x20; 64]);
        signed_data.extend_from_slice(b"TLS 1.3, server CertificateVerify");
        signed_data.push(0);
        signed_data.extend_from_slice(&cv_transcript_hash);
        let signature = signer.sign(&signed_data)?;

        let cv_msg = encode_certificate_verify(cert.scheme, &signature);
        self.hs.transcript.extend_from_slice(&cv_msg);
        self.hs
            .write_queue
            .push_back(self.hs.write_record.encrypt_record(ContentType::Handshake, &cv_msg)?);

        // 5) Server Finished
        let s_hs_traffic_for_fin = ks_ref.server_handshake_traffic_secret(&self.hs.server_hello_hash);
        let s_fin_key =
            provider.hkdf_expand_label(suite, &s_hs_traffic_for_fin, b"tls13 finished", &[], suite.hash_size());

        let fin_transcript_hash = provider.hash(suite, &self.hs.transcript);

        let verify_data = provider.hmac(suite, &s_fin_key, &fin_transcript_hash);
        let fin_msg = encode_finished(&verify_data);
        self.hs.transcript.extend_from_slice(&fin_msg);
        self.hs
            .write_queue
            .push_back(self.hs.write_record.encrypt_record(ContentType::Handshake, &fin_msg)?);

        let post_sfin_hash = provider.hash(suite, &self.hs.transcript);

        let keys = ks_ref.derive_keys(&self.hs.server_hello_hash, &post_sfin_hash, &post_sfin_hash);

        self.hs
            .read_record
            .set_read_keys(provider.create_aead(suite, &c_hs_key)?, c_hs_iv);

        self.hs.keys = Some(keys);
        self.state = ServerState::WaitClientFinished;
        Ok(true)
    }

    fn process_client_finished(&mut self) -> Result<bool, Error> {
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

        let fin = Finished::decode(&payload)?;
        self.hs.transcript.extend_from_slice(&fin.raw);

        let suite = self.hs.cipher_suite.unwrap();
        let provider = &self.config.provider;
        let ks = self.hs.key_schedule.as_ref().unwrap();
        let keys = self
            .hs
            .keys
            .as_ref()
            .ok_or_else(|| Error::InternalError("no keys".into()))?;

        let transcript_hash = provider.hash(suite, &self.hs.transcript[..self.hs.transcript.len() - fin.raw.len()]);

        ks.verify_finished(&keys.client_finished_key, &transcript_hash, &fin.verify_data)?;

        self.hs.read_record.set_read_keys(
            provider.create_aead(suite, &keys.client_application_key)?,
            keys.client_application_iv,
        );
        self.hs.write_record.set_write_keys(
            provider.create_aead(suite, &keys.server_application_key)?,
            keys.server_application_iv,
        );

        self.hs.handshake_done = true;
        self.state = ServerState::Done;
        Ok(true)
    }

    fn process_application_data(&mut self) -> Result<bool, Error> {
        let mut processed_any = false;
        loop {
            match self.try_read_record()? {
                Some((ContentType::ApplicationData, payload)) => {
                    self.hs.app_data_queue.push_back(payload);
                    processed_any = true;
                }
                Some((ContentType::Alert, _)) => return Err(Error::ConnectionClosed),
                Some(_) => continue,
                None => break,
            }
        }
        Ok(processed_any)
    }
}
