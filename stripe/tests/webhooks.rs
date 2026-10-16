//! Webhook signature verification tests.
//!
//! The HMAC values are pre-computed by an independent implementation (Python's
//! `hmac` + `hashlib`), so a bug in the crate's own HMAC use cannot make these
//! tests pass.

use crypto::{hmac::Hmac, sha2::Sha256};
use serde_json::Value;
use stripe::{Webhook, WebhookError, verify_signature};

fn sign(timestamp: i64, payload: &[u8], secret: &str) -> String {
    let mut mac = Hmac::<Sha256>::new(secret.as_bytes());
    mac.update(timestamp.to_string().as_bytes());
    mac.update(b".");
    mac.update(payload);
    let tag = mac.finalize();
    tag.as_ref().iter().map(|byte| format!("{byte:02x}")).collect()
}

const PAYLOAD: &[u8] = br#"{"id": "evt_test_webhook", "object": "event"}"#;
const SECRET: &str = "whsec_test_secret";
const TS: i64 = 1_234_567_890;
const EXPECTED_HEX: &str = "be00ab9ab4669e1720996c2187970a6b49ae2d4d10c0a3630c30f36e2669f2ed";

#[test]
fn signature_matches_independent_vector() {
    assert_eq!(sign(TS, PAYLOAD, SECRET), EXPECTED_HEX);
}

#[test]
fn verifies_valid_signature_with_zero_tolerance() {
    let header = format!("t={TS},v1={EXPECTED_HEX}");
    Webhook::verify_header(PAYLOAD, &header, SECRET, 0).expect("valid signature");
    assert!(
        verify_signature(PAYLOAD, &header, SECRET).is_err(),
        "recent timestamp would be outside the default window for the fixed test clock"
    );
}

#[test]
fn rejects_tampered_payload() {
    let header = format!("t={TS},v1={EXPECTED_HEX}");
    let tampered = br#"{"id": "evt_test_webhook", "object": "event", "extra": 1}"#;
    assert_eq!(
        Webhook::verify_header(tampered, &header, SECRET, 0),
        Err(WebhookError::SignatureMismatch)
    );
}

#[test]
fn rejects_wrong_secret() {
    let header = format!("t={TS},v1={EXPECTED_HEX}");
    assert_eq!(
        Webhook::verify_header(PAYLOAD, &header, "whsec_other", 0),
        Err(WebhookError::SignatureMismatch)
    );
}

#[test]
fn accepts_when_one_of_several_v1_signatures_matches() {
    // Simulates secret rotation: an old, wrong signature plus the right one.
    let header = format!("t={TS},v1=deadbeef,v1={EXPECTED_HEX}");
    Webhook::verify_header(PAYLOAD, &header, SECRET, 0).expect("one signature matches");
}

#[test]
fn ignores_v0_scheme() {
    // A v0-only header must never be trusted, even if the HMAC is correct.
    let header = format!("t={TS},v0={EXPECTED_HEX}");
    assert_eq!(
        Webhook::verify_header(PAYLOAD, &header, SECRET, 0),
        Err(WebhookError::NoSignatureForScheme)
    );
}

#[test]
fn rejects_old_timestamp_within_tolerance_window() {
    let header = format!("t={TS},v1={EXPECTED_HEX}");
    // TS is far in the past relative to the real clock.
    assert_eq!(
        Webhook::verify_header(PAYLOAD, &header, SECRET, 300),
        Err(WebhookError::TimestampOutOfTolerance)
    );
}

#[test]
fn rejects_a_signature_dated_in_the_future() {
    // A clock in the past must not open a replay window: the check is symmetric.
    let now = 1_700_000_000;
    let future = now + 3_600;
    let signature = sign(future, PAYLOAD, SECRET);
    let header = format!("t={future},v1={signature}");
    assert_eq!(
        Webhook::verify_header_at(PAYLOAD, &header, SECRET, 300, now),
        Err(WebhookError::TimestampOutOfTolerance)
    );
}

#[test]
fn verify_at_accepts_a_queued_delivery() {
    // A consumer that stores raw deliveries and processes them later can verify
    // against the time the delivery was received.
    let received = 1_700_000_000;
    let processed_later = received + 3_600;
    let signature = sign(received, PAYLOAD, SECRET);
    let header = format!("t={received},v1={signature}");
    Webhook::verify_header_at(PAYLOAD, &header, SECRET, 300, received).expect("within the window of receipt");
    assert_eq!(
        Webhook::verify_header_at(PAYLOAD, &header, SECRET, 300, processed_later),
        Err(WebhookError::TimestampOutOfTolerance),
        "the tolerance is measured against the supplied time"
    );
}

#[test]
fn tolerance_boundary_is_inclusive() {
    let now = 1_700_000_000;
    let signature = sign(now - 300, PAYLOAD, SECRET);
    let header = format!("t={},v1={signature}", now - 300);
    Webhook::verify_header_at(PAYLOAD, &header, SECRET, 300, now).expect("exactly at the boundary");
    let signature = sign(now - 301, PAYLOAD, SECRET);
    let header = format!("t={},v1={signature}", now - 301);
    assert_eq!(
        Webhook::verify_header_at(PAYLOAD, &header, SECRET, 300, now),
        Err(WebhookError::TimestampOutOfTolerance)
    );
}

#[test]
fn zero_tolerance_disables_recency_check() {
    let header = format!("t={TS},v1={EXPECTED_HEX}");
    Webhook::verify_header(PAYLOAD, &header, SECRET, 0).expect("tolerance 0 skips the recency check");
}

#[test]
fn extreme_timestamps_are_rejected_without_panicking() {
    // The tolerance check subtracts the signed timestamp from the supplied
    // `now`; both extremes must be reported, never panic (or wrap in a release
    // build and be accepted) — see the `abs_diff` comment in `webhooks.rs`.
    for (timestamp, now) in [
        (i64::MAX, 1_700_000_000),
        (0, i64::MIN),
        (i64::MIN, i64::MAX),
        (i64::MAX, i64::MIN),
    ] {
        let signature = sign(timestamp, PAYLOAD, SECRET);
        let header = format!("t={timestamp},v1={signature}");
        assert_eq!(
            Webhook::verify_header_at(PAYLOAD, &header, SECRET, 300, now),
            Err(WebhookError::TimestampOutOfTolerance),
            "timestamp {timestamp} against now {now}"
        );
    }
}

#[test]
fn a_negative_tolerance_behaves_like_its_magnitude() {
    let now = 1_700_000_000;
    let signature = sign(now - 2, PAYLOAD, SECRET);
    let header = format!("t={},v1={signature}", now - 2);
    assert_eq!(
        Webhook::verify_header_at(PAYLOAD, &header, SECRET, -5, now),
        Ok(()),
        "a tolerance of -5 allows a 2 second difference"
    );
    let signature = sign(now - 6, PAYLOAD, SECRET);
    let header = format!("t={},v1={signature}", now - 6);
    assert_eq!(
        Webhook::verify_header_at(PAYLOAD, &header, SECRET, -5, now),
        Err(WebhookError::TimestampOutOfTolerance)
    );
}

#[test]
fn recent_timestamp_is_accepted() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let signature = sign(now, PAYLOAD, SECRET);
    let header = format!("t={now},v1={signature}");
    Webhook::verify_header(PAYLOAD, &header, SECRET, 300).expect("fresh signature");
}

#[test]
fn missing_header_and_secret() {
    assert_eq!(Webhook::verify_header(PAYLOAD, "", SECRET, 0), Err(WebhookError::MissingHeader));
    let header = format!("t={TS},v1={EXPECTED_HEX}");
    assert_eq!(
        Webhook::verify_header(PAYLOAD, &header, "", 0),
        Err(WebhookError::MissingSecret)
    );
}

#[test]
fn malformed_headers_do_not_panic() {
    for header in [
        "garbage",
        "t=",
        "t=abc,v1=xyz",
        "=v1=1",
        ",,,,,,",
        "t=1,v1=",
        "t=99999999999999999999999999,v1=00",
    ] {
        let result = Webhook::verify_header(PAYLOAD, header, SECRET, 0);
        assert!(result.is_err(), "header {header:?} must be rejected");
    }
}

#[test]
fn hostile_header_is_rejected_without_panic() {
    // A megabyte of comma-separated junk.
    let header = "t=1,".to_string() + &"v1=00,".repeat(100_000);
    let result = Webhook::verify_header(PAYLOAD, &header, SECRET, 0);
    assert!(result.is_err());
}

#[test]
fn construct_event_returns_typed_payload() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let signature = sign(now, PAYLOAD, SECRET);
    let header = format!("t={now},v1={signature}");
    let event: Value = Webhook::construct_event(PAYLOAD, &header, SECRET, 300).expect("verified event");
    assert_eq!(event["id"], "evt_test_webhook");
}

#[test]
fn construct_event_does_not_parse_unverified_payload() {
    let header = format!("t={TS},v1={EXPECTED_HEX}");
    // Valid signature, but the payload is not JSON: the signature must be
    // checked first, so this is a decode error, not a webhook error.
    let result: Result<Value, _> = Webhook::construct_event(b"not json", &header, SECRET, 0);
    // Signature is over `not json`, so it won't match -> webhook error.
    assert!(matches!(result, Err(stripe::Error::Webhook(_))));
}
