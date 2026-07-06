mod alt_svc;
mod conn;
mod error;
mod pool;

use std::{sync::Arc, time::Duration};

use alt_svc::{AltSvcCache, parse_alt_svc_for_h3};
use bytes::Bytes;
use error::Error;
use pool::ConnectionPool;
use tokio::sync::Mutex;

use crate::common::{Request, Response};

/// A high-level HTTP client with connection pooling and automatic
/// protocol negotiation.
///
/// Supports HTTP/1.1 over plain TCP, HTTP/1.1 over TLS, HTTP/2
/// over TLS (ALPN-negotiated), and HTTP/3 via Alt-Svc discovery.
///
/// Connection pooling is automatic per (host, port, tls) tuple.
/// When a response carries an `Alt-Svc: h3=...` header, the client
/// will attempt HTTP/3 on subsequent requests to the same host.
///
/// # Example
///
/// ```ignore
/// use http::Client;
/// use http::common::{Method, Request, Uri};
///
/// let client = Client::new();
/// let req = Request::new(Method::Get, Uri::parse("http://example.com/").unwrap(), None);
/// let resp = client.send(req).await.unwrap();
/// println!("{}", resp.status);
/// ```
pub struct Client {
    pool: ConnectionPool,
    alt_svc_cache: Arc<Mutex<AltSvcCache>>,
}

/// Builder for configuring a [`Client`].
pub struct ClientBuilder {
    max_idle_per_host: usize,
    idle_timeout: Duration,
    enable_alt_svc: bool,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        ClientBuilder {
            max_idle_per_host: 5,
            idle_timeout: Duration::from_secs(90),
            enable_alt_svc: true,
        }
    }

    /// Maximum number of idle connections to keep per (host, port, tls) key.
    pub fn pool_max_idle(mut self, n: usize) -> Self {
        self.max_idle_per_host = n;
        self
    }

    /// Duration after which an idle connection is closed.
    pub fn pool_idle_timeout(mut self, d: Duration) -> Self {
        self.idle_timeout = d;
        self
    }

    /// Enable or disable automatic Alt-Svc / HTTP/3 discovery.
    pub fn alt_svc(mut self, enable: bool) -> Self {
        self.enable_alt_svc = enable;
        self
    }

    /// Build the [`Client`].
    pub fn build(self) -> Client {
        Client {
            pool: ConnectionPool::new(self.max_idle_per_host, self.idle_timeout),
            alt_svc_cache: Arc::new(Mutex::new(AltSvcCache::new())),
        }
    }
}

impl Client {
    /// Create a new `Client` with default configuration.
    pub fn new() -> Self {
        ClientBuilder::new().build()
    }

    /// Send an HTTP request and return the response.
    ///
    /// The URI scheme determines the transport:
    /// - `http://` — plain TCP, HTTP/1.1
    /// - `https://` — TLS with ALPN (`h2` preferred, falls back to `http/1.1`)
    ///
    /// If the server advertises HTTP/3 via `Alt-Svc`, subsequent requests
    /// will attempt HTTP/3 first, falling back to HTTP/1.1 or HTTP/2
    /// on failure.
    pub async fn send(&self, req: Request) -> Result<Response, Error> {
        let alpn = build_alpn();

        let host = req.uri.host().unwrap_or("").to_string();
        let port = req.uri.port().unwrap_or(443);
        let authority = format!("{host}:{port}");

        // Try HTTP/3 if cached via Alt-Svc
        #[cfg(feature = "http3")]
        if let Some(h3_authority) = {
            let cache = self.alt_svc_cache.lock().await;
            cache.get(&authority).map(|a| a.to_string())
        } {
            match self.try_send_h3(&req, &h3_authority).await {
                Ok(resp) => {
                    self.pool
                        .evict_http1_http2(&PoolKey {
                            host: host.clone(),
                            port,
                            tls: true,
                        })
                        .await;
                    return Ok(resp);
                }
                Err(_) => {
                    self.alt_svc_cache.lock().await.remove(&authority);
                }
            }
        }

        // Standard HTTP/1 or HTTP/2 path
        let mut conn = self.pool.get(&req.uri, alpn).await?;
        let resp = conn.send(req).await?;
        self.pool.put(conn).await;

        // Check for Alt-Svc in the response (first request discovery)
        let svc_entry = parse_alt_svc_for_h3(&resp, &host, port);
        if let Some((alt_authority, max_age)) = svc_entry {
            if alt_authority == "__clear__" {
                self.alt_svc_cache.lock().await.clear();
            } else {
                self.alt_svc_cache
                    .lock()
                    .await
                    .insert(authority, alt_authority, max_age);
            }
        }

        Ok(resp)
    }

    /// Try sending via HTTP/3 to an Alt-Svc-discovered endpoint.
    #[cfg(feature = "http3")]
    async fn try_send_h3(&self, req: &Request, alt_authority: &str) -> Result<Response, Error> {
        use conn::http3::Http3Conn;
        let host = req.uri.host().unwrap_or("");
        let default_port = req.uri.port().unwrap_or(443);
        let (h3_host, h3_port) = split_alt_authority(alt_authority);

        // Build a new key for the H3 connection (same host info for the pool)
        let h3_key = PoolKey {
            host: host.to_string(),
            port: default_port,
            tls: true,
        };

        // Check if we already have an H3 connection in the pool
        {
            let guard = self.pool.inner.lock().await;
            if let Some(entry) = guard.http3.get(&h3_key) {
                let conn = entry.0.clone();
                drop(guard);
                let mut guard = conn.lock().await;
                return guard.send_request(req.clone()).await;
            }
        }

        // Create new H3 connection
        let h3 = Http3Conn::connect(h3_host, h3_port).await?;
        let arc = Arc::new(tokio::sync::Mutex::new(h3));

        // Store in pool
        {
            let mut guard = self.pool.inner.lock().await;
            if !guard.closed {
                guard.http3.entry(h3_key).or_insert((arc.clone(), 0)).1 += 1;
            }
        }

        let mut guard = arc.lock().await;
        guard.send_request(req.clone()).await
    }

    /// Close all idle connections in the pool and clear the Alt-Svc cache.
    pub async fn close(&self) {
        self.pool.close().await;
        self.alt_svc_cache.lock().await.clear();
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn build_alpn() -> Vec<Bytes> {
    let alpn = vec![Bytes::from_static(b"http/1.1")];
    #[cfg(feature = "http2")]
    alpn.insert(0, Bytes::from_static(b"h2"));
    alpn
}
