#![cfg(any(target_os = "linux", target_os = "android"))]

//! Sandbox your program with the Linux [Landlock] LSM.
//!
//! This crate is a high-level wrapper around the Landlock
//! security module: build a policy, apply it, and let the kernel enforce it.
//! Landlock is unprivileged: no root, no namespace, no seccomp, and it cannot
//! be removed once enforced (it can only be made stricter).
//!
//! This crate only exists on Linux and Android: on any other target it
//! compiles to an empty crate and using its items fails to compile.
//!
//! # Quick start
//!
//! ```
//! use landlock::{Access, NetworkAccess, new_best_effort};
//!
//! # fn main() -> Result<(), landlock::Error> {
//! // The example is not run: restricting a program is irreversible.
//! # if false {
//! let abi = new_best_effort()
//!     .allow_filesystem(Access::Read, ["/etc", "/usr", "/lib"])
//!     .allow_filesystem(Access::ReadWrite, ["/tmp"])
//!     .allow_network(NetworkAccess::ConnectTcp, [443])
//!     .restrict_self()?;
//!
//! println!("sandboxed (Landlock ABI {abi})");
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! Call [`Ruleset::restrict_self`] as early as possible
//! and before spawning threads (see [Threads](#threads)).
//!
//! # Everything is denied by default
//!
//! A fresh policy has no rules, and **only rules grant access**. Everything the
//! running kernel can restrict is restricted:
//!
//! * filesystem access (`Access`),
//! * network access (`NetworkAccess`),
//! * IPC scoping: connections to abstract UNIX sockets and signals sent to
//!   processes outside the sandbox ([`Scope`]).
//!
//! [`Ruleset::allow_scope`] lifts the IPC restrictions for programs that need to
//! talk to services outside the sandbox (D-Bus, X11, supervisors); filesystem
//! and network access always stay denied until explicitly allowed.
//!
//! # Strict or best effort
//!
//! Each rule is checked against the features the running kernel actually
//! supports (its Landlock ABI version, see [Kernel support](#kernel-support)):
//!
//! * [`new_strict`] fails the whole policy if a rule cannot be enforced as
//!   requested. What you wrote is what is enforced, or you get
//!   [`Error::Unsupported`].
//! * [`new_best_effort`] drops the rights the kernel cannot enforce and applies
//!   the rest. Rules that end up with no access right at all are skipped, which
//!   leaves the affected path denied.
//!
//! Both stop on invalid input (a missing path, an access right that does not
//! apply to the object, …): [`Ruleset::restrict_self`] then returns the first
//! error and **nothing is enforced**.
//!
//! # Filesystem rules
//!
//! ```no_run
//! # fn main() -> Result<(), landlock::Error> {
//! use landlock::{Access, new_best_effort};
//!
//! let _abi = new_best_effort()
//!     .allow_filesystem(Access::Read, ["/etc", "/usr"])
//!     .allow_filesystem(Access::ReadWrite, ["/tmp"])
//!     .allow_filesystem(Access::Execute, ["/usr/bin"])
//!     .restrict_self()?;
//! # Ok(())
//! # }
//! ```
//!
//! * [`Ruleset::allow_filesystem`] opens each path with `O_PATH`, which works
//!   even for files and directories the process cannot read.
//! * [`Ruleset::allow_file_descriptor`] grants access through a file descriptor
//!   you already opened, for setups that need to control how the path is
//!   resolved.
//! * Rights that do not apply to the object are dropped: `Access::Write` on a
//!   regular file grants writing and truncating, not `Access::RemoveFile` (which
//!   applies to the containing directory).
//! * Paths are resolved by the kernel when the rule is added, so **symlinks are
//!   followed**. Rules are attached to the objects (inodes) rather than to the
//!   path, so later renames or bind mounts cannot move a rule somewhere else.
//! * Rule directories are *self-sufficient*: a rule on `/a` and a rule on
//!   `/a/b` are unrelated, so granting read-write on `/tmp` and read-write on
//!   `/tmp/important` is not enough for `/tmp/important` — grant it explicitly.
//!
//! # Network rules
//!
//! Landlock can restrict binding and connecting TCP and UDP sockets, but only
//! **one port per rule**: there is no wildcard, so a program that talks to
//! arbitrary ports has to enumerate them. Network access is denied by default:
//! a policy that never calls `allow_network` cannot bind or connect anything.
//!
//! ```no_run
//! # fn main() -> Result<(), landlock::Error> {
//! use landlock::{NetworkAccess, new_best_effort};
//!
//! let _abi = new_best_effort()
//!     // A DNS client: connect to the resolver, and bind an ephemeral source
//!     // port for UDP (port 0 means the ephemeral port range).
//!     .allow_network(NetworkAccess::ConnectTcp, [53])
//!     .allow_network(NetworkAccess::ConnectSendUdp, [53])
//!     .allow_network(NetworkAccess::BindUdp, [0])
//!     // A server listening on a privileged port.
//!     .allow_network(NetworkAccess::BindTcp, [8080])
//!     .restrict_self()?;
//! # Ok(())
//! # }
//! ```
//!
//! UDP datagrams need `NetworkAccess::BindUdp` on port 0 because sending
//! auto-binds an ephemeral source port. Multipath TCP and non-TCP/UDP socket
//! families are not restricted by Landlock.
//!
//! # Scope: abstract UNIX sockets and signals
//!
//! Alongside filesystem and network access, Landlock can isolate the sandbox
//! from IPC with processes outside it (Landlock ABI 6, Linux 6.12):
//!
//! * connections to **abstract** UNIX sockets created outside the sandbox are
//!   denied with `EPERM` (X11's `@/tmp/.X11-unix/X0`, D-Bus session buses,
//!   accessibility buses, …),
//! * signals sent to processes outside the sandbox are denied with `EPERM`
//!   (signalling children and threads still works).
//!
//! Unlike filesystem and network access, Landlock offers **no per-object
//! exception** for these: a rule cannot re-open a specific socket or process.
//! The only knob is [`Ruleset::allow_scope`], which lifts a whole scope:
//!
//! ```no_run
//! # fn main() -> Result<(), landlock::Error> {
//! use landlock::{Scope, new_best_effort};
//!
//! let abi = new_best_effort()
//!     .allow_scope(Scope::AbstractUnixSocket) // allow talking to D-Bus/X11
//!     .allow_scope(Scope::Signal)             // allow signalling outside processes
//!     .restrict_self()?;
//! # Ok(())
//! # }
//! ```
//!
//! Pathname UNIX sockets (Wayland, the D-Bus system bus, NSS sockets, …) are a
//! filesystem matter instead: on Landlock ABI 9 (Linux 7.1) and newer they
//! require the `Access::ResolveUnix` right, which *can* be granted for specific
//! paths. On older kernels they are not restricted at all.
//!
//! # Threads
//!
//! Landlock restrictions apply to the **calling thread** and to its future
//! children; sibling threads are not affected. This crate passes the `TSYNC`
//! flag (Landlock ABI 8, Linux 7.0) whenever the kernel supports it, which
//! applies the policy to every thread of the process at once. On older kernels:
//!
//! * call [`Ruleset::restrict_self`] **before spawning any thread**, otherwise
//!   the already-running threads stay unrestricted;
//! * threads created afterwards inherit the restriction.
//!
//! Note that `TSYNC` *replaces* the policy of the sibling threads, as
//! documented for `landlock_restrict_self(2)`: in a program whose threads each
//! applied their own policy, the last one applied ends up in effect
//! everywhere. Policies applied by the *same* thread stack instead (they
//! intersect).
//!
//! `restrict_self` also sets `PR_SET_NO_NEW_PRIVS` on the calling thread, which
//! is required by Landlock and prevents gaining privileges through set-user-ID
//! or set-group-ID programs.
//!
//! # Kernel support
//!
//! Landlock is available since Linux 5.13, but each version only knows about
//! the access rights and features introduced up to that point. The ABI version
//! below is reported by the kernel and returned by [`Ruleset::restrict_self`]:
//!
//! | ABI | Introduced in | Adds |
//! |-----|---------------|------|
//! | 1 | Linux 5.13 | basic filesystem rights (`Access::Execute` … `Access::MakeSym`) |
//! | 2 | Linux 5.19 | `Access::Refer` (link/rename across directories) |
//! | 3 | Linux 6.2 | `Access::Truncate` |
//! | 4 | Linux 6.7 | `NetworkAccess::BindTcp`, `NetworkAccess::ConnectTcp` |
//! | 5 | Linux 6.10 | `Access::IoctlDev` |
//! | 6 | Linux 6.12 | `Scope::AbstractUnixSocket`, `Scope::Signal` |
//! | 7 | Linux 6.15 | audit logging flags, errata reporting |
//! | 8 | Linux 7.0 | process-wide enforcement (`restrict_self` uses `TSYNC`) |
//! | 9 | Linux 7.1 | `Access::ResolveUnix` (pathname UNIX sockets) |
//! | 10 | Linux 7.2 | `NetworkAccess::BindUdp`, `NetworkAccess::ConnectSendUdp` |
//! | 11 | Linux 7.3 | atomic `no_new_privs` flag (this crate falls back to `prctl`) |
//!
//! The minimum supported kernel is **6.0** (Landlock ABI 2), which means
//! `Access::Write`, `Access::ReadWrite` and `Access::All` cannot be fully
//! enforced there in strict mode: truncation and I/O control on devices need a
//! newer kernel. Use [`new_best_effort`] for portable policies, or request the
//! individual rights (`Access::WriteFile`, …) explicitly.
//!
//! Rights a kernel does not know about cannot be restricted **at all**: not
//! handling a right means granting it everywhere. On a kernel older than
//! Landlock ABI 3, truncation is left unrestricted; before ABI 4 the network is
//! left alone, and before ABI 6 the IPC scopes have no effect. The ABI version
//! returned by [`Ruleset::restrict_self`] tells what the kernel could actually
//! enforce. Distributions backport Landlock fixes as well, so the ABI version —
//! not the kernel version string — is authoritative.
//!
//! Whether the kernel supports Landlock at all can be checked with
//! [`abi_version`] and [`is_supported`]. A kernel may have Landlock compiled in
//! but disabled at boot time; that is reported the same way.
//!
//! # Pitfalls
//!
//! * Files and directories **opened before** the sandbox is enforced stay
//!   usable; pass only the file descriptors you mean to keep. This also means a
//!   pre-opened file descriptor can be used to reach a file the policy denies.
//! * `exec`-ing a program needs `Access::Execute` on its binary **and**
//!   `Access::ReadFile` on its shared libraries (check with `ldd`).
//! * Some files are opened lazily, *after* the sandbox is in place: NSS lookups
//!   (`/etc/nsswitch.conf`, `/etc/hosts`, `/etc/resolv.conf`) are the classic
//!   example. Grant them explicitly.
//! * Landlock restricts the filesystem topology (no `mount`, no `pivot_root`)
//!   but not `chroot`, and it does not restrict `stat`, `chmod`, `chown`,
//!   `fcntl`, `flock` or `access`.
//! * Rights are decided when a file is opened: `stdin`/`stdout`/`stderr` keep
//!   the rights they had before the sandbox, and file descriptors keep their
//!   Landlock properties even when passed to another process.
//! * Denials surface as ordinary `EACCES`/`EPERM` errors, so check the return
//!   value of [`Ruleset::restrict_self`] and use `strace` when a program
//!   misbehaves inside the sandbox.
//! * Landlock does not prevent a sandboxed program from exploiting a kernel
//!   bug: it restricts the kernel interfaces a process may use, not the kernel
//!   itself. Combine it with seccomp for code that parses untrusted input.
//!
//! [Landlock]: https://docs.kernel.org/userspace-api/landlock.html

mod sys;

use std::{
    error::Error as StdError,
    fmt, io,
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd},
    path::{Path, PathBuf},
};

/// The largest path the kernel accepts, including the terminating NUL byte.
const PATH_MAX: usize = 4096;

/// `EBADFD` ("file descriptor in bad state", errno 77) is Linux-specific: the
/// kernel returns it from `landlock_add_rule` when the file descriptor does not
/// identify a file hierarchy, or refers to an internal filesystem. `libc` does
/// not define it for every platform this crate compiles on.
const EBADFD: i32 = 77;

// Access rights that may be granted on a non-directory (`ACCESS_FILE` in the
// kernel): the other rights only make sense for directories.
const FS_FILE_BITS: u64 = sys::FS_EXECUTE
    | sys::FS_WRITE_FILE
    | sys::FS_READ_FILE
    | sys::FS_TRUNCATE
    | sys::FS_IOCTL_DEV
    | sys::FS_RESOLVE_UNIX;

const FS_READ_BITS: u64 = sys::FS_READ_FILE | sys::FS_READ_DIR;

const FS_WRITE_BITS: u64 = sys::FS_WRITE_FILE
    | sys::FS_TRUNCATE
    | sys::FS_REMOVE_DIR
    | sys::FS_REMOVE_FILE
    | sys::FS_MAKE_CHAR
    | sys::FS_MAKE_DIR
    | sys::FS_MAKE_REG
    | sys::FS_MAKE_SOCK
    | sys::FS_MAKE_FIFO
    | sys::FS_MAKE_BLOCK
    | sys::FS_MAKE_SYM
    | sys::FS_REFER;

const FS_ALL_BITS: u64 = FS_READ_BITS | FS_WRITE_BITS | sys::FS_EXECUTE | sys::FS_IOCTL_DEV | sys::FS_RESOLVE_UNIX;

const NET_TCP_BITS: u32 = sys::NET_BIND_TCP | sys::NET_CONNECT_TCP;

const NET_UDP_BITS: u32 = sys::NET_BIND_UDP | sys::NET_CONNECT_SEND_UDP;

const NET_ALL_BITS: u32 = NET_TCP_BITS | NET_UDP_BITS;

const SCOPE_ALL_BITS: u32 = sys::SCOPE_ABSTRACT_UNIX_SOCKET | sys::SCOPE_SIGNAL;

/// For every filesystem access right, the Landlock ABI that introduced it.
const FS_ABI_TABLE: [(u64, u32); 17] = [
    (sys::FS_EXECUTE, 1),
    (sys::FS_WRITE_FILE, 1),
    (sys::FS_READ_FILE, 1),
    (sys::FS_READ_DIR, 1),
    (sys::FS_REMOVE_DIR, 1),
    (sys::FS_REMOVE_FILE, 1),
    (sys::FS_MAKE_CHAR, 1),
    (sys::FS_MAKE_DIR, 1),
    (sys::FS_MAKE_REG, 1),
    (sys::FS_MAKE_SOCK, 1),
    (sys::FS_MAKE_FIFO, 1),
    (sys::FS_MAKE_BLOCK, 1),
    (sys::FS_MAKE_SYM, 1),
    (sys::FS_REFER, 2),
    (sys::FS_TRUNCATE, 3),
    (sys::FS_IOCTL_DEV, 5),
    (sys::FS_RESOLVE_UNIX, 9),
];

/// For every network access right, the Landlock ABI that introduced it.
const NET_ABI_TABLE: [(u32, u32); 4] = [
    (sys::NET_BIND_TCP, 4),
    (sys::NET_CONNECT_TCP, 4),
    (sys::NET_BIND_UDP, 10),
    (sys::NET_CONNECT_SEND_UDP, 10),
];

/// For every scope, the Landlock ABI that introduced it.
const SCOPE_ABI_TABLE: [(u32, u32); 2] = [(sys::SCOPE_ABSTRACT_UNIX_SOCKET, 6), (sys::SCOPE_SIGNAL, 6)];

/// Filesystem rights supported by Landlock ABI `abi`.
const fn fs_supported(abi: u32) -> u64 {
    let mut mask = 0u64;
    let mut i = 0;
    while i < FS_ABI_TABLE.len() {
        if abi >= FS_ABI_TABLE[i].1 {
            mask |= FS_ABI_TABLE[i].0;
        }
        i += 1;
    }
    mask
}

/// Network rights supported by Landlock ABI `abi`.
const fn net_supported(abi: u32) -> u32 {
    let mut mask = 0u32;
    let mut i = 0;
    while i < NET_ABI_TABLE.len() {
        if abi >= NET_ABI_TABLE[i].1 {
            mask |= NET_ABI_TABLE[i].0;
        }
        i += 1;
    }
    mask
}

/// Scopes supported by Landlock ABI `abi`.
const fn scope_supported(abi: u32) -> u32 {
    let mut mask = 0u32;
    let mut i = 0;
    while i < SCOPE_ABI_TABLE.len() {
        if abi >= SCOPE_ABI_TABLE[i].1 {
            mask |= SCOPE_ABI_TABLE[i].0;
        }
        i += 1;
    }
    mask
}

/// The oldest Landlock ABI required to enforce every bit of `bits`, or 0 if
/// `bits` is empty.
const fn min_abi_fs(bits: u64) -> u32 {
    let mut abi = 0u32;
    let mut i = 0;
    while i < FS_ABI_TABLE.len() {
        let (bit, introduced) = FS_ABI_TABLE[i];
        if bits & bit != 0 && introduced > abi {
            abi = introduced;
        }
        i += 1;
    }
    abi
}

/// Same as [`min_abi_fs`], for network rights.
const fn min_abi_net(bits: u32) -> u32 {
    let mut abi = 0u32;
    let mut i = 0;
    while i < NET_ABI_TABLE.len() {
        let (bit, introduced) = NET_ABI_TABLE[i];
        if bits & bit != 0 && introduced > abi {
            abi = introduced;
        }
        i += 1;
    }
    abi
}

/// The kernel version that introduced `abi`, when it is known.
const fn abi_kernel_version(abi: u32) -> Option<&'static str> {
    match abi {
        1 => Some("5.13"),
        2 => Some("5.19"),
        3 => Some("6.2"),
        4 => Some("6.7"),
        5 => Some("6.10"),
        6 => Some("6.12"),
        7 => Some("6.15"),
        8 => Some("7.0"),
        9 => Some("7.1"),
        10 => Some("7.2"),
        11 => Some("7.3"),
        _ => None,
    }
}

/// A filesystem access right.
///
/// Values are either a single right or a group of related rights. Because this
/// is an enumeration (and not a raw bitmask), a policy cannot contain bits the
/// kernel does not define.
///
/// The groups require the Landlock ABI that introduced their newest member:
/// [`Access::Read`] needs ABI 1, [`Access::Write`] and [`Access::ReadWrite`]
/// need ABI 3 (`Access::Truncate`) and [`Access::All`] needs ABI 9
/// (`Access::ResolveUnix`). [`new_strict`] refuses them on older kernels, while
/// [`new_best_effort`] drops what the kernel does not support.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Access {
    /// Execute a file.
    Execute,
    /// Open a file for writing.
    WriteFile,
    /// Open a file for reading.
    ReadFile,
    /// Open a directory or list its content.
    ReadDir,
    /// Remove an empty directory, or rename one.
    RemoveDir,
    /// Unlink a file, or rename one.
    RemoveFile,
    /// Create (or rename or link) a character device.
    MakeChar,
    /// Create (or rename) a directory.
    MakeDir,
    /// Create (or rename or link) a regular file.
    MakeReg,
    /// Create (or rename or link) a UNIX domain socket.
    MakeSock,
    /// Create (or rename or link) a named pipe.
    MakeFifo,
    /// Create (or rename or link) a block device.
    MakeBlock,
    /// Create (or rename or link) a symbolic link.
    MakeSym,
    /// Link or rename a file to or from another directory.
    Refer,
    /// Truncate a file (`truncate`, `ftruncate`, `creat`, `open` with
    /// `O_TRUNC`).
    Truncate,
    /// Invoke `ioctl` commands on an opened character or block device.
    IoctlDev,
    /// Look up pathname UNIX domain sockets.
    ResolveUnix,
    /// Read access to files and directories ([`Access::ReadFile`] and
    /// [`Access::ReadDir`]).
    Read,
    /// Write access: everything that modifies the content of a directory
    /// ([`Access::WriteFile`], [`Access::Truncate`], [`Access::RemoveDir`],
    /// [`Access::RemoveFile`], the `Make*` rights and [`Access::Refer`]).
    Write,
    /// [`Access::Read`] and [`Access::Write`].
    ReadWrite,
    /// Every access right.
    All,
}

impl Access {
    const fn bits(self) -> u64 {
        match self {
            Access::Execute => sys::FS_EXECUTE,
            Access::WriteFile => sys::FS_WRITE_FILE,
            Access::ReadFile => sys::FS_READ_FILE,
            Access::ReadDir => sys::FS_READ_DIR,
            Access::RemoveDir => sys::FS_REMOVE_DIR,
            Access::RemoveFile => sys::FS_REMOVE_FILE,
            Access::MakeChar => sys::FS_MAKE_CHAR,
            Access::MakeDir => sys::FS_MAKE_DIR,
            Access::MakeReg => sys::FS_MAKE_REG,
            Access::MakeSock => sys::FS_MAKE_SOCK,
            Access::MakeFifo => sys::FS_MAKE_FIFO,
            Access::MakeBlock => sys::FS_MAKE_BLOCK,
            Access::MakeSym => sys::FS_MAKE_SYM,
            Access::Refer => sys::FS_REFER,
            Access::Truncate => sys::FS_TRUNCATE,
            Access::IoctlDev => sys::FS_IOCTL_DEV,
            Access::ResolveUnix => sys::FS_RESOLVE_UNIX,
            Access::Read => FS_READ_BITS,
            Access::Write => FS_WRITE_BITS,
            Access::ReadWrite => FS_READ_BITS | FS_WRITE_BITS,
            Access::All => FS_ALL_BITS,
        }
    }
}

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Access::Execute => "execute",
            Access::WriteFile => "write-file",
            Access::ReadFile => "read-file",
            Access::ReadDir => "read-dir",
            Access::RemoveDir => "remove-dir",
            Access::RemoveFile => "remove-file",
            Access::MakeChar => "make-char",
            Access::MakeDir => "make-dir",
            Access::MakeReg => "make-reg",
            Access::MakeSock => "make-sock",
            Access::MakeFifo => "make-fifo",
            Access::MakeBlock => "make-block",
            Access::MakeSym => "make-sym",
            Access::Refer => "refer",
            Access::Truncate => "truncate",
            Access::IoctlDev => "ioctl-dev",
            Access::ResolveUnix => "resolve-unix",
            Access::Read => "read",
            Access::Write => "write",
            Access::ReadWrite => "read-write",
            Access::All => "all",
        };
        f.write_str(name)
    }
}

/// A network access right.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum NetworkAccess {
    /// Bind a TCP socket to a local port.
    BindTcp,
    /// Connect a TCP socket to a remote port.
    ConnectTcp,
    /// Bind a UDP socket to a local port.
    BindUdp,
    /// Set the remote port of a UDP socket, or send datagrams to it.
    ConnectSendUdp,
    /// [`NetworkAccess::BindTcp`] and [`NetworkAccess::ConnectTcp`].
    Tcp,
    /// [`NetworkAccess::BindUdp`] and [`NetworkAccess::ConnectSendUdp`].
    Udp,
    /// Every network access right.
    All,
}

impl NetworkAccess {
    const fn bits(self) -> u32 {
        match self {
            NetworkAccess::BindTcp => sys::NET_BIND_TCP,
            NetworkAccess::ConnectTcp => sys::NET_CONNECT_TCP,
            NetworkAccess::BindUdp => sys::NET_BIND_UDP,
            NetworkAccess::ConnectSendUdp => sys::NET_CONNECT_SEND_UDP,
            NetworkAccess::Tcp => NET_TCP_BITS,
            NetworkAccess::Udp => NET_UDP_BITS,
            NetworkAccess::All => NET_ALL_BITS,
        }
    }
}

impl fmt::Display for NetworkAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            NetworkAccess::BindTcp => "bind-tcp",
            NetworkAccess::ConnectTcp => "connect-tcp",
            NetworkAccess::BindUdp => "bind-udp",
            NetworkAccess::ConnectSendUdp => "connect-send-udp",
            NetworkAccess::Tcp => "tcp",
            NetworkAccess::Udp => "udp",
            NetworkAccess::All => "all",
        };
        f.write_str(name)
    }
}

/// An IPC scope, restricting interactions with processes outside the sandbox.
///
/// Scopes are active by default; [`Ruleset::allow_scope`] lifts them.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Scope {
    /// Connecting to abstract UNIX sockets created outside the sandbox.
    AbstractUnixSocket,
    /// Sending signals to processes outside the sandbox.
    Signal,
    /// Every scope.
    All,
}

impl Scope {
    const fn bits(self) -> u32 {
        match self {
            Scope::AbstractUnixSocket => sys::SCOPE_ABSTRACT_UNIX_SOCKET,
            Scope::Signal => sys::SCOPE_SIGNAL,
            Scope::All => SCOPE_ALL_BITS,
        }
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Scope::AbstractUnixSocket => "abstract-unix-socket",
            Scope::Signal => "signal",
            Scope::All => "all",
        };
        f.write_str(name)
    }
}

/// Why a path cannot be used for a rule.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PathError {
    /// The path contains a NUL byte; the kernel would silently read a
    /// truncated path from it.
    ContainsNul,
    /// The path, including its terminating NUL byte, does not fit in
    /// `PATH_MAX` (4096) bytes.
    TooLong,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PathError::ContainsNul => "it contains a NUL byte",
            PathError::TooLong => "it is longer than PATH_MAX bytes (4096)",
        })
    }
}

/// A feature (access right or scope) that the running kernel cannot enforce.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Feature {
    /// A filesystem access right.
    FileSystem(Access),
    /// A network access right.
    Network(NetworkAccess),
    /// An IPC scope.
    Scope(Scope),
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Feature::FileSystem(access) => write!(f, "access right `{access}`"),
            Feature::Network(access) => write!(f, "network access `{access}`"),
            Feature::Scope(scope) => write!(f, "IPC scope `{scope}`"),
        }
    }
}

/// A Landlock operation that failed with an unexpected error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Operation {
    /// `landlock_create_ruleset(2)`.
    CreateRuleset,
    /// `landlock_add_rule(2)`.
    AddRule,
    /// `landlock_restrict_self(2)`.
    RestrictSelf,
    /// `prctl(PR_SET_NO_NEW_PRIVS)`.
    SetNoNewPrivs,
    /// `fstat(2)`, used to tell files from directories.
    Stat,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Operation::CreateRuleset => "landlock_create_ruleset",
            Operation::AddRule => "landlock_add_rule",
            Operation::RestrictSelf => "landlock_restrict_self",
            Operation::SetNoNewPrivs => "prctl(PR_SET_NO_NEW_PRIVS)",
            Operation::Stat => "fstat",
        })
    }
}

/// Everything that can prevent a policy from being enforced.
///
/// [`Ruleset::restrict_self`] returns the first error encountered while the
/// policy was built. When it returns an error, no restriction has been applied
/// (with one caveat: a failure while enforcing the second or third ruleset
/// layer can leave the process partially restricted, see [`Error::Syscall`]).
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Landlock is not available: the kernel does not implement it (`ENOSYS`),
    /// it is disabled at boot time (`EOPNOTSUPP`, `lsm=` kernel command line),
    /// or the system calls are blocked (for example by a seccomp filter).
    NotSupported {
        /// The error reported by the kernel.
        source: io::Error,
    },
    /// In strict mode, the running kernel cannot enforce a requested feature.
    Unsupported {
        /// The feature that cannot be enforced.
        feature: Feature,
        /// The ABI version supported by the running kernel.
        kernel_abi: u32,
        /// The oldest ABI version that supports `feature`.
        required_abi: u32,
    },
    /// The path is not usable for a rule.
    InvalidPath {
        /// The offending path.
        path: PathBuf,
        /// What is wrong with it.
        reason: PathError,
    },
    /// The path could not be opened.
    Open {
        /// The offending path.
        path: PathBuf,
        /// The error reported by the kernel (`ENOENT`, `EACCES`, `ELOOP`, …).
        source: io::Error,
    },
    /// The requested access has no meaning for the object (for example
    /// `Access::ReadDir` on a regular file).
    InapplicableAccess {
        /// The requested access.
        access: Access,
        /// The path, when the rule was created from a path.
        path: Option<PathBuf>,
    },
    /// The file descriptor does not identify a file hierarchy that Landlock can
    /// restrict: it refers to a pipe, a socket, an anonymous inode, a ruleset,
    /// or an internal filesystem such as `/proc/<pid>/ns/*`.
    InvalidFileDescriptor {
        /// The error reported by the kernel.
        source: io::Error,
    },
    /// The kernel's limit of 16 stacked rulesets is reached for this thread.
    ///
    /// A single [`Ruleset`] applies at most three layers (filesystem, network
    /// and scope). This error means the thread already inherited or applied
    /// many rulesets; build a single policy per thread where possible.
    LayerLimitReached,
    /// An unexpected error from a Landlock system call.
    Syscall {
        /// The operation that failed.
        operation: Operation,
        /// The error reported by the kernel.
        source: io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotSupported {
                source,
            } => {
                write!(f, "Landlock is not supported by this kernel: {source}")
            }
            Error::Unsupported {
                feature,
                kernel_abi,
                required_abi,
            } => {
                write!(
                    f,
                    "the running kernel cannot enforce {feature}: it needs Landlock ABI {}",
                    required_abi
                )?;
                if let Some(version) = abi_kernel_version(*required_abi) {
                    write!(f, " (Linux {version} or later)")?;
                }
                write!(f, ", but only ABI {kernel_abi} is available")
            }
            Error::InvalidPath {
                path,
                reason,
            } => {
                write!(f, "invalid path {path:?}: {reason}")
            }
            Error::Open {
                path,
                source,
            } => {
                write!(f, "failed to open {path:?}: {source}")
            }
            Error::InapplicableAccess {
                access,
                path,
            } => match path {
                Some(path) => write!(
                    f,
                    "access right `{access}` does not apply to {path:?} (or it grants nothing there)"
                ),
                None => write!(f, "access right `{access}` does not apply to the given file descriptor"),
            },
            Error::InvalidFileDescriptor {
                source,
            } => {
                write!(f, "the file descriptor does not identify a file hierarchy: {source}")
            }
            Error::LayerLimitReached => {
                write!(f, "the kernel's limit of 16 stacked rulesets is reached for this thread")
            }
            Error::Syscall {
                operation,
                source,
            } => {
                write!(f, "the {operation} system call failed: {source}")
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NotSupported {
                source,
            }
            | Error::Open {
                source, ..
            }
            | Error::InvalidFileDescriptor {
                source,
            }
            | Error::Syscall {
                source, ..
            } => Some(source),
            _ => None,
        }
    }
}

/// Whether a ruleset enforces every requested right or adapts to the kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    /// Fail when the kernel cannot enforce a requested right.
    Strict,
    /// Drop rights the kernel cannot enforce.
    BestEffort,
}

/// A Landlock policy under construction.
///
/// Rules are added with the `allow_*` methods and the policy is applied with
/// [`Ruleset::restrict_self`], which consumes the builder. Building never
/// fails: errors (invalid paths, unsupported rights, …) are remembered and
/// reported by `restrict_self`, and nothing is enforced when it returns them.
///
/// The builder is [`Send`] and [`Sync`], and every method takes `self` by
/// value, so a policy is never shared or mutated by two threads at once.
pub struct Ruleset {
    mode: Mode,
    abi: u32,
    handled_fs: u64,
    fs: Option<OwnedFd>,
    net: Option<OwnedFd>,
    scoped: u32,
    error: Option<Error>,
}

impl fmt::Debug for Ruleset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ruleset")
            .field("mode", &self.mode)
            .field("abi", &self.abi)
            .field("handled_access_fs", &format_args!("{:#x}", self.handled_fs))
            .field("scoped", &format_args!("{:#x}", self.scoped))
            .field("failed", &self.error.is_some())
            .finish()
    }
}

/// Creates a policy that fails if the running kernel cannot enforce a rule.
///
/// Rules that need a newer Landlock ABI (see the [kernel support
/// table](index.html#kernel-support)) make [`Ruleset::restrict_self`] return
/// [`Error::Unsupported`]. Use [`new_best_effort`] to degrade instead.
#[must_use]
pub fn new_strict() -> Ruleset {
    Ruleset::new(Mode::Strict)
}

/// Creates a policy that drops the rights the running kernel cannot enforce.
///
/// If a rule ends up granting nothing (for example `Access::ResolveUnix` on a
/// kernel older than Landlock ABI 9), the rule is skipped, which leaves the
/// affected path denied. See [`new_strict`] for the alternative.
#[must_use]
pub fn new_best_effort() -> Ruleset {
    Ruleset::new(Mode::BestEffort)
}

/// The Landlock ABI version supported by the running kernel.
///
/// # Errors
///
/// Returns [`Error::NotSupported`] when Landlock is not available. This is the
/// same condition as [`is_supported`] returning `false`.
pub fn abi_version() -> Result<u32, Error> {
    sys::abi_version().map_err(|source| Error::NotSupported {
        source,
    })
}

/// Whether the running kernel supports Landlock at all.
#[must_use]
pub fn is_supported() -> bool {
    abi_version().is_ok()
}

impl Ruleset {
    fn new(mode: Mode) -> Ruleset {
        match sys::abi_version() {
            Ok(abi) if abi >= 1 => Ruleset::with_abi(mode, abi),
            Ok(_) => Ruleset::unavailable(Error::NotSupported {
                source: io::Error::new(io::ErrorKind::Unsupported, "Landlock reported ABI version 0"),
            }),
            Err(source) => Ruleset::unavailable(Error::NotSupported {
                source,
            }),
        }
    }

    /// A ruleset whose policy is decided against Landlock ABI `abi`, used to
    /// simulate older kernels. `abi` must not exceed the running kernel's, or
    /// creating the rulesets fails.
    fn with_abi(mode: Mode, abi: u32) -> Ruleset {
        let handled_fs = FS_ALL_BITS & fs_supported(abi);
        let mut ruleset = Ruleset {
            mode,
            abi,
            handled_fs,
            fs: None,
            net: None,
            scoped: SCOPE_ALL_BITS & scope_supported(abi),
            error: None,
        };

        // These rulesets are always created, even without rules: everything is
        // denied by default.
        ruleset.fs = match sys::create_ruleset(handled_fs, 0, 0) {
            Ok(fd) => Some(fd),
            Err(source) => {
                ruleset.error = Some(Error::Syscall {
                    operation: Operation::CreateRuleset,
                    source,
                });
                return ruleset;
            }
        };

        let handled_net = NET_ALL_BITS & net_supported(abi);
        if handled_net != 0 {
            ruleset.net = match sys::create_ruleset(0, handled_net, 0) {
                Ok(fd) => Some(fd),
                Err(source) => {
                    ruleset.error = Some(Error::Syscall {
                        operation: Operation::CreateRuleset,
                        source,
                    });
                    None
                }
            };
        }

        ruleset
    }

    fn unavailable(error: Error) -> Ruleset {
        Ruleset {
            mode: Mode::BestEffort,
            abi: 0,
            handled_fs: 0,
            fs: None,
            net: None,
            scoped: 0,
            error: Some(error),
        }
    }

    fn fail(&mut self, error: Error) {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    /// The number of access rights handled by the filesystem ruleset.
    fn fs_fd(&self) -> Option<BorrowedFd<'_>> {
        self.fs.as_ref().map(OwnedFd::as_fd)
    }

    fn net_fd(&self) -> Option<BorrowedFd<'_>> {
        self.net.as_ref().map(OwnedFd::as_fd)
    }

    /// Grants `access` on each path of `paths`.
    ///
    /// The same access rights are granted on every path. Paths are opened with
    /// `O_PATH`, which requires no permission on the target, and rules are
    /// attached to the objects the paths resolve to (symlinks are followed).
    ///
    /// Rights that do not apply to the object are dropped (for example
    /// [`Access::RemoveFile`] on a regular file). Requesting access that grants
    /// nothing on the object (for example [`Access::ReadDir`] on a regular
    /// file) is an invalid rule and is reported by [`Ruleset::restrict_self`].
    ///
    /// Failure modes are deferred to [`Ruleset::restrict_self`]; this method
    /// never returns an error.
    #[must_use]
    pub fn allow_filesystem(mut self, access: Access, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Self {
        for path in paths {
            self.add_filesystem_path(access, path.as_ref());
        }
        self
    }

    /// Grants `access` on each of `fds`, which must identify file hierarchies.
    ///
    /// This is the equivalent of [`Ruleset::allow_filesystem`] for callers that
    /// want to control how the path is resolved (for example with `openat2` and
    /// `RESOLVE_NO_SYMLINKS`). The descriptors may be closed once
    /// [`Ruleset::restrict_self`] returned: the kernel keeps a reference to the
    /// underlying objects.
    ///
    /// Failure modes are deferred to [`Ruleset::restrict_self`]; this method
    /// never returns an error.
    #[must_use]
    pub fn allow_file_descriptor(mut self, access: Access, fds: impl IntoIterator<Item = impl AsFd>) -> Self {
        for fd in fds {
            self.add_filesystem_fd(access, fd.as_fd(), None);
        }
        self
    }

    /// Grants `access` on each of `ports`.
    ///
    /// Port `0` covers the ephemeral port range for binding. Landlock has no
    /// wildcard port, so every port has to be listed. Only TCP and UDP are
    /// restricted.
    ///
    /// Failure modes are deferred to [`Ruleset::restrict_self`]; this method
    /// never returns an error.
    #[must_use]
    pub fn allow_network(mut self, access: NetworkAccess, ports: impl IntoIterator<Item = u16>) -> Self {
        self.add_network(access, ports.into_iter());
        self
    }

    /// Lifts the IPC restriction described by `scope`.
    ///
    /// Scopes are active by default. Unlike filesystem and network access,
    /// Landlock cannot grant exceptions to a scope: it can only be lifted as a
    /// whole.
    ///
    /// Failure modes are deferred to [`Ruleset::restrict_self`]; this method
    /// never returns an error.
    #[must_use]
    pub fn allow_scope(mut self, scope: Scope) -> Self {
        self.scoped &= !scope.bits();
        self
    }

    fn add_filesystem_path(&mut self, access: Access, path: &Path) {
        if self.error.is_some() || self.fs.is_none() {
            return;
        }

        use std::os::unix::ffi::OsStrExt;
        let bytes = path.as_os_str().as_bytes();

        if bytes.contains(&0) {
            self.fail(Error::InvalidPath {
                path: path.to_path_buf(),
                reason: PathError::ContainsNul,
            });
            return;
        }
        if bytes.len() >= PATH_MAX {
            self.fail(Error::InvalidPath {
                path: path.to_path_buf(),
                reason: PathError::TooLong,
            });
            return;
        }

        let mut buffer = [0u8; PATH_MAX];
        buffer[..bytes.len()].copy_from_slice(bytes);

        let fd = match sys::open_path(&buffer[..=bytes.len()]) {
            Ok(fd) => fd,
            Err(source) => {
                self.fail(Error::Open {
                    path: path.to_path_buf(),
                    source,
                });
                return;
            }
        };
        self.add_filesystem_fd(access, fd.as_fd(), Some(path));
    }

    fn add_filesystem_fd(&mut self, access: Access, fd: BorrowedFd<'_>, path: Option<&Path>) {
        let Some(ruleset_fd) = self.fs_fd() else {
            return;
        };

        let is_dir = match sys::fd_is_dir(fd.as_raw_fd()) {
            Ok(is_dir) => is_dir,
            Err(source) => {
                self.fail(Error::Syscall {
                    operation: Operation::Stat,
                    source,
                });
                return;
            }
        };

        // Rights that do not apply to the object are dropped: the kernel
        // rejects a rule that grants directory-only rights on a file.
        let applicable = if is_dir {
            access.bits()
        } else {
            access.bits() & FS_FILE_BITS
        };
        if applicable == 0 {
            self.fail(Error::InapplicableAccess {
                access,
                path: path.map(Path::to_path_buf),
            });
            return;
        }

        let supported = fs_supported(self.abi);
        let allowed = match self.mode {
            Mode::Strict => {
                if applicable & !supported != 0 {
                    self.fail(Error::Unsupported {
                        feature: Feature::FileSystem(access),
                        kernel_abi: self.abi,
                        required_abi: min_abi_fs(applicable),
                    });
                    return;
                }
                applicable
            }
            Mode::BestEffort => {
                let trimmed = applicable & supported;
                if trimmed == 0 {
                    return;
                }
                trimmed
            }
        };

        if let Err(source) = sys::add_path_rule(ruleset_fd.as_raw_fd(), allowed, fd.as_raw_fd()) {
            match source.raw_os_error() {
                Some(EBADFD) => self.fail(Error::InvalidFileDescriptor {
                    source,
                }),
                // The kernel only rejects a path rule with EINVAL when the
                // access rights do not apply to the object.
                Some(libc::EINVAL) => self.fail(Error::InapplicableAccess {
                    access,
                    path: path.map(Path::to_path_buf),
                }),
                _ => self.fail(Error::Syscall {
                    operation: Operation::AddRule,
                    source,
                }),
            }
        }
    }

    fn add_network(&mut self, access: NetworkAccess, ports: impl Iterator<Item = u16>) {
        if self.error.is_some() {
            return;
        }

        let supported = net_supported(self.abi);
        let allowed = match self.mode {
            Mode::Strict => {
                if access.bits() & !supported != 0 {
                    self.fail(Error::Unsupported {
                        feature: Feature::Network(access),
                        kernel_abi: self.abi,
                        required_abi: min_abi_net(access.bits()),
                    });
                    return;
                }
                access.bits()
            }
            Mode::BestEffort => {
                let trimmed = access.bits() & supported;
                if trimmed == 0 {
                    return;
                }
                trimmed
            }
        };

        let Some(ruleset_fd) = self.net_fd().map(|fd| fd.as_raw_fd()) else {
            return;
        };

        for port in ports {
            if let Err(source) = sys::add_net_rule(ruleset_fd, allowed, port) {
                match source.raw_os_error() {
                    Some(libc::EAFNOSUPPORT) => {
                        // The kernel has no TCP/IP support; the rule cannot be
                        // enforced, and there is nothing to restrict.
                        if self.mode == Mode::Strict {
                            self.fail(Error::Syscall {
                                operation: Operation::AddRule,
                                source,
                            });
                        }
                        return;
                    }
                    _ => self.fail(Error::Syscall {
                        operation: Operation::AddRule,
                        source,
                    }),
                }
            }
        }
    }

    /// Applies the policy to the calling thread (and, on Landlock ABI 8 and
    /// newer, to every thread of the process).
    ///
    /// This consumes the ruleset, which is final: restrictions can never be
    /// relaxed afterwards, only added to. On success, the returned ABI version
    /// is the version reported by the running kernel, and the restrictions stay
    /// in place when the builder is dropped.
    ///
    /// `PR_SET_NO_NEW_PRIVS` is set on the calling thread first (on Landlock ABI
    /// 11 and newer the kernel does it as part of the enforcement). It is never
    /// unset, and it prevents the process from gaining privileges through
    /// set-user-ID programs.
    ///
    /// # Errors
    ///
    /// Returns the first error encountered while building the policy (see
    /// [`Error`]) without applying it, or an error reported by the kernel.
    pub fn restrict_self(self) -> Result<u32, Error> {
        if let Some(error) = self.error {
            return Err(error);
        }

        let (flags, atomic_no_new_privs) = restrict_flags(self.abi);
        if !atomic_no_new_privs {
            sys::set_no_new_privs().map_err(|source| Error::Syscall {
                operation: Operation::SetNoNewPrivs,
                source,
            })?;
        }

        // The scope restrictions have no rules, so this ruleset is created last
        // and only when a scope is still active.
        if self.scoped != 0 {
            let scoped = self.scoped & scope_supported(self.abi);
            if scoped != 0 {
                match sys::create_ruleset(0, 0, scoped) {
                    Ok(fd) => enforce(&fd, flags)?,
                    Err(source) => {
                        return Err(Error::Syscall {
                            operation: Operation::CreateRuleset,
                            source,
                        });
                    }
                }
            }
        }

        for fd in [self.fs.as_ref(), self.net.as_ref()].into_iter().flatten() {
            enforce(fd, flags)?;
        }

        Ok(self.abi)
    }
}

/// The `landlock_restrict_self(2)` flags to use on a kernel reporting `abi`,
/// and whether the kernel sets `no_new_privs` itself. Landlock ABI 8 added the
/// `TSYNC` flag (process-wide enforcement) and ABI 11 added the atomic
/// `no_new_privs` flag.
const fn restrict_flags(abi: u32) -> (u32, bool) {
    let mut flags = 0u32;
    if abi >= 8 {
        // Applies the policy to every thread of the process.
        flags |= sys::LANDLOCK_RESTRICT_SELF_TSYNC;
    }
    let atomic_no_new_privs = abi >= 11;
    if atomic_no_new_privs {
        flags |= sys::LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS;
    }
    (flags, atomic_no_new_privs)
}

/// Applies one ruleset layer, mapping the kernel's errors.
fn enforce(fd: &OwnedFd, flags: u32) -> Result<(), Error> {
    match sys::restrict_self(fd.as_raw_fd(), flags) {
        Ok(()) => Ok(()),
        Err(source) => match source.raw_os_error() {
            Some(libc::E2BIG) => Err(Error::LayerLimitReached),
            _ => Err(Error::Syscall {
                operation: Operation::RestrictSelf,
                source,
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::CStr, os::unix::ffi::OsStrExt};

    use super::*;

    /// Runs `f` in a forked child and returns its exit status. The closure must
    /// not allocate (see the integration tests for the rationale).
    fn in_child(f: impl FnOnce() -> i32) -> i32 {
        let pid = unsafe { libc::fork() };
        assert!(pid >= 0, "fork() failed");
        if pid == 0 {
            let code = f();
            unsafe { libc::_exit(code) };
        }
        let mut status = 0;
        let waited = unsafe { libc::waitpid(pid, &mut status, 0) };
        assert_eq!(waited, pid, "waitpid() failed");
        if libc::WIFEXITED(status) {
            libc::WEXITSTATUS(status)
        } else {
            -1
        }
    }

    fn real_abi() -> Option<u32> {
        abi_version().ok()
    }

    fn open_errno(path: &CStr, flags: libc::c_int) -> i32 {
        let fd = unsafe { libc::open(path.as_ptr(), flags) };
        if fd < 0 {
            return std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
        }
        unsafe { libc::close(fd) };
        0
    }

    #[test]
    fn groups_expand_to_the_documented_rights() {
        assert_eq!(Access::Read.bits(), sys::FS_READ_FILE | sys::FS_READ_DIR);
        assert_eq!(Access::Write.bits(), FS_WRITE_BITS);
        assert_eq!(Access::ReadWrite.bits(), FS_READ_BITS | FS_WRITE_BITS);
        assert_eq!(Access::All.bits(), FS_ALL_BITS);

        // `Write` covers the rights that make writing work, including
        // truncation and reparenting.
        assert_eq!(Access::Write.bits() & sys::FS_TRUNCATE, sys::FS_TRUNCATE);
        assert_eq!(Access::Write.bits() & sys::FS_REFER, sys::FS_REFER);
        assert_eq!(Access::Write.bits() & sys::FS_WRITE_FILE, sys::FS_WRITE_FILE);
        // Device ioctls and UNIX socket lookups are not part of `ReadWrite`.
        assert_eq!(Access::ReadWrite.bits() & sys::FS_IOCTL_DEV, 0);
        assert_eq!(Access::ReadWrite.bits() & sys::FS_RESOLVE_UNIX, 0);
        assert_eq!(
            Access::All.bits() & sys::FS_IOCTL_DEV,
            sys::FS_IOCTL_DEV,
            "`All` includes device ioctls"
        );
        assert_eq!(
            Access::All.bits() & sys::FS_RESOLVE_UNIX,
            sys::FS_RESOLVE_UNIX,
            "`All` includes UNIX socket lookups"
        );

        assert_eq!(NetworkAccess::Tcp.bits(), NET_TCP_BITS);
        assert_eq!(NetworkAccess::Udp.bits(), NET_UDP_BITS);
        assert_eq!(NetworkAccess::All.bits(), NET_ALL_BITS);
        assert_eq!(Scope::All.bits(), SCOPE_ALL_BITS);
    }

    #[test]
    fn the_abi_tables_cover_every_bit_exactly_once() {
        let mut covered = 0u64;
        for (bit, introduced) in FS_ABI_TABLE {
            assert!(introduced >= 1);
            assert_eq!(covered & bit, 0, "bit {bit:#x} is listed twice");
            covered |= bit;
        }
        assert_eq!(covered, FS_ALL_BITS, "a filesystem right is missing");

        let mut covered = 0u32;
        for (bit, introduced) in NET_ABI_TABLE {
            assert!(introduced >= 1);
            assert_eq!(covered & bit, 0, "bit {bit:#x} is listed twice");
            covered |= bit;
        }
        assert_eq!(covered, NET_ALL_BITS, "a network right is missing");

        let mut covered = 0u32;
        for (bit, introduced) in SCOPE_ABI_TABLE {
            assert!(introduced >= 1);
            assert_eq!(covered & bit, 0, "bit {bit:#x} is listed twice");
            covered |= bit;
        }
        assert_eq!(covered, SCOPE_ALL_BITS, "a scope is missing");
    }

    #[test]
    fn the_abi_tables_match_the_documented_kernels() {
        assert_eq!(fs_supported(0), 0);
        assert_eq!(fs_supported(1), 0x1FFF, "Linux 5.13: rights 0 to 12");
        assert_eq!(fs_supported(2), 0x3FFF, "Linux 5.19: adds refer");
        assert_eq!(fs_supported(3), 0x7FFF, "Linux 6.2: adds truncate");
        assert_eq!(fs_supported(4), 0x7FFF, "Linux 6.7: network only");
        assert_eq!(fs_supported(5), 0xFFFF, "Linux 6.10: adds ioctl-dev");
        assert_eq!(fs_supported(8), 0xFFFF, "Linux 7.0: no new filesystem right");
        assert_eq!(fs_supported(9), 0x1FFFF, "Linux 7.1: adds resolve-unix");
        assert_eq!(fs_supported(100), FS_ALL_BITS, "unknown rights are never sent");

        assert_eq!(net_supported(3), 0);
        assert_eq!(net_supported(4), 0b0011, "Linux 6.7: TCP");
        assert_eq!(net_supported(9), 0b0011);
        assert_eq!(net_supported(10), 0b1111, "Linux 7.2: UDP");

        assert_eq!(scope_supported(5), 0);
        assert_eq!(scope_supported(6), 0b11, "Linux 6.12: both scopes");

        assert_eq!(abi_kernel_version(1), Some("5.13"));
        assert_eq!(abi_kernel_version(2), Some("5.19"));
        assert_eq!(abi_kernel_version(8), Some("7.0"));
        assert_eq!(abi_kernel_version(12), None);
    }

    #[test]
    fn the_minimum_abi_of_every_group_is_the_documented_one() {
        assert_eq!(min_abi_fs(0), 0);
        assert_eq!(min_abi_fs(Access::Read.bits()), 1);
        assert_eq!(min_abi_fs(Access::Execute.bits()), 1);
        assert_eq!(min_abi_fs(Access::Refer.bits()), 2);
        assert_eq!(min_abi_fs(Access::Write.bits()), 3);
        assert_eq!(min_abi_fs(Access::ReadWrite.bits()), 3);
        assert_eq!(min_abi_fs(Access::IoctlDev.bits()), 5);
        assert_eq!(min_abi_fs(Access::All.bits()), 9);
        assert_eq!(min_abi_fs(Access::ResolveUnix.bits()), 9);
        assert_eq!(min_abi_net(NetworkAccess::Tcp.bits()), 4);
        assert_eq!(min_abi_net(NetworkAccess::Udp.bits()), 10);
    }

    #[test]
    fn unsupported_errors_name_the_required_abi_and_kernel() {
        let error = Error::Unsupported {
            feature: Feature::FileSystem(Access::All),
            kernel_abi: 2,
            required_abi: 9,
        };
        let message = error.to_string();
        assert!(message.contains("access right `all`"), "{message}");
        assert!(message.contains("ABI 9"), "{message}");
        assert!(message.contains("Linux 7.1"), "{message}");
        assert!(message.contains("ABI 2"), "{message}");
        assert!(std::error::Error::source(&error).is_none());
    }

    #[test]
    fn restrict_flags_follow_the_abi() {
        // Before Landlock ABI 8 there is no process-wide enforcement, and
        // `no_new_privs` is always set by this crate.
        assert_eq!(restrict_flags(1), (0, false));
        assert_eq!(restrict_flags(7), (0, false));
        // ABI 8: `TSYNC` for process-wide enforcement.
        assert_eq!(restrict_flags(8), (sys::LANDLOCK_RESTRICT_SELF_TSYNC, false));
        assert_eq!(restrict_flags(10), (sys::LANDLOCK_RESTRICT_SELF_TSYNC, false));
        // ABI 11: the kernel sets `no_new_privs` itself.
        let (flags, atomic) = restrict_flags(11);
        assert_eq!(
            flags,
            sys::LANDLOCK_RESTRICT_SELF_TSYNC | sys::LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS
        );
        assert!(atomic);
        assert!(restrict_flags(100).1, "newer kernels keep the behaviour");
    }

    #[test]
    fn strict_mode_rejects_rights_an_older_kernel_cannot_enforce() {
        let Some(real) = real_abi() else {
            return;
        };
        if real < 2 {
            return;
        }
        // Simulate Linux 6.0 (Landlock ABI 2): truncation needs ABI 3.
        let error = Ruleset::with_abi(Mode::Strict, 2)
            .allow_filesystem(Access::Write, ["/tmp"])
            .restrict_self()
            .unwrap_err();
        match error {
            Error::Unsupported {
                kernel_abi,
                required_abi,
                ..
            } => {
                assert_eq!(kernel_abi, 2);
                assert_eq!(required_abi, 3);
            }
            other => panic!("unexpected error: {other}"),
        }

        // Rights known to ABI 2 are still accepted.
        let code = in_child(|| {
            match Ruleset::with_abi(Mode::Strict, 2)
                .allow_filesystem(Access::Read, ["/tmp"])
                .restrict_self()
            {
                Ok(abi) => i32::from(abi != 2),
                Err(_) => 1,
            }
        });
        assert_eq!(code, 0);
    }

    #[test]
    fn best_effort_trims_rights_an_older_kernel_cannot_enforce() {
        let Some(real) = real_abi() else {
            return;
        };
        if real < 2 {
            return;
        }
        let code = in_child(|| {
            match Ruleset::with_abi(Mode::BestEffort, 2)
                .allow_filesystem(Access::Write, ["/tmp"])
                .restrict_self()
            {
                Ok(abi) => i32::from(abi != 2),
                Err(_) => 1,
            }
        });
        assert_eq!(code, 0);
    }

    /// End-to-end check of the ABI masking: a policy built against Landlock
    /// ABI 2 cannot restrict truncation, while the same policy built against
    /// the running kernel's ABI can.
    #[test]
    fn older_kernels_cannot_restrict_truncation() {
        let Some(real) = real_abi() else {
            return;
        };
        if real < 3 {
            // The running kernel cannot restrict truncation either.
            return;
        }

        let directory = std::env::temp_dir().join(format!("landlock-truncate-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).unwrap();
        let file = directory.join("file");
        std::fs::write(&file, b"hello").unwrap();
        let c_directory = std::ffi::CString::new(directory.as_os_str().as_bytes()).unwrap();
        let c_file = std::ffi::CString::new(file.as_os_str().as_bytes()).unwrap();
        let c_file_again = c_file.clone();

        let granted = directory.clone();
        // The running kernel knows about `Access::Truncate`, so a policy that
        // handles it without granting it makes `O_TRUNC` fail...
        let denied = in_child(move || {
            if Ruleset::with_abi(Mode::BestEffort, real)
                .allow_filesystem(Access::WriteFile, [granted])
                .restrict_self()
                .is_err()
            {
                return -1;
            }
            open_errno(&c_file, libc::O_WRONLY | libc::O_TRUNC)
        });
        assert_eq!(denied, libc::EACCES, "...but truncation must be denied");

        let granted = directory.clone();
        // ...while the same policy on a kernel that only knows ABI 2 cannot
        // restrict truncation at all, so it is allowed everywhere.
        let allowed = in_child(move || {
            if Ruleset::with_abi(Mode::BestEffort, 2)
                .allow_filesystem(Access::WriteFile, [granted])
                .restrict_self()
                .is_err()
            {
                return -1;
            }
            open_errno(&c_file_again, libc::O_WRONLY | libc::O_TRUNC)
        });
        assert_eq!(allowed, 0, "a Linux 6.0 kernel cannot restrict truncation");

        let _ = &c_directory;
        let _ = std::fs::remove_dir_all(&directory);
    }
}
