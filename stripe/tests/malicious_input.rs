//! Adversarial input tests: the client must never panic or hang on hostile
//! responses.

mod common;

use common::{Fields, MockClient};
use serde::Deserialize;
use stripe::Error;

#[derive(Debug, Deserialize)]
struct Charge {
    id: String,
    #[serde(default)]
    amount: i64,
}

#[tokio::test]
async fn deeply_nested_json_is_rejected_not_crashed() {
    let mock = MockClient::new();
    let depth = 200_000;
    let mut body = String::with_capacity(depth * 2);
    for _ in 0..depth {
        body.push('[');
    }
    for _ in 0..depth {
        body.push(']');
    }
    mock.push_json(200, &body);

    let error = common::test_client(mock)
        .get::<serde_json::Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("must be rejected");
    assert!(matches!(error, Error::Decode { .. }));
}

#[tokio::test]
async fn integer_overflow_is_a_decode_error() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"id":"ch_1","amount":99999999999999999999999999}"#);
    let error = common::test_client(mock)
        .get::<Charge, _>("/v1/charges", &Fields::new())
        .await
        .expect_err("overflow");
    assert!(matches!(error, Error::Decode { .. }));
}

#[tokio::test]
async fn invalid_utf8_is_a_decode_error() {
    let mock = MockClient::new();
    mock.push_response(200, &[], vec![0xff, 0xfe, 0xfd]);
    let error = common::test_client(mock)
        .get::<serde_json::Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("invalid utf8");
    assert!(matches!(error, Error::Decode { .. }));
}

#[tokio::test]
async fn truncated_json_is_a_decode_error() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"id":"ch_1","amount":"#);
    let error = common::test_client(mock)
        .get::<Charge, _>("/v1/charges", &Fields::new())
        .await
        .expect_err("truncated");
    assert!(matches!(error, Error::Decode { .. }));
}

#[tokio::test]
async fn unknown_fields_are_ignored() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"id":"ch_1","amount":10,"brand_new_field":{"nested":true}}"#);
    let charge = common::test_client(mock)
        .get::<Charge, _>("/v1/charges", &Fields::new())
        .await
        .expect("unknown fields are forward-compatible");
    assert_eq!(charge.id, "ch_1");
    assert_eq!(charge.amount, 10);
}

#[tokio::test]
async fn duplicate_keys_take_the_last_value() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"id":"first","id":"second"}"#);
    let value = common::test_client(mock)
        .get::<serde_json::Value, _>("/v1/charges", &Fields::new())
        .await
        .expect("duplicate keys");
    assert_eq!(value["id"], "second");
}

#[tokio::test]
async fn control_characters_in_parameters_are_encoded_not_injected() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    common::test_client(mock.clone())
        .get::<serde_json::Value, _>("/v1/customers", &Fields::new().field("name", "a\r\nX-Injected: yes"))
        .await
        .expect("get");
    let url = mock.last_request().url;
    assert!(!url.contains('\r'));
    assert!(!url.contains('\n'));
    assert!(url.contains("%0D%0A"));
}

#[tokio::test]
async fn a_multimegabyte_body_is_rejected_quickly() {
    let mock = MockClient::new();
    mock.push_response(200, &[], vec![b'a'; 8 * 1024 * 1024]);
    let client = common::test_client(mock).with_max_response_size(1024);
    let error = client
        .get::<serde_json::Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("oversized");
    assert!(matches!(error, Error::ResponseTooLarge { .. }));
}
