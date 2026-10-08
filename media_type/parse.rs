//! Parsing of media types.

use alloc::string::String;
use core::{error::Error, fmt};

use super::{MediaType, Source};

/// An error returned when parsing a [`MediaType`] from a string.
///
/// The [`Display`](fmt::Display) implementation describes what went wrong and, when relevant, the
/// position of the offending byte.
#[derive(Debug)]
pub struct FromStrError {
    inner: ParseError,
}

impl FromStrError {
    pub(crate) fn new(inner: ParseError) -> Self {
        Self {
            inner,
        }
    }
}

impl fmt::Display for FromStrError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "media type parse error: {}", self.inner)
    }
}

impl Error for FromStrError {}

/// The reason a media type could not be parsed.
#[derive(Debug)]
pub(crate) enum ParseError {
    /// A `/` was missing between the type and the subtype.
    MissingSlash,
    /// An `=` was missing between a parameter name and its value.
    MissingEqual,
    /// A closing `"` was missing from a parameter value.
    MissingQuote,
    /// An invalid byte was encountered at the given position.
    InvalidToken {
        /// The byte offset of the invalid token.
        pos: usize,
        /// The invalid byte.
        byte: u8,
    },
}

impl ParseError {
    fn message(&self) -> &'static str {
        match self {
            ParseError::MissingSlash => "a slash (/) was missing between the type and subtype",
            ParseError::MissingEqual => "an equals sign (=) was missing between a parameter and its value",
            ParseError::MissingQuote => "a quote (\") was missing from a parameter value",
            ParseError::InvalidToken {
                ..
            } => "an invalid token was encountered",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidToken {
                pos,
                byte,
            } => {
                write!(formatter, "{}, {:X} at position {}", self.message(), byte, pos)
            }
            _ => formatter.write_str(self.message()),
        }
    }
}

impl Error for ParseError {}

/// An iterator over the media types found in a string.
///
/// This is useful to parse header fields such as `Accept` or `Content-Type`, which may contain a
/// comma-separated list of media types. Each item is either a parsed [`MediaType`] or the slice of
/// input that could not be parsed.
///
/// # Examples
///
/// ```
/// use media_type::MediaTypeIter;
///
/// let mut iter = MediaTypeIter::new("text/html, application/json, not-a-media-type, text/xml");
///
/// assert_eq!(iter.next().unwrap().unwrap(), media_type::TEXT_HTML);
/// assert_eq!(iter.next().unwrap().unwrap(), media_type::APPLICATION_JSON);
/// assert_eq!(iter.next().unwrap().unwrap_err(), "not-a-media-type");
/// assert_eq!(iter.next().unwrap().unwrap(), media_type::TEXT_XML);
/// assert!(iter.next().is_none());
/// ```
#[derive(Clone, Debug)]
pub struct MediaTypeIter<'a> {
    pos: usize,
    source: &'a str,
}

impl<'a> MediaTypeIter<'a> {
    /// Create a new iterator over the media types contained in `source`.
    pub fn new(source: &'a str) -> Self {
        Self {
            pos: 0,
            source,
        }
    }
}

impl<'a> Iterator for MediaTypeIter<'a> {
    type Item = Result<MediaType, &'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pos;
        let len = self.source.len();

        if start >= len {
            return None;
        }

        // Try parsing the whole remaining slice.
        match parse(&self.source[start..len]) {
            Ok(value) => {
                self.pos = len;
                Some(Ok(value))
            }
            Err(ParseError::InvalidToken {
                pos, ..
            }) => {
                // The first token is immediately wrong: skip it and retry.
                if pos == 0 {
                    self.pos += 1;
                    return self.next();
                }

                // Try parsing the longest slice up to the first invalid token.
                let slice = &self.source[start..start + pos];
                match parse(slice) {
                    Ok(media_type) => {
                        self.pos = start + pos + 1;
                        Some(Ok(media_type))
                    }
                    Err(_) => {
                        if start + pos < len {
                            // Report the invalid slice and continue after it.
                            self.pos = start + pos;
                            Some(Err(slice))
                        } else {
                            None
                        }
                    }
                }
            }
            // A missing character means the remaining slice is malformed: stop there.
            Err(_) => None,
        }
    }
}

/// Parse a media type.
///
/// The returned `MediaType` is stored in canonical form (lowercase type, subtype and parameter
/// names; `; ` separators; unquoted values).
pub(crate) fn parse(s: &str) -> Result<MediaType, ParseError> {
    if s == "*/*" {
        return Ok(MediaType::from_static("*/*"));
    }

    let bytes = s.as_bytes();
    let len = bytes.len();

    // Top-level type.
    let mut i = 0;
    loop {
        if i >= len {
            return Err(ParseError::MissingSlash);
        }
        let byte = bytes[i];
        if is_token(byte) {
            i += 1;
        } else if byte == b'/' && i > 0 {
            break;
        } else {
            return Err(ParseError::InvalidToken {
                pos: i,
                byte,
            });
        }
    }

    // Subtype.
    let subtype_start = i + 1;
    let semicolon;
    loop {
        if i + 1 >= len {
            // No parameters: the whole string is the media type.
            return Ok(MediaType {
                source: Source::Dynamic(s.to_ascii_lowercase()),
            });
        }
        i += 1;
        let byte = bytes[i];
        if byte == b';' && i > subtype_start {
            semicolon = i;
            break;
        } else if is_token(byte) {
            continue;
        } else {
            return Err(ParseError::InvalidToken {
                pos: i,
                byte,
            });
        }
    }

    // Parameters: start from the canonical essence.
    let mut out = String::with_capacity(len + 2);
    out.push_str(&s[..semicolon].to_ascii_lowercase());

    let mut i = semicolon;
    loop {
        // Consume the ';' and any optional whitespace.
        i += 1;
        while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
        if i >= len {
            break;
        }

        // Parameter name.
        let name_start = i;
        while i < len && is_token(bytes[i]) {
            i += 1;
        }
        if i == name_start {
            return Err(ParseError::InvalidToken {
                pos: i,
                byte: bytes.get(i).copied().unwrap_or(0),
            });
        }
        if i >= len || bytes[i] != b'=' {
            return Err(ParseError::MissingEqual);
        }
        let name_end = i;
        i += 1;

        // Parameter value: either a quoted-string or a token.
        let value;
        if i < len && bytes[i] == b'"' {
            i += 1;
            let value_start = i;
            while i < len && bytes[i] != b'"' {
                let byte = bytes[i];
                if byte < 0x20 || byte == 0x7f {
                    return Err(ParseError::InvalidToken {
                        pos: i,
                        byte,
                    });
                }
                i += 1;
            }
            if i >= len {
                return Err(ParseError::MissingQuote);
            }
            value = &s[value_start..i];
            i += 1;
        } else {
            let value_start = i;
            while i < len && is_token(bytes[i]) {
                i += 1;
            }
            value = &s[value_start..i];
        }

        let name = &s[name_start..name_end];
        out.push_str("; ");
        out.push_str(&name.to_ascii_lowercase());
        out.push('=');
        if name.eq_ignore_ascii_case(super::CHARSET) {
            out.push_str(&value.to_ascii_lowercase());
        } else {
            out.push_str(value);
        }

        // After the value, allow optional whitespace followed by a ';' or the end of the input.
        while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
        if i >= len {
            break;
        }
        if bytes[i] != b';' {
            return Err(ParseError::InvalidToken {
                pos: i,
                byte: bytes[i],
            });
        }
    }

    Ok(MediaType {
        source: Source::Dynamic(out),
    })
}

// From [RFC 6838](http://tools.ietf.org/html/rfc6838#section-4.2) and
// [RFC 7231](https://tools.ietf.org/html/rfc7231#section-3.1.1.1):
//
//     token  = 1*tchar
//     tchar  = "!" / "#" / "$" / "%" / "&" / "'" / "*" / "+" / "-" / "." /
//              "^" / "_" / "`" / "|" / "~" / DIGIT / ALPHA
macro_rules! byte_map {
    ($($flag:expr,)*) => {
        [$($flag != 0,)*]
    };
}

static TOKEN_MAP: [bool; 256] = byte_map![
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1,
    1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[inline]
fn is_token(byte: u8) -> bool {
    TOKEN_MAP[byte as usize]
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_lookup_tables() {
        for (i, &valid) in TOKEN_MAP.iter().enumerate() {
            let i = i as u8;
            let should = matches!(
                i,
                b'a'..=b'z'
                    | b'A'..=b'Z'
                    | b'0'..=b'9'
                    | b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'.'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'|'
                    | b'~'
            );
            assert_eq!(valid, should, "{:?} ({}) should be {}", i as char, i, should);
        }
    }

    #[test]
    fn test_parse_iterator() {
        let mut iter = MediaTypeIter::new("application/json, application/json");
        assert_eq!(iter.next().unwrap().unwrap(), "application/json");
        assert_eq!(iter.next().unwrap().unwrap(), "application/json");
        assert_eq!(iter.next(), None);

        let mut iter = MediaTypeIter::new("application/json");
        assert_eq!(iter.next().unwrap().unwrap(), "application/json");
        assert_eq!(iter.next(), None);

        let mut iter = MediaTypeIter::new("application/json;  ");
        assert_eq!(iter.next().unwrap().unwrap(), "application/json");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_parse_iterator_invalid() {
        let mut iter = MediaTypeIter::new("application/json, invalid, application/json");
        assert_eq!(iter.next().unwrap().unwrap(), "application/json");
        assert_eq!(iter.next().unwrap().unwrap_err(), "invalid");
        assert_eq!(iter.next().unwrap().unwrap(), "application/json");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_parse_iterator_leading_invalid() {
        let mut iter = MediaTypeIter::new(";;application/json");
        assert_eq!(iter.next().unwrap().unwrap(), "application/json");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_parse_canonical_form() {
        assert_eq!(
            "TEXT/PLAIN; CHARSET=UTF-8; FOO=BAR"
                .parse::<MediaType>()
                .unwrap()
                .to_string(),
            "text/plain; charset=utf-8; foo=BAR"
        );
        assert_eq!(
            "text/plain;charset=\"utf-8\"".parse::<MediaType>().unwrap(),
            super::super::TEXT_PLAIN_UTF_8
        );
    }

    #[test]
    fn test_parse_errors() {
        assert!("f o o / bar".parse::<MediaType>().is_err());
        assert!("text\n/plain".parse::<MediaType>().is_err());
        assert!("text\r/plain".parse::<MediaType>().is_err());
        assert!("text/\r\nplain".parse::<MediaType>().is_err());
        assert!("text/plain;\r\ncharset=utf-8".parse::<MediaType>().is_err());
        assert!("text/plain; charset=\r\nutf-8".parse::<MediaType>().is_err());
        assert!("text/plain; charset=\"\r\nutf-8\"".parse::<MediaType>().is_err());
    }

    #[test]
    fn test_from_static_and_from_str_are_equal() {
        let parsed = "application/json".parse::<MediaType>().unwrap();
        assert_eq!(parsed, super::super::APPLICATION_JSON);
    }
}
