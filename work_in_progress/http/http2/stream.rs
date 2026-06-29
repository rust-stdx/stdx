use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamId(u32);

impl StreamId {
    pub const CONNECTION: StreamId = StreamId(0);

    pub fn new(id: u32) -> Option<Self> {
        if id & 0x8000_0000 != 0 {
            None
        } else {
            Some(StreamId(id))
        }
    }

    pub fn is_client_initiated(&self) -> bool {
        self.0 != 0 && self.0 % 2 == 1
    }

    pub fn is_server_initiated(&self) -> bool {
        self.0 != 0 && self.0 % 2 == 0
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn next_client(current: u32) -> u32 {
        current + 2
    }

    pub fn next_server(current: u32) -> u32 {
        current + 2
    }
}

impl From<u32> for StreamId {
    fn from(id: u32) -> Self {
        StreamId(id & 0x7FFF_FFFF)
    }
}

impl fmt::Display for StreamId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
    Idle,
    ReservedLocal,
    ReservedRemote,
    Open,
    HalfClosedLocal,
    HalfClosedRemote,
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_stream() {
        let s = StreamId::new(1).unwrap();
        assert!(s.is_client_initiated());
        assert!(!s.is_server_initiated());
    }

    #[test]
    fn test_server_stream() {
        let s = StreamId::new(2).unwrap();
        assert!(!s.is_client_initiated());
        assert!(s.is_server_initiated());
    }

    #[test]
    fn test_connection_stream() {
        let s = StreamId::CONNECTION;
        assert!(!s.is_client_initiated());
        assert!(!s.is_server_initiated());
    }

    #[test]
    fn test_invalid_stream() {
        assert!(StreamId::new(0x8000_0000).is_none());
    }

    #[test]
    fn test_next_ids() {
        assert_eq!(StreamId::next_client(1), 3);
        assert_eq!(StreamId::next_server(2), 4);
    }
}
