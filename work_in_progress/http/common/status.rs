use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusCode(u16);

macro_rules! status_codes {
    ($($name:ident = $code:expr, $reason:expr, $class:ident),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        impl StatusCode {
            $(pub const $name: StatusCode = StatusCode($code);)*

            pub fn from_u16(code: u16) -> Option<Self> {
                if (100..=599).contains(&code) {
                    Some(StatusCode(code))
                } else {
                    None
                }
            }

            pub fn as_u16(&self) -> u16 {
                self.0
            }

            pub fn canonical_reason(&self) -> Option<&'static str> {
                match self.0 {
                    $($code => Some($reason),)*
                    _ => None,
                }
            }

            pub fn is_informational(&self) -> bool {
                (100..200).contains(&self.0)
            }

            pub fn is_successful(&self) -> bool {
                (200..300).contains(&self.0)
            }

            pub fn is_redirection(&self) -> bool {
                (300..400).contains(&self.0)
            }

            pub fn is_client_error(&self) -> bool {
                (400..500).contains(&self.0)
            }

            pub fn is_server_error(&self) -> bool {
                (500..600).contains(&self.0)
            }
        }
    };
}

status_codes! {
    Continue = 100, "Continue", informational,
    SwitchingProtocols = 101, "Switching Protocols", informational,
    Ok = 200, "OK", successful,
    Created = 201, "Created", successful,
    Accepted = 202, "Accepted", successful,
    NonAuthoritativeInformation = 203, "Non-Authoritative Information", successful,
    NoContent = 204, "No Content", successful,
    ResetContent = 205, "Reset Content", successful,
    PartialContent = 206, "Partial Content", successful,
    MultipleChoices = 300, "Multiple Choices", redirection,
    MovedPermanently = 301, "Moved Permanently", redirection,
    Found = 302, "Found", redirection,
    SeeOther = 303, "See Other", redirection,
    NotModified = 304, "Not Modified", redirection,
    UseProxy = 305, "Use Proxy", redirection,
    TemporaryRedirect = 307, "Temporary Redirect", redirection,
    PermanentRedirect = 308, "Permanent Redirect", redirection,
    BadRequest = 400, "Bad Request", client_error,
    Unauthorized = 401, "Unauthorized", client_error,
    PaymentRequired = 402, "Payment Required", client_error,
    Forbidden = 403, "Forbidden", client_error,
    NotFound = 404, "Not Found", client_error,
    MethodNotAllowed = 405, "Method Not Allowed", client_error,
    NotAcceptable = 406, "Not Acceptable", client_error,
    ProxyAuthenticationRequired = 407, "Proxy Authentication Required", client_error,
    RequestTimeout = 408, "Request Timeout", client_error,
    Conflict = 409, "Conflict", client_error,
    Gone = 410, "Gone", client_error,
    LengthRequired = 411, "Length Required", client_error,
    PreconditionFailed = 412, "Precondition Failed", client_error,
    ContentTooLarge = 413, "Content Too Large", client_error,
    UriTooLong = 414, "URI Too Long", client_error,
    UnsupportedMediaType = 415, "Unsupported Media Type", client_error,
    RangeNotSatisfiable = 416, "Range Not Satisfiable", client_error,
    ExpectationFailed = 417, "Expectation Failed", client_error,
    MisdirectedRequest = 421, "Misdirected Request", client_error,
    UnprocessableContent = 422, "Unprocessable Content", client_error,
    UpgradeRequired = 426, "Upgrade Required", client_error,
    InternalServerError = 500, "Internal Server Error", server_error,
    NotImplemented = 501, "Not Implemented", server_error,
    BadGateway = 502, "Bad Gateway", server_error,
    ServiceUnavailable = 503, "Service Unavailable", server_error,
    GatewayTimeout = 504, "Gateway Timeout", server_error,
    HttpVersionNotSupported = 505, "HTTP Version Not Supported", server_error,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_status() {
        assert_eq!(StatusCode::from_u16(200), Some(StatusCode::Ok));
        assert_eq!(StatusCode::from_u16(404), Some(StatusCode::NotFound));
    }

    #[test]
    fn test_invalid_range() {
        assert!(StatusCode::from_u16(99).is_none());
        assert!(StatusCode::from_u16(600).is_none());
    }

    #[test]
    fn test_classification() {
        assert!(StatusCode::Ok.is_successful());
        assert!(StatusCode::NotFound.is_client_error());
        assert!(StatusCode::InternalServerError.is_server_error());
        assert!(StatusCode::Continue.is_informational());
        assert!(StatusCode::MovedPermanently.is_redirection());
    }

    #[test]
    fn test_canonical_reason() {
        assert_eq!(StatusCode::Ok.canonical_reason(), Some("OK"));
        assert_eq!(StatusCode::NotFound.canonical_reason(), Some("Not Found"));
    }

    #[test]
    fn test_custom_status() {
        let s = StatusCode::from_u16(299).unwrap();
        assert!(s.is_successful());
        assert_eq!(s.canonical_reason(), None);
    }
}
