/// Explicit Congestion Notification codepoint.
///
/// Represents the 2-bit ECN field in the IP header (ToS for IPv4, Traffic
/// Class for IPv6).
///
/// See [RFC 9331](https://datatracker.ietf.org/doc/rfc9331/) for the use of
/// ECN with QUIC and [RFC 3168](https://datatracker.ietf.org/doc/rfc3168/)
/// for the original specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ecn {
    /// Not ECN-Capable Transport — routers may drop the packet on congestion.
    NotEct,
    /// ECN-Capable Transport, codepoint 1.
    Ect1,
    /// ECN-Capable Transport, codepoint 0.
    Ect0,
    /// Congestion Experienced — set by a router on an ECT-marked packet.
    Ce,
}

impl Ecn {
    /// Encode the ECN codepoint as the low 2 bits of a TOS/Traffic Class byte.
    pub fn to_tos_bits(self) -> u8 {
        match self {
            Ecn::NotEct => 0b00,
            Ecn::Ect1 => 0b01,
            Ecn::Ect0 => 0b10,
            Ecn::Ce => 0b11,
        }
    }

    /// Decode the ECN codepoint from the low 2 bits of a TOS/Traffic Class byte.
    pub fn from_tos_bits(tos: u8) -> Self {
        match tos & 0b11 {
            0b00 => Ecn::NotEct,
            0b01 => Ecn::Ect1,
            0b10 => Ecn::Ect0,
            _ => Ecn::Ce,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        for ecn in [Ecn::NotEct, Ecn::Ect1, Ecn::Ect0, Ecn::Ce] {
            assert_eq!(Ecn::from_tos_bits(ecn.to_tos_bits()), ecn);
        }
    }

    #[test]
    fn test_masks_high_bits() {
        assert_eq!(Ecn::from_tos_bits(0b1010), Ecn::Ect0);
        assert_eq!(Ecn::from_tos_bits(0b1111), Ecn::Ce);
    }
}
