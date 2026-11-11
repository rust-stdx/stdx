#![cfg(feature = "alloc")]

use json::{DuplicateKeys, Options, Value, json};

#[test]
fn primitives() {
    assert!(json::from_str::<bool>("true").unwrap());
    assert!(!json::from_str::<bool>("false").unwrap());
    assert_eq!(json::from_str::<()>("null").unwrap(), ());
    assert_eq!(json::from_str::<i64>("-42").unwrap(), -42);
    assert_eq!(json::from_str::<u64>("18446744073709551615").unwrap(), u64::MAX);
    assert_eq!(json::from_str::<i64>("-9223372036854775808").unwrap(), i64::MIN);
    let max: Value = json::from_str("18446744073709551615").unwrap();
    assert_eq!(max.as_u64(), Some(u64::MAX));
    let min: Value = json::from_str("-9223372036854775808").unwrap();
    assert_eq!(min.as_i64(), Some(i64::MIN));
    assert_eq!(
        json::from_str::<u128>("340282366920938463463374607431768211455").unwrap(),
        u128::MAX
    );
    assert_eq!(json::from_str::<f64>("1.5").unwrap(), 1.5);
    assert_eq!(json::from_str::<String>("\"hello\"").unwrap(), "hello");
    assert_eq!(json::from_str::<&str>("\"hello\"").unwrap(), "hello");
}

#[test]
fn arrays_and_objects() {
    assert_eq!(json::from_str::<Vec<i32>>("[1,2,3]").unwrap(), vec![1, 2, 3]);
    let value: Value = json::from_str(r#"{"a":1,"b":[true,null,"x"]}"#).unwrap();
    assert_eq!(value["a"].as_i64(), Some(1));
    assert_eq!(value["b"][0].as_bool(), Some(true));
    assert_eq!(value["b"][2].as_str(), Some("x"));
    assert!(value["missing"].is_null());
}

#[test]
fn escapes() {
    let value: Value = json::from_str(r#""a\nb\t\"c\"\\\u00e9\uD834\uDD1E""#).unwrap();
    assert_eq!(value.as_str(), Some("a\nb\t\"c\"\\é𝄞"));
    let borrowed = json::from_str::<&str>(r#""plain""#).unwrap();
    assert_eq!(borrowed, "plain");
    assert!(json::from_str::<&str>(r#""needs\\escape""#).is_err());
    assert_eq!(json::from_str::<String>(r#""needs\\escape""#).unwrap(), r"needs\escape");
}

#[test]
fn invalid_documents() {
    for text in [
        "",
        "tru",
        "01",
        "1.",
        "1e",
        "[1,]",
        "{\"a\":1,}",
        "{\"a\" 1}",
        "[1 2]",
        "\"\\q\"",
        "\"\\uD800\"",
        "\"\n\"",
        "{\"a\":1} x",
        "{\"a\":1,}",
    ] {
        assert!(json::from_str::<Value>(text).is_err(), "accepted {text:?}");
    }
}

#[test]
fn duplicate_policies() {
    let text = r#"{"a":1,"a":2}"#;
    let value: Value = json::from_str(text).unwrap();
    assert_eq!(value["a"].as_i64(), Some(2));
    let options = Options {
        duplicate_keys: DuplicateKeys::FirstWins,
        ..Options::default()
    };
    let value: Value = json::from_str_with_options(text, options).unwrap();
    assert_eq!(value["a"].as_i64(), Some(1));
    let options = Options {
        duplicate_keys: DuplicateKeys::Reject,
        ..Options::default()
    };
    assert!(json::from_str_with_options::<Value>(text, options).is_err());
}

#[test]
fn depths() {
    let deep = "[".repeat(200) + &"]".repeat(200);
    assert!(json::from_str::<Value>(&deep).is_err());
    let ok = "[".repeat(100) + &"]".repeat(100);
    assert!(json::validate(ok.as_bytes()).is_ok());
}

#[test]
fn json_macro() {
    let value = json!({
        "name": "alice",
        "age": 42,
        "tags": [true, null, 1.5],
    });
    assert_eq!(value["name"].as_str(), Some("alice"));
    assert_eq!(value["age"].as_i64(), Some(42));
    assert_eq!(value["tags"][2].as_f64(), Some(1.5));
}

#[test]
fn validate_and_minify() {
    let text = br#" { "a" : [ 1, 2 ] } "#;
    json::validate(text).unwrap();
    let mut out = [0u8; 64];
    let written = json::minify(text, &mut out).unwrap();
    assert_eq!(&out[..written], br#"{"a":[1,2]}"#);
}

#[test]
fn raw_values() {
    let raw = json::RawValue::new(br#" {"a": [1,2]} "#).unwrap();
    assert_eq!(raw.as_str().unwrap(), r#"{"a": [1,2]}"#);
    let value: Value = raw.deserialize().unwrap();
    assert_eq!(value["a"][1].as_i64(), Some(2));
    assert!(json::RawValue::new(br#"{"a":}"#).is_err());
    let number = json::RawNumber::new("1.2300").unwrap();
    assert_eq!(number.as_str(), "1.2300");
    assert_eq!(number.as_f64().unwrap(), 1.23);
}

#[test]
fn lazy_document() {
    let document = json::Document::parse(br#"{"a":{"b":[1,2,3]},"s":"ok"}"#).unwrap();
    let root = document.root();
    assert_eq!(root.get("s").unwrap().unwrap().as_str().unwrap(), Some("ok"));
    assert_eq!(document.get("/a/b/2").unwrap().unwrap().as_i64().unwrap(), Some(3));
    let mut buffer = [0u8; 8];
    let escaped = json::Document::parse(br#""a\nb""#).unwrap();
    assert_eq!(escaped.root().string_into(&mut buffer).unwrap(), "a\nb");
}

#[test]
fn streams() {
    let mut stream = json::StreamDeserializer::<Value>::new(b"1 2 {\"a\":3}\n");
    assert_eq!(stream.next().unwrap().unwrap().as_i64(), Some(1));
    assert_eq!(stream.next().unwrap().unwrap().as_i64(), Some(2));
    assert_eq!(stream.next().unwrap().unwrap()["a"].as_i64(), Some(3));
    assert!(stream.next().is_none());
}

#[test]
fn encoding() {
    let value = json!({"a": 1, "b": [true, null], "c": "x\n"});
    assert_eq!(json::to_string(&value), r#"{"a":1,"b":[true,null],"c":"x\n"}"#);
    let mut buffer = [0u8; 64];
    let used = json::to_slice(&value, &mut buffer).unwrap();
    assert_eq!(used, json::to_string(&value).as_bytes());
    assert_eq!(json::encoded_len(&value), json::to_string(&value).len());
    let pretty = json::to_string_pretty(&value);
    assert!(pretty.contains("\n  \"a\""));
}

#[test]
fn value_into_owned() {
    let text = String::from(r#"{"a":"x","b":[1,2]}"#);
    let value: Value = json::from_str(&text).unwrap();
    let owned = value.into_owned();
    drop(text);
    assert_eq!(owned["a"].as_str(), Some("x"));
    assert_eq!(owned["b"][1].as_i64(), Some(2));
}

#[test]
fn container_limits() {
    let options = Options {
        max_container_entries: Some(2),
        ..Options::default()
    };
    assert!(json::from_str_with_options::<Value>("[1,2]", options).is_ok());
    assert!(json::from_str_with_options::<Value>("[1,2,3]", options).is_err());
    assert!(json::from_str_with_options::<Value>(r#"{"a":1,"b":2,"c":3}"#, options).is_err());
}

#[cfg(feature = "std")]
#[test]
fn readers_and_lines() {
    use std::io::Cursor;

    let value: Value = json::from_reader(Cursor::new(br#"{"a":1}"#)).unwrap();
    assert_eq!(value["a"].as_i64(), Some(1));

    let lines = json::JsonLines::<_, Value>::new(Cursor::new(b"{\"a\":1}\n\n{\"a\":2}\n"));
    let values: Vec<Value> = lines.map(|line| line.unwrap()).collect();
    assert_eq!(values.len(), 2);
    assert_eq!(values[1]["a"].as_i64(), Some(2));
}

#[cfg(all(feature = "heapless", feature = "derive"))]
#[test]
fn heapless_containers() {
    #[derive(Debug, PartialEq, json::Encode, json::Decode)]
    struct Small {
        name: heapless::String<16>,
        values: heapless::Vec<i32, 4>,
    }

    let value = Small {
        name: heapless::String::try_from("alice").unwrap(),
        values: heapless::Vec::from_slice(&[1, 2, 3]).unwrap(),
    };
    let text = json::to_string(&value);
    assert_eq!(text, r#"{"name":"alice","values":[1,2,3]}"#);
    assert_eq!(json::from_str::<Small>(&text).unwrap(), value);
}

#[cfg(all(feature = "small_collections", feature = "derive"))]
#[test]
fn small_collections_containers() {
    #[derive(Debug, PartialEq, json::Encode, json::Decode)]
    struct Small {
        name: small_collections::SmallString<16>,
        values: small_collections::SmallVec<i32, 4>,
    }

    let mut name = small_collections::SmallString::new();
    name.push_str("alice");
    let mut values = small_collections::SmallVec::new();
    values.push(1);
    values.push(2);
    let value = Small {
        name,
        values,
    };
    let text = json::to_string(&value);
    assert_eq!(text, r#"{"name":"alice","values":[1,2]}"#);
    assert_eq!(json::from_str::<Small>(&text).unwrap(), value);
}

#[test]
fn object_equality_ignores_order() {
    let a = json!({"a": 1, "b": 2});
    let b = json!({"b": 2, "a": 1});
    assert_eq!(a, b);
}

#[test]
fn large_object_duplicate_policy() {
    let mut text = String::from("{");
    for i in 0..20 {
        if i != 0 {
            text.push(',');
        }
        text.push_str(&format!("\"k{i}\":{i}"));
    }
    text.push_str(",\"k5\":99}");
    let value: Value = json::from_str(&text).unwrap();
    assert_eq!(value["k5"].as_i64(), Some(99));
    assert_eq!(value.as_object().unwrap().len(), 20);
    let options = Options {
        duplicate_keys: DuplicateKeys::Reject,
        ..Options::default()
    };
    assert!(json::from_str_with_options::<Value>(&text, options).is_err());
}

#[test]
fn owned_decode_helpers() {
    let value = json::from_str_owned::<json::OwnedValue>(r#"{"a":1}"#).unwrap();
    assert_eq!(value["a"].as_i64(), Some(1));
    let raw = json::RawValueBuf::try_from_slice(br#"{"a":1}"#).unwrap();
    assert_eq!(raw.as_str().unwrap(), r#"{"a":1}"#);
    assert!(json::RawValueBuf::try_from_slice(br#"{"a":}"#).is_err());
}

#[test]
fn large_object_duplicate_policies_past_inline_capacity() {
    let mut text = String::from("{");
    for i in 0..200 {
        if i != 0 {
            text.push(',');
        }
        text.push_str(&format!("\"k{i}\":{i}"));
    }
    // Duplicate keys beyond the 64-entry inline path.
    text.push_str(",\"k5\":999,\"k150\":888}");
    let value: Value = json::from_str(&text).unwrap();
    assert_eq!(value["k5"].as_i64(), Some(999));
    assert_eq!(value["k150"].as_i64(), Some(888));
    assert_eq!(value.as_object().unwrap().len(), 200);

    let first: Value = json::from_str_with_options(
        &text,
        Options {
            duplicate_keys: DuplicateKeys::FirstWins,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(first["k5"].as_i64(), Some(5));
    assert_eq!(first["k150"].as_i64(), Some(150));

    let reject: json::Result<Value> = json::from_str_with_options(
        &text,
        Options {
            duplicate_keys: DuplicateKeys::Reject,
            ..Options::default()
        },
    );
    assert!(reject.is_err());
}
