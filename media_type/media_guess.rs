//! Guessing of media types by file extension, and the reverse lookup.

use core::{cmp::Ordering, iter, slice};

#[cfg(any(feature = "alloc", feature = "reverse-lookup"))]
use crate::MediaType;

include!("media_types.rs");

#[cfg(feature = "reverse-lookup")]
include!(env!("REVERSE_LOOKUP_PATH"));

#[cfg(feature = "reverse-lookup")]
#[derive(Copy, Clone)]
struct TopLevelExts {
    start: usize,
    end: usize,
    subs: &'static [(&'static str, (usize, usize))],
}

fn get_media_types(ext: &str) -> Option<&'static [&'static str]> {
    map_lookup(MEDIA_TYPES, ext)
}

/// Get the extensions for a given top-level and sub-level of a media type
/// (`{toplevel}/{sublevel}`).
///
/// Returns `None` if `toplevel` or `sublevel` are unknown.
///
/// # Wildcards
///
/// If the top-level type is a wildcard (`*`), returns all known extensions.
///
/// If the sub-level is a wildcard, returns all extensions for the top-level type.
#[cfg(feature = "reverse-lookup")]
pub fn get_extensions(toplevel: &str, sublevel: &str) -> Option<&'static [&'static str]> {
    if toplevel == "*" {
        return Some(EXTS);
    }

    let top = map_lookup(REV_MAPPINGS, toplevel)?;

    if sublevel == "*" {
        return Some(&EXTS[top.start..top.end]);
    }

    let sub = map_lookup(top.subs, sublevel)?;
    Some(&EXTS[sub.0..sub.1])
}

/// Get a list of known extensions for a given [`MediaType`].
///
/// Parameters are ignored: only the type and subtype are searched.
///
/// Returns `None` if the media type is unknown. See [`get_extensions`] for the wildcard behavior.
#[cfg(feature = "reverse-lookup")]
pub fn get_media_extensions(media_type: &MediaType) -> Option<&'static [&'static str]> {
    get_extensions(media_type.type_(), media_type.subtype())
}

/// Get a list of known extensions for a media type string.
///
/// Parameters (everything after the first `;`) are ignored. The search is case-insensitive.
///
/// Returns `None` if `media_type` is not valid or unknown. See [`get_extensions`] for the wildcard
/// behavior.
#[cfg(feature = "reverse-lookup")]
pub fn get_media_extensions_str(mut media_type: &str) -> Option<&'static [&'static str]> {
    media_type = media_type.trim();

    if let Some(separator) = media_type.find(';') {
        media_type = &media_type[..separator];
    }

    let (toplevel, sublevel) = {
        let slash = media_type.find('/')?;
        (&media_type[..slash], &media_type[slash + 1..])
    };

    get_extensions(toplevel, sublevel)
}

/// A guess of the media type(s) of a file extension or path, as one or more [`MediaType`] values.
///
/// # Ordering
///
/// A given file format may have one or more applicable media types. The first one is the one
/// declared by the most recent IETF RFC or the one that explicitly supersedes all others; the
/// ordering of the additional ones is arbitrary.
///
/// # Values are not stable
///
/// The exact media types returned by a guess are not part of the crate's stable API and may be
/// updated in patch releases to be as accurate as possible.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MediaTypeGuess(&'static [&'static str]);

impl MediaTypeGuess {
    /// Guess the media type(s) of a file with the given extension.
    ///
    /// The search is case-insensitive. If `ext` is empty or has no known media type, an empty guess
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(media_type::from_ext("png").first(), Some(media_type::IMAGE_PNG));
    /// assert!(media_type::from_ext("").is_empty());
    /// ```
    pub fn from_ext(ext: &str) -> MediaTypeGuess {
        if ext.is_empty() {
            return MediaTypeGuess(&[]);
        }

        get_media_types(ext).map_or(MediaTypeGuess(&[]), MediaTypeGuess)
    }

    /// Guess the media type(s) of `path` by its extension (as defined by
    /// [`Path::extension`](std::path::Path::extension)). **No disk access is performed.**
    ///
    /// If `path` has no extension, its extension is not valid UTF-8, or it has no known media type,
    /// an empty guess is returned. The search is case-insensitive.
    ///
    /// # Note
    ///
    /// There is no guarantee that the contents of the file match the media type associated with its
    /// extension. Take care when making assumptions based on the return value of this function.
    #[cfg(feature = "std")]
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> MediaTypeGuess {
        path.as_ref()
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map_or(MediaTypeGuess(&[]), MediaTypeGuess::from_ext)
    }

    /// Returns `true` if the guess did not find any media type for the given extension or path.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the number of media types in this guess.
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Get the first guessed [`MediaType`], if any.
    #[cfg(feature = "alloc")]
    pub fn first(&self) -> Option<MediaType> {
        self.first_raw().map(expect_media_type)
    }

    /// Get the first guessed media type as a raw string, if any.
    pub fn first_raw(&self) -> Option<&'static str> {
        self.0.first().copied()
    }

    /// Get the first guessed [`MediaType`], or [`APPLICATION_OCTET_STREAM`] if the guess is empty.
    ///
    /// [`APPLICATION_OCTET_STREAM`]: crate::APPLICATION_OCTET_STREAM
    #[cfg(feature = "alloc")]
    pub fn first_or_octet_stream(&self) -> MediaType {
        self.first_or(crate::APPLICATION_OCTET_STREAM)
    }

    /// Get the first guessed [`MediaType`], or [`TEXT_PLAIN`] if the guess is empty.
    ///
    /// [`TEXT_PLAIN`]: crate::TEXT_PLAIN
    #[cfg(feature = "alloc")]
    pub fn first_or_text_plain(&self) -> MediaType {
        self.first_or(crate::TEXT_PLAIN)
    }

    /// Get the first guessed [`MediaType`], or the given `default` if the guess is empty.
    #[cfg(feature = "alloc")]
    pub fn first_or(&self, default: MediaType) -> MediaType {
        self.first().unwrap_or(default)
    }

    /// Get the first guessed [`MediaType`], or the result of `default` if the guess is empty.
    #[cfg(feature = "alloc")]
    pub fn first_or_else<F>(&self, default: F) -> MediaType
    where
        F: FnOnce() -> MediaType,
    {
        self.first().unwrap_or_else(default)
    }

    /// Return an iterator over the [`MediaType`] values contained in this guess.
    #[cfg(feature = "alloc")]
    pub fn iter(&self) -> Iter {
        Iter(self.iter_raw().map(expect_media_type))
    }

    /// Return an iterator over the raw media type strings contained in this guess.
    pub fn iter_raw(&self) -> IterRaw {
        IterRaw(self.0.iter().copied())
    }
}

#[cfg(feature = "alloc")]
impl IntoIterator for MediaTypeGuess {
    type Item = MediaType;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(feature = "alloc")]
impl<'a> IntoIterator for &'a MediaTypeGuess {
    type Item = MediaType;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the [`MediaType`] values of a [`MediaTypeGuess`].
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub struct Iter(iter::Map<IterRaw, fn(&'static str) -> MediaType>);

#[cfg(feature = "alloc")]
impl Iterator for Iter {
    type Item = MediaType;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

#[cfg(feature = "alloc")]
impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

#[cfg(feature = "alloc")]
impl ExactSizeIterator for Iter {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(feature = "alloc")]
impl iter::FusedIterator for Iter {}

/// An iterator over the raw media type strings of a [`MediaTypeGuess`].
#[derive(Clone, Debug)]
pub struct IterRaw(iter::Copied<slice::Iter<'static, &'static str>>);

impl Iterator for IterRaw {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for IterRaw {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl ExactSizeIterator for IterRaw {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl iter::FusedIterator for IterRaw {}

/// Guess the media type(s) of a file with the given extension.
///
/// See [`MediaTypeGuess::from_ext`].
pub fn from_ext(ext: &str) -> MediaTypeGuess {
    MediaTypeGuess::from_ext(ext)
}

/// Guess the media type(s) of `path` by its extension.
///
/// See [`MediaTypeGuess::from_path`].
#[cfg(feature = "std")]
pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> MediaTypeGuess {
    MediaTypeGuess::from_path(path)
}

#[cfg(feature = "alloc")]
fn expect_media_type(s: &str) -> MediaType {
    // The strings come from the static table, so they always parse.
    s.parse()
        .unwrap_or_else(|error| panic!("failed to parse media type {:?}: {}", s, error))
}

/// Looks up `key` in `map`, comparing ASCII letters case-insensitively.
///
/// `map` must be sorted by its keys in ascending byte order and the stored keys must be lowercase
/// ASCII, so that folding only the query keeps the ordering valid. No allocation is performed.
fn map_lookup<K, V>(map: &'static [(K, V)], key: &str) -> Option<V>
where
    K: Copy + Into<&'static str>,
    V: Copy,
{
    let mut left = 0;
    let mut right = map.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match cmp_ignore_ascii_case(map[mid].0.into(), key) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
            Ordering::Equal => return Some(map[mid].1),
        }
    }

    None
}

/// Compares two strings byte-wise, folding ASCII letters to lowercase.
fn cmp_ignore_ascii_case(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let common = a.len().min(b.len());

    for i in 0..common {
        let (ca, cb) = (a[i].to_ascii_lowercase(), b[i].to_ascii_lowercase());
        if ca != cb {
            return ca.cmp(&cb);
        }
    }

    a.len().cmp(&b.len())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_borrow)]

    #[cfg(feature = "alloc")]
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_type_bounds() {
        fn assert_bounds<T: Clone + core::fmt::Debug + Send + Sync + 'static>() {}

        assert_bounds::<MediaTypeGuess>();
        assert_bounds::<IterRaw>();
        #[cfg(feature = "alloc")]
        assert_bounds::<Iter>();
    }

    #[test]
    fn test_are_extensions_ascii() {
        for (ext, _) in MEDIA_TYPES {
            assert!(ext.is_ascii(), "extension is not ASCII: {:?}", ext);
        }
    }

    #[test]
    fn test_are_extensions_sorted() {
        // This also checks that duplicate extension entries are adjacent.
        for (&(ext, _), &(next, _)) in MEDIA_TYPES.iter().zip(MEDIA_TYPES.iter().skip(1)) {
            assert!(
                ext <= next,
                "extensions in media_types.rs must be sorted lexicographically: {:?} <= {:?}",
                ext,
                next
            );
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_are_media_types_parseable() {
        for (_, media_types) in MEDIA_TYPES {
            for value in *media_types {
                expect_media_type(value);
            }
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_media_type_guessing() {
        assert_eq!(from_ext("gif").first_or_octet_stream().to_string(), "image/gif");
        assert_eq!(from_ext("TXT").first_or_octet_stream().to_string(), "text/plain");
        assert_eq!(
            from_ext("blahblah").first_or_octet_stream().to_string(),
            "application/octet-stream"
        );
        assert_eq!(from_ext("gif").first().unwrap().to_string(), "image/gif");
        assert_eq!(from_ext("blahblah").first(), None);
        assert_eq!(from_ext("zst").first(), Some(crate::APPLICATION_ZSTD));
    }

    #[cfg(all(feature = "std", feature = "alloc"))]
    #[test]
    fn test_media_type_guessing_from_path() {
        assert_eq!(from_path("/path/to/file.gif").first(), Some(crate::IMAGE_GIF));
        assert_eq!(from_path("file").first(), None);
    }

    #[cfg(feature = "reverse-lookup")]
    #[test]
    fn test_get_media_extensions_str_no_panic_if_bad_media_type() {
        assert_eq!(get_media_extensions_str(""), None);
    }

    #[cfg(feature = "reverse-lookup")]
    #[test]
    fn test_reverse_mappings() {
        assert!(get_extensions("image", "png").unwrap().contains(&"png"));
        assert!(get_extensions("image", "*").unwrap().contains(&"png"));
        assert!(get_extensions("*", "*").unwrap().contains(&"png"));
        assert_eq!(get_extensions("nope", "nope"), None);
        assert!(get_media_extensions(&crate::IMAGE_PNG).unwrap().contains(&"png"));
    }
}
