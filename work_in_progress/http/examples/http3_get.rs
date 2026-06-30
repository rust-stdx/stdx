//! Example: connect to an HTTP/3 server, send GET /, and print the response.
use std::{
    net::{SocketAddr, ToSocketAddrs},
    time::Duration,
};

use http::http3::{self, frame, qpack};
use quic::{Config, Instant, IoError, Transport};
use tokio::net::UdpSocket as TokioUdpSocket;

struct UdpTransport {
    socket: TokioUdpSocket,
    epoch: std::time::Instant,
}

#[async_trait::async_trait]
impl Transport for UdpTransport {
    async fn send_to(&self, dest: SocketAddr, data: &[u8]) -> Result<usize, IoError> {
        self.socket.send_to(data, dest).await.map_err(IoError::from)
    }
    async fn receive_from(&self, buf: &mut [u8], deadline: Option<Duration>) -> Result<(usize, SocketAddr), IoError> {
        if let Some(dur) = deadline {
            tokio::time::timeout(dur, self.socket.recv_from(buf))
                .await
                .map_err(|_| IoError::TimedOut)?
                .map_err(IoError::from)
        } else {
            self.socket.recv_from(buf).await.map_err(IoError::from)
        }
    }
    fn local_addr(&self) -> Result<SocketAddr, IoError> {
        self.socket.local_addr().map_err(IoError::from)
    }
    fn now(&self) -> Instant {
        let us = std::time::Instant::now().duration_since(self.epoch).as_micros() as u64;
        Instant::from_micros(us)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_name = std::env::args().nth(1).unwrap_or("cloudflare-quic.com".to_string());
    let addr = format!("{server_name}:443")
        .to_socket_addrs()?
        .filter(|a| a.is_ipv4())
        .next()
        .unwrap();
    println!("Connecting to {server_name} at {addr}...");
    let mut config = Config::default();
    config.alpn_protocols = vec![bytes::Bytes::from_static(b"h3")];
    let transport = UdpTransport {
        socket: TokioUdpSocket::bind("0.0.0.0:0").await?,
        epoch: std::time::Instant::now(),
    };
    let mut conn = quic::Connection::new(transport, config);
    conn.connect(addr, &server_name).await?;
    println!("QUIC handshake complete");

    let mut ctl = conn.open_unidirectional_stream().await?;
    let mut ctl_buf = Vec::new();
    quic::varint::encode(http3::CONTROL_STREAM_TYPE, &mut ctl_buf);
    ctl_buf.extend_from_slice(&frame::encode_frame(&frame::Frame::Settings(Vec::new())));
    ctl.send(&ctl_buf, false)?;
    conn.receive_one().await?;

    let mut enc = conn.open_unidirectional_stream().await?;
    let mut buf = Vec::new();
    quic::varint::encode(http3::QPACK_ENCODER_STREAM_TYPE, &mut buf);
    quic::varint::encode(0x20, &mut buf);
    enc.send(&buf, false)?;
    conn.receive_one().await?;

    let mut dec = conn.open_unidirectional_stream().await?;
    let mut buf = Vec::new();
    quic::varint::encode(http3::QPACK_DECODER_STREAM_TYPE, &mut buf);
    dec.send(&buf, false)?;
    conn.receive_one().await?;

    let (mut req_send, mut req_recv) = conn.open_bidirectional_stream().await?;

    println!("Waiting for server response...");
    let mut server = conn.accept_unidirectional_stream().await?;

    tokio::spawn(async move {
        loop {
            if let Err(_) = conn.receive_one().await {
                break;
            }
        }
    });

    let mut buf = vec![0u8; 65536];
    loop {
        if let Some(n) = server.receive(&mut buf).await? {
            let data = &buf[..n];
            if data.is_empty() {
                continue;
            }
            if let Ok((_st, stl)) = quic::varint::decode(data) {
                let fd = &data[stl..];
                if fd.is_empty() {
                    continue;
                }
                if let Ok((f, _)) = frame::decode_frame(fd) {
                    if let frame::Frame::Settings(s) = f {
                        println!("Server SETTINGS: {s:?}");
                        break;
                    }
                }
            }
            if let Ok((f, _)) = frame::decode_frame(data) {
                if let frame::Frame::Settings(s) = f {
                    println!("Server SETTINGS: {s:?}");
                    break;
                }
            }
        }
    }

    println!("Sending GET / ...");
    let mut enc = qpack::QpackEncoder::new();
    req_send.send(
        &frame::encode_frame(&frame::Frame::Headers(enc.encode(&[
            (":method", "GET"),
            (":scheme", "https"),
            (":authority", &server_name),
            (":path", "/"),
            ("user-agent", "stdx-h3/0.1"),
        ]))),
        true,
    )?;

    println!("Reading response...");
    let mut dec = qpack::QpackDecoder::new();
    let mut headers = Vec::new();
    let mut body = Vec::new();
    let mut pending = Vec::new();
    let mut stream_done = false;
    loop {
        if pending.is_empty() && !stream_done {
            match req_recv.receive(&mut buf).await? {
                Some(n) => pending.extend_from_slice(&buf[..n]),
                None => stream_done = true,
            }
        }
        if pending.is_empty() && stream_done {
            break;
        }
        match frame::decode_frame(&pending) {
            Ok((f, consumed)) => {
                match f {
                    frame::Frame::Headers(d) => {
                        pending.drain(..consumed);
                        headers = dec.decode(&d).map_err(|e| format!("qpack: {e:?}"))?;
                    }
                    frame::Frame::Data(d) => {
                        body.extend_from_slice(&d);
                        pending.drain(..consumed);
                    }
                    frame::Frame::GoAway {
                        stream_id,
                    } => {
                        println!("Server GoAway (stream_id={stream_id})");
                        pending.drain(..consumed);
                    }
                    frame::Frame::Grease(ty, _) => {
                        println!("GREASE frame type=0x{ty:x}");
                        pending.drain(..consumed);
                    }
                    frame::Frame::CancelPush(_) | frame::Frame::PushPromise(_) | frame::Frame::MaxPushId(_) => {
                        // Server push frames — not expected for a simple GET.
                        pending.drain(..consumed);
                    }
                    frame::Frame::Settings(_) => {
                        // Already handled on the control stream above; ignore here.
                        pending.drain(..consumed);
                    }
                    frame::Frame::Unknown(ty, _payload) => {
                        // A small type value likely means we're reading HTML body content
                        // that happens to look like a valid frame header.
                        if ty < 0x100 {
                            body.extend_from_slice(&pending[..consumed]);
                            pending.drain(..consumed);
                            body.extend_from_slice(&pending);
                            pending.clear();
                            while let Some(n) = req_recv.receive(&mut buf).await? {
                                body.extend_from_slice(&buf[..n]);
                            }
                            stream_done = true;
                        } else {
                            pending.drain(..consumed);
                        }
                    }
                }
            }
            Err(frame::FrameDecodeError::Incomplete) | Err(frame::FrameDecodeError::BadVarint) => {
                if stream_done {
                    // End of stream: consume any trailing data as body content
                    if !pending.is_empty() {
                        // Check if it starts with a DATA frame header
                        if pending.len() >= 2 && pending[0] == 0x00 {
                            if let Ok((_ty, tc)) = quic::varint::decode(&pending) {
                                if let Ok((_len, lc)) = quic::varint::decode(&pending[tc..]) {
                                    let payload_start = tc + lc;
                                    if pending.len() > payload_start {
                                        body.extend_from_slice(&pending[payload_start..]);
                                    }
                                }
                            }
                        }
                        // Still have bytes after attempted parse: add to body
                        if !pending.is_empty() {
                            body.extend_from_slice(&pending);
                        }
                    }
                    break;
                }
                match req_recv.receive(&mut buf).await? {
                    Some(n) => pending.extend_from_slice(&buf[..n]),
                    None => stream_done = true,
                }
            }
            Err(e) => return Err(format!("frame decode error: {e:?}").into()),
        }
    }
    for (n, v) in &headers {
        println!("{n}: {v}");
    }
    let body = String::from_utf8_lossy(&body);
    if !body.is_empty() {
        println!("\n{body}");
    }
    Ok(())
}
