use std::ffi::OsString;

/// Get the standard host name for the current machine using libc (POSIX `gethostname`).
/// [gethostname]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
/// [sysconf]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/sysconf.html
pub fn hostname() -> OsString {
    use std::os::unix::ffi::OsStringExt;

    use libc::{_SC_HOST_NAME_MAX, c_char, sysconf};

    // Get the maximum size of host names on this system, and account for the
    // trailing NUL byte. `sysconf` returns -1 on error; fall back to the
    // traditional 255-byte limit instead of casting -1 to a huge `usize` (which
    // would overflow on `+ 1` and abort in overflow-checks builds).
    let hostname_max = unsafe { sysconf(_SC_HOST_NAME_MAX) };
    let hostname_max = usize::try_from(hostname_max).unwrap_or(255).min(64 * 1024);
    let mut buffer = vec![0; hostname_max.saturating_add(1)];
    let returncode = unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) };
    if returncode != 0 {
        // There are no reasonable failures, so lets panic
        panic!("gethostname failed: {}", std::io::Error::last_os_error());
    }

    // We explicitly search for the trailing NUL byte and cap at the buffer
    // length: If the buffer's too small (which shouldn't happen since we
    // explicitly use the max hostname size above but just in case) POSIX
    // doesn't specify whether there's a NUL byte at the end, so if we didn't
    // check we might read from memory that's not ours.
    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    buffer.truncate(end);
    OsString::from_vec(buffer)
}
