//! A minimal S3 client focused on the most common operations.
//!
//! Based on the commonly used S3 request categories and API operations
//! documented by AWS.
//!
//! References:
//! - <https://docs.aws.amazon.com/AmazonS3/latest/API/API_Operations.html>
//!
//! # Example
//!
//! ```no_run
//! # #[cfg(feature = "reqwest")] {
//! use s3::{Client, ClientConfig, StaticCredentials};
//!
//! let cfg = ClientConfig {
//!     endpoint: "http://127.0.0.1:9000",
//!     credentials: StaticCredentials {
//!         access_key_id: "minioadmin",
//!         secret_access_key: "minioadmin",
//!         session_token: "",
//!     },
//!     region: "auto",
//!     virtual_hosted: false,
//! };
//!
//! let client = Client::new(&cfg)?;
//! # }
//! # Ok::<(), s3::Error>(())
//! ```
pub mod buckets;
pub mod client;
pub mod objects;

pub use buckets::{Bucket, GetBucketLocationOutput, ListBucketsOutput, ListObject, ListObjectsOutput};
#[cfg(feature = "reqwest")]
pub use client::ReqwestHttpClient;
pub use client::{
    ByteStream, Client, ClientConfig, DEFAULT_MAX_RESPONSE_BYTES, Error, HttpClient, HttpError, HttpMethod,
    HttpRequest, HttpResponseData, StaticCredentials,
};
pub use objects::{
    CompleteMultipartUploadOutput, CompletedPart, DeleteObjectsError, DeleteObjectsOutput, DeletedObject,
    GetObjectOutput, GetObjectTaggingOutput, HeadObjectOutput, ListMultipartUploadsOutput, ListPartsOutput,
    MultipartUpload, Tag, UploadPartOutput, UploadedPart,
};
