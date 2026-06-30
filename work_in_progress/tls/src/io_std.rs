use std::{
    future::Future,
    io::{self, Read, Write},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use bytes::Bytes;

use crate::{
    ClientConfig, Error, IoError, IoErrorKind, ServerConfig,
    connection::{ClientConnection, ServerConnection, TlsState},
    crypto::{CipherSuite, KeyExchangeGroup, SignatureScheme},
};

// ── Minimal block_on ────────────────────────────────────────────────────

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

// ── Error helpers ───────────────────────────────────────────────────────

fn io_kind_from_std(k: io::ErrorKind) -> IoErrorKind {
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

fn io_kind_to_std(k: IoErrorKind) -> io::ErrorKind {
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
    Error::Io(IoError::new(io_kind_from_std(e.kind()), "io_std"))
}

fn into_io_err(e: Error) -> io::Error {
    match e {
        Error::Io(ioe) => io::Error::new(io_kind_to_std(ioe.kind()), ioe.to_string()),
        other => io::Error::new(io::ErrorKind::Other, other.to_string()),
    }
}

// ── Handshake helpers ───────────────────────────────────────────────────

fn client_handshake<S: Read + Write>(mut conn: ClientConnection, stream: &mut S) -> Result<TlsState, Error> {
    while let Some(data) = conn.write_tls() {
        stream.write_all(&data).map_err(into_tls_err)?;
    }
    loop {
        while let Some(data) = conn.write_tls() {
            stream.write_all(&data).map_err(into_tls_err)?;
        }
        if conn.handshake_done() {
            return Ok(TlsState::Client(conn));
        }
        let mut buf = [0u8; 16384];
        let n = stream.read(&mut buf).map_err(into_tls_err)?;
        if n == 0 {
            return Err(Error::ConnectionClosed);
        }
        conn.inject(&buf[..n]);
        block_on(conn.process())?;
    }
}

fn server_handshake<S: Read + Write>(mut conn: ServerConnection, stream: &mut S) -> Result<TlsState, Error> {
    loop {
        while let Some(data) = conn.write_tls() {
            stream.write_all(&data).map_err(into_tls_err)?;
        }
        if conn.handshake_done() {
            return Ok(TlsState::Server(conn));
        }
        let mut buf = [0u8; 16384];
        let n = stream.read(&mut buf).map_err(into_tls_err)?;
        if n == 0 {
            return Err(Error::ConnectionClosed);
        }
        conn.inject(&buf[..n]);
        block_on(conn.process())?;
    }
}

// ── TlsStream ───────────────────────────────────────────────────────────

/// A TLS stream over a blocking [`Read`] + [`Write`] stream.
///
/// Created by [`TlsConnector::connect`] or [`TlsAcceptor::accept`].
/// After construction the handshake is complete and the stream is ready
/// for application data.
pub struct TlsStream<S: Read + Write> {
    stream: S,
    state: TlsState,
    read_buf: [u8; 16384],
    pending_read: Option<Bytes>,
}

impl<S: Read + Write> TlsStream<S> {
    pub fn cipher_suite(&self) -> Option<CipherSuite> {
        self.state.cipher_suite()
    }

    pub fn key_exchange_group(&self) -> Option<KeyExchangeGroup> {
        self.state.key_exchange_group()
    }

    pub fn alpn_protocol(&self) -> Option<&Bytes> {
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
}

impl<S: Read + Write> Read for TlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if let Some(data) = self.pending_read.take() {
                let len = data.len().min(buf.len());
                buf[..len].copy_from_slice(&data[..len]);
                if len < data.len() {
                    self.pending_read = Some(data.slice(len..));
                }
                return Ok(len);
            }
            if let Some(data) = self.state.read_app_data() {
                let len = data.len().min(buf.len());
                buf[..len].copy_from_slice(&data[..len]);
                if len < data.len() {
                    self.pending_read = Some(data.slice(len..));
                }
                return Ok(len);
            }

            if self.state.close_notified() {
                return Ok(0);
            }

            let n = self.stream.read(&mut self.read_buf)?;
            if n == 0 {
                return Ok(0);
            }
            self.state.inject(&self.read_buf[..n]);
            match block_on(self.state.process_app_data()) {
                Ok(_) => {}
                Err(Error::ConnectionClosed) => {}
                Err(e) => return Err(into_io_err(e)),
            }
        }
    }
}

impl<S: Read + Write> Write for TlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let encrypted = self.state.send(buf).map_err(into_io_err)?;
        self.stream.write_all(&encrypted)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
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
    pub fn connect<S: Read + Write>(&self, server_name: &str, mut stream: S) -> Result<TlsStream<S>, Error> {
        let conn = block_on(ClientConnection::new(self.config.clone(), Some(server_name.into())))?;
        let state = client_handshake(conn, &mut stream)?;
        Ok(TlsStream {
            stream,
            state,
            read_buf: [0u8; 16384],
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
    pub fn accept<S: Read + Write>(&self, mut stream: S) -> Result<TlsStream<S>, Error> {
        let conn = ServerConnection::new(self.config.clone());
        let state = server_handshake(conn, &mut stream)?;
        Ok(TlsStream {
            stream,
            state,
            read_buf: [0u8; 16384],
            pending_read: None,
        })
    }
}
