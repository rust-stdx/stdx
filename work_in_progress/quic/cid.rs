/// A QUIC Connection ID (0-20 bytes, RFC 9000 §5.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConnectionId {
    bytes: heapless::Vec<u8, 20>,
}

impl ConnectionId {
    /// Create from a slice (panics if length > 20).
    pub fn new(data: &[u8]) -> Self {
        Self {
            bytes: data.try_into().expect("Connection ID length must be <= 20"),
        }
    }

    /// Generate a random CID of `len` bytes.
    pub fn random(len: usize) -> Self {
        assert!(len <= 20, "Connection ID length must be ≤ 20");
        let mut buf = heapless::Vec::new();
        crypto::random_fill(&mut buf);
        Self {
            bytes: buf,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl core::fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for b in &self.bytes {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}
