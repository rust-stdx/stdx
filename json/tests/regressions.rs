use json::{DuplicateKeys, Options, Value, json};

#[test]
fn negative_zero_roundtrips() {
    let value: Value = json::from_str("-0").unwrap();
    assert_eq!(value.as_f64().map(f64::to_bits), Some((-0.0f64).to_bits()));
    let text = json::to_string(&value);
    let back: Value = json::from_str(&text).unwrap();
    assert_eq!(value, back);
}

#[test]
fn max_string_len_handles_trailing_escapes() {
    let options = Options {
        max_string_len: Some(1),
        ..Options::default()
    };
    for text in [r#""\u0041\u0041""#, r#""a\u0041""#] {
        assert!(json::validate_with_options(text.as_bytes(), options).is_err(), "{text}");
        assert!(
            json::from_slice_with_options::<String>(text.as_bytes(), options).is_err(),
            "{text}"
        );
    }
}

#[test]
fn document_pointer_and_duplicate_semantics() {
    let document = json::Document::parse(b"  [10,20]").unwrap();
    assert_eq!(document.get("/0").unwrap().unwrap().as_i64().unwrap(), Some(10));

    let document = json::Document::parse(br#"{"a":1,"a":2}"#).unwrap();
    assert_eq!(document.get("/a").unwrap().unwrap().as_i64().unwrap(), Some(2));
}

#[test]
fn typed_maps_honor_duplicate_policy() {
    let first: std::collections::BTreeMap<String, i64> = json::from_str_with_options(
        r#"{"a":1,"a":2}"#,
        Options {
            duplicate_keys: DuplicateKeys::FirstWins,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(first["a"], 1);

    let reject: json::Result<std::collections::BTreeMap<String, i64>> = json::from_str_with_options(
        r#"{"a":1,"a":2}"#,
        Options {
            duplicate_keys: DuplicateKeys::Reject,
            ..Options::default()
        },
    );
    assert!(reject.is_err());
}

#[test]
fn deep_container_limit_is_rejected() {
    let mut text = String::new();
    for _ in 0..140 {
        text.push('[');
    }
    text.push_str("[1,2]");
    for _ in 0..140 {
        text.push(']');
    }
    let options = Options {
        max_depth: 200,
        max_container_entries: Some(1),
        ..Options::default()
    };
    assert!(json::validate_with_options(text.as_bytes(), options).is_err());
}

#[test]
fn masks_produce_the_same_value() {
    let mut text = String::from("{\"items\":[");
    for i in 0..2000 {
        if i != 0 {
            text.push(',');
        }
        text.push_str(&format!(
            "{{\"id\":{i},\"text\":\"hello \\\"{i}\\\" {} \",\"ok\":true}}",
            "日本語テキスト".repeat(8)
        ));
    }
    text.push_str("]}");

    let plain: Value = json::from_str(&text).unwrap();
    let masked: Value = json::from_str_with_options(
        &text,
        Options {
            use_simd_masks: true,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(plain, masked);
    assert!(
        json::validate_with_options(
            text.as_bytes(),
            Options {
                use_simd_masks: true,
                ..Options::default()
            }
        )
        .is_ok()
    );
}

#[test]
fn invalid_raw_utf8_never_panics() {
    use json::{Encode, Encoder, Write};

    struct Invalid;
    impl Encode for Invalid {
        fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
            encoder.writer_mut().write_all(b"\xff").ok();
            Ok(())
        }
    }

    let text = json::to_string(&Invalid);
    assert!(text.is_char_boundary(0));
    assert_eq!(json::to_vec(&Invalid), b"\xff");
}

#[test]
fn json_macro_still_works() {
    let value = json!({"a": 1, "b": [true, null]});
    assert_eq!(value["a"].as_i64(), Some(1));
}

#[test]
fn masks_reject_control_after_escape() {
    let mut doc = Vec::new();
    doc.extend_from_slice(b"{\"x\":\"\\r");
    doc.push(0x1e);
    doc.extend_from_slice(b"\"}");
    let options = Options {
        use_simd_masks: true,
        ..Options::default()
    };
    assert!(json::from_slice_with_options::<Value>(&doc, options).is_err());
    assert!(json::validate_with_options(&doc, options).is_err());
}
