Fork of <https://github.com/oschwald/maxminddb-rust>.

# Rust MaxMind DB Reader #

This library reads the MaxMind DB file format, including the GeoIP2 and
GeoLite2 databases published by MaxMind.

## Usage ##

stdx crates are used directly from git; there is no crates.io release.

```toml
[dependencies]
maxminddb = { git = "https://github.com/rust-stdx/stdx", branch = "main" }
```

Open a database and look up an address:

```rust
use maxminddb::geoip2;
use std::net::IpAddr;

let reader = maxminddb::Reader::open_readfile("GeoIP2-City.mmdb")?;
let ip: IpAddr = "89.160.20.128".parse()?;
let city: geoip2::City = reader.lookup(ip)?;
println!("{:?}", city);
```

`Reader::from_source` accepts any buffer that implements `AsRef<[u8]>`, which
is useful when the database is already in memory or memory mapped by the
caller.

To iterate over every network in the database, use `Reader::within`. See
`examples/lookup.rs`, `examples/within.rs`, and `examples/write_database.rs`.

## Resource limits ##

A MaxMind DB encodes values with pointers, and many pointers may share one
target, so a small record can describe a very large or deeply nested structure
when expanded. To keep hostile or corrupt databases from consuming unbounded
time or memory, decoding is bounded by `DecodeLimits`:

* a maximum nesting depth of `512` (maps, arrays, and followed pointers),
* a maximum of `65_536` decoded values per operation, and
* a maximum of `2 MiB` of string and byte payload per operation.

These are the values recommended by the MaxMind DB specification. When a limit
is exceeded, decoding fails with `MaxMindDBError::ResourceLimit` instead of
panicking or exhausting memory. The limits apply to metadata decoding at open
as well as to record lookups.

Use `Reader::from_source_with_options` or `Reader::open_readfile_with_options`
with a `ReaderOptions` value to configure the limits.

A database that is a valid IPv4-only file but is looked up with an IPv6 address
or CIDR is rejected with `MaxMindDBError::InvalidInputError`; the address family
must match the database.

## Supported databases ##

Both IPv4 and IPv6 databases are supported, as are the 24-, 28-, and 32-bit
search tree record sizes. The `geoip2` module provides typed records for the
common GeoIP2 and GeoLite2 database shapes. For other databases, deserialize
into your own type or into `maxminddb::Value`.

## License ##

This is free software, licensed under the ISC license.
