use core::{
    cmp::Ordering,
    mem::{drop, forget, size_of},
    sync::atomic::Ordering::SeqCst,
};
use std::{
    io,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    panic::resume_unwind,
    sync::Mutex,
    thread,
};

use super::{Address, Instruction};

mod fd;
mod signal_safe;

#[cfg_attr(target_os = "linux", path = "ptrace_self_linux.rs")]
mod ptrace_self;

use fd::RawOwnedFd;
use ptrace_self::{AtomicState, STATE_COUNT, STATE_INIT, STATE_READY, STATE_STOP, TraceToken, trace};

static TRACE_MUTEX: Mutex<TraceToken> = Mutex::new(TraceToken);

/// Creates a pipe whose file descriptors are closed when the child of a
/// `fork()` calls `exec()`.
fn pipe_cloexec() -> io::Result<(OwnedFd, OwnedFd)> {
    let mut fds = [0 as libc::c_int; 2];
    // SAFETY: `fds` points to two writable `c_int`s.
    if unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `pipe2` succeeded, so both descriptors are valid and now owned by us.
    unsafe { Ok((OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1]))) }
}

/// Calls `f`, retrying while it fails with `EINTR`.
fn retry_on_intr<T>(mut f: impl FnMut() -> io::Result<T>) -> io::Result<T> {
    loop {
        match f() {
            Err(err) if err.kind() == io::ErrorKind::Interrupted => {}
            result => return result,
        }
    }
}

/// Reads up to `buf.len()` bytes, retrying on `EINTR`.
fn read(fd: &impl AsRawFd, buf: &mut [u8]) -> io::Result<usize> {
    retry_on_intr(|| {
        // SAFETY: `buf` points to `buf.len()` writable bytes.
        let ret = unsafe { libc::read(fd.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(ret as usize)
        }
    })
}

/// Writes up to `buf.len()` bytes, retrying on `EINTR`.
fn write(fd: &impl AsRawFd, buf: &[u8]) -> io::Result<usize> {
    retry_on_intr(|| {
        // SAFETY: `buf` points to `buf.len()` readable bytes.
        let ret = unsafe { libc::write(fd.as_raw_fd(), buf.as_ptr().cast(), buf.len()) };
        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(ret as usize)
        }
    })
}

/// Waits for the child process `pid` to exit, returning its wait status and
/// retrying on `EINTR`.
fn waitpid(pid: libc::pid_t) -> io::Result<libc::c_int> {
    let mut status = 0;
    retry_on_intr(|| {
        // SAFETY: `status` is a valid pointer to a `c_int`.
        let ret = unsafe { libc::waitpid(pid, &mut status, 0) };
        if ret == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(status)
        }
    })
}

/// Allows the process `pid` to trace this process, even when the kernel is
/// configured with a restrictive `kernel.yama.ptrace_scope`.
///
/// Returns `Ok(())` when the kernel does not implement `PR_SET_PTRACER`
/// (which fails with `EINVAL`): that happens on kernels without the Yama
/// security module, where ptrace is not restricted and there is nothing to
/// relax.
fn set_ptracer(pid: libc::pid_t) -> io::Result<()> {
    // SAFETY: `PR_SET_PTRACER` has no preconditions; its argument is an
    // `unsigned long` holding the tracer's pid.
    let ret = unsafe {
        libc::prctl(
            libc::PR_SET_PTRACER,
            pid as libc::c_ulong,
            0 as libc::c_ulong,
            0 as libc::c_ulong,
            0 as libc::c_ulong,
        )
    };
    if ret == -1 {
        let err = io::Error::last_os_error();
        if err.raw_os_error() != Some(libc::EINVAL) {
            return Err(err);
        }
    }
    Ok(())
}

pub fn count_instructions(f: &mut dyn FnMut(), counter: &mut (dyn FnMut(&Instruction) + Send)) -> std::io::Result<()> {
    let state = &AtomicState::new(STATE_INIT);
    let write_state = |data| state.store(data, SeqCst);

    let (control_read, control_write) = pipe_cloexec()?;
    let (data_read, data_write) = pipe_cloexec()?;
    let (ready_read, ready_write) = pipe_cloexec()?;

    // These file descriptors will be used from the child of the fork(),
    // which must use only async-signal-safe functions, but `OwnedFd`
    // might use functions which are not async-signal-safe.
    let control_read = RawOwnedFd::from(control_read);
    let control_write = RawOwnedFd::from(control_write);
    let data_read = RawOwnedFd::from(data_read);
    let data_write = RawOwnedFd::from(data_write);
    let ready_read = RawOwnedFd::from(ready_read);
    let ready_write = RawOwnedFd::from(ready_write);

    // The read end of the control pipe is referenced by both threads.
    // Borrow it here, to ensure it can be dropped only after the scope.
    let control_read = &control_read;

    thread::scope(|s| {
        let traced_pid = getpid();
        let traced_tid = gettid();
        assert!(traced_pid > 0 && traced_tid > 0);

        let helper = thread::Builder::new().spawn_scoped(s, move || {
            let mut guard = TRACE_MUTEX.lock().unwrap();

            let pid = {
                // SAFETY: the child will only use async-signal-safe functions from libc
                // or raw system calls.
                let pid = unsafe { libc::fork() };
                match pid.cmp(&0) {
                    Ordering::Equal => {
                        // This block runs in a forked child of a multithreaded program, so only
                        // async-signal-safe functions from libc or raw system calls can be used here.

                        // Close the unused end of the pipes.
                        // SAFETY: `control_read` will only be closed in the parent process
                        // after the `thread::scope` ends, which means it's still open here.
                        // SAFETY: nothing else in this child acts on this file descriptor.
                        let _ = unsafe { libc::close(control_read.as_raw_fd()) };
                        drop(data_read);
                        drop(ready_write);

                        // SAFETY: the state variable outlives the thread scope, and the helper
                        // thread within that scope always waits for this child process to exit.
                        // SAFETY: the tracer will only read it while the thread is in a stopped state.
                        unsafe {
                            libc::_exit(
                                match trace(
                                    traced_pid,
                                    traced_tid,
                                    state.as_ptr().cast_const(),
                                    control_write,
                                    data_write,
                                    ready_read,
                                    &mut guard,
                                ) {
                                    Ok(()) => libc::EXIT_SUCCESS,
                                    Err(_) => libc::EXIT_FAILURE,
                                },
                            );
                        }
                    }
                    Ordering::Greater => PidGuard(pid),
                    Ordering::Less => return Err(std::io::Error::last_os_error()),
                }
            };

            // Close the unused end of the pipes.
            drop(control_write);
            drop(data_write);
            drop(ready_read);

            // Necessary when "restricted ptrace" mode is enabled.
            set_ptracer(pid.get())?;
            write(&ready_write, &[0])?;
            drop(ready_write);

            // Wait for child PTRACE_SEIZE call before releasing the mutex.
            let mut buf = [0; size_of::<Address>()];
            let size = read(&data_read, &mut buf)?;
            assert_eq!(size, buf.len());
            if cfg!(target_os = "linux") {
                // On Linux, multiple threads can be traced independently,
                // so the mutex can be released early.
                drop(guard);
            }

            loop {
                let size = read(&data_read, &mut buf)?;
                if size == 0 {
                    break;
                }
                assert_eq!(size, buf.len());

                let address = Address::from_ne_bytes(buf);
                let instruction = Instruction::new(address);
                counter(&instruction);
            }
            drop(data_read);

            let status = pid.wait()?;
            if libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0 {
                Ok(())
            } else {
                Err(std::io::Error::other("error in tracer"))
            }
        })?;

        write_state(STATE_READY);
        read(control_read, &mut [0])?;

        write_state(STATE_COUNT);
        f();
        write_state(STATE_STOP);

        match helper.join() {
            Ok(result) => result,
            Err(e) => resume_unwind(e),
        }
    })
}

struct PidGuard(libc::pid_t);

impl PidGuard {
    #[inline]
    fn get(&self) -> libc::pid_t {
        self.0
    }

    fn wait(self) -> io::Result<libc::c_int> {
        let result = waitpid(self.0);
        forget(self);
        result
    }
}

impl Drop for PidGuard {
    fn drop(&mut self) {
        // SAFETY: `kill` and `waitpid` have no preconditions.
        let _ = unsafe { libc::kill(self.0, libc::SIGTERM) };
        let _ = waitpid(self.0);
    }
}

fn getpid() -> libc::pid_t {
    // SAFETY: getpid() is safe
    unsafe { libc::getpid() }
}

fn gettid() -> libc::pid_t {
    // SAFETY: gettid() is safe
    unsafe { libc::gettid() }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        #[inline(never)]
        fn add(left: usize, right: usize) -> usize {
            left + right
        }

        #[inline(never)]
        fn count() -> std::vec::Vec<Address> {
            let mut addresses = std::vec::Vec::new();
            let result = count_instructions(|| add(2, 2), |instruction| addresses.push(instruction.address())).unwrap();
            assert_eq!(result, 4);
            assert!(!addresses.is_empty());
            addresses
        }

        #[inline(never)]
        fn count_other() -> std::vec::Vec<Address> {
            let mut addresses = std::vec::Vec::new();
            count_instructions(|| (), |instruction| addresses.push(instruction.address())).unwrap();
            assert!(!addresses.is_empty());
            addresses
        }

        let first = count();
        let second = count();
        let third = count_other();
        assert_eq!(first, second);
        assert_ne!(first, third);
    }
}
