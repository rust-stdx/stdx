# stdx Package Index

Complete catalog of all crates in the stdx monorepo. Import any of these directly from git:

```toml
<name> = { git = "https://github.com/rust-stdx/stdx", branch = "main" }
```

## Encoding & Data Formats

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `base64` | Fast base64 encoding/decoding with SIMD acceleration | `std` (default), `alloc` | — |
| `base32` | Base32 encoding/decoding | `std` (default), `alloc` | — |
| `hex` | Fast hex encoding/decoding with SIMD, constant-time ops, const fn | `std` (default), `alloc` | — |
| `percent_encoding` | Percent encoding/decoding for URLs | `std` (default), `alloc` | servo/rust-url |
| `form_urlencoded` | Parser/serializer for `application/x-www-form-urlencoded` | `std` (default), `alloc` | servo/rust-url |
| `itoa` | Fast integer-to-ASCII conversion | — | dtolnay/itoa |
| `ryu` | Fast floating point to string conversion | `std`, `alloc` | dtolnay/ryu |
| `html_escape` | HTML entity encoding/escaping | — | — |

## Error Handling

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `anyerr` | Flexible concrete Error type built on `std::error::Error` (anyhow fork) | `std` (default) | dtolnay/anyhow |
| `thiserror` | `derive(Error)` macro for structured error types | `std` | dtolnay/thiserror |

## Cryptography & Security

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `crypto` | Hashing (SHA-2, BLAKE3), PBKDF2, constant-time ops | `std` (default), `zeroize` (default), `alloc` | — |
| `constant_time_eq` | Constant-time byte string comparison | `std` | cesarb/constant_time_eq |

## Concurrency & Async

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `singleflight` | Deduplicate concurrent async function calls by key (requires tokio) | — | — |
| `errgroup` | Async task group with error propagation and optional concurrency limiting (requires tokio) | — | — |
| `retry` | Retry async operations with configurable backoff strategies (requires tokio) | — | — |
| `scheduler` | Task scheduler with tracing support (requires tokio) | `tracing` (default) | — |
| `pin_project_lite` | Safe pin projections with minimal overhead | `std`, `alloc` | — |

## Networking & HTTP

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `ipnetwork` | IP CIDR parsing and manipulation | — | achanda/ipnetwork |
| `httpdate` | HTTP date parsing and formatting (RFC 7231) | `std` | pyfisch/httpdate |
| `net` | Networking utilities | — | — |
| `hostname` | Get the system hostname | — | — |
| `json_rpc` | JSON-RPC 2.0 types and serialization | `std` | — |
| `acme` | ACME protocol client for certificate automation | — | instant-labs/instant-acme |
| `hyper_utils` | Utilities for the Hyper HTTP framework | — | — |

## Serialization

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `serde_yaml` | YAML data format for Serde | `std` | dtolnay/serde-yaml |
| `serde_urlencoded` | URL-encoded form data for Serde | `std` | nox/serde_urlencoded |
| `csv` | CSV reading and writing (no_std compatible) | `std` (default), `alloc` | — |

## Database

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `pg` | PostgreSQL client helpers | — | — |
| `pg_derive` | Derive macros for PostgreSQL types | — | — |

## File & Data Formats

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `uuid` | UUID generation, encoding, and decoding (RFC 9562) | `std` | uuid-rs/uuid |
| `semver` | Semantic Versioning 2.0 parser and comparator | — | — |
| `maxminddb` | Read MaxMind DB format (GeoIP2, GeoLite2) | `std` | oschwald/maxminddb-rust |
| `embed` | Compile-time file embedding (loads from fs during dev) | `std` | pyrossh/rust-embed |
| `mime_guess` | MIME type detection by file extension | `std`, `rev-mappings` (default) | abonander/mime_guess |
| `tld` | Public suffix / TLD parsing | — | rushmorem/publicsuffix |
| `countries` | ISO country code data and lookups | — | — |
| `cron` | Cron expression parser and schedule explorer | `std` | zslayton/cron |
| `bel` | Better Expression Language parser and interpreter | `std`, `regex` (default), `time` (default), `ip` (default) | cel-rust/cel-rust |
| `quic` | QUIC transport protocol utilities | — | — |

## Configuration & Environment

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `dotenv` | Load environment variables from `.env` files, with `FromEnv` derive macro | — | — |
| `cfg_if` | `cfg_if!` macro for conditional compilation | `std` | — |
| `getopts` | Command-line option parsing | `std` | — |

## Strings & Text

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `strings` | String utilities (uses itoa + ryu) | — | — |
| `template` | Text template rendering engine | — | — |
| `memmem` | Substring searching (TwoWay, Boyer-Moore) | — | jneem/memmem |
| `small_collections` | Small-string optimization types (SmallString) | `std` (default), `serde` | — |

## Checksums & Hashing

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `crc32fast` | Fast, SIMD-accelerated CRC32 (IEEE) checksum | `std` (default) | srijs/rust-crc32fast |
| `xxhash` | xxHash non-cryptographic hash (XXH32, XXH64, XXH3-64, XXH3-128) | `std` (default) | — |

## Math

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `big_number` | Arbitrary-precision integer arithmetic | `std` (default), `alloc` | — |

## Cloud & Services

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `s3` | Amazon S3-compatible client (requires reqwest) | `reqwest` (default) | — |
| `postmark` | Postmark email API client | — | — |
| `mail_builder` | E-mail message builder | `std` | — |
| `docker` | Docker API client | — | — |

## Developer Tools

| Crate | Description | Features | Forked from |
|-------|-------------|----------|-------------|
| `term` | Terminal utilities (colors, styling) | — | — |
| `single_instance` | Ensure only one instance of an application runs | — | — |
| `num_cpus` | Get the number of CPUs on the machine | `std` | seanmonstar/num_cpus |
| `unsafe_libyaml` | libyaml transpiled to Rust by c2rust | `std` | unsafe-libyaml |

## Feature Flag Guide

- **`std`**: Enables standard library support. Disable for `no_std` / embedded. Usually default-on.
- **`alloc`**: Enables heap allocation. Useful for `no_std`+`alloc` targets.
- **`zeroize`**: Enables memory zeroing for sensitive cryptographic material (crypto crate).
- **`tracing`**: Enables tracing/logging spans (scheduler crate).
- **`reqwest`**: Enables HTTP client support (s3 crate).

### no_std Example

```toml
[dependencies]
base64 = { git = "https://github.com/rust-stdx/stdx", default-features = false, features = ["alloc"] }
csv = { git = "https://github.com/rust-stdx/stdx", default-features = false, features = ["alloc"] }
hex = { git = "https://github.com/rust-stdx/stdx", default-features = false }
```
