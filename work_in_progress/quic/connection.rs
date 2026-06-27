use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
};

use bytes::Bytes;

use crate::{
    cid::ConnectionId,
    config::Config,
    crypto_keys::{self, DirectionKeys},
    error::Error,
    frame::{self, Frame},
    packet::{self, LongPacketType},
    stream::{Stream, StreamAllocator},
    tls_adapter::{TlsAdapter, TlsEvent},
    transport::Transport,
    transport_params::{self, Param, ParamType},
    varint,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionLevel {
    Initial,
    Handshake,
    OneRtt,
}

/// Sending state for one encryption level.
struct LevelSendState {
    keys: DirectionKeys,
    pn: u64,
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
    dcid: ConnectionId,
    scid: ConnectionId,
    init_send: LevelSendState,
    init_recv: DirectionKeys,
    hs_send: Option<LevelSendState>,
    hs_recv: Option<DirectionKeys>,
    app_send: Option<LevelSendState>,
    app_recv: Option<DirectionKeys>,
    tls: Option<TlsAdapter>,
    pn_recv: [u64; 3],
    streams: HashMap<u64, Stream>,
    stream_alloc: StreamAllocator,
    datagram_queue: VecDeque<Vec<u8>>,
    stream_data_queue: VecDeque<(u64, Vec<u8>)>,
}

impl<T: Transport> Connection<T> {
    pub fn new(transport: T, config: Config) -> Self {
        let remote = "0.0.0.0:0".parse().unwrap();
        let dcid = ConnectionId::new(&[0; 8]);
        let (ck, sk) = crypto_keys::derive_initial_keys(dcid.as_bytes());
        Connection {
            transport,
            config,
            remote,
            server_name: String::new(),
            state: ConnState::Connecting,
            dcid,
            scid: ConnectionId::random(8),
            init_send: LevelSendState {
                keys: ck,
                pn: 0,
            },
            init_recv: sk,
            hs_send: None,
            hs_recv: None,
            app_send: None,
            app_recv: None,
            pn_recv: [0; 3],
            tls: None,
            streams: HashMap::new(),
            stream_alloc: StreamAllocator::new(),
            datagram_queue: VecDeque::new(),
            stream_data_queue: VecDeque::new(),
        }
    }

    pub async fn connect(&mut self, remote: SocketAddr, server_name: &str) -> Result<(), Error> {
        self.remote = remote;
        self.server_name = server_name.to_owned();
        let dcid = ConnectionId::random(8);
        self.dcid = dcid.clone();
        let (ck, sk) = crypto_keys::derive_initial_keys(dcid.as_bytes());
        self.init_send = LevelSendState {
            keys: ck,
            pn: 0,
        };
        self.init_recv = sk;

        let tps = transport_params::encode(&build_transport_params(&self.config, &self.scid));
        let alpn: Vec<Bytes> = self
            .config
            .alpn_protocols
            .iter()
            .map(|a| Bytes::copy_from_slice(a))
            .collect();
        self.tls = Some(TlsAdapter::new(self.config.tls_config.clone(), server_name, &tps, &alpn)?);
        let ch = self
            .tls
            .as_mut()
            .unwrap()
            .write_handshake()
            .ok_or(Error::InvalidState("no CH".into()))?;

        // RFC 9000 §14: Initial packets must be padded to at least 1200 bytes
        let send = prepare_initial_packet(&ch, &self.init_send, &self.dcid, &self.scid);
        self.init_send.pn += 1;
        self.transport.send_to(self.remote, &send).await?;

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            if self.is_established() {
                return Ok(());
            }
            if std::time::Instant::now() > deadline {
                return Err(Error::ConnectionTimedOut);
            }
            self.recv_one().await?;
        }
    }

    pub fn is_established(&self) -> bool {
        matches!(self.state, ConnState::Established)
    }

    pub async fn open_bi(&mut self) -> Result<u64, Error> {
        let id = self.stream_alloc.next_bi();
        self.streams.insert(id, Stream::new(id));
        Ok(id)
    }

    pub async fn open_uni(&mut self) -> Result<u64, Error> {
        let id = self.stream_alloc.next_uni();
        self.streams.insert(id, Stream::new(id));
        Ok(id)
    }

    pub async fn stream_send(&mut self, id: u64, data: &[u8], fin: bool) -> Result<(), Error> {
        self.streams
            .get_mut(&id)
            .ok_or(Error::StreamNotFound(id))?
            .write(data, fin);
        if let Some(ref mut ss) = self.app_send {
            let frames: Vec<Frame> = self
                .streams
                .iter()
                .filter(|(_, s)| !s.send_buffer.is_empty() || s.fin_sent)
                .map(|(&id, s)| Frame::Stream {
                    id,
                    offset: s.send_offset,
                    data: s.send_buffer.clone(),
                    fin: s.fin_sent,
                })
                .collect();
            if !frames.is_empty() {
                send_packet(&self.transport, self.remote, &self.dcid, &self.scid, false, false, ss, &frames).await?;
            }
        }
        Ok(())
    }

    pub async fn stream_recv(&mut self, buf: &mut [u8]) -> Result<Option<(u64, usize)>, Error> {
        loop {
            if let Some((sid, data)) = self.stream_data_queue.pop_front() {
                let n = data.len().min(buf.len());
                buf[..n].copy_from_slice(&data[..n]);
                return Ok(Some((sid, n)));
            }
            if self.recv_one().await.is_err() {
                return Ok(None);
            }
        }
    }

    pub async fn send_datagram(&mut self, data: &[u8]) -> Result<(), Error> {
        if let Some(ref mut ss) = self.app_send {
            send_packet(
                &self.transport,
                self.remote,
                &self.dcid,
                &self.scid,
                false,
                false,
                ss,
                &[Frame::Datagram {
                    data: data.to_vec(),
                }],
            )
            .await?;
        }
        Ok(())
    }

    pub async fn recv_datagram(&mut self) -> Result<Option<Vec<u8>>, Error> {
        if let Some(d) = self.datagram_queue.pop_front() {
            return Ok(Some(d));
        }
        self.recv_one().await.ok();
        Ok(self.datagram_queue.pop_front())
    }

    pub async fn close(&mut self, error_code: u64, reason: &[u8]) -> Result<(), Error> {
        let frame = Frame::ConnectionClose {
            error_code,
            frame_type: None,
            reason_phrase: reason.to_vec(),
        };
        if let Some(ref mut ss) = self.app_send {
            send_packet(&self.transport, self.remote, &self.dcid, &self.scid, false, false, ss, &[frame]).await?;
        } else if let Some(ref mut ss) = self.hs_send {
            send_packet(&self.transport, self.remote, &self.dcid, &self.scid, true, true, ss, &[frame]).await?;
        } else {
            send_packet(
                &self.transport,
                self.remote,
                &self.dcid,
                &self.scid,
                true,
                true,
                &mut self.init_send,
                &[frame],
            )
            .await?;
        }
        self.state = ConnState::Closed;
        Ok(())
    }

    // ── Receive ─────────────────────────────────────────────────────────

    async fn recv_one(&mut self) -> Result<(), Error> {
        let mut buf = [0u8; 1500];
        match tokio::time::timeout(std::time::Duration::from_secs(1), self.transport.recv_from(&mut buf)).await {
            Ok(Ok((0, _))) => Ok(()),
            Ok(Ok((n, _))) => {
                let mut data = &buf[..n];
                // Handle coalesced QUIC packets: process packets one at a time
                // from the datagram until all data is consumed.
                while !data.is_empty() {
                    let consumed = if data[0] >> 7 == 1 {
                        self.process_long(data).await?
                    } else if self.app_recv.is_none() {
                        // Short header during handshake: remaining data is
                        // zero-padding in the UDP datagram, not a real QUIC packet.
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
            Ok(Err(e)) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(()),
            Ok(Err(e)) => Err(Error::Io(e)),
            Err(_) => Ok(()), // timeout, no packet received
        }
    }

    /// Process one long-header packet from `data`. Returns bytes consumed.
    async fn process_long(&mut self, data: &[u8]) -> Result<usize, Error> {
        let header = packet::parse_long_header(data)?;
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
            _ => return Err(Error::ProtocolViolation("unexpected long packet type".into())),
        }
        Ok(pkt_end)
    }

    async fn process_initial(&mut self, h: &packet::LongHeader, pkt: &[u8]) -> Result<(), Error> {
        let sample_start = h.pn_offset + 4;
        if sample_start + 16 > pkt.len() {
            return Err(Error::PacketDecode("packet too short for HP sample".into()));
        }
        let sample = &pkt[sample_start..sample_start + 16];
        let mut fb = pkt[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(&self.init_recv, true, &mut fb, &mut pn_b, sample);

        // RFC 9000 §7.2: client updates its DCID to the server's SCID
        let server_scid = h.scid.clone();
        if server_scid != self.dcid {
            self.dcid = server_scid;
        }
        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], self.pn_recv[0]);
        let payload_offset = h.pn_offset + pn_len;

        // The encrypted payload ends at hdr.pn_offset + header.payload_length.
        // This is the QUIC packet boundary (includes AEAD tag).
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

        // Send ACK for received Initial packet
        send_packet(
            &self.transport,
            self.remote,
            &self.dcid,
            &self.scid,
            true,
            true,
            &mut self.init_send,
            &[Frame::Ack {
                largest_acknowledged: pn,
                ack_delay: 0,
                ack_ranges: Vec::new(),
            }],
        )
        .await?;

        let frames = frames_from(&payload)?;
        self.handle_crypto(frames).await
    }

    async fn process_handshake(&mut self, h: &packet::LongHeader, pkt: &[u8]) -> Result<(), Error> {
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

        // Send ACK for received Handshake packet
        if let Some(ref mut ss) = self.hs_send {
            send_packet(
                &self.transport,
                self.remote,
                &self.dcid,
                &self.scid,
                true,
                false,
                ss,
                &[Frame::Ack {
                    largest_acknowledged: pn,
                    ack_delay: 0,
                    ack_ranges: Vec::new(),
                }],
            )
            .await?;
        }
        let frames = frames_from(&payload)?;
        self.handle_crypto(frames).await
    }

    async fn process_short(&mut self, data: &[u8]) -> Result<usize, Error> {
        let rk = self
            .app_recv
            .as_ref()
            .ok_or(Error::InvalidState("no 1RTT recv keys".into()))?;
        let dcid_len = self.dcid.len();
        if data.len() < 1 + dcid_len + 4 {
            return Err(Error::PacketDecode("short packet too short".into()));
        }
        let pn_offset = 1 + dcid_len;
        let sample_start = pn_offset + 4;
        if sample_start + 16 > data.len() {
            return Err(Error::PacketDecode("short packet too short for HP sample".into()));
        }
        let sample = &data[sample_start..sample_start + 16];
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
        crypto_keys::decrypt_payload(rk, pn, &aad, &mut payload)?;
        self.pn_recv[2] += 1;
        for f in frames_from(&payload)? {
            match f {
                Frame::Stream {
                    id,
                    data: d,
                    fin,
                    ..
                } => {
                    self.streams.entry(id).or_insert_with(|| Stream::new(id));
                    if let Some(s) = self.streams.get_mut(&id) {
                        s.recv_buffer.extend_from_slice(&d);
                        if fin {
                            s.fin_received = true;
                        }
                    }
                    self.stream_data_queue.push_back((id, d));
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
                    if let Some(ref mut ss) = self.app_send {
                        send_packet(
                            &self.transport,
                            self.remote,
                            &self.dcid,
                            &self.scid,
                            false,
                            false,
                            ss,
                            &[Frame::PathResponse {
                                data,
                            }],
                        )
                        .await?;
                    }
                }
                _ => {}
            }
        }
        Ok(pkt_end)
    }

    async fn handle_crypto(&mut self, frames: Vec<Frame>) -> Result<(), Error> {
        let mut close = None;
        let mut crypto_data = Vec::new();
        for f in frames {
            match f {
                Frame::Crypto {
                    data, ..
                } => crypto_data.push(data),
                Frame::ConnectionClose {
                    error_code,
                    reason_phrase,
                    ..
                } => {
                    close = Some((error_code, reason_phrase));
                }
                _ => {}
            }
        }
        if let Some((code, reason)) = close {
            self.state = ConnState::Closed;
            return Err(Error::ConnectionClosed(code, String::from_utf8_lossy(&reason).into()));
        }
        for data in crypto_data {
            let tls = self.tls.as_mut().unwrap();
            tls.inject_handshake(&data);
            match tls.process().await? {
                TlsEvent::HandshakeComplete => {
                    let suite = tls
                        .cipher_suite()
                        .ok_or(Error::InvalidState("no cipher suite".into()))?;
                    let s = tls.quic_secrets().unwrap();
                    if self.hs_send.is_none() {
                        let rh = crypto_keys::derive_level_keys(s.server_handshake_traffic_secret.as_slice(), suite);
                        let lh = crypto_keys::derive_level_keys(s.client_handshake_traffic_secret.as_slice(), suite);
                        self.hs_send = Some(LevelSendState {
                            keys: lh,
                            pn: 0,
                        });
                        self.hs_recv = Some(rh);
                    }
                    if let Some(fin) = tls.write_handshake() {
                        let ss = self.hs_send.as_mut().unwrap();
                        send_packet(
                            &self.transport,
                            self.remote,
                            &self.dcid,
                            &self.scid,
                            true,
                            false,
                            ss,
                            &[Frame::Crypto {
                                offset: 0,
                                data: fin,
                            }],
                        )
                        .await?;
                    }
                    let ra = crypto_keys::derive_level_keys(s.server_application_traffic_secret.as_slice(), suite);
                    let la = crypto_keys::derive_level_keys(s.client_application_traffic_secret.as_slice(), suite);
                    self.app_send = Some(LevelSendState {
                        keys: la,
                        pn: 0,
                    });
                    self.app_recv = Some(ra);
                    // Client must not send HANDSHAKE_DONE; handshake is considered complete
                    // when TLS handshake finishes. The server sends HANDSHAKE_DONE.
                    self.state = ConnState::Established;
                }
                _ => {
                    // Derive handshake keys as soon as the TLS key schedule is available
                    // (after ServerHello processing). The server sends EncryptedExtensions,
                    // Certificate, CertificateVerify, and Finished in Handshake packets,
                    // which the client must be able to decrypt.
                    if self.hs_send.is_none() {
                        if let Some(suite) = tls.cipher_suite() {
                            if let Some(s) = tls.quic_secrets() {
                                let rh =
                                    crypto_keys::derive_level_keys(s.server_handshake_traffic_secret.as_slice(), suite);
                                let lh =
                                    crypto_keys::derive_level_keys(s.client_handshake_traffic_secret.as_slice(), suite);
                                self.hs_send = Some(LevelSendState {
                                    keys: lh,
                                    pn: 0,
                                });
                                self.hs_recv = Some(rh);
                            }
                        }
                    }
                    if let Some(d) = tls.write_handshake() {
                        if let Some(ref mut ss) = self.hs_send {
                            send_packet(
                                &self.transport,
                                self.remote,
                                &self.dcid,
                                &self.scid,
                                true,
                                false,
                                ss,
                                &[Frame::Crypto {
                                    offset: 0,
                                    data: d,
                                }],
                            )
                            .await?;
                        } else {
                            // HRR case: re-ClientHello at Initial encryption level
                            let send = prepare_initial_packet(&d, &self.init_send, &self.dcid, &self.scid);
                            self.transport.send_to(self.remote, &send).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn frames_from(data: &[u8]) -> Result<Vec<Frame>, Error> {
    frame::decode_all(data)
}

/// Standalone packet builder and sender (no borrow conflicts).
async fn send_packet<T: Transport>(
    transport: &T,
    remote: SocketAddr,
    dcid: &ConnectionId,
    scid: &ConnectionId,
    long_header: bool,
    is_initial: bool,
    ss: &mut LevelSendState,
    frames: &[Frame],
) -> Result<(), Error> {
    let pn = ss.pn;
    ss.pn += 1;

    let mut payload = Vec::new();
    for f in frames {
        frame::encode(f, &mut payload);
    }

    let pn_len = crypto_keys::pn_encoding_len(pn, 0);
    let mut header = Vec::new();
    let pn_start = if long_header {
        let flag: u8 = match is_initial {
            true => 0xc0,  // Initial
            false => 0xe0, // Handshake (0x20 | 0xc0)
        };
        header.push(flag | ((pn_len - 1) as u8));
        header.extend_from_slice(&packet::QUIC_VERSION_V1.to_be_bytes());
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
        header.push(0x40 | ((pn_len - 1) as u8));
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

    // Apply header protection using a sample from the encrypted payload.
    let sample_start = pn_start + 4;
    if sample_start + 16 <= full.len() {
        let sample = {
            let mut s = [0u8; 16];
            s.copy_from_slice(&full[sample_start..sample_start + 16]);
            s
        };
        let (before, pn_and_after) = full.split_at_mut(pn_start);
        let pn_region = &mut pn_and_after[..pn_len];
        crypto_keys::apply_header_protection(&ss.keys, long_header, &mut before[0], pn_region, &sample);
    }

    transport.send_to(remote, &full).await.map(|_| ()).map_err(Error::from)
}

fn build_transport_params(config: &Config, scid: &ConnectionId) -> Vec<Param> {
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

fn enc_varint(v: u64) -> Vec<u8> {
    let mut b = Vec::new();
    varint::encode(v, &mut b);
    b
}

/// Build and encrypt an Initial packet with padding to at least 1200 bytes.
fn prepare_initial_packet(
    crypto_data: &[u8],
    ss: &LevelSendState,
    dcid: &ConnectionId,
    scid: &ConnectionId,
) -> Vec<u8> {
    let pn = ss.pn;
    let pn_len = crypto_keys::pn_encoding_len(ss.pn, 0);

    // Build frames: CRYPTO + PADDING to reach ~1180 payload (header ~20 bytes)
    let mut payload = Vec::new();
    frame::encode(
        &Frame::Crypto {
            offset: 0,
            data: crypto_data.to_vec(),
        },
        &mut payload,
    );
    let pad_needed = 1180usize.saturating_sub(payload.len());
    if pad_needed > 0 {
        frame::pad_to(1180, payload.len(), &mut payload);
    }

    let mut header = Vec::new();
    let flag: u8 = 0xc0 | ((pn_len - 1) as u8);
    header.push(flag);
    header.extend_from_slice(&packet::QUIC_VERSION_V1.to_be_bytes());
    header.push(dcid.len() as u8);
    header.extend_from_slice(dcid.as_bytes());
    header.push(scid.len() as u8);
    header.extend_from_slice(scid.as_bytes());
    header.push(0); // empty token length
    let pkt_len = pn_len + payload.len() + 16;
    varint::encode(pkt_len as u64, &mut header);
    let pn_start = header.len();
    crypto_keys::encode_pn(pn, pn_len, &mut header);

    let aad = header.clone();
    let mut encrypted = payload;
    crypto_keys::encrypt_payload(&ss.keys, pn, &aad, &mut encrypted).unwrap();
    let mut full = aad;
    full.extend_from_slice(&encrypted);

    // Apply header protection using a sample from the encrypted payload.
    let sample_start = pn_start + 4;
    if sample_start + 16 <= full.len() {
        let sample = {
            let mut s = [0u8; 16];
            s.copy_from_slice(&full[sample_start..sample_start + 16]);
            s
        };
        let (before, pn_and_after) = full.split_at_mut(pn_start);
        let pn_region = &mut pn_and_after[..pn_len];
        crypto_keys::apply_header_protection(&ss.keys, true, &mut before[0], pn_region, &sample);
    }

    full
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_packet_send_receive_roundtrip() {
        let dcid = ConnectionId::random(8);
        let scid = ConnectionId::random(8);
        let (client_keys, _server_keys) = crypto_keys::derive_initial_keys(dcid.as_bytes());
        let ss = LevelSendState {
            keys: client_keys.clone(),
            pn: 0,
        };
        let payload = b"hello quic";
        let pkt = prepare_initial_packet(payload, &ss, &dcid, &scid);

        assert!(pkt.len() >= 1200, "Initial packet must be at least 1200 bytes total");

        let h = packet::parse_long_header(&pkt).unwrap();
        assert_eq!(h.ty, packet::LongPacketType::Initial);

        let sample_start = h.pn_offset + 4;
        assert!(sample_start + 16 <= pkt.len(), "packet too short for sample");
        let sample = &pkt[sample_start..sample_start + 16];
        let mut fb = pkt[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(&client_keys, true, &mut fb, &mut pn_b, sample);

        let pn_len = ((fb & 0x03) + 1) as usize;
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], 0);
        let payload_offset = h.pn_offset + pn_len;

        let mut aad = pkt[..payload_offset].to_vec();
        aad[0] = fb;
        aad[h.pn_offset..h.pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);

        let mut decrypted = pkt[payload_offset..].to_vec();
        crypto_keys::decrypt_payload(&client_keys, pn, &aad, &mut decrypted).unwrap();

        assert_eq!(&decrypted[..3], &[0x06, 0x00, payload.len() as u8]);
        assert_eq!(&decrypted[3..3 + payload.len()], payload.as_slice());
    }

    #[test]
    fn initial_packet_with_zero_byte_pn_roundtrip() {
        let dcid = ConnectionId::random(8);
        let scid = ConnectionId::random(8);
        let (client_keys, _server_keys) = crypto_keys::derive_initial_keys(dcid.as_bytes());
        let ss = LevelSendState {
            keys: client_keys.clone(),
            pn: 0,
        };
        let payload = b"0123456789";
        let pkt = prepare_initial_packet(payload, &ss, &dcid, &scid);

        assert!(pkt.len() >= 1200);

        let h = packet::parse_long_header(&pkt).unwrap();
        let sample_start = h.pn_offset + 4;
        let sample = &pkt[sample_start..sample_start + 16];
        let mut fb = pkt[0];
        let mut pn_b = h.pn_raw.clone();
        crypto_keys::remove_header_protection(&client_keys, true, &mut fb, &mut pn_b, sample);

        let pn_len = ((fb & 0x03) + 1) as usize;
        assert_eq!(pn_len, 1, "pn should be encoded in 1 byte");
        let pn = crypto_keys::decode_pn(&pn_b[..pn_len], 0);
        assert_eq!(pn, 0);

        let payload_offset = h.pn_offset + pn_len;
        let mut aad = pkt[..payload_offset].to_vec();
        aad[0] = fb;
        aad[h.pn_offset..h.pn_offset + pn_len].copy_from_slice(&pn_b[..pn_len]);

        let mut decrypted = pkt[payload_offset..].to_vec();
        crypto_keys::decrypt_payload(&client_keys, pn, &aad, &mut decrypted).unwrap();
        assert_eq!(&decrypted[..3], &[0x06, 0x00, payload.len() as u8]);
        assert_eq!(&decrypted[3..3 + payload.len()], payload.as_slice());
    }
}
