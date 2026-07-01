use std::sync::Arc;

use async_trait::async_trait;
use tls::{
    CertType, ClientConfig, ClientConnection, Error, HandshakeFailure, SIGNING_PUBLIC_KEY_MAX_SIZE, ServerConfig,
    ServerConnection,
    config::{CertificateProvider, ClientHello, ProvidedCertificate},
    crypto::CryptoProvider,
    crypto_default_provider::DefaultCryptoProvider,
};

struct DummyCertProvider;

#[async_trait]
impl CertificateProvider for DummyCertProvider {
    async fn provide(&self, client_hello: &ClientHello<'_>) -> Result<ProvidedCertificate, Error> {
        let mut seed = [0u8; 32];
        client_hello
            .cipher_suites
            .first()
            .ok_or(Error::NoCipherSuitesInCommon)?;
        let scheme = *client_hello
            .sig_schemes
            .first()
            .ok_or(Error::NoKeyExchangeGroupInCommon)?;
        let provider = DefaultCryptoProvider::new();
        provider.secure_random(&mut seed);
        let sk = crypto::curve25519::ed25519::SecretKey::from_bytes(&seed);
        let pk = sk.public_key();
        let signer = provider.create_signer(scheme, &seed)?;
        let pk_bytes = pk.to_bytes();
        let spki = build_ed25519_spki(&pk_bytes);
        Ok(ProvidedCertificate {
            scheme,
            payload: tls::config::RawPublicKeyCert {
                public_key: spki,
                signer,
            },
        })
    }
}

fn build_ed25519_spki(public_key: &[u8; 32]) -> heapless::Vec<u8, SIGNING_PUBLIC_KEY_MAX_SIZE> {
    let mut spki = heapless::Vec::new();
    let alg_id = [0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70];
    let bitstring_len = 2 + 1 + 32; // 0x03, 0x21, 0x00, 32 bytes
    let total_len = 2 + alg_id.len() + bitstring_len; // outer SEQUENCE header + content
    spki.push(0x30).unwrap();
    spki.push(total_len as u8).unwrap();
    spki.extend_from_slice(&alg_id).unwrap();
    spki.push(0x03).unwrap();
    spki.push(33).unwrap(); // bitstring length (32 + 1 unused bits byte)
    spki.push(0x00).unwrap();
    spki.extend_from_slice(public_key).unwrap();
    spki
}

struct AcceptAllValidator;

#[async_trait]
impl tls::config::CertificateValidator for AcceptAllValidator {
    async fn validate(
        &self,
        _cert: &tls::config::ReceivedCertificate,
        _server_name: Option<&str>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[tokio::test]
async fn selfcontained_app_data() {
    let provider = Arc::new(DefaultCryptoProvider::new());

    let server_cfg = ServerConfig::new(provider.clone(), heapless::Vec::new(), Arc::new(DummyCertProvider));
    let mut server = ServerConnection::new(server_cfg);

    let client_cfg = ClientConfig::new(provider.clone(), heapless::Vec::new(), Arc::new(AcceptAllValidator))
        .with_cert_types([CertType::X509, CertType::RawPublicKey].into());
    let mut client = ClientConnection::new(client_cfg, Some("localhost"))
        .await
        .unwrap();

    let mut ch_bytes = Vec::new();
    while let Some(data) = client.write_tls() {
        ch_bytes.extend_from_slice(&data);
    }
    assert!(!ch_bytes.is_empty(), "ClientHello should be generated");

    server.inject(&ch_bytes);
    server.process().await.unwrap();
    let mut server_response = Vec::new();
    while let Some(data) = server.write_tls() {
        server_response.extend_from_slice(&data);
    }
    assert!(!server_response.is_empty(), "Server should respond");

    client.inject(&server_response);
    client.process().await.unwrap();
    let mut client_fin = Vec::new();
    while let Some(data) = client.write_tls() {
        client_fin.extend_from_slice(&data);
    }
    assert!(
        client.handshake_done(),
        "Client handshake should complete after first server response"
    );

    if !client_fin.is_empty() {
        server.inject(&client_fin);
        server.process().await.unwrap();
    }

    assert!(server.handshake_done(), "Full handshake should complete");

    let server_app = server.encrypt_application_data(b"HELLO FROM SERVER").unwrap();
    client.inject(&server_app);
    client.process().await.unwrap();
    let mut received_server = false;
    while let Some(data) = client.read_app_data() {
        assert_eq!(&data[..], b"HELLO FROM SERVER", "Client should receive server's app data");
        received_server = true;
    }
    assert!(received_server, "Client should receive app data from server");

    let client_app = client.encrypt_application_data(b"HELLO FROM CLIENT").unwrap();
    server.inject(&client_app);
    server.process().await.unwrap();
    let mut received_client = false;
    while let Some(data) = server.read_app_data() {
        assert_eq!(&data[..], b"HELLO FROM CLIENT", "Server should receive client's app data");
        received_client = true;
    }
    assert!(received_client, "Server should receive app data from client");

    eprintln!("SELF-CONTAINED TEST PASSED: bidirectional app data works!");
}

#[tokio::test]
async fn selfcontained_app_data_pq() {
    let provider = Arc::new(DefaultCryptoProvider::new());

    let server_cfg = ServerConfig::new(provider.clone(), heapless::Vec::new(), Arc::new(DummyCertProvider));
    let mut server = ServerConnection::new(server_cfg);

    let client_cfg = ClientConfig::new(provider.clone(), heapless::Vec::new(), Arc::new(AcceptAllValidator))
        .with_cert_types([CertType::X509, CertType::RawPublicKey].into());
    let mut client = ClientConnection::new(client_cfg, Some("localhost"))
        .await
        .unwrap();

    let mut ch_bytes = Vec::new();
    while let Some(data) = client.write_tls() {
        ch_bytes.extend_from_slice(&data);
    }
    assert!(!ch_bytes.is_empty(), "ClientHello should be generated");

    server.inject(&ch_bytes);
    server.process().await.unwrap();
    let mut server_response = Vec::new();
    while let Some(data) = server.write_tls() {
        server_response.extend_from_slice(&data);
    }
    assert!(!server_response.is_empty(), "Server should respond");

    client.inject(&server_response);
    client.process().await.unwrap();
    let mut client_fin = Vec::new();
    while let Some(data) = client.write_tls() {
        client_fin.extend_from_slice(&data);
    }
    assert!(
        client.handshake_done(),
        "Client handshake should complete after first server response"
    );

    if !client_fin.is_empty() {
        server.inject(&client_fin);
        server.process().await.unwrap();
    }

    assert!(server.handshake_done(), "Full handshake should complete");

    eprintln!("PQ SELF-CONTAINED TEST PASSED: X25519MLKEM768 handshake works!");
}
