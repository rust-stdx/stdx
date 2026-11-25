use std::collections::HashMap;

use yaml::{Decode, Encode, Value};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[yaml(rename_all = "camelCase")]
struct Config {
    listen_port: u16,
    #[yaml(default)]
    workers: u16,
    name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair(i64, bool);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Newtype(i64);

#[derive(Debug, PartialEq, Encode, Decode)]
enum Shape {
    Empty,
    Number(i64),
    Pair(i64, i64),
    Named { x: i64 },
}

#[test]
fn derived_struct_roundtrip() {
    let point = Point {
        x: 1.5,
        y: -2.0,
    };
    let text = yaml::to_string(&point).unwrap();
    assert!(text.contains("x: 1.5"), "{text}");
    let decoded: Point = yaml::from_str(&text).unwrap();
    assert_eq!(decoded, point);
}

#[test]
fn rename_default_and_plain_mapping() {
    let config: Config = yaml::from_str(
        r#"
name: api
listenPort: 8080
"#,
    )
    .unwrap();
    assert_eq!(
        config,
        Config {
            listen_port: 8080,
            workers: 0,
            name: "api".to_string(),
        }
    );
    let text = yaml::to_string(&config).unwrap();
    assert!(text.contains("listenPort: 8080"), "{text}");
}

#[test]
fn tuple_and_newtype_structs() {
    assert_eq!(yaml::from_str::<Pair>("- 1\n- true\n").unwrap(), Pair(1, true));
    assert_eq!(yaml::from_str::<Newtype>("7\n").unwrap(), Newtype(7));
    assert_eq!(yaml::to_string(&Newtype(7)).unwrap().trim(), "7");
}

#[test]
fn enums_roundtrip() {
    for shape in [
        Shape::Empty,
        Shape::Number(4),
        Shape::Pair(1, 2),
        Shape::Named {
            x: 9,
        },
    ] {
        let text = yaml::to_string(&shape).unwrap();
        let decoded: Shape = yaml::from_str(&text).unwrap();
        assert_eq!(decoded, shape, "{text}");
    }
}

#[test]
fn strings_that_look_like_scalars_stay_strings() {
    let values = vec![
        "true".to_string(),
        "null".to_string(),
        "42".to_string(),
        "1.5".to_string(),
    ];
    let text = yaml::to_string(&values).unwrap();
    let decoded: Vec<String> = yaml::from_str(&text).unwrap();
    assert_eq!(decoded, values);
}

#[test]
fn dynamic_values_and_tags() {
    let value: Value = yaml::from_str(
        r#"
name: demo
enabled: true
count: 3
items:
  - one
  - 2
"#,
    )
    .unwrap();
    assert_eq!(value.get("name").and_then(Value::as_str), Some("demo"));
    assert_eq!(value.get("enabled").and_then(Value::as_bool), Some(true));
    assert_eq!(value.get("count").and_then(Value::as_i64), Some(3));
    assert_eq!(value.get("items").and_then(Value::as_sequence).map(Vec::len), Some(2));

    let tagged: Value = yaml::from_str("!example 42").unwrap();
    assert!(matches!(tagged, Value::Tagged(_)));
    let text = yaml::to_string(&tagged).unwrap();
    assert!(text.contains("!example"), "{text}");
}

#[test]
fn anchors_and_aliases() {
    let value: Value = yaml::from_str(
        r#"
base: &base
  x: 1
copy: *base
"#,
    )
    .unwrap();
    assert_eq!(value.get("base"), value.get("copy"));
}

#[test]
fn maps_roundtrip() {
    let value = HashMap::from([("a".to_string(), 1i64), ("b".to_string(), 2i64)]);
    let text = yaml::to_string(&value).unwrap();
    let decoded: HashMap<String, i64> = yaml::from_str(&text).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn special_floats() {
    let values: Vec<f64> = yaml::from_str("- .inf\n- -.inf\n- .nan\n").unwrap();
    assert_eq!(values[0], f64::INFINITY);
    assert_eq!(values[1], f64::NEG_INFINITY);
    assert!(values[2].is_nan());
}
