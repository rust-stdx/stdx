use crate::{
    CertType, CipherSuite, CryptoProvider, KeyExchangeGroup, KeyExchangeSecretKey, ReceivedCertificate,
    SignatureScheme,
    errors::Error,
    key_schedule, message,
    record::{self, ContentType, RecordHeader, decrypt_record, encrypt_record},
};

/// Configuration for a TLS 1.3 client connection.
///
/// Created via [`ClientConfig::new`] with a [`CryptoProvider`]. The provider
/// supplies all cryptographic primitives (AEAD, key exchange, signatures)
/// and certificate validation.
///
/// By default only X.509 certificates are negotiated. Call
/// [`with_certificate_types`][Self::with_certificate_types] to also accept
/// RawPublicKey certificates (RFC 7250).
#[derive(Clone)]
pub struct ClientConfig<C: CryptoProvider> {
    crypto_provider: C,
    supported_certificate_types: heapless::Vec<CertType, 2>,
}

impl<C: CryptoProvider> ClientConfig<C> {
    /// Create a new client configuration backed by the given crypto provider.
    ///
    /// The initial configuration accepts X.509 certificates only.
    pub fn new(crypto_provider: C) -> Self {
        Self {
            crypto_provider,
            supported_certificate_types: [CertType::X509].into(),
        }
    }
    /// Set the set of acceptable certificate types.
    ///
    /// The default is `[CertType::X509]`. To also negotiate raw public keys
    /// (RFC 7250), pass `&[CertType::X509, CertType::RawPublicKey]`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidConfiguration`] when more than two types are
    /// supplied.
    pub fn with_certificate_types(mut self, types: &[CertType]) -> Result<Self, Error> {
        self.supported_certificate_types = types.try_into().map_err(|_| Error::InvalidConfiguration)?;
        Ok(self)
    }
}

/// A sans-IO TLS 1.3 client state machine.
///
/// The client progresses through the handshake phases:
///
/// | Phase | What happens |
/// |-------|-------------|
/// | [`ClientHello`](Phase::ClientHello) | Waiting for [`start_handshake`](Self::start_handshake) to write the ClientHello |
/// | [`ServerHello`](Phase::ServerHello) / [`ServerFlight`](Phase::ServerFlight) | Processing the server's response |
/// | [`ClientFinished`](Phase::ClientFinished) | Sending the client Finished, then app keys are installed |
/// | [`ApplicationData`](Phase::ApplicationData) | Connection is established; use [`encrypt`](Self::encrypt) / [`decrypt`](Self::decrypt) |
/// | [`Closed`](Phase::Closed) | Connection has been terminated |
///
/// The caller is responsible for all network I/O. Use [`receive_buffer`](Self::receive_buffer)
/// and [`commit_received`](Self::commit_received) to feed data in, and
/// [`outgoing_data`](Self::outgoing_data) to extract data to send.
///
/// # Panics
///
/// Methods that dereference [`suite`](CipherSuite) (e.g. [`encrypt`](Self::encrypt),
/// [`decrypt`](Self::decrypt)) will panic if called before the handshake completes.
pub struct Client<'a, C: CryptoProvider> {
    pub(crate) config: ClientConfig<C>,
    pub(crate) receive_buffer: &'a mut [u8],
    pub(crate) send_buffer: &'a mut [u8],

    // ── Buffer tracking ──
    pub(crate) receive_decoded: usize,
    pub(crate) receive_pending: usize,
    pub(crate) out_len: usize,
    pub(crate) send_consumed: usize,

    pub(crate) app_data_offset: usize,
    pub(crate) app_data_decrypted_len: usize,
    pub(crate) app_data_consumed: usize,
    pub(crate) key_update_response: heapless::Vec<u8, 256>,
    pub(crate) key_update_sent: usize,
    pub(crate) ticket_offset: usize,
    pub(crate) ticket_len: usize,

    // ── Connection state ──
    pub(crate) phase: Phase,
    pub(crate) opened: bool,
    pub(crate) close_received: bool,
    pub(crate) close_sent: bool,

    // ── Negotiated ──
    pub(crate) ciphersuite: Option<CipherSuite>,
    pub(crate) alpn: Option<([u8; 255], usize)>,
    pub(crate) negotiated_cert_type: CertType,

    // ── Key exchange ──
    pub(crate) key_exchange_group: KeyExchangeGroup,
    pub(crate) key_exchange_secret: [u8; 32],

    // ── Key schedule ──
    pub(crate) keys: KeySchedule,
    pub(crate) app_write_key: [u8; 32],
    pub(crate) app_write_iv: [u8; 12],
    pub(crate) app_read_key: [u8; 32],
    pub(crate) app_read_iv: [u8; 12],
    pub(crate) app_read_traffic_secret: [u8; 48],
    pub(crate) app_write_traffic_secret: [u8; 48],

    // ── Handshake-only (zeroed after Done) ──
    pub(crate) handshake_client_finished_key: [u8; 48],
    pub(crate) handshake_server_finished_key: [u8; 48],

    pub(crate) client_random: [u8; 32],
    pub(crate) server_random: [u8; 32],
    pub(crate) server_public_key: heapless::Vec<u8, 294>,
    pub(crate) server_signature_scheme: Option<SignatureScheme>,
    pub(crate) server_name: heapless::Vec<u8, 256>,
    pub(crate) transcript_bytes: heapless::Vec<u8, 8192>,
    pub(crate) resumption_secret: [u8; 48],
}

/// Handshake phases the TLS 1.3 client progresses through.
///
/// See the [`Client`] documentation for the phase transition diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Phase {
    ClientHello,
    ServerHello,
    ServerFlight,
    ClientFinished,
    ApplicationData,
    Closed,
}

pub(crate) struct KeySchedule {
    pub(crate) secret: [u8; 48],
    pub(crate) read_key: [u8; 32],
    pub(crate) read_iv: [u8; 12],
    pub(crate) read_seq: u64,
    pub(crate) read_traffic_secret: [u8; 48],
    pub(crate) write_key: [u8; 32],
    pub(crate) write_iv: [u8; 12],
    pub(crate) write_seq: u64,
    pub(crate) write_traffic_secret: [u8; 48],
}

impl KeySchedule {
    fn new() -> Self {
        Self {
            secret: [0u8; 48],
            read_key: [0u8; 32],
            read_iv: [0u8; 12],
            read_seq: 0,
            read_traffic_secret: [0u8; 48],
            write_key: [0u8; 32],
            write_iv: [0u8; 12],
            write_seq: 0,
            write_traffic_secret: [0u8; 48],
        }
    }
}

// ── Public API ──

impl<'a, C: CryptoProvider> Client<'a, C> {
    /// Create a new TLS 1.3 client.
    ///
    /// `receive_buffer` and `send_buffer` are caller-owned scratch buffers
    /// that the client uses to hold incoming and outgoing TLS records.
    /// Each must be at least [`MAX_RECORD_SIZE`] bytes long.
    ///
    /// The buffers must outlive the `Client` (hence the `'a` lifetime).
    pub fn new(config: ClientConfig<C>, receive_buffer: &'a mut [u8], send_buffer: &'a mut [u8]) -> Self {
        Self {
            config,
            receive_buffer,
            send_buffer,
            receive_decoded: 0,
            receive_pending: 0,
            out_len: 0,
            send_consumed: 0,
            app_data_offset: 0,
            app_data_decrypted_len: 0,
            app_data_consumed: 0,
            key_update_response: heapless::Vec::new(),
            key_update_sent: 0,
            ticket_offset: 0,
            ticket_len: 0,
            phase: Phase::ClientHello,
            opened: false,
            close_received: false,
            close_sent: false,
            ciphersuite: None,
            alpn: None,
            negotiated_cert_type: CertType::X509,
            key_exchange_group: C::KEY_EXCHANGE_GROUPS
                .first()
                .copied()
                .unwrap_or(KeyExchangeGroup::X25519),
            key_exchange_secret: [0u8; 32],
            keys: KeySchedule::new(),
            app_write_key: [0u8; 32],
            app_write_iv: [0u8; 12],
            app_read_key: [0u8; 32],
            app_read_iv: [0u8; 12],
            app_read_traffic_secret: [0u8; 48],
            app_write_traffic_secret: [0u8; 48],
            handshake_client_finished_key: [0u8; 48],
            handshake_server_finished_key: [0u8; 48],
            client_random: [0u8; 32],
            server_random: [0u8; 32],
            server_public_key: heapless::Vec::new(),
            server_signature_scheme: None,
            server_name: heapless::Vec::new(),
            transcript_bytes: heapless::Vec::new(),
            resumption_secret: [0u8; 48],
        }
    }

    fn hash_size(&self) -> usize {
        self.ciphersuite.map_or(32, |s| s.hash_size())
    }
    fn key_size(&self) -> usize {
        self.ciphersuite.map_or(16, |s| s.key_size())
    }
    fn transcript_hash(&self, suite: CipherSuite, out: &mut [u8]) -> Result<(), Error> {
        let crypto_provider = &self.config.crypto_provider;
        if self.transcript_bytes.is_empty() {
            crypto_provider.hash(suite, &[], out)?;
        } else {
            crypto_provider.hash(suite, &self.transcript_bytes, out)?;
        }
        Ok(())
    }

    // ── I/O (buffer fill) ──
    /// Return the tail of the receive buffer where the caller should write
    /// incoming network data.
    pub fn receive_buffer(&mut self) -> &mut [u8] {
        let start = self.receive_decoded + self.receive_pending;
        &mut self.receive_buffer[start..]
    }
    /// Inform the client that `n` bytes have been written into the buffer
    /// returned by [`receive_buffer`](Self::receive_buffer).
    pub fn commit_received(&mut self, n: usize) {
        self.receive_pending += n;
    }

    // ── Handshake ──
    /// Begin the TLS 1.3 handshake by writing a ClientHello into the send
    /// buffer.
    ///
    /// `server_name` is the SNI host name (optional).  `alpn_protocols`
    /// lists application-layer protocol identifiers to negotiate (e.g.
    /// `b"h2"`, `b"http/1.1"`).
    ///
    /// On success returns [`Send`](ClientHandshakeEvent::Send) — the caller
    /// should transmit [`outgoing_data`](Self::outgoing_data)
    /// over the network and then call [`continue_handshake`](Self::continue_handshake).
    ///
    /// # Errors
    ///
    /// Returns [`Error::CryptoError`] if key-pair generation fails, or
    /// [`Error::EncodeError`] if the ClientHello cannot be encoded.
    pub fn start_handshake(
        &mut self,
        server_name: Option<&str>,
        alpn_protocols: &[&[u8]],
    ) -> Result<ClientHandshakeEvent, Error> {
        let crypto_provider = &self.config.crypto_provider;
        crypto_provider.secure_random(&mut self.client_random);

        let group = self.key_exchange_group;
        let (secret, public) = crypto_provider.key_exchange_generate_keypair(group)?;
        self.key_exchange_secret[..secret.bytes().len()].copy_from_slice(secret.bytes());

        self.server_name.clear();
        if let Some(name) = server_name {
            self.server_name
                .extend_from_slice(name.as_bytes())
                .map_err(|_| Error::InvalidConfiguration)?;
        }

        // Write TLS record header: placeholder (5 bytes), filled after encoding
        let mut off = 0;
        self.send_buffer[0] = ContentType::Handshake as u8;
        self.send_buffer[1] = 0x03;
        self.send_buffer[2] = 0x03;
        off = 5; // handshake message starts after record header

        message::encode_client_hello(
            &mut self.send_buffer,
            &mut off,
            &self.client_random,
            &[],
            C::CIPHER_SUITES,
            group,
            public.bytes(),
            server_name,
            alpn_protocols,
            C::KEY_EXCHANGE_GROUPS,
            C::SIGNATURE_SCHEMES,
            &self.config.supported_certificate_types,
        )?;

        // Fill record header length
        let body_len = (off - 5) as u16;
        self.send_buffer[3..5].copy_from_slice(&body_len.to_be_bytes());
        self.out_len = off;
        self.phase = Phase::ServerHello;
        Ok(ClientHandshakeEvent::Send)
    }

    /// Advance the handshake state machine.
    ///
    /// Must be called after each [`Send`](ClientHandshakeEvent::Send) or
    /// [`Receive`](ClientHandshakeEvent::Receive) event produced by the
    /// previous call.  The caller should:
    ///
    /// 1. Inspect the returned event.
    /// 2. If [`Send`](ClientHandshakeEvent::Send): transmit
    ///    [`outgoing_data`](Self::outgoing_data) over the network.
    /// 3. If [`Receive`](ClientHandshakeEvent::Receive): read data from the
    ///    network into [`receive_buffer`](Self::receive_buffer), then call
    ///    [`commit_received`](Self::commit_received).
    /// 4. Call `continue_handshake` again.
    ///
    /// When [`Done`](ClientHandshakeEvent::Done) is returned the connection
    /// is established and application data may be exchanged with
    /// [`encrypt`](Self::encrypt) / [`decrypt`](Self::decrypt).
    ///
    /// # Errors
    ///
    /// Returns an error if the server's messages are malformed, the
    /// certificate chain is invalid, the signature verification fails, or
    /// the transcript hash does not match the expected Finished verify_data.
    pub fn continue_handshake(&mut self) -> Result<ClientHandshakeEvent, Error> {
        match self.phase {
            Phase::ClientHello | Phase::ServerHello => self.process_server_hello(),
            Phase::ServerFlight => self.process_server_flight(),
            Phase::ClientFinished => {
                let ksz = self.key_size();
                let hs = self.hash_size();
                self.keys.write_key[..ksz].copy_from_slice(&self.app_write_key[..ksz]);
                self.keys.write_iv = self.app_write_iv;
                self.keys.write_traffic_secret[..hs].copy_from_slice(&self.app_write_traffic_secret[..hs]);
                self.keys.read_key[..ksz].copy_from_slice(&self.app_read_key[..ksz]);
                self.keys.read_iv = self.app_read_iv;
                self.keys.read_traffic_secret[..hs].copy_from_slice(&self.app_read_traffic_secret[..hs]);
                self.keys.read_seq = 0;
                self.keys.write_seq = 0;
                self.phase = Phase::ApplicationData;
                self.opened = true;
                self.clear_send_buffer();
                Ok(ClientHandshakeEvent::Done {
                    ciphersuite: self.ciphersuite.unwrap(),
                    tls_version: 0x0304,
                    key_exchange_group: self.key_exchange_group,
                    signature_scheme: self.server_signature_scheme.unwrap(),
                })
            }
            Phase::ApplicationData => Ok(ClientHandshakeEvent::Done {
                ciphersuite: self.ciphersuite.unwrap(),
                tls_version: 0x0304,
                key_exchange_group: self.key_exchange_group,
                signature_scheme: self.server_signature_scheme.unwrap(),
            }),
            Phase::Closed => Ok(ClientHandshakeEvent::Closed),
        }
    }

    /// Return the data that should be sent over the network.
    ///
    /// Used during both the handshake and application-data phases.  After
    /// [`encrypt`](Self::encrypt) or after
    /// [`Send`](ClientHandshakeEvent::Send) from the handshake, this returns
    /// the contents of the send buffer.
    pub fn outgoing_data(&self) -> &[u8] {
        &self.send_buffer[self.send_consumed..self.out_len]
    }

    /// Advance the sent position within the send buffer.
    ///
    /// Call this after transmitting `n` bytes from the slice returned by
    /// [`outgoing_data`](Self::outgoing_data).  The send buffer is reset
    /// automatically once all bytes have been sent.
    ///
    /// # Panics
    ///
    /// Panics if `n` exceeds the remaining unsent length.
    pub fn commit_sent(&mut self, n: usize) {
        assert!(n <= self.out_len - self.send_consumed);
        self.send_consumed += n;
        if self.send_consumed == self.out_len {
            self.send_consumed = 0;
            self.out_len = 0;
        }
    }

    /// Discard any data in the send buffer without sending it.
    ///
    /// Useful after the handshake to clear handshake messages from the
    /// buffer without transmitting them again.
    #[inline]
    fn clear_send_buffer(&mut self) {
        self.send_consumed = 0;
        self.out_len = 0;
    }

    // ── Application data ──
    /// Encrypt application data and write the resulting TLS record into the
    /// send buffer.
    ///
    /// Returns the number of plaintext bytes written (always equal to
    /// `data.len()`).  The encrypted record is available via
    /// [`outgoing_data`](Self::outgoing_data).
    ///
    /// # Panics
    ///
    /// Panics if called before the handshake has completed (i.e. before
    /// [`Done`](ClientHandshakeEvent::Done) is returned).
    ///
    /// # Errors
    ///
    /// Returns [`Error::InsufficientBuffer`] if the send buffer is too small
    /// for the encrypted record.
    pub fn encrypt(&mut self, data: &[u8]) -> Result<usize, Error> {
        let suite = self.ciphersuite.unwrap();
        let ksz = suite.key_size();
        let p = &self.config.crypto_provider;
        let total = encrypt_record(
            p,
            suite,
            &self.keys.write_key[..ksz],
            &self.keys.write_iv,
            self.keys.write_seq,
            ContentType::ApplicationData,
            data,
            self.send_buffer,
        )?;
        self.keys.write_seq += 1;
        self.out_len = total;
        self.send_consumed = 0;
        Ok(data.len())
    }

    /// Decrypt one or more TLS records from the receive buffer.
    ///
    /// Processes records in a loop until it runs out of complete records,
    /// encounters application data, receives a NewSessionTicket, or
    /// receives a close_notify alert.
    ///
    /// # Panics
    ///
    /// Panics if called before the handshake has completed.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ConnectionClosed`] when the peer sends a close_notify
    /// alert.  Returns [`Error::AeadError`] if record decryption fails.
    pub fn decrypt(&mut self) -> Result<ClientApplicationDataEvent, Error> {
        let suite = self.ciphersuite.unwrap();
        let hash_size = suite.hash_size();
        let key_size = suite.key_size();
        let crypto_provider = &self.config.crypto_provider;

        // Don't process new records while the caller still has unconsumed
        // application data from the previous record.
        if self.app_data_consumed < self.app_data_decrypted_len {
            return Ok(ClientApplicationDataEvent::AppData);
        }

        loop {
            let buf_end = self.receive_decoded + self.receive_pending;
            let buf = &self.receive_buffer[self.receive_decoded..buf_end];
            if buf.len() < RecordHeader::SIZE {
                return Ok(ClientApplicationDataEvent::None);
            }
            let Some((header, body)) = record::try_read_record(buf, buf.len())? else {
                return Ok(ClientApplicationDataEvent::None);
            };
            let total = RecordHeader::SIZE + header.length as usize;

            match header.content_type {
                ContentType::ApplicationData => {
                    let body_start = self.receive_decoded + RecordHeader::SIZE;
                    let receive_base = self.receive_buffer.as_ptr() as usize;
                    let (inner_type, payload) = decrypt_record(
                        crypto_provider,
                        suite,
                        &self.keys.read_key[..key_size],
                        &self.keys.read_iv,
                        self.keys.read_seq,
                        &header,
                        &mut self.receive_buffer[body_start..body_start + header.length as usize],
                    )?;
                    self.keys.read_seq += 1;
                    self.receive_decoded += total;
                    self.receive_pending -= total;

                    match inner_type {
                        ContentType::ApplicationData => {
                            let off = payload.as_ptr() as usize - receive_base;
                            self.app_data_offset = off;
                            self.app_data_decrypted_len = payload.len();
                            self.app_data_consumed = 0;
                            return Ok(ClientApplicationDataEvent::AppData);
                        }
                        ContentType::Alert => {
                            if payload.len() >= 2 && payload[0] == 1 && payload[1] == 0 {
                                self.close_received = true;
                                return Err(Error::ConnectionClosed);
                            }
                        }
                        ContentType::Handshake => {
                            if payload.len() < 4 {
                                continue;
                            }
                            let (msg_type, msg_body) = message::decode_handshake_frame(payload, &mut 0)?;
                            match msg_type {
                                message::HandshakeType::NewSessionTicket => {
                                    let ticket = message::decode_new_session_ticket(msg_body)?;
                                    let mut psk = [0u8; 48];
                                    key_schedule::derive_ticket_psk(
                                        crypto_provider,
                                        suite,
                                        &self.resumption_secret[..hash_size],
                                        ticket.nonce,
                                        &mut psk[..hash_size],
                                    )?;
                                    self.ticket_offset = ticket.ticket.as_ptr() as usize - receive_base;
                                    self.ticket_len = ticket.ticket.len();
                                    return Ok(ClientApplicationDataEvent::Ticket {
                                        psk,
                                        lifetime_s: ticket.lifetime_s,
                                        age_add: ticket.age_add,
                                    });
                                }
                                message::HandshakeType::KeyUpdate => {
                                    let _ = message::decode_key_update(msg_body)?;
                                    let mut nr = [0u8; 48];
                                    key_schedule::key_update_secret(
                                        crypto_provider,
                                        suite,
                                        &self.keys.read_traffic_secret[..hash_size],
                                        &mut nr[..hash_size],
                                    )?;
                                    self.keys.read_traffic_secret[..hash_size].copy_from_slice(&nr[..hash_size]);
                                    key_schedule::derive_traffic_keys(
                                        crypto_provider,
                                        suite,
                                        &nr[..hash_size],
                                        &mut self.keys.read_key,
                                        &mut self.keys.read_iv,
                                    )?;
                                    self.keys.read_seq = 0;
                                    let mut nw = [0u8; 48];
                                    key_schedule::key_update_secret(
                                        crypto_provider,
                                        suite,
                                        &self.keys.write_traffic_secret[..hash_size],
                                        &mut nw[..hash_size],
                                    )?;
                                    self.keys.write_traffic_secret[..hash_size].copy_from_slice(&nw[..hash_size]);
                                    key_schedule::derive_traffic_keys(
                                        crypto_provider,
                                        suite,
                                        &nw[..hash_size],
                                        &mut self.keys.write_key,
                                        &mut self.keys.write_iv,
                                    )?;
                                    self.keys.write_seq = 0;
                                    let mut ku = [0u8; 8];
                                    let mut ko = 0;
                                    ku[ko] = message::HandshakeType::KeyUpdate as u8;
                                    ko += 1;
                                    message::put_u24(&mut ku, &mut ko, 1);
                                    ku[ko] = 0;
                                    ko += 1;
                                    let mut resp_buf = [0u8; 256];
                                    let te = encrypt_record(
                                        crypto_provider,
                                        suite,
                                        &self.keys.write_key[..key_size],
                                        &self.keys.write_iv,
                                        self.keys.write_seq,
                                        ContentType::Handshake,
                                        &ku[..ko],
                                        &mut resp_buf,
                                    )?;
                                    self.keys.write_seq += 1;
                                    self.key_update_response.clear();
                                    self.key_update_response
                                        .extend_from_slice(&resp_buf[..te])
                                        .map_err(|_| Error::InsufficientBuffer)?;
                                    return Ok(ClientApplicationDataEvent::KeyUpdate);
                                }
                                _ => continue,
                            }
                        }
                        _ => continue,
                    }
                }
                ContentType::Alert => {
                    if body.len() >= 2 && body[0] == 1 && body[1] == 0 {
                        self.close_received = true;
                        self.receive_decoded += total;
                        self.receive_pending -= total;
                        return Err(Error::ConnectionClosed);
                    }
                    self.receive_decoded += total;
                    self.receive_pending -= total;
                }
                _ => {
                    self.receive_decoded += total;
                    self.receive_pending -= total;
                }
            }
        }
    }

    /// Get the last decrypted application data (after
    /// [`decrypt`](Self::decrypt) returned
    /// [`AppData`](ClientApplicationDataEvent::AppData)).
    ///
    /// Returns only the unconsumed portion of the data.  Advance the
    /// consumed position with [`commit_app_data`](Self::commit_app_data)
    /// so that subsequent calls return the remainder.
    pub fn received_app_data(&self) -> &[u8] {
        &self.receive_buffer
            [self.app_data_offset + self.app_data_consumed..self.app_data_offset + self.app_data_decrypted_len]
    }

    /// Advance the consumed position within the last decrypted record.
    ///
    /// Call this after reading `n` bytes from the slice returned by
    /// [`received_app_data`](Self::received_app_data).  The next call to
    /// [`decrypt`](Self::decrypt) will not process new records until all
    /// data from the current record has been consumed (i.e. the total
    /// consumed equals the record length).
    ///
    /// # Panics
    ///
    /// Panics if `n` exceeds the remaining unconsumed length.
    pub fn commit_app_data(&mut self, n: usize) {
        assert!(n <= self.app_data_decrypted_len - self.app_data_consumed);
        self.app_data_consumed += n;
    }

    /// Return the unsent portion of the pending KeyUpdate response, if any.
    ///
    /// The caller should transmit this data over the network and then call
    /// [`commit_key_update_data`](Self::commit_key_update_data) with the
    /// number of bytes successfully sent.
    pub fn outgoing_key_update_data(&self) -> &[u8] {
        &self.key_update_response[self.key_update_sent..]
    }

    /// Advance the sent position within the pending KeyUpdate response.
    ///
    /// Call this after transmitting `n` bytes from the slice returned by
    /// [`outgoing_key_update_data`](Self::outgoing_key_update_data).  The
    /// response is cleared automatically once all bytes have been sent.
    ///
    /// # Panics
    ///
    /// Panics if `n` exceeds the remaining unsent length.
    pub fn commit_key_update_data(&mut self, n: usize) {
        assert!(n <= self.key_update_response.len() - self.key_update_sent);
        self.key_update_sent += n;
        if self.key_update_sent == self.key_update_response.len() {
            self.key_update_response.clear();
            self.key_update_sent = 0;
        }
    }

    /// Get the raw ticket bytes from the last NewSessionTicket (after
    /// [`decrypt`](Self::decrypt) returned
    /// [`Ticket`](ClientApplicationDataEvent::Ticket)).
    pub fn received_ticket_data(&self) -> &[u8] {
        &self.receive_buffer[self.ticket_offset..self.ticket_offset + self.ticket_len]
    }

    /// Send a close_notify alert to the peer.
    ///
    /// The encrypted alert is returned.
    pub fn close(&mut self) -> Result<&[u8], Error> {
        let suite = self.ciphersuite.unwrap();
        let key_size = suite.key_size();
        let crypto_provider = &self.config.crypto_provider;
        let total = encrypt_record(
            crypto_provider,
            suite,
            &self.keys.write_key[..key_size],
            &self.keys.write_iv,
            self.keys.write_seq,
            ContentType::Alert,
            &[1u8, 0],
            self.send_buffer,
        )?;
        self.keys.write_seq += 1;
        self.out_len = total;
        self.send_consumed = 0;
        self.close_sent = true;
        Ok(self.outgoing_data())
    }

    pub fn is_established(&self) -> bool {
        self.opened
    }

    // Allow the wrapper to inspect buffer state for debugging
    #[doc(hidden)]
    pub fn receive_decoded(&self) -> usize {
        self.receive_decoded
    }
    #[doc(hidden)]
    pub fn receive_pending(&self) -> usize {
        self.receive_pending
    }
}

// ── Internal handshake processing ──

impl<'a, C: CryptoProvider> Client<'a, C> {
    fn process_server_hello(&mut self) -> Result<ClientHandshakeEvent, Error> {
        let buf_end = self.receive_decoded + self.receive_pending;
        let buf = &self.receive_buffer[self.receive_decoded..buf_end];
        if buf.len() < RecordHeader::SIZE {
            return Ok(ClientHandshakeEvent::Receive);
        }
        let Some((header, body)) = record::try_read_record(buf, buf.len())? else {
            return Ok(ClientHandshakeEvent::Receive);
        };
        let total = RecordHeader::SIZE + header.length as usize;

        match header.content_type {
            ContentType::Handshake => {
                let (msg_type, msg_body) = match message::decode_handshake_frame(body, &mut 0) {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };
                if msg_type != message::HandshakeType::ServerHello {
                    return Err(Error::UnexpectedMessage);
                }
                let sh = match message::decode_server_hello(msg_body) {
                    Ok(s) => s,
                    Err(e) => return Err(e),
                };
                self.server_random = sh.random;
                self.ciphersuite = Some(sh.cipher_suite);
                self.key_exchange_group = sh.key_share_group;
                let suite = sh.cipher_suite;
                let hash_size = suite.hash_size();
                let key_size = suite.key_size();
                let crypto_provider = &self.config.crypto_provider;

                let shared = crypto_provider.key_exchange(
                    &KeyExchangeSecretKey::new(sh.key_share_group, &self.key_exchange_secret[..32]),
                    sh.key_share_public,
                )?;

                let ch_len = self.out_len.checked_sub(5).unwrap_or(0);
                if ch_len > 0 {
                    let ch_msg = &self.send_buffer[5..5 + ch_len];
                    self.transcript_bytes
                        .extend_from_slice(ch_msg)
                        .map_err(|_| Error::CryptoError)?;
                }

                let blen = body.len().min(16384usize.saturating_sub(self.transcript_bytes.len()));
                self.transcript_bytes
                    .extend_from_slice(&body[..blen])
                    .map_err(|_| Error::CryptoError)?;

                let mut th = [0u8; 48];
                self.transcript_hash(suite, &mut th[..hash_size])?;

                let z: &[u8] = &[0u8; 48][..hash_size];
                let mut es = [0u8; 48];
                crypto_provider.hkdf_extract(&mut es[..hash_size], suite, z, z)?;

                let mut empty_hash = [0u8; 48];
                crypto_provider.hash(suite, &[], &mut empty_hash[..hash_size])?;

                let mut d = [0u8; 48];
                key_schedule::derive_secret(
                    crypto_provider,
                    suite,
                    &es[..hash_size],
                    b"derived",
                    &empty_hash[..hash_size],
                    &mut d[..hash_size],
                )?;

                crypto_provider.hkdf_extract(&mut self.keys.secret[..hash_size], suite, &d[..hash_size], &shared)?;

                let mut ch = [0u8; 48];
                let mut shs = [0u8; 48];
                key_schedule::derive_secret(
                    crypto_provider,
                    suite,
                    &self.keys.secret[..hash_size],
                    b"c hs traffic",
                    &th[..hash_size],
                    &mut ch[..hash_size],
                )?;
                key_schedule::derive_secret(
                    crypto_provider,
                    suite,
                    &self.keys.secret[..hash_size],
                    b"s hs traffic",
                    &th[..hash_size],
                    &mut shs[..hash_size],
                )?;
                key_schedule::derive_traffic_keys(
                    crypto_provider,
                    suite,
                    &ch[..hash_size],
                    &mut self.keys.write_key[..key_size],
                    &mut self.keys.write_iv,
                )?;
                key_schedule::derive_traffic_keys(
                    crypto_provider,
                    suite,
                    &shs[..hash_size],
                    &mut self.keys.read_key[..key_size],
                    &mut self.keys.read_iv,
                )?;
                self.keys.read_traffic_secret[..hash_size].copy_from_slice(&shs[..hash_size]);
                key_schedule::derive_finished_key(
                    crypto_provider,
                    suite,
                    &ch[..hash_size],
                    &mut self.handshake_client_finished_key[..hash_size],
                )?;
                key_schedule::derive_finished_key(
                    crypto_provider,
                    suite,
                    &shs[..hash_size],
                    &mut self.handshake_server_finished_key[..hash_size],
                )?;

                self.receive_decoded += total;
                self.receive_pending -= total;
                self.phase = Phase::ServerFlight;
                if self.receive_pending > 0 {
                    self.process_server_flight()
                } else {
                    Ok(ClientHandshakeEvent::Receive)
                }
            }
            ContentType::Alert => {
                if body.len() >= 2 {
                    return Err(Error::HandshakeAborted {
                        level: body[0],
                        description: body[1],
                    });
                }
                Err(Error::ConnectionClosed)
            }
            _ => Err(Error::UnexpectedMessage),
        }
    }

    /// Process the remaining flight after the ServerHello.
    ///
    /// This handles one or more encrypted records containing the server's
    /// EncryptedExtensions, Certificate, CertificateVerify and Finished
    /// messages.  Multiple handshake messages inside a single record are
    /// processed in order.  Returns [`Send`](ClientHandshakeEvent::Send) once
    /// the server's Finished has been verified and the client's own Finished
    /// has been written to the send buffer.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TranscriptMismatch`] if the server's Finished
    /// verify_data does not match.  Returns [`Error::InvalidCertificate`]
    /// or [`Error::InvalidSignature`] if the certificate chain or
    /// CertificateVerify fails validation.
    fn process_server_flight(&mut self) -> Result<ClientHandshakeEvent, Error> {
        let suite = self.ciphersuite.unwrap();
        let hs = suite.hash_size();
        let ksz = suite.key_size();
        let crypto_provider = &self.config.crypto_provider;

        loop {
            let len = self.receive_pending;
            if len < RecordHeader::SIZE {
                return Ok(ClientHandshakeEvent::Receive);
            }
            let start = self.receive_decoded;
            let buf_slice = &self.receive_buffer[start..start + len];
            let Some((header, _body)) = record::try_read_record(buf_slice, len)? else {
                return Ok(ClientHandshakeEvent::Receive);
            };
            let total = RecordHeader::SIZE + header.length as usize;

            match header.content_type {
                ContentType::ChangeCipherSpec => {
                    self.receive_decoded += total;
                    self.receive_pending -= total;
                    continue;
                }
                ContentType::ApplicationData => {
                    let bs = start + RecordHeader::SIZE;
                    let body_len = header.length as usize;
                    let (inner_type, payload) = decrypt_record(
                        crypto_provider,
                        suite,
                        &self.keys.read_key[..ksz],
                        &self.keys.read_iv,
                        self.keys.read_seq,
                        &header,
                        &mut self.receive_buffer[bs..bs + body_len],
                    )?;
                    self.keys.read_seq += 1;
                    self.receive_decoded += total;
                    self.receive_pending -= total;

                    match inner_type {
                        ContentType::Handshake => {
                            let mut pl_buf = [0u8; 16662];
                            let pl_len = payload.len();
                            pl_buf[..pl_len].copy_from_slice(payload);

                            let mut frame_off = 0;
                            while frame_off < pl_len {
                                let frame_start = frame_off;
                                let (msg_type, msg_body) = message::decode_handshake_frame(&pl_buf, &mut frame_off)?;
                                let frame_bytes = &pl_buf[frame_start..frame_off];

                                match msg_type {
                                    message::HandshakeType::EncryptedExtensions => {
                                        self.transcript_bytes
                                            .extend_from_slice(frame_bytes)
                                            .map_err(|_| Error::CryptoError)?;
                                        let (alpn_proto, cert_type) = message::decode_encrypted_extensions(msg_body)?;
                                        if let Some(proto) = alpn_proto {
                                            let mut b = [0u8; 255];
                                            let l = proto.len().min(255);
                                            b[..l].copy_from_slice(&proto[..l]);
                                            self.alpn = Some((b, l));
                                        }
                                        if let Some(ct) = cert_type {
                                            self.negotiated_cert_type = ct;
                                        }
                                    }
                                    message::HandshakeType::Certificate => {
                                        self.transcript_bytes
                                            .extend_from_slice(frame_bytes)
                                            .map_err(|_| Error::CryptoError)?;
                                        let cert = message::decode_certificate(msg_body, self.negotiated_cert_type)?;
                                        let server_name = if !self.server_name.is_empty() {
                                            core::str::from_utf8(&self.server_name).ok()
                                        } else {
                                            None
                                        };
                                        crypto_provider.verify_certificate(&cert, server_name)?;
                                        let (scheme, pk_vec) = extract_ee_key(&cert)?;
                                        self.server_public_key = pk_vec;
                                        self.server_signature_scheme = Some(scheme);
                                    }
                                    message::HandshakeType::CertificateVerify => {
                                        let mut th = [0u8; 48];
                                        self.transcript_hash(suite, &mut th[..hs])?;
                                        let cv = message::decode_certificate_verify(msg_body)?;
                                        let ctx = b"TLS 1.3, server CertificateVerify\x00";
                                        let mut s = [0u8; 200];
                                        let mut so = 0;
                                        s[..64].fill(0x20);
                                        so += 64;
                                        s[so..so + ctx.len()].copy_from_slice(ctx);
                                        so += ctx.len();
                                        s[so..so + hs].copy_from_slice(&th[..hs]);
                                        so += hs;
                                        crypto_provider.verify(
                                            cv.scheme,
                                            &self.server_public_key,
                                            &s[..so],
                                            cv.signature,
                                        )?;
                                        self.transcript_bytes
                                            .extend_from_slice(frame_bytes)
                                            .map_err(|_| Error::CryptoError)?;
                                    }
                                    message::HandshakeType::Finished => {
                                        let mut th = [0u8; 48];
                                        self.transcript_hash(suite, &mut th[..hs])?;
                                        let vd = message::decode_finished(msg_body)?;
                                        let mut exp = [0u8; 48];
                                        key_schedule::compute_finished(
                                            crypto_provider,
                                            suite,
                                            &self.handshake_server_finished_key[..hs],
                                            &th[..hs],
                                            &mut exp[..hs],
                                        )?;
                                        if vd != &exp[..hs] {
                                            return Err(Error::TranscriptMismatch);
                                        }

                                        self.transcript_bytes
                                            .extend_from_slice(frame_bytes)
                                            .map_err(|_| Error::CryptoError)?;

                                        let mut th2 = [0u8; 48];
                                        self.transcript_hash(suite, &mut th2[..hs])?;

                                        let mut eh = [0u8; 48];
                                        crypto_provider.hash(suite, &[], &mut eh[..hs])?;
                                        let mut d = [0u8; 48];
                                        key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret[..hs],
                                            b"derived",
                                            &eh[..hs],
                                            &mut d[..hs],
                                        )?;
                                        let z: &[u8] = &[0u8; 48][..hs];
                                        crypto_provider.hkdf_extract(
                                            &mut self.keys.secret[..hs],
                                            suite,
                                            &d[..hs],
                                            z,
                                        )?;

                                        let mut ca = [0u8; 48];
                                        let mut sa = [0u8; 48];
                                        key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret[..hs],
                                            b"c ap traffic",
                                            &th2[..hs],
                                            &mut ca[..hs],
                                        )?;
                                        key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret[..hs],
                                            b"s ap traffic",
                                            &th2[..hs],
                                            &mut sa[..hs],
                                        )?;
                                        key_schedule::derive_traffic_keys(
                                            crypto_provider,
                                            suite,
                                            &ca[..hs],
                                            &mut self.app_write_key[..ksz],
                                            &mut self.app_write_iv,
                                        )?;
                                        key_schedule::derive_traffic_keys(
                                            crypto_provider,
                                            suite,
                                            &sa[..hs],
                                            &mut self.app_read_key[..ksz],
                                            &mut self.app_read_iv,
                                        )?;
                                        self.app_write_traffic_secret[..hs].copy_from_slice(&ca[..hs]);
                                        self.app_read_traffic_secret[..hs].copy_from_slice(&sa[..hs]);

                                        let mut fv = [0u8; 48];
                                        key_schedule::compute_finished(
                                            crypto_provider,
                                            suite,
                                            &self.handshake_client_finished_key[..hs],
                                            &th2[..hs],
                                            &mut fv[..hs],
                                        )?;
                                        let mut fm = [0u8; 64];
                                        let mut fo = 0;
                                        message::encode_handshake_frame(
                                            &mut fm,
                                            &mut fo,
                                            message::HandshakeType::Finished,
                                            hs,
                                        );
                                        fm[fo..fo + hs].copy_from_slice(&fv[..hs]);
                                        fo += hs;
                                        let te = encrypt_record(
                                            crypto_provider,
                                            suite,
                                            &self.keys.write_key[..ksz],
                                            &self.keys.write_iv,
                                            self.keys.write_seq,
                                            ContentType::Handshake,
                                            &fm[..fo],
                                            self.send_buffer,
                                        )?;
                                        self.keys.write_seq += 1;
                                        self.out_len = te;
                                        self.send_consumed = 0;

                                        let mut rs = [0u8; 48];
                                        key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret[..hs],
                                            b"res master",
                                            &th2[..hs],
                                            &mut rs[..hs],
                                        )?;
                                        self.resumption_secret[..hs].copy_from_slice(&rs[..hs]);
                                        self.phase = Phase::ClientFinished;
                                        return Ok(ClientHandshakeEvent::Send);
                                    }
                                    _ => return Err(Error::UnexpectedMessage),
                                }
                            }
                        }
                        ContentType::Alert => {
                            return Err(Error::HandshakeAborted {
                                level: payload[0],
                                description: payload[1],
                            });
                        }
                        _ => return Err(Error::UnexpectedMessage),
                    }
                }
                ContentType::Alert => {
                    return Err(Error::HandshakeAborted {
                        level: _body[0],
                        description: _body[1],
                    });
                }
                _ => return Err(Error::UnexpectedMessage),
            }
        }
    }
}

// ── Events ──

/// Events produced by the handshake state machine.
///
/// Returned by [`start_handshake`](Client::start_handshake) and
/// [`continue_handshake`](Client::continue_handshake).
pub enum ClientHandshakeEvent {
    /// The caller should transmit
    /// [`outgoing_data`](Client::outgoing_data) over the
    /// network, then call [`continue_handshake`](Client::continue_handshake).
    Send,
    /// The caller should read data from the network into
    /// [`receive_buffer`](Client::receive_buffer), call
    /// [`commit_received`](Client::commit_received), then call
    /// [`continue_handshake`](Client::continue_handshake).
    Receive,
    /// The handshake is complete.  The negotiated parameters are included.
    Done {
        ciphersuite: CipherSuite,
        /// Wire-encoded protocol version (`0x0304` for TLS 1.3).
        tls_version: u16,
        key_exchange_group: KeyExchangeGroup,
        /// The signature scheme used by the server's CertificateVerify.
        signature_scheme: SignatureScheme,
    },
    /// The peer has closed the connection.
    Closed,
}

/// Events produced when decrypting application data.
///
/// Returned by [`Client::decrypt`].
pub enum ClientApplicationDataEvent {
    /// No complete TLS record is available in the receive buffer.
    None,
    /// Decrypted application data is available via
    /// [`Client::received_app_data`].
    AppData,
    /// A NewSessionTicket was received from the server.  The pre-shared key
    /// (`psk`), lifetime (`lifetime_s`), and obfuscated ticket age addition
    /// (`age_add`) are provided for session resumption.
    Ticket {
        psk: [u8; 48],
        lifetime_s: u32,
        age_add: u32,
    },
    /// A KeyUpdate was processed.  The caller should flush
    /// [`outgoing_data`](Client::outgoing_data) before
    /// reading more data.
    KeyUpdate,
}

/// Extract the EE public key from a [`ReceivedCertificate`] and determine
/// its [`SignatureScheme`] by probing the key length.
///
/// For `RawPublicKey` the scheme is taken directly from the enum.
/// For `X509` the public key is already parsed in the `ParsedCertificate`.
fn extract_ee_key(cert: &ReceivedCertificate) -> Result<(SignatureScheme, heapless::Vec<u8, 294>), Error> {
    match cert {
        ReceivedCertificate::RawPublicKey {
            public_key,
            scheme,
        } => {
            let mut pk = heapless::Vec::new();
            pk.extend_from_slice(public_key)
                .map_err(|_| Error::CertificateParseFailed)?;
            Ok((*scheme, pk))
        }
        ReceivedCertificate::X509 {
            chain,
        } => {
            let ee = chain.first().ok_or(Error::CertificateEmptyChain)?;
            detect_key_scheme(ee.public_key)
        }
    }
}

fn detect_key_scheme(key: &[u8]) -> Result<(SignatureScheme, heapless::Vec<u8, 294>), Error> {
    let mut pk = heapless::Vec::new();
    pk.extend_from_slice(key).map_err(|_| Error::CertificateParseFailed)?;
    let scheme = match key.len() {
        65 => SignatureScheme::EcdsaP256Sha256,
        97 => SignatureScheme::EcdsaP384Sha384,
        32 => SignatureScheme::Ed25519,
        _ if key.len() <= 294 => SignatureScheme::RsaPkcs1Sha256,
        _ => return Err(Error::CertificateParseFailed),
    };
    Ok((scheme, pk))
}
