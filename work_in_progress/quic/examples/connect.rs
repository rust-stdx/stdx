//! Example: connect to a server via QUIC.
//!
//! Run: `cargo run --example connect -p quic -- example.com`

use std::net::{SocketAddr, ToSocketAddrs};

use quic::{Config, Transport};
use tokio::net::UdpSocket as TokioUdpSocket;

struct UdpTransport(TokioUdpSocket);

#[async_trait::async_trait]
impl Transport for UdpTransport {
    async fn send_to(&self, dest: SocketAddr, data: &[u8]) -> std::io::Result<usize> {
        self.0.send_to(data, dest).await
    }
    async fn recv_from(&self, buf: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        self.0.recv_from(buf).await
    }
    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.0.local_addr()
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
    let transport = UdpTransport(socket);

    let mut config = Config::default();
    config.alpn_protocols = vec![bytes::Bytes::from_static(b"h3")];

    let mut conn = quic::Connection::new(transport, config);
    match conn.connect(addr, &server_name).await {
        Ok(()) => println!("QUIC handshake successful!"),
        Err(e) => eprintln!("Connection failed: {e}"),
    }

    Ok(())
}
