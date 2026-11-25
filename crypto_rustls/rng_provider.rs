//! Cryptographically secure random number source for rustls.
//!
//! Backed by [`random::SystemRng`], which reads from the operating system's
//! entropy source.

use random::{SystemRng, TryRng};
use rustls::crypto::{GetRandomFailed, SecureRandom};

/// The [`SecureRandom`] implementation used by this provider.
#[derive(Debug)]
pub(crate) struct RngProvider;

impl SecureRandom for RngProvider {
    fn fill(&self, buf: &mut [u8]) -> Result<(), GetRandomFailed> {
        let mut rng = SystemRng;
        rng.try_fill_bytes(buf).map_err(|_| GetRandomFailed)
    }
}
