//! QUIC server-side connection (RFC 9000 §7).
//!
//! `ServerConnection` handles an incoming QUIC connection from a client,
//! completes the TLS 1.3 handshake, and provides stream/datagram I/O.
//!
//! # Usage
//!
//! ```ignore
//! let mut server = quic::server::ServerConnection::new(transport, config);
//! server.accept().await?;
//! // use streams/datagrams
//! ```

use alloc::{
    collections::{BTreeMap, VecDeque},
    vec,
    vec::Vec,
};
use core::{net::SocketAddr, time::Duration};

use bytes::Bytes;
use hashbrown::HashMap;
use tls::QuicHandshakeEvent;

use crate::{
    ack::{AckRanges, AckTracker},
    cid::ConnectionId,
    cmd_queue::{CmdReceiver, CmdSender, cmd_queue},
    config::Config,
    connection::{CryptoBuffer, LevelSendState, build_transport_params, enc_varint, frames_from, send_one_packet},
    crypto_keys::{self, DirectionKeys},
    error::{Error, IoError},
    frame::{self, Frame},
    instant::Instant,
    loss::LossDetection,
    packet::{self, LongPacketType},
    stream::{ReceiveChunk, RecvFlowController, SendFlowController, Stream, StreamAllocator},
    tls_adapter::TlsAdapter,
    transport::Transport,
    transport_params::{self, Param, ParamType},
    varint,
};

/// Server-side QUIC connection.
pub struct ServerConnection<T: Transport> {
    transport: T,
    config: Config,
    remote: SocketAddr,
    state: ServerState,
    version: u32,
    dcid: ConnectionId,
    scid: ConnectionId,
    original_dcid: Option<ConnectionId>,
    init_send: LevelSendState,
    init_recv: DirectionKeys,
    hs_send: Option<LevelSendState>,
    hs_recv: Option<DirectionKeys>,
    app_send: Option<LevelSendState>,
    app_recv: Option<DirectionKeys>,
    app_traffic_secret: Option<Vec<u8>>,
    tls: Option<TlsAdapter>,
    pn_recv: [u64; 3],
    crypto_buffer: CryptoBuffer,
    crypto_buffer_hs: CryptoBuffer,
    streams: HashMap<u64, Stream>,
    stream_alloc: StreamAllocator,
    datagram_queue: VecDeque<Vec<u8>>,
    cmd_rx: CmdReceiver<crate::stream::StreamCommand>,
    base_cmd_tx: CmdSender<crate::stream::StreamCommand>,
    stream_data_tx: HashMap<u64, CmdSender<ReceiveChunk>>,
    pending_accepts: VecDeque<(u64, crate::stream::StreamDir)>,
    ack_tracker: [AckTracker; 3],
    loss_detect: [LossDetection; 3],
    last_activity: Instant,
    ack_deadline: [Option<Instant>; 3],
    send_flow: SendFlowController,
    recv_flow: RecvFlowController,
    idle_deadline: Instant,
}

enum ServerState {
    Accepting,
    Handshaking,
    Established,
    Closed,
}

impl<T: Transport> ServerConnection<T> {
    pub fn new(transport: T, config: Config) -> Self {
        let max_ack_delay = Duration::from_millis(config.max_ack_delay_ms);
        let initial_max_data = config.initial_max_data;
        let max_idle_timeout = Duration::from_millis(config.max_idle_timeout_ms);
        let (cmd_tx, cmd_rx) = cmd_queue();
        let now = transport.now();
        let (ck, sk) = crypto_keys::derive_initial_keys(config.tls_config.crypto_provider(), &[0u8; 8]);
        Self {
            transport,
            config,
            remote: "0.0.0.0:0".parse().unwrap(),
            state: ServerState::Accepting,
            version: 0,
            dcid: ConnectionId::new(&[0; 8]),
            scid: ConnectionId::random(8),
            original_dcid: None,
            init_send: LevelSendState {
                keys: sk,
                pn: 0,
            },
            init_recv: ck,
            hs_send: None,
            hs_recv: None,
            app_send: None,
            app_recv: None,
            app_traffic_secret: None,
            tls: None,
            pn_recv: [0; 3],
            crypto_buffer: CryptoBuffer::new(),
            crypto_buffer_hs: CryptoBuffer::new(),
            streams: HashMap::new(),
            stream_alloc: StreamAllocator::new(),
            datagram_queue: VecDeque::new(),
            cmd_rx,
            base_cmd_tx: cmd_tx,
            stream_data_tx: HashMap::new(),
            pending_accepts: VecDeque::new(),
            ack_tracker: [AckTracker::new(), AckTracker::new(), AckTracker::new()],
            loss_detect: [
                LossDetection::new(max_ack_delay),
                LossDetection::new(max_ack_delay),
                LossDetection::new(max_ack_delay),
            ],
            last_activity: now,
            ack_deadline: [None, None, None],
            send_flow: SendFlowController::new(initial_max_data),
            recv_flow: RecvFlowController::new(initial_max_data),
            idle_deadline: now + max_idle_timeout,
        }
    }

    /// Accept an incoming QUIC connection.
    ///
    /// Drives the full TLS 1.3 handshake using the [`Config`] and [`Transport`]
    /// provided at construction. Blocks until the handshake is complete or fails.
    ///
    /// This method handles all packet I/O internally — no manual
    /// [`receive_one`](Self::receive_one) calls are needed during the handshake.
    pub async fn accept(&mut self) -> Result<(), Error> {
        // Receive first datagram from the client
        let mut buf = vec![0u8; self.config.recv_buf_size];
        let (n, src) = self
            .transport
            .receive_from(&mut buf, Some(Duration::from_secs(10)))
            .await
            .map_err(|_| Error::ConnectionTimedOut)?;
        let data = &buf[..n];

        // Parse the Initial packet header
        let header = packet::parse_long_header(data)?;
        if header.ty != LongPacketType::Initial {
            return Err(Error::ProtocolViolation("expected Initial packet".into()));
        }

        self.remote = src;
        self.last_activity = self.clock();
        self.version = header.version;
        self.dcid = header.scid.clone();
        self.original_dcid = Some(header.dcid.clone());

        // Derive Initial keys
        let (ck, sk) = crypto_keys::derive_initial_keys_for_version(
            self.config.tls_config.crypto_provider(),
            header.dcid.as_bytes(),
            self.version,
        );
        self.init_recv = ck;
        self.init_send = LevelSendState {
            keys: sk,
            pn: 0,
        };
        self.pn_recv[0] = 0;

        // Remove header protection and decrypt
        let (pkt, _pn) = self.decrypt_initial(&header, data)?;
        self.pn_recv[0] = _pn + 1;

        // Track received packet for ACK
        let now = self.clock();
        self.ack_tracker[0].on_packet_received(now, _pn, true);
        self.loss_detect[0].on_packet_received(now, true);

        // Extract CRYPTO frames
        let frames = frames_from(&pkt)?;
        let mut crypto_data = Vec::new();
        for f in &frames {
            if let Frame::Crypto {
                offset,
                data: d,
            } = f
            {
                if let Some(contiguous) = self.crypto_buffer.ingest(*offset, d) {
                    crypto_data.push(contiguous);
                }
            }
        }
        if crypto_data.is_empty() {
            return Err(Error::ProtocolViolation("no CRYPTO frame in Initial".into()));
        }

        // Build transport params
        let tps = transport_params::encode(&build_transport_params(&self.config, &self.scid));

        // Set up TLS server connection
        let tls_server_config = match &self.config.tls_config {
            crate::config::TlsConfig::Server(cfg) => cfg.clone(),
            crate::config::TlsConfig::Client(_) => {
                return Err(Error::InvalidState("cannot use client TLS config for server connection".into()));
            }
        };
        let mut tls_server = tls::ServerConnection::new_quic(tls_server_config);
        tls_server.set_quic_transport_params(&tps);
        let mut adapter = TlsAdapter::new(tls_server);

        // Feed client CRYPTO data and process
        for chunk in &crypto_data {
            adapter.inject_handshake(chunk);
        }
        adapter
            .process()
            .await
            .map_err(|e| Error::ConnectionRejected(format!("TLS error: {e}")))?;

        // Send ServerHello at Initial level
        if let Some(sh) = adapter.write_handshake() {
            self.send_frames_at_level(
                0,
                true,
                true,
                &[Frame::Crypto {
                    offset: 0,
                    data: sh,
                }],
            )
            .await?;
        }

        // Derive Handshake keys now that we have ServerHello processed
        if let Some(suite) = adapter.cipher_suite() {
            if let Some(s) = adapter.quic_secrets() {
                let provider = self.config.tls_config.crypto_provider();
                let lh = crypto_keys::derive_level_keys(
                    provider.clone(),
                    s.client_handshake_traffic_secret.as_slice(),
                    suite,
                );
                let rh = crypto_keys::derive_level_keys(
                    provider.clone(),
                    s.server_handshake_traffic_secret.as_slice(),
                    suite,
                );
                self.hs_recv = Some(lh);
                self.hs_send = Some(LevelSendState {
                    keys: rh,
                    pn: 0,
                });
            }
        }

        self.tls = Some(adapter);

        // Send remaining handshake messages (EE, Cert, CV, Fin) at Handshake level
        // with proper CRYPTO stream offset tracking.
        {
            let tls = self.tls.as_mut().unwrap();
            let mut crypto_offset: u64 = 0;
            let mut out = Vec::new();
            while let Some(d) = tls.write_handshake() {
                out.push((crypto_offset, d));
                crypto_offset += out.last().unwrap().1.len() as u64;
            }
            for (offset, data) in &out {
                self.send_frames_at_level(
                    1,
                    true,
                    false,
                    &[Frame::Crypto {
                        offset: *offset,
                        data: data.clone(),
                    }],
                )
                .await?;
            }
        }

        // Send ACK for the received packet
        self.send_ack(0).await?;

        // Handshake loop
        let deadline = self.clock() + Duration::from_secs(10);
        let idle_timeout = Duration::from_millis(self.config.max_idle_timeout_ms);
        loop {
            if self.is_established() {
                return Ok(());
            }
            if self.clock() > deadline {
                return Err(Error::ConnectionTimedOut);
            }
            if self.clock().duration_since(self.last_activity) >= idle_timeout {
                return Err(Error::ConnectionTimedOut);
            }
            self.check_pto_and_retransmit().await?;
            let result = self.receive_one().await;
            match result {
                Ok(()) => {}
                Err(Error::ConnectionClosed(_, _)) => return result,
                Err(e) => return Err(e),
            }
        }
    }

    pub fn is_established(&self) -> bool {
        matches!(self.state, ServerState::Established)
    }

    /// Drive one I/O cycle: check PTO, receive and process one datagram.
    pub async fn receive_one(&mut self) -> Result<(), Error> {
        self.check_pto_and_retransmit().await?;

        let buf_size = self.config.recv_buf_size;
        let mut buf = vec![0u8; buf_size];
        let recv_result = self
            .transport
            .receive_from(&mut buf, Some(Duration::from_millis(50)))
            .await;

        match recv_result {
            Ok((0, _)) => Ok(()),
            Ok((n, _src)) => {
                self.last_activity = self.clock();
                let mut data = &buf[..n];
                while !data.is_empty() {
                    let consumed = if data[0] >> 7 == 1 {
                        self.process_long(data).await?
                    } else {
                        self.process_short(data).await?
                    };
                    if consumed >= data.len() {
                        break;
                    }
                    data = &data[consumed..];
                }
                Ok(())
            }
            Err(IoError::WouldBlock | IoError::TimedOut) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn clock(&self) -> Instant {
        self.transport.now()
    }

    // ── Long header processing ────────────────────────────────────────

    async fn process_long(&mut self, data: &[u8]) -> Result<usize, Error> {
        let header = packet::parse_long_header(data)?;
        let pkt_end = header.pn_offset + header.payload_length as usize;
        if pkt_end > data.len() {
            return Err(Error::PacketDecode("payload_length exceeds available data".into()));
        }
        let pkt = &data[..pkt_end];
        match header.ty {
            LongPacketType::Initial => self.process_initial(&header, pkt).await,
            LongPacketType::Handshake => self.process_handshake(&header, pkt).await,
            _ => Err(Error::ProtocolViolation("unexpected long packet type".into())),
        }
    }

    async fn process_initial(&mut self, h: &packet::LongHeader, pkt: &[u8]) -> Result<usize, Error> {
        let (payload, pn) = self.decrypt_initial(h, pkt)?;
        self.pn_recv[0] = pn + 1;
        let now = self.clock();
        self.ack_tracker[0].on_packet_received(now, pn, true);
        self.loss_detect[0].on_packet_received(now, true);
        self.send_ack(0).await?;
        let pkt_end = h.pn_offset + h.payload_length as usize;
        let frames = frames_from(&payload)?;
        self.handle_crypto(frames, 0).await?;
        Ok(pkt_end)
    }

    async fn process_handshake(&mut self, h: &packet::LongHeader, pkt: &[u8]) -> Result<usize, Error> {
        let now = self.clock();
        let rk = self
            .hs_recv
            .as_ref()
            .ok_or(Error::InvalidState("no HS recv keys".into()))?;
        let sample_start = h.pn_offset + 4;
        if sample_start + 16 > pkt.len() {
            return Err(Error::PacketDecode("packet too short for HP sample".into()));
        }
        let sample = &pkt[sample_start..sample_start + 16];
        let mut fb = pkt[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(rk, true, &mut fb, &mut pn_b, sample);
        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], self.pn_recv[1]);
        let payload_offset = h.pn_offset + pn_len;
        let encrypted_end = h.pn_offset + h.payload_length as usize;
        if encrypted_end > pkt.len() || encrypted_end < payload_offset {
            return Err(Error::PacketDecode("payload_length inconsistent".into()));
        }
        let mut aad = pkt[..payload_offset].to_vec();
        aad[0] = fb;
        aad[h.pn_offset..h.pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);
        let mut payload = pkt[payload_offset..encrypted_end].to_vec();
        crypto_keys::decrypt_payload(rk, pn, &aad, &mut payload)?;
        self.pn_recv[1] = pn + 1;

        self.ack_tracker[1].on_packet_received(now, pn, true);
        self.loss_detect[1].on_packet_received(now, true);
        self.send_ack(1).await?;

        let frames = frames_from(&payload)?;
        self.handle_crypto(frames, 1).await?;
        Ok(encrypted_end)
    }

    async fn process_short(&mut self, data: &[u8]) -> Result<usize, Error> {
        let dcid_len = self.scid.len();
        if data.len() < 1 + dcid_len + 4 {
            return Err(Error::PacketDecode("short packet too short".into()));
        }
        let pn_offset = 1 + dcid_len;
        let sample_start = pn_offset + 4;
        if sample_start + 16 > data.len() {
            return Err(Error::PacketDecode("short packet too short for HP sample".into()));
        }
        let sample = &data[sample_start..sample_start + 16];
        let rk = self
            .app_recv
            .as_ref()
            .ok_or(Error::InvalidState("no app recv keys".into()))?;
        let mut fb = data[0];
        let mut pn_b = data[pn_offset..pn_offset + 4].to_vec();
        crypto_keys::remove_header_protection(rk, false, &mut fb, &mut pn_b, sample);
        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], self.pn_recv[2]);
        let payload_offset = pn_offset + pn_len;
        let pkt_end = data.len();
        let mut aad = data[..payload_offset].to_vec();
        aad[0] = fb;
        aad[pn_offset..pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);
        let mut payload = data[payload_offset..pkt_end].to_vec();
        crypto_keys::decrypt_payload(rk, pn, &aad, &mut payload)?;
        self.pn_recv[2] = pn + 1;

        let now = self.clock();
        let frames = frames_from(&payload)?;
        let has_ack_eliciting = frames.iter().any(|f| !matches!(f, Frame::Ack { .. } | Frame::Padding));
        self.ack_tracker[2].on_packet_received(now, pn, has_ack_eliciting);
        self.loss_detect[2].on_packet_received(now, has_ack_eliciting);
        if has_ack_eliciting {
            self.schedule_ack(2);
        }

        self.handle_post_handshake_frames(frames).await?;
        Ok(pkt_end)
    }

    // ── Decryption helper ─────────────────────────────────────────────

    fn decrypt_initial(&self, h: &packet::LongHeader, pkt: &[u8]) -> Result<(Vec<u8>, u64), Error> {
        let sample_start = h.pn_offset + 4;
        if sample_start + 16 > pkt.len() {
            return Err(Error::PacketDecode("packet too short for HP sample".into()));
        }
        let sample = &pkt[sample_start..sample_start + 16];
        let mut fb = pkt[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(&self.init_recv, true, &mut fb, &mut pn_b, sample);
        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], self.pn_recv[0]);
        let payload_offset = h.pn_offset + pn_len;
        let encrypted_end = h.pn_offset + h.payload_length as usize;
        if encrypted_end > pkt.len() || encrypted_end < payload_offset {
            return Err(Error::PacketDecode("payload_length inconsistent".into()));
        }
        let mut aad = pkt[..payload_offset].to_vec();
        aad[0] = fb;
        aad[h.pn_offset..h.pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);
        let mut payload = pkt[payload_offset..encrypted_end].to_vec();
        crypto_keys::decrypt_payload(&self.init_recv, pn, &aad, &mut payload)?;
        Ok((payload, pn))
    }

    // ── CRYPTO / TLS handling ─────────────────────────────────────────

    async fn handle_crypto(&mut self, frames: Vec<Frame>, space: usize) -> Result<(), Error> {
        let now = self.clock();
        for f in frames {
            match f {
                Frame::Crypto {
                    offset,
                    data,
                } => {
                    let buf = if space == 1 {
                        &mut self.crypto_buffer_hs
                    } else {
                        &mut self.crypto_buffer
                    };
                    if let Some(contiguous) = buf.ingest(offset, &data) {
                        self.process_crypto_data(&contiguous).await?;
                    }
                }
                Frame::Ack {
                    largest_acknowledged,
                    ack_delay,
                    first_ack_range,
                    ack_ranges,
                } => {
                    let ranges = AckRanges {
                        largest: largest_acknowledged,
                        delay: ack_delay,
                        first_range: first_ack_range,
                        extra_ranges: ack_ranges,
                    };
                    let (acked, sent_times) = self.ack_tracker[space].on_ack_received(&ranges);
                    if !acked.is_empty() {
                        self.loss_detect[space].on_ack_received(now);
                        if let Some(time_sent) = sent_times.last() {
                            let rtt = now.duration_since(*time_sent);
                            self.loss_detect[space].on_rtt_measurement(rtt, Duration::from_micros(ack_delay));
                        }
                    }
                }
                Frame::ConnectionClose {
                    error_code,
                    reason_phrase,
                    ..
                } => {
                    self.state = ServerState::Closed;
                    return Err(Error::ConnectionClosed(
                        error_code,
                        String::from_utf8_lossy(&reason_phrase).into(),
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn process_crypto_data(&mut self, data: &[u8]) -> Result<(), Error> {
        let provider = self.config.tls_config.crypto_provider();
        let tls = self.tls.as_mut().unwrap();
        tls.inject_handshake(data);
        match tls
            .process()
            .await
            .map_err(|e| Error::ConnectionRejected(format!("TLS error: {e}")))?
        {
            QuicHandshakeEvent::HandshakeComplete => {
                let suite = tls
                    .cipher_suite()
                    .ok_or(Error::InvalidState("no cipher suite".into()))?;
                let s = tls.quic_secrets().unwrap();
                if self.hs_send.is_none() {
                    let lh = crypto_keys::derive_level_keys(
                        provider.clone(),
                        s.client_handshake_traffic_secret.as_slice(),
                        suite,
                    );
                    let rh = crypto_keys::derive_level_keys(
                        provider.clone(),
                        s.server_handshake_traffic_secret.as_slice(),
                        suite,
                    );
                    self.hs_recv = Some(lh);
                    self.hs_send = Some(LevelSendState {
                        keys: rh,
                        pn: 0,
                    });
                }
                if let Some(fin) = tls.write_handshake() {
                    self.send_frames_at_level(
                        1,
                        true,
                        false,
                        &[Frame::Crypto {
                            offset: 0,
                            data: fin,
                        }],
                    )
                    .await?;
                }
                let la = crypto_keys::derive_level_keys(
                    provider.clone(),
                    s.client_application_traffic_secret.as_slice(),
                    suite,
                );
                let ra = crypto_keys::derive_level_keys(
                    provider.clone(),
                    s.server_application_traffic_secret.as_slice(),
                    suite,
                );
                self.app_traffic_secret = Some(s.server_application_traffic_secret.to_vec());
                self.app_recv = Some(la);
                self.app_send = Some(LevelSendState {
                    keys: ra,
                    pn: 0,
                });
                self.state = ServerState::Established;
            }
            _ => {
                if self.hs_send.is_none() {
                    if let Some(suite) = tls.cipher_suite() {
                        if let Some(s) = tls.quic_secrets() {
                            let lh = crypto_keys::derive_level_keys(
                                provider.clone(),
                                s.client_handshake_traffic_secret.as_slice(),
                                suite,
                            );
                            let rh = crypto_keys::derive_level_keys(
                                provider.clone(),
                                s.server_handshake_traffic_secret.as_slice(),
                                suite,
                            );
                            self.hs_recv = Some(lh);
                            self.hs_send = Some(LevelSendState {
                                keys: rh,
                                pn: 0,
                            });
                        }
                    }
                }
                self.flush_tls_hs().await?;
            }
        }
        Ok(())
    }

    /// Flush outgoing TLS data at Handshake level with proper CRYPTO offset tracking.
    async fn flush_tls_hs(&mut self) -> Result<(), Error> {
        let mut out = Vec::new();
        {
            let tls = self.tls.as_mut().unwrap();
            let mut crypto_offset: u64 = 0;
            while let Some(d) = tls.write_handshake() {
                out.push((crypto_offset, d.clone()));
                crypto_offset += d.len() as u64;
            }
        }
        for (offset, data) in out {
            self.send_frames_at_level(
                1,
                true,
                false,
                &[Frame::Crypto {
                    offset,
                    data,
                }],
            )
            .await?;
        }
        Ok(())
    }

    // ── Post-handshake frame handling ─────────────────────────────────

    async fn handle_post_handshake_frames(&mut self, frames: Vec<Frame>) -> Result<(), Error> {
        let now = self.clock();
        for f in frames {
            match f {
                Frame::Ack {
                    largest_acknowledged,
                    ack_delay,
                    first_ack_range,
                    ack_ranges,
                } => {
                    let ranges = AckRanges {
                        largest: largest_acknowledged,
                        delay: ack_delay,
                        first_range: first_ack_range,
                        extra_ranges: ack_ranges,
                    };
                    let (acked, sent_times) = self.ack_tracker[2].on_ack_received(&ranges);
                    if !acked.is_empty() {
                        self.loss_detect[2].on_ack_received(now);
                        if let Some(time_sent) = sent_times.last() {
                            let rtt = now.duration_since(*time_sent);
                            self.loss_detect[2].on_rtt_measurement(rtt, Duration::from_micros(ack_delay));
                        }
                    }
                }
                Frame::Stream {
                    id,
                    data: d,
                    fin,
                    offset,
                } => {
                    let dlen = d.len() as u64;
                    self.recv_flow.on_received(dlen);
                    let is_new_stream = !self.streams.contains_key(&id);
                    self.streams.entry(id).or_insert_with(|| {
                        let mut s = Stream::new(id);
                        s.local_max_stream_data = self.config.initial_max_stream_data_bidi_local;
                        s.max_stream_data = self.config.initial_max_stream_data_bidi_remote;
                        s
                    });
                    if let Some(s) = self.streams.get_mut(&id) {
                        s.recv_buffer.extend_from_slice(&d);
                        s.recv_offset = s.recv_offset.max(offset + dlen);
                        if fin {
                            s.fin_received = true;
                        }
                        let consumed = s.local_max_stream_data.saturating_sub(s.recv_offset);
                        if consumed <= s.local_max_stream_data / 2 {
                            s.local_max_stream_data += s.local_max_stream_data / 2;
                            s.needs_max_stream_data = true;
                        }
                    }
                    let chunk = ReceiveChunk {
                        data: d.clone(),
                        fin,
                    };
                    if let Some(tx) = self.stream_data_tx.get(&id) {
                        let _ = tx.push(chunk);
                    }
                    if is_new_stream {
                        let dir = if id & 0x02 != 0 {
                            crate::stream::StreamDir::Uni
                        } else {
                            crate::stream::StreamDir::Bi
                        };
                        self.pending_accepts.push_back((id, dir));
                    }
                }
                Frame::Datagram {
                    data: d,
                } => self.datagram_queue.push_back(d),
                Frame::ConnectionClose {
                    error_code,
                    reason_phrase,
                    ..
                } => {
                    self.state = ServerState::Closed;
                    return Err(Error::ConnectionClosed(
                        error_code,
                        String::from_utf8_lossy(&reason_phrase).into(),
                    ));
                }
                Frame::PathChallenge {
                    data,
                } => {
                    self.send_frames_at_level(
                        2,
                        false,
                        false,
                        &[Frame::PathResponse {
                            data,
                        }],
                    )
                    .await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    // ── Packet sending ────────────────────────────────────────────────

    async fn send_frames_at_level(
        &mut self,
        level: u8,
        long_header: bool,
        is_initial: bool,
        frames: &[Frame],
    ) -> Result<(), Error> {
        let kp = 0u8;
        match level {
            0 => {
                send_one_packet(
                    &self.transport,
                    self.remote,
                    &self.dcid,
                    &self.scid,
                    &mut self.init_send,
                    long_header,
                    is_initial,
                    frames,
                    &mut self.ack_tracker[0],
                    &mut self.loss_detect[0],
                    &mut self.last_activity,
                    level,
                    self.version,
                    kp,
                )
                .await?;
            }
            1 => {
                let ss = self
                    .hs_send
                    .as_mut()
                    .ok_or(Error::InvalidState("no HS send keys".into()))?;
                send_one_packet(
                    &self.transport,
                    self.remote,
                    &self.dcid,
                    &self.scid,
                    ss,
                    long_header,
                    is_initial,
                    frames,
                    &mut self.ack_tracker[1],
                    &mut self.loss_detect[1],
                    &mut self.last_activity,
                    level,
                    self.version,
                    kp,
                )
                .await?;
            }
            2 => {
                let ss = self
                    .app_send
                    .as_mut()
                    .ok_or(Error::InvalidState("no app send keys".into()))?;
                send_one_packet(
                    &self.transport,
                    self.remote,
                    &self.dcid,
                    &self.scid,
                    ss,
                    long_header,
                    is_initial,
                    frames,
                    &mut self.ack_tracker[2],
                    &mut self.loss_detect[2],
                    &mut self.last_activity,
                    level,
                    self.version,
                    kp,
                )
                .await?;
            }
            _ => return Err(Error::InvalidState("unknown encryption level".into())),
        }
        Ok(())
    }

    async fn send_ack(&mut self, space: usize) -> Result<(), Error> {
        let now = self.clock();
        let ack_delay_us = if let Some(first) = self.ack_tracker[space].first_ack_eliciting {
            now.duration_since(first).as_micros().min(u64::MAX as u128) as u64
        } else {
            0
        };
        let ranges = self.ack_tracker[space].build_ack(ack_delay_us);
        self.ack_deadline[space] = None;
        let ack_frame = Frame::Ack {
            largest_acknowledged: ranges.largest,
            ack_delay: ranges.delay,
            first_ack_range: ranges.first_range,
            ack_ranges: ranges.extra_ranges,
        };
        let long_header = space != 2;
        let is_initial = space == 0;
        self.send_frames_at_level(space as u8, long_header, is_initial, &[ack_frame])
            .await
    }

    fn schedule_ack(&mut self, space: usize) {
        if self.ack_deadline[space].is_none() {
            let deadline = self.clock() + Duration::from_millis(self.config.max_ack_delay_ms);
            self.ack_deadline[space] = Some(deadline);
        }
    }

    // ── PTO / retransmission ──────────────────────────────────────────

    async fn check_pto_and_retransmit(&mut self) -> Result<(), Error> {
        let now = self.clock();
        for level in [0u8, 1u8, 2u8] {
            let idx = level as usize;
            if level == 0 && self.hs_send.is_some() {
                continue;
            }
            if level != 2 && self.app_send.is_some() {
                continue;
            }
            if let Some(deadline) = self.ack_deadline[idx] {
                if now >= deadline && self.ack_tracker[idx].ack_eliciting_since_last_ack {
                    self.send_ack(idx).await?;
                }
            }
            if self.ack_tracker[idx].is_empty() {
                continue;
            }
            if !self.loss_detect[idx].pto_expired(now) {
                continue;
            }
            let largest_acked = self.ack_tracker[idx].largest_acked;
            let time_threshold = {
                let loss_detect = &mut self.loss_detect[idx];
                loss_detect.on_pto_timeout();
                loss_detect.loss_time_threshold()
            };
            let lost_pns = self.ack_tracker[idx].detect_lost_packets(now, time_threshold, largest_acked);
            if !lost_pns.is_empty() {
                let lost_data = self.ack_tracker[idx].remove_lost(&lost_pns);
                for (_, enc_level, long_header, is_initial, frames) in lost_data {
                    let lvl = if enc_level == 1 {
                        1u8
                    } else if enc_level == 2 {
                        2u8
                    } else {
                        0u8
                    };
                    self.send_frames_at_level(lvl, long_header, is_initial, &frames).await?;
                }
            } else {
                let lvl = match self.ack_tracker[idx].last_sent_level() {
                    Some(0) => 0u8,
                    Some(1) => 1u8,
                    _ => 2u8,
                };
                let long_header = lvl != 2;
                let is_initial = lvl == 0;
                self.send_frames_at_level(lvl, long_header, is_initial, &[Frame::Ping])
                    .await?;
            }
        }
        Ok(())
    }
}
