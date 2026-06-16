#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A [Semantic Versioning 2.0.0] parser and comparator.
//!
//! # Usage
//!
//! ```
//! use semver::parse;
//!
//! let v = parse("1.2.3").unwrap();
//! assert_eq!(v.major, 1);
//! assert_eq!(v.minor, 2);
//! assert_eq!(v.patch, 3);
//!
//! let v = parse("1.0.0-alpha.1+build.123").unwrap();
//! assert_eq!(v.to_string(), "1.0.0-alpha.1+build.123");
//! ```
//!
//! Parsing follows the full SemVer 2.0.0 BNF grammar, including pre-release
//! identifiers with hyphens, leading-zero rejection on numeric identifiers,
//! and build metadata validation.
//!
//! # Version struct
//!
//! The [`Version`] struct borrows its pre-release and build-metadata strings
//! from the input, so no allocation is needed for parsing.
//!
//! # Precedence
//!
//! [`Version`] implements `PartialEq`, `Eq`, `PartialOrd`, and `Ord`
//! following the SemVer precedence rules:
//!
//! - Compare `major`, `minor`, `patch` numerically.
//! - A pre-release version has lower precedence than a normal version.
//! - Pre-release identifiers are compared left-to-right: numeric identifiers
//!   are compared numerically, alphanumeric identifiers are compared lexically
//!   (ASCII), and numeric always precedes alpha.
//! - A longer pre-release has higher precedence when all preceding identifiers
//!   are equal.
//! - Build metadata is ignored for equality and ordering.
//!
//! ```
//! use semver::parse;
//!
//! assert!(parse("1.0.0-alpha").unwrap() < parse("1.0.0").unwrap());
//! assert!(parse("1.0.0-beta.2").unwrap() < parse("1.0.0-beta.11").unwrap());
//! assert!(parse("1.0.0-1").unwrap() < parse("1.0.0-alpha").unwrap());
//! assert_eq!(parse("1.0.0+build1").unwrap(), parse("1.0.0+build2").unwrap());
//! ```
//!
//! # Serde support
//!
//! Enable the `serde` feature to serialize/deserialize [`Version`] as a string:
//!
//! ```toml
//! [dependencies]
//! semver = { path = "../semver", features = ["serde"] }
//! ```
//!
//! ```
//! # #[cfg(feature = "serde")] fn _serde_example() {
//! use serde_json;
//! use semver::parse;
//!
//! let v: semver::Version<'_> = serde_json::from_str("\"1.2.3-alpha+build\"").unwrap();
//! assert_eq!(v, parse("1.2.3-alpha+build").unwrap());
//! # }
//! ```
//!
//! # Error handling
//!
//! ```
//! use semver::parse;
//!
//! assert!(parse("01.2.3").is_err());
//! assert!(parse("1.0.0-").is_err());
//! assert!(parse("").is_err());
//! ```
//!
//! [Semantic Versioning 2.0.0]: https://semver.org/spec/v2.0.0.html

extern crate alloc;

#[cfg(feature = "serde")]
mod serde;

use alloc::vec::Vec;
use core::{cmp::Ordering, fmt};

/// Errors that can occur when parsing a SemVer version string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    InvalidCharacter,
    LeadingZero,
    EmptyIdentifier,
    InvalidNumber,
    InvalidFormat,
    TrailingData,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => f.write_str("empty input"),
            ParseError::InvalidCharacter => f.write_str("invalid character in version string"),
            ParseError::LeadingZero => f.write_str("numeric identifier contains leading zero"),
            ParseError::EmptyIdentifier => f.write_str("empty identifier"),
            ParseError::InvalidNumber => f.write_str("invalid numeric identifier"),
            ParseError::InvalidFormat => f.write_str("invalid version format"),
            ParseError::TrailingData => f.write_str("unexpected trailing data"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// A parsed pre-release identifier: either a numeric value or an alphanumeric string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Identifier<'a> {
    Numeric(u64),
    Alpha(&'a str),
}

impl<'a> Identifier<'a> {
    fn parse(input: &'a str) -> Result<Self, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyIdentifier);
        }
        for c in input.chars() {
            if !c.is_ascii_alphanumeric() && c != '-' {
                return Err(ParseError::InvalidCharacter);
            }
        }
        let all_digits = input.chars().all(|c| c.is_ascii_digit());
        if all_digits {
            if input.len() > 1 && input.starts_with('0') {
                return Err(ParseError::LeadingZero);
            }
            let n = input.parse::<u64>().map_err(|_| ParseError::InvalidNumber)?;
            Ok(Identifier::Numeric(n))
        } else {
            Ok(Identifier::Alpha(input))
        }
    }
}

impl<'a> PartialOrd for Identifier<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Identifier<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Identifier::Numeric(_), Identifier::Alpha(_)) => Ordering::Less,
            (Identifier::Alpha(_), Identifier::Numeric(_)) => Ordering::Greater,
            (Identifier::Numeric(a), Identifier::Numeric(b)) => a.cmp(b),
            (Identifier::Alpha(a), Identifier::Alpha(b)) => a.cmp(b),
        }
    }
}

/// The pre-release portion of a SemVer version (after the `-`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreRelease<'a> {
    identifiers: Vec<Identifier<'a>>,
}

impl<'a> PreRelease<'a> {
    fn parse(input: &'a str) -> Result<Self, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyIdentifier);
        }
        let identifiers = input.split('.').map(Identifier::parse).collect::<Result<Vec<_>, _>>()?;
        Ok(PreRelease {
            identifiers,
        })
    }

    pub fn identifiers(&self) -> &[Identifier<'a>] {
        &self.identifiers
    }
}

impl<'a> fmt::Display for PreRelease<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, ident) in self.identifiers.iter().enumerate() {
            if i > 0 {
                f.write_str(".")?;
            }
            match ident {
                Identifier::Numeric(n) => write!(f, "{n}"),
                Identifier::Alpha(s) => f.write_str(s),
            }?;
        }
        Ok(())
    }
}

impl<'a> PartialOrd for PreRelease<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for PreRelease<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.identifiers.iter().zip(other.identifiers.iter()) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        self.identifiers.len().cmp(&other.identifiers.len())
    }
}

/// The build metadata portion of a SemVer version (after the `+`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildMetadata<'a> {
    raw: &'a str,
}

impl<'a> BuildMetadata<'a> {
    fn parse(input: &'a str) -> Result<Self, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyIdentifier);
        }
        for part in input.split('.') {
            if part.is_empty() {
                return Err(ParseError::EmptyIdentifier);
            }
            for c in part.chars() {
                if !c.is_ascii_alphanumeric() && c != '-' {
                    return Err(ParseError::InvalidCharacter);
                }
            }
        }
        Ok(BuildMetadata {
            raw: input,
        })
    }

    pub fn as_str(&self) -> &'a str {
        self.raw
    }
}

impl<'a> fmt::Display for BuildMetadata<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.raw)
    }
}

/// A parsed SemVer version (Semantic Versioning 2.0.0).
///
/// ```
/// use semver::{parse, Version};
///
/// let v = parse("1.2.3-alpha.1+build.42").unwrap();
/// assert_eq!(v.major, 1);
/// assert_eq!(v.minor, 2);
/// assert_eq!(v.patch, 3);
/// assert_eq!(v.to_string(), "1.2.3-alpha.1+build.42");
/// ```
/// use semver::parse;
///
/// let a = parse("1.0.0+build1").unwrap();
/// let b = parse("1.0.0+build2").unwrap();
/// assert_eq!(a, b); // build metadata ignored
///
/// let a = parse("1.0.0-alpha").unwrap();
/// let b = parse("1.0.0").unwrap();
/// assert_ne!(a, b); // pre-release is significant
/// assert!(a < b);   // pre-release < release
/// ```
#[derive(Debug, Clone)]
pub struct Version<'a> {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre_release: Option<PreRelease<'a>>,
    pub build: Option<BuildMetadata<'a>>,
}

/// Parses a SemVer version string.
///
/// ```
/// use semver::parse;
///
/// let v = parse("1.0.0").unwrap();
/// assert_eq!(v.major, 1);
/// assert_eq!(v.to_string(), "1.0.0");
///
/// let v = parse("1.0.0-alpha.1+build.123").unwrap();
/// assert_eq!(v.to_string(), "1.0.0-alpha.1+build.123");
///
/// assert!(parse("01.2.3").is_err());
/// assert!(parse("").is_err());
/// ```
pub fn parse(input: &str) -> Result<Version<'_>, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let after_major = check_digits(input)?;
    let (major_str, rest) = input.split_at(after_major);
    let major = parse_number(major_str)?;
    let rest = expect_dot(rest)?;

    let after_minor = check_digits(rest)?;
    let (minor_str, rest) = rest.split_at(after_minor);
    let minor = parse_number(minor_str)?;
    let rest = expect_dot(rest)?;

    let after_patch = check_digits(rest)?;
    let (patch_str, rest) = rest.split_at(after_patch);
    let patch = parse_number(patch_str)?;

    let (pre_release, after_pre_release) = match rest.strip_prefix('-') {
        Some(r) => {
            let end = r.find('+').unwrap_or(r.len());
            let pre_str = &r[..end];
            (Some(PreRelease::parse(pre_str)?), &r[end..])
        }
        None => (None, rest),
    };

    let build = match after_pre_release.strip_prefix('+') {
        Some(r) => Some(BuildMetadata::parse(r)?),
        None => {
            if !after_pre_release.is_empty() {
                return Err(ParseError::TrailingData);
            }
            None
        }
    };

    Ok(Version {
        major,
        minor,
        patch,
        pre_release,
        build,
    })
}

fn check_digits(s: &str) -> Result<usize, ParseError> {
    if s.is_empty() {
        return Err(ParseError::InvalidCharacter);
    }
    match s.find(|c: char| !c.is_ascii_digit()) {
        Some(0) => Err(ParseError::InvalidCharacter),
        Some(pos) => Ok(pos),
        None => Ok(s.len()),
    }
}

fn parse_number(s: &str) -> Result<u64, ParseError> {
    if s.len() > 1 && s.starts_with('0') {
        return Err(ParseError::LeadingZero);
    }
    s.parse::<u64>().map_err(|_| ParseError::InvalidNumber)
}

fn expect_dot(s: &str) -> Result<&str, ParseError> {
    match s.as_bytes().first() {
        Some(b'.') => Ok(&s[1..]),
        _ => Err(ParseError::InvalidFormat),
    }
}

impl<'a> fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre_release) = self.pre_release {
            write!(f, "-{pre_release}")?;
        }
        if let Some(ref build) = self.build {
            write!(f, "+{build}")?;
        }
        Ok(())
    }
}

impl<'a> PartialEq for Version<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.pre_release == other.pre_release
    }
}

impl<'a> Eq for Version<'a> {}

impl<'a> PartialOrd for Version<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Version<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match self.minor.cmp(&other.minor) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match self.patch.cmp(&other.patch) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match (&self.pre_release, &other.pre_release) {
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (None, None) => Ordering::Equal,
            (Some(a), Some(b)) => a.cmp(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec};

    use super::*;

    #[test]
    fn parse_basic() {
        let v = parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert!(v.pre_release.is_none());
        assert!(v.build.is_none());
    }

    #[test]
    fn parse_zero() {
        let v = parse("0.0.0").unwrap();
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);
    }

    #[test]
    fn parse_large_numbers() {
        let v = parse("999999999999999999.0.0").unwrap();
        assert_eq!(v.major, 999999999999999999);
    }

    #[test]
    fn parse_prerelease() {
        let v = parse("1.0.0-alpha").unwrap();
        assert_eq!(v.pre_release.unwrap().identifiers(), &[Identifier::Alpha("alpha")]);
    }

    #[test]
    fn parse_prerelease_dotted() {
        let v = parse("1.0.0-alpha.1").unwrap();
        assert_eq!(
            v.pre_release.as_ref().unwrap().identifiers(),
            &[Identifier::Alpha("alpha"), Identifier::Numeric(1)]
        );
    }

    #[test]
    fn parse_prerelease_numeric() {
        let v = parse("1.0.0-1.2.3").unwrap();
        assert_eq!(
            v.pre_release.as_ref().unwrap().identifiers(),
            &[Identifier::Numeric(1), Identifier::Numeric(2), Identifier::Numeric(3)]
        );
    }

    #[test]
    fn parse_build_metadata() {
        let v = parse("1.0.0+001").unwrap();
        assert_eq!(v.build.unwrap().as_str(), "001");
    }

    #[test]
    fn parse_prerelease_and_build() {
        let v = parse("1.0.0-alpha.1+build.123").unwrap();
        assert!(v.pre_release.is_some());
        assert_eq!(v.build.unwrap().as_str(), "build.123");
    }

    #[test]
    fn parse_complex_prerelease() {
        let v = parse("1.0.0-0.3.7").unwrap();
        assert_eq!(
            v.pre_release.as_ref().unwrap().identifiers(),
            &[Identifier::Numeric(0), Identifier::Numeric(3), Identifier::Numeric(7)]
        );
    }

    #[test]
    fn parse_prerelease_with_hyphens() {
        let v = parse("1.0.0-x-y-z.--").unwrap();
        let idents = v.pre_release.unwrap().identifiers().to_vec();
        assert_eq!(idents.len(), 2);
        assert_eq!(idents[0], Identifier::Alpha("x-y-z"));
        assert_eq!(idents[1], Identifier::Alpha("--"));
    }

    #[test]
    fn parse_build_with_dots() {
        let v = parse("1.0.0+21AF26D3----117B344092BD").unwrap();
        assert_eq!(v.build.unwrap().as_str(), "21AF26D3----117B344092BD");
    }

    #[test]
    fn parse_build_with_multiple_dots() {
        let v = parse("1.0.0+20130313144700").unwrap();
        assert_eq!(v.build.unwrap().as_str(), "20130313144700");
    }

    #[test]
    fn parse_build_after_prerelease() {
        let v = parse("1.0.0-beta+exp.sha.5114f85").unwrap();
        assert_eq!(v.build.unwrap().as_str(), "exp.sha.5114f85");
    }

    #[test]
    fn error_empty_input() {
        assert_eq!(parse(""), Err(ParseError::EmptyInput));
    }

    #[test]
    fn error_leading_zero_major() {
        assert_eq!(parse("01.2.3"), Err(ParseError::LeadingZero));
    }

    #[test]
    fn error_leading_zero_minor() {
        assert_eq!(parse("1.02.3"), Err(ParseError::LeadingZero));
    }

    #[test]
    fn error_leading_zero_patch() {
        assert_eq!(parse("1.2.03"), Err(ParseError::LeadingZero));
    }

    #[test]
    fn error_leading_zero_in_prerelease() {
        assert_eq!(parse("1.0.0-01"), Err(ParseError::LeadingZero));
    }

    #[test]
    fn error_invalid_char() {
        assert!(parse("1.2.3!").is_err());
    }

    #[test]
    fn error_invalid_char_in_prerelease() {
        assert!(parse("1.0.0-alpha$").is_err());
    }

    #[test]
    fn error_empty_prerelease() {
        assert_eq!(parse("1.0.0-"), Err(ParseError::EmptyIdentifier));
    }

    #[test]
    fn error_empty_build() {
        assert_eq!(parse("1.0.0+"), Err(ParseError::EmptyIdentifier));
    }

    #[test]
    fn error_missing_dot() {
        assert_eq!(parse("1.2"), Err(ParseError::InvalidFormat));
    }

    #[test]
    fn error_non_digit_major() {
        assert_eq!(parse("a.2.3"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn display_basic() {
        assert_eq!(parse("1.2.3").unwrap().to_string(), "1.2.3");
    }

    #[test]
    fn display_with_prerelease() {
        assert_eq!(parse("1.0.0-alpha.1").unwrap().to_string(), "1.0.0-alpha.1");
    }

    #[test]
    fn display_with_build() {
        assert_eq!(parse("1.0.0+build.42").unwrap().to_string(), "1.0.0+build.42");
    }

    #[test]
    fn display_with_both() {
        assert_eq!(parse("1.0.0-rc.2+sha.abc123").unwrap().to_string(), "1.0.0-rc.2+sha.abc123");
    }

    #[test]
    fn eq_ignores_build_metadata() {
        let a = parse("1.0.0+build1").unwrap();
        let b = parse("1.0.0+build2").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn eq_respects_prerelease() {
        let a = parse("1.0.0-alpha").unwrap();
        let b = parse("1.0.0-beta").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn precedence_spec_examples() {
        // 1.0.0-alpha < 2.0.0 < 2.1.0 < 2.1.1
        let versions: Vec<Version> = vec!["1.0.0-alpha", "2.0.0", "2.1.0", "2.1.1"]
            .into_iter()
            .map(|s| parse(s).unwrap())
            .collect();
        for i in 0..versions.len() - 1 {
            assert!(versions[i] < versions[i + 1], "{} < {}", versions[i], versions[i + 1]);
        }
    }

    #[test]
    fn precedence_prerelease_vs_release() {
        assert!(parse("1.0.0-alpha").unwrap() < parse("1.0.0").unwrap());
    }

    #[test]
    fn precedence_complex_prerelease() {
        let spec_order = [
            "1.0.0-alpha",
            "1.0.0-alpha.1",
            "1.0.0-alpha.beta",
            "1.0.0-beta",
            "1.0.0-beta.2",
            "1.0.0-beta.11",
            "1.0.0-rc.1",
            "1.0.0",
        ];
        let versions: Vec<Version> = spec_order.iter().map(|s| parse(s).unwrap()).collect();
        for i in 0..versions.len() - 1 {
            assert!(
                versions[i] < versions[i + 1],
                "expected {} < {}",
                spec_order[i],
                spec_order[i + 1]
            );
        }
    }

    #[test]
    fn precedence_build_ignored() {
        assert_eq!(parse("1.0.0+1").unwrap().cmp(&parse("1.0.0+2").unwrap()), Ordering::Equal);
    }

    #[test]
    fn precedence_numeric_before_alpha() {
        let a = parse("1.0.0-1").unwrap();
        let b = parse("1.0.0-alpha").unwrap();
        assert!(a < b);
    }

    // ===== Error precision =====

    #[test]
    fn error_leading_non_digit_major() {
        assert_eq!(parse("a.0.0"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn error_leading_non_digit_minor() {
        assert_eq!(parse("1.a.0"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn error_leading_non_digit_patch() {
        assert_eq!(parse("1.0.a"), Err(ParseError::InvalidCharacter));
    }

    // ===== Empty identifier edge cases =====

    #[test]
    fn error_empty_identifier_middle_prerelease() {
        assert_eq!(parse("1.0.0-alpha..1"), Err(ParseError::EmptyIdentifier));
    }

    #[test]
    fn error_empty_identifier_middle_build() {
        assert_eq!(parse("1.0.0+a..b"), Err(ParseError::EmptyIdentifier));
    }

    #[test]
    fn error_trailing_dot_build() {
        assert_eq!(parse("1.0.0+build."), Err(ParseError::EmptyIdentifier));
    }

    #[test]
    fn parse_prerelease_double_dash() {
        // first `-` is the version/prerelease separator, second starts the identifier
        let v = parse("1.0.0--alpha").unwrap();
        assert_eq!(v.pre_release.as_ref().unwrap().identifiers(), &[Identifier::Alpha("-alpha")]);
    }

    // ===== Trailing / invalid characters =====

    #[test]
    fn error_trailing_dot_after_patch() {
        assert_eq!(parse("1.2.3."), Err(ParseError::TrailingData));
    }

    #[test]
    fn error_dot_only_prerelease() {
        assert_eq!(parse("1.0.0-."), Err(ParseError::EmptyIdentifier));
    }

    #[test]
    fn error_trailing_plus_with_prerelease() {
        assert_eq!(parse("1.0.0-alpha+"), Err(ParseError::EmptyIdentifier));
    }

    #[test]
    fn error_space_in_prerelease() {
        assert_eq!(parse("1.0.0-alpha beta"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn error_unicode_in_prerelease() {
        assert_eq!(parse("1.0.0-α"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn error_space_in_build() {
        assert_eq!(parse("1.0.0+build extra"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn error_double_plus_build() {
        assert_eq!(parse("1.0.0+build+extra"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn error_unicode_in_build() {
        assert_eq!(parse("1.0.0+é"), Err(ParseError::InvalidCharacter));
    }

    #[test]
    fn error_prerelease_numeric_overflow() {
        assert_eq!(parse("1.0.0-18446744073709551616"), Err(ParseError::InvalidNumber));
    }

    // ===== Valid edge case parsing =====

    #[test]
    fn parse_prerelease_alpha_starting_with_zero() {
        let v = parse("1.0.0-0abc").unwrap();
        assert_eq!(v.pre_release.as_ref().unwrap().identifiers(), &[Identifier::Alpha("0abc")]);
    }

    #[test]
    fn parse_prerelease_hyphens_only() {
        let v = parse("1.0.0-----").unwrap();
        assert_eq!(v.pre_release.as_ref().unwrap().identifiers(), &[Identifier::Alpha("----")]);
    }

    #[test]
    fn parse_prerelease_single_zero() {
        let v = parse("1.0.0-0").unwrap();
        assert_eq!(v.pre_release.as_ref().unwrap().identifiers(), &[Identifier::Numeric(0)]);
    }

    #[test]
    fn parse_prerelease_with_plus_split() {
        let v = parse("1.0.0-alpha+beta").unwrap();
        assert_eq!(v.pre_release.as_ref().unwrap().identifiers(), &[Identifier::Alpha("alpha")]);
        assert_eq!(v.build.unwrap().as_str(), "beta");
    }

    #[test]
    fn parse_u64_max_major() {
        let v = parse("18446744073709551615.0.0").unwrap();
        assert_eq!(v.major, u64::MAX);
    }

    #[test]
    fn parse_build_with_hyphen() {
        let v = parse("1.0.0+build-id").unwrap();
        assert_eq!(v.build.unwrap().as_str(), "build-id");
    }

    // ===== Precedence / ordering =====

    #[test]
    fn precedence_prerelease_length_tiebreak() {
        assert!(parse("1.0.0-1").unwrap() < parse("1.0.0-1.0").unwrap());
    }

    #[test]
    fn precedence_prerelease_length_tiebreak_alpha() {
        assert!(parse("1.0.0-alpha").unwrap() < parse("1.0.0-alpha.0").unwrap());
    }

    #[test]
    fn precedence_prerelease_zero_length() {
        assert!(parse("1.0.0-0").unwrap() < parse("1.0.0-0.0").unwrap());
    }

    #[test]
    fn precedence_alpha_case_ascii() {
        assert!(parse("1.0.0-A").unwrap() < parse("1.0.0-a").unwrap());
    }

    #[test]
    fn precedence_hyphen_in_alpha() {
        assert!(parse("1.0.0--a").unwrap() < parse("1.0.0-a").unwrap());
    }

    // ===== Display / round-trip =====

    #[test]
    fn display_roundtrip() {
        let inputs = [
            "1.2.3",
            "0.0.0",
            "1.0.0-alpha",
            "1.0.0-alpha.1",
            "1.0.0-0.3.7",
            "1.0.0-x-y-z.--",
            "1.0.0+001",
            "1.0.0+21AF26D3----117B344092BD",
            "1.0.0+20130313144700",
            "1.0.0-beta+exp.sha.5114f85",
            "1.0.0-rc.2+sha.abc123",
            "1.0.0----",
            "1.0.0-0abc",
            "1.0.0+build-id",
        ];
        for input in &inputs {
            let v = parse(input).unwrap();
            assert_eq!(v.to_string(), *input, "round-trip failed for: {input}");
        }
    }

    #[test]
    fn display_direct_construction() {
        let v = Version {
            major: 2,
            minor: 5,
            patch: 1,
            pre_release: Some(PreRelease {
                identifiers: vec![Identifier::Alpha("rc"), Identifier::Numeric(3)],
            }),
            build: Some(BuildMetadata {
                raw: "sha.abc",
            }),
        };
        assert_eq!(v.to_string(), "2.5.1-rc.3+sha.abc");
    }

    // ===== Direct construction equality/ordering =====

    #[test]
    fn direct_version_eq() {
        let a = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            build: None,
        };
        let b = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            build: Some(BuildMetadata {
                raw: "x",
            }),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn direct_version_ord() {
        let a = Version {
            major: 1,
            minor: 0,
            patch: 0,
            pre_release: None,
            build: None,
        };
        let b = Version {
            major: 1,
            minor: 0,
            patch: 0,
            pre_release: Some(PreRelease {
                identifiers: vec![Identifier::Alpha("alpha")],
            }),
            build: None,
        };
        assert!(b < a);
    }

    // ===== Identifier ordering unit tests =====

    #[test]
    fn identifier_cmp_numeric_vs_alpha() {
        assert!(Identifier::Numeric(1) < Identifier::Alpha("a"));
        assert!(Identifier::Alpha("a") > Identifier::Numeric(1));
    }

    #[test]
    fn identifier_cmp_numeric_values() {
        assert!(Identifier::Numeric(1) < Identifier::Numeric(2));
        assert!(Identifier::Numeric(5) == Identifier::Numeric(5));
    }

    #[test]
    fn identifier_cmp_alpha_values() {
        assert!(Identifier::Alpha("alpha") < Identifier::Alpha("beta"));
        assert!(Identifier::Alpha("a") < Identifier::Alpha("b"));
        assert!(Identifier::Alpha("--") < Identifier::Alpha("a"));
    }

    // ===== PreRelease ordering unit tests =====

    #[test]
    fn prerelease_cmp_length() {
        let a = PreRelease {
            identifiers: vec![Identifier::Numeric(1)],
        };
        let b = PreRelease {
            identifiers: vec![Identifier::Numeric(1), Identifier::Numeric(0)],
        };
        assert!(a < b);

        let c = PreRelease {
            identifiers: vec![Identifier::Alpha("alpha")],
        };
        let d = PreRelease {
            identifiers: vec![Identifier::Alpha("alpha"), Identifier::Numeric(1)],
        };
        assert!(c < d);
    }

    // ===== Serde tests =====

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        use serde_json;
        let v = parse("1.2.3-alpha+build").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let v2: Version = serde_json::from_str(&json).unwrap();
        assert_eq!(v, v2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_invalid_errors() {
        use serde_json;
        let result: Result<Version, _> = serde_json::from_str("\"01.2.3\"");
        assert!(result.is_err());
    }
}
