//! Transport: TLS is always required, so the only transport is a TLS session.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A PostgreSQL transport. TLS is always required.
pub enum PgStream {
    /// TLS over TCP (rustls).
    Tls(Box<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>),
}

impl AsyncRead for PgStream {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            PgStream::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for PgStream {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        match self.get_mut() {
            PgStream::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            PgStream::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            PgStream::Tls(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

pub(crate) mod tls {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex, OnceLock},
    };

    use rustls::{
        ClientConfig, DigitallySignedStruct, RootCertStore, SignatureScheme,
        client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
        pki_types::{CertificateDer, ServerName, UnixTime},
    };

    use crate::{config::SslMode, error::Error};

    /// A verifier that accepts any certificate.
    ///
    /// Used for [`SslMode::Require`], which (like libpq's `require`) provides
    /// **encryption only**: the server is never authenticated, so an active
    /// attacker can intercept the connection, impersonate the server and run
    /// its own authentication exchange. Use [`SslMode::VerifyFull`] whenever
    /// the server's identity matters.
    #[derive(Debug)]
    struct NoVerifier {
        algorithms: rustls::crypto::WebPkiSupportedAlgorithms,
    }

    impl ServerCertVerifier for NoVerifier {
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
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            // `require` means "encrypted, not authenticated" (like libpq): the
            // certificate chain is not checked and the handshake signature is
            // only verified when the certificate can be parsed at all (legacy
            // X.509 v1 certificates are accepted unverified). Nothing here
            // authenticates the server.
            Ok(rustls::crypto::verify_tls12_signature(message, cert, dss, &self.algorithms)
                .unwrap_or(HandshakeSignatureValid::assertion()))
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::crypto::verify_tls13_signature(message, cert, dss, &self.algorithms)
                .unwrap_or(HandshakeSignatureValid::assertion()))
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            self.algorithms.supported_schemes()
        }
    }

    /// A verifier for `sslmode=verify-ca`: it verifies the certificate chain up
    /// to a trusted root, but does **not** check the hostname.
    #[derive(Debug)]
    struct ChainOnlyVerifier {
        roots: Arc<RootCertStore>,
        algorithms: rustls::crypto::WebPkiSupportedAlgorithms,
    }

    impl ServerCertVerifier for ChainOnlyVerifier {
        fn verify_server_cert(
            &self,
            end_entity: &CertificateDer<'_>,
            intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp_response: &[u8],
            now: UnixTime,
        ) -> Result<ServerCertVerified, rustls::Error> {
            let cert = rustls::server::ParsedCertificate::try_from(end_entity)?;
            rustls::client::verify_server_cert_signed_by_trust_anchor(
                &cert,
                &self.roots,
                intermediates,
                now,
                self.algorithms.all,
            )?;
            // Deliberately no `verify_server_name`: `verify-ca` does not check
            // the hostname.
            Ok(ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            rustls::crypto::verify_tls12_signature(message, cert, dss, &self.algorithms)
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            rustls::crypto::verify_tls13_signature(message, cert, dss, &self.algorithms)
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            self.algorithms.supported_schemes()
        }
    }

    /// The shared TLS crypto provider.
    ///
    /// Building it sets up the post-quantum key exchange and signature
    /// algorithms, which is not free; it is done once per process so that
    /// opening a connection does not repeat it.
    fn provider() -> Arc<rustls::crypto::CryptoProvider> {
        static PROVIDER: OnceLock<Arc<rustls::crypto::CryptoProvider>> = OnceLock::new();
        PROVIDER
            .get_or_init(|| Arc::new(crypto_rustls::default_provider()))
            .clone()
    }

    /// Loads and caches trusted roots under `key`.
    ///
    /// The operating system store is thousands of certificates and a pool can
    /// open many connections per second, so it is parsed once per process.
    fn cached_roots(
        key: &str,
        load: impl FnOnce() -> Result<RootCertStore, Error>,
    ) -> Result<Arc<RootCertStore>, Error> {
        static CACHE: Mutex<Option<HashMap<String, Arc<RootCertStore>>>> = Mutex::new(None);
        let mut guard = CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let cache = guard.get_or_insert_with(HashMap::new);
        if let Some(store) = cache.get(key) {
            return Ok(store.clone());
        }
        let store = Arc::new(load()?);
        cache.insert(key.to_string(), store.clone());
        Ok(store)
    }

    fn base_builder(
        provider: Arc<rustls::crypto::CryptoProvider>,
    ) -> rustls::ConfigBuilder<ClientConfig, rustls::WantsVerifier> {
        ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("TLS 1.3 is supported by the provider")
    }

    fn load_pem_roots(path: &str) -> Result<RootCertStore, Error> {
        let data = std::fs::read(path).map_err(|e| Error::Config(format!("cannot read sslrootcert `{path}`: {e}")))?;
        let mut roots = RootCertStore::empty();
        for cert in parse_pem_certificates(&data) {
            roots
                .add(cert)
                .map_err(|e| Error::Config(format!("invalid certificate in `{path}`: {e}")))?;
        }
        Ok(roots)
    }

    /// Extracts DER certificates from a PEM bundle.
    fn parse_pem_certificates(data: &[u8]) -> Vec<CertificateDer<'static>> {
        const BEGIN: &[u8] = b"-----BEGIN CERTIFICATE-----";
        const END: &[u8] = b"-----END CERTIFICATE-----";
        let mut certs = Vec::new();
        let mut rest = data;
        while let Some(start) = find(rest, BEGIN) {
            let after = &rest[start + BEGIN.len()..];
            let Some(end) = find(after, END) else {
                break;
            };
            let body: Vec<u8> = after[..end]
                .iter()
                .copied()
                .filter(|b| !b.is_ascii_whitespace())
                .collect();
            if let Ok(der) = base64::decode(&body, base64::Alphabet::Standard) {
                certs.push(CertificateDer::from(der));
            }
            rest = &after[end + END.len()..];
        }
        certs
    }

    fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len()).position(|window| window == needle)
    }

    /// Loads trusted roots from a PEM file or the operating system store.
    fn roots(config: &crate::config::Config) -> Result<Arc<RootCertStore>, Error> {
        match &config.sslrootcert {
            Some(path) => cached_roots(path, || load_pem_roots(path)),
            None => cached_roots("<system>", || {
                let mut store = RootCertStore::empty();
                for dir in system_root_dirs() {
                    add_pem_dir(&mut store, dir);
                }
                if store.is_empty() {
                    return Err(Error::Config(
                        "no trusted root certificates found; provide `sslrootcert`".into(),
                    ));
                }
                Ok(store)
            }),
        }
    }

    fn system_root_dirs() -> &'static [&'static str] {
        #[cfg(target_os = "macos")]
        {
            &["/etc/ssl", "/usr/local/etc/openssl/certs"]
        }
        #[cfg(not(target_os = "macos"))]
        {
            &[
                "/etc/ssl/certs",
                "/etc/pki/tls/certs",
                "/usr/local/share/certs",
                "/usr/share/ca-certificates/mozilla",
            ]
        }
    }

    fn add_pem_dir(store: &mut RootCertStore, dir: &str) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let is_cert = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| matches!(e, "pem" | "crt" | "cer"))
                .unwrap_or(false);
            if !is_cert {
                continue;
            }
            if let Ok(data) = std::fs::read(&path) {
                for cert in parse_pem_certificates(&data) {
                    let _ = store.add(cert);
                }
            }
        }
    }

    /// Builds a rustls connector according to the configured [`SslMode`].
    ///
    /// Each call builds a fresh `ClientConfig` (the trust anchors and the
    /// crypto provider behind it are cached), so different connections may use
    /// different verification levels.
    pub(crate) fn connector(config: &crate::config::Config) -> Result<tokio_rustls::TlsConnector, Error> {
        let provider = provider();

        let client_config = match config.sslmode {
            SslMode::Require => {
                let verifier = Arc::new(NoVerifier {
                    algorithms: provider.signature_verification_algorithms,
                });
                base_builder(provider)
                    .dangerous()
                    .with_custom_certificate_verifier(verifier)
                    .with_no_client_auth()
            }
            SslMode::VerifyCa => {
                let verifier = Arc::new(ChainOnlyVerifier {
                    roots: roots(config)?,
                    algorithms: provider.signature_verification_algorithms,
                });
                base_builder(provider)
                    .dangerous()
                    .with_custom_certificate_verifier(verifier)
                    .with_no_client_auth()
            }
            SslMode::VerifyFull => {
                let store = roots(config)?;
                base_builder(provider)
                    .with_root_certificates(store)
                    .with_no_client_auth()
            }
        };

        Ok(tokio_rustls::TlsConnector::from(Arc::new(client_config)))
    }
}
