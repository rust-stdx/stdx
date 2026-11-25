#![cfg(feature = "json")]

use std::borrow::Cow;

use json::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Data {
    #[json(with = "hex")]
    array: [u8; 4],
    #[json(with = "hex")]
    vec: Vec<u8>,
    #[json(borrow, with = "hex")]
    cow: Cow<'static, [u8]>,
}

#[test]
fn roundtrip() {
    let data = Data {
        array: *b"abcd",
        vec: b"hello".to_vec(),
        cow: Cow::Borrowed(b"hi"),
    };
    let text = json::to_string(&data);
    assert_eq!(text, r#"{"array":"61626364","vec":"68656c6c6f","cow":"6869"}"#);
    let decoded: Data = json::from_str(&text).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn rejects_bad_values() {
    assert!(json::from_str::<Data>(r#"{"array":"616263","vec":"","cow":""}"#).is_err());
    assert!(json::from_str::<Data>(r#"{"array":"zzzz","vec":"","cow":""}"#).is_err());
}

#[cfg(feature = "small_collections")]
#[test]
fn small_vec_roundtrip() {
    use small_collections::SmallVec;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Small {
        #[json(with = "hex")]
        bytes: SmallVec<u8, 4>,
    }

    let small = Small {
        bytes: SmallVec::from_slice(b"abc"),
    };
    let text = json::to_string(&small);
    let decoded: Small = json::from_str(&text).unwrap();
    assert_eq!(decoded, small);
    assert!(decoded.bytes.is_inline());
}
