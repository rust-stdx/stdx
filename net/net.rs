mod hostname;

pub use hostname::hostname;

/// Splits the host part and the port part from an hostname
/// - `"127.0.0.1"` -> ("127.0.0.1", "")
/// - `"127.0.0.1:80"` -> ("127.0.0.1", "80")
/// - `"[::1]"` -> ("::1", "")
/// - `"[::1]:8080"` -> ("::1", "8080")
/// - `"localhost:8080"` -> ("localhost", "8080")
pub fn split_host_port(input: &str) -> (&str, &str) {
    if let Some(index) = input.rfind(':') {
        if index == (input.len() - 1) {
            // input is a malformed input. e.g. localhost:
            return (&input[..index], "");
        }
        let mut host = &input[..index];
        let mut port = &input[index + 1..];
        if port.rfind(']').is_some() {
            // input is an IPv6 without the port. e.g. [::1]
            host = input.trim_start_matches('[').trim_end_matches(']');
            port = "";
        } else if host.find('[').is_some() {
            // input was an IPv6 with the port. e.g. [::1]:8080
            host = host.trim_start_matches('[').trim_end_matches(']');
        }
        return (host, port);
    }
    return (input, "");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_host_port() {
        struct Expected {
            host: &'static str,
            port: &'static str,
        }
        let tests = [
            (
                "",
                Expected {
                    host: "",
                    port: "",
                },
            ),
            (
                "localhost",
                Expected {
                    host: "localhost",
                    port: "",
                },
            ),
            (
                "127.0.0.1",
                Expected {
                    host: "127.0.0.1",
                    port: "",
                },
            ),
            (
                "[::1]",
                Expected {
                    host: "::1",
                    port: "",
                },
            ),
            (
                "localhost:8080",
                Expected {
                    host: "localhost",
                    port: "8080",
                },
            ),
            (
                "127.0.0.1:8080",
                Expected {
                    host: "127.0.0.1",
                    port: "8080",
                },
            ),
            (
                "[::1]:8080",
                Expected {
                    host: "::1",
                    port: "8080",
                },
            ),
            (
                "localhost:",
                Expected {
                    host: "localhost",
                    port: "",
                },
            ),
        ];

        for test in tests {
            let (host, port) = split_host_port(test.0);
            assert_eq!(test.1.host, host);
            assert_eq!(test.1.port, port);
        }
    }
}
