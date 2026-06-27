use std::collections::VecDeque;

/// Tracks sent packets that need acknowledgement and received packets
/// that need to be acknowledged.
pub struct AckTracker {
    /// Packet numbers we sent that are pending ACK.
    pub sent_packets: VecDeque<SentPacket>,
    /// Largest packet number received from peer.
    pub largest_received: u64,
    /// Whether an ack-eliciting packet has been received since the last ACK.
    pub ack_eliciting_since_last_ack: bool,
}

pub struct SentPacket {
    pub pn: u64,
    pub time_sent: std::time::Instant,
    pub ack_eliciting: bool,
    pub encrypted_level: u8, // 0=Initial, 1=Handshake, 2=1-RTT
}

impl AckTracker {
    pub fn new() -> Self {
        Self {
            sent_packets: VecDeque::new(),
            largest_received: 0,
            ack_eliciting_since_last_ack: false,
        }
    }

    pub fn on_packet_sent(&mut self, pn: u64, ack_eliciting: bool, level: u8) {
        self.sent_packets.push_back(SentPacket {
            pn,
            time_sent: std::time::Instant::now(),
            ack_eliciting,
            encrypted_level: level,
        });
    }

    pub fn on_packet_received(&mut self, pn: u64, ack_eliciting: bool) {
        if pn > self.largest_received {
            self.largest_received = pn;
        }
        if ack_eliciting {
            self.ack_eliciting_since_last_ack = true;
        }
    }

    /// Remove acknowledged packets from the tracker. Returns the number removed.
    pub fn on_ack_received(&mut self, largest_acked: u64) -> usize {
        let before = self.sent_packets.len();
        self.sent_packets.retain(|p| p.pn > largest_acked);
        before - self.sent_packets.len()
    }

    /// Reset the ACK-needed flag (after sending an ACK).
    pub fn reset_ack_flag(&mut self) {
        self.ack_eliciting_since_last_ack = false;
    }
}
