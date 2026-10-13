//! Shared utilities used by the `embed` crate and by `embed_impl`.
//!
//! This crate is an implementation detail and should not be used directly.
#![forbid(unsafe_code)]

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt, fs, io,
    path::{Path, PathBuf},
    sync::{OnceLock, RwLock},
    time::SystemTime,
};

use crypto::{Hasher, blake3::Blake3};

/// A file discovered while walking the assets folder.
///
/// `rel_path` is the path of the file relative to the folder root, always using
/// `/` as the separator and always satisfying [`is_valid_key`].
///
/// `full_canonical_path` is the absolute, symlink-resolved path of the file and
/// is guaranteed to live inside the canonical folder root. Entries whose target
/// escapes the folder (for example a symlink pointing at `/etc/passwd`) are
/// skipped, so callers never embed or serve files outside the folder.
#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub struct FileEntry {
    pub rel_path: String,
    pub full_canonical_path: PathBuf,
}

/// Walks `folder_path` and yields every file that belongs to the folder.
///
/// Files are visited in a deterministic order (sorted by file name at each
/// level). Entries are skipped, never panicking, when they cannot be used:
///
/// - names that are not valid UTF-8;
/// - names that are not valid lookup keys (see [`is_valid_key`]), including
///   names containing a backslash on non-Windows platforms;
/// - paths that cannot be canonicalized (for example a symlink that was removed
///   while the folder was being walked);
/// - files whose canonical path leaves `folder_path` (symbolic links pointing
///   outside the folder).
#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub fn get_files(folder_path: &Path, matcher: &PathMatcher) -> impl Iterator<Item = FileEntry> {
    // Resolve the folder once; entries are checked against it so that symlinks
    // cannot escape the folder.
    let canonical_root = folder_path.canonicalize().ok();

    walkdir::WalkDir::new(folder_path)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(move |entry| {
            let rel_path = entry.path().strip_prefix(folder_path).ok()?;
            let rel_path = rel_path.to_str()?;
            let rel_path = normalize_rel_path(rel_path);
            if !is_valid_key(&rel_path) || !matcher.is_path_included(&rel_path) {
                return None;
            }

            let full_canonical_path = entry.path().canonicalize().ok()?;
            if !full_canonical_path.starts_with(canonical_root.as_ref()?) {
                return None;
            }

            Some(FileEntry {
                rel_path: rel_path.into_owned(),
                full_canonical_path,
            })
        })
}

/// On Windows, rewrites the platform separator (`\`) to `/` so that lookup keys
/// are slash-separated on every platform. On other platforms the name is left
/// untouched, so a file whose name literally contains a backslash is rejected
/// by [`is_valid_key`] rather than shadowing a nested file.
#[cfg(windows)]
fn normalize_rel_path(path: &str) -> Cow<'_, str> {
    if path.contains('\\') {
        Cow::Owned(path.replace('\\', "/"))
    } else {
        Cow::Borrowed(path)
    }
}

#[cfg(not(windows))]
fn normalize_rel_path(path: &str) -> Cow<'_, str> {
    Cow::Borrowed(path)
}

/// Returns `true` when `key` is a valid relative path that may be passed to
/// `get`.
///
/// Keys use `/` as the separator on every platform. A valid key is non-empty,
/// relative, contains no NUL byte, no backslash and no empty, `.` or `..`
/// segment. In other words it matches exactly the kind of path returned by
/// `iter`, and any other input is rejected instead of being normalized or
/// resolved against the file system.
#[doc(hidden)]
pub fn is_valid_key(key: &str) -> bool {
    if key.is_empty() || key.contains('\\') || key.contains('\0') || Path::new(key).is_absolute() {
        return false;
    }

    key.split('/')
        .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}

/// A file embedded into the binary
#[derive(Clone)]
pub struct EmbeddedFile {
    pub data: Cow<'static, [u8]>,
    pub metadata: Metadata,
}

impl fmt::Debug for EmbeddedFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmbeddedFile")
            .field("data", &format_args!("{} bytes", self.data.len()))
            .field("metadata", &self.metadata)
            .finish()
    }
}

/// Metadata about an embedded file
#[derive(Clone)]
pub struct Metadata {
    /// the BLAKE3 hash of the file
    hash: [u8; 32],
    last_modified: Option<u64>,
    created: Option<u64>,
    #[cfg(feature = "guess-media-type")]
    mimetype: Cow<'static, str>,
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Metadata");
        debug.field("hash", &Hex(&self.hash));
        debug.field("last_modified", &self.last_modified);
        debug.field("created", &self.created);
        #[cfg(feature = "guess-media-type")]
        debug.field("mimetype", &self.mimetype);
        debug.finish()
    }
}

/// Renders bytes as lowercase hexadecimal, used by `Metadata`'s [`fmt::Debug`].
struct Hex<'a>(&'a [u8]);

impl fmt::Debug for Hex<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl Metadata {
    #[doc(hidden)]
    pub const fn __embed_new(
        hash: [u8; 32],
        last_modified: Option<u64>,
        created: Option<u64>,
        #[cfg(feature = "guess-media-type")] mimetype: &'static str,
    ) -> Self {
        Self {
            hash,
            last_modified,
            created,
            #[cfg(feature = "guess-media-type")]
            mimetype: Cow::Borrowed(mimetype),
        }
    }

    /// The BLAKE3 hash of the file
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// The last modified date in seconds since the UNIX epoch. If the
    /// underlying platform/file-system does not support this, `None` is
    /// returned. When the `SOURCE_DATE_EPOCH` environment variable was set when
    /// the file was first read, its value is returned instead, for reproducible
    /// builds.
    pub fn last_modified(&self) -> Option<u64> {
        self.last_modified
    }

    /// The created date in seconds since the UNIX epoch. If the underlying
    /// platform/file-system does not support this, `None` is returned. When the
    /// `SOURCE_DATE_EPOCH` environment variable was set when the file was first
    /// read, its value is returned instead, for reproducible builds.
    pub fn created(&self) -> Option<u64> {
        self.created
    }

    /// The mime type of the file
    #[cfg(feature = "guess-media-type")]
    pub fn mimetype(&self) -> &str {
        &self.mimetype
    }
}

/// Builds a [`Metadata`] from the tokens emitted by the derive macro.
///
/// The derive macro is compiled for the host and cannot reliably branch on its
/// own features, so it always passes a mimetype. Whether the mimetype is kept
/// is decided here, by the crate compiled for the target.
#[cfg(feature = "guess-media-type")]
#[doc(hidden)]
#[macro_export]
macro_rules! __embed_metadata {
    ($hash:expr, $last_modified:expr, $created:expr, $mimetype:expr $(,)?) => {
        $crate::Metadata::__embed_new($hash, $last_modified, $created, $mimetype)
    };
}

/// See the `guess-media-type` variant above; this one drops the mimetype.
#[cfg(not(feature = "guess-media-type"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __embed_metadata {
    ($hash:expr, $last_modified:expr, $created:expr, $mimetype:expr $(,)?) => {
        $crate::Metadata::__embed_new($hash, $last_modified, $created)
    };
}

/// Guesses the media type of `path` at build time.
///
/// Always available, regardless of the `guess-media-type` feature, so that the
/// derive macro can emit a mimetype unconditionally; the value is only used
/// when the target enables the feature. Returns an empty string otherwise.
#[doc(hidden)]
pub fn mimetype_of(_path: &Path) -> String {
    #[cfg(feature = "guess-media-type")]
    {
        media_type::from_path(_path).first_or_octet_stream().to_string()
    }
    #[cfg(not(feature = "guess-media-type"))]
    {
        String::new()
    }
}

/// The `SOURCE_DATE_EPOCH` value, read at most once per process.
///
/// Used to override `last_modified` and `created` for reproducible builds.
fn source_date_epoch() -> Option<u64> {
    static SOURCE_DATE_EPOCH: OnceLock<Option<u64>> = OnceLock::new();
    *SOURCE_DATE_EPOCH.get_or_init(|| {
        std::env::var("SOURCE_DATE_EPOCH")
            .ok()
            .and_then(|value| parse_epoch(&value))
    })
}

/// Parses the `SOURCE_DATE_EPOCH` environment variable value.
fn parse_epoch(value: &str) -> Option<u64> {
    value.parse::<u64>().ok()
}

/// A cached hash, valid only while the file length and modification time are
/// unchanged.
#[derive(Clone, Copy)]
struct CachedHash {
    len: u64,
    modified: Option<SystemTime>,
    hash: [u8; 32],
}

/// Caches file hashes across calls so that repeatedly serving a file does not
/// re-hash its contents. Only used by the debug (file system) path.
fn hash_cache() -> &'static RwLock<HashMap<PathBuf, CachedHash>> {
    static HASH_CACHE: OnceLock<RwLock<HashMap<PathBuf, CachedHash>>> = OnceLock::new();
    HASH_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Reads a file from the file system and computes its metadata.
///
/// `file_path` is expected to be an absolute, canonical path inside the assets
/// folder. The hash of the file is cached and only recomputed when the file's
/// length or modification time changed. Returns an [`io::Error`] when the file
/// cannot be opened or read.
pub fn read_file_from_fs(file_path: &Path) -> io::Result<EmbeddedFile> {
    let mut file = fs::File::open(file_path)?;
    let metadata = file.metadata()?;

    let mut data = Vec::with_capacity(metadata.len() as usize);
    io::Read::read_to_end(&mut file, &mut data)?;
    let data = Cow::from(data);

    let len = metadata.len();
    let modified = metadata.modified().ok();

    let cached = hash_cache()
        .read()
        .ok()
        .and_then(|cache| cache.get(file_path).copied())
        .filter(|entry| entry.len == len && entry.modified == modified);

    let hash = match cached {
        Some(entry) => entry.hash,
        None => {
            let hash: [u8; 32] = Blake3::hash(&data)
                .as_ref()
                .try_into()
                .expect("BLAKE3 always produces a 32 byte hash");
            let cache = CachedHash {
                len,
                modified,
                hash,
            };
            if let Ok(mut cache_map) = hash_cache().write() {
                cache_map.insert(file_path.to_path_buf(), cache);
            }
            hash
        }
    };

    let last_modified = modified
        .and_then(|modified| modified.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|secs| secs.as_secs());

    let created = metadata
        .created()
        .ok()
        .and_then(|created| created.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|secs| secs.as_secs());

    #[cfg(feature = "guess-media-type")]
    let mimetype = media_type::from_path(file_path).first_or_octet_stream().to_string();

    let source_date_epoch = source_date_epoch();
    Ok(EmbeddedFile {
        data,
        metadata: Metadata {
            hash,
            last_modified: source_date_epoch.or(last_modified),
            created: source_date_epoch.or(created),
            #[cfg(feature = "guess-media-type")]
            mimetype: mimetype.into(),
        },
    })
}

#[derive(Clone)]
pub struct PathMatcher {
    #[cfg(feature = "include-exclude")]
    include_matcher: globset::GlobSet,
    #[cfg(feature = "include-exclude")]
    exclude_matcher: globset::GlobSet,
}

#[cfg(feature = "include-exclude")]
impl PathMatcher {
    pub fn new(includes: &[&str], excludes: &[&str]) -> Self {
        let mut include_matcher = globset::GlobSetBuilder::new();
        for include in includes {
            include_matcher
                .add(globset::Glob::new(include).unwrap_or_else(|_| panic!("invalid include pattern '{}'", include)));
        }
        let include_matcher = include_matcher
            .build()
            .unwrap_or_else(|_| panic!("Could not compile included patterns matcher"));

        let mut exclude_matcher = globset::GlobSetBuilder::new();
        for exclude in excludes {
            exclude_matcher
                .add(globset::Glob::new(exclude).unwrap_or_else(|_| panic!("invalid exclude pattern '{}'", exclude)));
        }
        let exclude_matcher = exclude_matcher
            .build()
            .unwrap_or_else(|_| panic!("Could not compile excluded patterns matcher"));

        Self {
            include_matcher,
            exclude_matcher,
        }
    }
    pub fn is_path_included(&self, path: &str) -> bool {
        !self.exclude_matcher.is_match(path) && (self.include_matcher.is_empty() || self.include_matcher.is_match(path))
    }
}

#[cfg(not(feature = "include-exclude"))]
impl PathMatcher {
    pub fn new(_includes: &[&str], _excludes: &[&str]) -> Self {
        Self {}
    }
    pub fn is_path_included(&self, _path: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_keys_are_accepted() {
        for key in ["a", "a.txt", "a/b/c.txt", "a-b_c.1", "…/file.txt", "a b"] {
            assert!(is_valid_key(key), "{key:?} should be valid");
        }
    }

    #[test]
    fn invalid_keys_are_rejected() {
        for key in [
            "", "/", "/abs", "a/", "a//b", "./a", "a/./b", "..", "../a", "a/..", "a\\b", "\\a", "a\0b",
        ] {
            assert!(!is_valid_key(key), "{key:?} should be invalid");
        }

        // A drive prefix is absolute on Windows and an ordinary name elsewhere.
        #[cfg(windows)]
        assert!(!is_valid_key("C:/x"));
        #[cfg(not(windows))]
        assert!(is_valid_key("C:/x"));
    }

    #[test]
    fn source_date_epoch_is_parsed() {
        assert_eq!(parse_epoch("0"), Some(0));
        assert_eq!(parse_epoch("1717171717"), Some(1_717_171_717));
        assert_eq!(parse_epoch("-1"), None);
        assert_eq!(parse_epoch("not a number"), None);
        assert_eq!(parse_epoch(""), None);
    }

    #[test]
    fn hash_is_cached_until_the_file_changes() {
        let dir = test_dir("hash_cache");
        let path = dir.join("file.txt");
        fs::write(&path, b"first").unwrap();

        let first = read_file_from_fs(&path).unwrap();
        let second = read_file_from_fs(&path).unwrap();
        assert_eq!(first.metadata.hash(), second.metadata.hash());
        assert_eq!(Blake3::hash(b"first").as_ref(), first.metadata.hash().as_slice());

        // Changing the contents (and length) must invalidate the cache.
        fs::write(&path, b"changed!").unwrap();
        let third = read_file_from_fs(&path).unwrap();
        assert_eq!(third.metadata.hash(), Blake3::hash(b"changed!").as_ref());

        fs::remove_dir_all(&dir).unwrap();
    }

    fn walk(dir: &Path, matcher: &PathMatcher) -> Vec<String> {
        let mut names: Vec<String> = get_files(dir, matcher).map(|entry| entry.rel_path).collect();
        names.sort();
        names
    }

    fn test_dir(name: &str) -> PathBuf {
        // `env::temp_dir` is unsupported on wasm32-wasip1, where the preopened
        // directory is the current directory. A UUID keeps the directory unique
        // across parallel test processes without relying on the process id,
        // which is also unsupported on wasm.
        let root = if cfg!(target_arch = "wasm32") {
            PathBuf::from(".")
        } else {
            std::env::temp_dir()
        };
        let dir = root.join(format!("embed_utils_{}_{}", name, uuid::Uuid::new_v4()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn get_files_lists_regular_files() {
        let dir = test_dir("regular");
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("a.txt"), b"a").unwrap();
        fs::write(dir.join("sub/b.txt"), b"b").unwrap();

        assert_eq!(walk(&dir, &PathMatcher::new(&[], &[])), ["a.txt", "sub/b.txt"]);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn get_files_skips_non_utf8_names() {
        let dir = test_dir("non_utf8");
        fs::write(dir.join("ok.txt"), b"ok").unwrap();

        #[cfg(unix)]
        {
            use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
            let bad = dir.join(OsStr::from_bytes(b"bad_\xff\xfe.txt"));
            fs::write(&bad, b"bad").unwrap();
        }

        assert_eq!(walk(&dir, &PathMatcher::new(&[], &[])), ["ok.txt"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn get_files_skips_backslash_names_including_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = test_dir("backslash");
        fs::write(dir.join("ok.txt"), b"ok").unwrap();
        fs::write(dir.join("target.txt"), b"target").unwrap();
        fs::write(dir.join("back\\slash.txt"), b"weird").unwrap();
        symlink(dir.join("target.txt"), dir.join("link\\name.txt")).unwrap();

        assert_eq!(walk(&dir, &PathMatcher::new(&[], &[])), ["ok.txt", "target.txt"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn get_files_keeps_in_folder_symlinks_and_drops_escaping_ones() {
        use std::os::unix::fs::symlink;

        let parent = test_dir("symlinks");
        let dir = parent.join("assets");
        let external = parent.join("external");
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(&external).unwrap();
        fs::write(dir.join("inside.txt"), b"inside").unwrap();
        fs::write(external.join("outside.txt"), b"outside").unwrap();

        // Target inside the folder: kept, with the canonical path as target.
        symlink(dir.join("inside.txt"), dir.join("inside_link.txt")).unwrap();
        // Target outside the folder: dropped.
        symlink(external.join("outside.txt"), dir.join("outside_link.txt")).unwrap();
        // Directory symlink whose target is outside: dropped with its contents.
        symlink(&external, dir.join("outside_dir")).unwrap();
        // Broken symlink: dropped.
        symlink(external.join("missing.txt"), dir.join("broken.txt")).unwrap();

        let entries: Vec<FileEntry> = get_files(&dir, &PathMatcher::new(&[], &[])).collect();
        let mut names: Vec<&str> = entries.iter().map(|entry| entry.rel_path.as_str()).collect();
        names.sort();
        assert_eq!(names, ["inside.txt", "inside_link.txt"]);

        let link = entries
            .iter()
            .find(|entry| entry.rel_path == "inside_link.txt")
            .unwrap();
        assert_eq!(link.full_canonical_path, dir.join("inside.txt").canonicalize().unwrap());

        fs::remove_dir_all(&parent).unwrap();
    }
}
