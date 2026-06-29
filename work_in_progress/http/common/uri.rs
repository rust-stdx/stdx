use std::fmt;

#[derive(Debug, Clone)]
pub struct Uri(String);

impl Uri {
    pub fn parse(input: &str) -> Option<Self> {
        if input.is_empty() {
            return None;
        }
        Some(Uri(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn scheme(&self) -> Option<&str> {
        if let Some(pos) = self.0.find("://") {
            Some(&self.0[..pos])
        } else {
            None
        }
    }

    pub fn authority(&self) -> Option<&str> {
        let after_scheme = self.0.find("://").map(|p| p + 3)?;
        let end = self.0[after_scheme..]
            .find('/')
            .map(|p| after_scheme + p)
            .unwrap_or(self.0.len());
        let end = self.0[after_scheme..end]
            .find('?')
            .map(|p| after_scheme + p)
            .unwrap_or(end);
        let auth = &self.0[after_scheme..end];
        if auth.is_empty() { None } else { Some(auth) }
    }

    pub fn host(&self) -> Option<&str> {
        let auth = self.authority()?;
        if let Some(pos) = auth.rfind(':') {
            Some(&auth[..pos])
        } else {
            Some(auth)
        }
    }

    pub fn port(&self) -> Option<u16> {
        let auth = self.authority()?;
        let pos = auth.rfind(':')?;
        auth[pos + 1..].parse().ok()
    }

    pub fn path(&self) -> &str {
        if let Some(pos) = self.0.find("://") {
            let after = pos + 3;
            let after_auth = self.0[after..].find('/').map(|p| after + p).unwrap_or(self.0.len());
            let end = self.0[after_auth..]
                .find('?')
                .map(|p| after_auth + p)
                .unwrap_or(self.0.len());
            if after_auth == self.0.len() || after_auth >= end {
                "/"
            } else {
                &self.0[after_auth..end]
            }
        } else {
            let end = self.0.find('?').unwrap_or(self.0.len());
            if self.0.is_empty() { "/" } else { &self.0[..end] }
        }
    }

    pub fn query(&self) -> Option<&str> {
        let pos = self.0.find('?')?;
        Some(&self.0[pos + 1..])
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Uri {
    fn from(s: String) -> Self {
        Uri(s)
    }
}

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Uri(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_only() {
        let u = Uri::parse("/hello").unwrap();
        assert_eq!(u.path(), "/hello");
        assert!(u.scheme().is_none());
        assert!(u.authority().is_none());
    }

    #[test]
    fn test_full_url() {
        let u = Uri::parse("https://example.com:443/path?q=1").unwrap();
        assert_eq!(u.scheme(), Some("https"));
        assert_eq!(u.authority(), Some("example.com:443"));
        assert_eq!(u.host(), Some("example.com"));
        assert_eq!(u.port(), Some(443));
        assert_eq!(u.path(), "/path");
        assert_eq!(u.query(), Some("q=1"));
    }

    #[test]
    fn test_url_no_port() {
        let u = Uri::parse("http://example.com/foo").unwrap();
        assert_eq!(u.authority(), Some("example.com"));
        assert_eq!(u.host(), Some("example.com"));
        assert!(u.port().is_none());
        assert_eq!(u.path(), "/foo");
    }

    #[test]
    fn test_url_no_path() {
        let u = Uri::parse("http://example.com").unwrap();
        assert_eq!(u.path(), "/");
    }

    #[test]
    fn test_empty_fails() {
        assert!(Uri::parse("").is_none());
    }
}
