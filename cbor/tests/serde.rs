#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    active: bool,
    tags: Vec<String>,
}

#[test]
fn serde_struct_roundtrip() {
    let user = User {
        id: 7,
        name: "ada".into(),
        active: true,
        tags: vec!["x".into(), "y".into()],
    };
    let bytes = cbor::serde::to_vec(&user).unwrap();
    let value: cbor::Value = cbor::from_slice(&bytes).unwrap();
    let map = value.as_map().unwrap();
    assert_eq!(map.get(&"id".into()).unwrap().as_u64(), Some(7));
    assert_eq!(cbor::serde::from_slice::<User>(&bytes).unwrap(), user);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Borrowed<'a> {
    #[serde(borrow)]
    name: &'a str,
    #[serde(borrow, with = "serde_bytes")]
    data: &'a [u8],
}

#[test]
fn serde_borrows_from_slice() {
    let bytes = cbor::serde::to_vec(&Borrowed {
        name: "hi",
        data: b"abc",
    })
    .unwrap();
    let value: Borrowed<'_> = cbor::serde::from_slice(&bytes).unwrap();
    assert_eq!(
        value,
        Borrowed {
            name: "hi",
            data: b"abc"
        }
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Message {
    Ping,
    Data(u8),
    Pair(u8, u8),
    Named { x: u8 },
}

#[test]
fn serde_enum_roundtrip() {
    for value in [
        Message::Ping,
        Message::Data(1),
        Message::Pair(2, 3),
        Message::Named {
            x: 4,
        },
    ] {
        let bytes = cbor::serde::to_vec(&value).unwrap();
        assert_eq!(cbor::serde::from_slice::<Message>(&bytes).unwrap(), value);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WithBytes {
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
}

#[test]
fn serde_bytes_are_byte_strings() {
    let value = WithBytes {
        payload: vec![0xde, 0xad],
    };
    let bytes = cbor::serde::to_vec(&value).unwrap();
    assert!(bytes.windows(2).any(|w| w == [0x42, 0xde]));
    assert_eq!(cbor::serde::from_slice::<WithBytes>(&bytes).unwrap(), value);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Signed {
    signature: Vec<u8>,
    payload: cbor::RawValueBuf,
}

#[test]
fn serde_raw_value() {
    let payload = cbor::RawValueBuf::new(vec![0x82, 0x01, 0x02]).unwrap();
    let value = Signed {
        signature: vec![1, 2],
        payload,
    };
    let bytes = cbor::serde::to_vec(&value).unwrap();
    let back: Signed = cbor::serde::from_slice(&bytes).unwrap();
    assert_eq!(back.signature, vec![1, 2]);
    assert_eq!(back.payload.as_bytes(), &[0x82, 0x01, 0x02]);
}

#[test]
fn serde_canonical_reorders_map_keys() {
    #[derive(Serialize)]
    struct Ordered {
        z: u8,
        aa: u8,
    }
    let bytes = cbor::serde::to_canonical_vec(&Ordered {
        z: 1,
        aa: 2,
    })
    .unwrap();
    assert!(cbor::validate_canonical(&bytes).is_ok());
}

#[test]
fn serde_errors_on_trailing_data() {
    let bytes = cbor::serde::to_vec(&1u8).unwrap();
    let mut trailing = bytes.clone();
    trailing.push(0x02);
    assert!(cbor::serde::from_slice::<u8>(&trailing).is_err());
}

#[cfg(feature = "std")]
#[test]
fn serde_from_reader() {
    let bytes = cbor::serde::to_vec(&User {
        id: 1,
        name: "a".into(),
        active: false,
        tags: Vec::new(),
    })
    .unwrap();
    let mut reader = std::io::BufReader::new(std::io::Cursor::new(bytes));
    let user: User = cbor::serde::from_reader(&mut reader).unwrap();
    assert_eq!(user.id, 1);
}

#[test]
fn serde_tagged_value() {
    let value = cbor::TaggedValue {
        tag: cbor::Tag::new(1),
        value: 1_700_000_000u64,
    };
    let bytes = cbor::serde::to_vec(&value).unwrap();
    assert_eq!(bytes, vec![0xc1, 0x1a, 0x65, 0x53, 0xf1, 0x00]);
    let back: cbor::TaggedValue<u64> = cbor::serde::from_slice(&bytes).unwrap();
    assert_eq!(back, value);

    let exact = cbor::Tagged::<u64, 1>(42);
    let bytes = cbor::serde::to_vec(&exact).unwrap();
    assert_eq!(cbor::serde::from_slice::<cbor::Tagged<u64, 1>>(&bytes).unwrap(), exact);
    assert!(cbor::serde::from_slice::<cbor::Tagged<u64, 2>>(&bytes).is_err());
}

#[test]
fn serde_value_roundtrip() {
    let value = cbor::Value::Map(cbor::Map::from_iter([
        ("a".into(), cbor::Value::from(1u8)),
        ("b".into(), cbor::Value::Array(vec![cbor::Value::Null, cbor::Value::Bool(true)])),
    ]));
    let bytes = cbor::serde::to_vec(&value).unwrap();
    let back: cbor::Value = cbor::serde::from_slice(&bytes).unwrap();
    assert_eq!(back, value);
}

#[test]
fn serde_simple_value() {
    let simple = cbor::Simple::new(59).unwrap();
    let bytes = cbor::serde::to_vec(&simple).unwrap();
    assert_eq!(bytes, vec![0xf8, 0x3b]);
    let back: cbor::Simple = cbor::serde::from_slice(&bytes).unwrap();
    assert_eq!(back, simple);
}

#[test]
fn serde_depth_limit() {
    let mut bytes = vec![0x81; 100];
    bytes.push(0x00);
    assert!(cbor::serde::from_slice::<cbor::Value>(&bytes).is_err());
}
