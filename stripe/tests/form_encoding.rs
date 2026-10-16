//! Request parameter encoding tests.

use stripe::params::{FormBuilder, FormEncode, encode, percent_encode_into};

struct Item {
    price: String,
    quantity: i64,
    tags: Vec<String>,
}

impl FormEncode for Item {
    fn encode_form(&self, f: &mut FormBuilder) {
        f.field("price", &self.price);
        f.field("quantity", &self.quantity);
        f.field_array_str("tags", &self.tags);
    }
}

struct CreateParams {
    email: Option<String>,
    active: Option<bool>,
    amount: Option<i64>,
    metadata: Vec<(String, String)>,
    items: Vec<Item>,
    expand: Vec<String>,
    description: Option<String>,
}

impl FormEncode for CreateParams {
    fn encode_form(&self, f: &mut FormBuilder) {
        if let Some(value) = &self.email {
            f.field("email", value);
        }
        if let Some(value) = &self.active {
            f.field("active", value);
        }
        if let Some(value) = &self.amount {
            f.field("amount", value);
        }
        for (key, value) in &self.metadata {
            f.field_map("metadata", key, value);
        }
        for (index, item) in self.items.iter().enumerate() {
            f.field_indexed("items", index, item);
        }
        f.field_array_str("expand", &self.expand);
        if let Some(value) = &self.description {
            f.field("description", value);
        }
    }
}

fn sample() -> CreateParams {
    CreateParams {
        email: Some("jenny@example.com".to_string()),
        active: Some(true),
        amount: Some(1000),
        metadata: vec![("order_id".to_string(), "6735".to_string())],
        items: vec![Item {
            price: "price_123".to_string(),
            quantity: 2,
            tags: vec!["a".to_string(), "b".to_string()],
        }],
        expand: vec!["customer".to_string()],
        description: Some(String::new()),
    }
}

#[test]
fn encodes_bracket_notation() {
    assert_eq!(
        encode(&sample()),
        "email=jenny%40example.com&active=true&amount=1000\
         &metadata[order_id]=6735\
         &items[0][price]=price_123&items[0][quantity]=2\
         &items[0][tags][]=a&items[0][tags][]=b\
         &expand[]=customer\
         &description="
    );
}

#[test]
fn skips_absent_fields() {
    let params = CreateParams {
        email: None,
        active: None,
        amount: None,
        metadata: vec![],
        items: vec![],
        expand: vec![],
        description: None,
    };
    assert_eq!(encode(&params), "");
}

#[test]
fn percent_encodes_reserved_characters_in_values() {
    let mut out = String::new();
    percent_encode_into(&mut out, "a b&c=d+e%f[g]h\n\0");
    assert_eq!(out, "a%20b%26c%3Dd%2Be%25f%5Bg%5Dh%0A%00");
}

#[test]
fn percent_encodes_utf8_as_bytes() {
    let mut out = String::new();
    percent_encode_into(&mut out, "héllo");
    assert_eq!(out, "h%C3%A9llo");
}

#[test]
fn leaves_unreserved_characters_alone() {
    let mut out = String::new();
    percent_encode_into(&mut out, "Aa0-._~");
    assert_eq!(out, "Aa0-._~");
}

#[test]
fn metadata_keys_cannot_inject_parameters() {
    // A hostile metadata key tries to close the bracket and inject a new key.
    let params = CreateParams {
        email: None,
        active: None,
        amount: None,
        metadata: vec![("evil]=x&currency=usd&metadata[".to_string(), "v".to_string())],
        items: vec![],
        expand: vec![],
        description: None,
    };
    let encoded = encode(&params);
    assert_eq!(encoded, "metadata[evil%5D%3Dx%26currency%3Dusd%26metadata%5B]=v");
    // The only parameter is `metadata[...]`; no bare `currency` was created.
    assert!(!encoded.contains("&currency="));
}

#[test]
fn empty_array_emits_nothing() {
    let mut builder = FormBuilder::new();
    builder.field_array_str("expand", &[]);
    assert!(builder.is_empty());
}

#[test]
fn nested_prefix_is_balanced_after_pop() {
    let mut builder = FormBuilder::new();
    builder.field("outer", &"one");
    builder.push_segment("nested");
    builder.field("inner", &"two");
    builder.pop();
    builder.field("after", &"three");
    assert_eq!(builder.as_str(), "outer=one&nested[inner]=two&after=three");
}

#[test]
fn explicit_empty_field_round_trips() {
    let mut builder = FormBuilder::new();
    builder.field_empty("description");
    assert_eq!(builder.as_str(), "description=");
}

#[test]
fn golden_vectors_match_stripe_ruby() {
    // The same inputs are asserted in stripe-ruby's `UtilTest`, so the wire
    // format is pinned to an independent implementation.
    let cases: &[(&str, &[(&str, &str)])] = &[
        // An empty value is sent as `name=`: that is how Stripe is asked to
        // clear a field.
        ("a=", &[("a", "")]),
        ("b=%2Bfoo%3F", &[("b", "+foo?")]),
        ("c=bar%26baz", &[("c", "bar&baz")]),
        ("d=%5Bfoo%5D", &[("d", "[foo]")]),
        ("e=foo%20bar", &[("e", "foo bar")]),
        ("f=%25", &[("f", "%")]),
        ("g=%C3%A9", &[("g", "é")]),
    ];
    for (expected, fields) in cases {
        let mut builder = FormBuilder::new();
        for (name, value) in *fields {
            builder.field(name, &value.to_string());
        }
        assert_eq!(builder.as_str(), *expected, "encoding {fields:?}");
    }
}

#[test]
fn repeated_arrays_use_the_v2_style() {
    // v2 query parameters repeat the key instead of using `[]`.
    let mut builder = FormBuilder::new();
    builder.field_repeated("types", &["a".to_string(), "b c".to_string()]);
    assert_eq!(builder.as_str(), "types=a&types=b%20c");
}

#[test]
fn clearing_a_value_is_an_empty_assignment() {
    // Stripe clears a field when it receives an empty value; `Some("")` must
    // therefore produce `name=` and not be dropped.
    let params = CreateParams {
        email: None,
        active: None,
        amount: None,
        metadata: vec![],
        items: vec![],
        expand: vec![],
        description: Some(String::new()),
    };
    assert_eq!(encode(&params), "description=");
}

#[test]
fn a_key_that_looks_like_a_nested_field_cannot_escape() {
    // Every bracket in a metadata key is encoded, so the key cannot address a
    // sibling parameter.
    let mut builder = FormBuilder::new();
    builder.field_map("metadata", "a[0]&b=c", "v");
    assert_eq!(builder.as_str(), "metadata[a%5B0%5D%26b%3Dc]=v");
}
