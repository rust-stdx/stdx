#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Universally Unique IDentifiers (RFC 9562).
//! Supports generating v4 and v7 UUIDs, and parsing all versions.
//!
//! UUIDs v7 support thread-local monotonic counters.
//!
//! # Feature flags
//!
//! | Flag   | Description                                                       | Default |
//! |--------|-------------------------------------------------------------------|---------|
//! | `std`  | Enables [`Uuid::new_v4`] and [`Uuid::new_v7`] via `rand`          | Yes     |
//! | `serde`| Enables [`serde`] serialization/deserialization                  | No      |
//! | `sqlx` | Enables [`sqlx`] integration for PostgreSQL (type, encode, decode) | No      |
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
//! // Generate new UUIDs (requires `std` feature, enabled by default)
//! let v4 = Uuid::new_v4();
//! let v7 = Uuid::new_v7();
//! assert_eq!(v4.version(), Version::V4);
//! assert_eq!(v7.version(), Version::V7);
//! ```

use core::{fmt, str::FromStr};

mod hex;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "sqlx")]
mod sqlx;

#[cfg(feature = "std")]
thread_local! {
    /// Per-thread state for UUIDv7 monotonic counter generation.
    /// Tracks `(last_timestamp_ms, counter_value)`.
    static V7_STATE: std::cell::RefCell<(u64, u32)> = std::cell::RefCell::new((0, 0));
}

/// A 128-bit UUID (RFC 9562).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uuid([u8; 16]);

/// The version of a UUID (RFC 9562 §4.1).
///
/// Returned by [`Uuid::version`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The input string is not a valid UUID.
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
        let mut uuid = Uuid(rand::random());

        // Set version nibble (bits 48-51) to 0100 (4)
        uuid.0[6] = (uuid.0[6] & 0x0f) | 0x40;

        // Set variant bits (bits 64-65) to 10
        uuid.0[8] = (uuid.0[8] & 0x3f) | 0x80;

        uuid
    }

    /// Generate a new version 7 UUID with a 32-bit monotonic counter.
    ///
    /// The 48-bit timestamp is milliseconds since the Unix epoch.
    /// A 32-bit monotonic counter occupies `rand_a` (12 bits) and the
    /// most-significant 20 bits of `rand_b`, guaranteeing up to
    /// 2³² UUIDs within a single millisecond per thread (per
    /// RFC 9562 §6.2, Method 1).
    ///
    /// The counter is seeded with 31 random bits each time the
    /// timestamp advances and incremented on repeated calls within
    /// the same millisecond. If the counter does overflow, this
    /// function spin-waits for the next millisecond tick.
    ///
    /// The 128-bit layout follows RFC 9562:
    ///
    /// ```text
    ///  0                   1                   2                   3
    ///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                           unix_ts_ms                          |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |          unix_ts_ms           |  ver  |       rand_a          |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |var|                        rand_b                             |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                            rand_b                             |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// ```
    ///
    /// The 32-bit counter spans `rand_a` (12 bits) and the most-
    /// significant 20 bits of `rand_b` (bytes 6–10).
    ///
    /// # Panics
    ///
    /// Panics if the system clock is before the Unix epoch.
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
        let mut timestamp = v7_now_ms();

        let counter = loop {
            match V7_STATE.with(|cell| {
                let mut state = cell.borrow_mut();

                if timestamp > state.0 {
                    state.0 = timestamp;
                    state.1 = v7_random_counter();
                } else if timestamp == state.0 {
                    if state.1 < u32::MAX {
                        state.1 += 1;
                    } else {
                        return None; // overflow, spin until next ms
                    }
                } else {
                    // clock moved backward, re-seed
                    state.0 = timestamp;
                    state.1 = v7_random_counter();
                }
                Some(state.1)
            }) {
                Some(ctr) => break ctr,
                None => {
                    while v7_now_ms() == timestamp {
                        std::hint::spin_loop();
                    }
                    timestamp = v7_now_ms();
                }
            }
        };

        let mut uuid = Uuid(rand::random());

        // unix_ts_ms: 48-bit big-endian timestamp
        uuid.0[0..6].copy_from_slice(&timestamp.to_be_bytes()[2..8]);
        // uuid.0[0] = (ts >> 40) as u8;
        // uuid.0[1] = (ts >> 32) as u8;
        // uuid.0[2] = (ts >> 24) as u8;
        // uuid.0[3] = (ts >> 16) as u8;
        // uuid.0[4] = (ts >> 8) as u8;
        // uuid.0[5] = ts as u8;

        // version (4 bits) + counter[31:28] (4 bits)
        uuid.0[6] = 0x70 | ((counter >> 28) & 0x0F) as u8;
        // counter[27:20] (8 bits)
        uuid.0[7] = (counter >> 20) as u8;
        // variant (2 bits) + counter[19:14] (6 bits)
        uuid.0[8] = 0x80 | ((counter >> 14) & 0x3F) as u8;
        // counter[13:6] (8 bits)
        uuid.0[9] = (counter >> 6) as u8;
        // counter[5:0] (6 bits) -> preserve upper 2 random bits
        uuid.0[10] = (uuid.0[10] & 0xC0) | (counter & 0x3F) as u8;

        uuid
    }
}

impl Uuid {
    /// Parse a UUID from its canonical 8-4-4-4-12 hexadecimal string form.
    ///
    /// Accepts both lowercase and uppercase hex characters.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidUuid`] if the input is not exactly 36 characters,
    /// contains misplaced hyphens, or has non-hexadecimal characters.
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
        let bytes = &self.0;
        let mut buf = [0u8; 36];

        let groups: [(usize, usize); 5] = [(0, 8), (9, 13), (14, 18), (19, 23), (24, 36)];

        let mut src = 0;
        for (gi, &(start, end)) in groups.iter().enumerate() {
            let mut j = start;
            while j < end {
                let b = bytes[src];
                src += 1;
                buf[j] = hex::HEX_ENCODE[(b >> 4) as usize];
                buf[j + 1] = hex::HEX_ENCODE[(b & 0x0f) as usize];
                j += 2;
            }
            if gi < 4 {
                buf[end] = b'-';
            }
        }

        // SAFETY: `buf` is filled by writing only ASCII bytes from our
        // precomputed hex encoding table plus the hyphen byte `b'-'`,
        // all of which are valid UTF-8. The loop guarantees every position
        // is initialized before we reach this point.
        let s = unsafe { core::str::from_utf8_unchecked(&buf) };
        f.write_str(s)
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
    // clear the leftmost bit to reduce the chance of generating a counter near u32::MAX that
    // would trigger the spinlock
    rand::random::<u32>() & 0x7FFFFFFF
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
        assert_eq!(Uuid::parse(&uuid.to_string()).unwrap(), uuid);
    }

    #[test]
    fn max() {
        let uuid = Uuid::max();
        assert_eq!(uuid.version(), Version::Max);
        assert_eq!(uuid.to_string(), "ffffffff-ffff-ffff-ffff-ffffffffffff");
        assert_eq!(Uuid::from_bytes(uuid.as_bytes()), uuid);
        assert_eq!(Uuid::parse(&uuid.to_string()).unwrap(), uuid);
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
            // counter[5:0] -> byte 10 low 6 bits
            | (uuid.0[10] & 0x3F) as u32
    }
}
