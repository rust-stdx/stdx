//! Tests for `interpolate-folder-path`.
//!
//! They only run when the `interpolate-folder-path` feature is enabled (it needs
//! the external `shellexpand` dependency, so it is disabled by default).
#![cfg(feature = "interpolate-folder-path")]

use embed::Embed;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/examples/public/"]
struct Asset;

#[test]
fn get_works() {
    assert!(Asset::get("index.html").is_some(), "index.html should exist");
    assert!(Asset::get("gg.html").is_none(), "gg.html should not exist");
    assert!(Asset::get("images/llama.png").is_some(), "llama.png should exist");
}

#[test]
fn iter_works() {
    let mut num_files = 0;
    for file in Asset::iter() {
        assert!(Asset::get(file.as_ref()).is_some());
        num_files += 1;
    }
    assert_eq!(num_files, 7);
}
