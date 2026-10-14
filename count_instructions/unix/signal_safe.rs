// Code in this file should be treated as #![no_std]; it should only use core and libc,
// and must only use async-signal-safe functions or raw system calls.

use std::os::fd::AsRawFd;

#[allow(clippy::wildcard_imports)]
use libc::*;

use super::{Address, RawOwnedFd};

#[inline]
pub fn errno() -> c_int {
    // SAFETY: reading errno is async-signal-safe
    // SAFETY: __errno_location() always points to this thread's errno
    unsafe { *__errno_location() }
}

#[allow(clippy::cast_sign_loss)]
fn retry_pipe_on_intr<F>(mut f: F, len: usize) -> Result<(), c_int>
where
    F: FnMut() -> ssize_t,
{
    loop {
        let ret = f();
        if ret < 0 {
            let err = errno();
            if err != EINTR {
                break Err(err);
            }
        } else if ret as usize == len {
            break Ok(());
        } else {
            break Err(EPIPE);
        }
    }
}

/// Waits for and reads one byte on the ready pipe, and closes it.
#[allow(clippy::needless_pass_by_value)]
pub fn wait_for_ready(ready_fd: RawOwnedFd) -> Result<(), c_int> {
    let mut buf: [u8; 1] = [0];
    let len = buf.len();
    retry_pipe_on_intr(
        // SAFETY: `read()` is async-signal-safe
        // SAFETY: the `fd` is a valid open file descriptor
        // SAFETY: the buffer is mutable and has `len` bytes
        || unsafe { read(ready_fd.as_raw_fd(), (&raw mut buf).cast(), len) },
        len,
    )
}

/// Writes one byte to the control pipe, and closes it.
#[allow(clippy::needless_pass_by_value)]
pub fn write_control(control_fd: RawOwnedFd) -> Result<(), c_int> {
    let buf: [u8; 1] = [0];
    let len = buf.len();
    retry_pipe_on_intr(
        // SAFETY: `write()` is async-signal-safe
        // SAFETY: the `fd` is a valid open file descriptor
        // SAFETY: the buffer is readable and has `len` bytes
        || unsafe { write(control_fd.as_raw_fd(), (&raw const buf).cast(), len) },
        len,
    )
}

/// Writes one address to the data pipe.
pub fn write_data(data_fd: &RawOwnedFd, address: Address) -> Result<(), c_int> {
    let buf = address.to_ne_bytes();
    let len = buf.len();
    retry_pipe_on_intr(
        // SAFETY: `write()` is async-signal-safe
        // SAFETY: the `fd` is a valid open file descriptor
        // SAFETY: the buffer is readable and has `len` bytes
        || unsafe { write(data_fd.as_raw_fd(), (&raw const buf).cast(), len) },
        len,
    )
}
