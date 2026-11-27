//! Audit tests for `term`.
//!
//! The unsafe surface is `tcgetattr`/`tcsetattr` plus a zeroed `libc::termios`.
//! These tests replace fd 0 with pipes and pseudo-terminals (in-process, guarded
//! by a mutex) to exercise: non-tty input, EOF, invalid UTF-8, CR handling, and
//! the RAII restoration of terminal flags on both success and error paths.

#![cfg(unix)]

use std::{
    io::Write,
    os::fd::{FromRawFd, RawFd},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

static FD_LOCK: Mutex<()> = Mutex::new(());

/// Restores fd 0 on drop, even if the test panics.
struct RestoreStdout(RawFd);

impl Drop for RestoreStdout {
    fn drop(&mut self) {
        unsafe {
            libc::dup2(self.0, 0);
            libc::close(self.0);
        }
    }
}

fn save_stdin() -> RestoreStdout {
    let saved = unsafe { libc::dup(0) };
    assert!(saved >= 0, "dup(0) failed: {}", std::io::Error::last_os_error());
    RestoreStdout(saved)
}

fn dup2_stdin(fd: RawFd) {
    let rc = unsafe { libc::dup2(fd, 0) };
    assert_eq!(rc, 0, "dup2 failed: {}", std::io::Error::last_os_error());
}

fn pipe() -> (RawFd, RawFd) {
    let mut fds = [0i32; 2];
    let rc = unsafe { libc::pipe(fds.as_mut_ptr()) };
    assert_eq!(rc, 0, "pipe failed");
    (fds[0], fds[1])
}

/// Install `bytes` as stdin through a pipe. The write end is closed so EOF is
/// observable. The read end is duplicated onto fd 0 and then closed.
fn stdin_from_bytes(bytes: &[u8]) {
    let (rd, wr) = pipe();
    let mut file = unsafe { std::fs::File::from_raw_fd(wr) };
    file.write_all(bytes).unwrap();
    drop(file); // close write end => EOF after the bytes
    dup2_stdin(rd);
    unsafe { libc::close(rd) };
}

fn stdin_from_pty() -> (RawFd, RawFd) {
    let mut master = -1;
    let mut slave = -1;
    let rc = unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null_mut::<libc::termios>(),
            std::ptr::null_mut::<libc::winsize>(),
        )
    };
    assert_eq!(rc, 0, "openpty failed: {}", std::io::Error::last_os_error());
    dup2_stdin(slave);
    (master, slave)
}

fn termios(fd: RawFd) -> libc::termios {
    let mut t: libc::termios = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::tcgetattr(fd, &mut t) };
    assert_eq!(rc, 0, "tcgetattr failed: {}", std::io::Error::last_os_error());
    t
}

#[test]
fn pipe_input_matrix() {
    let _lock = FD_LOCK.lock().unwrap();

    // read_line: ordinary LF, CRLF, lone CR, no newline at EOF.
    {
        let _r = save_stdin();
        stdin_from_bytes(b"hello\n");
        assert_eq!(term::read_line().unwrap(), "hello");
    }
    {
        let _r = save_stdin();
        // The implementation stops at CR, so the LF of a CRLF pair remains
        // buffered; the next call observes the empty remainder. Documented
        // here (also exercised by the CR case below); it is not unsafe.
        stdin_from_bytes(b"hello\r\n");
        assert_eq!(term::read_line().unwrap(), "hello");
        assert_eq!(term::read_line().unwrap(), "");
    }
    {
        let _r = save_stdin();
        // A CR terminates the line; the remainder is left for the next call.
        // This documents the (non-Go-like) interior-CR behaviour.
        stdin_from_bytes(b"a\rb\n");
        assert_eq!(term::read_line().unwrap(), "a");
        assert_eq!(term::read_line().unwrap(), "b");
    }
    {
        let _r = save_stdin();
        stdin_from_bytes(b"no newline");
        assert_eq!(term::read_line().unwrap(), "no newline");
    }

    // EOF yields an empty string, not an error.
    {
        let _r = save_stdin();
        stdin_from_bytes(b"");
        assert_eq!(term::read_line().unwrap(), "");
    }
    {
        let _r = save_stdin();
        stdin_from_bytes(b"");
        assert_eq!(term::read_password().unwrap(), "");
    }

    // Invalid UTF-8 is an InvalidData I/O error, on both entry points.
    for bytes in [&b"\xff\n"[..], &b"ab\x80cd\n"[..], &b"\xc3\n"[..]] {
        let _r = save_stdin();
        stdin_from_bytes(bytes);
        let err = term::read_line().unwrap_err();
        match err {
            term::Error::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData, "{bytes:?}"),
        }
    }
    {
        let _r = save_stdin();
        stdin_from_bytes(b"\xfe\xff\n");
        let err = term::read_password().unwrap_err();
        match err {
            term::Error::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData),
        }
    }

    // read_password on a non-tty still reads the line and must not try to
    // change terminal attributes (there are none to change).
    {
        let _r = save_stdin();
        stdin_from_bytes(b"secret value\n");
        assert_eq!(term::read_password().unwrap(), "secret value");
    }

    // Embedded NUL and interior/trailing whitespace are passed through.
    {
        let _r = save_stdin();
        stdin_from_bytes(b"a\0b \n");
        assert_eq!(term::read_line().unwrap(), "a\0b ");
    }
}

fn spin_until_echo_off(slave: RawFd, max: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < max {
        let t = unsafe {
            let mut t: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(slave, &mut t) == 0 {
                Some(t)
            } else {
                None
            }
        };
        if let Some(t) = t
            && t.c_lflag & (libc::ECHO as libc::tcflag_t) == 0
        {
            return true;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    false
}

fn write_master(master: RawFd, bytes: &[u8]) {
    let mut f = unsafe { std::fs::File::from_raw_fd(libc::dup(master)) };
    f.write_all(bytes).unwrap();
    // File drop closes the duplicate, not the master.
}

#[test]
fn pty_disables_and_restores_echo() {
    let _lock = FD_LOCK.lock().unwrap();
    let _r = save_stdin();
    let (master, slave) = stdin_from_pty();

    let before = termios(slave);
    assert_ne!(before.c_lflag & (libc::ECHO as libc::tcflag_t), 0, "pty should start with echo");

    let saw_echo_off = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&saw_echo_off);
    let writer = std::thread::spawn(move || {
        let off = spin_until_echo_off(slave, Duration::from_secs(3));
        flag.store(off, Ordering::SeqCst);
        write_master(master, b"hunter2\n");
    });

    let got = term::read_password().unwrap();
    writer.join().unwrap();
    assert_eq!(got, "hunter2");
    assert!(saw_echo_off.load(Ordering::SeqCst), "read_password never disabled ECHO");

    // On the success path the guard must restore the original flags.
    let after = termios(slave);
    assert_eq!(
        after.c_lflag & (libc::ECHO as libc::tcflag_t),
        before.c_lflag & (libc::ECHO as libc::tcflag_t),
        "ECHO not restored"
    );

    // If echo had been left on, the typed password would be queued on the
    // master side. Drain non-blocking and check it is absent.
    let flags = unsafe { libc::fcntl(master, libc::F_GETFL) };
    unsafe { libc::fcntl(master, libc::F_SETFL, flags | libc::O_NONBLOCK) };
    let mut echoed = Vec::new();
    loop {
        let mut buf = [0u8; 256];
        let n = unsafe { libc::read(master, buf.as_mut_ptr().cast(), buf.len()) };
        if n <= 0 {
            break;
        }
        echoed.extend_from_slice(&buf[..n as usize]);
    }
    assert!(
        !echoed.windows(7).any(|w| w == b"hunter2"),
        "password was echoed to the terminal: {echoed:?}"
    );

    unsafe {
        libc::close(master);
        libc::close(slave);
    }
}

#[test]
fn pty_restores_echo_on_error_path() {
    let _lock = FD_LOCK.lock().unwrap();
    let _r = save_stdin();
    let (master, slave) = stdin_from_pty();

    let before = termios(slave);
    let writer = std::thread::spawn(move || {
        // Wait until the guard has applied its change.
        spin_until_echo_off(slave, Duration::from_secs(3));
        write_master(master, b"\xff\n");
    });

    let err = term::read_password().unwrap_err();
    writer.join().unwrap();
    match err {
        term::Error::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData),
    }

    let after = termios(slave);
    assert_eq!(
        after.c_lflag & (libc::ECHO as libc::tcflag_t),
        before.c_lflag & (libc::ECHO as libc::tcflag_t),
        "ECHO not restored after an error return"
    );

    unsafe {
        libc::close(master);
        libc::close(slave);
    }
}
