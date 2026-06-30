use alloc::boxed::Box;
use core::net::SocketAddr;

use async_trait::async_trait;

use crate::{error::IoError, instant::Instant};

/// Pluggable async datagram transport.
///
/// Implementations wrap any UDP socket (or other datagram transport)
/// so the QUIC layer never directly depends on a particular socket type.
///
/// # Lifetime
///
/// The transport is owned by [`Connection`](crate::Connection) and
/// driven by its async methods.  The same transport can be shared
/// across multiple connections via `Arc<T>`.
#[async_trait]
pub trait Transport: Send + Sync + 'static {
    /// Write a datagram to `dest`.
    async fn send_to(&self, dest: SocketAddr, data: &[u8]) -> Result<usize, IoError>;

    /// Read a datagram into `buf`. Returns `(bytes_read, source_addr)`.
    async fn receive_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), IoError>;

    /// Local socket address.
    fn local_addr(&self) -> Result<SocketAddr, IoError>;

    /// `now` must return the current monotonic timestamp in microseconds.
    ///
    /// Used by the QUIC state machine for RTT measurement, loss
    /// detection, idle timeout, and ACK scheduling. The value must
    /// be monotonically non-decreasing across calls.
    ///
    /// A good default is to return the current microsecond timestamp since the Unix epoch.
    fn now(&self) -> Instant;
}
