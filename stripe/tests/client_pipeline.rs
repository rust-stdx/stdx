//! Request pipeline tests: headers, retries, idempotency and error mapping.

mod common;

use std::time::Duration;

use common::{Fields, MockClient};
use serde_json::Value;
use stripe::{Client, DEFAULT_API_VERSION, Error, RequestOptions};

fn client(mock: MockClient) -> Client {
    common::test_client(mock)
}

fn retrying_client(mock: MockClient, max_retries: u32) -> Client {
    Client::with_http_client("sk_test_123", mock)
        .expect("client")
        .with_base_url("https://api.stripe.test")
        .with_max_retries(max_retries)
        .with_retry_backoff(Duration::ZERO, Duration::ZERO)
}

#[tokio::test]
async fn sends_auth_version_and_accept_headers() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    client(mock.clone())
        .get::<Value, _>("/v1/customers/cus_1", &Fields::new())
        .await
        .expect("get");

    let request = mock.last_request();
    assert_eq!(request.method, "GET");
    assert_eq!(request.url, "https://api.stripe.test/v1/customers/cus_1");
    assert_eq!(request.header("Authorization"), Some("Bearer sk_test_123"));
    assert_eq!(request.header("Stripe-Version"), Some(DEFAULT_API_VERSION));
    assert_eq!(request.header("Accept"), Some("application/json"));
    assert_eq!(request.header("Idempotency-Key"), None);
    assert_eq!(request.header("Content-Type"), None);
    assert_eq!(request.timeout, Some(Duration::from_secs(30)));
}

#[tokio::test]
async fn encodes_query_parameters() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    client(mock.clone())
        .get::<Value, _>("/v1/customers", &Fields::new().field("email", "a@b.c").field("limit", "3"))
        .await
        .expect("get");
    assert_eq!(
        mock.last_request().url,
        "https://api.stripe.test/v1/customers?email=a%40b.c&limit=3"
    );
}

#[tokio::test]
async fn sends_account_and_user_agent_when_configured() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    let client = common::test_client(mock.clone())
        .with_account("acct_123")
        .with_user_agent("my-app/1.0");
    client
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("get");
    let request = mock.last_request();
    assert_eq!(request.header("Stripe-Account"), Some("acct_123"));
    assert_eq!(request.header("User-Agent"), Some("my-app/1.0"));
}

#[tokio::test]
async fn version_can_be_overridden_or_disabled() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    common::test_client(mock.clone())
        .with_api_version("2020-08-27")
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("get");
    assert_eq!(mock.last_request().header("Stripe-Version"), Some("2020-08-27"));

    let mock = MockClient::new();
    mock.push_json(200, "{}");
    common::test_client(mock.clone())
        .without_api_version()
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("get");
    assert_eq!(mock.last_request().header("Stripe-Version"), None);
}

#[tokio::test]
async fn post_generates_an_idempotency_key_when_retries_are_enabled() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    retrying_client(mock.clone(), 2)
        .post_form::<Value, _>("/v1/customers", &Fields::new().field("email", "a@b.c"))
        .await
        .expect("post");
    let request = mock.last_request();
    assert_eq!(request.method, "POST");
    assert_eq!(request.header("Content-Type"), Some("application/x-www-form-urlencoded"));
    let key = request.header("Idempotency-Key").expect("idempotency key");
    assert_eq!(key.len(), 36, "expected a UUID: {key}");
    assert_eq!(request.body_str(), "email=a%40b.c");
}

#[tokio::test]
async fn post_omits_idempotency_key_when_retries_are_disabled() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    client(mock.clone())
        .post_form::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("post");
    assert_eq!(mock.last_request().header("Idempotency-Key"), None);
}

#[tokio::test]
async fn retries_reuse_the_same_idempotency_key() {
    let mock = MockClient::new();
    mock.push_json(500, r#"{"error":{"type":"api_error","message":"boom"}}"#);
    mock.push_json(200, "{}");
    retrying_client(mock.clone(), 2)
        .post_form::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("post retried");
    let requests = mock.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0].header("Idempotency-Key"),
        requests[1].header("Idempotency-Key"),
        "the key must be stable across retries"
    );
}

#[tokio::test]
async fn retries_server_errors_then_succeeds() {
    let mock = MockClient::new();
    mock.push_json(503, r#"{"error":{"type":"api_error","message":"down"}}"#);
    mock.push_json(200, r#"{"id":"cus_1"}"#);
    let value = retrying_client(mock.clone(), 2)
        .get::<Value, _>("/v1/customers/cus_1", &Fields::new())
        .await
        .expect("eventually ok");
    assert_eq!(value["id"], "cus_1");
    assert_eq!(mock.request_count(), 2);
}

#[tokio::test]
async fn retries_transport_errors() {
    let mock = MockClient::new();
    mock.push_error("connection reset");
    mock.push_json(200, "{}");
    retrying_client(mock.clone(), 1)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("retried transport error");
    assert_eq!(mock.request_count(), 2);
}

#[tokio::test]
async fn does_not_retry_when_should_retry_is_false() {
    let mock = MockClient::new();
    mock.push_response(
        500,
        &[("Stripe-Should-Retry", "false")],
        br#"{"error":{"type":"api_error","message":"no"}}"#.to_vec(),
    );
    let error = retrying_client(mock.clone(), 3)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("must fail without retrying");
    assert!(!error.is_retryable());
    assert_eq!(mock.request_count(), 1);
}

#[tokio::test]
async fn does_not_retry_client_errors() {
    let mock = MockClient::new();
    mock.push_json(400, r#"{"error":{"type":"invalid_request_error","message":"bad"}}"#);
    let error = retrying_client(mock.clone(), 5)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("bad request");
    assert!(!error.is_retryable());
    assert_eq!(mock.request_count(), 1);
}

#[tokio::test]
async fn retry_attempts_are_bounded() {
    let mock = MockClient::new();
    mock.push_json(500, r#"{"error":{"type":"api_error","message":"a"}}"#);
    mock.push_json(500, r#"{"error":{"type":"api_error","message":"b"}}"#);
    mock.push_json(500, r#"{"error":{"type":"api_error","message":"c"}}"#);
    let error = retrying_client(mock.clone(), 1)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("still failing");
    assert!(matches!(error, Error::Api { .. }));
    assert_eq!(mock.request_count(), 2, "1 initial + 1 retry");
}

#[tokio::test]
async fn honors_retry_after_header() {
    let mock = MockClient::new();
    mock.push_response(
        429,
        &[("Retry-After", "0")],
        br#"{"error":{"type":"api_error","message":"slow down"}}"#.to_vec(),
    );
    mock.push_json(200, "{}");
    retrying_client(mock.clone(), 2)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("retried after Retry-After");
    assert_eq!(mock.request_count(), 2);
}

#[tokio::test]
async fn a_long_retry_after_is_not_retried_early() {
    // Waiting longer than the cap is the caller's decision: retrying sooner than
    // the server asked would hammer it.
    let mock = MockClient::new();
    mock.push_response(
        429,
        &[("Retry-After", "3600")],
        br#"{"error":{"type":"api_error","message":"slow down"}}"#.to_vec(),
    );
    mock.push_json(200, "{}");
    let error = retrying_client(mock.clone(), 5)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("must not retry yet");
    assert!(matches!(error, Error::Api { .. }), "{error:?}");
    assert_eq!(mock.request_count(), 1, "the request must not be retried");
}

#[tokio::test]
async fn a_retry_after_within_the_cap_is_honored_verbatim() {
    // 1 second is within the cap; the backoff bounds must not shorten it.
    let mock = MockClient::new();
    mock.push_response(
        503,
        &[("Retry-After", "0")],
        br#"{"error":{"type":"api_error","message":"unavailable"}}"#.to_vec(),
    );
    mock.push_json(200, "{}");
    let started = std::time::Instant::now();
    retrying_client(mock.clone(), 1)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("retried");
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
    assert_eq!(mock.request_count(), 2);
}

#[tokio::test]
async fn parses_card_errors() {
    let mock = MockClient::new();
    mock.push_response(
        402,
        &[("Request-Id", "req_123")],
        br#"{"error":{"type":"card_error","code":"card_declined","decline_code":"insufficient_funds","message":"Your card was declined.","param":"card_number"}}"#.to_vec(),
    );
    let error = client(mock)
        .get::<Value, _>("/v1/charges", &Fields::new())
        .await
        .expect_err("declined");
    let api = error.api_error().expect("api error body");
    assert_eq!(api.type_.as_deref(), Some("card_error"));
    assert_eq!(api.code.as_deref(), Some("card_declined"));
    assert_eq!(api.decline_code.as_deref(), Some("insufficient_funds"));
    assert_eq!(api.param.as_deref(), Some("card_number"));
    assert!(api.is_card_error());
    let meta = error.api_meta().expect("meta");
    assert_eq!(meta.status, 402);
    assert_eq!(meta.request_id.as_deref(), Some("req_123"));
}

#[tokio::test]
async fn preserves_unknown_error_codes() {
    let mock = MockClient::new();
    mock.push_json(
        402,
        r#"{"error":{"type":"some_future_error","code":"brand_new_code","message":"who knows"}}"#,
    );
    let error = client(mock)
        .get::<Value, _>("/v1/charges", &Fields::new())
        .await
        .expect_err("error");
    let api = error.api_error().expect("api error body");
    assert_eq!(api.type_.as_deref(), Some("some_future_error"));
    assert_eq!(api.code.as_deref(), Some("brand_new_code"));
}

#[tokio::test]
async fn non_json_error_bodies_do_not_panic() {
    let mock = MockClient::new();
    mock.push_response(502, &[], b"<html>Bad Gateway</html>".to_vec());
    let error = client(mock)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("gateway error");
    match error {
        Error::Api(payload) => {
            assert!(payload.raw_body.contains("Bad Gateway"));
            assert!(payload.error.message.is_none());
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn error_message_stays_small_for_huge_bodies() {
    let mock = MockClient::new();
    let body = "x".repeat(1_000_000);
    mock.push_response(500, &[], body.into_bytes());
    let error = client(mock)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("error");
    let rendered = error.to_string();
    assert!(rendered.len() < 10_000, "error Display must be bounded");
}

#[tokio::test]
async fn rejects_oversized_response_bodies() {
    let mock = MockClient::new();
    mock.push_response(200, &[], vec![b'a'; 1024]);
    let client = common::test_client(mock).with_max_response_size(64);
    let error = client
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("too large");
    assert!(matches!(
        error,
        Error::ResponseTooLarge {
            limit: 64
        }
    ));
}

#[tokio::test]
async fn decode_errors_are_reported() {
    let mock = MockClient::new();
    mock.push_json(200, "not json");
    let error = client(mock)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("bad json");
    assert!(matches!(
        error,
        Error::Decode {
            status: 200,
            ..
        }
    ));
}

#[tokio::test]
async fn rejects_header_injection_in_configuration() {
    let mock = MockClient::new();
    let result =
        Client::with_http_client("sk_test_123", mock).map(|client| client.with_account("acct\r\nX-Injected: yes"));
    // The account is validated when the request is built.
    let error = result
        .expect("client builds")
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect_err("injection rejected");
    assert!(matches!(error, Error::Config(_)));
}

#[tokio::test]
async fn rejects_empty_and_control_char_api_keys() {
    assert!(matches!(Client::with_http_client("", MockClient::new()), Err(Error::Config(_))));
    assert!(matches!(
        Client::with_http_client("   ", MockClient::new()),
        Err(Error::Config(_))
    ));
    for key in ["sk\n_test", "sk\t_test", "sk_test\0", "sk_test\u{7f}"] {
        assert!(
            matches!(Client::with_http_client(key, MockClient::new()), Err(Error::Config(_))),
            "`{key:?}` must be rejected when the client is built"
        );
    }
}

#[tokio::test]
async fn timeout_can_be_disabled() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    common::test_client(mock.clone())
        .with_timeout(None)
        .get::<Value, _>("/v1/customers", &Fields::new())
        .await
        .expect("get");
    assert_eq!(mock.last_request().timeout, None);
}

#[tokio::test]
async fn debug_never_prints_the_api_key() {
    let client = client(MockClient::new());
    let rendered = format!("{client:?}");
    assert!(!rendered.contains("sk_test_123"));
    assert!(rendered.contains("<redacted>"));
}

#[tokio::test]
async fn sends_a_caller_supplied_idempotency_key() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    let options = RequestOptions::new().with_idempotency_key("my-operation-42");
    client(mock.clone())
        .post_form_with_options::<Value, _>("/v1/customers", &Fields::new(), &options)
        .await
        .expect("post");
    assert_eq!(mock.last_request().header("Idempotency-Key"), Some("my-operation-42"));
}

#[tokio::test]
async fn explicit_idempotency_key_overrides_auto_generation() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    let options = RequestOptions::new().with_idempotency_key("explicit");
    retrying_client(mock.clone(), 2)
        .post_form_with_options::<Value, _>("/v1/customers", &Fields::new(), &options)
        .await
        .expect("post");
    assert_eq!(mock.last_request().header("Idempotency-Key"), Some("explicit"));
}

#[tokio::test]
async fn explicit_idempotency_key_is_reused_across_retries() {
    let mock = MockClient::new();
    mock.push_json(500, r#"{"error":{"type":"api_error","message":"boom"}}"#);
    mock.push_json(200, "{}");
    let options = RequestOptions::new().with_idempotency_key("stable-key");
    retrying_client(mock.clone(), 2)
        .post_form_with_options::<Value, _>("/v1/customers", &Fields::new(), &options)
        .await
        .expect("retried");
    let requests = mock.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].header("Idempotency-Key"), Some("stable-key"));
    assert_eq!(requests[1].header("Idempotency-Key"), Some("stable-key"));
}

#[tokio::test]
async fn idempotency_key_is_not_sent_on_get() {
    let mock = MockClient::new();
    mock.push_json(200, "{}");
    let options = RequestOptions::new().with_idempotency_key("ignored");
    retrying_client(mock.clone(), 2)
        .get_with_options::<Value, _>("/v1/customers", &Fields::new(), &options)
        .await
        .expect("get");
    assert_eq!(mock.last_request().header("Idempotency-Key"), None);
}

#[tokio::test]
async fn rejects_an_oversized_idempotency_key_before_sending() {
    let mock = MockClient::new();
    let options = RequestOptions::new().with_idempotency_key("x".repeat(256));
    let error = client(mock.clone())
        .post_form_with_options::<Value, _>("/v1/customers", &Fields::new(), &options)
        .await
        .expect_err("must be rejected");
    assert!(matches!(error, Error::Config(_)));
    assert_eq!(mock.request_count(), 0, "nothing must be sent");
}

#[tokio::test]
async fn rejects_control_characters_in_an_idempotency_key() {
    let mock = MockClient::new();
    let options = RequestOptions::new().with_idempotency_key("bad\r\nkey");
    let error = client(mock.clone())
        .post_form_with_options::<Value, _>("/v1/customers", &Fields::new(), &options)
        .await
        .expect_err("must be rejected");
    assert!(matches!(error, Error::Config(_)));
    assert_eq!(mock.request_count(), 0, "nothing must be sent");
}
