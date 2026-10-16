//! The request pipeline: URL and header construction, retries, response
//! decoding and error mapping.

use core::time::Duration;

use bytes::Bytes;
use futures_util::StreamExt as _;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    client::Client,
    error::{ApiErrorBody, ApiResponseMeta, Error},
    http::{ByteStream, HttpMethod, HttpRequest},
    params::{FormBuilder, FormEncode},
};

#[derive(serde::Deserialize)]
struct ErrorEnvelope {
    #[serde(default)]
    error: ApiErrorBody,
}

/// Maximum number of bytes of an unrecognised error body kept in
/// [`Error::Api`], to bound memory retained by a failing request.
const MAX_RETAINED_ERROR_BODY: usize = 4096;

/// The longest `Retry-After` (in seconds) the client honors.
///
/// A longer delay means the caller should decide when to try again, so the
/// request fails instead of sleeping (and retrying earlier than the server
/// asked). 60 seconds matches Stripe's official libraries.
const MAX_RETRY_AFTER_SECONDS: u64 = 60;

impl Client {
    /// `GET` with a pre-encoded query string, used by pagination.
    pub(crate) async fn get_raw<R: DeserializeOwned>(&self, path: &str, query: &str) -> Result<R, Error> {
        let url = self.build_url(path, query);
        self.execute(HttpMethod::Get, url, None, Bytes::new(), &crate::RequestOptions::default())
            .await?
            .decode()
    }

    /// `GET` an absolute or root-relative URL from a server-provided link
    /// (`next_page_url`).
    ///
    /// The URL is only followed when it stays within the configured base URL, so
    /// a hostile or buggy response cannot redirect the request (and the API key)
    /// to another host.
    pub(crate) async fn get_raw_url<R: DeserializeOwned>(&self, url: &str) -> Result<R, Error> {
        let url = self.resolve_url(url)?;
        self.execute(HttpMethod::Get, url, None, Bytes::new(), &crate::RequestOptions::default())
            .await?
            .decode()
    }

    /// Resolves a server-provided URL against the base URL, rejecting anything
    /// that leaves it.
    fn resolve_url(&self, url: &str) -> Result<String, Error> {
        let base = self.base_url.trim_end_matches('/');
        if url.starts_with('/') {
            return Ok(format!("{base}{url}"));
        }
        let within_base = url.strip_prefix(base).is_some_and(|rest| {
            rest.is_empty() || rest.starts_with('/') || rest.starts_with('?') || rest.starts_with('#')
        });
        if within_base && url.starts_with("http") {
            return Ok(url.to_string());
        }
        Err(Error::InvalidRequest(format!(
            "refusing to follow a link outside the configured base URL: {url}"
        )))
    }

    /// Returns a [`Pager`](crate::Pager) over a list endpoint.
    ///
    /// `base_query` is an already-encoded query string; the pager appends the
    /// pagination cursor. The first page is fetched on the first call to
    /// [`Pager::next_page`](crate::Pager::next_page).
    pub fn paginate<'a, T: crate::PagedItem>(
        &'a self,
        path: impl Into<String>,
        base_query: impl Into<String>,
    ) -> crate::Pager<'a, T> {
        crate::Pager::new(self, path, base_query)
    }

    /// Performs a `GET` request, encoding `params` as the query string.
    ///
    /// Intended for endpoints without a generated helper.
    pub async fn get<R: DeserializeOwned, Q: FormEncode + ?Sized>(&self, path: &str, query: &Q) -> Result<R, Error> {
        self.get_with_options(path, query, &crate::RequestOptions::default())
            .await
    }

    /// Like [`Client::get`], with per-request [`RequestOptions`](crate::RequestOptions).
    pub async fn get_with_options<R: DeserializeOwned, Q: FormEncode + ?Sized>(
        &self,
        path: &str,
        query: &Q,
        options: &crate::RequestOptions,
    ) -> Result<R, Error> {
        let mut form = FormBuilder::with_capacity(crate::params::REQUEST_FORM_CAPACITY);
        query.encode_form(&mut form);
        let url = self.build_url(path, form.as_str());
        self.execute(HttpMethod::Get, url, None, Bytes::new(), options)
            .await?
            .decode()
    }

    /// Performs a `POST` request with a form-encoded body (v1 API).
    pub async fn post_form<R: DeserializeOwned, B: FormEncode + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<R, Error> {
        self.post_form_with_options(path, body, &crate::RequestOptions::default())
            .await
    }

    /// Like [`Client::post_form`], with per-request [`RequestOptions`](crate::RequestOptions).
    pub async fn post_form_with_options<R: DeserializeOwned, B: FormEncode + ?Sized>(
        &self,
        path: &str,
        body: &B,
        options: &crate::RequestOptions,
    ) -> Result<R, Error> {
        let mut form = FormBuilder::with_capacity(crate::params::REQUEST_FORM_CAPACITY);
        body.encode_form(&mut form);
        let url = self.build_url(path, "");
        self.execute(
            HttpMethod::Post,
            url,
            Some("application/x-www-form-urlencoded"),
            Bytes::from(form.into_string().into_bytes()),
            options,
        )
        .await?
        .decode()
    }

    /// Performs a `DELETE` request with a form-encoded body (v1 API).
    pub async fn delete_form<R: DeserializeOwned, B: FormEncode + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<R, Error> {
        self.delete_form_with_options(path, body, &crate::RequestOptions::default())
            .await
    }

    /// Like [`Client::delete_form`], with per-request [`RequestOptions`](crate::RequestOptions).
    pub async fn delete_form_with_options<R: DeserializeOwned, B: FormEncode + ?Sized>(
        &self,
        path: &str,
        body: &B,
        options: &crate::RequestOptions,
    ) -> Result<R, Error> {
        let mut form = FormBuilder::with_capacity(crate::params::REQUEST_FORM_CAPACITY);
        body.encode_form(&mut form);
        let url = self.build_url(path, "");
        self.execute(
            HttpMethod::Delete,
            url,
            Some("application/x-www-form-urlencoded"),
            Bytes::from(form.into_string().into_bytes()),
            options,
        )
        .await?
        .decode()
    }

    /// Performs a `DELETE` request without a body (v1 API).
    pub async fn delete<R: DeserializeOwned>(&self, path: &str) -> Result<R, Error> {
        self.delete_with_options(path, &crate::RequestOptions::default()).await
    }

    /// Like [`Client::delete`], with per-request [`RequestOptions`](crate::RequestOptions).
    pub async fn delete_with_options<R: DeserializeOwned>(
        &self,
        path: &str,
        options: &crate::RequestOptions,
    ) -> Result<R, Error> {
        let url = self.build_url(path, "");
        self.execute(HttpMethod::Delete, url, None, Bytes::new(), options)
            .await?
            .decode()
    }

    /// Performs a `POST` request with a JSON body (v2 API).
    pub async fn post_json<R: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<R, Error> {
        self.post_json_with_options(path, body, &crate::RequestOptions::default())
            .await
    }

    /// Like [`Client::post_json`], with per-request [`RequestOptions`](crate::RequestOptions).
    pub async fn post_json_with_options<R: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        options: &crate::RequestOptions,
    ) -> Result<R, Error> {
        let encoded = Bytes::from(
            serde_json::to_vec(body)
                .map_err(|err| Error::InvalidRequest(format!("failed to encode JSON body: {err}")))?,
        );
        let url = self.build_url(path, "");
        self.execute(HttpMethod::Post, url, Some("application/json"), encoded, options)
            .await?
            .decode()
    }

    /// Performs a `DELETE` request with a JSON body (v2 API).
    pub async fn delete_json<R: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<R, Error> {
        self.delete_json_with_options(path, body, &crate::RequestOptions::default())
            .await
    }

    /// Like [`Client::delete_json`], with per-request [`RequestOptions`](crate::RequestOptions).
    pub async fn delete_json_with_options<R: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
        options: &crate::RequestOptions,
    ) -> Result<R, Error> {
        let encoded = Bytes::from(
            serde_json::to_vec(body)
                .map_err(|err| Error::InvalidRequest(format!("failed to encode JSON body: {err}")))?,
        );
        let url = self.build_url(path, "");
        self.execute(HttpMethod::Delete, url, Some("application/json"), encoded, options)
            .await?
            .decode()
    }

    fn build_url(&self, path: &str, query: &str) -> String {
        let base = self.base_url.trim_end_matches('/');
        if query.is_empty() {
            format!("{base}{path}")
        } else {
            format!("{base}{path}?{query}")
        }
    }

    fn build_headers(&self, content_type: Option<&str>) -> Result<Vec<(String, String)>, Error> {
        let mut headers: Vec<(String, String)> = Vec::with_capacity(6);
        headers.push(("Accept".to_string(), "application/json".to_string()));
        headers.push(("Authorization".to_string(), format!("Bearer {}", self.api_key.as_str())));
        if let Some(version) = &self.api_version {
            headers.push(("Stripe-Version".to_string(), version.clone()));
        }
        if let Some(account) = &self.account {
            headers.push(("Stripe-Account".to_string(), account.clone()));
        }
        if let Some(user_agent) = &self.user_agent {
            headers.push(("User-Agent".to_string(), user_agent.clone()));
        }
        if let Some(content_type) = content_type {
            headers.push(("Content-Type".to_string(), content_type.to_string()));
        }

        for (name, value) in &headers {
            validate_header(name, value)?;
        }

        Ok(headers)
    }

    /// Performs a request and returns its raw response.
    ///
    /// Deliberately *not* generic: the retry loop, body cap and error mapping
    /// are compiled once instead of once per response type.
    async fn execute(
        &self,
        method: HttpMethod,
        url: String,
        content_type: Option<&str>,
        body: Bytes,
        options: &crate::RequestOptions,
    ) -> Result<RawResponse, Error> {
        let mut headers = self.build_headers(content_type)?;

        // An explicit key wins; otherwise auto-generate one for retryable POSTs
        // so a retry can never create a second object. Keys are only meaningful
        // (and only sent) on POST and DELETE.
        if matches!(method, HttpMethod::Post | HttpMethod::Delete) {
            let key = match options.idempotency_key.as_deref().filter(|key| !key.is_empty()) {
                Some(key) => Some(key.to_string()),
                None if method == HttpMethod::Post && self.max_retries > 0 => Some(uuid::Uuid::new_v4().to_string()),
                None => None,
            };
            if let Some(key) = key {
                validate_idempotency_key(&key)?;
                headers.push(("Idempotency-Key".to_string(), key));
            }
        }

        // Fast path: no retries, so the request can be moved rather than cloned.
        if self.max_retries == 0 {
            let request = HttpRequest {
                method,
                url,
                headers,
                body,
                timeout: self.timeout,
            };
            return match self.http.send_erased(request).await {
                Ok(response) => self.handle_response(response).await,
                Err(err) => Err(Error::Http(err)),
            };
        }

        let mut attempt: u32 = 0;
        loop {
            let request = HttpRequest {
                method,
                url: url.clone(),
                headers: headers.clone(),
                body: body.clone(),
                timeout: self.timeout,
            };

            let outcome = match self.http.send_erased(request).await {
                Ok(response) => self.handle_response(response).await,
                Err(err) => Err(Error::Http(err)),
            };

            match outcome {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if attempt < self.max_retries && err.is_retryable() {
                        let retry_after = err.api_meta().and_then(|meta| meta.retry_after);
                        match self.retry_delay(attempt, retry_after) {
                            Some(delay) => {
                                attempt += 1;
                                sleep(delay).await;
                                continue;
                            }
                            // The server asked us to wait longer than we are
                            // willing to: report the error instead of retrying
                            // too early.
                            None => return Err(err),
                        }
                    }
                    return Err(err);
                }
            }
        }
    }

    /// How long to wait before retrying, or `None` when the wait the server
    /// asked for is longer than [`MAX_RETRY_AFTER_SECONDS`].
    fn retry_delay(&self, attempt: u32, retry_after: Option<u64>) -> Option<Duration> {
        if let Some(seconds) = retry_after {
            if seconds > MAX_RETRY_AFTER_SECONDS {
                return None;
            }
            return Some(Duration::from_secs(seconds));
        }
        let shift = attempt.min(16);
        let backoff = self
            .retry_base_delay
            .saturating_mul(1u32 << shift)
            .min(self.retry_max_delay);
        let jitter_cap = (backoff.as_millis() as u64 / 4).max(1);
        let jitter = if jitter_cap == 0 { 0 } else { now_nanos() % jitter_cap };
        Some(backoff + Duration::from_millis(jitter))
    }

    async fn handle_response(&self, response: crate::http::HttpResponseData) -> Result<RawResponse, Error> {
        let status = response.status_code;
        let meta = read_meta(&response);

        let body = read_body(response.body, self.max_response_size).await?;

        if (200..300).contains(&status) {
            Ok(RawResponse {
                status,
                request_id: meta.request_id,
                body: Bytes::from(body),
            })
        } else {
            let error = parse_error_body(&body);
            let raw_body = retained_body(&body);
            Err(Error::Api {
                meta,
                error,
                raw_body,
            })
        }
    }
}

/// A successful response, before it is decoded into a generated type.
pub(crate) struct RawResponse {
    /// The HTTP status code.
    pub(crate) status: u16,
    /// The `Request-Id` header, when present.
    pub(crate) request_id: Option<String>,
    /// The response body.
    pub(crate) body: Bytes,
}

impl RawResponse {
    /// Decodes the body into the expected type.
    ///
    /// This is the only generic step: a thin call into `serde_json`,
    /// instantiated per response type instead of the whole pipeline.
    pub(crate) fn decode<R: DeserializeOwned>(self) -> Result<R, Error> {
        serde_json::from_slice(&self.body).map_err(|source| Error::Decode {
            status: self.status,
            request_id: self.request_id,
            source,
        })
    }
}

fn read_meta(response: &crate::http::HttpResponseData) -> ApiResponseMeta {
    ApiResponseMeta {
        status: response.status_code,
        request_id: response.header("Request-Id").map(str::to_string),
        should_retry: response
            .header("Stripe-Should-Retry")
            .and_then(|value| match value.trim() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }),
        retry_after: response
            .header("Retry-After")
            .and_then(|value| value.trim().parse::<u64>().ok()),
    }
}

async fn read_body(mut stream: ByteStream, limit: usize) -> Result<Vec<u8>, Error> {
    let mut buffer = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(Error::Http)?;
        if buffer.len() + chunk.len() > limit {
            return Err(Error::ResponseTooLarge {
                limit,
            });
        }
        buffer.extend_from_slice(&chunk);
    }
    Ok(buffer)
}

fn parse_error_body(body: &[u8]) -> ApiErrorBody {
    serde_json::from_slice::<ErrorEnvelope>(body)
        .map(|envelope| envelope.error)
        .unwrap_or_default()
}

fn retained_body(body: &[u8]) -> String {
    let lossy = String::from_utf8_lossy(body);
    if lossy.chars().count() <= MAX_RETAINED_ERROR_BODY {
        return lossy.trim().to_string();
    }
    let truncated: String = lossy.chars().take(MAX_RETAINED_ERROR_BODY).collect();
    format!("{}…", truncated.trim_end())
}

/// Rejects header values that could be used for header injection.
fn validate_header(name: &str, value: &str) -> Result<(), Error> {
    if value
        .bytes()
        .any(|byte| byte == b'\r' || byte == b'\n' || byte == 0 || byte == 0x7f || (byte < 0x20 && byte != b'\t'))
    {
        return Err(Error::Config(format!("header `{name}` contains control characters")));
    }
    Ok(())
}

/// Validates a caller-supplied idempotency key before it reaches the transport.
fn validate_idempotency_key(key: &str) -> Result<(), Error> {
    validate_header("Idempotency-Key", key)?;
    if key.len() > 255 {
        return Err(Error::Config("idempotency key must be at most 255 characters".into()));
    }
    Ok(())
}

fn now_nanos() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.subsec_nanos() as u64)
        .unwrap_or(0)
}

/// Sleeps for `duration`.
///
/// With the default `reqwest` feature this uses the tokio timer, which is
/// already present because `reqwest` requires a tokio runtime. Without it (a
/// custom [`HttpClient`] on another runtime) the delay blocks, so configure
/// [`Client::with_max_retries`] to `0` if blocking is unacceptable.
#[cfg(feature = "reqwest")]
async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

#[cfg(not(feature = "reqwest"))]
async fn sleep(duration: Duration) {
    std::thread::sleep(duration);
}
