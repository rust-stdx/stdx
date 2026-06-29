//! QUIC server-side connection (RFC 9000 §7).
//!
//! `ServerConnection` handles incoming client Initial packets,
//! completes the TLS handshake as a server, and provides
//! stream/datagram I/O for the established connection.
//!
//! # Usage
//!
//! ```ignore
//! let mut server = quic::server::ServerConnection::new(transport, config);
//! server.accept(first_initial_packet).await?;
//! let (id, n) = server.stream_recv(&mut buf).await?;
//! ```

use core::net::SocketAddr;
use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

use crate::{
    ack::AckTracker,
    cid::ConnectionId,
    cmd_queue::CmdSender,
    config::Config,
    crypto_keys::{self, DirectionKeys},
    error::Error,
    loss::LossDetection,
    packet::{self, LongPacketType},
    stream::{ReceiveChunk, RecvFlowController, SendFlowController, Stream, StreamAllocator},
    tls_adapter::TlsAdapter,
    transport::Transport,
};

struct LevelSendState {
    keys: DirectionKeys,
    pn: u64,
}

/// Server-side QUIC connection.
///
/// Created with a transport and config, then fed incoming packets
/// via `accept()` to complete the handshake.
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
    streams: HashMap<u64, Stream>,
    stream_alloc: StreamAllocator,
    datagram_queue: VecDeque<Vec<u8>>,
    cmd_rx: crate::cmd_queue::CmdReceiver<crate::stream::StreamCommand>,
    base_cmd_tx: crate::cmd_queue::CmdSender<crate::stream::StreamCommand>,
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
        let _recv_buf_size = config.recv_buf_size;
        let max_idle_timeout = Duration::from_millis(config.max_idle_timeout_ms);
        let (cmd_tx, cmd_rx) = crate::cmd_queue::cmd_queue();
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
                keys: placeholder_keys(),
                pn: 0,
            },
            init_recv: placeholder_keys(),
            hs_send: None,
            hs_recv: None,
            app_send: None,
            app_recv: None,
            app_traffic_secret: None,
            tls: None,
            pn_recv: [0; 3],
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
            last_activity: Instant::now(),
            ack_deadline: [None, None, None],
            send_flow: SendFlowController::new(initial_max_data),
            recv_flow: RecvFlowController::new(initial_max_data),
            idle_deadline: Instant::now() + max_idle_timeout,
        }
    }

    /// Process an incoming Initial packet to begin the server handshake.
    /// Returns `Ok(true)` when the handshake is complete and the connection
    /// is ready for application data.
    pub async fn accept(&mut self, src: SocketAddr, initial_packet: &[u8]) -> Result<bool, Error> {
        self.remote = src;
        self.last_activity = Instant::now();

        let header = packet::parse_long_header(initial_packet)?;
        if header.ty != LongPacketType::Initial {
            return Err(Error::ProtocolViolation("expected Initial packet".into()));
        }
        self.version = header.version;
        self.dcid = header.scid.clone();
        self.original_dcid = Some(header.dcid.clone());
        let (ck, sk) = crypto_keys::derive_initial_keys_for_version(header.dcid.as_bytes(), self.version);
        // Server uses server keys for recv, client keys for send
        self.init_recv = ck;
        self.init_send = LevelSendState {
            keys: sk,
            pn: 0,
        };

        // TODO: Set up TLS with server config
        // For now, this is a scaffold that returns an error
        // Full server TLS requires a ServerConfig from the tls crate
        Err(Error::InvalidState(
            "ServerConnection.accept requires server TLS config — not yet fully wired".into(),
        ))
    }

    pub fn is_established(&self) -> bool {
        matches!(self.state, ServerState::Established)
    }

    pub async fn receive_one(&mut self) -> Result<(), Error> {
        let buf_size = self.config.recv_buf_size;
        let mut buf = vec![0u8; buf_size];
        let recv_result = self
            .transport
            .receive_from(&mut buf, Some(Duration::from_millis(200)))
            .await;
        match recv_result {
            Ok((n, _)) => {
                self.last_activity = Instant::now();
                let _data = &buf[..n];
                // Packet processing would go here
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => Ok(()),
            Err(e) => Err(Error::Io(e)),
        }
    }
}

fn placeholder_keys() -> crypto_keys::DirectionKeys {
    let provider = std::sync::Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new());
    let (ck, _sk) = crypto_keys::derive_initial_keys_for_version(&[0u8; 8], 0x00000001);
    ck
}
