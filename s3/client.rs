use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt,
    pin::Pin,
    time::{SystemTime, UNIX_EPOCH},
};

use bytes::Bytes;
use crypto::{Hasher, hmac::Hmac, sha2::Sha256};
use futures_util::{Stream, StreamExt};
use url::Url;

pub(crate) const EMPTY_PAYLOAD_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

#[derive(Debug, Clone)]
pub struct StaticCredentials<'a> {
    pub access_key_id: &'a str,
    pub secret_access_key: &'a str,
    pub session_token: &'a str,
}

#[derive(Debug, Clone)]
pub struct ClientConfig<'a> {
    pub endpoint: &'a str,
    pub credentials: StaticCredentials<'a>,
    pub region: &'a str,
    pub virtual_hosted: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct OwnedCredentials {
    pub(crate) access_key_id: String,
    pub(crate) secret_access_key: String,
    pub(crate) session_token: Option<String>,
}

#[derive(Debug)]
pub struct Client<H: HttpClient> {
    pub(crate) endpoint: Url,
    pub(crate) region: String,
    pub(crate) credentials: OwnedCredentials,
    pub(crate) virtual_hosted: bool,
    pub(crate) http: H,
}

#[derive(Debug)]
pub enum Error {
    InvalidConfig(&'static str),
    Http(Box<dyn core::error::Error + Send + Sync>),
    Time(std::time::SystemTimeError),
    Xml(quick_xml::DeError),
    Api { status: u16, body: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfig(msg) => write!(f, "invalid config: {msg}"),
            Error::Http(err) => write!(f, "http error: {err}"),
            Error::Time(err) => write!(f, "time error: {err}"),
            Error::Xml(err) => write!(f, "xml error: {err}"),
            Error::Api {
                status,
                body,
            } => write!(f, "s3 api error (status {status}): {body}"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Http(err) => Some(&**err),
            _ => None,
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(value: std::time::SystemTimeError) -> Self {
        Self::Time(value)
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(value: quick_xml::DeError) -> Self {
        Self::Xml(value)
    }
}

pub type HttpError = Box<dyn core::error::Error + Send + Sync>;

/// A pinned, boxed async stream of byte chunks.
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, HttpError>> + Send>>;

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Head,
}

impl HttpMethod {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Put => "PUT",
            HttpMethod::Post => "POST",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub struct HttpResponseData {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: ByteStream,
}

impl HttpResponseData {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

pub trait HttpClient: Send + Sync {
    fn send(
        &self,
        request: HttpRequest,
    ) -> impl std::future::Future<Output = Result<HttpResponseData, HttpError>> + Send;
}

#[cfg(feature = "reqwest")]
#[derive(Debug, Clone)]
pub struct ReqwestHttpClient {
    inner: reqwest::Client,
}

#[cfg(feature = "reqwest")]
impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "reqwest")]
impl ReqwestHttpClient {
    /// Builds an HTTP client backed by `reqwest`.
    ///
    /// Installs the stdx rustls `CryptoProvider` as the process default (if
    /// none is installed yet) before creating the underlying `reqwest` client,
    /// because `reqwest` is compiled with the `rustls-no-provider` feature and
    /// would otherwise panic for lack of a cryptography provider.
    pub fn new() -> Self {
        crypto_rustls::install_default_rustls_provider();
        Self {
            inner: reqwest::Client::new(),
        }
    }
}

#[cfg(feature = "reqwest")]
impl HttpClient for ReqwestHttpClient {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponseData, HttpError> {
        let method = reqwest::Method::from_bytes(request.method.as_str().as_bytes())?;
        let mut req = self.inner.request(method, &request.url);
        for (name, value) in request.headers {
            req = req.header(&name, &value);
        }
        if !request.body.is_empty() {
            req = req.body(request.body);
        }
        let response = req.send().await?;
        let status_code = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| value.to_str().ok().map(|v| (name.as_str().to_string(), v.to_string())))
            .collect();
        let body: ByteStream = Box::pin(
            response
                .bytes_stream()
                .map(|r| r.map_err(|e| -> HttpError { Box::new(e) })),
        );
        Ok(HttpResponseData {
            status_code,
            headers,
            body,
        })
    }
}

impl<H: HttpClient> Client<H> {
    pub fn with_http_client(config: &ClientConfig<'_>, http: H) -> Result<Self, Error> {
        if config.endpoint.trim().is_empty() {
            return Err(Error::InvalidConfig("endpoint must not be empty"));
        }
        if config.region.trim().is_empty() {
            return Err(Error::InvalidConfig("region must not be empty"));
        }
        if config.credentials.access_key_id.trim().is_empty() {
            return Err(Error::InvalidConfig("access key id must not be empty"));
        }
        if config.credentials.secret_access_key.trim().is_empty() {
            return Err(Error::InvalidConfig("secret access key must not be empty"));
        }

        let endpoint = Url::parse(config.endpoint).map_err(|_| Error::InvalidConfig("invalid endpoint URL"))?;

        Ok(Self {
            endpoint,
            region: config.region.to_string(),
            credentials: OwnedCredentials {
                access_key_id: config.credentials.access_key_id.to_string(),
                secret_access_key: config.credentials.secret_access_key.to_string(),
                session_token: if config.credentials.session_token.is_empty() {
                    None
                } else {
                    Some(config.credentials.session_token.to_string())
                },
            },
            virtual_hosted: config.virtual_hosted,
            http,
        })
    }

    pub(crate) async fn execute(
        &self,
        method: HttpMethod,
        canonical_uri: &str,
        canonical_query: &str,
        body: &[u8],
        bucket: &str,
    ) -> Result<HttpResponseData, Error> {
        self.execute_with_headers(method, canonical_uri, canonical_query, body, &[], bucket)
            .await
    }

    pub(crate) async fn execute_with_headers(
        &self,
        method: HttpMethod,
        canonical_uri: &str,
        canonical_query: &str,
        body: &[u8],
        extra_headers: &[(String, String)],
        bucket: &str,
    ) -> Result<HttpResponseData, Error> {
        fn canonical_header_value(value: &str) -> String {
            let trimmed = value.trim();
            let mut out = String::with_capacity(trimmed.len());
            let mut in_space = false;
            for ch in trimmed.chars() {
                if ch.is_whitespace() {
                    if !in_space {
                        out.push(' ');
                        in_space = true;
                    }
                } else {
                    out.push(ch);
                    in_space = false;
                }
            }
            out
        }

        let (date, amz_datetime) = amz_datetime(SystemTime::now())?;
        let credential_scope = format!("{}/{}/s3/aws4_request", date, self.region);

        let base_host = endpoint_host(&self.endpoint);
        let host = if self.virtual_hosted && !bucket.is_empty() {
            Cow::Owned(format!("{bucket}.{base_host}"))
        } else {
            Cow::Borrowed(&base_host)
        };
        let payload_hash = if body.is_empty() {
            EMPTY_PAYLOAD_SHA256.to_string()
        } else {
            hex::encode(&sha256(body))
        };

        let mut headers = vec![
            ("host".to_string(), host.to_string()),
            ("x-amz-date".to_string(), amz_datetime.clone()),
            ("x-amz-content-sha256".to_string(), payload_hash.clone()),
        ];
        headers.extend(extra_headers.iter().cloned());
        if let Some(token) = self.credentials.session_token.as_deref() {
            headers.push(("x-amz-security-token".to_string(), token.to_string()));
        }

        let mut canonical_headers = headers
            .iter()
            .map(|(name, value)| format!("{}:{}\n", name.to_ascii_lowercase(), canonical_header_value(value)))
            .collect::<Vec<_>>();
        let mut signed_headers = headers
            .iter()
            .map(|(name, _)| name.to_ascii_lowercase())
            .collect::<Vec<_>>();

        canonical_headers.sort();
        signed_headers.sort();

        let canonical_headers_joined = canonical_headers.concat();
        let signed_headers_joined = signed_headers.join(";");

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method.as_str(),
            canonical_uri,
            canonical_query,
            canonical_headers_joined,
            signed_headers_joined,
            payload_hash
        );

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_datetime,
            credential_scope,
            hex::encode(&sha256(canonical_request.as_bytes()))
        );

        let signature = hex::encode(&sign_v4(
            &self.credentials.secret_access_key,
            &date,
            &self.region,
            &string_to_sign,
        ));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.credentials.access_key_id, credential_scope, signed_headers_joined, signature
        );

        let url = if self.virtual_hosted && !bucket.is_empty() {
            let scheme = self.endpoint.scheme();
            let host_str = endpoint_host(&self.endpoint);
            let path = canonical_uri;
            if canonical_query.is_empty() {
                format!("{scheme}://{bucket}.{host_str}{path}")
            } else {
                format!("{scheme}://{bucket}.{host_str}{path}?{canonical_query}")
            }
        } else if canonical_query.is_empty() {
            format!("{}{}", self.endpoint.as_str().trim_end_matches('/'), canonical_uri)
        } else {
            format!(
                "{}{}?{}",
                self.endpoint.as_str().trim_end_matches('/'),
                canonical_uri,
                canonical_query
            )
        };

        headers.push(("authorization".to_string(), authorization));

        let request = HttpRequest {
            method,
            url,
            headers,
            body: body.to_vec(),
        };
        let response = self.http.send(request).await.map_err(Error::Http)?;

        if (200..300).contains(&response.status_code) {
            return Ok(response);
        }

        let status = response.status_code;
        let body_bytes = collect_body(response.body).await.unwrap_or_default();
        let body = String::from_utf8_lossy(&body_bytes).into_owned();
        Err(Error::Api {
            status,
            body,
        })
    }
}

#[cfg(feature = "reqwest")]
impl Client<ReqwestHttpClient> {
    pub fn new(config: &ClientConfig<'_>) -> Result<Self, Error> {
        Self::with_http_client(config, ReqwestHttpClient::new())
    }
}

pub(crate) fn consume_empty(_response: HttpResponseData) -> Result<(), Error> {
    Ok(())
}

pub(crate) fn canonical_bucket_uri(bucket: &str) -> String {
    canonical_uri(&format!("/{bucket}"))
}

pub(crate) fn canonical_object_uri(bucket: &str, key: &str) -> String {
    canonical_uri(&format!("/{bucket}/{key}"))
}

pub(crate) fn canonical_uri(path: &str) -> String {
    path.split('/').map(percent_encode).collect::<Vec<_>>().join("/")
}

pub(crate) fn canonical_query_string(params: &BTreeMap<String, String>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn endpoint_host(url: &Url) -> String {
    match (url.host_str(), url.port()) {
        (Some(host), Some(port)) => format!("{host}:{port}"),
        (Some(host), None) => host.to_string(),
        _ => String::new(),
    }
}

pub(crate) fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for &b in input.as_bytes() {
        if b.is_ascii_uppercase()
            || b.is_ascii_lowercase()
            || b.is_ascii_digit()
            || matches!(b, b'-' | b'_' | b'.' | b'~')
        {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(hex_nibble_upper((b >> 4) & 0x0f));
            out.push(hex_nibble_upper(b & 0x0f));
        }
    }
    out
}

fn hex_nibble_upper(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'A' + (value - 10)) as char,
        _ => unreachable!(),
    }
}

fn sign_v4(secret_access_key: &str, date: &str, region: &str, string_to_sign: &str) -> [u8; 32] {
    let k_date = hmac_sha256(format!("AWS4{secret_access_key}").as_bytes(), date.as_bytes());
    let k_region = hmac_sha256(&k_date, region.as_bytes());
    let k_service = hmac_sha256(&k_region, b"s3");
    let k_signing = hmac_sha256(&k_service, b"aws4_request");
    hmac_sha256(&k_signing, string_to_sign.as_bytes())
}

pub(crate) fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; Sha256::OUTPUT_SIZE] {
    let mut mac = Hmac::<Sha256>::new(key);
    mac.update(data);
    *mac.finalize().as_ref().as_array().unwrap()
}

pub(crate) fn sha256(data: &[u8]) -> [u8; Sha256::OUTPUT_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.sum().as_ref().try_into().unwrap()
}

fn amz_datetime(now: SystemTime) -> Result<(String, String), Error> {
    let elapsed = now.duration_since(UNIX_EPOCH)?;
    let total_seconds = elapsed.as_secs() as i64;

    let days = total_seconds.div_euclid(86_400);
    let seconds_of_day = total_seconds.rem_euclid(86_400);

    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;

    let date = format!("{year:04}{month:02}{day:02}");
    let datetime = format!("{date}T{hour:02}{minute:02}{second:02}Z");

    Ok((date, datetime))
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i32, i64, i64) {
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };

    (year as i32, month, day)
}

pub(crate) async fn collect_body(mut stream: ByteStream) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(Error::Http)?;
        out.extend_from_slice(&bytes);
    }
    Ok(out)
}

pub(crate) fn bytes_to_string(bytes: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(bytes).map_err(|e| Error::Http(Box::new(e)))
}

pub(crate) fn header_to_string(response: &HttpResponseData, name: &str) -> Option<String> {
    response.header(name).map(ToString::to_string)
}

pub(crate) fn header_to_u64(response: &HttpResponseData, name: &str) -> Option<u64> {
    response.header(name).and_then(|s| s.parse::<u64>().ok())
}

pub(crate) fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct NoopHttpClient;

    impl HttpClient for NoopHttpClient {
        async fn send(&self, _request: HttpRequest) -> Result<HttpResponseData, HttpError> {
            Ok(HttpResponseData {
                status_code: 200,
                headers: Vec::new(),
                body: Box::pin(futures_util::stream::empty()),
            })
        }
    }

    #[test]
    fn sha256_known_vectors() {
        assert_eq!(
            hex::encode(&sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex::encode(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn hmac_sha256_known_vector() {
        let key = [0x0b; 20];
        let sig = hmac_sha256(&key, b"Hi There");
        assert_eq!(
            hex::encode(&sig),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
    }

    #[test]
    fn percent_encoding_works() {
        assert_eq!(percent_encode("abcXYZ-_.~"), "abcXYZ-_.~");
        assert_eq!(percent_encode("hello world/é"), "hello%20world%2F%C3%A9");
    }

    #[test]
    fn canonical_uri_preserves_slashes() {
        assert_eq!(
            canonical_object_uri("my-bucket", "folder/my file.txt"),
            "/my-bucket/folder/my%20file.txt"
        );
    }

    #[test]
    fn amz_datetime_format_works() {
        let ts = UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
        let (date, datetime) = amz_datetime(ts).unwrap();
        assert_eq!(date.len(), 8);
        assert_eq!(datetime.len(), 16);
        assert!(datetime.ends_with('Z'));
    }

    #[test]
    fn client_config_validation_works() {
        let cfg = ClientConfig {
            endpoint: "",
            credentials: StaticCredentials {
                access_key_id: "a",
                secret_access_key: "b",
                session_token: "",
            },
            region: "auto",
            virtual_hosted: false,
        };
        assert!(matches!(
            Client::with_http_client(&cfg, NoopHttpClient),
            Err(Error::InvalidConfig(_))
        ));
    }

    #[test]
    fn sign_v4_known_output_length() {
        let sig = sign_v4(
            "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY",
            "20130524",
            "auto",
            "AWS4-HMAC-SHA256\n20130524T000000Z\n20130524/auto/s3/aws4_request\nabc",
        );
        assert_eq!(sig.len(), 32);
    }

    #[test]
    fn empty_payload_sha256_constant_is_correct() {
        assert_eq!(EMPTY_PAYLOAD_SHA256, hex::encode(&sha256(b"")));
    }
}
