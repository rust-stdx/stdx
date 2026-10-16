//! Webhook signature verification.
//!
//! Stripe signs every webhook delivery with HMAC-SHA256 over the string
//! `"{timestamp}.{raw_body}"` and sends the result in the `Stripe-Signature`
//! header. Verification has three requirements that are easy to get wrong:
//!
//! 1. The **raw** request body must be hashed. Re-serializing parsed JSON
//!    changes whitespace and key order and invalidates the signature.
//! 2. Signatures must be compared in **constant time** to avoid leaking the
//!    expected value through timing.
//! 3. Only the `v1` scheme may be trusted; other schemes (`v0` in particular)
//!    exist for testing and must be ignored to prevent downgrade attacks.
//!
//! This module implements all three. It also enforces a recency window so that
//! captured deliveries cannot be replayed indefinitely.

use crypto::{hmac::Hmac, sha2::Sha256};
use serde::de::DeserializeOwned;

use crate::error::{Error, WebhookError};

const HEX: &[u8; 16] = b"0123456789abcdef";

/// The signature scheme Stripe uses for live and test webhooks.
const EXPECTED_SCHEME: &str = "v1";

/// The default recency tolerance, in seconds (5 minutes), matching Stripe's
/// official libraries.
pub const DEFAULT_TOLERANCE_SECONDS: i64 = 300;

/// Webhook signature helpers.
pub struct Webhook;

impl Webhook {
    /// Verifies a `Stripe-Signature` header without parsing the payload.
    ///
    /// * `payload` is the raw request body, exactly as received.
    /// * `sig_header` is the value of the `Stripe-Signature` header.
    /// * `secret` is the endpoint's signing secret (it starts with `whsec_`).
    /// * `tolerance` is the maximum accepted difference between the signed
    ///   timestamp and now, in seconds, in **either** direction (a captured
    ///   delivery cannot be replayed, and a signature dated in the future is
    ///   rejected too). Passing `0` **disables the recency check entirely**
    ///   (matching Stripe's libraries); do not use `0` in production. A
    ///   negative value is treated as its magnitude.
    ///
    /// Returns `Ok(())` when at least one `v1` signature matches. Returns a
    /// [`WebhookError`] otherwise.
    pub fn verify_header(payload: &[u8], sig_header: &str, secret: &str, tolerance: i64) -> Result<(), WebhookError> {
        let now = now_unix_seconds().ok_or(WebhookError::InvalidTimestamp)?;
        Self::verify_header_at(payload, sig_header, secret, tolerance, now)
    }

    /// Like [`Webhook::verify_header`], but relative to `now`.
    ///
    /// Use it to verify deliveries that were queued (a webhook consumer that
    /// stores raw payloads before processing them, for example). Only pass a
    /// trusted value: a receiver that lets a client pick `now` loses the replay
    /// protection.
    pub fn verify_header_at(
        payload: &[u8],
        sig_header: &str,
        secret: &str,
        tolerance: i64,
        now: i64,
    ) -> Result<(), WebhookError> {
        if sig_header.is_empty() {
            return Err(WebhookError::MissingHeader);
        }
        if secret.is_empty() {
            return Err(WebhookError::MissingSecret);
        }

        let timestamp = extract_timestamp(sig_header).ok_or(WebhookError::InvalidTimestamp)?;

        // Compute the expected signature once, then compare it to every `v1`
        // value in the header. Signatures from other schemes are ignored.
        let mut mac = Hmac::<Sha256>::new(secret.as_bytes());
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(payload);
        let tag = mac.finalize();
        let tag = tag.as_ref();

        let mut expected_hex = [0u8; 64];
        let tag_len = tag.len().min(32);
        for (index, byte) in tag[..tag_len].iter().enumerate() {
            expected_hex[index * 2] = HEX[(byte >> 4) as usize];
            expected_hex[index * 2 + 1] = HEX[(byte & 0x0f) as usize];
        }
        let expected_hex = &expected_hex[..tag_len * 2];

        let mut found_scheme = false;
        let mut matched = false;
        for part in sig_header.split(',') {
            let Some((key, value)) = part.split_once('=') else {
                continue;
            };
            if key.trim() != EXPECTED_SCHEME {
                continue;
            }
            found_scheme = true;
            // Constant-time comparison: never use `==` here.
            if constant_time_eq::constant_time_eq(expected_hex, value.as_bytes()) {
                matched = true;
            }
        }

        if !found_scheme {
            return Err(WebhookError::NoSignatureForScheme);
        }
        if !matched {
            return Err(WebhookError::SignatureMismatch);
        }

        if tolerance != 0 {
            let timestamp: i64 = timestamp.parse().map_err(|_| WebhookError::InvalidTimestamp)?;
            // `abs_diff` rather than `abs`: a hostile (or clock-skewed) `now`
            // near `i64::MIN` must not panic the overflow check, and the
            // comparison must not wrap into accepting an old signature.
            if now.abs_diff(timestamp) > tolerance.unsigned_abs() {
                return Err(WebhookError::TimestampOutOfTolerance);
            }
        }

        Ok(())
    }

    /// Verifies the signature and then decodes the raw payload as `T`.
    ///
    /// Use this with the generated `Event` type to obtain a typed event, or
    /// with your own type. Decoding happens only after verification succeeds, so
    /// unauthenticated bytes are never parsed.
    pub fn construct_event<T: DeserializeOwned>(
        payload: &[u8],
        sig_header: &str,
        secret: &str,
        tolerance: i64,
    ) -> Result<T, Error> {
        Self::verify_header(payload, sig_header, secret, tolerance).map_err(Error::Webhook)?;
        Self::parse_event(payload)
    }

    /// Decodes a raw webhook payload **without** verifying its signature.
    ///
    /// Only call this on payloads that were verified already, or that come from
    /// a trusted source. Prefer [`Webhook::construct_event`].
    pub fn parse_event<T: DeserializeOwned>(payload: &[u8]) -> Result<T, Error> {
        serde_json::from_slice(payload).map_err(|source| Error::Decode {
            status: 200,
            request_id: None,
            source,
        })
    }
}

fn extract_timestamp(header: &str) -> Option<&str> {
    for part in header.split(',') {
        let Some((key, value)) = part.split_once('=') else {
            continue;
        };
        if key.trim() == "t" && !value.is_empty() {
            return Some(value);
        }
    }
    None
}

fn now_unix_seconds() -> Option<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| i64::try_from(duration.as_secs()).ok())
}

/// Verifies a webhook signature using the default 5-minute tolerance.
///
/// See [`Webhook::verify_header`].
pub fn verify_signature(payload: &[u8], sig_header: &str, secret: &str) -> Result<(), WebhookError> {
    Webhook::verify_header(payload, sig_header, secret, DEFAULT_TOLERANCE_SECONDS)
}
