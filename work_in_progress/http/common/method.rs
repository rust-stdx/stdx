use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Custom(String),
}

impl Method {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        match bytes {
            b"GET" => Method::Get,
            b"HEAD" => Method::Head,
            b"POST" => Method::Post,
            b"PUT" => Method::Put,
            b"DELETE" => Method::Delete,
            b"CONNECT" => Method::Connect,
            b"OPTIONS" => Method::Options,
            b"TRACE" => Method::Trace,
            other => Method::Custom(String::from_utf8_lossy(other).to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Custom(s) => s.as_str(),
        }
    }

    pub fn is_safe(&self) -> bool {
        matches!(self, Method::Get | Method::Head | Method::Options | Method::Trace)
    }

    pub fn is_idempotent(&self) -> bool {
        matches!(
            self,
            Method::Get | Method::Head | Method::Put | Method::Delete | Method::Options | Method::Trace
        )
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        Method::from_bytes(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_methods() {
        assert_eq!(Method::from_bytes(b"GET"), Method::Get);
        assert_eq!(Method::from_bytes(b"POST"), Method::Post);
        assert_eq!(Method::from_bytes(b"DELETE"), Method::Delete);
    }

    #[test]
    fn test_custom_method() {
        assert_eq!(Method::from_bytes(b"PURGE"), Method::Custom("PURGE".into()));
    }

    #[test]
    fn test_safe_methods() {
        assert!(Method::Get.is_safe());
        assert!(Method::Head.is_safe());
        assert!(!Method::Post.is_safe());
    }

    #[test]
    fn test_idempotent() {
        assert!(Method::Put.is_idempotent());
        assert!(Method::Delete.is_idempotent());
        assert!(!Method::Post.is_idempotent());
    }

    #[test]
    fn test_display() {
        assert_eq!(Method::Get.to_string(), "GET");
        assert_eq!(Method::Custom("PURGE".into()).to_string(), "PURGE");
    }
}
