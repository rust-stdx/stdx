//! The [`Client`] and its configuration.

use core::{fmt, time::Duration};
use std::sync::Arc;

use zeroize::Zeroizing;

use crate::{
    error::Error,
    http::{ErasedHttpClient, HttpClient},
};

/// The Stripe API version this crate was generated against.
///
/// Sent as the `Stripe-Version` header by default so that responses keep their
/// documented shape. Override it with [`Client::with_api_version`], or disable
/// it with [`Client::without_api_version`] to use the account's default version.
pub const DEFAULT_API_VERSION: &str = "2026-09-30.endive";

/// The default base URL for the Stripe API.
pub const DEFAULT_BASE_URL: &str = "https://api.stripe.com";

/// The default maximum response body size: 32 MiB.
///
/// Caps memory use when a server (or something impersonating one) returns an
/// enormous body.
pub const DEFAULT_MAX_RESPONSE_SIZE: usize = 32 * 1024 * 1024;

/// The default number of additional attempts after the first one.
///
/// Applies to requests that are safe to retry: `GET`/`DELETE`, and `POST`
/// requests for which the client attaches an `Idempotency-Key`.
pub const DEFAULT_MAX_RETRIES: u32 = 2;

/// Per-request options.
///
/// Pass these with the `*_with_options` request methods to override behaviour
/// for a single request.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RequestOptions {
    /// An explicit `Idempotency-Key`.
    ///
    /// When set on a `POST` or `DELETE`, the value is sent verbatim and reused
    /// unchanged across retries. Use it to make your own retries safe: generate
    /// one key per logical operation (a UUID is ideal) and pass the same key for
    /// every attempt within Stripe's 24-hour window.
    ///
    /// It must be at most 255 characters and contain no control characters, or
    /// the request fails with [`Error::Config`] before anything is sent.
    pub idempotency_key: Option<String>,
}

impl RequestOptions {
    /// Creates empty request options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an explicit idempotency key.
    pub fn with_idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    /// Returns the configured idempotency key, if any.
    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }
}

/// An asynchronous Stripe API client.
///
/// The client is cheap to share: it holds an [`HttpClient`] (whose default
/// `reqwest` implementation pools connections) and immutable configuration.
/// Every method takes `&self`, so one `Client` can be shared across tasks and
/// threads.
///
/// # Default client
///
/// With the default `reqwest` feature, [`Client::new`] builds a client using the
/// bundled HTTP transport, which pools connections and installs the stdx rustls
/// provider. Enable the feature from Cargo with the default feature set:
///
/// ```toml
/// [dependencies]
/// stripe = { git = "https://github.com/rust-stdx/stdx", features = ["reqwest"] }
/// ```
///
/// # Custom transport
///
/// Pass any [`HttpClient`] implementation to [`Client::with_http_client`]:
///
/// ```no_run
/// # use stripe::{Client, HttpClient, HttpRequest, HttpResponseData, HttpError};
/// # struct MyHttp;
/// # impl HttpClient for MyHttp {
/// #     async fn send(&self, _request: HttpRequest) -> Result<HttpResponseData, HttpError> { todo!() }
/// # }
/// # async fn example() -> Result<(), stripe::Error> {
/// let client = Client::with_http_client("sk_test_...", MyHttp)?;
/// # let _ = client;
/// # Ok(())
/// # }
/// ```
pub struct Client {
    pub(crate) http: Arc<dyn ErasedHttpClient>,
    pub(crate) api_key: Zeroizing<String>,
    pub(crate) base_url: String,
    pub(crate) api_version: Option<String>,
    pub(crate) account: Option<String>,
    pub(crate) user_agent: Option<String>,
    pub(crate) max_retries: u32,
    pub(crate) max_response_size: usize,
    pub(crate) timeout: Option<Duration>,
    pub(crate) retry_base_delay: Duration,
    pub(crate) retry_max_delay: Duration,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Never render the API key.
        f.debug_struct("Client")
            .field("base_url", &self.base_url)
            .field("api_version", &self.api_version)
            .field("account", &self.account)
            .field("max_retries", &self.max_retries)
            .field("max_response_size", &self.max_response_size)
            .field("timeout", &self.timeout)
            .field("api_key", &"<redacted>")
            .finish()
    }
}

impl Client {
    /// Builds a client around a custom [`HttpClient`].
    ///
    /// The transport is stored behind a trait object, so `Client` itself has no
    /// type parameters and every generated method accepts `&Client`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] if `api_key` is empty or contains control
    /// characters.
    pub fn with_http_client(api_key: impl Into<String>, http: impl HttpClient + 'static) -> Result<Self, Error> {
        Self::with_arc_http_client(api_key, Arc::new(http))
    }

    /// Builds a client around a shared [`HttpClient`].
    ///
    /// Use this to share one transport (and its connection pool) between
    /// several clients.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] if `api_key` is empty or contains control
    /// characters.
    pub fn with_arc_http_client(
        api_key: impl Into<String>,
        http: Arc<impl HttpClient + 'static>,
    ) -> Result<Self, Error> {
        let api_key = api_key.into();
        if api_key.trim().is_empty() {
            return Err(Error::Config("api key must not be empty".into()));
        }
        if api_key.bytes().any(|byte: u8| byte.is_ascii_control()) {
            return Err(Error::Config("api key contains control characters".into()));
        }

        Ok(Self {
            http,
            api_key: Zeroizing::new(api_key),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_version: Some(DEFAULT_API_VERSION.to_string()),
            account: None,
            user_agent: None,
            max_retries: DEFAULT_MAX_RETRIES,
            max_response_size: DEFAULT_MAX_RESPONSE_SIZE,
            timeout: Some(Duration::from_secs(30)),
            retry_base_delay: Duration::from_millis(500),
            retry_max_delay: Duration::from_secs(8),
        })
    }

    /// Sets the base URL, for example to target a test double.
    ///
    /// The value is used verbatim, with a trailing `/` trimmed, so it must be an
    /// absolute URL the transport accepts (`https://api.stripe.test`, or an
    /// application's own name for the API). It is not validated here; a transport
    /// that cannot use it reports the problem when the first request is sent.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Pins the `Stripe-Version` header.
    pub fn with_api_version(mut self, api_version: impl Into<String>) -> Self {
        self.api_version = Some(api_version.into());
        self
    }

    /// Stops sending a `Stripe-Version` header, so Stripe uses the account's
    /// default API version.
    pub fn without_api_version(mut self) -> Self {
        self.api_version = None;
        self
    }

    /// Sends the `Stripe-Account` header, for Connect platforms.
    pub fn with_account(mut self, account: impl Into<String>) -> Self {
        self.account = Some(account.into());
        self
    }

    /// Sets the value of the `User-Agent` header.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sets the number of additional attempts after the first one.
    ///
    /// `0` disables retries. Retries are only performed for requests that can
    /// be repeated safely (see [`Client`]), and a `Retry-After` header from
    /// Stripe takes precedence over the backoff; when it asks for a longer wait
    /// than the client is willing to take (see [`Client::with_retry_backoff`]),
    /// the request fails instead of retrying too early.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the maximum response body size, in bytes.
    pub fn with_max_response_size(mut self, max_response_size: usize) -> Self {
        self.max_response_size = max_response_size;
        self
    }

    /// Sets the per-request timeout. `None` disables the timeout.
    pub fn with_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the retry backoff bounds.
    ///
    /// Without a `Retry-After` header the backoff grows exponentially from
    /// `base`, doubles per attempt, and is capped at `max`. When Stripe sends a
    /// `Retry-After` of at most 60 seconds the client waits exactly that long;
    /// a longer one means the caller should decide when to try again, so the
    /// request fails rather than retrying earlier than the server asked. Useful
    /// for keeping tests fast.
    pub fn with_retry_backoff(mut self, base: Duration, max: Duration) -> Self {
        self.retry_base_delay = base;
        self.retry_max_delay = max;
        self
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the pinned API version, if any.
    pub fn api_version(&self) -> Option<&str> {
        self.api_version.as_deref()
    }

    /// Returns the configured maximum number of retries.
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

#[cfg(feature = "reqwest")]
impl Client {
    /// Builds a client using the default `reqwest`-based [`HttpClient`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] if `api_key` is empty or contains control
    /// characters.
    pub fn new(api_key: impl Into<String>) -> Result<Self, Error> {
        Self::with_http_client(api_key, crate::http::ReqwestHttpClient::new())
    }
}
