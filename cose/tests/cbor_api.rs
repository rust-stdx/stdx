#![cfg(feature = "alloc")]

use cose::{Header, HeaderValue, KeyBuilder, KeyType, Label};

#[test]
fn private_header_values_decode_with_workspace_cbor() {
    let header = Header::builder()
        .value(Label::Int(100), HeaderValue::Uint(7))
        .build_protected()
        .unwrap();

    assert_eq!(header.as_bytes(), &[0xa1, 0x18, 0x64, 0x07]);
    assert_eq!(header.decode::<u64>(Label::Int(100)).unwrap(), Some(7));

    // The workspace codec is re-exported so private parameters use the same
    // version as the crate itself.
    let _: cose::cbor::Tag = cose::cbor::Tag::new(1);
}

#[test]
fn private_key_values_decode_with_workspace_cbor() {
    let key = KeyBuilder::new(KeyType::Symmetric)
        .symmetric(&[0u8; 16])
        .value(Label::Int(100), HeaderValue::Uint(7))
        .build()
        .unwrap();

    assert_eq!(key.decode::<u64>(Label::Int(100)).unwrap(), Some(7));
}

#[test]
fn private_header_values_allow_duplicate_map_keys() {
    // {1: 1, 1: 2} used as an opaque private parameter.
    let raw = [0xa2, 0x01, 0x01, 0x01, 0x02];
    let header = Header::builder()
        .raw_value(Label::Int(100), &raw)
        .build_protected()
        .unwrap();

    let decoded = header
        .decode::<cose::cbor::Map<u64, u64>>(Label::Int(100))
        .unwrap()
        .unwrap();
    assert_eq!(decoded.len(), 2);
}
