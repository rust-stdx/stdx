#![no_std]

//! Fast integer-to-string conversion with a stack-allocated buffer.
//!
//! # Example
//!
//! ```rust
//! // Reusable buffer
//! let mut buf = format_number::Buffer::new();
//! assert_eq!(buf.format(42u64), "42");
//! assert_eq!(buf.format(-99i32), "-99");
//!
//! // One-shot convenience
//! let buf = format_number::format(2024);
//! assert!(buf == "2024");
//! ```

use core::{ops, ptr, str};

const MAX_LEN: usize = 40;

const DEC_DIGITS_LUT: &[u8; 200] = b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

/// Stack-allocated buffer for integer-to-string conversion.
///
/// The buffer is always exactly large enough to hold any integer
/// (`i128::MIN` = 40 characters).
///
/// # Example
///
/// ```rust
/// let mut buf = format_number::Buffer::new();
/// let s: &str = buf.format(1234);
/// assert_eq!(s, "1234");
/// ```
pub struct Buffer {
    buf: [u8; MAX_LEN],
    start: u8,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    /// Creates a new empty buffer.
    #[inline]
    pub const fn new() -> Self {
        Buffer {
            buf: [0u8; MAX_LEN],
            start: MAX_LEN as u8,
        }
    }

    /// Formats an integer into this buffer and returns a `&str` view of the
    /// result.
    ///
    /// The returned reference borrows from `self` and is valid until the next
    /// call to [`format`](Buffer::format) or the buffer is dropped.
    #[inline]
    pub fn format<I: Integer>(&mut self, n: I) -> &str {
        n.format_into(self);
        self.as_str()
    }

    /// Returns the formatted string.
    ///
    /// # Safety
    ///
    /// The returned `&str` is valid as long as the buffer is not mutated.
    /// All formatting functions only write ASCII digits and `'-'`.
    #[inline]
    pub fn as_str(&self) -> &str {
        let s = &self.buf[self.start as usize..];
        unsafe { str::from_utf8_unchecked(s) }
    }
}

impl Clone for Buffer {
    #[inline]
    fn clone(&self) -> Self {
        Buffer {
            buf: self.buf,
            start: self.start,
        }
    }
}

impl ops::Deref for Buffer {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Buffer {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for Buffer {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<Buffer> for Buffer {
    #[inline]
    fn eq(&self, other: &Buffer) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Buffer {}

/// One-shot convenience: formats an integer into a new [`Buffer`] and returns it.
///
/// ```rust
/// let buf = format_number::format(42);
/// assert!(buf == "42");
/// println!("{}", buf);
/// let s: &str = &buf;
/// ```
#[inline]
pub fn format<I: Integer>(n: I) -> Buffer {
    let mut buf = Buffer::new();
    buf.format(n);
    buf
}

// ---------------------------------------------------------------------------
// Sealed Integer trait
// ---------------------------------------------------------------------------

/// An integer type that can be formatted into a [`Buffer`].
///
/// This trait is sealed — it cannot be implemented outside of this crate.
pub trait Integer: private::Sealed {}

mod private {
    use super::Buffer;

    pub trait Sealed: Copy {
        fn format_into(self, buf: &mut Buffer);
    }
}

// ---------------------------------------------------------------------------
// Internal formatting helpers
// ---------------------------------------------------------------------------

#[inline]
fn format_u64<const FOUR_DIGIT: bool>(mut n: u64, buf: &mut [u8; MAX_LEN], mut pos: usize) -> usize {
    let buf_ptr = buf.as_mut_ptr();
    let lut_ptr = DEC_DIGITS_LUT.as_ptr();

    if FOUR_DIGIT {
        while n >= 10000 {
            let rem = (n % 10000) as usize;
            n /= 10000;

            let d1 = (rem / 100) * 2;
            let d2 = (rem % 100) * 2;
            pos -= 4;
            unsafe {
                ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(pos), 2);
                ptr::copy_nonoverlapping(lut_ptr.add(d2), buf_ptr.add(pos + 2), 2);
            }
        }
    }

    if n >= 100 {
        let d1 = (n % 100) as usize * 2;
        n /= 100;
        pos -= 2;
        unsafe {
            ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(pos), 2);
        }
    }

    if n < 10 {
        pos -= 1;
        unsafe {
            *buf_ptr.add(pos) = (n as u8) + b'0';
        }
    } else {
        let d1 = n as usize * 2;
        pos -= 2;
        unsafe {
            ptr::copy_nonoverlapping(lut_ptr.add(d1), buf_ptr.add(pos), 2);
        }
    }

    pos
}

/// Computes the upper 128 bits of a 128×128 multiplication.
#[inline]
fn u128_mulhi(x: u128, y: u128) -> u128 {
    let x_lo = x as u64;
    let x_hi = (x >> 64) as u64;
    let y_lo = y as u64;
    let y_hi = (y >> 64) as u64;

    let carry = (x_lo as u128 * y_lo as u128) >> 64;
    let m = x_lo as u128 * y_hi as u128 + carry;
    let high1 = m >> 64;

    let m_lo = m as u64;
    let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

    x_hi as u128 * y_hi as u128 + high1 + high2
}

/// Divides `n` by `10^19` and returns `(quotient, remainder)`.
///
/// Uses the Granlund–Montgomery algorithm for fast division by a constant.
#[inline]
fn udivmod_1e19(n: u128) -> (u128, u64) {
    const D: u64 = 10_000_000_000_000_000_000;

    let quot = if n < 1 << 83 {
        ((n >> 19) as u64 / (D >> 19)) as u128
    } else {
        u128_mulhi(n, 156927543384667019095894735580191660403) >> 62
    };

    let rem = (n - quot * D as u128) as u64;
    debug_assert_eq!(quot, n / D as u128);
    debug_assert_eq!(rem as u128, n % D as u128);

    (quot, rem)
}

#[inline]
fn format_u128(n: u128, buf: &mut [u8; MAX_LEN], mut pos: usize) -> usize {
    let buf_ptr = buf.as_mut_ptr();

    let (n, lo) = udivmod_1e19(n);
    pos = format_u64::<true>(lo, buf, pos);

    if n != 0 {
        let target = MAX_LEN - 19;
        unsafe {
            ptr::write_bytes(buf_ptr.add(target), b'0', pos - target);
        }
        pos = target;

        let (n, mid) = udivmod_1e19(n);
        pos = format_u64::<true>(mid, buf, pos);

        if n != 0 {
            let target = MAX_LEN - 38;
            unsafe {
                ptr::write_bytes(buf_ptr.add(target), b'0', pos - target);
            }
            pos = target;
            pos -= 1;
            unsafe {
                *buf_ptr.add(pos) = b'0' + n as u8;
            }
        }
    }

    pos
}

// ---------------------------------------------------------------------------
// Trait implementations for each integer type
// ---------------------------------------------------------------------------

macro_rules! impl_unsigned {
    ($t:ty, $four:literal) => {
        impl Integer for $t {}

        impl private::Sealed for $t {
            #[inline]
            fn format_into(self, buf: &mut Buffer) {
                buf.start = format_u64::<$four>(self as u64, &mut buf.buf, MAX_LEN) as u8;
            }
        }
    };
}

macro_rules! impl_signed {
    ($t:ty, $u:ty, $four:literal) => {
        impl Integer for $t {}

        impl private::Sealed for $t {
            #[inline]
            fn format_into(self, buf: &mut Buffer) {
                let is_neg = self < 0;
                let n: $u = if is_neg {
                    // unsigned_abs() correctly handles MIN (which has no positive counterpart).
                    self.unsigned_abs()
                } else {
                    self as $u
                };
                buf.start = format_u64::<$four>(n as u64, &mut buf.buf, MAX_LEN) as u8;
                if is_neg {
                    buf.start -= 1;
                    buf.buf[buf.start as usize] = b'-';
                }
            }
        }
    };
}

impl_unsigned!(u8, false);
impl_unsigned!(u16, true);
impl_unsigned!(u32, true);
impl_unsigned!(u64, true);

impl_signed!(i8, u8, false);
impl_signed!(i16, u16, true);
impl_signed!(i32, u32, true);
impl_signed!(i64, u64, true);

macro_rules! impl_signed_ptr {
    ($t:ty, $u:ty, $four:literal) => {
        impl Integer for $t {}

        impl private::Sealed for $t {
            #[inline]
            fn format_into(self, buf: &mut Buffer) {
                let is_neg = self < 0;
                let n = if is_neg {
                    self.unsigned_abs()
                } else {
                    self as usize
                };
                buf.start = format_u64::<$four>(n as u64, &mut buf.buf, MAX_LEN) as u8;
                if is_neg {
                    buf.start -= 1;
                    buf.buf[buf.start as usize] = b'-';
                }
            }
        }
    };
}

impl Integer for u128 {}

impl private::Sealed for u128 {
    #[inline]
    fn format_into(self, buf: &mut Buffer) {
        buf.start = format_u128(self, &mut buf.buf, MAX_LEN) as u8;
    }
}

impl Integer for i128 {}

impl private::Sealed for i128 {
    #[inline]
    fn format_into(self, buf: &mut Buffer) {
        let is_neg = self < 0;
        let n: u128 = if is_neg { self.unsigned_abs() } else { self as u128 };
        buf.start = format_u128(n, &mut buf.buf, MAX_LEN) as u8;
        if is_neg {
            buf.start -= 1;
            buf.buf[buf.start as usize] = b'-';
        }
    }
}

#[cfg(target_pointer_width = "16")]
impl_unsigned!(usize, true);
#[cfg(target_pointer_width = "32")]
impl_unsigned!(usize, true);
#[cfg(target_pointer_width = "64")]
impl_unsigned!(usize, true);

#[cfg(target_pointer_width = "16")]
impl_signed_ptr!(isize, u16, true);
#[cfg(target_pointer_width = "32")]
impl_signed_ptr!(isize, u32, true);
#[cfg(target_pointer_width = "64")]
impl_signed_ptr!(isize, u64, true);

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_u64() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(0u64), "0");
        assert_eq!(buf.format(1u64), "1");
        assert_eq!(buf.format(9u64), "9");
        assert_eq!(buf.format(10u64), "10");
        assert_eq!(buf.format(42u64), "42");
        assert_eq!(buf.format(99u64), "99");
        assert_eq!(buf.format(100u64), "100");
        assert_eq!(buf.format(999u64), "999");
        assert_eq!(buf.format(1000u64), "1000");
        assert_eq!(buf.format(9999u64), "9999");
        assert_eq!(buf.format(10000u64), "10000");
        assert_eq!(buf.format(12345u64), "12345");
        assert_eq!(buf.format(65535u64), "65535");
        assert_eq!(buf.format(u64::MAX), "18446744073709551615");
    }

    #[test]
    fn basic_i64() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(0i64), "0");
        assert_eq!(buf.format(-1i64), "-1");
        assert_eq!(buf.format(1i64), "1");
        assert_eq!(buf.format(-42i64), "-42");
        assert_eq!(buf.format(i64::MAX), "9223372036854775807");
        assert_eq!(buf.format(i64::MIN), "-9223372036854775808");
    }

    #[test]
    fn basic_u8() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(0u8), "0");
        assert_eq!(buf.format(255u8), "255");
        assert_eq!(buf.format(u8::MAX), "255");
    }

    #[test]
    fn basic_i8() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(0i8), "0");
        assert_eq!(buf.format(127i8), "127");
        assert_eq!(buf.format(-128i8), "-128");
    }

    #[test]
    fn basic_u16() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(u16::MAX), "65535");
    }

    #[test]
    fn basic_i16() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(i16::MAX), "32767");
        assert_eq!(buf.format(i16::MIN), "-32768");
    }

    #[test]
    fn basic_u32() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(u32::MAX), "4294967295");
    }

    #[test]
    fn basic_i32() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(i32::MAX), "2147483647");
        assert_eq!(buf.format(i32::MIN), "-2147483648");
    }

    #[test]
    fn basic_u128() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(0u128), "0");
        assert_eq!(buf.format(1u128), "1");
        assert_eq!(buf.format(10u128), "10");
        assert_eq!(buf.format(100u128), "100");
        assert_eq!(buf.format(1000u128), "1000");
        assert_eq!(buf.format(10000u128), "10000");
        assert_eq!(buf.format(u128::MAX), "340282366920938463463374607431768211455");
    }

    #[test]
    fn basic_i128() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(0i128), "0");
        assert_eq!(buf.format(-1i128), "-1");
        assert_eq!(buf.format(i128::MAX), "170141183460469231731687303715884105727");
        assert_eq!(buf.format(i128::MIN), "-170141183460469231731687303715884105728");
    }

    #[test]
    fn powers_of_ten() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(1u64), "1");
        assert_eq!(buf.format(10u64), "10");
        assert_eq!(buf.format(100u64), "100");
        assert_eq!(buf.format(1000u64), "1000");
        assert_eq!(buf.format(10000u64), "10000");
        assert_eq!(buf.format(100000u64), "100000");
        assert_eq!(buf.format(1000000u64), "1000000");
        assert_eq!(buf.format(10000000u64), "10000000");
        assert_eq!(buf.format(100000000u64), "100000000");
        assert_eq!(buf.format(1000000000u64), "1000000000");
        assert_eq!(buf.format(10000000000u64), "10000000000");
        assert_eq!(buf.format(100000000000u64), "100000000000");
        assert_eq!(buf.format(1000000000000u64), "1000000000000");
        assert_eq!(buf.format(10000000000000u64), "10000000000000");
        assert_eq!(buf.format(100000000000000u64), "100000000000000");
        assert_eq!(buf.format(1000000000000000u64), "1000000000000000");
        assert_eq!(buf.format(10000000000000000u64), "10000000000000000");
        assert_eq!(buf.format(100000000000000000u64), "100000000000000000");
        assert_eq!(buf.format(1000000000000000000u64), "1000000000000000000");
        assert_eq!(buf.format(10000000000000000000u64), "10000000000000000000");
    }

    #[test]
    fn u128_edge_cases() {
        let mut buf = Buffer::new();

        // 19 nines (right at 10^19 boundary)
        assert_eq!(buf.format(9999999999999999999u128), "9999999999999999999");

        // 10^19 exactly
        assert_eq!(buf.format(10000000000000000000u128), "10000000000000000000");

        // 10^19 + 1
        assert_eq!(buf.format(10000000000000000001u128), "10000000000000000001");

        // 20 digits
        assert_eq!(buf.format(99999999999999999999u128), "99999999999999999999");

        // 10^20
        assert_eq!(buf.format(100000000000000000000u128), "100000000000000000000");

        // 38 nines
        assert_eq!(
            buf.format(99999999999999999999999999999999999999u128),
            "99999999999999999999999999999999999999"
        );
    }

    #[test]
    fn oneshot() {
        assert!(format(42u64) == "42");
        assert!(format(-99i32) == "-99");
        assert!(format(0u8) == "0");
        assert!(format(255u8) == "255");
    }

    #[test]
    fn deref_to_str() {
        let mut buf = Buffer::new();
        buf.format(123u64);
        let s: &str = &buf;
        assert_eq!(s, "123");
    }

    #[test]
    fn as_ref() {
        let mut buf = Buffer::new();
        buf.format(456u64);
        assert_eq!(buf.as_ref(), "456");
    }

    #[test]
    fn partial_eq_str() {
        let mut buf = Buffer::new();
        buf.format(789u64);
        assert_eq!(buf.as_str(), "789");
        assert_ne!(buf.as_str(), "000");
    }

    #[test]
    fn partial_eq_buffer() {
        let mut a = Buffer::new();
        let mut b = Buffer::new();
        a.format(42u64);
        b.format(42u64);
        assert_eq!(a.as_str(), b.as_str());
        b.format(99u64);
        assert_ne!(a.as_str(), b.as_str());
    }

    #[test]
    fn clone() {
        let mut a = Buffer::new();
        a.format(42u64);
        let b = a.clone();
        assert_eq!(a.as_str(), b.as_str());
    }

    #[test]
    fn const_new() {
        const BUF: Buffer = Buffer::new();
        assert_eq!(BUF.as_str(), "");
    }

    #[test]
    fn reuse() {
        let mut buf = Buffer::new();
        assert_eq!(buf.format(1u64), "1");
        assert_eq!(buf.format(12u64), "12");
        assert_eq!(buf.format(123u64), "123");
        assert_eq!(buf.format(1234u64), "1234");
        assert_eq!(buf.format(12345u64), "12345");
    }
}
