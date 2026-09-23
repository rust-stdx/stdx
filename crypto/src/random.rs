use crate::RandomError;

/// Returns an array filled with cryptographically-random data.
///
/// Returns [`RandomError`] when the operating system's random number
/// generator is unavailable or fails.
#[inline]
pub fn bytes<const N: usize>() -> Result<[u8; N], RandomError> {
    let mut buf = [0u8; N];
    getrandom::fill(&mut buf)?;
    Ok(buf)
}

/// Fills the given buffer with cryptographically-random data.
///
/// Returns [`RandomError`] when the operating system's random number
/// generator is unavailable or fails.
#[inline]
pub fn fill(buf: &mut [u8]) -> Result<(), RandomError> {
    getrandom::fill(buf)?;
    Ok(())
}
