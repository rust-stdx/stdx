//! Fast Unix UDP I/O for tokio.
//!
//! Provides async UDP sockets that leverage modern OS syscalls for high
//! throughput:
//!
//! - **GSO** (Generic Segmentation Offload) on Linux: send a single large
//!   buffer that the kernel/NIC splits into multiple UDP datagrams.
//! - **GRO** (Generic Receive Offload) on Linux: receive multiple coalesced
//!   datagrams in one syscall.
//! - **ECN** (Explicit Congestion Notification): send and receive IP-level
//!   congestion marks.
//! - **SO_REUSEPORT** (Linux): bind multiple sockets to the same address:port
//!   for kernel-level load balancing across processes/threads.
//!
//! On platforms without these optimizations (macOS, iOS, etc.) a
//! portable `sendmsg`/`recvmsg` fallback is used so the same application code
//! compiles and runs across Unix targets.
//!
//! # Quick start
//!
//! ```no_run
//! use tokio_fast_udp::{FastUdpSocket, ReceiveItem, SendItem};
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let socket = FastUdpSocket::build("127.0.0.1:9000".parse().unwrap())
//!         .bind()?;
//!
//!     let dst = "127.0.0.1:9001".parse().unwrap();
//!     let items = [SendItem::new(dst, b"hello")];
//!     socket.send_many(&items).await?;
//!
//!     let mut buf = vec![0u8; 1500];
//!     let mut recv = [ReceiveItem::new(&mut buf)];
//!     socket.receive_many(&mut recv).await?;
//!     println!("got {} bytes", recv[0].len());
//!     Ok(())
//! }
//! ```
//!
//! # Sending many datagrams in one call
//!
//! On Linux, a single [`SendItem`] with a segment size is sent as one
//! `sendmsg` plus `UDP_SEGMENT`, and the kernel or NIC performs the
//! segmentation (GSO):
//!
//! ```no_run
//! use tokio_fast_udp::{FastUdpSocket, SendItem};
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let socket = FastUdpSocket::build("0.0.0.0:0".parse().unwrap()).bind()?;
//!     let segment_size = 1232; // a typical QUIC max datagram
//!     let payload = vec![0u8; segment_size * 64]; // 64 datagrams back-to-back
//!     let dst = "127.0.0.1:9001".parse().unwrap();
//!
//!     let items = [SendItem::new(dst, &payload).segment_size(segment_size as u16)];
//!     socket.send_many(&items).await?;
//!     Ok(())
//! }
//! ```
//!
//! # SO_REUSEPORT
//!
//! On Linux, multiple sockets can bind the same address and port, with the
//! kernel distributing incoming datagrams across them:
//!
//! ```no_run
//! use tokio_fast_udp::FastUdpSocket;
//!
//! fn main() -> std::io::Result<()> {
//!     let socket = FastUdpSocket::build("0.0.0.0:9000".parse().unwrap())
//!         .reuse_port(true)
//!         .bind()?;
//!     Ok(())
//! }
//! ```
//!
//! `SO_REUSEPORT` is a no-op on platforms that don't support it.
//!
//! # Kill switches
//!
//! On Unix, each optimization can be turned on or off through the builder,
//! without feature flags:
//!
//! ```no_run
//! use tokio_fast_udp::FastUdpSocket;
//!
//! fn main() -> std::io::Result<()> {
//!     let socket = FastUdpSocket::build("0.0.0.0:9000".parse().unwrap())
//!         .gso(false) // no segmentation offload on send
//!         .gro(false) // no coalescing on receive
//!         .ecn(false) // no ECN ancillary data
//!         .max_batch_size(64)
//!         .bind()?;
//!     Ok(())
//! }
//! ```
//!
//! # Platform support
//!
//! | Platform | GSO | GRO | ECN | ReusePort | Batch send | Batch recv |
//! |----------|-----|-----|-----|-----------|------------|------------|
//! | Linux    | yes | yes | yes | yes       | sendmmsg | recv_gro |
//! | macOS    | —   | —   | yes | yes       | sendmsg loop | recvmsg loop |
//! | iOS      | —   | —   | yes | yes       | sendmsg loop | recvmsg loop |
//!
//! **Linux** is the primary fast path. GSO and GRO are probed at build time and
//! gracefully disabled if the kernel doesn't support them.
//!
//! **macOS, iOS and other Unix** platforms use `sendmsg`/`recvmsg` with ECN
//! ancillary data where supported.
//!
//! # API
//!
//! ```text
//! impl FastUdpSocket {
//!     pub fn build(addr: SocketAddr) -> FastUdpSocketBuilder;
//!     pub async fn send(&self, item: SendItem<'_>) -> io::Result<()>;
//!     pub async fn send_many(&self, items: &[SendItem<'_>]) -> io::Result<usize>;
//!     pub async fn receive(&self, item: &mut ReceiveItem<'_>) -> io::Result<()>;
//!     pub async fn receive_many(&self, items: &mut [ReceiveItem<'_>]) -> io::Result<usize>;
//!     pub fn capabilities(&self) -> &Capabilities;
//!     pub fn local_addr(&self) -> io::Result<SocketAddr>;
//! }
//! ```
//!
//! The single-item methods are thin wrappers over the batch methods.
//!
//! # Design
//!
//! The crate uses tokio's `AsyncFd` for readiness notification and runs the
//! batch and offload syscalls synchronously inside the `async_io` closure. This
//! gives zero-copy, zero-allocation I/O: [`SendItem`] borrows the caller's
//! `&[u8]` directly, with no channels, background tasks, or buffer copies.

mod capability;
mod ecn;
mod item;
mod socket;

pub use capability::Capabilities;
pub use ecn::Ecn;
pub use item::{ReceiveItem, SendItem};
pub use socket::{FastUdpSocket, FastUdpSocketBuilder};

#[cfg(test)]
mod tests;
