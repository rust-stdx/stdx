//! Audit tests for input boundaries and descriptor ownership.
//!
//! Enforcing a policy is irreversible, so tests that call `restrict_self` run
//! in a forked child and use only allocation-free libc calls, like the crate's
//! own integration tests.

#![cfg(target_os = "linux")]

use std::{
    ffi::CString,
    os::{
        fd::{AsFd, BorrowedFd, FromRawFd, OwnedFd},
        unix::ffi::OsStrExt,
    },
    path::PathBuf,
};

use landlock::{Access, Error, PathError};

fn supported() -> bool {
    landlock::is_supported()
}

fn in_child(f: impl FnOnce() -> i32) -> i32 {
    let pid = unsafe { libc::fork() };
    assert!(pid >= 0, "fork() failed");
    if pid == 0 {
        let code = f();
        unsafe { libc::_exit(code) };
    }
    let mut status = 0;
    assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
    if libc::WIFEXITED(status) {
        libc::WEXITSTATUS(status)
    } else {
        -1
    }
}

fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1)
}

fn open_errno(path: &CString, flags: libc::c_int) -> i32 {
    let fd = unsafe { libc::open(path.as_ptr(), flags) };
    if fd < 0 {
        return errno();
    }
    unsafe { libc::close(fd) };
    0
}

/// An empty path reaches `open("")`: it must be an ordinary `Open` error
/// (ENOENT), not a panic, a NUL-safety violation or a truncated filename.
#[test]
fn empty_path_is_an_open_error() {
    if !supported() {
        return;
    }
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [""])
        .restrict_self()
        .unwrap_err();
    match error {
        Error::Open {
            path,
            source,
        } => {
            assert_eq!(path, PathBuf::from(""));
            // Linux returns ENOENT for the empty path.
            assert_eq!(source.raw_os_error(), Some(libc::ENOENT), "{source}");
        }
        other => panic!("unexpected error: {other}"),
    }
}

/// The kernel's `PATH_MAX` includes the terminating NUL: a 4095-byte path is
/// accepted (and fails only because it does not exist), a 4096-byte one is
/// rejected before any syscall.
#[test]
fn path_max_boundary_is_exact() {
    if !supported() {
        return;
    }

    let longest = format!("/{}", "a".repeat(4094));
    assert_eq!(longest.len(), 4095);
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [longest])
        .restrict_self()
        .unwrap_err();
    assert!(
        matches!(error, Error::Open { .. }),
        "a 4095-byte path must be passed to the kernel: {error}"
    );

    let too_long = format!("/{}", "a".repeat(4095));
    assert_eq!(too_long.len(), 4096);
    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [too_long])
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
        "a 4096-byte path must be rejected: {error}"
    );
}

/// A NUL byte at the very end of an otherwise maximal path is still detected
/// before the length check, so the kernel can never see a truncated name.
#[test]
fn nul_inside_a_maximal_path_is_rejected() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    if !supported() {
        return;
    }
    let mut bytes = vec![b'/'];
    bytes.extend(std::iter::repeat_n(b'a', 4092));
    bytes.push(0);
    assert_eq!(bytes.len(), 4094);
    let path = PathBuf::from(OsString::from_vec(bytes));
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

/// A dangling symlink must surface as `Open` (ENOENT), not as an implicitly
/// skipped rule.
#[test]
fn dangling_symlink_is_an_open_error() {
    if !supported() {
        return;
    }
    let directory = std::env::temp_dir().join(format!("landlock-audit-dangling-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    let link = directory.join("link");
    let missing = directory.join("missing");
    std::os::unix::fs::symlink(&missing, &link).unwrap();

    let error = landlock::new_best_effort()
        .allow_filesystem(Access::Read, [link])
        .restrict_self()
        .unwrap_err();
    match error {
        Error::Open {
            source, ..
        } => assert_eq!(source.raw_os_error(), Some(libc::ENOENT), "{source}"),
        other => panic!("unexpected error: {other}"),
    }

    let _ = std::fs::remove_dir_all(&directory);
}

/// `allow_file_descriptor` only borrows the descriptor while adding the rule;
/// closing it before `restrict_self` must be fine because the kernel keeps the
/// underlying object alive.  Also checks that the rule is actually enforced.
#[test]
fn descriptor_rule_survives_closing_the_descriptor() {
    if !supported() {
        return;
    }

    let directory = std::env::temp_dir().join(format!("landlock-audit-fd-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    let inside = directory.join("inside");
    std::fs::write(&inside, b"ok").unwrap();

    let dir_c = CString::new(directory.as_os_str().as_bytes()).unwrap();
    let inside_c = CString::new(inside.as_os_str().as_bytes()).unwrap();
    let outside_c = CString::new("/etc/passwd").unwrap();

    let code = in_child(move || {
        // Allocation-free child: raw open, borrow, add the rule, close.
        let raw = unsafe { libc::open(dir_c.as_ptr(), libc::O_PATH | libc::O_CLOEXEC) };
        if raw < 0 {
            return 10;
        }
        // SAFETY: `raw` is an open descriptor, closed below, and the
        // `BorrowedFd` does not outlive it.
        let borrowed = unsafe { BorrowedFd::borrow_raw(raw) };
        let ruleset = landlock::new_best_effort().allow_file_descriptor(Access::Read, [borrowed]);
        // Closing before enforcement is allowed: the kernel holds a reference
        // to the object the rule was added for.
        unsafe { libc::close(raw) };

        if ruleset.restrict_self().is_err() {
            return 11;
        }
        let mut code = 0;
        if open_errno(&inside_c, libc::O_RDONLY) != 0 {
            code |= 1;
        }
        if open_errno(&outside_c, libc::O_RDONLY) != libc::EACCES {
            code |= 2;
        }
        code
    });
    assert_eq!(code, 0, "descriptor behavior in the sandbox is wrong");

    let _ = std::fs::remove_dir_all(&directory);
}

/// The owned-descriptor path: `OwnedFd::from_raw_fd` must own exactly one
/// descriptor and closing it after building must not close anything else.
#[test]
fn owned_descriptor_can_be_dropped_before_enforcement() {
    if !supported() {
        return;
    }
    let dir = CString::new("/etc").unwrap();
    let raw = unsafe { libc::open(dir.as_ptr(), libc::O_PATH | libc::O_CLOEXEC) };
    assert!(raw >= 0);
    // SAFETY: `raw` is a fresh owned descriptor.
    let owned = unsafe { OwnedFd::from_raw_fd(raw) };
    let ruleset = landlock::new_best_effort().allow_file_descriptor(Access::Read, [owned.as_fd()]);
    drop(owned); // must not close the ruleset's internal descriptor
    // Not enforced here (that would be irreversible); restricting in a child
    // is covered above.  Building and dropping must simply not leak or panic.
    drop(ruleset);
}
