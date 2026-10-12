use std::borrow::Cow;

use crate::{
    encoders::{base64::MIME, encode::BodyEncoding},
    headers::{
        Header, HeaderType,
        content_type::ContentType,
        fold::{FoldWriter, write_control_free},
        message_id::MessageId,
        raw::Raw,
        text::Text,
    },
    writer::Writer,
};

/// MIME part of an e-mail.
#[derive(Clone, Debug)]
pub struct MimePart<'x> {
    pub headers: Vec<(Cow<'x, str>, HeaderType<'x>)>,
    pub contents: BodyPart<'x>,
}

#[derive(Clone, Debug)]
pub enum BodyPart<'x> {
    Text(Cow<'x, str>),
    Binary(Cow<'x, [u8]>),
    Multipart(Vec<MimePart<'x>>),
}

impl<'x> From<&'x str> for BodyPart<'x> {
    fn from(value: &'x str) -> Self {
        BodyPart::Text(value.into())
    }
}

impl<'x> From<&'x [u8]> for BodyPart<'x> {
    fn from(value: &'x [u8]) -> Self {
        BodyPart::Binary(value.into())
    }
}

impl From<String> for BodyPart<'_> {
    fn from(value: String) -> Self {
        BodyPart::Text(value.into())
    }
}

impl<'x> From<&'x String> for BodyPart<'x> {
    fn from(value: &'x String) -> Self {
        BodyPart::Text(value.as_str().into())
    }
}

impl<'x> From<Cow<'x, str>> for BodyPart<'x> {
    fn from(value: Cow<'x, str>) -> Self {
        BodyPart::Text(value)
    }
}

impl From<Vec<u8>> for BodyPart<'_> {
    fn from(value: Vec<u8>) -> Self {
        BodyPart::Binary(value.into())
    }
}

impl<'x> From<Vec<MimePart<'x>>> for BodyPart<'x> {
    fn from(value: Vec<MimePart<'x>>) -> Self {
        BodyPart::Multipart(value)
    }
}

impl<'x> From<&'x str> for ContentType<'x> {
    fn from(value: &'x str) -> Self {
        ContentType::new(value)
    }
}

impl From<String> for ContentType<'_> {
    fn from(value: String) -> Self {
        ContentType::new(value)
    }
}

impl<'x> From<&'x String> for ContentType<'x> {
    fn from(value: &'x String) -> Self {
        ContentType::new(value.as_str())
    }
}

const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Writes `value` as lowercase hexadecimal digits without allocating.
#[inline]
fn write_hex(output: &mut impl Writer, mut value: u64) {
    let mut buffer = [0u8; 16];
    let mut at = buffer.len();
    loop {
        at -= 1;
        if let Some(slot) = buffer.get_mut(at) {
            *slot = HEX_DIGITS[(value & 0x0F) as usize];
        }
        value >>= 4;
        if value == 0 || at == 0 {
            break;
        }
    }
    output.write(buffer.get(at..).unwrap_or_default());
}

/// Writes a random MIME boundary without allocating.
///
/// The boundary is 128 random bits split into two hexadecimal fields joined by
/// `separator`, which should be a non-empty string. Boundaries never contain
/// characters that are forbidden in a MIME boundary, nor any whitespace.
pub fn write_boundary(output: &mut impl Writer, separator: &str) {
    write_hex(output, rand::random::<u64>());
    output.write(separator.as_bytes());
    write_hex(output, rand::random::<u64>());
}

/// Returns a random MIME boundary. See [`write_boundary`].
pub fn make_boundary(separator: &str) -> String {
    let mut boundary = Vec::with_capacity(2 * 16 + separator.len());
    write_boundary(&mut boundary, separator);
    // The output is ASCII, so it is always valid UTF-8.
    String::from_utf8(boundary).unwrap_or_default()
}

/// Returns a random boundary that does not occur at the start of a line in any
/// of `parts`.
///
/// RFC 2046 section 5.1 requires that a boundary delimiter "MUST NOT appear
/// inside any of the encapsulated parts", so a generated boundary is verified
/// against the part contents before it is used. The chance of a false match is
/// negligible, but checking keeps messages unambiguous even when a part body
/// happens to contain an attacker-chosen value.
fn fresh_boundary(parts: &[MimePart<'_>]) -> String {
    loop {
        let boundary = make_boundary("_");
        if !parts
            .iter()
            .any(|part| part_contains_boundary(part, boundary.as_bytes()))
        {
            return boundary;
        }
    }
}

fn part_contains_boundary(part: &MimePart<'_>, boundary: &[u8]) -> bool {
    match &part.contents {
        BodyPart::Text(text) => contains_boundary_line(text.as_bytes(), boundary),
        BodyPart::Binary(binary) => contains_boundary_line(binary, boundary),
        BodyPart::Multipart(children) => children.iter().any(|child| part_contains_boundary(child, boundary)),
    }
}

/// Returns true when `data` contains a line beginning with `--boundary`.
///
/// Only the beginning of each line is compared, matching the way RFC 2046
/// section 5.1.1 says boundary delimiters must be recognised.
fn contains_boundary_line(data: &[u8], boundary: &[u8]) -> bool {
    let mut start = 0;
    while start <= data.len() {
        let rest = data.get(start..).unwrap_or_default();
        if rest.starts_with(b"--") && rest.get(2..).is_some_and(|tail| tail.starts_with(boundary)) {
            return true;
        }
        match rest.iter().position(|&byte| byte == b'\n') {
            Some(position) => start += position + 1,
            None => break,
        }
    }
    false
}

impl<'x> MimePart<'x> {
    /// Create a new MIME part.
    pub fn new(content_type: impl Into<ContentType<'x>>, contents: impl Into<BodyPart<'x>>) -> Self {
        let mut content_type = content_type.into();
        let contents = contents.into();

        if matches!(contents, BodyPart::Text(_)) && content_type.attributes.is_empty() {
            content_type.attributes.reserve_exact(1);
            content_type.attributes.push((Cow::from("charset"), Cow::from("utf-8")));
        }

        let mut headers = Vec::with_capacity(2);
        headers.push((Cow::from("Content-Type"), content_type.into()));

        Self {
            contents,
            headers,
        }
    }

    /// Create a new raw MIME part that includes both headers and body.
    pub fn raw(contents: impl Into<BodyPart<'x>>) -> Self {
        Self {
            contents: contents.into(),
            headers: vec![],
        }
    }

    /// Set the attachment filename of a MIME part.
    pub fn attachment(mut self, filename: impl Into<Cow<'x, str>>) -> Self {
        self.headers.push((
            "Content-Disposition".into(),
            ContentType::new("attachment").attribute("filename", filename).into(),
        ));
        self
    }

    /// Set the MIME part as inline.
    pub fn inline(mut self) -> Self {
        self.headers
            .push(("Content-Disposition".into(), ContentType::new("inline").into()));
        self
    }

    /// Set the Content-Language header of a MIME part.
    pub fn language(mut self, value: impl Into<Cow<'x, str>>) -> Self {
        self.headers.push(("Content-Language".into(), Text::new(value).into()));
        self
    }

    /// Set the Content-ID header of a MIME part.
    pub fn cid(mut self, value: impl Into<Cow<'x, str>>) -> Self {
        self.headers.push(("Content-ID".into(), MessageId::new(value).into()));
        self
    }

    /// Set the Content-Location header of a MIME part.
    pub fn location(mut self, value: impl Into<Cow<'x, str>>) -> Self {
        self.headers.push(("Content-Location".into(), Raw::new(value).into()));
        self
    }

    /// Disable automatic Content-Transfer-Encoding detection and treat this as a raw MIME part
    pub fn transfer_encoding(mut self, value: impl Into<Cow<'x, str>>) -> Self {
        self.headers
            .push(("Content-Transfer-Encoding".into(), Raw::new(value).into()));
        self
    }

    /// Set custom headers of a MIME part.
    pub fn header(mut self, header: impl Into<Cow<'x, str>>, value: impl Into<HeaderType<'x>>) -> Self {
        self.headers.push((header.into(), value.into()));
        self
    }

    /// Returns the part's size
    pub fn size(&self) -> usize {
        match &self.contents {
            BodyPart::Text(b) => b.len(),
            BodyPart::Binary(b) => b.len(),
            BodyPart::Multipart(bl) => bl.iter().map(|b| b.size()).sum(),
        }
    }

    pub(crate) fn estimated_len(&self) -> usize {
        self.estimated_len_at_depth(0)
    }

    fn estimated_len_at_depth(&self, depth: usize) -> usize {
        let headers = estimated_headers_len(&self.headers);
        match &self.contents {
            BodyPart::Text(text) => headers
                .saturating_add(MIME.encoded_len(text.len()))
                .saturating_add(LEAF_OVERHEAD),
            BodyPart::Binary(binary) => headers
                .saturating_add(MIME.encoded_len(binary.len()))
                .saturating_add(LEAF_OVERHEAD),
            BodyPart::Multipart(parts) if depth < MAX_ESTIMATE_DEPTH => {
                parts
                    .iter()
                    .fold(headers.saturating_add(MULTIPART_OVERHEAD), |total, part| {
                        total
                            .saturating_add(part.estimated_len_at_depth(depth + 1))
                            .saturating_add(BOUNDARY_OVERHEAD)
                    })
            }
            BodyPart::Multipart(parts) => headers
                .saturating_add(MULTIPART_OVERHEAD)
                .saturating_add(parts.len().saturating_mul(BOUNDARY_OVERHEAD)),
        }
    }

    /// Add a body part to a multipart/* MIME part.
    pub fn add_part(&mut self, part: MimePart<'x>) {
        if let BodyPart::Multipart(ref mut parts) = self.contents {
            parts.push(part);
        }
    }

    /// Write the MIME part to a writer.
    pub fn write_part(self, output: &mut impl Writer) {
        let children = match self.contents {
            BodyPart::Text(text) => {
                return write_leaf_part(&self.headers, text.as_bytes(), true, output);
            }
            BodyPart::Binary(binary) => {
                return write_leaf_part(&self.headers, binary.as_ref(), false, output);
            }
            BodyPart::Multipart(children) => children,
        };

        let mut boundary = write_multipart_headers(self.headers, &children, output);
        let mut parts = children.into_iter();
        let mut stack: Vec<(std::vec::IntoIter<MimePart<'x>>, Cow<'x, str>)> = Vec::new();

        loop {
            while let Some(part) = parts.next() {
                output.write(b"\r\n--");
                output.write(boundary.as_bytes());
                output.write(b"\r\n");

                match part.contents {
                    BodyPart::Text(text) => {
                        write_leaf_part(&part.headers, text.as_bytes(), true, output);
                    }
                    BodyPart::Binary(binary) => {
                        write_leaf_part(&part.headers, binary.as_ref(), false, output);
                    }
                    BodyPart::Multipart(children) => {
                        stack.push((parts, boundary));
                        boundary = write_multipart_headers(part.headers, &children, output);
                        parts = children.into_iter();
                    }
                }
            }

            output.write(b"\r\n--");
            output.write(boundary.as_bytes());
            output.write(b"--\r\n");

            match stack.pop() {
                Some((previous_parts, previous_boundary)) => {
                    parts = previous_parts;
                    boundary = previous_boundary;
                }
                None => return,
            }
        }
    }
}

const MAX_ESTIMATE_DEPTH: usize = 32;
const LEAF_OVERHEAD: usize = 96;
const MULTIPART_OVERHEAD: usize = 128;
const BOUNDARY_OVERHEAD: usize = 64;

pub(crate) fn estimated_headers_len(headers: &[(Cow<'_, str>, HeaderType<'_>)]) -> usize {
    headers.iter().map(|(name, _)| name.len() + HEADER_VALUE_ESTIMATE).sum()
}

const HEADER_VALUE_ESTIMATE: usize = 64;

fn write_leaf_part(
    headers: &[(Cow<'_, str>, HeaderType<'_>)],
    body: &[u8],
    is_text_body: bool,
    output: &mut impl Writer,
) {
    let mut is_text = is_text_body;
    let mut is_attachment = false;
    let mut is_raw = headers.is_empty();

    for (header_name, header_value) in headers {
        write_header_name(header_name, output);

        if !is_text && header_name.eq_ignore_ascii_case("Content-Type") {
            is_text = header_value.as_content_type().is_some_and(|value| value.is_text());
        } else if !is_attachment && header_name.eq_ignore_ascii_case("Content-Disposition") {
            is_attachment = header_value
                .as_content_type()
                .is_some_and(|value| value.is_attachment());
        } else if !is_raw && header_name.eq_ignore_ascii_case("Content-Transfer-Encoding") {
            is_raw = true;
        }

        header_value.write_header(output, header_name.len() + 2);
    }

    if !is_raw {
        if is_text {
            BodyEncoding::for_text(body, !is_attachment).write(body, output);
        } else {
            BodyEncoding::Base64.write(body, output);
        }
    } else {
        if !headers.is_empty() {
            output.write(b"\r\n");
        }
        output.write(body);
    }
}

fn write_multipart_headers<'x>(
    headers: Vec<(Cow<'x, str>, HeaderType<'x>)>,
    children: &[MimePart<'x>],
    output: &mut impl Writer,
) -> Cow<'x, str> {
    let mut boundary: Option<Cow<'x, str>> = None;

    for (header_name, header_value) in headers {
        write_header_name(&header_name, output);

        if boundary.is_none() && header_name.eq_ignore_ascii_case("Content-Type") {
            boundary = Some(match header_value {
                HeaderType::ContentType(mut content_type) => {
                    let position = match content_type
                        .attributes
                        .iter()
                        .position(|(attribute, _)| attribute.eq_ignore_ascii_case("boundary"))
                    {
                        Some(position) => position,
                        None => {
                            let position = content_type.attributes.len();
                            content_type
                                .attributes
                                .push(("boundary".into(), fresh_boundary(children).into()));
                            position
                        }
                    };
                    content_type.write_header(output, 14);
                    content_type.attributes.swap_remove(position).1
                }
                HeaderType::Raw(raw) => raw_boundary(raw, children, output),
                HeaderType::Text(text) => raw_boundary(Raw::new(text.text), children, output),
                other => {
                    other.write_header(output, header_name.len() + 2);
                    continue;
                }
            });
        } else {
            header_value.write_header(output, header_name.len() + 2);
        }
    }

    let boundary = boundary.unwrap_or_else(|| {
        output.write(b"Content-Type: ");
        let boundary = fresh_boundary(children);
        ContentType::new("multipart/mixed")
            .attribute("boundary", &boundary)
            .write_header(output, 14);
        boundary.into()
    });

    output.write(b"\r\n");
    boundary
}

fn raw_boundary<'x>(raw: Raw<'x>, children: &[MimePart<'x>], output: &mut impl Writer) -> Cow<'x, str> {
    match raw.raw.find("boundary=\"") {
        Some(position) => {
            raw.write_header(output, 14);
            match raw.raw {
                Cow::Borrowed(value) => value
                    .get(position..)
                    .and_then(|tail| tail.split('"').nth(1))
                    .map_or_else(|| fresh_boundary(children).into(), Cow::Borrowed),
                Cow::Owned(value) => value
                    .get(position..)
                    .and_then(|tail| tail.split('"').nth(1))
                    .map_or_else(|| fresh_boundary(children), str::to_string)
                    .into(),
            }
        }
        None => {
            let boundary = fresh_boundary(children);
            let mut folder = FoldWriter::new(output, 14);
            write_control_free(&mut folder, raw.raw.as_bytes());
            folder.write(b"; boundary=\"");
            folder.write(boundary.as_bytes());
            folder.write(b"\"");
            folder.finish();
            boundary.into()
        }
    }
}

/// Writes a header name, silently dropping the characters that could otherwise
/// end the field and inject another header (CR, LF and other controls). Header
/// names are ASCII by definition, so this only removes invalid bytes.
pub(crate) fn write_header_name(name: &str, output: &mut impl Writer) {
    let mut start = 0;
    for (at, byte) in name.bytes().enumerate() {
        if byte <= 0x20 || byte == 0x7F {
            if at > start {
                output.write(name.get(start..at).unwrap_or_default().as_bytes());
            }
            start = at + 1;
        }
    }
    if start < name.len() {
        output.write(name.get(start..).unwrap_or_default().as_bytes());
    }
    output.write(b": ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unexpected_multipart_content_type_values_do_not_panic() {
        let mut output = Vec::new();
        MimePart::raw(vec![MimePart::new("text/plain", "hello")])
            .header("Content-Type", Text::new("multipart/mixed"))
            .write_part(&mut output);
        let output = String::from_utf8(output).unwrap();
        assert!(output.starts_with("Content-Type: multipart/mixed; boundary=\""), "{output:?}");
        assert!(output.ends_with("--\r\n"), "{output:?}");

        let mut output = Vec::new();
        MimePart::raw(vec![MimePart::new("text/plain", "hello")])
            .header("Content-Type", MessageId::new("id@example.org"))
            .write_part(&mut output);
        let output = String::from_utf8(output).unwrap();
        assert!(
            output.contains("Content-Type: multipart/mixed;") && output.contains("boundary=\""),
            "{output:?}"
        );
    }

    #[test]
    fn raw_content_type_with_boundary_is_written_and_reused() {
        let mut output = Vec::new();
        MimePart::raw(vec![MimePart::new("text/plain", "hello")])
            .header("Content-Type", Raw::new("multipart/mixed; boundary=\"abc\""))
            .write_part(&mut output);
        let output = String::from_utf8(output).unwrap();
        assert!(
            output.starts_with("Content-Type: multipart/mixed; boundary=\"abc\"\r\n\r\n\r\n--abc\r\n"),
            "{output:?}"
        );
        assert!(output.ends_with("\r\n--abc--\r\n"), "{output:?}");
    }

    fn is_boundary_safe(value: &str) -> bool {
        !value.is_empty()
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-' | b'+' | b'=' | b':' | b'/')
            })
    }

    #[test]
    fn boundary_has_a_safe_shape() {
        let boundary = make_boundary("_");
        assert_eq!(boundary.split('_').count(), 2, "{boundary}");
        assert!(
            boundary
                .split('_')
                .all(|field| !field.is_empty() && field.bytes().all(|b| b.is_ascii_hexdigit())),
            "{boundary}"
        );
        assert!(boundary.len() <= 70, "{boundary}");
        assert!(is_boundary_safe(&boundary), "{boundary}");
        assert_eq!(make_boundary(".").split('.').count(), 2);
    }

    #[test]
    fn boundaries_are_unique_across_threads() {
        let per_thread = 5_000;
        let threads = (0..8)
            .map(|_| std::thread::spawn(move || (0..per_thread).map(|_| make_boundary("_")).collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        let mut seen = std::collections::HashSet::with_capacity(per_thread * 8);
        for thread in threads {
            for boundary in thread.join().expect("thread panicked") {
                assert!(seen.insert(boundary), "duplicate boundary");
            }
        }
        assert_eq!(seen.len(), per_thread * 8);
    }

    #[test]
    fn detects_boundary_lines_only_at_start_of_line() {
        assert!(contains_boundary_line(b"--abc\r\n", b"abc"));
        assert!(contains_boundary_line(b"x\r\n--abc\r\n", b"abc"));
        assert!(contains_boundary_line(b"x\n--abc--\r\n", b"abc"));
        assert!(!contains_boundary_line(b"x--abc\r\n", b"abc"));
        assert!(!contains_boundary_line(b"	--abc\r\n", b"abc"));
        assert!(!contains_boundary_line(b"", b"abc"));
    }

    #[test]
    fn header_names_are_sanitized() {
        let mut output = Vec::new();
        write_header_name("X-Evil\r\nBcc", &mut output);
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "X-EvilBcc: ");
        assert!(!output.contains('\n'));
    }

    #[test]
    fn estimates_cover_the_output() {
        let ascii = "lorem ipsum dolor sit amet ".repeat(2_000);
        let latin = "réunion d'équipe: résumé ".repeat(2_000);
        let cjk = "안녕하세요 세계 ".repeat(2_000);
        let parts = vec![
            MimePart::new("text/plain", ascii.as_str()),
            MimePart::new("text/plain", latin.as_str()),
            MimePart::new("text/plain", cjk.as_str()),
            MimePart::new("image/png", (0..200_000u32).map(|value| value as u8).collect::<Vec<_>>()),
            MimePart::new(
                "multipart/alternative",
                vec![
                    MimePart::new("text/plain", "short"),
                    MimePart::new("text/html", "<p>short</p>"),
                ],
            ),
        ];
        for part in parts {
            let estimate = part.estimated_len();
            let mut output = Vec::new();
            part.write_part(&mut output);
            assert!(estimate >= output.len(), "estimate {estimate} below output {}", output.len());
        }
    }
}
