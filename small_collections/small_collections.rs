//! Small inline-or-heap collections with a `Vec`/`String`-like API.
//!
//! This crate provides two containers that keep short sequences on the stack
//! and automatically spill onto the heap once they outgrow their inline
//! capacity:
//!
//! * [`SmallVec`] — a vector that stores its first `N` elements inline.
//! * [`SmallString`] — a string that stores its first `N` bytes inline.
//!
//! Both dereference to their standard-library counterparts (`[T]` and `str`
//! respectively), so the usual slice and string methods are available.
//!
//! # Examples
//!
//! ```
//! use small_collections::{SmallString, SmallVec};
//!
//! let mut v: SmallVec<i32, 4> = SmallVec::new();
//! v.extend([1, 2, 3]);
//! assert!(v.is_inline());
//!
//! let mut s: SmallString<8> = SmallString::new();
//! s.push_str("hello");
//! assert!(s.is_inline());
//! ```
//!
//! # Feature flags
//!
//! * `std` (default) — enables the standard library and the
//!   `std::io::Write` implementation for `SmallVec<u8, N>`.
//! * `serde` — enables `Serialize`/`Deserialize`. `SmallVec` serializes as a
//!   flat sequence and `SmallString` as a plain string, regardless of the
//!   underlying storage.
//! * `bytes` — enables the `bytes::BufMut` implementation for
//!   `SmallVec<u8, N>`.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod small_string;
mod small_vec;

pub use small_string::SmallString;
pub use small_vec::{Drain, IntoIter, SmallVec};
