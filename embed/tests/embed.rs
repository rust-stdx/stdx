//! Integration tests for the `embed` crate.
//!
//! They run against both build modes: in debug builds the assets are read from
//! the file system at runtime, while in release builds (or with the
//! `debug-embed` feature) they are embedded into the binary.

use embed::{Embed, RustEmbed};

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn get_works() {
    assert!(Asset::get("index.html").is_some(), "index.html should exist");
    assert!(Asset::get("gg.html").is_none(), "gg.html should not exist");
    assert!(Asset::get("images/llama.png").is_some(), "llama.png should exist");
}

#[test]
fn get_via_trait_works() {
    assert!(<Asset as RustEmbed>::get("index.html").is_some());
    assert!(<Asset as RustEmbed>::get("gg.html").is_none());
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

#[test]
fn trait_works_generic() {
    trait_works_generic_helper::<Asset>();
}

fn trait_works_generic_helper<E: RustEmbed>() {
    let mut num_files = 0;
    for file in E::iter() {
        assert!(E::get(file.as_ref()).is_some());
        num_files += 1;
    }
    assert_eq!(num_files, 7);
    assert!(E::get("gg.html").is_none(), "gg.html should not exist");
}
