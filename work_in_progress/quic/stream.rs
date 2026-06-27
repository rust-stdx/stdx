/// A QUIC stream (RFC 9000 §2-3).
pub struct Stream {
    pub id: u64,
    pub fin_sent: bool,
    pub fin_received: bool,
    pub send_buffer: Vec<u8>,
    pub recv_buffer: Vec<u8>,
    pub send_offset: u64,
    pub recv_offset: u64,
}

impl Stream {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            fin_sent: false,
            fin_received: false,
            send_buffer: Vec::new(),
            recv_buffer: Vec::new(),
            send_offset: 0,
            recv_offset: 0,
        }
    }

    pub fn write(&mut self, data: &[u8], fin: bool) {
        self.send_buffer.extend_from_slice(data);
        if fin {
            self.fin_sent = true;
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let n = self.recv_buffer.len().min(buf.len());
        buf[..n].copy_from_slice(&self.recv_buffer[..n]);
        self.recv_buffer.drain(..n);
        n
    }
}

/// Stream ID allocation (RFC 9000 §2.1).
/// Client-initiated bidirectional: 0, 4, 8, ...
/// Client-initiated unidirectional: 2, 6, 10, ...
pub struct StreamAllocator {
    next_bi: u64,
    next_uni: u64,
}

impl StreamAllocator {
    pub fn new() -> Self {
        Self {
            next_bi: 0,
            next_uni: 2,
        }
    }

    pub fn next_bi(&mut self) -> u64 {
        let id = self.next_bi;
        self.next_bi += 4;
        id
    }

    pub fn next_uni(&mut self) -> u64 {
        let id = self.next_uni;
        self.next_uni += 4;
        id
    }
}

/// Stream direction from the client's perspective.
pub enum StreamDir {
    Bi,
    Uni,
}
