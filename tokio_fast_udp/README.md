# tokio_fast_udp

Fast cross-platform UDP I/O for tokio with batch sends/recvs, GSO/GRO
segmentation offload, and ECN support.

## Why?

Modern operating systems offer UDP optimizations beyond the classic
`sendto`/`recvfrom`:

- **GSO** (Generic Segmentation Offload, Linux): send N packets via one
  `sendmsg` + `UDP_SEGMENT` cmsg. The kernel or NIC does the segmentation.
- **GRO** (Generic Receive Offload, Linux): receive multiple coalesced
  datagrams in one `recvmsg` + `UDP_GRO` cmsg.
- **ECN** (Explicit Congestion Notification): send/receive IP-level congestion
  marks via ancillary data — critical for QUIC/L4S.

`tokio_fast_udp` wraps these in a clean async API that automatically selects
the best available syscall matrix per platform. On platforms without
optimizations (macOS, Windows), a portable `sendmsg`/`recvmsg` fallback keeps
the same application code working.

## Quick start

```rust
use tokio_fast_udp::{FastUdpSocketBuilder, ReceiveItem, SendItem};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = FastUdpSocketBuilder::bind("127.0.0.1:9000".parse()?)
        .build()?;

    // Send a batch
    let dst = "127.0.0.1:9001".parse()?;
    let items = [
        SendItem::new(dst, b"hello"),
        SendItem::new(dst, b"world"),
    ];
    socket.send_many(&items).await?;

    // Receive a batch
    let mut bufs = vec![vec![0u8; 1500]; 32];
    let mut recv: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let n = socket.receive_many(&mut recv).await?;
    for i in 0..n {
        println!("{} bytes from {}", recv[i].len(), recv[i].source());
    }
    Ok(())
}
```

## GSO — send 64 packets in one syscall

```rust
use tokio_fast_udp::{FastUdpSocketBuilder, SendItem};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = FastUdpSocketBuilder::bind("0.0.0.0:0".parse()?).build()?;
    let seg = 1232; // typical QUIC max datagram
    let payload = vec![0u8; seg * 64]; // 64 QUIC packets back-to-back
    let dst = "93.184.216.34:443".parse()?;

    // One SendItem with segment_size → one sendmsg + UDP_SEGMENT
    let items = [SendItem::new(dst, &payload).segment_size(seg as u16)];
    socket.send_many(&items).await?;
    Ok(())
}
```

## Kill switches

Disable specific optimizations at runtime (no feature flags needed):

```rust
let socket = FastUdpSocketBuilder::bind(addr)
    .disable_gso()       // no segmentation offload on send
    .disable_gro()       // no coalescing on receive
    .disable_ecn()       // no ECN ancillary data
    .max_batch_size(64)
    .build()?;
```

## Platform support

| Platform | GSO | GRO | ECN | Batch send | Batch recv |
|----------|-----|-----|-----|------------|------------|
| Linux    | yes | yes | yes | sendmmsg | recv_gro |
| macOS    | —   | —   | yes | sendmsg loop | recvmsg loop |
| Windows  | —   | —   | —   | send_to loop | recv_from loop |

**Linux** is the primary fast path. GSO/GRO are probed at build time and
gracefully disabled if the kernel doesn't support them.

**macOS / other Unix** use `sendmsg`/`recvmsg` with ECN cmsg where supported.
`sendmsg_x`/`recvmsg_x` (undocumented) are not used.

**Windows** uses `tokio::net::UdpSocket` (IOCP-based). USO/URO are not enabled
in v1 due to driver stability issues; they will be added behind runtime
kill-switches when Microsoft fixes the known bugs.

## API

```rust
impl FastUdpSocket {
    pub async fn send(&self, item: SendItem<'_>) -> io::Result<()>;
    pub async fn send_many(&self, items: &[SendItem<'_>]) -> io::Result<usize>;
    pub async fn receive(&self, item: &mut ReceiveItem<'_>) -> io::Result<()>;
    pub async fn receive_many(&self, items: &mut [ReceiveItem<'_>]) -> io::Result<usize>;
    pub fn capabilities(&self) -> &Capabilities;
    pub fn local_addr(&self) -> io::Result<SocketAddr>;
}
```

The single-item methods are thin wrappers over the batch methods.

## Design

`tokio_fast_udp` uses tokio's `AsyncFd` for readiness notification and runs
the batch/offload syscalls synchronously inside the `async_io` closure. This
gives zero-copy, zero-allocation I/O: `SendItem` borrows the caller's `&[u8]`
directly — no channels, no background tasks, no buffer copies.

Cross-call batching (background task + channel) is planned as an opt-in mode
for fan-out workloads (DNS, syslog, metrics) where independent producers don't
naturally batch.
