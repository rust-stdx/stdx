#![cfg(feature = "alloc")]

use std::borrow::Cow;

#[test]
fn cow_str_borrows_definite_text() {
    let bytes = cbor::to_vec("hello");
    let value: Cow<'_, str> = cbor::from_slice(&bytes).unwrap();
    assert!(matches!(value, Cow::Borrowed("hello")));
}

#[test]
fn cow_str_owns_indefinite_text() {
    // 0x7f starts an indefinite-length text string, 0x63 is "abc", 0xff ends it.
    let bytes = [0x7f, 0x63, b'a', b'b', b'c', 0xff];
    let value: Cow<'_, str> = cbor::from_slice(&bytes).unwrap();
    assert_eq!(value, "abc");
    assert!(matches!(value, Cow::Owned(_)));
}

#[test]
fn cow_str_rejects_invalid_utf8() {
    let bytes = [0x61, 0xff];
    assert!(cbor::from_slice::<Cow<'_, str>>(&bytes).is_err());
}

#[test]
fn cow_str_encodes_as_text() {
    let value: Cow<'_, str> = Cow::Borrowed("hello");
    assert_eq!(cbor::to_vec(&value), cbor::to_vec("hello"));
}

#[test]
fn cow_bytes_encodes_as_a_byte_string() {
    let value: Cow<'_, [u8]> = Cow::Borrowed(b"abc");
    assert_eq!(cbor::to_vec(&value), vec![0x43, b'a', b'b', b'c']);
    assert_eq!(cbor::from_slice::<Cow<'_, [u8]>>(&cbor::to_vec(&value)).unwrap(), value);
}

#[test]
fn text_validation_around_simd_threshold() {
    for length in 0..=128 {
        // A valid text string whose body is exactly `length` bytes.
        let text: String = std::iter::repeat_n('a', length).collect();
        let mut encoded = cbor::to_vec(&text);
        assert_eq!(cbor::from_slice::<String>(&encoded).unwrap(), text);

        // Invalid UTF-8 at the end must still be rejected.
        if length > 0 {
            *encoded.last_mut().unwrap() = 0xff;
            assert!(cbor::from_slice::<String>(&encoded).is_err());
            assert!(cbor::validate(&encoded).is_err());
        }
    }
}

#[test]
fn text_with_multibyte_characters_across_block_boundaries() {
    for pad in 0..=64 {
        let mut text = String::new();
        for _ in 0..pad {
            text.push('a');
        }
        text.push('😀');
        for _ in 0..pad {
            text.push('b');
        }
        let bytes = cbor::to_vec(&text);
        assert!(cbor::validate(&bytes).is_ok());
        assert_eq!(cbor::from_slice::<String>(&bytes).unwrap(), text);
        assert_eq!(cbor::from_slice::<&str>(&bytes).unwrap(), text);
    }
}

#[test]
fn to_vec_into_reuses_the_buffer() {
    let mut buffer = Vec::new();
    cbor::to_vec_into(&("hello", 42u64), &mut buffer);
    let first = buffer.clone();

    cbor::to_vec_into(&("world", 7u64), &mut buffer);
    let second = cbor::to_vec(&("world", 7u64));
    assert_eq!(buffer, second);

    cbor::to_vec_into(&("hello", 42u64), &mut buffer);
    assert_eq!(buffer, first);
}

#[test]
fn to_canonical_vec_into_matches_to_canonical_vec() {
    let value = ("z", 1u8, vec![("aa", 2u8)]);
    let mut buffer = vec![0xff; 16];
    cbor::to_canonical_vec_into(&value, &mut buffer);
    assert_eq!(buffer, cbor::to_canonical_vec(&value));
}

#[cfg(feature = "std")]
#[test]
fn serde_to_vec_into_reuses_the_buffer() {
    let value = ("hello", 42u64);
    let mut buffer = vec![0xff; 16];
    cbor::serde::to_vec_into(&value, &mut buffer).unwrap();
    assert_eq!(buffer, cbor::serde::to_vec(&value).unwrap());
}

#[cfg(feature = "std")]
#[test]
fn reader_read_item_into_matches_read_item() {
    use std::io::{BufReader, Cursor};

    let item = cbor::to_vec(&("hello", 42u64));
    let mut stream = item.clone();
    stream.extend_from_slice(&item);

    let mut reader = BufReader::new(Cursor::new(stream));
    let mut buffer = Vec::new();
    assert_eq!(cbor::reader::read_item_into(&mut reader, &mut buffer).unwrap(), item.len());
    assert_eq!(buffer, item);
    assert_eq!(cbor::reader::read_item_into(&mut reader, &mut buffer).unwrap(), item.len());
    assert_eq!(buffer, item);
}
