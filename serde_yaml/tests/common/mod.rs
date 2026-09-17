//! Shared helpers for the integration tests.
//!
//! Provides a drop-in `indoc!` replacement so the tests can un-indent
//! multiline string literals without pulling in an external crate.

/// Un-indents `input` following the same rules as `indoc!`:
///
/// 1. Count the leading spaces/tabs of each line after the first, ignoring
///    lines that are empty or contain whitespace only.
/// 2. Take the minimum.
/// 3. If the string begins with a newline, remove the first (empty) line.
/// 4. Remove the computed number of leading whitespace bytes from every line.
pub fn dedent(input: &str) -> String {
    let stripped = if input.starts_with("\r\n") { &input[1..] } else { input };
    let ignore_first_line = input.starts_with('\n') || input.starts_with("\r\n");

    let spaces = stripped.split('\n').skip(1).filter_map(count_spaces).min().unwrap_or(0);

    let mut result = String::with_capacity(stripped.len());
    for (i, line) in stripped.split('\n').enumerate() {
        if i > 1 || (i == 1 && !ignore_first_line) {
            result.push('\n');
        }
        if i == 0 {
            result.push_str(line);
        } else if line.len() > spaces {
            result.push_str(&line[spaces..]);
        }
    }
    result
}

/// Returns `s` with a `'static` lifetime by leaking it.
///
/// Only intended for use in tests, where the process is short-lived.
pub fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// Number of leading spaces/tabs in the line, or `None` if the line is empty or
// entirely whitespace.
fn count_spaces(line: &str) -> Option<usize> {
    for (i, b) in line.bytes().enumerate() {
        if b != b' ' && b != b'\t' {
            return Some(i);
        }
    }
    None
}

/// Un-indents a multiline string literal at runtime.
#[macro_export]
macro_rules! indoc {
    ($s:literal) => {
        $crate::common::leak($crate::common::dedent($s))
    };
}
