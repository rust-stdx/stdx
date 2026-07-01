use std::{
    io::{Read, Write},
    sync::Arc,
};

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// End-to-end test: rustls server <-> our TLS client, including app data.
#[tokio::test]
async fn handshake_and_app_data_compat_test() {
    let (server_cfg, _server_cert, _server_key) = make_self_signed();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let barrier_clone = barrier.clone();

    let server_thread = std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut tls = rustls::ServerConnection::new(Arc::new(server_cfg)).unwrap();
        let mut tcp = stream.try_clone().unwrap();
        let mut buf = [0u8; 65536];

        // Complete handshake
        while tls.is_handshaking() {
            if tls.wants_write() {
                tls.write_tls(&mut tcp as &mut dyn Write).unwrap();
            }
            if tls.wants_read() {
                let n = tcp.read(&mut buf).unwrap();
                if n == 0 {
                    break;
                }
                tls.read_tls(&mut &buf[..n]).unwrap();
            }
            let _ = tls.process_new_packets();
        }

        // Send application data
        tls.writer().write_all(b"HELLO FROM SERVER").unwrap();
        tls.write_tls(&mut tcp as &mut dyn Write).unwrap();

        // Wait for client to read it
        barrier_clone.wait();

        // Read client's application data
        loop {
            let mut app_buf = [0u8; 1024];
            match tls.reader().read(&mut app_buf) {
                Ok(n) if n > 0 => {
                    eprintln!("Server received app data: {:?}", &app_buf[..n]);
                    assert_eq!(&app_buf[..n], b"HELLO FROM CLIENT");
                    break;
                }
                _ => {
                    let n = tcp.read(&mut buf).unwrap();
                    if n == 0 {
                        break;
                    }
                    tls.read_tls(&mut &buf[..n]).unwrap();
                    let _ = tls.process_new_packets();
                }
            }
        }
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let provider = Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new());
    let validator = Arc::new(AcceptAllValidator);
    let config = tls::config::ClientConfig::new(provider, heapless::Vec::new(), validator);

    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    let mut client = tls::ClientConnection::new(config, Some("localhost".to_string()))
        .await
        .unwrap();
    while let Some(data) = client.write_tls() {
        stream.write_all(&data).await.unwrap();
    }

    let mut buf = vec![0u8; 65536];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 {
            panic!("connection closed during handshake");
        }
        client.inject(&buf[..n]);
        client.process().await.unwrap();
        while let Some(data) = client.write_tls() {
            stream.write_all(&data).await.unwrap();
        }
        if client.handshake_done() {
            break;
        }
    }
    eprintln!("Handshake succeeded. Cipher: {:?}", client.cipher_suite().unwrap());

    // Read application data from server
    let mut got_server_data = false;
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }
        client.inject(&buf[..n]);
        client.process().await.unwrap();
        while let Some(data) = client.write_tls() {
            stream.write_all(&data).await.unwrap();
        }
        while let Some(data) = client.read_app_data() {
            eprintln!("Client received app data: {:?}", &data[..]);
            assert_eq!(&data[..], b"HELLO FROM SERVER");
            got_server_data = true;
        }
        if got_server_data {
            break;
        }
    }
    assert!(got_server_data, "Client should receive app data from server");
    barrier.wait();

    // Send application data to server
    let app_data = client.send(b"HELLO FROM CLIENT").unwrap();
    stream.write_all(&app_data).await.unwrap();

    let _ = server_thread.join();
}

fn make_self_signed() -> (rustls::ServerConfig, Vec<u8>, Vec<u8>) {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer};
    let subject_alt_names = vec!["localhost".to_string()];
    let cert = rcgen::generate_simple_self_signed(subject_alt_names).unwrap();
    let cert_der = cert.cert.der().to_vec();
    let key_der = cert.key_pair.serialize_der().to_vec();

    let config = rustls::ServerConfig::builder_with_provider(Arc::new(rustls::crypto::aws_lc_rs::default_provider()))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(cert_der.clone())],
            PrivateKeyDer::try_from(key_der.clone()).unwrap(),
        )
        .unwrap();

    (config, cert_der, key_der)
}

struct AcceptAllValidator;

#[async_trait]
impl tls::config::CertificateValidator for AcceptAllValidator {
    async fn validate(
        &self,
        _cert: &tls::config::ReceivedCertificate,
        _server_name: Option<&str>,
    ) -> Result<(), tls::Error> {
        Ok(())
    }
}

/// Regression test for the X25519MLKEM768 key share layout.
///
/// Per `draft-ietf-tls-ecdhe-mlkem`, the X25519MLKEM768 key share and shared
/// secret are ordered `ML-KEM-768 || X25519` (post-quantum component first).
/// Reordering them (e.g. `X25519 || ML-KEM-768`) causes the peer to reject the
/// ClientHello with `PeerMisbehaved(InvalidKeyShare)`. This test feeds our
/// ClientHello directly to a rustls server and asserts it is accepted.
#[tokio::test]
async fn x25519mlkem768_clienthello_accepted_by_rustls() {
    use rustls::ServerConnection;
    let (server_cfg, _c, _k) = make_self_signed();
    let mut conn = ServerConnection::new(Arc::new(server_cfg)).unwrap();

    let provider = Arc::new(tls::crypto_default_provider::DefaultCryptoProvider::new());
    let validator = Arc::new(AcceptAllValidator);
    let config = tls::config::ClientConfig::new(provider, heapless::Vec::new(), validator);
    let mut client = tls::ClientConnection::new(config, Some("localhost".to_string()))
        .await
        .unwrap();
    let mut ch_bytes = Vec::new();
    while let Some(data) = client.write_tls() {
        ch_bytes.extend_from_slice(&data);
    }

    conn.read_tls(&mut &ch_bytes[..]).unwrap();
    let st = conn.process_new_packets();
    assert!(
        st.is_ok(),
        "rustls rejected our ClientHello: {st:?} (likely wrong X25519MLKEM768 key share layout)"
    );

    let mut buf = Vec::new();
    conn.write_tls(&mut buf).unwrap();
    assert!(!buf.is_empty(), "rustls produced no ServerHello");
}
