use std::net::SocketAddr;

use crate::Ecn;

/// A single outbound UDP datagram.
///
/// Build with [`SendItem::new`] and optionally chain [`ecn`] / [`segment_size`]
/// to attach ancillary data.
///
/// When [`segment_size`] is set and the platform supports it (Linux GSO), the
/// kernel will segment `data` into multiple datagrams of `segment_size` bytes
/// each (the final segment may be shorter). This allows sending N packets in
/// a single syscall.
///
/// [`ecn`]: SendItem::ecn
/// [`segment_size`]: SendItem::segment_size
#[derive(Clone, Copy)]
pub struct SendItem<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) destination: SocketAddr,
    pub(crate) ecn: Option<Ecn>,
    pub(crate) segment_size: Option<u16>,
}

impl<'a> SendItem<'a> {
    /// Create a datagram targeting `destination` with the given `data` payload.
    /// Address comes first, then data.
    pub fn new(destination: SocketAddr, data: &'a [u8]) -> Self {
        SendItem {
            data,
            destination,
            ecn: None,
            segment_size: None,
        }
    }

    /// Attach an ECN codepoint to the outgoing datagram.
    pub fn ecn(mut self, ecn: Ecn) -> Self {
        self.ecn = Some(ecn);
        self
    }

    /// Enable UDP segmentation offload (GSO on Linux). The kernel will split
    /// `data` into consecutive chunks of `segment_size` bytes and send each as
    /// a separate UDP datagram to the same `destination`.
    ///
    /// `data.len()` must be a multiple of `segment_size`, except for the final
    /// segment which may be shorter.
    pub fn segment_size(mut self, segment_size: u16) -> Self {
        self.segment_size = Some(segment_size);
        self
    }
}

/// A single inbound UDP datagram slot.
///
/// Create with [`ReceiveItem::new`] passing a buffer, then call
/// [`FastUdpSocket::receive_many`] to fill it. After a successful receive,
/// [`data`] / [`source`] / [`ecn`] / [`len`] return the received datagram's
/// metadata.
///
/// [`data`]: ReceiveItem::data
/// [`source`]: ReceiveItem::source
/// [`ecn`]: ReceiveItem::ecn
/// [`len`]: ReceiveItem::len
/// [`FastUdpSocket::receive_many`]: crate::FastUdpSocket::receive_many
pub struct ReceiveItem<'a> {
    pub(crate) buf: &'a mut [u8],
    pub(crate) source: SocketAddr,
    pub(crate) ecn: Option<Ecn>,
    pub(crate) len: usize,
}

impl<'a> ReceiveItem<'a> {
    /// Create an empty receive slot backed by `buf`.
    pub fn new(buf: &'a mut [u8]) -> Self {
        ReceiveItem {
            buf,
            source: SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0),
            ecn: None,
            len: 0,
        }
    }

    /// The received payload (first `len` bytes of the internal buffer).
    pub fn data(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    /// The source address of the received datagram.
    pub fn source(&self) -> SocketAddr {
        self.source
    }

    /// The ECN codepoint received with the datagram, if any.
    pub fn ecn(&self) -> Option<Ecn> {
        self.ecn
    }

    /// Number of bytes received.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether zero bytes were received.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
