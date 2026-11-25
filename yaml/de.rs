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
pub fn from_str_value_with_options(input: &[u8], options: Options) -> Result<Value> {
    let mut documents = from_slice_multi_value(input, options)?;
    if documents.is_empty() {
        return Err(Error::new("empty YAML stream"));
    }
    Ok(documents.remove(0))
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
pub fn from_slice_multi_value(input: &[u8], options: Options) -> Result<Vec<Value>> {
    let mut parser = Parser::new(Cow::Borrowed(input));
    let mut documents = ParserState::new(options).parse(&mut parser)?;
    if documents.is_empty() {
        documents.push(Value::Null);
    }
    Ok(documents)
}

#[cfg(feature = "std")]
/// Parses the first document from a reader.
pub fn from_reader<T: crate::Decode, R: std::io::Read>(mut reader: R) -> Result<T> {
    let mut input = Vec::new();
    reader
        .read_to_end(&mut input)
        .map_err(|error| Error::new(alloc::format!("failed to read YAML input: {error}")))?;
    from_slice(&input)
}

struct ParserState {
    options: Options,
    stack: Vec<Frame>,
    anchors: BTreeMap<Vec<u8>, Value>,
    root: Option<Value>,
    documents: Vec<Value>,
    values: usize,
    aliases: usize,
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
        merges: Vec<Value>,
        anchor: Option<Anchor>,
        tag: Option<String>,
    },
}

impl ParserState {
    fn new(options: Options) -> Self {
        Self {
            options,
            stack: Vec::new(),
            anchors: BTreeMap::new(),
            root: None,
            documents: Vec::new(),
            values: 0,
            aliases: 0,
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
                }
                Event::StreamEnd => {
                    if self.root.is_some() {
                        self.finish_document()?;
                    }
                    break;
                }
                Event::DocumentEnd => self.finish_document()?,
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
                    self.check_value_budget(&value)?;
                    self.push(value)?;
                }
                Event::Scalar(scalar) => {
                    let value = scalar_value(&scalar);
                    if let Some(anchor) = &scalar.anchor {
                        self.anchors.insert(anchor.as_bytes().to_vec(), value.clone());
                    }
                    self.push(value)?;
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

    fn check_value_budget(&self, value: &Value) -> Result<()> {
        if let Some(limit) = self.options.max_values {
            let count = count_values(value);
            if self.values.saturating_add(count) > limit {
                return Err(Error::new("YAML value limit exceeded"));
            }
        }
        Ok(())
    }

    fn finish_container(&mut self, value: Value, anchor: Option<Anchor>, tag: Option<String>) -> Result<()> {
        let value = apply_tag(value, tag);
        if let Some(anchor) = anchor {
            self.anchors.insert(anchor.as_bytes().to_vec(), value.clone());
        }
        self.push(value)
    }

    fn push(&mut self, value: Value) -> Result<()> {
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
                merges,
                ..
            }) => match key.take() {
                None => *key = Some(value),
                Some(key) if is_merge_key(&key) && is_merge_value(&value) => match value.into_untagged() {
                    Value::Mapping(mapping) => merges.push(Value::Mapping(mapping)),
                    Value::Sequence(items) => merges.extend(items),
                    _ => unreachable!(),
                },
                Some(key) => insert_pair(values, key, value, self.options.duplicate_keys)?,
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

fn is_merge_key(key: &Value) -> bool {
    key.as_str() == Some("<<")
}

fn is_merge_value(value: &Value) -> bool {
    match value.untag() {
        Value::Mapping(_) => true,
        Value::Sequence(items) => items.iter().all(|item| matches!(item.untag(), Value::Mapping(_))),
        _ => false,
    }
}

fn count_values(value: &Value) -> usize {
    match value.untag() {
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
    let line = mark.line().saturating_add(1);
    let column = mark.column().saturating_add(1);
    Error::at(error.to_string(), line, column)
}

fn tag_string(tag: Option<&crate::libyaml::tag::Tag>) -> Option<String> {
    tag.map(|tag| String::from_utf8_lossy(tag).into_owned())
}

fn apply_tag(value: Value, tag: Option<String>) -> Value {
    let Some(tag) = tag else {
        return value;
    };
    match tag.as_str() {
        "tag:yaml.org,2002:seq" | "tag:yaml.org,2002:map" | "!" => value,
        _ => Value::Tagged(alloc::boxed::Box::new(crate::TaggedValue {
            tag,
            value,
        })),
    }
}

fn scalar_value(scalar: &Scalar<'_>) -> Value {
    let text = String::from_utf8_lossy(&scalar.value);
    if let Some(tag) = tag_string(scalar.tag.as_ref()) {
        let inner = match tag.as_str() {
            "tag:yaml.org,2002:null" | "!" => Value::Null,
            "tag:yaml.org,2002:bool" => parse_bool(&text).map_or_else(|| Value::String(text.to_string()), Value::Bool),
            "tag:yaml.org,2002:int" => {
                parse_integer(&text).map_or_else(|| Value::String(text.to_string()), Value::Number)
            }
            "tag:yaml.org,2002:float" => {
                parse_float(&text).map_or_else(|| Value::String(text.to_string()), Value::Number)
            }
            "tag:yaml.org,2002:str" | "tag:yaml.org,2002:binary" => Value::String(text.to_string()),
            _ => {
                let inner = resolve_plain(&text);
                return Value::Tagged(alloc::boxed::Box::new(crate::TaggedValue {
                    tag,
                    value: inner,
                }));
            }
        };
        return inner;
    }

    match scalar.style {
        ScalarStyle::Plain => resolve_plain(&text),
        _ => Value::String(text.to_string()),
    }
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
        let value = i64::try_from(parsed).ok()?.checked_neg()?;
        Some(Number::NegInt(value))
    } else {
        Some(Number::PosInt(parsed))
    }
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
