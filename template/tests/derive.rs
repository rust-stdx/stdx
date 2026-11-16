#![cfg(feature = "derive")]

use template::{Engine, EscapeMode, ToValue};

#[derive(ToValue)]
struct User {
    name: String,
    age: i32,
}

#[derive(ToValue)]
#[template(rename_all = "camelCase")]
struct Renamed {
    first_name: String,
    #[template(rename = "years")]
    age: i32,
    #[template(skip)]
    #[allow(dead_code)]
    ignored: bool,
}

#[derive(ToValue)]
struct Nested {
    user: User,
    tags: Vec<String>,
}

#[derive(ToValue)]
enum Message {
    Ping,
    Text(String),
    Pair(u8, u8),
    Named { code: u8 },
}

#[test]
fn derived_struct_renders() {
    let mut engine = Engine::new(EscapeMode::Text);
    engine.add_template("t", "{{ name }} is {{ age }}").unwrap();

    let result = engine
        .render(
            "t",
            &User {
                name: "Bob".into(),
                age: 30,
            },
        )
        .unwrap();
    assert_eq!(result, "Bob is 30");
}

#[test]
fn derived_renames_and_skips() {
    let mut engine = Engine::new(EscapeMode::Text);
    engine
        .add_template("t", "{{ firstName }} {{ years }} {{ ignored }}")
        .unwrap();

    let value = Renamed {
        first_name: "Alice".into(),
        age: 42,
        ignored: true,
    };
    assert_eq!(engine.render("t", &value).unwrap(), "Alice 42 ");
}

#[test]
fn derived_nested_struct_renders() {
    let mut engine = Engine::new(EscapeMode::Text);
    engine
        .add_template("t", "{{ user.name }}: {{ tags | join(\", \") }}")
        .unwrap();

    let value = Nested {
        user: User {
            name: "Carol".into(),
            age: 25,
        },
        tags: vec!["admin".into(), "ops".into()],
    };
    assert_eq!(engine.render("t", &value).unwrap(), "Carol: admin, ops");
}

#[test]
fn derived_enum_shapes_render() {
    let mut engine = Engine::new(EscapeMode::Text);
    engine.add_template("unit", "{{ value }}").unwrap();
    engine.add_template("newtype", "{{ value }}").unwrap();
    engine.add_template("tuple", "{{ value[0] }}/{{ value[1] }}").unwrap();
    engine.add_template("struct", "{{ value.code }}").unwrap();

    let mut unit_ctx = std::collections::BTreeMap::new();
    unit_ctx.insert("value", Message::Ping);
    assert_eq!(engine.render("unit", &unit_ctx).unwrap(), "Ping");

    let mut newtype_ctx = std::collections::BTreeMap::new();
    newtype_ctx.insert("value", Message::Text("hello".into()));
    assert_eq!(engine.render("newtype", &newtype_ctx).unwrap(), "hello");

    let mut tuple_ctx = std::collections::BTreeMap::new();
    tuple_ctx.insert("value", Message::Pair(1, 2));
    assert_eq!(engine.render("tuple", &tuple_ctx).unwrap(), "1/2");

    let mut struct_ctx = std::collections::BTreeMap::new();
    struct_ctx.insert(
        "value",
        Message::Named {
            code: 7,
        },
    );
    assert_eq!(engine.render("struct", &struct_ctx).unwrap(), "7");
}
