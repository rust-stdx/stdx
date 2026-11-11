#![cfg(all(feature = "derive", feature = "alloc"))]

use json::{Decode, Encode, Value, json};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn named_struct() {
    let point = Point {
        x: 1.5,
        y: -2.0,
    };
    let text = json::to_string(&point);
    assert_eq!(text, r#"{"x":1.5,"y":-2.0}"#);
    let back: Point = json::from_str(&text).unwrap();
    assert_eq!(point, back);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair(i32, String, bool);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Unit;

#[test]
fn tuple_and_unit_structs() {
    let pair = Pair(1, "x".into(), true);
    let text = json::to_string(&pair);
    assert_eq!(text, r#"[1,"x",true]"#);
    assert_eq!(json::from_str::<Pair>(&text).unwrap(), pair);
    assert_eq!(json::to_string(&Unit), "null");
    assert_eq!(json::from_str::<Unit>("null").unwrap(), Unit);
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[json(rename_all = "camelCase", deny_unknown_fields)]
struct Renamed {
    first_name: String,
    #[json(rename = "years")]
    age: u32,
    #[json(default)]
    nickname: Option<String>,
}

#[test]
fn rename_default_and_unknown_fields() {
    let value = Renamed {
        first_name: "Alice".into(),
        age: 42,
        nickname: None,
    };
    let text = json::to_string(&value);
    assert_eq!(text, r#"{"firstName":"Alice","years":42,"nickname":null}"#);
    let back: Renamed = json::from_str(r#"{"firstName":"Alice","years":42}"#).unwrap();
    assert_eq!(back, value);
    assert!(json::from_str::<Renamed>(r#"{"firstName":"Alice","years":42,"x":1}"#).is_err());
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[json(transparent)]
struct Transparent(Point);

#[test]
fn transparent() {
    let value = Transparent(Point {
        x: 1.0,
        y: 2.0,
    });
    let text = json::to_string(&value);
    assert_eq!(text, r#"{"x":1.0,"y":2.0}"#);
    assert_eq!(json::from_str::<Transparent>(&text).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum External {
    Unit,
    Newtype(i32),
    Tuple(i32, String),
    Struct { a: i32, b: bool },
}

#[test]
fn external_enum() {
    assert_eq!(json::to_string(&External::Unit), r#""Unit""#);
    assert_eq!(json::to_string(&External::Newtype(3)), r#"{"Newtype":3}"#);
    assert_eq!(json::to_string(&External::Tuple(1, "x".into())), r#"{"Tuple":[1,"x"]}"#);
    assert_eq!(
        json::to_string(&External::Struct {
            a: 1,
            b: true
        }),
        r#"{"Struct":{"a":1,"b":true}}"#
    );
    for value in [
        External::Unit,
        External::Newtype(3),
        External::Tuple(1, "x".into()),
        External::Struct {
            a: 1,
            b: true,
        },
    ] {
        let text = json::to_string(&value);
        assert_eq!(json::from_str::<External>(&text).unwrap(), value, "{text}");
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[json(tag = "type")]
enum Internal {
    Empty,
    Data { a: i32, b: bool },
}

#[test]
fn internal_enum() {
    assert_eq!(json::to_string(&Internal::Empty), r#"{"type":"Empty"}"#);
    assert_eq!(
        json::to_string(&Internal::Data {
            a: 1,
            b: false
        }),
        r#"{"type":"Data","a":1,"b":false}"#
    );
    for value in [
        Internal::Empty,
        Internal::Data {
            a: 1,
            b: false,
        },
    ] {
        let text = json::to_string(&value);
        assert_eq!(json::from_str::<Internal>(&text).unwrap(), value, "{text}");
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[json(tag = "t", content = "c")]
enum Adjacent {
    Unit,
    Newtype(i32),
    Struct { a: i32 },
}

#[test]
fn adjacent_enum() {
    assert_eq!(json::to_string(&Adjacent::Unit), r#"{"t":"Unit"}"#);
    assert_eq!(json::to_string(&Adjacent::Newtype(4)), r#"{"t":"Newtype","c":4}"#);
    assert_eq!(
        json::to_string(&Adjacent::Struct {
            a: 1
        }),
        r#"{"t":"Struct","c":{"a":1}}"#
    );
    for value in [
        Adjacent::Unit,
        Adjacent::Newtype(4),
        Adjacent::Struct {
            a: 1,
        },
    ] {
        let text = json::to_string(&value);
        assert_eq!(json::from_str::<Adjacent>(&text).unwrap(), value, "{text}");
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Borrowed<'a> {
    #[json(borrow)]
    name: &'a str,
    value: u32,
}

#[test]
fn borrowed_struct() {
    let value: Borrowed<'_> = json::from_str(r#"{"name":"alice","value":1}"#).unwrap();
    assert_eq!(value.name, "alice");
    assert_eq!(value.value, 1);
    assert_eq!(json::to_string(&value), r#"{"name":"alice","value":1}"#);
    assert!(json::from_str::<Borrowed<'_>>(r#"{"name":"a\nb","value":1}"#).is_err());
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Generic<T> {
    value: T,
}

#[test]
fn generic_struct() {
    let value = Generic {
        value: vec![1, 2, 3],
    };
    let text = json::to_string(&value);
    assert_eq!(text, r#"{"value":[1,2,3]}"#);
    assert_eq!(json::from_str::<Generic<Vec<i32>>>(&text).unwrap(), value);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithRaw<'a> {
    #[json(raw)]
    raw: json::RawValue<'a>,
}

#[test]
fn raw_field() {
    let value = WithRaw {
        raw: json::RawValue::new(br#"{"a": [1,2]}"#).unwrap(),
    };
    let text = json::to_string(&value);
    assert_eq!(text, r#"{"raw":{"a": [1,2]}}"#);
}

#[test]
fn generic_value_roundtrip() {
    let value = json!({"a": 1, "b": [true, null]});
    let text = json::to_string(&value);
    let back: Value = json::from_str(&text).unwrap();
    assert_eq!(value, back);
}

mod hex_string {
    use super::json_crate;

    pub fn serialize<W: json_crate::Write>(
        value: &str,
        encoder: &mut json_crate::Encoder<W>,
    ) -> core::result::Result<(), W::Error> {
        encoder.string(value)
    }

    pub fn deserialize<'a>(decoder: &mut json_crate::Decoder<'a>) -> json_crate::Result<String> {
        decoder.decode_string()
    }
}

// Alias the crate so the `with` module can refer to it without importing the
// test's `json` name into the macro expansion.
use json as json_crate;

#[derive(Debug, PartialEq, Encode, Decode)]
struct Extras {
    #[json(alias = "old_name")]
    name: String,
    #[json(skip_serializing_if = "Option::is_none")]
    maybe: Option<i32>,
    #[json(skip)]
    ignored: bool,
    #[json(with = "hex_string")]
    text: String,
}

#[test]
fn aliases_skip_if_and_with() {
    let value = Extras {
        name: "a".into(),
        maybe: None,
        ignored: true,
        text: "b".into(),
    };
    let text = json::to_string(&value);
    assert_eq!(text, r#"{"name":"a","text":"b"}"#);
    let back: Extras = json::from_str(r#"{"old_name":"a","maybe":2,"text":"b"}"#).unwrap();
    assert_eq!(back.name, "a");
    assert_eq!(back.maybe, Some(2));
    assert!(!back.ignored);
    assert_eq!(back.text, "b");
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Duplicates {
    value: i32,
}

#[test]
fn duplicate_field_policy() {
    use json::{DuplicateKeys, Options};

    let last: Duplicates = json::from_str(r#"{"value":1,"value":2}"#).unwrap();
    assert_eq!(last.value, 2);

    let first: Duplicates = json::from_str_with_options(
        r#"{"value":1,"value":2}"#,
        Options {
            duplicate_keys: DuplicateKeys::FirstWins,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(first.value, 1);

    let result: json::Result<Duplicates> = json::from_str_with_options(
        r#"{"value":1,"value":2}"#,
        Options {
            duplicate_keys: DuplicateKeys::Reject,
            ..Options::default()
        },
    );
    assert!(result.is_err());
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[json(untagged)]
enum Untagged {
    Point(Point),
    Text(String),
    Number(i64),
    Nothing,
}

#[test]
fn untagged_enum() {
    for value in [
        Untagged::Point(Point {
            x: 1.0,
            y: 2.0,
        }),
        Untagged::Text("hello".into()),
        Untagged::Number(7),
        Untagged::Nothing,
    ] {
        let text = json::to_string(&value);
        let back: Untagged = json::from_str(&text).unwrap();
        assert_eq!(back, value, "{text}");
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[json(deny_duplicate_fields)]
struct LargeStruct {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
    f4: u8,
    f5: u8,
    f6: u8,
    f7: u8,
    f8: u8,
    f9: u8,
    f10: u8,
    f11: u8,
    f12: u8,
    f13: u8,
    f14: u8,
    f15: u8,
    f16: u8,
    f17: u8,
    f18: u8,
    f19: u8,
    f20: u8,
    f21: u8,
    f22: u8,
    f23: u8,
    f24: u8,
    f25: u8,
    f26: u8,
    f27: u8,
    f28: u8,
    f29: u8,
    f30: u8,
    f31: u8,
    f32: u8,
    f33: u8,
    f34: u8,
    f35: u8,
    f36: u8,
    f37: u8,
    f38: u8,
    f39: u8,
    f40: u8,
    f41: u8,
    f42: u8,
    f43: u8,
    f44: u8,
    f45: u8,
    f46: u8,
    f47: u8,
    f48: u8,
    f49: u8,
    f50: u8,
    f51: u8,
    f52: u8,
    f53: u8,
    f54: u8,
    f55: u8,
    f56: u8,
    f57: u8,
    f58: u8,
    f59: u8,
    f60: u8,
    f61: u8,
    f62: u8,
    f63: u8,
    f64: u8,
    f65: u8,
    f66: u8,
    f67: u8,
    f68: u8,
    f69: u8,
    f70: u8,
    f71: u8,
    f72: u8,
    f73: u8,
    f74: u8,
    f75: u8,
    f76: u8,
    f77: u8,
    f78: u8,
    f79: u8,
    f80: u8,
    f81: u8,
    f82: u8,
    f83: u8,
    f84: u8,
    f85: u8,
    f86: u8,
    f87: u8,
    f88: u8,
    f89: u8,
    f90: u8,
    f91: u8,
    f92: u8,
    f93: u8,
    f94: u8,
    f95: u8,
    f96: u8,
    f97: u8,
    f98: u8,
    f99: u8,
    f100: u8,
    f101: u8,
    f102: u8,
    f103: u8,
    f104: u8,
    f105: u8,
    f106: u8,
    f107: u8,
    f108: u8,
    f109: u8,
    f110: u8,
    f111: u8,
    f112: u8,
    f113: u8,
    f114: u8,
    f115: u8,
    f116: u8,
    f117: u8,
    f118: u8,
    f119: u8,
    f120: u8,
    f121: u8,
    f122: u8,
    f123: u8,
    f124: u8,
    f125: u8,
    f126: u8,
    f127: u8,
    f128: u8,
    f129: u8,
}

#[test]
fn duplicate_detection_works_past_128_fields() {
    let mut text = String::from("{");
    for i in 0..130 {
        if i != 0 {
            text.push(',');
        }
        text.push_str(&format!("\"f{i}\":{i}"));
    }
    text.push_str(",\"f129\":1}");
    assert!(json::from_str::<LargeStruct>(&text).is_err());
}
