use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker, ready},
};

use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::{
    ClientConfig, Error, IoError, IoErrorKind, ServerConfig,
    connection::{ClientConnection, ServerConnection, TlsState},
    crypto::{CipherSuite, KeyExchangeGroup, SignatureScheme},
};

// ── Error helpers ───────────────────────────────────────────────────────

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

fn into_tls_err(e: io::Error) -> Error {
    Error::Io(IoError::new(io_kind_from_tokio(e.kind()), "io_tokio"))
}

fn into_io_err(e: Error) -> io::Error {
    match e {
        Error::Io(ioe) => io::Error::new(io_kind_to_tokio(ioe.kind()), ioe.to_string()),
        other => io::Error::new(io::ErrorKind::Other, other.to_string()),
    }
}

// ── Minimal block_on (for use inside poll-based trait impls) ─────────────

fn block_on<F: Future>(f: F) -> F::Output {
    let mut fut = Box::pin(f);
    static VTABLE: RawWakerVTable = RawWakerVTable::new(|p| RawWaker::new(p, &VTABLE), |_| {}, |_| {}, |_| {});
    let waker = unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => core::hint::spin_loop(),
        }
    }
}

// ── Handshake helpers ───────────────────────────────────────────────────

async fn client_handshake<S: AsyncRead + AsyncWrite + Unpin>(
    mut conn: ClientConnection,
    stream: &mut S,
) -> Result<TlsState, Error> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    while let Some(data) = conn.write_tls() {
        stream.write_all(&data).await.map_err(into_tls_err)?;
    }
    loop {
        while let Some(data) = conn.write_tls() {
            stream.write_all(&data).await.map_err(into_tls_err)?;
            stream.flush().await.map_err(into_tls_err)?;
        }
        if conn.handshake_done() {
            return Ok(TlsState::Client(conn));
        }
        let mut buf = [0u8; 16384];
        let n = stream.read(&mut buf).await.map_err(into_tls_err)?;
        if n == 0 {
            return Err(Error::ConnectionClosed);
        }
        conn.inject(&buf[..n]);
        conn.process().await?;
    }
}

async fn server_handshake<S: AsyncRead + AsyncWrite + Unpin>(
    mut conn: ServerConnection,
    stream: &mut S,
) -> Result<TlsState, Error> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    loop {
        while let Some(data) = conn.write_tls() {
            stream.write_all(&data).await.map_err(into_tls_err)?;
            stream.flush().await.map_err(into_tls_err)?;
        }
        if conn.handshake_done() {
            return Ok(TlsState::Server(conn));
        }
        let mut buf = [0u8; 16384];
        let n = stream.read(&mut buf).await.map_err(into_tls_err)?;
        if n == 0 {
            return Err(Error::ConnectionClosed);
        }
        conn.inject(&buf[..n]);
        conn.process().await?;
    }
}

// ── TlsStream ───────────────────────────────────────────────────────────

/// A TLS stream implementing tokio's [`AsyncRead`] + [`AsyncWrite`].
///
/// Created by [`TlsConnector::connect`] or [`TlsAcceptor::accept`].
/// After construction the handshake is complete and the stream is ready
/// for application data.
pub struct TlsStream<S: AsyncRead + AsyncWrite + Unpin> {
    stream: S,
    state: TlsState,
    pending_write: Option<(usize, Bytes)>,
    pending_read: Option<Bytes>,
}

impl<S: AsyncRead + AsyncWrite + Unpin> TlsStream<S> {
    pub fn cipher_suite(&self) -> Option<CipherSuite> {
        self.state.cipher_suite()
    }

    pub fn key_exchange_group(&self) -> Option<KeyExchangeGroup> {
        self.state.key_exchange_group()
    }

    pub fn alpn_protocol(&self) -> Option<&[u8]> {
        self.state.alpn_protocol()
    }

    pub fn server_name(&self) -> Option<&str> {
        self.state.server_name()
    }

    pub fn negotiated_version(&self) -> u16 {
        self.state.negotiated_version()
    }

    pub fn tls_version(&self) -> String {
        let v = self.negotiated_version();
        format!("TLS 1.{}/0x{:04x}", (v & 0xff).saturating_sub(1), v)
    }

    pub fn signature_scheme(&self) -> Option<SignatureScheme> {
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

            if self.state.close_notified() {
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
            match block_on(self.state.process_app_data()) {
                Ok(_) => {}
                Err(Error::ConnectionClosed) => {}
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

        let encrypted = self.state.encrypt_application_data(buf).map_err(into_io_err)?;

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

// ── TlsConnector ────────────────────────────────────────────────────────

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
    ) -> Result<TlsStream<S>, Error> {
        let conn = ClientConnection::new(self.config.clone(), Some(server_name)).await?;
        let state = client_handshake(conn, &mut stream).await?;
        Ok(TlsStream {
            stream,
            state,
            pending_write: None,
            pending_read: None,
        })
    }
}

// ── TlsAcceptor ─────────────────────────────────────────────────────────

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
    pub async fn accept<S: AsyncRead + AsyncWrite + Unpin>(&self, mut stream: S) -> Result<TlsStream<S>, Error> {
        let conn = ServerConnection::new(self.config.clone());
        let state = server_handshake(conn, &mut stream).await?;
        Ok(TlsStream {
            stream,
            state,
            pending_write: None,
            pending_read: None,
        })
    }
}
