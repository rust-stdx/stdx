/// Runtime-detected UDP optimization capabilities of a [`FastUdpSocket`].
///
/// All fields are determined at `build()` time by probing the kernel and
/// applying any kill-switches from [`FastUdpSocketBuilder`]. Call
/// [`FastUdpSocket::capabilities`] to inspect what the socket is using.
///
/// [`FastUdpSocket`]: crate::FastUdpSocket
/// [`FastUdpSocket::capabilities`]: crate::FastUdpSocket::capabilities
/// [`FastUdpSocketBuilder`]: crate::FastUdpSocketBuilder
#[derive(Debug, Clone)]
pub struct Capabilities {
    /// Generic Segmentation Offload — send N packets via one `sendmsg` with
    /// `UDP_SEGMENT`. Linux only.
    pub gso: bool,
    /// Generic Receive Offload — receive coalesced datagrams via `recvmsg`
    /// with `UDP_GRO`. Linux only.
    pub gro: bool,
    /// `sendmmsg` batch send — multiple UDP datagrams in a single syscall.
    /// Linux (and some other Unix) only.
    pub sendmmsg: bool,
    /// Explicit Congestion Notification — send/receive IP TOS/TCLASS ancillary
    /// data.
    pub ecn: bool,
    /// Maximum number of datagrams the socket will attempt in a single batch
    /// call.
    pub max_batch: usize,
}

impl Capabilities {
    /// No optimizations — portable single-datagram fallback on every path.
    pub fn none() -> Self {
        Capabilities {
            gso: false,
            gro: false,
            sendmmsg: false,
            ecn: false,
            max_batch: 1,
        }
    }
}
