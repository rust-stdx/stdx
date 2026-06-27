//! Bridge between the tls crate's `ClientConnection` and QUIC's CRYPTO frames.
//!
//! The tls crate's QUIC support (`new_quic`, `write_handshake`, `inject_handshake`)
//! already returns raw handshake bytes (no TLS record framing). This adapter
//! wraps the tls::ClientConnection and handles the handshake data flow.

use bytes::Bytes;
use tls::{ClientConfig, ClientConnection, quic::QuicSecrets};

use crate::error::Error;

/// Adapter that bridges the tls crate to QUIC.
pub struct TlsAdapter {
    inner: ClientConnection,
    handshake_done: bool,
}

impl TlsAdapter {
    /// Create a new TLS adapter for QUIC.
    ///
    /// * `config` - TLS client configuration (includes crypto provider and cert validator).
    /// * `server_name` - SNI hostname.
    /// * `transport_params` - Encoded QUIC transport parameters to include in the ClientHello.
    /// * `alpn` - ALPN protocol list.
    pub fn new(
        config: ClientConfig,
        server_name: &str,
        transport_params: &[u8],
        alpn: &[Bytes],
    ) -> Result<Self, Error> {
        Self::new_with_preferred_group(config, server_name, transport_params, alpn, None)
    }

    /// Create a new TLS adapter with a preferred key exchange group.
    ///
    /// When `preferred_group` is `Some(g)`, the ClientHello uses `g` as the
    /// primary key exchange group (for HelloRetryRequest handling).
    pub fn new_with_preferred_group(
        config: ClientConfig,
        server_name: &str,
        transport_params: &[u8],
        alpn: &[Bytes],
        preferred_group: Option<tls::KeyExchangeGroup>,
    ) -> Result<Self, Error> {
        let inner = ClientConnection::new_quic_with_preferred_group(
            config,
            Some(server_name.to_owned()),
            transport_params,
            alpn,
            preferred_group,
        )
        .map_err(|e| Error::ConnectionRejected(format!("TLS init failed: {e}")))?;

        Ok(Self {
            inner,
            handshake_done: false,
        })
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
    pub async fn process(&mut self) -> Result<TlsEvent, Error> {
        match self.inner.process().await {
            Ok(()) => {
                if self.inner.handshake_done() && !self.handshake_done {
                    self.handshake_done = true;
                    Ok(TlsEvent::HandshakeComplete)
                } else if self.handshake_done {
                    Ok(TlsEvent::Idle)
                } else {
                    Ok(TlsEvent::NeedMoreData)
                }
            }
            Err(tls::Error::ConnectionClosed) => Err(Error::ConnectionClosed(0, "TLS connection closed".into())),
            Err(e) => Err(Error::ConnectionRejected(format!("TLS error: {e}"))),
        }
    }

    /// Extract the QUIC traffic secrets after the handshake completes.
    pub fn quic_secrets(&self) -> Option<QuicSecrets> {
        self.inner.quic_secrets()
    }

    /// Return the negotiated cipher suite, if any.
    pub fn cipher_suite(&self) -> Option<tls::CipherSuite> {
        self.inner.cipher_suite()
    }

    /// Whether the handshake is complete.
    pub fn is_handshake_done(&self) -> bool {
        self.handshake_done
    }
}

/// Events returned by the TLS handshake state machine.
pub enum TlsEvent {
    /// More handshake data is needed from the peer.
    NeedMoreData,
    /// Handshake data is ready to be sent in a CRYPTO frame.
    /// Typically returned after a call to `write_handshake`.
    SendData,
    /// TLS handshake has completed and QUIC secrets are available.
    HandshakeComplete,
    /// No work to do.
    Idle,
    /// Server sent a HelloRetryRequest; `group` is the requested key exchange group.
    HelloRetryRequest(tls::KeyExchangeGroup),
}
