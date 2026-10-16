//! Tests for the generated types and methods, using Stripe's own fixtures.
//!
//! Requires `core` (the default feature set); the v2 tests need `v2_core`.

#![cfg(feature = "core")]

mod common;

use std::collections::HashMap;

use common::MockClient;
use serde_json::json;
#[cfg(feature = "v2_core")]
use stripe::v2_core::{CreateCoreAccountParams, V2CoreAccount, create_core_account};
use stripe::{
    Expandable,
    charges::{Charge, ChargeCustomerExpanded},
    customers::{
        CreateCustomerParams, Customer, CustomerObject, GetCustomerParams, GetCustomerResponseExpanded,
        ListCustomersParams, create_customer, get_customer, list_customers,
    },
    payment_intents::PaymentIntent,
};

const CHARGE_FIXTURE: &str = include_str!("fixtures/charge.json");
const CUSTOMER_FIXTURE: &str = include_str!("fixtures/customer.json");
const PAYMENT_INTENT_FIXTURE: &str = include_str!("fixtures/payment_intent.json");

#[test]
fn parses_official_customer_fixture() {
    let customer: Customer = serde_json::from_str(CUSTOMER_FIXTURE).expect("customer fixture");
    assert_eq!(customer.object.as_str(), "customer");
    assert!(matches!(customer.object, CustomerObject::Customer));
    assert!(!customer.id.is_empty());
    assert!(customer.created > 0);
}

#[test]
fn parses_official_charge_fixture() {
    let charge: Charge = serde_json::from_str(CHARGE_FIXTURE).expect("charge fixture");
    assert_eq!(charge.object.as_str(), "charge");
    assert!(!charge.id.is_empty());
    assert!(charge.amount > 0);
    // `metadata` is required on a charge, so it is not an Option.
    let _ = &charge.metadata;
}

#[test]
fn parses_official_payment_intent_fixture() {
    let intent: PaymentIntent = serde_json::from_str(PAYMENT_INTENT_FIXTURE).expect("intent fixture");
    assert_eq!(intent.object.as_str(), "payment_intent");
    assert!(!intent.id.is_empty());
}

#[test]
fn expandable_field_distinguishes_id_from_object() {
    let mut value: serde_json::Value = serde_json::from_str(CHARGE_FIXTURE).unwrap();

    value["customer"] = json!("cus_123");
    let charge: Charge = serde_json::from_value(value.clone()).unwrap();
    match charge.customer {
        Some(Expandable::Id(id)) => assert_eq!(id, "cus_123"),
        other => panic!("expected an id, got {other:?}"),
    }

    value["customer"] = json!({
        "id": "cus_123",
        "object": "customer",
        "created": 1700000000,
        "livemode": false
    });
    let charge: Charge = serde_json::from_value(value).unwrap();
    match charge.customer {
        Some(Expandable::Object(object)) => match *object {
            ChargeCustomerExpanded::Customer(customer) => assert_eq!(customer.id, "cus_123"),
            other => panic!("expected a customer, got {other:?}"),
        },
        other => panic!("expected an object, got {other:?}"),
    }
}

#[test]
fn unknown_enum_value_is_preserved() {
    let mut value: serde_json::Value = serde_json::from_str(CUSTOMER_FIXTURE).unwrap();
    value["object"] = json!("a_future_object_type");
    let customer: Customer = serde_json::from_value(value).unwrap();
    assert_eq!(customer.object.as_str(), "a_future_object_type");
    assert!(matches!(customer.object, CustomerObject::Unknown(ref value) if value == "a_future_object_type"));
}

#[test]
fn list_type_decodes_a_page() {
    let body = format!(
        r#"{{"object":"list","url":"/v1/customers","has_more":false,"data":[{}]}}"#,
        CUSTOMER_FIXTURE
    );
    let list: stripe::List<Customer> = serde_json::from_str(&body).unwrap();
    assert_eq!(list.data.len(), 1);
    assert!(!list.has_more);
    assert!(!list.has_next_page());
}

#[test]
fn list_type_decodes_a_v2_page() {
    let body = format!(r#"{{"data":[{CUSTOMER_FIXTURE}],"next_page_url":"/v1/customers?page=2"}}"#);
    let list: stripe::List<Customer> = serde_json::from_str(&body).unwrap();
    assert_eq!(list.data.len(), 1);
    assert!(!list.has_more);
    assert!(list.has_next_page());
}

#[tokio::test]
async fn generated_post_encodes_form_and_decodes_response() {
    let mock = MockClient::new();
    mock.push_json(200, CUSTOMER_FIXTURE);
    let client = common::test_client(mock.clone());

    let mut metadata = HashMap::new();
    metadata.insert("order_id".to_string(), "6735".to_string());
    let params = CreateCustomerParams {
        email: Some("jenny@example.com".to_string()),
        metadata: Some(metadata),
        ..Default::default()
    };

    let customer = create_customer(&client, &params).await.expect("create");
    assert!(!customer.id.is_empty());

    let request = mock.last_request();
    assert_eq!(request.method, "POST");
    assert_eq!(request.url, "https://api.stripe.test/v1/customers");
    assert!(request.body_str().contains("email=jenny%40example.com"));
    assert!(request.body_str().contains("metadata[order_id]=6735"));
}

#[tokio::test]
async fn generated_get_encodes_path_and_expand_array() {
    let mock = MockClient::new();
    mock.push_json(200, CUSTOMER_FIXTURE);
    let client = common::test_client(mock.clone());

    let params = GetCustomerParams {
        expand: Some(vec!["default_source".to_string()]),
    };
    let response = get_customer(&client, "cus_123", &params).await.expect("retrieve");
    // The fixture is a customer, so the union resolves to that arm.
    match response {
        GetCustomerResponseExpanded::Customer(customer) => assert!(!customer.id.is_empty()),
        other => panic!("unexpected response: {other:?}"),
    }

    assert_eq!(
        mock.last_request().url,
        "https://api.stripe.test/v1/customers/cus_123?expand[]=default_source"
    );
}

#[tokio::test]
async fn generated_path_arguments_are_percent_encoded() {
    let mock = MockClient::new();
    mock.push_json(200, CUSTOMER_FIXTURE);
    let client = common::test_client(mock.clone());
    let params = GetCustomerParams {
        expand: None,
    };
    let _ = get_customer(&client, "cus_1/2 3", &params).await;
    assert_eq!(mock.last_request().url, "https://api.stripe.test/v1/customers/cus_1%2F2%203");
}

#[tokio::test]
async fn generated_list_returns_a_page() {
    let mock = MockClient::new();
    mock.push_json(
        200,
        &format!(
            r#"{{"object":"list","url":"/v1/customers","has_more":false,"data":[{}]}}"#,
            CUSTOMER_FIXTURE
        ),
    );
    let client = common::test_client(mock.clone());
    let params = ListCustomersParams {
        limit: Some(1),
        ..Default::default()
    };
    let list = list_customers(&client, &params).await.expect("list");
    assert_eq!(list.data.len(), 1);
    assert_eq!(mock.last_request().url, "https://api.stripe.test/v1/customers?limit=1");
}

#[cfg(feature = "v2_core")]
#[tokio::test]
async fn generated_v2_method_sends_json() {
    let mock = MockClient::new();
    mock.push_json(
        200,
        r#"{"id":"acct_1","object":"v2.core.account","created":"2026-01-01T00:00:00Z","livemode":false,"applied_configurations":[]}"#,
    );
    let client = common::test_client(mock.clone());
    let params = CreateCoreAccountParams {
        contact_email: Some("jenny@example.com".to_string()),
        ..Default::default()
    };
    let account: V2CoreAccount = create_core_account(&client, &params).await.expect("create");
    assert_eq!(account.id, "acct_1");

    let request = mock.last_request();
    assert_eq!(request.method, "POST");
    assert_eq!(request.header("Content-Type"), Some("application/json"));
    assert!(request.body_str().contains(r#""contact_email":"jenny@example.com""#));
}

#[cfg(feature = "v2_core")]
#[tokio::test]
async fn v2_list_query_arrays_repeat_the_key() {
    let mock = MockClient::new();
    mock.push_json(200, r#"{"data":[],"next_page_url":null}"#);
    let client = common::test_client(mock.clone());

    let params = stripe::v2_core::ListCoreEventsParams {
        types: Some(vec!["v1.account.created".to_string(), "v1.account.updated".to_string()]),
        ..Default::default()
    };
    let _ = stripe::v2_core::list_core_events(&client, &params).await;

    let url = mock.last_request().url;
    assert!(
        url.contains("types=v1.account.created&types=v1.account.updated"),
        "v2 arrays repeat the key, got {url}"
    );
    assert!(!url.contains("types[]="), "v2 arrays must not use bracket notation, got {url}");
}

#[tokio::test]
async fn multipart_endpoints_fail_before_sending() {
    let mock = MockClient::new();
    let client = common::test_client(mock.clone());
    let params = stripe::files::CreateFileParams {
        file: "file content".to_string(),
        purpose: stripe::files::CreateFileParamsPurpose::BusinessIcon,
        file_link_data: None,
        expand: None,
    };
    let error = stripe::files::create_file(&client, &params)
        .await
        .expect_err("unsupported");
    assert!(
        matches!(error, stripe::Error::UnsupportedEncoding { encoding, .. } if encoding == "multipart/form-data"),
        "unexpected error: {error:?}"
    );
    assert!(mock.requests().is_empty(), "nothing must be sent");
}
