use alloc::boxed::Box;
use core::{net::SocketAddr, time::Duration};

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

    /// Read a datagram into `buf`.  Returns `(bytes_read, source_addr)`.
    ///
    /// If `deadline` is `Some`, the implementation must return
    /// `Err(IoError::TimedOut)` when the deadline is exceeded
    /// instead of blocking indefinitely.
    async fn receive_from(&self, buf: &mut [u8], deadline: Option<Duration>) -> Result<(usize, SocketAddr), IoError>;

    /// Local socket address.
    fn local_addr(&self) -> Result<SocketAddr, IoError>;

    /// Current monotonic timestamp.
    ///
    /// Used by the QUIC state machine for RTT measurement, loss
    /// detection, idle timeout, and ACK scheduling. The value must
    /// be monotonically non-decreasing across calls.
    ///
    /// The default implementation uses [`std::time::Instant`] when
    /// the `std` feature is available.
    fn now(&self) -> Instant;
}
