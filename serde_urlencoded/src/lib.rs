//! `x-www-form-urlencoded` meets Serde.
//!
//! This crate serialises to and deserialises from the
//! [`application/x-www-form-urlencoded`][urlencoded] format. It is built upon
//! [Serde], a high performance generic serialization framework, and the
//! `form_urlencoded` crate in this workspace.
//!
//! [urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded
//! [Serde]: https://github.com/serde-rs/serde
//!
//! # Example
//!
//! ```
//! let params = vec![("key", "value"), ("foo", "bar")];
//! let body = serde_urlencoded::to_string(&params).unwrap();
//! assert_eq!(body, "key=value&foo=bar");
//! ```
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use crate::de::{Deserializer, from_bytes, from_reader, from_str};
#[doc(inline)]
pub use crate::ser::{Serializer, to_string};
