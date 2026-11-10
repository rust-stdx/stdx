#![cfg(all(feature = "alloc", feature = "derive"))]

use alloc_counter::measure;
use json::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Borrowed<'a> {
    #[json(borrow)]
    name: &'a str,
    value: u64,
    flag: bool,
}

#[test]
fn borrowed_decode_is_allocation_free() {
    let value = Borrowed {
        name: "hello",
        value: 42,
        flag: true,
    };
    let bytes = json::to_vec(&value);
    let (decoded, counters) = measure(|| json::from_slice::<Borrowed<'_>>(&bytes).unwrap());
    assert_eq!(decoded, value);
    assert_eq!(counters.calls, 0, "borrowed decode allocated");
}

#[test]
fn fixed_buffer_encode_is_allocation_free() {
    let value = Borrowed {
        name: "hello",
        value: 42,
        flag: true,
    };
    let mut buffer = [0u8; 128];
    let (used, counters) = measure(|| json::to_slice(&value, &mut buffer).unwrap().len());
    assert!(used > 0);
    assert_eq!(counters.calls, 0, "fixed-buffer encode allocated");
}

#[test]
fn validation_and_raw_values_are_allocation_free() {
    let text = br#"{"a":[1,2,3],"b":"x"}"#;
    let (_, counters) = measure(|| json::validate(text).unwrap());
    assert_eq!(counters.calls, 0, "validate allocated");

    let (raw, counters) = measure(|| json::RawValue::new(text).unwrap());
    assert_eq!(raw.get(), text);
    assert_eq!(counters.calls, 0, "RawValue allocated");

    let (number, counters) = measure(|| json::RawNumber::new("123.456").unwrap());
    assert_eq!(number.as_str(), "123.456");
    assert_eq!(counters.calls, 0, "RawNumber allocated");
}

#[test]
fn lazy_document_is_allocation_free() {
    let text = br#"{"a":{"b":[1,2,3]},"s":"ok"}"#;
    let (value, counters) = measure(|| {
        let document = json::Document::parse(text).unwrap();
        document.get("/a/b/2").unwrap().unwrap().as_i64().unwrap()
    });
    assert_eq!(value, Some(3));
    assert_eq!(counters.calls, 0, "lazy document allocated");
}

#[test]
fn empty_containers_do_not_allocate() {
    let (array, counters) = measure(|| json::from_str::<json::Value>("[]").unwrap());
    assert!(array.is_array());
    assert_eq!(counters.calls, 0, "empty array allocated");

    let (object, counters) = measure(|| json::from_str::<json::Value>("{}").unwrap());
    assert!(object.is_object());
    assert_eq!(counters.calls, 0, "empty object allocated");
}

#[test]
fn mask_ring_does_not_allocate() {
    let text = format!(
        "{{\"items\":[{}]}}",
        (0..2000)
            .map(|i| format!("{{\"id\":{i},\"name\":\"item {i}\"}}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let options = json::Options {
        use_simd_masks: true,
        ..json::Options::default()
    };
    let (_, plain) = measure(|| json::from_str::<json::Value>(&text).unwrap());
    let (_, masked) = measure(|| json::from_str_with_options::<json::Value>(&text, options).unwrap());
    assert_eq!(plain.calls, masked.calls, "mask ring changed the allocation count");
}
