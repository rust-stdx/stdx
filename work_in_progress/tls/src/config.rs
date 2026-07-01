use alloc::{boxed::Box, sync::Arc, vec::Vec};

use async_trait::async_trait;
use bytes::Bytes;

use crate::{
    Error,
    crypto::{
        CertType, CipherSuite, CryptoProvider, KeyExchangeGroup, MAX_PUBLIC_KEY_BYTES, PSK_MAX_SIZE, SignatureScheme,
    },
};

/// Parsed client hello information passed to [`CertificateProvider`].
///
/// All fields borrow from the raw ClientHello bytes to avoid allocations.
/// The `raw` field provides access to the complete ClientHello for
/// fingerprinting (e.g. JA4).
#[derive(Debug, Clone)]
pub struct ClientHello<'a> {
    /// Server Name Indication (SNI), if provided.
    pub server_name: Option<&'a str>,
    /// ALPN protocols offered by the client.
    pub alpn_protocols: &'a [Bytes],
    /// Cipher suites offered by the client.
    pub cipher_suites: &'a [CipherSuite],
    /// Key exchange negotiated (from the client's `key_share` extension).
    pub key_exchange_group: KeyExchangeGroup,
    /// Signature schemes offered by the client (from `signature_algorithms` extension).
    pub sig_schemes: &'a [SignatureScheme],
    /// The raw ClientHello bytes, for fingerprinting (e.g. JA4).
    pub raw: &'a [u8],
}

/// The server uses a [`CertificateProvider`] to produce its certificate (raw
/// public key) in response to the client's [`ClientHello`].
///
/// This is called *after* the server has selected the cipher suite and key
/// exchange group, so the provider can inspect the client's preferences.
#[async_trait]
pub trait CertificateProvider: Send + Sync {
    /// Return the raw public key certificate and signer.
    async fn provide(&self, client_hello: &ClientHello<'_>) -> Result<ProvidedCertificate, Error>;
}

#[async_trait]
impl<T: CertificateProvider + Send + Sync> CertificateProvider for Arc<T> {
    async fn provide(&self, client_hello: &ClientHello<'_>) -> Result<ProvidedCertificate, Error> {
        (**self).provide(client_hello).await
    }
}

/// The certificate (or raw public key) that the server sends to the client.
pub struct ProvidedCertificate {
    /// The signature scheme used for this certificate.
    pub scheme: SignatureScheme,
    /// The certificate payload.
    pub payload: RawPublicKeyCert,
}

/// The certificate data and the signer for the CertificateVerify handshake
/// message.
pub struct RawPublicKeyCert {
    /// The raw public key bytes (placed in the Certificate message).
    pub public_key: heapless::Vec<u8, MAX_PUBLIC_KEY_BYTES>,
    /// Signer for the CertificateVerify signature.
    pub signer: Box<dyn crate::crypto::Signer>,
}

/// A received certificate, either an X.509 chain or a raw public key.
///
/// The variant depends on the negotiated `server_certificate_type` extension.
/// If no extension was negotiated, TLS 1.3 defaults to X.509.
#[derive(Debug, Clone)]
pub enum ReceivedCertificate {
    /// An X.509 certificate chain (end-entity first, then intermediates).
    X509 {
        /// DER-encoded certificate chain.
        chain: Vec<Vec<u8>>,
        /// Signature scheme used for CertificateVerify.
        verify_scheme: SignatureScheme,
    },
    /// A raw public key (RFC 7250).
    RawPublicKey {
        /// Raw public key bytes (SubjectPublicKeyInfo DER).
        public_key: heapless::Vec<u8, MAX_PUBLIC_KEY_BYTES>,
        /// Signature scheme used for this certificate.
        scheme: SignatureScheme,
    },
}

/// The client uses a [`CertificateValidator`] to decide whether to trust a
/// received raw public key [`ReceivedCertificate`].
///
/// The validator also receives the SNI name (if any) and a verifier to check
/// the CertificateVerify signature.
#[async_trait]
pub trait CertificateValidator: Send + Sync {
    /// Validate the received certificate.
    ///
    /// Returns `Ok(())` if trusted, or an error describing why it was rejected.
    /// The `server_name` is the SNI the client requested (if any).
    async fn validate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error>;
}

#[async_trait]
impl<T: CertificateValidator + Send + Sync> CertificateValidator for Arc<T> {
    async fn validate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error> {
        (**self).validate(cert, server_name).await
    }
}

/// Fingerprints the raw ClientHello bytes (e.g. for JA4).
///
/// Receives the complete raw ClientHello message and returns a 64-byte
/// fingerprint. If an error is returned the connection is terminated.
#[async_trait]
pub trait TlsFingerprinter: Send + Sync {
    async fn fingerprint(&self, client_hello: &[u8]) -> Result<[u8; 64], Error>;
}

#[async_trait]
impl<T: TlsFingerprinter + Send + Sync> TlsFingerprinter for Arc<T> {
    async fn fingerprint(&self, client_hello: &[u8]) -> Result<[u8; 64], Error> {
        (**self).fingerprint(client_hello).await
    }
}

/// A no-op fingerprinter placeholder. It is never actually invoked because
/// the field is stored in `Option` and defaults to `None`.
#[derive(Clone, Copy, Debug)]
pub struct NoFingerprinter;

#[async_trait]
impl TlsFingerprinter for NoFingerprinter {
    async fn fingerprint(&self, _client_hello: &[u8]) -> Result<[u8; 64], Error> {
        unreachable!("NoFingerprinter is only a placeholder")
    }
}

/// Configuration for a TLS client connection.
///
/// Holds the crypto provider, the server name to connect to, ALPN protocols,
/// and a [`CertificateValidator`] that decides whether to trust the server's
/// raw public key.
#[derive(Clone)]
pub struct ClientConfig {
    /// The crypto provider.
    pub crypto: Arc<dyn CryptoProvider>,
    /// ALPN protocols to offer, in preference order.
    pub alpn_protocols: heapless::Vec<Bytes, 8>,
    /// Certificate types the client supports (sent via `server_certificate_type`
    /// extension). Defaults to `[CertType::X509]`. To enable raw public keys
    /// add `CertType::RawPublicKey` as well.
    pub cert_types: heapless::Vec<CertType, 2>,
    /// Validates the server's certificate / raw public key.
    pub cert_validator: Arc<dyn CertificateValidator>,
    /// Optional provider for a client certificate when the server requests
    /// one (mutual TLS).
    pub client_cert: Option<Arc<ClientCertificate>>,
    /// Optional session cache for PSK resumption.
    pub session_cache: Option<Arc<dyn ClientSessionCache>>,
}

/// A client certificate and its signer, used when the server sends a
/// `CertificateRequest` (mutual TLS / client authentication).
pub struct ClientCertificate {
    /// The certificate chain (X.509 DER, end-entity first).
    pub cert_chain: Vec<Vec<u8>>,
    /// Signer for the CertificateVerify message.
    pub signer: Box<dyn crate::crypto::Signer>,
}

impl ClientConfig {
    pub fn new(
        crypto_provider: Arc<dyn CryptoProvider>,
        alpn_protocols: heapless::Vec<Bytes, 8>,
        cert_validator: Arc<dyn CertificateValidator>,
    ) -> Self {
        Self {
            crypto: crypto_provider,
            alpn_protocols,
            cert_types: [CertType::X509].into(),
            cert_validator,
            client_cert: None,
            session_cache: None,
        }
    }

    /// Set the supported certificate types (sent via `server_certificate_type`
    /// extension). If only `[CertType::X509]` the extension is not sent.
    pub fn with_cert_types(mut self, cert_types: heapless::Vec<CertType, 2>) -> Self {
        self.cert_types = cert_types;
        self
    }

    /// Set the client certificate to present when the server requests one.
    pub fn with_client_cert(mut self, cert: Arc<ClientCertificate>) -> Self {
        self.client_cert = Some(cert);
        self
    }
}

/// Configuration for a TLS server connection.
///
/// Holds the crypto provider, ALPN protocols to accept, and a
/// [`CertificateProvider`] that produces the server's raw public key
/// certificate.
#[derive(Clone)]
pub struct ServerConfig {
    /// The crypto provider.
    pub provider: Arc<dyn CryptoProvider>,
    /// ALPN protocols the server is willing to select.
    pub alpn_protocols: heapless::Vec<Bytes, 8>,
    /// Produces the server's certificate (raw public key) on each connection.
    pub cert_provider: Arc<dyn CertificateProvider>,
    /// Whether to request a client certificate.
    pub require_client_auth: bool,
    /// Optional fingerprinter for the raw ClientHello (e.g. JA4).
    pub fingerprinter: Option<Arc<dyn TlsFingerprinter>>,
    /// Optional session ticket store for PSK resumption.
    pub session_tickets: Option<Arc<dyn SessionTicketStore>>,
}

impl ServerConfig {
    pub fn new(
        provider: Arc<dyn CryptoProvider>,
        alpn_protocols: heapless::Vec<Bytes, 8>,
        cert_provider: Arc<dyn CertificateProvider>,
    ) -> Self {
        Self {
            provider,
            alpn_protocols,
            cert_provider,
            require_client_auth: false,
            fingerprinter: None,
            session_tickets: None,
        }
    }
}

/// A session ticket store for TLS 1.3 PSK resumption.
///
/// The server stores a mapping from opaque ticket bytes to the PSK (derived
/// from the resumption_master_secret). On a subsequent connection the client
/// presents the ticket, the server looks up the PSK, and the handshake
/// proceeds with a PSK-based early secret.
#[async_trait]
pub trait SessionTicketStore: Send + Sync {
    /// Store a ticket and its associated PSK.
    async fn put_ticket(
        &self,
        server_name: &str,
        ticket: Vec<u8>,
        psk: heapless::Vec<u8, PSK_MAX_SIZE>,
        lifetime_s: u32,
    );
    /// Look up a PSK by ticket, if it exists.
    async fn get_psk(&self, ticket: &[u8]) -> Option<heapless::Vec<u8, PSK_MAX_SIZE>>;
    /// Remove a ticket (e.g. after failed resumption).
    async fn remove_ticket(&self, ticket: &[u8]);
}

/// A simple in-memory session ticket store backed by a `HashMap`.
///
/// Tickets expire after the configured lifetime. This is suitable for
/// single-process servers but NOT for multi-process or distributed
/// deployments.
///
/// Available only when the `std` feature is enabled.
#[cfg(feature = "std")]
pub struct InMemorySessionTicketStore {
    tickets: std::sync::Mutex<std::collections::HashMap<Vec<u8>, (heapless::Vec<u8, PSK_MAX_SIZE>, u64)>>,
    ticket_lifetime: u32,
}

#[cfg(feature = "std")]
impl InMemorySessionTicketStore {
    pub fn new(ticket_lifetime: u32) -> Self {
        Self {
            tickets: std::sync::Mutex::new(std::collections::HashMap::new()),
            ticket_lifetime,
        }
    }
}

#[cfg(feature = "std")]
#[async_trait]
impl SessionTicketStore for InMemorySessionTicketStore {
    async fn put_ticket(
        &self,
        _server_name: &str,
        ticket: Vec<u8>,
        psk: heapless::Vec<u8, PSK_MAX_SIZE>,
        _lifetime_s: u32,
    ) {
        let expiry = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .saturating_add(self.ticket_lifetime as u64);
        self.tickets.lock().unwrap().insert(ticket, (psk, expiry));
    }

    async fn get_psk(&self, ticket: &[u8]) -> Option<heapless::Vec<u8, PSK_MAX_SIZE>> {
        let mut map = self.tickets.lock().unwrap();
        let (psk, expiry) = map.get(ticket)?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if now > *expiry {
            map.remove(ticket);
            return None;
        }
        Some(psk.clone())
    }

    async fn remove_ticket(&self, ticket: &[u8]) {
        self.tickets.lock().unwrap().remove(ticket);
    }
}

/// A wall clock used to check certificate validity periods.
///
/// Returns the current Unix timestamp (seconds since epoch). This trait
/// exists so that no_std environments can inject their own time source
/// (hardware RTC, NTP, etc.) instead of relying on `std::time::SystemTime`.
pub trait Clock: Send + Sync {
    fn now(&self) -> u64;
}

/// A [`Clock`] backed by `std::time::SystemTime`.
///
/// Available only when the `std` feature is enabled.
#[cfg(feature = "std")]
pub struct SystemClock;

#[cfg(feature = "std")]
impl Clock for SystemClock {
    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// A client-side session cache for PSK resumption.
///
/// Stores the ticket and PSK received from a server so that a subsequent
/// connection can resume the session with a PSK-based ClientHello.
#[async_trait]
pub trait ClientSessionCache: Send + Sync {
    /// Store a ticket and PSK for a given server.
    async fn put(&self, server_name: &str, ticket: Vec<u8>, psk: Vec<u8>);
    /// Retrieve the (ticket, PSK) for a given server, if available.
    async fn get(&self, server_name: &str) -> Option<(Vec<u8>, Vec<u8>)>;
    /// Remove a cached entry (e.g. after failed resumption).
    async fn remove(&self, server_name: &str);
}
