#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Universally Unique IDentifiers (RFC 9562).
//!
//! Supports parsing all versions and generating version 4 (random) and version
//! 7 (Unix-epoch time-based) UUIDs.
//!
//! Parsing is strict: [`Uuid::parse`] accepts only the canonical 8-4-4-4-12
//! hexadecimal form (lowercase or uppercase). Alternative textual forms such as
//! the simple (unhyphenated), URN or brace-wrapped representations are
//! intentionally rejected, which avoids the lax-parsing pitfalls of some other
//! UUID implementations.
//!
//! Version 7 UUIDs carry a per-thread monotonic counter. Values produced by a
//! single thread are strictly increasing; ordering across threads is not
//! guaranteed. If the system clock moves backwards the generator keeps the last
//! logical timestamp and advances the counter, so ordering is preserved. See
//! `Uuid::new_v7` for details.
//!
//! # Feature flags
//!
//! | Flag   | Description                                                       | Default |
//! |--------|-------------------------------------------------------------------|---------|
//! | `std`  | Enables `Uuid::new_v4` and `Uuid::new_v7` via `rand`            | Yes     |
//! | `serde`| Enables `serde` serialization/deserialization                  | No      |
//!
//! When `std` is not enabled, the crate is `#![no_std]` compatible.
//! Parsing, formatting, and version detection are all available.
//!
//! # Example
//!
//! ```rust
//! use uuid::{Uuid, Version};
//!
//! // Parse a version-4 UUID from its canonical form
//! let uuid = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
//! assert_eq!(uuid.version(), Version::V4);
//!
//! // Format it without allocating
//! let text = uuid.to_heapless_string();
//! assert_eq!(text.as_str(), "f47ac10b-58cc-4372-a567-0e02b2c3d479");
//!
//! // Generate new UUIDs (requires the `std` feature, enabled by default)
//! # #[cfg(feature = "std")] {
//! let v4 = Uuid::new_v4();
//! let v7 = Uuid::new_v7();
//! assert_eq!(v4.version(), Version::V4);
//! assert_eq!(v7.version(), Version::V7);
//! # }
//! ```

use core::{fmt, str::FromStr};

#[cfg(feature = "std")]
use rand::Rng as _;

mod hex;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "std")]
thread_local! {
    /// Per-thread state for UUIDv7 monotonic counter generation.
    ///
    /// Tracks `(last_logical_timestamp_ms, counter_value)`. The timestamp is a
    /// *logical* value: it never moves backwards, even if the system clock
    /// does.
    static V7_STATE: std::cell::RefCell<(u64, u32)> = const { std::cell::RefCell::new((0, 0)) };
}

/// A 128-bit UUID (RFC 9562).
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Uuid([u8; 16]);

/// The version of a UUID (RFC 9562 §4.1).
///
/// Returned by [`Uuid::version`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Version {
    /// The Nil UUID (`00000000-0000-0000-0000-000000000000`).
    Nil,
    /// Version 1: Date-time and MAC address.
    V1,
    /// Version 2: DCE Security.
    V2,
    /// Version 3: Name-based (MD5).
    V3,
    /// Version 4: Random.
    V4,
    /// Version 5: Name-based (SHA-1).
    V5,
    /// Version 6: Reordered time-based.
    V6,
    /// Version 7: Unix Epoch time-based.
    V7,
    /// Version 8: Custom / experimental.
    V8,
    /// The Max UUID (`ffffffff-ffff-ffff-ffff-ffffffffffff`).
    Max,
    /// An unrecognized or future version.
    Unknown,
}

/// Errors that can occur when working with UUIDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The input is not a valid UUID: neither the canonical 36-byte string
    /// form, nor a 16-byte buffer where one is expected.
    InvalidUuid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUuid => f.write_str("invalid uuid"),
        }
    }
}

impl core::error::Error for Error {}

#[cfg(feature = "std")]
impl Uuid {
    /// Generate a new version 4 (random) UUID.
    ///
    /// # Panics
    ///
    /// Panics if the thread-local random number generator cannot be seeded from
    /// the operating system's entropy source. The generator is seeded once per
    /// thread and reused afterwards, so this can only happen on the first
    /// generation on a thread (or after seeding fails).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::{Uuid, Version};
    ///
    /// let uuid = Uuid::new_v4();
    /// assert_eq!(uuid.version(), Version::V4);
    /// ```
    #[inline]
    pub fn new_v4() -> Uuid {
        let mut bytes = [0u8; 16];
        rand::rng().fill_bytes(&mut bytes);

        // Set version nibble (bits 48-51) to 0100 (4)
        bytes[6] = (bytes[6] & 0x0f) | 0x40;

        // Set variant bits (bits 64-65) to 10
        bytes[8] = (bytes[8] & 0x3f) | 0x80;

        Uuid(bytes)
    }

    /// Generate a new version 7 UUID with a 32-bit monotonic counter.
    ///
    /// The 48-bit timestamp is milliseconds since the Unix epoch. A 32-bit
    /// monotonic counter occupies `rand_a` (12 bits) and the most-significant
    /// 20 bits of `rand_b`; the counter is placed immediately after the
    /// timestamp so that values sort lexicographically by creation order
    /// (RFC 9562 §6.2, Method 1).
    ///
    /// The counter is seeded with 31 random bits each time the logical
    /// timestamp advances and incremented on repeated calls within the same
    /// millisecond, so a single thread can generate at least 2³¹ UUIDs per
    /// millisecond without repeating a value. (The most-significant bit is
    /// reserved as a rollover guard, as suggested by RFC 9562.)
    ///
    /// # Clock rollback and counter overflow
    ///
    /// The timestamp used is a *logical* clock that never moves backwards. If
    /// the system clock is read as being behind the last used timestamp, the
    /// last timestamp is kept and the counter is advanced, preserving strict
    /// monotonicity. When the counter is exhausted (fewer than 2³¹ UUIDs later)
    /// the logical timestamp is advanced by one millisecond and the counter is
    /// re-seeded; generation never blocks or spins.
    ///
    /// Monotonicity only holds within a single thread: two threads generating
    /// UUIDs concurrently may return values out of order relative to each other.
    ///
    /// The 128-bit layout follows RFC 9562:
    ///
    /// ```text
    /// bytes 0..6   unix_ts_ms (48 bits, big-endian)
    /// byte  6      version (4 bits) | counter[31:28] (4 bits)
    /// byte  7      counter[27:20] (8 bits)
    /// byte  8      variant (2 bits) | counter[19:14] (6 bits)
    /// byte  9      counter[13:6] (8 bits)
    /// byte  10     counter[5:0] (6 bits) | random (2 bits)
    /// bytes 11..16 random (40 bits)
    /// ```
    ///
    /// The counter sits immediately after the timestamp/version/variant fields
    /// and above all random bits, so the 128-bit value sorts by creation order.
    ///
    /// # Panics
    ///
    /// Panics if the system clock is before the Unix epoch, or if the
    /// thread-local random number generator cannot be seeded from the operating
    /// system's entropy source.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::{Uuid, Version};
    ///
    /// let uuid = Uuid::new_v7();
    /// assert_eq!(uuid.version(), Version::V7);
    /// ```
    pub fn new_v7() -> Uuid {
        new_v7_at(v7_now_ms())
    }
}

/// Generate a version 7 UUID from an explicit wall-clock reading.
///
/// Split out from [`Uuid::new_v7`] so the counter, clock-rollback and overflow
/// logic can be exercised deterministically in tests.
#[cfg(feature = "std")]
fn new_v7_at(now_ms: u64) -> Uuid {
    let (timestamp, counter) = V7_STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let (mut ts, mut ctr) = *state;

        if now_ms > ts {
            // Normal case: the clock advanced, re-seed the counter.
            ts = now_ms;
            ctr = v7_random_counter();
        } else if ctr == u32::MAX {
            // Counter exhausted: advance the logical clock by one millisecond
            // and re-seed instead of spin-waiting, so generation never blocks
            // (RFC 9562 §6.2 "Counter Rollover Handling").
            ts += 1;
            ctr = v7_random_counter();
        } else {
            // Same millisecond, or the clock moved backwards: keep the logical
            // timestamp and advance the counter to preserve ordering.
            ctr += 1;
        }

        *state = (ts, ctr);
        (ts, ctr)
    });

    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);

    // unix_ts_ms: 48-bit big-endian timestamp
    bytes[0..6].copy_from_slice(&timestamp.to_be_bytes()[2..8]);

    // version (4 bits) + counter[31:28] (4 bits)
    bytes[6] = 0x70 | ((counter >> 28) & 0x0F) as u8;
    // counter[27:20] (8 bits)
    bytes[7] = (counter >> 20) as u8;
    // variant (2 bits) + counter[19:14] (6 bits)
    bytes[8] = 0x80 | ((counter >> 14) & 0x3F) as u8;
    // counter[13:6] (8 bits)
    bytes[9] = (counter >> 6) as u8;
    // counter[5:0] (6 bits) in the high bits, keeping the 2 low random bits
    // below all counter bits so that values remain strictly increasing.
    bytes[10] = (((counter & 0x3F) << 2) as u8) | (bytes[10] & 0x03);

    Uuid(bytes)
}

impl Uuid {
    /// Parse a UUID from its canonical 8-4-4-4-12 hexadecimal string form.
    ///
    /// Accepts both lowercase and uppercase hex characters.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidUuid`] if the input is not exactly 36 bytes,
    /// contains misplaced hyphens, or has non-hexadecimal bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
    /// assert_eq!(uuid.to_string(), "f47ac10b-58cc-4372-a567-0e02b2c3d479");
    /// ```
    pub fn parse(input: impl AsRef<[u8]>) -> Result<Uuid, Error> {
        let bytes = input.as_ref();
        if bytes.len() != 36 {
            return Err(Error::InvalidUuid);
        }

        if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
            return Err(Error::InvalidUuid);
        }

        let positions: [usize; 8] = [0, 4, 9, 14, 19, 24, 28, 32];
        let mut buf = [0u8; 16];

        for (j, &pos) in positions.iter().enumerate() {
            let b0 = hex::decode_pair(bytes[pos], bytes[pos + 1]).ok_or(Error::InvalidUuid)?;
            let b1 = hex::decode_pair(bytes[pos + 2], bytes[pos + 3]).ok_or(Error::InvalidUuid)?;
            buf[j * 2] = b0;
            buf[j * 2 + 1] = b1;
        }

        Ok(Uuid(buf))
    }

    /// Encode this UUID into its canonical 8-4-4-4-12 lowercase hexadecimal
    /// form without allocating.
    ///
    /// This is a zero-allocation alternative to `Uuid::to_string`: the text is
    /// written into a fixed-capacity [`heapless::String`] with a capacity of 36
    /// bytes, which is exactly the length of the canonical form, so it never
    /// spills and never touches the heap. Unlike a `String` it works in
    /// `no_std` environments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
    /// let text = uuid.to_heapless_string();
    /// assert_eq!(text.as_str(), "f47ac10b-58cc-4372-a567-0e02b2c3d479");
    /// assert_eq!(text.to_string(), uuid.to_string());
    /// ```
    #[inline]
    pub fn to_heapless_string(&self) -> heapless::String<36> {
        let buf = self.encode_lower();

        // SAFETY: `encode_lower` only writes ASCII hex digits (`0-9a-f`) and
        // `-`, all of which are valid UTF-8.
        let text = unsafe { core::str::from_utf8_unchecked(&buf) };

        let mut out = heapless::String::new();
        // The canonical form is exactly 36 bytes and `String<36>` has a 36-byte
        // capacity, so this can never fail.
        out.push_str(text).expect("36 bytes fit in heapless::String<36>");
        out
    }

    /// Raw 36-byte lowercase hyphenated encoding of `self`.
    #[inline]
    const fn encode_lower(&self) -> [u8; 36] {
        let bytes = &self.0;
        let mut buf = [0u8; 36];

        let groups: [(usize, usize); 5] = [(0, 8), (9, 13), (14, 18), (19, 23), (24, 36)];

        let mut src = 0;
        let mut gi = 0;
        while gi < groups.len() {
            let (start, end) = groups[gi];
            let mut j = start;
            while j < end {
                let b = bytes[src];
                src += 1;
                buf[j] = hex::encode_nibble(b >> 4);
                buf[j + 1] = hex::encode_nibble(b & 0x0f);
                j += 2;
            }
            if gi < 4 {
                buf[end] = b'-';
            }
            gi += 1;
        }

        buf
    }

    /// Create a UUID from a 16-byte array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::from_bytes([0; 16]);
    /// assert_eq!(uuid, Uuid::nil());
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: [u8; 16]) -> Uuid {
        Uuid(bytes)
    }

    /// Create a UUID from a byte slice of length 16.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidUuid`] if the slice is not exactly 16 bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::from_slice(&[0; 16]).unwrap();
    /// assert_eq!(uuid, Uuid::nil());
    /// ```
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Uuid, Error> {
        <[u8; 16]>::try_from(bytes).map(Uuid).map_err(|_| Error::InvalidUuid)
    }

    /// Return the 16-byte array representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::nil();
    /// assert_eq!(uuid.as_bytes(), [0; 16]);
    /// ```
    #[inline]
    pub const fn as_bytes(&self) -> [u8; 16] {
        self.0
    }

    /// Create a UUID from a `u128` value (big-endian).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::from_u128(0);
    /// assert_eq!(uuid, Uuid::nil());
    /// ```
    #[inline]
    pub const fn from_u128(v: u128) -> Uuid {
        Uuid([
            (v >> 120) as u8,
            (v >> 112) as u8,
            (v >> 104) as u8,
            (v >> 96) as u8,
            (v >> 88) as u8,
            (v >> 80) as u8,
            (v >> 72) as u8,
            (v >> 64) as u8,
            (v >> 56) as u8,
            (v >> 48) as u8,
            (v >> 40) as u8,
            (v >> 32) as u8,
            (v >> 24) as u8,
            (v >> 16) as u8,
            (v >> 8) as u8,
            v as u8,
        ])
    }

    /// Return the UUID as a `u128` value (big-endian).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::max();
    /// assert_eq!(uuid.as_u128(), !0);
    /// ```
    #[inline]
    pub const fn as_u128(&self) -> u128 {
        u128::from_be_bytes(self.0)
    }

    /// The Nil UUID: all 128 bits set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// assert_eq!(Uuid::nil().to_string(), "00000000-0000-0000-0000-000000000000");
    /// ```
    #[inline]
    pub const fn nil() -> Uuid {
        Uuid([0; 16])
    }

    /// The Max UUID: all 128 bits set to one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// assert_eq!(Uuid::max().to_string(), "ffffffff-ffff-ffff-ffff-ffffffffffff");
    /// ```
    #[inline]
    pub const fn max() -> Uuid {
        Uuid([0xff; 16])
    }

    /// Return the [`Version`] of this UUID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::{Uuid, Version};
    ///
    /// assert_eq!(Uuid::nil().version(), Version::Nil);
    /// assert_eq!(Uuid::max().version(), Version::Max);
    /// ```
    pub const fn version(&self) -> Version {
        match self.0[6] >> 4 {
            1 => return Version::V1,
            2 => return Version::V2,
            3 => return Version::V3,
            4 => return Version::V4,
            5 => return Version::V5,
            6 => return Version::V6,
            7 => return Version::V7,
            8 => return Version::V8,
            _ => {}
        }

        let mut all_zero = true;
        let mut all_max = true;

        let mut i = 0;
        while i < 16 {
            if self.0[i] != 0 {
                all_zero = false;
            }
            if self.0[i] != 0xff {
                all_max = false;
            }
            i += 1;
        }

        if all_zero {
            return Version::Nil;
        }
        if all_max {
            return Version::Max;
        }

        Version::Unknown
    }

    /// Return the Unix millisecond timestamp embedded in a UUIDv7.
    ///
    /// Only UUID version 7 (Unix Epoch time-based) carries a meaningful
    /// timestamp. All other versions return `None`.
    ///
    /// The timestamp is a 48-bit value representing milliseconds since
    /// the Unix epoch (1970-01-01 00:00:00 UTC).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::nil();
    /// assert_eq!(uuid.timestamp(), None);
    /// ```
    #[inline]
    pub fn timestamp(&self) -> Option<u64> {
        if self.version() != Version::V7 {
            return None;
        }
        Some(
            (self.0[0] as u64) << 40
                | (self.0[1] as u64) << 32
                | (self.0[2] as u64) << 24
                | (self.0[3] as u64) << 16
                | (self.0[4] as u64) << 8
                | self.0[5] as u64,
        )
    }
}

impl fmt::Display for Uuid {
    /// Formats the UUID as a lowercase 8-4-4-4-12 hyphenated string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let uuid = uuid::Uuid::nil();
    /// assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf = self.encode_lower();

        // SAFETY: `encode_lower` only writes ASCII hex digits (`0-9a-f`) and
        // `-`, all of which are valid UTF-8.
        let text = unsafe { core::str::from_utf8_unchecked(&buf) };
        f.write_str(text)
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl FromStr for Uuid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Uuid, Error> {
        Uuid::parse(s)
    }
}

impl TryFrom<&str> for Uuid {
    type Error = Error;

    #[inline]
    fn try_from(value: &str) -> Result<Uuid, Error> {
        Uuid::parse(value)
    }
}

impl TryFrom<&[u8]> for Uuid {
    type Error = Error;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Uuid, Error> {
        Uuid::parse(value)
    }
}

#[cfg(feature = "std")]
#[inline]
fn v7_now_ms() -> u64 {
    std::time::SystemTime::UNIX_EPOCH
        .elapsed()
        .expect("SystemTime::UNIX_EPOCH elapsed should not fail")
        .as_millis() as u64
}

#[cfg(feature = "std")]
#[inline]
fn v7_random_counter() -> u32 {
    // Reserve the most-significant bit as a rollover guard (RFC 9562 §6.2), so
    // a freshly seeded counter always has at least 2^31 increments of headroom
    // before it can reach u32::MAX.
    let mut seed = [0u8; 4];
    rand::rng().fill_bytes(&mut seed);
    u32::from_be_bytes(seed) & 0x7FFF_FFFF
}

/// Overwrite the thread-local v7 state (test-only).
#[cfg(all(test, feature = "std"))]
fn v7_set_state(timestamp_ms: u64, counter: u32) {
    V7_STATE.with(|cell| *cell.borrow_mut() = (timestamp_ms, counter));
}

/// Read the thread-local v7 state (test-only).
#[cfg(all(test, feature = "std"))]
fn v7_get_state() -> (u64, u32) {
    V7_STATE.with(|cell| *cell.borrow())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nil() {
        let uuid = Uuid::nil();
        assert_eq!(uuid.version(), Version::Nil);
        assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
        assert_eq!(Uuid::from_bytes(uuid.as_bytes()), uuid);
        assert_eq!(Uuid::parse(uuid.to_string()).unwrap(), uuid);
    }

    #[test]
    fn max() {
        let uuid = Uuid::max();
        assert_eq!(uuid.version(), Version::Max);
        assert_eq!(uuid.to_string(), "ffffffff-ffff-ffff-ffff-ffffffffffff");
        assert_eq!(Uuid::from_bytes(uuid.as_bytes()), uuid);
        assert_eq!(Uuid::parse(uuid.to_string()).unwrap(), uuid);
    }

    #[test]
    fn parse_v4() {
        let uuid = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
        assert_eq!(uuid.version(), Version::V4);
    }

    #[test]
    fn parse_v4_uppercase() {
        let uuid = Uuid::parse("F47AC10B-58CC-4372-A567-0E02B2C3D479").unwrap();
        assert_eq!(uuid.version(), Version::V4);
    }

    #[test]
    fn parse_v4_mixed_case() {
        let uuid = Uuid::parse("F47ac10B-58cC-4372-A567-0e02B2c3d479").unwrap();
        assert_eq!(uuid.version(), Version::V4);
    }

    #[test]
    fn parse_invalid_short() {
        assert_eq!(Uuid::parse("too-short"), Err(Error::InvalidUuid));
    }

    #[test]
    fn parse_invalid_long() {
        assert_eq!(
            Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479-extra"),
            Err(Error::InvalidUuid)
        );
    }

    #[test]
    fn parse_invalid_hyphen() {
        assert_eq!(Uuid::parse("f47ac10b-58cc-4372-a567x0e02b2c3d479"), Err(Error::InvalidUuid));
    }

    #[test]
    fn parse_invalid_hex() {
        assert_eq!(Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d47g"), Err(Error::InvalidUuid));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        ];
        let uuid = Uuid::from_bytes(bytes);
        assert_eq!(uuid.to_string(), "01020304-0506-0708-090a-0b0c0d0e0f10");
    }

    #[test]
    fn parse_then_to_string() {
        let s = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
        let uuid = Uuid::parse(s).unwrap();
        assert_eq!(uuid.to_string(), s);
    }

    #[test]
    fn eq_and_clone() {
        let a = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
        let b = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.clone(), b);
    }

    #[test]
    fn copy_works() {
        let a = Uuid::nil();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn from_str_valid() {
        let uuid: Uuid = "f47ac10b-58cc-4372-a567-0e02b2c3d479".parse().unwrap();
        assert_eq!(uuid.version(), Version::V4);
    }

    #[test]
    fn from_str_invalid() {
        let result: Result<Uuid, _> = "not-a-uuid".parse();
        assert_eq!(result, Err(Error::InvalidUuid));
    }

    #[test]
    fn unknown_version() {
        // Version nibble = 0, but not nil. Should be Unknown, not Nil.
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];
        let uuid = Uuid::from_bytes(bytes);
        assert_eq!(uuid.version(), Version::Unknown);
    }

    #[test]
    fn version_unknown_when_not_recognized() {
        // Version nibble = 0, variant = RFC4122, but not Nil
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];
        let uuid = Uuid::from_bytes(bytes);
        assert_eq!(uuid.version(), Version::Unknown);
    }

    #[cfg(feature = "std")]
    #[test]
    fn new_v4_has_correct_version() {
        let uuid = Uuid::new_v4();
        assert_eq!(uuid.version(), Version::V4);
    }

    #[cfg(feature = "std")]
    #[test]
    fn new_v4_has_correct_variant() {
        let uuid = Uuid::new_v4();
        // Variant bits: 10xxxxxx in byte 8
        assert_eq!(uuid.0[8] & 0xc0, 0x80);
    }

    #[cfg(feature = "std")]
    #[test]
    fn new_v7_has_correct_version() {
        let uuid = Uuid::new_v7();
        assert_eq!(uuid.version(), Version::V7);
    }

    #[cfg(feature = "std")]
    #[test]
    fn new_v7_has_correct_variant() {
        let uuid = Uuid::new_v7();
        assert_eq!(uuid.0[8] & 0xc0, 0x80);
    }

    #[cfg(feature = "std")]
    #[test]
    fn new_v7_has_recent_timestamp() {
        for _ in 0..100 {
            let uuid = Uuid::new_v7();
            let ts = uuid.timestamp().expect("v7 UUID should have a timestamp");

            assert!(ts > 1_577_836_800_000, "timestamp should be recent: {}", ts);
            assert!(ts < 4_102_444_800_000, "timestamp should not be in distant future: {}", ts);

            let now_ms = std::time::SystemTime::UNIX_EPOCH
                .elapsed()
                .expect("SystemTime::UNIX_EPOCH elapsed should not fail")
                .as_millis() as u64;

            let diff = now_ms.saturating_sub(ts);
            assert!(
                diff < 10_000,
                "v7 timestamp should be within 10s of system time: ts={ts}, now={now_ms}, diff={diff}",
            );
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn new_v7_has_monotonic_counter() {
        // Generate many UUIDs and verify the counter is monotonic
        // within the same millisecond and across timestamps.
        let uuids: Vec<Uuid> = (0..50).map(|_| Uuid::new_v7()).collect();

        for w in uuids.windows(2) {
            let (a, b) = (w[0], w[1]);
            let ts_a = a.timestamp().unwrap();
            let ts_b = b.timestamp().unwrap();
            let ctr_a = extract_v7_counter(&a);
            let ctr_b = extract_v7_counter(&b);

            assert!(
                (ts_a, ctr_a) < (ts_b, ctr_b),
                "UUIDs not monotonic: ts=({ts_a}, {ctr_a}), ts=({ts_b}, {ctr_b})",
            );
        }
    }

    /// Extract the 32-bit monotonic counter from a v7 UUID.
    #[cfg(feature = "std")]
    fn extract_v7_counter(uuid: &Uuid) -> u32 {
        // counter[31:28] -> byte 6 low nibble
        ((uuid.0[6] & 0x0F) as u32) << 28
            // counter[27:20] -> byte 7
            | (uuid.0[7] as u32) << 20
            // counter[19:14] -> byte 8 low 6 bits (below variant)
            | ((uuid.0[8] & 0x3F) as u32) << 14
            // counter[13:6] -> byte 9
            | (uuid.0[9] as u32) << 6
            // counter[5:0] -> byte 10 high 6 bits
            | ((uuid.0[10] >> 2) & 0x3F) as u32
    }

    /// Deterministic pseudo-random generator for property-style tests.
    fn xorshift64(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        x
    }

    // -- to_heapless_string -------------------------------------------------

    #[test]
    fn to_heapless_string_matches_to_string() {
        let samples = [
            Uuid::nil(),
            Uuid::max(),
            Uuid::from_u128(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10),
            Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap(),
        ];
        for uuid in samples {
            let text = uuid.to_heapless_string();
            assert_eq!(text.as_str(), uuid.to_string());
            assert_eq!(text.len(), 36);
            assert_eq!(text.as_bytes(), uuid.to_string().as_bytes());
            assert_eq!(format!("{text}"), uuid.to_string());
            assert_eq!(format!("{text:?}"), format!("{:?}", uuid.to_string()));
            // PartialEq<&str> and AsRef
            assert_eq!(text, uuid.to_string().as_str());
            assert_eq!(AsRef::<str>::as_ref(&text), uuid.to_string());
        }
    }

    #[test]
    fn to_heapless_string_allocates_nothing() {
        let uuid = Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap();
        let (text, c) = alloc_counter::measure(|| uuid.to_heapless_string());
        assert_eq!(c.calls, 0, "to_heapless_string must not allocate");
        assert_eq!(text.len(), 36);
        assert_eq!(text.as_str(), "f47ac10b-58cc-4372-a567-0e02b2c3d479");
    }

    #[test]
    fn parse_allocates_nothing() {
        let (_, c) = alloc_counter::measure(|| Uuid::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479").unwrap());
        assert_eq!(c.calls, 0, "parse must not allocate");
    }

    // -- trait impls --------------------------------------------------------

    #[test]
    fn hash_ord_default() {
        use std::collections::{BTreeMap, HashMap};

        let a = Uuid::from_u128(1);
        let b = Uuid::from_u128(2);

        // Hash: usable as a HashMap key.
        let mut map = HashMap::new();
        map.insert(a, "a");
        map.insert(b, "b");
        assert_eq!(map.get(&a), Some(&"a"));

        // Ord: sortable and usable as a BTreeMap key.
        let mut v = [b, a];
        v.sort();
        assert_eq!(v, [a, b]);
        let mut tree = BTreeMap::new();
        tree.insert(b, ());
        tree.insert(a, ());
        assert_eq!(tree.keys().next(), Some(&a));

        // Default: the nil UUID.
        assert_eq!(Uuid::default(), Uuid::nil());
    }

    #[test]
    fn try_from_impls() {
        let s = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
        let expected = Uuid::parse(s).unwrap();
        assert_eq!(Uuid::try_from(s).unwrap(), expected);
        assert_eq!(Uuid::try_from(s.as_bytes()).unwrap(), expected);
        assert!(Uuid::try_from("nope").is_err());
        assert!(Uuid::try_from(&b"nope"[..]).is_err());
    }

    // -- parsing strictness / robustness ------------------------------------

    #[test]
    fn parse_strict_rejections() {
        // Cases other implementations (notably Java's UUID.fromString) accept
        // leniently, plus encoding/length tricks. All must be rejected.
        let rejected: &[&[u8]] = &[
            b"",
            b"0",
            b"0-0-0-0-0",
            b"1-1-1-1-1",
            b"00112233-4455-6677-8899-aabbccddee",    // 34 chars
            b"00112233-4455-6677-8899-aabbccddeeff0", // 37 chars
            b"00112233445566778899aabbccddeeff",      // simple form
            b"urn:uuid:00112233-4455-6677-8899-aabbccddeeff",
            b"{00112233-4455-6677-8899-aabbccddeeff}",
            b" 00112233-4455-6677-8899-aabbccddeeff",
            b"00112233-4455-6677-8899-aabbccddeeff ",
            b"\t00112233-4455-6677-8899-aabbccddeeff",
            b"00112233-4455-6677-8899-aabbccddeeff\n",
            b"00112233-4455-6677-8899-aabbccddeefg",
            b"00112233-4455-6677-8899-aabbccddeefG",
            b"00112233+4455-6677-8899-aabbccddeeff",
            b"00112233-4455-6677-8899-aabbccddee f",
            b"00112233-4455-6677-8899-aabbccddee\x00",
            b"00112233-4455-6677-8899-aabbccddef\xff",
            b"00112233-4455-6677-8899-aabbccdde\xc3\xa9f", // multi-byte UTF-8
            b"00112233-4455-6677-8899-aabbccddeeff--",
            b"--00112233-4455-6677-8899-aabbccddee",
            b"00112233-4455-6677-8899-aabbccddee fffffffff",
        ];
        for input in rejected {
            assert_eq!(Uuid::parse(*input), Err(Error::InvalidUuid), "should reject {input:?}");
            if let Ok(s) = std::str::from_utf8(input) {
                assert_eq!(s.parse::<Uuid>(), Err(Error::InvalidUuid), "FromStr should reject {s:?}");
            }
        }
    }

    #[test]
    fn parse_accepts_both_cases() {
        let lower = Uuid::parse("00112233-4455-6677-8899-aabbccddeeff").unwrap();
        let upper = Uuid::parse("00112233-4455-6677-8899-AABBCCDDEEFF").unwrap();
        let mixed = Uuid::parse("00112233-4455-6677-8899-AabbCCddEEff").unwrap();
        assert_eq!(lower, upper);
        assert_eq!(lower, mixed);
        assert_eq!(lower.to_string(), "00112233-4455-6677-8899-aabbccddeeff");
    }

    #[test]
    fn parse_mutation_sweep() {
        // Every single-byte mutation of a valid UUID must either be rejected or
        // produce a value that round-trips; it must never panic.
        let good: [u8; 36] = *b"f47ac10b-58cc-4372-a567-0e02b2c3d479";
        for pos in 0..36 {
            for byte in 0..=255u8 {
                let mut buf = good;
                buf[pos] = byte;
                if let Ok(uuid) = Uuid::parse(&buf[..]) {
                    assert_eq!(Uuid::parse(uuid.to_string()).unwrap(), uuid);
                }
            }
        }
    }

    #[test]
    fn parse_random_garbage_never_panics() {
        let mut state = 0xdead_beef_cafe_babeu64;
        for _ in 0..100_000 {
            let len = (xorshift64(&mut state) % 65) as usize;
            let mut buf = [0u8; 64];
            for b in buf.iter_mut().take(len) {
                *b = xorshift64(&mut state) as u8;
            }
            // Must not panic; result is intentionally ignored.
            let _ = Uuid::parse(&buf[..len]);
        }
    }

    #[test]
    fn roundtrip_pseudo_random() {
        let mut state = 0x1234_5678_9abc_def0u64;
        for _ in 0..100_000 {
            let v = ((xorshift64(&mut state) as u128) << 64) | xorshift64(&mut state) as u128;
            let uuid = Uuid::from_u128(v);
            assert_eq!(uuid.as_u128(), v);
            assert_eq!(Uuid::parse(uuid.to_string()).unwrap(), uuid);
            assert_eq!(Uuid::parse(uuid.to_heapless_string().as_str()).unwrap(), uuid);
        }
    }

    // -- version / timestamp edge cases -------------------------------------

    #[test]
    fn version_edge_cases() {
        // Version nibble 0 but not all-zero -> Unknown.
        let unknown = Uuid::from_bytes([0x00, 0, 0, 0, 0, 0, 0x00, 0x80, 0x80, 0, 0, 0, 0, 0, 0, 0x01]);
        assert_eq!(unknown.version(), Version::Unknown);

        // Version nibble 0xf but not the Max UUID -> Unknown (matches RFC and
        // the corrected uuid-rs semantics).
        let almost_max = Uuid::from_bytes([0xff; 16]).as_u128() & !(1u128 << 64);
        assert_eq!(Uuid::from_u128(almost_max).version(), Version::Unknown);

        // All-ones is Max even though its nibble is 0xf.
        assert_eq!(Uuid::max().version(), Version::Max);
        assert_eq!(Uuid::nil().version(), Version::Nil);

        // Every recognized nibble maps to its version.
        for (nibble, expected) in [
            (1u8, Version::V1),
            (2, Version::V2),
            (3, Version::V3),
            (4, Version::V4),
            (5, Version::V5),
            (6, Version::V6),
            (7, Version::V7),
            (8, Version::V8),
        ] {
            let mut bytes = [0u8; 16];
            bytes[6] = nibble << 4;
            bytes[8] = 0x80;
            assert_eq!(Uuid::from_bytes(bytes).version(), expected);
        }
    }

    #[test]
    fn timestamp_only_for_v7() {
        assert_eq!(Uuid::nil().timestamp(), None);
        assert_eq!(Uuid::max().timestamp(), None);
        // A v1 UUID carries a timestamp too, but we do not decode it.
        let v1 = Uuid::from_u128(0x1234_5678_9abc_1def_8000_0000_0000_0001);
        assert_eq!(v1.version(), Version::V1);
        assert_eq!(v1.timestamp(), None);

        let v7 = Uuid::from_bytes([
            0x00, 0x00, 0x01, 0x90, 0x12, 0x34, 0x70, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ]);
        assert_eq!(v7.version(), Version::V7);
        assert_eq!(v7.timestamp(), Some(0x0190_1234));
    }

    // -- v7 counter / clock behaviour (deterministic) -----------------------

    #[cfg(feature = "std")]
    #[test]
    fn v7_same_ms_values_strictly_increase() {
        // Regression test for the byte-10 layout: within one logical
        // millisecond, consecutive UUIDs must be strictly increasing as u128
        // (i.e. lexicographically/bytewise sortable), not just by counter.
        v7_set_state(1_000, 0);
        let mut previous: Option<Uuid> = None;
        for _ in 0..10_000 {
            let uuid = new_v7_at(1_000);
            if let Some(prev) = previous {
                assert!(prev.as_u128() < uuid.as_u128(), "not increasing: {prev} -> {uuid}");
            }
            previous = Some(uuid);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn v7_clock_rollback_keeps_logical_timestamp() {
        v7_set_state(1_000, 5);
        // Clock reads far in the past: the logical timestamp is held and the
        // counter advances.
        let uuid = new_v7_at(500);
        assert_eq!(uuid.timestamp(), Some(1_000));
        assert_eq!(extract_v7_counter(&uuid), 6);
        assert_eq!(v7_get_state(), (1_000, 6));

        let uuid = new_v7_at(500);
        assert_eq!(uuid.timestamp(), Some(1_000));
        assert_eq!(extract_v7_counter(&uuid), 7);
    }

    #[cfg(feature = "std")]
    #[test]
    fn v7_counter_overflow_advances_logical_clock() {
        v7_set_state(100, u32::MAX);
        let uuid = new_v7_at(100);
        // The logical timestamp advanced by one ms and the counter was
        // re-seeded (rollover guard keeps it below 2^31), so generation does
        // not block.
        assert_eq!(uuid.timestamp(), Some(101));
        let counter = extract_v7_counter(&uuid);
        assert!(counter <= 0x7FFF_FFFF, "counter should be re-seeded: {counter}");
        assert_eq!(v7_get_state(), (101, counter));
    }

    #[cfg(feature = "std")]
    #[test]
    fn v7_advancing_clock_reseeds_counter() {
        v7_set_state(100, 5);
        let uuid = new_v7_at(200);
        assert_eq!(uuid.timestamp(), Some(200));
        assert!(extract_v7_counter(&uuid) <= 0x7FFF_FFFF);
    }

    #[cfg(feature = "std")]
    #[test]
    fn v7_real_calls_are_strictly_increasing() {
        let mut previous = Uuid::new_v7();
        for _ in 0..5_000 {
            let uuid = Uuid::new_v7();
            assert!(
                previous.as_u128() < uuid.as_u128(),
                "consecutive v7 UUIDs must be strictly increasing: {previous} -> {uuid}"
            );
            previous = uuid;
        }
    }

    #[cfg(all(feature = "std", not(target_arch = "wasm32")))]
    #[test]
    fn v7_multithread_unique() {
        use std::{
            collections::HashSet,
            sync::{Arc, Mutex},
        };

        const THREADS: usize = 4;
        const PER_THREAD: usize = 20_000;

        let seen = Arc::new(Mutex::new(HashSet::with_capacity(THREADS * PER_THREAD)));
        let handles: Vec<_> = (0..THREADS)
            .map(|_| {
                let seen = Arc::clone(&seen);
                std::thread::spawn(move || {
                    let local: Vec<Uuid> = (0..PER_THREAD).map(|_| Uuid::new_v7()).collect();
                    let mut seen = seen.lock().unwrap();
                    for uuid in local {
                        assert!(seen.insert(uuid), "duplicate v7 UUID across threads: {uuid}");
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(seen.lock().unwrap().len(), THREADS * PER_THREAD);
    }

    #[cfg(all(feature = "std", not(target_arch = "wasm32")))]
    #[test]
    fn v4_multithread_unique() {
        use std::{
            collections::HashSet,
            sync::{Arc, Mutex},
        };

        const THREADS: usize = 4;
        const PER_THREAD: usize = 20_000;

        let seen = Arc::new(Mutex::new(HashSet::with_capacity(THREADS * PER_THREAD)));
        let handles: Vec<_> = (0..THREADS)
            .map(|_| {
                let seen = Arc::clone(&seen);
                std::thread::spawn(move || {
                    let local: Vec<Uuid> = (0..PER_THREAD).map(|_| Uuid::new_v4()).collect();
                    let mut seen = seen.lock().unwrap();
                    for uuid in local {
                        assert!(seen.insert(uuid), "duplicate v4 UUID across threads: {uuid}");
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(seen.lock().unwrap().len(), THREADS * PER_THREAD);
    }
}
