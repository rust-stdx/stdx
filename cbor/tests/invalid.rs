#![cfg(feature = "alloc")]

use cbor::{Value, validate, validate_canonical};

fn assert_invalid(bytes: &[u8]) {
    assert!(validate(bytes).is_err(), "expected invalid: {bytes:02x?}");
    assert!(cbor::from_slice::<Value>(bytes).is_err(), "expected decode error: {bytes:02x?}");
}

#[test]
fn truncated_heads() {
    for bytes in [
        &[0x18][..],
        &[0x19, 0x01][..],
        &[0x1a, 0x01, 0x02][..],
        &[0x1b, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06][..],
        &[0x38][..],
        &[0x58][..],
        &[0x78][..],
        &[0x98][..],
        &[0x9a, 0x01, 0xff, 0x00][..],
        &[0xb8][..],
        &[0xd8][..],
        &[0xf8][..],
        &[0xf9, 0x00][..],
        &[0xfa, 0x00, 0x00][..],
        &[0xfb, 0x00, 0x00, 0x00][..],
    ] {
        assert_invalid(bytes);
    }
}

#[test]
fn truncated_bodies() {
    assert_invalid(&[0x41]);
    assert_invalid(&[0x5a, 0xff, 0xff, 0xff, 0xff, 0x00]);
    assert_invalid(&[0x61]);
    assert_invalid(&[0x7a, 0xff, 0xff, 0xff, 0xff, 0x00]);
    assert_invalid(&[0x81]);
    assert_invalid(&[0x82, 0x00]);
    assert_invalid(&[0xa1]);
    assert_invalid(&[0xa2, 0x01, 0x02]);
    assert_invalid(&[0xc0]);
}

#[test]
fn unclosed_indefinite_items() {
    assert_invalid(&[0x5f, 0x41, 0x00]);
    assert_invalid(&[0x7f, 0x61, 0x00]);
    assert_invalid(&[0x9f]);
    assert_invalid(&[0x9f, 0x01, 0x02]);
    assert_invalid(&[0xbf]);
    assert_invalid(&[0xbf, 0x01, 0x02, 0x01, 0x02]);
}

#[test]
fn reserved_additional_information() {
    for byte in [
        0x1c, 0x1d, 0x1e, 0x3c, 0x3d, 0x3e, 0x5c, 0x5d, 0x5e, 0x7c, 0x7d, 0x7e, 0x9c, 0x9d, 0x9e, 0xbc, 0xbd, 0xbe,
        0xdc, 0xdd, 0xde, 0xfc, 0xfd, 0xfe,
    ] {
        assert_invalid(&[byte]);
    }
}

#[test]
fn reserved_simple_values() {
    for byte in [0x00, 0x01, 0x18, 0x1f] {
        assert_invalid(&[0xf8, byte]);
    }
}

#[test]
fn invalid_indefinite_string_chunks() {
    assert_invalid(&[0x5f, 0x00, 0xff]);
    assert_invalid(&[0x5f, 0x21, 0xff]);
    assert_invalid(&[0x5f, 0x61, 0x00, 0xff]);
    assert_invalid(&[0x5f, 0x80, 0xff]);
    assert_invalid(&[0x5f, 0xa0, 0xff]);
    assert_invalid(&[0x5f, 0xc0, 0x00, 0xff]);
    assert_invalid(&[0x7f, 0x41, 0x00, 0xff]);
    assert_invalid(&[0x5f, 0x5f, 0x41, 0x00, 0xff, 0xff]);
    assert_invalid(&[0x7f, 0x7f, 0x61, 0x00, 0xff, 0xff]);
}

#[test]
fn break_in_wrong_position() {
    assert_invalid(&[0xff]);
    assert_invalid(&[0x81, 0xff]);
    assert_invalid(&[0x82, 0x00, 0xff]);
    assert_invalid(&[0xa1, 0xff]);
    assert_invalid(&[0xa1, 0xff, 0x00]);
    assert_invalid(&[0xa1, 0x00, 0xff]);
    assert_invalid(&[0xbf, 0x00, 0xff]);
    assert_invalid(&[0x9f, 0x81, 0xff]);
}

#[test]
fn additional_information_31_on_integers_and_tags() {
    assert_invalid(&[0x1f]);
    assert_invalid(&[0x3f]);
    assert_invalid(&[0xdf]);
}

#[test]
fn invalid_utf8() {
    assert_invalid(&[0x61, 0xff]);
    assert_invalid(&[0x62, 0xc3, 0x28]);
    // A code point may not be split across indefinite text chunks.
    assert_invalid(&[0x7f, 0x61, 0xc3, 0x61, 0xa9, 0xff]);
}

#[test]
fn trailing_data() {
    assert!(validate(&[0x01, 0x02]).is_err());
    assert!(cbor::from_slice::<u8>(&[0x01, 0x02]).is_err());
    assert_eq!(cbor::from_slice_prefix::<u8>(&[0x01, 0x02]).unwrap(), (1, &[0x02][..]));
}

#[test]
fn depth_limit() {
    // 40 nested arrays with the default depth limit of 32.
    let mut bytes = vec![0x81; 40];
    bytes.push(0x00);
    assert!(validate(&bytes).is_err());
    assert!(cbor::from_slice::<Value>(&bytes).is_err());

    let options = cbor::Options {
        max_depth: 64,
        ..Default::default()
    };
    let mut decoder = cbor::Decoder::with_options(&bytes, options);
    assert!(decoder.skip().is_ok());
}

#[test]
fn canonical_rejections() {
    // Non-shortest integer.
    assert!(validate(&[0x18, 0x01]).is_ok());
    assert!(validate_canonical(&[0x18, 0x01]).is_err());

    // Indefinite array.
    assert!(validate(&[0x9f, 0x01, 0xff]).is_ok());
    assert!(validate_canonical(&[0x9f, 0x01, 0xff]).is_err());

    // Unsorted map keys.
    assert!(validate_canonical(&[0xa2, 0x01, 0x00, 0x00, 0x00]).is_err());
    // Sorted map keys.
    assert!(validate_canonical(&[0xa2, 0x00, 0x00, 0x01, 0x00]).is_ok());

    // Duplicate key.
    assert!(validate_canonical(&[0xa2, 0x00, 0x00, 0x00, 0x01]).is_err());

    // Non-shortest float (1.0 as f64 instead of f16).
    assert!(validate_canonical(&[0xfb, 0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).is_err());
    // Non-canonical NaN.
    assert!(validate_canonical(&[0xfb, 0x7f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).is_err());
    // Canonical NaN.
    assert!(validate_canonical(&[0xf9, 0x7e, 0x00]).is_ok());
}

#[test]
fn length_first_order() {
    // Keys: 10 (0x0a), -1 (0x20), false (0xf4), 100 (0x1864), "z" (0x617a).
    let core = [
        0xa5, 0x0a, 0x00, 0x18, 0x64, 0x00, 0x20, 0x00, 0x61, 0x7a, 0x00, 0xf4, 0x00,
    ];
    let length_first = [
        0xa5, 0x0a, 0x00, 0x20, 0x00, 0xf4, 0x00, 0x18, 0x64, 0x00, 0x61, 0x7a, 0x00,
    ];
    assert!(validate_canonical(&core).is_ok());
    assert!(validate_canonical(&length_first).is_err());
    assert!(cbor::validate_canonical_with(&length_first, cbor::KeyOrder::LengthFirst).is_ok());
    assert!(cbor::validate_canonical_with(&core, cbor::KeyOrder::LengthFirst).is_err());
}
