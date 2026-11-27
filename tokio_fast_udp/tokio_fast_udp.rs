//! Fast Unix UDP I/O for tokio.
//!
//! Provides async UDP sockets that leverage modern OS syscalls for high
//! throughput:
//!
//! - **GSO** (Generic Segmentation Offload) on Linux: send a single large
//!   buffer that the kernel/NIC splits into multiple UDP datagrams.
//! - **GRO** (Generic Receive Offload) on Linux: receive multiple coalesced
//!   datagrams in one syscall. Opt-in, with strict receive-batch requirements
//!   (see [GRO (opt-in)](#gro-opt-in)).
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
//! segmentation (GSO). On platforms without GSO the payload is split into one
//! datagram per segment in user space, so the same code runs everywhere.
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
//!         .gro(false) // GRO is off by default; this is explicit
//!         .ecn(false) // no ECN ancillary data
//!         .max_batch_size(64)
//!         .bind()?;
//!     Ok(())
//! }
//! ```
//!
//! # GRO (opt-in)
//!
//! GRO is **disabled by default**. Enable it explicitly with
//! [`FastUdpSocketBuilder::gro(true)`] on Linux; on other kernels the
//! capability probe leaves [`Capabilities::gro`] `false` and the portable
//! receive loop is used, so enabling it is harmless.
//!
//! Because one `recvmsg` can return an aggregate of many coalesced datagrams,
//! a GRO-enabled socket imposes two rules:
//!
//! 1. [`FastUdpSocket::receive_many`] must be called with **at least 64
//!    receive items** (`UDP_GRO_CNT_MAX`) and the socket must have
//!    `max_batch_size >= 64`. Otherwise it returns
//!    [`std::io::ErrorKind::InvalidInput`] *before* issuing the syscall, so no
//!    datagram is consumed.
//! 2. [`FastUdpSocket::receive`] is unavailable while GRO is enabled: a single
//!    slot cannot hold an aggregate, so it returns
//!    [`std::io::ErrorKind::InvalidInput`] without dropping any segments. Use
//!    `receive_many` with 64+ slots, or disable GRO.
//!
//! The receive staging buffer is currently 64 KiB. A coalesced aggregate that
//! exceeds it is reported as [`std::io::ErrorKind::InvalidData`] (as is
//! malformed kernel segment metadata) instead of returning a truncated,
//! silently lossy batch.
//!
//! ```no_run
//! use tokio_fast_udp::{FastUdpSocket, ReceiveItem};
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let socket = FastUdpSocket::build("127.0.0.1:9000".parse().unwrap())
//!         .gro(true)
//!         .bind()?;
//!     if !socket.capabilities().gro {
//!         return Ok(()); // kernel does not support GRO
//!     }
//!
//!     let mut bufs = vec![vec![0u8; 1500]; 64];
//!     let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
//!     let n = socket.receive_many(&mut items).await?;
//!     println!("received {n} datagrams");
//!     Ok(())
//! }
//! ```
//!
//! [`FastUdpSocketBuilder::gro(true)`]: FastUdpSocketBuilder::gro
//! [`Capabilities::gro`]: Capabilities::gro
//!
//! # Platform support
//!
//! | Platform | GSO | GRO | ECN | ReusePort | Batch send | Batch recv |
//! |----------|-----|-----|-----|-----------|------------|------------|
//! | Linux    | yes | yes | yes | yes       | sendmmsg | recv_gro |
//! | macOS    | —   | —   | yes | yes       | sendmsg loop | recvmsg loop |
//! | iOS      | —   | —   | yes | yes       | sendmsg loop | recvmsg loop |
//!
//! **Linux** is the primary fast path. GSO is probed at build time and
//! gracefully disabled if the kernel doesn't support it. GRO is probed only
//! when explicitly enabled and is otherwise off.
//!
//! **macOS, iOS and other Unix** platforms use `sendmsg`/`recvmsg` with ECN
//! ancillary data where supported. Segmented sends (`SendItem::segment_size`)
//! are emulated there by splitting the payload into one datagram per segment.
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
//! `send` is a thin wrapper over `send_many`. `receive` wraps `receive_many`
//! only while GRO is disabled; with GRO enabled it is rejected (see
//! [GRO (opt-in)](#gro-opt-in)).
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
