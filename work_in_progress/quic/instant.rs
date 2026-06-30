use core::{ops::Add, time::Duration};

/// Monotonic timestamp in microseconds since an arbitrary fixed epoch.
///
/// Used internally for RTT measurement, loss detection, and idle timeout.
/// Wraps the value returned by [`Transport::now`](crate::Transport::now).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(u64);

impl Instant {
    pub const ZERO: Self = Instant(0);

    pub fn from_micros(us: u64) -> Self {
        Self(us)
    }

    pub fn to_micros(self) -> u64 {
        self.0
    }

    pub fn duration_since(self, earlier: Instant) -> Duration {
        Duration::from_micros(self.0.saturating_sub(earlier.0))
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Instant {
        Instant(self.0.saturating_add(rhs.as_micros() as u64))
    }
}
