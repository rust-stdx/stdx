//! Concrete random number generators.
//!
//! [`ChaChaRng`] is the seedable, deterministic generator. [`SystemRng`] reads
//! entropy from the operating system and is only available with the `system`
//! feature.

use crypto::{StreamCipher, chacha::ChaCha12Djb};

use crate::rng::{Rng, SeedableRng};

/// The standard, seedable random number generator.
///
/// `ChaChaRng` uses the 12-round ChaCha stream cipher from the stdx [`crypto`]
/// crate. It is deterministic: two generators created from the same 40-byte
/// seed produce exactly the same sequence.
///
/// The seed layout is 32 bytes of key followed by 8 bytes of stream identifier
/// (nonce). [`SeedableRng::seed_from_u64`] expands its input to 40 bytes with
/// SplitMix64. The thread-local generator draws a fresh key *and* nonce from
/// the operating system on every re-seed.
///
/// This generator is suitable for tests, simulations and reproducible
/// workloads. It is **not** automatically re-seeded; for long-running use where
/// forward secrecy matters, use [`thread_rng`](crate::thread_rng), which
/// re-seeds periodically from the operating system.
///
/// # Examples
///
/// ```
/// use random::{ChaChaRng, Rng, SeedableRng};
///
/// let mut rng = ChaChaRng::seed_from_u64(42);
/// let mut bytes = [0u8; 16];
/// rng.fill_bytes(&mut bytes);
///
/// // The same seed produces the same bytes.
/// let mut other = ChaChaRng::seed_from_u64(42);
/// let mut same = [0u8; 16];
/// other.fill_bytes(&mut same);
/// assert_eq!(bytes, same);
/// ```
pub struct ChaChaRng {
    cipher: ChaCha12Djb,
}

impl core::fmt::Debug for ChaChaRng {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("ChaChaRng { .. }")
    }
}

impl Rng for ChaChaRng {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        self.fill_bytes(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        self.fill_bytes(&mut bytes);
        u64::from_le_bytes(bytes)
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.cipher.fill_keystream(dst).expect("ChaCha keystream exhausted");
    }
}

impl SeedableRng for ChaChaRng {
    /// 32-byte key followed by an 8-byte nonce.
    type Seed = [u8; 40];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut key = [0u8; 32];
        key.copy_from_slice(&seed[..32]);
        let mut nonce = [0u8; 8];
        nonce.copy_from_slice(&seed[32..]);
        Self {
            cipher: ChaCha12Djb::new(&key, &nonce),
        }
    }

    fn seed_from_u64(state: u64) -> Self {
        let mut state = state;
        let mut seed = [0u8; 40];
        for chunk in seed.chunks_mut(8) {
            let word = splitmix64(&mut state).to_le_bytes();
            chunk.copy_from_slice(&word[..chunk.len()]);
        }
        Self::from_seed(seed)
    }

    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut seed = [0u8; 40];
        rng.fill_bytes(&mut seed);
        Self::from_seed(seed)
    }

    fn try_from_rng<R: crate::rng::TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        let mut seed = [0u8; 40];
        rng.try_fill_bytes(&mut seed)?;
        Ok(Self::from_seed(seed))
    }
}

/// Advances a SplitMix64 state and returns the next output word.
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// A fallible source of operating-system entropy.
///
/// Unlike [`ChaChaRng`], `SystemRng` produces different data on every call and
/// is suitable for cryptographic use. It is fallible because the operating
/// system may not provide an entropy source on every platform.
///
/// For high-throughput use, prefer [`thread_rng`](crate::thread_rng), which
/// batches entropy from `SystemRng` and re-seeds a ChaCha12 generator.
///
/// # Examples
///
/// ```
/// use random::{SystemRng, TryRng};
///
/// let mut rng = SystemRng;
/// let mut key = [0u8; 32];
/// rng.try_fill_bytes(&mut key).expect("OS entropy is unavailable");
/// ```
#[cfg(feature = "system")]
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemRng;

#[cfg(feature = "system")]
impl crate::rng::TryRng for SystemRng {
    type Error = getrandom::Error;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut bytes = [0u8; 4];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut bytes = [0u8; 8];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        getrandom::fill(dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chacha_rng_fills_are_independent_of_chunking() {
        let mut one_shot = ChaChaRng::seed_from_u64(0xdead_beef);
        let mut expected = [0u8; 1024];
        one_shot.fill_bytes(&mut expected);

        for chunk_size in [1usize, 7, 31, 32, 63, 64, 65, 127, 128, 255] {
            let mut rng = ChaChaRng::seed_from_u64(0xdead_beef);
            let mut actual = [0u8; 1024];
            let mut written = 0;
            while written < actual.len() {
                let take = core::cmp::min(chunk_size, actual.len() - written);
                rng.fill_bytes(&mut actual[written..written + take]);
                written += take;
            }
            assert_eq!(actual, expected, "chunk size {chunk_size}");
        }
    }

    #[test]
    fn chacha_rng_zero_length_fill_does_not_advance() {
        let mut rng = ChaChaRng::seed_from_u64(1);
        let mut first = [0u8; 32];
        rng.fill_bytes(&mut first);
        rng.fill_bytes(&mut []);
        let mut second = [0u8; 32];
        rng.fill_bytes(&mut second);

        let mut reference = ChaChaRng::seed_from_u64(1);
        let mut first_reference = [0u8; 32];
        reference.fill_bytes(&mut first_reference);
        let mut second_reference = [0u8; 32];
        reference.fill_bytes(&mut second_reference);

        assert_eq!(first, first_reference);
        assert_eq!(second, second_reference);
    }

    #[test]
    fn chacha_rng_debug_does_not_reveal_state() {
        let rng = ChaChaRng::seed_from_u64(1);
        assert_eq!(std::format!("{rng:?}"), "ChaChaRng { .. }");
    }

    #[test]
    fn chacha_rng_from_rng_matches_from_seed() {
        struct Constant(u8);
        impl Rng for Constant {
            fn next_u32(&mut self) -> u32 {
                0
            }

            fn next_u64(&mut self) -> u64 {
                0
            }

            fn fill_bytes(&mut self, dst: &mut [u8]) {
                dst.fill(self.0);
            }
        }

        let mut from_rng = ChaChaRng::from_rng(&mut Constant(0x5a));
        let mut actual = [0u8; 97];
        from_rng.fill_bytes(&mut actual);

        let mut from_seed = ChaChaRng::from_seed([0x5a; 40]);
        let mut expected = [0u8; 97];
        from_seed.fill_bytes(&mut expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn chacha_rng_nonce_changes_the_stream() {
        let mut with_zero_nonce = ChaChaRng::from_seed([7u8; 40]);
        let mut with_random_nonce = ChaChaRng::from_seed({
            let mut seed = [7u8; 40];
            seed[32..].copy_from_slice(&[9u8; 8]);
            seed
        });

        let mut a = [0u8; 128];
        let mut b = [0u8; 128];
        with_zero_nonce.fill_bytes(&mut a);
        with_random_nonce.fill_bytes(&mut b);
        assert_ne!(a, b);
    }

    #[cfg(feature = "system")]
    #[test]
    fn system_rng_produces_entropy() {
        use crate::rng::TryRng;

        let mut rng = SystemRng;
        let mut bytes = [0u8; 64];
        rng.try_fill_bytes(&mut bytes).expect("OS entropy is unavailable");
        assert!(bytes.iter().any(|&byte| byte != 0));
    }

    #[cfg(feature = "system")]
    #[test]
    fn system_rng_handles_empty_slices() {
        use crate::rng::TryRng;

        let mut rng = SystemRng;
        rng.try_fill_bytes(&mut []).expect("OS entropy is unavailable");
        let mut bytes = [0u8; 4];
        rng.try_fill_bytes(&mut bytes).expect("OS entropy is unavailable");
        assert!(bytes.iter().any(|&byte| byte != 0));
    }
}
