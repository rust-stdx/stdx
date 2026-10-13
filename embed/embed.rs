//! Load files into the Rust binary at compile time.
//!
//! A custom derive macro that embeds files into the binary in release builds
//! and reads them from the file system during development. Use it to bundle
//! CSS, JS, and images into a single executable which can be deployed to your
//! servers, and to build small container images.
//!
//! The folder path given to `#[folder]` is resolved as follows:
//!
//! - In `release` builds, or when the `debug-embed` feature is enabled, the
//!   files are embedded into the binary and the folder is resolved relative to
//!   the directory containing `Cargo.toml` at build time.
//! - In `debug` builds, and when the `debug-embed` feature is not enabled, the
//!   files are read from the file system at runtime. The folder captured at
//!   build time (relative to the directory containing `Cargo.toml`) is used if
//!   it still exists; otherwise the folder is resolved relative to the
//!   directory the binary is run from. The fallback keeps debug builds working
//!   when the build directory is unavailable, for example when the binary was
//!   moved to another machine or runs in a sandbox such as WASI.
//!
//! The folder must be a directory; deriving `Embed` on a path that does not
//! exist or is not a directory is a compile-time error.
//!
//! Add the `Embed` derive to a struct with a `folder` attribute pointing at the
//! static folder:
//!
//! ```
//! use embed::Embed;
//!
//! #[derive(Embed)]
//! #[folder = "examples/public/"]
//! struct Asset;
//! ```
//!
//! The macro generates two associated functions:
//!
//! - `get(file_path: &str) -> Option<embed::EmbeddedFile>` returns the file at
//!   a path relative to the assets folder.
//! - `iter()` iterates over the relative paths of the files in the folder.
//!
//! When the files are embedded (release builds, or with `debug-embed`), the
//! data is returned as a `Cow::Borrowed(&'static [u8])` and the file list is a
//! static array. Otherwise the data is read from the file system on each call
//! and the file list is produced by walking the folder.
//!
//! An [`EmbeddedFile`] holds the file `data` and its [`Metadata`].
//!
//! # Lookup paths
//!
//! `get` accepts the same relative paths that `iter` yields: `/`-separated
//! paths with no empty, `.` or `..` component and no backslash, for example
//! `"index.html"` or `"images/llama.png"`. Any other input returns `None`
//! instead of being normalized, so `get("./index.html")`,
//! `get("images/../index.html")` and absolute paths always return `None`. This
//! guarantees that debug builds behave exactly like release builds: in both
//! cases the only accepted inputs are the exact paths listed by `iter`.
//!
//! Paths are relative to the folder and never escape it, including through
//! symbolic links: a symlink whose target is outside the folder is ignored by
//! `iter` and returns `None` from `get`, in debug and release builds alike.
//! Symlinks that stay inside the folder are followed normally.
//!
//! Files whose names are not valid UTF-8, or whose names contain a backslash,
//! are skipped when the folder is walked (they cannot be addressed by a `&str`
//! path). This matches the behaviour of Go's `embed` package.
//!
//! Two limits are worth knowing about:
//!
//! - On case-insensitive file systems (for example the default macOS
//!   configuration), a debug lookup may resolve a path whose casing differs
//!   from the stored name, while embedded lookups are always case sensitive.
//!   Pass names exactly as returned by `iter`.
//! - Debug lookups check the canonical path and then read it. A local attacker
//!   able to modify the assets folder while the program runs could swap a path
//!   between the check and the read; making this race-free requires
//!   handle-relative APIs that are not available in safe `std`. Release builds
//!   are unaffected, since the data is embedded.
//!
//! # Attributes
//!
//! ## `prefix`
//!
//! `#[prefix = "my_prefix/"]` prepends a prefix to every file path. The prefix
//! is required on `get` calls and is included in the paths returned by `iter`.
//!
//! ## `metadata_only`
//!
//! `#[metadata_only = true]` excludes the file contents from the binary. Only
//! file paths and metadata are embedded.
//!
//! ## `crate_path`
//!
//! `#[crate_path = "some::path::Embed"]` names the crate that provides the
//! `EmbeddedFile`, `Metadata` and `RustEmbed` items used by the generated code.
//! It defaults to `embed` and is meant for re-exporting scenarios.
//!
//! # Features
//!
//! - `debug-embed`: always embed the files in the binary, even in debug builds.
//! - `guess-media-type`: add `Metadata::mimetype()` and guess each file's
//!   media type from its path.
//!
//! # Reproducible builds
//!
//! When the `SOURCE_DATE_EPOCH` environment variable is set, its value (in
//! seconds since the UNIX epoch) is used for both
//! [`Metadata::last_modified`] and [`Metadata::created`], so builds of the same
//! sources produce the same metadata. It is read at most once per process.
//!
//! # Example
//!
//! ```
//! use embed::Embed;
//!
//! #[derive(Embed)]
//! #[folder = "examples/public/"]
//! #[prefix = "prefix/"]
//! struct Asset;
//!
//! fn main() {
//!     let index_html = Asset::get("prefix/index.html").unwrap();
//!     println!("{:?}", std::str::from_utf8(index_html.data.as_ref()));
//!
//!     for file in Asset::iter() {
//!         println!("{}", file.as_ref());
//!     }
//! }
//! ```
#![forbid(unsafe_code)]
pub use embed_utils::{EmbeddedFile, Metadata};
#[cfg(feature = "compression")]
#[cfg_attr(feature = "compression", doc(hidden))]
pub use include_flate::flate;

#[doc(hidden)]
pub extern crate embed_utils as utils;

/// A directory of binary assets.
///
/// The files in the specified folder will be embedded into the executable in
/// release builds. Debug builds will read the data from the file system at
/// runtime.
///
/// This trait is meant to be derived like so:
/// ```
/// use embed::Embed;
///
/// #[derive(Embed)]
/// #[folder = "examples/public/"]
/// struct Asset;
///
/// fn main() {}
/// ```
pub trait RustEmbed {
    /// Get an embedded file and its metadata.
    ///
    /// If the feature `debug-embed` is enabled or the binary was compiled in
    /// release mode, the file information is embedded in the binary and the file
    /// data is returned as a `Cow::Borrowed(&'static [u8])`.
    ///
    /// Otherwise, the information is read from the file system on each call and
    /// the file data is returned as a `Cow::Owned(Vec<u8>)`.
    ///
    /// Returns `None` for paths that are not valid relative paths (see the
    /// crate documentation) and for files that do not exist.
    fn get(file_path: &str) -> Option<EmbeddedFile>;

    /// Iterates over the file paths in the folder.
    ///
    /// If the feature `debug-embed` is enabled or the binary is compiled in
    /// release mode, a static array containing the list of relative file paths
    /// is used.
    ///
    /// Otherwise, the files are listed from the file system on each call.
    fn iter() -> Filenames;
}

pub use embed_impl::RustEmbed as Embed;

/// An iterator over filenames.
///
/// This enum exists for optimization purposes, to avoid boxing the iterator in
/// some cases. Do not try and match on it: which variant is constructed depends
/// on the compilation context.
///
/// Both variants are always defined, even though only one of them is ever
/// constructed for a given build. Gating them on `debug_assertions` would make
/// the shape of this enum depend on the profile of the crate that compiles it,
/// which breaks down when the crate and its users are not built with the same
/// profile (for example rustdoc, which builds doctests with debug assertions
/// enabled while linking the release library).
pub enum Filenames {
    /// Release builds use a named iterator type, which can be stack-allocated.
    Embedded(std::slice::Iter<'static, &'static str>),

    /// The debug iterator type is currently unnameable and still needs to be
    /// boxed.
    Dynamic(Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>>),
}

impl Iterator for Filenames {
    type Item = std::borrow::Cow<'static, str>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Filenames::Embedded(names) => names.next().map(|x| std::borrow::Cow::from(*x)),
            Filenames::Dynamic(boxed) => boxed.next(),
        }
    }
}
