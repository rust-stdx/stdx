use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderName(String);

impl HeaderName {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut s = String::from_utf8_lossy(bytes).into_owned();
        s.make_ascii_lowercase();
        HeaderName(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for HeaderName {
    fn from(s: &str) -> Self {
        HeaderName(s.to_ascii_lowercase())
    }
}

impl PartialEq<&str> for HeaderName {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

#[derive(Debug, Clone)]
pub struct HeaderValue(Vec<u8>);

impl HeaderValue {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let trimmed = trim_ows(bytes);
        HeaderValue(trimmed.to_vec())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or("")
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for HeaderValue {
    fn from(s: &str) -> Self {
        HeaderValue::from_bytes(s.as_bytes())
    }
}

impl From<String> for HeaderValue {
    fn from(s: String) -> Self {
        HeaderValue::from_bytes(s.as_bytes())
    }
}

impl PartialEq for HeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn trim_ows(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|b| *b != b' ' && *b != b'\t')
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|b| *b != b' ' && *b != b'\t')
        .map(|p| p + 1)
        .unwrap_or(0);
    &bytes[start..end]
}

pub type Headers = Vec<(HeaderName, HeaderValue)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_name_case_insensitive() {
        let a = HeaderName::from("Content-Type");
        let b = HeaderName::from("content-type");
        assert_eq!(a, b);
        assert_eq!(a.as_str(), "content-type");
    }

    #[test]
    fn test_header_value_trim() {
        let v = HeaderValue::from("  hello  ");
        assert_eq!(v.as_str(), "hello");
    }

    #[test]
    fn test_header_value_no_trim() {
        let v = HeaderValue::from("hello world");
        assert_eq!(v.as_str(), "hello world");
    }

    #[test]
    fn test_name_equality_with_str() {
        let n = HeaderName::from("Content-Type");
        assert!(n == "content-type");
        assert!(n == "CONTENT-TYPE");
    }
}
