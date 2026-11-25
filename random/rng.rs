//! Core traits for random number generation, value sampling and seeding.

use core::ops::{Range, RangeInclusive};

/// A fallible source of random data.
///
/// This is implemented by [`SystemRng`](crate::SystemRng), whose underlying
/// operating-system call can fail, and by user-provided hardware generators.
/// Implement this trait to seed [`ChaChaRng`](crate::ChaChaRng) from a HAL or SDK
/// entropy source in `no_std` environments.
pub trait TryRng {
    /// Error returned when the source cannot produce random data.
    type Error;

    /// Returns the next 32-bit value.
    fn try_next_u32(&mut self) -> Result<u32, Self::Error>;

    /// Returns the next 64-bit value.
    ///
    /// The default implementation combines two 32-bit values.
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let high = u64::from(self.try_next_u32()?);
        let low = u64::from(self.try_next_u32()?);
        Ok((high << 32) | low)
    }

    /// Fills `dst` with random bytes.
    ///
    /// The default implementation uses [`TryRng::try_next_u64`].
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        let (chunks, remainder) = dst.as_chunks_mut::<8>();
        for chunk in chunks {
            *chunk = self.try_next_u64()?.to_le_bytes();
        }
        if !remainder.is_empty() {
            let bytes = self.try_next_u64()?.to_le_bytes();
            remainder.copy_from_slice(&bytes[..remainder.len()]);
        }
        Ok(())
    }
}

/// An infallible source of random data.
///
/// Values returned by `next_u32`/`next_u64` are interpreted as little-endian
/// when they are turned into bytes by [`Rng::fill_bytes`].
pub trait Rng {
    /// Returns the next 32-bit value.
    fn next_u32(&mut self) -> u32;

    /// Returns the next 64-bit value.
    ///
    /// The default implementation combines two 32-bit values.
    fn next_u64(&mut self) -> u64 {
        let high = u64::from(self.next_u32());
        let low = u64::from(self.next_u32());
        (high << 32) | low
    }

    /// Fills `dst` with random bytes.
    ///
    /// The default implementation uses [`Rng::next_u64`].
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        let (chunks, remainder) = dst.as_chunks_mut::<8>();
        for chunk in chunks {
            *chunk = self.next_u64().to_le_bytes();
        }
        if !remainder.is_empty() {
            let bytes = self.next_u64().to_le_bytes();
            remainder.copy_from_slice(&bytes[..remainder.len()]);
        }
    }
}

impl<R: Rng + ?Sized> Rng for &mut R {
    fn next_u32(&mut self) -> u32 {
        (**self).next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        (**self).next_u64()
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        (**self).fill_bytes(dst);
    }
}

impl<R: TryRng + ?Sized> TryRng for &mut R {
    type Error = R::Error;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        (**self).try_next_u32()
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        (**self).try_next_u64()
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        (**self).try_fill_bytes(dst)
    }
}

/// A generator that can be created from a fixed seed.
///
/// Implementations are deterministic: the same seed always produces the same
/// sequence.
pub trait SeedableRng: Sized {
    /// The seed accepted by [`SeedableRng::from_seed`].
    type Seed: AsRef<[u8]> + AsMut<[u8]> + Clone;

    /// Creates a generator from `seed`.
    fn from_seed(seed: Self::Seed) -> Self;

    /// Creates a generator from a 64-bit seed.
    ///
    /// This is convenient for tests, not for deriving cryptographic keys.
    fn seed_from_u64(state: u64) -> Self;

    /// Creates a generator seeded from an infallible RNG.
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self;

    /// Creates a generator seeded from a fallible RNG.
    ///
    /// This is the entry point for seeding from a hardware generator in
    /// `no_std` environments.
    fn try_from_rng<R: TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error>;
}

/// A type that can be produced from an [`Rng`].
///
/// Implemented for the primitive integer types, [`bool`] and arrays of any
/// length.
pub trait Random: Sized {
    /// Produces a random value using `rng`.
    fn random_from<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

macro_rules! impl_random_integer {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Random for $ty {
                fn random_from<R: Rng + ?Sized>(rng: &mut R) -> Self {
                    let mut bytes = [0u8; core::mem::size_of::<$ty>()];
                    rng.fill_bytes(&mut bytes);
                    <$ty>::from_le_bytes(bytes)
                }
            }
        )*
    };
}

impl_random_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Random for bool {
    fn random_from<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut byte = [0u8; 1];
        rng.fill_bytes(&mut byte);
        byte[0] & 1 != 0
    }
}

impl<T: Random, const N: usize> Random for [T; N] {
    fn random_from<R: Rng + ?Sized>(rng: &mut R) -> Self {
        core::array::from_fn(|_| T::random_from(rng))
    }
}

/// A range that can be sampled uniformly into `T`.
///
/// Implemented for exclusive and inclusive ranges of every primitive integer
/// type except `u128`/`i128`. Sampling panics if the range is empty.
pub trait SampleRange<T> {
    /// Samples a value from `self` using `rng`.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

/// Samples a value in `0..span`.
///
/// Uses rejection sampling so every value is equally likely even when `span`
/// does not divide 2^128.
fn sample_below<R: Rng + ?Sized>(rng: &mut R, span: u128) -> u128 {
    debug_assert!(span > 0);
    if span == 1 {
        return 0;
    }
    // Largest multiple of `span` representable in `u128` (exclusive).
    let zone = u128::MAX - (u128::MAX % span);
    loop {
        let value = (u128::from(rng.next_u64()) << 64) | u128::from(rng.next_u64());
        if value < zone {
            return value % span;
        }
    }
}

macro_rules! impl_sample_range_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl SampleRange<$ty> for Range<$ty> {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                    assert!(self.start < self.end, "cannot sample an empty range");
                    let span = self.end as u128 - self.start as u128;
                    self.start + sample_below(rng, span) as $ty
                }
            }

            impl SampleRange<$ty> for RangeInclusive<$ty> {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                    assert!(self.start() <= self.end(), "cannot sample an empty range");
                    let span = *self.end() as u128 - *self.start() as u128 + 1;
                    *self.start() + sample_below(rng, span) as $ty
                }
            }
        )*
    };
}

macro_rules! impl_sample_range_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl SampleRange<$ty> for Range<$ty> {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                    assert!(self.start < self.end, "cannot sample an empty range");
                    let start = self.start as i128;
                    let span = (self.end as i128 - start) as u128;
                    (start + sample_below(rng, span) as i128) as $ty
                }
            }

            impl SampleRange<$ty> for RangeInclusive<$ty> {
                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                    assert!(self.start() <= self.end(), "cannot sample an empty range");
                    let start = *self.start() as i128;
                    let span = (*self.end() as i128 - start) as u128 + 1;
                    (start + sample_below(rng, span) as i128) as $ty
                }
            }
        )*
    };
}

impl_sample_range_unsigned!(u8, u16, u32, u64, usize);
impl_sample_range_signed!(i8, i16, i32, i64, isize);

/// Convenience methods built on top of [`Rng`].
pub trait RngExt: Rng {
    /// Produces a random value.
    fn random<T: Random>(&mut self) -> T {
        T::random_from(self)
    }

    /// Fills `dest` with random values.
    fn fill<T: Random>(&mut self, dest: &mut [T]) {
        for value in dest {
            *value = self.random();
        }
    }

    /// Produces a value uniformly distributed in `range`.
    ///
    /// # Panics
    ///
    /// Panics if `range` is empty.
    fn random_range<T, R: SampleRange<T>>(&mut self, range: R) -> T {
        range.sample(self)
    }
}

impl<R: Rng + ?Sized> RngExt for R {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChaChaRng, SeedableRng};

    /// A tiny deterministic RNG for testing the default trait methods.
    struct CountingRng(u64);

    impl Rng for CountingRng {
        fn next_u32(&mut self) -> u32 {
            self.0 = self.0.wrapping_add(1);
            self.0 as u32
        }
    }

    #[test]
    fn default_fill_bytes_is_little_endian() {
        let mut rng = CountingRng(0);
        let mut bytes = [0u8; 12];
        rng.fill_bytes(&mut bytes);

        // `next_u64` combines two `next_u32` calls as `(high << 32) | low`.
        // The first chunk uses 1 (high) and 2 (low); the remainder uses the
        // first four bytes of the next value, which is 4.
        assert_eq!(&bytes[..4], &2u32.to_le_bytes());
        assert_eq!(&bytes[4..8], &1u32.to_le_bytes());
        assert_eq!(&bytes[8..], &4u32.to_le_bytes());
    }

    #[test]
    fn default_try_fill_bytes_is_little_endian() {
        struct InfallibleRng(CountingRng);
        impl TryRng for InfallibleRng {
            type Error = core::convert::Infallible;

            fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
                Ok(self.0.next_u32())
            }
        }

        let mut rng = InfallibleRng(CountingRng(10));
        let mut bytes = [0u8; 8];
        rng.try_fill_bytes(&mut bytes).unwrap();
        assert_eq!(&bytes[..4], &12u32.to_le_bytes());
        assert_eq!(&bytes[4..], &11u32.to_le_bytes());
    }

    #[test]
    fn seed_from_u64_is_deterministic() {
        let a = ChaChaRng::seed_from_u64(0x1234_5678_9abc_def0);
        let b = ChaChaRng::seed_from_u64(0x1234_5678_9abc_def0);
        let c = ChaChaRng::seed_from_u64(0x1234_5678_9abc_def1);
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut left = [0u8; 333];
        let mut right = [0u8; 333];
        let mut other = [0u8; 333];
        a.fill_bytes(&mut left);
        b.fill_bytes(&mut right);
        c.fill_bytes(&mut other);
        assert_eq!(left, right);
        assert_ne!(left, other);
    }

    #[test]
    fn try_from_rng_seeds_from_a_fallible_source() {
        struct Source(u8);
        impl TryRng for Source {
            type Error = core::convert::Infallible;

            fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
                Ok(0)
            }

            fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
                dst.fill(self.0);
                Ok(())
            }
        }

        let mut rng = ChaChaRng::try_from_rng(&mut Source(0x42)).unwrap();
        let mut from_source = [0u8; 64];
        rng.fill_bytes(&mut from_source);

        let mut expected = ChaChaRng::from_seed([0x42; 40]);
        let mut expected_bytes = [0u8; 64];
        expected.fill_bytes(&mut expected_bytes);

        assert_eq!(from_source, expected_bytes);
    }

    #[test]
    fn random_produces_all_integer_types() {
        let mut rng = ChaChaRng::seed_from_u64(9);
        let _: u8 = rng.random();
        let _: u16 = rng.random();
        let _: u32 = rng.random();
        let _: u64 = rng.random();
        let _: u128 = rng.random();
        let _: usize = rng.random();
        let _: i8 = rng.random();
        let _: i16 = rng.random();
        let _: i32 = rng.random();
        let _: i64 = rng.random();
        let _: i128 = rng.random();
        let _: isize = rng.random();
        let _: bool = rng.random();
        let _: [u8; 32] = rng.random();
        let _: [u64; 25] = rng.random();
    }

    #[test]
    fn bool_covers_both_values() {
        let mut rng = ChaChaRng::seed_from_u64(10);
        let mut seen = [false; 2];
        for _ in 0..128 {
            seen[usize::from(rng.random::<bool>())] = true;
        }
        assert_eq!(seen, [true, true]);
    }

    #[test]
    fn fill_values_round_trips() {
        let mut rng = ChaChaRng::seed_from_u64(11);
        let mut values = [0u64; 17];
        rng.fill(&mut values);
        assert!(values.iter().any(|&value| value != 0));
    }

    #[test]
    fn exclusive_range_stays_below_end() {
        let mut rng = ChaChaRng::seed_from_u64(12);
        for _ in 0..10_000 {
            let value: u8 = rng.random_range(0u8..10u8);
            assert!(value < 10);
        }
    }

    #[test]
    fn full_u8_inclusive_range_hits_both_ends() {
        let mut rng = ChaChaRng::seed_from_u64(13);
        let mut seen = [false; 256];
        for _ in 0..100_000 {
            seen[usize::from(rng.random_range(0..=u8::MAX))] = true;
        }
        assert!(seen.iter().all(|&value| value));
    }

    #[test]
    fn full_u64_inclusive_range_does_not_overflow() {
        let mut rng = ChaChaRng::seed_from_u64(14);
        for _ in 0..1_000 {
            let _: u64 = rng.random_range(0..=u64::MAX);
        }
    }

    #[test]
    fn single_value_ranges_are_deterministic() {
        let mut rng = ChaChaRng::seed_from_u64(15);
        for _ in 0..100 {
            assert_eq!(rng.random_range(7..8), 7);
            assert_eq!(rng.random_range(7..=7), 7);
            assert_eq!(rng.random_range(-3..=-3), -3);
        }
    }

    #[test]
    fn signed_ranges_stay_in_bounds() {
        let mut rng = ChaChaRng::seed_from_u64(16);
        for _ in 0..10_000 {
            let value: i16 = rng.random_range(-5i16..=5i16);
            assert!((-5..=5).contains(&value));
        }
    }

    #[test]
    fn full_signed_range_is_supported() {
        let mut rng = ChaChaRng::seed_from_u64(17);
        for _ in 0..1_000 {
            let _: i32 = rng.random_range(i32::MIN..=i32::MAX);
        }
    }

    #[test]
    fn usize_range_is_supported() {
        let mut rng = ChaChaRng::seed_from_u64(18);
        let len = 37usize;
        for _ in 0..10_000 {
            let value = rng.random_range(0..len);
            assert!(value < len);
        }
    }

    #[test]
    #[should_panic(expected = "cannot sample an empty range")]
    #[allow(clippy::reversed_empty_ranges)]
    fn empty_exclusive_range_panics() {
        let mut rng = ChaChaRng::seed_from_u64(19);
        let _: u32 = rng.random_range(5u32..5u32);
    }

    #[test]
    #[should_panic(expected = "cannot sample an empty range")]
    #[allow(clippy::reversed_empty_ranges)]
    fn inverted_inclusive_range_panics() {
        let mut rng = ChaChaRng::seed_from_u64(20);
        let _: u32 = rng.random_range(5u32..=4u32);
    }

    #[test]
    fn sampling_is_broadly_uniform() {
        let mut rng = ChaChaRng::seed_from_u64(21);
        let mut buckets = [0usize; 4];
        for _ in 0..10_000 {
            buckets[rng.random_range(0usize..4)] += 1;
        }
        for count in buckets {
            // With 10,000 samples the expected count is 2,500. A very wide
            // margin keeps this stable across seeds while still catching a
            // badly biased implementation.
            assert!((1_500..3_500).contains(&count), "{buckets:?}");
        }
    }
}
