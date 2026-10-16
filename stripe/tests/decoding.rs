//! Response decoding must never lose a whole payload because one field is
//! missing, `null`, or shaped like a deleted object.
//!
//! Stripe's specification is regularised but not a guarantee: it marks fields as
//! required that deleted objects omit, and Stripe adds fields between API
//! versions. These tests pin the resulting contract: a *missing* field decodes to
//! its default, `null` is accepted where the specification allows it (nullable
//! fields, enums), a *wrongly typed* value is an error, and a list element that
//! cannot be decoded is an error rather than a silently dropped item.

#![cfg(feature = "core")]

use serde_json::{Value, json};
use stripe::{
    charges::Charge,
    coupons::Coupon,
    customers::{Customer, CustomerObject},
    invoiceitems::Invoiceitem,
    invoices::Invoice,
    plans::Plan,
    prices::Price,
    products::Product,
    subscription_items::SubscriptionItem,
    tax_ids::TaxId,
};

const CUSTOMER_FIXTURE: &str = include_str!("fixtures/customer.json");
const CHARGE_FIXTURE: &str = include_str!("fixtures/charge.json");

/// Decodes `value` (a JSON object) as `T`, returning the error message.
fn decode<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T, String> {
    serde_json::from_value(value.clone()).map_err(|error| error.to_string())
}

/// Deleted objects are the minimal shape `{id, object, deleted}`.
///
/// They appear when an `expand`ed field (or a webhook payload) refers to a
/// resource that was deleted, and must decode as the live type instead of
/// failing the whole response.
#[test]
fn deleted_objects_decode_as_their_live_type() {
    assert!(decode::<Customer>(&json!({"id": "cus_1", "object": "customer", "deleted": true})).is_ok());
    assert!(decode::<Coupon>(&json!({"id": "c_1", "object": "coupon", "deleted": true})).is_ok());
    assert!(decode::<Invoice>(&json!({"id": "in_1", "object": "invoice", "deleted": true})).is_ok());
    assert!(decode::<Invoiceitem>(&json!({"id": "ii_1", "object": "invoiceitem", "deleted": true})).is_ok());
    assert!(decode::<Plan>(&json!({"id": "plan_1", "object": "plan", "deleted": true})).is_ok());
    assert!(decode::<Price>(&json!({"id": "price_1", "object": "price", "deleted": true})).is_ok());
    assert!(decode::<Product>(&json!({"id": "prod_1", "object": "product", "deleted": true})).is_ok());
    assert!(decode::<SubscriptionItem>(&json!({"id": "si_1", "object": "subscription_item", "deleted": true})).is_ok());
    assert!(decode::<TaxId>(&json!({"id": "txi_1", "object": "tax_id", "deleted": true})).is_ok());
}

#[test]
fn a_deleted_object_decodes_inside_an_expandable_field() {
    let mut value: Value = serde_json::from_str(CHARGE_FIXTURE).expect("fixture");
    value["customer"] = json!({"id": "cus_1", "object": "customer", "deleted": true});

    let charge: Charge = decode(&value).expect("a deleted customer must not fail the charge");
    let expanded = charge.customer.expect("customer").into_object().expect("expanded");
    let stripe::charges::ChargeCustomerExpanded::Customer(customer) = expanded else {
        panic!("expected the customer arm");
    };
    assert_eq!(customer.id, "cus_1", "the id survives even though the rest is absent");
}

#[test]
fn removing_any_field_keeps_the_response_decodable() {
    let fixture: Value = serde_json::from_str(CUSTOMER_FIXTURE).expect("fixture");
    let object = fixture.as_object().expect("object");
    assert!(object.len() > 10, "the fixture should be a realistic customer");

    for key in object.keys() {
        let mut value = fixture.clone();
        value.as_object_mut().expect("object").remove(key);
        let decoded = decode::<Customer>(&value);
        assert!(decoded.is_ok(), "removing `{key}` failed the whole response: {decoded:?}");
    }
}

#[test]
fn nulling_a_nullable_field_keeps_the_response_decodable() {
    // Stripe sends `null` for the fields its specification marks nullable, which
    // are `Option` here.
    let fixture: Value = serde_json::from_str(CUSTOMER_FIXTURE).expect("fixture");
    for key in [
        "address",
        "balance",
        "description",
        "email",
        "metadata",
        "name",
        "phone",
        "preferred_locales",
        "shipping",
    ] {
        let mut value = fixture.clone();
        value[key] = Value::Null;
        let decoded = decode::<Customer>(&value);
        assert!(decoded.is_ok(), "nulling nullable `{key}` failed the response: {decoded:?}");
    }
}

#[test]
fn a_null_where_a_value_is_required_is_an_error() {
    // A `null` in a field that is not nullable is a broken payload; reporting it
    // beats inventing a value.
    for key in ["created", "id", "livemode"] {
        let mut value: Value = serde_json::from_str(CUSTOMER_FIXTURE).expect("fixture");
        value[key] = Value::Null;
        assert!(decode::<Customer>(&value).is_err(), "`{key}` is not nullable");
    }
}

#[test]
fn nulling_any_field_never_panics() {
    let fixture: Value = serde_json::from_str(CUSTOMER_FIXTURE).expect("fixture");
    let object = fixture.as_object().expect("object");
    for key in object.keys() {
        let mut value = fixture.clone();
        value[key] = Value::Null;
        let _ = decode::<Customer>(&value);
    }
}

#[test]
fn a_null_enum_is_an_unknown_value() {
    // Enums tolerate `null` themselves, so this holds inside containers too,
    // where there is no field-level default to fall back to.
    let mut value: Value = serde_json::from_str(CUSTOMER_FIXTURE).expect("fixture");
    value["object"] = Value::Null;
    let customer: Customer = decode(&value).expect("a null discriminator decodes");
    assert!(matches!(customer.object, CustomerObject::Unknown(ref value) if value.is_empty()));

    let list = json!({"object": "list", "has_more": false, "data": [value]});
    let decoded: stripe::List<Customer> = decode(&list).expect("a null enum decodes inside a list");
    assert!(matches!(decoded.data[0].object, CustomerObject::Unknown(_)));
}

#[test]
fn a_duplicate_key_is_an_error_for_a_generated_type() {
    // Stripe does not send repeated keys, and a payload that did would silently
    // lose a value if the last one won. `serde_json::Value` takes the last
    // value, so this pins the behaviour of the generated types specifically.
    let body = r#"{"id":"first","id":"second","object":"customer"}"#;
    let error = serde_json::from_str::<Customer>(body).expect_err("a repeated field is reported");
    assert!(error.to_string().contains("duplicate"), "unexpected error: {error}");
    let value: Value = serde_json::from_str(body).expect("value");
    assert_eq!(value["id"], "second", "`Value` keeps the last value instead");
}

#[test]
fn a_wrongly_typed_field_is_an_error() {
    let mut value: Value = serde_json::from_str(CUSTOMER_FIXTURE).expect("fixture");
    value["created"] = json!("not a number");
    assert!(decode::<Customer>(&value).is_err(), "a string is not a timestamp");

    let mut value: Value = serde_json::from_str(CHARGE_FIXTURE).expect("fixture");
    value["amount"] = json!([1, 2]);
    assert!(decode::<Charge>(&value).is_err(), "an array is not an amount");
}

#[test]
fn a_field_stripped_of_its_type_is_still_reported() {
    // `object` is the discriminator: an unexpected value is preserved, not fatal.
    let mut value: Value = serde_json::from_str(CUSTOMER_FIXTURE).expect("fixture");
    value["object"] = json!("something_new");
    let customer: Customer = decode(&value).expect("unknown discriminators are preserved");
    assert!(matches!(customer.object, CustomerObject::Unknown(ref value) if value == "something_new"));
}

#[test]
fn a_list_item_that_cannot_decode_is_an_error_not_a_dropped_item() {
    let body = json!({
        "object": "list",
        "has_more": false,
        "url": "/v1/customers",
        "data": [
            serde_json::from_str::<Value>(CUSTOMER_FIXTURE).expect("fixture"),
            {"id": "cus_broken", "object": "customer", "created": "not a number"}
        ]
    });
    // The element that does not decode fails the page. Deserializers that skip
    // bad elements (miniserde, for example) would silently return a one-item
    // list, which reconciliation logic would read as "the object is gone".
    let error = decode::<stripe::List<Customer>>(&body).expect_err("the bad item must be reported");
    assert!(
        error.contains("invalid type") || error.contains("expected"),
        "the error should describe the mismatch: {error}"
    );
}

#[test]
fn hostile_json_is_rejected_without_panicking() {
    for body in [
        json!(null),
        json!([]),
        json!({"id": 1}),
        json!({"id": "cus_1", "object": {"nested": true}}),
        json!({"id": "cus_1", "object": "customer", "metadata": "not a map"}),
        json!({"id": "cus_1", "object": "customer", "metadata": {"k": 5}}),
    ] {
        let _ = decode::<Customer>(&body);
    }
}

#[test]
fn responses_with_huge_values_are_bounded_by_the_type() {
    // An integer that does not fit in `i64` is a decoding error, not a wrap.
    let body = json!({"id": "cus_1", "object": "customer", "created": u64::MAX});
    assert!(decode::<Customer>(&body).is_err());
}
