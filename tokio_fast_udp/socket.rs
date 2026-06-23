// FastUdpSocket and FastUdpSocketBuilder for Unix (AsyncFd + nix syscalls)
// and Windows (tokio::net::UdpSocket).

// ---------------------------------------------------------------------------
// Unix implementation
// ---------------------------------------------------------------------------
#[cfg(unix)]
mod unix_socket {
    use std::{
        io,
        net::SocketAddr,
        os::fd::{AsRawFd, OwnedFd},
    };

    // -- platform alias --------------------------------------------------------
    #[cfg(target_os = "linux")]
    use linux_inner as plat;
    use tokio::io::{Interest, unix::AsyncFd};
    #[cfg(all(unix, not(target_os = "linux")))]
    use unix_inner as plat;

    use crate::{
        capability::Capabilities,
        ecn::Ecn,
        item::{ReceiveItem, SendItem},
    };

    // -- FastUdpSocket ---------------------------------------------------------
    pub struct FastUdpSocket {
        fd: AsyncFd<OwnedFd>,
        caps: Capabilities,
    }

    impl FastUdpSocket {
        pub async fn send(&self, item: SendItem<'_>) -> io::Result<()> {
            let items = [item];
            self.send_many(&items).await?;
            Ok(())
        }

        pub async fn send_many(&self, items: &[SendItem<'_>]) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }
            let caps = &self.caps;
            let limit = items.len().min(caps.max_batch);
            let items = &items[..limit];
            let mut gso_einval = false;
            let mut chunk_offset = 0usize;
            self.fd
                .async_io(Interest::WRITABLE, |inner| {
                    plat::send_batch(inner.as_raw_fd(), items, caps, &mut gso_einval, &mut chunk_offset)
                })
                .await
        }

        pub async fn receive(&self, item: &mut ReceiveItem<'_>) -> io::Result<()> {
            self.receive_many(std::slice::from_mut(item)).await?;
            Ok(())
        }

        pub async fn receive_many(&self, items: &mut [ReceiveItem<'_>]) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }
            let caps = &self.caps;
            let limit = items.len().min(caps.max_batch);
            let items = &mut items[..limit];
            self.fd
                .async_io(Interest::READABLE, |inner| plat::recv_batch(inner.as_raw_fd(), items, caps))
                .await
        }

        pub fn capabilities(&self) -> &Capabilities {
            &self.caps
        }

        pub fn local_addr(&self) -> io::Result<SocketAddr> {
            plat::local_addr(self.fd.get_ref())
        }
    }

    // -- FastUdpSocketBuilder --------------------------------------------------
    pub struct FastUdpSocketBuilder {
        addr: SocketAddr,
        disable_gso: bool,
        disable_gro: bool,
        disable_sendmmsg: bool,
        disable_ecn: bool,
        max_batch: usize,
    }

    impl FastUdpSocketBuilder {
        pub fn bind(addr: SocketAddr) -> Self {
            FastUdpSocketBuilder {
                addr,
                disable_gso: false,
                disable_gro: false,
                disable_sendmmsg: false,
                disable_ecn: false,
                max_batch: 64,
            }
        }

        pub fn disable_gso(mut self) -> Self {
            self.disable_gso = true;
            self
        }

        pub fn disable_gro(mut self) -> Self {
            self.disable_gro = true;
            self
        }

        pub fn disable_sendmmsg(mut self) -> Self {
            self.disable_sendmmsg = true;
            self
        }

        pub fn disable_ecn(mut self) -> Self {
            self.disable_ecn = true;
            self
        }

        pub fn max_batch_size(mut self, n: usize) -> Self {
            self.max_batch = n;
            self
        }

        pub fn build(self) -> io::Result<FastUdpSocket> {
            let std_socket = std::net::UdpSocket::bind(self.addr)?;
            std_socket.set_nonblocking(true)?;

            let is_ipv6 = self.addr.is_ipv6();

            use std::os::fd::{FromRawFd, IntoRawFd, OwnedFd};
            let raw_fd = std_socket.into_raw_fd();
            let fd: OwnedFd = unsafe { OwnedFd::from_raw_fd(raw_fd) };

            let config = BuildConfig {
                disable_gso: self.disable_gso,
                disable_gro: self.disable_gro,
                disable_sendmmsg: self.disable_sendmmsg,
                disable_ecn: self.disable_ecn,
                max_batch: self.max_batch,
            };

            let caps = plat::setup_and_probe(&fd, &config, is_ipv6)?;
            let async_fd = AsyncFd::new(fd)?;

            Ok(FastUdpSocket {
                fd: async_fd,
                caps,
            })
        }
    }

    // -- shared nix helpers (used by the platform inner modules via super::) --

    use nix::sys::socket::{ControlMessageOwned, SockaddrStorage};

    fn nix_err(e: nix::errno::Errno) -> io::Error {
        e.into()
    }

    fn ecn_to_tos(ecn: Ecn) -> u8 {
        ecn.to_tos_bits()
    }

    fn parse_ecn(cmsgs: &[ControlMessageOwned]) -> Option<Ecn> {
        for cmsg in cmsgs {
            match cmsg {
                ControlMessageOwned::Ipv4Tos(tos) => return Some(Ecn::from_tos_bits(*tos)),
                ControlMessageOwned::Ipv6TClass(tc) => return Some(Ecn::from_tos_bits(*tc as u8)),
                _ => {}
            }
        }
        None
    }

    fn storage_to_addr(addr: &SockaddrStorage) -> SocketAddr {
        if let Some(v4) = addr.as_sockaddr_in() {
            return (*v4).into();
        }
        if let Some(v6) = addr.as_sockaddr_in6() {
            return (*v6).into();
        }
        SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0)
    }

    pub(crate) struct BuildConfig {
        pub disable_gso: bool,
        pub disable_gro: bool,
        pub disable_sendmmsg: bool,
        pub disable_ecn: bool,
        pub max_batch: usize,
    }

    // -- Linux fast path ----------------------------------------------------
    #[cfg(target_os = "linux")]
    mod linux_inner {
        use std::{
            io,
            net::SocketAddr,
            os::fd::{AsRawFd, OwnedFd},
        };

        use nix::sys::socket::{ControlMessage, MsgFlags, SockaddrStorage, recvmsg, sendmsg};

        use super::{BuildConfig, ecn_to_tos, nix_err, parse_ecn, storage_to_addr};
        use crate::{
            capability::Capabilities,
            ecn::Ecn,
            item::{ReceiveItem, SendItem},
        };

        fn send_one(fd: std::os::fd::RawFd, item: &SendItem, ecn_enabled: bool, gso_enabled: bool) -> io::Result<()> {
            let iov = [std::io::IoSlice::new(item.data)];
            let addr = SockaddrStorage::from(item.destination);

            let has_gso = gso_enabled && item.segment_size.is_some();
            let has_ecn = ecn_enabled && item.ecn.is_some();
            let is_v4 = item.destination.is_ipv4();

            match (has_gso, has_ecn, is_v4) {
                (true, true, true) => {
                    let s = item.segment_size.as_ref().unwrap();
                    let tos = &ecn_to_tos(item.ecn.unwrap());
                    sendmsg(
                        fd,
                        &iov,
                        &[ControlMessage::UdpGsoSegments(s), ControlMessage::Ipv4Tos(tos)],
                        MsgFlags::empty(),
                        Some(&addr),
                    )
                }
                (true, true, false) => {
                    let s = item.segment_size.as_ref().unwrap();
                    let tc = &(ecn_to_tos(item.ecn.unwrap()) as i32);
                    sendmsg(
                        fd,
                        &iov,
                        &[ControlMessage::UdpGsoSegments(s), ControlMessage::Ipv6TClass(tc)],
                        MsgFlags::empty(),
                        Some(&addr),
                    )
                }
                (true, false, _) => {
                    let s = item.segment_size.as_ref().unwrap();
                    sendmsg(fd, &iov, &[ControlMessage::UdpGsoSegments(s)], MsgFlags::empty(), Some(&addr))
                }
                (false, true, true) => {
                    let tos = &ecn_to_tos(item.ecn.unwrap());
                    sendmsg(fd, &iov, &[ControlMessage::Ipv4Tos(tos)], MsgFlags::empty(), Some(&addr))
                }
                (false, true, false) => {
                    let tc = &(ecn_to_tos(item.ecn.unwrap()) as i32);
                    sendmsg(fd, &iov, &[ControlMessage::Ipv6TClass(tc)], MsgFlags::empty(), Some(&addr))
                }
                (false, false, _) => sendmsg::<SockaddrStorage>(fd, &iov, &[], MsgFlags::empty(), Some(&addr)),
            }
            .map_err(nix_err)?;
            Ok(())
        }

        const SENDMMSG_MAX: usize = 64;
        const SENDMMSG_CMSG: usize = 32;

        fn fill_sockaddr(addr: &SocketAddr, ss: &mut libc::sockaddr_storage) -> libc::socklen_t {
            unsafe {
                std::ptr::write_bytes(ss as *mut _ as *mut u8, 0, std::mem::size_of::<libc::sockaddr_storage>());
            }
            match addr {
                SocketAddr::V4(v4) => {
                    let sin = ss as *mut _ as *mut libc::sockaddr_in;
                    unsafe {
                        (*sin).sin_family = libc::AF_INET as u16;
                        (*sin).sin_port = v4.port().to_be();
                        (*sin).sin_addr.s_addr = u32::from_be_bytes(v4.ip().octets()).to_be();
                    }
                    std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t
                }
                SocketAddr::V6(v6) => {
                    let sin6 = ss as *mut _ as *mut libc::sockaddr_in6;
                    unsafe {
                        (*sin6).sin6_family = libc::AF_INET6 as u16;
                        (*sin6).sin6_port = v6.port().to_be();
                        (*sin6).sin6_addr.s6_addr = v6.ip().octets();
                        (*sin6).sin6_flowinfo = v6.flowinfo().to_be();
                        (*sin6).sin6_scope_id = v6.scope_id();
                    }
                    std::mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t
                }
            }
        }

        fn encode_ecn_cmsg(buf: &mut [u8], ecn: Option<Ecn>, is_v6: bool) -> usize {
            let ecn = match ecn {
                Some(e) => e,
                None => return 0,
            };
            let mut mhdr: libc::msghdr = unsafe { std::mem::zeroed() };
            mhdr.msg_control = buf.as_mut_ptr() as *mut _;
            mhdr.msg_controllen = buf.len() as _;
            let cmsg = unsafe { libc::CMSG_FIRSTHDR(&mhdr) };
            if is_v6 {
                unsafe {
                    (*cmsg).cmsg_level = libc::IPPROTO_IPV6;
                    (*cmsg).cmsg_type = libc::IPV6_TCLASS;
                    (*cmsg).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<i32>() as u32) as _;
                    *(libc::CMSG_DATA(cmsg) as *mut i32) = ecn.to_tos_bits() as i32;
                }
                unsafe { libc::CMSG_SPACE(std::mem::size_of::<i32>() as u32) as usize }
            } else {
                unsafe {
                    (*cmsg).cmsg_level = libc::IPPROTO_IP;
                    (*cmsg).cmsg_type = libc::IP_TOS;
                    (*cmsg).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<u8>() as u32) as _;
                    *libc::CMSG_DATA(cmsg) = ecn.to_tos_bits();
                }
                unsafe { libc::CMSG_SPACE(std::mem::size_of::<u8>() as u32) as usize }
            }
        }

        fn sendmmsg_batch(fd: std::os::fd::RawFd, items: &[SendItem], caps: &Capabilities) -> io::Result<usize> {
            let mut total_sent = 0;

            while total_sent < items.len() {
                let chunk_len = (items.len() - total_sent).min(SENDMMSG_MAX);
                let chunk = &items[total_sent..total_sent + chunk_len];

                let mut addrs: [libc::sockaddr_storage; SENDMMSG_MAX] = unsafe { std::mem::zeroed() };
                let mut iovs: [libc::iovec; SENDMMSG_MAX] = unsafe { std::mem::zeroed() };
                let mut cmsg_bufs: [[u8; SENDMMSG_CMSG]; SENDMMSG_MAX] = [[0u8; SENDMMSG_CMSG]; SENDMMSG_MAX];
                let mut msgs: [libc::mmsghdr; SENDMMSG_MAX] = unsafe { std::mem::zeroed() };

                for i in 0..chunk_len {
                    let item = &chunk[i];
                    let addr_len = fill_sockaddr(&item.destination, &mut addrs[i]);
                    let cmsg_len = if caps.ecn {
                        encode_ecn_cmsg(&mut cmsg_bufs[i], item.ecn, item.destination.is_ipv6())
                    } else {
                        0
                    };

                    iovs[i].iov_base = item.data.as_ptr() as *mut _;
                    iovs[i].iov_len = item.data.len();

                    let mhdr = &mut msgs[i].msg_hdr;
                    mhdr.msg_name = &mut addrs[i] as *mut _ as *mut _;
                    mhdr.msg_namelen = addr_len;
                    mhdr.msg_iov = &mut iovs[i] as *mut _ as *mut _;
                    mhdr.msg_iovlen = 1;
                    mhdr.msg_control = cmsg_bufs[i].as_mut_ptr() as *mut _;
                    mhdr.msg_controllen = cmsg_len as _;
                    mhdr.msg_flags = 0;
                }

                let ret = unsafe { libc::sendmmsg(fd, msgs.as_mut_ptr(), chunk_len as u32, 0) };
                if ret < 0 {
                    let e = io::Error::last_os_error();
                    if total_sent > 0 && e.kind() == io::ErrorKind::WouldBlock {
                        return Ok(total_sent);
                    }
                    return Err(e);
                }
                let sent = ret as usize;
                total_sent += sent;
                if sent < chunk_len {
                    break;
                }
            }

            Ok(total_sent)
        }

        pub fn send_batch(
            fd: std::os::fd::RawFd,
            items: &[SendItem<'_>],
            caps: &Capabilities,
            gso_einval: &mut bool,
            chunk_offset: &mut usize,
        ) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }

            // Single-item GSO fast path — only attempted once
            if items.len() == 1 && items[0].segment_size.is_some() && caps.gso && !*gso_einval {
                match send_one(fd, &items[0], caps.ecn, true) {
                    Ok(()) => {
                        *chunk_offset = 0;
                        return Ok(1);
                    }
                    Err(e) if e.raw_os_error() == Some(libc::EINVAL) => {
                        *gso_einval = true;
                        // Fall through — GSO rejected by this interface,
                        // manually chunk below
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Err(e),
                    Err(e) => return Err(e),
                }
            }

            // Multi-item non-segmented batch via sendmmsg
            if caps.sendmmsg && items.len() > 1 && !items.iter().any(|i| i.segment_size.is_some()) {
                return sendmmsg_batch(fd, items, caps);
            }

            let mut count = 0usize;
            for item in items {
                if let Some(seg) = item.segment_size {
                    // Manual chunking — GSO unavailable or rejected
                    let seg = seg as usize;
                    let data_slice = &item.data[*chunk_offset..];
                    for chunk in data_slice.chunks(seg) {
                        let mut chunk_item = SendItem::new(item.destination, chunk);
                        if let Some(ecn) = item.ecn {
                            chunk_item = chunk_item.ecn(ecn);
                        }
                        match send_one(fd, &chunk_item, caps.ecn, false) {
                            Ok(()) => *chunk_offset += chunk.len(),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                if count == 0 && *chunk_offset == 0 {
                                    return Err(e);
                                }
                                return Ok(count);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    *chunk_offset = 0;
                    count += 1;
                } else {
                    match send_one(fd, item, caps.ecn, false) {
                        Ok(()) => count += 1,
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            if count == 0 {
                                return Err(e);
                            }
                            return Ok(count);
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
            Ok(count)
        }

        const RECV_CMSG_BUF_SIZE: usize = 128;

        fn recv_one(fd: std::os::fd::RawFd, item: &mut ReceiveItem<'_>, ecn_enabled: bool) -> io::Result<()> {
            let mut iov = [std::io::IoSliceMut::new(item.buf)];
            let mut cmsg_buf = [0u8; RECV_CMSG_BUF_SIZE];

            let result =
                recvmsg::<SockaddrStorage>(fd, &mut iov, Some(&mut cmsg_buf), MsgFlags::empty()).map_err(nix_err)?;

            item.len = result.bytes;
            item.source = result
                .address
                .as_ref()
                .map(storage_to_addr)
                .unwrap_or_else(|| SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0));

            if ecn_enabled && let Ok(cmsgs) = result.cmsgs() {
                let collected: Vec<_> = cmsgs.collect();
                item.ecn = parse_ecn(&collected);
            }

            Ok(())
        }

        fn recv_gro(fd: std::os::fd::RawFd, items: &mut [ReceiveItem<'_>], ecn_enabled: bool) -> io::Result<usize> {
            let mut gro_buf = [0u8; 65536];
            let mut iov = [std::io::IoSliceMut::new(&mut gro_buf)];
            let mut cmsg_buf = [0u8; RECV_CMSG_BUF_SIZE];

            let result =
                recvmsg::<SockaddrStorage>(fd, &mut iov, Some(&mut cmsg_buf), MsgFlags::empty()).map_err(nix_err)?;

            let total = result.bytes;
            let source = result
                .address
                .as_ref()
                .map(storage_to_addr)
                .unwrap_or_else(|| SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0));

            // Single-pass cmsg parsing — always inspect GRO segment info
            // (GRO is independent of ECN). No heap allocation.
            let mut ecn_val: Option<Ecn> = None;
            let mut seg_size: Option<u32> = None;
            if let Ok(cmsgs) = result.cmsgs() {
                for cmsg in cmsgs {
                    match cmsg {
                        nix::sys::socket::ControlMessageOwned::UdpGroSegments(seg) if seg_size.is_none() => {
                            seg_size = Some(seg as u32);
                        }
                        nix::sys::socket::ControlMessageOwned::Ipv4Tos(tos) if ecn_enabled && ecn_val.is_none() => {
                            ecn_val = Some(Ecn::from_tos_bits(tos));
                        }
                        nix::sys::socket::ControlMessageOwned::Ipv6TClass(tc) if ecn_enabled && ecn_val.is_none() => {
                            ecn_val = Some(Ecn::from_tos_bits(tc as u8));
                        }
                        _ => {}
                    }
                }
            }

            if seg_size.is_none() {
                if items.is_empty() {
                    return Ok(0);
                }
                let copy_len = total.min(items[0].buf.len());
                items[0].buf[..copy_len].copy_from_slice(&gro_buf[..copy_len]);
                items[0].len = copy_len;
                items[0].source = source;
                items[0].ecn = ecn_val;
                return Ok(1);
            }

            let seg = seg_size.unwrap() as usize;
            let mut offset = 0;
            let mut count = 0;
            for item in items.iter_mut() {
                if offset >= total {
                    break;
                }
                let seg_end = (offset + seg).min(total);
                let copy_len = (seg_end - offset).min(item.buf.len());
                item.buf[..copy_len].copy_from_slice(&gro_buf[offset..offset + copy_len]);
                item.len = copy_len;
                item.source = source;
                item.ecn = ecn_val;
                count += 1;
                offset = seg_end;
            }
            Ok(count)
        }

        pub fn recv_batch(
            fd: std::os::fd::RawFd,
            items: &mut [ReceiveItem<'_>],
            caps: &Capabilities,
        ) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }

            if caps.gro {
                return recv_gro(fd, items, caps.ecn);
            }

            let mut count = 0;
            for item in items.iter_mut() {
                match recv_one(fd, item, caps.ecn) {
                    Ok(()) => count += 1,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if count == 0 {
                            return Err(e);
                        }
                        return Ok(count);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(count)
        }

        fn probe_gso(fd: &OwnedFd) -> bool {
            use nix::sys::socket::{getsockopt, sockopt::UdpGsoSegment};
            getsockopt(fd, UdpGsoSegment).is_ok()
        }

        fn probe_gro(fd: &OwnedFd) -> bool {
            use nix::sys::socket::{setsockopt, sockopt::UdpGroSegment};
            setsockopt(fd, UdpGroSegment, &true).is_ok()
        }

        fn enable_ecn_recv(fd: &OwnedFd, is_ipv6: bool) -> bool {
            use nix::sys::socket::{
                setsockopt,
                sockopt::{IpRecvTos, Ipv6RecvTClass},
            };
            if is_ipv6 {
                setsockopt(fd, Ipv6RecvTClass, &true).is_ok()
            } else {
                setsockopt(fd, IpRecvTos, &true).is_ok()
            }
        }

        pub fn setup_and_probe(fd: &OwnedFd, config: &BuildConfig, is_ipv6: bool) -> io::Result<Capabilities> {
            use nix::sys::socket::{setsockopt, sockopt::ReuseAddr};

            setsockopt(fd, ReuseAddr, &true).map_err(nix_err)?;

            let gso = if config.disable_gso { false } else { probe_gso(fd) };
            let gro = if config.disable_gro { false } else { probe_gro(fd) };
            let sendmmsg = !config.disable_sendmmsg;
            let ecn = if config.disable_ecn {
                false
            } else {
                enable_ecn_recv(fd, is_ipv6)
            };

            Ok(Capabilities {
                gso,
                gro,
                sendmmsg,
                ecn,
                max_batch: config.max_batch,
            })
        }

        pub fn local_addr(fd: &OwnedFd) -> io::Result<SocketAddr> {
            use nix::sys::socket::getsockname;
            let addr: SockaddrStorage = getsockname(fd.as_raw_fd()).map_err(nix_err)?;
            Ok(storage_to_addr(&addr))
        }
    }

    // -- Generic Unix fallback -------------------------------------------------
    #[cfg(all(unix, not(target_os = "linux")))]
    mod unix_inner {
        use std::{
            io,
            net::SocketAddr,
            os::fd::{AsRawFd, OwnedFd},
        };

        use nix::sys::socket::{ControlMessage, MsgFlags, SockaddrStorage, recvmsg, sendmsg};

        use super::{BuildConfig, ecn_to_tos, nix_err, parse_ecn, storage_to_addr};
        use crate::{
            capability::Capabilities,
            item::{ReceiveItem, SendItem},
        };

        fn send_one(fd: std::os::fd::RawFd, item: &SendItem, ecn_enabled: bool) -> io::Result<()> {
            let iov = [std::io::IoSlice::new(item.data)];
            let addr = SockaddrStorage::from(item.destination);

            let has_ecn = ecn_enabled && item.ecn.is_some();
            let is_v4 = item.destination.is_ipv4();

            match (has_ecn, is_v4) {
                (true, true) => {
                    let tos = &ecn_to_tos(item.ecn.unwrap());
                    sendmsg(fd, &iov, &[ControlMessage::Ipv4Tos(tos)], MsgFlags::empty(), Some(&addr))
                }
                (true, false) => {
                    let tc = &(ecn_to_tos(item.ecn.unwrap()) as i32);
                    sendmsg(fd, &iov, &[ControlMessage::Ipv6TClass(tc)], MsgFlags::empty(), Some(&addr))
                }
                (false, _) => sendmsg::<SockaddrStorage>(fd, &iov, &[], MsgFlags::empty(), Some(&addr)),
            }
            .map_err(nix_err)?;
            Ok(())
        }

        pub fn send_batch(fd: std::os::fd::RawFd, items: &[SendItem<'_>], caps: &Capabilities) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }

            let mut count = 0;
            for item in items {
                match send_one(fd, item, caps.ecn) {
                    Ok(()) => count += 1,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if count == 0 {
                            return Err(e);
                        }
                        return Ok(count);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(count)
        }

        const RECV_CMSG_BUF_SIZE: usize = 64;

        fn recv_one(fd: std::os::fd::RawFd, item: &mut ReceiveItem<'_>, ecn_enabled: bool) -> io::Result<()> {
            let mut iov = [std::io::IoSliceMut::new(item.buf)];
            let mut cmsg_buf = [0u8; RECV_CMSG_BUF_SIZE];

            let result =
                recvmsg::<SockaddrStorage>(fd, &mut iov, Some(&mut cmsg_buf), MsgFlags::empty()).map_err(nix_err)?;

            item.len = result.bytes;
            item.source = result
                .address
                .as_ref()
                .map(storage_to_addr)
                .unwrap_or_else(|| SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0));

            if ecn_enabled {
                if let Ok(cmsgs) = result.cmsgs() {
                    let collected: Vec<_> = cmsgs.collect();
                    item.ecn = parse_ecn(&collected);
                }
            }

            Ok(())
        }

        pub fn recv_batch(
            fd: std::os::fd::RawFd,
            items: &mut [ReceiveItem<'_>],
            caps: &Capabilities,
        ) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }

            let mut count = 0;
            for item in items.iter_mut() {
                match recv_one(fd, item, caps.ecn) {
                    Ok(()) => count += 1,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if count == 0 {
                            return Err(e);
                        }
                        return Ok(count);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(count)
        }

        pub fn setup_and_probe(fd: &OwnedFd, config: &BuildConfig, is_ipv6: bool) -> io::Result<Capabilities> {
            use nix::sys::socket::{setsockopt, sockopt::ReuseAddr};

            setsockopt(fd, ReuseAddr, &true).map_err(nix_err)?;

            let ecn = if config.disable_ecn {
                false
            } else {
                use nix::sys::socket::sockopt::{IpRecvTos, Ipv6RecvTClass};
                if is_ipv6 {
                    setsockopt(fd, Ipv6RecvTClass, &true).is_ok()
                } else {
                    setsockopt(fd, IpRecvTos, &true).is_ok()
                }
            };

            Ok(Capabilities {
                gso: false,
                gro: false,
                sendmmsg: false,
                ecn,
                max_batch: config.max_batch,
            })
        }

        pub fn local_addr(fd: &OwnedFd) -> io::Result<SocketAddr> {
            use nix::sys::socket::getsockname;
            let addr: SockaddrStorage = getsockname(fd.as_raw_fd()).map_err(nix_err)?;
            Ok(storage_to_addr(&addr))
        }
    }
}

// ---------------------------------------------------------------------------
// Windows implementation
// ---------------------------------------------------------------------------
#[cfg(windows)]
mod windows_socket_inner {
    use std::{io, net::SocketAddr};

    use tokio::net::UdpSocket;

    use crate::{
        capability::Capabilities,
        ecn::Ecn,
        item::{ReceiveItem, SendItem},
    };

    pub struct FastUdpSocket {
        socket: UdpSocket,
        caps: Capabilities,
    }

    impl FastUdpSocket {
        pub async fn send(&self, item: SendItem<'_>) -> io::Result<()> {
            self.socket.send_to(item.data, item.destination).await?;
            Ok(())
        }

        pub async fn send_many(&self, items: &[SendItem<'_>]) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }
            let mut count = 0;
            for item in items {
                match self.socket.send_to(item.data, item.destination).await {
                    Ok(()) => count += 1,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if count == 0 {
                            return Err(e);
                        }
                        return Ok(count);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(count)
        }

        pub async fn receive(&self, item: &mut ReceiveItem<'_>) -> io::Result<()> {
            let (n, addr) = self.socket.recv_from(item.buf).await?;
            item.len = n;
            item.source = addr;
            Ok(())
        }

        pub async fn receive_many(&self, items: &mut [ReceiveItem<'_>]) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }
            let mut count = 0;
            for item in items.iter_mut() {
                match self.socket.recv_from(item.buf).await {
                    Ok((n, addr)) => {
                        item.len = n;
                        item.source = addr;
                        count += 1;
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if count == 0 {
                            return Err(e);
                        }
                        return Ok(count);
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(count)
        }

        pub fn capabilities(&self) -> &Capabilities {
            &self.caps
        }

        pub fn local_addr(&self) -> io::Result<SocketAddr> {
            self.socket.local_addr()
        }
    }

    pub struct FastUdpSocketBuilder {
        addr: SocketAddr,
        max_batch: usize,
    }

    impl FastUdpSocketBuilder {
        pub fn bind(addr: SocketAddr) -> Self {
            FastUdpSocketBuilder {
                addr,
                max_batch: 64,
            }
        }

        pub fn disable_gso(self) -> Self {
            self
        }

        pub fn disable_gro(self) -> Self {
            self
        }

        pub fn disable_sendmmsg(self) -> Self {
            self
        }

        pub fn disable_ecn(self) -> Self {
            self
        }

        pub fn max_batch_size(mut self, n: usize) -> Self {
            self.max_batch = n;
            self
        }

        pub fn build(self) -> io::Result<FastUdpSocket> {
            let std_socket = std::net::UdpSocket::bind(self.addr)?;
            let socket = UdpSocket::from_std(std_socket);

            Ok(FastUdpSocket {
                socket,
                caps: Capabilities::none(),
            })
        }
    }
}

// -- re-exports ------------------------------------------------------------
#[cfg(unix)]
pub use unix_socket::{FastUdpSocket, FastUdpSocketBuilder};
#[cfg(windows)]
pub use windows_socket_inner::{FastUdpSocket, FastUdpSocketBuilder};
