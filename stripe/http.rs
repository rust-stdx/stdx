//! The transport abstraction used by [`Client`](crate::Client).

use core::{fmt, pin::Pin, time::Duration};

use bytes::Bytes;
use futures_util::Stream;

use crate::error::HttpError;

/// A pinned, boxed async stream of response body chunks.
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, HttpError>> + Send>>;

/// Builds a [`ByteStream`] that yields a single chunk.
///
/// Handy for implementing [`HttpClient`] in tests and application code.
pub fn byte_stream(body: impl Into<Bytes>) -> ByteStream {
    let bytes: Bytes = body.into();
    Box::pin(futures_util::stream::once(async move { Ok(bytes) }))
}

/// Builds a [`ByteStream`] from an iterator of chunks.
pub fn chunk_stream<I, B>(chunks: I) -> ByteStream
where
    I: IntoIterator<Item = B> + Send + 'static,
    B: Into<Bytes>,
{
    let chunks: Vec<Result<Bytes, HttpError>> = chunks.into_iter().map(|c| Ok(c.into())).collect();
    Box::pin(futures_util::stream::iter(chunks))
}

/// The HTTP methods used by the Stripe API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    /// `GET`
    Get,
    /// `POST`
    Post,
    /// `DELETE`
    Delete,
}

impl HttpMethod {
    /// Returns the uppercase method name.
    pub fn as_str(self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Delete => "DELETE",
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A fully-formed HTTP request, waiting to be performed by an [`HttpClient`].
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// The HTTP method.
    pub method: HttpMethod,
    /// The absolute request URL, including any query string.
    pub url: String,
    /// Header name/value pairs. Names are validated by the caller.
    pub headers: Vec<(String, String)>,
    /// The request body. Empty for `GET`.
    ///
    /// `Bytes` is a reference-counted buffer, so the client can hand the same
    /// body to a retry without copying it.
    pub body: Bytes,
    /// Optional request timeout, applied by the transport when supported.
    pub timeout: Option<Duration>,
}

/// A response returned by an [`HttpClient`].
pub struct HttpResponseData {
    /// The HTTP status code.
    pub status_code: u16,
    /// Response header name/value pairs.
    pub headers: Vec<(String, String)>,
    /// The streaming response body.
    pub body: ByteStream,
}

impl HttpResponseData {
    /// Returns the first header whose name matches `name`, case-insensitively.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

/// Performs HTTP requests on behalf of a [`Client`](crate::Client).
///
/// Implement this trait to use the client with a transport other than the
/// default `reqwest`-based one (for example a shared client, a mock, or an
/// in-house HTTP stack). Implementations must be usable from multiple tasks
/// concurrently, which is why the trait requires `Send + Sync`.
///
/// A [`Client`](crate::Client) stores the implementation behind a trait object,
/// so one client type works with every transport and the generated methods stay
/// free of transport type parameters.
pub trait HttpClient: Send + Sync {
    /// Sends `request` and returns the response headers and body stream.
    ///
    /// Returning an error leaves the outcome of the request indeterminate.
    fn send(&self, request: HttpRequest) -> impl Future<Output = Result<HttpResponseData, HttpError>> + Send;
}

/// Object-safe view of [`HttpClient`], used by [`Client`](crate::Client).
///
/// `HttpClient::send` returns `impl Future`, which object safety forbids, so the
/// client stores this erased form and the blanket implementation below boxes the
/// future. Implementations never have to deal with it.
pub(crate) trait ErasedHttpClient: Send + Sync {
    /// Sends `request`, boxing the future so the call works through `dyn`.
    fn send_erased<'a>(
        &'a self,
        request: HttpRequest,
    ) -> futures_util::future::BoxFuture<'a, Result<HttpResponseData, HttpError>>;
}

impl<H: HttpClient> ErasedHttpClient for H {
    fn send_erased<'a>(
        &'a self,
        request: HttpRequest,
    ) -> futures_util::future::BoxFuture<'a, Result<HttpResponseData, HttpError>> {
        Box::pin(self.send(request))
    }
}

#[cfg(feature = "reqwest")]
mod reqwest_client {
    use futures_util::StreamExt as _;

    use super::{ByteStream, HttpClient, HttpRequest, HttpResponseData};
    use crate::error::HttpError;

    /// The default [`HttpClient`], backed by [`reqwest`].
    ///
    /// Every clone shares the same connection pool, so a single instance should
    /// be built once and reused for the lifetime of the application.
    #[derive(Debug, Clone)]
    pub struct ReqwestHttpClient {
        inner: reqwest::Client,
    }

    impl Default for ReqwestHttpClient {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ReqwestHttpClient {
        /// Builds a client with default settings.
        ///
        /// Installs the stdx rustls [`CryptoProvider`](crypto_rustls) as the
        /// process default (if none is installed yet) before creating the
        /// underlying `reqwest` client: `reqwest` is compiled with the
        /// `rustls-no-provider` feature, so without a provider the first
        /// request would fail.
        pub fn new() -> Self {
            crypto_rustls::install_default_rustls_provider();
            Self {
                inner: reqwest::Client::new(),
            }
        }

        /// Builds a client from a pre-configured [`reqwest::Client`].
        ///
        /// Use this to set connection-pool size, proxies, TCP keepalive, or
        /// redirect policy. The stdx rustls provider is installed first, as in
        /// [`ReqwestHttpClient::new`].
        pub fn with_reqwest_client(inner: reqwest::Client) -> Self {
            crypto_rustls::install_default_rustls_provider();
            Self {
                inner,
            }
        }

        /// Returns a [`reqwest::ClientBuilder`] with the stdx rustls provider
        /// installed, for building a custom underlying client.
        pub fn builder() -> reqwest::ClientBuilder {
            crypto_rustls::install_default_rustls_provider();
            reqwest::Client::builder()
        }
    }

    impl HttpClient for ReqwestHttpClient {
        async fn send(&self, request: HttpRequest) -> Result<HttpResponseData, HttpError> {
            let method = reqwest::Method::from_bytes(request.method.as_str().as_bytes())?;
            let mut builder = self.inner.request(method, &request.url);
            for (name, value) in &request.headers {
                builder = builder.header(name, value);
            }
            if let Some(timeout) = request.timeout {
                builder = builder.timeout(timeout);
            }
            if !request.body.is_empty() {
                builder = builder.body(request.body);
            }

            let response = builder.send().await?;
            let status_code = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .filter_map(|(name, value)| {
                    value
                        .to_str()
                        .ok()
                        .map(|value| (name.as_str().to_string(), value.to_string()))
                })
                .collect();

            let body: ByteStream = Box::pin(
                response
                    .bytes_stream()
                    .map(|result| result.map_err(|err| -> HttpError { Box::new(err) })),
            );

            Ok(HttpResponseData {
                status_code,
                headers,
                body,
            })
        }
    }
}

#[cfg(feature = "reqwest")]
pub use reqwest_client::ReqwestHttpClient;
