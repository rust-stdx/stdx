use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use bytes::Bytes;

#[cfg(feature = "http2")]
use super::conn::http2::Http2Conn;
#[cfg(feature = "http3")]
use super::conn::http3::Http3Conn;
use super::{
    conn::{ConnRef, http1::Http1Conn},
    error::Error,
};
use crate::common::Uri;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PoolKey {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

impl PoolKey {
    pub fn from_uri(uri: &Uri) -> Option<Self> {
        let scheme = uri.scheme()?;
        let host = uri.host()?.to_string();
        let tls = scheme == "https";
        let port = uri.port().unwrap_or(if tls { 443 } else { 80 });
        Some(PoolKey {
            host,
            port,
            tls,
        })
    }
}

pub(crate) struct PoolInner {
    pub http1: HashMap<PoolKey, VecDeque<Http1Conn>>,
    #[cfg(feature = "http2")]
    pub http2: HashMap<PoolKey, (Arc<Http2Conn>, usize)>,
    #[cfg(feature = "http3")]
    pub http3: HashMap<PoolKey, (Arc<tokio::sync::Mutex<Http3Conn>>, usize)>,
    pub max_idle_per_host: usize,
    pub closed: bool,
}

pub(crate) struct ConnectionPool {
    pub inner: tokio::sync::Mutex<PoolInner>,
    _idle_timeout: Duration,
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self {
            inner: tokio::sync::Mutex::new(PoolInner {
                http1: HashMap::new(),
                #[cfg(feature = "http2")]
                http2: HashMap::new(),
                #[cfg(feature = "http3")]
                http3: HashMap::new(),
                max_idle_per_host: 5,
                closed: false,
            }),
            _idle_timeout: Duration::from_secs(90),
        }
    }
}

impl ConnectionPool {
    pub fn new(max_idle_per_host: usize, idle_timeout: Duration) -> Self {
        Self {
            inner: tokio::sync::Mutex::new(PoolInner {
                http1: HashMap::new(),
                #[cfg(feature = "http2")]
                http2: HashMap::new(),
                #[cfg(feature = "http3")]
                http3: HashMap::new(),
                max_idle_per_host,
                closed: false,
            }),
            _idle_timeout: idle_timeout,
        }
    }

    pub async fn get(&self, uri: &Uri, alpn: Vec<Bytes>) -> Result<ConnRef, Error> {
        let key = PoolKey::from_uri(uri).ok_or_else(|| Error::UnsupportedScheme("missing scheme or host".into()))?;

        match uri.scheme() {
            Some("http") => self.get_http1(&key).await,
            Some("https") => self.get_https(&key, alpn).await,
            _ => Err(Error::UnsupportedScheme(uri.scheme().unwrap_or("unknown").into())),
        }
    }

    pub async fn put(&self, conn: ConnRef) {
        let key = conn.pool_key().clone();
        if !conn.can_reuse() {
            return;
        }
        let mut inner = self.inner.lock().await;
        match conn {
            ConnRef::Http1(c) => {
                let max_idle = inner.max_idle_per_host;
                let list = inner.http1.entry(key).or_default();
                if list.len() < max_idle {
                    list.push_back(c);
                }
            }
            #[cfg(feature = "http2")]
            ConnRef::Http2(_arc) => {
                if let Some(entry) = inner.http2.get_mut(&key) {
                    entry.1 = entry.1.saturating_sub(1);
                }
            }
            #[cfg(feature = "http3")]
            ConnRef::Http3(_key, _arc) => {
                if let Some(entry) = inner.http3.get_mut(&key) {
                    entry.1 = entry.1.saturating_sub(1);
                }
            }
        }
    }

    /// Evict all HTTP/1.x and HTTP/2 pooled connections for `key`.
    /// Used after a successful HTTP/3 migration.
    pub async fn evict_http1_http2(&self, key: &PoolKey) {
        let mut inner = self.inner.lock().await;
        inner.http1.remove(key);
        #[cfg(feature = "http2")]
        inner.http2.remove(key);
    }

    async fn get_http1(&self, key: &PoolKey) -> Result<ConnRef, Error> {
        let mut inner = self.inner.lock().await;
        if inner.closed {
            return Err(Error::PoolClosed);
        }
        if let Some(conn) = inner.http1.get_mut(key).and_then(|v| v.pop_front()) {
            return Ok(ConnRef::Http1(conn));
        }
        drop(inner);

        let conn = Http1Conn::connect_tcp(&key.host, key.port).await?;
        Ok(ConnRef::Http1(conn))
    }

    async fn get_https(&self, key: &PoolKey, alpn: Vec<Bytes>) -> Result<ConnRef, Error> {
        #[cfg(feature = "http2")]
        {
            let mut inner = self.inner.lock().await;
            if inner.closed {
                return Err(Error::PoolClosed);
            }
            if let Some(entry) = inner.http2.get_mut(key) {
                let conn = entry.0.clone();
                entry.1 += 1;
                return Ok(ConnRef::Http2(conn));
            }
        }

        connect_https_fresh(&self.inner, key, alpn).await
    }

    pub async fn close(&self) {
        let mut inner = self.inner.lock().await;
        inner.closed = true;
        inner.http1.clear();
        #[cfg(feature = "http2")]
        inner.http2.clear();
        #[cfg(feature = "http3")]
        inner.http3.clear();
    }
}

#[cfg(feature = "tls")]
async fn connect_https_fresh(
    _inner: &tokio::sync::Mutex<PoolInner>,
    key: &PoolKey,
    alpn: Vec<Bytes>,
) -> Result<ConnRef, Error> {
    let mut conn = Http1Conn::connect_tls(&key.host, key.port, alpn).await?;

    #[cfg(feature = "http2")]
    if let Some(stream) = conn.take_tls_stream() {
        let h2 = Http2Conn::from_stream(&key.host, key.port, stream).await?;
        let arc = Arc::new(h2);
        let mut inner = _inner.lock().await;
        if inner.closed {
            return Err(Error::PoolClosed);
        }
        inner.http2.entry(key.clone()).or_insert((arc.clone(), 0)).1 += 1;
        return Ok(ConnRef::Http2(arc));
    }

    Ok(ConnRef::Http1(conn))
}

#[cfg(not(feature = "tls"))]
async fn connect_https_fresh(
    _inner: &tokio::sync::Mutex<PoolInner>,
    _key: &PoolKey,
    _alpn: Vec<Bytes>,
) -> Result<ConnRef, Error> {
    Err(Error::UnsupportedScheme("https requires the `tls` feature".into()))
}
