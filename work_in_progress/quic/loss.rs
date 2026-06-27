use std::time::{Duration, Instant};

/// Basic loss detection as per RFC 9002.
pub struct LossDetection {
    pub latest_rtt: Duration,
    pub min_rtt: Duration,
    pub smoothed_rtt: Duration,
    pub rttvar: Duration,
    pub pto_count: u32,
    pub max_ack_delay: Duration,
    pub time_last_ack_eliciting: Option<Instant>,
}

impl LossDetection {
    pub fn new() -> Self {
        Self {
            latest_rtt: Duration::ZERO,
            min_rtt: Duration::MAX,
            smoothed_rtt: Duration::ZERO,
            rttvar: Duration::ZERO,
            pto_count: 0,
            max_ack_delay: Duration::from_millis(25),
            time_last_ack_eliciting: None,
        }
    }

    /// Called when an RTT sample is available.
    pub fn on_rtt_measurement(&mut self, rtt: Duration, ack_delay: Duration) {
        self.latest_rtt = rtt;
        if rtt < self.min_rtt {
            self.min_rtt = rtt;
        }

        let adjusted_rtt = if self.smoothed_rtt == Duration::ZERO {
            rtt
        } else {
            rtt.min(ack_delay + self.latest_rtt)
        };

        if self.smoothed_rtt == Duration::ZERO {
            self.smoothed_rtt = rtt;
            self.rttvar = rtt / 2;
        } else {
            self.rttvar = (self.rttvar * 3 / 4) + (smoothed_rtt_diff(self.smoothed_rtt, adjusted_rtt) / 4);
            self.smoothed_rtt = self.smoothed_rtt * 7 / 8 + adjusted_rtt / 8;
        }
    }

    /// Compute PTO (Probe Timeout).
    pub fn pto(&self) -> Duration {
        self.smoothed_rtt + self.rttvar * 4 + self.max_ack_delay
    }

    /// Call when PTO fires.
    pub fn on_pto_timeout(&mut self) {
        self.pto_count += 1;
    }

    /// Reset PTO counter after receiving an ACK.
    pub fn on_ack_received(&mut self) {
        self.pto_count = 0;
    }
}

fn smoothed_rtt_diff(a: Duration, b: Duration) -> Duration {
    if a > b { a - b } else { b - a }
}
