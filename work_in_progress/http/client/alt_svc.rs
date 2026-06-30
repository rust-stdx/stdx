use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::common::Response;

/// Parses the `Alt-Svc` response header (RFC 7838) and caches
/// discovered HTTP/3 endpoints.
///
/// Used by [`Client`](super::Client) to automatically upgrade to
/// HTTP/3 when the server advertises it via Alt-Svc.
pub(crate) struct AltSvcCache {
    /// Maps an authority `"host:port"` to a known HTTP/3 endpoint.
    entries: HashMap<String, AltSvcEntry>,
}

struct AltSvcEntry {
    /// The alt-authority from the header, e.g. `":443"` or `"other.example:443"`.
    alt_authority: String,
    expires_at: Instant,
}

impl AltSvcCache {
    pub fn new() -> Self {
        AltSvcCache {
            entries: HashMap::new(),
        }
    }

    /// Look up a cached HTTP/3 endpoint for `authority` (`"host:port"`).
    /// Returns `None` if the entry is expired or doesn't exist.
    pub fn get(&self, authority: &str) -> Option<&str> {
        let entry = self.entries.get(authority)?;
        if Instant::now() >= entry.expires_at {
            return None;
        }
        Some(&entry.alt_authority)
    }

    /// Insert or update a cached entry. `authority` is the original host:port,
    /// `alt_authority` is the advertised alternative (e.g. `":443"`),
    /// `max_age_secs` is from the `ma` parameter.
    pub fn insert(&mut self, authority: String, alt_authority: String, max_age_secs: u64) {
        let expires_at = Instant::now() + Duration::from_secs(max_age_secs.max(60));
        self.entries.insert(
            authority,
            AltSvcEntry {
                alt_authority,
                expires_at,
            },
        );
    }

    /// Remove a cached entry (used when an Alt-Svc endpoint fails, or on `clear`).
    pub fn remove(&mut self, authority: &str) {
        self.entries.remove(authority);
    }

    /// Clear all cached entries (used on `Alt-Svc: clear`).
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Parse the Alt-Svc header from a response.
/// Returns `Some((alt_authority, max_age))` if an `h3` entry is found,
/// or `None` if no h3 Alt-Svc is advertised.
pub(crate) fn parse_alt_svc_for_h3(resp: &Response, default_host: &str, default_port: u16) -> Option<(String, u64)> {
    let header_value = resp
        .headers
        .iter()
        .find(|(n, _)| n.as_str() == "alt-svc")
        .map(|(_, v)| v.as_str())?;

    if header_value.trim().eq_ignore_ascii_case("clear") {
        return Some(("__clear__".to_string(), 0));
    }

    for entry_str in header_value.split(',') {
        let entry_str = entry_str.trim();
        if entry_str.is_empty() {
            continue;
        }

        // Split protocol part from parameters
        let mut parts = entry_str.split(';');
        let protocol_part = parts.next()?.trim();

        let eq_pos = protocol_part.find('=')?;
        let protocol = protocol_part[..eq_pos].trim();
        if protocol != "h3" {
            continue;
        }

        let raw_authority = protocol_part[eq_pos + 1..].trim().trim_matches('"');

        // Parse max-age from parameters
        let mut max_age = 86400u64; // RFC default is 24h
        for param in parts {
            let param = param.trim();
            if let Some(rest) = param.strip_prefix("ma=") {
                max_age = rest.parse().unwrap_or(86400);
            }
        }

        // Resolve the alt-authority: if it starts with ':', use default host
        let resolved = if raw_authority.starts_with(':') {
            format!("{}{}", default_host, raw_authority)
        } else if !raw_authority.contains(':') {
            format!("{}:{}", raw_authority, default_port)
        } else {
            raw_authority.to_string()
        };

        return Some((resolved, max_age));
    }

    None
}

/// Parse an alt-authority into (host, port).
/// `alt_authority` is like `"example.com:443"` or `"example.com"`.
pub(crate) fn split_alt_authority(alt_authority: &str) -> (&str, u16) {
    if let Some(colon) = alt_authority.rfind(':') {
        let host = &alt_authority[..colon];
        let port_str = &alt_authority[colon + 1..];
        if let Ok(port) = port_str.parse::<u16>() {
            return (host, port);
        }
    }
    (alt_authority, 443)
}
