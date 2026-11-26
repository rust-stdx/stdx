//! Regression tests for numeric emission, tag resolution, resource limits, and
//! mapping semantics.

use yaml::{Mapping, Number, Options, TaggedValue, Value, from_str, from_str_multi, from_str_with_options, to_string};

fn number(value: Number) -> Value {
    Value::Number(value)
}

fn tagged(tag: &str, value: Value) -> Value {
    Value::Tagged(Box::new(TaggedValue {
        tag: tag.to_string(),
        value,
    }))
}

#[test]
fn floats_roundtrip_with_their_type_and_bits() {
    for value in [
        0.0,
        -0.0,
        1.0,
        -1.0,
        1.5,
        1e20,
        1e300,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let text = to_string(&value).unwrap();
        let decoded: f64 = from_str(&text).unwrap();
        assert_eq!(decoded.to_bits(), value.to_bits(), "text: {text:?}");
    }

    let text = to_string(&f64::NAN).unwrap();
    let decoded: f64 = from_str(&text).unwrap();
    assert!(decoded.is_nan(), "text: {text:?}");

    for value in [1.0, 1e20, f64::INFINITY] {
        let original = number(Number::Float(value));
        let text = to_string(&original).unwrap();
        let decoded: Value = from_str(&text).unwrap();
        assert_eq!(decoded, original, "text: {text:?}");
    }
}

#[test]
fn i64_min_parses_in_every_integer_syntax() {
    let expected = number(Number::NegInt(i64::MIN));
    for text in [
        "-9223372036854775808",
        "-0x8000000000000000",
        "-0o1000000000000000000000",
        "-0b1000000000000000000000000000000000000000000000000000000000000000",
    ] {
        assert_eq!(from_str::<Value>(text).unwrap(), expected, "text: {text}");
        assert_eq!(from_str::<i64>(text).unwrap(), i64::MIN, "text: {text}");
    }

    let text = to_string(&expected).unwrap();
    assert_eq!(from_str::<Value>(&text).unwrap(), expected);
    assert_eq!(from_str::<i64>(&text).unwrap(), i64::MIN);
}

#[test]
fn negative_zero_normalizes_to_positive_zero() {
    assert_eq!(from_str::<Value>("-0").unwrap(), number(Number::PosInt(0)));
    assert_eq!(from_str::<Value>("-0x0").unwrap(), number(Number::PosInt(0)));
}

#[test]
fn non_specific_tag_keeps_scalar_content() {
    assert_eq!(from_str::<Value>("! 123").unwrap(), Value::from("123"));
    assert_eq!(from_str::<Value>("! '123'").unwrap(), Value::from("123"));
    assert_eq!(from_str::<Value>("! null").unwrap(), Value::from("null"));
}

#[test]
fn invalid_explicit_scalar_tags_are_errors() {
    assert!(from_str::<Value>("!!int abc").is_err());
    assert!(from_str::<Value>("!!bool maybe").is_err());
    assert!(from_str::<Value>("!!float abc").is_err());
    assert!(from_str::<Value>("!!seq {a: 1}").is_err());
    assert!(from_str::<Value>("!!map [1, 2]").is_err());
    assert!(from_str::<Value>("!!str [1, 2]").is_err());
}

#[test]
fn explicit_float_accepts_integer_form() {
    assert_eq!(from_str::<Value>("!!float 1").unwrap(), number(Number::Float(1.0)));
    assert_eq!(from_str::<Value>("!!float -1").unwrap(), number(Number::Float(-1.0)));
    assert_eq!(from_str::<Value>("!!float +1").unwrap(), number(Number::Float(1.0)));
}

#[test]
fn unknown_tag_respects_scalar_style() {
    assert_eq!(from_str::<Value>("!point '123'").unwrap(), tagged("!point", Value::from("123")));
    assert_eq!(from_str::<Value>("!point ''").unwrap(), tagged("!point", Value::from("")));
    assert_eq!(
        from_str::<Value>("!point 123").unwrap(),
        tagged("!point", number(Number::PosInt(123)))
    );
}

#[test]
fn only_plain_untagged_merge_keys_merge() {
    let merged = from_str::<Value>("<<: {a: 1}\n").unwrap();
    assert_eq!(merged["a"], Value::from(1));

    // An explicit merge tag takes precedence over the scalar style.
    let merged = from_str::<Value>("!!merge '<<': {a: 1}\n").unwrap();
    assert_eq!(merged["a"], Value::from(1));

    for key in ["\"<<\"", "'<<'", "!!str <<"] {
        let value = from_str::<Value>(&format!("{key}: {{a: 1}}\n")).unwrap();
        assert!(value["<<"].is_mapping(), "key: {key}, value: {value:?}");
        assert!(value.get("a").is_none(), "key: {key}, value: {value:?}");
    }
}

#[test]
fn limits_are_per_document() {
    let values = Options {
        max_values: Some(2),
        ..Options::default()
    };
    let parsed = yaml::from_str_multi_with_options::<Value>("---\n1\n---\n2\n---\n3\n---\n4\n", values).unwrap();
    assert_eq!(parsed.len(), 4);

    let aliases = Options {
        max_aliases: Some(1),
        ..Options::default()
    };
    let parsed = yaml::from_str_multi_with_options::<Value>("a: &a 1\nb: *a\n---\nc: &c 2\nd: *c\n", aliases).unwrap();
    assert_eq!(parsed.len(), 2);
}

#[test]
fn single_document_apis_stop_after_the_first_document() {
    // The second document is malformed, but the first-document API must not
    // parse it.
    assert_eq!(from_str::<Value>("1\n---\n[").unwrap(), number(Number::PosInt(1)));

    let options = Options {
        max_documents: Some(1),
        ..Options::default()
    };
    assert_eq!(
        from_str_with_options::<Value>("1\n---\n2\n", options).unwrap(),
        number(Number::PosInt(1))
    );
}

#[test]
fn multi_document_api_distinguishes_empty_streams_from_null_documents() {
    assert_eq!(from_str_multi::<Value>("").unwrap(), Vec::new());
    assert_eq!(from_str_multi::<Value>("# comment only\n").unwrap(), Vec::new());
    assert_eq!(from_str_multi::<Value>("---\n").unwrap(), vec![Value::Null]);
    assert_eq!(from_str::<Value>("").unwrap(), Value::Null);
}

#[test]
fn mapping_equality_is_unordered() {
    let mut left = Mapping::new();
    left.insert(Value::from("x"), Value::from(1));
    left.insert(Value::from("y"), Value::from(2));

    let mut right = Mapping::new();
    right.insert(Value::from("y"), Value::from(2));
    right.insert(Value::from("x"), Value::from(1));

    assert_eq!(Value::Mapping(left), Value::Mapping(right));
}

#[test]
fn complex_duplicate_keys_are_detected() {
    let input = "? {x: 1, y: 2}\n: first\n? {y: 2, x: 1}\n: second\n";
    assert!(from_str::<Value>(input).is_err());
}

#[test]
fn nan_values_are_reflexive_and_duplicate_nan_keys_are_detected() {
    let value = number(Number::Float(f64::NAN));
    assert_eq!(value, value.clone());
    assert_eq!(Value::Sequence(vec![value.clone()]), Value::Sequence(vec![value]));
    assert!(from_str::<Value>(".nan: 1\n.nan: 2\n").is_err());
}

#[test]
fn mapping_from_iter_deduplicates_keys() {
    let mapping: Mapping = vec![(Value::from("a"), Value::from(1)), (Value::from("a"), Value::from(2))]
        .into_iter()
        .collect();
    assert_eq!(mapping.len(), 1);
    assert_eq!(mapping.get_str("a"), Some(&Value::from(2)));
}

#[test]
fn timestamps_and_binary_keep_their_tag_and_text() {
    let timestamp = tagged("tag:yaml.org,2002:timestamp", Value::from("2020"));
    assert_eq!(from_str::<Value>("!!timestamp 2020").unwrap(), timestamp);
    assert_eq!(from_str::<Value>(&to_string(&timestamp).unwrap()).unwrap(), timestamp);
    assert!(from_str::<Value>("!!timestamp {a: 1}").is_err());

    let binary = tagged("tag:yaml.org,2002:binary", Value::from("aGVsbG8="));
    assert_eq!(from_str::<Value>("!!binary aGVsbG8=").unwrap(), binary);
    assert_eq!(from_str::<Value>(&to_string(&binary).unwrap()).unwrap(), binary);
}

#[test]
fn nested_and_non_specific_tags_are_rejected_by_the_emitter() {
    let nested = tagged("!outer", tagged("!inner", Value::from(1)));
    assert!(to_string(&nested).is_err());
    assert!(to_string(&tagged("!", Value::from(1))).is_err());
}

#[test]
fn input_size_limit_applies_to_slices_and_readers() {
    let options = Options {
        max_input_bytes: Some(4),
        ..Options::default()
    };
    assert!(from_str_with_options::<Value>("12345", options).is_err());
    assert_eq!(
        from_str_with_options::<Value>("1234", options).unwrap(),
        number(Number::PosInt(1234))
    );

    #[cfg(feature = "std")]
    {
        use std::io::Cursor;

        let input = b"12345".to_vec();
        assert!(yaml::from_reader_with_options::<Value, _>(Cursor::new(input.clone()), options).is_err());
        let input = b"1234".to_vec();
        assert_eq!(
            yaml::from_reader_with_options::<Value, _>(Cursor::new(input), options).unwrap(),
            number(Number::PosInt(1234))
        );
    }
}

#[test]
fn anchors_and_aliases_are_charged_to_the_value_budget() {
    let tight = Options {
        max_values: Some(8),
        ..Options::default()
    };
    assert!(from_str_with_options::<Value>("a: &a [1, 2, 3]\n", tight).is_err());

    let aliases = Options {
        max_values: Some(12),
        ..Options::default()
    };
    assert!(from_str_with_options::<Value>("a: &a [1, 2, 3]\nb: *a\nc: *a\n", aliases).is_err());
}
