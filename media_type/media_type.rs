//! Media types (formerly known as MIME types).
//!
//! This crate provides the [`MediaType`] type to parse and inspect media types such as
//! `text/plain; charset=utf-8` or `image/svg+xml`, a curated set of constants for the most common
//! types, and helpers to guess a media type from a file extension or path.
//!
//! # Parsing
//!
//! ```
//! use media_type::MediaType;
//!
//! let media_type: MediaType = "text/plain; charset=utf-8".parse().unwrap();
//! assert_eq!(media_type, media_type::TEXT_PLAIN_UTF_8);
//! assert_eq!(media_type.type_(), "text");
//! assert_eq!(media_type.subtype(), "plain");
//! assert_eq!(media_type.get_parameter("charset"), Some("utf-8"));
//! ```
//!
//! # Guessing from a path or extension
//!
//! The file does not have to exist: only its extension is inspected.
//!
//! ```
//! let guess = media_type::from_path("images/logo.png");
//! assert_eq!(guess.first(), Some(media_type::IMAGE_PNG));
//!
//! let guess = media_type::from_ext("zst");
//! assert_eq!(guess.first(), Some(media_type::APPLICATION_ZSTD));
//! ```
//!
//! # Parsing a list of media types
//!
//! HTTP header fields such as `Accept` or `Content-Type` may contain a comma-separated list of
//! media types. [`MediaTypeIter`] extracts the valid ones and reports the fragments that could not
//! be parsed:
//!
//! ```
//! use media_type::MediaTypeIter;
//!
//! let header = "text/html, application/json, invalid, text/xml";
//! let parsed: Vec<_> = MediaTypeIter::new(header).collect();
//!
//! assert_eq!(parsed.len(), 4);
//! assert_eq!(parsed[0].as_ref().unwrap(), &media_type::TEXT_HTML);
//! assert_eq!(parsed[1].as_ref().unwrap(), &media_type::APPLICATION_JSON);
//! assert_eq!(*parsed[2].as_ref().unwrap_err(), "invalid");
//! assert_eq!(parsed[3].as_ref().unwrap(), &media_type::TEXT_XML);
//! ```
//!
//! # Cargo features
//!
//! | Feature | Default | Description |
//! | --- | --- | --- |
//! | `std` | yes | Enables `std`-only conveniences such as [`from_path`] and [`MediaTypeGuess::from_path`]. Implies `alloc`. |
//! | `alloc` | yes (via `std`) | Enables the owned, allocating API: parsing, [`FromStr`](core::str::FromStr), [`MediaTypeIter`], [`MediaTypeGuess::first`], formatting, ... Without it, only the constants, the [`MediaType`] views, and the extension lookup are available. |
//! | `reverse-lookup` | yes | Generates and exposes the media type -> extensions reverse lookup ([`get_extensions`], [`get_media_extensions`], [`get_media_extensions_str`]). Disable it to shrink the binary for size-constrained builds. |
//!
//! This crate is `no_std` by default. Disable the default features to use it without `std`:
//!
//! - without `alloc`: only the constants, [`MediaType`] views, and the extension lookup are
//!   available;
//! - with `alloc`: parsing, formatting, [`MediaTypeIter`], and [`MediaTypeGuess::first`] become
//!   available;
//! - with `std`: [`from_path`] and [`MediaTypeGuess::from_path`] are also available.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "alloc")]
use alloc::string::String;
use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

mod media_guess;
#[cfg(feature = "alloc")]
mod parse;

#[cfg(feature = "alloc")]
pub use media_guess::Iter;
#[cfg(feature = "std")]
pub use media_guess::from_path;
pub use media_guess::{IterRaw, MediaTypeGuess, from_ext};
#[cfg(feature = "reverse-lookup")]
pub use media_guess::{get_extensions, get_media_extensions, get_media_extensions_str};
#[cfg(feature = "alloc")]
pub use parse::{FromStrError, MediaTypeIter};

#[cfg(feature = "alloc")]
impl core::str::FromStr for MediaType {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<MediaType, Self::Err> {
        parse::parse(s).map_err(FromStrError::new)
    }
}

/// The source string of a [`MediaType`].
#[derive(Clone)]
enum Source {
    /// A compile-time known string, used by the crate constants.
    Static(&'static str),
    /// An owned string produced while parsing.
    #[cfg(feature = "alloc")]
    Dynamic(String),
}

impl Source {
    #[inline]
    fn as_str(&self) -> &str {
        match self {
            Source::Static(source) => source,
            #[cfg(feature = "alloc")]
            Source::Dynamic(source) => source.as_str(),
        }
    }
}

/// A parsed media type (formerly known as a MIME type).
///
/// The type and subtype (including any `+suffix`) are compared case-insensitively, as are the
/// parameter names and the value of the `charset` parameter. Other parameter values are compared
/// case-sensitively.
///
/// Values of this type are always stored in their canonical form: the type, subtype and parameter
/// names are lowercase, the parameters are separated by `; `, and parameter values are unquoted.
///
/// # Examples
///
/// ```
/// use media_type::MediaType;
///
/// let media_type: MediaType = "IMAGE/SVG+XML; Charset=UTF-8".parse().unwrap();
/// assert_eq!(media_type.type_(), "image");
/// assert_eq!(media_type.subtype(), "svg");
/// assert_eq!(media_type.suffix(), Some("xml"));
/// assert_eq!(media_type.essence(), "image/svg+xml");
/// assert_eq!(media_type.to_string(), "image/svg+xml; charset=utf-8");
/// ```
#[derive(Clone)]
pub struct MediaType {
    source: Source,
}

impl MediaType {
    /// Creates a media type from a `'static` string without validating it.
    ///
    /// The string **must** be a valid media type in canonical lowercase form, otherwise the
    /// accessors may return unexpected results. Prefer [`MediaType::from_str`](core::str::FromStr)
    /// for untrusted input.
    ///
    /// This is mostly useful to define your own constants:
    ///
    /// ```
    /// use media_type::MediaType;
    ///
    /// const TEXT_X_LOG: MediaType = MediaType::from_static("text/x-log");
    /// assert_eq!(TEXT_X_LOG.subtype(), "x-log");
    /// ```
    pub const fn from_static(source: &'static str) -> MediaType {
        MediaType {
            source: Source::Static(source),
        }
    }

    /// Get the top-level media type for this `MediaType`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(media_type::TEXT_PLAIN.type_(), "text");
    /// assert_eq!(media_type::TEXT_PLAIN.type_(), media_type::TEXT);
    /// ```
    #[inline]
    pub fn type_(&self) -> &str {
        let source = self.source.as_str();
        &source[..source.find('/').unwrap_or(source.len())]
    }

    /// Get the subtype of this `MediaType`, without its `+suffix`.
    ///
    /// # Examples
    ///
    /// ```
    /// use media_type::MediaType;
    ///
    /// assert_eq!(media_type::TEXT_PLAIN.subtype(), "plain");
    /// let svg: MediaType = "image/svg+xml".parse().unwrap();
    /// assert_eq!(svg.subtype(), "svg");
    /// ```
    #[inline]
    pub fn subtype(&self) -> &str {
        let source = self.source.as_str();
        let start = source.find('/').map_or(0, |i| i + 1);
        let rest = &source[start..];
        let end = rest.find(['+', ';']).unwrap_or(rest.len());
        &rest[..end]
    }

    /// Get the optional `+suffix` of this `MediaType`.
    ///
    /// # Examples
    ///
    /// ```
    /// use media_type::MediaType;
    ///
    /// assert_eq!(media_type::TEXT_PLAIN.suffix(), None);
    /// let svg: MediaType = "image/svg+xml".parse().unwrap();
    /// assert_eq!(svg.suffix(), Some("xml"));
    /// ```
    #[inline]
    pub fn suffix(&self) -> Option<&str> {
        let source = self.source.as_str();
        let slash = source.find('/').unwrap_or(0);
        let plus = slash + source[slash..].find('+')?;
        let start = plus + 1;
        let end = source[start..].find(';').map_or(source.len(), |i| start + i);
        Some(&source[start..end])
    }

    /// Get the "essence" of this `MediaType`, that is the type and subtype without any parameter.
    ///
    /// See the [WHATWG definition][essence].
    ///
    /// [essence]: https://mimesniff.spec.whatwg.org/#mime-type-essence
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(media_type::TEXT_PLAIN_UTF_8.essence(), "text/plain");
    /// ```
    #[inline]
    pub fn essence(&self) -> &str {
        let source = self.source.as_str();
        &source[..source.find(';').unwrap_or(source.len())]
    }

    /// Return an iterator over the parameters of this `MediaType`.
    ///
    /// # Examples
    ///
    /// ```
    /// use media_type::MediaType;
    ///
    /// let media_type: MediaType = "text/plain; charset=utf-8; foo=bar".parse().unwrap();
    /// let params: Vec<_> = media_type.parameters().collect();
    /// assert_eq!(params, [("charset", "utf-8"), ("foo", "bar")]);
    /// ```
    #[inline]
    pub fn parameters(&self) -> Parameters<'_> {
        Parameters::new(self.source.as_str())
    }

    /// Look up a parameter by name.
    ///
    /// The name is matched case-insensitively. Parameter values are returned verbatim, except for
    /// the `charset` parameter whose value is lowercased.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(media_type::TEXT_PLAIN_UTF_8.get_parameter("charset"), Some("utf-8"));
    /// assert_eq!(media_type::TEXT_PLAIN_UTF_8.get_parameter("boundary"), None);
    /// ```
    #[inline]
    pub fn get_parameter<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.parameters()
            .find(|(parameter, _)| parameter.eq_ignore_ascii_case(name))
            .map(|(_, value)| value)
    }
}

/// Compare a `MediaType` against a raw media type string.
fn media_type_eq_str(media_type: &MediaType, other: &str) -> bool {
    let source = media_type.source.as_str();

    // Fast path: the whole canonical string matches.
    if source.eq_ignore_ascii_case(other) {
        return true;
    }

    // Compare the essence and then each parameter in order.
    let other_essence = &other[..other.find(';').unwrap_or(other.len())];
    if !media_type.essence().eq_ignore_ascii_case(other_essence) {
        return false;
    }

    let mut left = media_type.parameters();
    let mut right = Parameters::new(other);

    loop {
        match (left.next(), right.next()) {
            (None, None) => return true,
            (Some((name, value)), Some((other_name, other_value))) => {
                if !name.eq_ignore_ascii_case(other_name) {
                    return false;
                }
                let insensitive = name.eq_ignore_ascii_case(CHARSET);
                let equal = if insensitive {
                    value.eq_ignore_ascii_case(other_value)
                } else {
                    value == other_value
                };
                if !equal {
                    return false;
                }
            }
            _ => return false,
        }
    }
}

impl PartialEq for MediaType {
    #[inline]
    fn eq(&self, other: &MediaType) -> bool {
        self.source.as_str() == other.source.as_str()
    }
}

impl Eq for MediaType {}

impl PartialOrd for MediaType {
    #[inline]
    fn partial_cmp(&self, other: &MediaType) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MediaType {
    #[inline]
    fn cmp(&self, other: &MediaType) -> Ordering {
        self.source.as_str().cmp(other.source.as_str())
    }
}

impl Hash for MediaType {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.as_str().hash(state);
    }
}

impl PartialEq<&str> for MediaType {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        media_type_eq_str(self, other)
    }
}

impl PartialEq<MediaType> for &str {
    #[inline]
    fn eq(&self, other: &MediaType) -> bool {
        media_type_eq_str(other, self)
    }
}

impl AsRef<str> for MediaType {
    #[inline]
    fn as_ref(&self) -> &str {
        self.source.as_str()
    }
}

impl fmt::Display for MediaType {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.source.as_str())
    }
}

impl fmt::Debug for MediaType {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.source.as_str(), formatter)
    }
}

/// An iterator over the parameters of a [`MediaType`].
///
/// Each item is a `(name, value)` pair. See [`MediaType::parameters`].
#[derive(Clone)]
pub struct Parameters<'a> {
    remaining: Option<&'a str>,
}

impl<'a> Parameters<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            remaining: source.find(';').map(|index| &source[index + 1..]),
        }
    }
}

impl fmt::Debug for Parameters<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Parameters").finish()
    }
}

impl<'a> Iterator for Parameters<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining?;
        let remaining = remaining.trim_start_matches([' ', '\t']);
        if remaining.is_empty() {
            self.remaining = None;
            return None;
        }

        let equals = remaining.find('=')?;
        let name = remaining[..equals].trim_end();
        let after_equals = &remaining[equals + 1..];

        if let Some(quoted) = after_equals.strip_prefix('"') {
            let close = quoted.find('"')?;
            let value = &quoted[..close];
            let rest = &quoted[close + 1..];
            self.remaining = rest.find(';').map(|i| &rest[i + 1..]);
            Some((name, value))
        } else {
            let end = after_equals.find(';').unwrap_or(after_equals.len());
            let value = after_equals[..end].trim_end();
            self.remaining = if end < after_equals.len() {
                Some(&after_equals[end + 1..])
            } else {
                None
            };
            Some((name, value))
        }
    }
}

macro_rules! names {
    ($($(#[$meta:meta])* $name:ident = $value:literal;)*) => {
        $(
            $(#[$meta])*
            #[doc = concat!("`", $value, "`")]
            pub const $name: &str = $value;
        )*
    };
}

macro_rules! media_types {
    ($($(#[$meta:meta])* $name:ident = $value:literal;)*) => {
        $(
            $(#[$meta])*
            #[doc = concat!("`", $value, "`")]
            pub const $name: MediaType = MediaType::from_static($value);
        )*
    };
}

names! {
    /// The `*` wildcard.
    STAR = "*";

    /// The `text` top-level type.
    TEXT = "text";
    /// The `image` top-level type.
    IMAGE = "image";
    /// The `audio` top-level type.
    AUDIO = "audio";
    /// The `video` top-level type.
    VIDEO = "video";
    /// The `application` top-level type.
    APPLICATION = "application";
    /// The `multipart` top-level type.
    MULTIPART = "multipart";
    /// The `message` top-level type.
    MESSAGE = "message";
    /// The `model` top-level type.
    MODEL = "model";
    /// The `font` top-level type.
    FONT = "font";

    /// The `plain` subtype.
    PLAIN = "plain";
    /// The `html` subtype.
    HTML = "html";
    /// The `xml` subtype or `+xml` suffix.
    XML = "xml";
    /// The `javascript` subtype.
    JAVASCRIPT = "javascript";
    /// The `css` subtype.
    CSS = "css";
    /// The `csv` subtype.
    CSV = "csv";
    /// The `event-stream` subtype.
    EVENT_STREAM = "event-stream";
    /// The `vcard` subtype.
    VCARD = "vcard";
    /// The `markdown` subtype.
    MARKDOWN = "markdown";
    /// The `bmp` subtype.
    BMP = "bmp";
    /// The `gif` subtype.
    GIF = "gif";
    /// The `jpeg` subtype.
    JPEG = "jpeg";
    /// The `png` subtype.
    PNG = "png";
    /// The `svg` subtype.
    SVG = "svg";
    /// The `webp` subtype.
    WEBP = "webp";
    /// The `avif` subtype.
    AVIF = "avif";
    /// The `json` subtype.
    JSON = "json";
    /// The `x-www-form-urlencoded` subtype.
    WWW_FORM_URLENCODED = "x-www-form-urlencoded";
    /// The `msgpack` subtype.
    MSGPACK = "msgpack";
    /// The `octet-stream` subtype.
    OCTET_STREAM = "octet-stream";
    /// The `pdf` subtype.
    PDF = "pdf";
    /// The `zstd` subtype.
    ZSTD = "zstd";
    /// The `brotli` subtype.
    BROTLI = "brotli";
    /// The `gzip` subtype.
    GZIP = "gzip";
    /// The `x-tar` subtype.
    X_TAR = "x-tar";
    /// The `zip` subtype.
    ZIP = "zip";
    /// The `x-7z-compressed` subtype.
    X_7Z_COMPRESSED = "x-7z-compressed";
    /// The `wasm` subtype.
    WASM = "wasm";
    /// The `protobuf` subtype.
    PROTOBUF = "protobuf";
    /// The `yaml` subtype.
    YAML = "yaml";
    /// The `woff` subtype.
    WOFF = "woff";
    /// The `woff2` subtype.
    WOFF2 = "woff2";
    /// The `ttf` subtype.
    TTF = "ttf";
    /// The `otf` subtype.
    OTF = "otf";
    /// The `form-data` subtype.
    FORM_DATA = "form-data";
    /// The `mpeg` subtype.
    MPEG = "mpeg";
    /// The `mp4` subtype.
    MP4 = "mp4";
    /// The `ogg` subtype.
    OGG = "ogg";
    /// The `opus` subtype.
    OPUS = "opus";
    /// The `flac` subtype.
    FLAC = "flac";
    /// The `webm` subtype.
    WEBM = "webm";

    /// The `charset` parameter name.
    CHARSET = "charset";
    /// The `boundary` parameter name.
    BOUNDARY = "boundary";
    /// The `utf-8` charset value.
    UTF_8 = "utf-8";
}

media_types! {
    /// `*/*`
    STAR_STAR = "*/*";

    /// `text/*`
    TEXT_STAR = "text/*";
    /// `text/plain`
    TEXT_PLAIN = "text/plain";
    /// `text/plain; charset=utf-8`
    TEXT_PLAIN_UTF_8 = "text/plain; charset=utf-8";
    /// `text/html`
    TEXT_HTML = "text/html";
    /// `text/html; charset=utf-8`
    TEXT_HTML_UTF_8 = "text/html; charset=utf-8";
    /// `text/css`
    TEXT_CSS = "text/css";
    /// `text/css; charset=utf-8`
    TEXT_CSS_UTF_8 = "text/css; charset=utf-8";
    /// `text/javascript`
    TEXT_JAVASCRIPT = "text/javascript";
    /// `text/xml`
    TEXT_XML = "text/xml";
    /// `text/event-stream`
    TEXT_EVENT_STREAM = "text/event-stream";
    /// `text/csv`
    TEXT_CSV = "text/csv";
    /// `text/csv; charset=utf-8`
    TEXT_CSV_UTF_8 = "text/csv; charset=utf-8";
    /// `text/tab-separated-values`
    TEXT_TAB_SEPARATED_VALUES = "text/tab-separated-values";
    /// `text/tab-separated-values; charset=utf-8`
    TEXT_TAB_SEPARATED_VALUES_UTF_8 = "text/tab-separated-values; charset=utf-8";
    /// `text/vcard`
    TEXT_VCARD = "text/vcard";
    /// `text/markdown`
    TEXT_MARKDOWN = "text/markdown";

    /// `image/*`
    IMAGE_STAR = "image/*";
    /// `image/jpeg`
    IMAGE_JPEG = "image/jpeg";
    /// `image/gif`
    IMAGE_GIF = "image/gif";
    /// `image/png`
    IMAGE_PNG = "image/png";
    /// `image/bmp`
    IMAGE_BMP = "image/bmp";
    /// `image/svg+xml`
    IMAGE_SVG = "image/svg+xml";
    /// `image/webp`
    IMAGE_WEBP = "image/webp";
    /// `image/avif`
    IMAGE_AVIF = "image/avif";

    /// `font/woff`
    FONT_WOFF = "font/woff";
    /// `font/woff2`
    FONT_WOFF2 = "font/woff2";
    /// `font/ttf`
    FONT_TTF = "font/ttf";
    /// `font/otf`
    FONT_OTF = "font/otf";

    /// `application/json`
    APPLICATION_JSON = "application/json";
    /// `application/javascript`
    APPLICATION_JAVASCRIPT = "application/javascript";
    /// `application/javascript; charset=utf-8`
    APPLICATION_JAVASCRIPT_UTF_8 = "application/javascript; charset=utf-8";
    /// `application/x-www-form-urlencoded`
    APPLICATION_WWW_FORM_URLENCODED = "application/x-www-form-urlencoded";
    /// `application/octet-stream`
    APPLICATION_OCTET_STREAM = "application/octet-stream";
    /// `application/msgpack`
    APPLICATION_MSGPACK = "application/msgpack";
    /// `application/pdf`
    APPLICATION_PDF = "application/pdf";
    /// `application/zstd`
    APPLICATION_ZSTD = "application/zstd";
    /// `application/brotli`
    APPLICATION_BROTLI = "application/brotli";
    /// `application/gzip`
    APPLICATION_GZIP = "application/gzip";
    /// `application/x-tar`
    APPLICATION_X_TAR = "application/x-tar";
    /// `application/zip`
    APPLICATION_ZIP = "application/zip";
    /// `application/x-7z-compressed`
    APPLICATION_X_7Z_COMPRESSED = "application/x-7z-compressed";
    /// `application/wasm`
    APPLICATION_WASM = "application/wasm";
    /// `application/protobuf`
    APPLICATION_PROTOBUF = "application/protobuf";
    /// `application/yaml`
    APPLICATION_YAML = "application/yaml";

    /// `multipart/form-data`
    MULTIPART_FORM_DATA = "multipart/form-data";

    /// `audio/opus`
    AUDIO_OPUS = "audio/opus";
    /// `audio/flac`
    AUDIO_FLAC = "audio/flac";

    /// `video/mp4`
    VIDEO_MP4 = "video/mp4";
    /// `video/webm`
    VIDEO_WEBM = "video/webm";
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_type_and_subtype() {
        assert_eq!(TEXT_PLAIN.type_(), TEXT);
        assert_eq!(TEXT_PLAIN.subtype(), PLAIN);
        assert_eq!(TEXT_PLAIN_UTF_8.subtype(), PLAIN);
        let svg: MediaType = "image/svg+xml".parse().unwrap();
        assert_eq!(svg.type_(), IMAGE);
        assert_eq!(svg.subtype(), SVG);
        assert_eq!(svg.suffix(), Some(XML));
        assert_eq!(TEXT_PLAIN.suffix(), None);
    }

    #[test]
    fn test_essence() {
        assert_eq!(TEXT_PLAIN.essence(), "text/plain");
        assert_eq!(TEXT_PLAIN_UTF_8.essence(), "text/plain");
        assert_eq!(IMAGE_SVG.essence(), "image/svg+xml");
    }

    #[test]
    fn test_get_parameter() {
        assert_eq!(TEXT_PLAIN.get_parameter("charset"), None);
        assert_eq!(TEXT_PLAIN_UTF_8.get_parameter(CHARSET), Some(UTF_8));

        let media_type: MediaType = "text/plain; charset=utf-8; foo=bar".parse().unwrap();
        assert_eq!(media_type.get_parameter("CHARSET"), Some("utf-8"));
        assert_eq!(media_type.get_parameter("foo"), Some("bar"));
        assert_eq!(media_type.get_parameter("baz"), None);
    }

    #[test]
    fn test_case_sensitive_parameter_values() {
        let media_type: MediaType = "multipart/form-data; charset=BASE64; boundary=ABCDEFG".parse().unwrap();
        assert_eq!(media_type.get_parameter(CHARSET), Some("base64"));
        assert_eq!(media_type.get_parameter(BOUNDARY), Some("ABCDEFG"));
        assert_ne!(media_type.get_parameter(BOUNDARY), Some("abcdefg"));
    }

    #[test]
    fn test_equality() {
        assert_eq!("TEXT/PLAIN".parse::<MediaType>().unwrap(), TEXT_PLAIN);
        assert_eq!("text/plain;charset=utf-8".parse::<MediaType>().unwrap(), TEXT_PLAIN_UTF_8);
        assert_eq!("text/plain; charset=\"utf-8\"".parse::<MediaType>().unwrap(), TEXT_PLAIN_UTF_8);
        assert_eq!(TEXT_PLAIN, "text/PLAIN");
        assert_eq!("text/plain", TEXT_PLAIN);
        assert_eq!(TEXT_PLAIN_UTF_8, "text/plain;charset=UTF-8");
        assert_eq!(STAR_STAR, "*/*");
        assert_eq!("image/*".parse::<MediaType>().unwrap(), IMAGE_STAR);
    }

    #[test]
    fn test_display() {
        assert_eq!(TEXT_PLAIN.to_string(), "text/plain");
        assert_eq!(TEXT_PLAIN_UTF_8.to_string(), "text/plain; charset=utf-8");
    }

    #[test]
    fn test_from_static_constant() {
        const TEXT_X_LOG: MediaType = MediaType::from_static("text/x-log");
        assert_eq!(TEXT_X_LOG.subtype(), "x-log");
        assert_eq!(TEXT_X_LOG.essence(), "text/x-log");
    }
}
