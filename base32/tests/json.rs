#![cfg(feature = "json")]

use std::borrow::Cow;

use json::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Data {
    #[json(with = "base32")]
    array: [u8; 4],
    #[json(with = "base32")]
    vec: Vec<u8>,
    #[json(borrow, with = "base32")]
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
    assert_eq!(text, r#"{"array":"MFRGGZA=","vec":"NBSWY3DP","cow":"NBUQ===="}"#);
    let decoded: Data = json::from_str(&text).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn rejects_bad_values() {
    assert!(json::from_str::<Data>(r#"{"array":"MFRG","vec":"","cow":""}"#).is_err());
    assert!(json::from_str::<Data>(r#"{"array":"!!!!","vec":"","cow":""}"#).is_err());
}

#[cfg(feature = "small_collections")]
#[test]
fn small_vec_roundtrip() {
    use small_collections::SmallVec;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Small {
        #[json(with = "base32")]
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
