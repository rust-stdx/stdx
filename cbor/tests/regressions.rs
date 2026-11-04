#![cfg(feature = "alloc")]

use cbor::{Value, validate, validate_canonical};

#[test]
fn canonical_encoding_sorts_ordered_map_keys() {
    let mut map = cbor::Map::<String, u8>::new();
    map.push("aa".to_string(), 2);
    map.push("z".to_string(), 1);
    let bytes = cbor::to_canonical_vec(&map);
    assert!(validate_canonical(&bytes).is_ok());
    let back: cbor::Value = cbor::from_slice(&bytes).unwrap();
    let map = back.as_map().unwrap();
    assert_eq!(map.iter().next().unwrap().0, Value::from("z"));
}

#[test]
fn canonical_validation_rejects_non_preferred_bignums() {
    // Basic integer encoded as tag 2 bignum.
    assert!(validate_canonical(&[0xc2, 0x41, 0x01]).is_err());
    // Zero encoded as a bignum.
    assert!(validate_canonical(&[0xc2, 0x40]).is_err());
    assert!(validate_canonical(&[0xc2, 0x41, 0x00]).is_err());
    // Leading zero bytes.
    assert!(validate_canonical(&[0xc2, 0x42, 0x00, 0x01]).is_err());
    // Negative bignum that fits a basic negative integer.
    assert!(validate_canonical(&[0xc3, 0x41, 0x01]).is_err());
    // 2^64 needs a bignum and is canonical.
    assert!(validate_canonical(&[0xc2, 0x49, 0x01, 0, 0, 0, 0, 0, 0, 0, 0]).is_ok());
}

#[test]
fn canonical_value_bignum_is_normalized() {
    let value = Value::Bignum(cbor::Bignum::new(false, vec![1]));
    let bytes = cbor::to_canonical_vec(&value);
    assert_eq!(bytes, vec![0x01]);
    assert!(validate_canonical(&bytes).is_ok());
}

#[test]
fn raw_value_decode_rejects_invalid_utf8() {
    let cases: &[&[u8]] = &[
        &[0x61, 0xff],
        &[0x62, 0xc3, 0x28],
        &[0x7f, 0x61, 0xc3, 0x61, 0xa9, 0xff],
    ];
    for bytes in cases {
        assert!(validate(bytes).is_err());
        let mut decoder = cbor::Decoder::new(bytes);
        assert!(decoder.decode::<cbor::RawValue<'_>>().is_err(), "raw accepted {bytes:02x?}");
        let mut decoder = cbor::Decoder::new(bytes);
        assert!(decoder.decode::<cbor::RawValueBuf>().is_err(), "raw buf accepted {bytes:02x?}");
        let mut decoder = cbor::Decoder::new(bytes);
        assert!(decoder.skip().is_err(), "skip accepted {bytes:02x?}");
    }
}

#[test]
fn u128_and_i128_accept_all_bignum_encodings() {
    // Non-shortest tag 2.
    assert_eq!(cbor::from_slice::<u128>(&[0xd8, 0x02, 0x41, 0x01]).unwrap(), 1);
    // Indefinite-length content.
    assert_eq!(
        cbor::from_slice::<u128>(&[0xc2, 0x5f, 0x41, 0x01, 0x41, 0x02, 0xff]).unwrap(),
        0x0102
    );
    // More than 16 bytes, but only leading zeroes make it fit.
    let mut leading_zeroes = vec![0xc2, 0x54];
    leading_zeroes.extend_from_slice(&[0; 19]);
    leading_zeroes.push(0x01);
    assert_eq!(cbor::from_slice::<u128>(&leading_zeroes).unwrap(), 1);

    // Non-shortest tag 3.
    assert_eq!(cbor::from_slice::<i128>(&[0xd8, 0x03, 0x41, 0x01]).unwrap(), -2);
    // Indefinite-length negative content.
    assert_eq!(
        cbor::from_slice::<i128>(&[0xc3, 0x5f, 0x41, 0x01, 0x41, 0x02, 0xff]).unwrap(),
        -0x0103
    );
}

#[cfg(feature = "derive")]
mod derive_regressions {
    use cbor::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[cbor(array)]
    enum Compact {
        A,
        B,
    }

    #[test]
    fn enum_array_rejects_missing_variant_index() {
        assert!(cbor::from_slice::<Compact>(&[0x80, 0x00]).is_err());
        assert!(cbor::from_slice::<Compact>(&[0x80, 0x01]).is_err());
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum Named {
        V { x: u8 },
    }

    #[test]
    fn enum_struct_variant_rejects_duplicate_fields() {
        // {"V": {"x": 1, "x": 2}}
        let bytes = [0xa1, 0x61, b'V', 0xa2, 0x61, b'x', 0x01, 0x61, b'x', 0x02];
        assert!(cbor::from_slice::<Named>(&bytes).is_err());
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum WithPayload {
        V(u8),
    }

    #[test]
    fn enum_accepts_indefinite_length_map() {
        // {_ "V": 7}
        let bytes = [0xbf, 0x61, b'V', 0x07, 0xff];
        assert_eq!(cbor::from_slice::<WithPayload>(&bytes).unwrap(), WithPayload::V(7));
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[cbor(array)]
    enum IndefiniteArray {
        V(u8),
    }

    #[test]
    fn enum_accepts_indefinite_length_array() {
        // [_ 0, 7]
        let bytes = [0x9f, 0x00, 0x07, 0xff];
        assert_eq!(cbor::from_slice::<IndefiniteArray>(&bytes).unwrap(), IndefiniteArray::V(7));
    }
}

#[cfg(feature = "serde")]
mod serde_regressions {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    enum Enum {
        A,
        B,
    }

    #[test]
    fn enum_rejects_missing_variant_index() {
        assert!(cbor::serde::from_slice::<Enum>(&[0x80, 0x00]).is_err());
        assert!(cbor::serde::from_slice::<Enum>(&[0x80, 0x01]).is_err());
    }

    #[derive(Debug, PartialEq, Deserialize)]
    enum WithPayload {
        V(u8),
    }

    #[test]
    fn enum_accepts_indefinite_length_map() {
        let bytes = [0xbf, 0x61, b'V', 0x07, 0xff];
        assert!(matches!(cbor::serde::from_slice::<WithPayload>(&bytes), Ok(WithPayload::V(7))));
    }

    #[test]
    fn duplicate_map_keys_are_rejected() {
        // {1: 2, 1: 3}
        let bytes = [0xa2, 0x01, 0x02, 0x01, 0x03];
        assert!(cbor::serde::from_slice::<std::collections::BTreeMap<u8, u8>>(&bytes).is_err());
    }

    #[test]
    fn duplicate_map_keys_are_rejected_after_spilling() {
        let mut bytes = Vec::new();
        let mut encoder = cbor::Encoder::new(&mut bytes);
        encoder.map(10).unwrap();
        for key in 0u8..9 {
            encoder.u8(key).unwrap();
            encoder.u8(key).unwrap();
        }
        encoder.u8(0).unwrap();
        encoder.u8(99).unwrap();
        assert!(cbor::serde::from_slice::<std::collections::BTreeMap<u8, u8>>(&bytes).is_err());
    }

    #[test]
    fn tagged_values_do_not_impose_a_payload_depth_limit() {
        let mut value = cbor::Value::Null;
        for _ in 0..40 {
            value = cbor::Value::Array(vec![value]);
        }
        let tagged = cbor::TaggedValue {
            tag: cbor::Tag::new(24),
            value: &value,
        };
        let bytes = cbor::serde::to_vec(&tagged).unwrap();
        assert_eq!(&bytes[..2], &[0xd8, 0x18]);

        let options = cbor::Options {
            max_depth: 64,
            ..Default::default()
        };
        let mut decoder = cbor::Decoder::with_options(&bytes, options);
        let decoded = decoder.decode::<cbor::TaggedValue<cbor::Value>>().unwrap();
        assert_eq!(decoded.tag, cbor::Tag::new(24));
        assert_eq!(decoded.value, value);
    }
}

#[cfg(feature = "small_collections")]
mod allocation_regressions {
    use alloc_counter::measure;

    #[test]
    fn smallvec_decode_does_not_preallocate_from_untrusted_length() {
        let n: u16 = 4096;
        let mut bytes = Vec::new();
        bytes.push(0x99);
        bytes.extend_from_slice(&n.to_be_bytes());
        bytes.extend(std::iter::repeat_n(0u8, n as usize));

        let (result, counters) = measure(|| cbor::from_slice::<small_collections::SmallVec<[u8; 1024], 4>>(&bytes));
        assert!(result.is_err());
        assert!(
            counters.bytes < 100_000,
            "preallocated {} bytes from a {} byte input",
            counters.bytes,
            bytes.len()
        );
    }
}
