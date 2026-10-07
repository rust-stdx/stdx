#![cfg_attr(not(feature = "std"), no_std)]

//! A fast, low-allocation CSV parser with `no_std` and serde support.
//!
//! # Quick start
//!
//! ```rust
//! let data: &[u8] = b"name,age,city\nAlice,30,NYC\nBob,25,LA\n";
//! let mut reader = csv::Reader::new(data);
//!
//! for row in reader.rows() {
//!     for field in row.iter() {
//!         let field: &str = field.unwrap();
//!     }
//!     let fields: Vec<&str> = row.to_vec().unwrap();
//! }
//! ```
//!
//! # Features
//!
//! | Flag | Default | Description |
//! |------|---------|-------------|
//! | `std` | on | Enables `std::io::Read` support for `Reader`, and `Writer` impls. |
//! | `serde` | off | Enables `Row::deserialize()` + `Writer::serialize()` for serde support. |
//!
//! # Streaming from any [`Read`] source
//!
//! ```no_run
//! use std::fs::File;
//! use csv::Reader;
//!
//! let file = File::open("data.csv")?;
//! let mut reader = Reader::new(file);
//!
//! for row in reader.rows() {
//!     let name = row.get(0).unwrap().unwrap();
//!     println!("{name}");
//! }
//! # Ok::<_, csv::ReadError>(())
//! ```
//!
//! # Headers
//!
//! ```no_run
//! use csv::Reader;
//! let data = b"name,age\nAlice,30\n";
//! let mut reader = Reader::new(std::io::Cursor::new(data));
//! let headers = reader.parse_headers()?;
//! for row in reader.rows() {
//! }
//! # Ok::<_, csv::ReadError>(())
//! ```
//!
//! # Serde (requires `serde` feature)
//!
//! ```no_run
//! # #[cfg(feature = "serde")] {
//! use csv::Reader;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Record {
//!     name: String,
//!     age: u32,
//! }
//!
//! let mut reader = Reader::new(std::io::Cursor::new(b"name,age\nAlice,30\n"));
//! reader.parse_headers()?;
//!
//! for row in reader.rows() {
//!     let rec: Record = row.deserialize()?;
//!     println!("{} is {}", rec.name, rec.age);
//! }
//! # }
//! # Ok::<_, csv::ReadError>(())
//! ```
//!
//! # Writer
//!
//! ```no_run
//! use csv::Writer;
//!
//! let mut w = Writer::new(Vec::new());
//! w.write_headers(["name", "age"])?;
//! w.write_row(["Alice", "30"])?;
//! let bytes = w.into_inner()?;
//! # Ok::<_, csv::WriteError>(())
//! ```
//!
//! With serde:
//!
//! ```no_run
//! # #[cfg(feature = "serde")] {
//! use csv::Writer;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Person { name: String, age: u32 }
//!
//! let mut w = Writer::new(Vec::new())
//!     .set_headers(vec!["name".into(), "age".into()]);
//! let p = Person { name: "Alice".into(), age: 30 };
//! w.serialize(&p)?;
//! # }
//! # Ok::<_, csv::WriteError>(())
//! ```
//!
//! # Design
//!
//! * **Owned rows**: `Row` and `BytesRow` own their data. Rows can outlive the `Reader`.
//! * **Single buffer**: Each row stores all fields contiguously in one `Vec<u8>`.
//! * **SIMD scanning**: uses `memchr3` to bulk-scan for delimiters, quotes, and newlines.
//! * **Deferred errors**: Row parsing errors are stored in the row and surfaced on access.
//! * **BytesRow / Row split**: `BytesRow` works with raw bytes; `Row` adds UTF-8 validation.
//! * **Streaming**: rows are parsed on-demand from any [`Read`] source.

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod reader;

mod writer;

#[cfg(feature = "serde")]
mod serde;

pub use error::{ReadError, ReadErrorKind, WriteError};
pub use reader::{BytesFields, BytesRow, BytesRows, FieldRange, Fields, Read, Reader, Row, Rows};
pub use writer::{Write, Writer};
