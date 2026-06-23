//! Fast cross-platform UDP I/O for tokio.
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
//! On platforms without these optimizations (macOS, Windows, etc.) a
//! portable `sendmsg`/`recvmsg` fallback is used so the same application code
//! compiles and runs everywhere.
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
