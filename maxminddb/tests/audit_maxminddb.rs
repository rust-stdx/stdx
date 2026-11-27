//! Audit tests for `maxminddb`.
//!
//! The crate contains no active `unsafe` (only a commented-out mmap example),
//! so these tests (a) enforce that with a source scan, (b) exercise the decoder
//! resource limits at open time, and (c) throw truncated / bit-flipped / marker
//! only inputs at `Reader::from_source` and prove malformed input is reported as
//! an error rather than panicking.
//!
//! The existing unit tests already cover individual lookups, the known-bad
//! corpus, and record-level resource limits. This file keeps the source scan
//! and the open-time limit/fuzz coverage; it drops the duplicated
//! "missing file" and hard-coded database-list tests.
//!
//! The real-database tests need the `test-data` submodule. If it is not
//! checked out they print a skip message and return instead of failing.

use std::{net::IpAddr, path::Path};

use ipnetwork::IpNetwork;
use maxminddb::{DecodeLimits, MaxMindDBError, Reader, ReaderOptions, Value};

const CITY: &str = "test-data/test-data/GeoIP2-City-Test.mmdb";

fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}

/// The database submodule is optional; skip data-dependent tests when absent.
fn city_data_available() -> bool {
    let available = Path::new(CITY).is_file();
    if !available {
        eprintln!("skipping: {CITY} not found (test-data submodule not checked out)");
    }
    available
}

#[test]
fn no_active_unsafe_in_source() {
    let mut offenders = Vec::new();
    for entry in std::fs::read_dir("src").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        for (i, line) in text.lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") || code.starts_with("///") || code.starts_with("//!") {
                continue;
            }
            if code.contains("unsafe") {
                offenders.push(format!("{}:{}: {line}", path.display(), i + 1));
            }
        }
    }
    assert!(offenders.is_empty(), "active unsafe found:\n{}", offenders.join("\n"));
}

#[test]
fn real_database_lookup_and_iteration() {
    if !city_data_available() {
        return;
    }
    let reader = Reader::open_readfile(CITY).unwrap();
    assert_eq!(reader.metadata.ip_version, 6);
    assert!(matches!(reader.metadata.record_size, 24 | 28 | 32));
    assert!(reader.metadata.node_count > 0);

    let city: maxminddb::geoip2::City<'_> = reader.lookup(ip("89.160.20.128")).unwrap();
    let names = city.city.as_ref().and_then(|c| c.names.as_ref()).unwrap();
    assert_eq!(names.get("en"), Some(&"Linköping"));

    // Iterating a prefix walks the search tree and decodes every record.
    let net: IpNetwork = "89.160.20.0/24".parse().unwrap();
    let iter: maxminddb::Within<'_, Value, _> = reader.within(net).unwrap();
    let mut count = 0usize;
    for item in iter {
        item.unwrap();
        count += 1;
    }
    assert!(count > 0);
}

#[test]
fn all_shipped_test_databases_open() {
    let dir = Path::new("test-data/test-data");
    if !dir.is_dir() {
        eprintln!("skipping: {dir:?} not found (test-data submodule not checked out)");
        return;
    }
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("mmdb") {
            continue;
        }
        match Reader::open_readfile(&path) {
            Ok(reader) => {
                // A successful open must not be followed by a panic anywhere.
                let _ = reader.lookup::<Value>(ip("1.2.3.4"));
                if let Ok(iter) = reader.within::<Value>(IpNetwork::V6("::/0".parse().unwrap())) {
                    for (n, item) in iter.enumerate() {
                        let _ = item;
                        if n > 10_000 {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                // Known-broken files in the corpus are expected to fail cleanly.
                assert!(
                    matches!(
                        e,
                        MaxMindDBError::InvalidDatabaseError(_)
                            | MaxMindDBError::DecodingError(_)
                            | MaxMindDBError::ResourceLimit(_)
                            | MaxMindDBError::IoError(_)
                    ),
                    "{}: {e:?}",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn resource_limits_are_enforced_at_open() {
    if !city_data_available() {
        return;
    }
    let buf = std::fs::read(CITY).unwrap();
    let cases = [
        DecodeLimits {
            max_depth: 0,
            max_values: 65_536,
            max_payload_bytes: 2 * 1024 * 1024,
        },
        DecodeLimits {
            max_depth: 512,
            max_values: 1,
            max_payload_bytes: 2 * 1024 * 1024,
        },
        DecodeLimits {
            max_depth: 512,
            max_values: 65_536,
            max_payload_bytes: 1,
        },
        DecodeLimits {
            max_depth: 1,
            max_values: 65_536,
            max_payload_bytes: 2 * 1024 * 1024,
        },
        DecodeLimits {
            max_depth: 0,
            max_values: 0,
            max_payload_bytes: 0,
        },
    ];
    for limits in cases {
        let err = Reader::from_source_with_options(
            buf.clone(),
            ReaderOptions {
                limits,
            },
        )
        .unwrap_err();
        assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{limits:?}: {err:?}");
    }
    // The defaults open the same bytes fine.
    Reader::from_source_with_options(buf, ReaderOptions::default()).unwrap();
}

#[test]
fn truncated_and_mutated_databases_never_panic() {
    if !city_data_available() {
        return;
    }
    let original = std::fs::read(CITY).unwrap();

    let check = |buf: Vec<u8>| match Reader::from_source(buf) {
        Ok(reader) => {
            for addr in ["1.2.3.4", "89.160.20.128", "2001:db8::1"] {
                let _ = reader.lookup::<Value>(ip(addr));
            }
            if let Ok(iter) = reader.within::<Value>(IpNetwork::V6("::/0".parse().unwrap())) {
                for (n, item) in iter.enumerate() {
                    let _ = item;
                    if n > 2_000 {
                        break;
                    }
                }
            }
        }
        Err(e) => {
            let _ = e.to_string();
        }
    };

    // Every boundary length around the metadata marker and the first/last bytes.
    for len in [
        0usize,
        1,
        2,
        8,
        13,
        14,
        16,
        28,
        29,
        64,
        128,
        256,
        512,
        1024,
        original.len() / 4,
        original.len() / 2,
        original.len().saturating_sub(64),
        original.len().saturating_sub(16),
        original.len().saturating_sub(1),
    ] {
        check(original[..len.min(original.len())].to_vec());
    }

    // Deterministic truncations across the whole file.
    let mut state = 0xdead_beef_0bad_f00du64;
    for _ in 0..300 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let len = (state as usize) % (original.len() + 1);
        check(original[..len].to_vec());
    }

    // Single-bit flips, including inside the metadata and search tree.
    for _ in 0..400 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let pos = (state as usize) % original.len();
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let bit = (state % 8) as u8;
        let mut buf = original.clone();
        buf[pos] ^= 1 << bit;
        check(buf);
    }

    // Trailing garbage after a valid database.
    let mut extended = original.clone();
    extended.extend_from_slice(&[0xff; 32]);
    check(extended);
    let mut extended = original.clone();
    extended.extend_from_slice(b"\xab\xcd\xefMaxMind.com");
    check(extended);
}

#[test]
fn empty_and_marker_only_inputs() {
    for buf in [
        Vec::new(),
        b"\xab\xcd\xefMaxMind.com".to_vec(),
        b"\xab\xcd\xefMaxMind.com\x00".to_vec(),
        vec![0u8; 1024],
        vec![0xffu8; 1024],
    ] {
        let _ = Reader::from_source(buf);
    }
}
