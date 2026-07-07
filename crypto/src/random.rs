/// Returns an array filled with cryptographically-random data.
#[inline]
pub fn random_bytes<const N: usize>() -> [u8; N] {
    let mut buf = [0u8; N];
    getrandom::fill(&mut buf).expect("getrandom failed");
    buf
}

/// Fill the given buffer with cryptographically-random data.
#[inline]
pub fn random_fill(buf: &mut [u8]) {
    getrandom::fill(buf).expect("getrandom failed");
}
