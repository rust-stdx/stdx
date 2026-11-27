//! Regression and edge-case tests for the instruction tracer.
//!
//! The tracer forks a helper that `ptrace`s the calling thread. A bug in the
//! stop state machine kills the whole test binary with `SIGTRAP`, and a bug in
//! the panic paths hangs it. To get a *failing* test instead of a dead or
//! hanging harness, every risky scenario runs in a re-executed child of the
//! test binary with a timeout; the parent only inspects the exit status.
//!
//! `count_instructions` needs Linux; on other platforms the whole file is
//! compiled out.

#![cfg(target_os = "linux")]

use std::{
    os::unix::process::ExitStatusExt,
    panic::catch_unwind,
    process::{Child, Command, ExitStatus, Stdio},
    time::{Duration, Instant},
};

/// The parent-visible outcome of a scenario executed in a child process.
fn run_isolated(case: &str, timeout: Duration) -> std::process::Output {
    let exe = std::env::current_exe().expect("current_exe");
    let mut child: Child = Command::new(exe)
        .args(["--exact", &format!("child_{case}"), "--test-threads=1"])
        .env("AUDIT_CI_CHILD", case)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("cannot re-execute the test binary");

    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait().expect("try_wait") {
            Some(_) => return child.wait_with_output().expect("wait_with_output"),
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("scenario `{case}` did not finish within {timeout:?} (hang)");
            }
            None => std::thread::sleep(Duration::from_millis(5)),
        }
    }
}

fn assert_clean(case: &str, output: std::process::Output) {
    let status: ExitStatus = output.status;
    assert_eq!(
        status.signal(),
        None,
        "scenario `{case}` died from signal {:?} (status {status:?}, stderr: {})",
        status.signal(),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(
        status.success(),
        "scenario `{case}` failed: {status:?}, stderr: {}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("1 passed"),
        "scenario `{case}` did not actually run (stdout: {stdout:?})"
    );
}

fn is_child(case: &str) -> bool {
    std::env::var_os("AUDIT_CI_CHILD").is_some_and(|v| v == case)
}

#[inline(never)]
fn pure(a: u64, b: u64) -> u64 {
    std::hint::black_box(a.wrapping_mul(b).wrapping_add(3))
}

#[inline(never)]
fn traced_getpid() -> u64 {
    // SAFETY: `getpid` takes no arguments and cannot fail.
    unsafe { libc::syscall(libc::SYS_getpid) as u64 }
}

// ---------------------------------------------------------------------------
// In-process checks (they must not kill the process; the bug would)
// ---------------------------------------------------------------------------

/// The crate compares instruction streams only for one monomorphisation, so
/// both measured calls go through this single (non-inlined) function; using
/// two different counter closures would instantiate the wrapper twice.
#[inline(never)]
fn trace_into_addresses(out: &mut Vec<usize>) -> u64 {
    count_instructions::count_instructions(traced_getpid, |i| out.push(i.address())).unwrap()
}

#[test]
fn addresses_are_stable_and_nonzero_across_calls() {
    // Warm up lazy PLT resolution of `syscall`.
    trace_into_addresses(&mut Vec::new());

    let mut first = Vec::new();
    let value = trace_into_addresses(&mut first);
    assert!(value > 0);
    assert!(!first.is_empty());
    assert!(first.iter().all(|&a| a != 0), "instruction address must not be 0");

    let mut second = Vec::new();
    trace_into_addresses(&mut second);
    assert_eq!(first, second, "the same code must produce the same addresses");
}

#[test]
fn instruction_count_is_positive() {
    let mut count = 0usize;
    let value = count_instructions::count_instructions(|| pure(6, 7), |_| count += 1).unwrap();
    assert_eq!(value, 45);
    assert!(count > 0);
}

// ---------------------------------------------------------------------------
// Child scenarios: ordinary syscalls
// ---------------------------------------------------------------------------

#[test]
fn child_syscalls() {
    if !is_child("syscalls") {
        return;
    }

    // Raw syscalls, including ones that block or return data.
    let pid = count_instructions::count_instructions(|| unsafe { libc::syscall(libc::SYS_getpid) as u64 }, |_| {})
        .expect("getpid trace");
    assert_eq!(pid, std::process::id() as u64);

    let ppid = count_instructions::count_instructions(|| unsafe { libc::syscall(libc::SYS_getppid) as u64 }, |_| {})
        .expect("getppid trace");
    assert!(ppid > 0);

    let tid = count_instructions::count_instructions(|| unsafe { libc::syscall(libc::SYS_gettid) as u64 }, |_| {})
        .expect("gettid trace");
    assert!(tid > 0);

    // A zero-length write still crosses the syscall boundary.
    count_instructions::count_instructions(
        || unsafe { libc::syscall(libc::SYS_write, 2, b"".as_ptr(), 0usize) as u64 },
        |_| {},
    )
    .expect("write trace");

    // sched_yield(), sleep and file metadata/open/read/getcwd all syscall.
    count_instructions::count_instructions(|| unsafe { libc::sched_yield() }, |_| {}).expect("sched_yield trace");
    count_instructions::count_instructions(|| std::thread::sleep(Duration::from_micros(10)), |_| {})
        .expect("sleep trace");
    count_instructions::count_instructions(|| std::fs::metadata("/").map(|m| m.is_dir()), |_| {})
        .expect("metadata trace")
        .expect("metadata");
    count_instructions::count_instructions(|| std::fs::read("/etc/hostname"), |_| {})
        .expect("open/read trace")
        .expect("open/read");
    count_instructions::count_instructions(|| std::env::current_dir().map(|p| p.as_os_str().len()), |_| {})
        .expect("getcwd trace")
        .expect("getcwd");
}

#[test]
fn child_spawned_thread() {
    if !is_child("spawned_thread") {
        return;
    }
    let value =
        count_instructions::count_instructions(|| std::thread::spawn(|| pure(20, 21)).join().expect("join"), |_| {})
            .expect("thread spawn trace");
    assert_eq!(value, 423);
}

#[test]
fn child_panic_caught_inside() {
    if !is_child("panic_caught_inside") {
        return;
    }
    // `catch_unwind` inside `f` still runs the panic hook and writes to stderr.
    let caught = count_instructions::count_instructions(
        || catch_unwind(|| -> u64 { panic!("caught inside the traced closure") }).is_err(),
        |_| {},
    )
    .expect("trace with an inner catch_unwind");
    assert!(caught);
}

// ---------------------------------------------------------------------------
// Child scenarios: panics must propagate, not kill or hang
// ---------------------------------------------------------------------------

#[test]
fn child_panic_in_closure() {
    if !is_child("panic_in_closure") {
        return;
    }
    let result = catch_unwind(|| count_instructions::count_instructions(|| -> u64 { panic!("boom in f") }, |_| {}));
    assert!(result.is_err(), "the panic must reach the caller");
    // Reaching this line proves the tracer released the thread and the scope
    // joined; the process must still be able to reach the end of the child.
}

#[test]
fn child_panic_in_counter() {
    if !is_child("panic_in_counter") {
        return;
    }
    let result = catch_unwind(|| count_instructions::count_instructions(|| pure(3, 4), |_| panic!("boom in counter")));
    assert!(result.is_err(), "the counter panic must reach the caller");

    // The traced closure keeps running after the counter panicked, including
    // syscalls; the tracer must keep draining addresses so the tracee survives.
    let result = catch_unwind(|| {
        count_instructions::count_instructions(
            || {
                std::fs::metadata("/").map(|m| m.is_dir()).unwrap_or(false);
                std::thread::sleep(Duration::from_micros(10));
                pure(3, 4)
            },
            |_| panic!("boom in counter with a syscall-heavy closure"),
        )
    });
    assert!(result.is_err(), "the counter panic must reach the caller");
}

#[test]
fn child_nested() {
    if !is_child("nested") {
        return;
    }
    // A thread can only be traced by one tracer, so the inner call cannot
    // succeed. Whatever it does, it must not kill the process or hang.
    let result = catch_unwind(|| {
        count_instructions::count_instructions(|| count_instructions::count_instructions(|| pure(5, 6), |_| {}), |_| {})
    });
    assert!(
        !matches!(result, Ok(Ok(Ok(_)))),
        "nested tracing unexpectedly succeeded: {result:?}"
    );
}

#[test]
fn child_repeated() {
    if !is_child("repeated") {
        return;
    }
    let mut values = Vec::new();
    for _ in 0..4 {
        values.push(count_instructions::count_instructions(traced_getpid, |_| {}).expect("trace"));
    }
    assert!(values.windows(2).all(|w| w[0] == w[1]), "pid changed: {values:?}");
    assert_eq!(values[0], std::process::id() as u64);
}

#[test]
fn child_genuine_sigtrap() {
    if !is_child("genuine_sigtrap") {
        return;
    }
    // Control: the pseudo-step suppression must not swallow a real SIGTRAP.
    // `raise` produces SI_TKILL, not the kernel pseudo-step (SI_USER, pid 0).
    let _ = count_instructions::count_instructions(
        || {
            // SAFETY: `raise` is a plain libc call; the default action for
            // SIGTRAP terminates this process, which the parent asserts.
            unsafe { libc::raise(libc::SIGTRAP) };
            pure(1, 1)
        },
        |_| {},
    );
    panic!("a genuine SIGTRAP must not be swallowed");
}

#[test]
fn genuine_sigtrap_kills_the_child() {
    let output = run_isolated("genuine_sigtrap", Duration::from_secs(20));
    assert_eq!(
        output.status.signal(),
        Some(libc::SIGTRAP),
        "a real SIGTRAP must still terminate the tracee: {:?}",
        output.status
    );
}

// ---------------------------------------------------------------------------
// Parent drivers
// ---------------------------------------------------------------------------

macro_rules! isolated {
    ($name:ident, $case:literal) => {
        #[test]
        fn $name() {
            let output = run_isolated($case, Duration::from_secs(30));
            assert_clean($case, output);
        }
    };
}

isolated!(syscalls_do_not_raise_sigtrap, "syscalls");
isolated!(spawned_threads_do_not_raise_sigtrap, "spawned_thread");
isolated!(panics_inside_the_closure_are_traced, "panic_caught_inside");
isolated!(panic_in_the_traced_closure_propagates, "panic_in_closure");
isolated!(panic_in_the_counter_propagates, "panic_in_counter");
isolated!(nested_tracing_does_not_kill_the_process, "nested");
isolated!(repeated_calls_are_exact, "repeated");
