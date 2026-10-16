//! Error types for the Stripe client.

use core::fmt;

use serde::Deserialize;

/// A transport-level error returned by an [`HttpClient`](crate::HttpClient).
pub type HttpError = Box<dyn core::error::Error + Send + Sync>;

/// The `error` object returned by the Stripe API on a failed request.
///
/// Every field is optional and stored as a plain string on purpose: Stripe adds
/// new error `type`s, `code`s and `decline_code`s over time, and a request must
/// never fail to *report its failure* because the client did not know a value.
/// Unknown fields are ignored, and unknown values are preserved verbatim in the
/// corresponding `String`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct ApiErrorBody {
    /// The kind of error: `api_error`, `card_error`, `invalid_request_error`,
    /// `idempotency_error`, or a newer value.
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
    /// A machine-readable error code, for example `card_declined`.
    ///
    /// Codes are not an enum here: Stripe introduces new codes regularly and
    /// clients must keep working when they do.
    #[serde(default)]
    pub code: Option<String>,
    /// For card errors, the specific decline code returned by the issuer.
    #[serde(default)]
    pub decline_code: Option<String>,
    /// A human-readable message describing the error.
    #[serde(default)]
    pub message: Option<String>,
    /// The parameter that caused the error, when the error is tied to one.
    #[serde(default)]
    pub param: Option<String>,
    /// A link to the API reference for the error.
    #[serde(default)]
    pub doc_url: Option<String>,
    /// The ID of the failed charge, for card errors.
    #[serde(default)]
    pub charge: Option<String>,
    /// The ID of the PaymentIntent, when the error is tied to one.
    #[serde(default)]
    pub payment_intent: Option<String>,
    /// The ID of the SetupIntent, when the error is tied to one.
    #[serde(default)]
    pub setup_intent: Option<String>,
    /// The ID of the PaymentMethod, when the error is tied to one.
    #[serde(default)]
    pub payment_method: Option<String>,
    /// The ID of the Source, when the error is tied to one.
    #[serde(default)]
    pub source: Option<String>,
    /// A URL to the request in the Stripe dashboard.
    #[serde(default)]
    pub request_log_url: Option<String>,
}

impl ApiErrorBody {
    /// Returns the error `type` (`api_error`, `card_error`, ...), if present.
    pub fn error_type(&self) -> Option<&str> {
        self.type_.as_deref()
    }

    /// Returns the machine-readable error code, if present.
    pub fn error_code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns true when Stripe reported a card error (`card_error`).
    pub fn is_card_error(&self) -> bool {
        self.type_.as_deref() == Some("card_error")
    }
}

/// Metadata carried by an error response, sourced from the HTTP response.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApiResponseMeta {
    /// The HTTP status code.
    pub status: u16,
    /// Value of the `Request-Id` header, useful when contacting Stripe support.
    pub request_id: Option<String>,
    /// Value of `Stripe-Should-Retry`, when Stripe sent one.
    ///
    /// `Some(true)` means the request may be retried with backoff;
    /// `Some(false)` means it must not be retried.
    pub should_retry: Option<bool>,
    /// Number of seconds from `Retry-After`, when Stripe sent one.
    pub retry_after: Option<u64>,
}

/// Errors returned by the Stripe client.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The client or a header value was configured incorrectly. The request was
    /// never sent.
    Config(String),
    /// The request could not be encoded. The request was never sent.
    InvalidRequest(String),
    /// The endpoint needs a request encoding this client does not implement,
    /// such as `multipart/form-data`. Nothing was sent.
    UnsupportedEncoding {
        /// The path template of the endpoint.
        endpoint: &'static str,
        /// The content type the endpoint requires.
        encoding: &'static str,
    },
    /// The transport failed (DNS, TLS, connection reset, timeout, ...).
    ///
    /// A failed transport leaves the outcome of a mutating request
    /// **indeterminate**: it may or may not have been executed.
    Http(HttpError),
    /// The response body exceeded the configured
    /// [`max_response_size`](crate::Client::with_max_response_size).
    ResponseTooLarge {
        /// The configured limit, in bytes.
        limit: usize,
    },
    /// Stripe returned a non-2xx response.
    Api {
        /// HTTP status and response headers.
        meta: ApiResponseMeta,
        /// The parsed `error` object.
        error: ApiErrorBody,
        /// The raw response body, retained for errors that do not follow the
        /// documented error shape (for example a proxy's HTML error page).
        raw_body: String,
    },
    /// A 2xx response could not be decoded into the expected type.
    Decode {
        /// The HTTP status code of the response.
        status: u16,
        /// The `Request-Id` header, when present.
        request_id: Option<String>,
        /// The underlying decoding error.
        source: serde_json::Error,
    },
    /// A webhook signature could not be verified. See [`WebhookError`].
    Webhook(WebhookError),
}

impl Error {
    /// Returns the response metadata for [`Error::Api`] errors.
    pub fn api_meta(&self) -> Option<&ApiResponseMeta> {
        match self {
            Error::Api {
                meta, ..
            } => Some(meta),
            _ => None,
        }
    }

    /// Returns the parsed Stripe error object for [`Error::Api`] errors.
    pub fn api_error(&self) -> Option<&ApiErrorBody> {
        match self {
            Error::Api {
                error, ..
            } => Some(error),
            _ => None,
        }
    }

    /// Returns true when this error can be retried safely by the client.
    ///
    /// Lock timeouts are reported by Stripe as `429` responses *without* the
    /// rate-limit headers; like rate limits they should be retried, so this
    /// treats every retryable status the same way.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Http(_) => true,
            Error::Api {
                meta,
                error,
                ..
            } => {
                if let Some(should_retry) = meta.should_retry {
                    return should_retry;
                }
                matches!(meta.status, 409 | 429 | 500 | 502 | 503 | 504)
                    || error.code.as_deref().is_some_and(|code| code == "lock_timeout")
            }
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(msg) => write!(f, "invalid client configuration: {msg}"),
            Error::InvalidRequest(msg) => write!(f, "invalid request: {msg}"),
            Error::UnsupportedEncoding {
                endpoint,
                encoding,
            } => write!(
                f,
                "endpoint `{endpoint}` requires a `{encoding}` request body, which this version of the client does not encode"
            ),
            Error::Http(err) => write!(f, "http error: {err}"),
            Error::ResponseTooLarge {
                limit,
            } => write!(f, "response body exceeded the {limit} byte limit"),
            Error::Api {
                meta,
                error,
                raw_body,
            } => {
                let kind = error.type_.as_deref().unwrap_or("error");
                let message = error
                    .message
                    .as_deref()
                    .or_else(|| {
                        if raw_body.is_empty() {
                            None
                        } else {
                            Some(raw_body.as_str())
                        }
                    })
                    .unwrap_or("no message");
                write!(f, "stripe api error ({} {}): {}", meta.status, kind, message)?;
                if let Some(request_id) = &meta.request_id {
                    write!(f, " [request_id: {request_id}]")?;
                }
                Ok(())
            }
            Error::Decode {
                status,
                request_id,
                source,
            } => {
                write!(f, "failed to decode stripe response (status {status}): {source}")?;
                if let Some(request_id) = request_id {
                    write!(f, " [request_id: {request_id}]")?;
                }
                Ok(())
            }
            Error::Webhook(err) => write!(f, "webhook error: {err}"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Http(err) => Some(&**err),
            Error::Decode {
                source, ..
            } => Some(source),
            Error::Webhook(err) => Some(err),
            _ => None,
        }
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Error::Http(err)
    }
}

/// Why a `Stripe-Signature` header was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum WebhookError {
    /// No `Stripe-Signature` header value was provided.
    MissingHeader,
    /// No webhook signing secret was provided.
    MissingSecret,
    /// The header did not contain a parseable `t=` timestamp.
    InvalidTimestamp,
    /// The header contained no signature for the expected `v1` scheme.
    NoSignatureForScheme,
    /// No provided signature matched the expected signature.
    SignatureMismatch,
    /// The signed timestamp is outside the allowed tolerance (a replay).
    TimestampOutOfTolerance,
}

impl fmt::Display for WebhookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            WebhookError::MissingHeader => "no Stripe-Signature header value was provided",
            WebhookError::MissingSecret => "no webhook signing secret was provided",
            WebhookError::InvalidTimestamp => "unable to extract a timestamp from the signature header",
            WebhookError::NoSignatureForScheme => "no signature found for the v1 scheme",
            WebhookError::SignatureMismatch => "no signature matched the expected signature for the payload",
            WebhookError::TimestampOutOfTolerance => "timestamp outside the tolerance zone",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for WebhookError {}
