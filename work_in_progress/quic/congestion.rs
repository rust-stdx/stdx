/// Congestion control trait.
pub trait CongestionController: Send {
    fn on_packet_sent(&mut self, _pn: u64, _size: usize, _bytes_in_flight: usize) {}
    fn on_packet_acked(&mut self, _pn: u64, _size: usize, _rtt: std::time::Duration) {}
    fn on_packet_lost(&mut self, _pn: u64, _size: usize, _bytes_in_flight: usize) {}
    fn cwnd(&self) -> usize;
    fn can_send(&self, bytes_in_flight: usize) -> bool;
    fn in_recovery(&self) -> bool {
        false
    }
}

/// Fixed-window congestion control: never reduces the window.
pub struct FixedWindowCongestionControl {
    cwnd_bytes: usize,
}

impl FixedWindowCongestionControl {
    pub fn new(max_datagram_size: usize) -> Self {
        // RFC 9002: initial window = 10 * max_datagram_size
        let cwnd_bytes = (10 * max_datagram_size).max(14720);
        Self {
            cwnd_bytes,
        }
    }
}

impl CongestionController for FixedWindowCongestionControl {
    fn cwnd(&self) -> usize {
        self.cwnd_bytes
    }
    fn can_send(&self, bytes_in_flight: usize) -> bool {
        bytes_in_flight < self.cwnd_bytes
    }
    fn in_recovery(&self) -> bool {
        false
    }
}
