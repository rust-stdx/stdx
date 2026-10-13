//! Tests for the `crate_path` attribute.
//!
//! The generated code must refer to the crate named by the attribute rather than
//! to `embed`, so a decoy `embed` module is introduced to break resolution if
//! the attribute is ignored.

use custom::path::embed::Embed;

mod custom {
    pub mod path {
        pub use embed;
    }
}

mod embed {}

#[derive(Embed)]
#[crate_path = "custom::path::embed"]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn crate_path_redirects_the_generated_code() {
    assert!(Asset::get("index.html").is_some());
    assert!(Asset::get("images/llama.png").is_some());
}

#[test]
fn crate_path_keeps_iter_and_get_consistent() {
    for name in Asset::iter() {
        assert!(Asset::get(name.as_ref()).is_some(), "{name:?} should resolve");
    }
}
