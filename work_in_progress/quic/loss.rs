use core::time::Duration;

use crate::instant::Instant;

/// Loss detection and RTT tracking per RFC 9002.
///
/// Tracks RTT samples, computes smoothed RTT and RTTVAR,
/// and provides PTO (Probe Timeout) and loss detection thresholds.
pub struct LossDetection {
    pub latest_rtt: Duration,
    pub min_rtt: Duration,
    pub smoothed_rtt: Duration,
    pub rttvar: Duration,
    pub pto_count: u32,
    pub max_ack_delay: Duration,
    pub time_last_ack_eliciting: Option<Instant>,
    /// When the last ack-eliciting packet was sent.
    pub time_last_sent: Option<Instant>,
}

impl LossDetection {
    pub fn new(max_ack_delay: Duration) -> Self {
        Self {
            latest_rtt: Duration::ZERO,
            min_rtt: Duration::MAX,
            smoothed_rtt: Duration::ZERO,
            rttvar: Duration::ZERO,
            pto_count: 0,
            max_ack_delay,
            time_last_ack_eliciting: None,
            time_last_sent: None,
        }
    }

    /// Called when an RTT sample is available (from a received ACK).
    pub fn on_rtt_measurement(&mut self, rtt: Duration, ack_delay: Duration) {
        self.latest_rtt = rtt;
        if rtt < self.min_rtt {
            self.min_rtt = rtt;
        }

        // RFC 9002 §5.3: adjust for peer's ack delay
        let adjusted_rtt = if self.smoothed_rtt == Duration::ZERO {
            rtt
        } else {
            // Use the min of ack_delay and max_ack_delay to avoid
            // persistent over-estimation from delayed ACKs.
            let capped_delay = ack_delay.min(self.max_ack_delay);
            if rtt >= self.min_rtt + capped_delay {
                rtt - capped_delay
            } else {
                rtt
            }
        };

        if self.smoothed_rtt == Duration::ZERO {
            self.smoothed_rtt = adjusted_rtt;
            self.rttvar = adjusted_rtt / 2;
        } else {
            let diff = if self.smoothed_rtt > adjusted_rtt {
                self.smoothed_rtt - adjusted_rtt
            } else {
                adjusted_rtt - self.smoothed_rtt
            };
            self.rttvar = (self.rttvar * 3 / 4) + (diff / 4);
            self.smoothed_rtt = self.smoothed_rtt * 7 / 8 + adjusted_rtt / 8;
        }
    }

    /// Record that we sent an ack-eliciting packet.
    pub fn on_packet_sent(&mut self, now: Instant, ack_eliciting: bool) {
        if ack_eliciting {
            self.time_last_sent = Some(now);
        }
    }

    /// Record that we received an ack-eliciting packet.
    pub fn on_packet_received(&mut self, now: Instant, ack_eliciting: bool) {
        if ack_eliciting {
            self.time_last_ack_eliciting = Some(now);
        }
    }

    /// Compute PTO (Probe Timeout) per RFC 9002 §6.2.
    pub fn pto_duration(&self) -> Duration {
        if self.smoothed_rtt == Duration::ZERO {
            // Initial PTO: 1 second if no RTT estimate
            Duration::from_secs(1)
        } else {
            self.smoothed_rtt + (self.rttvar * 4).max(self.timer_granularity()) + self.max_ack_delay
        }
    }

    /// Time threshold for declaring a packet lost per RFC 9002 §6.1.2.
    /// Packets older than max(pto_time_threshold, kTimeThreshold * max(smoothed_rtt, latest_rtt))
    /// are candidates for loss detection.
    pub fn loss_time_threshold(&self) -> Duration {
        let pto_threshold = self.pto_duration() * (2u32.pow(self.pto_count.min(5)));
        let max_rtt = self.smoothed_rtt.max(self.latest_rtt);
        let k_time_threshold = Duration::from_nanos(max_rtt.as_nanos() as u64 * 9 / 8);
        pto_threshold.max(k_time_threshold)
    }

    /// Call when PTO fires (no response from peer).
    pub fn on_pto_timeout(&mut self) {
        self.pto_count += 1;
    }

    /// Reset PTO counter and timer after receiving an ACK.
    pub fn on_ack_received(&mut self, now: Instant) {
        self.pto_count = 0;
        self.time_last_sent = Some(now);
    }

    /// Whether the PTO timer has expired (i.e. we should send a probe).
    pub fn pto_expired(&self, now: Instant) -> bool {
        if let Some(last_sent) = self.time_last_sent {
            now.duration_since(last_sent) >= self.pto_duration()
        } else {
            true
        }
    }

    /// Timer granularity (RFC 9002 §6.1.1).
    fn timer_granularity(&self) -> Duration {
        Duration::from_millis(1)
    }
}
