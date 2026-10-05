//! Cryptographically secure random number source for rustls.
//!
//! Backed by [`crypto::random`], which reads from the operating system's
//! entropy source.

use rustls::crypto::{GetRandomFailed, SecureRandom};

/// The [`SecureRandom`] implementation used by this provider.
#[derive(Debug)]
pub(crate) struct RngProvider;

impl SecureRandom for RngProvider {
    fn fill(&self, buf: &mut [u8]) -> Result<(), GetRandomFailed> {
        crypto::random::fill(buf).map_err(|_| GetRandomFailed)
    }
}
