use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, ready},
};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, ReadBuf};

use crate::{
    ALPN_PROTOCOL_MAX_SIZE, CertificateVerifier, CipherSuite, Client, ClientApplicationDataEvent, ClientConfig,
    ClientHandshakeEvent, CryptoProvider, Error, KeyExchangeGroup, MAX_RECORD_SIZE, SignatureScheme,
};

/// Tokio-based TLS stream wrapping the sans-IO `Client`.
pub struct TlsClient<S, C>
where
    S: AsyncRead + AsyncWrite + Unpin,
    C: CryptoProvider + Unpin,
{
    stream: S,
    client: Client<Vec<u8>, C>,
    certificate_verifier: Arc<dyn CertificateVerifier>,
    close_notify_sent: bool,
}

impl<S: AsyncRead + AsyncWrite + Unpin, P: CryptoProvider + Unpin> TlsClient<S, P> {
    pub fn new(config: ClientConfig<P>, certificate_verifier: Arc<dyn CertificateVerifier>, stream: S) -> Self {
        let client = Client::new(config, vec![0u8; MAX_RECORD_SIZE], vec![0u8; MAX_RECORD_SIZE]);

        Self {
            stream,
            client,
            certificate_verifier,
            close_notify_sent: false,
        }
    }

    /// Run the full TLS 1.3 handshake.
    pub async fn handshake(
        &mut self,
        server_name: Option<&str>,
        alpn_protocols: &[&[u8]],
    ) -> Result<HandshakeData, Error> {
        let mut event = self.client.start_handshake(server_name, alpn_protocols)?;
        loop {
            match event {
                ClientHandshakeEvent::Send => {
                    tokio::io::AsyncWriteExt::write_all(&mut self.stream, self.client.outgoing_data())
                        .await
                        .map_err(|_| Error::ConnectionClosed)?;
                }
                ClientHandshakeEvent::Receive => {
                    let n = self
                        .stream
                        .read(self.client.receive_buffer())
                        .await
                        .map_err(|_| Error::ConnectionClosed)?;
                    if n == 0 {
                        return Err(Error::ConnectionClosed);
                    }
                    self.client.commit_received(n);
                }
                ClientHandshakeEvent::VerifyServerCertificate => {
                    {
                        let (cert, server_name) =
                            self.client.server_certificate().ok_or(Error::CertificateParseFailed)?;
                        self.certificate_verifier.verify_certificate(&cert, server_name).await?;
                    }
                    self.client.accept_certificate(Ok(()));
                }
                ClientHandshakeEvent::Done {
                    ciphersuite,
                    tls_version,
                    key_exchange_group,
                    signature_scheme,
                    alpn,
                } => {
                    return Ok(HandshakeData {
                        ciphersuite,
                        tls_version,
                        key_exchange_group,
                        signature_scheme,
                        alpn: alpn.try_into().unwrap(),
                    });
                }
                ClientHandshakeEvent::Closed => return Err(Error::ConnectionClosed),
            }
            event = self.client.continue_handshake()?;
        }
    }
}

// ── AsyncRead ─────────────────────────────────────────────────────────────

impl<S: AsyncRead + AsyncWrite + Unpin, C: CryptoProvider + Unpin> AsyncRead for TlsClient<S, C> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        loop {
            // 1. Flush any pending KeyUpdate response before reading more.
            if !this.client.outgoing_key_update_data().is_empty() {
                let resp = this.client.outgoing_key_update_data();
                match Pin::new(&mut this.stream).poll_write(cx, resp) {
                    Poll::Ready(Ok(n)) => {
                        this.client.commit_key_update_data(n);
                        if !this.client.outgoing_key_update_data().is_empty() {
                            return Poll::Pending;
                        }
                    }
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                }
            }

            // 2. Try to decrypt any data already in the receive buffer.
            match this.client.decrypt() {
                Ok(ClientApplicationDataEvent::AppData) => {
                    let data = this.client.received_app_data();
                    let n = data.len().min(buf.remaining());
                    buf.put_slice(&data[..n]);
                    this.client.commit_app_data(n);
                    return Poll::Ready(Ok(()));
                }
                Ok(ClientApplicationDataEvent::Ticket {
                    ..
                }) => continue,
                Ok(ClientApplicationDataEvent::KeyUpdate) => continue,
                Ok(ClientApplicationDataEvent::None) => {}
                Err(Error::ConnectionClosed) => return Poll::Ready(Ok(())),
                Err(e) => return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{e:?}")))),
            }

            // 3. No complete record in buffer — read more from the network.
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

            // 4. Decrypt the newly read data.
            match this.client.decrypt() {
                Ok(ClientApplicationDataEvent::AppData) => {
                    let data = this.client.received_app_data();
                    let n = data.len().min(buf.remaining());
                    buf.put_slice(&data[..n]);
                    this.client.commit_app_data(n);
                    return Poll::Ready(Ok(()));
                }
                Ok(ClientApplicationDataEvent::Ticket {
                    ..
                }) => continue,
                Ok(ClientApplicationDataEvent::KeyUpdate) => continue,
                Ok(ClientApplicationDataEvent::None) => continue,
                Err(Error::ConnectionClosed) => return Poll::Ready(Ok(())),
                Err(e) => return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{e:?}")))),
            }
        }
    }
}

// ── AsyncWrite ────────────────────────────────────────────────────────────

impl<S: AsyncRead + AsyncWrite + Unpin, C: CryptoProvider + Unpin> AsyncWrite for TlsClient<S, C> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();

        // 0. Flush any pending KeyUpdate response before encrypting new data.
        if !this.client.outgoing_key_update_data().is_empty() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, this.client.outgoing_key_update_data()))?;
            this.client.commit_key_update_data(n);
            if !this.client.outgoing_key_update_data().is_empty() {
                return Poll::Pending;
            }
        }

        // 1. Flush any buffered encrypted data first.
        if !this.client.outgoing_data().is_empty() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, this.client.outgoing_data()))?;
            this.client.commit_sent(n);
            if !this.client.outgoing_data().is_empty() {
                return Poll::Pending;
            }
        }

        // 2. Encrypt new plaintext.
        let n = match this.client.encrypt(buf) {
            Ok(n) => n,
            Err(e) => return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{e:?}")))),
        };

        // 3. Try to send the encrypted record.
        match Pin::new(&mut this.stream).poll_write(cx, this.client.outgoing_data()) {
            Poll::Ready(Ok(m)) => {
                this.client.commit_sent(m);
                Poll::Ready(Ok(n))
            }
            Poll::Pending => Poll::Ready(Ok(n)),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();

        // Flush any pending KeyUpdate response first.
        if !this.client.outgoing_key_update_data().is_empty() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, this.client.outgoing_key_update_data()))?;
            this.client.commit_key_update_data(n);
            if !this.client.outgoing_key_update_data().is_empty() {
                return Poll::Pending;
            }
        }

        // Then flush any buffered outgoing data
        if !this.client.outgoing_data().is_empty() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, this.client.outgoing_data()))?;
            this.client.commit_sent(n);
            if !this.client.outgoing_data().is_empty() {
                return Poll::Pending;
            }
        }

        Pin::new(&mut this.stream).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();

        // 1. Flush any buffered data.
        if !this.client.outgoing_data().is_empty() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, this.client.outgoing_data()))?;
            this.client.commit_sent(n);
            if !this.client.outgoing_data().is_empty() {
                return Poll::Pending;
            }
        }

        // 2. Send close_notify (only once).
        if !this.close_notify_sent {
            this.close_notify_sent = true;
            match this.client.close() {
                Ok(data) => {
                    let n = ready!(Pin::new(&mut this.stream).poll_write(cx, data))?;
                    if n < data.len() {
                        this.client.commit_sent(n);
                        return Poll::Pending;
                    }
                }
                Err(_) => {}
            }
        }

        // 3. Flush any remaining close_notify bytes.
        if !this.client.outgoing_data().is_empty() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, this.client.outgoing_data()))?;
            this.client.commit_sent(n);
            if !this.client.outgoing_data().is_empty() {
                return Poll::Pending;
            }
        }

        Pin::new(&mut this.stream).poll_shutdown(cx)
    }
}

/// The settings negotiated during the handshake
#[derive(Clone, Debug)]
pub struct HandshakeData {
    ciphersuite: CipherSuite,
    tls_version: u16,
    key_exchange_group: KeyExchangeGroup,
    signature_scheme: SignatureScheme,
    alpn: heapless::Vec<u8, ALPN_PROTOCOL_MAX_SIZE>,
}

impl HandshakeData {
    #[inline]
    pub fn ciphersuite(&self) -> CipherSuite {
        return self.ciphersuite;
    }

    /// Wire-encoded protocol version (`0x0304` for TLS 1.3).
    #[inline]
    pub fn tls_version(&self) -> u16 {
        self.tls_version
    }

    #[inline]
    pub fn key_exchange_group(&self) -> KeyExchangeGroup {
        self.key_exchange_group
    }

    /// The signature scheme used by the server's CertificateVerify.
    #[inline]
    pub fn signature_scheme(&self) -> SignatureScheme {
        self.signature_scheme
    }

    #[inline]
    pub fn alpn(&self) -> &[u8] {
        &self.alpn
    }
}
