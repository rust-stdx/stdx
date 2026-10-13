//! Metadata tests, running in both build modes.

use std::{fs, time::SystemTime};

use crypto::{Hasher, blake3::Blake3};
use embed::Embed;

#[derive(Embed)]
#[folder = "tests/fixtures/assets"]
struct Asset;

#[derive(Embed)]
#[folder = "tests/fixtures/assets"]
#[metadata_only = true]
struct OnlyMetadata;

const INDEX: &str = "index.html";

/// Fixtures are resolved relative to the current directory: Cargo runs test
/// binaries from the package root, and the wasm test runner preopens it as `/`.
/// This also works inside WASI sandboxes, where the build time
/// `CARGO_MANIFEST_DIR` is not mounted.
fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new("tests/fixtures/assets").join(name)
}

#[test]
fn hash_matches_file_contents() {
    let file = Asset::get(INDEX).unwrap();
    let expected = Blake3::hash(&fs::read(fixture_path(INDEX)).unwrap());
    assert_eq!(file.metadata.hash().as_slice(), expected.as_ref());
}

#[test]
fn in_folder_symlink_hash_matches_its_target() {
    let file = Asset::get("inside_link.txt").unwrap();
    let expected = Blake3::hash(&fs::read(fixture_path(INDEX)).unwrap());
    assert_eq!(file.metadata.hash().as_slice(), expected.as_ref());
}

#[test]
fn timestamps_match_the_file_system() {
    // SOURCE_DATE_EPOCH overrides the timestamps; only assert equality when it
    // is not in use.
    if std::env::var_os("SOURCE_DATE_EPOCH").is_some() {
        return;
    }

    let file = Asset::get(INDEX).unwrap();
    let metadata = fs::metadata(fixture_path(INDEX)).unwrap();

    let expected_modified = metadata
        .modified()
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert_eq!(file.metadata.last_modified(), Some(expected_modified));

    if let Ok(created) = metadata.created() {
        let expected_created = created.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        assert_eq!(file.metadata.created(), Some(expected_created));
    }
}

#[test]
fn metadata_only_files_have_no_data_but_keep_their_hash() {
    let only = OnlyMetadata::get(INDEX).unwrap();
    assert!(only.data.is_empty());

    let full = Asset::get(INDEX).unwrap();
    assert_eq!(only.metadata.hash(), full.metadata.hash());
    assert_eq!(only.metadata.last_modified(), full.metadata.last_modified());
}

#[test]
fn metadata_only_iteration_still_lists_files() {
    assert!(OnlyMetadata::iter().any(|name| name == INDEX));
}

#[test]
fn debug_impls_summarize_instead_of_dumping_contents() {
    let file = Asset::get(INDEX).unwrap();
    let rendered = format!("{file:?}");
    assert!(rendered.contains("EmbeddedFile"), "{rendered}");
    assert!(rendered.contains("bytes"), "{rendered}");
    assert!(rendered.contains("Metadata"), "{rendered}");
    // The body is summarized, not printed, and the hash is hexadecimal.
    assert!(!rendered.contains("fixture index"), "{rendered}");
    assert!(!rendered.contains("[224,"), "{rendered}");
}

#[cfg(feature = "guess-media-type")]
mod media_type {
    use super::*;

    #[test]
    fn mimetypes_are_guessed() {
        assert_eq!(Asset::get("index.html").unwrap().metadata.mimetype(), "text/html");
        assert_eq!(Asset::get("sub/nested.txt").unwrap().metadata.mimetype(), "text/plain");
    }

    #[test]
    fn in_folder_symlink_uses_the_type_of_its_target() {
        // The mimetype is guessed from the resolved path, so a symlink to an
        // HTML file reports `text/html` in both build modes.
        assert_eq!(Asset::get("inside_link.txt").unwrap().metadata.mimetype(), "text/html");
    }
}
