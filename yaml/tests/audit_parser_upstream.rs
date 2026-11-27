//! Documentation tests for libyaml fixes that landed after the commit that
//! `libyaml_unsafe` 0.2.11 was translated from and are intentionally *not*
//! ported here.
//!
//! Run with:
//!     cargo test -p yaml --test audit_parser_upstream -- --ignored --nocapture

use yaml::{Value, from_slice};

#[test]
#[ignore = "unfixed (libyaml PR #296): `[?]` with an empty explicit key is valid YAML but rejected"]
fn flow_sequence_with_empty_explicit_key_is_valid_yaml() {
    // `[?]` is a flow sequence containing a mapping with an empty key and an
    // empty value. libyaml 0.2.6 fixed the parser to accept it; the vendored
    // 0.2.5 parser reports "did not find expected ',' or ']'".
    for input in ["[?]", "[? ]", "[ ? ]", "[? : b]"] {
        match from_slice::<Value>(input.as_bytes()) {
            Ok(value) => println!("{input:8} => {value:?}"),
            Err(error) => {
                println!("{input:8} => ERROR: {error}");
                panic!("valid YAML {input:?} was rejected: {error}");
            }
        }
    }
}
