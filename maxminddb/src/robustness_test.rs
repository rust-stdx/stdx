//! Robustness tests for hostile and corrupt databases.
//!
//! These tests exercise the decoder's resource limits and the reader's
//! validation. They must never panic, recurse without bound, or exhaust memory;
//! malformed input is reported as an error instead.

use std::net::IpAddr;

use super::{
    DecodeLimits, MaxMindDBError, Reader, ReaderOptions, Value,
    test_support::{Mmdb, Val, db_with_data, db_with_record, enc_val, nested_arrays},
};

fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}

fn path(name: &str) -> String {
    format!("test-data/test-data/{name}")
}

fn bad_path(name: &str) -> String {
    format!("test-data/{name}")
}

// ===== Decoder hardening =====

#[test]
fn extended_type_byte_does_not_overflow() {
    // The extended type byte is added to 7. Bytes 249..=255 used to overflow a
    // u8: a debug build panicked and a release build silently decoded the value
    // as an unrelated type.
    for byte in [249_u8, 250, 254, 255] {
        let db = db_with_record(&Val::Raw(vec![0x00, byte, 0x00]), 4);
        let reader = Reader::from_source(db).unwrap();
        let err = reader.lookup::<Value>(ip("1.2.3.4")).unwrap_err();
        assert!(
            matches!(err, MaxMindDBError::InvalidDatabaseError(_)),
            "type byte {byte}: expected InvalidDatabaseError, got {err:?}"
        );
    }
}

#[test]
fn pointer_cycle_is_bounded() {
    let mut data = Vec::new();
    enc_val(&Val::Ptr(0), &mut data); // points at itself
    let reader = Reader::from_source(db_with_data(data, 4)).unwrap();
    let err = reader.lookup::<Value>(ip("1.2.3.4")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");
}

#[test]
fn long_pointer_chain_is_bounded() {
    let mut data = Vec::new();
    for _ in 0..5_000 {
        let here = data.len();
        // Choose a pointer width consistent with the target it encodes.
        let width = (2..=4_usize)
            .find(|&w| {
                let target = here + w;
                let target_width = if target < 2048 {
                    2
                } else if target < 526_336 {
                    3
                } else {
                    4
                };
                target_width == w
            })
            .unwrap();
        enc_val(&Val::Ptr(here + width), &mut data);
    }
    enc_val(&Val::Str("end"), &mut data);

    let reader = Reader::from_source(db_with_data(data, 4)).unwrap();
    let err = reader.lookup::<Value>(ip("1.2.3.4")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");
}

#[test]
fn deep_nesting_is_bounded() {
    let reader = Reader::from_source(db_with_data(nested_arrays(5_000), 4)).unwrap();
    let err = reader.lookup::<Value>(ip("1.2.3.4")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");
}

#[test]
fn nesting_within_the_default_limit_succeeds() {
    let reader = Reader::from_source(db_with_data(nested_arrays(100), 4)).unwrap();
    assert!(reader.lookup::<Value>(ip("1.2.3.4")).is_ok());
}

#[test]
fn deep_nesting_in_an_ignored_field_is_bounded() {
    // A typed destination skips unknown fields, but skipping must still be
    // bounded by the nesting limit.
    let mut data = Vec::new();
    data.push(0xE1); // map with one entry
    enc_val(&Val::Str("zz_unknown"), &mut data);
    data.extend_from_slice(&nested_arrays(5_000));

    let reader = Reader::from_source(db_with_data(data, 4)).unwrap();
    let err = reader.lookup::<super::geoip2::City>(ip("1.2.3.4")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");
}

#[test]
fn deep_nesting_in_metadata_is_bounded() {
    let db = Mmdb {
        node_count: 1,
        nodes: vec![[0, 0]],
        extra_metadata: vec![("zz_unknown", Val::Raw(nested_arrays(5_000)))],
        ..Default::default()
    }
    .build();

    let err = Reader::from_source(db).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");
}

// ===== Search tree hardening =====

#[test]
fn within_a_cyclic_search_tree_returns_an_error() {
    let db = Mmdb {
        node_count: 1,
        nodes: vec![[0, 0]], // both children point back at node 0
        ..Default::default()
    }
    .build();

    let reader = Reader::from_source(db).unwrap();
    let mut iter = reader.within::<Value>("0.0.0.0/0".parse().unwrap()).unwrap();
    let item = iter.next().expect("the iterator should report the corrupt tree");
    assert!(matches!(item, Err(MaxMindDBError::InvalidDatabaseError(_))), "{item:?}");
}

#[test]
fn within_broken_search_tree_does_not_panic() {
    let reader = Reader::open_readfile(path("MaxMind-DB-test-broken-search-tree-24.mmdb")).unwrap();
    let iter = reader.within::<Value>("0.0.0.0/0".parse().unwrap()).unwrap();
    // Iterating must terminate with an error rather than panic or loop.
    for item in iter.take(1_000) {
        let _ = item;
    }
}

// ===== IPv4/IPv6 family validation =====

#[test]
fn within_a_low_ipv6_network_does_not_underflow() {
    // Regression: an IPv6 network below the IPv4-mapped range must be reported
    // as IPv6 instead of underflowing `prefix - 96`.
    let reader = Reader::open_readfile(path("MaxMind-DB-no-ipv4-search-tree.mmdb")).unwrap();
    let mut iter = reader.within::<String>("::/0".parse().unwrap()).unwrap();
    let first = iter.next().unwrap().unwrap();
    assert_eq!(first.ip_net.to_string(), "::/64");
}

#[test]
fn ipv6_lookup_in_an_ipv4_database_is_rejected() {
    let reader = Reader::open_readfile(path("MaxMind-DB-test-ipv4-24.mmdb")).unwrap();
    let err = reader.lookup::<Value>(ip("101:101::")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::InvalidInputError(_)), "{err:?}");
}

#[test]
fn ipv6_cidr_in_an_ipv4_database_is_rejected() {
    let reader = Reader::open_readfile(path("MaxMind-DB-test-ipv4-24.mmdb")).unwrap();
    let err = reader.within::<Value>("::/0".parse().unwrap()).unwrap_err();
    assert!(matches!(err, MaxMindDBError::InvalidInputError(_)), "{err:?}");
}

// ===== Metadata validation =====

#[test]
fn metadata_validation_rejects_invalid_fields() {
    let cases: Vec<(&str, Mmdb)> = vec![
        (
            "unsupported major version",
            Mmdb {
                meta_major_version: Some(3),
                ..valid_db()
            },
        ),
        (
            "unsupported ip version",
            Mmdb {
                meta_ip_version: Some(5),
                ..valid_db()
            },
        ),
        (
            "unsupported record size",
            Mmdb {
                meta_record_size: Some(20),
                ..valid_db()
            },
        ),
        (
            "zero node count",
            Mmdb {
                node_count: 0,
                nodes: vec![],
                meta_node_count: Some(0),
                ..Default::default()
            },
        ),
        (
            "search tree larger than the file",
            Mmdb {
                meta_node_count: Some(100_000),
                ..valid_db()
            },
        ),
    ];

    for (label, db) in cases {
        let err = Reader::from_source(db.build()).unwrap_err();
        assert!(
            matches!(err, MaxMindDBError::InvalidDatabaseError(_)),
            "{label}: expected InvalidDatabaseError, got {err:?}"
        );
    }
}

fn valid_db() -> Mmdb {
    let node_count = 1;
    Mmdb {
        node_count,
        nodes: vec![[
            super::test_support::data_ptr(node_count, 0),
            super::test_support::data_ptr(node_count, 0),
        ]],
        data: vec![0x00],
        ..Default::default()
    }
}

// ===== Configurable limits =====

#[test]
fn resource_limits_are_configurable() {
    let db = db_with_data(nested_arrays(50), 4);

    // Default limits accept the database.
    let reader = Reader::from_source(db.clone()).unwrap();
    assert!(reader.lookup::<Value>(ip("1.2.3.4")).is_ok());

    // A tighter depth limit rejects it.
    let tight = ReaderOptions {
        limits: DecodeLimits {
            max_depth: 10,
            ..Default::default()
        },
    };
    let reader = Reader::from_source_with_options(db.clone(), tight).unwrap();
    let err = reader.lookup::<Value>(ip("1.2.3.4")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");

    // A more generous depth limit accepts a structure slightly deeper than the
    // default would allow. Raising the limit increases decoder stack usage, so
    // keep the increase modest.
    let generous = ReaderOptions {
        limits: DecodeLimits {
            max_depth: 700,
            ..Default::default()
        },
    };
    let deep = db_with_data(nested_arrays(600), 4);
    let reader = Reader::from_source_with_options(deep, generous).unwrap();
    assert!(reader.lookup::<Value>(ip("1.2.3.4")).is_ok());

    // The value budget applies independently of depth. Keep it large enough for
    // the metadata to open but small enough to reject the record.
    let value_tight = ReaderOptions {
        limits: DecodeLimits {
            max_values: 100,
            ..Default::default()
        },
    };
    let reader = Reader::from_source_with_options(db_with_data(nested_arrays(200), 4), value_tight).unwrap();
    let err = reader.lookup::<Value>(ip("1.2.3.4")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");

    // The payload budget bounds string materialization. The metadata fits, the
    // record's string does not.
    let mut raw = vec![0x5E, 0x00, 0x0F]; // string of size 285 + 15 = 300
    raw.extend_from_slice(&[b'x'; 300]);
    let payload_tight = ReaderOptions {
        limits: DecodeLimits {
            max_payload_bytes: 200,
            ..Default::default()
        },
    };
    let reader = Reader::from_source_with_options(db_with_data(raw, 4), payload_tight).unwrap();
    let err = reader.lookup::<String>(ip("1.2.3.4")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");
}

#[test]
fn custom_payload_limit_accepts_large_strings() {
    let mut data = Vec::new();
    enc_val(&Val::Str("0123456789"), &mut data);
    let opts = ReaderOptions {
        limits: DecodeLimits {
            max_payload_bytes: 1024,
            ..Default::default()
        },
    };
    let reader = Reader::from_source_with_options(db_with_data(data, 4), opts).unwrap();
    let s: String = reader.lookup(ip("1.2.3.4")).unwrap();
    assert_eq!(s, "0123456789");
}

// ===== Corpus resource-limit boundaries =====

#[test]
fn corpus_accepts_records_at_the_recommended_limits() {
    for name in [
        "MaxMind-DB-test-decoder-value-limit.mmdb",
        "MaxMind-DB-test-decoder-value-limit-pointer-heavy.mmdb",
        "MaxMind-DB-test-decoder-payload-limit.mmdb",
    ] {
        let reader = Reader::open_readfile(path(name)).unwrap();
        let mut decoded = 0;
        for item in reader.within::<Value>("0.0.0.0/0".parse().unwrap()).unwrap() {
            match item {
                Ok(_) => decoded += 1,
                Err(e) => panic!("{name} should decode within the default limits, got {e:?}"),
            }
        }
        assert!(decoded > 0, "{name} should contain decodable records");
    }
}

#[test]
fn corpus_rejects_records_over_the_recommended_limits() {
    for name in [
        "MaxMind-DB-test-decoder-value-limit-over.mmdb",
        "MaxMind-DB-test-decoder-payload-limit-over.mmdb",
    ] {
        let reader = Reader::open_readfile(path(name)).unwrap();
        let mut limited = false;
        for item in reader.within::<Value>("0.0.0.0/0".parse().unwrap()).unwrap() {
            match item {
                Ok(_) => {}
                Err(MaxMindDBError::ResourceLimit(_)) => limited = true,
                Err(e) => panic!("{name}: unexpected error {e:?}"),
            }
        }
        assert!(limited, "{name} should exceed a resource limit");
    }

    let err = Reader::open_readfile(path("MaxMind-DB-test-metadata-payload-limit.mmdb")).unwrap_err();
    assert!(matches!(err, MaxMindDBError::ResourceLimit(_)), "{err:?}");
}

#[test]
fn corpus_dos_fixtures_are_bounded() {
    for name in [
        "MaxMind-DB-test-pointer-decoder-dos.mmdb",
        "MaxMind-DB-test-pointer-decoder-dos-ipv6.mmdb",
        "MaxMind-DB-test-payload-amplification-dos.mmdb",
        "MaxMind-DB-test-payload-amplification-dos-string.mmdb",
        "MaxMind-DB-test-payload-amplification-dos-worst-case.mmdb",
    ] {
        let reader = Reader::open_readfile(path(name)).unwrap();
        // The first record exceeds a limit; decoding must stop there instead of
        // expanding the entry (some of these files describe gigabytes).
        let mut iter = reader.within::<Value>("0.0.0.0/0".parse().unwrap()).unwrap();
        let first = iter.next();
        assert!(
            matches!(first, Some(Err(MaxMindDBError::ResourceLimit(_)))),
            "{name}: expected the first record to hit a limit, got {first:?}"
        );
    }
}

// ===== Corrupt database corpus =====

#[test]
fn bad_data_databases_survive_iteration() {
    let bad_dbs = [
        "bad-data/libmaxminddb/libmaxminddb-corrupt-search-tree.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-deep-array-nesting.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-deep-nesting.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-empty-array-last-in-metadata.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-empty-map-last-in-metadata.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-metadata-marker-only.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-offset-integer-overflow.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-oversized-array.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-oversized-map.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-separator-record-max-left.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-separator-record-min-left.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-separator-record-min-right.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-uint64-max-epoch.mmdb",
        "bad-data/maxminddb-golang/cyclic-data-structure.mmdb",
        "bad-data/maxminddb-golang/invalid-bytes-length.mmdb",
        "bad-data/maxminddb-golang/invalid-data-record-offset.mmdb",
        "bad-data/maxminddb-golang/invalid-map-key-length.mmdb",
        "bad-data/maxminddb-golang/invalid-string-length.mmdb",
        "bad-data/maxminddb-golang/metadata-is-an-uint128.mmdb",
        "bad-data/maxminddb-golang/unexpected-bytes.mmdb",
        "bad-data/maxminddb-python/bad-unicode-in-map-key.mmdb",
    ];

    for db in &bad_dbs {
        let buf = std::fs::read(bad_path(db)).expect("bad-data file should exist");
        // Opening may fail cleanly; if it succeeds, iteration and lookups must
        // not panic, overflow the stack, or allocate without bound.
        if let Ok(reader) = Reader::from_source(buf) {
            let _ = reader.lookup::<Value>(ip("1.1.1.1"));
            let _ = reader.lookup::<Value>(ip("::1"));
            for cidr in ["0.0.0.0/0", "::/0"] {
                let cidr: ipnetwork::IpNetwork = cidr.parse().unwrap();
                if let Ok(iter) = reader.within::<Value>(cidr) {
                    for item in iter.take(1_000) {
                        let _ = item;
                    }
                }
            }
        }
    }
}
