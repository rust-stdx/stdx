#![allow(missing_docs)]

mod client;
pub mod error;
mod expandable;
pub mod http;
pub mod pagination;
pub mod params;
mod request;
pub mod webhooks;

pub use client::{
    Client, DEFAULT_API_VERSION, DEFAULT_BASE_URL, DEFAULT_MAX_RESPONSE_SIZE, DEFAULT_MAX_RETRIES, RequestOptions,
};
pub use error::{ApiErrorBody, ApiResponseMeta, Error, HttpError, WebhookError};
pub use expandable::Expandable;
#[cfg(feature = "reqwest")]
pub use http::ReqwestHttpClient;
pub use http::{ByteStream, HttpClient, HttpMethod, HttpRequest, HttpResponseData, byte_stream, chunk_stream};
pub use pagination::{List, PagedItem, Pager};
pub use params::{FormBuilder, FormEncode, encode, percent_encode_into};
pub use webhooks::{DEFAULT_TOLERANCE_SECONDS, Webhook, verify_signature};

/// The Stripe API version this crate was generated from.
pub const SPEC_VERSION: &str = "2026-09-30.endive";
