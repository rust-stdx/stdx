//! Example: connect to google.com:443 via TLS and display connection info.
//!
//! Requires the `webpki-validator` feature (enabled by default).
//!
//! Run with:
//!   cargo run --example google

use std::sync::Arc;

use tls::{
    ClientConfig, ClientConnection, WebPkiValidator, crypto_default_provider::DefaultCryptoProvider, io::ClientAsyncIo,
};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[tokio::main]
async fn main() -> Result<(), anyerr::Error> {
    let server_name = "google.com";

    println!("Connecting to {server_name}:443 ...");
    let stream = TcpStream::connect((server_name, 443)).await?;
    println!("TCP connected.\n");

    // The default provider prefers the post-quantum X25519MLKEM768 key exchange,
    // falling back to X25519 only if the server does not support it.
    let provider = Arc::new(DefaultCryptoProvider::new());
    let validator = Arc::new(WebPkiValidator::with_default_roots());
    let config = ClientConfig::new(provider, vec![], validator);

    let client = ClientConnection::new(config, Some(server_name.to_string()))?;

    let mut tls = ClientAsyncIo::new(client, stream.compat()).await?;

    tls.handshake().await?;
    println!("═══ TLS Connection Established ═══");
    println!("  Server:          {}", server_name);
    println!("  Cipher suite:    {:?}", tls.conn().cipher_suite().unwrap());
    println!("  Key exchange:    {:?}", tls.conn().kx_group());
    println!(
        "  ALPN negotiated: {:?}",
        tls.conn().alpn_protocol().map(|b| String::from_utf8_lossy(b))
    );
    println!("  SNI:             {:?}", tls.conn().server_name());
    println!();

    let request = b"GET / HTTP/1.1\r\nHost: google.com\r\nConnection: close\r\n\r\n";
    tls.write(request).await?;
    println!("Sent HTTP request.");

    let mut response = Vec::new();
    loop {
        match tls.read().await {
            Ok(data) => response.extend_from_slice(&data),
            Err(tls::Error::ConnectionClosed) => break,
            Err(e) => return Err(e.into()),
        }
    }

    let resp_str = String::from_utf8_lossy(&response);
    let headers_end = resp_str.find("\r\n\r\n").unwrap_or(resp_str.len());
    println!("\n═══ HTTP Response ═══");
    println!("{}", &resp_str[..headers_end.min(1024)]);

    tls.close().await?;
    println!("\nConnection closed.");

    Ok(())
}
