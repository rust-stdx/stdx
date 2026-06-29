use core::net::SocketAddr;
use std::{io, time::Duration};

use async_trait::async_trait;

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
    async fn send_to(&self, dest: SocketAddr, data: &[u8]) -> io::Result<usize>;

    /// Read a datagram into `buf`.  Returns `(bytes_read, source_addr)`.
    ///
    /// If `deadline` is `Some`, the implementation must return
    /// `Err(io::ErrorKind::TimedOut)` when the deadline is exceeded
    /// instead of blocking indefinitely.
    async fn receive_from(&self, buf: &mut [u8], deadline: Option<Duration>) -> io::Result<(usize, SocketAddr)>;

    /// Local socket address.
    fn local_addr(&self) -> io::Result<SocketAddr>;
}
