use std::fmt;

use crate::common::{HeaderName, HeaderValue, Headers, Method, Request, Response, StatusCode, Uri, Version};

#[derive(Debug)]
pub enum H1Error {
    Incomplete,
    BadRequestLine(String),
    BadStatusLine(String),
    BadHeader(String),
    BadBody(String),
    BodyLengthMismatch { expected: usize, actual: usize },
}

impl fmt::Display for H1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            H1Error::Incomplete => write!(f, "incomplete HTTP/1.1 message"),
            H1Error::BadRequestLine(s) => write!(f, "bad request line: {}", s),
            H1Error::BadStatusLine(s) => write!(f, "bad status line: {}", s),
            H1Error::BadHeader(s) => write!(f, "bad header: {}", s),
            H1Error::BadBody(s) => write!(f, "bad body: {}", s),
            H1Error::BodyLengthMismatch {
                expected,
                actual,
            } => {
                write!(f, "body length mismatch: expected {}, got {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for H1Error {}

fn find_line(data: &[u8], start: usize) -> Option<(usize, usize)> {
    let mut i = start;
    while i < data.len() {
        if data[i] == b'\r' && i + 1 < data.len() && data[i + 1] == b'\n' {
            return Some((i, i + 2));
        }
        if data[i] == b'\n' {
            return Some((i, i + 1));
        }
        i += 1;
    }
    None
}

fn trim_end(data: &[u8]) -> &[u8] {
    let mut end = data.len();
    while end > 0 && (data[end - 1] == b'\r' || data[end - 1] == b'\n') {
        end -= 1;
    }
    &data[..end]
}

pub fn decode_request(data: &[u8]) -> Result<(Request, usize), H1Error> {
    // Parse request line
    let (line_end, line_len) = find_line(data, 0).ok_or(H1Error::Incomplete)?;
    let request_line = trim_end(&data[..line_end]);
    let request_str = std::str::from_utf8(request_line).map_err(|_| H1Error::BadRequestLine("not utf-8".into()))?;

    let parts: Vec<&str> = request_str.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return Err(H1Error::BadRequestLine("expected 3 parts".into()));
    }
    let method = Method::from_bytes(parts[0].as_bytes());
    let uri = Uri::parse(parts[1]).ok_or_else(|| H1Error::BadRequestLine("bad uri".into()))?;
    let version = Version::from_str(parts[2]).unwrap_or(Version::Http11);

    // Parse headers
    let mut pos = line_len;
    let (headers, consumed) = parse_headers(data, pos)?;
    pos = consumed;

    // Determine body length
    let content_length = get_content_length(&headers);

    let body = if let Some(cl) = content_length {
        let body_start = pos;
        let body_end = body_start + cl;
        if data.len() < body_end {
            return Err(H1Error::Incomplete);
        }
        Some(bytes::Bytes::copy_from_slice(&data[body_start..body_end]))
    } else {
        None
    };

    let total = pos + body.as_ref().map(|b| b.len()).unwrap_or(0);
    Ok((
        Request {
            method,
            uri,
            version,
            headers,
            body,
        },
        total,
    ))
}

pub fn decode_response(data: &[u8]) -> Result<(Response, usize), H1Error> {
    // Parse status line
    let (line_end, line_len) = find_line(data, 0).ok_or(H1Error::Incomplete)?;
    let status_line = trim_end(&data[..line_end]);
    let status_str = std::str::from_utf8(status_line).map_err(|_| H1Error::BadStatusLine("not utf-8".into()))?;

    let parts: Vec<&str> = status_str.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return Err(H1Error::BadStatusLine("expected at least status code".into()));
    }
    let version = Version::from_str(parts[0]).unwrap_or(Version::Http11);
    let code: u16 = parts[1]
        .parse()
        .map_err(|_| H1Error::BadStatusLine("bad status code".into()))?;
    let status = StatusCode::from_u16(code).ok_or_else(|| H1Error::BadStatusLine("invalid status code".into()))?;

    // Parse headers
    let mut pos = line_len;
    let (headers, consumed) = parse_headers(data, pos)?;
    pos = consumed;

    // Determine body length
    let content_length = get_content_length(&headers);
    let is_chunked = is_chunked_encoding(&headers);

    let body = if let Some(cl) = content_length {
        let body_start = pos;
        let body_end = body_start + cl;
        if data.len() < body_end {
            return Err(H1Error::Incomplete);
        }
        bytes::Bytes::copy_from_slice(&data[body_start..body_end])
    } else if is_chunked || pos < data.len() {
        bytes::Bytes::copy_from_slice(&data[pos..])
    } else {
        bytes::Bytes::new()
    };

    let total = pos + body.len();
    Ok((
        Response {
            version,
            status,
            headers,
            body,
        },
        total,
    ))
}

fn parse_headers(data: &[u8], start: usize) -> Result<(Headers, usize), H1Error> {
    let mut pos = start;
    let mut headers = Vec::new();

    loop {
        let (line_end, next_start) = find_line(data, pos).ok_or(H1Error::Incomplete)?;

        if line_end == pos {
            return Ok((headers, next_start));
        }

        let line = trim_end(&data[pos..line_end]);
        let line_str = std::str::from_utf8(line).map_err(|_| H1Error::BadHeader("not utf-8".into()))?;

        if let Some(colon) = line_str.find(':') {
            let name = HeaderName::from_bytes(&line[..colon]);
            let value = HeaderValue::from_bytes(&line[colon + 1..]);
            headers.push((name, value));
        } else if line_str.starts_with(' ') || line_str.starts_with('\t') {
            if let Some((_, last_val)) = headers.last_mut() {
                let mut new_val = last_val.as_bytes().to_vec();
                new_val.push(b' ');
                new_val.extend_from_slice(line_str.trim_start().as_bytes());
                *last_val = HeaderValue::from_bytes(&new_val);
            }
        } else {
            return Err(H1Error::BadHeader("malformed header line".into()));
        }

        pos = next_start;
    }
}

fn get_content_length(headers: &Headers) -> Option<usize> {
    for (name, value) in headers {
        if name.as_str() == "content-length" {
            if let Ok(len) = value.as_str().trim().parse::<usize>() {
                return Some(len);
            }
        }
    }
    None
}

fn is_chunked_encoding(headers: &Headers) -> bool {
    for (name, value) in headers {
        if name.as_str() == "transfer-encoding" {
            if value.as_str().to_ascii_lowercase().contains("chunked") {
                return true;
            }
        }
    }
    false
}

pub struct ResponseDecoder {
    buf: Vec<u8>,
}

impl ResponseDecoder {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
        }
    }

    pub fn feed(&mut self, data: &[u8]) -> Result<Option<(Response, &[u8])>, H1Error> {
        self.buf.extend_from_slice(data);
        match decode_response(&self.buf) {
            Ok((resp, consumed)) => {
                let tail = self.buf.split_off(consumed);
                self.buf = tail;
                Ok(Some((resp, &self.buf)))
            }
            Err(H1Error::Incomplete) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request_simple() {
        let data = b"GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: test\r\n\r\n";
        let (req, _) = decode_request(data).unwrap();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.uri.as_str(), "/");
        assert_eq!(req.version, Version::Http11);
        assert!(req.body.is_none());
    }

    #[test]
    fn test_parse_request_with_body() {
        let data = b"POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\n\r\nhello";
        let (req, _) = decode_request(data).unwrap();
        assert_eq!(req.method, Method::Post);
        assert_eq!(&req.body.unwrap()[..], b"hello");
    }

    #[test]
    fn test_parse_response_simple() {
        let data = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhello";
        let (resp, _) = decode_response(data).unwrap();
        assert_eq!(resp.status, StatusCode::Ok);
        assert_eq!(&resp.body[..], b"hello");
    }

    #[test]
    fn test_parse_response_no_body() {
        let data = b"HTTP/1.1 204 No Content\r\n\r\n";
        let (resp, _) = decode_response(data).unwrap();
        assert_eq!(resp.status, StatusCode::NoContent);
        assert!(resp.body.is_empty());
    }

    #[test]
    fn test_parse_not_found() {
        let data = b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        let (resp, _) = decode_response(data).unwrap();
        assert_eq!(resp.status, StatusCode::NotFound);
        assert!(resp.body.is_empty());
    }

    #[test]
    fn test_parse_incomplete() {
        let data = b"GET / HTTP/1.1\r\nHost: ";
        assert!(matches!(decode_request(data), Err(H1Error::Incomplete)));
    }

    #[test]
    fn test_parse_bad_request_line() {
        let data = b"\r\n\r\n";
        assert!(matches!(decode_request(data), Err(H1Error::BadRequestLine(_))));
    }

    #[test]
    fn test_parse_bare_newlines() {
        let data = b"GET / HTTP/1.1\nHost: example.com\n\n";
        let (req, _) = decode_request(data).unwrap();
        assert_eq!(req.method, Method::Get);
    }

    #[test]
    fn test_header_order_preserved() {
        let data = b"GET / HTTP/1.1\r\nHost: a.com\r\nAccept: */*\r\nUser-Agent: test\r\n\r\n";
        let (req, _) = decode_request(data).unwrap();
        let h: Vec<&str> = req.headers.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(h, vec!["host", "accept", "user-agent"]);
    }

    #[test]
    fn test_absolute_uri_request() {
        let data = b"GET http://example.com/path HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let (req, _) = decode_request(data).unwrap();
        assert_eq!(req.uri.as_str(), "http://example.com/path");
    }
}
