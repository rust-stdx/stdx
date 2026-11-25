use crate::RandomError;

/// Returns an array filled with cryptographically-random data.
///
/// Returns [`RandomError`] when the operating system's random number
/// generator is unavailable or fails.
#[inline]
pub(crate) fn bytes<const N: usize>() -> Result<[u8; N], RandomError> {
    let mut buf = [0u8; N];
    getrandom::fill(&mut buf)?;
    Ok(buf)
}
