use alloc::{
    borrow::ToOwned,
    collections::{BTreeMap, VecDeque},
    format,
    string::String,
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
    crypto_keys::{self, DirectionKeys},
    error::{Error, IoError},
    frame::{self, Frame},
    instant::Instant,
    loss::LossDetection,
    packet::{self, LongPacketType},
    stream::{
        ReceiveChunk, ReceiveStream, RecvFlowController, SendFlowController, SendStream, Stream, StreamAllocator,
        StreamCommand, StreamCommandKind, StreamDir,
    },
    tls_adapter::TlsAdapter,
    transport::Transport,
    transport_params::{self, Param, ParamType},
    varint,
};

/// QUIC v1 Retry integrity key (RFC 9001 §5.8).
const RETRY_INTEGRITY_KEY_V1: [u8; 16] = [
    0xbe, 0x0c, 0x69, 0x0b, 0x9f, 0x66, 0x57, 0x5a, 0x1d, 0x76, 0x6b, 0x54, 0xe3, 0x68, 0xc8, 0x4e,
];

/// QUIC v1 Retry integrity nonce (RFC 9001 §5.8).
const RETRY_INTEGRITY_NONCE_V1: [u8; 12] = [0x46, 0x15, 0x99, 0xd3, 0x5d, 0x63, 0x2b, 0xf2, 0x23, 0x98, 0x25, 0xbb];

/// QUIC v2 Retry integrity key (RFC 9369).
const RETRY_INTEGRITY_KEY_V2: [u8; 32] = [
    0xc4, 0xdd, 0x24, 0x84, 0x45, 0x43, 0x47, 0xf6, 0x3a, 0x0f, 0xbb, 0x0d, 0x63, 0x0a, 0x3a, 0x3b, 0x7c, 0x75, 0xda,
    0x46, 0x75, 0xc2, 0x2c, 0xb3, 0xc0, 0x3d, 0x21, 0xbe, 0x2b, 0x0b, 0xf2, 0xa7,
];

/// QUIC v2 Retry integrity nonce (RFC 9369).
const RETRY_INTEGRITY_NONCE_V2: [u8; 12] = [0x4c, 0x6f, 0x2c, 0x6f, 0x2c, 0x6f, 0x2c, 0x6f, 0x2c, 0x6f, 0x2c, 0x6f];

/// Reassembles CRYPTO frame data by offset (RFC 9001 §4.5).
///
/// CRYPTO frames may arrive out of order. This buffer stores received
/// chunks keyed by offset and delivers contiguous data as gaps are filled.
pub(crate) struct CryptoBuffer {
    next_offset: u64,
    chunks: BTreeMap<u64, Vec<u8>>,
}

impl CryptoBuffer {
    pub(crate) fn new() -> Self {
        Self {
            next_offset: 0,
            chunks: BTreeMap::new(),
        }
    }

    pub(crate) fn ingest(&mut self, offset: u64, data: &[u8]) -> Option<Vec<u8>> {
        // Trim data that we've already consumed
        if offset < self.next_offset {
            let skip = (self.next_offset - offset) as usize;
            if skip >= data.len() {
                return None;
            }
            let trimmed = &data[skip..];
            self.chunks.insert(self.next_offset, trimmed.to_vec());
        } else {
            self.chunks.insert(offset, data.to_vec());
        }
        let mut contiguous = Vec::new();
        while let Some(chunk) = self.chunks.remove(&self.next_offset) {
            self.next_offset += chunk.len() as u64;
            contiguous.extend_from_slice(&chunk);
        }
        if contiguous.is_empty() { None } else { Some(contiguous) }
    }

    pub(crate) fn reset(&mut self) {
        self.next_offset = 0;
        self.chunks.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionLevel {
    Initial,
    Handshake,
    OneRtt,
}

impl EncryptionLevel {
    fn index(self) -> usize {
        match self {
            EncryptionLevel::Initial => 0,
            EncryptionLevel::Handshake => 1,
            EncryptionLevel::OneRtt => 2,
        }
    }
}

pub(crate) struct LevelSendState {
    pub(crate) keys: DirectionKeys,
    pub(crate) pn: u64,
}

enum ConnState {
    Connecting,
    Established,
    _Closing,
    Closed,
}

pub struct Connection<T: Transport> {
    transport: T,
    config: Config,
    remote: SocketAddr,
    server_name: String,
    state: ConnState,
    /// QUIC version in use (default: v1).
    version: u32,
    dcid: ConnectionId,
    scid: ConnectionId,
    original_dcid: Option<ConnectionId>,
    retry_token: Option<Vec<u8>>,
    init_send: LevelSendState,
    init_recv: DirectionKeys,
    hs_send: Option<LevelSendState>,
    hs_recv: Option<DirectionKeys>,
    app_send: Option<LevelSendState>,
    app_recv: Option<DirectionKeys>,
    /// Client application traffic secret (for send-side key updates, RFC 9001 §6).
    app_traffic_secret: Option<Vec<u8>>,
    /// Server application traffic secret (for receive-side key updates).
    server_app_traffic_secret: Option<Vec<u8>>,
    /// Current send key phase (toggles 0↔1 on key update).
    key_phase_send: u8,
    /// Current receive key phase.
    key_phase_recv: u8,
    /// Previous receive keys (for out-of-order packets during transition).
    prev_app_recv: Option<DirectionKeys>,
    /// Whether we have initiated a key update that awaits ACK.
    key_update_pending: bool,
    /// Count of send packets since last key update (trigger auto-update).
    sent_since_key_update: u64,
    /// Count of recv packets with new key phase (confirm transition).
    recv_with_new_phase: u32,
    /// Stateless reset tokens we've issued via NEW_CONNECTION_ID frames.
    /// When a stateless reset packet arrives, we check its token against this list.
    stateless_reset_tokens: Vec<[u8; 16]>,
    /// Path validation: stores challenge data for pending path validations.
    pending_path_challenges: Vec<[u8; 8]>,
    tls: Option<TlsAdapter>,
    pn_recv: [u64; 3],
    streams: HashMap<u64, Stream>,
    stream_alloc: StreamAllocator,
    datagram_queue: VecDeque<Vec<u8>>,
    crypto_buffer: CryptoBuffer,
    /// CRYPTO stream buffer for Handshake encryption level (separate stream).
    crypto_buffer_hs: CryptoBuffer,
    /// ACK/loss tracking per PN space: 0=Initial, 1=Handshake, 2=1RTT.
    ack_tracker: [AckTracker; 3],
    /// Loss detection per PN space.
    loss_detect: [LossDetection; 3],
    /// When we last sent or received data (for idle timeout).
    last_activity: Instant,
    /// When established (for idle timeout enforcement).
    established_at: Option<Instant>,
    /// ACK deadline per PN space (for delayed ACK).
    ack_deadline: [Option<Instant>; 3],
    /// Connection-level flow control: sending.
    send_flow: SendFlowController,
    /// Connection-level flow control: receiving.
    recv_flow: RecvFlowController,
    /// Commands from stream objects (queue for send, reset, finish, stop).
    cmd_rx: CmdReceiver<StreamCommand>,
    /// Template for cloning command senders into new stream objects.
    base_cmd_tx: CmdSender<StreamCommand>,
    /// Per-stream data push channels. When a STREAM frame arrives, the data
    /// is pushed to the corresponding sender so the ReceiveStream can consume it.
    stream_data_tx: HashMap<u64, CmdSender<ReceiveChunk>>,
    /// Peer-initiated streams waiting to be accepted.
    pending_accepts: VecDeque<(u64, StreamDir)>,
}

impl<T: Transport> Connection<T> {
    fn clock(&self) -> Instant {
        self.transport.now()
    }

    pub fn new(transport: T, config: Config) -> Self {
        let remote = "0.0.0.0:0".parse().unwrap();
        let dcid = ConnectionId::new(&[0; 8]);
        let (ck, sk) = crypto_keys::derive_initial_keys(config.tls_config.crypto_provider(), dcid.as_bytes());
        let max_ack_delay = Duration::from_millis(config.max_ack_delay_ms);
        let initial_max_data = config.initial_max_data;
        let (cmd_tx, cmd_rx) = cmd_queue();
        let now = transport.now();
        Connection {
            transport,
            config,
            remote,
            server_name: String::new(),
            state: ConnState::Connecting,
            version: packet::QUIC_VERSION_V1,
            dcid,
            scid: ConnectionId::random(8),
            original_dcid: None,
            retry_token: None,
            init_send: LevelSendState {
                keys: ck,
                pn: 0,
            },
            init_recv: sk,
            hs_send: None,
            hs_recv: None,
            app_send: None,
            app_recv: None,
            app_traffic_secret: None,
            server_app_traffic_secret: None,
            key_phase_send: 0,
            key_phase_recv: 0,
            prev_app_recv: None,
            key_update_pending: false,
            sent_since_key_update: 0,
            recv_with_new_phase: 0,
            stateless_reset_tokens: Vec::new(),
            pending_path_challenges: Vec::new(),
            tls: None,
            pn_recv: [0; 3],
            streams: HashMap::new(),
            stream_alloc: StreamAllocator::new(),
            datagram_queue: VecDeque::new(),
            crypto_buffer: CryptoBuffer::new(),
            crypto_buffer_hs: CryptoBuffer::new(),
            ack_tracker: [AckTracker::new(), AckTracker::new(), AckTracker::new()],
            loss_detect: [
                LossDetection::new(max_ack_delay),
                LossDetection::new(max_ack_delay),
                LossDetection::new(max_ack_delay),
            ],
            last_activity: now,
            established_at: None,
            ack_deadline: [None, None, None],
            send_flow: SendFlowController::new(initial_max_data),
            recv_flow: RecvFlowController::new(initial_max_data),
            cmd_rx,
            base_cmd_tx: cmd_tx,
            stream_data_tx: HashMap::new(),
            pending_accepts: VecDeque::new(),
        }
    }

    pub async fn connect(&mut self, remote: SocketAddr, server_name: &str) -> Result<(), Error> {
        self.remote = remote;
        self.server_name = server_name.to_owned();
        let dcid = ConnectionId::random(8);
        self.dcid = dcid.clone();
        self.original_dcid = Some(dcid.clone());
        let (ck, sk) = crypto_keys::derive_initial_keys_for_version(
            self.config.tls_config.crypto_provider(),
            dcid.as_bytes(),
            self.version,
        );
        self.init_send = LevelSendState {
            keys: ck,
            pn: 0,
        };
        self.init_recv = sk;
        self.crypto_buffer.reset();
        self.crypto_buffer_hs.reset();
        self.last_activity = self.clock();

        let tps = transport_params::encode(&build_transport_params(&self.config, &self.scid));
        let alpn_refs: heapless::Vec<&[u8], 8> = self.config.alpn_protocols.iter().map(|p| p.as_ref()).collect();
        let tls_client_config = match &self.config.tls_config {
            crate::config::TlsConfig::Client(cfg) => cfg.clone(),
            crate::config::TlsConfig::Server(_) => {
                return Err(Error::InvalidState("cannot use server TLS config for client connection".into()));
            }
        };
        let tls_conn = tls::ClientConnection::new_quic_with_preferred_group(
            tls_client_config,
            Some(server_name.to_owned()),
            &tps,
            &alpn_refs[..],
            None,
        )
        .map_err(|e| Error::ConnectionRejected(format!("TLS init failed: {e}")))?;
        drop(alpn_refs);
        self.tls = Some(TlsAdapter::new(tls_conn));
        let ch = self
            .tls
            .as_mut()
            .unwrap()
            .write_handshake()
            .ok_or(Error::InvalidState("no CH".into()))?;

        self.send_crypto_initial(&ch, None).await?;

        let deadline = self.clock() + Duration::from_secs(10);
        let idle_timeout = Duration::from_millis(self.config.max_idle_timeout_ms);
        loop {
            if self.is_established() {
                return Ok(());
            }
            if self.clock() > deadline {
                return Err(Error::ConnectionTimedOut);
            }
            // Idle timeout during handshake
            if self.clock().duration_since(self.last_activity) >= idle_timeout {
                return Err(Error::ConnectionTimedOut);
            }
            // Check PTO before blocking on recv
            self.check_pto_and_retransmit().await?;
            let result = self.poll().await;
            match result {
                Ok(()) => {}
                Err(Error::ConnectionClosed(_, _)) => return result,
                Err(e) => return Err(e),
            }
        }
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub fn is_established(&self) -> bool {
        matches!(self.state, ConnState::Established)
    }

    /// Initiate path validation for connection migration (RFC 9000 §9).
    ///
    /// Sends a PATH_CHALLENGE frame to the new peer address and stores
    /// the challenge data for later validation.
    async fn initiate_path_validation(&mut self) -> Result<(), Error> {
        let mut challenge = [0u8; 8];
        crypto::random_fill(&mut challenge);
        self.pending_path_challenges.push(challenge);

        if self.app_send.is_some() {
            self.send_frames_at_level(
                EncryptionLevel::OneRtt,
                false,
                false,
                &[Frame::PathChallenge {
                    data: challenge,
                }],
            )
            .await?;
        }
        Ok(())
    }

    /// Confirm path validation when a PATH_RESPONSE frame is received.
    fn confirm_path_validation(&mut self, data: &[u8; 8]) {
        self.pending_path_challenges.retain(|c| c != data);
    }
    ///
    /// Derives new send keys from the traffic secret, toggles the key phase,
    /// and keeps the old keys for a short period to handle out-of-order
    /// acknowledgements. The caller should send a packet with the new key
    /// phase to signal the update to the peer.
    pub fn initiate_key_update(&mut self) -> Result<(), Error> {
        let secret = self
            .app_traffic_secret
            .as_ref()
            .ok_or(Error::InvalidState("no traffic secret for key update".into()))?;
        let suite = self
            .app_send
            .as_ref()
            .map(|s| s.keys.cipher_suite)
            .ok_or(Error::InvalidState("no app send keys".into()))?;
        let (new_secret, new_keys) =
            crypto_keys::derive_next_keys(self.config.tls_config.crypto_provider(), secret, suite)
                .map_err(|e| Error::InvalidState(format!("key update: {e:?}")))?;
        self.app_traffic_secret = Some(new_secret);
        self.key_phase_send ^= 1;
        self.sent_since_key_update = 0;
        self.key_update_pending = true;

        if let Some(ref mut ss) = self.app_send {
            ss.keys = new_keys;
        }
        Ok(())
    }

    /// Trigger an automatic key update if the send packet threshold is reached.
    /// RFC 9001 §6.4 recommends updating keys frequently to limit forgery.
    pub fn maybe_auto_update_key(&mut self) -> Result<(), Error> {
        const KEY_UPDATE_THRESHOLD: u64 = 1_000_000; // ~1M packets
        if self.sent_since_key_update >= KEY_UPDATE_THRESHOLD && !self.key_update_pending {
            self.initiate_key_update()?;
        }
        Ok(())
    }

    /// Handle a key phase change on the receive side.
    fn handle_recv_key_phase(&mut self, packet_key_phase: bool) {
        if packet_key_phase as u8 == self.key_phase_recv {
            return;
        }
        let secret = match self.server_app_traffic_secret.as_ref() {
            Some(s) => s,
            None => return,
        };
        let suite = match self.app_recv.as_ref() {
            Some(r) => r.cipher_suite,
            None => return,
        };

        let (new_secret, new_keys) =
            crypto_keys::derive_next_keys(self.config.tls_config.crypto_provider(), secret, suite)
                .expect("receive key update derivation failed");
        self.server_app_traffic_secret = Some(new_secret);

        let old_recv = self.app_recv.take();
        self.app_recv = Some(new_keys);
        self.key_phase_recv ^= 1;
        self.recv_with_new_phase = 0;

        if let Some(keys) = old_recv {
            self.prev_app_recv = Some(keys);
        }
    }

    /// Check if a packet is a stateless reset (RFC 9000 §10.3).
    ///
    /// Returns true if the packet's last 16 bytes match one of our
    /// stateless reset tokens.
    fn check_stateless_reset(&self, data: &[u8]) -> bool {
        if data.len() < 38 {
            return false;
        }
        let token_start = data.len() - 16;
        let mut token = [0u8; 16];
        token.copy_from_slice(&data[token_start..]);
        self.stateless_reset_tokens.contains(&token)
    }
    fn try_decrypt_old_phase(&self, data: &[u8], pn_offset: usize, pn: u64, _payload_offset: usize) -> Option<Vec<u8>> {
        let old_keys = self.prev_app_recv.as_ref()?;
        let sample_start = pn_offset + 4;
        if sample_start + 16 > data.len() {
            return None;
        }
        let sample = &data[sample_start..sample_start + 16];
        let mut fb = data[0];
        let mut pn_b = data[pn_offset..pn_offset + 4].to_vec();
        crypto_keys::remove_header_protection(old_keys, false, &mut fb, &mut pn_b, sample);
        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], pn);
        let payload_off = pn_offset + pn_len;

        let pkt_end = payload_off + (data.len() - payload_off);
        let mut aad = data[..payload_off].to_vec();
        aad[0] = fb;
        aad[pn_offset..pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);

        let mut payload = data[payload_off..pkt_end].to_vec();
        crypto_keys::decrypt_payload(old_keys, pn, &aad, &mut payload).ok()?;
        Some(payload)
    }

    /// Whether the idle timeout has been reached since last activity.
    pub fn idle_timeout_reached(&self) -> bool {
        let timeout = Duration::from_millis(self.config.max_idle_timeout_ms);
        self.clock().duration_since(self.last_activity) >= timeout
    }

    pub async fn open_bidirectional_stream(&mut self) -> Result<(SendStream, ReceiveStream), Error> {
        let id = self.stream_alloc.next_bi();
        let mut s = Stream::new(id);
        s.max_stream_data = self.config.initial_max_stream_data_bidi_remote;
        s.local_max_stream_data = self.config.initial_max_stream_data_bidi_local;
        self.streams.insert(id, s);

        let (data_tx, data_rx) = cmd_queue();
        self.stream_data_tx.insert(id, data_tx);

        let send = SendStream {
            id,
            cmd_tx: self.base_cmd_tx.clone(),
            fin_sent: false,
        };
        let recv = ReceiveStream {
            id,
            cmd_tx: self.base_cmd_tx.clone(),
            data_rx,
            pending: Vec::new(),
            fin_received: false,
        };
        Ok((send, recv))
    }

    pub async fn open_unidirectional_stream(&mut self) -> Result<SendStream, Error> {
        let id = self.stream_alloc.next_uni();
        let mut s = Stream::new(id);
        s.max_stream_data = self.config.initial_max_stream_data_uni;
        s.local_max_stream_data = self.config.initial_max_stream_data_uni;
        self.streams.insert(id, s);

        Ok(SendStream {
            id,
            cmd_tx: self.base_cmd_tx.clone(),
            fin_sent: false,
        })
    }

    /// Accept a peer-initiated bidirectional stream.
    ///
    /// Blocks until a peer-initiated bidirectional stream is available
    /// (by calling [`poll`] internally if needed).
    pub async fn accept_bidirectional_stream(&mut self) -> Result<(SendStream, ReceiveStream), Error> {
        loop {
            if let Some(pos) = self.pending_accepts.iter().position(|(_, dir)| *dir == StreamDir::Bi) {
                let (id, _) = self.pending_accepts.remove(pos).unwrap();
                let (data_tx, data_rx) = cmd_queue();
                self.stream_data_tx.insert(id, data_tx);
                let (fin_received, pending_data) = self.drain_pending_stream_data(id);
                let send = SendStream {
                    id,
                    cmd_tx: self.base_cmd_tx.clone(),
                    fin_sent: false,
                };
                let recv = ReceiveStream {
                    id,
                    cmd_tx: self.base_cmd_tx.clone(),
                    data_rx,
                    pending: pending_data,
                    fin_received,
                };
                return Ok((send, recv));
            }
            if self.poll().await.is_err() {
                if self.pending_accepts.iter().any(|(_, d)| *d == StreamDir::Bi) {
                    continue;
                }
                return Err(Error::ConnectionClosed(0, "connection closed".into()));
            }
        }
    }

    /// Accept a peer-initiated unidirectional stream.
    ///
    /// Blocks until a peer-initiated unidirectional stream is available.
    pub async fn accept_unidirectional_stream(&mut self) -> Result<ReceiveStream, Error> {
        loop {
            if let Some(recv) = self.try_accept_unidirectional_stream() {
                return Ok(recv);
            }
            if self.poll().await.is_err() {
                if self.pending_accepts.iter().any(|(_, d)| *d == StreamDir::Uni) {
                    continue;
                }
                return Err(Error::ConnectionClosed(0, "connection closed".into()));
            }
        }
    }

    /// Check for a pending unidirectional stream without blocking.
    ///
    /// Returns `Some(ReceiveStream)` if a peer-initiated unidirectional stream
    /// is waiting, or `None` if none are available.  Does not call
    /// `poll()` — the caller is responsible for driving the connection.
    pub fn try_accept_unidirectional_stream(&mut self) -> Option<ReceiveStream> {
        if let Some(pos) = self.pending_accepts.iter().position(|(_, d)| *d == StreamDir::Uni) {
            let (id, _) = self.pending_accepts.remove(pos).unwrap();
            let (data_tx, data_rx) = cmd_queue();
            self.stream_data_tx.insert(id, data_tx);
            let (fin_received, pending_data) = self.drain_pending_stream_data(id);
            Some(ReceiveStream {
                id,
                cmd_tx: self.base_cmd_tx.clone(),
                data_rx,
                pending: pending_data,
                fin_received,
            })
        } else {
            None
        }
    }

    pub fn try_accept_bidirectional_stream(&mut self) -> Option<(SendStream, ReceiveStream)> {
        if let Some(pos) = self.pending_accepts.iter().position(|(_, d)| *d == StreamDir::Bi) {
            let (id, _) = self.pending_accepts.remove(pos).unwrap();
            let (data_tx, data_rx) = cmd_queue();
            self.stream_data_tx.insert(id, data_tx);
            let (fin_received, pending_data) = self.drain_pending_stream_data(id);
            let send = SendStream {
                id,
                cmd_tx: self.base_cmd_tx.clone(),
                fin_sent: false,
            };
            let recv = ReceiveStream {
                id,
                cmd_tx: self.base_cmd_tx.clone(),
                data_rx,
                pending: pending_data,
                fin_received,
            };
            Some((send, recv))
        } else {
            None
        }
    }

    fn drain_pending_stream_data(&mut self, id: u64) -> (bool, Vec<u8>) {
        if let Some(s) = self.streams.get_mut(&id) {
            let fin = s.fin_received;
            let data = core::mem::take(&mut s.recv_buffer);
            (fin, data)
        } else {
            (false, Vec::new())
        }
    }

    // ── Stream command processing ──────────────────────────────────────

    /// Process all pending commands from stream objects.
    ///
    /// Called at the start of each I/O cycle so that queued `send`,
    /// `finish`, `reset`, and `stop` commands are applied before
    /// we read the next packet.
    async fn process_stream_commands(&mut self) -> Result<(), Error> {
        let mut immediate_frames = Vec::new();

        while let Some(cmd) = self.cmd_rx.try_recv() {
            match cmd.kind {
                StreamCommandKind::Send {
                    data,
                    fin,
                } => {
                    self.streams
                        .get_mut(&cmd.stream_id)
                        .ok_or(Error::StreamNotFound(cmd.stream_id))?
                        .write(&data, fin);
                }
                StreamCommandKind::Finish => {
                    self.streams
                        .get_mut(&cmd.stream_id)
                        .ok_or(Error::StreamNotFound(cmd.stream_id))?
                        .write(&[], true);
                }
                StreamCommandKind::Reset(error_code) => {
                    immediate_frames.push(Frame::ResetStream {
                        stream_id: cmd.stream_id,
                        error_code,
                        final_size: 0,
                    });
                }
                StreamCommandKind::StopSending(error_code) => {
                    immediate_frames.push(Frame::StopSending {
                        stream_id: cmd.stream_id,
                        error_code,
                    });
                }
            }
        }

        if !immediate_frames.is_empty() && self.app_send.is_some() {
            self.send_frames_at_level(EncryptionLevel::OneRtt, false, false, &immediate_frames)
                .await?;
        }

        if self.app_send.is_some() {
            let frames = self.drain_stream_frames();
            if !frames.is_empty() {
                self.send_frames_at_level(EncryptionLevel::OneRtt, false, false, &frames)
                    .await?;
            }
        }

        Ok(())
    }

    /// Build up to one packet's worth of STREAM frames from pending data,
    /// respecting connection-level and per-stream flow control.
    fn drain_stream_frames(&mut self) -> Vec<Frame> {
        let max_payload = self.config.initial_max_datagram_size.saturating_sub(32); // header + tag overhead
        let mut frames = Vec::new();
        let mut total_bytes = 0u64;

        // First, send any pending MAX_DATA / MAX_STREAM_DATA updates
        if self.recv_flow.needs_max_data_update {
            self.recv_flow.needs_max_data_update = false;
            frames.push(Frame::MaxData {
                maximum_data: self.recv_flow.local_max_data,
            });
        }

        let stream_ids: Vec<u64> = self.streams.keys().copied().collect();
        for id in stream_ids {
            if total_bytes as usize >= max_payload {
                break;
            }
            let conn_credit = self.send_flow.available();
            if conn_credit == 0 {
                if !self.send_flow.blocked {
                    self.send_flow.blocked = true;
                    frames.push(Frame::DataBlocked {
                        data_limit: self.send_flow.max_data,
                    });
                }
                break;
            }

            let s = match self.streams.get_mut(&id) {
                Some(s) => s,
                None => continue,
            };
            if s.send_buffer.is_empty() && (!s.fin_sent || s.send_offset > 0) {
                // Check for MAX_STREAM_DATA needed
                if s.needs_max_stream_data {
                    s.needs_max_stream_data = false;
                    let f = Frame::MaxStreamData {
                        stream_id: id,
                        maximum_stream_data: s.local_max_stream_data,
                    };
                    if !frames
                        .iter()
                        .any(|f2| matches!(f2, Frame::MaxStreamData { stream_id: sid, .. } if *sid == id))
                    {
                        frames.push(f);
                    }
                }
                continue;
            }

            // Check stream-level flow control
            let stream_credit = s.max_stream_data.saturating_sub(s.send_offset);
            if stream_credit == 0 {
                frames.push(Frame::StreamDataBlocked {
                    stream_id: id,
                    stream_data_limit: s.max_stream_data,
                });
                continue;
            }

            let space_left = max_payload.saturating_sub(total_bytes as usize);
            let can_send = conn_credit.min(stream_credit).min(space_left as u64) as usize;
            let data_len = s.send_buffer.len();
            let to_send = data_len.min(can_send);
            if to_send == 0 && !s.fin_sent {
                continue;
            }

            let chunk = s.send_buffer[..to_send].to_vec();
            let is_fin = s.fin_sent && to_send == data_len;

            frames.push(Frame::Stream {
                id,
                offset: s.send_offset,
                data: chunk,
                fin: is_fin,
            });

            let sent = to_send as u64;
            s.send_offset += sent;
            s.send_buffer.drain(..to_send);
            if is_fin {
                s.fin_sent = true;
            }
            self.send_flow.on_sent(sent);
            total_bytes += sent;
        }
        frames
    }

    /// Send any pending MAX_DATA / MAX_STREAM_DATA frames.
    async fn send_flow_control_updates(&mut self) -> Result<(), Error> {
        if self.app_send.is_none() {
            return Ok(());
        }
        let mut frames = Vec::new();

        if self.recv_flow.needs_max_data_update {
            self.recv_flow.needs_max_data_update = false;
            frames.push(Frame::MaxData {
                maximum_data: self.recv_flow.local_max_data,
            });
        }

        for s in self.streams.values_mut() {
            if s.needs_max_stream_data {
                s.needs_max_stream_data = false;
                frames.push(Frame::MaxStreamData {
                    stream_id: s.id,
                    maximum_stream_data: s.local_max_stream_data,
                });
            }
        }

        if !frames.is_empty() {
            self.send_frames_at_level(EncryptionLevel::OneRtt, false, false, &frames)
                .await?;
        }
        Ok(())
    }

    pub async fn send_datagram(&mut self, data: &[u8]) -> Result<(), Error> {
        self.send_frames_at_level(
            EncryptionLevel::OneRtt,
            false,
            false,
            &[Frame::Datagram {
                data: data.to_vec(),
            }],
        )
        .await?;
        Ok(())
    }

    pub async fn receive_datagram(&mut self) -> Result<Vec<u8>, Error> {
        loop {
            if let Some(d) = self.datagram_queue.pop_front() {
                return Ok(d);
            }
            self.poll().await?;
        }
    }

    pub async fn close(&mut self, error_code: u64, reason: &[u8]) -> Result<(), Error> {
        let frame = Frame::ConnectionClose {
            error_code,
            frame_type: None,
            reason_phrase: reason.to_vec(),
        };
        let (level, long_header, is_initial) = match (&self.app_send, &self.hs_send) {
            (Some(_), _) => (EncryptionLevel::OneRtt, false, false),
            (None, Some(_)) => (EncryptionLevel::Handshake, true, false),
            _ => (EncryptionLevel::Initial, true, true),
        };
        self.send_frames_at_level(level, long_header, is_initial, &[frame])
            .await?;
        self.state = ConnState::Closed;
        Ok(())
    }

    // ── Packet sending (with ACK tracking) ────────────────────────────

    /// Send a CRYPTO frame in an Initial packet (with optional Retry token).
    async fn send_crypto_initial(&mut self, data: &[u8], token: Option<&[u8]>) -> Result<(), Error> {
        let pn = self.init_send.pn;
        let pn_len = crypto_keys::pn_encoding_len(pn, 0);
        let target_size = self.config.initial_max_datagram_size;

        let mut payload = Vec::new();
        frame::encode(
            &Frame::Crypto {
                offset: 0,
                data: data.to_vec(),
            },
            &mut payload,
        );
        let pad_needed = target_size.saturating_sub(pn_len + payload.len() + 16 + 25).max(0);
        if pad_needed > 0 {
            frame::pad_to(target_size, payload.len(), &mut payload);
        }

        let mut header = Vec::new();
        let flag: u8 = 0xc0 | ((pn_len - 1) as u8);
        header.push(flag);
        header.extend_from_slice(&self.version.to_be_bytes());
        header.push(self.dcid.len() as u8);
        header.extend_from_slice(self.dcid.as_bytes());
        header.push(self.scid.len() as u8);
        header.extend_from_slice(self.scid.as_bytes());
        if let Some(tok) = token {
            varint::encode(tok.len() as u64, &mut header);
            header.extend_from_slice(tok);
        } else {
            header.push(0);
        }
        let pkt_len = pn_len + payload.len() + 16;
        varint::encode(pkt_len as u64, &mut header);
        let pn_start = header.len();
        crypto_keys::encode_pn(pn, pn_len, &mut header);

        let aad = header.clone();
        let mut encrypted = payload;
        crypto_keys::encrypt_payload(&self.init_send.keys, pn, &aad, &mut encrypted)?;
        let mut full = aad;
        full.extend_from_slice(&encrypted);

        let sample_start = pn_start + 4;
        if sample_start + 16 <= full.len() {
            let mut s = [0u8; 16];
            s.copy_from_slice(&full[sample_start..sample_start + 16]);
            let (before, pn_and_after) = full.split_at_mut(pn_start);
            let pn_region = &mut pn_and_after[..pn_len];
            crypto_keys::apply_header_protection(&self.init_send.keys, true, &mut before[0], pn_region, &s);
        }

        self.transport.send_to(self.remote, &full).await?;
        self.init_send.pn += 1;

        let now = self.clock();
        let frames = vec![Frame::Crypto {
            offset: 0,
            data: data.to_vec(),
        }];
        self.ack_tracker[0].on_packet_sent(now, pn, true, 0, true, true, frames);
        self.loss_detect[0].on_packet_sent(now, true);
        self.last_activity = now;
        Ok(())
    }
    /// Send frames at a specific encryption level.
    async fn send_frames_at_level(
        &mut self,
        level: EncryptionLevel,
        long_header: bool,
        is_initial: bool,
        frames: &[Frame],
    ) -> Result<(), Error> {
        let level_u8 = level.index() as u8;
        let ver = self.version;
        let kp = if level == EncryptionLevel::OneRtt {
            self.key_phase_send
        } else {
            0
        };
        match level {
            EncryptionLevel::Initial => {
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
                    level_u8,
                    ver,
                    kp,
                )
                .await?;
            }
            EncryptionLevel::Handshake => {
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
                    level_u8,
                    ver,
                    kp,
                )
                .await?;
            }
            EncryptionLevel::OneRtt => {
                let ss = self
                    .app_send
                    .as_mut()
                    .ok_or(Error::InvalidState("no 1RTT send keys".into()))?;
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
                    level_u8,
                    ver,
                    kp,
                )
                .await?;
            }
        }
        if level == EncryptionLevel::OneRtt {
            self.sent_since_key_update = self.sent_since_key_update.saturating_add(1);
        }
        Ok(())
    }

    /// Send an ACK frame for a given PN space.
    async fn send_ack(&mut self, level: EncryptionLevel) -> Result<(), Error> {
        let idx = level.index();
        let now = self.clock();
        let ack_delay_us = if let Some(first) = self.ack_tracker[idx].first_ack_eliciting {
            now.duration_since(first).as_micros().min(u64::MAX as u128) as u64
        } else {
            0
        };
        let ranges = self.ack_tracker[idx].build_ack(ack_delay_us);
        self.ack_deadline[idx] = None;

        let ack_frame = Frame::Ack {
            largest_acknowledged: ranges.largest,
            ack_delay: ranges.delay,
            first_ack_range: ranges.first_range,
            ack_ranges: ranges.extra_ranges,
        };

        let long_header = level != EncryptionLevel::OneRtt;
        let is_initial = level == EncryptionLevel::Initial;
        self.send_frames_at_level(level, long_header, is_initial, &[ack_frame])
            .await
    }

    // ── PTO / retransmission ──────────────────────────────────────────

    async fn check_pto_and_retransmit(&mut self) -> Result<(), Error> {
        let now = self.clock();
        for level in [
            EncryptionLevel::Initial,
            EncryptionLevel::Handshake,
            EncryptionLevel::OneRtt,
        ] {
            let idx = level.index();
            // Skip lower levels once higher keys are available
            if level == EncryptionLevel::Initial && self.hs_send.is_some() {
                continue;
            }
            if level != EncryptionLevel::OneRtt && self.app_send.is_some() {
                continue;
            }
            // Check if we need to send a delayed ACK
            if let Some(deadline) = self.ack_deadline[idx] {
                if now >= deadline && self.ack_tracker[idx].ack_eliciting_since_last_ack {
                    self.send_ack(level).await?;
                }
            }

            if self.ack_tracker[idx].is_empty() {
                continue;
            }
            if !self.loss_detect[idx].pto_expired(now) {
                continue;
            }

            // PTO expired: detect lost packets and retransmit
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
                    let level = match enc_level {
                        0 => EncryptionLevel::Initial,
                        1 => EncryptionLevel::Handshake,
                        _ => EncryptionLevel::OneRtt,
                    };
                    self.send_frames_at_level(level, long_header, is_initial, &frames)
                        .await?;
                }
            } else {
                // No packets to retransmit; send a PING probe to elicit an ACK
                let level = match self.ack_tracker[idx].last_sent_level() {
                    Some(0) => EncryptionLevel::Initial,
                    Some(1) => EncryptionLevel::Handshake,
                    _ => EncryptionLevel::OneRtt,
                };
                let long_header = level != EncryptionLevel::OneRtt;
                let is_initial = level == EncryptionLevel::Initial;
                self.send_frames_at_level(level, long_header, is_initial, &[Frame::Ping])
                    .await?;
            }
        }
        Ok(())
    }

    // ── I/O poll ────────────────────────────────────────────────────

    /// Drive the connection forward: flush outgoing frames, check
    /// timers, receive and process incoming datagrams.
    ///
    /// Returns `Ok(())` when idle. Call in a loop (or spawn a
    /// background task) to keep the connection alive while using
    /// [`SendStream`] and [`ReceiveStream`] objects.
    pub async fn poll(&mut self) -> Result<(), Error> {
        self.process_stream_commands().await?;
        self.check_pto_and_retransmit().await?;

        let buf_size = self.config.recv_buf_size;
        let mut buf = vec![0u8; buf_size];
        match self.transport.receive_from(&mut buf).await {
            Ok((0, _)) => Ok(()),
            Ok((n, src)) => {
                self.last_activity = self.clock();
                // Detect connection migration (peer address change)
                if src != self.remote {
                    // Initiate path validation
                    self.initiate_path_validation().await?;
                }
                let mut data = &buf[..n];
                while !data.is_empty() {
                    let consumed = if data[0] >> 7 == 1 {
                        self.process_long(data).await?
                    } else if self.app_recv.is_none() || data.iter().all(|&b| b == 0) {
                        break;
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
            Err(IoError::WouldBlock) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    async fn process_long(&mut self, data: &[u8]) -> Result<usize, Error> {
        let header = packet::parse_long_header(data)?;

        // Version Negotiation: version == 0
        if header.version == 0 {
            return self.process_version_negotiation(data).await;
        }

        let pkt_end = header.pn_offset + header.payload_length as usize;
        if pkt_end > data.len() {
            return Err(Error::PacketDecode("payload_length exceeds available data".into()));
        }
        let pkt = &data[..pkt_end];
        match header.ty {
            LongPacketType::Initial => {
                self.process_initial(&header, pkt).await?;
            }
            LongPacketType::Handshake => {
                self.process_handshake(&header, pkt).await?;
            }
            LongPacketType::Retry => {
                return self.process_retry(&header, data).await;
            }
            _ => return Err(Error::ProtocolViolation("unexpected long packet type".into())),
        }
        Ok(pkt_end)
    }

    /// Handle a Version Negotiation packet (RFC 9000 §6).
    async fn process_version_negotiation(&mut self, data: &[u8]) -> Result<usize, Error> {
        // First byte is header form + type, then 4 bytes version (0), then DCID + SCID lengths + data
        // The remaining data after the initial header bytes contains supported versions.
        if data.len() < 6 {
            return Err(Error::PacketDecode("Version Negotiation too short".into()));
        }
        // DCID
        let dcid_len = data[5] as usize;
        let off = 6 + dcid_len;
        if data.len() < off + 1 {
            return Err(Error::PacketDecode("Version Negotiation truncated".into()));
        }
        let scid_len = data[off] as usize;
        let versions_start = off + 1 + scid_len;

        let mut server_versions = Vec::new();
        let mut pos = versions_start;
        while pos + 4 <= data.len() {
            let v = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            server_versions.push(v);
            pos += 4;
        }

        // Check if our current version is NOT in the server's list.
        // The server sends VN if it doesn't support our version.
        // We should pick the best version from our SUPPORTED_VERSIONS that's also in server's list.
        let current = self.version;
        if server_versions.contains(&current) {
            // Our version is supported — VN is spurious, ignore
            return Ok(data.len());
        }

        // Try to find a mutually supported version (prefer newest from our list)
        let new_version = packet::SUPPORTED_VERSIONS.iter().find(|v| server_versions.contains(v));

        if let Some(&v) = new_version {
            // Reconnect with the new version
            self.version = v;
            // Reset connection state
            let (ck, sk) = crypto_keys::derive_initial_keys_for_version(
                self.config.tls_config.crypto_provider(),
                self.dcid.as_bytes(),
                v,
            );
            self.init_send = LevelSendState {
                keys: ck,
                pn: 0,
            };
            self.init_recv = sk;
            self.crypto_buffer.reset();
            self.crypto_buffer_hs.reset();
            self.ack_tracker[0] = AckTracker::new();
            self.loss_detect[0] = LossDetection::new(Duration::from_millis(self.config.max_ack_delay_ms));

            // Re-send ClientHello with new version
            let ch = self
                .tls
                .as_mut()
                .ok_or(Error::InvalidState("no TLS for VN re-send".into()))?
                .write_handshake()
                .ok_or(Error::InvalidState("no CH for VN re-send".into()))?;

            let token = self.retry_token.clone();
            self.send_crypto_initial(&ch, token.as_deref()).await?;
        } else {
            return Err(Error::ConnectionRejected("no mutually supported QUIC version".into()));
        }

        Ok(data.len())
    }

    async fn process_retry(&mut self, h: &packet::LongHeader, data: &[u8]) -> Result<usize, Error> {
        if data.len() < h.pn_offset + 16 {
            return Err(Error::PacketDecode("Retry packet too short for integrity tag".into()));
        }

        let tag_start = data.len() - 16;
        let pseudo_packet = &data[..tag_start];

        let original_dcid = self
            .original_dcid
            .as_ref()
            .ok_or(Error::InvalidState("no original DCID for Retry verification".into()))?;

        let mut aad = pseudo_packet.to_vec();
        aad.extend_from_slice(original_dcid.as_bytes());

        let (retry_key, retry_nonce): (&[u8], &[u8]) = if self.version == packet::QUIC_VERSION_V2 {
            (&RETRY_INTEGRITY_KEY_V2, &RETRY_INTEGRITY_NONCE_V2)
        } else {
            (&RETRY_INTEGRITY_KEY_V1, &RETRY_INTEGRITY_NONCE_V1)
        };

        let expected_tag = {
            use crypto::Aead;
            // Retry integrity uses AES-128-GCM regardless of cipher suite
            let k: &[u8; 16] = retry_key.try_into().expect("16-byte retry key");
            let cipher = crypto::aes::Aes128Gcm::new(k);
            let n: &[u8; 12] = retry_nonce.try_into().expect("12-byte retry nonce");
            let tag_arr = cipher.encrypt_in_place(&mut Vec::new(), n, &aad);
            let mut t = [0u8; 16];
            t.copy_from_slice(tag_arr.as_ref());
            t
        };

        let actual_tag = &data[tag_start..];
        if !constant_time_eq::constant_time_eq(&expected_tag, actual_tag) {
            return Err(Error::Crypto(crypto::AeadError::InvalidCiphertext));
        }

        let new_dcid = h.scid.clone();
        self.dcid = new_dcid.clone();

        if tag_start > h.pn_offset {
            self.retry_token = Some(data[h.pn_offset..tag_start].to_vec());
        }

        let (ck, sk) = crypto_keys::derive_initial_keys_for_version(
            self.config.tls_config.crypto_provider(),
            new_dcid.as_bytes(),
            self.version,
        );
        self.init_send.keys = ck;
        self.init_recv = sk;
        self.init_send.pn = 0;
        self.pn_recv[0] = 0;
        self.crypto_buffer.reset();
        self.crypto_buffer_hs.reset();
        // Reset Initial-level ACK/loss tracking
        self.ack_tracker[0] = AckTracker::new();
        self.loss_detect[0] = LossDetection::new(Duration::from_millis(self.config.max_ack_delay_ms));

        let ch = self
            .tls
            .as_mut()
            .ok_or(Error::InvalidState("no TLS state for Retry re-send".into()))?
            .write_handshake()
            .ok_or(Error::InvalidState("no CH to re-send after Retry".into()))?;

        let token = self.retry_token.clone();
        self.send_crypto_initial(&ch, token.as_deref()).await?;

        Ok(data.len())
    }

    async fn process_initial(&mut self, h: &packet::LongHeader, pkt: &[u8]) -> Result<(), Error> {
        let now = self.clock();
        let sample_start = h.pn_offset + 4;
        if sample_start + 16 > pkt.len() {
            return Err(Error::PacketDecode("packet too short for HP sample".into()));
        }
        let sample = &pkt[sample_start..sample_start + 16];
        let mut fb = pkt[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(&self.init_recv, true, &mut fb, &mut pn_b, sample);

        let server_scid = h.scid.clone();
        if server_scid != self.dcid {
            self.dcid = server_scid;
        }
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
        self.pn_recv[0] = pn + 1;

        // Track received packet for ACK
        self.ack_tracker[0].on_packet_received(now, pn, true);
        self.loss_detect[0].on_packet_received(now, true);
        // Send ACK immediately for Initial (don't schedule)
        self.send_ack(EncryptionLevel::Initial).await?;

        let frames = frames_from(&payload)?;
        self.handle_crypto(frames, EncryptionLevel::Initial).await
    }

    async fn process_handshake(&mut self, h: &packet::LongHeader, pkt: &[u8]) -> Result<(), Error> {
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

        // Track received packet for ACK
        self.ack_tracker[1].on_packet_received(now, pn, true);
        self.loss_detect[1].on_packet_received(now, true);
        self.send_ack(EncryptionLevel::Handshake).await?;

        let frames = frames_from(&payload)?;
        self.handle_crypto(frames, EncryptionLevel::Handshake).await
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

        if let Some(payload) = self.try_short_decrypt(data, pn_offset, sample, false) {
            let frames = frames_from(&payload)?;
            return self.process_short_frames(frames, data.len()).await;
        }

        if let Some(payload) = self.try_short_decrypt(data, pn_offset, sample, true) {
            let frames = frames_from(&payload)?;
            return self.process_short_frames(frames, data.len()).await;
        }

        Err(Error::Crypto(crypto::AeadError::InvalidCiphertext))
    }

    fn try_short_decrypt(&mut self, data: &[u8], pn_offset: usize, sample: &[u8], use_old: bool) -> Option<Vec<u8>> {
        let rk = if use_old {
            self.prev_app_recv.as_ref()?
        } else {
            self.app_recv.as_ref()?
        };
        let mut fb = data[0];
        let mut pn_b = data[pn_offset..pn_offset + 4].to_vec();
        crypto_keys::remove_header_protection(rk, false, &mut fb, &mut pn_b, sample);
        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], self.pn_recv[2]);
        let payload_offset = pn_offset + pn_len;
        let pkt_end = payload_offset + (data.len() - payload_offset);
        let mut aad = data[..payload_offset].to_vec();
        aad[0] = fb;
        aad[pn_offset..pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);
        let mut payload = data[payload_offset..pkt_end].to_vec();
        match crypto_keys::decrypt_payload(rk, pn, &aad, &mut payload) {
            Ok(()) => {
                self.pn_recv[2] = pn + 1;
                Some(payload)
            }
            Err(_) => None,
        }
    }

    async fn process_short_frames(&mut self, frames: Vec<Frame>, pkt_end: usize) -> Result<usize, Error> {
        let now = self.clock();
        let pn = self.pn_recv[2] - 1;
        let has_ack_eliciting = frames.iter().any(|f| !matches!(f, Frame::Ack { .. } | Frame::Padding));
        self.ack_tracker[2].on_packet_received(now, pn, has_ack_eliciting);
        self.loss_detect[2].on_packet_received(now, has_ack_eliciting);
        if has_ack_eliciting {
            self.schedule_ack(EncryptionLevel::OneRtt);
        }

        for f in frames {
            match f {
                Frame::Ack {
                    largest_acknowledged,
                    ack_delay,
                    first_ack_range,
                    ack_ranges,
                } => {
                    // Process ACK for 1RTT PN space
                    let ranges = AckRanges {
                        largest: largest_acknowledged,
                        delay: ack_delay,
                        first_range: first_ack_range,
                        extra_ranges: ack_ranges,
                    };
                    let (acked, sent_times) = self.ack_tracker[2].on_ack_received(&ranges);
                    if !acked.is_empty() {
                        self.loss_detect[2].on_ack_received(now);
                        // Compute RTT from best (most recent) sample
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
                        // Trigger MAX_STREAM_DATA update if approaching limit
                        let consumed = s.local_max_stream_data.saturating_sub(s.recv_offset);
                        if consumed <= s.local_max_stream_data / 2 {
                            s.local_max_stream_data += s.local_max_stream_data / 2;
                            s.needs_max_stream_data = true;
                        }
                    }
                    // Push data to the per-stream recv channel
                    let chunk = ReceiveChunk {
                        data: d.clone(),
                        fin,
                    };
                    if let Some(tx) = self.stream_data_tx.get(&id) {
                        let _ = tx.push(chunk);
                    }
                    // Queue peer-initiated streams for accept
                    if is_new_stream {
                        let dir = if id & 0x02 != 0 { StreamDir::Uni } else { StreamDir::Bi };
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
                    self.state = ConnState::Closed;
                    return Err(Error::ConnectionClosed(
                        error_code,
                        String::from_utf8_lossy(&reason_phrase).into(),
                    ));
                }
                Frame::PathChallenge {
                    data,
                } => {
                    self.send_frames_at_level(
                        EncryptionLevel::OneRtt,
                        false,
                        false,
                        &[Frame::PathResponse {
                            data,
                        }],
                    )
                    .await?;
                }
                Frame::PathResponse {
                    data,
                } => {
                    self.confirm_path_validation(&data);
                }
                Frame::MaxData {
                    maximum_data,
                } => {
                    self.send_flow.max_data = self.send_flow.max_data.max(maximum_data);
                    self.send_flow.blocked = false;
                }
                Frame::MaxStreamData {
                    stream_id,
                    maximum_stream_data,
                } => {
                    if let Some(s) = self.streams.get_mut(&stream_id) {
                        s.max_stream_data = s.max_stream_data.max(maximum_stream_data);
                    }
                }
                Frame::MaxStreams {
                    ..
                } => {
                    // Peer increased our stream limit — allow more opens
                }
                Frame::DataBlocked {
                    ..
                } => {
                    // Peer is blocked; we may need to send MAX_DATA.
                    // This is informational for us as receiver.
                }
                Frame::StreamDataBlocked {
                    stream_id, ..
                } => {
                    // Peer is blocked on this stream; send MAX_STREAM_DATA.
                    if let Some(s) = self.streams.get_mut(&stream_id) {
                        s.needs_max_stream_data = true;
                    }
                }
                Frame::StreamsBlocked {
                    ..
                } => {
                    // Peer is blocked on stream count; send MAX_STREAMS.
                }
                Frame::HandshakeDone => {
                    // Client: server has confirmed handshake is complete
                }
                _ => {}
            }
        }
        Ok(pkt_end)
    }

    fn schedule_ack(&mut self, level: EncryptionLevel) {
        let idx = level.index();
        let max_delay = Duration::from_millis(self.config.max_ack_delay_ms);
        let deadline = self.clock() + max_delay;
        self.ack_deadline[idx] = Some(deadline);
    }

    async fn handle_crypto(&mut self, frames: Vec<Frame>, level: EncryptionLevel) -> Result<(), Error> {
        let now = self.clock();
        let mut close = None;
        let mut crypto_frames = Vec::new();

        for f in frames {
            match f {
                Frame::Crypto {
                    offset,
                    data,
                } => crypto_frames.push((offset, data)),
                Frame::ConnectionClose {
                    error_code,
                    reason_phrase,
                    ..
                } => {
                    close = Some((error_code, reason_phrase));
                }
                Frame::Ack {
                    largest_acknowledged,
                    ack_delay,
                    first_ack_range,
                    ack_ranges,
                } => {
                    let space = if level == EncryptionLevel::Handshake { 1 } else { 0 };
                    let ranges = AckRanges {
                        largest: largest_acknowledged,
                        delay: ack_delay,
                        first_range: first_ack_range,
                        extra_ranges: ack_ranges,
                    };
                    let (acked, sent_times) = self.ack_tracker[space].on_ack_received(&ranges);
                    if !acked.is_empty() {
                        self.loss_detect[space].on_ack_received(now);
                        if space == 2 && self.key_update_pending {
                            self.key_update_pending = false;
                        }
                        if let Some(time_sent) = sent_times.last() {
                            let rtt = now.duration_since(*time_sent);
                            self.loss_detect[space].on_rtt_measurement(rtt, Duration::from_micros(ack_delay));
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some((code, reason)) = close {
            self.state = ConnState::Closed;
            return Err(Error::ConnectionClosed(code, String::from_utf8_lossy(&reason).into()));
        }

        // Collect contiguous chunks into a Vec without holding a self borrow
        let mut contiguous_chunks: Vec<Vec<u8>> = Vec::new();
        {
            let buf = match level {
                EncryptionLevel::Initial => &mut self.crypto_buffer,
                EncryptionLevel::Handshake => &mut self.crypto_buffer_hs,
                _ => return Ok(()),
            };
            for (offset, data) in &crypto_frames {
                if let Some(contiguous) = buf.ingest(*offset, data) {
                    contiguous_chunks.push(contiguous);
                }
            }
        }

        for chunk in contiguous_chunks {
            self.process_crypto_data(&chunk).await?;
        }
        Ok(())
    }

    async fn process_crypto_data(&mut self, data: &[u8]) -> Result<(), Error> {
        let tls = self.tls.as_mut().unwrap();
        tls.inject_handshake(data);
        let provider = self.config.tls_config.crypto_provider();
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
                    let rh = crypto_keys::derive_level_keys(
                        provider.clone(),
                        s.server_handshake_traffic_secret.as_slice(),
                        suite,
                    );
                    let lh = crypto_keys::derive_level_keys(
                        provider.clone(),
                        s.client_handshake_traffic_secret.as_slice(),
                        suite,
                    );
                    self.hs_send = Some(LevelSendState {
                        keys: lh,
                        pn: 0,
                    });
                    self.hs_recv = Some(rh);
                }
                if let Some(fin) = tls.write_handshake() {
                    self.send_frames_at_level(
                        EncryptionLevel::Handshake,
                        true,
                        false,
                        &[Frame::Crypto {
                            offset: 0,
                            data: fin,
                        }],
                    )
                    .await?;
                }
                let ra = crypto_keys::derive_level_keys(
                    provider.clone(),
                    s.server_application_traffic_secret.as_slice(),
                    suite,
                );
                let la = crypto_keys::derive_level_keys(
                    provider.clone(),
                    s.client_application_traffic_secret.as_slice(),
                    suite,
                );
                self.app_traffic_secret = Some(s.client_application_traffic_secret.to_vec());
                self.server_app_traffic_secret = Some(s.server_application_traffic_secret.to_vec());
                self.app_send = Some(LevelSendState {
                    keys: la,
                    pn: 0,
                });
                self.app_recv = Some(ra);
                self.state = ConnState::Established;
                self.established_at = Some(self.clock());
            }
            _ => {
                if self.hs_send.is_none() {
                    if let Some(suite) = tls.cipher_suite() {
                        if let Some(s) = tls.quic_secrets() {
                            let rh = crypto_keys::derive_level_keys(
                                provider.clone(),
                                s.server_handshake_traffic_secret.as_slice(),
                                suite,
                            );
                            let lh = crypto_keys::derive_level_keys(
                                provider.clone(),
                                s.client_handshake_traffic_secret.as_slice(),
                                suite,
                            );
                            self.hs_send = Some(LevelSendState {
                                keys: lh,
                                pn: 0,
                            });
                            self.hs_recv = Some(rh);
                        }
                    }
                }
                if let Some(d) = tls.write_handshake() {
                    if self.hs_send.is_some() {
                        self.send_frames_at_level(
                            EncryptionLevel::Handshake,
                            true,
                            false,
                            &[Frame::Crypto {
                                offset: 0,
                                data: d,
                            }],
                        )
                        .await?;
                    } else {
                        // HRR case: re-ClientHello at Initial encryption level
                        self.send_crypto_initial(&d, None).await?;
                    }
                }
            }
        }
        Ok(())
    }
}

// ── Free send helper (avoid borrow conflicts) ────────────────────────

/// General-purpose packet sender with ACK tracking.
pub(crate) async fn send_one_packet<T: Transport>(
    transport: &T,
    remote: SocketAddr,
    dcid: &ConnectionId,
    scid: &ConnectionId,
    ss: &mut LevelSendState,
    long_header: bool,
    is_initial: bool,
    frames: &[Frame],
    ack_tracker: &mut AckTracker,
    loss_detect: &mut LossDetection,
    last_activity: &mut Instant,
    level_u8: u8,
    version: u32,
    key_phase: u8,
) -> Result<(), Error> {
    let pn = ss.pn;
    ss.pn += 1;

    let mut payload = Vec::new();
    let ack_eliciting = frames.iter().any(|f| !matches!(f, Frame::Padding | Frame::Ack { .. }));
    for f in frames {
        frame::encode(f, &mut payload);
    }

    let pn_len = crypto_keys::pn_encoding_len(pn, 0);
    let mut header = Vec::new();
    let pn_start = if long_header {
        let flag: u8 = match is_initial {
            true => 0xc0,
            false => 0xe0,
        };
        header.push(flag | ((pn_len - 1) as u8));
        header.extend_from_slice(&version.to_be_bytes());
        header.push(dcid.len() as u8);
        header.extend_from_slice(dcid.as_bytes());
        header.push(scid.len() as u8);
        header.extend_from_slice(scid.as_bytes());
        if is_initial {
            header.push(0);
        }
        let pkt_len = pn_len + payload.len() + 16;
        varint::encode(pkt_len as u64, &mut header);
        let start = header.len();
        crypto_keys::encode_pn(pn, pn_len, &mut header);
        start
    } else {
        let mut first = 0x40u8 | ((pn_len - 1) as u8);
        if key_phase != 0 {
            first |= 0x04;
        }
        header.push(first);
        header.extend_from_slice(dcid.as_bytes());
        let start = header.len();
        crypto_keys::encode_pn(pn, pn_len, &mut header);
        start
    };

    let aad = header.clone();
    let mut encrypted = payload;
    crypto_keys::encrypt_payload(&ss.keys, pn, &aad, &mut encrypted)?;
    let mut full = aad;
    full.extend_from_slice(&encrypted);

    let sample_start = pn_start + 4;
    if sample_start + 16 <= full.len() {
        let mut s = [0u8; 16];
        s.copy_from_slice(&full[sample_start..sample_start + 16]);
        let (before, pn_and_after) = full.split_at_mut(pn_start);
        let pn_region = &mut pn_and_after[..pn_len];
        crypto_keys::apply_header_protection(&ss.keys, long_header, &mut before[0], pn_region, &s);
    }

    transport.send_to(remote, &full).await?;

    let now = transport.now();
    if ack_eliciting {
        let frames_vec: Vec<Frame> = frames.to_vec();
        ack_tracker.on_packet_sent(now, pn, true, level_u8, long_header, is_initial, frames_vec);
    }
    loss_detect.on_packet_sent(now, ack_eliciting);
    *last_activity = now;
    Ok(())
}

pub(crate) fn frames_from(data: &[u8]) -> Result<Vec<Frame>, Error> {
    frame::decode_all(data)
}

pub(crate) fn build_transport_params(config: &Config, scid: &ConnectionId) -> Vec<Param> {
    vec![
        Param {
            ty: ParamType::InitialSourceConnectionId as u64,
            value: scid.as_bytes().to_vec(),
        },
        Param {
            ty: ParamType::InitialMaxData as u64,
            value: enc_varint(config.initial_max_data),
        },
        Param {
            ty: ParamType::InitialMaxStreamDataBidiLocal as u64,
            value: enc_varint(config.initial_max_stream_data_bidi_local),
        },
        Param {
            ty: ParamType::InitialMaxStreamDataBidiRemote as u64,
            value: enc_varint(config.initial_max_stream_data_bidi_remote),
        },
        Param {
            ty: ParamType::InitialMaxStreamDataUni as u64,
            value: enc_varint(config.initial_max_stream_data_uni),
        },
        Param {
            ty: ParamType::InitialMaxStreamsBidi as u64,
            value: enc_varint(config.initial_max_streams_bidi),
        },
        Param {
            ty: ParamType::InitialMaxStreamsUni as u64,
            value: enc_varint(config.initial_max_streams_uni),
        },
        Param {
            ty: ParamType::MaxIdleTimeout as u64,
            value: enc_varint(config.max_idle_timeout_ms),
        },
    ]
}

pub(crate) fn enc_varint(v: u64) -> Vec<u8> {
    let mut b = Vec::new();
    varint::encode(v, &mut b);
    b
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    use super::*;

    fn test_provider() -> Arc<dyn tls::crypto::CryptoProvider> {
        Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new())
    }

    #[test]
    fn initial_packet_send_receive_roundtrip() {
        let dcid = ConnectionId::random(8);
        let scid = ConnectionId::random(8);
        let (client_keys, _server_keys) = crypto_keys::derive_initial_keys(test_provider(), dcid.as_bytes());
        let ss = LevelSendState {
            keys: client_keys,
            pn: 0,
        };
        let payload = b"hello quic";

        // Build a minimal Initial packet manually for test
        let pn = ss.pn;
        let pn_len = crypto_keys::pn_encoding_len(pn, 0);
        let mut pkt_payload = Vec::new();
        frame::encode(
            &Frame::Crypto {
                offset: 0,
                data: payload.to_vec(),
            },
            &mut pkt_payload,
        );
        let pad_needed = 1180usize.saturating_sub(pkt_payload.len());
        if pad_needed > 0 {
            frame::pad_to(1180, pkt_payload.len(), &mut pkt_payload);
        }
        let mut header = Vec::new();
        let flag: u8 = 0xc0 | ((pn_len - 1) as u8);
        header.push(flag);
        header.extend_from_slice(&packet::QUIC_VERSION_V1.to_be_bytes());
        header.push(dcid.len() as u8);
        header.extend_from_slice(dcid.as_bytes());
        header.push(scid.len() as u8);
        header.extend_from_slice(scid.as_bytes());
        header.push(0);
        let pkt_len = pn_len + pkt_payload.len() + 16;
        varint::encode(pkt_len as u64, &mut header);
        let pn_start = header.len();
        crypto_keys::encode_pn(pn, pn_len, &mut header);
        let aad = header.clone();
        let mut encrypted = pkt_payload;
        crypto_keys::encrypt_payload(&ss.keys, pn, &aad, &mut encrypted).unwrap();
        let mut full = aad;
        full.extend_from_slice(&encrypted);
        let sample_start = pn_start + 4;
        if sample_start + 16 <= full.len() {
            let mut s = [0u8; 16];
            s.copy_from_slice(&full[sample_start..sample_start + 16]);
            let (before, pn_and_after) = full.split_at_mut(pn_start);
            let pn_region = &mut pn_and_after[..pn_len];
            crypto_keys::apply_header_protection(&ss.keys, true, &mut before[0], pn_region, &s);
        }

        assert!(full.len() >= 1200, "Initial packet must be at least 1200 bytes total");

        let h = packet::parse_long_header(&full).unwrap();
        assert_eq!(h.ty, packet::LongPacketType::Initial);

        let sample_start = h.pn_offset + 4;
        assert!(sample_start + 16 <= full.len(), "packet too short for sample");
        let sample = &full[sample_start..sample_start + 16];
        let mut fb = full[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(&ss.keys, true, &mut fb, &mut pn_b, sample);

        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], 0);
        let payload_offset = h.pn_offset + pn_len;

        let mut aad = full[..payload_offset].to_vec();
        aad[0] = fb;
        aad[h.pn_offset..h.pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);

        let mut decrypted = full[payload_offset..].to_vec();
        crypto_keys::decrypt_payload(&ss.keys, pn, &aad, &mut decrypted).unwrap();

        assert_eq!(&decrypted[..3], &[0x06, 0x00, payload.len() as u8]);
        assert_eq!(&decrypted[3..3 + payload.len()], payload.as_slice());
    }

    #[test]
    fn initial_packet_with_zero_byte_pn_roundtrip() {
        let dcid = ConnectionId::random(8);
        let scid = ConnectionId::random(8);
        let (client_keys, _server_keys) = crypto_keys::derive_initial_keys(test_provider(), dcid.as_bytes());
        let ss = LevelSendState {
            keys: client_keys,
            pn: 0,
        };
        let payload = b"0123456789";
        let pn = ss.pn;
        let pn_len = crypto_keys::pn_encoding_len(pn, 0);
        let mut pkt_payload = Vec::new();
        frame::encode(
            &Frame::Crypto {
                offset: 0,
                data: payload.to_vec(),
            },
            &mut pkt_payload,
        );
        let pad_needed = 1180usize.saturating_sub(pkt_payload.len());
        if pad_needed > 0 {
            frame::pad_to(1180, pkt_payload.len(), &mut pkt_payload);
        }
        let mut header = Vec::new();
        let flag: u8 = 0xc0 | ((pn_len - 1) as u8);
        header.push(flag);
        header.extend_from_slice(&packet::QUIC_VERSION_V1.to_be_bytes());
        header.push(dcid.len() as u8);
        header.extend_from_slice(dcid.as_bytes());
        header.push(scid.len() as u8);
        header.extend_from_slice(scid.as_bytes());
        header.push(0);
        let pkt_len = pn_len + pkt_payload.len() + 16;
        varint::encode(pkt_len as u64, &mut header);
        let pn_start = header.len();
        crypto_keys::encode_pn(pn, pn_len, &mut header);
        let aad = header.clone();
        let mut encrypted = pkt_payload;
        crypto_keys::encrypt_payload(&ss.keys, pn, &aad, &mut encrypted).unwrap();
        let mut full = aad;
        full.extend_from_slice(&encrypted);
        let sample_start = pn_start + 4;
        if sample_start + 16 <= full.len() {
            let mut s = [0u8; 16];
            s.copy_from_slice(&full[sample_start..sample_start + 16]);
            let (before, pn_and_after) = full.split_at_mut(pn_start);
            let pn_region = &mut pn_and_after[..pn_len];
            crypto_keys::apply_header_protection(&ss.keys, true, &mut before[0], pn_region, &s);
        }

        assert!(full.len() >= 1200);

        let h = packet::parse_long_header(&full).unwrap();
        let sample_start = h.pn_offset + 4;
        let sample = &full[sample_start..sample_start + 16];
        let mut fb = full[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(&ss.keys, true, &mut fb, &mut pn_b, sample);

        let pn_len = ((fb & 0x03) + 1) as usize;
        assert_eq!(pn_len, 1, "pn should be encoded in 1 byte");
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], 0);
        assert_eq!(pn, 0);

        let payload_offset = h.pn_offset + pn_len;
        let mut aad = full[..payload_offset].to_vec();
        aad[0] = fb;
        aad[h.pn_offset..h.pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);

        let mut decrypted = full[payload_offset..].to_vec();
        crypto_keys::decrypt_payload(&ss.keys, pn, &aad, &mut decrypted).unwrap();
        assert_eq!(&decrypted[..3], &[0x06, 0x00, payload.len() as u8]);
        assert_eq!(&decrypted[3..3 + payload.len()], payload.as_slice());
    }
}
