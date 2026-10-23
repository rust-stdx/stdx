use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use retry::{
    Config, Error,
    delay::{Delay, Exponential, Fixed},
    retry,
};

#[tokio::test]
async fn succeeds_first_try() {
    let result = retry(
        || async { Ok::<_, &'static str>(42) },
        Config::new(Fixed::new(Duration::from_millis(10))).with_attempts(3),
    )
    .await;

    assert_eq!(result, Ok(42));
}

#[tokio::test]
async fn succeeds_after_retries() {
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();

    let result = retry(
        move || {
            let c = c.clone();
            async move {
                if c.fetch_add(1, Ordering::SeqCst) < 2 {
                    Err::<&'static str, _>("not yet")
                } else {
                    Ok("done")
                }
            }
        },
        Config::new(Fixed::new(Duration::from_millis(5))).with_attempts(5),
    )
    .await;

    assert_eq!(result, Ok("done"));
    assert_eq!(count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn exhausts_attempts() {
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();

    let result = retry(
        move || {
            let c = c.clone();
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Err::<(), _>("always fails")
            }
        },
        Config::new(Fixed::new(Duration::from_millis(5))).with_attempts(3),
    )
    .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.attempts, 3);
    assert_eq!(count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn infinite_retries_until_success() {
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();

    let result = retry(
        move || {
            let c = c.clone();
            async move {
                if c.fetch_add(1, Ordering::SeqCst) < 10 {
                    Err::<&'static str, _>("not yet")
                } else {
                    Ok("finally")
                }
            }
        },
        Config::new(Fixed::new(Duration::from_millis(1))),
    )
    .await;

    assert_eq!(result, Ok("finally"));
    assert_eq!(count.load(Ordering::SeqCst), 11);
}

#[tokio::test]
async fn exponential_backoff_grows() {
    let mut d = Exponential::new(Duration::from_millis(10))
        .with_max(Duration::from_secs(1))
        .with_multiplier(3);

    assert_eq!(d.next_delay(), Duration::from_millis(10));
    assert_eq!(d.next_delay(), Duration::from_millis(30));
    assert_eq!(d.next_delay(), Duration::from_millis(90));
    assert_eq!(d.next_delay(), Duration::from_millis(270));
    assert_eq!(d.next_delay(), Duration::from_millis(810));
    assert_eq!(d.next_delay(), Duration::from_secs(1)); // capped at max
}

#[tokio::test]
async fn exponential_jitter_in_range() {
    // Cap max at initial so current never grows — all delays are 10s
    let mut d = Exponential::new(Duration::from_secs(10))
        .with_max(Duration::from_secs(10))
        .with_jitter();

    let mut delays = Vec::new();
    for _ in 0..100 {
        delays.push(d.next_delay());
    }

    // With ±25% on 10s: range should be [7.5s, 12.5s]
    for delay in &delays {
        let millis = delay.as_millis();
        assert!(millis >= 7_500, "delay {millis}ms is below 7.5s");
        assert!(millis <= 12_500, "delay {millis}ms is above 12.5s");
    }

    // At least 2 distinct values (jitter is working)
    let unique_count = {
        let mut set = std::collections::HashSet::new();
        for d in &delays {
            set.insert(*d);
        }
        set.len()
    };
    assert!(unique_count > 1, "jitter produced only {unique_count} unique values");
}

#[tokio::test]
async fn jitter_does_not_wrap_huge_durations() {
    // The scaled nanosecond value does not fit `u64`; it must clamp to the
    // longest representable delay instead of wrapping to an arbitrary short one.
    let mut d = Exponential::new(Duration::MAX).with_max(Duration::MAX).with_jitter();
    let delay = d.next_delay();
    assert!(delay.as_secs() > 1_000_000_000, "huge jittered delay wrapped to {delay:?}");
}

#[tokio::test]
async fn fixed_delay_does_not_jitter() {
    let mut d = Fixed::new(Duration::from_millis(100));
    for _ in 0..10 {
        assert_eq!(d.next_delay(), Duration::from_millis(100));
    }
}

#[tokio::test]
async fn on_retry_callback_invoked() {
    let retries = Arc::new(AtomicUsize::new(0));
    let r = retries.clone();

    let result = retry(
        || async { Err::<(), &'static str>("fail") },
        Config::new(Fixed::new(Duration::from_millis(1)))
            .with_attempts(4)
            .with_on_retry(move |attempt, _delay| {
                r.store(attempt, Ordering::SeqCst);
            }),
    )
    .await;

    assert!(result.is_err());
    // Last retry attempt before final error
    assert_eq!(retries.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn error_type_helpers() {
    let err = Error::<&str>::new("oops", 5);
    assert_eq!(err.attempts(), 5);
    assert_eq!(err.inner(), &"oops");
    assert_eq!(err.into_inner(), "oops");
}

#[tokio::test]
async fn error_display() {
    let err = Error::new("oops", 3);
    let msg = err.to_string();
    assert!(msg.contains("oops"));
    assert!(msg.contains("3"));
}
