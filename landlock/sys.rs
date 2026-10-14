//! Low-level Landlock syscall layer.
//!
//! This module is an implementation detail of the crate. It mirrors the Linux
//! UAPI (`include/uapi/linux/landlock.h`): the access-right bitmasks, the
//! `landlock_ruleset_attr`, `landlock_path_beneath_attr` and
//! `landlock_net_port_attr` structures, the raw system calls and the flag
//! values. The structure layouts are checked at compile time and the system
//! calls are the three documented entry points:
//!
//! - `landlock_create_ruleset(2)` (`SYS_landlock_create_ruleset`)
//! - `landlock_add_rule(2)` (`SYS_landlock_add_rule`)
//! - `landlock_restrict_self(2)` (`SYS_landlock_restrict_self`)
//!
//! The syscall numbers are architecture-independent (`libc` provides them for
//! every Linux target, including Android), and Landlock is available since
//! Linux 5.13.

// Filesystem access rights (`LANDLOCK_ACCESS_FS_*`), one bit each.
pub(crate) const FS_EXECUTE: u64 = 1 << 0;
pub(crate) const FS_WRITE_FILE: u64 = 1 << 1;
pub(crate) const FS_READ_FILE: u64 = 1 << 2;
pub(crate) const FS_READ_DIR: u64 = 1 << 3;
pub(crate) const FS_REMOVE_DIR: u64 = 1 << 4;
pub(crate) const FS_REMOVE_FILE: u64 = 1 << 5;
pub(crate) const FS_MAKE_CHAR: u64 = 1 << 6;
pub(crate) const FS_MAKE_DIR: u64 = 1 << 7;
pub(crate) const FS_MAKE_REG: u64 = 1 << 8;
pub(crate) const FS_MAKE_SOCK: u64 = 1 << 9;
pub(crate) const FS_MAKE_FIFO: u64 = 1 << 10;
pub(crate) const FS_MAKE_BLOCK: u64 = 1 << 11;
pub(crate) const FS_MAKE_SYM: u64 = 1 << 12;
pub(crate) const FS_REFER: u64 = 1 << 13;
pub(crate) const FS_TRUNCATE: u64 = 1 << 14;
pub(crate) const FS_IOCTL_DEV: u64 = 1 << 15;
pub(crate) const FS_RESOLVE_UNIX: u64 = 1 << 16;

// Network access rights (`LANDLOCK_ACCESS_NET_*`).
pub(crate) const NET_BIND_TCP: u32 = 1 << 0;
pub(crate) const NET_CONNECT_TCP: u32 = 1 << 1;
pub(crate) const NET_BIND_UDP: u32 = 1 << 2;
pub(crate) const NET_CONNECT_SEND_UDP: u32 = 1 << 3;

// Scope flags (`LANDLOCK_SCOPE_*`).
pub(crate) const SCOPE_ABSTRACT_UNIX_SOCKET: u32 = 1 << 0;
pub(crate) const SCOPE_SIGNAL: u32 = 1 << 1;

mod platform {
    use std::{
        io,
        mem::size_of,
        os::fd::{FromRawFd, OwnedFd, RawFd},
    };

    /// `enum landlock_rule_type`: a rule covering a file hierarchy.
    const LANDLOCK_RULE_PATH_BENEATH: u32 = 1;
    /// `enum landlock_rule_type`: a rule covering a TCP or UDP port.
    const LANDLOCK_RULE_NET_PORT: u32 = 2;
    /// Return the highest supported Landlock ABI version instead of creating a
    /// ruleset. Must be passed with a null `attr` and a zero `size`.
    const LANDLOCK_CREATE_RULESET_VERSION: u32 = 1;
    /// Apply the ruleset to every thread of the process (Landlock ABI 8).
    pub(crate) const LANDLOCK_RESTRICT_SELF_TSYNC: u32 = 1 << 3;
    /// Set `no_new_privs` as part of a successful enforcement (Landlock ABI 11).
    pub(crate) const LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS: u32 = 1 << 4;

    /// `struct landlock_ruleset_attr`.
    ///
    /// Newer kernels extend this structure (`quiet_access_fs`,
    /// `quiet_access_net` and `quiet_scoped` were added in Landlock ABI 10).
    /// We only ever fill the first three fields: the kernel zero-fills the
    /// fields it knows about but that we do not pass, and older kernels accept
    /// a larger size as long as the trailing bytes are zero.
    #[repr(C)]
    struct RulesetAttr {
        handled_access_fs: u64,
        handled_access_net: u64,
        scoped: u64,
    }

    /// `struct landlock_path_beneath_attr`. The kernel UAPI structure is
    /// packed: its size is 12 bytes, without the trailing padding a
    /// `#[repr(C)]` layout would add.
    #[repr(C, packed)]
    struct PathBeneathAttr {
        allowed_access: u64,
        parent_fd: i32,
    }

    /// `struct landlock_net_port_attr`. `port` is a `__u64` in the UAPI, even
    /// though only values up to 65535 are accepted.
    #[repr(C)]
    struct NetPortAttr {
        allowed_access: u64,
        port: u64,
    }

    // The structures are directly passed to the kernel: a layout mismatch
    // would be a correctness bug (the UAPI demands exact sizes).
    const _: () = {
        assert!(size_of::<RulesetAttr>() == 24);
        assert!(size_of::<PathBeneathAttr>() == 12);
        assert!(size_of::<NetPortAttr>() == 16);
    };

    /// Returns the highest Landlock ABI version supported by the running
    /// kernel.
    ///
    /// Fails with `ENOSYS` when the kernel does not implement Landlock and with
    /// `EOPNOTSUPP` when Landlock is compiled in but disabled at boot time
    /// (`lsm=` kernel command line). Any other failure (for example a seccomp
    /// filter blocking the system call) is also reported as an error: the
    /// caller treats every failure as "Landlock is not available".
    pub(crate) fn abi_version() -> io::Result<u32> {
        let ret = unsafe {
            libc::syscall(
                libc::SYS_landlock_create_ruleset,
                std::ptr::null::<libc::c_void>(),
                0usize,
                LANDLOCK_CREATE_RULESET_VERSION as libc::c_uint,
            )
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(ret as u32)
    }

    /// Creates a ruleset and returns its (owned) file descriptor.
    ///
    /// At least one of `handled_access_fs`, `handled_access_net` and `scoped`
    /// must be non-zero, otherwise the kernel fails with `ENOMSG`.
    pub(crate) fn create_ruleset(handled_access_fs: u64, handled_access_net: u32, scoped: u32) -> io::Result<OwnedFd> {
        let attr = RulesetAttr {
            handled_access_fs,
            handled_access_net: u64::from(handled_access_net),
            scoped: u64::from(scoped),
        };
        let ret = unsafe {
            libc::syscall(
                libc::SYS_landlock_create_ruleset,
                &attr as *const RulesetAttr,
                size_of::<RulesetAttr>(),
                0u32,
            )
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        // The kernel returns a fresh file descriptor owned by the caller.
        Ok(unsafe { OwnedFd::from_raw_fd(ret as RawFd) })
    }

    /// Adds a rule granting `allowed_access` on the file hierarchy identified
    /// by `parent_fd` (an `O_PATH` file descriptor).
    pub(crate) fn add_path_rule(ruleset_fd: RawFd, allowed_access: u64, parent_fd: RawFd) -> io::Result<()> {
        let attr = PathBeneathAttr {
            allowed_access,
            parent_fd,
        };
        let ret = unsafe {
            libc::syscall(
                libc::SYS_landlock_add_rule,
                ruleset_fd as libc::c_int,
                LANDLOCK_RULE_PATH_BENEATH as libc::c_uint,
                &attr as *const PathBeneathAttr as *const libc::c_void,
                0u32,
            )
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Adds a rule granting `allowed_access` on the TCP or UDP `port`.
    pub(crate) fn add_net_rule(ruleset_fd: RawFd, allowed_access: u32, port: u16) -> io::Result<()> {
        let attr = NetPortAttr {
            allowed_access: u64::from(allowed_access),
            port: u64::from(port),
        };
        let ret = unsafe {
            libc::syscall(
                libc::SYS_landlock_add_rule,
                ruleset_fd as libc::c_int,
                LANDLOCK_RULE_NET_PORT as libc::c_uint,
                &attr as *const NetPortAttr as *const libc::c_void,
                0u32,
            )
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Enforces `ruleset_fd` on the calling thread (and, when
    /// `LANDLOCK_RESTRICT_SELF_TSYNC` is set, on every thread of the process).
    ///
    /// The caller must have `no_new_privs` set unless
    /// `LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS` is passed in `flags` or it has
    /// `CAP_SYS_ADMIN` in its user namespace.
    pub(crate) fn restrict_self(ruleset_fd: RawFd, flags: u32) -> io::Result<()> {
        let ret = unsafe {
            libc::syscall(
                libc::SYS_landlock_restrict_self,
                ruleset_fd as libc::c_int,
                flags as libc::c_uint,
            )
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Sets `PR_SET_NO_NEW_PRIVS` on the calling thread.
    pub(crate) fn set_no_new_privs() -> io::Result<()> {
        let ret = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Opens `path` — a NUL-terminated byte string — with `O_PATH`.
    ///
    /// `O_PATH` needs no permission on the target itself, so rules can be
    /// created for files and directories the process may not be able to open;
    /// rules can still be created for paths they cannot read. `O_CLOEXEC`
    /// makes sure the descriptor is not leaked to executed programs.
    pub(crate) fn open_path(path: &[u8]) -> io::Result<OwnedFd> {
        debug_assert_eq!(path.last(), Some(&0));
        let ret = unsafe { libc::open(path.as_ptr() as *const libc::c_char, libc::O_PATH | libc::O_CLOEXEC) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(unsafe { OwnedFd::from_raw_fd(ret) })
    }

    /// Returns whether the file descriptor refers to a directory.
    pub(crate) fn fd_is_dir(fd: RawFd) -> io::Result<bool> {
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        if unsafe { libc::fstat(fd, &mut stat) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok((stat.st_mode & libc::S_IFMT) == libc::S_IFDIR)
    }
}

pub(crate) use platform::*;
