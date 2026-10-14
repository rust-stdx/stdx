//! Black-box tests for the `landlock` crate.
//!
//! Enforcing a policy is irreversible and applies to the calling thread, so
//! every test that may enforce one runs in a forked child. Children only use
//! allocation-free system calls: the test harness runs many threads, and
//! allocating in a forked child can deadlock on an allocator lock held by
//! another thread at `fork` time.

#![cfg(target_os = "linux")]

use std::{
    ffi::{CStr, CString, OsString},
    io::Write,
    os::{
        fd::{AsFd, BorrowedFd, FromRawFd, OwnedFd},
        linux::net::SocketAddrExt,
        unix::{
            ffi::{OsStrExt, OsStringExt},
            net::{SocketAddr, UnixListener},
        },
    },
    path::{Path, PathBuf},
};

use landlock::{Access, Error, NetworkAccess, PathError, Scope};

/// Runs `f` in a forked child and returns its exit status (`-1` when the child
/// was killed by a signal). The closure must not allocate.
fn in_child(f: impl FnOnce() -> i32) -> i32 {
    let _ = std::io::stdout().flush();
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

fn supported() -> bool {
    landlock::is_supported()
}

fn landlock_abi() -> u32 {
    landlock::abi_version().unwrap_or(0)
}

fn cstr(path: &Path) -> CString {
    CString::new(path.as_os_str().as_bytes()).unwrap()
}

fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1)
}

/// Returns 0 when the path could be opened, its errno otherwise.
fn open_errno(path: &CStr, flags: libc::c_int) -> i32 {
    let fd = unsafe { libc::open(path.as_ptr(), flags) };
    if fd < 0 {
        return errno();
    }
    unsafe { libc::close(fd) };
    0
}

fn mkdir_errno(path: &CStr) -> i32 {
    if unsafe { libc::mkdir(path.as_ptr(), 0o700) } < 0 {
        errno()
    } else {
        0
    }
}

fn kill_errno(pid: libc::pid_t) -> i32 {
    if unsafe { libc::kill(pid, 0) } < 0 { errno() } else { 0 }
}

fn tcp_bind_errno(port: u16) -> i32 {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    if fd < 0 {
        return errno();
    }
    let addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port.to_be(),
        sin_addr: libc::in_addr {
            s_addr: u32::from(std::net::Ipv4Addr::LOCALHOST).to_be(),
        },
        sin_zero: [0; 8],
    };
    let ret = unsafe {
        libc::bind(
            fd,
            &addr as *const libc::sockaddr_in as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        )
    };
    let code = if ret < 0 { errno() } else { 0 };
    unsafe { libc::close(fd) };
    code
}

fn tcp_connect_errno(port: u16) -> i32 {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    if fd < 0 {
        return errno();
    }
    let addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port.to_be(),
        sin_addr: libc::in_addr {
            s_addr: u32::from(std::net::Ipv4Addr::LOCALHOST).to_be(),
        },
        sin_zero: [0; 8],
    };
    let ret = unsafe {
        libc::connect(
            fd,
            &addr as *const libc::sockaddr_in as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        )
    };
    let code = if ret < 0 { errno() } else { 0 };
    unsafe { libc::close(fd) };
    code
}

/// Connects to an abstract UNIX socket (`name` without its leading NUL byte).
fn connect_abstract_errno(name: &[u8]) -> i32 {
    let fd = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0) };
    if fd < 0 {
        return errno();
    }
    let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    addr.sun_family = libc::AF_UNIX as libc::sa_family_t;
    // The abstract name is a NUL byte followed by the name.
    let path = unsafe { std::slice::from_raw_parts_mut(addr.sun_path.as_mut_ptr().cast::<u8>(), addr.sun_path.len()) };
    path[1..=name.len()].copy_from_slice(name);
    let len = (std::mem::size_of::<libc::sa_family_t>() + 1 + name.len()) as libc::socklen_t;
    let ret = unsafe { libc::connect(fd, &addr as *const _ as *const libc::sockaddr, len) };
    let code = if ret < 0 { errno() } else { 0 };
    unsafe { libc::close(fd) };
    code
}

/// A directory below the system temporary directory, removed on drop.
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> TempDir {
        let path = std::env::temp_dir().join(format!("landlock-test-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("cannot create the temporary directory");
        TempDir {
            path,
        }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn join(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }

    fn file(&self, name: &str, contents: &[u8]) -> PathBuf {
        let path = self.join(name);
        std::fs::write(&path, contents).expect("cannot write the test file");
        path
    }

    fn dir(&self, name: &str) -> PathBuf {
        let path = self.join(name);
        std::fs::create_dir_all(&path).expect("cannot create the test directory");
        path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

// ---------------------------------------------------------------------------
// Availability
// ---------------------------------------------------------------------------

#[test]
fn abi_version_matches_is_supported() {
    assert_eq!(landlock::is_supported(), landlock::abi_version().is_ok());
    match landlock::abi_version() {
        Ok(abi) => {
            assert!(abi >= 1, "unexpected ABI version {abi}");
        }
        Err(Error::NotSupported {
            ..
        }) => {}
        Err(other) => panic!("unexpected error: {other}"),
    }
}

// ---------------------------------------------------------------------------
// Invalid input: reported by `restrict_self`, without enforcing anything
// ---------------------------------------------------------------------------

#[test]
fn path_with_a_nul_byte_is_rejected() {
    if !supported() {
        return;
    }
    let path = PathBuf::from(OsString::from_vec(b"/etc/hostname\0evil".to_vec()));
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [path])
        .restrict_self()
        .unwrap_err();
    assert!(
        matches!(
            error,
            Error::InvalidPath {
                reason: PathError::ContainsNul,
                ..
            }
        ),
        "unexpected error: {error}"
    );
}

#[test]
fn path_longer_than_path_max_is_rejected() {
    if !supported() {
        return;
    }
    let path = format!("/{}", "a".repeat(5000));
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [path])
        .restrict_self()
        .unwrap_err();
    assert!(
        matches!(
            error,
            Error::InvalidPath {
                reason: PathError::TooLong,
                ..
            }
        ),
        "unexpected error: {error}"
    );
}

#[test]
fn missing_path_is_rejected() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("missing");
    let missing = directory.join("does-not-exist");
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [missing.clone()])
        .restrict_self()
        .unwrap_err();
    match error {
        Error::Open {
            path,
            source,
        } => {
            assert_eq!(path, missing);
            assert_eq!(source.raw_os_error(), Some(libc::ENOENT));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn directory_only_access_on_a_file_is_rejected() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("inapplicable");
    let file = directory.file("file", b"data");
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::ReadDir, [file])
        .restrict_self()
        .unwrap_err();
    assert!(
        matches!(
            error,
            Error::InapplicableAccess {
                access: Access::ReadDir,
                ..
            }
        ),
        "unexpected error: {error}"
    );
}

#[test]
fn a_descriptor_that_is_not_a_file_hierarchy_is_rejected() {
    if !supported() {
        return;
    }
    let mut fds = [0; 2];
    assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
    // `borrow_raw` is sound here: the descriptor stays open until after the
    // policy is built.
    let read_end = unsafe { BorrowedFd::borrow_raw(fds[0]) };
    let error = landlock::new_best_effort()
        .allow_file_descriptor(Access::Read, [read_end])
        .restrict_self()
        .unwrap_err();
    unsafe {
        libc::close(fds[0]);
        libc::close(fds[1]);
    }
    assert!(
        matches!(error, Error::InvalidFileDescriptor { .. }),
        "unexpected error: {error}"
    );
}

#[test]
fn the_first_error_is_reported() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("first-error");
    let first = directory.join("nope");
    let second = directory.join("nope-too");
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [first.clone()])
        .allow_filesystem(Access::Read, [second])
        .restrict_self()
        .unwrap_err();
    match error {
        Error::Open {
            path, ..
        } => assert_eq!(path, first),
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn errors_prevent_enforcement() {
    if !supported() {
        return;
    }
    let readable = CString::new("/etc/passwd").unwrap();
    let missing = PathBuf::from("/does/not/exist");
    let code = in_child(move || {
        let error = landlock::new_best_effort()
            .allow_filesystem(Access::Read, [missing])
            .restrict_self();
        if error.is_ok() {
            return 1;
        }
        // No restriction must have been applied.
        if open_errno(&readable, libc::O_RDONLY) == 0 {
            0
        } else {
            2
        }
    });
    assert_eq!(code, 0, "an invalid policy must not restrict anything");
}

// ---------------------------------------------------------------------------
// Strict and best-effort policies
// ---------------------------------------------------------------------------

#[test]
fn strict_rejects_filesystem_access_the_kernel_cannot_enforce() {
    if !supported() {
        return;
    }
    let abi = landlock_abi();
    let directory = TempDir::new("strict");
    if abi < 9 {
        // `Access::All` includes `Access::ResolveUnix` (Landlock ABI 9).
        let error = landlock::new_strict()
            .allow_filesystem(Access::All, [directory.path().to_path_buf()])
            .restrict_self()
            .unwrap_err();
        match error {
            Error::Unsupported {
                feature,
                kernel_abi,
                required_abi,
            } => {
                assert_eq!(feature, landlock::Feature::FileSystem(Access::All));
                assert_eq!(kernel_abi, abi);
                assert_eq!(required_abi, 9);
            }
            other => panic!("unexpected error: {other}"),
        }
    }
    if abi < 3 {
        let error = landlock::new_strict()
            .allow_filesystem(Access::Write, [directory.path().to_path_buf()])
            .restrict_self()
            .unwrap_err();
        match error {
            Error::Unsupported {
                required_abi, ..
            } => assert_eq!(required_abi, 3),
            other => panic!("unexpected error: {other}"),
        }
    }
}

#[test]
fn strict_rejects_network_access_the_kernel_cannot_enforce() {
    if !supported() {
        return;
    }
    if landlock_abi() >= 10 {
        return;
    }
    let error = landlock::new_strict()
        .allow_network(NetworkAccess::Udp, [53])
        .restrict_self()
        .unwrap_err();
    match error {
        Error::Unsupported {
            required_abi, ..
        } => assert_eq!(required_abi, 10),
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn best_effort_trims_access_the_kernel_cannot_enforce() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("best-effort");
    // Everything the child needs is built here: children must not allocate.
    let path = directory.path().to_path_buf();
    let other = path.clone();
    let code = in_child(move || {
        match landlock::new_best_effort()
            .allow_filesystem(Access::All, [path])
            .allow_network(NetworkAccess::All, [1])
            .allow_filesystem(Access::ResolveUnix, [other])
            .restrict_self()
        {
            Ok(abi) => i32::from(abi < 1),
            Err(_) => 1,
        }
    });
    assert_eq!(code, 0);
}

#[test]
fn strict_accepts_access_the_kernel_supports() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("strict-supported");
    let path = directory.path().to_path_buf();
    let other = path.clone();
    let code = in_child(move || {
        match landlock::new_strict()
            .allow_filesystem(Access::Read, [path])
            .allow_filesystem(Access::Execute, [other])
            .restrict_self()
        {
            Ok(_) => 0,
            Err(_) => 1,
        }
    });
    assert_eq!(code, 0);
}

// ---------------------------------------------------------------------------
// Filesystem enforcement
// ---------------------------------------------------------------------------

#[test]
fn filesystem_is_denied_without_a_rule() {
    if !supported() {
        return;
    }
    let outside = CString::new("/etc/passwd").unwrap();
    let code = in_child(|| {
        if landlock::new_best_effort().restrict_self().is_err() {
            return 1;
        }
        i32::from(open_errno(&outside, libc::O_RDONLY) != libc::EACCES)
    });
    assert_eq!(code, 0, "reading a file without a rule must fail with EACCES");
}

#[test]
fn granted_paths_are_readable_and_others_are_not() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("read");
    let file = cstr(&directory.file("file", b"hello"));
    let outside = CString::new("/etc/passwd").unwrap();
    let new_dir = cstr(&directory.join("sub"));
    let granted = directory.path().to_path_buf();
    let code = in_child(move || {
        if landlock::new_best_effort()
            .allow_filesystem(Access::Read, [granted])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        if open_errno(&file, libc::O_RDONLY) != 0 {
            return 2; // granted: must be readable
        }
        if open_errno(&outside, libc::O_RDONLY) != libc::EACCES {
            return 3; // not granted: denied
        }
        if mkdir_errno(&new_dir) != libc::EACCES {
            return 4; // write access was not granted
        }
        0
    });
    assert_eq!(code, 0);
}

#[test]
fn write_access_is_granted_where_allowed() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("write");
    let created = cstr(&directory.join("created"));
    let outside = cstr(&std::env::temp_dir().join(format!("landlock-outsider-{}", std::process::id())));
    let granted = directory.path().to_path_buf();
    let code = in_child(move || {
        if landlock::new_best_effort()
            .allow_filesystem(Access::ReadWrite, [granted])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        let fd = unsafe { libc::open(created.as_ptr(), libc::O_CREAT | libc::O_WRONLY | libc::O_CLOEXEC, 0o600) };
        if fd < 0 {
            return 2; // creating a file inside the granted directory must work
        }
        if unsafe { libc::write(fd, b"x".as_ptr().cast(), 1) } != 1 {
            return 3;
        }
        unsafe { libc::close(fd) };
        let denied = unsafe { libc::open(outside.as_ptr(), libc::O_CREAT | libc::O_WRONLY | libc::O_CLOEXEC, 0o600) };
        if denied >= 0 {
            unsafe { libc::close(denied) };
            return 4; // writing outside the granted directory is denied
        }
        0
    });
    assert_eq!(code, 0);
}

#[test]
fn group_access_on_a_file_grants_what_applies_to_it() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("file-trimming");
    let path = directory.file("file", b"hello");
    let file = cstr(&path);
    let code = in_child(move || {
        // `Access::ReadWrite` includes directory-only rights, which must be
        // dropped for a regular file instead of being rejected.
        if landlock::new_best_effort()
            .allow_filesystem(Access::ReadWrite, [path])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        let fd = unsafe { libc::open(file.as_ptr(), libc::O_WRONLY | libc::O_CLOEXEC) };
        if fd < 0 {
            return 2;
        }
        let written = unsafe { libc::write(fd, b"y".as_ptr().cast(), 1) };
        unsafe { libc::close(fd) };
        i32::from(written != 1)
    });
    assert_eq!(code, 0);
}

#[test]
fn non_utf8_paths_are_supported() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("non-utf8");
    // A directory whose name is not valid UTF-8.
    let mut name = directory.path().as_os_str().as_bytes().to_vec();
    name.push(b'/');
    name.extend_from_slice(b"invalid-\xff\xfe");
    let granted = PathBuf::from(OsString::from_vec(name));
    std::fs::create_dir_all(&granted).unwrap();
    let file = cstr(&granted.join("file"));
    std::fs::write(granted.join("file"), b"hello").unwrap();
    let code = in_child(move || {
        if landlock::new_best_effort()
            .allow_filesystem(Access::Read, [granted])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        i32::from(open_errno(&file, libc::O_RDONLY) != 0)
    });
    assert_eq!(code, 0);
}

#[test]
fn symlinked_paths_are_resolved_when_the_rule_is_added() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("symlink");
    let target = directory.dir("target");
    let link = directory.join("link");
    std::os::unix::fs::symlink(&target, &link).unwrap();
    let file = cstr(&target.join("file"));
    std::fs::write(target.join("file"), b"hello").unwrap();
    // The rule is created from the symlink: it must apply to its target.
    let code = in_child(move || {
        if landlock::new_best_effort()
            .allow_filesystem(Access::Read, [link])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        i32::from(open_errno(&file, libc::O_RDONLY) != 0)
    });
    assert_eq!(code, 0);
}

#[test]
fn file_descriptor_rules_work() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("fd");
    let file = cstr(&directory.file("file", b"hello"));
    let raw = unsafe { libc::open(cstr(directory.path()).as_ptr(), libc::O_PATH | libc::O_CLOEXEC) };
    assert!(raw >= 0, "cannot open the test directory");
    let owned = unsafe { OwnedFd::from_raw_fd(raw) };
    let code = in_child(|| {
        if landlock::new_best_effort()
            .allow_file_descriptor(Access::Read, [owned.as_fd()])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        i32::from(open_errno(&file, libc::O_RDONLY) != 0)
    });
    assert_eq!(code, 0);
}

#[test]
fn stacked_policies_intersect() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("stacking");
    let first_path = directory.dir("first");
    let second_path = directory.dir("second");
    let first_file = cstr(&directory.file("first/file", b"hello"));
    let second_file = cstr(&directory.file("second/file", b"hello"));
    let first_again = first_path.clone();
    let first = first_path;
    let second = second_path;
    let code = in_child(move || {
        if landlock::new_best_effort()
            .allow_filesystem(Access::Read, [first, second])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        if open_errno(&first_file, libc::O_RDONLY) != 0 || open_errno(&second_file, libc::O_RDONLY) != 0 {
            return 2;
        }
        // A second policy can only remove access: this one does not grant
        // `second_path` any more.
        if landlock::new_best_effort()
            .allow_filesystem(Access::Read, [first_again])
            .restrict_self()
            .is_err()
        {
            return 3;
        }
        if open_errno(&first_file, libc::O_RDONLY) != 0 {
            return 4;
        }
        i32::from(open_errno(&second_file, libc::O_RDONLY) != libc::EACCES)
    });
    assert_eq!(code, 0);
}

#[test]
fn restrictions_are_inherited_by_children() {
    if !supported() {
        return;
    }
    let outside = CString::new("/etc/passwd").unwrap();
    let code = in_child(|| {
        if landlock::new_best_effort().restrict_self().is_err() {
            return 1;
        }
        // Fork a grandchild: it inherits the (empty, deny-all) policy.
        in_child(|| i32::from(open_errno(&outside, libc::O_RDONLY) != libc::EACCES))
    });
    assert_eq!(code, 0);
}

#[test]
fn no_new_privs_is_set() {
    if !supported() {
        return;
    }
    let code = in_child(|| {
        if landlock::new_best_effort().restrict_self().is_err() {
            return 1;
        }
        let no_new_privs = unsafe { libc::prctl(libc::PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0) };
        i32::from(no_new_privs != 1)
    });
    assert_eq!(code, 0, "restrict_self must set PR_SET_NO_NEW_PRIVS");
}

// ---------------------------------------------------------------------------
// Network enforcement
// ---------------------------------------------------------------------------

#[test]
fn network_is_denied_without_a_rule() {
    if !supported() || landlock_abi() < 4 {
        return;
    }
    let code = in_child(|| {
        if landlock::new_best_effort().restrict_self().is_err() {
            return 1;
        }
        // Binding an ephemeral port is denied unless it is allowed.
        i32::from(tcp_bind_errno(0) == 0)
    });
    assert_eq!(code, 0);
}

#[test]
fn granted_ports_are_bindable() {
    if !supported() || landlock_abi() < 4 {
        return;
    }
    let code = in_child(|| {
        if landlock::new_best_effort()
            .allow_network(NetworkAccess::BindTcp, [0])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        // Port 0 means "any ephemeral port".
        if tcp_bind_errno(0) != 0 {
            return 2;
        }
        // This specific port was not granted.
        i32::from(tcp_bind_errno(40000) != libc::EACCES)
    });
    assert_eq!(code, 0);
}

#[test]
fn granted_ports_are_connectable() {
    if !supported() || landlock_abi() < 4 {
        return;
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("cannot listen");
    let port = listener.local_addr().unwrap().port();
    let code = in_child(|| {
        if landlock::new_best_effort()
            .allow_network(NetworkAccess::ConnectTcp, [port])
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        if tcp_connect_errno(port) != 0 {
            return 2; // granted: must be connectable
        }
        // A port that was not granted is denied.
        i32::from(tcp_connect_errno(port.wrapping_add(1)) != libc::EACCES)
    });
    drop(listener);
    assert_eq!(code, 0);
}

// ---------------------------------------------------------------------------
// Scope
// ---------------------------------------------------------------------------

#[test]
fn abstract_unix_sockets_are_denied_by_default() {
    if !supported() || landlock_abi() < 6 {
        return;
    }
    let name = format!("landlock-test-abstract-{}", std::process::id());
    let listener = UnixListener::bind_addr(&SocketAddr::from_abstract_name(name.as_bytes()).unwrap())
        .expect("cannot bind the abstract socket");
    let code = in_child(|| {
        if landlock::new_best_effort().restrict_self().is_err() {
            return 1;
        }
        i32::from(connect_abstract_errno(name.as_bytes()) != libc::EPERM)
    });
    drop(listener);
    assert_eq!(code, 0);
}

#[test]
fn allow_scope_permits_abstract_unix_sockets() {
    if !supported() || landlock_abi() < 6 {
        return;
    }
    let name = format!("landlock-test-scope-{}", std::process::id());
    let listener = UnixListener::bind_addr(&SocketAddr::from_abstract_name(name.as_bytes()).unwrap())
        .expect("cannot bind the abstract socket");
    let code = in_child(|| {
        if landlock::new_best_effort()
            .allow_scope(Scope::AbstractUnixSocket)
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        i32::from(connect_abstract_errno(name.as_bytes()) != 0)
    });
    drop(listener);
    assert_eq!(code, 0);
}

#[test]
fn signals_outside_the_domain_are_denied_by_default() {
    if !supported() || landlock_abi() < 6 {
        return;
    }
    let code = in_child(|| {
        if landlock::new_best_effort().restrict_self().is_err() {
            return 1;
        }
        // Signalling the parent (outside the sandbox) is denied...
        if kill_errno(unsafe { libc::getppid() }) != libc::EPERM {
            return 2;
        }
        // ...but the sandbox can still signal itself.
        i32::from(kill_errno(unsafe { libc::getpid() }) != 0)
    });
    assert_eq!(code, 0);
}

#[test]
fn allow_scope_permits_signalling_outside() {
    if !supported() || landlock_abi() < 6 {
        return;
    }
    let code = in_child(|| {
        if landlock::new_best_effort()
            .allow_scope(Scope::Signal)
            .restrict_self()
            .is_err()
        {
            return 1;
        }
        i32::from(kill_errno(unsafe { libc::getppid() }) != 0)
    });
    assert_eq!(code, 0);
}

// ---------------------------------------------------------------------------
// Resource handling
// ---------------------------------------------------------------------------

/// Counts the open file descriptors of the current process without allocating.
///
/// `/proc/self/fd` would allocate, so probe the descriptor table instead. Only
/// reliable in a freshly forked child, where no other thread can open or close
/// descriptors concurrently.
fn open_descriptor_count() -> i32 {
    let mut count = 0;
    for fd in 0..4096 {
        if unsafe { libc::fcntl(fd, libc::F_GETFD) } != -1 {
            count += 1;
        }
    }
    count
}

#[test]
fn building_a_policy_does_not_leak_descriptors() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("fds");
    let dir = directory.path().to_path_buf();
    let file = directory.file("file", b"hello");
    let missing = directory.join("does-not-exist");

    // Run in a child: the harness opens and closes descriptors from other
    // tests in parallel, which would make the count unstable.
    let code = in_child(move || {
        let before = open_descriptor_count();
        for _ in 0..64 {
            let builder = landlock::new_best_effort()
                .allow_filesystem(Access::Read, [&dir])
                .allow_filesystem(Access::ReadWrite, [&file])
                .allow_filesystem(Access::Read, [&missing]);
            drop(builder);
        }
        let after = open_descriptor_count();
        i32::from(before != after)
    });
    assert_eq!(code, 0, "descriptors leaked while building policies");
}

#[test]
fn many_rules_are_accepted() {
    if !supported() {
        return;
    }
    let directory = TempDir::new("many");
    let mut paths = Vec::new();
    for index in 0..256 {
        paths.push(directory.file(&format!("file-{index}"), b"x"));
    }
    let code = in_child(|| {
        match landlock::new_best_effort()
            .allow_filesystem(Access::Read, &paths)
            .restrict_self()
        {
            Ok(_) => 0,
            Err(_) => 1,
        }
    });
    assert_eq!(code, 0);
}

#[test]
fn ruleset_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<landlock::Ruleset>();
}

// ---------------------------------------------------------------------------
// Error reporting
// ---------------------------------------------------------------------------

#[test]
fn unsupported_errors_explain_what_is_needed() {
    if !supported() || landlock_abi() >= 9 {
        return;
    }
    let directory = TempDir::new("messages");
    let error = landlock::new_strict()
        .allow_filesystem(Access::All, [directory.path().to_path_buf()])
        .restrict_self()
        .unwrap_err();
    let message = error.to_string();
    assert!(message.contains("ABI 9"), "{message}");
    assert!(message.contains("Linux 7.1"), "{message}");
}

#[test]
fn access_rights_have_readable_names() {
    assert_eq!(Access::Read.to_string(), "read");
    assert_eq!(Access::WriteFile.to_string(), "write-file");
    assert_eq!(Access::ReadWrite.to_string(), "read-write");
    assert_eq!(Access::All.to_string(), "all");
    assert_eq!(NetworkAccess::ConnectTcp.to_string(), "connect-tcp");
    assert_eq!(Scope::AbstractUnixSocket.to_string(), "abstract-unix-socket");
}
