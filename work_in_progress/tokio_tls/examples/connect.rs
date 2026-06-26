//! Connect to a TLS server and display connection info.
//!
//! Usage:
//!   cargo run --example connect -- example.com

use std::sync::Arc;

use tls::{ClientConfig, WebPkiValidator, crypto_default_provider::DefaultCryptoProvider};
use tokio::net::TcpStream;
use tokio_tls::TlsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_name = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: connect <hostname>");
        eprintln!();
        eprintln!("Connects to the given host on port 443 via TLS 1.3 and prints");
        eprintln!("connection information along with the HTTP response headers.");
        std::process::exit(1);
    });

    println!("Connecting to {server_name}:443 ...");
    let stream = TcpStream::connect((&*server_name, 443)).await?;
    println!("TCP connected.\n");

    let provider = Arc::new(DefaultCryptoProvider::new());
    let validator = Arc::new(WebPkiValidator::with_default_roots());
    let config = ClientConfig::new(provider, vec![], validator);

    let connector = TlsConnector::new(config);
    let mut tls = connector.connect(&server_name, stream).await?;

    println!("═══ TLS Connection Established ═══");
    println!("  Server:          {server_name}");
    println!("  TLS version:     {}", tls.tls_version());
    println!("  Cipher suite:    {:?}", tls.cipher_suite().unwrap());
    println!("  Key exchange:    {:?}", tls.kx_group().unwrap());
    println!("  SNI:             {:?}", tls.server_name());
    println!();

    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let request = format!("GET / HTTP/1.1\r\nHost: {server_name}\r\nConnection: close\r\n\r\n");
    tls.write_all(request.as_bytes()).await?;
    tls.flush().await?;
    println!("Sent HTTP request.\n");

    let mut response = Vec::new();
    tls.read_to_end(&mut response).await?;
    println!("Received {} bytes.\n", response.len());

    let resp_str = String::from_utf8_lossy(&response);
    let headers_end = resp_str.find("\r\n\r\n").unwrap_or(resp_str.len());
    println!("═══ HTTP Response (headers) ═══");
    println!("{}", &resp_str[..headers_end.min(2048)]);

    tls.shutdown().await?;
    println!("\nConnection closed.");

    Ok(())
}
