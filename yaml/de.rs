//! Parsing YAML text into the dynamic [`Value`] model.

use alloc::{
    borrow::Cow,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    Error, Mapping, Number, Options, Result, Value,
    libyaml::{
        error::Error as LibyamlError,
        parser::{Anchor, Event, Parser, Scalar, ScalarStyle},
    },
    options::DuplicateKeys,
};

/// Parses the first YAML document from `input` into `T`.
pub fn from_str<T: crate::Decode>(input: &str) -> Result<T> {
    from_str_with_options(input, Options::default())
}

/// Parses the first YAML document from `input` into `T` with custom limits.
pub fn from_str_with_options<T: crate::Decode>(input: &str, options: Options) -> Result<T> {
    from_slice_with_options(input.as_bytes(), options)
}

/// Parses the first YAML document from `input` into `T`.
pub fn from_slice<T: crate::Decode>(input: &[u8]) -> Result<T> {
    from_slice_with_options(input, Options::default())
}

/// Parses the first YAML document from `input` into `T` with custom limits.
pub fn from_slice_with_options<T: crate::Decode>(input: &[u8], options: Options) -> Result<T> {
    let value = from_str_value_with_options(input, options)?;
    T::decode(&value)
}

/// Parses the first YAML document into a dynamic [`Value`] with custom limits.
///
/// A stream with no documents (empty input or comments only) yields
/// [`Value::Null`]. A malformed or over-limit later document is not parsed.
pub fn from_str_value_with_options(input: &[u8], options: Options) -> Result<Value> {
    ensure_input_size(input, options)?;
    let mut parser = Parser::new(Cow::Borrowed(input));
    let mut documents = ParserState::new(options, true).parse(&mut parser)?;
    Ok(documents.pop().unwrap_or(Value::Null))
}

/// Parses every document in `input` into `T`.
pub fn from_str_multi<T: crate::Decode>(input: &str) -> Result<Vec<T>> {
    from_str_multi_with_options(input, Options::default())
}

/// Parses every document in `input` into `T` with custom limits.
pub fn from_str_multi_with_options<T: crate::Decode>(input: &str, options: Options) -> Result<Vec<T>> {
    from_slice_multi_with_options(input.as_bytes(), options)
}

/// Parses every document in `input` into `T`.
pub fn from_slice_multi<T: crate::Decode>(input: &[u8]) -> Result<Vec<T>> {
    from_slice_multi_with_options(input, Options::default())
}

/// Parses every document in `input` into `T` with custom limits.
pub fn from_slice_multi_with_options<T: crate::Decode>(input: &[u8], options: Options) -> Result<Vec<T>> {
    from_slice_multi_value(input, options)?.iter().map(T::decode).collect()
}

/// Parses every document in `input` into a dynamic [`Value`].
///
/// An empty stream yields an empty vector. Use [`from_str`] when an empty
/// stream should be treated as a single null document.
pub fn from_slice_multi_value(input: &[u8], options: Options) -> Result<Vec<Value>> {
    ensure_input_size(input, options)?;
    let mut parser = Parser::new(Cow::Borrowed(input));
    ParserState::new(options, false).parse(&mut parser)
}

fn ensure_input_size(input: &[u8], options: Options) -> Result<()> {
    if let Some(limit) = options.max_input_bytes
        && input.len() > limit
    {
        return Err(Error::new(alloc::format!("YAML input exceeds the {limit} byte limit")));
    }
    Ok(())
}

#[cfg(feature = "std")]
/// Parses the first document from a reader.
///
/// The default [`Options::max_input_bytes`] limit is enforced while reading.
pub fn from_reader<T: crate::Decode, R: std::io::Read>(reader: R) -> Result<T> {
    from_reader_with_options(reader, Options::default())
}

#[cfg(feature = "std")]
/// Parses the first document from a reader with custom limits.
pub fn from_reader_with_options<T: crate::Decode, R: std::io::Read>(mut reader: R, options: Options) -> Result<T> {
    use std::io::Read as _;

    let mut input = Vec::new();
    match options.max_input_bytes {
        Some(limit) => {
            reader
                .by_ref()
                .take(limit.saturating_add(1) as u64)
                .read_to_end(&mut input)
                .map_err(|error| Error::new(alloc::format!("failed to read YAML input: {error}")))?;
            if input.len() > limit {
                return Err(Error::new(alloc::format!("YAML input exceeds the {limit} byte limit")));
            }
        }
        None => {
            reader
                .read_to_end(&mut input)
                .map_err(|error| Error::new(alloc::format!("failed to read YAML input: {error}")))?;
        }
    }
    from_slice_with_options(&input, options)
}

struct ParserState {
    options: Options,
    stack: Vec<Frame>,
    anchors: BTreeMap<Vec<u8>, Value>,
    root: Option<Value>,
    documents: Vec<Value>,
    values: usize,
    aliases: usize,
    first_only: bool,
}

enum Frame {
    Sequence {
        values: Vec<Value>,
        anchor: Option<Anchor>,
        tag: Option<String>,
    },
    Mapping {
        values: Mapping,
        key: Option<Value>,
        merge_key: bool,
        merges: Vec<Value>,
        anchor: Option<Anchor>,
        tag: Option<String>,
    },
}

impl ParserState {
    fn new(options: Options, first_only: bool) -> Self {
        Self {
            options,
            stack: Vec::new(),
            anchors: BTreeMap::new(),
            root: None,
            documents: Vec::new(),
            values: 0,
            aliases: 0,
            first_only,
        }
    }

    fn parse(mut self, parser: &mut Parser<'_>) -> Result<Vec<Value>> {
        loop {
            let (event, _mark) = parser.next().map_err(convert_error)?;
            match event {
                Event::StreamStart => {}
                Event::DocumentStart => {
                    self.stack.clear();
                    self.anchors.clear();
                    self.root = None;
                    // The limits are documented per document, not per stream.
                    self.values = 0;
                    self.aliases = 0;
                }
                Event::StreamEnd => {
                    if self.root.is_some() {
                        self.finish_document()?;
                    }
                    break;
                }
                Event::DocumentEnd => {
                    self.finish_document()?;
                    if self.first_only {
                        break;
                    }
                }
                Event::Alias(anchor) => {
                    self.aliases += 1;
                    if let Some(limit) = self.options.max_aliases
                        && self.aliases > limit
                    {
                        return Err(Error::new("YAML alias limit exceeded"));
                    }
                    let value = self
                        .anchors
                        .get(anchor.as_bytes())
                        .cloned()
                        .ok_or_else(|| Error::new("unknown YAML alias"))?;
                    self.account_materialized(&value)?;
                    self.push(value, false)?;
                }
                Event::Scalar(scalar) => {
                    let value = scalar_value(&scalar)?;
                    let merge_key = is_merge_scalar(&scalar);
                    if let Some(anchor) = &scalar.anchor {
                        self.account_materialized(&value)?;
                        self.anchors.insert(anchor.as_bytes().to_vec(), value.clone());
                    }
                    self.push(value, merge_key)?;
                }
                Event::SequenceStart(start) => {
                    self.check_depth()?;
                    self.stack.push(Frame::Sequence {
                        values: Vec::new(),
                        anchor: start.anchor,
                        tag: tag_string(start.tag.as_ref()),
                    });
                }
                Event::MappingStart(start) => {
                    self.check_depth()?;
                    self.stack.push(Frame::Mapping {
                        values: Mapping::new(),
                        key: None,
                        merge_key: false,
                        merges: Vec::new(),
                        anchor: start.anchor,
                        tag: tag_string(start.tag.as_ref()),
                    });
                }
                Event::SequenceEnd => {
                    let frame = self.stack.pop().ok_or_else(|| Error::new("unexpected sequence end"))?;
                    let Frame::Sequence {
                        values,
                        anchor,
                        tag,
                    } = frame
                    else {
                        return Err(Error::new("mismatched YAML container"));
                    };
                    self.finish_container(Value::Sequence(values), anchor, tag)?;
                }
                Event::MappingEnd => {
                    let frame = self.stack.pop().ok_or_else(|| Error::new("unexpected mapping end"))?;
                    let Frame::Mapping {
                        mut values,
                        key,
                        merges,
                        anchor,
                        tag,
                        ..
                    } = frame
                    else {
                        return Err(Error::new("mismatched YAML container"));
                    };
                    if key.is_some() {
                        return Err(Error::new("YAML mapping ended with a key but no value"));
                    }
                    for merged in merges {
                        let Value::Mapping(merged) = merged.into_untagged() else {
                            continue;
                        };
                        for (key, value) in merged.iter() {
                            if values.get(key).is_none() {
                                self.account_materialized(key)?;
                                self.account_materialized(value)?;
                                values.insert(key.clone(), value.clone());
                            }
                        }
                    }
                    self.finish_container(Value::Mapping(values), anchor, tag)?;
                }
            }
        }

        if !self.stack.is_empty() {
            return Err(Error::new("unexpected end of YAML document"));
        }
        Ok(self.documents)
    }

    fn finish_document(&mut self) -> Result<()> {
        if !self.stack.is_empty() {
            return Err(Error::new("unexpected end of YAML document"));
        }
        let value = self.root.take().unwrap_or(Value::Null);
        if let Some(limit) = self.options.max_documents
            && self.documents.len() >= limit
        {
            return Err(Error::new("YAML document limit exceeded"));
        }
        self.documents.push(value);
        Ok(())
    }

    fn check_depth(&self) -> Result<()> {
        if self.stack.len() >= self.options.max_depth {
            return Err(Error::new("YAML nesting depth limit exceeded"));
        }
        Ok(())
    }

    /// Charges `value` to the materialized-value budget.
    ///
    /// The parser builds one owned copy of every anchored node and every
    /// merge-inserted key/value. Those copies are not events, so they have to
    /// be charged explicitly; otherwise `max_values` would not bound memory.
    fn account_materialized(&mut self, value: &Value) -> Result<()> {
        if let Some(limit) = self.options.max_values {
            self.values = self.values.saturating_add(count_values(value));
            if self.values > limit {
                return Err(Error::new("YAML value limit exceeded"));
            }
        }
        Ok(())
    }

    fn finish_container(&mut self, value: Value, anchor: Option<Anchor>, tag: Option<String>) -> Result<()> {
        let value = apply_tag(value, tag)?;
        if let Some(anchor) = anchor {
            self.account_materialized(&value)?;
            self.anchors.insert(anchor.as_bytes().to_vec(), value.clone());
        }
        self.push(value, false)
    }

    fn push(&mut self, value: Value, merge_key: bool) -> Result<()> {
        if let Some(limit) = self.options.max_values {
            self.values += 1;
            if self.values > limit {
                return Err(Error::new("YAML value limit exceeded"));
            }
        }
        match self.stack.last_mut() {
            None => self.root = Some(value),
            Some(Frame::Sequence {
                values, ..
            }) => values.push(value),
            Some(Frame::Mapping {
                values,
                key,
                merge_key: pending_merge,
                merges,
                ..
            }) => match key.take() {
                None => {
                    *key = Some(value);
                    *pending_merge = merge_key;
                }
                Some(_) if *pending_merge && is_merge_value(&value) => {
                    *pending_merge = false;
                    match value.into_untagged() {
                        Value::Mapping(mapping) => merges.push(Value::Mapping(mapping)),
                        Value::Sequence(items) => merges.extend(items),
                        _ => unreachable!(),
                    }
                }
                Some(key) => {
                    *pending_merge = false;
                    insert_pair(values, key, value, self.options.duplicate_keys)?;
                }
            },
        }
        Ok(())
    }
}

fn insert_pair(values: &mut Mapping, key: Value, value: Value, policy: DuplicateKeys) -> Result<()> {
    if values.get(&key).is_some() {
        match policy {
            DuplicateKeys::Error => return Err(Error::new(alloc::format!("duplicate mapping key `{key:?}`"))),
            DuplicateKeys::First => return Ok(()),
            DuplicateKeys::Last => {}
        }
    }
    values.insert(key, value);
    Ok(())
}

/// Returns true for a node that YAML can resolve as a merge key.
///
/// The merge key is a plain, untagged `<<` scalar (or one explicitly tagged
/// `tag:yaml.org,2002:merge`). Quoted scalars and `!!str <<` are ordinary keys.
fn is_merge_scalar(scalar: &Scalar<'_>) -> bool {
    match scalar.tag.as_ref() {
        // Implicit merge keys must be plain scalars: quoting `<<` makes it an
        // ordinary string key.
        None => scalar.style == ScalarStyle::Plain && scalar.value.as_ref() == b"<<",
        // An explicit `!!merge` tag takes precedence over the presentation
        // style, so `!!merge '<<'` is still a merge key.
        Some(tag) => tag.eq("tag:yaml.org,2002:merge") && scalar.value.as_ref() == b"<<",
    }
}

fn is_merge_value(value: &Value) -> bool {
    match value.untag() {
        Value::Mapping(_) => true,
        Value::Sequence(items) => items.iter().all(|item| matches!(item.untag(), Value::Mapping(_))),
        _ => false,
    }
}

fn count_values(value: &Value) -> usize {
    match value {
        Value::Tagged(tagged) => 1 + count_values(&tagged.value),
        Value::Sequence(values) => 1 + values.iter().map(count_values).sum::<usize>(),
        Value::Mapping(mapping) => {
            1 + mapping
                .iter()
                .map(|(key, value)| count_values(key) + count_values(value))
                .sum::<usize>()
        }
        _ => 1,
    }
}

fn convert_error(error: LibyamlError) -> Error {
    let mark = error.mark();
    if mark.line() == 0 && mark.column() == 0 {
        return Error::new(error.to_string());
    }
    let line = mark.line().saturating_add(1);
    let column = mark.column().saturating_add(1);
    Error::at(error.to_string(), line, column)
}

fn tag_string(tag: Option<&crate::libyaml::tag::Tag>) -> Option<String> {
    tag.map(|tag| String::from_utf8_lossy(tag).into_owned())
}

fn apply_tag(value: Value, tag: Option<String>) -> Result<Value> {
    let Some(tag) = tag else {
        return Ok(value);
    };
    let tagged = |tag: String, value: Value| {
        Value::Tagged(alloc::boxed::Box::new(crate::TaggedValue {
            tag,
            value,
        }))
    };
    match tag.as_str() {
        "tag:yaml.org,2002:seq" => match value {
            Value::Sequence(_) => Ok(value),
            _ => Err(Error::new("explicit `!!seq` tag on a non-sequence")),
        },
        "tag:yaml.org,2002:map" => match value {
            Value::Mapping(_) => Ok(value),
            _ => Err(Error::new("explicit `!!map` tag on a non-mapping")),
        },
        "tag:yaml.org,2002:set" | "tag:yaml.org,2002:omap" | "tag:yaml.org,2002:pairs" => match value {
            Value::Mapping(_) | Value::Sequence(_) => Ok(tagged(tag, value)),
            _ => Err(Error::new("explicit collection tag on a scalar")),
        },
        "tag:yaml.org,2002:null"
        | "tag:yaml.org,2002:bool"
        | "tag:yaml.org,2002:int"
        | "tag:yaml.org,2002:float"
        | "tag:yaml.org,2002:str"
        | "tag:yaml.org,2002:binary"
        | "tag:yaml.org,2002:timestamp" => Err(Error::new("explicit scalar tag on a collection")),
        "!" => Ok(value),
        _ => Ok(tagged(tag, value)),
    }
}

fn scalar_value(scalar: &Scalar<'_>) -> Result<Value> {
    let text = String::from_utf8_lossy(&scalar.value);
    if let Some(tag) = tag_string(scalar.tag.as_ref()) {
        let inner = match tag.as_str() {
            "tag:yaml.org,2002:null" => Value::Null,
            // `!` is the non-specific primary tag: scalars are strings.
            "!" => Value::String(text.to_string()),
            "tag:yaml.org,2002:bool" => Value::Bool(
                parse_bool(&text).ok_or_else(|| Error::new(alloc::format!("invalid `!!bool` value `{text}`")))?,
            ),
            "tag:yaml.org,2002:int" => Value::Number(
                parse_integer(&text).ok_or_else(|| Error::new(alloc::format!("invalid `!!int` value `{text}`")))?,
            ),
            "tag:yaml.org,2002:float" => Value::Number(
                parse_float_explicit(&text)
                    .ok_or_else(|| Error::new(alloc::format!("invalid `!!float` value `{text}`")))?,
            ),
            "tag:yaml.org,2002:str" => Value::String(text.to_string()),
            // Binary and timestamp values are not converted into native types,
            // but keep the tag and exact scalar text instead of dropping the
            // tag or resolving dates such as `2020` to numbers.
            "tag:yaml.org,2002:binary" | "tag:yaml.org,2002:timestamp" => {
                return Ok(Value::Tagged(alloc::boxed::Box::new(crate::TaggedValue {
                    tag,
                    value: Value::String(text.to_string()),
                })));
            }
            "tag:yaml.org,2002:seq"
            | "tag:yaml.org,2002:map"
            | "tag:yaml.org,2002:set"
            | "tag:yaml.org,2002:omap"
            | "tag:yaml.org,2002:pairs" => {
                return Err(Error::new(alloc::format!("explicit collection tag `{tag}` on a scalar")));
            }
            _ => {
                // Keep the typed inner value for the crate's own tagged
                // round-trips, but respect the scalar style: a quoted or block
                // scalar is a string, not a number/bool/null.
                let inner = match scalar.style {
                    ScalarStyle::Plain => resolve_plain(&text),
                    _ => Value::String(text.to_string()),
                };
                return Ok(Value::Tagged(alloc::boxed::Box::new(crate::TaggedValue {
                    tag,
                    value: inner,
                })));
            }
        };
        return Ok(inner);
    }

    Ok(match scalar.style {
        ScalarStyle::Plain => resolve_plain(&text),
        _ => Value::String(text.to_string()),
    })
}

fn resolve_plain(text: &str) -> Value {
    if text.is_empty() || matches!(text, "~" | "null" | "Null" | "NULL") {
        return Value::Null;
    }
    if let Some(value) = parse_bool(text) {
        return Value::Bool(value);
    }
    if let Some(value) = parse_integer(text) {
        return Value::Number(value);
    }
    if let Some(value) = parse_float(text) {
        return Value::Number(value);
    }
    Value::String(text.to_string())
}

fn parse_bool(text: &str) -> Option<bool> {
    match text {
        "true" | "True" | "TRUE" => Some(true),
        "false" | "False" | "FALSE" => Some(false),
        _ => None,
    }
}

fn parse_integer(text: &str) -> Option<Number> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    };
    if digits.is_empty() {
        return None;
    }
    let parsed = if let Some(hex) = digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else if let Some(octal) = digits.strip_prefix("0o").or_else(|| digits.strip_prefix("0O")) {
        u64::from_str_radix(octal, 8).ok()
    } else if let Some(binary) = digits.strip_prefix("0b").or_else(|| digits.strip_prefix("0B")) {
        u64::from_str_radix(binary, 2).ok()
    } else if digits.bytes().all(|byte| byte.is_ascii_digit()) {
        digits.parse::<u64>().ok()
    } else {
        None
    }?;
    if negative {
        // `parsed` is the magnitude, so `i64::MIN` (whose magnitude does not
        // fit in `i64`) needs its own case.
        let value = if parsed == i64::MAX as u64 + 1 {
            i64::MIN
        } else {
            i64::try_from(parsed).ok()?.checked_neg()?
        };
        if value == 0 {
            Some(Number::PosInt(0))
        } else {
            Some(Number::NegInt(value))
        }
    } else {
        Some(Number::PosInt(parsed))
    }
}

/// Parses the content of an explicit `!!float` tag.
///
/// Unlike implicit resolution, an explicit float tag also accepts integer-form
/// content such as `1` or `-1`; plain `1` must stay an integer elsewhere.
fn parse_float_explicit(text: &str) -> Option<Number> {
    parse_float(text).or_else(|| text.parse::<f64>().ok().map(Number::Float))
}

fn parse_float(text: &str) -> Option<Number> {
    match text {
        ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => return Some(Number::Float(f64::INFINITY)),
        "-.inf" | "-.Inf" | "-.INF" => return Some(Number::Float(f64::NEG_INFINITY)),
        ".nan" | ".NaN" | ".NAN" => return Some(Number::Float(f64::NAN)),
        _ => {}
    }
    if !text.contains(['.', 'e', 'E']) {
        return None;
    }
    text.parse::<f64>().ok().map(Number::Float)
}

#[allow(dead_code)]
fn _to_string_is_used(value: &str) -> String {
    value.to_string()
}
