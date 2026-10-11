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
//! # Parsing rules
//!
//! * `\r\n`, `\n` and a bare `\r` all terminate a record.
//! * Blank lines are skipped.
//! * A record with a different field count than the first one is an error
//!   unless [`Reader::set_flexible`] is enabled.
//! * Content after a closing quote (`"a"x`) and an unterminated quote are
//!   errors, not silently accepted. When a record is malformed the reader skips
//!   to the next record boundary, so one bad record does not corrupt the rest
//!   of the input.
//! * A UTF-8 byte-order mark at the very start of the input is skipped, once.
//!   A BOM anywhere else is data. Pass [`Reader::set_strip_bom`]`(false)` to keep
//!   it; [`Writer::set_write_bom`]`(true)` emits one.
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
//! # Security
//!
//! By default the reader applies **no size limits**: a single crafted record
//! can make it buffer an unbounded amount of memory. Applications that parse
//! untrusted input should set [`Reader::set_max_record_bytes`], and optionally
//! [`Reader::set_max_field_bytes`] and [`Reader::set_max_columns`]. Exceeding a
//! limit yields an error on the row and stops reading, rather than exhausting
//! memory or reading the rest of an oversized record.
//!
//! The writer does not defend against CSV / formula injection (CWE-1236); see
//! [`Writer`] for details.
//!
//! # Design
//!
//! * **Owned rows**: `Row` and `BytesRow` own their data. Rows can outlive the `Reader`.
//! * **Single buffer**: Each row stores all fields contiguously in one `Vec<u8>`.
//! * **`memchr` scanning**: uses `memchr3` to bulk-scan unquoted fields for
//!   delimiters, quotes and newlines, and `memchr` to scan quoted fields.
//! * **Deferred errors**: Row parsing errors are stored in the row and surfaced
//!   by every accessor (`get`, `to_vec`, `iter`, `error`). I/O errors are also
//!   stored on the row.
//! * **BytesRow / Row split**: `BytesRow` works with raw bytes; `Row` adds UTF-8 validation.
//! * **Streaming**: rows are parsed on-demand from any [`Read`] source, with a
//!   single reusable input buffer that is compacted amortised rather than once
//!   per record.
//!
//! # Performance
//!
//! Parsing is a single pass over the input with `memchr`-accelerated scanning.
//! The reader's input buffer is only compacted once a meaningful amount of it
//! has been consumed, so throughput does not degrade for inputs made of many
//! small records. Each row allocates two small buffers (its field bytes and its
//! field offsets); the writer reuses one buffer and allocates nothing per row.

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod reader;

mod writer;

#[cfg(feature = "serde")]
mod serde;

/// The UTF-8 byte-order mark (`U+FEFF`).
pub(crate) const BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

pub use error::{ReadError, ReadErrorKind, WriteError};
pub use reader::{BytesFields, BytesRow, BytesRows, FieldRange, Fields, Read, Reader, Row, Rows};
pub use writer::{Write, Writer};
