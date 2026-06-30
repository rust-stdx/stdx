//! Bridge between the tls crate's `QuicHandshake` trait and QUIC's CRYPTO frames.
//!
//! `TlsAdapter` wraps any type implementing [`tls::QuicHandshake`] (both
//! [`tls::ClientConnection`] and [`tls::ServerConnection`]) and handles the
//! handshake data flow.
//!
//! # Client example
//!
//! ```ignore
//! let conn = tls::ClientConnection::new_quic_with_preferred_group(config, None, params, alpn, None)?;
//! let mut adapter = TlsAdapter::new(conn);
//! loop {
//!     if let Some(data) = adapter.write_handshake() { /* send CRYPTO frame */ }
//!     match adapter.process().await? {
//!         tls::QuicHandshakeEvent::HandshakeComplete => break,
//!         _ => {}
//!     }
//!     /* receive CRYPTO frame data, call adapter.inject_handshake(data) */
//! }
//! let secrets = adapter.quic_secrets().unwrap();
//! ```
//!
//! # Server example
//!
//! ```ignore
//! let conn = tls::ServerConnection::new_quic(config);
//! let mut adapter = TlsAdapter::new(conn);
//! // feed ClientHello via inject_handshake, drive process() until complete
//! ```

use alloc::{boxed::Box, vec::Vec};

use tls::{CipherSuite, QuicHandshake, QuicHandshakeEvent, quic::QuicSecrets};

/// Adapter that bridges the TLS `QuicHandshake` trait to QUIC CRYPTO frames.
pub struct TlsAdapter {
    inner: Box<dyn QuicHandshake>,
    handshake_done: bool,
}

impl TlsAdapter {
    /// Wrap an existing QUIC handshake connection.
    ///
    /// Both client and server connections are accepted.
    pub fn new(inner: impl QuicHandshake + 'static) -> Self {
        let done = inner.is_handshake_done();
        Self {
            inner: Box::new(inner),
            handshake_done: done,
        }
    }

    /// Get the next chunk of raw handshake bytes to send in a CRYPTO frame.
    pub fn write_handshake(&mut self) -> Option<Vec<u8>> {
        self.inner.write_handshake().map(|b| b.to_vec())
    }

    /// Feed raw handshake bytes received from a CRYPTO frame.
    pub fn inject_handshake(&mut self, data: &[u8]) {
        self.inner.inject_handshake(data);
    }

    /// Advance the TLS state machine. Should be called after `inject_handshake`.
    pub async fn process(&mut self) -> Result<QuicHandshakeEvent, tls::Error> {
        let event = self.inner.process().await?;
        if event == QuicHandshakeEvent::HandshakeComplete && !self.handshake_done {
            self.handshake_done = true;
        }
        Ok(event)
    }

    /// Extract the QUIC traffic secrets after the handshake completes.
    pub fn quic_secrets(&self) -> Option<QuicSecrets> {
        self.inner.quic_secrets()
    }

    /// Return the negotiated cipher suite, if any.
    pub fn cipher_suite(&self) -> Option<CipherSuite> {
        self.inner.cipher_suite()
    }

    /// Whether the handshake is complete.
    pub fn is_handshake_done(&self) -> bool {
        self.inner.is_handshake_done()
    }
}
