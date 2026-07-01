use std::{net::ToSocketAddrs, sync::Arc};

use http::http2::{
    self, frame,
    hpack::{HpackDecoder, HpackEncoder},
};
use tls::{
    CertificateValidator, ClientConfig, Error as TlsError, ReceivedCertificate,
    crypto_default_provider::DefaultCryptoProvider, io_tokio::TlsConnector,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

struct AcceptAll;

#[async_trait::async_trait]
impl CertificateValidator for AcceptAll {
    async fn validate(&self, _cert: &ReceivedCertificate, _server_name: Option<&str>) -> Result<(), TlsError> {
        Ok(())
    }
}

async fn read_raw_frame<S: AsyncReadExt + Unpin>(
    stream: &mut S,
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut hdr = [0u8; 9];
    match stream.read_exact(&mut hdr).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e.into()),
    }
    let length = ((hdr[0] as u32) << 16) | ((hdr[1] as u32) << 8) | (hdr[2] as u32);
    let mut frame = hdr.to_vec();
    frame.resize(9 + length as usize, 0);
    stream.read_exact(&mut frame[9..]).await?;
    Ok(Some(frame))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::args().nth(1).unwrap_or_else(|| "example.com".to_string());
    let addr = format!("{host}:443").to_socket_addrs()?.next().unwrap();
    println!("Connecting to {host} at {addr}...");

    let tcp = TcpStream::connect(addr).await?;
    println!("TCP connected");

    let config = ClientConfig::new(
        Arc::new(DefaultCryptoProvider::new()),
        [bytes::Bytes::from_static(b"h2")].into(),
        Arc::new(AcceptAll),
    );
    let connector = TlsConnector::new(config);
    let mut stream = connector.connect(&host, tcp).await?;
    println!("TLS handshake complete (ALPN: h2)");

    stream.write_all(http2::PREFACE).await?;
    stream
        .write_all(&frame::encode_frame(&frame::Frame::Settings {
            ack: false,
            settings: vec![],
        }))
        .await?;
    println!("Sent preface + SETTINGS");

    let mut acked = false;
    let mut sv = None;
    while !acked {
        let raw = match read_raw_frame(&mut stream).await? {
            Some(r) => r,
            None => {
                eprintln!("Server closed connection early");
                return Ok(());
            }
        };
        let (f, _) = frame::decode_frame(&raw).map_err(|e| format!("frame decode: {e:?}"))?;
        match f {
            frame::Frame::Settings {
                ack: true, ..
            } => acked = true,
            frame::Frame::Settings {
                ack: false,
                settings: s,
            } => sv = Some(s),
            frame::Frame::GoAway {
                last_stream_id,
                error_code,
                ..
            } => {
                eprintln!("GOAWAY before request: stream={last_stream_id}, err={error_code:?}");
                return Ok(());
            }
            _ => {}
        }
    }

    if let Some(s) = sv {
        println!("Server SETTINGS: {s:?}");
        stream
            .write_all(&frame::encode_frame(&frame::Frame::Settings {
                ack: true,
                settings: vec![],
            }))
            .await?;
    }

    let mut encoder = HpackEncoder::new();
    let hp = encoder.encode(&[
        (":method", "GET"),
        (":scheme", "https"),
        (":authority", host.as_str()),
        (":path", "/"),
        ("accept", "*/*"),
    ]);

    stream
        .write_all(&frame::encode_frame(&frame::Frame::Headers {
            stream_id: 1,
            end_stream: true,
            end_headers: true,
            padded: false,
            priority: false,
            exclusive: false,
            stream_dependency: 0,
            weight: 0,
            fragment: hp,
            padding: vec![],
        }))
        .await?;
    println!("Sent GET /");

    let mut body = Vec::new();
    let mut decoder = HpackDecoder::new();
    let mut done = false;
    while !done {
        let raw = match read_raw_frame(&mut stream).await? {
            Some(r) => r,
            None => break,
        };
        let (f, _) = frame::decode_frame(&raw).map_err(|e| format!("frame decode: {e:?}"))?;
        match f {
            frame::Frame::Headers {
                fragment, ..
            }
            | frame::Frame::Continuation {
                fragment, ..
            } => match decoder.decode(&fragment) {
                Ok(headers) => {
                    for (n, v) in &headers {
                        println!("{n}: {v}");
                    }
                }
                Err(e) => eprintln!("HPACK decode error: {e:?}"),
            },
            frame::Frame::Data {
                data,
                end_stream,
                ..
            } => {
                body.extend_from_slice(&data);
                if end_stream {
                    done = true;
                }
            }
            frame::Frame::Settings {
                ack: false, ..
            } => {
                stream
                    .write_all(&frame::encode_frame(&frame::Frame::Settings {
                        ack: true,
                        settings: vec![],
                    }))
                    .await?;
            }
            frame::Frame::GoAway {
                last_stream_id,
                error_code,
                ..
            } => {
                println!("GOAWAY: stream={last_stream_id}, err={error_code:?}");
                done = true;
            }
            frame::Frame::Ping {
                ack: false,
                opaque_data,
            } => {
                stream
                    .write_all(&frame::encode_frame(&frame::Frame::Ping {
                        ack: true,
                        opaque_data,
                    }))
                    .await?;
            }
            _ => {}
        }
    }
    if !body.is_empty() {
        println!("\n{}", String::from_utf8_lossy(&body));
    }
    Ok(())
}
