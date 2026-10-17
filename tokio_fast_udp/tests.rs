use std::net::SocketAddr;

use crate::{Ecn, FastUdpSocket, ReceiveItem, SendItem};

fn loop_addr(port: u16) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port))
}

fn loop_addr_v6(port: u16) -> SocketAddr {
    SocketAddr::from((std::net::Ipv6Addr::LOCALHOST, port))
}

#[tokio::test]
async fn test_send_receive_single() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();

    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let msg = b"hello world";
    sender.send(SendItem::new(raddr, msg)).await.unwrap();

    let mut buf = vec![0u8; 1500];
    let mut item = ReceiveItem::new(&mut buf);
    receiver.receive(&mut item).await.unwrap();

    assert_eq!(item.len(), msg.len());
    assert_eq!(item.data(), msg);
    assert_eq!(item.source(), sender.local_addr().unwrap());
}

#[tokio::test]
async fn test_send_many_receive_many() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let messages: Vec<&[u8]> = vec![b"one", b"two", b"three", b"four"];
    let items: Vec<SendItem> = messages.iter().map(|m| SendItem::new(raddr, m)).collect();
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..8).map(|_| vec![0u8; 1500]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let mut total = 0;
    while total < messages.len() {
        let n = receiver.receive_many(&mut recv_items[total..]).await.unwrap();
        total += n;
    }

    assert_eq!(total, messages.len());
    for i in 0..messages.len() {
        assert_eq!(recv_items[i].data(), messages[i]);
    }
}

#[tokio::test]
async fn test_empty_batch() {
    let socket = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let n = socket.send_many(&[]).await.unwrap();
    assert_eq!(n, 0);

    let n = socket.receive_many(&mut []).await.unwrap();
    assert_eq!(n, 0);
}

#[tokio::test]
async fn test_capabilities() {
    let socket = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let caps = socket.capabilities();
    assert!(caps.max_batch > 0);
}

#[tokio::test]
async fn test_disable_gso() {
    let socket = FastUdpSocket::build(loop_addr(0)).gso(false).bind().unwrap();
    assert!(!socket.capabilities().gso);
}

#[tokio::test]
async fn test_disable_gro() {
    let socket = FastUdpSocket::build(loop_addr(0)).gro(false).bind().unwrap();
    assert!(!socket.capabilities().gro);
}

#[tokio::test]
async fn test_disable_ecn() {
    let socket = FastUdpSocket::build(loop_addr(0)).ecn(false).bind().unwrap();
    assert!(!socket.capabilities().ecn);
}

#[tokio::test]
async fn test_disable_sendmmsg() {
    let socket = FastUdpSocket::build(loop_addr(0)).sendmmsg(false).bind().unwrap();
    assert!(!socket.capabilities().sendmmsg);
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn test_sendmmsg_batch() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    if !sender.capabilities().sendmmsg {
        eprintln!("sendmmsg not supported on this kernel — skipping");
        return;
    }

    let messages: Vec<&[u8]> = vec![b"one", b"two", b"three", b"four"];
    let items: Vec<SendItem> = messages.iter().map(|m| SendItem::new(raddr, m)).collect();
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..8).map(|_| vec![0u8; 1500]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let mut total = 0;
    while total < messages.len() {
        let n = receiver.receive_many(&mut recv_items[total..]).await.unwrap();
        total += n;
    }

    assert_eq!(total, messages.len());
    for i in 0..messages.len() {
        assert_eq!(recv_items[i].data(), messages[i]);
    }
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn test_gso_send() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    if !sender.capabilities().gso {
        eprintln!("GSO not supported on this kernel — skipping");
        return;
    }

    let seg = 10usize;
    let count = 4;
    let payload = vec![0xABu8; seg * count];

    let items = [SendItem::new(raddr, &payload).segment_size(seg as u16)];
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..count).map(|_| vec![0u8; 1500]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let mut total = 0;
    while total < count {
        let n = receiver.receive_many(&mut recv_items[total..]).await.unwrap();
        total += n;
    }

    assert_eq!(total, count);
    for i in 0..count {
        assert_eq!(recv_items[i].len(), seg);
        assert_eq!(&recv_items[i].data()[..seg], &payload[..seg]);
    }
}

#[tokio::test]
async fn test_ecn_send_receive() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    if !sender.capabilities().ecn {
        eprintln!("ECN not supported on this platform — skipping");
        return;
    }

    let msg = b"ecn test";
    sender.send(SendItem::new(raddr, msg).ecn(Ecn::Ect0)).await.unwrap();

    let mut buf = vec![0u8; 1500];
    let mut item = ReceiveItem::new(&mut buf);
    receiver.receive(&mut item).await.unwrap();

    assert_eq!(item.data(), msg);

    // Loopback preserves ECN marks on Linux
    #[cfg(target_os = "linux")]
    assert_eq!(item.ecn(), Some(Ecn::Ect0));
}

#[tokio::test]
async fn test_large_payload() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let msg = vec![0x42u8; 1400];
    sender.send(SendItem::new(raddr, &msg)).await.unwrap();

    let mut buf = vec![0u8; 2048];
    let mut item = ReceiveItem::new(&mut buf);
    receiver.receive(&mut item).await.unwrap();

    assert_eq!(item.len(), msg.len());
    assert_eq!(item.data(), &msg[..]);
}

#[tokio::test]
async fn test_local_addr_matches_bind() {
    let socket = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let addr = socket.local_addr().unwrap();

    assert_eq!(addr.ip(), std::net::Ipv4Addr::LOCALHOST);
    assert_ne!(addr.port(), 0);
}

#[tokio::test]
async fn test_zero_length_datagram() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    sender.send(SendItem::new(raddr, b"")).await.unwrap();

    let mut buf = vec![0u8; 64];
    let mut item = ReceiveItem::new(&mut buf);
    receiver.receive(&mut item).await.unwrap();

    assert!(item.is_empty());
    assert_eq!(item.len(), 0);
    assert_eq!(item.source(), sender.local_addr().unwrap());
}

#[tokio::test]
async fn test_source_addr_v4() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let saddr = sender.local_addr().unwrap();

    sender.send(SendItem::new(raddr, b"whoami")).await.unwrap();

    let mut buf = vec![0u8; 64];
    let mut item = ReceiveItem::new(&mut buf);
    receiver.receive(&mut item).await.unwrap();

    assert_eq!(item.source(), saddr);
    assert!(matches!(item.source(), SocketAddr::V4(_)));
}

#[tokio::test]
async fn test_many_datagrams_roundtrip() {
    const COUNT: usize = 64;

    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let payloads: Vec<Vec<u8>> = (0..COUNT).map(|i| vec![i as u8; 32]).collect();
    let items: Vec<SendItem> = payloads.iter().map(|p| SendItem::new(raddr, p)).collect();
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..COUNT).map(|_| vec![0u8; 128]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let mut total = 0;
    while total < COUNT {
        total += receiver.receive_many(&mut recv_items[total..]).await.unwrap();
    }

    assert_eq!(total, COUNT);
    for (i, item) in recv_items[..total].iter().enumerate() {
        assert_eq!(item.data(), payloads[i].as_slice(), "datagram {i} out of order");
    }
}

/// GSO disabled: the send path must fall back to manually chunking the payload
/// into one datagram per segment.
#[tokio::test]
async fn test_gso_fallback_manual_chunking() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).gso(false).bind().unwrap();

    assert!(!sender.capabilities().gso);

    let seg = 10usize;
    let count = 4;
    let payload = vec![0xCDu8; seg * count];

    let items = [SendItem::new(raddr, &payload).segment_size(seg as u16)];
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..count).map(|_| vec![0u8; 128]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let mut total = 0;
    while total < count {
        let n = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            receiver.receive_many(&mut recv_items[total..]),
        )
        .await
        .expect("timed out waiting for manually chunked datagrams")
        .unwrap();
        total += n;
    }

    assert_eq!(total, count);
    for item in &recv_items[..total] {
        assert_eq!(item.len(), seg);
        assert_eq!(item.data(), &payload[..seg]);
    }
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn test_gso_multi_item_batch() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    if !sender.capabilities().gso {
        eprintln!("GSO not supported on this kernel — skipping");
        return;
    }

    let seg = 8usize;
    let count = 3;
    let first = vec![0x11u8; seg * count];
    let second = vec![0x22u8; seg * count];

    let items = [
        SendItem::new(raddr, &first).segment_size(seg as u16),
        SendItem::new(raddr, &second).segment_size(seg as u16),
    ];
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..(count * 2)).map(|_| vec![0u8; 128]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let expected = count * 2;
    let mut total = 0;
    while total < expected {
        total += receiver.receive_many(&mut recv_items[total..]).await.unwrap();
    }

    assert_eq!(total, expected);
    for item in &recv_items[..count] {
        assert_eq!(item.data(), &first[..seg]);
    }
    for item in &recv_items[count..expected] {
        assert_eq!(item.data(), &second[..seg]);
    }
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn test_gso_with_ecn() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    if !sender.capabilities().gso {
        eprintln!("GSO not supported on this kernel — skipping");
        return;
    }
    if !sender.capabilities().ecn {
        eprintln!("ECN not supported on this platform — skipping");
        return;
    }

    let seg = 12usize;
    let count = 3;
    let payload = vec![0x5Au8; seg * count];

    let items = [SendItem::new(raddr, &payload).segment_size(seg as u16).ecn(Ecn::Ect0)];
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..count).map(|_| vec![0u8; 128]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let mut total = 0;
    while total < count {
        total += receiver.receive_many(&mut recv_items[total..]).await.unwrap();
    }

    assert_eq!(total, count);
    for item in &recv_items[..total] {
        assert_eq!(item.data(), &payload[..seg]);
        assert_eq!(item.ecn(), Some(Ecn::Ect0));
    }
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn test_ecn_marks_roundtrip() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    if !sender.capabilities().ecn {
        eprintln!("ECN not supported on this platform — skipping");
        return;
    }

    for mark in [Ecn::Ect0, Ecn::Ect1] {
        let msg = b"marked";
        sender.send(SendItem::new(raddr, msg).ecn(mark)).await.unwrap();

        let mut buf = vec![0u8; 64];
        let mut item = ReceiveItem::new(&mut buf);
        receiver.receive(&mut item).await.unwrap();

        assert_eq!(item.data(), msg);
        assert_eq!(item.ecn(), Some(mark));
    }
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn test_gro_batch_receive() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    if !receiver.capabilities().gro {
        eprintln!("GRO not supported on this kernel — skipping");
        return;
    }

    let payloads: Vec<Vec<u8>> = (0..8u8).map(|i| vec![i; 16]).collect();
    let items: Vec<SendItem> = payloads.iter().map(|p| SendItem::new(raddr, p)).collect();
    sender.send_many(&items).await.unwrap();

    let expected: Vec<u8> = payloads.iter().flatten().copied().collect();

    let mut bufs: Vec<Vec<u8>> = (0..16).map(|_| vec![0u8; 128]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    // GRO coalesces back-to-back datagrams into fewer receive slots; the
    // concatenated payload must still equal the sent byte stream. Datagrams
    // may arrive across several reads.
    let received = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        let mut received = Vec::with_capacity(expected.len());
        while received.len() < expected.len() {
            let count = receiver.receive_many(&mut recv_items).await.unwrap();
            for item in &recv_items[..count] {
                received.extend_from_slice(item.data());
            }
        }
        received
    })
    .await
    .expect("timed out while receiving the GRO batch");

    assert_eq!(received, expected);
}

#[tokio::test]
async fn test_ipv6_roundtrip() {
    let receiver = match FastUdpSocket::build(loop_addr_v6(0)).bind() {
        Ok(socket) => socket,
        Err(_) => {
            eprintln!("IPv6 loopback unavailable — skipping");
            return;
        }
    };
    let raddr = receiver.local_addr().unwrap();
    assert!(raddr.is_ipv6());

    let sender = FastUdpSocket::build(loop_addr_v6(0)).bind().unwrap();
    let saddr = sender.local_addr().unwrap();

    let msg = b"ipv6 hello";
    sender.send(SendItem::new(raddr, msg)).await.unwrap();

    let mut buf = vec![0u8; 64];
    let mut item = ReceiveItem::new(&mut buf);
    receiver.receive(&mut item).await.unwrap();

    assert_eq!(item.data(), msg);
    assert!(matches!(item.source(), SocketAddr::V6(_)));
    assert_eq!(item.source(), saddr);
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn test_ipv6_ecn() {
    let receiver = match FastUdpSocket::build(loop_addr_v6(0)).bind() {
        Ok(socket) => socket,
        Err(_) => {
            eprintln!("IPv6 loopback unavailable — skipping");
            return;
        }
    };
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr_v6(0)).bind().unwrap();

    if !sender.capabilities().ecn {
        eprintln!("ECN not supported on this platform — skipping");
        return;
    }

    let msg = b"v6 ecn";
    sender.send(SendItem::new(raddr, msg).ecn(Ecn::Ect0)).await.unwrap();

    let mut buf = vec![0u8; 64];
    let mut item = ReceiveItem::new(&mut buf);
    receiver.receive(&mut item).await.unwrap();

    assert_eq!(item.data(), msg);
    assert_eq!(item.ecn(), Some(Ecn::Ect0));
}

#[tokio::test]
async fn test_reuse_port_bind() {
    let first = FastUdpSocket::build(loop_addr(0)).reuse_port(true).bind().unwrap();
    let addr = first.local_addr().unwrap();

    // With SO_REUSEPORT set on both sockets, a second bind to the same address
    // and port must succeed.
    let second = FastUdpSocket::build(addr).reuse_port(true).bind();
    #[cfg(target_os = "linux")]
    assert!(second.is_ok(), "SO_REUSEPORT second bind failed: {:?}", second.as_ref().err());
    let _ = second;
}

/// When the payload is not a whole multiple of the segment size, the final
/// datagram must be the short remainder. This exercises the user-space
/// segmentation fallback (GSO is disabled on the sender and GRO on the
/// receiver so the datagrams are observed individually).
#[tokio::test]
async fn test_segment_size_short_final_segment() {
    let receiver = FastUdpSocket::build(loop_addr(0)).gro(false).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).gso(false).bind().unwrap();

    let seg = 16usize;
    let tail = 5usize;
    let payload = vec![0x7Fu8; seg * 2 + tail];

    let items = [SendItem::new(raddr, &payload).segment_size(seg as u16)];
    sender.send_many(&items).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..3).map(|_| vec![0u8; 128]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let mut total = 0;
    while total < 3 {
        let n = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            receiver.receive_many(&mut recv_items[total..]),
        )
        .await
        .expect("timed out waiting for segmented datagrams")
        .unwrap();
        total += n;
    }

    assert_eq!(total, 3);
    assert_eq!(recv_items[0].data(), &payload[..seg]);
    assert_eq!(recv_items[1].data(), &payload[seg..seg * 2]);
    assert_eq!(recv_items[2].len(), tail);
    assert_eq!(recv_items[2].data(), &payload[seg * 2..]);
}

/// A zero segment size is meaningless and must surface as an error instead of
/// panicking inside the send path.
#[tokio::test]
async fn test_zero_segment_size_is_error() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let err = sender
        .send(SendItem::new(raddr, b"payload").segment_size(0))
        .await
        .expect_err("a zero segment size must be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}
