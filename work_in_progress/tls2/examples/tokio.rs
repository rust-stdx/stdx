use std::sync::Arc;

use tls2::{
    ClientConfig, DefaultCertificateVerifier, crypto_default_provider::DefaultCryptoProvider, tokio::TlsClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_name = std::env::args().nth(1).unwrap_or("example.com".to_string());

    let stream = tokio::net::TcpStream::connect((&*server_name, 443)).await?;
    let crypto_provider = DefaultCryptoProvider;
    let verifier = Arc::new(DefaultCertificateVerifier::new(crypto_provider.clone()).with_system_roots());
    let config = ClientConfig::new(crypto_provider);
    let mut tls = TlsClient::new(config, verifier, stream);

    let handshake_data = tls.handshake(Some(&server_name), &[b"http/1.1"]).await?;

    println!("═══ TLS Connection Established ═══");
    println!("  Server:           {server_name}");
    println!("  TLS version:      0x{:04x}", handshake_data.tls_version());
    println!("  Cipher suite:     {:?}", handshake_data.ciphersuite());
    println!("  Key exchange:     {:?}", handshake_data.key_exchange_group());
    println!("  Signature scheme: {:?}", handshake_data.signature_scheme());
    println!("  ALPN:             {:?}", String::from_utf8_lossy(&handshake_data.alpn()));
    println!();

    use tokio::io::AsyncWriteExt;
    let request = format!("GET / HTTP/1.1\r\nHost: {server_name}\r\nConnection: close\r\n\r\n");
    tls.write_all(request.as_bytes()).await?;
    tls.flush().await?;

    let mut response = Vec::new();
    use tokio::io::AsyncReadExt;
    tls.read_to_end(&mut response).await?;

    let resp_str = String::from_utf8_lossy(&response);
    let headers_end = resp_str.find("\r\n\r\n").unwrap_or(resp_str.len());
    println!("═══ HTTP Response (headers) ═══");
    println!("{}", &resp_str[..headers_end.min(2048)]);
    println!("\nConnection closed ({} bytes received).", response.len());
    Ok(())
}
