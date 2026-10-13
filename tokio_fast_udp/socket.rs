// FastUdpSocket and FastUdpSocketBuilder for Unix (AsyncFd + libc syscalls).

// ---------------------------------------------------------------------------
// Unix implementation
// ---------------------------------------------------------------------------
#[cfg(unix)]
mod unix_socket {
    use std::{
        io,
        net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6},
        os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd},
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
            socket_local_addr(self.fd.get_ref())
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

    // -- shared syscall helpers (used by the platform inner modules via super::) --

    /// Size of the ancillary-data buffer used for one datagram.
    ///
    /// The largest control-message set this crate produces is a GSO segment size
    /// plus an ECN codepoint, which fits comfortably.
    const CMSG_BUF_SIZE: usize = 128;

    /// Alignment-safe outgoing ancillary-data buffer.
    #[repr(align(8))]
    struct CmsgBuf {
        buf: [u8; CMSG_BUF_SIZE],
        len: usize,
    }

    impl CmsgBuf {
        fn new() -> Self {
            Self {
                buf: [0u8; CMSG_BUF_SIZE],
                len: 0,
            }
        }

        /// Append one control message, returning `false` if it does not fit.
        #[cfg(any(target_os = "linux", target_vendor = "apple"))]
        fn push(&mut self, level: libc::c_int, ty: libc::c_int, data: *const u8, data_len: usize) -> bool {
            let space = unsafe { libc::CMSG_SPACE(data_len as u32) as usize };
            if self.len + space > self.buf.len() {
                return false;
            }

            // Control messages are laid out sequentially, each occupying
            // `CMSG_SPACE(payload_len)` bytes starting at the previous message's
            // end. The buffer is aligned via `#[repr(align(8))]`.
            let header = unsafe { self.buf.as_mut_ptr().add(self.len) as *mut libc::cmsghdr };
            unsafe {
                (*header).cmsg_len = libc::CMSG_LEN(data_len as u32) as _;
                (*header).cmsg_level = level;
                (*header).cmsg_type = ty;
                std::ptr::copy_nonoverlapping(data, libc::CMSG_DATA(header), data_len);
            }
            self.len += space;
            true
        }

        /// The kernel only reads the ancillary data on `sendmsg`.
        fn as_ptr(&self) -> *const libc::c_void {
            self.buf.as_ptr() as *const libc::c_void
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    /// Alignment-safe buffer for received ancillary data.
    #[repr(align(8))]
    struct RecvCmsgBuf([u8; CMSG_BUF_SIZE]);

    impl RecvCmsgBuf {
        fn new() -> Self {
            RecvCmsgBuf([0u8; CMSG_BUF_SIZE])
        }
    }

    /// Set a boolean socket option (an `int` read as `0`/`1`).
    fn set_bool_opt(fd: std::os::fd::RawFd, level: libc::c_int, name: libc::c_int) -> io::Result<()> {
        let enable: libc::c_int = 1;
        let ret = unsafe {
            libc::setsockopt(
                fd,
                level,
                name,
                &enable as *const libc::c_int as *const libc::c_void,
                std::mem::size_of::<libc::c_int>() as libc::socklen_t,
            )
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Read an `int`-valued socket option.
    #[cfg(target_os = "linux")]
    fn get_int_opt(fd: std::os::fd::RawFd, level: libc::c_int, name: libc::c_int) -> io::Result<libc::c_int> {
        let mut value: libc::c_int = 0;
        let mut len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
        let ret =
            unsafe { libc::getsockopt(fd, level, name, &mut value as *mut libc::c_int as *mut libc::c_void, &mut len) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(value)
    }

    fn create_socket_with_reuse_port(addr: SocketAddr) -> io::Result<OwnedFd> {
        let domain = if addr.is_ipv6() { libc::AF_INET6 } else { libc::AF_INET };

        // `SOCK_NONBLOCK` / `SOCK_CLOEXEC` are not accepted by `socket()` on
        // every Unix (notably macOS), so create a plain socket and apply the
        // flags portably via `fcntl`.
        let raw_fd = unsafe { libc::socket(domain, libc::SOCK_DGRAM, 0) };
        if raw_fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let fd = unsafe { OwnedFd::from_raw_fd(raw_fd) };

        set_cloexec(&fd)?;
        set_nonblocking(&fd)?;
        set_bool_opt(fd.as_raw_fd(), libc::SOL_SOCKET, libc::SO_REUSEPORT)?;

        let mut sock_addr: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let addr_len = fill_sockaddr(&addr, &mut sock_addr);
        let ret = unsafe { libc::bind(fd.as_raw_fd(), &sock_addr as *const _ as *const libc::sockaddr, addr_len) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

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

    /// Fill a `sockaddr_storage` from a `SocketAddr`, returning its length.
    fn fill_sockaddr(addr: &SocketAddr, ss: &mut libc::sockaddr_storage) -> libc::socklen_t {
        unsafe {
            std::ptr::write_bytes(ss as *mut _ as *mut u8, 0, std::mem::size_of::<libc::sockaddr_storage>());
        }
        match addr {
            SocketAddr::V4(v4) => {
                let sin = ss as *mut _ as *mut libc::sockaddr_in;
                unsafe {
                    (*sin).sin_family = libc::AF_INET as libc::sa_family_t;
                    (*sin).sin_port = v4.port().to_be();
                    (*sin).sin_addr.s_addr = u32::from_be_bytes(v4.ip().octets()).to_be();
                }
                // BSD-style sockets carry their own length field.
                #[cfg(any(
                    target_vendor = "apple",
                    target_os = "freebsd",
                    target_os = "dragonfly",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                unsafe {
                    (*sin).sin_len = std::mem::size_of::<libc::sockaddr_in>() as u8;
                }
                std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t
            }
            SocketAddr::V6(v6) => {
                let sin6 = ss as *mut _ as *mut libc::sockaddr_in6;
                unsafe {
                    (*sin6).sin6_family = libc::AF_INET6 as libc::sa_family_t;
                    (*sin6).sin6_port = v6.port().to_be();
                    (*sin6).sin6_addr.s6_addr = v6.ip().octets();
                    (*sin6).sin6_flowinfo = v6.flowinfo().to_be();
                    (*sin6).sin6_scope_id = v6.scope_id();
                }
                #[cfg(any(
                    target_vendor = "apple",
                    target_os = "freebsd",
                    target_os = "dragonfly",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                unsafe {
                    (*sin6).sin6_len = std::mem::size_of::<libc::sockaddr_in6>() as u8;
                }
                std::mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t
            }
        }
    }

    /// Convert a `sockaddr_storage` into a `SocketAddr`.
    ///
    /// Addresses of an unknown family fall back to `0.0.0.0:0`.
    fn storage_to_addr(ss: &libc::sockaddr_storage) -> SocketAddr {
        match ss.ss_family as libc::c_int {
            libc::AF_INET => {
                let sin = unsafe { &*(ss as *const _ as *const libc::sockaddr_in) };
                let ip = Ipv4Addr::from(sin.sin_addr.s_addr.to_ne_bytes());
                SocketAddr::new(IpAddr::V4(ip), u16::from_be(sin.sin_port))
            }
            libc::AF_INET6 => {
                let sin6 = unsafe { &*(ss as *const _ as *const libc::sockaddr_in6) };
                let ip = Ipv6Addr::from(sin6.sin6_addr.s6_addr);
                SocketAddr::V6(SocketAddrV6::new(
                    ip,
                    u16::from_be(sin6.sin6_port),
                    u32::from_be(sin6.sin6_flowinfo),
                    sin6.sin6_scope_id,
                ))
            }
            _ => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        }
    }

    /// Send a single datagram to `destination`, with optional ancillary data.
    fn sendmsg_raw(fd: std::os::fd::RawFd, data: &[u8], destination: &SocketAddr, cmsgs: &CmsgBuf) -> io::Result<()> {
        // `sendmsg` only reads the payload, so the cast from a shared slice to a
        // mutable pointer is sound.
        let mut iov = libc::iovec {
            iov_base: data.as_ptr() as *mut libc::c_void,
            iov_len: data.len(),
        };

        let mut addr: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let addr_len = fill_sockaddr(destination, &mut addr);

        let mut mhdr: libc::msghdr = unsafe { std::mem::zeroed() };
        mhdr.msg_name = &mut addr as *mut _ as *mut libc::c_void;
        mhdr.msg_namelen = addr_len;
        mhdr.msg_iov = &mut iov as *mut libc::iovec;
        mhdr.msg_iovlen = 1;
        mhdr.msg_control = cmsgs.as_ptr() as *mut libc::c_void;
        mhdr.msg_controllen = cmsgs.len() as _;

        let ret = unsafe { libc::sendmsg(fd, &mhdr, 0) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Receive one datagram into `buf`.
    ///
    /// Returns the number of bytes received, the source address, and the decoded
    /// ancillary data: the GRO segment size and the ECN codepoint, if present.
    fn recvmsg_raw(
        fd: std::os::fd::RawFd,
        buf: &mut [u8],
        cmsg_buf: &mut RecvCmsgBuf,
        ecn_enabled: bool,
    ) -> io::Result<(usize, SocketAddr, Option<u32>, Option<Ecn>)> {
        let mut iov = libc::iovec {
            iov_base: buf.as_mut_ptr() as *mut libc::c_void,
            iov_len: buf.len(),
        };
        let mut addr: libc::sockaddr_storage = unsafe { std::mem::zeroed() };

        let mut mhdr: libc::msghdr = unsafe { std::mem::zeroed() };
        mhdr.msg_name = &mut addr as *mut _ as *mut libc::c_void;
        mhdr.msg_namelen = std::mem::size_of::<libc::sockaddr_storage>() as _;
        mhdr.msg_iov = &mut iov as *mut libc::iovec;
        mhdr.msg_iovlen = 1;
        mhdr.msg_control = cmsg_buf.0.as_mut_ptr() as *mut libc::c_void;
        mhdr.msg_controllen = cmsg_buf.0.len() as _;

        let ret = unsafe { libc::recvmsg(fd, &mut mhdr, 0) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

        let (gro_seg, ecn) = decode_cmsgs(&mhdr, ecn_enabled);
        Ok((ret as usize, storage_to_addr(&addr), gro_seg, ecn))
    }

    /// Decode the GRO/ECN ancillary data attached to a received datagram.
    ///
    /// GRO segment sizes and ECN codepoints are read straight from the kernel's
    /// `cmsghdr` layout, since the levels/types and payload widths differ across
    /// platforms (and across Apple OS versions for IPv6 traffic class).
    fn decode_cmsgs(mhdr: &libc::msghdr, ecn_enabled: bool) -> (Option<u32>, Option<Ecn>) {
        #[cfg_attr(not(target_os = "linux"), allow(unused_mut))]
        let mut gro_seg = None;
        let mut ecn = None;

        let mut cmsg = unsafe { libc::CMSG_FIRSTHDR(mhdr) };
        while !cmsg.is_null() {
            let level = unsafe { (*cmsg).cmsg_level };
            let ty = unsafe { (*cmsg).cmsg_type };
            let data = unsafe { libc::CMSG_DATA(cmsg) };
            let header_len = unsafe { libc::CMSG_LEN(0) } as usize;
            let payload_len = (unsafe { (*cmsg).cmsg_len } as usize).saturating_sub(header_len);

            // UDP Generic Receive Offload (Linux): the segment size is an `int`.
            #[cfg(target_os = "linux")]
            if level == libc::SOL_UDP && ty == libc::UDP_GRO && gro_seg.is_none() && payload_len >= 4 {
                gro_seg = Some(unsafe { std::ptr::read_unaligned(data as *const libc::c_int) } as u32);
            }

            if ecn_enabled && ecn.is_none() {
                // IPv4: `IP_TOS` on Linux, `IP_RECVTOS` on the BSDs and Apple,
                // reported as a single byte.
                if level == libc::IPPROTO_IP && (ty == libc::IP_TOS || ty == libc::IP_RECVTOS) && payload_len >= 1 {
                    ecn = Some(Ecn::from_tos_bits(unsafe { std::ptr::read_unaligned(data as *const u8) }));
                }
                // IPv6: `IPV6_TCLASS` is an `int`, but Apple has historically
                // reported it as a single byte (Apple bug 48761855).
                if level == libc::IPPROTO_IPV6 && ty == libc::IPV6_TCLASS {
                    if payload_len >= std::mem::size_of::<libc::c_int>() {
                        let tc = unsafe { std::ptr::read_unaligned(data as *const libc::c_int) };
                        ecn = Some(Ecn::from_tos_bits(tc as u8));
                    } else if payload_len >= 1 {
                        ecn = Some(Ecn::from_tos_bits(unsafe { std::ptr::read_unaligned(data as *const u8) }));
                    }
                }
            }

            cmsg = unsafe { libc::CMSG_NXTHDR(mhdr, cmsg) };
        }

        (gro_seg, ecn)
    }

    /// Append the UDP GSO segment-size control message.
    #[cfg(target_os = "linux")]
    fn push_gso_cmsg(cmsgs: &mut CmsgBuf, segment_size: u16) -> bool {
        cmsgs.push(
            libc::SOL_UDP,
            libc::UDP_SEGMENT,
            &segment_size as *const u16 as *const u8,
            std::mem::size_of::<u16>(),
        )
    }

    /// Append the outgoing ECN codepoint control message.
    #[cfg(any(target_os = "linux", target_vendor = "apple"))]
    fn push_ecn_cmsg(cmsgs: &mut CmsgBuf, ecn: Ecn, is_v6: bool) -> bool {
        if is_v6 {
            let value = ecn_to_tos(ecn) as libc::c_int;
            cmsgs.push(
                libc::IPPROTO_IPV6,
                libc::IPV6_TCLASS,
                &value as *const libc::c_int as *const u8,
                std::mem::size_of::<libc::c_int>(),
            )
        } else if cfg!(target_vendor = "apple") {
            // Apple expects an `int`-valued `IP_TOS`.
            let value = ecn_to_tos(ecn) as libc::c_int;
            cmsgs.push(
                libc::IPPROTO_IP,
                libc::IP_TOS,
                &value as *const libc::c_int as *const u8,
                std::mem::size_of::<libc::c_int>(),
            )
        } else {
            // Linux requires exactly one byte for `IP_TOS`.
            let value = ecn_to_tos(ecn);
            cmsgs.push(libc::IPPROTO_IP, libc::IP_TOS, &value as *const u8, std::mem::size_of::<u8>())
        }
    }

    fn socket_local_addr(fd: &OwnedFd) -> io::Result<SocketAddr> {
        let mut addr: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let mut len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let ret = unsafe { libc::getsockname(fd.as_raw_fd(), &mut addr as *mut _ as *mut libc::sockaddr, &mut len) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(storage_to_addr(&addr))
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
            os::fd::{AsRawFd, OwnedFd},
        };

        use super::{
            BuildConfig, CmsgBuf, RecvCmsgBuf, fill_sockaddr, get_int_opt, push_ecn_cmsg, push_gso_cmsg, recvmsg_raw,
            sendmsg_raw, set_bool_opt,
        };
        use crate::{
            capability::Capabilities,
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
            let mut cmsgs = CmsgBuf::new();

            if gso_enabled
                && let Some(segment_size) = item.segment_size
                && !push_gso_cmsg(&mut cmsgs, segment_size)
            {
                return Err(io::Error::other("control message buffer overflow"));
            }

            if ecn_enabled
                && let Some(ecn) = item.ecn
                && !push_ecn_cmsg(&mut cmsgs, ecn, item.destination.is_ipv6())
            {
                return Err(io::Error::other("control message buffer overflow"));
            }

            sendmsg_raw(fd, item.data, &item.destination, &cmsgs)
        }

        const SENDMMSG_MAX: usize = 64;

        fn sendmmsg_batch(fd: std::os::fd::RawFd, items: &[SendItem], caps: &Capabilities) -> io::Result<usize> {
            let mut total_sent = 0;

            while total_sent < items.len() {
                let chunk_len = (items.len() - total_sent).min(SENDMMSG_MAX);
                let chunk = &items[total_sent..total_sent + chunk_len];

                let mut addrs: [libc::sockaddr_storage; SENDMMSG_MAX] = unsafe { std::mem::zeroed() };
                let mut iovs: [libc::iovec; SENDMMSG_MAX] = unsafe { std::mem::zeroed() };
                let mut cmsg_bufs: [CmsgBuf; SENDMMSG_MAX] = std::array::from_fn(|_| CmsgBuf::new());
                let mut msgs: [libc::mmsghdr; SENDMMSG_MAX] = unsafe { std::mem::zeroed() };

                for i in 0..chunk_len {
                    let item = &chunk[i];
                    let addr_len = fill_sockaddr(&item.destination, &mut addrs[i]);

                    if caps.ecn
                        && let Some(ecn) = item.ecn
                        && !push_ecn_cmsg(&mut cmsg_bufs[i], ecn, item.destination.is_ipv6())
                    {
                        return Err(io::Error::other("control message buffer overflow"));
                    }

                    iovs[i].iov_base = item.data.as_ptr() as *mut _;
                    iovs[i].iov_len = item.data.len();

                    let mhdr = &mut msgs[i].msg_hdr;
                    mhdr.msg_name = &mut addrs[i] as *mut _ as *mut _;
                    mhdr.msg_namelen = addr_len;
                    mhdr.msg_iov = &mut iovs[i] as *mut _ as *mut _;
                    mhdr.msg_iovlen = 1;
                    mhdr.msg_control = cmsg_bufs[i].as_ptr() as *mut _;
                    mhdr.msg_controllen = cmsg_bufs[i].len() as _;
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

        fn recv_one(fd: std::os::fd::RawFd, item: &mut ReceiveItem<'_>, ecn_enabled: bool) -> io::Result<()> {
            let mut cmsg_buf = RecvCmsgBuf::new();
            let (len, source, _gro_seg, ecn) = recvmsg_raw(fd, item.buf, &mut cmsg_buf, ecn_enabled)?;

            item.len = len;
            item.source = source;
            item.ecn = ecn;

            Ok(())
        }

        fn recv_gro(fd: std::os::fd::RawFd, items: &mut [ReceiveItem<'_>], ecn_enabled: bool) -> io::Result<usize> {
            let mut gro_buf = [0u8; 65536];
            let mut cmsg_buf = RecvCmsgBuf::new();

            let (total, source, seg_size, ecn_val) = recvmsg_raw(fd, &mut gro_buf, &mut cmsg_buf, ecn_enabled)?;

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
            get_int_opt(fd.as_raw_fd(), libc::SOL_UDP, libc::UDP_SEGMENT).is_ok()
        }

        fn probe_gro(fd: &OwnedFd) -> bool {
            set_bool_opt(fd.as_raw_fd(), libc::IPPROTO_UDP, libc::UDP_GRO).is_ok()
        }

        fn enable_ecn_recv(fd: &OwnedFd, is_ipv6: bool) -> bool {
            let (level, name) = if is_ipv6 {
                (libc::IPPROTO_IPV6, libc::IPV6_RECVTCLASS)
            } else {
                (libc::IPPROTO_IP, libc::IP_RECVTOS)
            };
            set_bool_opt(fd.as_raw_fd(), level, name).is_ok()
        }

        pub fn setup_and_probe(fd: &OwnedFd, config: &BuildConfig, is_ipv6: bool) -> io::Result<Capabilities> {
            set_bool_opt(fd.as_raw_fd(), libc::SOL_SOCKET, libc::SO_REUSEADDR)?;

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
    }

    // -- Generic Unix fallback -------------------------------------------------
    #[cfg(all(unix, not(target_os = "linux")))]
    mod unix_inner {
        use std::{
            io,
            os::fd::{AsRawFd, OwnedFd},
        };

        #[cfg(target_vendor = "apple")]
        use super::push_ecn_cmsg;
        use super::{BuildConfig, CmsgBuf, RecvCmsgBuf, recvmsg_raw, sendmsg_raw, set_bool_opt};
        use crate::{
            capability::Capabilities,
            item::{ReceiveItem, SendItem},
        };

        /// The generic-Unix send path carries no per-call state; this exists
        /// only to match the contract used by `send_many`.
        #[derive(Default)]
        pub struct SendState;

        fn send_one(fd: std::os::fd::RawFd, item: &SendItem, ecn_enabled: bool) -> io::Result<()> {
            #[cfg_attr(not(target_vendor = "apple"), allow(unused_mut))]
            let mut cmsgs = CmsgBuf::new();

            // ECN is only implemented on Apple targets among non-Linux Unixes.
            #[cfg(target_vendor = "apple")]
            if ecn_enabled
                && let Some(ecn) = item.ecn
                && !push_ecn_cmsg(&mut cmsgs, ecn, item.destination.is_ipv6())
            {
                return Err(io::Error::other("control message buffer overflow"));
            }
            #[cfg(not(target_vendor = "apple"))]
            let _ = ecn_enabled;

            sendmsg_raw(fd, item.data, &item.destination, &cmsgs)
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

        fn recv_one(fd: std::os::fd::RawFd, item: &mut ReceiveItem<'_>, ecn_enabled: bool) -> io::Result<()> {
            let mut cmsg_buf = RecvCmsgBuf::new();
            let (len, source, _gro_seg, ecn) = recvmsg_raw(fd, item.buf, &mut cmsg_buf, ecn_enabled)?;

            item.len = len;
            item.source = source;
            item.ecn = ecn;

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
        /// An IPv4 socket reports the TOS byte via `IP_RECVTOS`; an IPv6 socket
        /// reports the traffic class via `IPV6_RECVTCLASS`, which also covers
        /// IPv4-mapped traffic. Setting `IP_RECVTOS` on a dual-stack IPv6 socket
        /// fails on macOS, so only the option matching the socket family is set.
        #[cfg(target_vendor = "apple")]
        fn enable_ecn_recv(fd: &OwnedFd, is_ipv6: bool) -> bool {
            let (level, name) = if is_ipv6 {
                (libc::IPPROTO_IPV6, libc::IPV6_RECVTCLASS)
            } else {
                (libc::IPPROTO_IP, libc::IP_RECVTOS)
            };
            set_bool_opt(fd.as_raw_fd(), level, name).is_ok()
        }

        /// ECN is only implemented on Apple targets among non-Linux Unixes.
        #[cfg(not(target_vendor = "apple"))]
        fn enable_ecn_recv(_fd: &OwnedFd, _is_ipv6: bool) -> bool {
            false
        }

        pub fn setup_and_probe(fd: &OwnedFd, config: &BuildConfig, is_ipv6: bool) -> io::Result<Capabilities> {
            set_bool_opt(fd.as_raw_fd(), libc::SOL_SOCKET, libc::SO_REUSEADDR)?;

            let ecn = !config.disable_ecn && enable_ecn_recv(fd, is_ipv6);

            Ok(Capabilities {
                gso: false,
                gro: false,
                sendmmsg: false,
                ecn,
                max_batch: config.max_batch,
            })
        }
    }
}

// -- re-exports ------------------------------------------------------------
#[cfg(unix)]
pub use unix_socket::{FastUdpSocket, FastUdpSocketBuilder};

// This crate is Unix-only: it is built on `AsyncFd` and libc syscalls.
#[cfg(not(unix))]
compile_error!("tokio_fast_udp only supports Unix targets");
