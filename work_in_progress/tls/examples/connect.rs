//! Connect to a TLS server using the low-level `tls` primitives.
//!
//! Usage:
//!   cargo run --example connect -- example.com

use std::sync::Arc;

use tls::{ClientConfig, WebPkiValidator, crypto_default_provider::DefaultCryptoProvider, io::ClientAsyncIo};
use tokio::net::TcpStream;
use tokio_tls::TokioStreamAdapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_name = std::env::args().nth(1).unwrap_or("example.com".to_string());

    println!("Connecting to {server_name}:443 ...");
    let stream = TcpStream::connect((&*server_name, 443)).await?;
    println!("TCP connected.\n");

    let provider = Arc::new(DefaultCryptoProvider::new());
    let validator = Arc::new(WebPkiValidator::with_default_roots());
    let config = ClientConfig::new(provider, vec![], validator);

    let client = tls::ClientConnection::new(config, Some(server_name.clone()))?;
    let mut tls = ClientAsyncIo::new(client, TokioStreamAdapter(stream)).await?;
    tls.handshake().await?;

    println!("═══ TLS Connection Established ═══");
    println!("  Server:          {server_name}");
    println!(
        "  TLS version:     TLS 1.{}",
        (tls.conn().negotiated_version() & 0xff).saturating_sub(1)
    );
    println!("  Cipher suite:    {:?}", tls.conn().cipher_suite().unwrap());
    println!("  Key exchange:    {:?}", tls.conn().kx_group());
    println!("  Signature scheme: {:?}", tls.conn().signature_scheme().unwrap());
    println!("  SNI:             {:?}", tls.conn().server_name());
    println!();

    let request = format!("GET / HTTP/1.1\r\nHost: {server_name}\r\nConnection: close\r\n\r\n");
    tls.write(request.as_bytes()).await?;
    println!("Sent HTTP request.\n");

    let mut response = Vec::new();
    loop {
        match tls.read().await {
            Ok(data) => response.extend_from_slice(&data),
            Err(tls::Error::ConnectionClosed) => break,
            Err(e) => return Err(e.into()),
        }
    }
    println!("Received {} bytes.\n", response.len());

    let resp_str = String::from_utf8_lossy(&response);
    let headers_end = resp_str.find("\r\n\r\n").unwrap_or(resp_str.len());
    println!("═══ HTTP Response (headers) ═══");
    println!("{}", &resp_str[..headers_end.min(2048)]);

    tls.close().await?;
    println!("\nConnection closed.");

    Ok(())
}
