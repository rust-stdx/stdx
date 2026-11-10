//! Whole-document validation and whitespace removal.

use crate::{
    decode::{Decoder, Options},
    error::Result,
};

/// Validates that `input` contains exactly one JSON value, optionally
/// surrounded by whitespace.
pub fn validate(input: &[u8]) -> Result<()> {
    validate_with_options(input, Options::default())
}

/// Validates with explicit decoding options.
///
/// The whole input is validated as UTF-8 once first; string scanning then
/// skips redundant UTF-8 checks.
pub fn validate_with_options(input: &[u8], options: Options) -> Result<()> {
    utf8::from_utf8(input)
        .map_err(|error| crate::Error::at(crate::ErrorKind::InvalidUtf8, input, error.valid_up_to()))?;
    let mut decoder = Decoder::from_validated_utf8(input, options);
    decoder.skip_value().map_err(|error| decoder.locate_error(error))?;
    decoder.finish()
}

/// Removes insignificant whitespace from `input` and writes the result to
/// `output`.
///
/// Returns the number of bytes written. The input is validated first, so an
/// invalid document is never partially copied.
pub fn minify(input: &[u8], output: &mut [u8]) -> Result<usize> {
    validate(input)?;
    let mut written = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for &byte in input {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else {
            match byte {
                b' ' | b'\t' | b'\n' | b'\r' => continue,
                b'"' => in_string = true,
                _ => {}
            }
        }
        if written >= output.len() {
            return Err(crate::Error::buffer_too_small());
        }
        output[written] = byte;
        written += 1;
    }
    Ok(written)
}
