use core::ops::{Deref, DerefMut};

use crate::{
    ALPN_PROTOCOL_MAX_SIZE, CertType, CipherSuite, CryptoProvider, Hash, KEY_EXCHANGE_MAX_GROUPS, KeyExchangeGroup,
    KeyExchangePublicKey, KeyExchangeSecretKey, PSK_MAX_SIZE, ReceivedCertificate, SIGNING_PUBLIC_KEY_MAX_SIZE,
    SignatureScheme,
    errors::Error,
    key_schedule, message,
    record::{self, ContentType, RecordHeader, decrypt_record, encrypt_record},
};

/// Trait for buffer types that can hold TLS record data.
///
/// Automatically implemented for any type that implements
/// [`Deref<Target = [u8]>`] + [`DerefMut`], such as `Vec<u8>` (owned),
/// `&mut [u8]` (borrowed), `Box<[u8]>`, and `bytes::BytesMut`.
pub trait Buffer: Deref<Target = [u8]> + DerefMut {}
impl<T: Deref<Target = [u8]> + DerefMut> Buffer for T {}

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
pub struct Client<B: Buffer, C: CryptoProvider> {
    pub(crate) config: ClientConfig<C>,
    pub(crate) receive_buffer: B,
    pub(crate) send_buffer: B,

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

    // ── Negotiated ──
    pub(crate) ciphersuite: Option<CipherSuite>,
    pub(crate) alpn: Option<heapless::Vec<u8, ALPN_PROTOCOL_MAX_SIZE>>,
    pub(crate) negotiated_cert_type: CertType,

    // ── Key exchange ──
    pub(crate) key_exchange_group: KeyExchangeGroup,
    pub(crate) key_exchange_pairs: heapless::Vec<KeyExchangeSecretKey, KEY_EXCHANGE_MAX_GROUPS>,

    // ── Key schedule ──
    pub(crate) keys: KeySchedule<C>,

    // ── Handshake-only (zeroed after Done) ──
    pub(crate) handshake_client_finished_key: Hash,
    pub(crate) handshake_server_finished_key: Hash,

    pub(crate) server_public_key: heapless::Vec<u8, SIGNING_PUBLIC_KEY_MAX_SIZE>,
    pub(crate) server_signature_scheme: Option<SignatureScheme>,
    pub(crate) server_name: heapless::Vec<u8, 256>,
    pub(crate) hash_state: Option<C::Hasher>,
    pub(crate) resumption_secret: Hash,
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

pub(crate) struct KeySchedule<C: CryptoProvider + ?Sized> {
    pub(crate) secret: Hash,
    pub(crate) read_key: Option<C::AeadKey>,
    pub(crate) read_iv: [u8; 12],
    pub(crate) read_seq: u64,
    pub(crate) read_traffic_secret: Hash,
    pub(crate) write_key: Option<C::AeadKey>,
    pub(crate) write_iv: [u8; 12],
    pub(crate) write_seq: u64,
    pub(crate) write_traffic_secret: Hash,
}

impl<C: CryptoProvider> KeySchedule<C> {
    fn new() -> Self {
        Self {
            secret: Hash::new_zeroed(48),
            read_key: None,
            read_iv: [0u8; 12],
            read_seq: 0,
            read_traffic_secret: Hash::new_zeroed(48),
            write_key: None,
            write_iv: [0u8; 12],
            write_seq: 0,
            write_traffic_secret: Hash::new_zeroed(48),
        }
    }
}

// ── Public API ──

impl<B: Buffer, C: CryptoProvider> Client<B, C> {
    /// Create a new TLS 1.3 client.
    ///
    /// `receive_buffer` and `send_buffer` are scratch buffers the client uses
    /// to hold incoming and outgoing TLS records.  Each must be at least
    /// [`MAX_RECORD_SIZE`] bytes long.
    ///
    /// Owned types such as `Vec<u8>` may be passed directly.  Borrowed
    /// slices (`&mut [u8]`) are also accepted — they must outlive the
    /// `Client`.
    pub fn new(config: ClientConfig<C>, receive_buffer: B, send_buffer: B) -> Self {
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
            ciphersuite: None,
            alpn: None,
            negotiated_cert_type: CertType::X509,
            key_exchange_group: C::KEY_EXCHANGE_GROUPS
                .first()
                .copied()
                .unwrap_or(KeyExchangeGroup::X25519),
            key_exchange_pairs: heapless::Vec::new(),
            keys: KeySchedule::new(),
            handshake_client_finished_key: Hash::new_zeroed(48),
            handshake_server_finished_key: Hash::new_zeroed(48),
            server_public_key: heapless::Vec::new(),
            server_signature_scheme: None,
            server_name: heapless::Vec::new(),
            hash_state: None,
            resumption_secret: Hash::new_zeroed(48),
        }
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
    ) -> Result<ClientHandshakeEvent<'_>, Error> {
        let crypto_provider = &self.config.crypto_provider;
        let mut client_random = [0u8; 32];
        crypto_provider.secure_random(&mut client_random);

        // Generate key pairs for ALL supported groups; send all in key_share.
        self.key_exchange_pairs.clear();
        let mut key_exchange_public_keys: heapless::Vec<KeyExchangePublicKey, KEY_EXCHANGE_MAX_GROUPS> =
            heapless::Vec::new();

        for group in C::KEY_EXCHANGE_GROUPS.iter().take(KEY_EXCHANGE_MAX_GROUPS) {
            let (secret, public) = crypto_provider.key_exchange_generate_keypair(*group)?;
            self.key_exchange_pairs
                .push(secret)
                .map_err(|_| Error::InvalidConfiguration)?;
            key_exchange_public_keys
                .push(public)
                .map_err(|_| Error::InvalidConfiguration)?;
        }

        self.key_exchange_group = C::KEY_EXCHANGE_GROUPS
            .first()
            .copied()
            .unwrap_or(KeyExchangeGroup::X25519);

        self.server_name.clear();
        if let Some(name) = server_name {
            self.server_name
                .extend_from_slice(name.as_bytes())
                .map_err(|_| Error::InvalidConfiguration)?;
        }

        // Write TLS record header: placeholder (5 bytes), filled after encoding
        self.send_buffer[0] = ContentType::Handshake as u8;
        self.send_buffer[1] = 0x03;
        self.send_buffer[2] = 0x03;
        let mut offset = 5; // handshake message starts after record header

        message::encode_client_hello(
            &mut self.send_buffer,
            &mut offset,
            &client_random,
            &[],
            C::CIPHER_SUITES,
            &key_exchange_public_keys,
            server_name,
            alpn_protocols,
            C::SIGNATURE_SCHEMES,
            &self.config.supported_certificate_types,
        )?;

        // Fill record header length
        let body_len = (offset - 5) as u16;
        self.send_buffer[3..5].copy_from_slice(&body_len.to_be_bytes());
        self.out_len = offset;
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
    pub fn continue_handshake(&mut self) -> Result<ClientHandshakeEvent<'_>, Error> {
        match self.phase {
            Phase::ClientHello | Phase::ServerHello => self.process_server_hello(),
            Phase::ServerFlight => self.process_server_flight(),
            Phase::ClientFinished => {
                self.keys.read_seq = 0;
                self.keys.write_seq = 0;
                self.phase = Phase::ApplicationData;
                self.opened = true;
                #[cfg(feature = "zeroize")]
                {
                    use zeroize::Zeroize;
                    self.handshake_client_finished_key.zeroize();
                    self.handshake_server_finished_key.zeroize();
                }
                self.hash_state = None;

                self.clear_send_buffer();
                Ok(ClientHandshakeEvent::Done {
                    ciphersuite: self.ciphersuite.unwrap(),
                    tls_version: 0x0304,
                    key_exchange_group: self.key_exchange_group,
                    signature_scheme: self.server_signature_scheme.unwrap(),
                    alpn: &self.alpn.as_ref().unwrap(),
                })
            }
            Phase::ApplicationData => Err(Error::HandshakeDone),
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
        let crypto_provider = &self.config.crypto_provider;
        let total = encrypt_record(
            crypto_provider,
            self.keys.write_key.as_ref().unwrap(),
            &self.keys.write_iv,
            self.keys.write_seq,
            ContentType::ApplicationData,
            data,
            &mut *self.send_buffer,
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
        // Don't process new records while the caller still has unconsumed
        // application data from the previous record.
        if self.app_data_consumed < self.app_data_decrypted_len {
            return Ok(ClientApplicationDataEvent::AppData);
        }

        self.compact_receive_buffer();

        let suite = self.ciphersuite.unwrap();
        let crypto_provider = &self.config.crypto_provider;
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
                        self.keys.read_key.as_ref().unwrap(),
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
                            let payload_offset = payload.as_ptr() as usize - receive_base;
                            self.app_data_offset = payload_offset;
                            self.app_data_decrypted_len = payload.len();
                            self.app_data_consumed = 0;
                            return Ok(ClientApplicationDataEvent::AppData);
                        }
                        ContentType::Alert => {
                            if payload.len() >= 2 && payload[0] == 1 && payload[1] == 0 {
                                self.phase = Phase::Closed;
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
                                    let psk = key_schedule::derive_ticket_psk(
                                        crypto_provider,
                                        suite,
                                        &self.resumption_secret,
                                        ticket.nonce,
                                    )?;
                                    self.ticket_offset = ticket.ticket.as_ptr() as usize - receive_base;
                                    self.ticket_len = ticket.ticket.len();
                                    return Ok(ClientApplicationDataEvent::Ticket {
                                        psk: heapless::Vec::from_slice(&psk).unwrap(), // TODO: avoid copy
                                        lifetime_s: ticket.lifetime_s,
                                        age_add: ticket.age_add,
                                    });
                                }
                                message::HandshakeType::KeyUpdate => {
                                    let request_update = message::decode_key_update(msg_body)?;
                                    let new_read_secret = key_schedule::key_update_secret(
                                        crypto_provider,
                                        suite,
                                        &self.keys.read_traffic_secret,
                                    )?;
                                    self.keys.read_traffic_secret = new_read_secret;
                                    let (read_key, read_iv) = key_schedule::derive_traffic_keys(
                                        crypto_provider,
                                        suite,
                                        &self.keys.read_traffic_secret,
                                    )?;
                                    self.keys.read_iv = read_iv;
                                    self.keys.read_key = Some(read_key);
                                    self.keys.read_seq = 0;
                                    if request_update == 1 {
                                        let new_write_secret = key_schedule::key_update_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.write_traffic_secret,
                                        )?;
                                        self.keys.write_traffic_secret = new_write_secret;
                                        let (write_key, write_iv) = key_schedule::derive_traffic_keys(
                                            crypto_provider,
                                            suite,
                                            &self.keys.write_traffic_secret,
                                        )?;
                                        self.keys.write_iv = write_iv;
                                        self.keys.write_key = Some(write_key);
                                        self.keys.write_seq = 0;
                                        let mut key_update_frame = [0u8; 8];
                                        let mut frame_offset = 0;
                                        key_update_frame[frame_offset] = message::HandshakeType::KeyUpdate as u8;
                                        frame_offset += 1;
                                        message::put_u24(&mut key_update_frame, &mut frame_offset, 1);
                                        key_update_frame[frame_offset] = 0;
                                        frame_offset += 1;
                                        let mut resp_buf = [0u8; 256];
                                        let total_encrypted = encrypt_record(
                                            crypto_provider,
                                            self.keys.write_key.as_ref().unwrap(),
                                            &self.keys.write_iv,
                                            self.keys.write_seq,
                                            ContentType::Handshake,
                                            &key_update_frame[..frame_offset],
                                            &mut resp_buf,
                                        )?;
                                        self.keys.write_seq += 1;
                                        self.key_update_response.clear();
                                        self.key_update_response
                                            .extend_from_slice(&resp_buf[..total_encrypted])
                                            .map_err(|_| Error::InsufficientBuffer)?;
                                    }
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
                        self.phase = Phase::Closed;
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
    #[inline]
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
    #[inline]
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
        let crypto_provider = &self.config.crypto_provider;
        let total = encrypt_record(
            crypto_provider,
            self.keys.write_key.as_ref().unwrap(),
            &self.keys.write_iv,
            self.keys.write_seq,
            ContentType::Alert,
            &[1u8, 0],
            &mut *self.send_buffer,
        )?;
        self.keys.write_seq += 1;
        self.out_len = total;
        self.send_consumed = 0;
        self.phase = Phase::Closed;
        Ok(self.outgoing_data())
    }

    pub fn is_established(&self) -> bool {
        self.opened
    }

    // ── Buffer management ──

    #[inline]
    fn compact_receive_buffer(&mut self) {
        if self.receive_decoded > 0 {
            let len = self.receive_pending;
            if len > 0 {
                self.receive_buffer
                    .copy_within(self.receive_decoded..self.receive_decoded + len, 0);
            }
            self.receive_decoded = 0;
        }
    }

    // ── Internal handshake processing ──

    fn process_server_hello(&mut self) -> Result<ClientHandshakeEvent<'_>, Error> {
        self.compact_receive_buffer();
        let buf_end = self.receive_decoded + self.receive_pending;
        let start = self.receive_decoded;
        if buf_end - start < RecordHeader::SIZE {
            return Ok(ClientHandshakeEvent::Receive);
        }
        // Read header directly to avoid borrowing the full buffer
        let header_len = u16::from_be_bytes([self.receive_buffer[start + 3], self.receive_buffer[start + 4]]) as usize;
        let total = RecordHeader::SIZE + header_len;
        if buf_end - start < total {
            return Ok(ClientHandshakeEvent::Receive);
        }

        self.receive_decoded += total;
        self.receive_pending -= total;

        let body_start = start + RecordHeader::SIZE;
        let body = &self.receive_buffer[body_start..body_start + header_len];
        let content_type = self.receive_buffer[start];

        match content_type {
            22 => {
                // ContentType::Handshake
                let (_msg_type, msg_body) = message::decode_handshake_frame(body, &mut 0)?;
                let server_hello = message::decode_server_hello(msg_body)?;
                self.ciphersuite = Some(server_hello.cipher_suite);
                self.key_exchange_group = server_hello.key_share_group;
                let suite = server_hello.cipher_suite;
                let hash_size = suite.hash_size();
                let crypto_provider = &self.config.crypto_provider;

                let secret = self
                    .key_exchange_pairs
                    .iter()
                    .find(|k| k.group() == server_hello.key_share_group)
                    .ok_or(Error::UnsupportedKeyExchangeGroup)?;

                let shared = crypto_provider.key_exchange(secret, server_hello.key_share_public)?;

                let client_hello_len = self.out_len.checked_sub(5).unwrap_or(0);
                if client_hello_len > 0 {
                    let client_hello_message = &self.send_buffer[5..5 + client_hello_len];
                    if let Some(ref mut state) = self.hash_state {
                        crypto_provider.hash_update(state, client_hello_message);
                    } else {
                        let mut state = crypto_provider.new_hash(suite);
                        crypto_provider.hash_update(&mut state, client_hello_message);
                        self.hash_state = Some(state);
                    }
                }

                if let Some(ref mut state) = self.hash_state {
                    crypto_provider.hash_update(state, body);
                } else {
                    let mut state = crypto_provider.new_hash(suite);
                    crypto_provider.hash_update(&mut state, body);
                    self.hash_state = Some(state);
                }

                let transcript_hash = if let Some(ref state) = self.hash_state {
                    let copy = state.clone();
                    crypto_provider.hash_finalize(copy)?
                } else {
                    crypto_provider.hash(suite, &[])?
                };

                let early_secret =
                    crypto_provider.hkdf_extract(suite, &Hash::new_zeroed(hash_size as u8), &[0u8; 48][..hash_size])?;
                let empty_hash = crypto_provider.hash(suite, &[])?;
                let derived_secret =
                    key_schedule::derive_secret(crypto_provider, suite, &early_secret, b"derived", &empty_hash)?;
                self.keys.secret = crypto_provider.hkdf_extract(suite, &derived_secret, &shared)?;

                let client_handshake_traffic_secret = key_schedule::derive_secret(
                    crypto_provider,
                    suite,
                    &self.keys.secret,
                    b"c hs traffic",
                    &transcript_hash,
                )?;
                let server_handshake_traffic_secret = key_schedule::derive_secret(
                    crypto_provider,
                    suite,
                    &self.keys.secret,
                    b"s hs traffic",
                    &transcript_hash,
                )?;

                let (write_key, write_iv) =
                    key_schedule::derive_traffic_keys(crypto_provider, suite, &client_handshake_traffic_secret)?;
                self.keys.write_iv = write_iv;
                self.keys.write_key = Some(write_key);

                let (read_key, read_iv) =
                    key_schedule::derive_traffic_keys(crypto_provider, suite, &server_handshake_traffic_secret)?;
                self.keys.read_iv = read_iv;
                self.keys.read_key = Some(read_key);

                self.handshake_client_finished_key =
                    key_schedule::derive_finished_key(crypto_provider, suite, &client_handshake_traffic_secret)?;
                self.handshake_server_finished_key =
                    key_schedule::derive_finished_key(crypto_provider, suite, &server_handshake_traffic_secret)?;

                self.phase = Phase::ServerFlight;
                if self.receive_pending > 0 {
                    self.process_server_flight()
                } else {
                    Ok(ClientHandshakeEvent::Receive)
                }
            }
            21 => {
                if body.len() >= 2 {
                    return Err(Error::HandshakeAborted {
                        level: body[0],
                        description: body[1],
                    });
                }
                Err(Error::DecodeError)
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
    fn process_server_flight(&mut self) -> Result<ClientHandshakeEvent<'_>, Error> {
        self.compact_receive_buffer();
        let suite = self.ciphersuite.unwrap();
        let hash_size = suite.hash_size();
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
                    let body_start = start + RecordHeader::SIZE;
                    let body_len = header.length as usize;
                    let (inner_type, payload) = decrypt_record(
                        crypto_provider,
                        self.keys.read_key.as_ref().unwrap(),
                        &self.keys.read_iv,
                        self.keys.read_seq,
                        &header,
                        &mut self.receive_buffer[body_start..body_start + body_len],
                    )?;
                    self.keys.read_seq += 1;
                    self.receive_decoded += total;
                    self.receive_pending -= total;

                    match inner_type {
                        ContentType::Handshake => {
                            // Phase 1: decode frame boundaries (shared borrow on payload)
                            struct HandshakeFrame {
                                msg_type: message::HandshakeType,
                                start: u16,
                                len: u16,
                            }
                            let mut frames: heapless::Vec<HandshakeFrame, 8> = heapless::Vec::new();
                            let pl_len = payload.len();
                            let mut frame_off = 0;
                            while frame_off < pl_len {
                                let frame_start = frame_off as u16;
                                let (msg_type, _) = message::decode_handshake_frame(payload, &mut frame_off)?;
                                frames
                                    .push(HandshakeFrame {
                                        msg_type,
                                        start: frame_start,
                                        len: (frame_off as u16) - frame_start,
                                    })
                                    .map_err(|_| Error::DecodeError)?;
                            }
                            // payload borrow ends here

                            // Phase 2: process frames with full self access
                            let body_base = start + RecordHeader::SIZE;
                            for frame in &frames {
                                let f_start = body_base + frame.start as usize;
                                let frame_bytes = &self.receive_buffer[f_start..f_start + frame.len as usize];
                                let msg_body = &frame_bytes[4..];

                                match frame.msg_type {
                                    message::HandshakeType::EncryptedExtensions => {
                                        if let Some(ref mut state) = self.hash_state {
                                            crypto_provider.hash_update(state, frame_bytes);
                                        } else {
                                            let mut state = crypto_provider.new_hash(suite);
                                            crypto_provider.hash_update(&mut state, frame_bytes);
                                            self.hash_state = Some(state);
                                        }
                                        let (alpn_proto, cert_type) = message::decode_encrypted_extensions(msg_body)?;
                                        if let Some(proto) = alpn_proto {
                                            let mut alpn_buffer = heapless::Vec::new();
                                            let alpn_length = proto.len().min(ALPN_PROTOCOL_MAX_SIZE);
                                            let _ = alpn_buffer.extend_from_slice(&proto[..alpn_length]);
                                            self.alpn = Some(alpn_buffer);
                                        }
                                        if let Some(ct) = cert_type {
                                            self.negotiated_cert_type = ct;
                                        }
                                    }
                                    message::HandshakeType::Certificate => {
                                        if let Some(ref mut state) = self.hash_state {
                                            crypto_provider.hash_update(state, frame_bytes);
                                        } else {
                                            let mut hasher = crypto_provider.new_hash(suite);
                                            crypto_provider.hash_update(&mut hasher, frame_bytes);
                                            self.hash_state = Some(hasher);
                                        }
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
                                        let transcript_hash = if let Some(ref state) = self.hash_state {
                                            let copy = state.clone();
                                            crypto_provider.hash_finalize(copy)?
                                        } else {
                                            crypto_provider.hash(suite, &[])?
                                        };
                                        let certificate_verify = message::decode_certificate_verify(msg_body)?;
                                        let ctx = b"TLS 1.3, server CertificateVerify\x00";
                                        let mut signed_content = [0u8; 200];
                                        let mut signed_offset = 0;
                                        signed_content[..64].fill(0x20);
                                        signed_offset += 64;
                                        signed_content[signed_offset..signed_offset + ctx.len()].copy_from_slice(ctx);
                                        signed_offset += ctx.len();
                                        signed_content[signed_offset..signed_offset + hash_size]
                                            .copy_from_slice(&transcript_hash);
                                        signed_offset += hash_size;
                                        crypto_provider.verify(
                                            certificate_verify.scheme,
                                            &self.server_public_key,
                                            &signed_content[..signed_offset],
                                            certificate_verify.signature,
                                        )?;
                                        if let Some(ref mut state) = self.hash_state {
                                            crypto_provider.hash_update(state, frame_bytes);
                                        } else {
                                            let mut state = crypto_provider.new_hash(suite);
                                            crypto_provider.hash_update(&mut state, frame_bytes);
                                            self.hash_state = Some(state);
                                        }
                                    }
                                    message::HandshakeType::Finished => {
                                        let transcript_hash = if let Some(ref state) = self.hash_state {
                                            let copy = state.clone();
                                            crypto_provider.hash_finalize(copy)?
                                        } else {
                                            crypto_provider.hash(suite, &[])?
                                        };
                                        let verify_data = message::decode_finished(msg_body)?;
                                        let expected_verify_data = key_schedule::compute_finished(
                                            crypto_provider,
                                            suite,
                                            &self.handshake_server_finished_key,
                                            &transcript_hash,
                                        )?;
                                        if verify_data != &*expected_verify_data {
                                            return Err(Error::TranscriptMismatch);
                                        }

                                        if let Some(ref mut state) = self.hash_state {
                                            crypto_provider.hash_update(state, frame_bytes);
                                        } else {
                                            let mut state = crypto_provider.new_hash(suite);
                                            crypto_provider.hash_update(&mut state, frame_bytes);
                                            self.hash_state = Some(state);
                                        }

                                        let final_transcript_hash = if let Some(ref state) = self.hash_state {
                                            let copy = state.clone();
                                            crypto_provider.hash_finalize(copy)?
                                        } else {
                                            crypto_provider.hash(suite, &[])?
                                        };

                                        let empty_hash = crypto_provider.hash(suite, &[])?;
                                        let derived_secret = key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret,
                                            b"derived",
                                            &empty_hash,
                                        )?;
                                        self.keys.secret = crypto_provider.hkdf_extract(
                                            suite,
                                            &derived_secret,
                                            &[0u8; 48][..hash_size],
                                        )?;

                                        let client_application_secret = key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret,
                                            b"c ap traffic",
                                            &final_transcript_hash,
                                        )?;
                                        let server_application_secret = key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret,
                                            b"s ap traffic",
                                            &final_transcript_hash,
                                        )?;
                                        let (write_key, write_iv) = key_schedule::derive_traffic_keys(
                                            crypto_provider,
                                            suite,
                                            &client_application_secret,
                                        )?;
                                        let (read_key, read_iv) = key_schedule::derive_traffic_keys(
                                            crypto_provider,
                                            suite,
                                            &server_application_secret,
                                        )?;
                                        self.keys.write_traffic_secret = client_application_secret;
                                        self.keys.read_traffic_secret = server_application_secret;

                                        let finished_verify_data = key_schedule::compute_finished(
                                            crypto_provider,
                                            suite,
                                            &self.handshake_client_finished_key,
                                            &final_transcript_hash,
                                        )?;
                                        let mut finished_frame = [0u8; 64];
                                        let mut finished_frame_offset = 0;
                                        message::encode_handshake_frame(
                                            &mut finished_frame,
                                            &mut finished_frame_offset,
                                            message::HandshakeType::Finished,
                                            hash_size,
                                        );
                                        finished_frame[finished_frame_offset..finished_frame_offset + hash_size]
                                            .copy_from_slice(&finished_verify_data);
                                        finished_frame_offset += hash_size;
                                        let total_encrypted = encrypt_record(
                                            crypto_provider,
                                            self.keys.write_key.as_ref().unwrap(),
                                            &self.keys.write_iv,
                                            self.keys.write_seq,
                                            ContentType::Handshake,
                                            &finished_frame[..finished_frame_offset],
                                            &mut *self.send_buffer,
                                        )?;
                                        self.keys.write_seq += 1;
                                        self.keys.write_key = Some(write_key);
                                        self.keys.write_iv = write_iv;
                                        self.keys.read_key = Some(read_key);
                                        self.keys.read_iv = read_iv;
                                        self.out_len = total_encrypted;
                                        self.send_consumed = 0;

                                        let resumption_secret = key_schedule::derive_secret(
                                            crypto_provider,
                                            suite,
                                            &self.keys.secret,
                                            b"res master",
                                            &final_transcript_hash,
                                        )?;
                                        self.resumption_secret = resumption_secret;
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
pub enum ClientHandshakeEvent<'a> {
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
        alpn: &'a [u8],
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
        psk: heapless::Vec<u8, PSK_MAX_SIZE>,
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
            let mut public_key_vec = heapless::Vec::new();
            public_key_vec
                .extend_from_slice(public_key)
                .map_err(|_| Error::CertificateParseFailed)?;
            Ok((*scheme, public_key_vec))
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
    let mut public_key_vec = heapless::Vec::new();
    public_key_vec
        .extend_from_slice(key)
        .map_err(|_| Error::CertificateParseFailed)?;
    let scheme = match key.len() {
        65 => SignatureScheme::EcdsaP256Sha256,
        97 => SignatureScheme::EcdsaP384Sha384,
        32 => SignatureScheme::Ed25519,
        _ if key.len() <= 294 => SignatureScheme::RsaPkcs1Sha256,
        _ => return Err(Error::CertificateParseFailed),
    };
    Ok((scheme, public_key_vec))
}
