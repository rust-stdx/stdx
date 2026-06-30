use alloc::{collections::VecDeque, vec::Vec};

use hashbrown::HashSet;

use crate::{frame::Frame, instant::Instant};

/// Tracks sent packets (for loss detection / retransmission) and
/// received packets (for generating ACK frames with proper ranges).
pub struct AckTracker {
    /// Packets we sent that are pending ACK from the peer.
    sent_packets: VecDeque<SentPacket>,
    /// Largest packet number received from peer.
    pub largest_received: u64,
    /// Largest packet number acknowledged by the peer.
    pub largest_acked: u64,
    /// Packet numbers received since the last ACK was sent.
    received_since_last_ack: Vec<u64>,
    /// Whether an ack-eliciting packet has been received since the last ACK.
    pub ack_eliciting_since_last_ack: bool,
    /// When the first ack-eliciting packet in the current batch arrived.
    pub first_ack_eliciting: Option<Instant>,
}

/// A packet we sent that the peer has not yet acknowledged.
pub struct SentPacket {
    pub pn: u64,
    pub time_sent: Instant,
    pub ack_eliciting: bool,
    /// Encryption level: 0 = Initial, 1 = Handshake, 2 = 1-RTT.
    pub encrypted_level: u8,
    /// Whether the packet used a long header (for retransmission format).
    pub long_header: bool,
    /// Whether this was an Initial packet (for retransmission padding).
    pub is_initial: bool,
    /// The frames carried in this packet (for retransmission).
    pub frames: Vec<Frame>,
}

/// A decoded ACK range suitable for building an ACK frame.
pub struct AckRanges {
    pub largest: u64,
    pub delay: u64,
    pub first_range: u64,
    pub extra_ranges: Vec<(u64, u64)>,
}

impl AckTracker {
    pub fn new() -> Self {
        Self {
            sent_packets: VecDeque::new(),
            largest_received: 0,
            largest_acked: 0,
            received_since_last_ack: Vec::new(),
            ack_eliciting_since_last_ack: false,
            first_ack_eliciting: None,
        }
    }

    /// Record a packet we just sent.
    pub fn on_packet_sent(
        &mut self,
        now: Instant,
        pn: u64,
        ack_eliciting: bool,
        level: u8,
        long_header: bool,
        is_initial: bool,
        frames: Vec<Frame>,
    ) {
        self.sent_packets.push_back(SentPacket {
            pn,
            time_sent: now,
            ack_eliciting,
            encrypted_level: level,
            long_header,
            is_initial,
            frames,
        });
        // Limit queue size to avoid unbounded growth
        while self.sent_packets.len() > 1000 {
            self.sent_packets.pop_front();
        }
    }

    /// Record a packet we received. Call this for every incoming packet.
    pub fn on_packet_received(&mut self, now: Instant, pn: u64, ack_eliciting: bool) {
        if pn > self.largest_received {
            self.largest_received = pn;
        }
        self.received_since_last_ack.push(pn);
        if ack_eliciting {
            if !self.ack_eliciting_since_last_ack {
                self.first_ack_eliciting = Some(now);
            }
            self.ack_eliciting_since_last_ack = true;
        }
    }

    /// Process a received ACK frame. Returns the list of newly-acked PNs and their
    /// time_sent (for RTT measurement).
    pub fn on_ack_received(&mut self, ranges: &AckRanges) -> (Vec<u64>, Vec<Instant>) {
        let acked_set = ranges_to_set(ranges);

        // Update largest acked
        if ranges.largest > self.largest_acked {
            self.largest_acked = ranges.largest;
        }

        let mut newly_acked = Vec::new();
        let mut sent_times = Vec::new();

        let keep: Vec<SentPacket> = core::mem::take(&mut self.sent_packets)
            .into_iter()
            .filter_map(|p| {
                if acked_set.contains(&p.pn) {
                    newly_acked.push(p.pn);
                    sent_times.push(p.time_sent);
                    None
                } else {
                    Some(p)
                }
            })
            .collect();
        self.sent_packets = keep.into();
        (newly_acked, sent_times)
    }

    /// Build an ACK frame from the accumulated received packet numbers.
    /// Clears the accumulator after generating ranges.
    pub fn build_ack(&mut self, ack_delay: u64) -> AckRanges {
        let mut pns = core::mem::take(&mut self.received_since_last_ack);
        pns.sort_unstable();
        pns.dedup();
        self.ack_eliciting_since_last_ack = false;
        self.first_ack_eliciting = None;

        if pns.is_empty() {
            return AckRanges {
                largest: self.largest_received,
                delay: ack_delay,
                first_range: 0,
                extra_ranges: Vec::new(),
            };
        }

        let largest = *pns.last().unwrap();
        // Build contiguous blocks in descending order, then reverse
        let mut blocks: Vec<(u64, u64)> = Vec::new(); // (start, end) inclusive
        let mut i = pns.len();
        while i > 0 {
            i -= 1;
            let end = pns[i];
            let mut start = end;
            while i > 0 && pns[i - 1] == start.wrapping_sub(1) {
                i -= 1;
                start = pns[i];
            }
            blocks.push((start, end));
        }
        blocks.reverse();
        // blocks[0] is the lowest range, blocks[last] is the highest

        let first_range = if let Some(&(start, end)) = blocks.last() {
            debug_assert_eq!(end, largest);
            end - start
        } else {
            0
        };

        let mut extra_ranges = Vec::new();
        let mut prev_start = blocks.last().map(|&(s, _)| s).unwrap_or(largest + 1);

        for idx in (0..blocks.len().saturating_sub(1)).rev() {
            let (start, end) = blocks[idx];
            let gap = prev_start.saturating_sub(end + 1);
            let len = end - start + 1;
            if len > 0 {
                extra_ranges.push((gap, len));
            }
            prev_start = start;
        }

        AckRanges {
            largest,
            delay: ack_delay,
            first_range,
            extra_ranges,
        }
    }

    /// Detect sent packets that should be declared lost.
    /// Uses time-based loss: any unacked packet older than `time_threshold`
    /// AND with a PN less than `largest_acked` is potentially lost.
    pub fn detect_lost_packets(
        &self,
        now: Instant,
        time_threshold: core::time::Duration,
        largest_acked: u64,
    ) -> Vec<u64> {
        self.sent_packets
            .iter()
            .filter(|p| p.pn < largest_acked && now.duration_since(p.time_sent) >= time_threshold)
            .map(|p| p.pn)
            .collect()
    }

    /// Remove packets that have been declared lost. Returns (pn, level, long_header, frames) for retransmission.
    pub fn remove_lost(&mut self, lost_pns: &[u64]) -> Vec<(u64, u8, bool, bool, Vec<Frame>)> {
        let lost_set: HashSet<u64> = lost_pns.iter().copied().collect();
        let mut result = Vec::new();
        let keep: Vec<SentPacket> = core::mem::take(&mut self.sent_packets)
            .into_iter()
            .filter_map(|p| {
                if lost_set.contains(&p.pn) {
                    result.push((p.pn, p.encrypted_level, p.long_header, p.is_initial, p.frames));
                    None
                } else {
                    Some(p)
                }
            })
            .collect();
        self.sent_packets = keep.into();
        result
    }

    /// True if there are any unacknowledged ack-eliciting packets.
    pub fn has_unacked(&self) -> bool {
        self.sent_packets.iter().any(|p| p.ack_eliciting)
    }

    /// The number of unacknowledged sent packets.
    pub fn unacked_count(&self) -> usize {
        self.sent_packets.len()
    }

    /// Whether the sent-packet queue is empty.
    pub fn is_empty(&self) -> bool {
        self.sent_packets.is_empty()
    }

    /// Return the encryption level of the most recently sent unacked packet, if any.
    pub fn last_sent_level(&self) -> Option<u8> {
        self.sent_packets.back().map(|p| p.encrypted_level)
    }
}

/// Convert AckRanges into a HashSet of acknowledged PNs.
fn ranges_to_set(ranges: &AckRanges) -> HashSet<u64> {
    let mut set = HashSet::new();
    let first_end = ranges.largest.saturating_sub(ranges.first_range);
    for pn in first_end..=ranges.largest {
        set.insert(pn);
    }
    let mut prev_largest = first_end;
    for (gap, len) in &ranges.extra_ranges {
        let next_largest = prev_largest.saturating_sub(*gap + 1);
        if next_largest >= *len {
            let start = next_largest.saturating_sub(*len - 1);
            for pn in start..=next_largest {
                set.insert(pn);
            }
        }
        if next_largest >= *len {
            prev_largest = next_largest.saturating_sub(*len);
        }
    }
    set
}
