// Audit tests for the Unix `FastUdpSocket` implementation.
//
// This module is a child of `unix_socket`, so it can call the private
// `decode_cmsgs`, `CmsgBuf`, `RecvCmsgBuf`, `fill_sockaddr` and
// `storage_to_addr` helpers with production behaviour unchanged.

#![allow(clippy::needless_range_loop)]

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use super::*;
use crate::{Ecn, FastUdpSocket, ReceiveItem, SendItem};

// ---------------------------------------------------------------------------
// Synthetic control-message buffers
// ---------------------------------------------------------------------------

/// An exactly `N`-byte, 8-aligned heap buffer.
///
/// The alignment matches the strongest requirement of `libc::cmsghdr`
/// (`align_of::<cmsghdr>() == 8` on 64-bit Linux). Because the slice length is
/// exactly `N`, Miri reports any read past `N`, which is what we want when
/// probing `decode_cmsgs` with adversarial `cmsg_len` values.
#[repr(align(8))]
struct Ctrl<const N: usize>([u8; N]);

impl<const N: usize> Ctrl<N> {
    fn boxed() -> Box<Self> {
        Box::new(Ctrl([0u8; N]))
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

/// Write one well-formed control message at `offset`, returning its
/// `CMSG_SPACE` stride.
fn write_cmsg(buf: *mut u8, offset: usize, level: libc::c_int, ty: libc::c_int, payload: &[u8]) -> usize {
    let space = unsafe { libc::CMSG_SPACE(payload.len() as u32) as usize };
    unsafe {
        let header = buf.add(offset) as *mut libc::cmsghdr;
        (*header).cmsg_len = libc::CMSG_LEN(payload.len() as u32) as _;
        (*header).cmsg_level = level;
        (*header).cmsg_type = ty;
        std::ptr::copy_nonoverlapping(payload.as_ptr(), libc::CMSG_DATA(header), payload.len());
    }
    space
}

/// Overwrite only the header fields of the cmsg at `offset`.
fn write_cmsg_header(buf: *mut u8, offset: usize, level: libc::c_int, ty: libc::c_int, cmsg_len: usize) {
    unsafe {
        let header = buf.add(offset) as *mut libc::cmsghdr;
        (*header).cmsg_len = cmsg_len as _;
        (*header).cmsg_level = level;
        (*header).cmsg_type = ty;
    }
}

fn make_msghdr(control: *mut u8, controllen: usize, flags: libc::c_int) -> libc::msghdr {
    let mut hdr: libc::msghdr = unsafe { std::mem::zeroed() };
    hdr.msg_control = control as *mut libc::c_void;
    hdr.msg_controllen = controllen as _;
    hdr.msg_flags = flags;
    hdr
}

// ---------------------------------------------------------------------------
// decode_cmsgs: well-formed messages
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
#[test]
fn decode_gro_cmsg() {
    // CMSG_SPACE(4) == 24 on Linux: exact-size allocation.
    let mut ctrl = Ctrl::<24>::boxed();
    let used = write_cmsg(ctrl.as_mut_ptr(), 0, libc::SOL_UDP, libc::UDP_GRO, &1400i32.to_ne_bytes());
    assert_eq!(used, 24);
    let hdr = make_msghdr(ctrl.as_mut_ptr(), used, 0);

    let (gro, ecn) = decode_cmsgs(&hdr, false);
    assert_eq!(gro, Some(1400));
    assert_eq!(ecn, None);
}

#[cfg(target_os = "linux")]
#[test]
fn decode_ipv4_ecn_one_byte() {
    // Linux reports IP_TOS as a single byte on receive.
    let mut ctrl = Ctrl::<24>::boxed();
    let used = write_cmsg(ctrl.as_mut_ptr(), 0, libc::IPPROTO_IP, libc::IP_TOS, &[0b10]);
    assert_eq!(used, 24);
    let hdr = make_msghdr(ctrl.as_mut_ptr(), used, 0);

    let (gro, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(gro, None);
    assert_eq!(ecn, Some(Ecn::Ect0));

    // `IP_RECVTOS` is the name used by the BSDs/Apple; decoding must accept it.
    let mut ctrl = Ctrl::<24>::boxed();
    let used = write_cmsg(ctrl.as_mut_ptr(), 0, libc::IPPROTO_IP, libc::IP_RECVTOS, &[0b01]);
    let hdr = make_msghdr(ctrl.as_mut_ptr(), used, 0);
    let (_, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(ecn, Some(Ecn::Ect1));
}

#[cfg(target_os = "linux")]
#[test]
fn decode_ipv6_tclass_int_and_byte() {
    // Native representation: 4-byte int.
    let mut ctrl = Ctrl::<24>::boxed();
    let used = write_cmsg(
        ctrl.as_mut_ptr(),
        0,
        libc::IPPROTO_IPV6,
        libc::IPV6_TCLASS,
        &0b11i32.to_ne_bytes(),
    );
    let hdr = make_msghdr(ctrl.as_mut_ptr(), used, 0);
    let (_, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(ecn, Some(Ecn::Ce));

    // Apple bug 48761855: 1-byte payload.
    let mut ctrl = Ctrl::<24>::boxed();
    let used = write_cmsg(ctrl.as_mut_ptr(), 0, libc::IPPROTO_IPV6, libc::IPV6_TCLASS, &[0b10]);
    let hdr = make_msghdr(ctrl.as_mut_ptr(), used, 0);
    let (_, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(ecn, Some(Ecn::Ect0));
}

#[cfg(target_os = "linux")]
#[test]
fn decode_two_chained_cmsgs_with_padding() {
    // GRO uses 24 bytes (4 bytes payload + 4 bytes CMSG_ALIGN padding);
    // a 1-byte ECN cmsg also occupies 24 bytes, so the second header lives at
    // offset 24. The padding bytes between them stay zero.
    let mut ctrl = Ctrl::<48>::boxed();
    let first = write_cmsg(ctrl.as_mut_ptr(), 0, libc::SOL_UDP, libc::UDP_GRO, &1200i32.to_ne_bytes());
    let second = write_cmsg(ctrl.as_mut_ptr(), first, libc::IPPROTO_IP, libc::IP_RECVTOS, &[0b01]);
    assert_eq!((first, second), (24, 24));
    let hdr = make_msghdr(ctrl.as_mut_ptr(), first + second, 0);

    let (gro, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(gro, Some(1200));
    assert_eq!(ecn, Some(Ecn::Ect1));
}

#[cfg(target_os = "linux")]
#[test]
fn decode_ignores_unknown_levels_and_types() {
    let mut ctrl = Ctrl::<80>::boxed();
    let a = write_cmsg(ctrl.as_mut_ptr(), 0, libc::SOL_SOCKET, libc::SO_TIMESTAMP, &[0u8; 16]);
    let b = write_cmsg(ctrl.as_mut_ptr(), a, libc::SOL_UDP, libc::UDP_GRO + 100, &7i32.to_ne_bytes());
    let c = write_cmsg(
        ctrl.as_mut_ptr(),
        a + b,
        libc::IPPROTO_IPV6,
        libc::IPV6_TCLASS,
        &0b10i32.to_ne_bytes(),
    );
    let hdr = make_msghdr(ctrl.as_mut_ptr(), a + b + c, 0);
    let (gro, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(gro, None);
    assert_eq!(ecn, Some(Ecn::Ect0));
}

// ---------------------------------------------------------------------------
// decode_cmsgs: malformed / adversarial cmsg_len values
// ---------------------------------------------------------------------------

/// Iteration must terminate and must not read a payload when `cmsg_len` is
/// smaller than the header. `controllen` is the full CMSG_SPACE(4) == 24.
#[cfg(target_os = "linux")]
#[test]
fn decode_rejects_short_cmsg_len() {
    for len in [0usize, 1, 15, 16, 17, 19] {
        let mut ctrl = Ctrl::<24>::boxed();
        write_cmsg_header(ctrl.as_mut_ptr(), 0, libc::SOL_UDP, libc::UDP_GRO, len);
        let hdr = make_msghdr(ctrl.as_mut_ptr(), 24, 0);
        let (gro, ecn) = decode_cmsgs(&hdr, true);
        assert_eq!(gro, None, "cmsg_len={len}");
        assert_eq!(ecn, None, "cmsg_len={len}");
    }
}

/// A `cmsg_len` that claims a payload larger than `msg_controllen` must not be
/// trusted (finding F-1). The bounds check stops before reading the payload,
/// so this is safe under Miri even though the allocation is only 16 bytes.
#[cfg(target_os = "linux")]
#[test]
fn decode_huge_cmsg_len_is_rejected() {
    // Exactly one `cmsghdr` (16 bytes) and nothing else. `cmsg_len` claims a
    // four-byte payload; the decoder must reject it instead of reading
    // `read_unaligned::<c_int>(buf + 16)` past the allocation.
    let mut ctrl = Ctrl::<16>::boxed();
    write_cmsg_header(ctrl.as_mut_ptr(), 0, libc::SOL_UDP, libc::UDP_GRO, unsafe {
        libc::CMSG_LEN(4) as usize
    });
    let hdr = make_msghdr(ctrl.as_mut_ptr(), 16, 0);
    let (gro, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(gro, None);
    assert_eq!(ecn, None);
}

/// Same as above, but the "payload" would be the single byte read for IPv4 ECN.
#[cfg(target_os = "linux")]
#[test]
fn decode_ecn_payload_past_controllen_is_rejected() {
    let mut ctrl = Ctrl::<16>::boxed();
    write_cmsg_header(ctrl.as_mut_ptr(), 0, libc::IPPROTO_IP, libc::IP_TOS, unsafe {
        libc::CMSG_LEN(1) as usize
    });
    let hdr = make_msghdr(ctrl.as_mut_ptr(), 16, 0);
    let (_, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(ecn, None);
}

/// `MSG_CTRUNC` is not a reason to stop decoding a well-formed message: the
/// received control data is still bounded by `msg_controllen`.
#[cfg(target_os = "linux")]
#[test]
fn decode_ignores_msg_ctrunc() {
    let mut ctrl = Ctrl::<24>::boxed();
    let used = write_cmsg(ctrl.as_mut_ptr(), 0, libc::SOL_UDP, libc::UDP_GRO, &900i32.to_ne_bytes());
    let hdr = make_msghdr(ctrl.as_mut_ptr(), used, libc::MSG_CTRUNC);
    let (gro, _) = decode_cmsgs(&hdr, true);
    assert_eq!(gro, Some(900));
}

/// A `cmsg_len` near `usize::MAX` makes `CMSG_NXTHDR`'s `CMSG_ALIGN`
/// arithmetic wrap. The bounds check rejects it before advancing, so the loop
/// terminates for every build profile (finding F-2).
#[cfg(target_os = "linux")]
#[test]
fn decode_huge_cmsg_len_termination() {
    let mut ctrl = Ctrl::<24>::boxed();
    write_cmsg_header(ctrl.as_mut_ptr(), 0, libc::SOL_UDP, libc::UDP_GRO, usize::MAX);
    let hdr = make_msghdr(ctrl.as_mut_ptr(), 24, 0);
    let (gro, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(gro, None);
    assert_eq!(ecn, None);
}

/// Worst case at the production buffer size: the last possible `cmsghdr`
/// starts at offset 112 (the last 16 bytes of `RecvCmsgBuf`'s 128-byte array)
/// and claims a 4-byte payload. The decoder must not read bytes 128..132 (F-1
/// at the real buffer bound).
#[cfg(target_os = "linux")]
#[test]
fn decode_tail_payload_past_128_byte_buffer_is_rejected() {
    let mut ctrl = Ctrl::<128>::boxed();
    // First header: consume 112 bytes, no payload we care about.
    write_cmsg_header(ctrl.as_mut_ptr(), 0, libc::SOL_SOCKET, libc::SO_TIMESTAMP, 112);
    // Second header at the tail: CMSG_LEN(4) == 20 so the payload would start
    // at byte 128, immediately past the 128-byte buffer.
    write_cmsg_header(ctrl.as_mut_ptr(), 112, libc::SOL_UDP, libc::UDP_GRO, unsafe {
        libc::CMSG_LEN(4) as usize
    });
    let hdr = make_msghdr(ctrl.as_mut_ptr(), 128, 0);
    let (gro, ecn) = decode_cmsgs(&hdr, true);
    assert_eq!(gro, None);
    assert_eq!(ecn, None);
}

// ---------------------------------------------------------------------------
// Apple CMSG_NXTHDR progress
// ---------------------------------------------------------------------------

/// Reproduce the exact Apple/`libc` `CMSG_NXTHDR` formula and show that a
/// zeroed cmsg (cmsg_len == 0) yields the *same* pointer again. `decode_cmsgs`
/// compensates with its explicit `cmsg_len < CMSG_LEN(0)` check, which makes
/// the loop terminate (finding F-3).
#[test]
fn apple_nxthdr_zero_len_does_not_advance() {
    // __DARWIN_ALIGN32(p) = (p + 3) & !3
    fn align32(p: usize) -> usize {
        (p + 3) & !3
    }
    // Model of libc's Apple CMSG_NXTHDR.
    fn apple_next(cmsg: usize, control: usize, controllen: usize, cmsg_len: usize) -> usize {
        let next = cmsg + align32(cmsg_len);
        let max = control + controllen;
        if next + align32(std::mem::size_of::<libc::cmsghdr>()) > max {
            0
        } else {
            next
        }
    }

    let control = 0x1000usize;
    let controllen = 24usize;
    // A zeroed cmsg, as produced by macOS < 14 according to quinn-udp.
    let next = apple_next(control, control, controllen, 0);
    assert_eq!(next, control, "Apple CMSG_NXTHDR re-returns the same cmsg");

    // A loop of the shape used by `decode_cmsgs` never advances without the
    // explicit length check; document the underlying libc behaviour.
    let mut cmsg = control;
    let mut iterations = 0usize;
    while cmsg != 0 {
        cmsg = apple_next(cmsg, control, controllen, 0);
        iterations += 1;
        if iterations > 1_000 {
            break;
        }
    }
    assert!(iterations > 1_000, "libc's Apple CMSG_NXTHDR does not advance on len 0");
}

// ---------------------------------------------------------------------------
// fill_sockaddr / storage_to_addr
// ---------------------------------------------------------------------------

fn v4(ip: [u8; 4], port: u16) -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(ip), port))
}

fn v6(ip: Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> SocketAddr {
    SocketAddr::V6(SocketAddrV6::new(ip, port, flowinfo, scope_id))
}

#[test]
fn sockaddr_v4_round_trip_and_layout() {
    let ips = [
        [0, 0, 0, 0],
        [127, 0, 0, 1],
        [192, 168, 1, 2],
        [255, 255, 255, 255],
        [1, 2, 3, 4],
    ];
    for ip in ips {
        for port in [0u16, 1, 8080, u16::MAX] {
            let addr = v4(ip, port);
            let mut ss: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
            let len = fill_sockaddr(&addr, &mut ss);
            assert_eq!(len as usize, std::mem::size_of::<libc::sockaddr_in>());
            assert_eq!(ss.ss_family as libc::c_int, libc::AF_INET);

            let sin = unsafe { &*(&ss as *const _ as *const libc::sockaddr_in) };
            assert_eq!(sin.sin_family as libc::c_int, libc::AF_INET);
            assert_eq!(u16::from_be(sin.sin_port), port);
            assert_eq!(sin.sin_addr.s_addr.to_ne_bytes(), ip);
            assert_eq!(sin.sin_zero, [0u8; 8]);

            assert_eq!(storage_to_addr(&ss), addr);
        }
    }
}

#[test]
fn sockaddr_v6_round_trip_and_layout() {
    let ips = [
        Ipv6Addr::LOCALHOST,
        Ipv6Addr::UNSPECIFIED,
        Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
        Ipv6Addr::new(0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff),
        // IPv4-mapped.
        Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc000, 0x0201),
    ];
    for ip in ips {
        for port in [0u16, 1, 443, u16::MAX] {
            for flowinfo in [0u32, 1, 0x000f_ffff, u32::MAX] {
                for scope_id in [0u32, 1, 42, u32::MAX] {
                    let addr = v6(ip, port, flowinfo, scope_id);
                    let mut ss: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
                    let len = fill_sockaddr(&addr, &mut ss);
                    assert_eq!(len as usize, std::mem::size_of::<libc::sockaddr_in6>());
                    assert_eq!(ss.ss_family as libc::c_int, libc::AF_INET6);

                    let sin6 = unsafe { &*(&ss as *const _ as *const libc::sockaddr_in6) };
                    assert_eq!(sin6.sin6_family as libc::c_int, libc::AF_INET6);
                    assert_eq!(u16::from_be(sin6.sin6_port), port);
                    assert_eq!(sin6.sin6_addr.s6_addr, ip.octets());
                    assert_eq!(sin6.sin6_scope_id, scope_id);

                    // Self-consistency: the crate's own encode/decode are inverses.
                    assert_eq!(storage_to_addr(&ss), addr);
                }
            }
        }
    }
}

/// `fill_sockaddr` must store `sin6_flowinfo` verbatim, matching `std`,
/// `socket2`, `mio` and `quinn-udp` (finding F-4: no byte swap).
#[test]
fn sockaddr_v6_flowinfo_matches_std_convention() {
    let flowinfo = 0x0000_1234u32;
    let addr = v6(Ipv6Addr::LOCALHOST, 0, flowinfo, 0);
    let mut ss: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    fill_sockaddr(&addr, &mut ss);
    let sin6 = unsafe { &*(&ss as *const _ as *const libc::sockaddr_in6) };
    // std::sys::net::socket_addr_v6_to_c does `sin6_flowinfo: addr.flowinfo()`.
    assert_eq!(sin6.sin6_flowinfo, flowinfo);
    assert_eq!(storage_to_addr(&ss), addr);
}

#[test]
fn storage_to_addr_unknown_family_falls_back() {
    let mut ss: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    ss.ss_family = libc::AF_UNIX as _;
    assert_eq!(storage_to_addr(&ss), SocketAddr::from(([0, 0, 0, 0], 0)));

    ss.ss_family = 0x7fff;
    assert_eq!(storage_to_addr(&ss), SocketAddr::from(([0, 0, 0, 0], 0)));
}

/// `storage_to_addr` only accepts `&sockaddr_storage` (8-aligned), so the
/// `sockaddr_in`/`sockaddr_in6` casts are always sufficiently aligned. Verify
/// with a storage located at a non-zero but still 8-aligned offset.
#[test]
fn storage_to_addr_at_odd_but_aligned_offset() {
    #[repr(align(8))]
    struct Holder(#[allow(dead_code)] [u64; 2], libc::sockaddr_storage);

    let addr = v4([10, 20, 30, 40], 5353);
    let mut holder = Holder([0; 2], unsafe { std::mem::zeroed() });
    fill_sockaddr(&addr, &mut holder.1);
    assert_eq!(storage_to_addr(&holder.1), addr);
}

// ---------------------------------------------------------------------------
// Async / loopback behaviour
// ---------------------------------------------------------------------------

fn loop_addr(port: u16) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port))
}

fn loop_addr_v6(port: u16) -> SocketAddr {
    SocketAddr::from((Ipv6Addr::LOCALHOST, port))
}

/// GRO is off unless explicitly requested, even on kernels that support it.
#[tokio::test]
async fn gro_is_disabled_by_default() {
    let socket = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    assert!(!socket.capabilities().gro);
}

/// Diagnostic: does loopback GRO coalesce on this kernel, and into how many
/// bytes does one `recvmsg` hand us?
#[cfg(target_os = "linux")]
#[tokio::test]
async fn probe_gro_coalescing() {
    let receiver = FastUdpSocket::build(loop_addr(0)).gro(true).bind().unwrap();
    if !receiver.capabilities().gro {
        eprintln!("PROBE: GRO not enabled on this kernel");
        return;
    }
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    for (size, count) in [(1000usize, 16usize), (1400, 64), (64, 64)] {
        let payload = vec![0x5Au8; size];
        let items: Vec<SendItem> = (0..count).map(|_| SendItem::new(raddr, &payload)).collect();
        sender.send_many(&items).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;

        // GRO requires the full 64-slot batch even when fewer datagrams arrive.
        let mut bufs: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; size]).collect();
        let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
        let n = match tokio::time::timeout(std::time::Duration::from_secs(1), receiver.receive_many(&mut recv_items))
            .await
        {
            Ok(Ok(n)) => n,
            other => {
                eprintln!("PROBE size={size} count={count}: receive_many -> {other:?}");
                continue;
            }
        };
        let lens: Vec<usize> = recv_items[..n].iter().map(|i| i.len()).collect();
        let total: usize = lens.iter().sum();
        eprintln!("PROBE size={size} count={count}: one recvmsg produced n={n} items, lens={lens:?}, total={total}");
        // Drain any remainder so the next iteration starts clean.
        loop {
            let m = match tokio::time::timeout(
                std::time::Duration::from_millis(50),
                receiver.receive_many(&mut recv_items),
            )
            .await
            {
                Ok(Ok(m)) => m,
                _ => break,
            };
            if m == 0 {
                break;
            }
        }
    }

    // GSO send: one large skb segmented by the sender may re-coalesce on the
    // receive path.
    if sender.capabilities().gso {
        for (size, count) in [(1400usize, 40usize), (1000, 32)] {
            let payload = vec![0x6Bu8; size * count];
            let items = [SendItem::new(raddr, &payload).segment_size(size as u16)];
            if let Err(e) = sender.send_many(&items).await {
                eprintln!("PROBE gso size={size} count={count}: send_many -> {e:?}");
                continue;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;

            let mut bufs: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; size]).collect();
            let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
            let n =
                match tokio::time::timeout(std::time::Duration::from_secs(1), receiver.receive_many(&mut recv_items))
                    .await
                {
                    Ok(Ok(n)) => n,
                    other => {
                        eprintln!("PROBE gso size={size}: receive_many -> {other:?}");
                        continue;
                    }
                };
            let lens: Vec<usize> = recv_items[..n].iter().map(|i| i.len()).collect();
            let total: usize = lens.iter().sum();
            eprintln!("PROBE gso size={size} count={count}: n={n} lens={lens:?} total={total}");
            loop {
                match tokio::time::timeout(std::time::Duration::from_millis(50), receiver.receive_many(&mut recv_items))
                    .await
                {
                    Ok(Ok(m)) if m > 0 => {}
                    _ => break,
                }
            }
        }
    }
}

/// With GRO enabled, `receive()` is rejected before the syscall, so a pending
/// datagram (or coalesced batch) is not consumed or partially dropped (F-5).
#[cfg(target_os = "linux")]
#[tokio::test]
async fn gro_single_item_receive_does_not_lose_segments() {
    const COUNT: usize = 32;
    const SIZE: usize = 1000;

    let receiver = FastUdpSocket::build(loop_addr(0)).gro(true).bind().unwrap();
    if !receiver.capabilities().gro {
        eprintln!("GRO not supported on this kernel — skipping");
        return;
    }
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let payload = vec![0x33u8; SIZE];
    let items: Vec<SendItem> = (0..COUNT).map(|_| SendItem::new(raddr, &payload)).collect();
    sender.send_many(&items).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;

    // A single slot is refused without issuing `recvmsg`.
    let mut buf = vec![0u8; SIZE];
    let mut item = ReceiveItem::new(&mut buf);
    let err = tokio::time::timeout(std::time::Duration::from_secs(2), receiver.receive(&mut item))
        .await
        .expect("receive() must not block")
        .expect_err("receive() must be rejected while GRO is enabled");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(err.to_string().contains("receive_many"), "misleading error: {err}");
    assert_eq!(item.len(), 0);

    // Nothing was consumed: a full 64-slot batch still sees every byte.
    let mut bufs: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; SIZE]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let mut total = 0;
    while total < COUNT * SIZE {
        match tokio::time::timeout(std::time::Duration::from_millis(500), receiver.receive_many(&mut recv_items)).await
        {
            Ok(Ok(n)) if n > 0 => {
                total += recv_items[..n].iter().map(|i| i.len()).sum::<usize>();
            }
            other => panic!("timed out after {total} of {} bytes: {other:?}", COUNT * SIZE),
        }
    }
    assert_eq!(total, COUNT * SIZE);
}

/// A 63-item batch and a 64-item batch with `max_batch_size(63)` must both be
/// rejected before `recvmsg`, leaving the datagram queued. 64 items and
/// `max_batch_size >= 64` then receive it.
#[cfg(target_os = "linux")]
#[tokio::test]
async fn gro_receive_requires_64_slots_and_batch() {
    let receiver = FastUdpSocket::build(loop_addr(0)).gro(true).bind().unwrap();
    if !receiver.capabilities().gro {
        eprintln!("GRO not supported on this kernel — skipping");
        return;
    }
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    sender.send(SendItem::new(raddr, b"pending-gro")).await.unwrap();

    let mut bufs63: Vec<Vec<u8>> = (0..63).map(|_| vec![0u8; 64]).collect();
    let mut items63: Vec<ReceiveItem> = bufs63.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let err = receiver
        .receive_many(&mut items63)
        .await
        .expect_err("63 receive items must be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(items63.iter().all(|i| i.is_empty()));

    // The datagram is still queued: 64 slots receive it.
    let mut bufs: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; 64]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let n = tokio::time::timeout(std::time::Duration::from_secs(2), receiver.receive_many(&mut items))
        .await
        .expect("64-slot receive timed out")
        .unwrap();
    assert_eq!(n, 1);
    assert_eq!(items[0].data(), b"pending-gro");

    // A socket whose batch cap is below 64 must reject even a 64-item slice.
    let tight = FastUdpSocket::build(loop_addr(0))
        .gro(true)
        .max_batch_size(63)
        .bind()
        .unwrap();
    if !tight.capabilities().gro {
        return;
    }
    assert_eq!(tight.capabilities().max_batch, 63);
    let taddr = tight.local_addr().unwrap();
    sender.send(SendItem::new(taddr, b"tight")).await.unwrap();

    let mut bufs64: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; 64]).collect();
    let mut items64: Vec<ReceiveItem> = bufs64.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let err = tight
        .receive_many(&mut items64)
        .await
        .expect_err("max_batch_size < 64 must be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}

/// Datagrams larger than the receive buffer must be truncated to the buffer
/// length and must not write out of bounds (`item.len() <= buf.len()`).
#[tokio::test]
async fn oversized_datagram_is_truncated_to_buffer() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let msg = vec![0xA7u8; 2000];
    sender.send(SendItem::new(raddr, &msg)).await.unwrap();

    let mut buf = vec![0u8; 100];
    let mut item = ReceiveItem::new(&mut buf);
    tokio::time::timeout(std::time::Duration::from_secs(2), receiver.receive(&mut item))
        .await
        .expect("receive timed out")
        .unwrap();
    assert_eq!(item.len(), 100);
    assert_eq!(item.data(), &msg[..100]);
}

/// Same, but through the GRO path with a single datagram (no GRO cmsg). The
/// 64 KiB staging buffer holds the whole datagram, and the per-item copy is
/// truncated to the slot.
#[cfg(target_os = "linux")]
#[tokio::test]
async fn oversized_datagram_gro_path_is_truncated() {
    let receiver = FastUdpSocket::build(loop_addr(0)).gro(true).bind().unwrap();
    if !receiver.capabilities().gro {
        return;
    }
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

    let msg = vec![0xB7u8; 2000];
    sender.send(SendItem::new(raddr, &msg)).await.unwrap();

    let mut bufs: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; 100]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let n = tokio::time::timeout(std::time::Duration::from_secs(2), receiver.receive_many(&mut items))
        .await
        .expect("receive timed out")
        .unwrap();
    assert_eq!(n, 1);
    assert_eq!(items[0].len(), 100);
    assert_eq!(items[0].data(), &msg[..100]);
}

/// `max_batch_size(0)` is clamped to one so `send`/`receive` still issue a
/// syscall instead of silently dropping a datagram (F-6).
#[tokio::test]
async fn max_batch_zero_is_clamped_to_one() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).max_batch_size(0).bind().unwrap();
    assert_eq!(sender.capabilities().max_batch, 1);

    sender.send(SendItem::new(raddr, b"never dropped")).await.unwrap();

    let mut buf = vec![0u8; 64];
    let mut item = ReceiveItem::new(&mut buf);
    tokio::time::timeout(std::time::Duration::from_secs(2), receiver.receive(&mut item))
        .await
        .expect("receive timed out")
        .unwrap();
    assert_eq!(item.data(), b"never dropped");

    // The receive side clamps too: a pending datagram is not reported as an
    // empty success.
    let tight = FastUdpSocket::build(loop_addr(0)).max_batch_size(0).bind().unwrap();
    assert_eq!(tight.capabilities().max_batch, 1);
    let taddr = tight.local_addr().unwrap();
    sender.send(SendItem::new(taddr, b"pending")).await.unwrap();

    let mut buf2 = vec![0u8; 64];
    let mut item2 = ReceiveItem::new(&mut buf2);
    tokio::time::timeout(std::time::Duration::from_secs(2), tight.receive(&mut item2))
        .await
        .expect("receive() must not return an empty success immediately")
        .unwrap();
    assert_eq!(item2.data(), b"pending");
}

/// `max_batch_size(1)` still sends/receives one datagram per call and reports
/// partial progress for larger batches.
#[tokio::test]
async fn max_batch_one_sends_one_at_a_time() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).max_batch_size(1).bind().unwrap();

    let payloads: Vec<Vec<u8>> = (0..5u8).map(|i| vec![i; 8]).collect();
    let items: Vec<SendItem> = payloads.iter().map(|p| SendItem::new(raddr, p)).collect();
    let mut sent = 0;
    while sent < items.len() {
        let n = sender.send_many(&items[sent..]).await.unwrap();
        assert!(n <= 1);
        assert!(n > 0);
        sent += n;
    }

    let mut bufs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 32]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let mut total = 0;
    while total < 5 {
        total += receiver.receive_many(&mut recv_items[total..]).await.unwrap();
    }
    assert_eq!(total, 5);
    for (i, item) in recv_items.iter().enumerate() {
        assert_eq!(item.data(), payloads[i].as_slice());
    }
}

/// `max_batch_size(usize::MAX)` must not overflow batch arithmetic and must
/// send everything across the 64-entry `sendmmsg` chunks.
#[cfg(target_os = "linux")]
#[tokio::test]
async fn max_batch_usize_max_sends_large_batches() {
    const COUNT: usize = 130;

    let receiver = FastUdpSocket::build(loop_addr(0))
        .max_batch_size(usize::MAX)
        .bind()
        .unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0))
        .max_batch_size(usize::MAX)
        .bind()
        .unwrap();

    let payloads: Vec<Vec<u8>> = (0..COUNT).map(|i| vec![(i % 251) as u8; 16]).collect();
    let items: Vec<SendItem> = payloads.iter().map(|p| SendItem::new(raddr, p)).collect();
    let n = sender.send_many(&items).await.unwrap();
    assert_eq!(n, COUNT);

    let mut bufs: Vec<Vec<u8>> = (0..COUNT).map(|_| vec![0u8; 64]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let mut total = 0;
    while total < COUNT {
        total += receiver.receive_many(&mut recv_items[total..]).await.unwrap();
    }
    assert_eq!(total, COUNT);
    for (i, item) in recv_items[..total].iter().enumerate() {
        assert_eq!(item.data(), payloads[i].as_slice(), "datagram {i}");
    }
}

/// Empty datagrams mixed with non-empty ones in one `sendmmsg` batch.
#[cfg(target_os = "linux")]
#[tokio::test]
async fn sendmmsg_mixed_empty_datagrams() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    if !sender.capabilities().sendmmsg {
        return;
    }

    let payloads: [&[u8]; 4] = [b"", b"a", b"", b"bc"];
    let items: Vec<SendItem> = payloads.iter().map(|p| SendItem::new(raddr, p)).collect();
    assert_eq!(sender.send_many(&items).await.unwrap(), 4);

    let mut bufs: Vec<Vec<u8>> = (0..4).map(|_| vec![0u8; 16]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let mut total = 0;
    while total < 4 {
        total += receiver.receive_many(&mut recv_items[total..]).await.unwrap();
    }
    for (i, item) in recv_items[..4].iter().enumerate() {
        assert_eq!(item.data(), payloads[i]);
    }
}

/// User-space (non-GSO) segmentation with `segment_size` 1, 2 and `u16::MAX`.
#[tokio::test]
async fn segment_size_boundaries_manual_chunking() {
    for (seg, len, expected) in [(1u16, 5usize, 5usize), (2, 5, 3), (2, 4, 2), (u16::MAX, 4, 1)] {
        let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
        let raddr = receiver.local_addr().unwrap();
        let sender = FastUdpSocket::build(loop_addr(0)).gso(false).bind().unwrap();

        let payload: Vec<u8> = (0..len).map(|i| i as u8).collect();
        let items = [SendItem::new(raddr, &payload).segment_size(seg)];
        sender.send_many(&items).await.unwrap();

        let mut bufs: Vec<Vec<u8>> = (0..expected).map(|_| vec![0u8; 64]).collect();
        let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
        let mut total = 0;
        while total < expected {
            total += tokio::time::timeout(
                std::time::Duration::from_secs(2),
                receiver.receive_many(&mut recv_items[total..]),
            )
            .await
            .expect("timed out")
            .unwrap();
        }

        // Reassemble and compare with the original payload.
        let mut got = Vec::new();
        for item in &recv_items[..expected] {
            got.extend_from_slice(item.data());
            assert!(item.len() <= seg as usize);
        }
        assert_eq!(got, payload, "seg={seg} len={len}");
    }
}

/// GSO (kernel) segmentation with `segment_size` 1 and 2 where supported.
#[cfg(target_os = "linux")]
#[tokio::test]
async fn segment_size_boundaries_gso() {
    let probe = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    if !probe.capabilities().gso {
        eprintln!("GSO not supported — skipping");
        return;
    }
    drop(probe);

    for (seg, len, expected) in [(1u16, 5usize, 5usize), (2, 5, 3)] {
        let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
        let raddr = receiver.local_addr().unwrap();
        let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();

        let payload: Vec<u8> = (0..len).map(|i| i as u8).collect();
        let items = [SendItem::new(raddr, &payload).segment_size(seg)];
        let sent = sender.send_many(&items).await.unwrap();
        assert_eq!(sent, 1);

        let mut bufs: Vec<Vec<u8>> = (0..expected).map(|_| vec![0u8; 64]).collect();
        let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
        let mut total = 0;
        while total < expected {
            total += tokio::time::timeout(
                std::time::Duration::from_secs(2),
                receiver.receive_many(&mut recv_items[total..]),
            )
            .await
            .expect("timed out")
            .unwrap();
        }
        let mut got = Vec::new();
        for item in &recv_items[..expected] {
            got.extend_from_slice(item.data());
        }
        assert_eq!(got, payload, "gso seg={seg} len={len}");
    }
}

/// A batch that mixes segmented and non-segmented items must not use the
/// `sendmmsg` fast path and must preserve order.
#[tokio::test]
async fn mixed_segmented_and_plain_items() {
    let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr(0)).gso(false).bind().unwrap();

    let a = b"AAAA";
    let b = b"BB";
    let c = b"CCCC";
    let items = [
        SendItem::new(raddr, a).segment_size(2),
        SendItem::new(raddr, b),
        SendItem::new(raddr, c).segment_size(3),
    ];
    sender.send_many(&items).await.unwrap();

    // a -> "AA","AA"; b -> "BB"; c -> "CCC","C"
    let expected: [&[u8]; 5] = [b"AA", b"AA", b"BB", b"CCC", b"C"];
    let mut bufs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 16]).collect();
    let mut recv_items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    let mut total = 0;
    while total < 5 {
        total += tokio::time::timeout(
            std::time::Duration::from_secs(2),
            receiver.receive_many(&mut recv_items[total..]),
        )
        .await
        .expect("timed out")
        .unwrap();
    }
    for (i, item) in recv_items[..5].iter().enumerate() {
        assert_eq!(item.data(), expected[i], "datagram {i}");
    }
}

// ---------------------------------------------------------------------------
// recv_gro splitting logic (synthetic)
// ---------------------------------------------------------------------------

/// The failure modes of the splitting loop in `recv_gro`.
#[derive(Debug, PartialEq, Eq)]
enum GroSplitError {
    /// `MSG_TRUNC`: the aggregate exceeded the 64 KiB staging buffer.
    Truncated,
    ZeroSegmentSize,
    SegmentLargerThanTotal,
    TooManySegments,
    OffsetOverflow,
}

/// Faithful copy of the splitting logic in `recv_gro`, parameterised over the
/// already-received byte stream so it can be exercised without a kernel that
/// coalesces on loopback. Mirrors the production checks: reject `MSG_TRUNC`,
/// zero/oversized segment metadata and aggregates with more segments than
/// slots. Keeping the copy in the test suite lets us pin down the semantics on
/// hosts where the kernel never emits `UDP_GRO` cmsgs.
fn gro_split_model(
    gro_buf: &[u8],
    total: usize,
    seg_size: Option<usize>,
    msg_flags: libc::c_int,
    items: &mut [ReceiveItem<'_>],
) -> Result<usize, GroSplitError> {
    if msg_flags & libc::MSG_TRUNC != 0 {
        return Err(GroSplitError::Truncated);
    }

    let Some(seg) = seg_size else {
        if items.is_empty() {
            return Ok(0);
        }
        let copy_len = total.min(items[0].buf.len());
        items[0].buf[..copy_len].copy_from_slice(&gro_buf[..copy_len]);
        items[0].len = copy_len;
        return Ok(1);
    };

    if seg == 0 {
        return Err(GroSplitError::ZeroSegmentSize);
    }
    if seg > total {
        return Err(GroSplitError::SegmentLargerThanTotal);
    }
    let segments = total.div_ceil(seg);
    if segments > items.len() {
        return Err(GroSplitError::TooManySegments);
    }

    let mut offset = 0usize;
    let mut count = 0usize;
    for item in items.iter_mut().take(segments) {
        if offset >= total {
            break;
        }
        let seg_end = offset.checked_add(seg).ok_or(GroSplitError::OffsetOverflow)?.min(total);
        let copy_len = (seg_end - offset).min(item.buf.len());
        item.buf[..copy_len].copy_from_slice(&gro_buf[offset..offset + copy_len]);
        item.len = copy_len;
        count += 1;
        offset = seg_end;
    }
    debug_assert_eq!(offset, total);
    Ok(count)
}

#[test]
fn gro_split_model_with_enough_slots_delivers_everything() {
    let src: Vec<u8> = (0..2000u32).map(|i| i as u8).collect();
    let mut bufs: Vec<Vec<u8>> = (0..2).map(|_| vec![0u8; 1000]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let count = gro_split_model(&src, src.len(), Some(1000), 0, &mut items).unwrap();
    assert_eq!(count, 2);
    assert_eq!(items[0].data(), &src[..1000]);
    assert_eq!(items[1].data(), &src[1000..]);
}

/// A receive slice with fewer slots than segments is rejected instead of
/// silently discarding the tail (F-5). This is why GRO requires 64 slots.
#[test]
fn gro_split_model_too_few_slots_is_an_error() {
    let src: Vec<u8> = vec![0x22; 16 * 1000];
    let mut bufs: Vec<Vec<u8>> = (0..8).map(|_| vec![0u8; 1000]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    assert_eq!(
        gro_split_model(&src, src.len(), Some(1000), 0, &mut items),
        Err(GroSplitError::TooManySegments)
    );
    assert!(items.iter().all(|i| i.is_empty()), "no partial delivery on error");
}

/// 63 slots cannot hold 64 coalesced segments; 64 slots can.
#[test]
fn gro_split_model_requires_one_slot_per_segment() {
    let src: Vec<u8> = vec![0x23; 64 * 1000];

    let mut bufs63: Vec<Vec<u8>> = (0..63).map(|_| vec![0u8; 1000]).collect();
    let mut items63: Vec<ReceiveItem> = bufs63.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    assert_eq!(
        gro_split_model(&src, src.len(), Some(1000), 0, &mut items63),
        Err(GroSplitError::TooManySegments)
    );

    let mut bufs64: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; 1000]).collect();
    let mut items64: Vec<ReceiveItem> = bufs64.iter_mut().map(|b| ReceiveItem::new(b)).collect();
    assert_eq!(gro_split_model(&src, src.len(), Some(1000), 0, &mut items64), Ok(64));
}

/// A zero `seg_size` is rejected rather than "delivering" zero-length items
/// and losing the whole datagram.
#[test]
fn gro_split_model_zero_seg_is_an_error() {
    let src: Vec<u8> = vec![0x33; 100];
    let mut bufs: Vec<Vec<u8>> = (0..4).map(|_| vec![0u8; 100]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    assert_eq!(
        gro_split_model(&src, src.len(), Some(0), 0, &mut items),
        Err(GroSplitError::ZeroSegmentSize)
    );
    assert!(items.iter().all(|i| i.is_empty()));
}

/// A segment size larger than the aggregate is nonsensical kernel metadata and
/// is rejected.
#[test]
fn gro_split_model_seg_larger_than_total_is_an_error() {
    let src: Vec<u8> = vec![0x34; 100];
    let mut bufs: Vec<Vec<u8>> = (0..4).map(|_| vec![0u8; 100]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    assert_eq!(
        gro_split_model(&src, src.len(), Some(101), 0, &mut items),
        Err(GroSplitError::SegmentLargerThanTotal)
    );
    assert!(items.iter().all(|i| i.is_empty()));
}

/// `MSG_TRUNC` means the aggregate did not fit the staging buffer; splitting
/// the surviving prefix would silently lose data, so it is an error and no
/// slot is written.
#[test]
fn gro_split_model_msg_trunc_is_an_error() {
    let src: Vec<u8> = vec![0x35; 65536];
    let mut bufs: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; 1500]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    assert_eq!(
        gro_split_model(&src, src.len(), Some(1500), libc::MSG_TRUNC, &mut items),
        Err(GroSplitError::Truncated)
    );
    assert!(items.iter().all(|i| i.is_empty()));
}

/// Without a GRO control message the datagram is a normal single datagram and
/// fills one slot.
#[test]
fn gro_split_model_without_gro_cmsg_fills_one_slot() {
    let src: Vec<u8> = vec![0x36; 1500];
    let mut bufs: Vec<Vec<u8>> = (0..64).map(|_| vec![0u8; 64]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let count = gro_split_model(&src, src.len(), None, 0, &mut items).unwrap();
    assert_eq!(count, 1);
    assert_eq!(items[0].len(), 64);
    assert_eq!(items[0].data(), &src[..64]);
    assert!(items[1..].iter().all(|i| i.is_empty()));
}

/// A segment larger than every slot is truncated per item, but the offset still
/// advances by `seg`; `item.len() == buf.len()` is the only (ambiguous) hint.
/// This per-datagram truncation is the documented behaviour of `ReceiveItem`.
#[test]
fn gro_split_model_small_buffers_truncate_per_segment() {
    let src: Vec<u8> = vec![0x44; 2000];
    let mut bufs: Vec<Vec<u8>> = (0..2).map(|_| vec![0u8; 100]).collect();
    let mut items: Vec<ReceiveItem> = bufs.iter_mut().map(|b| ReceiveItem::new(b)).collect();

    let count = gro_split_model(&src, src.len(), Some(1000), 0, &mut items).unwrap();
    assert_eq!(count, 2);
    assert_eq!(items[0].len, 100);
    assert_eq!(items[1].len, 100);
}

// ---------------------------------------------------------------------------
// sendmmsg ENOSYS (seccomp subprocess)
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
fn install_sendmmsg_enosys_filter() -> std::io::Result<()> {
    const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
    const SECCOMP_RET_ERRNO: u32 = 0x0005_0000;
    const SECCOMP_MODE_FILTER: libc::c_ulong = 2;

    let filter = [
        // A = seccomp_data.nr
        libc::sock_filter {
            code: 0x20,
            jt: 0,
            jf: 0,
            k: 0,
        },
        // if A == SYS_sendmmsg, fall through; else skip the ERRNO return.
        libc::sock_filter {
            code: 0x15,
            jt: 0,
            jf: 1,
            k: libc::SYS_sendmmsg as u32,
        },
        libc::sock_filter {
            code: 0x06,
            jt: 0,
            jf: 0,
            k: SECCOMP_RET_ERRNO | (libc::ENOSYS as u32 & 0xffff),
        },
        libc::sock_filter {
            code: 0x06,
            jt: 0,
            jf: 0,
            k: SECCOMP_RET_ALLOW,
        },
    ];
    let prog = libc::sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_ptr() as *mut libc::sock_filter,
    };
    unsafe {
        if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        if libc::prctl(libc::PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &prog as *const libc::sock_fprog) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}

/// Child half of `sendmmsg_enosys_falls_back_to_sendmsg`: installs a seccomp
/// filter that makes `sendmmsg` fail with `ENOSYS`, then runs the two-item send
/// path. Exits 43 when the crate falls back to per-item `sendmsg`, 42 when it
/// surfaces ENOSYS, and 77 when seccomp is unavailable.
#[cfg(target_os = "linux")]
#[test]
fn sendmmsg_enosys_repro_child() {
    if std::env::var_os("AUDIT_SENDMMSG_ENOSYS_CHILD").is_none() {
        return;
    }
    if let Err(e) = install_sendmmsg_enosys_filter() {
        eprintln!("SKIP: seccomp unavailable: {e}");
        std::process::exit(77);
    }
    // Confirm the filter actually applies to the syscalls the process makes.
    // Under qemu-user the guest syscall number is translated before seccomp
    // runs, so the filter is ineffective and the test is not meaningful.
    let probe = unsafe { libc::syscall(libc::SYS_sendmmsg, -1, std::ptr::null::<u8>(), 0, 0) };
    if probe != -1 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ENOSYS) {
        eprintln!("SKIP: seccomp filter did not intercept sendmmsg (emulated target?)");
        std::process::exit(77);
    }
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let code = rt.block_on(async {
        let receiver = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
        let raddr = receiver.local_addr().unwrap();
        let sender = FastUdpSocket::build(loop_addr(0)).bind().unwrap();
        eprintln!("child: caps.sendmmsg={}", sender.capabilities().sendmmsg);
        let items = [SendItem::new(raddr, b"first"), SendItem::new(raddr, b"second")];
        match sender.send_many(&items).await {
            Ok(n) => {
                eprintln!("child: send_many -> Ok({n}) (fallback worked)");
                if n == 2 { 43 } else { 1 }
            }
            Err(e) => {
                eprintln!("child: send_many -> Err({e:?}) raw={:?}", e.raw_os_error());
                if e.raw_os_error() == Some(libc::ENOSYS) { 42 } else { 1 }
            }
        }
    });
    std::process::exit(code);
}

/// On a kernel (or seccomp policy) where `sendmmsg` returns ENOSYS, a
/// multi-item send must fall back to one `sendmsg` per item (F-7).
#[cfg(target_os = "linux")]
#[test]
fn sendmmsg_enosys_falls_back_to_sendmsg() {
    let exe = std::env::current_exe().expect("current_exe");
    let out = std::process::Command::new(exe)
        .args([
            "--exact",
            "socket::unix_socket::audit_socket_tests::sendmmsg_enosys_repro_child",
            "--nocapture",
        ])
        .env("AUDIT_SENDMMSG_ENOSYS_CHILD", "1")
        .output()
        .expect("spawn child");
    let stderr = String::from_utf8_lossy(&out.stderr);
    eprintln!("child status: {:?}\n{stderr}", out.status);
    match out.status.code() {
        Some(77) => eprintln!("seccomp unavailable — skipping"),
        Some(43) => assert!(stderr.contains("fallback worked"), "unexpected child output"),
        Some(42) => panic!("sendmmsg ENOSYS was not handled by the fallback:\n{stderr}"),
        other => panic!("child exited with {other:?}; stderr:\n{stderr}"),
    }
}

/// IPv6 loopback: empty datagrams and datagrams larger than the receive
/// buffer (must truncate, never write out of bounds).
#[tokio::test]
async fn ipv6_loopback_empty_and_oversized() {
    let receiver = match FastUdpSocket::build(loop_addr_v6(0)).bind() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("IPv6 loopback unavailable — skipping");
            return;
        }
    };
    let raddr = receiver.local_addr().unwrap();
    let sender = FastUdpSocket::build(loop_addr_v6(0)).bind().unwrap();
    let saddr = sender.local_addr().unwrap();

    // Empty datagram.
    sender.send(SendItem::new(raddr, b"")).await.unwrap();
    let mut buf = vec![0u8; 32];
    let mut item = ReceiveItem::new(&mut buf);
    tokio::time::timeout(std::time::Duration::from_secs(2), receiver.receive(&mut item))
        .await
        .expect("v6 empty receive timed out")
        .unwrap();
    assert_eq!(item.len(), 0);
    assert_eq!(item.source(), saddr);

    // Oversized datagram.
    let msg = vec![0x9Cu8; 2000];
    sender.send(SendItem::new(raddr, &msg)).await.unwrap();
    let mut buf = vec![0u8; 64];
    let mut item = ReceiveItem::new(&mut buf);
    tokio::time::timeout(std::time::Duration::from_secs(2), receiver.receive(&mut item))
        .await
        .expect("v6 oversized receive timed out")
        .unwrap();
    assert_eq!(item.len(), 64);
    assert_eq!(item.data(), &msg[..64]);
}
