use bytes::Bytes;
use quic::{Config, Connection, Instant, IoError, varint};

use super::super::{error::Error, pool::PoolKey};
use crate::{
    common::{Body, Frame as HttpFrame, Request, Response, StatusCode, Version},
    http3::{self, frame, qpack},
};

pub(crate) struct Http3Conn {
    pub(crate) conn: std::sync::Arc<Connection>,
    #[allow(dead_code)]
    pub(crate) key: PoolKey,
    qpack_enc: qpack::QpackEncoder,
    qpack_dec: qpack::QpackDecoder,
}

struct UdpTransport {
    socket: tokio::net::UdpSocket,
    epoch: std::time::Instant,
}

#[async_trait::async_trait]
impl quic::Transport for UdpTransport {
    async fn send_to(&self, dest: std::net::SocketAddr, data: &[u8]) -> Result<usize, IoError> {
        self.socket.send_to(data, dest).await.map_err(IoError::from)
    }

    async fn receive_from(
        &self,
        buf: &mut [u8],
        deadline: Option<std::time::Duration>,
    ) -> Result<(usize, std::net::SocketAddr), IoError> {
        let recv = self.socket.recv_from(buf);
        match deadline {
            Some(d) => match tokio::time::timeout(d, recv).await {
                Ok(result) => result.map_err(IoError::from),
                Err(_) => Err(IoError::WouldBlock),
            },
            None => recv.await.map_err(IoError::from),
        }
    }

    fn local_addr(&self) -> Result<std::net::SocketAddr, IoError> {
        self.socket.local_addr().map_err(IoError::from)
    }

    fn now(&self) -> Instant {
        let us = std::time::Instant::now().duration_since(self.epoch).as_micros() as u64;
        Instant::from_micros(us)
    }
}

#[allow(dead_code)]
impl Http3Conn {
    pub async fn connect(host: &str, port: u16) -> Result<Self, Error> {
        let remote_addr = format!("{host}:{port}")
            .parse::<std::net::SocketAddr>()
            .map_err(|e| Error::Connect(format!("invalid address: {e}")))?;

        let udp = tokio::net::UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(|e| Error::Connect(e.to_string()))?;

        let mut config = Config::default();
        config.alpn_protocols = vec![Bytes::from_static(b"h3")];

        let conn = Connection::connect(std::sync::Arc::new(
            UdpTransport {
                socket: udp,
                epoch: std::time::Instant::now(),
            },
            config,
            remote_addr,
            host,
        ))
        .await
        .map_err(|e| Error::H3(format!("QUIC connect: {e:?}")))?;

        // Spawn background poll task
        let bg = conn.clone();
        tokio::spawn(async move {
            loop {
                if let Err(_) = bg.poll().await {
                    break;
                }
            }
        });

        // Open control stream
        let mut ctl = conn
            .open_unidirectional_stream()
            .await
            .map_err(|e| Error::H3(format!("open control: {e:?}")))?;

        let mut ctl_buf = Vec::new();
        varint::encode(http3::CONTROL_STREAM_TYPE, &mut ctl_buf);
        ctl_buf.extend_from_slice(&frame::encode_frame(&frame::Frame::Settings(Vec::new())));
        ctl.send(&ctl_buf, false)
            .await
            .map_err(|e| Error::H3(format!("send control: {e:?}")))?;

        // Open QPACK encoder stream
        let mut enc_stream = conn
            .open_unidirectional_stream()
            .await
            .map_err(|e| Error::H3(format!("open qpack enc: {e:?}")))?;

        let mut enc_buf = Vec::new();
        varint::encode(http3::QPACK_ENCODER_STREAM_TYPE, &mut enc_buf);
        varint::encode(0x20, &mut enc_buf);
        enc_stream
            .send(&enc_buf, false)
            .await
            .map_err(|e| Error::H3(format!("send qpack enc: {e:?}")))?;

        // Open QPACK decoder stream
        let mut dec_stream = conn
            .open_unidirectional_stream()
            .await
            .map_err(|e| Error::H3(format!("open qpack dec: {e:?}")))?;

        let mut dec_buf = Vec::new();
        varint::encode(http3::QPACK_DECODER_STREAM_TYPE, &mut dec_buf);
        dec_stream
            .send(&dec_buf, false)
            .await
            .map_err(|e| Error::H3(format!("send qpack dec: {e:?}")))?;

        // Read server SETTINGS from control stream
        let mut server = conn
            .accept_unidirectional_stream()
            .await
            .map_err(|e| Error::H3(format!("accept ctl: {e:?}")))?;

        let mut buf = vec![0u8; 65536];
        loop {
            let n = server
                .receive(&mut buf)
                .await
                .map_err(|e| Error::H3(format!("server ctl recv: {e:?}")))?;
            match n {
                Some(n) => {
                    let data = &buf[..n];
                    if let Ok((_st, stl)) = varint::decode(data) {
                        let fd = &data[stl..];
                        if !fd.is_empty() {
                            let _ = frame::decode_frame(fd);
                        }
                    }
                    break;
                }
                None => return Err(Error::ConnectionClosed),
            }
        }

        let key = PoolKey {
            host: host.to_string(),
            port,
            tls: true,
        };

        Ok(Http3Conn {
            conn,
            key,
            qpack_enc: qpack::QpackEncoder::new(),
            qpack_dec: qpack::QpackDecoder::new(),
        })
    }

    pub async fn send_request(&mut self, req: Request) -> Result<Response, Error> {
        let _ = self.conn.poll().await;

        let (mut req_send, mut req_recv) = self
            .conn
            .open_bidirectional_stream()
            .await
            .map_err(|e| Error::H3(format!("open bi stream: {e:?}")))?;

        let authority = req
            .headers
            .iter()
            .find(|(n, _)| n.as_str() == "host")
            .map(|(_, v)| v.as_str().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let headers_encoded = self.qpack_enc.encode(&[
            (":method", req.method.as_str()),
            (":scheme", "https"),
            (":authority", &authority),
            (":path", req.uri.path()),
        ]);

        let has_body = req.body.is_some();
        req_send
            .send(&frame::encode_frame(&frame::Frame::Headers(headers_encoded)), !has_body)
            .await
            .map_err(|e| Error::H3(format!("send headers: {e:?}")))?;

        if let Some(mut body) = req.body {
            loop {
                let frame_data = body.next_frame().await;
                match frame_data {
                    Some(Ok(HttpFrame::Data(data))) => {
                        req_send
                            .send(&frame::encode_frame(&frame::Frame::Data(data.to_vec())), false)
                            .await
                            .map_err(|e| Error::H3(format!("send data: {e:?}")))?;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                    None => {
                        req_send
                            .send(&Vec::new(), true)
                            .await
                            .map_err(|e| Error::H3(format!("send fin: {e:?}")))?;
                        break;
                    }
                }
            }
        }

        let _ = self.conn.poll().await;

        let mut buf = vec![0u8; 65536];
        let mut pending_buf = Vec::new();
        let mut headers = Vec::new();
        let mut body = Vec::new();
        let mut status = None;

        loop {
            if pending_buf.is_empty() {
                match req_recv
                    .receive(&mut buf)
                    .await
                    .map_err(|e| Error::H3(format!("recv: {e:?}")))?
                {
                    Some(n) => pending_buf.extend_from_slice(&buf[..n]),
                    None => break,
                }
            }

            match frame::decode_frame(&pending_buf) {
                Ok((f, consumed)) => {
                    pending_buf.drain(..consumed);
                    match f {
                        frame::Frame::Headers(d) => {
                            let decoded = self
                                .qpack_dec
                                .decode(&d)
                                .map_err(|e| Error::H3(format!("qpack: {e:?}")))?;
                            for (name, value) in decoded {
                                if name == ":status" {
                                    status = value.parse::<u16>().ok().and_then(StatusCode::from_u16);
                                } else {
                                    headers.push((name, value));
                                }
                            }
                        }
                        frame::Frame::Data(d) => {
                            body.extend_from_slice(&d);
                        }
                        _ => {}
                    }
                }
                Err(_) => {
                    match req_recv
                        .receive(&mut buf)
                        .await
                        .map_err(|e| Error::H3(format!("recv more: {e:?}")))?
                    {
                        Some(n) => pending_buf.extend_from_slice(&buf[..n]),
                        None => break,
                    }
                }
            }
        }

        Ok(Response {
            version: Version::Http3,
            status: status.unwrap_or(StatusCode::Ok),
            headers: headers
                .into_iter()
                .map(|(n, v)| (n.as_str().into(), v.as_str().into()))
                .collect(),
            body: Bytes::from(body),
        })
    }
}
