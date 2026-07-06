//! Example: connect a Client and Server using hardcoded Ed25519 raw public keys
//! (RFC 7250) with a custom validator.
//!
//! The server uses a [`StaticCertProvider`] that returns a raw public key derived
//! from hardcoded Ed25519 seed bytes. The client uses a [`PinnedKeyValidator`]
//! that accepts only the exact expected public key.
//!
//! # Run
//!
//! ```sh
//! cargo run --example raw_key -p quic
//! ```

use core::net::SocketAddr;
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use quic::{Config, Instant, IoError, Transport};
use tls::{
    CertType, ClientConfig, SIGNING_PUBLIC_KEY_MAX_SIZE, ServerConfig,
    config::{
        CertificateProvider, CertificateValidator, ClientHello, ProvidedCertificate, RawPublicKeyCert,
        ReceivedCertificate,
    },
    crypto::{CryptoProvider, SignatureScheme},
    crypto_default_provider::DefaultCryptoProvider,
};
use tokio::net::UdpSocket as TokioUdpSocket;

// ── Hardcoded Ed25519 key pair (deterministic) ──────────────────────────

const SERVER_SECRET_KEY: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];

fn server_public_key() -> [u8; 32] {
    // Generate the public key from the deterministic seed at runtime
    let sk = crypto::curve25519::ed25519::SecretKey::from_bytes(&SERVER_SECRET_KEY);
    sk.public_key().to_bytes()
}

// ── DER SPKI builder ───────────────────────────────────────────────────

fn build_ed25519_spki(public_key: [u8; 32]) -> heapless::Vec<u8, SIGNING_PUBLIC_KEY_MAX_SIZE> {
    let mut spki = heapless::Vec::new();
    let alg_id: [u8; 7] = [0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70];
    let bitstring_len = 2 + 1 + 32;
    let total_len = 2 + alg_id.len() + bitstring_len;
    spki.push(0x30).unwrap();
    spki.push(total_len as u8).unwrap();
    spki.extend_from_slice(&alg_id).unwrap();
    spki.push(0x03).unwrap();
    spki.push(33).unwrap();
    spki.push(0x00).unwrap();
    spki.extend_from_slice(&public_key).unwrap();
    spki
}

// ── Transport ──────────────────────────────────────────────────────────

struct UdpTransport {
    socket: TokioUdpSocket,
    epoch: std::time::Instant,
}

#[async_trait]
impl Transport for UdpTransport {
    async fn send_to(&self, dest: SocketAddr, data: &[u8]) -> Result<usize, IoError> {
        self.socket.send_to(data, dest).await.map_err(IoError::from)
    }

    async fn receive_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), IoError> {
        self.socket.recv_from(buf).await.map_err(IoError::from)
    }

    fn local_addr(&self) -> Result<SocketAddr, IoError> {
        self.socket.local_addr().map_err(IoError::from)
    }

    fn now(&self) -> Instant {
        let us = std::time::Instant::now().duration_since(self.epoch).as_micros() as u64;
        Instant::from_micros(us)
    }
}

// ── Server certificate provider ────────────────────────────────────────

struct StaticCertProvider;

#[async_trait]
impl CertificateProvider for StaticCertProvider {
    async fn provide(&self, _client_hello: &ClientHello<'_>) -> Result<ProvidedCertificate, tls::Error> {
        let provider = DefaultCryptoProvider::new();
        let signer = provider.create_signer(SignatureScheme::Ed25519, &SERVER_SECRET_KEY)?;
        let pk = server_public_key();
        let spki = build_ed25519_spki(pk);
        Ok(ProvidedCertificate {
            scheme: SignatureScheme::Ed25519,
            payload: RawPublicKeyCert {
                public_key: spki,
                signer,
            },
        })
    }
}

// ── Client custom validator (pins expected public key) ─────────────────

struct PinnedKeyValidator;

#[async_trait]
impl CertificateValidator for PinnedKeyValidator {
    async fn validate(&self, cert: &ReceivedCertificate, _server_name: Option<&str>) -> Result<(), tls::Error> {
        let expected_pk = server_public_key();
        match cert {
            ReceivedCertificate::RawPublicKey {
                public_key,
                scheme,
            } => {
                if *scheme != SignatureScheme::Ed25519 {
                    return Err(tls::Error::CertificateValidationFailed(
                        tls::CertificateValidationFailure::SignatureVerificationFailed,
                    ));
                }
                if public_key.as_slice() == expected_pk {
                    Ok(())
                } else {
                    Err(tls::Error::CertificateValidationFailed(
                        tls::CertificateValidationFailure::SignatureVerificationFailed,
                    ))
                }
            }
            _ => Err(tls::Error::CertificateValidationFailed(
                tls::CertificateValidationFailure::RawPublicKeyRequiresCustomValidator,
            )),
        }
    }
}

// ── Main ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Arc::new(DefaultCryptoProvider::new());
    let alpn: heapless::Vec<_, _> = [tls::AlpnProtocol::from_static(b"h3")].into();

    // ── Server config ───────────────────────────────────────────────────
    let server_cfg = ServerConfig::new(provider.clone(), alpn.clone(), Arc::new(StaticCertProvider));
    let server_quic_cfg = Config {
        tls_config: quic::config::TlsConfig::Server(server_cfg),
        alpn_protocols: alpn.clone(),
        ..Config::default()
    };

    // ── Client config ───────────────────────────────────────────────────
    let cert_types = [CertType::X509, CertType::RawPublicKey].into();
    let client_cfg =
        ClientConfig::new(provider, alpn.clone(), Arc::new(PinnedKeyValidator)).with_cert_types(cert_types);
    let client_quic_cfg = Config {
        tls_config: quic::config::TlsConfig::Client(client_cfg),
        alpn_protocols: alpn,
        ..Config::default()
    };

    // ── Bind sockets ────────────────────────────────────────────────────
    let server_socket = TokioUdpSocket::bind("127.0.0.1:0").await?;
    let server_addr = server_socket.local_addr()?;
    let client_socket = TokioUdpSocket::bind("127.0.0.1:0").await?;

    let server_transport = UdpTransport {
        socket: server_socket,
        epoch: std::time::Instant::now(),
    };
    let client_transport = UdpTransport {
        socket: client_socket,
        epoch: std::time::Instant::now(),
    };

    // ── Server task ─────────────────────────────────────────────────────
    let mut server_conn = quic::server::ServerConnection::new(server_transport, server_quic_cfg);

    let server_handle = tokio::spawn(async move {
        eprintln!("[server] Waiting for connection...");
        match server_conn.accept().await {
            Ok(()) => {
                eprintln!("[server] Handshake complete!");
                match tokio::time::timeout(Duration::from_secs(5), server_conn.receive_datagram()).await {
                    Ok(Ok(data)) => {
                        let msg = String::from_utf8_lossy(&data);
                        eprintln!("[server] Received datagram: {msg}");
                    }
                    Ok(Err(e)) => eprintln!("[server] Receive error: {e}"),
                    Err(_) => eprintln!("[server] Timed out waiting for datagram"),
                }
            }
            Err(e) => eprintln!("[server] Accept failed: {e}"),
        }
    });

    // Small delay to ensure server is listening
    tokio::time::sleep(Duration::from_millis(50)).await;

    // ── Client connect ──────────────────────────────────────────────────
    let mut client_conn = quic::Connection::new(client_transport, client_quic_cfg);

    eprintln!("[client] Connecting to server at {server_addr}...");
    match client_conn.connect(server_addr, "localhost").await {
        Ok(()) => {
            eprintln!("[client] QUIC handshake successful!");
            client_conn
                .send_datagram(b"Hello, world")
                .await
                .expect("failed to send datagram");
            eprintln!("[client] Sent datagram: Hello, world");
        }
        Err(e) => {
            eprintln!("[client] Connection failed: {e}");
        }
    }

    // ── Wait for server ─────────────────────────────────────────────────
    let _ = server_handle.await;

    Ok(())
}
