//! Tokio integration for the `tls` crate.
//!
//! Provides [`TokioStreamAdapter`] to bridge tokio's I/O traits into `tls::io`,
//! plus [`TlsConnector`], [`TlsAcceptor`], and [`TlsStream`] for full
//! tokio [`AsyncRead`] + [`AsyncWrite`] interop.
//!
//! # Client example
//!
//! ```no_run
//! use std::sync::Arc;
//! use tls::{ClientConfig, ReceivedCertificate};
//! use tls::config::CertificateValidator;
//! use tls::crypto_default_provider::DefaultCryptoProvider;
//! use tokio::net::TcpStream;
//! use tokio_tls::TlsConnector;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     struct TrustAll;
//!     #[async_trait::async_trait]
//!     impl CertificateValidator for TrustAll {
//!         async fn validate(&self, _: &ReceivedCertificate, _: Option<&str>) -> Result<(), tls::Error> {
//!             Ok(())
//!         }
//!     }
//!
//!     let provider = Arc::new(DefaultCryptoProvider::new());
//!     let config = ClientConfig::new(provider, vec![], Arc::new(TrustAll));
//!     let connector = TlsConnector::new(config);
//!
//!     let stream = TcpStream::connect("example.com:443").await?;
//!     let mut tls = connector.connect("example.com", stream).await?;
//!
//!     use tokio::io::AsyncWriteExt;
//!     tls.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n").await?;
//!     tls.flush().await?;
//!
//!     let mut response = Vec::new();
//!     tls.read_to_end(&mut response).await?;
//!     println!("{}", String::from_utf8_lossy(&response));
//!     Ok(())
//! }
//! ```
//!
//! # Server example
//!
//! ```no_run
//! use std::sync::Arc;
//! use tls::ServerConfig;
//! use tls::config::{CertificateProvider, ProvidedCertificate, ClientHello};
//! use tls::crypto_default_provider::DefaultCryptoProvider;
//! use tokio::net::TcpListener;
//! use tokio_tls::TlsAcceptor;
//!
//! struct MyProvider;
//! #[async_trait::async_trait]
//! impl CertificateProvider for MyProvider {
//!     async fn provide(&self, _: &ClientHello<'_>) -> Result<ProvidedCertificate, tls::Error> {
//!         todo!("load certificate")
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let provider = Arc::new(DefaultCryptoProvider::new());
//!     let config = ServerConfig::new(provider, vec![], Arc::new(MyProvider));
//!     let acceptor = TlsAcceptor::new(config);
//!
//!     let listener = TcpListener::bind("0.0.0.0:4433").await?;
//!     loop {
//!         let (stream, _) = listener.accept().await?;
//!         let mut tls = acceptor.accept(stream).await?;
//!         tokio::spawn(async move {
//!             use tokio::io::{AsyncReadExt, AsyncWriteExt};
//!             let mut buf = [0u8; 4096];
//!             let n = tls.read(&mut buf).await.unwrap();
//!             let _ = tls.write_all(&buf[..n]).await;
//!         });
//!     }
//! }
//! ```

use std::{
    io,
    pin::Pin,
    task::{Context, Poll, ready},
};

use bytes::Bytes;
use tls::{ClientConfig, ClientConnection, IoError, IoErrorKind, ServerConfig, ServerConnection};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

// ── Error helpers ───────────────────────────────────────────────────────────

fn io_kind_from_tokio(k: io::ErrorKind) -> IoErrorKind {
    use io::ErrorKind as K;
    match k {
        K::ConnectionReset => IoErrorKind::ConnectionReset,
        K::ConnectionAborted => IoErrorKind::ConnectionAborted,
        K::NotConnected => IoErrorKind::NotConnected,
        K::TimedOut => IoErrorKind::TimedOut,
        K::Interrupted => IoErrorKind::Interrupted,
        K::UnexpectedEof => IoErrorKind::UnexpectedEof,
        K::WriteZero => IoErrorKind::WriteZero,
        _ => IoErrorKind::Other,
    }
}

fn into_io_err(e: tls::Error) -> io::Error {
    match e {
        tls::Error::Io(ioe) => io::Error::new(io_kind_to_tokio(ioe.kind()), ioe.to_string()),
        other => io::Error::new(io::ErrorKind::Other, other.to_string()),
    }
}

fn io_kind_to_tokio(k: IoErrorKind) -> io::ErrorKind {
    match k {
        IoErrorKind::ConnectionReset => io::ErrorKind::ConnectionReset,
        IoErrorKind::ConnectionAborted => io::ErrorKind::ConnectionAborted,
        IoErrorKind::NotConnected => io::ErrorKind::NotConnected,
        IoErrorKind::TimedOut => io::ErrorKind::TimedOut,
        IoErrorKind::Interrupted => io::ErrorKind::Interrupted,
        IoErrorKind::UnexpectedEof => io::ErrorKind::UnexpectedEof,
        IoErrorKind::WriteZero => io::ErrorKind::WriteZero,
        IoErrorKind::Other => io::ErrorKind::Other,
    }
}

// ── TokioStreamAdapter ────────────────────────────────────────────────────────────

/// Wraps a tokio [`AsyncRead`] + [`AsyncWrite`] stream so it implements
/// `tls::io::AsyncRead` and `tls::io::AsyncWrite`.
///
/// Useful when you want to use [`ClientAsyncIo`](tls::io::ClientAsyncIo) or
/// [`ServerAsyncIo`](tls::io::ServerAsyncIo) directly instead of
/// [`TlsStream`].
pub struct TokioStreamAdapter<S>(pub S);

impl<S: tokio::io::AsyncRead + Unpin> tls::io::AsyncRead for TokioStreamAdapter<S> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        use tokio::io::AsyncReadExt;
        self.0
            .read(buf)
            .await
            .map_err(|e| IoError::new(io_kind_from_tokio(e.kind()), "read"))
    }
}

impl<S: tokio::io::AsyncWrite + Unpin> tls::io::AsyncWrite for TokioStreamAdapter<S> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        use tokio::io::AsyncWriteExt;
        self.0
            .write(buf)
            .await
            .map_err(|e| IoError::new(io_kind_from_tokio(e.kind()), "write"))
    }

    async fn flush(&mut self) -> Result<(), IoError> {
        use tokio::io::AsyncWriteExt;
        self.0
            .flush()
            .await
            .map_err(|e| IoError::new(io_kind_from_tokio(e.kind()), "flush"))
    }
}

// ── TlsState (internal dispatch) ────────────────────────────────────────────

enum TlsState {
    Client(ClientConnection),
    Server(ServerConnection),
}

impl TlsState {
    fn inject(&mut self, data: &[u8]) {
        match self {
            TlsState::Client(c) => c.inject(data),
            TlsState::Server(c) => c.inject(data),
        }
    }

    fn process_app_data(&mut self) -> Result<bool, tls::Error> {
        match self {
            TlsState::Client(c) => c.process_app_data(),
            TlsState::Server(c) => c.process_app_data(),
        }
    }

    fn read_app_data(&mut self) -> Option<Bytes> {
        match self {
            TlsState::Client(c) => c.read_app_data(),
            TlsState::Server(c) => c.read_app_data(),
        }
    }

    fn send(&mut self, data: &[u8]) -> Result<Bytes, tls::Error> {
        match self {
            TlsState::Client(c) => c.send(data),
            TlsState::Server(c) => c.send(data),
        }
    }

    fn close(&mut self) -> Result<Bytes, tls::Error> {
        match self {
            TlsState::Client(c) => c.close(),
            TlsState::Server(c) => c.close(),
        }
    }

    fn cipher_suite(&self) -> Option<tls::CipherSuite> {
        match self {
            TlsState::Client(c) => c.cipher_suite(),
            _ => None,
        }
    }

    fn kx_group(&self) -> Option<tls::KeyExchangeGroup> {
        match self {
            TlsState::Client(c) => Some(c.kx_group()),
            _ => None,
        }
    }

    fn alpn_protocol(&self) -> Option<&bytes::Bytes> {
        match self {
            TlsState::Client(c) => c.alpn_protocol(),
            TlsState::Server(c) => c.alpn_protocol(),
        }
    }

    fn server_name(&self) -> Option<&str> {
        match self {
            TlsState::Client(c) => c.server_name(),
            _ => None,
        }
    }

    fn negotiated_version(&self) -> u16 {
        match self {
            TlsState::Client(c) => c.negotiated_version(),
            TlsState::Server(c) => c.negotiated_version(),
        }
    }

    fn signature_scheme(&self) -> Option<tls::SignatureScheme> {
        match self {
            TlsState::Client(c) => c.signature_scheme(),
            _ => None,
        }
    }
}

// ── TlsStream ───────────────────────────────────────────────────────────────

/// A TLS stream implementing tokio's [`AsyncRead`] + [`AsyncWrite`].
///
/// Created by [`TlsConnector::connect`] or [`TlsAcceptor::accept`].
/// After construction the handshake is complete and the stream is ready
/// for application data.
pub struct TlsStream<S> {
    stream: S,
    state: TlsState,
    pending_write: Option<(usize, Bytes)>,
    pending_read: Option<Bytes>,
}

impl<S: AsyncRead + AsyncWrite + Unpin> TlsStream<S> {
    /// The negotiated cipher suite.
    pub fn cipher_suite(&self) -> Option<tls::CipherSuite> {
        self.state.cipher_suite()
    }

    /// The negotiated key exchange group.
    pub fn kx_group(&self) -> Option<tls::KeyExchangeGroup> {
        self.state.kx_group()
    }

    /// The selected ALPN protocol, if any.
    pub fn alpn_protocol(&self) -> Option<&bytes::Bytes> {
        self.state.alpn_protocol()
    }

    /// The server name (SNI) used for this connection.
    pub fn server_name(&self) -> Option<&str> {
        self.state.server_name()
    }

    /// The negotiated TLS protocol version (e.g. `0x0304` for TLS 1.3).
    pub fn negotiated_version(&self) -> u16 {
        self.state.negotiated_version()
    }

    /// Human-readable TLS version string.
    pub fn tls_version(&self) -> String {
        let v = self.negotiated_version();
        format!("TLS 1.{}/0x{:04x}", (v & 0xff).saturating_sub(1), v)
    }

    /// The signature scheme used by the server's CertificateVerify message,
    /// if available.
    pub fn signature_scheme(&self) -> Option<tls::SignatureScheme> {
        self.state.signature_scheme()
    }

    fn poll_flush_pending(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        loop {
            let (offset, data) = match &self.pending_write {
                Some((off, d)) => (*off, d.clone()),
                None => return Poll::Ready(Ok(())),
            };

            let n = ready!(Pin::new(&mut self.stream).poll_write(cx, &data[offset..]))?;
            if n == 0 {
                return Poll::Ready(Err(io::Error::new(io::ErrorKind::WriteZero, "pending tls write failed")));
            }
            let new_off = offset + n;
            if new_off >= data.len() {
                self.pending_write = None;
            } else {
                self.pending_write = Some((new_off, data));
                return Poll::Ready(Ok(()));
            }
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for TlsStream<S> {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        loop {
            if self.pending_write.is_some() {
                ready!(self.as_mut().poll_flush_pending(cx))?;
            }

            if let Some(data) = self.pending_read.take() {
                let len = data.len().min(buf.remaining());
                buf.put_slice(&data[..len]);
                if len < data.len() {
                    self.pending_read = Some(data.slice(len..));
                }
                return Poll::Ready(Ok(()));
            }

            if let Some(data) = self.state.read_app_data() {
                let len = data.len().min(buf.remaining());
                buf.put_slice(&data[..len]);
                if len < data.len() {
                    self.pending_read = Some(data.slice(len..));
                }
                return Poll::Ready(Ok(()));
            }

            let mut tmp = [0u8; 16384];
            let mut inner = ReadBuf::new(&mut tmp);
            ready!(Pin::new(&mut self.stream).poll_read(cx, &mut inner))?;
            let n = inner.filled().len();
            if n == 0 {
                return Poll::Ready(Ok(()));
            }

            self.state.inject(&tmp[..n]);
            match self.state.process_app_data() {
                Ok(_) => {}
                Err(tls::Error::ConnectionClosed) => {}
                Err(e) => return Poll::Ready(Err(into_io_err(e))),
            }
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for TlsStream<S> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        if self.pending_write.is_some() {
            ready!(self.as_mut().poll_flush_pending(cx))?;
        }

        let encrypted = self.state.send(buf).map_err(into_io_err)?;

        let len = encrypted.len();
        let n = ready!(Pin::new(&mut self.stream).poll_write(cx, &encrypted))?;
        if n == 0 {
            return Poll::Ready(Err(io::Error::new(io::ErrorKind::WriteZero, "tls write returned 0")));
        }
        if n < len {
            self.pending_write = Some((n, encrypted));
        }

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        if self.pending_write.is_some() {
            ready!(self.as_mut().poll_flush_pending(cx))?;
        }
        Pin::new(&mut self.stream).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        if self.pending_write.is_some() {
            ready!(self.as_mut().poll_flush_pending(cx))?;
        }

        let close_msg = self.state.close().map_err(into_io_err)?;
        let n = ready!(Pin::new(&mut self.stream).poll_write(cx, &close_msg))?;
        if n < close_msg.len() {
            self.pending_write = Some((n, close_msg));
            return Poll::Pending;
        }

        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}

// ── TlsConnector ────────────────────────────────────────────────────────────

/// Holds a [`ClientConfig`] and produces a [`TlsStream`] after the handshake.
pub struct TlsConnector {
    config: ClientConfig,
}

impl TlsConnector {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
        }
    }

    /// Connect to a TLS server and complete the handshake.
    ///
    /// The returned [`TlsStream`] implements [`AsyncRead`] + [`AsyncWrite`]
    /// and is ready for application data.
    pub async fn connect<S: AsyncRead + AsyncWrite + Unpin>(
        &self,
        server_name: &str,
        mut stream: S,
    ) -> Result<TlsStream<S>, tls::Error> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut conn = ClientConnection::new(self.config.clone(), Some(server_name.into()))?;

        while let Some(data) = conn.write_tls() {
            stream
                .write_all(&data)
                .await
                .map_err(|e| tls::Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "handshake")))?;
        }

        loop {
            while let Some(data) = conn.write_tls() {
                stream
                    .write_all(&data)
                    .await
                    .map_err(|e| tls::Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "handshake")))?;
                stream
                    .flush()
                    .await
                    .map_err(|e| tls::Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "handshake")))?;
            }
            if conn.handshake_done() {
                break;
            }
            let mut buf = [0u8; 16384];
            let n = stream
                .read(&mut buf)
                .await
                .map_err(|e| tls::Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "handshake")))?;
            if n == 0 {
                return Err(tls::Error::ConnectionClosed);
            }
            conn.inject(&buf[..n]);
            conn.process().await?;
        }

        Ok(TlsStream {
            stream,
            state: TlsState::Client(conn),
            pending_write: None,
            pending_read: None,
        })
    }
}

// ── TlsAcceptor ────────────────────────────────────────────────────────────

/// Holds a [`ServerConfig`] and produces a [`TlsStream`] after the handshake.
pub struct TlsAcceptor {
    config: ServerConfig,
}

impl TlsAcceptor {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
        }
    }

    /// Accept an incoming TLS handshake.
    ///
    /// The returned [`TlsStream`] implements [`AsyncRead`] + [`AsyncWrite`]
    /// and is ready for application data.
    pub async fn accept<S: AsyncRead + AsyncWrite + Unpin>(&self, mut stream: S) -> Result<TlsStream<S>, tls::Error> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut conn = ServerConnection::new(self.config.clone());

        loop {
            while let Some(data) = conn.write_tls() {
                stream
                    .write_all(&data)
                    .await
                    .map_err(|e| tls::Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "handshake")))?;
                stream
                    .flush()
                    .await
                    .map_err(|e| tls::Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "handshake")))?;
            }
            if conn.handshake_done() {
                break;
            }
            let mut buf = [0u8; 16384];
            let n = stream
                .read(&mut buf)
                .await
                .map_err(|e| tls::Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "handshake")))?;
            if n == 0 {
                return Err(tls::Error::ConnectionClosed);
            }
            conn.inject(&buf[..n]);
            conn.process().await?;
        }

        Ok(TlsStream {
            stream,
            state: TlsState::Server(conn),
            pending_write: None,
            pending_read: None,
        })
    }
}
