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
