use crate::common::Version;

/// Determine whether the connection should be kept alive after the current message.
pub fn should_keep_alive(version: Version, close_requested: bool) -> bool {
    match version {
        Version::Http11 => !close_requested,
        Version::Http10 => false,
        _ => false,
    }
}

/// Check if the "Connection" header contains "close".
pub fn is_close_requested(headers: &[(impl AsRef<str>, impl AsRef<str>)]) -> bool {
    for (name, value) in headers {
        if name.as_ref().eq_ignore_ascii_case("connection") {
            if value.as_ref().eq_ignore_ascii_case("close") {
                return true;
            }
        }
    }
    false
}

/// Check if the "Connection" header contains "upgrade".
pub fn is_upgrade_requested(headers: &[(impl AsRef<str>, impl AsRef<str>)]) -> bool {
    for (name, value) in headers {
        if name.as_ref().eq_ignore_ascii_case("connection") {
            if value.as_ref().to_ascii_lowercase().contains("upgrade") {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_alive_http11_default() {
        assert!(should_keep_alive(Version::Http11, false));
    }

    #[test]
    fn test_keep_alive_http11_close() {
        assert!(!should_keep_alive(Version::Http11, true));
    }

    #[test]
    fn test_close_not_kept_alive() {
        assert!(!should_keep_alive(Version::Http10, false));
    }

    #[test]
    fn test_is_close_requested() {
        let headers = vec![("connection", "close")];
        assert!(is_close_requested(&headers));
    }

    #[test]
    fn test_is_close_not_requested() {
        let headers = vec![("connection", "keep-alive")];
        assert!(!is_close_requested(&headers));
    }

    #[test]
    fn test_is_upgrade_requested() {
        let headers = vec![("connection", "upgrade")];
        assert!(is_upgrade_requested(&headers));
    }

    #[test]
    fn test_upgrade_mixed() {
        let headers = vec![("connection", "keep-alive, upgrade")];
        assert!(is_upgrade_requested(&headers));
    }
}
