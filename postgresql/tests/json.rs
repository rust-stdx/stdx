//! JSON support: `#[derive(Json)]` and the `Json<T>` wrapper.
//!
//! These tests exercise the encode/decode traits directly and need no
//! database.

use std::collections::HashMap;

use postgresql::{
    __private::BytesMut,
    FromSql, Json, RowShape, ToSql,
    types::{INT4OID, JSONBOID, JSONOID},
};

#[derive(serde::Serialize, serde::Deserialize, Json, Debug, PartialEq)]
struct User {
    name: String,
    admin: bool,
}

/// Encodes a value the way the bind path does, returning the `jsonb` payload.
fn encode<T: ToSql>(value: &T) -> Vec<u8> {
    let mut buf = BytesMut::new();
    value.encode(&mut buf).unwrap();
    buf.to_vec()
}

#[test]
fn derive_roundtrips_through_jsonb() {
    let user = User {
        name: "alice".to_string(),
        admin: true,
    };

    let buf = encode(&user);
    assert_eq!(buf[0], 1, "jsonb payload starts with the format version byte");

    let back = User::from_sql(JSONBOID, &buf).unwrap();
    assert_eq!(back, user);
}

#[test]
fn derive_decodes_plain_json() {
    let payload = br#"{"name":"bob","admin":false}"#;
    let user = User::from_sql(JSONOID, payload).unwrap();
    assert_eq!(
        user,
        User {
            name: "bob".to_string(),
            admin: false,
        }
    );
}

#[test]
fn derive_reports_the_json_oids() {
    assert!(User::accepts(JSONBOID));
    assert!(User::accepts(JSONOID));
    assert!(!User::accepts(INT4OID));
}

#[test]
fn derive_rejects_a_non_json_column() {
    // The type check runs before any byte is reinterpreted.
    let err = User::from_sql(INT4OID, &1i32.to_be_bytes()).unwrap_err();
    assert!(err.to_string().contains("cannot decode OID"), "{err}");
}

#[test]
fn derive_declares_a_single_jsonb_column() {
    let shape = <User as RowShape>::SHAPE;
    assert_eq!(shape.len(), 1);
    assert_eq!(shape[0].type_oid, JSONBOID);
    assert!(!shape[0].nullable);
}

#[test]
fn wrapper_roundtrips_any_serde_type() {
    let mut attributes = HashMap::new();
    attributes.insert("score".to_string(), 42i32);

    let wrapped = Json(attributes.clone());
    let buf = encode(&wrapped);
    assert_eq!(buf[0], 1);

    let back: Json<HashMap<String, i32>> = Json::from_sql(JSONBOID, &buf).unwrap();
    assert_eq!(back.into_inner(), attributes);

    let shape = <Json<HashMap<String, i32>> as RowShape>::SHAPE;
    assert_eq!(shape[0].type_oid, JSONBOID);
}

#[derive(serde::Serialize, serde::Deserialize, Json, Debug, PartialEq)]
enum Level {
    Low,
    High(u8),
}

#[derive(serde::Serialize, serde::Deserialize, Json, Debug, PartialEq)]
struct Wrapper<T> {
    value: T,
}

#[test]
fn derive_works_for_enums_and_generic_types() {
    let level = Level::High(3);
    let buf = encode(&level);
    assert_eq!(Level::from_sql(JSONBOID, &buf).unwrap(), level);

    let wrapped = Wrapper {
        value: 7i32,
    };
    let buf = encode(&wrapped);
    assert_eq!(Wrapper::<i32>::from_sql(JSONBOID, &buf).unwrap(), wrapped);
}

#[test]
fn option_is_nullable_and_vec_is_an_array() {
    let none: Option<User> = FromSql::from_sql_null().unwrap();
    assert_eq!(none, None);

    // Optional JSON decodes from either representation.
    let one = encode(&User {
        name: "a".to_string(),
        admin: true,
    });
    assert_eq!(User::from_sql(JSONBOID, &one).unwrap().name, "a");

    // `Vec<User>` maps to `jsonb[]`.
    let shape = <Vec<User> as RowShape>::SHAPE;
    assert_eq!(shape.len(), 1);
    assert_eq!(shape[0].type_oid, postgresql::types::JSONB_ARRAY_OID);
}
