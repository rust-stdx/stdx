use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Version {
    Http10,
    Http11,
    Http2,
    Http3,
}

impl Version {
    pub fn as_str(&self) -> &'static str {
        match self {
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
            Version::Http2 => "h2",
            Version::Http3 => "h3",
        }
    }

    pub fn as_alpn(&self) -> &'static [u8] {
        match self {
            Version::Http10 => b"http/1.0",
            Version::Http11 => b"http/1.1",
            Version::Http2 => b"h2",
            Version::Http3 => b"h3",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "HTTP/1.0" | "HTTP/1" => Some(Version::Http10),
            "HTTP/1.1" => Some(Version::Http11),
            "h2" => Some(Version::Http2),
            "h3" => Some(Version::Http3),
            _ => None,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_str() {
        assert_eq!(Version::Http11.as_str(), "HTTP/1.1");
        assert_eq!(Version::Http2.as_str(), "h2");
    }

    #[test]
    fn test_version_parse() {
        assert_eq!(Version::from_str("HTTP/1.1"), Some(Version::Http11));
        assert_eq!(Version::from_str("h2"), Some(Version::Http2));
        assert!(Version::from_str("unknown").is_none());
    }
}
