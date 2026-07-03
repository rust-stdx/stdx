use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tls2::{CipherSuite, Client, ClientApplicationDataEvent, ClientConfig, ClientHandshakeEvent, CryptoProvider, KeyExchangeGroup, MAX_RECORD_SIZE, SignatureScheme};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// Tokio-based TLS stream wrapping the sans-IO `Client`.
pub struct TlsStream<S, P: CryptoProvider + Unpin> {
    stream: S,
    client: Client<'static, P>,
    #[allow(dead_code)]
    read_buf: Vec<u8>,
    #[allow(dead_code)]
    write_buf: Vec<u8>,
    decrypted: Vec<u8>,
    /// Pending KeyUpdate response to flush before reading more.
    key_update: heapless::Vec<u8, 256>,
}

impl<S, P: CryptoProvider + Unpin> TlsStream<S, P> {
    pub fn new(config: ClientConfig<P>, stream: S) -> Self {
        let mut read_buf = vec![0u8; MAX_RECORD_SIZE];
        let mut write_buf = vec![0u8; MAX_RECORD_SIZE];

        let ptr_r = read_buf.as_mut_ptr();
        let ptr_w = write_buf.as_mut_ptr();

        // SAFETY: Vec allocations are heap-stable. The allocation outlives
        // Self because read_buf/write_buf are fields of Self.
        let client = unsafe {
            Client::new(
                config,
                std::slice::from_raw_parts_mut(ptr_r, MAX_RECORD_SIZE),
                std::slice::from_raw_parts_mut(ptr_w, MAX_RECORD_SIZE),
            )
        };

        Self {
            stream,
            client,
            read_buf,
            write_buf,
            decrypted: Vec::new(),
            key_update: heapless::Vec::new(),
        }
    }

    /// Run the full TLS 1.3 handshake.
    ///
    /// Returns the [`Done`](ClientHandshakeEvent::Done) event containing the
    /// negotiated parameters on success.
    pub async fn handshake(
        &mut self,
        server_name: Option<&str>,
        alpn_protocols: &[&[u8]],
    ) -> Result<HandshakeData, tls2::Error>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        use tokio::io::AsyncReadExt;
        let mut event = self.client.start_handshake(server_name, alpn_protocols)?;
        loop {
            match event {
                ClientHandshakeEvent::Send => {
                    tokio::io::AsyncWriteExt::write_all(&mut self.stream, self.client.outgoing_hadnshake_data())
                        .await
                        .map_err(|_| tls2::Error::ConnectionClosed)?;
                }
                ClientHandshakeEvent::Receive => {
                    let n = self
                        .stream
                        .read(self.client.receive_buffer())
                        .await
                        .map_err(|_| tls2::Error::ConnectionClosed)?;
                    if n == 0 {
                        return Err(tls2::Error::ConnectionClosed);
                    }
                    self.client.commit_received(n);
                }
                ClientHandshakeEvent::Done { ciphersuite, tls_version, key_exchange_group, signature_scheme  } => return Ok(HandshakeData { ciphersuite, tls_version, key_exchange_group, signature_scheme }),
                ClientHandshakeEvent::Closed => return Err(tls2::Error::ConnectionClosed),
            }
            event = self.client.continue_handshake()?;
        }
    }
}

// ── AsyncRead ─────────────────────────────────────────────────────────────

impl<S: AsyncRead + AsyncWrite + Unpin, P: CryptoProvider + Unpin> AsyncRead for TlsStream<S, P> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        loop {
            if !this.decrypted.is_empty() {
                let n = this.decrypted.len().min(buf.remaining());
                buf.put_slice(&this.decrypted[..n]);
                this.decrypted.drain(..n);
                return Poll::Ready(Ok(()));
            }

            // 2. Flush any pending KeyUpdate response.
            if !this.key_update.is_empty() {
                let n = match Pin::new(&mut this.stream).poll_write(cx, &this.key_update) {
                    Poll::Ready(Ok(n)) => n,
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                    Poll::Pending => return Poll::Pending,
                };
                if n < this.key_update.len() {
                    let tail = &this.key_update[n..];
                    let mut shifted = heapless::Vec::new();
                    let _ = shifted.extend_from_slice(tail);
                    this.key_update = shifted;
                    return Poll::Pending;
                }
                this.key_update.clear();
            }

            // 3. Try to decrypt any data already in the receive buffer.
            match this.client.decrypt() {
                Ok(ClientApplicationDataEvent::AppData) => {
                    let data = this.client.received_app_data();
                    let n = data.len().min(buf.remaining());
                    buf.put_slice(&data[..n]);
                    if n < data.len() {
                        this.decrypted.extend_from_slice(&data[n..]);
                    }
                    return Poll::Ready(Ok(()));
                }
                Ok(ClientApplicationDataEvent::Ticket { .. }) => continue,
                Ok(ClientApplicationDataEvent::KeyUpdate) => {
                    let _ = this.key_update.extend_from_slice(this.client.outgoing_hadnshake_data());
                    continue;
                }
                Ok(ClientApplicationDataEvent::None) => {}
                Err(tls2::Error::ConnectionClosed) => return Poll::Ready(Ok(())),
                Err(e) => return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{e:?}")))),
            }

            // 4. No complete record in buffer — read more from the network.
            let recv_buf = this.client.receive_buffer();
            let mut rb = ReadBuf::new(recv_buf);
            match Pin::new(&mut this.stream).poll_read(cx, &mut rb) {
                Poll::Ready(Ok(())) => {
                    let n = rb.filled().len();
                    if n == 0 {
                        return Poll::Ready(Ok(()));
                    }
                    this.client.commit_received(n);
                }
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            }

            // 5. Decrypt the newly read data.
            match this.client.decrypt() {
                Ok(ClientApplicationDataEvent::AppData) => {
                    let data = this.client.received_app_data();
                    let n = data.len().min(buf.remaining());
                    buf.put_slice(&data[..n]);
                    if n < data.len() {
                        this.decrypted.extend_from_slice(&data[n..]);
                    }
                    return Poll::Ready(Ok(()));
                }
                Ok(ClientApplicationDataEvent::Ticket { .. }) => continue,
                Ok(ClientApplicationDataEvent::KeyUpdate) => {
                    let _ = this.key_update.extend_from_slice(this.client.outgoing_hadnshake_data());
                    continue;
                }
                Ok(ClientApplicationDataEvent::None) => continue,
                Err(tls2::Error::ConnectionClosed) => return Poll::Ready(Ok(())),
                Err(e) => return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{e:?}")))),
            }
        }
    }
}

// ── AsyncWrite ────────────────────────────────────────────────────────────

impl<S: AsyncRead + AsyncWrite + Unpin, P: CryptoProvider + Unpin> AsyncWrite for TlsStream<S, P> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let chunk = &buf[..buf.len().min(16384)];
        match this.client.encrypt(chunk) {
            Ok(()) => {
                let out = this.client.outgoing_hadnshake_data().to_vec();
                match Pin::new(&mut this.stream).poll_write(cx, &out) {
                    Poll::Ready(Ok(_)) => Poll::Ready(Ok(chunk.len())),
                    Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
                    Poll::Pending => Poll::Pending,
                }
            }
            Err(e) => Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{e:?}")))),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        match this.client.close() {
            Ok(()) => {
                let out = this.client.outgoing_hadnshake_data().to_vec();
                let _ = Pin::new(&mut this.stream).poll_write(cx, &out);
                Pin::new(&mut this.stream).poll_shutdown(cx)
            }
            Err(_) => Pin::new(&mut this.stream).poll_shutdown(cx),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HandshakeData {
    ciphersuite: CipherSuite,
    /// Wire-encoded protocol version (`0x0304` for TLS 1.3).
    tls_version: u16,
    key_exchange_group: KeyExchangeGroup,
    /// The signature scheme used by the server's CertificateVerify.
    signature_scheme: SignatureScheme,
}

// ── Main example ──────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_name = std::env::args().nth(1).unwrap_or("example.com".to_string());

    let stream = tokio::net::TcpStream::connect((&*server_name, 443)).await?;
    let provider = tls2::crypto_default_provider::DefaultCryptoProvider::new();
    let config = ClientConfig::new(provider);
    let mut tls = TlsStream::new(config, stream);

    let handshake_data = tls.handshake(Some(&server_name), &[b"http/1.1"]).await?;

    println!("═══ TLS Connection Established ═══");
    println!("  Server:          {server_name}");
    println!("  TLS version:     0x{:04x}", handshake_data.tls_version);
    println!("  Cipher suite:    {:?}", handshake_data.ciphersuite);
    println!("  Key exchange:    {:?}", handshake_data.key_exchange_group);
    println!("  Signature scheme: {:?}", handshake_data.signature_scheme);
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
