//! An async PostgreSQL client with a macro-only, compile-time checked query API.
//!
//! # Features
//!
//! * **Compile-time checked queries** — [`query!`] and [`query_as!`] connect to
//!   the database at build time (when `DATABASE_URL` is set) and verify the
//!   statement, its parameters and its result shape against your Rust types.
//! * **Async, tokio-only** — one background task per connection; no busy
//!   polling and no async runtime dependency beyond tokio.
//! * **Connection pooling** — bounded pool with idle/lifetime limits and
//!   connections returned automatically on drop.
//! * **Ergonomic row mapping** — `#[derive(FromRow)]`, tuples, and scalars.
//! * **JSON structs** — `#[derive(Json)]` maps a `serde` type straight to a
//!   `json` / `jsonb` column, with no `serde_json::Value` in between.
//! * **Iterator array binding** — bind `UNNEST` / `= ANY` arrays from a lazy
//!   iterator with no intermediate `Vec`.
//! * **TLS** — rustls with the stdx `crypto_rustls` provider (TLS 1.3).
//! * **COPY** — binary `COPY ... FROM STDIN` / `TO STDOUT` for bulk loads.
//! * **Scripts** — `execute_script` runs multi-statement SQL (migrations, `SET`
//!   scripts, `VACUUM`) via the simple query protocol.
//! * **`LISTEN` / `NOTIFY`** — [`Connection::notifications`] streams
//!   notifications from the server.
//! * **Cancellation** — an abandoned query is drained in the background and,
//!   if it is still running two seconds later, cancelled with a
//!   `CancelRequest`.
//! * **Zero-copy accessors** — [`Row::get_str`] / [`Row::get_bytes`] borrow
//!   from the received frame.
//! * **Transactions without lifetimes** — `Connection`, `Pool`, `PooledConnection`
//!   and `Transaction` all implement [`Executor`], so generic code works with any
//!   of them.
//!
//! # Quick start
//!
//! ```ignore
//! use postgresql::{query, query_as, FromRow};
//!
//! #[derive(FromRow, Debug)]
//! struct User {
//!     id: i32,
//!     name: String,
//!     email: Option<String>,
//! }
//!
//! # async fn run(pool: postgresql::Pool) -> Result<(), postgresql::Error> {
//! # use postgresql::{query, query_as};
//! // Compile-time checked when DATABASE_URL is set at build time.
//! let users: Vec<User> = query_as!(User,
//!     "SELECT id, name, email FROM users WHERE age >= $1",
//!     18)
//!     .fetch_all(&pool)
//!     .await?;
//!
//! let count: i64 = query_as!(i64, "SELECT count(*)::int8 FROM users")
//!     .fetch_one(&pool)
//!     .await?;
//!
//! let affected: u64 = query!("UPDATE users SET active = true WHERE age > $1", 18)
//!     .execute(&pool)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Connecting
//!
//! ```no_run
//! use postgresql::{Connection, Pool};
//!
//! # async fn run() -> Result<(), postgresql::Error> {
//! let conn = Connection::connect("postgres://user:pass@localhost/app").await?;
//! let pool = Pool::connect("postgres://user:pass@localhost/app?pool_size=16", None).await?;
//! # let _ = (conn, pool);
//! # Ok(())
//! # }
//! ```
//!
//! Only `postgres://` and `postgresql://` URLs are accepted. Supported query
//! parameters are `sslmode`, `sslrootcert`, `connect_timeout` (seconds),
//! `application_name`, `options`, and the pool settings `pool_size`,
//! `min_connections`, `acquire_timeout`, `idle_timeout` and `max_lifetime`
//! (seconds). `timezone` sets the IANA zone used for zone-less
//! `timestamp`/`date`/`time` values (default UTC), and `statement_cache_size`
//! bounds the prepared-statement cache (`0` disables it, for transaction-mode
//! poolers). `max_message_len` bounds the largest backend message the client
//! will buffer, in bytes (default 128 MiB); `max_scram_iterations` caps the
//! SCRAM iteration count the server may request (default 2,000,000). Unknown
//! parameters are rejected so typos surface immediately.
//!
//! `connect_timeout` bounds the **whole** connection attempt: the TCP connect,
//! the TLS handshake and authentication. A server that accepts TCP and then
//! stalls cannot hang `Connection::connect`.
//!
//! # TLS is required
//!
//! This client **always** connects over TLS; plaintext connections are not
//! possible. `sslmode=disable`, `prefer` and `allow` are rejected with a
//! configuration error. `sslmode` selects the verification level:
//!
//! | `sslmode` | Behaviour |
//! | --- | --- |
//! | `require` (default) | TLS, certificate **not** verified (like libpq's `require`) |
//! | `verify-ca` | TLS, certificate chain verified, hostname **not** checked |
//! | `verify-full` | TLS, chain and hostname verified |
//!
//! TLS uses rustls with the stdx `crypto_rustls` provider (TLS 1.3 only).
//!
//! # Transactions
//!
//! ```ignore
//! # async fn run(pool: postgresql::Pool) -> Result<(), postgresql::Error> {
//! use postgresql::query;
//! let txn = pool.begin().await?;
//! query!("INSERT INTO accounts (id, balance) VALUES ($1, $2)", 1i32, 100i64)
//!     .execute(&txn)
//!     .await?;
//! txn.commit().await?;
//!
//! // Dropping without committing rolls back and returns the connection.
//! # Ok(())
//! # }
//! ```
//!
//! # Generic code over any executor
//!
//! [`Executor`] is a sealed marker implemented by [`Connection`],
//! [`PooledConnection`], [`Pool`] and [`Transaction`]. Queries are always run
//! through the macros, so generic helpers look like this:
//!
//! ```ignore
//! async fn count_users<E: postgresql::Executor>(db: &E, min_age: i32)
//!     -> Result<i64, postgresql::Error>
//! {
//! # use postgresql::query_as;
//!     query_as!(i64, "SELECT count(*)::int8 FROM users WHERE age >= $1", min_age)
//!         .fetch_one(db)
//!         .await
//! }
//! ```
//!
//! # Iterator array binding (`UNNEST` / `= ANY`)
//!
//! [`array()`](crate::array) wraps a lazy iterator as a PostgreSQL array without allocating an
//! intermediate `Vec`:
//!
//! ```ignore
//! # async fn run(db: impl postgresql::Executor, invitations: Vec<(i32, String)>) -> Result<(), postgresql::Error> {
//! use postgresql::query;
//! query!(
//!     "INSERT INTO invitations (id, name)
//!      SELECT * FROM unnest($1::int4[], $2::text[])",
//!     postgresql::array(invitations.iter().map(|i| i.0)),
//!     postgresql::array(invitations.iter().map(|i| i.1.as_str())),
//! )
//! .execute(&db)
//! .await?;
//! # Ok(())
//! # }
//! ```
//!
//! `Vec<T>` and `&[T]` bind as arrays directly; `array(iter)` is for lazy
//! iterators and is single-use.
//!
//! # Bulk loads (`COPY`)
//!
//! [`copy_in!`] and [`copy_out!`] check the `COPY` statement against the target
//! type at build time (by resolving the relation in the catalog) and stream
//! binary rows. `#[derive(ToRow)]` encodes a struct's fields in declaration
//! order.
//!
//! ```ignore
//! # async fn run(conn: postgresql::Connection) -> Result<(), postgresql::Error> {
//! use postgresql::copy_in;
//! let copy = copy_in!(conn, (i32, String), "COPY users (id, name) FROM STDIN BINARY").await?;
//! for i in 0..1_000_000i32 {
//!     copy.write_row(&(i, format!("user-{i}"))).await?;
//! }
//! let copied = copy.finish().await?;
//! # Ok(())
//! # }
//! ```
//!
//! Only `COPY ... FROM STDIN BINARY` and `COPY ... TO STDOUT BINARY` are
//! supported.
//!
//! # Scripts, notifications and cancellation
//!
//! ```ignore
//! use postgresql::{ScriptExecutor, query};
//!
//! # async fn run(conn: postgresql::Connection) -> Result<(), postgresql::Error> {
//! // Multi-statement scripts (unchecked, parameterless).
//! conn.execute_script("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT)").await?;
//!
//! // LISTEN / NOTIFY.
//! conn.execute_script("LISTEN jobs").await?;
//! let mut notifications = conn.notifications();
//! query!("NOTIFY jobs, 'go'").execute(&conn).await?;
//! let n = notifications.recv().await.unwrap();
//! # let _ = n;
//!
//! // Dropping a row stream drains the result and, if the query is still
//! // running after a short delay, cancels it server-side.
//! # Ok(())
//! # }
//! ```
//!
//! # Supported types
//!
//! | Rust type | PostgreSQL type |
//! | --- | --- |
//! | `bool` | `bool` |
//! | `i16`, `i32`, `i64` | `int2`, `int4`, `int8` |
//! | `f32`, `f64` | `float4`, `float8` |
//! | `String` / `&str` | `text`, `varchar`, `bpchar`, `name` |
//! | `Vec<u8>` / `&[u8]` | `bytea` |
//! | `uuid::Uuid` | `uuid` |
//! | `time::DateTime` | `timestamptz`, `timestamp`, `date`, `time`, `timetz` |
//! | `serde_json::Value` | `json`, `jsonb` |
//! | `#[derive(Json)]` types, [`struct@Json<T>`] | `json`, `jsonb` |
//! | `ipnetwork::IpNetwork` | `inet`, `cidr` |
//! | [`Interval`] | `interval` |
//! | `Option<T>` | `NULL` if `None` |
//! | `Vec<T>`, `&[T]`, [`Array`] | arrays |
//!
//! `numeric` / `decimal` is **not supported yet**. Cast it in SQL to a
//! supported type (`amount::text` for exact values, `amount::float8` for
//! approximate ones) or store it as `int8` / `f64`.
//!
//! Decoding a column into a Rust type it cannot represent (say an `int4`
//! column into `bool`) is an error, not a reinterpretation of the bytes.
//!
//! # Limits and unsupported values
//!
//! * Arrays must be **one-dimensional**; a multi-dimensional array is a decode
//!   error.
//! * PostgreSQL's `infinity` / `-infinity` `date` and `timestamp` values are
//!   rejected with a decode error: they are not a real instant and cannot be
//!   represented as a [`time::DateTime`].
//! * Temporal values are bounded by `time::DateTime`'s range, **years −9999 to
//!   9999**, even though PostgreSQL can represent dates from 4713 BC to 294276
//!   AD. A value outside that range is a decode error. `time` and `timetz`
//!   values must fall within a single day (`24:00:00` is rejected).
//! * `time` and `timetz` are **decode-only**. They decode to a
//!   `time::DateTime` whose civil fields are the time of day (on the Unix
//!   epoch day); there is no encoder for them (use `Timestamp` / `Date` to
//!   write temporal values).
//! * A single backend message, and a single binary `COPY` row, is bounded by
//!   `max_message_len` (128 MiB by default). A larger message or row is a
//!   decode error rather than unbounded buffering.
//! * `COPY ... FROM STDIN` and `COPY ... TO STDOUT` can only be run through
//!   [`copy_in!`](crate::copy_in) / [`copy_out!`](crate::copy_out). Running one
//!   through `query!` / `query_as!` or `execute_script` is an error; the client
//!   leaves COPY mode and keeps the connection usable.
//! * SCRAM authentication is capped at `max_scram_iterations` iterations
//!   (2,000,000 by default); the server's PBKDF2 work runs on the blocking
//!   pool, so it never stalls an async worker. The password is prepared with
//!   SASLprep before key derivation.
//! * Not yet supported: SCRAM channel binding (`SCRAM-SHA-256-PLUS`),
//!   multi-host connection URLs, the `numeric` / `decimal` type, and streaming
//!   replication (`CopyBothResponse`).
//! * [`Connection::notifications`] delivers notifications through a bounded
//!   broadcast channel; a subscriber that falls behind misses the oldest
//!   notifications instead of buffering them without bound.
//!
//! The examples below use a fictional schema, so they are marked `ignore`
//! (the `query!` macro checks the SQL against a live database at build time).
//! The same code paths are exercised by the crate's integration tests.
//!
//! # JSON
//!
//! The preferred way to map a Rust type to a `json` / `jsonb` column is
//! `#[derive(Json)]`, together with serde's `Serialize` and `Deserialize`:
//!
//! ```ignore
//! use postgresql::{FromRow, Json, query, query_as};
//!
//! #[derive(serde::Serialize, serde::Deserialize, Json, Debug)]
//! struct User {
//!     name: String,
//!     admin: bool,
//! }
//!
//! #[derive(FromRow, Debug)]
//! struct Something {
//!     id: i32,
//!     name: String,
//!     user: User,
//! }
//!
//! # async fn run(pool: postgresql::Pool, something: Something) -> Result<(), postgresql::Error> {
//! let rows: Vec<Something> = query_as!(Something, "SELECT id, name, user FROM something")
//!     .fetch_all(&pool)
//!     .await?;
//! query!("UPDATE something SET user = $1", something.user)
//!     .execute(&pool)
//!     .await?;
//! # let _ = rows;
//! # Ok(())
//! # }
//! ```
//!
//! `#[derive(Json)]` makes the type usable directly as a field, a parameter, a
//! tuple element, or a whole result column, and its `jsonb` type is checked at
//! compile time like any other. `Option<T>` gives a nullable column and
//! `Vec<T>` a `jsonb[]`.
//!
//! A fully automatic mapping for any `Serialize` / `Deserialize` type is not
//! possible: a blanket `impl<T: DeserializeOwned> FromSql for T` would overlap
//! the impls for `i32`, `String` and every other built-in type, so the mapping
//! has to be opted into. Two forms are provided:
//!
//! * `#[derive(Json)]` — preferred; see above.
//! * [`Json<T>`] — for cases the derive cannot cover: a type that
//!   already derives [`FromRow`] (deriving `Json` too would define `FromRow`
//!   and [`RowShape`] twice), or a value with no nameable type, such as
//!   `HashMap<String, i32>`:
//!
//! ```ignore
//! # use std::collections::HashMap;
//! use postgresql::{FromRow, Json, query};
//!
//! #[derive(FromRow)]
//! struct Something {
//!     id: i32,
//!     attributes: Json<HashMap<String, i32>>,
//! }
//!
//! # async fn run(pool: postgresql::Pool, something: Something) -> Result<(), postgresql::Error> {
//! query!("UPDATE something SET attributes = $1", something.attributes)
//!     .execute(&pool)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Both forms encode as `jsonb` (with the usual version byte) and accept both
//! `json` and `jsonb` columns on decode. Writing to an actual `json` (not
//! `jsonb`) column needs the same SQL cast as `serde_json::Value`
//! (`$1::jsonb::json`).
//!
//! # Compile-time checking
//!
//! When `DATABASE_URL` is set at build time and `STDX_POSTGRESQL_CHECK` is not
//! `false`, the query and COPY macros connect to the database and check the
//! statement. `query!` / `query_as!` verify the SQL, the number and types of
//! parameters, and the result columns (names, types, nullability) against the
//! target type. `copy_in!` / `copy_out!` verify the COPY column list against
//! the target type by resolving the relation in `pg_catalog`. A mismatch is a
//! compile error. When the SQL is not a string literal, or checking is
//! disabled, the code compiles and is checked at runtime instead.
//!
//! # Authentication
//!
//! SCRAM-SHA-256 is used for password authentication; MD5 is rejected. The
//! server's SCRAM challenge is validated (bounded iteration count and salt,
//! nonce, server signature), so a hostile endpoint cannot make the client do
//! unbounded work during login.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate self as postgresql;

mod array;
mod config;
mod connection;
mod copy;
mod decode;
mod encode;
mod error;
mod interval;
mod json;
mod pool;
mod query;
mod row;
mod shape;
mod stream;
mod transaction;
pub mod types;

pub use array::{Array, array};
pub use config::{Config, PoolConfig, SslMode};
pub use connection::{CancelToken, Connection, Notification, RowStream, encode_copy_row};
pub use copy::{CopyIn, CopyOut, FromCopyRow, ToRow};
pub use decode::FromSql;
pub use encode::{Date, IsNull, StaticType, Timestamp, ToSql};
pub use error::Error;
pub use interval::Interval;
pub use json::Json;
pub use pool::{Pool, PooledConnection};
pub use postgresql_derive::{FromRow, Json, ToRow};
pub use postgresql_macros::{copy_in, copy_out, query, query_as};
pub use query::{Executor, Query, QueryAs, ScriptExecutor};
pub use row::{Column, Columns, FromRow, Row};
pub use shape::{ColumnSpec, RowShape};
pub use transaction::Transaction;

/// Items used by macro expansions. Not part of the stable public API.
#[doc(hidden)]
pub mod __private {
    pub use bytes::{Bytes, BytesMut};
    pub use serde;

    use crate::{
        connection::Connection,
        copy::{CopyIn, CopyOut, FromCopyRow, ToRow},
        encode::{StaticType, ToSql},
        error::Error,
        types::Oid,
    };
    pub use crate::{
        decode::{json_accepts, json_from_sql},
        encode::json_encode,
        shape::verify_shape,
        types::JSONBOID,
    };

    /// Starts a typed binary `COPY ... FROM STDIN` (used by `copy_in!`).
    pub async fn copy_in<T: ToRow>(conn: &Connection, sql: &str) -> Result<CopyIn<T>, Error> {
        conn.copy_in(sql).await.map(CopyIn::new)
    }

    /// Starts a typed binary `COPY ... TO STDOUT` (used by `copy_out!`).
    pub async fn copy_out<T: FromCopyRow>(conn: &Connection, sql: &str, oids: &[Oid]) -> Result<CopyOut<T>, Error> {
        let max_row_len = conn.max_message_len();
        conn.copy_out(sql)
            .await
            .map(|stream| CopyOut::new(stream, oids.to_vec(), max_row_len))
    }

    /// Compile-time check that a parameter's Rust type matches the OID the
    /// server inferred for the placeholder.
    ///
    /// Emitted once per macro argument; a mismatch is a compile error.
    pub fn check_param<const EXPECTED: Oid, T: StaticType>(_value: &T) {
        const {
            assert!(<T as StaticType>::OID == EXPECTED, "parameter type does not match the query");
        }
    }

    /// Borrows a parameter, checking at compile time that its Rust type matches
    /// the server-inferred OID, and returns a reference suitable for binding.
    pub fn param<const EXPECTED: Oid, T: ToSql + StaticType>(value: &T) -> &T {
        const {
            assert!(<T as StaticType>::OID == EXPECTED, "parameter type does not match the query");
        }
        value
    }

    /// Benchmark helper: builds shared column metadata for `oids`. Not part of
    /// the stable API and not for use outside benchmarks.
    pub fn bench_columns(oids: &[Oid]) -> std::sync::Arc<crate::row::Columns> {
        let fields = oids
            .iter()
            .map(|&type_oid| postgresql_protocol::backend::FieldDescription {
                name: String::new(),
                table_oid: 0,
                column_attr: 0,
                type_oid,
                type_size: -1,
                type_mod: -1,
                format: 1,
            })
            .collect::<Vec<_>>();
        crate::row::Columns::from_fields(&fields)
    }

    /// Benchmark helper: parses a raw `DataRow` payload. Not part of the stable
    /// API and not for use outside benchmarks.
    pub fn bench_parse_row(columns: std::sync::Arc<crate::row::Columns>, payload: &Bytes) -> Result<crate::Row, Error> {
        crate::row::Row::parse(columns, payload, time::TimeZone::UTC)
    }
}
