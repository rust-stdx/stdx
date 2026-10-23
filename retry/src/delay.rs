use std::time::Duration;

/// A retry delay strategy.
///
/// Implementations determine how long to wait before each retry attempt.
pub trait Delay: Send {
    /// Returns the delay before the next retry attempt.
    fn next_delay(&mut self) -> Duration;
}

/// Fixed delay between retries.
///
/// Every retry attempt waits the same amount of time.
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// use retry::delay::{Delay, Fixed};
///
/// let mut d = Fixed::new(Duration::from_secs(2));
/// assert_eq!(d.next_delay(), Duration::from_secs(2));
/// assert_eq!(d.next_delay(), Duration::from_secs(2));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Fixed {
    duration: Duration,
}

impl Fixed {
    pub const fn new(duration: Duration) -> Self {
        Self {
            duration,
        }
    }
}

impl Delay for Fixed {
    fn next_delay(&mut self) -> Duration {
        self.duration
    }
}

/// Exponential backoff delay.
///
/// Each retry multiplies the delay by `multiplier` (default 2),
/// capped at `max` (default 60s). Optional jitter applies
/// ±25% random variation to each delay value.
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// use retry::delay::{Delay, Exponential};
///
/// let mut d = Exponential::new(Duration::from_millis(100))
///     .with_max(Duration::from_secs(2))
///     .with_multiplier(2);
///
/// assert_eq!(d.next_delay(), Duration::from_millis(100));
/// assert_eq!(d.next_delay(), Duration::from_millis(200));
/// assert_eq!(d.next_delay(), Duration::from_millis(400));
/// assert_eq!(d.next_delay(), Duration::from_millis(800));
/// assert_eq!(d.next_delay(), Duration::from_millis(1600));
/// // capped at max
/// assert_eq!(d.next_delay(), Duration::from_secs(2));
/// assert_eq!(d.next_delay(), Duration::from_secs(2));
/// ```
#[derive(Debug, Clone)]
pub struct Exponential {
    current: Duration,
    max: Duration,
    multiplier: u32,
    jitter: bool,
    rng_state: u64,
}

impl Exponential {
    /// Creates a new exponential backoff starting with `initial` delay.
    pub fn new(initial: Duration) -> Self {
        Self {
            current: initial,
            max: Duration::from_secs(60),
            multiplier: 2,
            jitter: false,
            rng_state: initial.as_nanos() as u64 ^ 0x9e3779b97f4a7c15,
        }
    }

    /// Sets the maximum delay cap (default 60s).
    pub fn with_max(mut self, max: Duration) -> Self {
        self.max = max;
        self
    }

    /// Sets the multiplier applied to the delay after each retry (default 2).
    pub fn with_multiplier(mut self, multiplier: u32) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// Enables ±25% random jitter on each delay value.
    ///
    /// The actual delay will be uniformly distributed in the range
    /// `[0.75 * delay, 1.25 * delay]`.
    pub fn with_jitter(mut self) -> Self {
        self.jitter = true;
        self.rng_state = random_seed();
        self
    }

    fn jitter(&mut self, delay: Duration) -> Duration {
        if delay == Duration::ZERO {
            return Duration::ZERO;
        }

        // xorshift64
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;

        let nanos = delay.as_nanos();
        // ±25% → multiplier in range [7500, 12500] per 10000
        let offset = (self.rng_state % 5001) as u128;
        let multiplier = 7500 + offset;
        // Saturate instead of wrapping: a duration so large that the scaled
        // value does not fit `u64` nanoseconds becomes the longest representable
        // delay rather than an arbitrary shorter one.
        let scaled = nanos.saturating_mul(multiplier) / 10000;
        Duration::from_nanos(u64::try_from(scaled).unwrap_or(u64::MAX))
    }
}

impl Delay for Exponential {
    fn next_delay(&mut self) -> Duration {
        let delay = self.current;

        // Grow current for next call, capped at max
        let current_nanos = self.current.as_nanos();
        let next = current_nanos.saturating_mul(self.multiplier as u128);
        let max_nanos = self.max.as_nanos();
        self.current = Duration::from_nanos(u64::try_from(std::cmp::min(next, max_nanos)).unwrap_or(u64::MAX));

        if self.jitter { self.jitter(delay) } else { delay }
    }
}

fn random_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e3779b97f4a7c15)
}
