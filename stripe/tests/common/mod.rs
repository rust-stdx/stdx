//! Shared test helpers: a recording, queue-driven [`HttpClient`].

#![allow(dead_code)]

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, MutexGuard},
};

use stripe::{
    ByteStream, HttpClient, HttpError, HttpRequest, HttpResponseData, byte_stream,
    params::{FormBuilder, FormEncode},
};

/// A minimal form-encodable list of top-level fields, for tests that only need
/// to observe the request the client builds.
#[derive(Debug, Clone, Default)]
pub struct Fields(pub Vec<(String, String)>);

impl Fields {
    /// Creates an empty parameter set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a field, returning `self` for chaining.
    pub fn field(mut self, name: &str, value: &str) -> Self {
        self.0.push((name.to_string(), value.to_string()));
        self
    }
}

impl FormEncode for Fields {
    fn encode_form(&self, form: &mut FormBuilder) {
        for (name, value) in &self.0 {
            form.field(name, value);
        }
    }
}

/// A request captured by [`MockClient`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedRequest {
    /// HTTP method name.
    pub method: String,
    /// Full request URL.
    pub url: String,
    /// Request headers.
    pub headers: Vec<(String, String)>,
    /// Request body bytes.
    pub body: Vec<u8>,
    /// Request timeout, if any.
    pub timeout: Option<std::time::Duration>,
}

impl RecordedRequest {
    /// Returns the value of a header, case-insensitively.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// Returns the request body decoded as UTF-8.
    pub fn body_str(&self) -> &str {
        std::str::from_utf8(&self.body).expect("request body must be valid UTF-8")
    }
}

/// A queued response.
#[derive(Debug, Clone)]
pub enum Response {
    /// A successful transport exchange.
    Ok {
        /// HTTP status code.
        status: u16,
        /// Response headers.
        headers: Vec<(String, String)>,
        /// Response body.
        body: Vec<u8>,
    },
    /// A transport-level failure.
    Error(String),
}

#[derive(Default)]
struct State {
    requests: Vec<RecordedRequest>,
    responses: VecDeque<Response>,
}

/// A mock [`HttpClient`] that records requests and replays queued responses.
#[derive(Clone, Default)]
pub struct MockClient {
    state: Arc<Mutex<State>>,
}

impl MockClient {
    /// Creates an empty mock.
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|err| err.into_inner())
    }

    /// Queues a JSON response with the given status.
    pub fn push_json(&self, status: u16, body: &str) {
        self.push_response(status, &[], body.as_bytes().to_vec());
    }

    /// Queues a response with the given status, headers and body.
    pub fn push_response(&self, status: u16, headers: &[(&str, &str)], body: Vec<u8>) {
        self.lock().responses.push_back(Response::Ok {
            status,
            headers: headers
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
            body,
        });
    }

    /// Queues a transport error.
    pub fn push_error(&self, message: &str) {
        self.lock().responses.push_back(Response::Error(message.to_string()));
    }

    /// Returns every recorded request, in order.
    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.lock().requests.clone()
    }

    /// Returns the number of recorded requests.
    pub fn request_count(&self) -> usize {
        self.lock().requests.len()
    }

    /// Returns the most recent recorded request.
    pub fn last_request(&self) -> RecordedRequest {
        self.lock().requests.last().cloned().expect("no request was recorded")
    }
}

impl HttpClient for MockClient {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponseData, HttpError> {
        let response = {
            let mut state = self.lock();
            state.requests.push(RecordedRequest {
                method: request.method.as_str().to_string(),
                url: request.url,
                headers: request.headers,
                body: request.body.to_vec(),
                timeout: request.timeout,
            });
            state.responses.pop_front()
        };

        match response {
            Some(Response::Ok {
                status,
                headers,
                body,
            }) => {
                let body: ByteStream = byte_stream(body);
                Ok(HttpResponseData {
                    status_code: status,
                    headers,
                    body,
                })
            }
            Some(Response::Error(message)) => Err(message.into()),
            None => Err("mock client: no queued response".into()),
        }
    }
}

/// Builds a client over a mock with retries disabled and a short timeout.
pub fn test_client(mock: MockClient) -> stripe::Client {
    stripe::Client::with_http_client("sk_test_123", mock)
        .expect("client")
        .with_max_retries(0)
        .with_base_url("https://api.stripe.test")
}
