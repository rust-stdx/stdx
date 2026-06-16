//! HTML escaping functions for auto-escaping mode and the `escape` filter.

use alloc::string::String;

use memchr::{memchr2, memchr3};

use crate::error::Error;

pub type EscaperFn = fn(&str, &mut String) -> Result<(), Error>;

/// HTML escaper (HTML mode)
pub fn html_escape(input: &str, output: &mut String) -> Result<(), Error> {
    output.reserve(input.len());
    let bytes = input.as_bytes();
    let mut start = 0;

    loop {
        // Find the next position of any special byte (<, >, &, ", ')
        let pos = match memchr3(b'<', b'>', b'&', &bytes[start..]) {
            Some(p) => {
                // Check if " or ' occurs before position p
                match memchr2(b'"', b'\'', &bytes[start..][..p]) {
                    Some(q) => start + q,
                    None => start + p,
                }
            }
            None => match memchr2(b'"', b'\'', &bytes[start..]) {
                Some(q) => start + q,
                None => {
                    // No more special bytes, flush remaining
                    output.push_str(&input[start..]);
                    return Ok(());
                }
            },
        };

        // Copy safe prefix
        if pos > start {
            output.push_str(&input[start..pos]);
        }

        // Process the special byte
        match bytes[pos] {
            b'<' => output.push_str("&lt;"),
            b'>' => output.push_str("&gt;"),
            b'&' => output.push_str("&amp;"),
            b'"' => output.push_str("&quot;"),
            b'\'' => output.push_str("&#39;"),
            _ => unreachable!(),
        }
        start = pos + 1;
    }
}

/// Convenience wrapper that returns a new `String`.
pub fn html_escape_to_string(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    html_escape(input, &mut output).unwrap();
    output
}
