#![cfg(all(feature = "alloc", feature = "derive"))]

use cbor::{Decode, Encode};

#[test]
fn value_canonicalization() {
    let mut value = cbor::Value::Map(cbor::Map::from_iter([
        ("z".into(), cbor::Value::from(1u8)),
        ("aa".into(), cbor::Value::from(2u8)),
    ]));
    value.canonicalize();
    let bytes = cbor::to_canonical_vec(&value);
    assert!(cbor::validate_canonical(&bytes).is_ok());
    let back: cbor::Value = cbor::from_slice(&bytes).unwrap();
    assert_eq!(back, value);
}

#[test]
fn bignum_normalization() {
    let value = cbor::Value::Bignum(cbor::Bignum::new(false, vec![0, 0, 0x01, 0x00]));
    let mut value = value;
    value.canonicalize();
    assert_eq!(value, cbor::Value::Integer(cbor::Integer::Unsigned(256)));
    assert_eq!(cbor::Bignum::new(false, vec![0, 0]).bytes, Vec::<u8>::new());
}

#[test]
fn encoded_len_and_slice() {
    let value = ("hello", 42u64, vec![1u8, 2, 3]);
    let len = cbor::encoded_len(&value);
    let mut buffer = vec![0u8; len];
    let used = cbor::to_slice(&value, &mut buffer).unwrap();
    assert_eq!(used.len(), len);
    assert_eq!(&buffer[..len], cbor::to_vec(&value));
    let mut small = [0u8; 1];
    assert!(cbor::to_slice(&value, &mut small).is_err());
}

#[test]
fn skip_all_item_kinds() {
    let items: &[&[u8]] = &[
        &[0x00],
        &[0x20],
        &[0xf4],
        &[0xf5],
        &[0xf6],
        &[0xf7],
        &[0xf0],
        &[0xf9, 0x3e, 0x00],
        &[0x41, 0x00],
        &[0x61, b'a'],
        &[0x82, 0x01, 0x02],
        &[0xa1, 0x01, 0x02],
        &[0xc1, 0x1a, 0x65, 0x53, 0xf1, 0x00],
        &[0x5f, 0x41, 0x00, 0xff],
        &[0x7f, 0x61, b'a', 0xff],
        &[0x9f, 0x01, 0x02, 0xff],
        &[0xbf, 0x01, 0x02, 0xff],
    ];
    for item in items {
        let mut encoded = item.to_vec();
        encoded.extend_from_slice(&[0x00]);
        let mut decoder = cbor::Decoder::new(&encoded);
        decoder.skip().unwrap();
        assert_eq!(decoder.position(), item.len());
        assert!(!decoder.is_empty());
    }
}

#[test]
fn text_and_bytes_iterators() {
    let bytes = [0x7f, 0x63, b'a', b'b', b'c', 0x63, b'd', b'e', b'f', 0xff];
    let mut decoder = cbor::Decoder::new(&bytes);
    let mut iter = decoder.text_iter().unwrap();
    assert_eq!(iter.next().unwrap().unwrap(), "abc");
    assert_eq!(iter.next().unwrap().unwrap(), "def");
    assert!(iter.next().is_none());

    let bytes = [0x5f, 0x42, 1, 2, 0x41, 3, 0xff];
    let mut decoder = cbor::Decoder::new(&bytes);
    let mut iter = decoder.bytes_iter().unwrap();
    assert_eq!(iter.next().unwrap().unwrap(), &[1, 2]);
    assert_eq!(iter.next().unwrap().unwrap(), &[3]);
    assert!(iter.next().is_none());
}

#[test]
fn map_duplicate_policies() {
    let bytes = [0xa2, 0x01, 0x02, 0x01, 0x03];
    let options = cbor::Options {
        duplicate_keys: cbor::DuplicateKeys::First,
        ..Default::default()
    };
    let map: std::collections::BTreeMap<u8, u8> = cbor::from_slice_with_options(&bytes, options).unwrap();
    assert_eq!(map[&1], 2);

    let options = cbor::Options {
        duplicate_keys: cbor::DuplicateKeys::Last,
        ..Default::default()
    };
    let map: std::collections::BTreeMap<u8, u8> = cbor::from_slice_with_options(&bytes, options).unwrap();
    assert_eq!(map[&1], 3);

    let options = cbor::Options {
        duplicate_keys: cbor::DuplicateKeys::Allow,
        ..Default::default()
    };
    let map: cbor::Map<u8, u8> = cbor::from_slice_with_options(&bytes, options).unwrap();
    assert_eq!(map.len(), 2);
}

#[test]
fn tagged_runtime() {
    let value = cbor::TaggedValue {
        tag: cbor::Tag::new(99),
        value: 7u8,
    };
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes, vec![0xd8, 0x63, 0x07]);
    assert_eq!(cbor::from_slice::<cbor::TaggedValue<u8>>(&bytes).unwrap(), value);
}

#[test]
fn simple_values() {
    let simple = cbor::Simple::new(59).unwrap();
    let bytes = cbor::to_vec(&simple);
    assert_eq!(bytes, vec![0xf8, 0x3b]);
    assert_eq!(cbor::from_slice::<cbor::Simple>(&bytes).unwrap(), simple);
    assert!(cbor::Simple::new(20).is_err());
}

#[test]
fn full_integer_ranges() {
    for value in [u64::MAX, u64::MAX - 1, 1 << 63, (1 << 63) + 1] {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<u64>(&bytes).unwrap(), value);
    }
    for value in [i64::MIN, i64::MIN + 1, -1, i64::MAX] {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<i64>(&bytes).unwrap(), value);
    }
    let bytes = cbor::to_vec(&i128::MIN);
    assert_eq!(cbor::from_slice::<i128>(&bytes).unwrap(), i128::MIN);
}

#[test]
fn sequences() {
    let mut bytes = cbor::to_vec(&1u8);
    bytes.extend(cbor::to_vec("two"));
    bytes.extend(cbor::to_vec(&(3u8, 4u8)));
    assert!(cbor::validate(&bytes).is_err());
    assert!(cbor::validate_sequence(&bytes).is_ok());
    let mut decoder = cbor::Decoder::new(&bytes);
    assert_eq!(decoder.decode::<u8>().unwrap(), 1);
    assert_eq!(decoder.decode::<String>().unwrap(), "two");
    assert_eq!(decoder.decode::<(u8, u8)>().unwrap(), (3, 4));
    assert!(decoder.is_empty());
}

#[cfg(all(feature = "std", feature = "serde"))]
#[test]
fn reader_reads_one_item_at_a_time() {
    let mut bytes = cbor::to_vec(&1u8);
    bytes.extend(cbor::to_vec("two"));
    let mut reader = std::io::BufReader::new(std::io::Cursor::new(bytes));
    let first: u8 = cbor::serde::from_reader(&mut reader).unwrap();
    let second: String = cbor::serde::from_reader(&mut reader).unwrap();
    assert_eq!(first, 1);
    assert_eq!(second, "two");
}

#[cfg(feature = "alloc")]
#[test]
fn owned_raw_value() {
    let raw = cbor::RawValueBuf::new(cbor::to_vec(&(1u8, 2u8))).unwrap();
    let bytes = cbor::to_vec(&raw);
    let back: (u8, u8) = raw.deserialize().unwrap();
    assert_eq!(back, (1, 2));
    assert_eq!(bytes, cbor::to_vec(&(1u8, 2u8)));
}

#[cfg(feature = "serde")]
#[test]
fn serde_to_slice_and_size() {
    #[derive(serde::Serialize)]
    struct Value {
        a: u64,
        b: String,
    }
    let value = Value {
        a: 7,
        b: "hi".into(),
    };
    let size = cbor::serde::serialized_size(&value).unwrap();
    let mut buffer = vec![0u8; size];
    let used = cbor::serde::to_slice(&value, &mut buffer).unwrap();
    assert_eq!(used.len(), size);
    assert_eq!(&used[..], &cbor::serde::to_vec(&value).unwrap()[..]);
}

#[cfg(feature = "serde")]
#[test]
fn serde_integer_keys_into_map() {
    let mut map = std::collections::BTreeMap::new();
    map.insert(1u64, "one".to_string());
    map.insert(2u64, "two".to_string());
    let bytes = cbor::serde::to_vec(&map).unwrap();
    let back: std::collections::BTreeMap<u64, String> = cbor::serde::from_slice(&bytes).unwrap();
    assert_eq!(back, map);
}

#[cfg(feature = "serde")]
#[test]
fn derive_and_serde_wire_compatibility() {
    #[derive(Debug, PartialEq, Encode, Decode, serde::Serialize, serde::Deserialize)]
    struct Both {
        a: u64,
        b: String,
    }
    let value = Both {
        a: 1,
        b: "x".into(),
    };
    let typed = cbor::to_vec(&value);
    let serde = cbor::serde::to_vec(&value).unwrap();
    assert_eq!(cbor::from_slice::<Both>(&serde).unwrap(), value);
    assert_eq!(cbor::serde::from_slice::<Both>(&typed).unwrap(), value);
}

#[cfg(feature = "serde")]
#[test]
fn typed_array_and_serde_tuple_are_compatible() {
    #[derive(Debug, PartialEq, Encode, Decode, serde::Serialize, serde::Deserialize)]
    #[cbor(array)]
    struct Both(u8, u16, String);

    let value = Both(1, 2, "x".into());
    let typed = cbor::to_vec(&value);
    let serde = cbor::serde::to_vec(&value).unwrap();
    assert_eq!(typed, serde);
    assert_eq!(cbor::serde::from_slice::<Both>(&typed).unwrap(), value);
    assert_eq!(cbor::from_slice::<Both>(&serde).unwrap(), value);
}

#[cfg(feature = "serde")]
#[test]
fn serde_to_writer_and_prefix() {
    let bytes = cbor::serde::to_vec(&("a", 1u8)).unwrap();
    let mut out = Vec::new();
    cbor::serde::to_writer(&("a", 1u8), &mut out).unwrap();
    assert_eq!(out, bytes);

    let mut sequence = bytes.clone();
    sequence.extend(cbor::serde::to_vec(&2u8).unwrap());
    let ((a, n), rest) = cbor::serde::from_slice_prefix::<(&str, u8)>(&sequence).unwrap();
    assert_eq!((a, n), ("a", 1));
    assert_eq!(cbor::serde::from_slice::<u8>(rest).unwrap(), 2);
}

#[cfg(feature = "serde")]
#[test]
fn serde_struct_map_and_array_accept_both_shapes() {
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct S {
        a: u8,
        b: u8,
    }
    let as_map = cbor::serde::to_vec(&S {
        a: 1,
        b: 2,
    })
    .unwrap();
    let as_array = cbor::serde::to_vec(&(1u8, 2u8)).unwrap();
    assert_eq!(
        cbor::serde::from_slice::<S>(&as_map).unwrap(),
        S {
            a: 1,
            b: 2
        }
    );
    assert_eq!(
        cbor::serde::from_slice::<S>(&as_array).unwrap(),
        S {
            a: 1,
            b: 2
        }
    );
}

#[cfg(feature = "alloc")]
#[test]
fn canonical_sorts_btreemap_keys() {
    let mut map = std::collections::BTreeMap::new();
    map.insert("z".to_string(), 1u8);
    map.insert("aa".to_string(), 2u8);
    let bytes = cbor::to_canonical_vec(&map);
    assert!(cbor::validate_canonical(&bytes).is_ok());
}

#[test]
fn try_canonical_validates_the_output() {
    let value = ("hello", 42u64);
    let bytes = cbor::try_to_canonical_vec(&value).unwrap();
    assert_eq!(bytes, cbor::to_canonical_vec(&value));
    assert!(cbor::validate_canonical(&bytes).is_ok());

    let mut buffer = Vec::new();
    cbor::try_to_canonical_vec_into(&value, &mut buffer).unwrap();
    assert_eq!(buffer, bytes);

    let mut fixed = [0u8; 64];
    let used = cbor::try_to_canonical_slice(&value, &mut fixed).unwrap();
    assert_eq!(used, &bytes[..]);
}

#[test]
fn try_canonical_rejects_duplicate_and_collapsing_keys() {
    let mut map = cbor::Map::<cbor::Value, cbor::Value>::new();
    map.push("a".into(), 1u8.into());
    map.push("a".into(), 2u8.into());
    assert!(cbor::try_to_canonical_vec(&cbor::Value::Map(map)).is_err());

    let mut map = cbor::Map::<cbor::Value, cbor::Value>::new();
    map.push(cbor::Value::Float(f64::from_bits(0x7ff8_0000_0000_0001)), 1u8.into());
    map.push(cbor::Value::Float(f64::from_bits(0x7ff8_0000_0000_0002)), 2u8.into());
    assert!(cbor::try_to_canonical_vec(&cbor::Value::Map(map)).is_err());
}

#[test]
fn try_canonical_rejects_declaration_order_maps() {
    #[derive(Encode)]
    #[cbor(declaration_order)]
    struct Unsorted {
        aa: u8,
        z: u8,
    }

    assert!(
        cbor::try_to_canonical_vec(&Unsorted {
            aa: 1,
            z: 2
        })
        .is_err()
    );
}

#[test]
fn diagnostics_use_rfc_style_floats_and_inline_errors() {
    assert_eq!(cbor::to_diagnostic_string(&[0xf9, 0x3c, 0x00]), "1.0");
    assert_eq!(cbor::to_diagnostic_string(&[0xf9, 0x00, 0x00]), "0.0");
    assert_eq!(cbor::to_diagnostic_string(&[0xf9, 0x80, 0x00]), "-0.0");
    assert_eq!(cbor::to_diagnostic_string(&cbor::to_vec(&1.0e300f64)), "1e300");

    let invalid_utf8 = cbor::to_diagnostic_string(&[0x61, 0xff]);
    assert!(invalid_utf8.contains("!invalid UTF-8"), "{invalid_utf8}");

    let truncated = cbor::to_diagnostic_string(&[0x41]);
    assert!(truncated.contains("!unexpected end of input"), "{truncated}");

    let bytes = cbor::to_vec(&cbor::Value::Bytes(vec![0xde, 0xad]));
    assert_eq!(cbor::to_diagnostic_string(&bytes), "h'dead'");
}

#[test]
fn diagnostics_escape_del_and_c1_controls() {
    let text = "a\u{1b}b\u{7f}c\u{9b}d";
    let bytes = cbor::to_vec(&text);
    let diagnostic = cbor::to_diagnostic_string(&bytes);
    assert!(diagnostic.contains("\\u001b"), "{diagnostic}");
    assert!(diagnostic.contains("\\u007f"), "{diagnostic}");
    assert!(diagnostic.contains("\\u009b"), "{diagnostic}");
    assert!(!diagnostic.contains('\u{7f}'), "{diagnostic}");
    assert!(!diagnostic.contains('\u{9b}'), "{diagnostic}");
}

#[test]
fn documented_nan_and_value_duplicate_behavior() {
    // Map<Value, _> uses PartialEq: NaN != NaN, so two byte-identical NaN keys
    // are kept. This is a documented limitation.
    let nan_keys = [0xa2, 0xf9, 0x7e, 0x00, 0x01, 0xf9, 0x7e, 0x00, 0x02];
    let map: cbor::Map<cbor::Value, u8> = cbor::from_slice(&nan_keys).unwrap();
    assert_eq!(map.len(), 2);

    // Value preserves duplicate keys regardless of Options::duplicate_keys.
    let duplicates = [0xa2, 0x01, 0x02, 0x01, 0x03];
    let value: cbor::Value = cbor::from_slice(&duplicates).unwrap();
    assert_eq!(value.as_map().unwrap().len(), 2);
}

#[test]
fn canonical_validation_documents_the_negative_zero_gap() {
    // RFC 8949 section 5.6.1 treats -0.0 and 0.0 as the same map key, but
    // canonical validation compares deterministic encodings, so both are
    // currently accepted. This is documented.
    let bytes = [0xa2, 0xf9, 0x00, 0x00, 0x01, 0xf9, 0x80, 0x00, 0x02];
    assert!(cbor::validate_canonical(&bytes).is_ok());
}

#[test]
fn duplicate_key_errors_point_at_the_key() {
    // [ {1: 2, 1: 3} ]: the second (duplicate) key starts at offset 4.
    let bytes = [0x81, 0xa2, 0x01, 0x02, 0x01, 0x03];
    let error = cbor::from_slice::<Vec<std::collections::BTreeMap<u8, u8>>>(&bytes).unwrap_err();
    assert_eq!(error.kind(), cbor::ErrorKind::DuplicateKey);
    assert_eq!(error.offset(), 4);
}

#[test]
fn map_duplicate_check_budget_is_enforced() {
    fn encode_map(n: u8) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut encoder = cbor::Encoder::new(&mut bytes);
        encoder.map(usize::from(n)).unwrap();
        for i in 0..n {
            encoder.u8(i).unwrap();
            encoder.u8(i).unwrap();
        }
        bytes
    }

    let options = cbor::Options {
        max_duplicate_checks: Some(10),
        ..Default::default()
    };

    // 5 entries need 5 * 4 / 2 = 10 comparisons.
    let map: cbor::Map<u8, u8> = cbor::from_slice_with_options(&encode_map(5), options).unwrap();
    assert_eq!(map.len(), 5);

    // 6 entries need 15 comparisons.
    let error = cbor::from_slice_with_options::<cbor::Map<u8, u8>>(&encode_map(6), options).unwrap_err();
    assert_eq!(error.kind(), cbor::ErrorKind::DuplicateCheckLimitExceeded);
}

#[test]
fn default_map_duplicate_check_budget() {
    assert_eq!(cbor::DEFAULT_MAX_MAP_ENTRIES, 2048);
    assert_eq!(cbor::DEFAULT_MAX_DUPLICATE_CHECKS, 2_096_128);

    // A single 2049-entry map exceeds the default budget before any entries
    // are decoded.
    let mut bytes = vec![0xb9, 0x08, 0x01];
    bytes.resize(3 + 2049 * 2, 0x00);
    let error = cbor::from_slice::<cbor::Map<u64, u64>>(&bytes).unwrap_err();
    assert_eq!(error.kind(), cbor::ErrorKind::DuplicateCheckLimitExceeded);
}

#[test]
fn container_entry_limit_bounds_definite_containers() {
    let options = cbor::Options {
        max_container_entries: Some(2),
        ..Default::default()
    };

    // Arrays: two entries fit, three do not.
    let array_2 = [0x82, 0x01, 0x02];
    let array_3 = [0x83, 0x01, 0x02, 0x03];
    assert_eq!(cbor::from_slice_with_options::<Vec<u8>>(&array_2, options).unwrap(), vec![1, 2]);
    let error = cbor::from_slice_with_options::<Vec<u8>>(&array_3, options).unwrap_err();
    assert_eq!(error.kind(), cbor::ErrorKind::ContainerLimitExceeded);
    assert_eq!(error.offset(), 0);

    // Maps: the limit counts pairs, not items.
    let map_2 = [0xa2, 0x01, 0x01, 0x02, 0x02];
    let map_3 = [0xa3, 0x01, 0x01, 0x02, 0x02, 0x03, 0x03];
    assert_eq!(
        cbor::from_slice_with_options::<std::collections::BTreeMap<u8, u8>>(&map_2, options)
            .unwrap()
            .len(),
        2
    );
    let error = cbor::from_slice_with_options::<std::collections::BTreeMap<u8, u8>>(&map_3, options).unwrap_err();
    assert_eq!(error.kind(), cbor::ErrorKind::ContainerLimitExceeded);

    // The header readers enforce it directly.
    let mut decoder = cbor::Decoder::with_options(&array_3, options);
    assert_eq!(decoder.array().unwrap_err().kind(), cbor::ErrorKind::ContainerLimitExceeded);

    // `skip` and `Value` enforce it too, including nested containers.
    assert_eq!(
        cbor::Decoder::with_options(&array_3, options)
            .skip()
            .unwrap_err()
            .kind(),
        cbor::ErrorKind::ContainerLimitExceeded
    );
    let nested = [0x81, 0x83, 0x01, 0x02, 0x03];
    let error = cbor::from_slice_with_options::<cbor::Value>(&nested, options).unwrap_err();
    assert_eq!(error.kind(), cbor::ErrorKind::ContainerLimitExceeded);
    assert_eq!(error.offset(), 1);

    // Validation uses the same bound.
    let error = cbor::validate_prefix_with(&array_3, options, cbor::KeyOrder::Core, false).unwrap_err();
    assert_eq!(error.kind(), cbor::ErrorKind::ContainerLimitExceeded);

    // `None` keeps the previous behaviour.
    assert!(cbor::from_slice::<Vec<u8>>(&array_3).is_ok());
}

#[test]
fn container_entry_limit_rejects_indefinite_containers() {
    let options = cbor::Options {
        max_container_entries: Some(16),
        ..Default::default()
    };

    // Indefinite array: `[_ 1, 2]`, indefinite map: `{_ 1: 1}`.
    let array = [0x9f, 0x01, 0x02, 0xff];
    let map = [0xbf, 0x01, 0x01, 0xff];
    for bytes in [&array[..], &map[..]] {
        let error = cbor::from_slice_with_options::<cbor::Value>(bytes, options).unwrap_err();
        assert_eq!(error.kind(), cbor::ErrorKind::ContainerLimitExceeded);
        assert_eq!(
            cbor::Decoder::with_options(bytes, options).skip().unwrap_err().kind(),
            cbor::ErrorKind::ContainerLimitExceeded
        );
    }

    // Without a limit, indefinite-length containers still decode.
    assert!(cbor::from_slice::<cbor::Value>(&array).is_ok());
    assert!(cbor::from_slice::<cbor::Value>(&map).is_ok());
}
