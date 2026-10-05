//! End-to-end TLS 1.3 handshake tests using this provider on both ends.
//!
//! These use a `UnixStream` pair as an in-memory transport.
#![cfg(unix)]

use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    sync::Arc,
};

use rustls::{
    CipherSuite, ClientConfig, ClientConnection, DigitallySignedStruct, NamedGroup, RootCertStore, ServerConfig,
    ServerConnection, SignatureScheme, StreamOwned,
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    crypto::CryptoProvider,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName, UnixTime},
    sign::{CertifiedKey, SingleCertAndKey},
};

// An OpenSSL-generated ML-DSA-65 PKI: a self-signed CA and a `localhost` leaf
// certificate signed by it, plus the leaf's PKCS#8 private key. The key is in
// OpenSSL's `both` form (seed followed by the expanded key), which exercises
// the RFC 9881 seed-extraction path.
const ML_DSA_65_CA_DER: &[u8] = include_bytes!("assets/mldsa65-ca-cert.der");
const ML_DSA_65_LEAF_DER: &[u8] = include_bytes!("assets/mldsa65-leaf-cert.der");
const ML_DSA_65_LEAF_KEY_DER: &[u8] = include_bytes!("assets/mldsa65-leaf-key.der");

/// A verifier that accepts any certificate. These tests exercise the
/// provider's key exchange, AEAD and signing paths; certificate verification
/// is covered by the unit tests in `verify.rs`.
#[derive(Debug)]
struct AcceptAnyServerCert;

impl ServerCertVerifier for AcceptAnyServerCert {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ED25519,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
        ]
    }
}

fn ed25519_key() -> PrivateKeyDer<'static> {
    let key = crypto::curve25519::ed25519::SecretKey::generate().unwrap();
    let der = crypto::encoding::pkcs8::encode_ed25519_pkcs8_der(&key).to_vec();
    PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(der))
}

fn p256_key() -> PrivateKeyDer<'static> {
    let key = crypto::p256::SecretKey::generate().unwrap();
    let der = crypto::encoding::pkcs8::encode_p256_pkcs8_der(&key).unwrap().to_vec();
    PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(der))
}

fn configs(provider: CryptoProvider, key: PrivateKeyDer<'static>) -> (Arc<ClientConfig>, Arc<ServerConfig>) {
    let certs = vec![CertificateDer::from(vec![0x30, 0x00])];

    // `with_single_cert` would validate that the certificate's public key
    // matches the private key. These tests use a dummy certificate (the client
    // verifier accepts anything), so build the `CertifiedKey` directly.
    let signing_key = provider.key_provider.load_private_key(key).unwrap();
    let certified = CertifiedKey::new(certs, signing_key);

    let server = ServerConfig::builder_with_provider(Arc::new(provider.clone()))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(SingleCertAndKey::from(certified)));

    let client = ClientConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(AcceptAnyServerCert))
        .with_no_client_auth();

    (Arc::new(client), Arc::new(server))
}

/// Runs a full handshake plus one application-data exchange, returning the
/// negotiated key exchange group and cipher suite.
fn run_handshake(
    client_config: Arc<ClientConfig>,
    server_config: Arc<ServerConfig>,
) -> (Option<NamedGroup>, CipherSuite) {
    let (client_sock, server_sock) = UnixStream::pair().unwrap();
    let client_conn = ClientConnection::new(client_config, ServerName::try_from("example.com").unwrap()).unwrap();
    let server_conn = ServerConnection::new(server_config).unwrap();

    let mut client = StreamOwned::new(client_conn, client_sock);
    let mut server = StreamOwned::new(server_conn, server_sock);

    let handle = std::thread::spawn(move || {
        let mut buf = [0u8; 5];
        server.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"ping!");
        server.write_all(b"pong!").unwrap();
        server.flush().unwrap();
    });

    client.write_all(b"ping!").unwrap();
    client.flush().unwrap();
    let mut buf = [0u8; 5];
    client.read_exact(&mut buf).unwrap();
    assert_eq!(&buf, b"pong!");

    handle.join().unwrap();

    let group = client.conn.negotiated_key_exchange_group().map(|group| group.name());
    let suite = client.conn.negotiated_cipher_suite().unwrap().suite();
    (group, suite)
}

#[test]
fn negotiates_post_quantum_hybrid_by_default() {
    let (client, server) = configs(crypto_rustls::default_provider(), ed25519_key());
    let (group, suite) = run_handshake(client, server);

    assert_eq!(group, Some(NamedGroup::X25519MLKEM768));
    assert!(matches!(
        suite,
        CipherSuite::TLS13_AES_128_GCM_SHA256
            | CipherSuite::TLS13_AES_256_GCM_SHA384
            | CipherSuite::TLS13_CHACHA20_POLY1305_SHA256
    ));
}

#[test]
fn every_cipher_suite_handshakes() {
    for suite in [
        CipherSuite::TLS13_AES_128_GCM_SHA256,
        CipherSuite::TLS13_AES_256_GCM_SHA384,
        CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
    ] {
        let mut provider = crypto_rustls::default_provider();
        provider.cipher_suites.retain(|candidate| candidate.suite() == suite);

        let (client, server) = configs(provider, ed25519_key());
        let (group, negotiated) = run_handshake(client, server);

        assert_eq!(negotiated, suite);
        assert_eq!(group, Some(NamedGroup::X25519MLKEM768));
    }
}

#[test]
fn falls_back_to_classical_x25519() {
    let mut provider = crypto_rustls::default_provider();
    provider
        .kx_groups
        .retain(|group| group.name() != NamedGroup::X25519MLKEM768);

    let (client, server) = configs(provider, ed25519_key());
    let (group, _) = run_handshake(client, server);
    assert_eq!(group, Some(NamedGroup::X25519));
}

#[test]
fn serves_with_ecdsa_p256_key() {
    let (client, server) = configs(crypto_rustls::default_provider(), p256_key());
    let (group, _) = run_handshake(client, server);
    assert_eq!(group, Some(NamedGroup::X25519MLKEM768));
}

/// A complete post-quantum handshake: `X25519MLKEM768` key exchange with an
/// ML-DSA-65 certificate that the client verifies through webpki using this
/// provider's own ML-DSA verification algorithm.
#[test]
fn full_post_quantum_handshake_with_mldsa_certificate() {
    let provider = crypto_rustls::default_provider();

    let ca = CertificateDer::from(ML_DSA_65_CA_DER.to_vec());
    let leaf = CertificateDer::from(ML_DSA_65_LEAF_DER.to_vec());
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(ML_DSA_65_LEAF_KEY_DER.to_vec()));

    // `with_single_cert` also checks that the private key's public key matches
    // the end-entity certificate's SubjectPublicKeyInfo.
    let server = ServerConfig::builder_with_provider(Arc::new(provider.clone()))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![leaf, ca.clone()], key)
        .unwrap();

    let mut roots = RootCertStore::empty();
    roots.add(ca).unwrap();

    let client = ClientConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let (group, suite) = run_handshake(Arc::new(client), Arc::new(server));
    assert_eq!(group, Some(NamedGroup::X25519MLKEM768));
    assert!(matches!(
        suite,
        CipherSuite::TLS13_AES_128_GCM_SHA256
            | CipherSuite::TLS13_AES_256_GCM_SHA384
            | CipherSuite::TLS13_CHACHA20_POLY1305_SHA256
    ));
}
