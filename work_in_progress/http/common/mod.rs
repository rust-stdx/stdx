pub(crate) mod hpack_qpack_shared;

pub mod body;
mod header;
mod method;
mod status;
mod uri;
mod version;

use std::fmt;

pub use body::{Body, Frame};
pub use header::{HeaderName, HeaderValue, Headers};
pub use method::Method;
pub use status::StatusCode;
pub use uri::Uri;
pub use version::Version;

#[derive(Debug, Clone)]
pub struct Request<B: Body = bytes::Bytes> {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: Headers,
    pub body: Option<B>,
}

impl Request<bytes::Bytes> {
    pub fn new(method: Method, uri: Uri, body: Option<bytes::Bytes>) -> Self {
        Request {
            method,
            uri,
            version: Version::Http11,
            headers: Vec::new(),
            body: body,
        }
    }
}

impl<B: Body> fmt::Display for Request<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {:?}", self.method, self.uri, self.version)
    }
}

#[derive(Debug, Clone)]
pub struct Response<B: Body = bytes::Bytes> {
    pub version: Version,
    pub status: StatusCode,
    pub headers: Headers,
    pub body: B,
}

impl Response<bytes::Bytes> {
    pub fn new(status: StatusCode) -> Self {
        Response {
            version: Version::Http11,
            status,
            headers: Vec::new(),
            body: bytes::Bytes::new(),
        }
    }
}

impl<B: Body> fmt::Display for Response<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.version,
            self.status.as_u16(),
            self.status.canonical_reason().unwrap_or("")
        )
    }
}
