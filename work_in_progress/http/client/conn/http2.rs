use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use bytes::Bytes;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::super::{error::Error, pool::PoolKey};
use crate::{
    common::{Body, Frame, Request, Response, StatusCode, Version},
    http2::{
        PREFACE, frame,
        hpack::{HpackDecoder, HpackEncoder},
    },
};

pub(crate) struct Http2Conn {
    pub key: PoolKey,
    tx: tokio::sync::mpsc::UnboundedSender<H2Command>,
    _closed: Arc<AtomicBool>,
}

type H2Respond = tokio::sync::oneshot::Sender<Result<Response, Error>>;

struct H2Command {
    req: Request,
    respond: H2Respond,
}

struct PendingRequest {
    respond: H2Respond,
    status: Option<StatusCode>,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    end_stream: bool,
}

impl Http2Conn {
    pub async fn from_stream(
        host: &str,
        port: u16,
        mut stream: impl AsyncReadExt + AsyncWriteExt + Unpin + Send + 'static,
    ) -> Result<Self, Error> {
        stream.write_all(PREFACE).await.map_err(Error::from)?;

        let settings_frame = frame::encode_frame(&frame::Frame::Settings {
            ack: false,
            settings: vec![],
        });
        stream.write_all(&settings_frame).await.map_err(Error::from)?;

        let mut acked = false;
        let mut read_buf = vec![0u8; 65536];
        let mut frame_buf = Vec::new();

        while !acked {
            let n = stream.read(&mut read_buf).await.map_err(Error::from)?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            frame_buf.extend_from_slice(&read_buf[..n]);
            loop {
                match frame::decode_frame(&frame_buf) {
                    Ok((f, consumed)) => {
                        frame_buf.drain(..consumed);
                        match f {
                            frame::Frame::Settings {
                                ack: false, ..
                            } => {
                                let ack = frame::encode_frame(&frame::Frame::Settings {
                                    ack: true,
                                    settings: vec![],
                                });
                                stream.write_all(&ack).await.map_err(Error::from)?;
                                acked = true;
                            }
                            frame::Frame::Settings {
                                ack: true, ..
                            } => {
                                acked = true;
                            }
                            frame::Frame::GoAway {
                                last_stream_id,
                                error_code,
                                ..
                            } => {
                                return Err(Error::H2(format!(
                                    "GOAWAY before request: stream={last_stream_id}, err={error_code:?}"
                                )));
                            }
                            _ => {}
                        }
                    }
                    Err(frame::H2FrameError::Incomplete) => break,
                    Err(e) => return Err(Error::H2(format!("frame decode: {e:?}"))),
                }
            }
        }

        let key = PoolKey {
            host: host.to_string(),
            port,
            tls: true,
        };
        let closed = Arc::new(AtomicBool::new(false));
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let closed_clone = closed.clone();
        tokio::spawn(async move {
            h2_driver(stream, rx, closed_clone).await;
        });

        Ok(Http2Conn {
            key,
            tx,
            _closed: closed,
        })
    }

    pub async fn send_request(&self, req: Request) -> Result<Response, Error> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(H2Command {
                req,
                respond,
            })
            .map_err(|_| Error::DriverTerminated)?;
        rx.await.map_err(|_| Error::DriverTerminated)?
    }
}

async fn h2_driver(
    mut stream: impl AsyncReadExt + AsyncWriteExt + Unpin,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<H2Command>,
    closed: Arc<AtomicBool>,
) {
    let mut hpack_enc = HpackEncoder::new();
    let mut hpack_dec = HpackDecoder::new();
    let mut next_stream_id: u32 = 1;
    let mut pending: HashMap<u32, PendingRequest> = HashMap::new();
    let mut read_buf = vec![0u8; 65536];
    let mut frame_buf = Vec::new();

    loop {
        tokio::select! {
            cmd = rx.recv() => {
                let H2Command { req, respond } = match cmd {
                    Some(c) => c,
                    None => break,
                };

                let stream_id = next_stream_id;
                next_stream_id += 2;

                let authority = req.headers
                    .iter()
                    .find(|(n, _)| n.as_str() == "host")
                    .map(|(_, v)| v.as_str().to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                let mut hpack_headers: Vec<(&str, &str)> = vec![
                    (":method", req.method.as_str()),
                    (":scheme", "https"),
                    (":authority", &authority),
                    (":path", req.uri.path()),
                ];
                for (name, value) in &req.headers {
                    let n = name.as_str();
                    if n != "host" && !n.starts_with(':') {
                        hpack_headers.push((n, value.as_str()));
                    }
                }

                let fragment = hpack_enc.encode(&hpack_headers);
                let has_body = req.body.is_some();
                let end_stream = !has_body;

                let hdr_frame = frame::Frame::Headers {
                    stream_id,
                    end_stream,
                    end_headers: true,
                    padded: false,
                    priority: false,
                    exclusive: false,
                    stream_dependency: 0,
                    weight: 0,
                    fragment,
                    padding: vec![],
                };

                if let Err(e) = write_frame(&mut stream, &hdr_frame).await {
                    let _ = respond.send(Err(Error::H2(format!("write HEADERS: {e}"))));
                    closed.store(true, Ordering::Release);
                    return;
                }

                pending.insert(stream_id, PendingRequest {
                    respond,
                    status: None,
                    headers: Vec::new(),
                    body: Vec::new(),
                    end_stream,
                });

                if let Some(mut body) = req.body {
                    loop {
                        let frame_data = body.next_frame().await;
                        let frame_data = match frame_data {
                            Some(Ok(f)) => f,
                            Some(Err(_)) => {
                                fail_pending(&mut pending, stream_id, Error::BodyError("body stream error".into()));
                                break;
                            }
                            None => {
                                let end = frame::Frame::Data {
                                    stream_id,
                                    end_stream: true,
                                    padded: false,
                                    data: Vec::new(),
                                    padding: vec![],
                                };
                                let _ = write_frame(&mut stream, &end).await;
                                if let Some(p) = pending.get_mut(&stream_id) {
                                    p.end_stream = true;
                                }
                                break;
                            }
                        };
                        match frame_data {
                            Frame::Data(data) => {
                                let df = frame::Frame::Data {
                                    stream_id,
                                    end_stream: false,
                                    padded: false,
                                    data: data.to_vec(),
                                    padding: vec![],
                                };
                                if let Err(e) = write_frame(&mut stream, &df).await {
                                    fail_pending(&mut pending, stream_id, Error::H2(format!("write DATA: {e}")));
                                    closed.store(true, Ordering::Release);
                                    return;
                                }
                            }
                            Frame::Trailers(t) => {
                                let trailers: Vec<(&str, &str)> = t.iter()
                                    .map(|(n, v)| (n.as_str(), v.as_str()))
                                    .collect();
                                let tf = hpack_enc.encode(&trailers);
                                let hf = frame::Frame::Headers {
                                    stream_id,
                                    end_stream: true,
                                    end_headers: true,
                                    padded: false,
                                    priority: false,
                                    exclusive: false,
                                    stream_dependency: 0,
                                    weight: 0,
                                    fragment: tf,
                                    padding: vec![],
                                };
                                if let Err(e) = write_frame(&mut stream, &hf).await {
                                    fail_pending(&mut pending, stream_id, Error::H2(format!("write trailers: {e}")));
                                    closed.store(true, Ordering::Release);
                                    return;
                                }
                                if let Some(p) = pending.get_mut(&stream_id) {
                                    p.end_stream = true;
                                }
                            }
                        }
                    }
                }
            }

            result = async {
                let n = stream.read(&mut read_buf).await?;
                Ok::<_, std::io::Error>(n)
            } => {
                let n = match result {
                    Ok(n) => n,
                    Err(e) => {
                        fail_all_pending(&mut pending, Error::Io(e.to_string()));
                        closed.store(true, Ordering::Release);
                        break;
                    }
                };
                if n == 0 {
                    fail_all_pending(&mut pending, Error::ConnectionClosed);
                    closed.store(true, Ordering::Release);
                    break;
                }
                frame_buf.extend_from_slice(&read_buf[..n]);

                loop {
                    match frame::decode_frame(&frame_buf) {
                        Ok((f, consumed)) => {
                            frame_buf.drain(..consumed);
                            handle_h2_frame(f, &mut pending, &mut hpack_dec);
                        }
                        Err(frame::H2FrameError::Incomplete) => break,
                        Err(e) => {
                            fail_all_pending(&mut pending, Error::H2(format!("frame decode: {e:?}")));
                            closed.store(true, Ordering::Release);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn handle_h2_frame(f: frame::Frame, pending: &mut HashMap<u32, PendingRequest>, hpack_dec: &mut HpackDecoder) {
    match f {
        frame::Frame::Headers {
            stream_id,
            fragment,
            end_headers,
            end_stream,
            ..
        } => {
            if end_headers {
                if let Ok(decoded) = hpack_dec.decode(&fragment) {
                    if let Some(p) = pending.get_mut(&stream_id) {
                        for (name, value) in decoded {
                            if name == ":status" {
                                p.status = value.parse::<u16>().ok().and_then(StatusCode::from_u16);
                            } else {
                                p.headers.push((name, value));
                            }
                        }
                    }
                }
            }
            if end_stream {
                complete_pending(pending, stream_id);
            }
        }
        frame::Frame::Data {
            stream_id,
            data,
            end_stream,
            ..
        } => {
            if let Some(p) = pending.get_mut(&stream_id) {
                p.body.extend_from_slice(&data);
                p.end_stream = end_stream;
            }
            if end_stream {
                complete_pending(pending, stream_id);
            }
        }
        frame::Frame::Continuation {
            stream_id,
            fragment,
            end_headers,
            ..
        } => {
            if end_headers {
                if let Ok(decoded) = hpack_dec.decode(&fragment) {
                    if let Some(p) = pending.get_mut(&stream_id) {
                        for (name, value) in decoded {
                            if name == ":status" {
                                p.status = value.parse::<u16>().ok().and_then(StatusCode::from_u16);
                            } else {
                                p.headers.push((name, value));
                            }
                        }
                    }
                }
            }
        }
        frame::Frame::RstStream {
            stream_id,
            error_code,
        } => {
            fail_pending(pending, stream_id, Error::H2(format!("stream reset: {error_code:?}")));
        }
        _ => {}
    }
}

fn complete_pending(pending: &mut HashMap<u32, PendingRequest>, stream_id: u32) {
    if let Some(p) = pending.remove(&stream_id) {
        let resp = Response {
            version: Version::Http2,
            status: p.status.unwrap_or(StatusCode::Ok),
            headers: p
                .headers
                .into_iter()
                .map(|(n, v)| (n.as_str().into(), v.as_str().into()))
                .collect(),
            body: Bytes::from(p.body),
        };
        let _ = p.respond.send(Ok(resp));
    }
}

fn fail_pending(pending: &mut HashMap<u32, PendingRequest>, stream_id: u32, err: Error) {
    if let Some(p) = pending.remove(&stream_id) {
        let _ = p.respond.send(Err(err));
    }
}

fn fail_all_pending(pending: &mut HashMap<u32, PendingRequest>, err: Error) {
    for (_id, p) in pending.drain() {
        let _ = p.respond.send(Err(Error::H2(format!("{err}"))));
    }
}

async fn write_frame(stream: &mut (impl AsyncWriteExt + Unpin), f: &frame::Frame) -> Result<(), String> {
    let encoded = frame::encode_frame(f);
    stream.write_all(&encoded).await.map_err(|e| format!("{e}"))
}
