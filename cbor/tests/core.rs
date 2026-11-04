#![cfg(feature = "alloc")]

#[test]
fn primitives_roundtrip() {
    assert_eq!(cbor::to_vec(&0u8), vec![0x00]);
    assert_eq!(cbor::to_vec(&23u8), vec![0x17]);
    assert_eq!(cbor::to_vec(&24u8), vec![0x18, 0x18]);
    assert_eq!(cbor::to_vec(&255u8), vec![0x18, 0xff]);
    assert_eq!(cbor::to_vec(&256u16), vec![0x19, 0x01, 0x00]);
    assert_eq!(cbor::to_vec(&-1i8), vec![0x20]);
    assert_eq!(cbor::to_vec(&-24i8), vec![0x37]);
    assert_eq!(cbor::to_vec(&-25i8), vec![0x38, 0x18]);

    for value in [0u64, 1, 23, 24, 255, 256, 65535, 65536, u32::MAX as u64, u64::MAX] {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<u64>(&bytes).unwrap(), value);
    }
    for value in [0i64, 1, 23, 24, -1, -24, -25, -256, -257, i64::MIN, i64::MAX] {
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<i64>(&bytes).unwrap(), value);
    }
}

#[test]
fn floats_roundtrip_and_are_shortest() {
    assert_eq!(cbor::to_vec(&1.5f64), vec![0xf9, 0x3e, 0x00]);
    assert_eq!(cbor::to_vec(&100000.0f64), vec![0xfa, 0x47, 0xc3, 0x50, 0x00]);
    assert_eq!(cbor::to_vec(&f64::INFINITY), vec![0xf9, 0x7c, 0x00]);
    assert_eq!(cbor::to_vec(&f64::NAN), vec![0xf9, 0x7e, 0x00]);

    for value in [0.0f64, -0.0, 1.0, 1.5, -4.1, 1.0e300, f64::INFINITY, f64::NEG_INFINITY] {
        let bytes = cbor::to_vec(&value);
        let decoded: f64 = cbor::from_slice(&bytes).unwrap();
        assert_eq!(decoded.to_bits(), value.to_bits());
    }
}

#[test]
fn text_and_bytes() {
    let bytes = cbor::to_vec("hello");
    assert_eq!(bytes, vec![0x65, b'h', b'e', b'l', b'l', b'o']);
    assert_eq!(cbor::from_slice::<&str>(&bytes).unwrap(), "hello");
    assert_eq!(cbor::from_slice::<String>(&bytes).unwrap(), "hello");

    let bytes = cbor::to_vec(&vec![1u8, 2, 3]);
    assert_eq!(bytes, vec![0x83, 0x01, 0x02, 0x03]);
    assert_eq!(cbor::from_slice::<Vec<u8>>(&bytes).unwrap(), vec![1, 2, 3]);
}

#[test]
fn arrays_maps_options_and_tags() {
    let value = (1u8, "two", Some(3u8), None::<u8>);
    let bytes = cbor::to_vec(&value);
    let back: (u8, String, Option<u8>, Option<u8>) = cbor::from_slice(&bytes).unwrap();
    assert_eq!(back, (1, "two".into(), Some(3), None));

    let mut map = std::collections::BTreeMap::new();
    map.insert(1u8, "one".to_string());
    map.insert(2u8, "two".to_string());
    let bytes = cbor::to_vec(&map);
    let back: std::collections::BTreeMap<u8, String> = cbor::from_slice(&bytes).unwrap();
    assert_eq!(back, map);

    let tagged = cbor::Tagged::<u64, 1>(1_700_000_000);
    let bytes = cbor::to_vec(&tagged);
    assert_eq!(bytes, vec![0xc1, 0x1a, 0x65, 0x53, 0xf1, 0x00]);
    assert_eq!(cbor::from_slice::<cbor::Tagged<u64, 1>>(&bytes).unwrap(), tagged);
}

#[test]
fn bignums() {
    let value = u128::MAX;
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes[0], 0xc2);
    assert_eq!(cbor::from_slice::<u128>(&bytes).unwrap(), value);

    let value = i128::MIN;
    let bytes = cbor::to_vec(&value);
    assert_eq!(bytes[0], 0xc3);
    assert_eq!(cbor::from_slice::<i128>(&bytes).unwrap(), value);
}

#[test]
fn value_roundtrip() {
    let value = cbor::Value::Map(cbor::Map::from_iter([
        ("a".into(), cbor::Value::from(1u8)),
        ("b".into(), cbor::Value::Array(vec![cbor::Value::Null, cbor::Value::Bool(true)])),
    ]));
    let bytes = cbor::to_vec(&value);
    let back: cbor::Value = cbor::from_slice(&bytes).unwrap();
    assert_eq!(back, value);
    assert!(cbor::validate(&bytes).is_ok());
    assert!(cbor::validate_canonical(&bytes).is_ok());
}

#[test]
fn raw_value_preserves_bytes() {
    let bytes = vec![0x82, 0x01, 0x02];
    let raw = cbor::RawValue::new(&bytes).unwrap();
    assert_eq!(raw.as_bytes(), &bytes);
    assert_eq!(cbor::to_vec(&raw), bytes);
    assert!(cbor::RawValue::new(&[0x82, 0x01]).is_err());
    assert!(cbor::RawValue::new(&[0x01, 0x02]).is_err());
}

#[test]
fn canonical_validation() {
    // 10, 100, -1, "z", "aa" in core deterministic order.
    let canonical = [
        0xa5, 0x0a, 0x00, 0x18, 0x64, 0x00, 0x20, 0x00, 0x61, 0x7a, 0x00, 0x62, 0x61, 0x61, 0x00,
    ];
    assert!(cbor::validate(&canonical).is_ok());
    assert!(cbor::validate_canonical(&canonical).is_ok());

    // Same keys in a different order.
    let unsorted = [
        0xa5, 0x18, 0x64, 0x00, 0x0a, 0x00, 0x20, 0x00, 0x61, 0x7a, 0x00, 0x62, 0x61, 0x61, 0x00,
    ];
    assert!(cbor::validate(&unsorted).is_ok());
    assert!(cbor::validate_canonical(&unsorted).is_err());

    // Non-shortest integer.
    assert!(cbor::validate(&[0x18, 0x01]).is_ok());
    assert!(cbor::validate_canonical(&[0x18, 0x01]).is_err());
}

#[test]
fn indefinite_items() {
    let bytes = [0x9f, 0x01, 0x02, 0xff];
    assert_eq!(cbor::from_slice::<Vec<u8>>(&bytes).unwrap(), vec![1, 2]);
    assert!(cbor::validate(&bytes).is_ok());

    let bytes = [0x5f, 0x41, 0x01, 0x41, 0x02, 0xff];
    assert!(cbor::validate(&bytes).is_ok());
    let mut decoder = cbor::Decoder::new(&bytes);
    let mut iter = decoder.bytes_iter().unwrap();
    assert_eq!(iter.next().unwrap().unwrap(), &[1]);
    assert_eq!(iter.next().unwrap().unwrap(), &[2]);
    assert!(iter.next().is_none());
}

#[test]
fn errors_are_located() {
    assert_eq!(cbor::from_slice::<u8>(&[0x18]).unwrap_err().offset(), 1);
    assert!(cbor::from_slice::<u8>(&[0x01, 0x02]).is_err());
    assert!(cbor::from_slice::<String>(&[0x61, 0xff]).is_err());
    assert!(cbor::from_slice::<u8>(&[0x1c]).is_err());
}
