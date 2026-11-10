//! JSON number scanning, parsing and representation.

use core::fmt;

use crate::error::{Error, ErrorKind, Result};

/// A JSON number.
///
/// Integers that fit in `u64` or `i64` are stored exactly. Other integers are
/// stored as `f64` when `Options::big_numbers` is [`BigNumberPolicy::Float`],
/// matching the behaviour of `serde_json`; use [`crate::RawNumber`] when the
/// exact decimal representation is required.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    /// A non-negative integer.
    PosInt(u64),
    /// A negative integer.
    NegInt(i64),
    /// A finite floating-point number.
    Float(f64),
}

impl Number {
    /// Creates a number from an `f64`. Returns `None` for NaN and infinities.
    #[inline]
    pub fn from_f64(value: f64) -> Option<Self> {
        if value.is_finite() {
            Some(Number::Float(value))
        } else {
            None
        }
    }

    /// Returns the value as a `u64` if it is a non-negative integer.
    #[inline]
    pub const fn as_u64(self) -> Option<u64> {
        match self {
            Number::PosInt(value) => Some(value),
            Number::NegInt(value) => {
                if value >= 0 {
                    Some(value as u64)
                } else {
                    None
                }
            }
            Number::Float(_) => None,
        }
    }

    /// Returns the value as an `i64` if it is an integer.
    #[inline]
    pub const fn as_i64(self) -> Option<i64> {
        match self {
            Number::PosInt(value) => {
                if value <= i64::MAX as u64 {
                    Some(value as i64)
                } else {
                    None
                }
            }
            Number::NegInt(value) => Some(value),
            Number::Float(_) => None,
        }
    }

    /// Returns the value as an `f64`.
    #[inline]
    pub fn as_f64(self) -> f64 {
        match self {
            Number::PosInt(value) => value as f64,
            Number::NegInt(value) => value as f64,
            Number::Float(value) => value,
        }
    }

    /// Returns the value as a `u128` if it is a non-negative integer.
    #[inline]
    pub const fn as_u128(self) -> Option<u128> {
        match self {
            Number::PosInt(value) => Some(value as u128),
            Number::NegInt(value) => {
                if value >= 0 {
                    Some(value as u128)
                } else {
                    None
                }
            }
            Number::Float(_) => None,
        }
    }

    /// Returns the value as an `i128` if it is an integer.
    #[inline]
    pub const fn as_i128(self) -> Option<i128> {
        match self {
            Number::PosInt(value) => Some(value as i128),
            Number::NegInt(value) => Some(value as i128),
            Number::Float(_) => None,
        }
    }

    /// Returns `true` if this is a floating-point number.
    #[inline]
    pub const fn is_f64(self) -> bool {
        matches!(self, Number::Float(_))
    }

    /// Returns `true` if this is an integer.
    #[inline]
    pub const fn is_integer(self) -> bool {
        matches!(self, Number::PosInt(_) | Number::NegInt(_))
    }

    pub(crate) fn parse(text: &str, big: BigNumberPolicy, is_float: bool, offset: usize) -> Result<Self> {
        if is_float {
            let value = parse_f64(text, offset)?;
            return Ok(Number::Float(value));
        }

        if let Some(rest) = text.strip_prefix('-') {
            match parse_u64_digits(rest) {
                // `-0` must preserve the sign, like `serde_json`. An `i64`
                // cannot represent negative zero, so use a float.
                Ok(0) => Ok(Number::Float(-0.0)),
                Ok(magnitude) if magnitude <= i64::MAX as u64 => Ok(Number::NegInt(-(magnitude as i64))),
                Ok(magnitude) if magnitude == (i64::MAX as u64) + 1 => Ok(Number::NegInt(i64::MIN)),
                Ok(_) => match big {
                    BigNumberPolicy::Float => Ok(Number::Float(parse_f64(text, offset)?)),
                    BigNumberPolicy::Error => Err(Error::new(ErrorKind::NumberOutOfRange, offset)),
                },
                Err(ErrorKind::NumberOutOfRange) => match big {
                    BigNumberPolicy::Float => Ok(Number::Float(parse_f64(text, offset)?)),
                    BigNumberPolicy::Error => Err(Error::new(ErrorKind::NumberOutOfRange, offset)),
                },
                Err(kind) => Err(Error::new(kind, offset)),
            }
        } else {
            match parse_u64_digits(text) {
                Ok(value) => Ok(Number::PosInt(value)),
                Err(ErrorKind::NumberOutOfRange) => match big {
                    BigNumberPolicy::Float => Ok(Number::Float(parse_f64(text, offset)?)),
                    BigNumberPolicy::Error => Err(Error::new(ErrorKind::NumberOutOfRange, offset)),
                },
                Err(kind) => Err(Error::new(kind, offset)),
            }
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Number::PosInt(value) => write!(f, "{value}"),
            Number::NegInt(value) => write!(f, "{value}"),
            Number::Float(value) => {
                let mut buffer = zmij::Buffer::new();
                f.write_str(buffer.format_finite(value))
            }
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Number::PosInt(a), Number::PosInt(b)) => a == b,
            (Number::NegInt(a), Number::NegInt(b)) => a == b,
            (Number::Float(a), Number::Float(b)) => a == b,
            _ => false,
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::PosInt(value as u64)
    }
}
impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::PosInt(value as u64)
    }
}
impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::PosInt(value as u64)
    }
}
impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::PosInt(value)
    }
}
impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Number::PosInt(value as u64)
    }
}
impl From<i8> for Number {
    fn from(value: i8) -> Self {
        if value >= 0 {
            Number::PosInt(value as u64)
        } else {
            Number::NegInt(value as i64)
        }
    }
}
impl From<i16> for Number {
    fn from(value: i16) -> Self {
        if value >= 0 {
            Number::PosInt(value as u64)
        } else {
            Number::NegInt(value as i64)
        }
    }
}
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        if value >= 0 {
            Number::PosInt(value as u64)
        } else {
            Number::NegInt(value as i64)
        }
    }
}
impl From<i64> for Number {
    fn from(value: i64) -> Self {
        if value >= 0 {
            Number::PosInt(value as u64)
        } else {
            Number::NegInt(value)
        }
    }
}
impl From<isize> for Number {
    fn from(value: isize) -> Self {
        if value >= 0 {
            Number::PosInt(value as u64)
        } else {
            Number::NegInt(value as i64)
        }
    }
}
impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(value as f64)
    }
}
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

#[inline]
fn is_8digits(value: u64) -> bool {
    let a = value.wrapping_add(0x4646_4646_4646_4646);
    let b = value.wrapping_sub(0x3030_3030_3030_3030);
    (a | b) & 0x8080_8080_8080_8080 == 0
}

#[inline]
fn parse_8digits(mut value: u64) -> u64 {
    const MASK: u64 = 0x0000_00FF_0000_00FF;
    const MUL1: u64 = 0x000F_4240_0000_0064;
    const MUL2: u64 = 0x0000_2710_0000_0001;
    value -= 0x3030_3030_3030_3030;
    value = (value * 10) + (value >> 8);
    let v1 = (value & MASK).wrapping_mul(MUL1);
    let v2 = ((value >> 16) & MASK).wrapping_mul(MUL2);
    ((v1.wrapping_add(v2) >> 32) as u32) as u64
}

#[inline]
fn read_8digits(input: &[u8], i: &mut usize) -> Option<u64> {
    if *i + 8 > input.len() {
        return None;
    }
    let word = u64::from_le_bytes(input[*i..*i + 8].try_into().unwrap());
    if !is_8digits(word) {
        return None;
    }
    *i += 8;
    Some(parse_8digits(word))
}

/// Scans a JSON number starting at `start`.
///
/// Returns the end offset and whether the number contains a fraction or
/// exponent.
#[inline]
pub(crate) fn scan(input: &[u8], start: usize) -> Result<(usize, bool)> {
    let mut i = start;
    if input.get(i) == Some(&b'-') {
        i += 1;
    }

    match input.get(i).copied() {
        Some(b'0') => {
            i += 1;
            if matches!(input.get(i), Some(b'0'..=b'9')) {
                return Err(Error::new(ErrorKind::InvalidNumber, start));
            }
        }
        Some(b'1'..=b'9') => {
            i += 1;
            while read_8digits(input, &mut i).is_some() {}
            while matches!(input.get(i), Some(b'0'..=b'9')) {
                i += 1;
            }
        }
        _ => return Err(Error::new(ErrorKind::InvalidNumber, start)),
    }

    let mut is_float = false;
    if input.get(i) == Some(&b'.') {
        is_float = true;
        i += 1;
        if !matches!(input.get(i), Some(b'0'..=b'9')) {
            return Err(Error::new(ErrorKind::InvalidNumber, start));
        }
        while read_8digits(input, &mut i).is_some() {}
        while matches!(input.get(i), Some(b'0'..=b'9')) {
            i += 1;
        }
    }

    if matches!(input.get(i), Some(b'e') | Some(b'E')) {
        is_float = true;
        i += 1;
        if matches!(input.get(i), Some(b'+') | Some(b'-')) {
            i += 1;
        }
        if !matches!(input.get(i), Some(b'0'..=b'9')) {
            return Err(Error::new(ErrorKind::InvalidNumber, start));
        }
        while read_8digits(input, &mut i).is_some() {}
        while matches!(input.get(i), Some(b'0'..=b'9')) {
            i += 1;
        }
    }

    Ok((i, is_float))
}

const POW10_FAST: [f64; 23] = [
    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20,
    1e21, 1e22,
];

struct Fused {
    end: usize,
    is_float: bool,
    negative: bool,
    magnitude: u64,
    fallback: bool,
    explicit_exponent: i32,
    fraction_digits: i32,
}

#[inline]
fn scan_fused(input: &[u8], start: usize) -> Result<Fused> {
    let mut i = start;
    let negative = input.get(i) == Some(&b'-');
    if negative {
        i += 1;
    }

    let mut magnitude = 0u64;
    let mut digits = 0u32;
    let mut fallback = false;

    macro_rules! push_digit {
        ($byte:expr) => {{
            if !fallback {
                if digits == 19 {
                    fallback = true;
                } else {
                    // At most 18 prior digits, so `* 10 + digit` fits in u64.
                    magnitude = magnitude * 10 + ($byte - b'0') as u64;
                }
            }
            digits = digits.saturating_add(1);
        }};
    }

    match input.get(i).copied() {
        Some(b'0') => {
            i += 1;
            if matches!(input.get(i), Some(b'0'..=b'9')) {
                return Err(Error::new(ErrorKind::InvalidNumber, start));
            }
        }
        Some(b'1'..=b'9') => {
            loop {
                if !fallback
                    && digits + 8 <= 19
                    && let Some(chunk) = read_8digits(input, &mut i)
                {
                    // At most 11 prior digits, so this fits in u64.
                    magnitude = magnitude * 100_000_000 + chunk;
                    digits += 8;
                    continue;
                }
                break;
            }
            while let Some(&byte @ b'0'..=b'9') = input.get(i) {
                push_digit!(byte);
                i += 1;
            }
        }
        _ => return Err(Error::new(ErrorKind::InvalidNumber, start)),
    }

    let mut is_float = false;
    let mut fraction_digits = 0i32;
    if input.get(i) == Some(&b'.') {
        is_float = true;
        i += 1;
        if !matches!(input.get(i), Some(b'0'..=b'9')) {
            return Err(Error::new(ErrorKind::InvalidNumber, start));
        }
        loop {
            if !fallback
                && digits + 8 <= 19
                && let Some(chunk) = read_8digits(input, &mut i)
            {
                magnitude = magnitude * 100_000_000 + chunk;
                digits += 8;
                fraction_digits += 8;
                continue;
            }
            break;
        }
        while let Some(&byte @ b'0'..=b'9') = input.get(i) {
            push_digit!(byte);
            fraction_digits = fraction_digits.saturating_add(1);
            if fraction_digits > 10_000 {
                fallback = true;
            }
            i += 1;
        }
    }

    let mut explicit_exponent = 0i32;
    if matches!(input.get(i), Some(b'e') | Some(b'E')) {
        is_float = true;
        i += 1;
        let exponent_negative = match input.get(i) {
            Some(b'+') => {
                i += 1;
                false
            }
            Some(b'-') => {
                i += 1;
                true
            }
            _ => false,
        };
        if !matches!(input.get(i), Some(b'0'..=b'9')) {
            return Err(Error::new(ErrorKind::InvalidNumber, start));
        }
        let mut exponent = 0i32;
        while let Some(&byte @ b'0'..=b'9') = input.get(i) {
            if !fallback {
                match exponent
                    .checked_mul(10)
                    .and_then(|value| value.checked_add((byte - b'0') as i32))
                {
                    Some(value) if value <= 10_000 => exponent = value,
                    _ => fallback = true,
                }
            }
            i += 1;
        }
        explicit_exponent = if exponent_negative { -exponent } else { exponent };
    }

    Ok(Fused {
        end: i,
        is_float,
        negative,
        magnitude,
        fallback,
        explicit_exponent,
        fraction_digits,
    })
}

#[inline]
fn fused_f64(fused: &Fused) -> Option<f64> {
    if fused.fallback {
        return None;
    }
    let exponent = fused.explicit_exponent.checked_sub(fused.fraction_digits)?;
    let mut value = if fused.magnitude <= (1u64 << 53) && (-22..=22).contains(&exponent) {
        let mut value = fused.magnitude as f64;
        if exponent > 0 {
            value *= POW10_FAST[exponent as usize];
        } else if exponent < 0 {
            value /= POW10_FAST[(-exponent) as usize];
        }
        value
    } else {
        let fp = crate::float_lemire::compute_float(exponent as i64, fused.magnitude);
        if fp.p_biased < 0 {
            return None;
        }
        f64::from_bits(fp.m | ((fp.p_biased as u64) << 52))
    };
    if !value.is_finite() {
        return None;
    }
    if fused.negative {
        value = -value;
    }
    Some(value)
}

#[allow(unsafe_code)]
#[inline]
pub(crate) fn parse_f64_at(input: &[u8], start: usize) -> Result<(f64, usize)> {
    let fused = scan_fused(input, start)?;
    let text = unsafe { core::str::from_utf8_unchecked(&input[start..fused.end]) };
    let value = if let Some(value) = fused_f64(&fused) {
        value
    } else {
        return Ok((parse_f64(text, start)?, fused.end));
    };
    Ok((value, fused.end))
}

#[allow(unsafe_code)]
#[inline]
pub(crate) fn parse_number_at(input: &[u8], start: usize, big: BigNumberPolicy) -> Result<(Number, usize)> {
    let fused = scan_fused(input, start)?;
    let text = unsafe { core::str::from_utf8_unchecked(&input[start..fused.end]) };

    if !fused.is_float {
        if !fused.fallback {
            if !fused.negative {
                return Ok((Number::PosInt(fused.magnitude), fused.end));
            }
            if fused.magnitude == 0 {
                return Ok((Number::Float(-0.0), fused.end));
            }
            if fused.magnitude <= i64::MAX as u64 {
                return Ok((Number::NegInt(-(fused.magnitude as i64)), fused.end));
            }
            if fused.magnitude == (i64::MAX as u64) + 1 {
                return Ok((Number::NegInt(i64::MIN), fused.end));
            }
        }
        return Ok((Number::parse(text, big, false, start)?, fused.end));
    }

    if let Some(value) = fused_f64(&fused) {
        Ok((Number::Float(value), fused.end))
    } else {
        Ok((Number::Float(parse_f64(text, start)?), fused.end))
    }
}

/// Scans and parses a non-negative integer in one pass.
#[inline]
pub(crate) fn parse_u64_fast(input: &[u8], start: usize) -> Result<(u64, usize)> {
    let mut i = start;
    let mut value = 0u64;
    match input.get(i).copied() {
        Some(b'0') => {
            i += 1;
            if matches!(input.get(i), Some(b'0'..=b'9')) {
                return Err(Error::new(ErrorKind::InvalidNumber, start));
            }
        }
        Some(byte @ b'1'..=b'9') => {
            value = (byte - b'0') as u64;
            i += 1;
            while let Some(chunk) = read_8digits(input, &mut i) {
                value = value
                    .checked_mul(100_000_000)
                    .and_then(|value| value.checked_add(chunk))
                    .ok_or_else(|| Error::new(ErrorKind::NumberOutOfRange, start))?;
            }
            while let Some(&byte @ b'0'..=b'9') = input.get(i) {
                let digit = (byte - b'0') as u64;
                value = value
                    .checked_mul(10)
                    .and_then(|value| value.checked_add(digit))
                    .ok_or_else(|| Error::new(ErrorKind::NumberOutOfRange, start))?;
                i += 1;
            }
        }
        _ => return Err(Error::new(ErrorKind::InvalidNumber, start)),
    }
    if matches!(input.get(i), Some(b'.') | Some(b'e') | Some(b'E')) {
        return Err(Error::new(ErrorKind::NumberOutOfRange, start));
    }
    Ok((value, i))
}

/// Scans and parses a signed integer in one pass.
pub(crate) fn parse_i64_fast(input: &[u8], start: usize) -> Result<(i64, usize)> {
    let (negative, digits_start) = match input.get(start) {
        Some(b'-') => (true, start + 1),
        _ => (false, start),
    };
    let (magnitude, end) = parse_u64_fast(input, digits_start)?;
    if negative {
        if magnitude > (i64::MAX as u64) + 1 {
            return Err(Error::new(ErrorKind::NumberOutOfRange, start));
        }
        if magnitude == (i64::MAX as u64) + 1 {
            Ok((i64::MIN, end))
        } else {
            Ok((-((magnitude) as i64), end))
        }
    } else {
        if magnitude > i64::MAX as u64 {
            return Err(Error::new(ErrorKind::NumberOutOfRange, start));
        }
        Ok((magnitude as i64, end))
    }
}

pub(crate) fn parse_u64(text: &str, offset: usize) -> Result<u64> {
    parse_u64_digits(text).map_err(|kind| Error::new(kind, offset))
}

#[inline]
pub(crate) fn parse_i64(text: &str, offset: usize) -> Result<i64> {
    match text.strip_prefix('-') {
        Some(rest) => {
            let magnitude = parse_u64_digits(rest).map_err(|kind| Error::new(kind, offset))?;
            if magnitude > (i64::MAX as u64) + 1 {
                return Err(Error::new(ErrorKind::NumberOutOfRange, offset));
            }
            if magnitude == (i64::MAX as u64) + 1 {
                Ok(i64::MIN)
            } else {
                Ok(-(magnitude as i64))
            }
        }
        None => {
            let value = parse_u64_digits(text).map_err(|kind| Error::new(kind, offset))?;
            if value > i64::MAX as u64 {
                return Err(Error::new(ErrorKind::NumberOutOfRange, offset));
            }
            Ok(value as i64)
        }
    }
}

#[inline]
pub(crate) fn parse_u128(text: &str, offset: usize) -> Result<u128> {
    let mut value = 0u128;
    if text.is_empty() {
        return Err(Error::new(ErrorKind::InvalidNumber, offset));
    }
    for byte in text.bytes() {
        if !byte.is_ascii_digit() {
            return Err(Error::new(ErrorKind::InvalidNumber, offset));
        }
        value = value
            .checked_mul(10)
            .and_then(|value| value.checked_add((byte - b'0') as u128))
            .ok_or_else(|| Error::new(ErrorKind::NumberOutOfRange, offset))?;
    }
    Ok(value)
}

#[inline]
pub(crate) fn parse_i128(text: &str, offset: usize) -> Result<i128> {
    match text.strip_prefix('-') {
        Some(rest) => {
            let magnitude = parse_u128(rest, offset)?;
            if magnitude > (i128::MAX as u128) + 1 {
                return Err(Error::new(ErrorKind::NumberOutOfRange, offset));
            }
            if magnitude == (i128::MAX as u128) + 1 {
                Ok(i128::MIN)
            } else {
                Ok(-(magnitude as i128))
            }
        }
        None => Ok(parse_u128(text, offset)? as i128),
    }
}

#[inline]
#[allow(dead_code)]
pub(crate) fn parse_f32(text: &str, offset: usize) -> Result<f32> {
    let value = parse_f64(text, offset)?;
    let value = value as f32;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::new(ErrorKind::NumberOutOfRange, offset))
    }
}

#[inline]
pub(crate) fn parse_f64(text: &str, offset: usize) -> Result<f64> {
    if let Some(value) = parse_f64_fast(text) {
        return Ok(value);
    }
    let value: f64 = text.parse().map_err(|_| Error::new(ErrorKind::InvalidNumber, offset))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::new(ErrorKind::NumberOutOfRange, offset))
    }
}

/// Fast path for the common case of at most 15 significant digits and a
/// decimal exponent in `[-22, 22]`.
///
/// Both the significand and `10^exp` are exactly representable in `f64` in
/// this range, so the single IEEE multiplication/division is correctly
/// rounded. Anything outside the range falls back to the standard library.
fn parse_f64_fast(text: &str) -> Option<f64> {
    // Long inputs often need the exact fallback anyway; skip the fast-path
    // setup instead of scanning twice.
    if text.len() > 16 {
        return None;
    }
    const POW10: [f64; 23] = [
        1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19,
        1e20, 1e21, 1e22,
    ];

    let bytes = text.as_bytes();
    let mut i = 0usize;
    let negative = bytes.first() == Some(&b'-');
    if negative {
        i = 1;
    }

    let mut significand = 0u64;
    let mut fraction_digits = 0i32;
    let mut seen_digit = false;

    while let Some(&byte) = bytes.get(i) {
        if !byte.is_ascii_digit() {
            break;
        }
        if significand > (u64::MAX - 9) / 10 {
            return None;
        }
        significand = significand * 10 + (byte - b'0') as u64;
        seen_digit = true;
        i += 1;
    }
    if !seen_digit {
        return None;
    }

    if bytes.get(i) == Some(&b'.') {
        i += 1;
        while let Some(&byte) = bytes.get(i) {
            if !byte.is_ascii_digit() {
                break;
            }
            if significand > (u64::MAX - 9) / 10 {
                return None;
            }
            significand = significand * 10 + (byte - b'0') as u64;
            fraction_digits = fraction_digits.checked_add(1)?;
            i += 1;
        }
    }

    let mut explicit_exponent = 0i32;
    if matches!(bytes.get(i), Some(b'e') | Some(b'E')) {
        i += 1;
        let exponent_negative = match bytes.get(i) {
            Some(b'+') => {
                i += 1;
                false
            }
            Some(b'-') => {
                i += 1;
                true
            }
            _ => false,
        };
        let mut saw_exponent_digit = false;
        while let Some(&byte) = bytes.get(i) {
            if !byte.is_ascii_digit() {
                break;
            }
            explicit_exponent = explicit_exponent.checked_mul(10)?.checked_add((byte - b'0') as i32)?;
            if explicit_exponent > 10_000 {
                return None;
            }
            saw_exponent_digit = true;
            i += 1;
        }
        if !saw_exponent_digit {
            return None;
        }
        if exponent_negative {
            explicit_exponent = -explicit_exponent;
        }
    }

    if i != bytes.len() {
        return None;
    }
    if significand > (1u64 << 53) {
        return None;
    }
    let exponent = explicit_exponent.checked_sub(fraction_digits)?;
    if !(-22..=22).contains(&exponent) {
        return None;
    }

    let mut value = significand as f64;
    if exponent > 0 {
        value *= POW10[exponent as usize];
    } else if exponent < 0 {
        value /= POW10[(-exponent) as usize];
    }
    if !value.is_finite() {
        return None;
    }
    if negative {
        value = -value;
    }
    Some(value)
}

fn parse_u64_digits(text: &str) -> core::result::Result<u64, ErrorKind> {
    let mut value = 0u64;
    if text.is_empty() {
        return Err(ErrorKind::InvalidNumber);
    }
    for byte in text.bytes() {
        if !byte.is_ascii_digit() {
            return Err(ErrorKind::InvalidNumber);
        }
        value = value
            .checked_mul(10)
            .and_then(|value| value.checked_add((byte - b'0') as u64))
            .ok_or(ErrorKind::NumberOutOfRange)?;
    }
    Ok(value)
}

/// Policy for integers that do not fit in `u64`/`i64`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BigNumberPolicy {
    /// Parse the value as `f64`, which may lose precision. This matches
    /// `serde_json`'s default behaviour and is the default here.
    #[default]
    Float,
    /// Reject the value with [`ErrorKind::NumberOutOfRange`].
    Error,
}
