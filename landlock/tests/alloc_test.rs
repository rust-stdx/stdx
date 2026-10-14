//! The policy fast path must not allocate: check it with `alloc_counter`.
//!
//! Only the calling thread is measured, and only when Landlock is available.

#![cfg(target_os = "linux")]

use std::{
    ffi::CString,
    os::fd::{AsFd, FromRawFd, OwnedFd},
};

use alloc_counter::measure;
use landlock::{Access, NetworkAccess};

#[test]
fn building_a_policy_does_not_allocate() {
    if !landlock::is_supported() {
        return;
    }

    // Everything that allocates happens outside the measured region.
    let directory = CString::new("/etc").unwrap();
    let raw = unsafe { libc::open(directory.as_ptr(), libc::O_PATH | libc::O_CLOEXEC) };
    assert!(raw >= 0, "cannot open /etc");
    let owned = unsafe { OwnedFd::from_raw_fd(raw) };
    let fd = owned.as_fd();

    // Warm up the per-thread counters so their initialisation is not measured.
    let _ = measure(|| ());

    let (abi, counters) = measure(|| {
        let ruleset = landlock::new_best_effort()
            .allow_filesystem(Access::Read, ["/etc", "/usr"])
            .allow_file_descriptor(Access::Read, [fd])
            .allow_network(NetworkAccess::BindTcp, [8080])
            .allow_filesystem(Access::ReadWrite, ["/"]);
        drop(ruleset);
        landlock::abi_version().unwrap()
    });

    assert!(abi >= 1);
    assert_eq!(counters.calls, 0, "building a policy allocated {} times", counters.calls);
    assert_eq!(counters.bytes, 0);
}

#[test]
fn enforcing_a_policy_does_not_allocate() {
    if !landlock::is_supported() {
        return;
    }

    // Enforcing a policy is irreversible: do it in a child process.
    let pid = unsafe { libc::fork() };
    assert!(pid >= 0, "fork() failed");
    if pid == 0 {
        let _ = measure(|| ());
        let (result, counters) = measure(|| {
            landlock::new_best_effort()
                .allow_filesystem(Access::Read, ["/etc"])
                .restrict_self()
        });
        let success = result.is_ok() && counters.calls == 0 && counters.bytes == 0;
        unsafe { libc::_exit(i32::from(!success)) };
    }

    let mut status = 0;
    let waited = unsafe { libc::waitpid(pid, &mut status, 0) };
    assert_eq!(waited, pid);
    let code = if libc::WIFEXITED(status) {
        libc::WEXITSTATUS(status)
    } else {
        -1
    };
    assert_eq!(code, 0, "enforcing a policy must not allocate");
}
