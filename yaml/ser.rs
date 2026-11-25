//! Emitting [`Value`]s as YAML text.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    Encode, Error, Result, Value,
    libyaml::{
        EmitEvent, EmitScalar, EmitScalarStyle, Emitter, Mapping as EmitMapping, Sequence as EmitSequence,
        error::Error as LibyamlError,
    },
};

/// Serializes an encodable value as a YAML document.
pub fn to_string<T: Encode + ?Sized>(value: &T) -> Result<String> {
    let bytes = to_vec(value)?;
    String::from_utf8(bytes).map_err(|_| Error::new("YAML emitter produced invalid UTF-8"))
}

/// Serializes an encodable value as a YAML document into a byte vector.
pub fn to_vec<T: Encode + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let value = value.encode();
    let mut emitter = Emitter::new();
    emitter.emit(EmitEvent::StreamStart).map_err(convert_error)?;
    emitter.emit(EmitEvent::DocumentStart).map_err(convert_error)?;
    emit_value(&value, &mut emitter)?;
    emitter.emit(EmitEvent::DocumentEnd).map_err(convert_error)?;
    emitter.emit(EmitEvent::StreamEnd).map_err(convert_error)?;
    emitter.flush().map_err(convert_error)?;
    Ok(emitter.into_output())
}

#[cfg(feature = "std")]
/// Serializes an encodable value as a YAML document into a writer.
pub fn to_writer<T: Encode + ?Sized, W: std::io::Write>(value: &T, mut writer: W) -> std::io::Result<()> {
    let bytes = to_vec(value).map_err(std::io::Error::other)?;
    writer.write_all(&bytes)
}

fn emit_value(value: &Value, emitter: &mut Emitter) -> Result<()> {
    match value {
        Value::Null => emit_scalar("null", EmitScalarStyle::Plain, None, emitter),
        Value::Bool(value) => emit_scalar(if *value { "true" } else { "false" }, EmitScalarStyle::Plain, None, emitter),
        Value::Number(value) => emit_scalar(&value.to_string(), EmitScalarStyle::Plain, None, emitter),
        Value::String(value) => emit_scalar(value, string_style(value), None, emitter),
        Value::Sequence(values) => {
            emitter
                .emit(EmitEvent::SequenceStart(EmitSequence {
                    tag: None,
                }))
                .map_err(convert_error)?;
            for value in values {
                emit_value(value, emitter)?;
            }
            emitter.emit(EmitEvent::SequenceEnd).map_err(convert_error)
        }
        Value::Mapping(mapping) => {
            emitter
                .emit(EmitEvent::MappingStart(EmitMapping {
                    tag: None,
                }))
                .map_err(convert_error)?;
            for (key, value) in mapping.iter() {
                emit_value(key, emitter)?;
                emit_value(value, emitter)?;
            }
            emitter.emit(EmitEvent::MappingEnd).map_err(convert_error)
        }
        Value::Tagged(tagged) => {
            let tag = Some(tagged.tag.clone());
            match &tagged.value {
                Value::Null => emit_scalar("null", EmitScalarStyle::Plain, tag, emitter),
                Value::Bool(value) => {
                    emit_scalar(if *value { "true" } else { "false" }, EmitScalarStyle::Plain, tag, emitter)
                }
                Value::Number(value) => emit_scalar(&value.to_string(), EmitScalarStyle::Plain, tag, emitter),
                Value::String(value) => emit_scalar(value, string_style(value), tag, emitter),
                Value::Sequence(values) => {
                    emitter
                        .emit(EmitEvent::SequenceStart(EmitSequence {
                            tag,
                        }))
                        .map_err(convert_error)?;
                    for value in values {
                        emit_value(value, emitter)?;
                    }
                    emitter.emit(EmitEvent::SequenceEnd).map_err(convert_error)
                }
                Value::Mapping(mapping) => {
                    emitter
                        .emit(EmitEvent::MappingStart(EmitMapping {
                            tag,
                        }))
                        .map_err(convert_error)?;
                    for (key, value) in mapping.iter() {
                        emit_value(key, emitter)?;
                        emit_value(value, emitter)?;
                    }
                    emitter.emit(EmitEvent::MappingEnd).map_err(convert_error)
                }
                Value::Tagged(inner) => {
                    let inner = Value::Tagged(inner.clone());
                    emit_value(&inner, emitter)
                }
            }
        }
    }
}

fn emit_scalar(value: &str, style: EmitScalarStyle, tag: Option<String>, emitter: &mut Emitter) -> Result<()> {
    emitter
        .emit(EmitEvent::Scalar(EmitScalar {
            tag,
            value,
            style,
        }))
        .map_err(convert_error)
}

fn string_style(value: &str) -> EmitScalarStyle {
    if value.contains('\n') {
        return EmitScalarStyle::Literal;
    }
    if value
        .chars()
        .any(|character| character.is_control() && character != '\t')
    {
        return EmitScalarStyle::DoubleQuoted;
    }
    if value.contains('\t') {
        return EmitScalarStyle::DoubleQuoted;
    }
    if plain_safe(value) {
        EmitScalarStyle::Plain
    } else {
        EmitScalarStyle::SingleQuoted
    }
}

fn plain_safe(value: &str) -> bool {
    if value.is_empty()
        || matches!(
            value,
            "null" | "Null" | "NULL" | "~" | "true" | "True" | "TRUE" | "false" | "False" | "FALSE"
        )
    {
        return false;
    }
    if value.parse::<i64>().is_ok() || value.parse::<f64>().is_ok() {
        return false;
    }
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.' | '/'))
}

fn convert_error(error: LibyamlError) -> Error {
    Error::new(error.to_string())
}
