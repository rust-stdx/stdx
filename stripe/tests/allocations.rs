//! Allocation budgets for the hot paths.
//!
//! The counters are measured, not guessed: run the tests to see the current
//! budget. Keep them tight so a regression (an extra copy of the request body,
//! a per-field key allocation, ...) fails CI.

use alloc_counter::measure;
use serde::Deserialize;
use stripe::params::{FormBuilder, FormEncode, encode, percent_encode_into};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Charge {
    id: String,
    object: String,
    amount: i64,
    currency: String,
    customer: Option<String>,
    description: Option<String>,
    paid: bool,
    refunded: bool,
    status: String,
}

struct CreateParams {
    email: Option<String>,
    amount: Option<i64>,
    metadata: Vec<(String, String)>,
    expand: Vec<String>,
}

impl FormEncode for CreateParams {
    fn encode_form(&self, form: &mut FormBuilder) {
        if let Some(value) = &self.email {
            form.field("email", value);
        }
        if let Some(value) = &self.amount {
            form.field("amount", value);
        }
        for (key, value) in &self.metadata {
            form.field_map("metadata", key, value);
        }
        form.field_array_str("expand", &self.expand);
    }
}

fn sample_params() -> CreateParams {
    CreateParams {
        email: Some("jenny@example.com".to_string()),
        amount: Some(2000),
        metadata: vec![("order_id".to_string(), "6735".to_string())],
        expand: vec!["customer".to_string()],
    }
}

const CHARGE_JSON: &[u8] = br#"{"id":"ch_3P0m4k2eZvKYlo2C1","object":"charge","amount":2000,"currency":"usd","customer":"cus_123","description":null,"paid":true,"refunded":false,"status":"succeeded","extra_unknown_field":42}"#;

#[test]
fn percent_encoding_does_not_allocate_into_the_buffer() {
    // The buffer is pre-sized, so encoding a small string must not allocate.
    let mut out = String::with_capacity(64);
    let (_, counters) = measure(|| percent_encode_into(&mut out, "jenny@example.com"));
    assert_eq!(counters.calls, 0, "counters: {counters:?}");
    assert_eq!(out, "jenny%40example.com");
}

#[test]
fn form_encoding_budget() {
    let params = sample_params();
    let _ = encode(&params); // warm up any lazy state

    let (encoded, counters) = measure(|| encode(&params));
    assert_eq!(
        encoded,
        "email=jenny%40example.com&amount=2000&metadata[order_id]=6735&expand[]=customer"
    );
    // Measured: exactly three allocations, the three reusable buffers
    // (output, key prefix, prefix stack). The encoder performs no per-field
    // allocation. A regression here means a per-field copy crept back in.
    assert!(
        counters.calls <= 3,
        "form encoding allocated {} times: {counters:?}",
        counters.calls
    );
}

#[test]
fn reused_builder_amortizes_allocations() {
    let params = sample_params();

    // Encoding into a fresh builder twice: the second time the capacity is
    // already available.
    let (_, first) = measure(|| {
        let mut builder = FormBuilder::with_capacity(256);
        params.encode_form(&mut builder);
        builder.into_string()
    });
    // Measured: three (output, prefix, stack).
    assert!(first.calls <= 3, "first: {first:?}");
}

#[test]
fn json_decoding_budget() {
    // Warm up.
    let _ = serde_json::from_slice::<Charge>(CHARGE_JSON).unwrap();

    let (charge, counters) = measure(|| serde_json::from_slice::<Charge>(CHARGE_JSON).unwrap());
    assert_eq!(charge.amount, 2000);
    assert_eq!(charge.id, "ch_3P0m4k2eZvKYlo2C1");
    // Measured: five allocations, one per owned string field. serde_json does
    // not copy the raw body.
    assert!(
        counters.calls <= 5,
        "json decoding allocated {} times: {counters:?}",
        counters.calls
    );
}
