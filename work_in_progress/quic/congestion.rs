use std::time::Duration;

/// Congestion control trait (RFC 9002 §7).
pub trait CongestionController: Send {
    /// Called when a packet is sent.
    fn on_packet_sent(&mut self, bytes_in_flight: usize);
    /// Called when a packet is acknowledged. `rtt` is the latest RTT sample.
    fn on_packet_acked(&mut self, size: usize, rtt: Duration, bytes_in_flight: usize);
    /// Called when a packet is declared lost.
    fn on_packet_lost(&mut self, size: usize, bytes_in_flight: usize);
    /// Current congestion window in bytes.
    fn cwnd(&self) -> usize;
    /// Whether we can send more data (bytes_in_flight < cwnd).
    fn can_send(&self, bytes_in_flight: usize) -> bool {
        bytes_in_flight < self.cwnd()
    }
    /// Whether the controller is in recovery mode.
    fn in_recovery(&self) -> bool;
    /// The number of bytes acknowledged (for slow start tracking).
    fn bytes_in_flight(&self) -> usize;
}

/// NewReno congestion controller (RFC 9002 §7).
///
/// Implements slow start, congestion avoidance, and fast recovery.
/// Uses packet-based congestion detection (not byte-based) suitable for
/// QUIC's loss detection mechanism.
pub struct NewReno {
    cwnd_bytes: usize,
    ssthresh: usize,
    bytes_in_flight: usize,
    max_datagram_size: usize,
    /// In recovery mode after packet loss.
    in_recovery: bool,
    /// The largest packet number we've sent (for recovery exit).
    recovery_pn: u64,
    /// Total acknowledged bytes in current recovery phase.
    recovery_acked: usize,
    /// Slow start threshold in packets (initial: infinite).
    slow_start: bool,
}

impl NewReno {
    /// Create a new NewReno congestion controller.
    ///
    /// `max_datagram_size` is the maximum payload size per packet (typically
    /// 1200 for Initial, negotiated up to larger values).
    pub fn new(max_datagram_size: usize) -> Self {
        // RFC 9002: initial window = max(10 * max_datagram_size, 14720)
        let initial_cwnd = (10 * max_datagram_size).max(14720);
        Self {
            cwnd_bytes: initial_cwnd,
            ssthresh: usize::MAX,
            bytes_in_flight: 0,
            max_datagram_size,
            in_recovery: false,
            recovery_pn: 0,
            recovery_acked: 0,
            slow_start: true,
        }
    }

    /// Update the max datagram size (e.g. after PMTUD).
    pub fn set_max_datagram_size(&mut self, size: usize) {
        self.max_datagram_size = size;
    }

    /// Called when the last sent packet number changes (for recovery tracking).
    pub fn set_last_sent_pn(&mut self, pn: u64) {
        self.recovery_pn = pn;
    }
}

impl CongestionController for NewReno {
    fn on_packet_sent(&mut self, bytes: usize) {
        self.bytes_in_flight += bytes;
    }

    fn on_packet_acked(&mut self, size: usize, _rtt: Duration, bytes_in_flight: usize) {
        self.bytes_in_flight = bytes_in_flight;

        if self.in_recovery {
            self.recovery_acked += size;
            if self.recovery_acked >= self.ssthresh {
                self.in_recovery = false;
                self.recovery_acked = 0;
            }
            // During recovery, don't increase cwnd
            return;
        }

        if self.slow_start {
            // Slow start: cwnd += min(size, max_datagram_size) per ACK
            self.cwnd_bytes += size.min(self.max_datagram_size);
            if self.cwnd_bytes >= self.ssthresh {
                self.slow_start = false;
            }
        } else {
            // Congestion avoidance: additive increase
            // cwnd += max_datagram_size * size / cwnd  (approximate)
            let inc = (self.max_datagram_size as u64 * size as u64 / self.cwnd_bytes as u64) as usize;
            self.cwnd_bytes += inc.max(1);
        }
    }

    fn on_packet_lost(&mut self, _size: usize, bytes_in_flight: usize) {
        self.bytes_in_flight = bytes_in_flight;
        if !self.in_recovery {
            self.ssthresh = (self.cwnd_bytes / 2).max(2 * self.max_datagram_size);
            self.cwnd_bytes = self.ssthresh;
            self.slow_start = false;
            self.in_recovery = true;
            self.recovery_acked = 0;
        }
    }

    fn cwnd(&self) -> usize {
        self.cwnd_bytes
    }

    fn in_recovery(&self) -> bool {
        self.in_recovery
    }

    fn bytes_in_flight(&self) -> usize {
        self.bytes_in_flight
    }
}

/// Fixed-window congestion control: never reduces the window.
/// Used when no congestion control is desired.
pub struct FixedWindowCongestionControl {
    cwnd_bytes: usize,
}

impl FixedWindowCongestionControl {
    pub fn new(max_datagram_size: usize) -> Self {
        let cwnd_bytes = (10 * max_datagram_size).max(14720);
        Self {
            cwnd_bytes,
        }
    }
}

impl CongestionController for FixedWindowCongestionControl {
    fn on_packet_sent(&mut self, _bytes: usize) {}
    fn on_packet_acked(&mut self, _size: usize, _rtt: Duration, _bytes_in_flight: usize) {}
    fn on_packet_lost(&mut self, _size: usize, _bytes_in_flight: usize) {}
    fn cwnd(&self) -> usize {
        self.cwnd_bytes
    }
    fn in_recovery(&self) -> bool {
        false
    }
    fn bytes_in_flight(&self) -> usize {
        0
    }
}
