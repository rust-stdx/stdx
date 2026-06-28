//! Connect to a TLS server using the `tls` crate's blocking std integration.
//!
//! Usage:
//!   cargo run --example connect -- example.com

use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use tls::{ClientConfig, WebPkiValidator, crypto_default_provider::DefaultCryptoProvider, io_std::TlsConnector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_name = std::env::args().nth(1).unwrap_or("example.com".to_string());

    println!("Connecting to {server_name}:443 ...");
    let stream = TcpStream::connect((&*server_name, 443))?;
    println!("TCP connected.\n");

    let provider = Arc::new(DefaultCryptoProvider::new());
    let validator = Arc::new(WebPkiValidator::with_default_roots(provider.clone()));
    let config = ClientConfig::new(provider, vec![], validator);

    let connector = TlsConnector::new(config);
    let mut tls = connector.connect(&server_name, stream)?;

    println!("═══ TLS Connection Established ═══");
    println!("  Server:          {server_name}");
    println!("  TLS version:     {}", tls.tls_version());
    println!("  Cipher suite:    {:?}", tls.cipher_suite().unwrap());
    println!("  Key exchange:    {:?}", tls.key_exchange_group().unwrap());
    println!("  Signature scheme: {:?}", tls.signature_scheme().unwrap());
    println!("  SNI:             {:?}", tls.server_name());
    println!();

    let request = format!("GET / HTTP/1.1\r\nHost: {server_name}\r\nConnection: close\r\n\r\n");
    tls.write_all(request.as_bytes())?;
    tls.flush()?;
    println!("Sent HTTP request.\n");

    let mut response = Vec::new();
    tls.read_to_end(&mut response)?;
    println!("Received {} bytes.\n", response.len());

    let resp_str = String::from_utf8_lossy(&response);
    let headers_end = resp_str.find("\r\n\r\n").unwrap_or(resp_str.len());
    println!("═══ HTTP Response (headers) ═══");
    println!("{}", &resp_str[..headers_end.min(2048)]);

    // close via drop
    println!("\nConnection closed.");

    Ok(())
}
