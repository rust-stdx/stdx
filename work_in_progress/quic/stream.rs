/// A QUIC stream (RFC 9000 §2-3) with flow control tracking.
pub struct Stream {
    pub id: u64,
    /// Whether we have sent a FIN.
    pub fin_sent: bool,
    /// Whether we have received a FIN from the peer.
    pub fin_received: bool,
    /// Unsent data waiting to be transmitted (bytes from write() that haven't been sent yet).
    pub send_buffer: Vec<u8>,
    /// Byte offset of the next byte to send (i.e. total bytes sent so far on this stream).
    pub send_offset: u64,
    /// Received data buffer (for the application to read).
    pub recv_buffer: Vec<u8>,
    /// Total bytes received on this stream (offset of next expected byte from peer).
    pub recv_offset: u64,
    /// Maximum offset the peer has allowed us to send on this stream.
    pub max_stream_data: u64,
    /// Local flow control limit for incoming data on this stream (from config).
    pub local_max_stream_data: u64,
    /// Whether the stream needs a MAX_STREAM_DATA update sent to the peer.
    pub needs_max_stream_data: bool,
}

impl Stream {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            fin_sent: false,
            fin_received: false,
            send_buffer: Vec::new(),
            send_offset: 0,
            recv_buffer: Vec::new(),
            recv_offset: 0,
            max_stream_data: 0,
            local_max_stream_data: 0,
            needs_max_stream_data: false,
        }
    }

    /// Queue data for sending. Does not send anything.
    pub fn write(&mut self, data: &[u8], fin: bool) {
        self.send_buffer.extend_from_slice(data);
        if fin {
            self.fin_sent = true;
        }
    }

    /// How many bytes are available to send (up to flow control limits).
    pub fn sendable(&self) -> usize {
        let remaining = self.send_buffer.len();
        let credit = self.max_stream_data.saturating_sub(self.send_offset) as usize;
        remaining.min(credit)
    }

    /// Read received data into a buffer. Returns bytes read.
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let n = self.recv_buffer.len().min(buf.len());
        buf[..n].copy_from_slice(&self.recv_buffer[..n]);
        self.recv_buffer.drain(..n);
        n
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Stream ID allocation (RFC 9000 §2.1).
///
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamDir {
    Bi,
    Uni,
}

/// Connection-level flow controller for outgoing data.
pub struct SendFlowController {
    /// Maximum total data the peer has allowed us to send (from initial_max_data or MAX_DATA frames).
    pub max_data: u64,
    /// Total bytes sent across all streams.
    pub bytes_sent: u64,
    /// Whether we should send a DATA_BLOCKED frame.
    pub blocked: bool,
}

impl SendFlowController {
    pub fn new(max_data: u64) -> Self {
        Self {
            max_data,
            bytes_sent: 0,
            blocked: false,
        }
    }

    /// How many more connection-level bytes we can send.
    pub fn available(&self) -> u64 {
        self.max_data.saturating_sub(self.bytes_sent)
    }

    /// Record that `n` bytes were sent at the connection level.
    pub fn on_sent(&mut self, n: u64) {
        self.bytes_sent += n;
    }
}

/// Connection-level flow controller for incoming data.
pub struct RecvFlowController {
    /// Maximum data we will allow the peer to send (from config).
    pub local_max_data: u64,
    /// Total bytes received across all streams.
    pub bytes_received: u64,
    /// Threshold at which we send a MAX_DATA update (when consumed reaches half).
    pub update_threshold: u64,
    /// Whether we need to send a MAX_DATA frame.
    pub needs_max_data_update: bool,
}

impl RecvFlowController {
    pub fn new(local_max_data: u64) -> Self {
        Self {
            local_max_data,
            bytes_received: 0,
            update_threshold: local_max_data / 2,
            needs_max_data_update: false,
        }
    }

    /// Record that `n` bytes were received.
    pub fn on_received(&mut self, n: u64) {
        self.bytes_received += n;
        let consumed = self.local_max_data.saturating_sub(self.bytes_received);
        if consumed <= self.update_threshold {
            self.local_max_data += self.update_threshold;
            self.needs_max_data_update = true;
        }
    }
}

use crate::{
    cmd_queue::{CmdReceiver, CmdSender},
    error::Error,
};

pub(crate) struct ReceiveChunk {
    pub data: Vec<u8>,
    pub fin: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum StreamCommandKind {
    Send { data: Vec<u8>, fin: bool },
    Finish,
    Reset(u64),
    StopSending(u64),
}

#[derive(Debug, Clone)]
pub(crate) struct StreamCommand {
    pub stream_id: u64,
    pub kind: StreamCommandKind,
}

/// The sending half of a QUIC stream.
///
/// Created by [`Connection::open_unidirectional_stream`] or as part of [`Connection::open_bidirectional_stream`].
///
/// Data queued via [`send`](SendStream::send) is transmitted by the connection
/// on its next I/O cycle.
pub struct SendStream {
    pub(crate) id: u64,
    pub(crate) cmd_tx: CmdSender<StreamCommand>,
    pub(crate) fin_sent: bool,
}

impl SendStream {
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Queue data for sending on this stream.
    ///
    /// The data is not sent immediately; the connection will process it
    /// during its next I/O cycle (e.g. on the next call to `receive_one`
    /// or any stream-receiving operation).
    ///
    /// If `fin` is `true`, no further data can be sent on this stream.
    pub fn send(&mut self, data: &[u8], fin: bool) -> Result<(), Error> {
        if self.fin_sent {
            return Err(Error::InvalidState("stream already finished".into()));
        }
        if fin {
            self.fin_sent = true;
        }
        self.cmd_tx.push(StreamCommand {
            stream_id: self.id,
            kind: StreamCommandKind::Send {
                data: data.to_vec(),
                fin,
            },
        });
        Ok(())
    }

    /// Signal the end of the stream (FIN) without attaching data.
    pub fn finish(&mut self) -> Result<(), Error> {
        if self.fin_sent {
            return Err(Error::InvalidState("stream already finished".into()));
        }
        self.fin_sent = true;
        self.cmd_tx.push(StreamCommand {
            stream_id: self.id,
            kind: StreamCommandKind::Finish,
        });
        Ok(())
    }

    /// Abruptly terminate the sending side of this stream with an
    /// application error code (RESET_STREAM).
    pub fn reset(&mut self, error_code: u64) -> Result<(), Error> {
        self.fin_sent = true;
        self.cmd_tx.push(StreamCommand {
            stream_id: self.id,
            kind: StreamCommandKind::Reset(error_code),
        });
        Ok(())
    }
}

/// The receiving half of a QUIC stream.
///
/// Created by [`Connection::accept_unidirectional_stream`], [`Connection::accept_bidirectional_stream`],
/// or as part of [`Connection::open_bidirectional_stream`].
///
/// Uses an internal buffer so that partial reads do not discard data.
pub struct ReceiveStream {
    pub(crate) id: u64,
    pub(crate) cmd_tx: CmdSender<StreamCommand>,
    pub(crate) data_rx: CmdReceiver<ReceiveChunk>,
    pub(crate) pending: Vec<u8>,
    pub(crate) fin_received: bool,
}

impl ReceiveStream {
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Receive data from this stream. Blocks until data is available,
    /// the stream is finished by the peer, or the connection closes.
    ///
    /// Returns `Ok(Some(n))` when `n` bytes have been copied into `buf`.
    /// Returns `Ok(None)` when the peer has finished sending and all data
    /// has been consumed.
    pub async fn receive(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Error> {
        if !self.pending.is_empty() {
            let n = self.pending.len().min(buf.len());
            buf[..n].copy_from_slice(&self.pending[..n]);
            self.pending.drain(..n);
            return Ok(Some(n));
        }
        if self.fin_received {
            return Ok(None);
        }
        match self.data_rx.recv().await {
            Some(chunk) => {
                if chunk.fin {
                    self.fin_received = true;
                }
                let n = chunk.data.len().min(buf.len());
                buf[..n].copy_from_slice(&chunk.data[..n]);
                self.pending = chunk.data[n..].to_vec();
                Ok(Some(n))
            }
            None => Ok(None),
        }
    }

    /// Request the peer to stop sending on this stream (STOP_SENDING).
    pub fn stop(&mut self, error_code: u64) -> Result<(), Error> {
        self.cmd_tx.push(StreamCommand {
            stream_id: self.id,
            kind: StreamCommandKind::StopSending(error_code),
        });
        Ok(())
    }
}
