use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};

use libc::close;

/// Similar to `OwnedFd`, but guaranteed to call only async-signal-safe libc
/// functions, except on `AsFd::as_fd()` which goes through `BorrowedFd`.
#[repr(transparent)]
pub struct RawOwnedFd {
    fd: RawFd,
}

impl Drop for RawOwnedFd {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: fd is a valid open file descriptor
        let _ = unsafe { close(self.fd) };
    }
}

impl From<OwnedFd> for RawOwnedFd {
    #[inline]
    fn from(fd: OwnedFd) -> RawOwnedFd {
        RawOwnedFd {
            fd: fd.into_raw_fd(),
        }
    }
}

impl AsFd for RawOwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // SAFETY: `RawOwnedFd` has the same validity invariants as `OwnedFd`,
        // and this is identical to the implementation for `OwnedFd`.
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

impl AsRawFd for RawOwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl FromRawFd for RawOwnedFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        RawOwnedFd {
            fd,
        }
    }
}

impl IntoRawFd for RawOwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.fd
    }
}
