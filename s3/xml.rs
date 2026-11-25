//! Minimal, allocation-friendly XML tree used for the S3 REST API.
//!
//! S3 responses are small, shallow documents. Building a tiny element tree is
//! simpler and safer than deriving a full XML/serde data model, and it keeps
//! the decoder independent from any serialization framework.
//!
//! Namespaces are ignored: only the local name of each element is compared.
//! Text content is unescaped and whitespace-trimmed.

use quick_xml::{Reader, escape::unescape, events::Event};

/// A parsed XML element.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Element {
    /// Local element name, without any namespace prefix.
    pub name: String,
    /// Concatenated text directly contained in this element.
    pub text: String,
    /// Direct child elements, in document order.
    pub children: Vec<Element>,
}

impl Element {
    /// Parses a complete XML document and returns its root element.
    ///
    /// Returns [`quick_xml::Error`] when the input is not well-formed XML, and
    /// [`quick_xml::escape::EscapeError`] when a text node contains an invalid
    /// escape sequence.
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let mut reader = Reader::from_str(input);
        let mut stack: Vec<Element> = Vec::new();
        let mut root: Option<Element> = None;

        loop {
            match reader.read_event()? {
                Event::Start(event) => stack.push(Element {
                    name: local_name(event.local_name().as_ref()),
                    text: String::new(),
                    children: Vec::new(),
                }),
                Event::Empty(event) => {
                    let element = Element {
                        name: local_name(event.local_name().as_ref()),
                        text: String::new(),
                        children: Vec::new(),
                    };
                    attach(element, &mut stack, &mut root);
                }
                Event::Text(event) => {
                    if let Some(element) = stack.last_mut() {
                        let text = unescape(&event)?;
                        element.text.push_str(text.as_ref());
                    }
                }
                Event::CData(event) => {
                    if let Some(element) = stack.last_mut() {
                        element.text.push_str(event.as_ref());
                    }
                }
                Event::GeneralRef(event) => {
                    if let Some(element) = stack.last_mut() {
                        element.text.push_str(&decode_entity(event.as_ref())?);
                    }
                }
                Event::End(_) => {
                    let element = stack.pop().ok_or(ParseError::UnexpectedEnd)?;
                    attach(element, &mut stack, &mut root);
                }
                Event::Eof => break,
                _ => {}
            }
        }

        if !stack.is_empty() {
            return Err(ParseError::UnexpectedEof);
        }
        root.ok_or(ParseError::Empty)
    }

    /// Returns the first direct child with the given local name.
    pub fn child(&self, name: &str) -> Option<&Self> {
        self.children.iter().find(|child| child.name == name)
    }

    /// Iterates over the direct children with the given local name.
    pub fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Self> + 'a {
        self.children.iter().filter(move |child| child.name == name)
    }

    /// Returns the trimmed text of the first matching child.
    pub fn child_text(&self, name: &str) -> Option<&str> {
        self.child(name).map(Element::text)
    }

    /// Returns this element's trimmed text, if any.
    pub fn text(&self) -> &str {
        self.text.trim()
    }

    /// Returns the text of the first matching child, or an error when the
    /// element is missing or empty.
    pub fn required_text(&self, name: &str) -> Result<&str, ParseError> {
        self.child(name)
            .map(Element::text)
            .filter(|text| !text.is_empty())
            .ok_or_else(|| ParseError::Missing(name.to_string()))
    }
}

fn attach(element: Element, stack: &mut [Element], root: &mut Option<Element>) {
    match stack.last_mut() {
        Some(parent) => parent.children.push(element),
        None => *root = Some(element),
    }
}

fn decode_entity(reference: &str) -> Result<String, ParseError> {
    let decoded = match reference {
        "amp" => "&".to_string(),
        "lt" => "<".to_string(),
        "gt" => ">".to_string(),
        "quot" => "\"".to_string(),
        "apos" => "'".to_string(),
        _ => {
            let number = reference
                .strip_prefix("#x")
                .or_else(|| reference.strip_prefix("#X"))
                .map(|hex| u32::from_str_radix(hex, 16))
                .or_else(|| reference.strip_prefix('#').map(str::parse::<u32>))
                .ok_or_else(|| ParseError::Escape(format!("unknown entity &{reference};")))?;
            let number = number.map_err(|_| ParseError::Escape(format!("invalid entity &{reference};")))?;
            char::from_u32(number)
                .ok_or_else(|| ParseError::Escape(format!("invalid character reference &{reference};")))?
                .to_string()
        }
    };
    Ok(decoded)
}

fn local_name(raw: &str) -> String {
    let name = raw;
    match name.rsplit_once(':') {
        Some((_, local)) => local.to_string(),
        None => name.to_string(),
    }
}

/// Error returned by [`Element::parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParseError {
    /// The XML parser reported a malformed document.
    Xml(String),
    /// A text node contained an invalid escape sequence.
    Escape(String),
    /// An end tag appeared without a matching start tag.
    UnexpectedEnd,
    /// The document ended before all start tags were closed.
    UnexpectedEof,
    /// The document contained no root element.
    Empty,
    /// A required element was missing or empty.
    Missing(String),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::Xml(message) => formatter.write_str(message),
            ParseError::Escape(message) => formatter.write_str(message),
            ParseError::UnexpectedEnd => formatter.write_str("unexpected closing tag"),
            ParseError::UnexpectedEof => formatter.write_str("unexpected end of document"),
            ParseError::Empty => formatter.write_str("empty XML document"),
            ParseError::Missing(name) => write!(formatter, "missing <{name}> element"),
        }
    }
}

impl core::error::Error for ParseError {}

impl From<quick_xml::Error> for ParseError {
    fn from(error: quick_xml::Error) -> Self {
        Self::Xml(error.to_string())
    }
}

impl From<quick_xml::escape::EscapeError> for ParseError {
    fn from(error: quick_xml::escape::EscapeError) -> Self {
        Self::Escape(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn parses_nested_elements_and_unescapes_text() {
        let root =
            Element::parse("<Root><Name>a &amp; b</Name><Item><Key>x</Key></Item><Item><Key>y</Key></Item></Root>")
                .unwrap();
        assert_eq!(root.name, "Root");
        assert_eq!(root.child_text("Name"), Some("a & b"));
        let keys: Vec<_> = root
            .children_named("Item")
            .map(|item| item.child_text("Key").unwrap())
            .collect();
        assert_eq!(keys, ["x", "y"]);
    }

    #[test]
    fn ignores_namespace_prefixes_and_whitespace() {
        let root = Element::parse("<ns:Root xmlns:ns=\"urn:x\"><ns:Name> value </ns:Name></ns:Root>").unwrap();
        assert_eq!(root.name, "Root");
        assert_eq!(root.child_text("Name"), Some("value"));
    }

    #[test]
    fn rejects_malformed_documents() {
        assert!(Element::parse("<Root><Child></Root>").is_err());
        assert!(Element::parse("<Root>").is_err());
        assert!(Element::parse("").is_err());
    }
}
