#![cfg(feature = "derive")]

use std::collections::HashMap;

use bel::{Context, IntoValue, Value};

#[derive(IntoValue)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(IntoValue)]
#[bel(rename_all = "camelCase")]
struct Config {
    listen_port: i64,
    #[bel(skip)]
    #[allow(dead_code)]
    internal: bool,
    #[bel(rename = "tag")]
    label: String,
}

#[derive(IntoValue)]
struct Pair(i64, bool);

#[derive(IntoValue)]
struct Newtype(i64);

#[derive(IntoValue)]
#[bel(transparent)]
struct Transparent {
    inner: i64,
}

#[derive(IntoValue)]
enum Shape {
    Empty,
    Number(i64),
    Pair(i64, i64),
    Named { x: i64 },
}

#[test]
fn struct_becomes_map() {
    let value = Value::from(Point {
        x: 1,
        y: 2,
    });
    let Value::Map(map) = value else {
        panic!("expected map");
    };
    assert_eq!(map.get(&"x".into()), Some(&Value::from(1)));
    assert_eq!(map.get(&"y".into()), Some(&Value::from(2)));
}

#[test]
fn rename_skip_and_transparent() {
    let value = Value::from(Config {
        listen_port: 8080,
        internal: true,
        label: "api".to_string(),
    });
    let Value::Map(map) = value else {
        panic!("expected map");
    };
    assert_eq!(map.get(&"listenPort".into()), Some(&Value::from(8080)));
    assert_eq!(map.get(&"tag".into()), Some(&Value::from("api")));
    assert_eq!(map.get(&"internal".into()), None);
    assert_eq!(map.get(&"label".into()), None);

    assert_eq!(
        Value::from(Transparent {
            inner: 7
        }),
        Value::from(7)
    );
}

#[test]
fn tuple_and_newtype_structs() {
    assert_eq!(Value::from(Newtype(3)), Value::from(3));
    assert_eq!(Value::from(Pair(1, true)), Value::from(vec![Value::from(1), Value::from(true)]));
}

#[test]
fn enums_use_external_tags() {
    assert_eq!(Value::from(Shape::Empty), Value::from("Empty"));
    assert_eq!(
        Value::from(Shape::Number(4)),
        Value::from(HashMap::from([("Number", Value::from(4))]))
    );
    assert_eq!(
        Value::from(Shape::Pair(1, 2)),
        Value::from(HashMap::from([("Pair", Value::from(vec![Value::from(1), Value::from(2)]))]))
    );
    assert_eq!(
        Value::from(Shape::Named {
            x: 9
        }),
        Value::from(HashMap::from([("Named", Value::from(HashMap::from([("x", Value::from(9))])))]))
    );
}

#[test]
fn derived_values_can_be_added_to_a_context() {
    let mut context = Context::default();
    context
        .add_variable(
            "point",
            Point {
                x: 1,
                y: 2,
            },
        )
        .unwrap();
    let program = bel::Program::compile("point.x + point.y == 3").unwrap();
    assert_eq!(program.execute(&context).unwrap(), Value::Bool(true));
}
