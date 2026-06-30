//! Example: connect to a server via QUIC.
//!
//! Run: `cargo run --example connect -p quic -- example.com`

use core::net::SocketAddr;
use std::{net::ToSocketAddrs, time::Duration};

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
    async fn receive_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), IoError> {
        self.socket.recv_from(buf).await.map_err(IoError::from)
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
        .ok_or(format!("no IPv4 address for {server_name}"))?;
    println!("Connecting to {server_name} at {addr}...");

    let socket = TokioUdpSocket::bind("0.0.0.0:0").await?;
    let transport = UdpTransport {
        socket,
        epoch: std::time::Instant::now(),
    };

    let mut config = Config::default();
    config.alpn_protocols = vec![bytes::Bytes::from_static(b"h3")];

    let mut conn = quic::Connection::new(transport, config);
    match conn.connect(addr, &server_name).await {
        Ok(()) => println!("QUIC handshake successful!"),
        Err(e) => {
            eprintln!("Connection failed: {e}");
            return Ok(());
        }
    }

    Ok(())
}
