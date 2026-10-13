//! Load files into the Rust binary at compile time.
//!
//! A custom derive macro that embeds files into the binary in release builds
//! and reads them from the file system during development. Use it to bundle
//! CSS, JS, and images into a single executable which can be deployed to your
//! servers, and to build small container images.
//!
//! The folder path given to `#[folder]` is resolved as follows:
//!
//! - In `debug` builds, and when the `debug-embed` feature is not enabled, it
//!   is resolved relative to the directory the binary is run from.
//! - In `release` builds, or when the `debug-embed` feature is enabled, it is
//!   resolved relative to the directory containing `Cargo.toml`.
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
//! # Features
//!
//! - `debug-embed`: always embed the files in the binary, even in debug builds.
//! - `guess-media-type`: add `EmbeddedFile::mimetype()` and guess each file's
//!   media type from its path.
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
/// some cases. Do not try and match on it, as different variants will exist
/// depending on the compilation context.
pub enum Filenames {
    /// Release builds use a named iterator type, which can be stack-allocated.
    #[cfg(any(not(debug_assertions), feature = "debug-embed"))]
    Embedded(std::slice::Iter<'static, &'static str>),

    /// The debug iterator type is currently unnameable and still needs to be
    /// boxed.
    #[cfg(all(debug_assertions, not(feature = "debug-embed")))]
    Dynamic(Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>>),
}

impl Iterator for Filenames {
    type Item = std::borrow::Cow<'static, str>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            #[cfg(any(not(debug_assertions), feature = "debug-embed"))]
            Filenames::Embedded(names) => names.next().map(|x| std::borrow::Cow::from(*x)),

            #[cfg(all(debug_assertions, not(feature = "debug-embed")))]
            Filenames::Dynamic(boxed) => boxed.next(),
        }
    }
}
