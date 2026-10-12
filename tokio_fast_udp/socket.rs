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

    #[cfg(any(target_os = "linux", target_vendor = "apple"))]
    use crate::ecn::Ecn;
    use crate::{
        capability::Capabilities,
        item::{ReceiveItem, SendItem},
    };

    // -- FastUdpSocket ---------------------------------------------------------
    pub struct FastUdpSocket {
        fd: AsyncFd<OwnedFd>,
        caps: Capabilities,
    }

    impl FastUdpSocket {
        pub fn build(addr: SocketAddr) -> FastUdpSocketBuilder {
            FastUdpSocketBuilder {
                addr,
                disable_gso: false,
                disable_gro: false,
                disable_sendmmsg: false,
                disable_ecn: false,
                max_batch: 64,
                enable_reuse_port: false,
            }
        }
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
            let mut state = plat::SendState::default();
            self.fd
                .async_io(Interest::WRITABLE, |inner| {
                    plat::send_batch(inner.as_raw_fd(), items, caps, &mut state)
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
        enable_reuse_port: bool,
    }

    impl FastUdpSocketBuilder {
        pub fn gso(mut self, enable: bool) -> Self {
            self.disable_gso = !enable;
            self
        }

        pub fn gro(mut self, enable: bool) -> Self {
            self.disable_gro = !enable;
            self
        }

        pub fn sendmmsg(mut self, enable: bool) -> Self {
            self.disable_sendmmsg = !enable;
            self
        }

        pub fn ecn(mut self, enable: bool) -> Self {
            self.disable_ecn = !enable;
            self
        }

        pub fn reuse_port(mut self, enable: bool) -> Self {
            self.enable_reuse_port = enable;
            self
        }

        pub fn max_batch_size(mut self, n: usize) -> Self {
            self.max_batch = n;
            self
        }

        pub fn bind(self) -> io::Result<FastUdpSocket> {
            let is_ipv6 = self.addr.is_ipv6();

            let fd: OwnedFd = if self.enable_reuse_port {
                create_socket_with_reuse_port(self.addr)?
            } else {
                let std_socket = std::net::UdpSocket::bind(self.addr)?;
                std_socket.set_nonblocking(true)?;

                use std::os::fd::{FromRawFd, IntoRawFd};
                let raw_fd = std_socket.into_raw_fd();
                unsafe { OwnedFd::from_raw_fd(raw_fd) }
            };

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

    #[cfg(target_os = "linux")]
    use nix::sys::socket::ControlMessageOwned;
    use nix::sys::socket::SockaddrStorage;

    fn nix_err(e: nix::errno::Errno) -> io::Error {
        e.into()
    }

    fn create_socket_with_reuse_port(addr: SocketAddr) -> io::Result<OwnedFd> {
        use nix::sys::socket::{AddressFamily, SockFlag, SockType, bind, setsockopt, socket, sockopt::ReusePort};

        let is_ipv6 = addr.is_ipv6();
        let domain = if is_ipv6 {
            AddressFamily::Inet6
        } else {
            AddressFamily::Inet
        };

        // `SOCK_NONBLOCK` / `SOCK_CLOEXEC` are not accepted by `socket()` on
        // every Unix (notably macOS), so create a plain socket and apply the
        // flags portably via `fcntl`.
        let fd = socket(domain, SockType::Datagram, SockFlag::empty(), None).map_err(nix_err)?;
        set_cloexec(&fd)?;
        set_nonblocking(&fd)?;

        setsockopt(&fd, ReusePort, &true).map_err(nix_err)?;

        let sock_addr = SockaddrStorage::from(addr);
        bind(fd.as_raw_fd(), &sock_addr).map_err(nix_err)?;

        Ok(fd)
    }

    /// Set `FD_CLOEXEC` on `fd` using the portable `fcntl` interface.
    fn set_cloexec(fd: &OwnedFd) -> io::Result<()> {
        let ret = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Put `fd` into non-blocking mode using the portable `fcntl` interface.
    fn set_nonblocking(fd: &OwnedFd) -> io::Result<()> {
        let flags = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETFL) };
        if flags < 0 {
            return Err(io::Error::last_os_error());
        }
        let ret = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_vendor = "apple"))]
    fn ecn_to_tos(ecn: Ecn) -> u8 {
        ecn.to_tos_bits()
    }

    /// Decode ECN from the `Ipv4Tos` / `Ipv6TClass` ancillary messages that
    /// nix decodes natively on Linux.
    #[cfg(target_os = "linux")]
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

    // Fields for Linux-only offload features are still populated by the
    // builder on every platform, but only the Linux path reads them.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
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

        /// Per-call state threaded through [`send_batch`] so it survives the
        /// retries performed by `AsyncFd::async_io` (GSO `EINVAL` back-off and
        /// manual-chunking progress).
        #[derive(Default)]
        pub struct SendState {
            pub gso_einval: bool,
            pub chunk_offset: usize,
        }

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
            state: &mut SendState,
        ) -> io::Result<usize> {
            if items.is_empty() {
                return Ok(0);
            }

            // Single-item GSO fast path — only attempted once
            if items.len() == 1 && items[0].segment_size.is_some() && caps.gso && !state.gso_einval {
                match send_one(fd, &items[0], caps.ecn, true) {
                    Ok(()) => {
                        state.chunk_offset = 0;
                        return Ok(1);
                    }
                    Err(e) if e.raw_os_error() == Some(libc::EINVAL) => {
                        state.gso_einval = true;
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
                    let data_slice = &item.data[state.chunk_offset..];
                    for chunk in data_slice.chunks(seg) {
                        let mut chunk_item = SendItem::new(item.destination, chunk);
                        if let Some(ecn) = item.ecn {
                            chunk_item = chunk_item.ecn(ecn);
                        }
                        match send_one(fd, &chunk_item, caps.ecn, false) {
                            Ok(()) => state.chunk_offset += chunk.len(),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                if count == 0 && state.chunk_offset == 0 {
                                    return Err(e);
                                }
                                return Ok(count);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    state.chunk_offset = 0;
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

        #[cfg(target_vendor = "apple")]
        use nix::sys::socket::SockaddrLike;
        use nix::sys::socket::{ControlMessageOwned, MsgFlags, SockaddrStorage, recvmsg, sendmsg};

        #[cfg(target_vendor = "apple")]
        use super::ecn_to_tos;
        use super::{BuildConfig, nix_err, storage_to_addr};
        use crate::{
            capability::Capabilities,
            ecn::Ecn,
            item::{ReceiveItem, SendItem},
        };

        /// The generic-Unix send path carries no per-call state; this exists
        /// only to match the contract used by `send_many`.
        #[derive(Default)]
        pub struct SendState;

        /// Control-message buffer, aligned for `cmsghdr`.
        #[cfg(target_vendor = "apple")]
        #[repr(align(8))]
        struct CmsgBuf([u8; 32]);

        fn send_one(fd: std::os::fd::RawFd, item: &SendItem, ecn_enabled: bool) -> io::Result<()> {
            // ECN on Apple targets has to be sent as hand-built ancillary data,
            // because nix does not expose the relevant control messages there.
            #[cfg(target_vendor = "apple")]
            if ecn_enabled && item.ecn.is_some() {
                return send_one_ecn(fd, item);
            }
            #[cfg(not(target_vendor = "apple"))]
            let _ = ecn_enabled;

            let iov = [std::io::IoSlice::new(item.data)];
            let addr = SockaddrStorage::from(item.destination);
            sendmsg::<SockaddrStorage>(fd, &iov, &[], MsgFlags::empty(), Some(&addr)).map_err(nix_err)?;
            Ok(())
        }

        /// Send a single datagram carrying an ECN codepoint.
        #[cfg(target_vendor = "apple")]
        fn send_one_ecn(fd: std::os::fd::RawFd, item: &SendItem) -> io::Result<()> {
            let Some(ecn) = item.ecn else {
                return Ok(());
            };
            let addr = SockaddrStorage::from(item.destination);

            let mut iov = libc::iovec {
                iov_base: item.data.as_ptr() as *mut libc::c_void,
                iov_len: item.data.len(),
            };

            let mut cmsg = CmsgBuf([0u8; 32]);
            let cmsg_len = encode_ecn_cmsg(&mut cmsg.0, ecn, item.destination.is_ipv6());

            let mut mhdr: libc::msghdr = unsafe { std::mem::zeroed() };
            mhdr.msg_name = addr.as_ptr() as *mut libc::c_void;
            mhdr.msg_namelen = addr.len();
            mhdr.msg_iov = &mut iov as *mut libc::iovec;
            mhdr.msg_iovlen = 1;
            mhdr.msg_control = cmsg.0.as_mut_ptr() as *mut libc::c_void;
            mhdr.msg_controllen = cmsg_len as _;

            let ret = unsafe { libc::sendmsg(fd, &mhdr, 0) };
            if ret < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        }

        /// Build the per-packet ECN control message.
        ///
        /// On Apple targets, outgoing ECN is expressed as an `int` with level
        /// `IPPROTO_IP`/type `IP_TOS` for IPv4 and level `IPPROTO_IPV6`/type
        /// `IPV6_TCLASS` for IPv6. nix does not expose these control messages
        /// on Apple targets, so they are built directly with `libc`.
        #[cfg(target_vendor = "apple")]
        fn encode_ecn_cmsg(buf: &mut [u8], ecn: Ecn, is_v6: bool) -> usize {
            let mut mhdr: libc::msghdr = unsafe { std::mem::zeroed() };
            mhdr.msg_control = buf.as_mut_ptr() as *mut _;
            mhdr.msg_controllen = buf.len() as _;

            let cmsg = unsafe { libc::CMSG_FIRSTHDR(&mhdr) };
            if cmsg.is_null() {
                return 0;
            }

            let (level, ty) = if is_v6 {
                (libc::IPPROTO_IPV6, libc::IPV6_TCLASS)
            } else {
                (libc::IPPROTO_IP, libc::IP_TOS)
            };
            let value = ecn_to_tos(ecn) as libc::c_int;
            unsafe {
                (*cmsg).cmsg_level = level;
                (*cmsg).cmsg_type = ty;
                (*cmsg).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<libc::c_int>() as u32) as _;
                *(libc::CMSG_DATA(cmsg) as *mut libc::c_int) = value;
            }
            unsafe { libc::CMSG_SPACE(std::mem::size_of::<libc::c_int>() as u32) as usize }
        }

        /// Decode an ECN codepoint from a received control message.
        ///
        /// nix only decodes `Ipv4Tos` / `Ipv6TClass` on Linux and FreeBSD, so
        /// on Apple targets these arrive as [`ControlMessageOwned::Unknown`]
        /// and are decoded here from their raw level/type.
        #[cfg(target_vendor = "apple")]
        fn parse_ecn_msg(cmsg: &ControlMessageOwned) -> Option<Ecn> {
            let ControlMessageOwned::Unknown(u) = cmsg else {
                return None;
            };
            let level = u.cmsg_header.cmsg_level;
            let ty = u.cmsg_header.cmsg_type;

            // IPv4: `IP_TOS` on Linux/FreeBSD, `IP_RECVTOS` on Apple/BSD.
            if level == libc::IPPROTO_IP && (ty == libc::IP_TOS || ty == libc::IP_RECVTOS) {
                return u.data_bytes.first().map(|b| Ecn::from_tos_bits(*b));
            }
            // IPv6: macOS may report the traffic class as either a byte or an
            // `int` depending on OS version (see Apple bug 48761855).
            if level == libc::IPPROTO_IPV6 && ty == libc::IPV6_TCLASS {
                let int_size = std::mem::size_of::<libc::c_int>();
                if u.data_bytes.len() >= int_size {
                    let mut raw = [0u8; std::mem::size_of::<libc::c_int>()];
                    raw.copy_from_slice(&u.data_bytes[..int_size]);
                    return Some(Ecn::from_tos_bits(i32::from_ne_bytes(raw) as u8));
                }
                return u.data_bytes.first().map(|b| Ecn::from_tos_bits(*b));
            }
            None
        }

        /// ECN receive decoding is only implemented on Apple targets.
        #[cfg(not(target_vendor = "apple"))]
        fn parse_ecn_msg(_cmsg: &ControlMessageOwned) -> Option<Ecn> {
            None
        }

        pub fn send_batch(
            fd: std::os::fd::RawFd,
            items: &[SendItem<'_>],
            caps: &Capabilities,
            _state: &mut SendState,
        ) -> io::Result<usize> {
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

            if ecn_enabled && let Ok(mut cmsgs) = result.cmsgs() {
                item.ecn = cmsgs.find_map(|cmsg| parse_ecn_msg(&cmsg));
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

        /// Enable reception of ECN codepoints.
        ///
        /// Uses `libc` directly because nix does not expose `IP_RECVTOS` /
        /// `IPV6_RECVTCLASS` on Apple targets. An IPv4 socket reports TOS via
        /// `IP_RECVTOS`; an IPv6 socket reports the traffic class via
        /// `IPV6_RECVTCLASS`, which also covers IPv4-mapped traffic. Setting
        /// `IP_RECVTOS` on a dual-stack IPv6 socket fails on macOS, so only the
        /// option matching the socket family is set.
        #[cfg(target_vendor = "apple")]
        fn enable_ecn_recv(fd: &OwnedFd, is_ipv6: bool) -> bool {
            let (level, name) = if is_ipv6 {
                (libc::IPPROTO_IPV6, libc::IPV6_RECVTCLASS)
            } else {
                (libc::IPPROTO_IP, libc::IP_RECVTOS)
            };
            let enable: libc::c_int = 1;
            let ret = unsafe {
                libc::setsockopt(
                    fd.as_raw_fd(),
                    level,
                    name,
                    &enable as *const libc::c_int as *const libc::c_void,
                    std::mem::size_of::<libc::c_int>() as libc::socklen_t,
                )
            };
            ret == 0
        }

        /// ECN is only implemented on Apple targets among non-Linux Unixes.
        #[cfg(not(target_vendor = "apple"))]
        fn enable_ecn_recv(_fd: &OwnedFd, _is_ipv6: bool) -> bool {
            false
        }

        pub fn setup_and_probe(fd: &OwnedFd, config: &BuildConfig, is_ipv6: bool) -> io::Result<Capabilities> {
            use nix::sys::socket::{setsockopt, sockopt::ReuseAddr};

            setsockopt(fd, ReuseAddr, &true).map_err(nix_err)?;

            let ecn = !config.disable_ecn && enable_ecn_recv(fd, is_ipv6);

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

    impl FastUdpSocket {
        pub fn build(addr: SocketAddr) -> FastUdpSocketBuilder {
            FastUdpSocketBuilder::new(addr)
        }
    }

    pub struct FastUdpSocketBuilder {
        addr: SocketAddr,
        max_batch: usize,
    }

    impl FastUdpSocketBuilder {
        pub fn new(addr: SocketAddr) -> Self {
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

        pub fn enable_reuse_port(self) -> Self {
            self
        }

        pub fn bind(self) -> io::Result<FastUdpSocket> {
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
