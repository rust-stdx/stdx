use std::sync::Arc;

use bytes::Bytes;

/// QUIC connection configuration.
///
/// Holds transport parameters and the TLS configuration for either
/// client or server mode.
pub struct Config {
    pub initial_max_streams_bidi: u64,
    pub initial_max_streams_uni: u64,
    pub initial_max_data: u64,
    pub initial_max_stream_data_bidi_local: u64,
    pub initial_max_stream_data_bidi_remote: u64,
    pub initial_max_stream_data_uni: u64,
    pub max_idle_timeout_ms: u64,
    pub max_ack_delay_ms: u64,
    pub ack_delay_exponent: u8,
    pub active_connection_id_limit: u64,
    pub max_datagram_frame_size: u64,
    pub alpn_protocols: Vec<Bytes>,

    /// TLS configuration.
    pub tls_config: TlsConfig,

    /// Max size of the receive buffer for datagrams (default: 8192).
    pub recv_buf_size: usize,

    /// Initial max datagram size for sending QUIC packets.
    /// Default is 1200 (minimum required by RFC 9000 §14).
    pub initial_max_datagram_size: usize,
}

/// Which side of the TLS handshake to run.
pub enum TlsConfig {
    /// Client configuration.
    Client(tls::ClientConfig),
    /// Server configuration.
    Server(tls::ServerConfig),
}

impl Default for Config {
    fn default() -> Self {
        let provider = Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new());
        let validator = Arc::new(tls::default_validator::WebPkiValidator::with_default_roots(provider.clone()));
        let alpn = vec![Bytes::from_static(b"h3")];
        Self {
            initial_max_streams_bidi: 100,
            initial_max_streams_uni: 100,
            initial_max_data: 1_048_576,
            initial_max_stream_data_bidi_local: 262_144,
            initial_max_stream_data_bidi_remote: 262_144,
            initial_max_stream_data_uni: 131_072,
            max_idle_timeout_ms: 30_000,
            max_ack_delay_ms: 25,
            ack_delay_exponent: 3,
            active_connection_id_limit: 2,
            max_datagram_frame_size: 1500,
            alpn_protocols: alpn.clone(),
            tls_config: TlsConfig::Client(tls::ClientConfig::new(provider, alpn, validator)),
            recv_buf_size: 8192,
            initial_max_datagram_size: 1200,
        }
    }
}
