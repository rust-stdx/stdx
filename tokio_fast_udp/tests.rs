use std::net::SocketAddr;

use crate::{Ecn, FastUdpSocket, ReceiveItem, SendItem};

fn loop_addr(port: u16) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port))
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
    let items: Vec<SendItem> = messages.iter().map(|m| SendItem::new(raddr, *m)).collect();
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
    let items: Vec<SendItem> = messages.iter().map(|m| SendItem::new(raddr, *m)).collect();
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
