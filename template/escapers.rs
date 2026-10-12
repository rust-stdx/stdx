//! HTML escaping functions for auto-escaping mode and the `escape` filter.

use alloc::string::String;
use core::fmt;

use memchr::{memchr2, memchr3};

/// Signature of an output escaper.
///
/// The escaper appends `input` to `output`, escaping it as appropriate for the
/// active [`EscapeMode`](crate::EscapeMode). It returns a [`fmt::Error`] only if
/// the underlying writer fails.
pub type EscaperFn = fn(&str, &mut dyn fmt::Write) -> fmt::Result;

/// HTML escaper (HTML mode).
///
/// Escapes `&`, `<`, `>`, `"` and `'` so the result is safe to embed in both
/// text and quoted-attribute contexts. Writes directly into `output`.
pub fn html_escape(input: &str, output: &mut dyn fmt::Write) -> fmt::Result {
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
                    output.write_str(&input[start..])?;
                    return Ok(());
                }
            },
        };

        // Copy safe prefix
        if pos > start {
            output.write_str(&input[start..pos])?;
        }

        // Process the special byte
        output.write_str(match bytes[pos] {
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'&' => "&amp;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => unreachable!(),
        })?;
        start = pos + 1;
    }
}

/// Convenience wrapper that returns a new `String`.
pub fn html_escape_to_string(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    html_escape(input, &mut output).expect("writing to a String never fails");
    output
}
