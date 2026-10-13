//! Tests that paths cannot escape the embedded folder.
//!
//! This matters the most in debug builds, where the assets are read from the
//! file system at runtime.

use embed::Embed;

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn path_traversal_attack_fails() {
    assert!(Asset::get("../basic.rs").is_none());
    assert!(Asset::get("../../Cargo.toml").is_none());
}
