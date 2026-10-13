//! Path handling tests that must hold in both build modes.
//!
//! The same assertions run when assets are read from the file system at runtime
//! (debug builds) and when they are embedded in the binary (release builds or
//! the `debug-embed` feature), so that debug and release behaviour cannot drift
//! apart.

use embed::{Embed, RustEmbed};

#[derive(Embed)]
#[folder = "tests/fixtures/assets"]
struct Asset;

#[test]
fn valid_paths_resolve() {
    for path in ["index.html", "sub/nested.txt", "inside_link.txt"] {
        assert!(Asset::get(path).is_some(), "{path} should resolve");
    }
}

#[test]
fn valid_paths_resolve_through_the_trait() {
    for path in ["index.html", "sub/nested.txt", "inside_link.txt"] {
        assert!(<Asset as RustEmbed>::get(path).is_some(), "{path} should resolve");
    }
}

#[test]
fn unknown_files_do_not_resolve() {
    for path in ["missing.html", "sub/missing.txt", "index.htm"] {
        assert!(Asset::get(path).is_none(), "{path} should not resolve");
    }
}

#[test]
fn case_sensitivity_follows_the_file_system() {
    // Embedded lookups always require an exact match. Debug lookups resolve
    // through the file system, so on a case-insensitive file system (for
    // example the default macOS configuration) a case variant may resolve. Pin
    // the debug result to what canonicalization does on this machine.
    let resolved = Asset::get("INDEX.HTML").is_some();
    let embedded = !cfg!(debug_assertions) || cfg!(feature = "debug-embed");

    if embedded {
        assert!(!resolved, "embedded lookups must be case sensitive");
    } else {
        // Probe the same directory the crate resolves to. It is relative to the
        // current directory, which works natively and inside WASI sandboxes.
        let probe = std::path::Path::new("tests/fixtures/assets/INDEX.HTML")
            .canonicalize()
            .is_ok();
        assert_eq!(resolved, probe);
    }
}

#[test]
fn paths_that_are_not_valid_keys_do_not_resolve() {
    // None of these may be normalized, resolved or otherwise accepted: the only
    // valid inputs are the exact paths returned by `iter`.
    for path in [
        "",
        ".",
        "..",
        "/",
        "/index.html",
        "index.html/",
        "./index.html",
        "sub/../index.html",
        "sub//nested.txt",
        "sub/./nested.txt",
        "sub/",
        "../Cargo.toml",
        "../../Cargo.toml",
        "sub/../../Cargo.toml",
        "..\\Cargo.toml",
        "sub\\nested.txt",
        "%2e%2e/Cargo.toml",
        "index.html\0",
    ] {
        assert!(Asset::get(path).is_none(), "{path:?} should not resolve");
    }
}

#[test]
fn symlinks_that_escape_the_folder_are_not_served() {
    assert!(Asset::get("outside_link.txt").is_none());
    assert!(Asset::get("outside_dir_link").is_none());
    assert!(Asset::get("outside_dir_link/secret.txt").is_none());
    assert!(Asset::get("broken_link.txt").is_none());
}

#[test]
fn iter_lists_all_and_only_retrievable_files() {
    let mut names: Vec<String> = Asset::iter().map(|name| name.into_owned()).collect();
    names.sort();

    // The escaping and broken symlinks are excluded, the in-folder symlink is
    // kept.
    assert_eq!(names, ["index.html", "inside_link.txt", "sub/nested.txt"]);

    for name in &names {
        assert!(Asset::get(name).is_some(), "iter listed {name:?} but get returned None");
    }
}

#[test]
fn iter_and_get_agree_via_the_trait() {
    let mut count = 0;
    for name in <Asset as RustEmbed>::iter() {
        assert!(<Asset as RustEmbed>::get(name.as_ref()).is_some(), "{name:?} should resolve");
        count += 1;
    }
    assert_eq!(count, 3);
}

#[test]
fn escaping_symlink_targets_are_not_listed() {
    for name in Asset::iter() {
        assert!(
            !name.contains("outside") && !name.contains("broken"),
            "iter must not list escaping symlinks, got {name:?}"
        );
    }
}
