//! Hostname and host/port parsing edge cases.

use std::os::unix::ffi::OsStrExt;

#[test]
fn hostname_is_a_bounded_well_formed_os_string() {
    let name = net::hostname();
    let bytes = name.as_os_str().as_bytes();
    assert!(!bytes.is_empty(), "the container should have a hostname");
    assert!(!bytes.contains(&0), "the result must not contain a NUL byte");
    assert!(bytes.len() <= 64 * 1024, "the result is capped at 64 KiB");
    // Hostnames on Linux are limited to 64 bytes; the fallback buffer is 255.
    assert!(bytes.len() <= 255, "unexpectedly long hostname: {bytes:?}");
    assert_eq!(name, net::hostname(), "hostname() must be deterministic");
}

#[test]
fn hostname_is_thread_safe() {
    let expected = net::hostname();
    let handles: Vec<_> = (0..4).map(|_| std::thread::spawn(net::hostname)).collect();
    for handle in handles {
        assert_eq!(handle.join().unwrap(), expected);
    }
}

/// Numeric ports not covered by the in-crate `test_split_host_port` table.
/// The documented examples themselves are already asserted there, so this
/// only pins the boundary values.
#[test]
fn split_host_port_numeric_port_boundaries() {
    let cases = [("host:0", ("host", "0")), ("host:65535", ("host", "65535"))];
    for (input, expected) in cases {
        assert_eq!(net::split_host_port(input), expected, "input: {input:?}");
    }
}

/// Malformed inputs must not panic (the function returns borrowed slices and
/// must never underflow `len() - 1` or slice on a non-char boundary).
#[test]
fn split_host_port_never_panics() {
    let inputs = [
        ":",
        "::",
        ":::",
        "[",
        "]",
        "[]",
        "[]:",
        "[]:80",
        "[::1]:",
        "[::1]::80",
        "[::1]:80:90",
        "a:b:c",
        ":",
        "host::",
        "::1",
        "1:2",
        "héllo:80",
        "é",
        "🦀:443",
        "[::1",
        "::1]",
    ];
    for input in inputs {
        let (host, port) = net::split_host_port(input);
        // The returned slices must be sub-slices of the input.
        assert!(input.as_ptr() <= host.as_ptr());
        assert!(host.len() + port.len() <= input.len());
    }
}

/// Current behavior for inputs with brackets but no port keeps the brackets in
/// the host.  This is arguably inconsistent with `"[::1]"`/`"[::1]:80"`, but it
/// is not memory-unsafe; the test documents it.
#[test]
fn split_host_port_bracketed_empty_port_current_behavior() {
    assert_eq!(net::split_host_port("[::1]:"), ("[::1]", ""));
    assert_eq!(net::split_host_port("[]:"), ("[]", ""));
}
